pub fn sub_822E9900(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x822E9900 size=16
    let mut pc: u32 = 0x822E9900;
    'dispatch: loop {
        match pc {
            0x822E9900 => {
    //   block [0x822E9900..0x822E9910)
	// 822E9900: 81630050  lwz r11, 0x50(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(80 as u32) ) } as u64;
	// 822E9904: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 822E9908: 5563DFFE  rlwinm r3, r11, 0x1b, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 822E990C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E9910(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x822E9910 size=444
    let mut pc: u32 = 0x822E9910;
    'dispatch: loop {
        match pc {
            0x822E9910 => {
    //   block [0x822E9910..0x822E9ACC)
	// 822E9910: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E9914: 48EBE859  bl 0x831a816c
	ctx.lr = 0x822E9918;
	sub_831A8130(ctx, base);
	// 822E9918: DBC1FFD0  stfd f30, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[30].u64 ) };
	// 822E991C: DBE1FFD8  stfd f31, -0x28(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 822E9920: 3980FFC0  li r12, -0x40
	ctx.r[12].s64 = -64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E9AD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x822E9AD0 size=12
    let mut pc: u32 = 0x822E9AD0;
    'dispatch: loop {
        match pc {
            0x822E9AD0 => {
    //   block [0x822E9AD0..0x822E9ADC)
	// 822E9AD0: E8A50000  ld r5, 0(r5)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) };
	// 822E9AD4: 80630004  lwz r3, 4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 822E9AD8: 48BEB688  b 0x82ed5160
	sub_82ED5160(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E9AE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822E9AE0 size=80
    let mut pc: u32 = 0x822E9AE0;
    'dispatch: loop {
        match pc {
            0x822E9AE0 => {
    //   block [0x822E9AE0..0x822E9B30)
	// 822E9AE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E9AE4: 48EBE689  bl 0x831a816c
	ctx.lr = 0x822E9AE8;
	sub_831A8130(ctx, base);
	// 822E9AE8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E9AEC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822E9AF0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 822E9AF4: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 822E9AF8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E9AFC: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 822E9B00: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 822E9B04: 4E800421  bctrl
	ctx.lr = 0x822E9B08;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 822E9B08: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 822E9B0C: E8BD0000  ld r5, 0(r29)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) };
	// 822E9B10: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 822E9B14: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 822E9B18: 4182000C  beq 0x822e9b24
	if ctx.cr[0].eq {
	pc = 0x822E9B24; continue 'dispatch;
	}
	// 822E9B1C: 48BEB78D  bl 0x82ed52a8
	ctx.lr = 0x822E9B20;
	sub_82ED52A8(ctx, base);
	// 822E9B20: 48000008  b 0x822e9b28
	pc = 0x822E9B28; continue 'dispatch;
	// 822E9B24: 48BEB63D  bl 0x82ed5160
	ctx.lr = 0x822E9B28;
	sub_82ED5160(ctx, base);
	// 822E9B28: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 822E9B2C: 48EBE690  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E9B30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x822E9B30 size=12
    let mut pc: u32 = 0x822E9B30;
    'dispatch: loop {
        match pc {
            0x822E9B30 => {
    //   block [0x822E9B30..0x822E9B3C)
	// 822E9B30: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 822E9B34: B08B002A  sth r4, 0x2a(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(42 as u32), ctx.r[4].u16 ) };
	// 822E9B38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E9B40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x822E9B40 size=12
    let mut pc: u32 = 0x822E9B40;
    'dispatch: loop {
        match pc {
            0x822E9B40 => {
    //   block [0x822E9B40..0x822E9B4C)
	// 822E9B40: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 822E9B44: 386B00E0  addi r3, r11, 0xe0
	ctx.r[3].s64 = ctx.r[11].s64 + 224;
	// 822E9B48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E9B50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822E9B50 size=80
    let mut pc: u32 = 0x822E9B50;
    'dispatch: loop {
        match pc {
            0x822E9B50 => {
    //   block [0x822E9B50..0x822E9BA0)
	// 822E9B50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E9B54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822E9B58: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 822E9B5C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822E9B60: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E9B64: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 822E9B68: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 822E9B6C: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 822E9B70: 419A0018  beq cr6, 0x822e9b88
	if ctx.cr[6].eq {
	pc = 0x822E9B88; continue 'dispatch;
	}
	// 822E9B74: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 822E9B78: 4BFFEB91  bl 0x822e8708
	ctx.lr = 0x822E9B7C;
	sub_822E8708(ctx, base);
	// 822E9B7C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 822E9B80: 807E005C  lwz r3, 0x5c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(92 as u32) ) } as u64;
	// 822E9B84: 4BFFEBCD  bl 0x822e8750
	ctx.lr = 0x822E9B88;
	sub_822E8750(ctx, base);
	// 822E9B88: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 822E9B8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822E9B90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822E9B94: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 822E9B98: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822E9B9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E9BA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822E9BA0 size=124
    let mut pc: u32 = 0x822E9BA0;
    'dispatch: loop {
        match pc {
            0x822E9BA0 => {
    //   block [0x822E9BA0..0x822E9C1C)
	// 822E9BA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E9BA4: 48EBE5C9  bl 0x831a816c
	ctx.lr = 0x822E9BA8;
	sub_831A8130(ctx, base);
	// 822E9BA8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E9BAC: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 822E9BB0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 822E9BB4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 822E9BB8: 419A005C  beq cr6, 0x822e9c14
	if ctx.cr[6].eq {
	pc = 0x822E9C14; continue 'dispatch;
	}
	// 822E9BBC: 83DF0008  lwz r30, 8(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 822E9BC0: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 822E9BC4: 419A000C  beq cr6, 0x822e9bd0
	if ctx.cr[6].eq {
	pc = 0x822E9BD0; continue 'dispatch;
	}
	// 822E9BC8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 822E9BCC: 48BE4C35  bl 0x82ece800
	ctx.lr = 0x822E9BD0;
	sub_82ECE800(ctx, base);
	// 822E9BD0: 817D0050  lwz r11, 0x50(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(80 as u32) ) } as u64;
	// 822E9BD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 822E9BD8: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 822E9BDC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 822E9BE0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E9BE4: 419A0018  beq cr6, 0x822e9bfc
	if ctx.cr[6].eq {
	pc = 0x822E9BFC; continue 'dispatch;
	}
	// 822E9BE8: 38800006  li r4, 6
	ctx.r[4].s64 = 6;
	// 822E9BEC: 48BE1FA5  bl 0x82ecbb90
	ctx.lr = 0x822E9BF0;
	sub_82ECBB90(ctx, base);
	// 822E9BF0: 39600005  li r11, 5
	ctx.r[11].s64 = 5;
	// 822E9BF4: B17F002A  sth r11, 0x2a(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(42 as u32), ctx.r[11].u16 ) };
	// 822E9BF8: 4800000C  b 0x822e9c04
	pc = 0x822E9C04; continue 'dispatch;
	// 822E9BFC: 38800007  li r4, 7
	ctx.r[4].s64 = 7;
	// 822E9C00: 48BE1F91  bl 0x82ecbb90
	ctx.lr = 0x822E9C04;
	sub_82ECBB90(ctx, base);
	// 822E9C04: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 822E9C08: 419A000C  beq cr6, 0x822e9c14
	if ctx.cr[6].eq {
	pc = 0x822E9C14; continue 'dispatch;
	}
	// 822E9C0C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 822E9C10: 48BE33F9  bl 0x82ecd008
	ctx.lr = 0x822E9C14;
	sub_82ECD008(ctx, base);
	// 822E9C14: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 822E9C18: 48EBE5A4  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E9C20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x822E9C20 size=204
    let mut pc: u32 = 0x822E9C20;
    'dispatch: loop {
        match pc {
            0x822E9C20 => {
    //   block [0x822E9C20..0x822E9CEC)
	// 822E9C20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E9C24: 48EBE545  bl 0x831a8168
	ctx.lr = 0x822E9C28;
	sub_831A8130(ctx, base);
	// 822E9C28: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E9C2C: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 822E9C30: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 822E9C34: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 822E9C38: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 822E9C3C: 419A00A8  beq cr6, 0x822e9ce4
	if ctx.cr[6].eq {
	pc = 0x822E9CE4; continue 'dispatch;
	}
	// 822E9C40: 83DF0008  lwz r30, 8(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 822E9C44: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 822E9C48: 419A000C  beq cr6, 0x822e9c54
	if ctx.cr[6].eq {
	pc = 0x822E9C54; continue 'dispatch;
	}
	// 822E9C4C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 822E9C50: 48BE4BB1  bl 0x82ece800
	ctx.lr = 0x822E9C54;
	sub_82ECE800(ctx, base);
	// 822E9C54: 817D0050  lwz r11, 0x50(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(80 as u32) ) } as u64;
	// 822E9C58: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 822E9C5C: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 822E9C60: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 822E9C64: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E9C68: 419A0018  beq cr6, 0x822e9c80
	if ctx.cr[6].eq {
	pc = 0x822E9C80; continue 'dispatch;
	}
	// 822E9C6C: 38800006  li r4, 6
	ctx.r[4].s64 = 6;
	// 822E9C70: 48BE1F21  bl 0x82ecbb90
	ctx.lr = 0x822E9C74;
	sub_82ECBB90(ctx, base);
	// 822E9C74: 39600005  li r11, 5
	ctx.r[11].s64 = 5;
	// 822E9C78: B17F002A  sth r11, 0x2a(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(42 as u32), ctx.r[11].u16 ) };
	// 822E9C7C: 4800000C  b 0x822e9c88
	pc = 0x822E9C88; continue 'dispatch;
	// 822E9C80: 38800007  li r4, 7
	ctx.r[4].s64 = 7;
	// 822E9C84: 48BE1F0D  bl 0x82ecbb90
	ctx.lr = 0x822E9C88;
	sub_82ECBB90(ctx, base);
	// 822E9C88: 807C0000  lwz r3, 0(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E9C8C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E9C90: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 822E9C94: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 822E9C98: 4E800421  bctrl
	ctx.lr = 0x822E9C9C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 822E9C9C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 822E9CA0: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 822E9CA4: 4BFFEBE5  bl 0x822e8888
	ctx.lr = 0x822E9CA8;
	sub_822E8888(ctx, base);
	// 822E9CA8: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 822E9CAC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E9CB0: 48BE23E9  bl 0x82ecc098
	ctx.lr = 0x822E9CB4;
	sub_82ECC098(ctx, base);
	// 822E9CB4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 822E9CB8: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 822E9CBC: 396BBC40  addi r11, r11, -0x43c0
	ctx.r[11].s64 = ctx.r[11].s64 + -17344;
	// 822E9CC0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 822E9CC4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E9CC8: 13E058C7  vcmpequd (lvx128) v31, v0, v11
	tmp.u32 = ctx.r[11].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E9CF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x822E9CF0 size=8
    let mut pc: u32 = 0x822E9CF0;
    'dispatch: loop {
        match pc {
            0x822E9CF0 => {
    //   block [0x822E9CF0..0x822E9CF8)
	// 822E9CF0: 80A30004  lwz r5, 4(r3)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 822E9CF4: 4BFFFE5C  b 0x822e9b50
	sub_822E9B50(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E9CF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822E9CF8 size=232
    let mut pc: u32 = 0x822E9CF8;
    'dispatch: loop {
        match pc {
            0x822E9CF8 => {
    //   block [0x822E9CF8..0x822E9DE0)
	// 822E9CF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E9CFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822E9D00: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 822E9D04: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822E9D08: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E9D0C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822E9D10: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 822E9D14: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 822E9D18: 396BBC60  addi r11, r11, -0x43a0
	ctx.r[11].s64 = ctx.r[11].s64 + -17312;
	// 822E9D1C: 394ABC54  addi r10, r10, -0x43ac
	ctx.r[10].s64 = ctx.r[10].s64 + -17324;
	// 822E9D20: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 822E9D24: 3BDF0048  addi r30, r31, 0x48
	ctx.r[30].s64 = ctx.r[31].s64 + 72;
	// 822E9D28: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 822E9D2C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 822E9D30: 915F0048  stw r10, 0x48(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[10].u32 ) };
	// 822E9D34: 419A000C  beq cr6, 0x822e9d40
	if ctx.cr[6].eq {
	pc = 0x822E9D40; continue 'dispatch;
	}
	// 822E9D38: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 822E9D3C: 48B28805  bl 0x82e12540
	ctx.lr = 0x822E9D40;
	sub_82E12540(ctx, base);
	// 822E9D40: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 822E9D44: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 822E9D48: 419A000C  beq cr6, 0x822e9d54
	if ctx.cr[6].eq {
	pc = 0x822E9D54; continue 'dispatch;
	}
	// 822E9D4C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 822E9D50: 48002961  bl 0x822ec6b0
	ctx.lr = 0x822E9D54;
	sub_822EC6B0(ctx, base);
	// 822E9D54: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 822E9D58: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E9D5C: 484F027D  bl 0x827d9fd8
	ctx.lr = 0x822E9D60;
	sub_827D9FD8(ctx, base);
	// 822E9D60: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 822E9D64: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 822E9D68: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 822E9D6C: 419A0008  beq cr6, 0x822e9d74
	if ctx.cr[6].eq {
	pc = 0x822E9D74; continue 'dispatch;
	}
	// 822E9D70: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 822E9D74: 817F005C  lwz r11, 0x5c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 822E9D78: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 822E9D7C: 419A0008  beq cr6, 0x822e9d84
	if ctx.cr[6].eq {
	pc = 0x822E9D84; continue 'dispatch;
	}
	// 822E9D80: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 822E9D84: 807F005C  lwz r3, 0x5c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 822E9D88: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 822E9D8C: 419A0008  beq cr6, 0x822e9d94
	if ctx.cr[6].eq {
	pc = 0x822E9D94; continue 'dispatch;
	}
	// 822E9D90: 4BFFE4D9  bl 0x822e8268
	ctx.lr = 0x822E9D94;
	sub_822E8268(ctx, base);
	// 822E9D94: 807F0058  lwz r3, 0x58(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 822E9D98: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 822E9D9C: 419A0008  beq cr6, 0x822e9da4
	if ctx.cr[6].eq {
	pc = 0x822E9DA4; continue 'dispatch;
	}
	// 822E9DA0: 4BFFE4C9  bl 0x822e8268
	ctx.lr = 0x822E9DA4;
	sub_822E8268(ctx, base);
	// 822E9DA4: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 822E9DA8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 822E9DAC: 419A0008  beq cr6, 0x822e9db4
	if ctx.cr[6].eq {
	pc = 0x822E9DB4; continue 'dispatch;
	}
	// 822E9DB0: 4BFD6AE1  bl 0x822c0890
	ctx.lr = 0x822E9DB4;
	sub_822C0890(ctx, base);
	// 822E9DB4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 822E9DB8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E9DBC: 396BA980  addi r11, r11, -0x5680
	ctx.r[11].s64 = ctx.r[11].s64 + -22144;
	// 822E9DC0: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 822E9DC4: 481A6F4D  bl 0x82490d10
	ctx.lr = 0x822E9DC8;
	sub_82490D10(ctx, base);
	// 822E9DC8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 822E9DCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822E9DD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822E9DD4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 822E9DD8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822E9DDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E9DE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x822E9DE0 size=8
    let mut pc: u32 = 0x822E9DE0;
    'dispatch: loop {
        match pc {
            0x822E9DE0 => {
    //   block [0x822E9DE0..0x822E9DE8)
	// 822E9DE0: 3863FFB8  addi r3, r3, -0x48
	ctx.r[3].s64 = ctx.r[3].s64 + -72;
	// 822E9DE4: 480001FC  b 0x822e9fe0
	sub_822E9FE0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E9DE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x822E9DE8 size=8
    let mut pc: u32 = 0x822E9DE8;
    'dispatch: loop {
        match pc {
            0x822E9DE8 => {
    //   block [0x822E9DE8..0x822E9DF0)
	// 822E9DE8: 80A30004  lwz r5, 4(r3)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 822E9DEC: 4BFFFDB4  b 0x822e9ba0
	sub_822E9BA0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E9DF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x822E9DF0 size=136
    let mut pc: u32 = 0x822E9DF0;
    'dispatch: loop {
        match pc {
            0x822E9DF0 => {
    //   block [0x822E9DF0..0x822E9E78)
	// 822E9DF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E9DF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822E9DF8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822E9DFC: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E9E00: 81630050  lwz r11, 0x50(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(80 as u32) ) } as u64;
	// 822E9E04: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 822E9E08: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 822E9E0C: 419A001C  beq cr6, 0x822e9e28
	if ctx.cr[6].eq {
	pc = 0x822E9E28; continue 'dispatch;
	}
	// 822E9E10: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E9E14: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 822E9E18: 816A000C  lwz r11, 0xc(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 822E9E1C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 822E9E20: 4E800421  bctrl
	ctx.lr = 0x822E9E24;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 822E9E24: 48000014  b 0x822e9e38
	pc = 0x822E9E38; continue 'dispatch;
	// 822E9E28: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 822E9E2C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 822E9E30: 388B00E0  addi r4, r11, 0xe0
	ctx.r[4].s64 = ctx.r[11].s64 + 224;
	// 822E9E34: 4BFFE865  bl 0x822e8698
	ctx.lr = 0x822E9E38;
	sub_822E8698(ctx, base);
	// 822E9E38: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 822E9E3C: 13E01C07  vcmpneb. (lvlx128) v31, v0, v3
	tmp.u32 = ctx.r[3].u32;
	// load shuffled into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 822E9E40: 39400020  li r10, 0x20
	ctx.r[10].s64 = 32;
	// 822E9E44: 39600030  li r11, 0x30
	ctx.r[11].s64 = 48;
	// 822E9E48: 13C91C07  vcmpneb. (lvlx128) v30, v9, v3
	tmp.u32 = ctx.r[9].u32 + ctx.r[3].u32;
	// load shuffled into ctx.v[62] using VectorMaskL[(tmp.u32 & 0xF)]
	// 822E9E4C: 13AA1C07  vcmpneb. (lvlx128) v29, v10, v3
	tmp.u32 = ctx.r[10].u32 + ctx.r[3].u32;
	// load shuffled into ctx.v[61] using VectorMaskL[(tmp.u32 & 0xF)]
	// 822E9E50: 138B1C07  vcmpneb. (lvlx128) v28, v11, v3
	tmp.u32 = ctx.r[11].u32 + ctx.r[3].u32;
	// load shuffled into ctx.v[60] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E9E78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x822E9E78 size=68
    let mut pc: u32 = 0x822E9E78;
    'dispatch: loop {
        match pc {
            0x822E9E78 => {
    //   block [0x822E9E78..0x822E9EBC)
	// 822E9E78: 81640004  lwz r11, 4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 822E9E7C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 822E9E80: 812B0078  lwz r9, 0x78(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(120 as u32) ) } as u64;
	// 822E9E84: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 822E9E88: 40990028  ble cr6, 0x822e9eb0
	if !ctx.cr[6].gt {
	pc = 0x822E9EB0; continue 'dispatch;
	}
	// 822E9E8C: 810B0074  lwz r8, 0x74(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(116 as u32) ) } as u64;
	// 822E9E90: 7D0B4378  mr r11, r8
	ctx.r[11].u64 = ctx.r[8].u64;
	// 822E9E94: 80EB0000  lwz r7, 0(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E9E98: 7F072840  cmplw cr6, r7, r5
	ctx.cr[6].compare_u32(ctx.r[7].u32, ctx.r[5].u32, &mut ctx.xer);
	// 822E9E9C: 419A0020  beq cr6, 0x822e9ebc
	if ctx.cr[6].eq {
		sub_822E9EBC(ctx, base);
		return;
	}
	// 822E9EA0: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 822E9EA4: 396B0010  addi r11, r11, 0x10
	ctx.r[11].s64 = ctx.r[11].s64 + 16;
	// 822E9EA8: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 822E9EAC: 4198FFE8  blt cr6, 0x822e9e94
	if ctx.cr[6].lt {
	pc = 0x822E9E94; continue 'dispatch;
	}
	// 822E9EB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 822E9EB4: F9630000  std r11, 0(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u64 ) };
	// 822E9EB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E9EBC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x822E9EBC size=16
    let mut pc: u32 = 0x822E9EBC;
    'dispatch: loop {
        match pc {
            0x822E9EBC => {
    //   block [0x822E9EBC..0x822E9ECC)
	// 822E9EBC: 554B2036  slwi r11, r10, 4
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 822E9EC0: 7D6B4214  add r11, r11, r8
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[8].u64;
	// 822E9EC4: E96B0008  ld r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	// 822E9EC8: 4BFFFFEC  b 0x822e9eb4
	sub_822E9E78(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E9ED0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x822E9ED0 size=76
    let mut pc: u32 = 0x822E9ED0;
    'dispatch: loop {
        match pc {
            0x822E9ED0 => {
    //   block [0x822E9ED0..0x822E9F1C)
	// 822E9ED0: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 822E9ED4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 822E9ED8: 812B0078  lwz r9, 0x78(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(120 as u32) ) } as u64;
	// 822E9EDC: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 822E9EE0: 40990024  ble cr6, 0x822e9f04
	if !ctx.cr[6].gt {
	pc = 0x822E9F04; continue 'dispatch;
	}
	// 822E9EE4: 816B0074  lwz r11, 0x74(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(116 as u32) ) } as u64;
	// 822E9EE8: 810B0000  lwz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E9EEC: 7F082040  cmplw cr6, r8, r4
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[4].u32, &mut ctx.xer);
	// 822E9EF0: 419A002C  beq cr6, 0x822e9f1c
	if ctx.cr[6].eq {
		sub_822E9F1C(ctx, base);
		return;
	}
	// 822E9EF4: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 822E9EF8: 396B0010  addi r11, r11, 0x10
	ctx.r[11].s64 = ctx.r[11].s64 + 16;
	// 822E9EFC: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 822E9F00: 4198FFE8  blt cr6, 0x822e9ee8
	if ctx.cr[6].lt {
	pc = 0x822E9EE8; continue 'dispatch;
	}
	// 822E9F04: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 822E9F08: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 822E9F0C: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 822E9F10: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 822E9F14: 69630001  xori r3, r11, 1
	ctx.r[3].u64 = ctx.r[11].u64 ^ 1;
	// 822E9F18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E9F1C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x822E9F1C size=8
    let mut pc: u32 = 0x822E9F1C;
    'dispatch: loop {
        match pc {
            0x822E9F1C => {
    //   block [0x822E9F1C..0x822E9F24)
	// 822E9F1C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 822E9F20: 4BFFFFE8  b 0x822e9f08
	sub_822E9ED0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E9F28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822E9F28 size=64
    let mut pc: u32 = 0x822E9F28;
    'dispatch: loop {
        match pc {
            0x822E9F28 => {
    //   block [0x822E9F28..0x822E9F68)
	// 822E9F28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E9F2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822E9F30: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822E9F34: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E9F38: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822E9F3C: 80640058  lwz r3, 0x58(r4)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(88 as u32) ) } as u64;
	// 822E9F40: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 822E9F44: 907F0000  stw r3, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 822E9F48: 419A0008  beq cr6, 0x822e9f50
	if ctx.cr[6].eq {
	pc = 0x822E9F50; continue 'dispatch;
	}
	// 822E9F4C: 4BFFE2FD  bl 0x822e8248
	ctx.lr = 0x822E9F50;
	sub_822E8248(ctx, base);
	// 822E9F50: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E9F54: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 822E9F58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822E9F5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822E9F60: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822E9F64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E9F68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822E9F68 size=120
    let mut pc: u32 = 0x822E9F68;
    'dispatch: loop {
        match pc {
            0x822E9F68 => {
    //   block [0x822E9F68..0x822E9FE0)
	// 822E9F68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E9F6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822E9F70: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822E9F74: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E9F78: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822E9F7C: 481A6FA5  bl 0x82490f20
	ctx.lr = 0x822E9F80;
	sub_82490F20(ctx, base);
	// 822E9F80: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 822E9F84: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 822E9F88: 3D008201  lis r8, -0x7dff
	ctx.r[8].s64 = -2113863680;
	// 822E9F8C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 822E9F90: 394AA980  addi r10, r10, -0x5680
	ctx.r[10].s64 = ctx.r[10].s64 + -22144;
	// 822E9F94: 3929BC60  addi r9, r9, -0x43a0
	ctx.r[9].s64 = ctx.r[9].s64 + -17312;
	// 822E9F98: 917F004C  stw r11, 0x4c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(76 as u32), ctx.r[11].u32 ) };
	// 822E9F9C: 3908BC54  addi r8, r8, -0x43ac
	ctx.r[8].s64 = ctx.r[8].s64 + -17324;
	// 822E9FA0: 915F0048  stw r10, 0x48(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[10].u32 ) };
	// 822E9FA4: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 822E9FA8: 395F0048  addi r10, r31, 0x48
	ctx.r[10].s64 = ctx.r[31].s64 + 72;
	// 822E9FAC: 911F0048  stw r8, 0x48(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[8].u32 ) };
	// 822E9FB0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E9FB4: 917F0050  stw r11, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 822E9FB8: 917F0054  stw r11, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 822E9FBC: 917F0058  stw r11, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 822E9FC0: 917F005C  stw r11, 0x5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 822E9FC4: 997F0060  stb r11, 0x60(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[11].u8 ) };
	// 822E9FC8: 997F0061  stb r11, 0x61(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(97 as u32), ctx.r[11].u8 ) };
	// 822E9FCC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 822E9FD0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822E9FD4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822E9FD8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822E9FDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E9FE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822E9FE0 size=76
    let mut pc: u32 = 0x822E9FE0;
    'dispatch: loop {
        match pc {
            0x822E9FE0 => {
    //   block [0x822E9FE0..0x822EA02C)
	// 822E9FE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E9FE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822E9FE8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 822E9FEC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822E9FF0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E9FF4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822E9FF8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 822E9FFC: 4BFFFCFD  bl 0x822e9cf8
	ctx.lr = 0x822EA000;
	sub_822E9CF8(ctx, base);
	// 822EA000: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 822EA004: 4182000C  beq 0x822ea010
	if ctx.cr[0].eq {
	pc = 0x822EA010; continue 'dispatch;
	}
	// 822EA008: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822EA00C: 48B083CD  bl 0x82df23d8
	ctx.lr = 0x822EA010;
	sub_82DF23D8(ctx, base);
	// 822EA010: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822EA014: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 822EA018: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822EA01C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822EA020: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 822EA024: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822EA028: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822EA030(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822EA030 size=84
    let mut pc: u32 = 0x822EA030;
    'dispatch: loop {
        match pc {
            0x822EA030 => {
    //   block [0x822EA030..0x822EA084)
	// 822EA030: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822EA034: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822EA038: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 822EA03C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822EA040: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822EA044: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822EA048: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 822EA04C: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 822EA050: 4BFFCC01  bl 0x822e6c50
	ctx.lr = 0x822EA054;
	sub_822E6C50(ctx, base);
	// 822EA054: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 822EA058: 93EB000C  stw r31, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 822EA05C: 817F005C  lwz r11, 0x5c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 822EA060: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 822EA064: 419A0008  beq cr6, 0x822ea06c
	if ctx.cr[6].eq {
	pc = 0x822EA06C; continue 'dispatch;
	}
	// 822EA068: 93EB000C  stw r31, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 822EA06C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 822EA070: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822EA074: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822EA078: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 822EA07C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822EA080: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822EA088(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822EA088 size=140
    let mut pc: u32 = 0x822EA088;
    'dispatch: loop {
        match pc {
            0x822EA088 => {
    //   block [0x822EA088..0x822EA114)
	// 822EA088: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822EA08C: 48EBE0E1  bl 0x831a816c
	ctx.lr = 0x822EA090;
	sub_831A8130(ctx, base);
	// 822EA090: 9421FF10  stwu r1, -0xf0(r1)
	ea = ctx.r[1].u32.wrapping_add(-240 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822EA094: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 822EA098: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 822EA09C: 817E005C  lwz r11, 0x5c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(92 as u32) ) } as u64;
	// 822EA0A0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 822EA0A4: 419A0034  beq cr6, 0x822ea0d8
	if ctx.cr[6].eq {
	pc = 0x822EA0D8; continue 'dispatch;
	}
	// 822EA0A8: 83FD0000  lwz r31, 0(r29)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 822EA0AC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 822EA0B0: 419A000C  beq cr6, 0x822ea0bc
	if ctx.cr[6].eq {
	pc = 0x822EA0BC; continue 'dispatch;
	}
	// 822EA0B4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822EA0B8: 48BE4749  bl 0x82ece800
	ctx.lr = 0x822EA0BC;
	sub_82ECE800(ctx, base);
	// 822EA0BC: 809E005C  lwz r4, 0x5c(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(92 as u32) ) } as u64;
	// 822EA0C0: 807D0000  lwz r3, 0(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 822EA0C4: 48BE497D  bl 0x82ecea40
	ctx.lr = 0x822EA0C8;
	sub_82ECEA40(ctx, base);
	// 822EA0C8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 822EA0CC: 419A000C  beq cr6, 0x822ea0d8
	if ctx.cr[6].eq {
	pc = 0x822EA0D8; continue 'dispatch;
	}
	// 822EA0D0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822EA0D4: 48BE2F35  bl 0x82ecd008
	ctx.lr = 0x822EA0D8;
	sub_82ECD008(ctx, base);
	// 822EA0D8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 822EA0DC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 822EA0E0: 4BFFFD11  bl 0x822e9df0
	ctx.lr = 0x822EA0E4;
	sub_822E9DF0(ctx, base);
	// 822EA0E4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 822EA0E8: 38610090  addi r3, r1, 0x90
	ctx.r[3].s64 = ctx.r[1].s64 + 144;
	// 822EA0EC: 83FE0004  lwz r31, 4(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 822EA0F0: 4BFFE799  bl 0x822e8888
	ctx.lr = 0x822EA0F4;
	sub_822E8888(ctx, base);
	// 822EA0F4: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 822EA0F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822EA0FC: 48BE1F9D  bl 0x82ecc098
	ctx.lr = 0x822EA100;
	sub_82ECC098(ctx, base);
	// 822EA100: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 822EA104: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 822EA108: 48002629  bl 0x822ec730
	ctx.lr = 0x822EA10C;
	sub_822EC730(ctx, base);
	// 822EA10C: 382100F0  addi r1, r1, 0xf0
	ctx.r[1].s64 = ctx.r[1].s64 + 240;
	// 822EA110: 48EBE0AC  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822EA118(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822EA118 size=100
    let mut pc: u32 = 0x822EA118;
    'dispatch: loop {
        match pc {
            0x822EA118 => {
    //   block [0x822EA118..0x822EA17C)
	// 822EA118: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822EA11C: 48EBE051  bl 0x831a816c
	ctx.lr = 0x822EA120;
	sub_831A8130(ctx, base);
	// 822EA120: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822EA124: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822EA128: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 822EA12C: 3BDF0050  addi r30, r31, 0x50
	ctx.r[30].s64 = ctx.r[31].s64 + 80;
	// 822EA130: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 822EA134: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 822EA138: 419A000C  beq cr6, 0x822ea144
	if ctx.cr[6].eq {
	pc = 0x822EA144; continue 'dispatch;
	}
	// 822EA13C: 389F0048  addi r4, r31, 0x48
	ctx.r[4].s64 = ctx.r[31].s64 + 72;
	// 822EA140: 48B28401  bl 0x82e12540
	ctx.lr = 0x822EA144;
	sub_82E12540(ctx, base);
	// 822EA144: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 822EA148: 389D0004  addi r4, r29, 4
	ctx.r[4].s64 = ctx.r[29].s64 + 4;
	// 822EA14C: 387E0004  addi r3, r30, 4
	ctx.r[3].s64 = ctx.r[30].s64 + 4;
	// 822EA150: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 822EA154: 4BFDA30D  bl 0x822c4460
	ctx.lr = 0x822EA158;
	sub_822C4460(ctx, base);
	// 822EA158: 389F0048  addi r4, r31, 0x48
	ctx.r[4].s64 = ctx.r[31].s64 + 72;
	// 822EA15C: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 822EA160: 48B28961  bl 0x82e12ac0
	ctx.lr = 0x822EA164;
	sub_82E12AC0(ctx, base);
	// 822EA164: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 822EA168: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822EA16C: 80BF0004  lwz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 822EA170: 4BFFFAB1  bl 0x822e9c20
	ctx.lr = 0x822EA174;
	sub_822E9C20(ctx, base);
	// 822EA174: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 822EA178: 48EBE044  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822EA180(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822EA180 size=220
    let mut pc: u32 = 0x822EA180;
    'dispatch: loop {
        match pc {
            0x822EA180 => {
    //   block [0x822EA180..0x822EA25C)
	// 822EA180: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822EA184: 48EBDFE5  bl 0x831a8168
	ctx.lr = 0x822EA188;
	sub_831A8130(ctx, base);
	// 822EA188: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822EA18C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822EA190: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 822EA194: 3B9F0058  addi r28, r31, 0x58
	ctx.r[28].s64 = ctx.r[31].s64 + 88;
	// 822EA198: 815F0058  lwz r10, 0x58(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 822EA19C: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 822EA1A0: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 822EA1A4: 419A00B0  beq cr6, 0x822ea254
	if ctx.cr[6].eq {
	pc = 0x822EA254; continue 'dispatch;
	}
	// 822EA1A8: 54AB063F  clrlwi. r11, r5, 0x18
	ctx.r[11].u64 = ctx.r[5].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 822EA1AC: 40820008  bne 0x822ea1b4
	if !ctx.cr[0].eq {
	pc = 0x822EA1B4; continue 'dispatch;
	}
	// 822EA1B0: 481A6AA9  bl 0x82490c58
	ctx.lr = 0x822EA1B4;
	sub_82490C58(ctx, base);
	// 822EA1B4: 817F005C  lwz r11, 0x5c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 822EA1B8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 822EA1BC: 419A001C  beq cr6, 0x822ea1d8
	if ctx.cr[6].eq {
	pc = 0x822EA1D8; continue 'dispatch;
	}
	// 822EA1C0: 5563003E  slwi r3, r11, 0
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 822EA1C4: 809D0000  lwz r4, 0(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 822EA1C8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 822EA1CC: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 822EA1D0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 822EA1D4: 4E800421  bctrl
	ctx.lr = 0x822EA1D8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 822EA1D8: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 822EA1DC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 822EA1E0: 419A0040  beq cr6, 0x822ea220
	if ctx.cr[6].eq {
	pc = 0x822EA220; continue 'dispatch;
	}
	// 822EA1E4: 83CB0008  lwz r30, 8(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 822EA1E8: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 822EA1EC: 419A000C  beq cr6, 0x822ea1f8
	if ctx.cr[6].eq {
	pc = 0x822EA1F8; continue 'dispatch;
	}
	// 822EA1F0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 822EA1F4: 48BE460D  bl 0x82ece800
	ctx.lr = 0x822EA1F8;
	sub_82ECE800(ctx, base);
	// 822EA1F8: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 822EA1FC: 809D0000  lwz r4, 0(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 822EA200: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 822EA204: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 822EA208: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 822EA20C: 4E800421  bctrl
	ctx.lr = 0x822EA210;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 822EA210: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 822EA214: 419A000C  beq cr6, 0x822ea220
	if ctx.cr[6].eq {
	pc = 0x822EA220; continue 'dispatch;
	}
	// 822EA218: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 822EA21C: 48BE2DED  bl 0x82ecd008
	ctx.lr = 0x822EA220;
	sub_82ECD008(ctx, base);
	// 822EA220: 83FD0000  lwz r31, 0(r29)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 822EA224: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 822EA228: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 822EA22C: 419A000C  beq cr6, 0x822ea238
	if ctx.cr[6].eq {
	pc = 0x822EA238; continue 'dispatch;
	}
	// 822EA230: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822EA234: 4BFFE015  bl 0x822e8248
	ctx.lr = 0x822EA238;
	sub_822E8248(ctx, base);
	// 822EA238: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 822EA23C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 822EA240: 4BFFCA11  bl 0x822e6c50
	ctx.lr = 0x822EA244;
	sub_822E6C50(ctx, base);
	// 822EA244: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 822EA248: 419A000C  beq cr6, 0x822ea254
	if ctx.cr[6].eq {
	pc = 0x822EA254; continue 'dispatch;
	}
	// 822EA24C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822EA250: 4BFFE019  bl 0x822e8268
	ctx.lr = 0x822EA254;
	sub_822E8268(ctx, base);
	// 822EA254: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 822EA258: 48EBDF60  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822EA260(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x822EA260 size=360
    let mut pc: u32 = 0x822EA260;
    'dispatch: loop {
        match pc {
            0x822EA260 => {
    //   block [0x822EA260..0x822EA3C8)
	// 822EA260: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822EA264: 48EBDEF9  bl 0x831a815c
	ctx.lr = 0x822EA268;
	sub_831A8130(ctx, base);
	// 822EA268: 9421FEA0  stwu r1, -0x160(r1)
	ea = ctx.r[1].u32.wrapping_add(-352 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822EA26C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822EA270: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 822EA274: 387F0058  addi r3, r31, 0x58
	ctx.r[3].s64 = ctx.r[31].s64 + 88;
	// 822EA278: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 822EA27C: 4BFFC9D5  bl 0x822e6c50
	ctx.lr = 0x822EA280;
	sub_822E6C50(ctx, base);
	// 822EA280: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 822EA284: 48BE2765  bl 0x82ecc9e8
	ctx.lr = 0x822EA288;
	sub_82ECC9E8(ctx, base);
	// 822EA288: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 822EA28C: 39400007  li r10, 7
	ctx.r[10].s64 = 7;
	// 822EA290: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 822EA294: 388100E0  addi r4, r1, 0xe0
	ctx.r[4].s64 = ctx.r[1].s64 + 224;
	// 822EA298: 99410110  stb r10, 0x110(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(272 as u32), ctx.r[10].u8 ) };
	// 822EA29C: C00BBCA4  lfs f0, -0x435c(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-17244 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 822EA2A0: D0010108  stfs f0, 0x108(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(264 as u32), tmp.u32 ) };
	// 822EA2A4: 817F0058  lwz r11, 0x58(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 822EA2A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 822EA2AC: 4BFFE835  bl 0x822e8ae0
	ctx.lr = 0x822EA2B0;
	sub_822E8AE0(ctx, base);
	// 822EA2B0: 83AD0000  lwz r29, 0(r13)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 822EA2B4: 3B800014  li r28, 0x14
	ctx.r[28].s64 = 20;
	// 822EA2B8: 38A0002D  li r5, 0x2d
	ctx.r[5].s64 = 45;
	// 822EA2BC: 38800200  li r4, 0x200
	ctx.r[4].s64 = 512;
	// 822EA2C0: 7C7CE82E  lwzx r3, r28, r29
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[28].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 822EA2C4: 48BB646D  bl 0x82ea0730
	ctx.lr = 0x822EA2C8;
	sub_82EA0730(ctx, base);
	// 822EA2C8: 39600200  li r11, 0x200
	ctx.r[11].s64 = 512;
	// 822EA2CC: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 822EA2D0: B1630004  sth r11, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 822EA2D4: 48BE160D  bl 0x82ecb8e0
	ctx.lr = 0x822EA2D8;
	sub_82ECB8E0(ctx, base);
	// 822EA2D8: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 822EA2DC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 822EA2E0: 38C00046  li r6, 0x46
	ctx.r[6].s64 = 70;
	// 822EA2E4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 822EA2E8: 93C10050  stw r30, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 822EA2EC: 388000FF  li r4, 0xff
	ctx.r[4].s64 = 255;
	// 822EA2F0: 386000FF  li r3, 0xff
	ctx.r[3].s64 = 255;
	// 822EA2F4: 836B7A1C  lwz r27, 0x7a1c(r11)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(31260 as u32) ) } as u64;
	// 822EA2F8: 3B3E0010  addi r25, r30, 0x10
	ctx.r[25].s64 = ctx.r[30].s64 + 16;
	// 822EA2FC: 48BDD9C5  bl 0x82ec7cc0
	ctx.lr = 0x822EA300;
	sub_82EC7CC0(ctx, base);
	// 822EA300: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 822EA304: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 822EA308: 7F25CB78  mr r5, r25
	ctx.r[5].u64 = ctx.r[25].u64;
	// 822EA30C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 822EA310: 48BDC571  bl 0x82ec6880
	ctx.lr = 0x822EA314;
	sub_82EC6880(ctx, base);
	// 822EA314: 38C00046  li r6, 0x46
	ctx.r[6].s64 = 70;
	// 822EA318: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 822EA31C: 388000FF  li r4, 0xff
	ctx.r[4].s64 = 255;
	// 822EA320: 386000FF  li r3, 0xff
	ctx.r[3].s64 = 255;
	// 822EA324: 3B610058  addi r27, r1, 0x58
	ctx.r[27].s64 = ctx.r[1].s64 + 88;
	// 822EA328: 48BDD999  bl 0x82ec7cc0
	ctx.lr = 0x822EA32C;
	sub_82EC7CC0(ctx, base);
	// 822EA32C: 7C6B07B4  extsw r11, r3
	ctx.r[11].s64 = ctx.r[3].s32 as i64;
	// 822EA330: 38801130  li r4, 0x1130
	ctx.r[4].s64 = 4400;
	// 822EA334: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 822EA338: F9610058  std r11, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u64 ) };
	// 822EA33C: E8BB0000  ld r5, 0(r27)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) };
	// 822EA340: 48BEAE21  bl 0x82ed5160
	ctx.lr = 0x822EA344;
	sub_82ED5160(ctx, base);
	// 822EA344: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 822EA348: 396BBC94  addi r11, r11, -0x436c
	ctx.r[11].s64 = ctx.r[11].s64 + -17260;
	// 822EA34C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822EA350: 917E0070  stw r11, 0x70(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(112 as u32), ctx.r[11].u32 ) };
	// 822EA354: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 822EA358: 4BFFFCD9  bl 0x822ea030
	ctx.lr = 0x822EA35C;
	sub_822EA030(ctx, base);
	// 822EA35C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822EA360: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 822EA364: 481A503D  bl 0x8248f3a0
	ctx.lr = 0x822EA368;
	sub_8248F3A0(ctx, base);
	// 822EA368: 38A00031  li r5, 0x31
	ctx.r[5].s64 = 49;
	// 822EA36C: 38800180  li r4, 0x180
	ctx.r[4].s64 = 384;
	// 822EA370: 7C7CE82E  lwzx r3, r28, r29
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[28].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 822EA374: 48BB63BD  bl 0x82ea0730
	ctx.lr = 0x822EA378;
	sub_82EA0730(ctx, base);
	// 822EA378: 39600180  li r11, 0x180
	ctx.r[11].s64 = 384;
	// 822EA37C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 822EA380: B1630004  sth r11, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 822EA384: 38DE00E0  addi r6, r30, 0xe0
	ctx.r[6].s64 = ctx.r[30].s64 + 224;
	// 822EA388: 80BF0058  lwz r5, 0x58(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 822EA38C: 80FE002C  lwz r7, 0x2c(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(44 as u32) ) } as u64;
	// 822EA390: 4BFFD831  bl 0x822e7bc0
	ctx.lr = 0x822EA394;
	sub_822E7BC0(ctx, base);
	// 822EA394: 397F005C  addi r11, r31, 0x5c
	ctx.r[11].s64 = ctx.r[31].s64 + 92;
	// 822EA398: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 822EA39C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 822EA3A0: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 822EA3A4: 480BC115  bl 0x823a64b8
	ctx.lr = 0x822EA3A8;
	sub_823A64B8(ctx, base);
	// 822EA3A8: 817F005C  lwz r11, 0x5c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 822EA3AC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 822EA3B0: 93EB000C  stw r31, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 822EA3B4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 822EA3B8: 995F0060  stb r10, 0x60(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[10].u8 ) };
	// 822EA3BC: 4BFFDEAD  bl 0x822e8268
	ctx.lr = 0x822EA3C0;
	sub_822E8268(ctx, base);
	// 822EA3C0: 38210160  addi r1, r1, 0x160
	ctx.r[1].s64 = ctx.r[1].s64 + 352;
	// 822EA3C4: 48EBDDE8  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822EA3C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x822EA3C8 size=340
    let mut pc: u32 = 0x822EA3C8;
    'dispatch: loop {
        match pc {
            0x822EA3C8 => {
    //   block [0x822EA3C8..0x822EA51C)
	// 822EA3C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822EA3CC: 48EBDDA1  bl 0x831a816c
	ctx.lr = 0x822EA3D0;
	sub_831A8130(ctx, base);
	// 822EA3D0: 9421FEC0  stwu r1, -0x140(r1)
	ea = ctx.r[1].u32.wrapping_add(-320 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822EA3D4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822EA3D8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 822EA3DC: 387F0058  addi r3, r31, 0x58
	ctx.r[3].s64 = ctx.r[31].s64 + 88;
	// 822EA3E0: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 822EA3E4: 4BFFC86D  bl 0x822e6c50
	ctx.lr = 0x822EA3E8;
	sub_822E6C50(ctx, base);
	// 822EA3E8: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 822EA3EC: 48BE25FD  bl 0x82ecc9e8
	ctx.lr = 0x822EA3F0;
	sub_82ECC9E8(ctx, base);
	// 822EA3F0: 388100E0  addi r4, r1, 0xe0
	ctx.r[4].s64 = ctx.r[1].s64 + 224;
	// 822EA3F4: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 822EA3F8: 4BFFE6E9  bl 0x822e8ae0
	ctx.lr = 0x822EA3FC;
	sub_822E8AE0(ctx, base);
	// 822EA3FC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 822EA400: 39400006  li r10, 6
	ctx.r[10].s64 = 6;
	// 822EA404: 812D0000  lwz r9, 0(r13)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 822EA408: 39000014  li r8, 0x14
	ctx.r[8].s64 = 20;
	// 822EA40C: 99410110  stb r10, 0x110(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(272 as u32), ctx.r[10].u8 ) };
	// 822EA410: 38A0002D  li r5, 0x2d
	ctx.r[5].s64 = 45;
	// 822EA414: 38800200  li r4, 0x200
	ctx.r[4].s64 = 512;
	// 822EA418: C00BBCA4  lfs f0, -0x435c(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-17244 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 822EA41C: D0010108  stfs f0, 0x108(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(264 as u32), tmp.u32 ) };
	// 822EA420: 7C68482E  lwzx r3, r8, r9
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 822EA424: 817F0058  lwz r11, 0x58(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 822EA428: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 822EA42C: 48BB6305  bl 0x82ea0730
	ctx.lr = 0x822EA430;
	sub_82EA0730(ctx, base);
	// 822EA430: 39600200  li r11, 0x200
	ctx.r[11].s64 = 512;
	// 822EA434: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 822EA438: B1630004  sth r11, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 822EA43C: 48BE14A5  bl 0x82ecb8e0
	ctx.lr = 0x822EA440;
	sub_82ECB8E0(ctx, base);
	// 822EA440: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 822EA444: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 822EA448: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 822EA44C: 93C10050  stw r30, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 822EA450: 396BBCE0  addi r11, r11, -0x4320
	ctx.r[11].s64 = ctx.r[11].s64 + -17184;
	// 822EA454: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 822EA458: 995E0080  stb r10, 0x80(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(128 as u32), ctx.r[10].u8 ) };
	// 822EA45C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822EA460: 917E0070  stw r11, 0x70(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(112 as u32), ctx.r[11].u32 ) };
	// 822EA464: 4BFFFBCD  bl 0x822ea030
	ctx.lr = 0x822EA468;
	sub_822EA030(ctx, base);
	// 822EA468: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 822EA46C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 822EA470: 388BBCA8  addi r4, r11, -0x4358
	ctx.r[4].s64 = ctx.r[11].s64 + -17240;
	// 822EA474: 38A0006B  li r5, 0x6b
	ctx.r[5].s64 = 107;
	// 822EA478: 38600008  li r3, 8
	ctx.r[3].s64 = 8;
	// 822EA47C: 4BFD5F5D  bl 0x822c03d8
	ctx.lr = 0x822EA480;
	sub_822C03D8(ctx, base);
	// 822EA480: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 822EA484: 41820010  beq 0x822ea494
	if ctx.cr[0].eq {
	pc = 0x822EA494; continue 'dispatch;
	}
	// 822EA488: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 822EA48C: 80BF0004  lwz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 822EA490: 4BFFD261  bl 0x822e76f0
	ctx.lr = 0x822EA494;
	sub_822E76F0(ctx, base);
	// 822EA494: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 822EA498: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822EA49C: 481A4F05  bl 0x8248f3a0
	ctx.lr = 0x822EA4A0;
	sub_8248F3A0(ctx, base);
	// 822EA4A0: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 822EA4A4: 3D408338  lis r10, -0x7cc8
	ctx.r[10].s64 = -2093481984;
	// 822EA4A8: 997F0060  stb r11, 0x60(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[11].u8 ) };
	// 822EA4AC: 38C000FF  li r6, 0xff
	ctx.r[6].s64 = 255;
	// 822EA4B0: 38A000FF  li r5, 0xff
	ctx.r[5].s64 = 255;
	// 822EA4B4: 3880001E  li r4, 0x1e
	ctx.r[4].s64 = 30;
	// 822EA4B8: 3860001E  li r3, 0x1e
	ctx.r[3].s64 = 30;
	// 822EA4BC: 3BBE0010  addi r29, r30, 0x10
	ctx.r[29].s64 = ctx.r[30].s64 + 16;
	// 822EA4C0: 83EA7A1C  lwz r31, 0x7a1c(r10)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(31260 as u32) ) } as u64;
	// 822EA4C4: 48BDD7FD  bl 0x82ec7cc0
	ctx.lr = 0x822EA4C8;
	sub_82EC7CC0(ctx, base);
	// 822EA4C8: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 822EA4CC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822EA4D0: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 822EA4D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 822EA4D8: 48BDC3A9  bl 0x82ec6880
	ctx.lr = 0x822EA4DC;
	sub_82EC6880(ctx, base);
	// 822EA4DC: 38C000FF  li r6, 0xff
	ctx.r[6].s64 = 255;
	// 822EA4E0: 38A000FF  li r5, 0xff
	ctx.r[5].s64 = 255;
	// 822EA4E4: 3880001E  li r4, 0x1e
	ctx.r[4].s64 = 30;
	// 822EA4E8: 3860001E  li r3, 0x1e
	ctx.r[3].s64 = 30;
	// 822EA4EC: 3BE10050  addi r31, r1, 0x50
	ctx.r[31].s64 = ctx.r[1].s64 + 80;
	// 822EA4F0: 48BDD7D1  bl 0x82ec7cc0
	ctx.lr = 0x822EA4F4;
	sub_82EC7CC0(ctx, base);
	// 822EA4F4: 7C6B07B4  extsw r11, r3
	ctx.r[11].s64 = ctx.r[3].s32 as i64;
	// 822EA4F8: 38801130  li r4, 0x1130
	ctx.r[4].s64 = 4400;
	// 822EA4FC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 822EA500: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 822EA504: E8BF0000  ld r5, 0(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) };
	// 822EA508: 48BEAC59  bl 0x82ed5160
	ctx.lr = 0x822EA50C;
	sub_82ED5160(ctx, base);
	// 822EA50C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 822EA510: 4BFFDD59  bl 0x822e8268
	ctx.lr = 0x822EA514;
	sub_822E8268(ctx, base);
	// 822EA514: 38210140  addi r1, r1, 0x140
	ctx.r[1].s64 = ctx.r[1].s64 + 320;
	// 822EA518: 48EBDCA4  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822EA520(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822EA520 size=88
    let mut pc: u32 = 0x822EA520;
    'dispatch: loop {
        match pc {
            0x822EA520 => {
    //   block [0x822EA520..0x822EA578)
	// 822EA520: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822EA524: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822EA528: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822EA52C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822EA530: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 822EA534: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822EA538: 48BEFAE9  bl 0x82eda020
	ctx.lr = 0x822EA53C;
	sub_82EDA020(ctx, base);
	// 822EA53C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 822EA540: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 822EA544: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 822EA548: 396BBD24  addi r11, r11, -0x42dc
	ctx.r[11].s64 = ctx.r[11].s64 + -17116;
	// 822EA54C: 394ABD18  addi r10, r10, -0x42e8
	ctx.r[10].s64 = ctx.r[10].s64 + -17128;
	// 822EA550: 3929BD08  addi r9, r9, -0x42f8
	ctx.r[9].s64 = ctx.r[9].s64 + -17144;
	// 822EA554: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 822EA558: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 822EA55C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822EA560: 913F000C  stw r9, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 822EA564: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 822EA568: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822EA56C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822EA570: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822EA574: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822EA578(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x822EA578 size=12
    let mut pc: u32 = 0x822EA578;
    'dispatch: loop {
        match pc {
            0x822EA578 => {
    //   block [0x822EA578..0x822EA584)
	// 822EA578: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 822EA57C: 806B1760  lwz r3, 0x1760(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(5984 as u32) ) } as u64;
	// 822EA580: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822EA588(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822EA588 size=112
    let mut pc: u32 = 0x822EA588;
    'dispatch: loop {
        match pc {
            0x822EA588 => {
    //   block [0x822EA588..0x822EA5F8)
	// 822EA588: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822EA58C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822EA590: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 822EA594: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822EA598: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822EA59C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 822EA5A0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 822EA5A4: 897F0036  lbz r11, 0x36(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(54 as u32) ) } as u64;
	// 822EA5A8: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 822EA5AC: 40820034  bne 0x822ea5e0
	if !ctx.cr[0].eq {
	pc = 0x822EA5E0; continue 'dispatch;
	}
	// 822EA5B0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822EA5B4: 481A4B3D  bl 0x8248f0f0
	ctx.lr = 0x822EA5B8;
	sub_8248F0F0(ctx, base);
	// 822EA5B8: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 822EA5BC: 41820024  beq 0x822ea5e0
	if ctx.cr[0].eq {
	pc = 0x822EA5E0; continue 'dispatch;
	}
	// 822EA5C0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 822EA5C4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 822EA5C8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822EA5CC: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 822EA5D0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 822EA5D4: 4E800421  bctrl
	ctx.lr = 0x822EA5D8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 822EA5D8: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 822EA5DC: 997F0036  stb r11, 0x36(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(54 as u32), ctx.r[11].u8 ) };
	// 822EA5E0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 822EA5E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822EA5E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822EA5EC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 822EA5F0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822EA5F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822EA5F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822EA5F8 size=84
    let mut pc: u32 = 0x822EA5F8;
    'dispatch: loop {
        match pc {
            0x822EA5F8 => {
    //   block [0x822EA5F8..0x822EA64C)
	// 822EA5F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822EA5FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822EA600: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822EA604: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822EA608: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 822EA60C: 897F0036  lbz r11, 0x36(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(54 as u32) ) } as u64;
	// 822EA610: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 822EA614: 41820024  beq 0x822ea638
	if ctx.cr[0].eq {
	pc = 0x822EA638; continue 'dispatch;
	}
	// 822EA618: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 822EA61C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 822EA620: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822EA624: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 822EA628: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 822EA62C: 4E800421  bctrl
	ctx.lr = 0x822EA630;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 822EA630: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 822EA634: 997F0036  stb r11, 0x36(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(54 as u32), ctx.r[11].u8 ) };
	// 822EA638: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 822EA63C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822EA640: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822EA644: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822EA648: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822EA650(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x822EA650 size=16
    let mut pc: u32 = 0x822EA650;
    'dispatch: loop {
        match pc {
            0x822EA650 => {
    //   block [0x822EA650..0x822EA660)
	// 822EA650: 81630100  lwz r11, 0x100(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(256 as u32) ) } as u64;
	// 822EA654: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 822EA658: 91630100  stw r11, 0x100(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(256 as u32), ctx.r[11].u32 ) };
	// 822EA65C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822EA660(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822EA660 size=136
    let mut pc: u32 = 0x822EA660;
    'dispatch: loop {
        match pc {
            0x822EA660 => {
    //   block [0x822EA660..0x822EA6E8)
	// 822EA660: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822EA664: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822EA668: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 822EA66C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822EA670: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822EA674: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 822EA678: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 822EA67C: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 822EA680: 409A0020  bne cr6, 0x822ea6a0
	if !ctx.cr[6].eq {
	pc = 0x822EA6A0; continue 'dispatch;
	}
	// 822EA684: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 822EA688: 419A0048  beq cr6, 0x822ea6d0
	if ctx.cr[6].eq {
	pc = 0x822EA6D0; continue 'dispatch;
	}
	// 822EA68C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 822EA690: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 822EA694: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 822EA698: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 822EA69C: 48000034  b 0x822ea6d0
	pc = 0x822EA6D0; continue 'dispatch;
	// 822EA6A0: 2F050001  cmpwi cr6, r5, 1
	ctx.cr[6].compare_i32(ctx.r[5].s32, 1, &mut ctx.xer);
	// 822EA6A4: 419A002C  beq cr6, 0x822ea6d0
	if ctx.cr[6].eq {
	pc = 0x822EA6D0; continue 'dispatch;
	}
	// 822EA6A8: 3D608325  lis r11, -0x7cdb
	ctx.r[11].s64 = -2094727168;
	// 822EA6AC: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 822EA6B0: 388B4360  addi r4, r11, 0x4360
	ctx.r[4].s64 = ctx.r[11].s64 + 17248;
	// 822EA6B4: 48EBDA45  bl 0x831a80f8
	ctx.lr = 0x822EA6B8;
	sub_831A80F8(ctx, base);
	// 822EA6B8: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 822EA6BC: 4182000C  beq 0x822ea6c8
	if ctx.cr[0].eq {
	pc = 0x822EA6C8; continue 'dispatch;
	}
	// 822EA6C0: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 822EA6C4: 4800000C  b 0x822ea6d0
	pc = 0x822EA6D0; continue 'dispatch;
	// 822EA6C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 822EA6CC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 822EA6D0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 822EA6D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822EA6D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822EA6DC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 822EA6E0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822EA6E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822EA6E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x822EA6E8 size=8
    let mut pc: u32 = 0x822EA6E8;
    'dispatch: loop {
        match pc {
            0x822EA6E8 => {
    //   block [0x822EA6E8..0x822EA6F0)
	// 822EA6E8: 3863FFF8  addi r3, r3, -8
	ctx.r[3].s64 = ctx.r[3].s64 + -8;
	// 822EA6EC: 480001B4  b 0x822ea8a0
	sub_822EA8A0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822EA6F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x822EA6F0 size=8
    let mut pc: u32 = 0x822EA6F0;
    'dispatch: loop {
        match pc {
            0x822EA6F0 => {
    //   block [0x822EA6F0..0x822EA6F8)
	// 822EA6F0: 3863FFF4  addi r3, r3, -0xc
	ctx.r[3].s64 = ctx.r[3].s64 + -12;
	// 822EA6F4: 480001AC  b 0x822ea8a0
	sub_822EA8A0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822EA6F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822EA6F8 size=68
    let mut pc: u32 = 0x822EA6F8;
    'dispatch: loop {
        match pc {
            0x822EA6F8 => {
    //   block [0x822EA6F8..0x822EA73C)
	// 822EA6F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822EA6FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822EA700: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822EA704: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822EA708: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822EA70C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 822EA710: 548A07FF  clrlwi. r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 822EA714: 396BBD68  addi r11, r11, -0x4298
	ctx.r[11].s64 = ctx.r[11].s64 + -17048;
	// 822EA718: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 822EA71C: 41820008  beq 0x822ea724
	if ctx.cr[0].eq {
	pc = 0x822EA724; continue 'dispatch;
	}
	// 822EA720: 4BFD5B49  bl 0x822c0268
	ctx.lr = 0x822EA724;
	sub_822C0268(ctx, base);
	// 822EA724: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822EA728: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 822EA72C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822EA730: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822EA734: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822EA738: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822EA740(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822EA740 size=68
    let mut pc: u32 = 0x822EA740;
    'dispatch: loop {
        match pc {
            0x822EA740 => {
    //   block [0x822EA740..0x822EA784)
	// 822EA740: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822EA744: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822EA748: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822EA74C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822EA750: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822EA754: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 822EA758: 548A07FF  clrlwi. r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 822EA75C: 396BBD74  addi r11, r11, -0x428c
	ctx.r[11].s64 = ctx.r[11].s64 + -17036;
	// 822EA760: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 822EA764: 41820008  beq 0x822ea76c
	if ctx.cr[0].eq {
	pc = 0x822EA76C; continue 'dispatch;
	}
	// 822EA768: 4BFD5B01  bl 0x822c0268
	ctx.lr = 0x822EA76C;
	sub_822C0268(ctx, base);
	// 822EA76C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822EA770: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 822EA774: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822EA778: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822EA77C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822EA780: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822EA788(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822EA788 size=68
    let mut pc: u32 = 0x822EA788;
    'dispatch: loop {
        match pc {
            0x822EA788 => {
    //   block [0x822EA788..0x822EA7CC)
	// 822EA788: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822EA78C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822EA790: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822EA794: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822EA798: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822EA79C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 822EA7A0: 548A07FF  clrlwi. r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 822EA7A4: 396BBD80  addi r11, r11, -0x4280
	ctx.r[11].s64 = ctx.r[11].s64 + -17024;
	// 822EA7A8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 822EA7AC: 41820008  beq 0x822ea7b4
	if ctx.cr[0].eq {
	pc = 0x822EA7B4; continue 'dispatch;
	}
	// 822EA7B0: 4BFD5AB9  bl 0x822c0268
	ctx.lr = 0x822EA7B4;
	sub_822C0268(ctx, base);
	// 822EA7B4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822EA7B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 822EA7BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822EA7C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822EA7C4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822EA7C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822EA7D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822EA7D0 size=68
    let mut pc: u32 = 0x822EA7D0;
    'dispatch: loop {
        match pc {
            0x822EA7D0 => {
    //   block [0x822EA7D0..0x822EA814)
	// 822EA7D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822EA7D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822EA7D8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822EA7DC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822EA7E0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822EA7E4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 822EA7E8: 548A07FF  clrlwi. r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 822EA7EC: 396BBD94  addi r11, r11, -0x426c
	ctx.r[11].s64 = ctx.r[11].s64 + -17004;
	// 822EA7F0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 822EA7F4: 41820008  beq 0x822ea7fc
	if ctx.cr[0].eq {
	pc = 0x822EA7FC; continue 'dispatch;
	}
	// 822EA7F8: 4BFD5A71  bl 0x822c0268
	ctx.lr = 0x822EA7FC;
	sub_822C0268(ctx, base);
	// 822EA7FC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822EA800: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 822EA804: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822EA808: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822EA80C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822EA810: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822EA818(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x822EA818 size=128
    let mut pc: u32 = 0x822EA818;
    'dispatch: loop {
        match pc {
            0x822EA818 => {
    //   block [0x822EA818..0x822EA898)
	// 822EA818: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 822EA81C: 39630014  addi r11, r3, 0x14
	ctx.r[11].s64 = ctx.r[3].s64 + 20;
	// 822EA820: 409A0008  bne cr6, 0x822ea828
	if !ctx.cr[6].eq {
	pc = 0x822EA828; continue 'dispatch;
	}
	// 822EA824: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 822EA828: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 822EA82C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 822EA830: 394ABD74  addi r10, r10, -0x428c
	ctx.r[10].s64 = ctx.r[10].s64 + -17036;
	// 822EA834: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 822EA838: 39630010  addi r11, r3, 0x10
	ctx.r[11].s64 = ctx.r[3].s64 + 16;
	// 822EA83C: 409A0008  bne cr6, 0x822ea844
	if !ctx.cr[6].eq {
	pc = 0x822EA844; continue 'dispatch;
	}
	// 822EA840: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 822EA844: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 822EA848: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 822EA84C: 394ABD94  addi r10, r10, -0x426c
	ctx.r[10].s64 = ctx.r[10].s64 + -17004;
	// 822EA850: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 822EA854: 3963000C  addi r11, r3, 0xc
	ctx.r[11].s64 = ctx.r[3].s64 + 12;
	// 822EA858: 409A0008  bne cr6, 0x822ea860
	if !ctx.cr[6].eq {
	pc = 0x822EA860; continue 'dispatch;
	}
	// 822EA85C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 822EA860: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 822EA864: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 822EA868: 394ABD80  addi r10, r10, -0x4280
	ctx.r[10].s64 = ctx.r[10].s64 + -17024;
	// 822EA86C: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 822EA870: 39630008  addi r11, r3, 8
	ctx.r[11].s64 = ctx.r[3].s64 + 8;
	// 822EA874: 409A0008  bne cr6, 0x822ea87c
	if !ctx.cr[6].eq {
	pc = 0x822EA87C; continue 'dispatch;
	}
	// 822EA878: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 822EA87C: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 822EA880: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 822EA884: 394ABD68  addi r10, r10, -0x4298
	ctx.r[10].s64 = ctx.r[10].s64 + -17048;
	// 822EA888: 39299EAC  addi r9, r9, -0x6154
	ctx.r[9].s64 = ctx.r[9].s64 + -24916;
	// 822EA88C: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 822EA890: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 822EA894: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822EA898(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x822EA898 size=4
    let mut pc: u32 = 0x822EA898;
    'dispatch: loop {
        match pc {
            0x822EA898 => {
    //   block [0x822EA898..0x822EA89C)
	// 822EA898: 48BEF618  b 0x82ed9eb0
	sub_82ED9EB0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822EA8A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822EA8A0 size=96
    let mut pc: u32 = 0x822EA8A0;
    'dispatch: loop {
        match pc {
            0x822EA8A0 => {
    //   block [0x822EA8A0..0x822EA900)
	// 822EA8A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822EA8A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822EA8A8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 822EA8AC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822EA8B0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822EA8B4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822EA8B8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 822EA8BC: 48BEF55D  bl 0x82ed9e18
	ctx.lr = 0x822EA8C0;
	sub_82ED9E18(ctx, base);
	// 822EA8C0: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 822EA8C4: 41820020  beq 0x822ea8e4
	if ctx.cr[0].eq {
	pc = 0x822EA8E4; continue 'dispatch;
	}
	// 822EA8C8: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 822EA8CC: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 822EA8D0: 38C0000D  li r6, 0xd
	ctx.r[6].s64 = 13;
	// 822EA8D4: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 822EA8D8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 822EA8DC: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 822EA8E0: 48BB5ED1  bl 0x82ea07b0
	ctx.lr = 0x822EA8E4;
	sub_82EA07B0(ctx, base);
	// 822EA8E4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822EA8E8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 822EA8EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822EA8F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822EA8F4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 822EA8F8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822EA8FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822EA900(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x822EA900 size=24
    let mut pc: u32 = 0x822EA900;
    'dispatch: loop {
        match pc {
            0x822EA900 => {
    //   block [0x822EA900..0x822EA918)
	// 822EA900: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 822EA904: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 822EA908: C00B08A4  lfs f0, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 822EA90C: D0030004  stfs f0, 4(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 822EA910: 99430008  stb r10, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u8 ) };
	// 822EA914: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822EA918(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822EA918 size=108
    let mut pc: u32 = 0x822EA918;
    'dispatch: loop {
        match pc {
            0x822EA918 => {
    //   block [0x822EA918..0x822EA984)
	// 822EA918: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822EA91C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822EA920: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822EA924: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822EA928: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822EA92C: 48C33095  bl 0x82f1d9c0
	ctx.lr = 0x822EA930;
	sub_82F1D9C0(ctx, base);
	// 822EA930: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 822EA934: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 822EA938: 396BBE04  addi r11, r11, -0x41fc
	ctx.r[11].s64 = ctx.r[11].s64 + -16892;
	// 822EA93C: 394ABDF8  addi r10, r10, -0x4208
	ctx.r[10].s64 = ctx.r[10].s64 + -16904;
	// 822EA940: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 822EA944: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 822EA948: 3D008201  lis r8, -0x7dff
	ctx.r[8].s64 = -2113863680;
	// 822EA94C: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 822EA950: 3CE08201  lis r7, -0x7dff
	ctx.r[7].s64 = -2113863680;
	// 822EA954: 3929BDE4  addi r9, r9, -0x421c
	ctx.r[9].s64 = ctx.r[9].s64 + -16924;
	// 822EA958: 3968BDD8  addi r11, r8, -0x4228
	ctx.r[11].s64 = ctx.r[8].s64 + -16936;
	// 822EA95C: 3947BDCC  addi r10, r7, -0x4234
	ctx.r[10].s64 = ctx.r[7].s64 + -16948;
	// 822EA960: 913F000C  stw r9, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 822EA964: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 822EA968: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822EA96C: 915F0014  stw r10, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 822EA970: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 822EA974: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822EA978: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822EA97C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822EA980: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822EA988(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x822EA988 size=8
    let mut pc: u32 = 0x822EA988;
    'dispatch: loop {
        match pc {
            0x822EA988 => {
    //   block [0x822EA988..0x822EA990)
	// 822EA988: 3863FFEC  addi r3, r3, -0x14
	ctx.r[3].s64 = ctx.r[3].s64 + -20;
	// 822EA98C: 48000304  b 0x822eac90
	sub_822EAC90(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822EA990(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x822EA990 size=8
    let mut pc: u32 = 0x822EA990;
    'dispatch: loop {
        match pc {
            0x822EA990 => {
    //   block [0x822EA990..0x822EA998)
	// 822EA990: 3863FFF8  addi r3, r3, -8
	ctx.r[3].s64 = ctx.r[3].s64 + -8;
	// 822EA994: 480002FC  b 0x822eac90
	sub_822EAC90(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822EA998(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x822EA998 size=8
    let mut pc: u32 = 0x822EA998;
    'dispatch: loop {
        match pc {
            0x822EA998 => {
    //   block [0x822EA998..0x822EA9A0)
	// 822EA998: 3863FFF4  addi r3, r3, -0xc
	ctx.r[3].s64 = ctx.r[3].s64 + -12;
	// 822EA99C: 480002F4  b 0x822eac90
	sub_822EAC90(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822EA9A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x822EA9A0 size=8
    let mut pc: u32 = 0x822EA9A0;
    'dispatch: loop {
        match pc {
            0x822EA9A0 => {
    //   block [0x822EA9A0..0x822EA9A8)
	// 822EA9A0: 3863FFF0  addi r3, r3, -0x10
	ctx.r[3].s64 = ctx.r[3].s64 + -16;
	// 822EA9A4: 480002EC  b 0x822eac90
	sub_822EAC90(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822EA9A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x822EA9A8 size=156
    let mut pc: u32 = 0x822EA9A8;
    'dispatch: loop {
        match pc {
            0x822EA9A8 => {
    //   block [0x822EA9A8..0x822EAA44)
	// 822EA9A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822EA9AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822EA9B0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822EA9B4: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822EA9B8: 39410060  addi r10, r1, 0x60
	ctx.r[10].s64 = ctx.r[1].s64 + 96;
	// 822EA9BC: 13E020C7  vcmpequd (lvx128) v31, v0, v4
	tmp.u32 = ctx.r[4].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 822EA9C0: 39210070  addi r9, r1, 0x70
	ctx.r[9].s64 = ctx.r[1].s64 + 112;
	// 822EA9C4: 13C028C7  vcmpequd (lvx128) v30, v0, v5
	tmp.u32 = ctx.r[5].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[62] using VectorMaskL[(tmp.u32 & 0xF)]
	// 822EA9C8: 3D008200  lis r8, -0x7e00
	ctx.r[8].s64 = -2113929216;
	// 822EA9CC: 3CE08201  lis r7, -0x7dff
	ctx.r[7].s64 = -2113863680;
	// 822EA9D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 822EA9D4: 38E7BDC0  addi r7, r7, -0x4240
	ctx.r[7].s64 = ctx.r[7].s64 + -16960;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822EAA48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x822EAA48 size=12
    let mut pc: u32 = 0x822EAA48;
    'dispatch: loop {
        match pc {
            0x822EAA48 => {
    //   block [0x822EAA48..0x822EAA54)
	// 822EAA48: 8164000C  lwz r11, 0xc(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(12 as u32) ) } as u64;
	// 822EAA4C: 7C8A2378  mr r10, r4
	ctx.r[10].u64 = ctx.r[4].u64;
	// 822EAA50: 4800000C  b 0x822eaa5c
	sub_822EAA54(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822EAA54(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x822EAA54 size=40
    let mut pc: u32 = 0x822EAA54;
    'dispatch: loop {
        match pc {
            0x822EAA54 => {
    //   block [0x822EAA54..0x822EAA7C)
	// 822EAA54: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 822EAA58: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 822EAA5C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 822EAA60: 409AFFF4  bne cr6, 0x822eaa54
	if !ctx.cr[6].eq {
	pc = 0x822EAA54; continue 'dispatch;
	}
	// 822EAA64: 896A0010  lbz r11, 0x10(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 822EAA68: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 822EAA6C: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 822EAA70: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 822EAA74: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 822EAA78: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822EAA7C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x822EAA7C size=8
    let mut pc: u32 = 0x822EAA7C;
    'dispatch: loop {
        match pc {
            0x822EAA7C => {
    //   block [0x822EAA7C..0x822EAA84)
	// 822EAA7C: 48C32EA4  b 0x82f1d920
	sub_82F1D920(ctx, base);
	return;
	// 822EAA80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822EAA88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x822EAA88 size=8
    let mut pc: u32 = 0x822EAA88;
    'dispatch: loop {
        match pc {
            0x822EAA88 => {
    //   block [0x822EAA88..0x822EAA90)
	// 822EAA88: 80630144  lwz r3, 0x144(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(324 as u32) ) } as u64;
	// 822EAA8C: 48B0EF4C  b 0x82df99d8
	sub_82DF99D8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822EAA90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x822EAA90 size=28
    let mut pc: u32 = 0x822EAA90;
    'dispatch: loop {
        match pc {
            0x822EAA90 => {
    //   block [0x822EAA90..0x822EAAAC)
	// 822EAA90: 396000C0  li r11, 0xc0
	ctx.r[11].s64 = 192;
	// 822EAA94: 394000D0  li r10, 0xd0
	ctx.r[10].s64 = 208;
	// 822EAA98: 13E358C7  vcmpequd (lvx128) v31, v3, v11
	tmp.u32 = ctx.r[3].u32 + ctx.r[11].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822EAAB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822EAAB0 size=104
    let mut pc: u32 = 0x822EAAB0;
    'dispatch: loop {
        match pc {
            0x822EAAB0 => {
    //   block [0x822EAAB0..0x822EAB18)
	// 822EAAB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822EAAB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822EAAB8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 822EAABC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822EAAC0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822EAAC4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 822EAAC8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 822EAACC: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 822EAAD0: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 822EAAD4: 388BBE28  addi r4, r11, -0x41d8
	ctx.r[4].s64 = ctx.r[11].s64 + -16856;
	// 822EAAD8: 38A00073  li r5, 0x73
	ctx.r[5].s64 = 115;
	// 822EAADC: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 822EAAE0: 48B07909  bl 0x82df23e8
	ctx.lr = 0x822EAAE4;
	sub_82DF23E8(ctx, base);
	// 822EAAE4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 822EAAE8: 41820014  beq 0x822eaafc
	if ctx.cr[0].eq {
	pc = 0x822EAAFC; continue 'dispatch;
	}
	// 822EAAEC: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 822EAAF0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 822EAAF4: 4BFFD8ED  bl 0x822e83e0
	ctx.lr = 0x822EAAF8;
	sub_822E83E0(ctx, base);
	// 822EAAF8: 48000008  b 0x822eab00
	pc = 0x822EAB00; continue 'dispatch;
	// 822EAAFC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 822EAB00: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 822EAB04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822EAB08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822EAB0C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 822EAB10: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822EAB14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822EAB18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822EAB18 size=152
    let mut pc: u32 = 0x822EAB18;
    'dispatch: loop {
        match pc {
            0x822EAB18 => {
    //   block [0x822EAB18..0x822EABB0)
	// 822EAB18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822EAB1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822EAB20: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 822EAB24: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822EAB28: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822EAB2C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822EAB30: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 822EAB34: 48C63C15  bl 0x82f4e748
	ctx.lr = 0x822EAB38;
	sub_82F4E748(ctx, base);
	// 822EAB38: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 822EAB3C: 41820058  beq 0x822eab94
	if ctx.cr[0].eq {
	pc = 0x822EAB94; continue 'dispatch;
	}
	// 822EAB40: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 822EAB44: 419A0050  beq cr6, 0x822eab94
	if ctx.cr[6].eq {
	pc = 0x822EAB94; continue 'dispatch;
	}
	// 822EAB48: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 822EAB4C: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 822EAB50: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 822EAB54: 8163009C  lwz r11, 0x9c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(156 as u32) ) } as u64;
	// 822EAB58: 81430034  lwz r10, 0x34(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(52 as u32) ) } as u64;
	// 822EAB5C: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 822EAB60: 41980018  blt cr6, 0x822eab78
	if ctx.cr[6].lt {
	pc = 0x822EAB78; continue 'dispatch;
	}
	// 822EAB64: 38C0000D  li r6, 0xd
	ctx.r[6].s64 = 13;
	// 822EAB68: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 822EAB6C: 3880000C  li r4, 0xc
	ctx.r[4].s64 = 12;
	// 822EAB70: 48BB54F1  bl 0x82ea0060
	ctx.lr = 0x822EAB74;
	sub_82EA0060(ctx, base);
	// 822EAB74: 48000020  b 0x822eab94
	pc = 0x822EAB94; continue 'dispatch;
	// 822EAB78: 8143009C  lwz r10, 0x9c(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(156 as u32) ) } as u64;
	// 822EAB7C: 39630098  addi r11, r3, 0x98
	ctx.r[11].s64 = ctx.r[3].s64 + 152;
	// 822EAB80: 81630098  lwz r11, 0x98(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(152 as u32) ) } as u64;
	// 822EAB84: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 822EAB88: 9143009C  stw r10, 0x9c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(156 as u32), ctx.r[10].u32 ) };
	// 822EAB8C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 822EAB90: 93E30098  stw r31, 0x98(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(152 as u32), ctx.r[31].u32 ) };
	// 822EAB94: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822EAB98: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 822EAB9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822EABA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822EABA4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 822EABA8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822EABAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822EABB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822EABB0 size=152
    let mut pc: u32 = 0x822EABB0;
    'dispatch: loop {
        match pc {
            0x822EABB0 => {
    //   block [0x822EABB0..0x822EAC48)
	// 822EABB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822EABB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822EABB8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 822EABBC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822EABC0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822EABC4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822EABC8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 822EABCC: 4BFFB9CD  bl 0x822e6598
	ctx.lr = 0x822EABD0;
	sub_822E6598(ctx, base);
	// 822EABD0: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 822EABD4: 41820058  beq 0x822eac2c
	if ctx.cr[0].eq {
	pc = 0x822EAC2C; continue 'dispatch;
	}
	// 822EABD8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 822EABDC: 419A0050  beq cr6, 0x822eac2c
	if ctx.cr[6].eq {
	pc = 0x822EAC2C; continue 'dispatch;
	}
	// 822EABE0: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 822EABE4: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 822EABE8: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 822EABEC: 8163009C  lwz r11, 0x9c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(156 as u32) ) } as u64;
	// 822EABF0: 81430034  lwz r10, 0x34(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(52 as u32) ) } as u64;
	// 822EABF4: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 822EABF8: 41980018  blt cr6, 0x822eac10
	if ctx.cr[6].lt {
	pc = 0x822EAC10; continue 'dispatch;
	}
	// 822EABFC: 38C00013  li r6, 0x13
	ctx.r[6].s64 = 19;
	// 822EAC00: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 822EAC04: 3880000C  li r4, 0xc
	ctx.r[4].s64 = 12;
	// 822EAC08: 48BB5459  bl 0x82ea0060
	ctx.lr = 0x822EAC0C;
	sub_82EA0060(ctx, base);
	// 822EAC0C: 48000020  b 0x822eac2c
	pc = 0x822EAC2C; continue 'dispatch;
	// 822EAC10: 8143009C  lwz r10, 0x9c(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(156 as u32) ) } as u64;
	// 822EAC14: 39630098  addi r11, r3, 0x98
	ctx.r[11].s64 = ctx.r[3].s64 + 152;
	// 822EAC18: 81630098  lwz r11, 0x98(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(152 as u32) ) } as u64;
	// 822EAC1C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 822EAC20: 9143009C  stw r10, 0x9c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(156 as u32), ctx.r[10].u32 ) };
	// 822EAC24: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 822EAC28: 93E30098  stw r31, 0x98(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(152 as u32), ctx.r[31].u32 ) };
	// 822EAC2C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822EAC30: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 822EAC34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822EAC38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822EAC3C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 822EAC40: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822EAC44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822EAC48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822EAC48 size=68
    let mut pc: u32 = 0x822EAC48;
    'dispatch: loop {
        match pc {
            0x822EAC48 => {
    //   block [0x822EAC48..0x822EAC8C)
	// 822EAC48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822EAC4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822EAC50: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822EAC54: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822EAC58: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822EAC5C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 822EAC60: 548A07FF  clrlwi. r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 822EAC64: 396BBDA0  addi r11, r11, -0x4260
	ctx.r[11].s64 = ctx.r[11].s64 + -16992;
	// 822EAC68: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 822EAC6C: 41820008  beq 0x822eac74
	if ctx.cr[0].eq {
	pc = 0x822EAC74; continue 'dispatch;
	}
	// 822EAC70: 4BFD55F9  bl 0x822c0268
	ctx.lr = 0x822EAC74;
	sub_822C0268(ctx, base);
	// 822EAC74: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822EAC78: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 822EAC7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822EAC80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822EAC84: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822EAC88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822EAC90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822EAC90 size=96
    let mut pc: u32 = 0x822EAC90;
    'dispatch: loop {
        match pc {
            0x822EAC90 => {
    //   block [0x822EAC90..0x822EACF0)
	// 822EAC90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822EAC94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822EAC98: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 822EAC9C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822EACA0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822EACA4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822EACA8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 822EACAC: 4BFFFB6D  bl 0x822ea818
	ctx.lr = 0x822EACB0;
	sub_822EA818(ctx, base);
	// 822EACB0: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 822EACB4: 41820020  beq 0x822eacd4
	if ctx.cr[0].eq {
	pc = 0x822EACD4; continue 'dispatch;
	}
	// 822EACB8: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 822EACBC: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 822EACC0: 38C00027  li r6, 0x27
	ctx.r[6].s64 = 39;
	// 822EACC4: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 822EACC8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 822EACCC: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 822EACD0: 48BB5AE1  bl 0x82ea07b0
	ctx.lr = 0x822EACD4;
	sub_82EA07B0(ctx, base);
	// 822EACD4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822EACD8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 822EACDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822EACE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822EACE4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 822EACE8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822EACEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822EACF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822EACF0 size=64
    let mut pc: u32 = 0x822EACF0;
    'dispatch: loop {
        match pc {
            0x822EACF0 => {
    //   block [0x822EACF0..0x822EAD30)
	// 822EACF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822EACF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822EACF8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822EACFC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822EAD00: 83E30000  lwz r31, 0(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 822EAD04: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 822EAD08: 419A0014  beq cr6, 0x822ead1c
	if ctx.cr[6].eq {
	pc = 0x822EAD1C; continue 'dispatch;
	}
	// 822EAD0C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822EAD10: 48BE3AF1  bl 0x82ece800
	ctx.lr = 0x822EAD14;
	sub_82ECE800(ctx, base);
	// 822EAD14: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822EAD18: 48BE22F1  bl 0x82ecd008
	ctx.lr = 0x822EAD1C;
	sub_82ECD008(ctx, base);
	// 822EAD1C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 822EAD20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822EAD24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822EAD28: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822EAD2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822EAD30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822EAD30 size=412
    let mut pc: u32 = 0x822EAD30;
    'dispatch: loop {
        match pc {
            0x822EAD30 => {
    //   block [0x822EAD30..0x822EAECC)
	// 822EAD30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822EAD34: 48EBD42D  bl 0x831a8160
	ctx.lr = 0x822EAD38;
	sub_831A8130(ctx, base);
	// 822EAD38: 9421FD70  stwu r1, -0x290(r1)
	ea = ctx.r[1].u32.wrapping_add(-656 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822EAD3C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 822EAD40: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 822EAD44: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 822EAD48: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 822EAD4C: 419A0178  beq cr6, 0x822eaec4
	if ctx.cr[6].eq {
	pc = 0x822EAEC4; continue 'dispatch;
	}
	// 822EAD50: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 822EAD54: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 822EAD58: 3B4BBD38  addi r26, r11, -0x42c8
	ctx.r[26].s64 = ctx.r[11].s64 + -17096;
	// 822EAD5C: 3B6ABE64  addi r27, r10, -0x419c
	ctx.r[27].s64 = ctx.r[10].s64 + -16796;
	// 822EAD60: 7F9EE378  mr r30, r28
	ctx.r[30].u64 = ctx.r[28].u64;
	// 822EAD64: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 822EAD68: 419A001C  beq cr6, 0x822ead84
	if ctx.cr[6].eq {
	pc = 0x822EAD84; continue 'dispatch;
	}
	// 822EAD6C: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 822EAD70: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 822EAD74: 3BDEFFFF  addi r30, r30, -1
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	// 822EAD78: 48BBADB9  bl 0x82ea5b30
	ctx.lr = 0x822EAD7C;
	sub_82EA5B30(ctx, base);
	// 822EAD7C: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 822EAD80: 409AFFEC  bne cr6, 0x822ead6c
	if !ctx.cr[6].eq {
	pc = 0x822EAD6C; continue 'dispatch;
	}
	// 822EAD84: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 822EAD88: 48C32DF1  bl 0x82f1db78
	ctx.lr = 0x822EAD8C;
	sub_82F1DB78(ctx, base);
	// 822EAD8C: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 822EAD90: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 822EAD94: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 822EAD98: 48BBAD99  bl 0x82ea5b30
	ctx.lr = 0x822EAD9C;
	sub_82EA5B30(ctx, base);
	// 822EAD9C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 822EADA0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822EADA4: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 822EADA8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 822EADAC: 4E800421  bctrl
	ctx.lr = 0x822EADB0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 822EADB0: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 822EADB4: 41820110  beq 0x822eaec4
	if ctx.cr[0].eq {
	pc = 0x822EAEC4; continue 'dispatch;
	}
	// 822EADB8: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 822EADBC: 2F0B001A  cmpwi cr6, r11, 0x1a
	ctx.cr[6].compare_i32(ctx.r[11].s32, 26, &mut ctx.xer);
	// 822EADC0: 409A0014  bne cr6, 0x822eadd4
	if !ctx.cr[6].eq {
	pc = 0x822EADD4; continue 'dispatch;
	}
	// 822EADC4: 38BC0001  addi r5, r28, 1
	ctx.r[5].s64 = ctx.r[28].s64 + 1;
	// 822EADC8: 809F0010  lwz r4, 0x10(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 822EADCC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 822EADD0: 4BFFFF61  bl 0x822ead30
	ctx.lr = 0x822EADD4;
	sub_822EAD30(ctx, base);
	// 822EADD4: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 822EADD8: 2F0B000B  cmpwi cr6, r11, 0xb
	ctx.cr[6].compare_i32(ctx.r[11].s32, 11, &mut ctx.xer);
	// 822EADDC: 409A0018  bne cr6, 0x822eadf4
	if !ctx.cr[6].eq {
	pc = 0x822EADF4; continue 'dispatch;
	}
	// 822EADE0: 83FF0034  lwz r31, 0x34(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 822EADE4: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 822EADE8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 822EADEC: 409AFF74  bne cr6, 0x822ead60
	if !ctx.cr[6].eq {
	pc = 0x822EAD60; continue 'dispatch;
	}
	// 822EADF0: 480000D4  b 0x822eaec4
	pc = 0x822EAEC4; continue 'dispatch;
	// 822EADF4: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 822EADF8: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 822EADFC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 822EAE00: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 822EAE04: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 822EAE08: 4E800421  bctrl
	ctx.lr = 0x822EAE0C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 822EAE0C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822EAE10: 2F1FFFFF  cmpwi cr6, r31, -1
	ctx.cr[6].compare_i32(ctx.r[31].s32, -1, &mut ctx.xer);
	// 822EAE14: 419A00B0  beq cr6, 0x822eaec4
	if ctx.cr[6].eq {
	pc = 0x822EAEC4; continue 'dispatch;
	}
	// 822EAE18: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 822EAE1C: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 822EAE20: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 822EAE24: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 822EAE28: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 822EAE2C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 822EAE30: 4E800421  bctrl
	ctx.lr = 0x822EAE34;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 822EAE34: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 822EAE38: 8164000C  lwz r11, 0xc(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(12 as u32) ) } as u64;
	// 822EAE3C: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 822EAE40: 409A000C  bne cr6, 0x822eae4c
	if !ctx.cr[6].eq {
	pc = 0x822EAE4C; continue 'dispatch;
	}
	// 822EAE44: 3B7B0001  addi r27, r27, 1
	ctx.r[27].s64 = ctx.r[27].s64 + 1;
	// 822EAE48: 48000010  b 0x822eae58
	pc = 0x822EAE58; continue 'dispatch;
	// 822EAE4C: 38BC0001  addi r5, r28, 1
	ctx.r[5].s64 = ctx.r[28].s64 + 1;
	// 822EAE50: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 822EAE54: 4BFFFEDD  bl 0x822ead30
	ctx.lr = 0x822EAE58;
	sub_822EAD30(ctx, base);
	// 822EAE58: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 822EAE5C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 822EAE60: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 822EAE64: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 822EAE68: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 822EAE6C: 4E800421  bctrl
	ctx.lr = 0x822EAE70;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 822EAE70: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822EAE74: 2F1FFFFF  cmpwi cr6, r31, -1
	ctx.cr[6].compare_i32(ctx.r[31].s32, -1, &mut ctx.xer);
	// 822EAE78: 409AFFA0  bne cr6, 0x822eae18
	if !ctx.cr[6].eq {
	pc = 0x822EAE18; continue 'dispatch;
	}
	// 822EAE7C: 2F1B0000  cmpwi cr6, r27, 0
	ctx.cr[6].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 822EAE80: 419A0044  beq cr6, 0x822eaec4
	if ctx.cr[6].eq {
	pc = 0x822EAEC4; continue 'dispatch;
	}
	// 822EAE84: 37FC0001  addic. r31, r28, 1
	ctx.xer.ca = (ctx.r[28].u32 > (!(1 as u32)));
	ctx.r[31].s64 = ctx.r[28].s64 + 1;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 822EAE88: 4182001C  beq 0x822eaea4
	if ctx.cr[0].eq {
	pc = 0x822EAEA4; continue 'dispatch;
	}
	// 822EAE8C: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 822EAE90: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 822EAE94: 3BFFFFFF  addi r31, r31, -1
	ctx.r[31].s64 = ctx.r[31].s64 + -1;
	// 822EAE98: 48BBAC99  bl 0x82ea5b30
	ctx.lr = 0x822EAE9C;
	sub_82EA5B30(ctx, base);
	// 822EAE9C: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 822EAEA0: 409AFFEC  bne cr6, 0x822eae8c
	if !ctx.cr[6].eq {
	pc = 0x822EAE8C; continue 'dispatch;
	}
	// 822EAEA4: 38600004  li r3, 4
	ctx.r[3].s64 = 4;
	// 822EAEA8: 48C32CD1  bl 0x82f1db78
	ctx.lr = 0x822EAEAC;
	sub_82F1DB78(ctx, base);
	// 822EAEAC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 822EAEB0: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 822EAEB4: 388BBE58  addi r4, r11, -0x41a8
	ctx.r[4].s64 = ctx.r[11].s64 + -16808;
	// 822EAEB8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 822EAEBC: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 822EAEC0: 48BBAC71  bl 0x82ea5b30
	ctx.lr = 0x822EAEC4;
	sub_82EA5B30(ctx, base);
	// 822EAEC4: 38210290  addi r1, r1, 0x290
	ctx.r[1].s64 = ctx.r[1].s64 + 656;
	// 822EAEC8: 48EBD2E8  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822EAED0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822EAED0 size=560
    //   switch @ 0x822EAFF8: r11 with 11 label(s)
    //       case  0 → 0x822EB008
    //       case  1 → 0x822EB010
    //       case  2 → 0x822EB018
    //       case  3 → 0x822EB020
    //       case  4 → 0x822EB028
    //       case  5 → 0x822EB030
    //       case  6 → 0x822EB038
    //       case  7 → 0x822EB040
    //       case  8 → 0x822EB048
    //       case  9 → 0x822EB050
    //       case 10 → 0x822EB058
    let mut pc: u32 = 0x822EAED0;
    'dispatch: loop {
        match pc {
            0x822EAED0 => {
    //   block [0x822EAED0..0x822EB008)
	// 822EAED0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822EAED4: 48EBD25D  bl 0x831a8130
	ctx.lr = 0x822EAED8;
	sub_831A8130(ctx, base);
	// 822EAED8: 9421FF00  stwu r1, -0x100(r1)
	ea = ctx.r[1].u32.wrapping_add(-256 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822EAEDC: 7C8E2378  mr r14, r4
	ctx.r[14].u64 = ctx.r[4].u64;
	// 822EAEE0: 7C761B78  mr r22, r3
	ctx.r[22].u64 = ctx.r[3].u64;
	// 822EAEE4: 91C1011C  stw r14, 0x11c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(284 as u32), ctx.r[14].u32 ) };
	// 822EAEE8: 2B0E0000  cmplwi cr6, r14, 0
	ctx.cr[6].compare_u32(ctx.r[14].u32, 0 as u32, &mut ctx.xer);
	// 822EAEEC: 419A020C  beq cr6, 0x822eb0f8
	if ctx.cr[6].eq {
	pc = 0x822EB0F8; continue 'dispatch;
	}
	// 822EAEF0: 814E004C  lwz r10, 0x4c(r14)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[14].u32.wrapping_add(76 as u32) ) } as u64;
	// 822EAEF4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 822EAEF8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 822EAEFC: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 822EAF00: 409901F8  ble cr6, 0x822eb0f8
	if !ctx.cr[6].gt {
	pc = 0x822EB0F8; continue 'dispatch;
	}
	// 822EAF04: 3CE08201  lis r7, -0x7dff
	ctx.r[7].s64 = -2113863680;
	// 822EAF08: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 822EAF0C: 3D008201  lis r8, -0x7dff
	ctx.r[8].s64 = -2113863680;
	// 822EAF10: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 822EAF14: 3FC08201  lis r30, -0x7dff
	ctx.r[30].s64 = -2113863680;
	// 822EAF18: 3FE08201  lis r31, -0x7dff
	ctx.r[31].s64 = -2113863680;
	// 822EAF1C: 3EE08201  lis r23, -0x7dff
	ctx.r[23].s64 = -2113863680;
	// 822EAF20: 3F008201  lis r24, -0x7dff
	ctx.r[24].s64 = -2113863680;
	// 822EAF24: 3F208201  lis r25, -0x7dff
	ctx.r[25].s64 = -2113863680;
	// 822EAF28: 3F408201  lis r26, -0x7dff
	ctx.r[26].s64 = -2113863680;
	// 822EAF2C: 3F608201  lis r27, -0x7dff
	ctx.r[27].s64 = -2113863680;
	// 822EAF30: 3F808201  lis r28, -0x7dff
	ctx.r[28].s64 = -2113863680;
	// 822EAF34: 3FA08201  lis r29, -0x7dff
	ctx.r[29].s64 = -2113863680;
	// 822EAF38: 3C608201  lis r3, -0x7dff
	ctx.r[3].s64 = -2113863680;
	// 822EAF3C: 3C808201  lis r4, -0x7dff
	ctx.r[4].s64 = -2113863680;
	// 822EAF40: 3CA08201  lis r5, -0x7dff
	ctx.r[5].s64 = -2113863680;
	// 822EAF44: 3CC08201  lis r6, -0x7dff
	ctx.r[6].s64 = -2113863680;
	// 822EAF48: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 822EAF4C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 822EAF50: 38E7C074  addi r7, r7, -0x3f8c
	ctx.r[7].s64 = ctx.r[7].s64 + -16268;
	// 822EAF54: 3908C058  addi r8, r8, -0x3fa8
	ctx.r[8].s64 = ctx.r[8].s64 + -16296;
	// 822EAF58: 394AC03C  addi r10, r10, -0x3fc4
	ctx.r[10].s64 = ctx.r[10].s64 + -16324;
	// 822EAF5C: 90E10060  stw r7, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[7].u32 ) };
	// 822EAF60: 3A9EC018  addi r20, r30, -0x3fe8
	ctx.r[20].s64 = ctx.r[30].s64 + -16360;
	// 822EAF64: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 822EAF68: 3A7FC000  addi r19, r31, -0x4000
	ctx.r[19].s64 = ctx.r[31].s64 + -16384;
	// 822EAF6C: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 822EAF70: 3AF7BFE0  addi r23, r23, -0x4020
	ctx.r[23].s64 = ctx.r[23].s64 + -16416;
	// 822EAF74: 3B18BFC4  addi r24, r24, -0x403c
	ctx.r[24].s64 = ctx.r[24].s64 + -16444;
	// 822EAF78: 3B39BFA4  addi r25, r25, -0x405c
	ctx.r[25].s64 = ctx.r[25].s64 + -16476;
	// 822EAF7C: 3B5ABF7C  addi r26, r26, -0x4084
	ctx.r[26].s64 = ctx.r[26].s64 + -16516;
	// 822EAF80: 3B7BBF5C  addi r27, r27, -0x40a4
	ctx.r[27].s64 = ctx.r[27].s64 + -16548;
	// 822EAF84: 3B9CBF40  addi r28, r28, -0x40c0
	ctx.r[28].s64 = ctx.r[28].s64 + -16576;
	// 822EAF88: 3ABDBF24  addi r21, r29, -0x40dc
	ctx.r[21].s64 = ctx.r[29].s64 + -16604;
	// 822EAF8C: 3A43BF08  addi r18, r3, -0x40f8
	ctx.r[18].s64 = ctx.r[3].s64 + -16632;
	// 822EAF90: 3A24BEDC  addi r17, r4, -0x4124
	ctx.r[17].s64 = ctx.r[4].s64 + -16676;
	// 822EAF94: 3A05BEBC  addi r16, r5, -0x4144
	ctx.r[16].s64 = ctx.r[5].s64 + -16708;
	// 822EAF98: 39E6BE90  addi r15, r6, -0x4170
	ctx.r[15].s64 = ctx.r[6].s64 + -16752;
	// 822EAF9C: 3BE9BE88  addi r31, r9, -0x4178
	ctx.r[31].s64 = ctx.r[9].s64 + -16760;
	// 822EAFA0: 3BCBBE68  addi r30, r11, -0x4198
	ctx.r[30].s64 = ctx.r[11].s64 + -16792;
	// 822EAFA4: 816E0048  lwz r11, 0x48(r14)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[14].u32.wrapping_add(72 as u32) ) } as u64;
	// 822EAFA8: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 822EAFAC: 7FAA582E  lwzx r29, r10, r11
	ctx.r[29].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 822EAFB0: 387D00D0  addi r3, r29, 0xd0
	ctx.r[3].s64 = ctx.r[29].s64 + 208;
	// 822EAFB4: 81DD0070  lwz r14, 0x70(r29)
	ctx.r[14].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(112 as u32) ) } as u64;
	// 822EAFB8: 48BEF5C1  bl 0x82eda578
	ctx.lr = 0x822EAFBC;
	sub_82EDA578(ctx, base);
	// 822EAFBC: D8210020  stfd f1, 0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(32 as u32), ctx.f[1].u64 ) };
	// 822EAFC0: E8A10020  ld r5, 0x20(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(32 as u32) ) };
	// 822EAFC4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 822EAFC8: 7EC3B378  mr r3, r22
	ctx.r[3].u64 = ctx.r[22].u64;
	// 822EAFCC: 7DC67378  mr r6, r14
	ctx.r[6].u64 = ctx.r[14].u64;
	// 822EAFD0: 48BBAB61  bl 0x82ea5b30
	ctx.lr = 0x822EAFD4;
	sub_82EA5B30(ctx, base);
	// 822EAFD4: 897D00D8  lbz r11, 0xd8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(216 as u32) ) } as u64;
	// 822EAFD8: 2B0B000A  cmplwi cr6, r11, 0xa
	ctx.cr[6].compare_u32(ctx.r[11].u32, 10 as u32, &mut ctx.xer);
	// 822EAFDC: 4199008C  bgt cr6, 0x822eb068
	if ctx.cr[6].gt {
	pc = 0x822EB068; continue 'dispatch;
	}
	// 822EAFE0: 3D808201  lis r12, -0x7dff
	ctx.r[12].s64 = -2113863680;
	// 822EAFE4: 398CBCF8  addi r12, r12, -0x4308
	ctx.r[12].s64 = ctx.r[12].s64 + -17160;
	// 822EAFE8: 7C0C58AE  lbzx r0, r12, r11
	ctx.r[0].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[12].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 822EAFEC: 3D80822F  lis r12, -0x7dd1
	ctx.r[12].s64 = -2110849024;
	// 822EAFF0: 398CB008  addi r12, r12, -0x4ff8
	ctx.r[12].s64 = ctx.r[12].s64 + -20472;
	// 822EAFF4: 7D8C0214  add r12, r12, r0
	ctx.r[12].u64 = ctx.r[12].u64 + ctx.r[0].u64;
	// 822EAFF8: 7D8903A6  mtctr r12
	ctx.ctr.u64 = ctx.r[12].u64;
	// 822EAFFC: 60000000  nop
	// 822EB000: 60000000  nop
	// 822EB004: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            0x822EB008 => {
    //   block [0x822EB008..0x822EB010)
	// 822EB008: 80A10058  lwz r5, 0x58(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 822EB00C: 48000050  b 0x822eb05c
	pc = 0x822EB05C; continue 'dispatch;
            }
            0x822EB010 => {
    //   block [0x822EB010..0x822EB018)
	// 822EB010: 80A1005C  lwz r5, 0x5c(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 822EB014: 48000048  b 0x822eb05c
	pc = 0x822EB05C; continue 'dispatch;
            }
            0x822EB018 => {
    //   block [0x822EB018..0x822EB020)
	// 822EB018: 80A10060  lwz r5, 0x60(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 822EB01C: 48000040  b 0x822eb05c
	pc = 0x822EB05C; continue 'dispatch;
            }
            0x822EB020 => {
    //   block [0x822EB020..0x822EB028)
	// 822EB020: 7DE57B78  mr r5, r15
	ctx.r[5].u64 = ctx.r[15].u64;
	// 822EB024: 48000038  b 0x822eb05c
	pc = 0x822EB05C; continue 'dispatch;
            }
            0x822EB028 => {
    //   block [0x822EB028..0x822EB030)
	// 822EB028: 7E058378  mr r5, r16
	ctx.r[5].u64 = ctx.r[16].u64;
	// 822EB02C: 48000030  b 0x822eb05c
	pc = 0x822EB05C; continue 'dispatch;
            }
            0x822EB030 => {
    //   block [0x822EB030..0x822EB038)
	// 822EB030: 7E258B78  mr r5, r17
	ctx.r[5].u64 = ctx.r[17].u64;
	// 822EB034: 48000028  b 0x822eb05c
	pc = 0x822EB05C; continue 'dispatch;
            }
            0x822EB038 => {
    //   block [0x822EB038..0x822EB040)
	// 822EB038: 7E459378  mr r5, r18
	ctx.r[5].u64 = ctx.r[18].u64;
	// 822EB03C: 48000020  b 0x822eb05c
	pc = 0x822EB05C; continue 'dispatch;
            }
            0x822EB040 => {
    //   block [0x822EB040..0x822EB048)
	// 822EB040: 7E659B78  mr r5, r19
	ctx.r[5].u64 = ctx.r[19].u64;
	// 822EB044: 48000018  b 0x822eb05c
	pc = 0x822EB05C; continue 'dispatch;
            }
            0x822EB048 => {
    //   block [0x822EB048..0x822EB050)
	// 822EB048: 7E85A378  mr r5, r20
	ctx.r[5].u64 = ctx.r[20].u64;
	// 822EB04C: 48000010  b 0x822eb05c
	pc = 0x822EB05C; continue 'dispatch;
            }
            0x822EB050 => {
    //   block [0x822EB050..0x822EB058)
	// 822EB050: 7EA5AB78  mr r5, r21
	ctx.r[5].u64 = ctx.r[21].u64;
	// 822EB054: 48000008  b 0x822eb05c
	pc = 0x822EB05C; continue 'dispatch;
            }
            0x822EB058 => {
    //   block [0x822EB058..0x822EB100)
	// 822EB058: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 822EB05C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 822EB060: 7EC3B378  mr r3, r22
	ctx.r[3].u64 = ctx.r[22].u64;
	// 822EB064: 48BBAACD  bl 0x82ea5b30
	ctx.lr = 0x822EB068;
	sub_82EA5B30(ctx, base);
	// 822EB068: 897D0080  lbz r11, 0x80(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(128 as u32) ) } as u64;
	// 822EB06C: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 822EB070: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 822EB074: 4198003C  blt cr6, 0x822eb0b0
	if ctx.cr[6].lt {
	pc = 0x822EB0B0; continue 'dispatch;
	}
	// 822EB078: 419A0030  beq cr6, 0x822eb0a8
	if ctx.cr[6].eq {
	pc = 0x822EB0A8; continue 'dispatch;
	}
	// 822EB07C: 2B0B0003  cmplwi cr6, r11, 3
	ctx.cr[6].compare_u32(ctx.r[11].u32, 3 as u32, &mut ctx.xer);
	// 822EB080: 41980020  blt cr6, 0x822eb0a0
	if ctx.cr[6].lt {
	pc = 0x822EB0A0; continue 'dispatch;
	}
	// 822EB084: 419A0014  beq cr6, 0x822eb098
	if ctx.cr[6].eq {
	pc = 0x822EB098; continue 'dispatch;
	}
	// 822EB088: 2B0B0005  cmplwi cr6, r11, 5
	ctx.cr[6].compare_u32(ctx.r[11].u32, 5 as u32, &mut ctx.xer);
	// 822EB08C: 40980034  bge cr6, 0x822eb0c0
	if !ctx.cr[6].lt {
	pc = 0x822EB0C0; continue 'dispatch;
	}
	// 822EB090: 7EE5BB78  mr r5, r23
	ctx.r[5].u64 = ctx.r[23].u64;
	// 822EB094: 48000020  b 0x822eb0b4
	pc = 0x822EB0B4; continue 'dispatch;
	// 822EB098: 7F05C378  mr r5, r24
	ctx.r[5].u64 = ctx.r[24].u64;
	// 822EB09C: 48000018  b 0x822eb0b4
	pc = 0x822EB0B4; continue 'dispatch;
	// 822EB0A0: 7F25CB78  mr r5, r25
	ctx.r[5].u64 = ctx.r[25].u64;
	// 822EB0A4: 48000010  b 0x822eb0b4
	pc = 0x822EB0B4; continue 'dispatch;
	// 822EB0A8: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 822EB0AC: 48000008  b 0x822eb0b4
	pc = 0x822EB0B4; continue 'dispatch;
	// 822EB0B0: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 822EB0B4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 822EB0B8: 7EC3B378  mr r3, r22
	ctx.r[3].u64 = ctx.r[22].u64;
	// 822EB0BC: 48BBAA75  bl 0x82ea5b30
	ctx.lr = 0x822EB0C0;
	sub_82EA5B30(ctx, base);
	// 822EB0C0: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 822EB0C4: 809D0010  lwz r4, 0x10(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 822EB0C8: 7EC3B378  mr r3, r22
	ctx.r[3].u64 = ctx.r[22].u64;
	// 822EB0CC: 4BFFFC65  bl 0x822ead30
	ctx.lr = 0x822EB0D0;
	sub_822EAD30(ctx, base);
	// 822EB0D0: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 822EB0D4: 81C1011C  lwz r14, 0x11c(r1)
	ctx.r[14].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(284 as u32) ) } as u64;
	// 822EB0D8: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 822EB0DC: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 822EB0E0: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 822EB0E4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 822EB0E8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 822EB0EC: 814E004C  lwz r10, 0x4c(r14)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[14].u32.wrapping_add(76 as u32) ) } as u64;
	// 822EB0F0: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 822EB0F4: 4198FEB0  blt cr6, 0x822eafa4
	if ctx.cr[6].lt {
	pc = 0x822EAFA4; continue 'dispatch;
	}
	// 822EB0F8: 38210100  addi r1, r1, 0x100
	ctx.r[1].s64 = ctx.r[1].s64 + 256;
	// 822EB0FC: 48EBD084  b 0x831a8180
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822EB100(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822EB100 size=172
    let mut pc: u32 = 0x822EB100;
    'dispatch: loop {
        match pc {
            0x822EB100 => {
    //   block [0x822EB100..0x822EB1AC)
	// 822EB100: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822EB104: 48EBD069  bl 0x831a816c
	ctx.lr = 0x822EB108;
	sub_831A8130(ctx, base);
	// 822EB108: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822EB10C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 822EB110: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822EB114: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 822EB118: 388BC098  addi r4, r11, -0x3f68
	ctx.r[4].s64 = ctx.r[11].s64 + -16232;
	// 822EB11C: 48BBABAD  bl 0x82ea5cc8
	ctx.lr = 0x822EB120;
	sub_82EA5CC8(ctx, base);
	// 822EB120: 817F002C  lwz r11, 0x2c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 822EB124: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 822EB128: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 822EB12C: 4099002C  ble cr6, 0x822eb158
	if !ctx.cr[6].gt {
	pc = 0x822EB158; continue 'dispatch;
	}
	// 822EB130: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 822EB134: 817F0028  lwz r11, 0x28(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 822EB138: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 822EB13C: 7C9E582E  lwzx r4, r30, r11
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 822EB140: 4BFFFD91  bl 0x822eaed0
	ctx.lr = 0x822EB144;
	sub_822EAED0(ctx, base);
	// 822EB144: 817F002C  lwz r11, 0x2c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 822EB148: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 822EB14C: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 822EB150: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 822EB154: 4198FFE0  blt cr6, 0x822eb134
	if ctx.cr[6].lt {
	pc = 0x822EB134; continue 'dispatch;
	}
	// 822EB158: 817F0038  lwz r11, 0x38(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 822EB15C: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 822EB160: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 822EB164: 4099002C  ble cr6, 0x822eb190
	if !ctx.cr[6].gt {
	pc = 0x822EB190; continue 'dispatch;
	}
	// 822EB168: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 822EB16C: 817F0034  lwz r11, 0x34(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 822EB170: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 822EB174: 7C8BF02E  lwzx r4, r11, r30
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 822EB178: 4BFFFD59  bl 0x822eaed0
	ctx.lr = 0x822EB17C;
	sub_822EAED0(ctx, base);
	// 822EB17C: 817F0038  lwz r11, 0x38(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 822EB180: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 822EB184: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 822EB188: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 822EB18C: 4198FFE0  blt cr6, 0x822eb16c
	if ctx.cr[6].lt {
	pc = 0x822EB16C; continue 'dispatch;
	}
	// 822EB190: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 822EB194: 809F0020  lwz r4, 0x20(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 822EB198: 4BFFFD39  bl 0x822eaed0
	ctx.lr = 0x822EB19C;
	sub_822EAED0(ctx, base);
	// 822EB19C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 822EB1A0: 48BBAC79  bl 0x82ea5e18
	ctx.lr = 0x822EB1A4;
	sub_82EA5E18(ctx, base);
	// 822EB1A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 822EB1A8: 48EBD014  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822EB1B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x822EB1B0 size=380
    let mut pc: u32 = 0x822EB1B0;
    'dispatch: loop {
        match pc {
            0x822EB1B0 => {
    //   block [0x822EB1B0..0x822EB32C)
	// 822EB1B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822EB1B4: 48EBCFB9  bl 0x831a816c
	ctx.lr = 0x822EB1B8;
	sub_831A8130(ctx, base);
	// 822EB1B8: 9421FEE0  stwu r1, -0x120(r1)
	ea = ctx.r[1].u32.wrapping_add(-288 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822EB1BC: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 822EB1C0: 13E028C7  vcmpequd (lvx128) v31, v0, v5
	tmp.u32 = ctx.r[5].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 822EB1C4: 39410070  addi r10, r1, 0x70
	ctx.r[10].s64 = ctx.r[1].s64 + 112;
	// 822EB1C8: 13C030C7  vcmpequd (lvx128) v30, v0, v6
	tmp.u32 = ctx.r[6].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[62] using VectorMaskL[(tmp.u32 & 0xF)]
	// 822EB1CC: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 822EB1D0: 3D008201  lis r8, -0x7dff
	ctx.r[8].s64 = -2113863680;
	// 822EB1D4: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822EB330(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822EB330 size=104
    let mut pc: u32 = 0x822EB330;
    'dispatch: loop {
        match pc {
            0x822EB330 => {
    //   block [0x822EB330..0x822EB398)
	// 822EB330: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822EB334: 48EBCE39  bl 0x831a816c
	ctx.lr = 0x822EB338;
	sub_831A8130(ctx, base);
	// 822EB338: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822EB33C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 822EB340: 83A50000  lwz r29, 0(r5)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 822EB344: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 822EB348: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 822EB34C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 822EB350: 7F045840  cmplw cr6, r4, r11
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[11].u32, &mut ctx.xer);
	// 822EB354: 419A0034  beq cr6, 0x822eb388
	if ctx.cr[6].eq {
	pc = 0x822EB388; continue 'dispatch;
	}
	// 822EB358: 81640004  lwz r11, 4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 822EB35C: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 822EB360: 81240000  lwz r9, 0(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 822EB364: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 822EB368: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 822EB36C: 81240004  lwz r9, 4(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 822EB370: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 822EB374: 806A110C  lwz r3, 0x110c(r10)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4364 as u32) ) } as u64;
	// 822EB378: 48B06E11  bl 0x82df2188
	ctx.lr = 0x822EB37C;
	sub_82DF2188(ctx, base);
	// 822EB37C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 822EB380: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 822EB384: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 822EB388: 93BE0000  stw r29, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 822EB38C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 822EB390: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 822EB394: 48EBCE28  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822EB398(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822EB398 size=84
    let mut pc: u32 = 0x822EB398;
    'dispatch: loop {
        match pc {
            0x822EB398 => {
    //   block [0x822EB398..0x822EB3EC)
	// 822EB398: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822EB39C: 48EBCDD1  bl 0x831a816c
	ctx.lr = 0x822EB3A0;
	sub_831A8130(ctx, base);
	// 822EB3A0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822EB3A4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 822EB3A8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 822EB3AC: 897F0019  lbz r11, 0x19(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(25 as u32) ) } as u64;
	// 822EB3B0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 822EB3B4: 409A0030  bne cr6, 0x822eb3e4
	if !ctx.cr[6].eq {
	pc = 0x822EB3E4; continue 'dispatch;
	}
	// 822EB3B8: 3FA08335  lis r29, -0x7ccb
	ctx.r[29].s64 = -2093678592;
	// 822EB3BC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 822EB3C0: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 822EB3C4: 4BFFFFD5  bl 0x822eb398
	ctx.lr = 0x822EB3C8;
	sub_822EB398(ctx, base);
	// 822EB3C8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 822EB3CC: 807D110C  lwz r3, 0x110c(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4364 as u32) ) } as u64;
	// 822EB3D0: 83FF0000  lwz r31, 0(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 822EB3D4: 48B06DB5  bl 0x82df2188
	ctx.lr = 0x822EB3D8;
	sub_82DF2188(ctx, base);
	// 822EB3D8: 897F0019  lbz r11, 0x19(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(25 as u32) ) } as u64;
	// 822EB3DC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 822EB3E0: 419AFFDC  beq cr6, 0x822eb3bc
	if ctx.cr[6].eq {
	pc = 0x822EB3BC; continue 'dispatch;
	}
	// 822EB3E4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 822EB3E8: 48EBCDD4  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822EB3F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822EB3F0 size=188
    let mut pc: u32 = 0x822EB3F0;
    'dispatch: loop {
        match pc {
            0x822EB3F0 => {
    //   block [0x822EB3F0..0x822EB4AC)
	// 822EB3F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822EB3F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822EB3F8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 822EB3FC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822EB400: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822EB404: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 822EB408: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 822EB40C: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 822EB410: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 822EB414: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 822EB418: 4BFD5521  bl 0x822c0938
	ctx.lr = 0x822EB41C;
	sub_822C0938(ctx, base);
	// 822EB41C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 822EB420: 41820028  beq 0x822eb448
	if ctx.cr[0].eq {
	pc = 0x822EB448; continue 'dispatch;
	}
	// 822EB424: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 822EB428: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 822EB42C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 822EB430: 392BBD54  addi r9, r11, -0x42ac
	ctx.r[9].s64 = ctx.r[11].s64 + -17068;
	// 822EB434: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 822EB438: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 822EB43C: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 822EB440: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 822EB444: 48000008  b 0x822eb44c
	pc = 0x822EB44C; continue 'dispatch;
	// 822EB448: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 822EB44C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 822EB450: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 822EB454: 409A003C  bne cr6, 0x822eb490
	if !ctx.cr[6].eq {
	pc = 0x822EB490; continue 'dispatch;
	}
	// 822EB458: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 822EB45C: 419A0014  beq cr6, 0x822eb470
	if ctx.cr[6].eq {
	pc = 0x822EB470; continue 'dispatch;
	}
	// 822EB460: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822EB464: 48B0E68D  bl 0x82df9af0
	ctx.lr = 0x822EB468;
	sub_82DF9AF0(ctx, base);
	// 822EB468: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822EB46C: 48B06F6D  bl 0x82df23d8
	ctx.lr = 0x822EB470;
	sub_82DF23D8(ctx, base);
	// 822EB470: 3D608325  lis r11, -0x7cdb
	ctx.r[11].s64 = -2094727168;
	// 822EB474: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 822EB478: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 822EB47C: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 822EB480: 816B4288  lwz r11, 0x4288(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(17032 as u32) ) } as u64;
	// 822EB484: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 822EB488: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 822EB48C: 4BFD4B75  bl 0x822c0000
	ctx.lr = 0x822EB490;
	sub_822C0000(ctx, base);
	// 822EB490: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 822EB494: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 822EB498: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822EB49C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822EB4A0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 822EB4A4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822EB4A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822EB4B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822EB4B0 size=64
    let mut pc: u32 = 0x822EB4B0;
    'dispatch: loop {
        match pc {
            0x822EB4B0 => {
    //   block [0x822EB4B0..0x822EB4F0)
	// 822EB4B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822EB4B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822EB4B8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822EB4BC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822EB4C0: 83E3000C  lwz r31, 0xc(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 822EB4C4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 822EB4C8: 419A0014  beq cr6, 0x822eb4dc
	if ctx.cr[6].eq {
	pc = 0x822EB4DC; continue 'dispatch;
	}
	// 822EB4CC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822EB4D0: 48B0E621  bl 0x82df9af0
	ctx.lr = 0x822EB4D4;
	sub_82DF9AF0(ctx, base);
	// 822EB4D4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822EB4D8: 48B06F01  bl 0x82df23d8
	ctx.lr = 0x822EB4DC;
	sub_82DF23D8(ctx, base);
	// 822EB4DC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 822EB4E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822EB4E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822EB4E8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822EB4EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822EB4F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822EB4F0 size=72
    let mut pc: u32 = 0x822EB4F0;
    'dispatch: loop {
        match pc {
            0x822EB4F0 => {
    //   block [0x822EB4F0..0x822EB538)
	// 822EB4F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822EB4F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822EB4F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822EB4FC: 2F050003  cmpwi cr6, r5, 3
	ctx.cr[6].compare_i32(ctx.r[5].s32, 3, &mut ctx.xer);
	// 822EB500: 419A001C  beq cr6, 0x822eb51c
	if ctx.cr[6].eq {
	pc = 0x822EB51C; continue 'dispatch;
	}
	// 822EB504: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 822EB508: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 822EB50C: 994B0000  stb r10, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 822EB510: 88C10050  lbz r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 822EB514: 4BFFF14D  bl 0x822ea660
	ctx.lr = 0x822EB518;
	sub_822EA660(ctx, base);
	// 822EB518: 48000010  b 0x822eb528
	pc = 0x822EB528; continue 'dispatch;
	// 822EB51C: 3D608325  lis r11, -0x7cdb
	ctx.r[11].s64 = -2094727168;
	// 822EB520: 396B4360  addi r11, r11, 0x4360
	ctx.r[11].s64 = ctx.r[11].s64 + 17248;
	// 822EB524: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 822EB528: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 822EB52C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822EB530: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822EB534: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822EB538(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x822EB538 size=116
    let mut pc: u32 = 0x822EB538;
    'dispatch: loop {
        match pc {
            0x822EB538 => {
    //   block [0x822EB538..0x822EB5AC)
	// 822EB538: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822EB53C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822EB540: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822EB544: DBE1FFE8  stfd f31, -0x18(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.f[31].u64 ) };
	// 822EB548: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822EB54C: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 822EB550: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 822EB554: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822EB558: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 822EB55C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 822EB560: 808B1790  lwz r4, 0x1790(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(6032 as u32) ) } as u64;
	// 822EB564: 48B06695  bl 0x82df1bf8
	ctx.lr = 0x822EB568;
	sub_82DF1BF8(ctx, base);
	// 822EB568: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 822EB56C: 4BFD4A95  bl 0x822c0000
	ctx.lr = 0x822EB570;
	sub_822C0000(ctx, base);
	// 822EB570: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 822EB574: 48B0671D  bl 0x82df1c90
	ctx.lr = 0x822EB578;
	sub_82DF1C90(ctx, base);
	// 822EB578: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 822EB57C: D3FF014C  stfs f31, 0x14c(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(332 as u32), tmp.u32 ) };
	// 822EB580: 997F00E1  stb r11, 0xe1(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(225 as u32), ctx.r[11].u8 ) };
	// 822EB584: 997F00E2  stb r11, 0xe2(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(226 as u32), ctx.r[11].u8 ) };
	// 822EB588: 48B06641  bl 0x82df1bc8
	ctx.lr = 0x822EB58C;
	sub_82DF1BC8(ctx, base);
	// 822EB58C: 807F0144  lwz r3, 0x144(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(324 as u32) ) } as u64;
	// 822EB590: 48B0E3E1  bl 0x82df9970
	ctx.lr = 0x822EB594;
	sub_82DF9970(ctx, base);
	// 822EB594: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 822EB598: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822EB59C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822EB5A0: CBE1FFE8  lfd f31, -0x18(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 822EB5A4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822EB5A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822EB5B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x822EB5B0 size=180
    let mut pc: u32 = 0x822EB5B0;
    'dispatch: loop {
        match pc {
            0x822EB5B0 => {
    //   block [0x822EB5B0..0x822EB664)
	// 822EB5B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822EB5B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822EB5B8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 822EB5BC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822EB5C0: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822EB5C4: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 822EB5C8: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 822EB5CC: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 822EB5D0: 7CE63B78  mr r6, r7
	ctx.r[6].u64 = ctx.r[7].u64;
	// 822EB5D4: 7D074378  mr r7, r8
	ctx.r[7].u64 = ctx.r[8].u64;
	// 822EB5D8: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 822EB5DC: 4BFFFBD5  bl 0x822eb1b0
	ctx.lr = 0x822EB5E0;
	sub_822EB1B0(ctx, base);
	// 822EB5E0: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 822EB5E4: 41820064  beq 0x822eb648
	if ctx.cr[0].eq {
	pc = 0x822EB648; continue 'dispatch;
	}
	// 822EB5E8: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 822EB5EC: C0010060  lfs f0, 0x60(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 822EB5F0: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 822EB5F4: C1A10064  lfs f13, 0x64(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 822EB5F8: 39210050  addi r9, r1, 0x50
	ctx.r[9].s64 = ctx.r[1].s64 + 80;
	// 822EB5FC: D0010050  stfs f0, 0x50(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 822EB600: C1810068  lfs f12, 0x68(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 822EB604: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 822EB608: D1A10054  stfs f13, 0x54(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 822EB60C: C00B08A4  lfs f0, 0x8a4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 822EB610: D1810058  stfs f12, 0x58(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), tmp.u32 ) };
	// 822EB614: D001005C  stfs f0, 0x5c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), tmp.u32 ) };
	// 822EB618: 13E050C7  vcmpequd (lvx128) v31, v0, v10
	tmp.u32 = ctx.r[10].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 822EB61C: C1A1006C  lfs f13, 0x6c(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(108 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 822EB620: C1810070  lfs f12, 0x70(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 822EB624: C1610074  lfs f11, 0x74(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 822EB628: D1A10050  stfs f13, 0x50(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 822EB62C: D1810054  stfs f12, 0x54(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 822EB630: D1610058  stfs f11, 0x58(r1)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), tmp.u32 ) };
	// 822EB634: D001005C  stfs f0, 0x5c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), tmp.u32 ) };
	// 822EB638: 13C048C7  vcmpequd (lvx128) v30, v0, v9
	tmp.u32 = ctx.r[9].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[62] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822EB668(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822EB668 size=84
    let mut pc: u32 = 0x822EB668;
    'dispatch: loop {
        match pc {
            0x822EB668 => {
    //   block [0x822EB668..0x822EB6BC)
	// 822EB668: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822EB66C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822EB670: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822EB674: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822EB678: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822EB67C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 822EB680: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 822EB684: 4BFFFD15  bl 0x822eb398
	ctx.lr = 0x822EB688;
	sub_822EB398(ctx, base);
	// 822EB688: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 822EB68C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 822EB690: 914A0004  stw r10, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 822EB694: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 822EB698: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 822EB69C: 914A0000  stw r10, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 822EB6A0: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 822EB6A4: 916B0008  stw r11, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 822EB6A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 822EB6AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822EB6B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822EB6B4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822EB6B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822EB6C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x822EB6C0 size=288
    let mut pc: u32 = 0x822EB6C0;
    'dispatch: loop {
        match pc {
            0x822EB6C0 => {
    //   block [0x822EB6C0..0x822EB7E0)
	// 822EB6C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822EB6C4: 48EBCAA1  bl 0x831a8164
	ctx.lr = 0x822EB6C8;
	sub_831A8130(ctx, base);
	// 822EB6C8: 9421FF10  stwu r1, -0xf0(r1)
	ea = ctx.r[1].u32.wrapping_add(-240 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822EB6CC: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 822EB6D0: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 822EB6D4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822EB6D8: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 822EB6DC: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 822EB6E0: 7CFB3B78  mr r27, r7
	ctx.r[27].u64 = ctx.r[7].u64;
	// 822EB6E4: 4BFFD63D  bl 0x822e8d20
	ctx.lr = 0x822EB6E8;
	sub_822E8D20(ctx, base);
	// 822EB6E8: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 822EB6EC: 13E0F0C7  vcmpequd (lvx128) v31, v0, v30
	tmp.u32 = ctx.r[30].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 822EB6F0: 39410090  addi r10, r1, 0x90
	ctx.r[10].s64 = ctx.r[1].s64 + 144;
	// 822EB6F4: 13C0E8C7  vcmpequd (lvx128) v30, v0, v29
	tmp.u32 = ctx.r[29].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[62] using VectorMaskL[(tmp.u32 & 0xF)]
	// 822EB6F8: 392100A0  addi r9, r1, 0xa0
	ctx.r[9].s64 = ctx.r[1].s64 + 160;
	// 822EB6FC: 936100B4  stw r27, 0xb4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(180 as u32), ctx.r[27].u32 ) };
	// 822EB700: 3D008201  lis r8, -0x7dff
	ctx.r[8].s64 = -2113863680;
	// 822EB704: 93E10070  stw r31, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[31].u32 ) };
	// 822EB708: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 822EB70C: 807C0000  lwz r3, 0(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 822EB710: 3908BE20  addi r8, r8, -0x41e0
	ctx.r[8].s64 = ctx.r[8].s64 + -16864;
	// 822EB714: C00B08A8  lfs f0, 0x8a8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2216 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 822EB718: 98E100B0  stb r7, 0xb0(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(176 as u32), ctx.r[7].u8 ) };
	// 822EB71C: D001006C  stfs f0, 0x6c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), tmp.u32 ) };
	// 822EB720: 91010068  stw r8, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[8].u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822EB7E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822EB7E0 size=80
    let mut pc: u32 = 0x822EB7E0;
    'dispatch: loop {
        match pc {
            0x822EB7E0 => {
    //   block [0x822EB7E0..0x822EB830)
	// 822EB7E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822EB7E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822EB7E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822EB7EC: 8163011C  lwz r11, 0x11c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(284 as u32) ) } as u64;
	// 822EB7F0: 80A30118  lwz r5, 0x118(r3)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(280 as u32) ) } as u64;
	// 822EB7F4: 48000014  b 0x822eb808
	pc = 0x822EB808; continue 'dispatch;
	// 822EB7F8: 81450000  lwz r10, 0(r5)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 822EB7FC: 7F0A2040  cmplw cr6, r10, r4
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[4].u32, &mut ctx.xer);
	// 822EB800: 419A0014  beq cr6, 0x822eb814
	if ctx.cr[6].eq {
	pc = 0x822EB814; continue 'dispatch;
	}
	// 822EB804: 38A50004  addi r5, r5, 4
	ctx.r[5].s64 = ctx.r[5].s64 + 4;
	// 822EB808: 7F055840  cmplw cr6, r5, r11
	ctx.cr[6].compare_u32(ctx.r[5].u32, ctx.r[11].u32, &mut ctx.xer);
	// 822EB80C: 409AFFEC  bne cr6, 0x822eb7f8
	if !ctx.cr[6].eq {
	pc = 0x822EB7F8; continue 'dispatch;
	}
	// 822EB810: 48000010  b 0x822eb820
	pc = 0x822EB820; continue 'dispatch;
	// 822EB814: 38830114  addi r4, r3, 0x114
	ctx.r[4].s64 = ctx.r[3].s64 + 276;
	// 822EB818: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 822EB81C: 482BC31D  bl 0x825a7b38
	ctx.lr = 0x822EB820;
	sub_825A7B38(ctx, base);
	// 822EB820: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 822EB824: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822EB828: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822EB82C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822EB830(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x822EB830 size=140
    let mut pc: u32 = 0x822EB830;
    'dispatch: loop {
        match pc {
            0x822EB830 => {
    //   block [0x822EB830..0x822EB8BC)
	// 822EB830: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822EB834: 48EBC935  bl 0x831a8168
	ctx.lr = 0x822EB838;
	sub_831A8130(ctx, base);
	// 822EB838: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822EB83C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 822EB840: 839E0088  lwz r28, 0x88(r30)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(136 as u32) ) } as u64;
	// 822EB844: 83FC0000  lwz r31, 0(r28)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 822EB848: 48000064  b 0x822eb8ac
	pc = 0x822EB8AC; continue 'dispatch;
	// 822EB84C: C03E014C  lfs f1, 0x14c(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(332 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 822EB850: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 822EB854: 481A3C3D  bl 0x8248f490
	ctx.lr = 0x822EB858;
	sub_8248F490(ctx, base);
	// 822EB858: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 822EB85C: 4082004C  bne 0x822eb8a8
	if !ctx.cr[0].eq {
	pc = 0x822EB8A8; continue 'dispatch;
	}
	// 822EB860: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 822EB864: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 822EB868: 389E0090  addi r4, r30, 0x90
	ctx.r[4].s64 = ctx.r[30].s64 + 144;
	// 822EB86C: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 822EB870: 816B0038  lwz r11, 0x38(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) } as u64;
	// 822EB874: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 822EB878: 48633651  bl 0x8291eec8
	ctx.lr = 0x822EB87C;
	sub_8291EEC8(ctx, base);
	// 822EB87C: 817E0094  lwz r11, 0x94(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(148 as u32) ) } as u64;
	// 822EB880: 83A10054  lwz r29, 0x54(r1)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 822EB884: 7F1D5840  cmplw cr6, r29, r11
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[11].u32, &mut ctx.xer);
	// 822EB888: 419A0020  beq cr6, 0x822eb8a8
	if ctx.cr[6].eq {
	pc = 0x822EB8A8; continue 'dispatch;
	}
	// 822EB88C: 389E0084  addi r4, r30, 0x84
	ctx.r[4].s64 = ctx.r[30].s64 + 132;
	// 822EB890: 80BD0010  lwz r5, 0x10(r29)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 822EB894: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 822EB898: 4BFFFA99  bl 0x822eb330
	ctx.lr = 0x822EB89C;
	sub_822EB330(ctx, base);
	// 822EB89C: 83E30000  lwz r31, 0(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 822EB8A0: 939D0010  stw r28, 0x10(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(16 as u32), ctx.r[28].u32 ) };
	// 822EB8A4: 48000008  b 0x822eb8ac
	pc = 0x822EB8AC; continue 'dispatch;
	// 822EB8A8: 83FF0000  lwz r31, 0(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 822EB8AC: 7F1FE040  cmplw cr6, r31, r28
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[28].u32, &mut ctx.xer);
	// 822EB8B0: 409AFF9C  bne cr6, 0x822eb84c
	if !ctx.cr[6].eq {
	pc = 0x822EB84C; continue 'dispatch;
	}
	// 822EB8B4: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 822EB8B8: 48EBC900  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822EB8C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822EB8C0 size=88
    let mut pc: u32 = 0x822EB8C0;
    'dispatch: loop {
        match pc {
            0x822EB8C0 => {
    //   block [0x822EB8C0..0x822EB918)
	// 822EB8C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822EB8C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822EB8C8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822EB8CC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822EB8D0: 9081008C  stw r4, 0x8c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(140 as u32), ctx.r[4].u32 ) };
	// 822EB8D4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822EB8D8: 38A1008C  addi r5, r1, 0x8c
	ctx.r[5].s64 = ctx.r[1].s64 + 140;
	// 822EB8DC: 389F0090  addi r4, r31, 0x90
	ctx.r[4].s64 = ctx.r[31].s64 + 144;
	// 822EB8E0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 822EB8E4: 486335E5  bl 0x8291eec8
	ctx.lr = 0x822EB8E8;
	sub_8291EEC8(ctx, base);
	// 822EB8E8: 815F0094  lwz r10, 0x94(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 822EB8EC: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 822EB8F0: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 822EB8F4: 419A000C  beq cr6, 0x822eb900
	if ctx.cr[6].eq {
	pc = 0x822EB900; continue 'dispatch;
	}
	// 822EB8F8: 806B0014  lwz r3, 0x14(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 822EB8FC: 48000008  b 0x822eb904
	pc = 0x822EB904; continue 'dispatch;
	// 822EB900: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 822EB904: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 822EB908: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822EB90C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822EB910: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822EB914: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822EB918(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822EB918 size=112
    let mut pc: u32 = 0x822EB918;
    'dispatch: loop {
        match pc {
            0x822EB918 => {
    //   block [0x822EB918..0x822EB988)
	// 822EB918: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822EB91C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822EB920: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 822EB924: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822EB928: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822EB92C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 822EB930: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822EB934: 93C10050  stw r30, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 822EB938: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 822EB93C: 4BFFFAB5  bl 0x822eb3f0
	ctx.lr = 0x822EB940;
	sub_822EB3F0(ctx, base);
	// 822EB940: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 822EB944: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 822EB948: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 822EB94C: 4BFD46B5  bl 0x822c0000
	ctx.lr = 0x822EB950;
	sub_822C0000(ctx, base);
	// 822EB950: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 822EB954: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 822EB958: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 822EB95C: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 822EB960: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 822EB964: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 822EB968: 419A0008  beq cr6, 0x822eb970
	if ctx.cr[6].eq {
	pc = 0x822EB970; continue 'dispatch;
	}
	// 822EB96C: 4BFD4F25  bl 0x822c0890
	ctx.lr = 0x822EB970;
	sub_822C0890(ctx, base);
	// 822EB970: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 822EB974: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822EB978: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822EB97C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 822EB980: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822EB984: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822EB988(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822EB988 size=428
    let mut pc: u32 = 0x822EB988;
    'dispatch: loop {
        match pc {
            0x822EB988 => {
    //   block [0x822EB988..0x822EBB34)
	// 822EB988: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822EB98C: 48EBC7DD  bl 0x831a8168
	ctx.lr = 0x822EB990;
	sub_831A8130(ctx, base);
	// 822EB990: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822EB994: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 822EB998: 3BBC0124  addi r29, r28, 0x124
	ctx.r[29].s64 = ctx.r[28].s64 + 292;
	// 822EB99C: 817C0128  lwz r11, 0x128(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(296 as u32) ) } as u64;
	// 822EB9A0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 822EB9A4: 419A00C0  beq cr6, 0x822eba64
	if ctx.cr[6].eq {
	pc = 0x822EBA64; continue 'dispatch;
	}
	// 822EB9A8: 815D0008  lwz r10, 8(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 822EB9AC: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 822EB9B0: 7D6B1671  srawi. r11, r11, 2
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 2) as i64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 822EB9B4: 418200B0  beq 0x822eba64
	if ctx.cr[0].eq {
	pc = 0x822EBA64; continue 'dispatch;
	}
	// 822EB9B8: 83FC0000  lwz r31, 0(r28)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 822EB9BC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 822EB9C0: 419A000C  beq cr6, 0x822eb9cc
	if ctx.cr[6].eq {
	pc = 0x822EB9CC; continue 'dispatch;
	}
	// 822EB9C4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822EB9C8: 48BE2E39  bl 0x82ece800
	ctx.lr = 0x822EB9CC;
	sub_82ECE800(ctx, base);
	// 822EB9CC: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 822EB9D0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 822EB9D4: 409A000C  bne cr6, 0x822eb9e0
	if !ctx.cr[6].eq {
	pc = 0x822EB9E0; continue 'dispatch;
	}
	// 822EB9D8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 822EB9DC: 48000010  b 0x822eb9ec
	pc = 0x822EB9EC; continue 'dispatch;
	// 822EB9E0: 815D0008  lwz r10, 8(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 822EB9E4: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 822EB9E8: 7D651670  srawi r5, r11, 2
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[5].s64 = (ctx.r[11].s32 >> 2) as i64;
	// 822EB9EC: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 822EB9F0: 807C0000  lwz r3, 0(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 822EB9F4: 809C0128  lwz r4, 0x128(r28)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(296 as u32) ) } as u64;
	// 822EB9F8: 48BE7429  bl 0x82ed2e20
	ctx.lr = 0x822EB9FC;
	sub_82ED2E20(ctx, base);
	// 822EB9FC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 822EBA00: 419A000C  beq cr6, 0x822eba0c
	if ctx.cr[6].eq {
	pc = 0x822EBA0C; continue 'dispatch;
	}
	// 822EBA04: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822EBA08: 48BE1601  bl 0x82ecd008
	ctx.lr = 0x822EBA0C;
	sub_82ECD008(ctx, base);
	// 822EBA0C: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 822EBA10: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 822EBA14: 409A000C  bne cr6, 0x822eba20
	if !ctx.cr[6].eq {
	pc = 0x822EBA20; continue 'dispatch;
	}
	// 822EBA18: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 822EBA1C: 48000010  b 0x822eba2c
	pc = 0x822EBA2C; continue 'dispatch;
	// 822EBA20: 815D0008  lwz r10, 8(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 822EBA24: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 822EBA28: 7D7F1670  srawi r31, r11, 2
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[31].s64 = (ctx.r[11].s32 >> 2) as i64;
	// 822EBA2C: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 822EBA30: 40990020  ble cr6, 0x822eba50
	if !ctx.cr[6].gt {
	pc = 0x822EBA50; continue 'dispatch;
	}
	// 822EBA34: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 822EBA38: 817C0128  lwz r11, 0x128(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(296 as u32) ) } as u64;
	// 822EBA3C: 7C6BF02E  lwzx r3, r11, r30
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 822EBA40: 48BE96C9  bl 0x82ed5108
	ctx.lr = 0x822EBA44;
	sub_82ED5108(ctx, base);
	// 822EBA44: 37FFFFFF  addic. r31, r31, -1
	ctx.xer.ca = (ctx.r[31].u32 > (!(-1 as u32)));
	ctx.r[31].s64 = ctx.r[31].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 822EBA48: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 822EBA4C: 4082FFEC  bne 0x822eba38
	if !ctx.cr[0].eq {
	pc = 0x822EBA38; continue 'dispatch;
	}
	// 822EBA50: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 822EBA54: 80DD0008  lwz r6, 8(r29)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 822EBA58: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 822EBA5C: 80BD0004  lwz r5, 4(r29)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 822EBA60: 4801CD09  bl 0x82308768
	ctx.lr = 0x822EBA64;
	sub_82308768(ctx, base);
	// 822EBA64: 817C0138  lwz r11, 0x138(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(312 as u32) ) } as u64;
	// 822EBA68: 3BBC0134  addi r29, r28, 0x134
	ctx.r[29].s64 = ctx.r[28].s64 + 308;
	// 822EBA6C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 822EBA70: 419A00BC  beq cr6, 0x822ebb2c
	if ctx.cr[6].eq {
	pc = 0x822EBB2C; continue 'dispatch;
	}
	// 822EBA74: 815D0008  lwz r10, 8(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 822EBA78: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 822EBA7C: 7D6B1671  srawi. r11, r11, 2
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 2) as i64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 822EBA80: 418200AC  beq 0x822ebb2c
	if ctx.cr[0].eq {
	pc = 0x822EBB2C; continue 'dispatch;
	}
	// 822EBA84: 83FC0000  lwz r31, 0(r28)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 822EBA88: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 822EBA8C: 419A000C  beq cr6, 0x822eba98
	if ctx.cr[6].eq {
	pc = 0x822EBA98; continue 'dispatch;
	}
	// 822EBA90: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822EBA94: 48BE2D6D  bl 0x82ece800
	ctx.lr = 0x822EBA98;
	sub_82ECE800(ctx, base);
	// 822EBA98: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 822EBA9C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 822EBAA0: 409A000C  bne cr6, 0x822ebaac
	if !ctx.cr[6].eq {
	pc = 0x822EBAAC; continue 'dispatch;
	}
	// 822EBAA4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 822EBAA8: 48000010  b 0x822ebab8
	pc = 0x822EBAB8; continue 'dispatch;
	// 822EBAAC: 815D0008  lwz r10, 8(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 822EBAB0: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 822EBAB4: 7D651670  srawi r5, r11, 2
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[5].s64 = (ctx.r[11].s32 >> 2) as i64;
	// 822EBAB8: 807C0000  lwz r3, 0(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 822EBABC: 809C0138  lwz r4, 0x138(r28)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(312 as u32) ) } as u64;
	// 822EBAC0: 48BE7A79  bl 0x82ed3538
	ctx.lr = 0x822EBAC4;
	sub_82ED3538(ctx, base);
	// 822EBAC4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 822EBAC8: 419A000C  beq cr6, 0x822ebad4
	if ctx.cr[6].eq {
	pc = 0x822EBAD4; continue 'dispatch;
	}
	// 822EBACC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822EBAD0: 48BE1539  bl 0x82ecd008
	ctx.lr = 0x822EBAD4;
	sub_82ECD008(ctx, base);
	// 822EBAD4: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 822EBAD8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 822EBADC: 409A000C  bne cr6, 0x822ebae8
	if !ctx.cr[6].eq {
	pc = 0x822EBAE8; continue 'dispatch;
	}
	// 822EBAE0: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 822EBAE4: 48000010  b 0x822ebaf4
	pc = 0x822EBAF4; continue 'dispatch;
	// 822EBAE8: 815D0008  lwz r10, 8(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 822EBAEC: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 822EBAF0: 7D7F1670  srawi r31, r11, 2
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[31].s64 = (ctx.r[11].s32 >> 2) as i64;
	// 822EBAF4: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 822EBAF8: 40990020  ble cr6, 0x822ebb18
	if !ctx.cr[6].gt {
	pc = 0x822EBB18; continue 'dispatch;
	}
	// 822EBAFC: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 822EBB00: 817C0138  lwz r11, 0x138(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(312 as u32) ) } as u64;
	// 822EBB04: 7C7E582E  lwzx r3, r30, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 822EBB08: 48BE9601  bl 0x82ed5108
	ctx.lr = 0x822EBB0C;
	sub_82ED5108(ctx, base);
	// 822EBB0C: 37FFFFFF  addic. r31, r31, -1
	ctx.xer.ca = (ctx.r[31].u32 > (!(-1 as u32)));
	ctx.r[31].s64 = ctx.r[31].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 822EBB10: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 822EBB14: 4082FFEC  bne 0x822ebb00
	if !ctx.cr[0].eq {
	pc = 0x822EBB00; continue 'dispatch;
	}
	// 822EBB18: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 822EBB1C: 80DD0008  lwz r6, 8(r29)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 822EBB20: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 822EBB24: 80BD0004  lwz r5, 4(r29)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 822EBB28: 4801CC41  bl 0x82308768
	ctx.lr = 0x822EBB2C;
	sub_82308768(ctx, base);
	// 822EBB2C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 822EBB30: 48EBC688  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822EBB38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822EBB38 size=76
    let mut pc: u32 = 0x822EBB38;
    'dispatch: loop {
        match pc {
            0x822EBB38 => {
    //   block [0x822EBB38..0x822EBB84)
	// 822EBB38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822EBB3C: 48EBC631  bl 0x831a816c
	ctx.lr = 0x822EBB40;
	sub_831A8130(ctx, base);
	// 822EBB40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822EBB44: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 822EBB48: 83BE00A4  lwz r29, 0xa4(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(164 as u32) ) } as u64;
	// 822EBB4C: 83FE00A0  lwz r31, 0xa0(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(160 as u32) ) } as u64;
	// 822EBB50: 48000010  b 0x822ebb60
	pc = 0x822EBB60; continue 'dispatch;
	// 822EBB54: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 822EBB58: 481A5231  bl 0x82490d88
	ctx.lr = 0x822EBB5C;
	sub_82490D88(ctx, base);
	// 822EBB5C: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 822EBB60: 7F1FE840  cmplw cr6, r31, r29
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[29].u32, &mut ctx.xer);
	// 822EBB64: 409AFFF0  bne cr6, 0x822ebb54
	if !ctx.cr[6].eq {
	pc = 0x822EBB54; continue 'dispatch;
	}
	// 822EBB68: 389E009C  addi r4, r30, 0x9c
	ctx.r[4].s64 = ctx.r[30].s64 + 156;
	// 822EBB6C: 80DE00A4  lwz r6, 0xa4(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(164 as u32) ) } as u64;
	// 822EBB70: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 822EBB74: 80BE00A0  lwz r5, 0xa0(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(160 as u32) ) } as u64;
	// 822EBB78: 4801CBF1  bl 0x82308768
	ctx.lr = 0x822EBB7C;
	sub_82308768(ctx, base);
	// 822EBB7C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 822EBB80: 48EBC63C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822EBB88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x822EBB88 size=340
    let mut pc: u32 = 0x822EBB88;
    'dispatch: loop {
        match pc {
            0x822EBB88 => {
    //   block [0x822EBB88..0x822EBCDC)
	// 822EBB88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822EBB8C: 48EBC5E1  bl 0x831a816c
	ctx.lr = 0x822EBB90;
	sub_831A8130(ctx, base);
	// 822EBB90: DBE1FFD8  stfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 822EBB94: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822EBB98: 3FC08335  lis r30, -0x7ccb
	ctx.r[30].s64 = -2093678592;
	// 822EBB9C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822EBBA0: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 822EBBA4: 897E1780  lbz r11, 0x1780(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(6016 as u32) ) } as u64;
	// 822EBBA8: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 822EBBAC: 41820010  beq 0x822ebbbc
	if ctx.cr[0].eq {
	pc = 0x822EBBBC; continue 'dispatch;
	}
	// 822EBBB0: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 822EBBB4: 4BFFF54D  bl 0x822eb100
	ctx.lr = 0x822EBBB8;
	sub_822EB100(ctx, base);
	// 822EBBB8: 9BBE1780  stb r29, 0x1780(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(6016 as u32), ctx.r[29].u8 ) };
	// 822EBBBC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822EBBC0: 4BFFFDC9  bl 0x822eb988
	ctx.lr = 0x822EBBC4;
	sub_822EB988(ctx, base);
	// 822EBBC4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 822EBBC8: C1BF014C  lfs f13, 0x14c(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(332 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 822EBBCC: C00BC0B0  lfs f0, -0x3f50(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16208 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 822EBBD0: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 822EBBD4: 419800FC  blt cr6, 0x822ebcd0
	if ctx.cr[6].lt {
	pc = 0x822EBCD0; continue 'dispatch;
	}
	// 822EBBD8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822EBBDC: 4BFFFC55  bl 0x822eb830
	ctx.lr = 0x822EBBE0;
	sub_822EB830(ctx, base);
	// 822EBBE0: 897F00E0  lbz r11, 0xe0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(224 as u32) ) } as u64;
	// 822EBBE4: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 822EBBE8: 41820054  beq 0x822ebc3c
	if ctx.cr[0].eq {
	pc = 0x822EBC3C; continue 'dispatch;
	}
	// 822EBBEC: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 822EBBF0: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 822EBBF4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 822EBBF8: 419A0030  beq cr6, 0x822ebc28
	if ctx.cr[6].eq {
	pc = 0x822EBC28; continue 'dispatch;
	}
	// 822EBBFC: 48BE1175  bl 0x82eccd70
	ctx.lr = 0x822EBC00;
	sub_82ECCD70(ctx, base);
	// 822EBC00: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 822EBC04: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 822EBC08: C03F014C  lfs f1, 0x14c(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(332 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 822EBC0C: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 822EBC10: 99410050  stb r10, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u8 ) };
	// 822EBC14: 88AB0000  lbz r5, 0(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 822EBC18: 48C62321  bl 0x82f4df38
	ctx.lr = 0x822EBC1C;
	sub_82F4DF38(ctx, base);
	// 822EBC1C: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 822EBC20: 48C62389  bl 0x82f4dfa8
	ctx.lr = 0x822EBC24;
	sub_82F4DFA8(ctx, base);
	// 822EBC24: 4800009C  b 0x822ebcc0
	pc = 0x822EBCC0; continue 'dispatch;
	// 822EBC28: 48BE1149  bl 0x82eccd70
	ctx.lr = 0x822EBC2C;
	sub_82ECCD70(ctx, base);
	// 822EBC2C: C03F014C  lfs f1, 0x14c(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(332 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 822EBC30: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 822EBC34: 48BE1E15  bl 0x82ecda48
	ctx.lr = 0x822EBC38;
	sub_82ECDA48(ctx, base);
	// 822EBC38: 48000088  b 0x822ebcc0
	pc = 0x822EBCC0; continue 'dispatch;
	// 822EBC3C: 83DF0000  lwz r30, 0(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 822EBC40: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 822EBC44: 419A000C  beq cr6, 0x822ebc50
	if ctx.cr[6].eq {
	pc = 0x822EBC50; continue 'dispatch;
	}
	// 822EBC48: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 822EBC4C: 48BE2BB5  bl 0x82ece800
	ctx.lr = 0x822EBC50;
	sub_82ECE800(ctx, base);
	// 822EBC50: C03F014C  lfs f1, 0x14c(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(332 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 822EBC54: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 822EBC58: 48BE10E9  bl 0x82eccd40
	ctx.lr = 0x822EBC5C;
	sub_82ECCD40(ctx, base);
	// 822EBC5C: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 822EBC60: 48BE1EE1  bl 0x82ecdb40
	ctx.lr = 0x822EBC64;
	sub_82ECDB40(ctx, base);
	// 822EBC64: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 822EBC68: 48BE10E1  bl 0x82eccd48
	ctx.lr = 0x822EBC6C;
	sub_82ECCD48(ctx, base);
	// 822EBC6C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 822EBC70: 40820040  bne 0x822ebcb0
	if !ctx.cr[0].eq {
	pc = 0x822EBCB0; continue 'dispatch;
	}
	// 822EBC74: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 822EBC78: C3EBC0AC  lfs f31, -0x3f54(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16212 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 822EBC7C: C01F014C  lfs f0, 0x14c(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(332 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 822EBC80: FF00F800  fcmpu cr6, f0, f31
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[31].f64);
	// 822EBC84: 41980008  blt cr6, 0x822ebc8c
	if ctx.cr[6].lt {
	pc = 0x822EBC8C; continue 'dispatch;
	}
	// 822EBC88: FC00F890  fmr f0, f31
	ctx.f[0].f64 = ctx.f[31].f64;
	// 822EBC8C: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 822EBC90: FC200090  fmr f1, f0
	ctx.f[1].f64 = ctx.f[0].f64;
	// 822EBC94: 48BE1DB5  bl 0x82ecda48
	ctx.lr = 0x822EBC98;
	sub_82ECDA48(ctx, base);
	// 822EBC98: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 822EBC9C: 48BE10B5  bl 0x82eccd50
	ctx.lr = 0x822EBCA0;
	sub_82ECCD50(ctx, base);
	// 822EBCA0: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 822EBCA4: 48BE10A5  bl 0x82eccd48
	ctx.lr = 0x822EBCA8;
	sub_82ECCD48(ctx, base);
	// 822EBCA8: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 822EBCAC: 4182FFD0  beq 0x822ebc7c
	if ctx.cr[0].eq {
	pc = 0x822EBC7C; continue 'dispatch;
	}
	// 822EBCB0: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 822EBCB4: 419A000C  beq cr6, 0x822ebcc0
	if ctx.cr[6].eq {
	pc = 0x822EBCC0; continue 'dispatch;
	}
	// 822EBCB8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 822EBCBC: 48BE134D  bl 0x82ecd008
	ctx.lr = 0x822EBCC0;
	sub_82ECD008(ctx, base);
	// 822EBCC0: 4BFFC541  bl 0x822e8200
	ctx.lr = 0x822EBCC4;
	sub_822E8200(ctx, base);
	// 822EBCC4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822EBCC8: 4BFFFE71  bl 0x822ebb38
	ctx.lr = 0x822EBCCC;
	sub_822EBB38(ctx, base);
	// 822EBCCC: 9BBF00E1  stb r29, 0xe1(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(225 as u32), ctx.r[29].u8 ) };
	// 822EBCD0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 822EBCD4: CBE1FFD8  lfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-40 as u32) ) };
	// 822EBCD8: 48EBC4E4  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822EBCE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822EBCE0 size=804
    let mut pc: u32 = 0x822EBCE0;
    'dispatch: loop {
        match pc {
            0x822EBCE0 => {
    //   block [0x822EBCE0..0x822EC004)
	// 822EBCE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822EBCE4: 48EBC471  bl 0x831a8154
	ctx.lr = 0x822EBCE8;
	sub_831A8130(ctx, base);
	// 822EBCE8: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822EBCEC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 822EBCF0: 7CB92B78  mr r25, r5
	ctx.r[25].u64 = ctx.r[5].u64;
	// 822EBCF4: 7CD73378  mr r23, r6
	ctx.r[23].u64 = ctx.r[6].u64;
	// 822EBCF8: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 822EBCFC: 7D79B850  subf r11, r25, r23
	ctx.r[11].s64 = ctx.r[23].s64 - ctx.r[25].s64;
	// 822EBD00: 813D0004  lwz r9, 4(r29)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 822EBD04: 7D781670  srawi r24, r11, 2
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[24].s64 = (ctx.r[11].s32 >> 2) as i64;
	// 822EBD08: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 822EBD0C: 409A000C  bne cr6, 0x822ebd18
	if !ctx.cr[6].eq {
	pc = 0x822EBD18; continue 'dispatch;
	}
	// 822EBD10: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 822EBD14: 48000010  b 0x822ebd24
	pc = 0x822EBD24; continue 'dispatch;
	// 822EBD18: 817D000C  lwz r11, 0xc(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 822EBD1C: 7D695850  subf r11, r9, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 822EBD20: 7D681670  srawi r8, r11, 2
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[8].s64 = (ctx.r[11].s32 >> 2) as i64;
	// 822EBD24: 2B180000  cmplwi cr6, r24, 0
	ctx.cr[6].compare_u32(ctx.r[24].u32, 0 as u32, &mut ctx.xer);
	// 822EBD28: 419A02D4  beq cr6, 0x822ebffc
	if ctx.cr[6].eq {
	pc = 0x822EBFFC; continue 'dispatch;
	}
	// 822EBD2C: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 822EBD30: 409A000C  bne cr6, 0x822ebd3c
	if !ctx.cr[6].eq {
	pc = 0x822EBD3C; continue 'dispatch;
	}
	// 822EBD34: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 822EBD38: 48000010  b 0x822ebd48
	pc = 0x822EBD48; continue 'dispatch;
	// 822EBD3C: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 822EBD40: 7D695850  subf r11, r9, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 822EBD44: 7D6B1670  srawi r11, r11, 2
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 2) as i64;
	// 822EBD48: 3D403FFF  lis r10, 0x3fff
	ctx.r[10].s64 = 1073676288;
	// 822EBD4C: 614AFFFF  ori r10, r10, 0xffff
	ctx.r[10].u64 = ctx.r[10].u64 | 65535;
	// 822EBD50: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 822EBD54: 7F0BC040  cmplw cr6, r11, r24
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[24].u32, &mut ctx.xer);
	// 822EBD58: 4098000C  bge cr6, 0x822ebd64
	if !ctx.cr[6].lt {
	pc = 0x822EBD64; continue 'dispatch;
	}
	// 822EBD5C: 488CBFC5  bl 0x82bb7d20
	ctx.lr = 0x822EBD60;
	sub_82BB7D20(ctx, base);
	// 822EBD60: 4800029C  b 0x822ebffc
	pc = 0x822EBFFC; continue 'dispatch;
	// 822EBD64: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 822EBD68: 409A000C  bne cr6, 0x822ebd74
	if !ctx.cr[6].eq {
	pc = 0x822EBD74; continue 'dispatch;
	}
	// 822EBD6C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 822EBD70: 48000010  b 0x822ebd80
	pc = 0x822EBD80; continue 'dispatch;
	// 822EBD74: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 822EBD78: 7D695850  subf r11, r9, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 822EBD7C: 7D6B1670  srawi r11, r11, 2
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 2) as i64;
	// 822EBD80: 7D6BC214  add r11, r11, r24
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[24].u64;
	// 822EBD84: 7F085840  cmplw cr6, r8, r11
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[11].u32, &mut ctx.xer);
	// 822EBD88: 4098014C  bge cr6, 0x822ebed4
	if !ctx.cr[6].lt {
	pc = 0x822EBED4; continue 'dispatch;
	}
	// 822EBD8C: 550BF87E  srwi r11, r8, 1
	ctx.r[11].u32 = ctx.r[8].u32.wrapping_shr(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 822EBD90: 7D4B5050  subf r10, r11, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 822EBD94: 7F0A4040  cmplw cr6, r10, r8
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[8].u32, &mut ctx.xer);
	// 822EBD98: 4098000C  bge cr6, 0x822ebda4
	if !ctx.cr[6].lt {
	pc = 0x822EBDA4; continue 'dispatch;
	}
	// 822EBD9C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 822EBDA0: 48000008  b 0x822ebda8
	pc = 0x822EBDA8; continue 'dispatch;
	// 822EBDA4: 7D6B4214  add r11, r11, r8
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[8].u64;
	// 822EBDA8: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 822EBDAC: 409A000C  bne cr6, 0x822ebdb8
	if !ctx.cr[6].eq {
	pc = 0x822EBDB8; continue 'dispatch;
	}
	// 822EBDB0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 822EBDB4: 48000010  b 0x822ebdc4
	pc = 0x822EBDC4; continue 'dispatch;
	// 822EBDB8: 815D0008  lwz r10, 8(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 822EBDBC: 7D495050  subf r10, r9, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[9].s64;
	// 822EBDC0: 7D4A1670  srawi r10, r10, 2
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[10].s32 >> 2) as i64;
	// 822EBDC4: 7D4AC214  add r10, r10, r24
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[24].u64;
	// 822EBDC8: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 822EBDCC: 40980024  bge cr6, 0x822ebdf0
	if !ctx.cr[6].lt {
	pc = 0x822EBDF0; continue 'dispatch;
	}
	// 822EBDD0: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 822EBDD4: 409A000C  bne cr6, 0x822ebde0
	if !ctx.cr[6].eq {
	pc = 0x822EBDE0; continue 'dispatch;
	}
	// 822EBDD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 822EBDDC: 48000010  b 0x822ebdec
	pc = 0x822EBDEC; continue 'dispatch;
	// 822EBDE0: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 822EBDE4: 7D695850  subf r11, r9, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 822EBDE8: 7D6B1670  srawi r11, r11, 2
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 2) as i64;
	// 822EBDEC: 7D6BC214  add r11, r11, r24
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[24].u64;
	// 822EBDF0: 557B103A  slwi r27, r11, 2
	ctx.r[27].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[27].u64 = ctx.r[27].u32 as u64;
	// 822EBDF4: 3F808335  lis r28, -0x7ccb
	ctx.r[28].s64 = -2093678592;
	// 822EBDF8: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 822EBDFC: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 822EBE00: 388B08B0  addi r4, r11, 0x8b0
	ctx.r[4].s64 = ctx.r[11].s64 + 2224;
	// 822EBE04: 38A0002D  li r5, 0x2d
	ctx.r[5].s64 = 45;
	// 822EBE08: 807C110C  lwz r3, 0x110c(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4364 as u32) ) } as u64;
	// 822EBE0C: 48B062BD  bl 0x82df20c8
	ctx.lr = 0x822EBE10;
	sub_82DF20C8(ctx, base);
	// 822EBE10: 80BD0004  lwz r5, 4(r29)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 822EBE14: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 822EBE18: 7D65D050  subf r11, r5, r26
	ctx.r[11].s64 = ctx.r[26].s64 - ctx.r[5].s64;
	// 822EBE1C: 7D6B1671  srawi. r11, r11, 2
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 2) as i64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 822EBE20: 5566103A  slwi r6, r11, 2
	ctx.r[6].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 822EBE24: 7FE6F214  add r31, r6, r30
	ctx.r[31].u64 = ctx.r[6].u64 + ctx.r[30].u64;
	// 822EBE28: 4182000C  beq 0x822ebe34
	if ctx.cr[0].eq {
	pc = 0x822EBE34; continue 'dispatch;
	}
	// 822EBE2C: 7CC43378  mr r4, r6
	ctx.r[4].u64 = ctx.r[6].u64;
	// 822EBE30: 48EBCED9  bl 0x831a8d08
	ctx.lr = 0x822EBE34;
	sub_831A8D08(ctx, base);
	// 822EBE34: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822EBE38: 7F19B840  cmplw cr6, r25, r23
	ctx.cr[6].compare_u32(ctx.r[25].u32, ctx.r[23].u32, &mut ctx.xer);
	// 822EBE3C: 419A0028  beq cr6, 0x822ebe64
	if ctx.cr[6].eq {
	pc = 0x822EBE64; continue 'dispatch;
	}
	// 822EBE40: 7D7FC850  subf r11, r31, r25
	ctx.r[11].s64 = ctx.r[25].s64 - ctx.r[31].s64;
	// 822EBE44: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 822EBE48: 419A000C  beq cr6, 0x822ebe54
	if ctx.cr[6].eq {
	pc = 0x822EBE54; continue 'dispatch;
	}
	// 822EBE4C: 7D43582E  lwzx r10, r3, r11
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[3].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 822EBE50: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 822EBE54: 38630004  addi r3, r3, 4
	ctx.r[3].s64 = ctx.r[3].s64 + 4;
	// 822EBE58: 7D435A14  add r10, r3, r11
	ctx.r[10].u64 = ctx.r[3].u64 + ctx.r[11].u64;
	// 822EBE5C: 7F0AB840  cmplw cr6, r10, r23
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[23].u32, &mut ctx.xer);
	// 822EBE60: 409AFFE4  bne cr6, 0x822ebe44
	if !ctx.cr[6].eq {
	pc = 0x822EBE44; continue 'dispatch;
	}
	// 822EBE64: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 822EBE68: 7D7A5850  subf r11, r26, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[26].s64;
	// 822EBE6C: 7D6B1671  srawi. r11, r11, 2
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 2) as i64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 822EBE70: 41820014  beq 0x822ebe84
	if ctx.cr[0].eq {
	pc = 0x822EBE84; continue 'dispatch;
	}
	// 822EBE74: 5564103A  slwi r4, r11, 2
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 822EBE78: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 822EBE7C: 7C862378  mr r6, r4
	ctx.r[6].u64 = ctx.r[4].u64;
	// 822EBE80: 48EBCE89  bl 0x831a8d08
	ctx.lr = 0x822EBE84;
	sub_831A8D08(ctx, base);
	// 822EBE84: 809D0004  lwz r4, 4(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 822EBE88: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 822EBE8C: 409A000C  bne cr6, 0x822ebe98
	if !ctx.cr[6].eq {
	pc = 0x822EBE98; continue 'dispatch;
	}
	// 822EBE90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 822EBE94: 48000010  b 0x822ebea4
	pc = 0x822EBEA4; continue 'dispatch;
	// 822EBE98: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 822EBE9C: 7D645850  subf r11, r4, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[4].s64;
	// 822EBEA0: 7D6B1670  srawi r11, r11, 2
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 2) as i64;
	// 822EBEA4: 7FEBC214  add r31, r11, r24
	ctx.r[31].u64 = ctx.r[11].u64 + ctx.r[24].u64;
	// 822EBEA8: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 822EBEAC: 419A000C  beq cr6, 0x822ebeb8
	if ctx.cr[6].eq {
	pc = 0x822EBEB8; continue 'dispatch;
	}
	// 822EBEB0: 807C110C  lwz r3, 0x110c(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4364 as u32) ) } as u64;
	// 822EBEB4: 48B062D5  bl 0x82df2188
	ctx.lr = 0x822EBEB8;
	sub_82DF2188(ctx, base);
	// 822EBEB8: 57EB103A  slwi r11, r31, 2
	ctx.r[11].u32 = ctx.r[31].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 822EBEBC: 93DD0004  stw r30, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 822EBEC0: 7D5BF214  add r10, r27, r30
	ctx.r[10].u64 = ctx.r[27].u64 + ctx.r[30].u64;
	// 822EBEC4: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 822EBEC8: 915D000C  stw r10, 0xc(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 822EBECC: 917D0008  stw r11, 8(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 822EBED0: 4800012C  b 0x822ebffc
	pc = 0x822EBFFC; continue 'dispatch;
	// 822EBED4: 83FD0008  lwz r31, 8(r29)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 822EBED8: 7D7AF850  subf r11, r26, r31
	ctx.r[11].s64 = ctx.r[31].s64 - ctx.r[26].s64;
	// 822EBEDC: 7D6B1670  srawi r11, r11, 2
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 2) as i64;
	// 822EBEE0: 7F0BC040  cmplw cr6, r11, r24
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[24].u32, &mut ctx.xer);
	// 822EBEE4: 409800A0  bge cr6, 0x822ebf84
	if !ctx.cr[6].lt {
	pc = 0x822EBF84; continue 'dispatch;
	}
	// 822EBEE8: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 822EBEEC: 4182001C  beq 0x822ebf08
	if ctx.cr[0].eq {
	pc = 0x822EBF08; continue 'dispatch;
	}
	// 822EBEF0: 5564103A  slwi r4, r11, 2
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 822EBEF4: 570B103A  slwi r11, r24, 2
	ctx.r[11].u32 = ctx.r[24].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 822EBEF8: 7C862378  mr r6, r4
	ctx.r[6].u64 = ctx.r[4].u64;
	// 822EBEFC: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 822EBF00: 7C6BD214  add r3, r11, r26
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[26].u64;
	// 822EBF04: 48EBCE05  bl 0x831a8d08
	ctx.lr = 0x822EBF08;
	sub_831A8D08(ctx, base);
	// 822EBF08: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 822EBF0C: 7D5A5850  subf r10, r26, r11
	ctx.r[10].s64 = ctx.r[11].s64 - ctx.r[26].s64;
	// 822EBF10: 7D695B78  mr r9, r11
	ctx.r[9].u64 = ctx.r[11].u64;
	// 822EBF14: 7D4B1670  srawi r11, r10, 2
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[10].s32 >> 2) as i64;
	// 822EBF18: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 822EBF1C: 7D6BCA14  add r11, r11, r25
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[25].u64;
	// 822EBF20: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 822EBF24: 7F0BB840  cmplw cr6, r11, r23
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[23].u32, &mut ctx.xer);
	// 822EBF28: 419A0024  beq cr6, 0x822ebf4c
	if ctx.cr[6].eq {
	pc = 0x822EBF4C; continue 'dispatch;
	}
	// 822EBF2C: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 822EBF30: 419A000C  beq cr6, 0x822ebf3c
	if ctx.cr[6].eq {
	pc = 0x822EBF3C; continue 'dispatch;
	}
	// 822EBF34: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 822EBF38: 91090000  stw r8, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 822EBF3C: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 822EBF40: 39290004  addi r9, r9, 4
	ctx.r[9].s64 = ctx.r[9].s64 + 4;
	// 822EBF44: 7F0AB840  cmplw cr6, r10, r23
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[23].u32, &mut ctx.xer);
	// 822EBF48: 409AFFE4  bne cr6, 0x822ebf2c
	if !ctx.cr[6].eq {
	pc = 0x822EBF2C; continue 'dispatch;
	}
	// 822EBF4C: 811D0008  lwz r8, 8(r29)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 822EBF50: 5709103A  slwi r9, r24, 2
	ctx.r[9].u32 = ctx.r[24].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 822EBF54: 7F2ACB78  mr r10, r25
	ctx.r[10].u64 = ctx.r[25].u64;
	// 822EBF58: 7D294214  add r9, r9, r8
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[8].u64;
	// 822EBF5C: 7F195840  cmplw cr6, r25, r11
	ctx.cr[6].compare_u32(ctx.r[25].u32, ctx.r[11].u32, &mut ctx.xer);
	// 822EBF60: 913D0008  stw r9, 8(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 822EBF64: 419A0098  beq cr6, 0x822ebffc
	if ctx.cr[6].eq {
	pc = 0x822EBFFC; continue 'dispatch;
	}
	// 822EBF68: 7D39D050  subf r9, r25, r26
	ctx.r[9].s64 = ctx.r[26].s64 - ctx.r[25].s64;
	// 822EBF6C: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 822EBF70: 7D0A492E  stwx r8, r10, r9
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32), ctx.r[8].u32) };
	// 822EBF74: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 822EBF78: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 822EBF7C: 409AFFF0  bne cr6, 0x822ebf6c
	if !ctx.cr[6].eq {
	pc = 0x822EBF6C; continue 'dispatch;
	}
	// 822EBF80: 4800007C  b 0x822ebffc
	pc = 0x822EBFFC; continue 'dispatch;
	// 822EBF84: 570B103A  slwi r11, r24, 2
	ctx.r[11].u32 = ctx.r[24].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 822EBF88: 7FCBF850  subf r30, r11, r31
	ctx.r[30].s64 = ctx.r[31].s64 - ctx.r[11].s64;
	// 822EBF8C: 7D7EF850  subf r11, r30, r31
	ctx.r[11].s64 = ctx.r[31].s64 - ctx.r[30].s64;
	// 822EBF90: 7D6B1671  srawi. r11, r11, 2
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 2) as i64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 822EBF94: 5566103A  slwi r6, r11, 2
	ctx.r[6].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 822EBF98: 7F86FA14  add r28, r6, r31
	ctx.r[28].u64 = ctx.r[6].u64 + ctx.r[31].u64;
	// 822EBF9C: 41820014  beq 0x822ebfb0
	if ctx.cr[0].eq {
	pc = 0x822EBFB0; continue 'dispatch;
	}
	// 822EBFA0: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 822EBFA4: 7CC43378  mr r4, r6
	ctx.r[4].u64 = ctx.r[6].u64;
	// 822EBFA8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822EBFAC: 48EBCD5D  bl 0x831a8d08
	ctx.lr = 0x822EBFB0;
	sub_831A8D08(ctx, base);
	// 822EBFB0: 7D7AF050  subf r11, r26, r30
	ctx.r[11].s64 = ctx.r[30].s64 - ctx.r[26].s64;
	// 822EBFB4: 939D0008  stw r28, 8(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(8 as u32), ctx.r[28].u32 ) };
	// 822EBFB8: 7D6B1671  srawi. r11, r11, 2
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 2) as i64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 822EBFBC: 4081001C  ble 0x822ebfd8
	if !ctx.cr[0].gt {
	pc = 0x822EBFD8; continue 'dispatch;
	}
	// 822EBFC0: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 822EBFC4: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 822EBFC8: 7D665B78  mr r6, r11
	ctx.r[6].u64 = ctx.r[11].u64;
	// 822EBFCC: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 822EBFD0: 7C6BF850  subf r3, r11, r31
	ctx.r[3].s64 = ctx.r[31].s64 - ctx.r[11].s64;
	// 822EBFD4: 48EBCD35  bl 0x831a8d08
	ctx.lr = 0x822EBFD8;
	sub_831A8D08(ctx, base);
	// 822EBFD8: 7F2BCB78  mr r11, r25
	ctx.r[11].u64 = ctx.r[25].u64;
	// 822EBFDC: 7F19B840  cmplw cr6, r25, r23
	ctx.cr[6].compare_u32(ctx.r[25].u32, ctx.r[23].u32, &mut ctx.xer);
	// 822EBFE0: 419A001C  beq cr6, 0x822ebffc
	if ctx.cr[6].eq {
	pc = 0x822EBFFC; continue 'dispatch;
	}
	// 822EBFE4: 7D59D050  subf r10, r25, r26
	ctx.r[10].s64 = ctx.r[26].s64 - ctx.r[25].s64;
	// 822EBFE8: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 822EBFEC: 7D2B512E  stwx r9, r11, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[9].u32) };
	// 822EBFF0: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 822EBFF4: 7F0BB840  cmplw cr6, r11, r23
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[23].u32, &mut ctx.xer);
	// 822EBFF8: 409AFFF0  bne cr6, 0x822ebfe8
	if !ctx.cr[6].eq {
	pc = 0x822EBFE8; continue 'dispatch;
	}
	// 822EBFFC: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 822EC000: 48EBC1A4  b 0x831a81a4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822EC008(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822EC008 size=88
    let mut pc: u32 = 0x822EC008;
    'dispatch: loop {
        match pc {
            0x822EC008 => {
    //   block [0x822EC008..0x822EC060)
	// 822EC008: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822EC00C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822EC010: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822EC014: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822EC018: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822EC01C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 822EC020: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 822EC024: 80DF0004  lwz r6, 4(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 822EC028: 80A60000  lwz r5, 0(r6)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 822EC02C: 481A4535  bl 0x82490560
	ctx.lr = 0x822EC030;
	sub_82490560(ctx, base);
	// 822EC030: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 822EC034: 809F0004  lwz r4, 4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 822EC038: 806B110C  lwz r3, 0x110c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4364 as u32) ) } as u64;
	// 822EC03C: 48B0614D  bl 0x82df2188
	ctx.lr = 0x822EC040;
	sub_82DF2188(ctx, base);
	// 822EC040: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 822EC044: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 822EC048: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 822EC04C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 822EC050: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822EC054: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822EC058: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822EC05C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822EC060(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822EC060 size=128
    let mut pc: u32 = 0x822EC060;
    'dispatch: loop {
        match pc {
            0x822EC060 => {
    //   block [0x822EC060..0x822EC0E0)
	// 822EC060: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822EC064: 48EBC109  bl 0x831a816c
	ctx.lr = 0x822EC068;
	sub_831A8130(ctx, base);
	// 822EC068: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822EC06C: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 822EC070: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 822EC074: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 822EC078: 3BEB1784  addi r31, r11, 0x1784
	ctx.r[31].s64 = ctx.r[11].s64 + 6020;
	// 822EC07C: 816A178C  lwz r11, 0x178c(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(6028 as u32) ) } as u64;
	// 822EC080: 556907FF  clrlwi. r9, r11, 0x1f
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 822EC084: 40820024  bne 0x822ec0a8
	if !ctx.cr[0].eq {
	pc = 0x822EC0A8; continue 'dispatch;
	}
	// 822EC088: 3D208256  lis r9, -0x7daa
	ctx.r[9].s64 = -2108293120;
	// 822EC08C: 3D00822F  lis r8, -0x7dd1
	ctx.r[8].s64 = -2110849024;
	// 822EC090: 616B0001  ori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 | 1;
	// 822EC094: 3929DA70  addi r9, r9, -0x2590
	ctx.r[9].s64 = ctx.r[9].s64 + -9616;
	// 822EC098: 3908B4F0  addi r8, r8, -0x4b10
	ctx.r[8].s64 = ctx.r[8].s64 + -19216;
	// 822EC09C: 916A178C  stw r11, 0x178c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(6028 as u32), ctx.r[11].u32 ) };
	// 822EC0A0: 913F0004  stw r9, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 822EC0A4: 911F0000  stw r8, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 822EC0A8: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 822EC0AC: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 822EC0B0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822EC0B4: 38BE0008  addi r5, r30, 8
	ctx.r[5].s64 = ctx.r[30].s64 + 8;
	// 822EC0B8: 9BAB0000  stb r29, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u8 ) };
	// 822EC0BC: 88C10050  lbz r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 822EC0C0: 482E6861  bl 0x825d2920
	ctx.lr = 0x822EC0C4;
	sub_825D2920(ctx, base);
	// 822EC0C4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 822EC0C8: 4182000C  beq 0x822ec0d4
	if ctx.cr[0].eq {
	pc = 0x822EC0D4; continue 'dispatch;
	}
	// 822EC0CC: 93FE0000  stw r31, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 822EC0D0: 48000008  b 0x822ec0d8
	pc = 0x822EC0D8; continue 'dispatch;
	// 822EC0D4: 93BE0000  stw r29, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 822EC0D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 822EC0DC: 48EBC0E0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822EC0E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822EC0E0 size=180
    let mut pc: u32 = 0x822EC0E0;
    'dispatch: loop {
        match pc {
            0x822EC0E0 => {
    //   block [0x822EC0E0..0x822EC194)
	// 822EC0E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822EC0E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822EC0E8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 822EC0EC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822EC0F0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822EC0F4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 822EC0F8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 822EC0FC: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 822EC100: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 822EC104: 409A0078  bne cr6, 0x822ec17c
	if !ctx.cr[6].eq {
	pc = 0x822EC17C; continue 'dispatch;
	}
	// 822EC108: 93DF0008  stw r30, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 822EC10C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 822EC110: 817E0220  lwz r11, 0x220(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(544 as u32) ) } as u64;
	// 822EC114: 394B0001  addi r10, r11, 1
	ctx.r[10].s64 = ctx.r[11].s64 + 1;
	// 822EC118: 915E0220  stw r10, 0x220(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(544 as u32), ctx.r[10].u32 ) };
	// 822EC11C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822EC120: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 822EC124: 917F0038  stw r11, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 822EC128: 816A0028  lwz r11, 0x28(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(40 as u32) ) } as u64;
	// 822EC12C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 822EC130: 4E800421  bctrl
	ctx.lr = 0x822EC134;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 822EC134: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822EC138: 481A2FB9  bl 0x8248f0f0
	ctx.lr = 0x822EC13C;
	sub_8248F0F0(ctx, base);
	// 822EC13C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 822EC140: 41820010  beq 0x822ec150
	if ctx.cr[0].eq {
	pc = 0x822EC150; continue 'dispatch;
	}
	// 822EC144: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 822EC148: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 822EC14C: 4BFFE43D  bl 0x822ea588
	ctx.lr = 0x822EC150;
	sub_822EA588(ctx, base);
	// 822EC150: 815F0038  lwz r10, 0x38(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 822EC154: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 822EC158: 93E10054  stw r31, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[31].u32 ) };
	// 822EC15C: 389E0090  addi r4, r30, 0x90
	ctx.r[4].s64 = ctx.r[30].s64 + 144;
	// 822EC160: 817E0088  lwz r11, 0x88(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(136 as u32) ) } as u64;
	// 822EC164: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 822EC168: 91410060  stw r10, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[10].u32 ) };
	// 822EC16C: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 822EC170: E9610050  ld r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 822EC174: F9610064  std r11, 0x64(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u64 ) };
	// 822EC178: 481A4699  bl 0x82490810
	ctx.lr = 0x822EC17C;
	sub_82490810(ctx, base);
	// 822EC17C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 822EC180: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822EC184: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822EC188: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 822EC18C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822EC190: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822EC198(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x822EC198 size=168
    let mut pc: u32 = 0x822EC198;
    'dispatch: loop {
        match pc {
            0x822EC198 => {
    //   block [0x822EC198..0x822EC240)
	// 822EC198: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822EC19C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822EC1A0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822EC1A4: 8164000C  lwz r11, 0xc(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(12 as u32) ) } as u64;
	// 822EC1A8: 7C691B78  mr r9, r3
	ctx.r[9].u64 = ctx.r[3].u64;
	// 822EC1AC: 7CAA2B78  mr r10, r5
	ctx.r[10].u64 = ctx.r[5].u64;
	// 822EC1B0: 4800000C  b 0x822ec1bc
	pc = 0x822EC1BC; continue 'dispatch;
	// 822EC1B4: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 822EC1B8: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 822EC1BC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 822EC1C0: 409AFFF4  bne cr6, 0x822ec1b4
	if !ctx.cr[6].eq {
	pc = 0x822EC1B4; continue 'dispatch;
	}
	// 822EC1C4: 89640010  lbz r11, 0x10(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(16 as u32) ) } as u64;
	// 822EC1C8: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 822EC1CC: 7D6B2214  add r11, r11, r4
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 822EC1D0: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 822EC1D4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 822EC1D8: 91610078  stw r11, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[11].u32 ) };
	// 822EC1DC: 419A0054  beq cr6, 0x822ec230
	if ctx.cr[6].eq {
	pc = 0x822EC230; continue 'dispatch;
	}
	// 822EC1E0: C1AA0000  lfs f13, 0(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 822EC1E4: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 822EC1E8: D1A10050  stfs f13, 0x50(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 822EC1EC: 80E10050  lwz r7, 0x50(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 822EC1F0: C00A0008  lfs f0, 8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 822EC1F4: 99610084  stb r11, 0x84(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), ctx.r[11].u8 ) };
	// 822EC1F8: D0010058  stfs f0, 0x58(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), tmp.u32 ) };
	// 822EC1FC: 81010058  lwz r8, 0x58(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 822EC200: C00A0004  lfs f0, 4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 822EC204: 91010068  stw r8, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[8].u32 ) };
	// 822EC208: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 822EC20C: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 822EC210: C00A0010  lfs f0, 0x10(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 822EC214: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 822EC218: D001007C  stfs f0, 0x7c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), tmp.u32 ) };
	// 822EC21C: 90E10060  stw r7, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[7].u32 ) };
	// 822EC220: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 822EC224: 80890008  lwz r4, 8(r9)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) } as u64;
	// 822EC228: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 822EC22C: 4BFFCD75  bl 0x822e8fa0
	ctx.lr = 0x822EC230;
	sub_822E8FA0(ctx, base);
	// 822EC230: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 822EC234: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822EC238: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822EC23C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822EC240(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822EC240 size=152
    let mut pc: u32 = 0x822EC240;
    'dispatch: loop {
        match pc {
            0x822EC240 => {
    //   block [0x822EC240..0x822EC2D8)
	// 822EC240: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822EC244: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822EC248: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822EC24C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822EC250: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822EC254: 817F0100  lwz r11, 0x100(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(256 as u32) ) } as u64;
	// 822EC258: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 822EC25C: 40990068  ble cr6, 0x822ec2c4
	if !ctx.cr[6].gt {
	pc = 0x822EC2C4; continue 'dispatch;
	}
	// 822EC260: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 822EC264: 917F0100  stw r11, 0x100(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(256 as u32), ctx.r[11].u32 ) };
	// 822EC268: 4082005C  bne 0x822ec2c4
	if !ctx.cr[0].eq {
	pc = 0x822EC2C4; continue 'dispatch;
	}
	// 822EC26C: 387F0124  addi r3, r31, 0x124
	ctx.r[3].s64 = ctx.r[31].s64 + 292;
	// 822EC270: 80DF010C  lwz r6, 0x10c(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(268 as u32) ) } as u64;
	// 822EC274: 80BF0108  lwz r5, 0x108(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(264 as u32) ) } as u64;
	// 822EC278: 88E10050  lbz r7, 0x50(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 822EC27C: 809F0128  lwz r4, 0x128(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(296 as u32) ) } as u64;
	// 822EC280: 4BFFFA61  bl 0x822ebce0
	ctx.lr = 0x822EC284;
	sub_822EBCE0(ctx, base);
	// 822EC284: 389F0104  addi r4, r31, 0x104
	ctx.r[4].s64 = ctx.r[31].s64 + 260;
	// 822EC288: 80DF010C  lwz r6, 0x10c(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(268 as u32) ) } as u64;
	// 822EC28C: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 822EC290: 80BF0108  lwz r5, 0x108(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(264 as u32) ) } as u64;
	// 822EC294: 4801C4D5  bl 0x82308768
	ctx.lr = 0x822EC298;
	sub_82308768(ctx, base);
	// 822EC298: 387F0134  addi r3, r31, 0x134
	ctx.r[3].s64 = ctx.r[31].s64 + 308;
	// 822EC29C: 88E10050  lbz r7, 0x50(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 822EC2A0: 80DF011C  lwz r6, 0x11c(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(284 as u32) ) } as u64;
	// 822EC2A4: 80BF0118  lwz r5, 0x118(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(280 as u32) ) } as u64;
	// 822EC2A8: 809F0138  lwz r4, 0x138(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(312 as u32) ) } as u64;
	// 822EC2AC: 4BFFFA35  bl 0x822ebce0
	ctx.lr = 0x822EC2B0;
	sub_822EBCE0(ctx, base);
	// 822EC2B0: 389F0114  addi r4, r31, 0x114
	ctx.r[4].s64 = ctx.r[31].s64 + 276;
	// 822EC2B4: 80DF011C  lwz r6, 0x11c(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(284 as u32) ) } as u64;
	// 822EC2B8: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 822EC2BC: 80BF0118  lwz r5, 0x118(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(280 as u32) ) } as u64;
	// 822EC2C0: 4801C4A9  bl 0x82308768
	ctx.lr = 0x822EC2C4;
	sub_82308768(ctx, base);
	// 822EC2C4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 822EC2C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822EC2CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822EC2D0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822EC2D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822EC2D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822EC2D8 size=88
    let mut pc: u32 = 0x822EC2D8;
    'dispatch: loop {
        match pc {
            0x822EC2D8 => {
    //   block [0x822EC2D8..0x822EC330)
	// 822EC2D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822EC2DC: 48EBBE8D  bl 0x831a8168
	ctx.lr = 0x822EC2E0;
	sub_831A8130(ctx, base);
	// 822EC2E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822EC2E4: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 822EC2E8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 822EC2EC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822EC2F0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 822EC2F4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 822EC2F8: 80BD0004  lwz r5, 4(r29)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 822EC2FC: 4826DA05  bl 0x82559d00
	ctx.lr = 0x822EC300;
	sub_82559D00(ctx, base);
	// 822EC300: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 822EC304: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 822EC308: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 822EC30C: 488DDAED  bl 0x82bc9df8
	ctx.lr = 0x822EC310;
	sub_82BC9DF8(ctx, base);
	// 822EC310: 939D0004  stw r28, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 822EC314: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 822EC318: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822EC31C: 938B0000  stw r28, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 822EC320: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 822EC324: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 822EC328: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 822EC32C: 48EBBE8C  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822EC330(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822EC330 size=100
    let mut pc: u32 = 0x822EC330;
    'dispatch: loop {
        match pc {
            0x822EC330 => {
    //   block [0x822EC330..0x822EC394)
	// 822EC330: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822EC334: 48EBBE39  bl 0x831a816c
	ctx.lr = 0x822EC338;
	sub_831A8130(ctx, base);
	// 822EC338: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822EC33C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 822EC340: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 822EC344: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 822EC348: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 822EC34C: 481A0835  bl 0x8248cb80
	ctx.lr = 0x822EC350;
	sub_8248CB80(ctx, base);
	// 822EC350: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 822EC354: 38A10054  addi r5, r1, 0x54
	ctx.r[5].s64 = ctx.r[1].s64 + 84;
	// 822EC358: 88C10050  lbz r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 822EC35C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 822EC360: 83E1005C  lwz r31, 0x5c(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 822EC364: 83C10058  lwz r30, 0x58(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 822EC368: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 822EC36C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 822EC370: 48632761  bl 0x8291ead0
	ctx.lr = 0x822EC374;
	sub_8291EAD0(ctx, base);
	// 822EC374: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 822EC378: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 822EC37C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 822EC380: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 822EC384: 481A41DD  bl 0x82490560
	ctx.lr = 0x822EC388;
	sub_82490560(ctx, base);
	// 822EC388: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 822EC38C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 822EC390: 48EBBE2C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822EC398(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822EC398 size=236
    let mut pc: u32 = 0x822EC398;
    'dispatch: loop {
        match pc {
            0x822EC398 => {
    //   block [0x822EC398..0x822EC484)
	// 822EC398: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822EC39C: 48EBBDCD  bl 0x831a8168
	ctx.lr = 0x822EC3A0;
	sub_831A8130(ctx, base);
	// 822EC3A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822EC3A4: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 822EC3A8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822EC3AC: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 822EC3B0: 3B9F0090  addi r28, r31, 0x90
	ctx.r[28].s64 = ctx.r[31].s64 + 144;
	// 822EC3B4: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 822EC3B8: 817D0038  lwz r11, 0x38(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(56 as u32) ) } as u64;
	// 822EC3BC: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 822EC3C0: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 822EC3C4: 48632B05  bl 0x8291eec8
	ctx.lr = 0x822EC3C8;
	sub_8291EEC8(ctx, base);
	// 822EC3C8: 815F0094  lwz r10, 0x94(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 822EC3CC: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 822EC3D0: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 822EC3D4: 419A0024  beq cr6, 0x822ec3f8
	if ctx.cr[6].eq {
	pc = 0x822EC3F8; continue 'dispatch;
	}
	// 822EC3D8: 815F0088  lwz r10, 0x88(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(136 as u32) ) } as u64;
	// 822EC3DC: 812B0010  lwz r9, 0x10(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 822EC3E0: 7F095040  cmplw cr6, r9, r10
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[10].u32, &mut ctx.xer);
	// 822EC3E4: 419A0014  beq cr6, 0x822ec3f8
	if ctx.cr[6].eq {
	pc = 0x822EC3F8; continue 'dispatch;
	}
	// 822EC3E8: 389F0084  addi r4, r31, 0x84
	ctx.r[4].s64 = ctx.r[31].s64 + 132;
	// 822EC3EC: 5525003E  slwi r5, r9, 0
	ctx.r[5].u32 = ctx.r[9].u32.wrapping_shl(0);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 822EC3F0: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 822EC3F4: 4BFFEF3D  bl 0x822eb330
	ctx.lr = 0x822EC3F8;
	sub_822EB330(ctx, base);
	// 822EC3F8: 817F00A0  lwz r11, 0xa0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(160 as u32) ) } as u64;
	// 822EC3FC: 815F00A4  lwz r10, 0xa4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 822EC400: 48000014  b 0x822ec414
	pc = 0x822EC414; continue 'dispatch;
	// 822EC404: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 822EC408: 7F09E840  cmplw cr6, r9, r29
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[29].u32, &mut ctx.xer);
	// 822EC40C: 419A0010  beq cr6, 0x822ec41c
	if ctx.cr[6].eq {
	pc = 0x822EC41C; continue 'dispatch;
	}
	// 822EC410: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 822EC414: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 822EC418: 409AFFEC  bne cr6, 0x822ec404
	if !ctx.cr[6].eq {
	pc = 0x822EC404; continue 'dispatch;
	}
	// 822EC41C: 7D655B78  mr r5, r11
	ctx.r[5].u64 = ctx.r[11].u64;
	// 822EC420: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 822EC424: 419A0044  beq cr6, 0x822ec468
	if ctx.cr[6].eq {
	pc = 0x822EC468; continue 'dispatch;
	}
	// 822EC428: 3BDF009C  addi r30, r31, 0x9c
	ctx.r[30].s64 = ctx.r[31].s64 + 156;
	// 822EC42C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 822EC430: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 822EC434: 482BB705  bl 0x825a7b38
	ctx.lr = 0x822EC438;
	sub_825A7B38(ctx, base);
	// 822EC438: 817F00A0  lwz r11, 0xa0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(160 as u32) ) } as u64;
	// 822EC43C: 815F00A4  lwz r10, 0xa4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 822EC440: 48000014  b 0x822ec454
	pc = 0x822EC454; continue 'dispatch;
	// 822EC444: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 822EC448: 7F09E840  cmplw cr6, r9, r29
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[29].u32, &mut ctx.xer);
	// 822EC44C: 419A0010  beq cr6, 0x822ec45c
	if ctx.cr[6].eq {
	pc = 0x822EC45C; continue 'dispatch;
	}
	// 822EC450: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 822EC454: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 822EC458: 409AFFEC  bne cr6, 0x822ec444
	if !ctx.cr[6].eq {
	pc = 0x822EC444; continue 'dispatch;
	}
	// 822EC45C: 7D655B78  mr r5, r11
	ctx.r[5].u64 = ctx.r[11].u64;
	// 822EC460: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 822EC464: 409AFFC8  bne cr6, 0x822ec42c
	if !ctx.cr[6].eq {
	pc = 0x822EC42C; continue 'dispatch;
	}
	// 822EC468: 817D0038  lwz r11, 0x38(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(56 as u32) ) } as u64;
	// 822EC46C: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 822EC470: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 822EC474: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 822EC478: 4BFFFEB9  bl 0x822ec330
	ctx.lr = 0x822EC47C;
	sub_822EC330(ctx, base);
	// 822EC47C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 822EC480: 48EBBD38  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822EC488(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822EC488 size=112
    let mut pc: u32 = 0x822EC488;
    'dispatch: loop {
        match pc {
            0x822EC488 => {
    //   block [0x822EC488..0x822EC4F8)
	// 822EC488: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822EC48C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822EC490: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822EC494: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822EC498: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822EC49C: 897F00E1  lbz r11, 0xe1(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(225 as u32) ) } as u64;
	// 822EC4A0: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 822EC4A4: 41820010  beq 0x822ec4b4
	if ctx.cr[0].eq {
	pc = 0x822EC4B4; continue 'dispatch;
	}
	// 822EC4A8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 822EC4AC: 807F0144  lwz r3, 0x144(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(324 as u32) ) } as u64;
	// 822EC4B0: 48B0D511  bl 0x82df99c0
	ctx.lr = 0x822EC4B4;
	sub_82DF99C0(ctx, base);
	// 822EC4B4: 897F00E2  lbz r11, 0xe2(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(226 as u32) ) } as u64;
	// 822EC4B8: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 822EC4BC: 41820010  beq 0x822ec4cc
	if ctx.cr[0].eq {
	pc = 0x822EC4CC; continue 'dispatch;
	}
	// 822EC4C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 822EC4C4: 997F00E2  stb r11, 0xe2(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(226 as u32), ctx.r[11].u8 ) };
	// 822EC4C8: 48B05719  bl 0x82df1be0
	ctx.lr = 0x822EC4CC;
	sub_82DF1BE0(ctx, base);
	// 822EC4CC: 387F00AC  addi r3, r31, 0xac
	ctx.r[3].s64 = ctx.r[31].s64 + 172;
	// 822EC4D0: 4BFDA6D9  bl 0x822c6ba8
	ctx.lr = 0x822EC4D4;
	sub_822C6BA8(ctx, base);
	// 822EC4D4: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 822EC4D8: 816A1760  lwz r11, 0x1760(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(5984 as u32) ) } as u64;
	// 822EC4DC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 822EC4E0: 916A1760  stw r11, 0x1760(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(5984 as u32), ctx.r[11].u32 ) };
	// 822EC4E4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 822EC4E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822EC4EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822EC4F0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822EC4F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822EC4F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822EC4F8 size=132
    let mut pc: u32 = 0x822EC4F8;
    'dispatch: loop {
        match pc {
            0x822EC4F8 => {
    //   block [0x822EC4F8..0x822EC57C)
	// 822EC4F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822EC4FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822EC500: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 822EC504: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822EC508: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822EC50C: 81640038  lwz r11, 0x38(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(56 as u32) ) } as u64;
	// 822EC510: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822EC514: 9081008C  stw r4, 0x8c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(140 as u32), ctx.r[4].u32 ) };
	// 822EC518: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 822EC51C: 389F0090  addi r4, r31, 0x90
	ctx.r[4].s64 = ctx.r[31].s64 + 144;
	// 822EC520: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 822EC524: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 822EC528: 486329A1  bl 0x8291eec8
	ctx.lr = 0x822EC52C;
	sub_8291EEC8(ctx, base);
	// 822EC52C: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 822EC530: 83C10054  lwz r30, 0x54(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 822EC534: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 822EC538: 419A002C  beq cr6, 0x822ec564
	if ctx.cr[6].eq {
	pc = 0x822EC564; continue 'dispatch;
	}
	// 822EC53C: 80BF0088  lwz r5, 0x88(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(136 as u32) ) } as u64;
	// 822EC540: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 822EC544: 7F0B2840  cmplw cr6, r11, r5
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[5].u32, &mut ctx.xer);
	// 822EC548: 409A001C  bne cr6, 0x822ec564
	if !ctx.cr[6].eq {
	pc = 0x822EC564; continue 'dispatch;
	}
	// 822EC54C: 38C1008C  addi r6, r1, 0x8c
	ctx.r[6].s64 = ctx.r[1].s64 + 140;
	// 822EC550: 389F0084  addi r4, r31, 0x84
	ctx.r[4].s64 = ctx.r[31].s64 + 132;
	// 822EC554: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 822EC558: 4BFFFD81  bl 0x822ec2d8
	ctx.lr = 0x822EC55C;
	sub_822EC2D8(ctx, base);
	// 822EC55C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 822EC560: 917E0010  stw r11, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 822EC564: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 822EC568: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822EC56C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822EC570: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 822EC574: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822EC578: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822EC580(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822EC580 size=300
    let mut pc: u32 = 0x822EC580;
    'dispatch: loop {
        match pc {
            0x822EC580 => {
    //   block [0x822EC580..0x822EC6AC)
	// 822EC580: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822EC584: 48EBBBE9  bl 0x831a816c
	ctx.lr = 0x822EC588;
	sub_831A8130(ctx, base);
	// 822EC588: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822EC58C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822EC590: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 822EC594: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 822EC598: 419A0008  beq cr6, 0x822ec5a0
	if ctx.cr[6].eq {
	pc = 0x822EC5A0; continue 'dispatch;
	}
	// 822EC59C: 48C61A0D  bl 0x82f4dfa8
	ctx.lr = 0x822EC5A0;
	sub_82F4DFA8(ctx, base);
	// 822EC5A0: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 822EC5A4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 822EC5A8: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 822EC5AC: 808B1790  lwz r4, 0x1790(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(6032 as u32) ) } as u64;
	// 822EC5B0: 48B05649  bl 0x82df1bf8
	ctx.lr = 0x822EC5B4;
	sub_82DF1BF8(ctx, base);
	// 822EC5B4: 80610058  lwz r3, 0x58(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 822EC5B8: 4BFD3A49  bl 0x822c0000
	ctx.lr = 0x822EC5BC;
	sub_822C0000(ctx, base);
	// 822EC5BC: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 822EC5C0: 48B056D1  bl 0x82df1c90
	ctx.lr = 0x822EC5C4;
	sub_82DF1C90(ctx, base);
	// 822EC5C4: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 822EC5C8: 83DF0094  lwz r30, 0x94(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 822EC5CC: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 822EC5D0: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 822EC5D4: 48000018  b 0x822ec5ec
	pc = 0x822EC5EC; continue 'dispatch;
	// 822EC5D8: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 822EC5DC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 822EC5E0: 93AB0008  stw r29, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 822EC5E4: 480B50A5  bl 0x823a1688
	ctx.lr = 0x822EC5E8;
	sub_823A1688(ctx, base);
	// 822EC5E8: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 822EC5EC: 7F0BF040  cmplw cr6, r11, r30
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[30].u32, &mut ctx.xer);
	// 822EC5F0: 409AFFE8  bne cr6, 0x822ec5d8
	if !ctx.cr[6].eq {
	pc = 0x822EC5D8; continue 'dispatch;
	}
	// 822EC5F4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822EC5F8: 488C1701  bl 0x82badcf8
	ctx.lr = 0x822EC5FC;
	sub_82BADCF8(ctx, base);
	// 822EC5FC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 822EC600: 396B9EAC  addi r11, r11, -0x6154
	ctx.r[11].s64 = ctx.r[11].s64 + -24916;
	// 822EC604: 917F0150  stw r11, 0x150(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(336 as u32), ctx.r[11].u32 ) };
	// 822EC608: 807F0148  lwz r3, 0x148(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(328 as u32) ) } as u64;
	// 822EC60C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 822EC610: 419A0008  beq cr6, 0x822ec618
	if ctx.cr[6].eq {
	pc = 0x822EC618; continue 'dispatch;
	}
	// 822EC614: 4BFD427D  bl 0x822c0890
	ctx.lr = 0x822EC618;
	sub_822C0890(ctx, base);
	// 822EC618: 387F0134  addi r3, r31, 0x134
	ctx.r[3].s64 = ctx.r[31].s64 + 308;
	// 822EC61C: 4817D9E5  bl 0x8246a000
	ctx.lr = 0x822EC620;
	sub_8246A000(ctx, base);
	// 822EC620: 387F0124  addi r3, r31, 0x124
	ctx.r[3].s64 = ctx.r[31].s64 + 292;
	// 822EC624: 4817D9DD  bl 0x8246a000
	ctx.lr = 0x822EC628;
	sub_8246A000(ctx, base);
	// 822EC628: 387F0114  addi r3, r31, 0x114
	ctx.r[3].s64 = ctx.r[31].s64 + 276;
	// 822EC62C: 4817D9D5  bl 0x8246a000
	ctx.lr = 0x822EC630;
	sub_8246A000(ctx, base);
	// 822EC630: 387F0104  addi r3, r31, 0x104
	ctx.r[3].s64 = ctx.r[31].s64 + 260;
	// 822EC634: 4817D9CD  bl 0x8246a000
	ctx.lr = 0x822EC638;
	sub_8246A000(ctx, base);
	// 822EC638: 387F00AC  addi r3, r31, 0xac
	ctx.r[3].s64 = ctx.r[31].s64 + 172;
	// 822EC63C: 481C4C65  bl 0x824b12a0
	ctx.lr = 0x822EC640;
	sub_824B12A0(ctx, base);
	// 822EC640: 387F009C  addi r3, r31, 0x9c
	ctx.r[3].s64 = ctx.r[31].s64 + 156;
	// 822EC644: 4817D9BD  bl 0x8246a000
	ctx.lr = 0x822EC648;
	sub_8246A000(ctx, base);
	// 822EC648: 387F0090  addi r3, r31, 0x90
	ctx.r[3].s64 = ctx.r[31].s64 + 144;
	// 822EC64C: 4BFFF9BD  bl 0x822ec008
	ctx.lr = 0x822EC650;
	sub_822EC008(ctx, base);
	// 822EC650: 387F0084  addi r3, r31, 0x84
	ctx.r[3].s64 = ctx.r[31].s64 + 132;
	// 822EC654: 4813C02D  bl 0x82428680
	ctx.lr = 0x822EC658;
	sub_82428680(ctx, base);
	// 822EC658: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 822EC65C: 809F0088  lwz r4, 0x88(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(136 as u32) ) } as u64;
	// 822EC660: 806B110C  lwz r3, 0x110c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4364 as u32) ) } as u64;
	// 822EC664: 48B05B25  bl 0x82df2188
	ctx.lr = 0x822EC668;
	sub_82DF2188(ctx, base);
	// 822EC668: 93BF0088  stw r29, 0x88(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(136 as u32), ctx.r[29].u32 ) };
	// 822EC66C: 387F0080  addi r3, r31, 0x80
	ctx.r[3].s64 = ctx.r[31].s64 + 128;
	// 822EC670: 48BB5FB9  bl 0x82ea2628
	ctx.lr = 0x822EC674;
	sub_82EA2628(ctx, base);
	// 822EC674: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 822EC678: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 822EC67C: 419A000C  beq cr6, 0x822ec688
	if ctx.cr[6].eq {
	pc = 0x822EC688; continue 'dispatch;
	}
	// 822EC680: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 822EC684: 4BFFE52D  bl 0x822eabb0
	ctx.lr = 0x822EC688;
	sub_822EABB0(ctx, base);
	// 822EC688: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 822EC68C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 822EC690: 419A000C  beq cr6, 0x822ec69c
	if ctx.cr[6].eq {
	pc = 0x822EC69C; continue 'dispatch;
	}
	// 822EC694: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 822EC698: 4BFFE481  bl 0x822eab18
	ctx.lr = 0x822EC69C;
	sub_822EAB18(ctx, base);
	// 822EC69C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822EC6A0: 488C1659  bl 0x82badcf8
	ctx.lr = 0x822EC6A4;
	sub_82BADCF8(ctx, base);
	// 822EC6A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 822EC6A8: 48EBBB14  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822EC6B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822EC6B0 size=124
    let mut pc: u32 = 0x822EC6B0;
    'dispatch: loop {
        match pc {
            0x822EC6B0 => {
    //   block [0x822EC6B0..0x822EC72C)
	// 822EC6B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822EC6B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822EC6B8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 822EC6BC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822EC6C0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822EC6C4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 822EC6C8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 822EC6CC: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 822EC6D0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 822EC6D4: 419A0040  beq cr6, 0x822ec714
	if ctx.cr[6].eq {
	pc = 0x822EC714; continue 'dispatch;
	}
	// 822EC6D8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 822EC6DC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 822EC6E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822EC6E4: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 822EC6E8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 822EC6EC: 4E800421  bctrl
	ctx.lr = 0x822EC6F0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 822EC6F0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 822EC6F4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 822EC6F8: 4BFFDF01  bl 0x822ea5f8
	ctx.lr = 0x822EC6FC;
	sub_822EA5F8(ctx, base);
	// 822EC6FC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 822EC700: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 822EC704: 4BFFFC95  bl 0x822ec398
	ctx.lr = 0x822EC708;
	sub_822EC398(ctx, base);
	// 822EC708: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 822EC70C: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 822EC710: 917F0038  stw r11, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 822EC714: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 822EC718: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822EC71C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822EC720: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 822EC724: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822EC728: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822EC730(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822EC730 size=272
    let mut pc: u32 = 0x822EC730;
    'dispatch: loop {
        match pc {
            0x822EC730 => {
    //   block [0x822EC730..0x822EC840)
	// 822EC730: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822EC734: 48EBBA39  bl 0x831a816c
	ctx.lr = 0x822EC738;
	sub_831A8130(ctx, base);
	// 822EC738: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822EC73C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 822EC740: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 822EC744: 93E1009C  stw r31, 0x9c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(156 as u32), ctx.r[31].u32 ) };
	// 822EC748: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 822EC74C: 419A00EC  beq cr6, 0x822ec838
	if ctx.cr[6].eq {
	pc = 0x822EC838; continue 'dispatch;
	}
	// 822EC750: 897F00D8  lbz r11, 0xd8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(216 as u32) ) } as u64;
	// 822EC754: 2B0B0007  cmplwi cr6, r11, 7
	ctx.cr[6].compare_u32(ctx.r[11].u32, 7 as u32, &mut ctx.xer);
	// 822EC758: 419A0064  beq cr6, 0x822ec7bc
	if ctx.cr[6].eq {
	pc = 0x822EC7BC; continue 'dispatch;
	}
	// 822EC75C: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822EC840(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822EC840 size=272
    let mut pc: u32 = 0x822EC840;
    'dispatch: loop {
        match pc {
            0x822EC840 => {
    //   block [0x822EC840..0x822EC950)
	// 822EC840: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822EC844: 48EBB929  bl 0x831a816c
	ctx.lr = 0x822EC848;
	sub_831A8130(ctx, base);
	// 822EC848: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822EC84C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 822EC850: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822EC854: 93A1009C  stw r29, 0x9c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(156 as u32), ctx.r[29].u32 ) };
	// 822EC858: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 822EC85C: 419A00EC  beq cr6, 0x822ec948
	if ctx.cr[6].eq {
	pc = 0x822EC948; continue 'dispatch;
	}
	// 822EC860: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 822EC864: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 822EC868: 409A0080  bne cr6, 0x822ec8e8
	if !ctx.cr[6].eq {
	pc = 0x822EC8E8; continue 'dispatch;
	}
	// 822EC86C: 817F0108  lwz r11, 0x108(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(264 as u32) ) } as u64;
	// 822EC870: 815F010C  lwz r10, 0x10c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(268 as u32) ) } as u64;
	// 822EC874: 48000014  b 0x822ec888
	pc = 0x822EC888; continue 'dispatch;
	// 822EC878: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 822EC87C: 7F09E840  cmplw cr6, r9, r29
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[29].u32, &mut ctx.xer);
	// 822EC880: 419A0010  beq cr6, 0x822ec890
	if ctx.cr[6].eq {
	pc = 0x822EC890; continue 'dispatch;
	}
	// 822EC884: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 822EC888: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 822EC88C: 409AFFEC  bne cr6, 0x822ec878
	if !ctx.cr[6].eq {
	pc = 0x822EC878; continue 'dispatch;
	}
	// 822EC890: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 822EC894: 419A0018  beq cr6, 0x822ec8ac
	if ctx.cr[6].eq {
	pc = 0x822EC8AC; continue 'dispatch;
	}
	// 822EC898: 3881009C  addi r4, r1, 0x9c
	ctx.r[4].s64 = ctx.r[1].s64 + 156;
	// 822EC89C: 387F0114  addi r3, r31, 0x114
	ctx.r[3].s64 = ctx.r[31].s64 + 276;
	// 822EC8A0: 481CC871  bl 0x824b9110
	ctx.lr = 0x822EC8A4;
	sub_824B9110(ctx, base);
	// 822EC8A4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 822EC8A8: 48BE8841  bl 0x82ed50e8
	ctx.lr = 0x822EC8AC;
	sub_82ED50E8(ctx, base);
	// 822EC8AC: 817F0128  lwz r11, 0x128(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(296 as u32) ) } as u64;
	// 822EC8B0: 815F012C  lwz r10, 0x12c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(300 as u32) ) } as u64;
	// 822EC8B4: 48000014  b 0x822ec8c8
	pc = 0x822EC8C8; continue 'dispatch;
	// 822EC8B8: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 822EC8BC: 7F09E840  cmplw cr6, r9, r29
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[29].u32, &mut ctx.xer);
	// 822EC8C0: 419A0010  beq cr6, 0x822ec8d0
	if ctx.cr[6].eq {
	pc = 0x822EC8D0; continue 'dispatch;
	}
	// 822EC8C4: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 822EC8C8: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 822EC8CC: 409AFFEC  bne cr6, 0x822ec8b8
	if !ctx.cr[6].eq {
	pc = 0x822EC8B8; continue 'dispatch;
	}
	// 822EC8D0: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 822EC8D4: 419A0074  beq cr6, 0x822ec948
	if ctx.cr[6].eq {
	pc = 0x822EC948; continue 'dispatch;
	}
	// 822EC8D8: 3881009C  addi r4, r1, 0x9c
	ctx.r[4].s64 = ctx.r[1].s64 + 156;
	// 822EC8DC: 387F0114  addi r3, r31, 0x114
	ctx.r[3].s64 = ctx.r[31].s64 + 276;
	// 822EC8E0: 481CC831  bl 0x824b9110
	ctx.lr = 0x822EC8E4;
	sub_824B9110(ctx, base);
	// 822EC8E4: 48000024  b 0x822ec908
	pc = 0x822EC908; continue 'dispatch;
	// 822EC8E8: 817F0100  lwz r11, 0x100(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(256 as u32) ) } as u64;
	// 822EC8EC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 822EC8F0: 40990024  ble cr6, 0x822ec914
	if !ctx.cr[6].gt {
	pc = 0x822EC914; continue 'dispatch;
	}
	// 822EC8F4: 3881009C  addi r4, r1, 0x9c
	ctx.r[4].s64 = ctx.r[1].s64 + 156;
	// 822EC8F8: 387F0114  addi r3, r31, 0x114
	ctx.r[3].s64 = ctx.r[31].s64 + 276;
	// 822EC8FC: 481CC815  bl 0x824b9110
	ctx.lr = 0x822EC900;
	sub_824B9110(ctx, base);
	// 822EC900: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 822EC904: 917D000C  stw r11, 0xc(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 822EC908: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 822EC90C: 48BE87DD  bl 0x82ed50e8
	ctx.lr = 0x822EC910;
	sub_82ED50E8(ctx, base);
	// 822EC910: 48000038  b 0x822ec948
	pc = 0x822EC948; continue 'dispatch;
	// 822EC914: 83DF0000  lwz r30, 0(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 822EC918: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 822EC91C: 419A000C  beq cr6, 0x822ec928
	if ctx.cr[6].eq {
	pc = 0x822EC928; continue 'dispatch;
	}
	// 822EC920: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 822EC924: 48BE1EDD  bl 0x82ece800
	ctx.lr = 0x822EC928;
	sub_82ECE800(ctx, base);
	// 822EC928: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 822EC92C: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 822EC930: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 822EC934: 48BE0B0D  bl 0x82ecd440
	ctx.lr = 0x822EC938;
	sub_82ECD440(ctx, base);
	// 822EC938: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 822EC93C: 419A000C  beq cr6, 0x822ec948
	if ctx.cr[6].eq {
	pc = 0x822EC948; continue 'dispatch;
	}
	// 822EC940: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 822EC944: 48BE06C5  bl 0x82ecd008
	ctx.lr = 0x822EC948;
	sub_82ECD008(ctx, base);
	// 822EC948: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 822EC94C: 48EBB870  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822EC950(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822EC950 size=44
    let mut pc: u32 = 0x822EC950;
    'dispatch: loop {
        match pc {
            0x822EC950 => {
    //   block [0x822EC950..0x822EC97C)
	// 822EC950: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822EC954: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822EC958: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822EC95C: 9081007C  stw r4, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[4].u32 ) };
	// 822EC960: 3881007C  addi r4, r1, 0x7c
	ctx.r[4].s64 = ctx.r[1].s64 + 124;
	// 822EC964: 3863009C  addi r3, r3, 0x9c
	ctx.r[3].s64 = ctx.r[3].s64 + 156;
	// 822EC968: 481CC7A9  bl 0x824b9110
	ctx.lr = 0x822EC96C;
	sub_824B9110(ctx, base);
	// 822EC96C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 822EC970: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822EC974: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822EC978: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822EC980(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x822EC980 size=1204
    let mut pc: u32 = 0x822EC980;
    'dispatch: loop {
        match pc {
            0x822EC980 => {
    //   block [0x822EC980..0x822ECE34)
	// 822EC980: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822EC984: 48EBB7C5  bl 0x831a8148
	ctx.lr = 0x822EC988;
	sub_831A8130(ctx, base);
	// 822EC988: 9421FEF0  stwu r1, -0x110(r1)
	ea = ctx.r[1].u32.wrapping_add(-272 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822EC98C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822EC990: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 822EC994: 7C942378  mr r20, r4
	ctx.r[20].u64 = ctx.r[4].u64;
	// 822EC998: 7CB92B78  mr r25, r5
	ctx.r[25].u64 = ctx.r[5].u64;
	// 822EC99C: 7FDBF378  mr r27, r30
	ctx.r[27].u64 = ctx.r[30].u64;
	// 822EC9A0: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 822EC9A4: 387F0080  addi r3, r31, 0x80
	ctx.r[3].s64 = ctx.r[31].s64 + 128;
	// 822EC9A8: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 822EC9AC: 38A003E8  li r5, 0x3e8
	ctx.r[5].s64 = 1000;
	// 822EC9B0: 93DF0008  stw r30, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 822EC9B4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 822EC9B8: 7CD83378  mr r24, r6
	ctx.r[24].u64 = ctx.r[6].u64;
	// 822EC9BC: 93610050  stw r27, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[27].u32 ) };
	// 822EC9C0: 7CF73B78  mr r23, r7
	ctx.r[23].u64 = ctx.r[7].u64;
	// 822EC9C4: 48BB5C25  bl 0x82ea25e8
	ctx.lr = 0x822EC9C8;
	sub_82EA25E8(ctx, base);
	// 822EC9C8: 387F0084  addi r3, r31, 0x84
	ctx.r[3].s64 = ctx.r[31].s64 + 132;
	// 822EC9CC: 48179B0D  bl 0x824664d8
	ctx.lr = 0x822EC9D0;
	sub_824664D8(ctx, base);
	// 822EC9D0: 907F0088  stw r3, 0x88(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(136 as u32), ctx.r[3].u32 ) };
	// 822EC9D4: 93DF008C  stw r30, 0x8c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(140 as u32), ctx.r[30].u32 ) };
	// 822EC9D8: 387F0090  addi r3, r31, 0x90
	ctx.r[3].s64 = ctx.r[31].s64 + 144;
	// 822EC9DC: 484AF21D  bl 0x8279bbf8
	ctx.lr = 0x822EC9E0;
	sub_8279BBF8(ctx, base);
	// 822EC9E0: 93DF00A0  stw r30, 0xa0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(160 as u32), ctx.r[30].u32 ) };
	// 822EC9E4: 93DF00A4  stw r30, 0xa4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(164 as u32), ctx.r[30].u32 ) };
	// 822EC9E8: 396000C0  li r11, 0xc0
	ctx.r[11].s64 = 192;
	// 822EC9EC: 93DF00A8  stw r30, 0xa8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(168 as u32), ctx.r[30].u32 ) };
	// 822EC9F0: 394000D0  li r10, 0xd0
	ctx.r[10].s64 = 208;
	// 822EC9F4: 93DF00B0  stw r30, 0xb0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(176 as u32), ctx.r[30].u32 ) };
	// 822EC9F8: 392000F0  li r9, 0xf0
	ctx.r[9].s64 = 240;
	// 822EC9FC: 93DF00B4  stw r30, 0xb4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(180 as u32), ctx.r[30].u32 ) };
	// 822ECA00: 3B5F0150  addi r26, r31, 0x150
	ctx.r[26].s64 = ctx.r[31].s64 + 336;
	// 822ECA04: 93DF00B8  stw r30, 0xb8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(184 as u32), ctx.r[30].u32 ) };
	// 822ECA08: 13E0C0C7  vcmpequd (lvx128) v31, v0, v24
	tmp.u32 = ctx.r[24].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822ECE38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x822ECE38 size=8
    let mut pc: u32 = 0x822ECE38;
    'dispatch: loop {
        match pc {
            0x822ECE38 => {
    //   block [0x822ECE38..0x822ECE40)
	// 822ECE38: 386300AC  addi r3, r3, 0xac
	ctx.r[3].s64 = ctx.r[3].s64 + 172;
	// 822ECE3C: 488C62CC  b 0x82bb3108
	sub_82BB3108(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822ECE40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x822ECE40 size=12
    let mut pc: u32 = 0x822ECE40;
    'dispatch: loop {
        match pc {
            0x822ECE40 => {
    //   block [0x822ECE40..0x822ECE4C)
	// 822ECE40: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 822ECE44: 38802001  li r4, 0x2001
	ctx.r[4].s64 = 8193;
	// 822ECE48: 481A26B0  b 0x8248f4f8
	sub_8248F4F8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822ECE50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x822ECE50 size=8
    let mut pc: u32 = 0x822ECE50;
    'dispatch: loop {
        match pc {
            0x822ECE50 => {
    //   block [0x822ECE50..0x822ECE58)
	// 822ECE50: 38802002  li r4, 0x2002
	ctx.r[4].s64 = 8194;
	// 822ECE54: 481A2714  b 0x8248f568
	sub_8248F568(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822ECE58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822ECE58 size=80
    let mut pc: u32 = 0x822ECE58;
    'dispatch: loop {
        match pc {
            0x822ECE58 => {
    //   block [0x822ECE58..0x822ECEA8)
	// 822ECE58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822ECE5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822ECE60: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 822ECE64: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822ECE68: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822ECE6C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822ECE70: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 822ECE74: 48BEBDF5  bl 0x82ed8c68
	ctx.lr = 0x822ECE78;
	sub_82ED8C68(ctx, base);
	// 822ECE78: 817F00D0  lwz r11, 0xd0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(208 as u32) ) } as u64;
	// 822ECE7C: 387F00D0  addi r3, r31, 0xd0
	ctx.r[3].s64 = ctx.r[31].s64 + 208;
	// 822ECE80: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 822ECE84: 816B0048  lwz r11, 0x48(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(72 as u32) ) } as u64;
	// 822ECE88: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 822ECE8C: 4E800421  bctrl
	ctx.lr = 0x822ECE90;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 822ECE90: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 822ECE94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822ECE98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822ECE9C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 822ECEA0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822ECEA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822ECEA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822ECEA8 size=80
    let mut pc: u32 = 0x822ECEA8;
    'dispatch: loop {
        match pc {
            0x822ECEA8 => {
    //   block [0x822ECEA8..0x822ECEF8)
	// 822ECEA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822ECEAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822ECEB0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 822ECEB4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822ECEB8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822ECEBC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822ECEC0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 822ECEC4: 48BEBDA5  bl 0x82ed8c68
	ctx.lr = 0x822ECEC8;
	sub_82ED8C68(ctx, base);
	// 822ECEC8: 817F00D0  lwz r11, 0xd0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(208 as u32) ) } as u64;
	// 822ECECC: 387F00D0  addi r3, r31, 0xd0
	ctx.r[3].s64 = ctx.r[31].s64 + 208;
	// 822ECED0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 822ECED4: 816B0050  lwz r11, 0x50(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(80 as u32) ) } as u64;
	// 822ECED8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 822ECEDC: 4E800421  bctrl
	ctx.lr = 0x822ECEE0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 822ECEE0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 822ECEE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822ECEE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822ECEEC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 822ECEF0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822ECEF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822ECEF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822ECEF8 size=80
    let mut pc: u32 = 0x822ECEF8;
    'dispatch: loop {
        match pc {
            0x822ECEF8 => {
    //   block [0x822ECEF8..0x822ECF48)
	// 822ECEF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822ECEFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822ECF00: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 822ECF04: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822ECF08: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822ECF0C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822ECF10: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 822ECF14: 48BEBD55  bl 0x82ed8c68
	ctx.lr = 0x822ECF18;
	sub_82ED8C68(ctx, base);
	// 822ECF18: 817F00D0  lwz r11, 0xd0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(208 as u32) ) } as u64;
	// 822ECF1C: 387F00D0  addi r3, r31, 0xd0
	ctx.r[3].s64 = ctx.r[31].s64 + 208;
	// 822ECF20: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 822ECF24: 816B0044  lwz r11, 0x44(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(68 as u32) ) } as u64;
	// 822ECF28: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 822ECF2C: 4E800421  bctrl
	ctx.lr = 0x822ECF30;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 822ECF30: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 822ECF34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822ECF38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822ECF3C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 822ECF40: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822ECF44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822ECF48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x822ECF48 size=12
    let mut pc: u32 = 0x822ECF48;
    'dispatch: loop {
        match pc {
            0x822ECF48 => {
    //   block [0x822ECF48..0x822ECF54)
	// 822ECF48: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 822ECF4C: 386B00E0  addi r3, r11, 0xe0
	ctx.r[3].s64 = ctx.r[11].s64 + 224;
	// 822ECF50: 48BB3E40  b 0x82ea0d90
	sub_82EA0D90(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822ECF58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x822ECF58 size=20
    let mut pc: u32 = 0x822ECF58;
    'dispatch: loop {
        match pc {
            0x822ECF58 => {
    //   block [0x822ECF58..0x822ECF6C)
	// 822ECF58: 81640004  lwz r11, 4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 822ECF5C: 39400110  li r10, 0x110
	ctx.r[10].s64 = 272;
	// 822ECF60: 13EB50C7  vcmpequd (lvx128) v31, v11, v10
	tmp.u32 = ctx.r[11].u32 + ctx.r[10].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822ECF70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x822ECF70 size=40
    let mut pc: u32 = 0x822ECF70;
    'dispatch: loop {
        match pc {
            0x822ECF70 => {
    //   block [0x822ECF70..0x822ECF98)
	// 822ECF70: 81640004  lwz r11, 4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 822ECF74: C00B0150  lfs f0, 0x150(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(336 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 822ECF78: C1AB0154  lfs f13, 0x154(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(340 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 822ECF7C: C18B0158  lfs f12, 0x158(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(344 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 822ECF80: C16B015C  lfs f11, 0x15c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(348 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 822ECF84: D0030000  stfs f0, 0(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 822ECF88: D1A30004  stfs f13, 4(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 822ECF8C: D1830008  stfs f12, 8(r3)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 822ECF90: D163000C  stfs f11, 0xc(r3)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 822ECF94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822ECF98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x822ECF98 size=20
    let mut pc: u32 = 0x822ECF98;
    'dispatch: loop {
        match pc {
            0x822ECF98 => {
    //   block [0x822ECF98..0x822ECFAC)
	// 822ECF98: 81640004  lwz r11, 4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 822ECF9C: 394001A0  li r10, 0x1a0
	ctx.r[10].s64 = 416;
	// 822ECFA0: 13EB50C7  vcmpequd (lvx128) v31, v11, v10
	tmp.u32 = ctx.r[11].u32 + ctx.r[10].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822ECFB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x822ECFB0 size=20
    let mut pc: u32 = 0x822ECFB0;
    'dispatch: loop {
        match pc {
            0x822ECFB0 => {
    //   block [0x822ECFB0..0x822ECFC4)
	// 822ECFB0: 81640004  lwz r11, 4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 822ECFB4: 394001B0  li r10, 0x1b0
	ctx.r[10].s64 = 432;
	// 822ECFB8: 13EB50C7  vcmpequd (lvx128) v31, v11, v10
	tmp.u32 = ctx.r[11].u32 + ctx.r[10].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822ECFC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x822ECFC8 size=20
    let mut pc: u32 = 0x822ECFC8;
    'dispatch: loop {
        match pc {
            0x822ECFC8 => {
    //   block [0x822ECFC8..0x822ECFDC)
	// 822ECFC8: 81640004  lwz r11, 4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 822ECFCC: 39400130  li r10, 0x130
	ctx.r[10].s64 = 304;
	// 822ECFD0: 13EB50C7  vcmpequd (lvx128) v31, v11, v10
	tmp.u32 = ctx.r[11].u32 + ctx.r[10].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822ECFE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822ECFE0 size=80
    let mut pc: u32 = 0x822ECFE0;
    'dispatch: loop {
        match pc {
            0x822ECFE0 => {
    //   block [0x822ECFE0..0x822ED030)
	// 822ECFE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822ECFE4: 48EBB189  bl 0x831a816c
	ctx.lr = 0x822ECFE8;
	sub_831A8130(ctx, base);
	// 822ECFE8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822ECFEC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 822ECFF0: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 822ECFF4: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 822ECFF8: 83EB0008  lwz r31, 8(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 822ECFFC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 822ED000: 419A000C  beq cr6, 0x822ed00c
	if ctx.cr[6].eq {
	pc = 0x822ED00C; continue 'dispatch;
	}
	// 822ED004: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822ED008: 48BE17F9  bl 0x82ece800
	ctx.lr = 0x822ED00C;
	sub_82ECE800(ctx, base);
	// 822ED00C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 822ED010: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 822ED014: 4BFFFE45  bl 0x822ece58
	ctx.lr = 0x822ED018;
	sub_822ECE58(ctx, base);
	// 822ED018: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 822ED01C: 419A000C  beq cr6, 0x822ed028
	if ctx.cr[6].eq {
	pc = 0x822ED028; continue 'dispatch;
	}
	// 822ED020: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822ED024: 48BDFFE5  bl 0x82ecd008
	ctx.lr = 0x822ED028;
	sub_82ECD008(ctx, base);
	// 822ED028: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 822ED02C: 48EBB190  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822ED030(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822ED030 size=112
    let mut pc: u32 = 0x822ED030;
    'dispatch: loop {
        match pc {
            0x822ED030 => {
    //   block [0x822ED030..0x822ED0A0)
	// 822ED030: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822ED034: 48EBB135  bl 0x831a8168
	ctx.lr = 0x822ED038;
	sub_831A8130(ctx, base);
	// 822ED038: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822ED03C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822ED040: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 822ED044: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 822ED048: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 822ED04C: 83CB0008  lwz r30, 8(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 822ED050: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 822ED054: 419A000C  beq cr6, 0x822ed060
	if ctx.cr[6].eq {
	pc = 0x822ED060; continue 'dispatch;
	}
	// 822ED058: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 822ED05C: 48BE17A5  bl 0x82ece800
	ctx.lr = 0x822ED060;
	sub_82ECE800(ctx, base);
	// 822ED060: 83FF0004  lwz r31, 4(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 822ED064: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822ED068: 48BEBC01  bl 0x82ed8c68
	ctx.lr = 0x822ED06C;
	sub_82ED8C68(ctx, base);
	// 822ED06C: 817F00D0  lwz r11, 0xd0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(208 as u32) ) } as u64;
	// 822ED070: 387F00D0  addi r3, r31, 0xd0
	ctx.r[3].s64 = ctx.r[31].s64 + 208;
	// 822ED074: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 822ED078: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 822ED07C: 816B004C  lwz r11, 0x4c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(76 as u32) ) } as u64;
	// 822ED080: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 822ED084: 4E800421  bctrl
	ctx.lr = 0x822ED088;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 822ED088: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 822ED08C: 419A000C  beq cr6, 0x822ed098
	if ctx.cr[6].eq {
	pc = 0x822ED098; continue 'dispatch;
	}
	// 822ED090: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 822ED094: 48BDFF75  bl 0x82ecd008
	ctx.lr = 0x822ED098;
	sub_82ECD008(ctx, base);
	// 822ED098: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 822ED09C: 48EBB11C  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822ED0A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822ED0A0 size=80
    let mut pc: u32 = 0x822ED0A0;
    'dispatch: loop {
        match pc {
            0x822ED0A0 => {
    //   block [0x822ED0A0..0x822ED0F0)
	// 822ED0A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822ED0A4: 48EBB0C9  bl 0x831a816c
	ctx.lr = 0x822ED0A8;
	sub_831A8130(ctx, base);
	// 822ED0A8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822ED0AC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 822ED0B0: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 822ED0B4: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 822ED0B8: 83EB0008  lwz r31, 8(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 822ED0BC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 822ED0C0: 419A000C  beq cr6, 0x822ed0cc
	if ctx.cr[6].eq {
	pc = 0x822ED0CC; continue 'dispatch;
	}
	// 822ED0C4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822ED0C8: 48BE1739  bl 0x82ece800
	ctx.lr = 0x822ED0CC;
	sub_82ECE800(ctx, base);
	// 822ED0CC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 822ED0D0: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 822ED0D4: 4BFFFDD5  bl 0x822ecea8
	ctx.lr = 0x822ED0D8;
	sub_822ECEA8(ctx, base);
	// 822ED0D8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 822ED0DC: 419A000C  beq cr6, 0x822ed0e8
	if ctx.cr[6].eq {
	pc = 0x822ED0E8; continue 'dispatch;
	}
	// 822ED0E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822ED0E4: 48BDFF25  bl 0x82ecd008
	ctx.lr = 0x822ED0E8;
	sub_82ECD008(ctx, base);
	// 822ED0E8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 822ED0EC: 48EBB0D0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822ED0F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822ED0F0 size=96
    let mut pc: u32 = 0x822ED0F0;
    'dispatch: loop {
        match pc {
            0x822ED0F0 => {
    //   block [0x822ED0F0..0x822ED150)
	// 822ED0F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822ED0F4: 48EBB079  bl 0x831a816c
	ctx.lr = 0x822ED0F8;
	sub_831A8130(ctx, base);
	// 822ED0F8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822ED0FC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 822ED100: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 822ED104: 83EB0008  lwz r31, 8(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 822ED108: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 822ED10C: 419A000C  beq cr6, 0x822ed118
	if ctx.cr[6].eq {
	pc = 0x822ED118; continue 'dispatch;
	}
	// 822ED110: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822ED114: 48BE16ED  bl 0x82ece800
	ctx.lr = 0x822ED118;
	sub_82ECE800(ctx, base);
	// 822ED118: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 822ED11C: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 822ED120: 3BAB6910  addi r29, r11, 0x6910
	ctx.r[29].s64 = ctx.r[11].s64 + 26896;
	// 822ED124: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 822ED128: 4BFFC699  bl 0x822e97c0
	ctx.lr = 0x822ED12C;
	sub_822E97C0(ctx, base);
	// 822ED12C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 822ED130: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 822ED134: 4BFFFDC5  bl 0x822ecef8
	ctx.lr = 0x822ED138;
	sub_822ECEF8(ctx, base);
	// 822ED138: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 822ED13C: 419A000C  beq cr6, 0x822ed148
	if ctx.cr[6].eq {
	pc = 0x822ED148; continue 'dispatch;
	}
	// 822ED140: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822ED144: 48BDFEC5  bl 0x82ecd008
	ctx.lr = 0x822ED148;
	sub_82ECD008(ctx, base);
	// 822ED148: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 822ED14C: 48EBB070  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822ED150(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822ED150 size=80
    let mut pc: u32 = 0x822ED150;
    'dispatch: loop {
        match pc {
            0x822ED150 => {
    //   block [0x822ED150..0x822ED1A0)
	// 822ED150: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822ED154: 48EBB019  bl 0x831a816c
	ctx.lr = 0x822ED158;
	sub_831A8130(ctx, base);
	// 822ED158: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822ED15C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 822ED160: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 822ED164: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 822ED168: 83EB0008  lwz r31, 8(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 822ED16C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 822ED170: 419A000C  beq cr6, 0x822ed17c
	if ctx.cr[6].eq {
	pc = 0x822ED17C; continue 'dispatch;
	}
	// 822ED174: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822ED178: 48BE1689  bl 0x82ece800
	ctx.lr = 0x822ED17C;
	sub_82ECE800(ctx, base);
	// 822ED17C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 822ED180: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 822ED184: 4BFFC63D  bl 0x822e97c0
	ctx.lr = 0x822ED188;
	sub_822E97C0(ctx, base);
	// 822ED188: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 822ED18C: 419A000C  beq cr6, 0x822ed198
	if ctx.cr[6].eq {
	pc = 0x822ED198; continue 'dispatch;
	}
	// 822ED190: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822ED194: 48BDFE75  bl 0x82ecd008
	ctx.lr = 0x822ED198;
	sub_82ECD008(ctx, base);
	// 822ED198: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 822ED19C: 48EBB020  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822ED1A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x822ED1A0 size=12
    let mut pc: u32 = 0x822ED1A0;
    'dispatch: loop {
        match pc {
            0x822ED1A0 => {
    //   block [0x822ED1A0..0x822ED1AC)
	// 822ED1A0: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 822ED1A4: D02B0184  stfs f1, 0x184(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(388 as u32), tmp.u32 ) };
	// 822ED1A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822ED1B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x822ED1B0 size=12
    let mut pc: u32 = 0x822ED1B0;
    'dispatch: loop {
        match pc {
            0x822ED1B0 => {
    //   block [0x822ED1B0..0x822ED1BC)
	// 822ED1B0: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 822ED1B4: D02B0188  stfs f1, 0x188(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(392 as u32), tmp.u32 ) };
	// 822ED1B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822ED1C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x822ED1C0 size=8
    let mut pc: u32 = 0x822ED1C0;
    'dispatch: loop {
        match pc {
            0x822ED1C0 => {
    //   block [0x822ED1C0..0x822ED1C8)
	// 822ED1C0: 80630004  lwz r3, 4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 822ED1C4: 48BDE35C  b 0x82ecb520
	sub_82ECB520(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822ED1C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x822ED1C8 size=8
    let mut pc: u32 = 0x822ED1C8;
    'dispatch: loop {
        match pc {
            0x822ED1C8 => {
    //   block [0x822ED1C8..0x822ED1D0)
	// 822ED1C8: 80630004  lwz r3, 4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 822ED1CC: 48BDE35C  b 0x82ecb528
	sub_82ECB528(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822ED1D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x822ED1D0 size=296
    let mut pc: u32 = 0x822ED1D0;
    'dispatch: loop {
        match pc {
            0x822ED1D0 => {
    //   block [0x822ED1D0..0x822ED2F8)
	// 822ED1D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822ED1D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822ED1D8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 822ED1DC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822ED1E0: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 822ED1E4: 3980FFD0  li r12, -0x30
	ctx.r[12].s64 = -48;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822ED2F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x822ED2F8 size=80
    let mut pc: u32 = 0x822ED2F8;
    'dispatch: loop {
        match pc {
            0x822ED2F8 => {
    //   block [0x822ED2F8..0x822ED348)
	// 822ED2F8: 81640004  lwz r11, 4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 822ED2FC: 39200060  li r9, 0x60
	ctx.r[9].s64 = 96;
	// 822ED300: 13E028C7  vcmpequd (lvx128) v31, v0, v5
	tmp.u32 = ctx.r[5].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 822ED304: 390000D0  li r8, 0xd0
	ctx.r[8].s64 = 208;
	// 822ED308: 396B00D0  addi r11, r11, 0xd0
	ctx.r[11].s64 = ctx.r[11].s64 + 208;
	// 822ED30C: 394B00E0  addi r10, r11, 0xe0
	ctx.r[10].s64 = ctx.r[11].s64 + 224;
	// 822ED310: 13CB48C7  vcmpequd (lvx128) v30, v11, v9
	tmp.u32 = ctx.r[11].u32 + ctx.r[9].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[62] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822ED348(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822ED348 size=80
    let mut pc: u32 = 0x822ED348;
    'dispatch: loop {
        match pc {
            0x822ED348 => {
    //   block [0x822ED348..0x822ED398)
	// 822ED348: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822ED34C: 48EBAE21  bl 0x831a816c
	ctx.lr = 0x822ED350;
	sub_831A8130(ctx, base);
	// 822ED350: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822ED354: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 822ED358: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 822ED35C: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 822ED360: 83EB0008  lwz r31, 8(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 822ED364: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 822ED368: 419A000C  beq cr6, 0x822ed374
	if ctx.cr[6].eq {
	pc = 0x822ED374; continue 'dispatch;
	}
	// 822ED36C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822ED370: 48BE1491  bl 0x82ece800
	ctx.lr = 0x822ED374;
	sub_82ECE800(ctx, base);
	// 822ED374: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 822ED378: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 822ED37C: 4BFFFB7D  bl 0x822ecef8
	ctx.lr = 0x822ED380;
	sub_822ECEF8(ctx, base);
	// 822ED380: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 822ED384: 419A000C  beq cr6, 0x822ed390
	if ctx.cr[6].eq {
	pc = 0x822ED390; continue 'dispatch;
	}
	// 822ED388: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822ED38C: 48BDFC7D  bl 0x82ecd008
	ctx.lr = 0x822ED390;
	sub_82ECD008(ctx, base);
	// 822ED390: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 822ED394: 48EBAE28  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822ED398(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x822ED398 size=120
    let mut pc: u32 = 0x822ED398;
    'dispatch: loop {
        match pc {
            0x822ED398 => {
    //   block [0x822ED398..0x822ED410)
	// 822ED398: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822ED39C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822ED3A0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 822ED3A4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822ED3A8: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 822ED3AC: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822ED3B0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 822ED3B4: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 822ED3B8: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 822ED3BC: 83EB0008  lwz r31, 8(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 822ED3C0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 822ED3C4: 419A000C  beq cr6, 0x822ed3d0
	if ctx.cr[6].eq {
	pc = 0x822ED3D0; continue 'dispatch;
	}
	// 822ED3C8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822ED3CC: 48BE1435  bl 0x82ece800
	ctx.lr = 0x822ED3D0;
	sub_82ECE800(ctx, base);
	// 822ED3D0: D3E10050  stfs f31, 0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 822ED3D4: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 822ED3D8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 822ED3DC: 386B018C  addi r3, r11, 0x18c
	ctx.r[3].s64 = ctx.r[11].s64 + 396;
	// 822ED3E0: 48BBA671  bl 0x82ea7a50
	ctx.lr = 0x822ED3E4;
	sub_82EA7A50(ctx, base);
	// 822ED3E4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 822ED3E8: 419A000C  beq cr6, 0x822ed3f4
	if ctx.cr[6].eq {
	pc = 0x822ED3F4; continue 'dispatch;
	}
	// 822ED3EC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822ED3F0: 48BDFC19  bl 0x82ecd008
	ctx.lr = 0x822ED3F4;
	sub_82ECD008(ctx, base);
	// 822ED3F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 822ED3F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822ED3FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822ED400: CBE1FFE0  lfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 822ED404: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 822ED408: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822ED40C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822ED410(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x822ED410 size=120
    let mut pc: u32 = 0x822ED410;
    'dispatch: loop {
        match pc {
            0x822ED410 => {
    //   block [0x822ED410..0x822ED488)
	// 822ED410: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822ED414: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822ED418: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 822ED41C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822ED420: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 822ED424: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822ED428: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 822ED42C: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 822ED430: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 822ED434: 83EB0008  lwz r31, 8(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 822ED438: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 822ED43C: 419A000C  beq cr6, 0x822ed448
	if ctx.cr[6].eq {
	pc = 0x822ED448; continue 'dispatch;
	}
	// 822ED440: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822ED444: 48BE13BD  bl 0x82ece800
	ctx.lr = 0x822ED448;
	sub_82ECE800(ctx, base);
	// 822ED448: D3E10050  stfs f31, 0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 822ED44C: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 822ED450: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 822ED454: 386B018D  addi r3, r11, 0x18d
	ctx.r[3].s64 = ctx.r[11].s64 + 397;
	// 822ED458: 48BBA5F9  bl 0x82ea7a50
	ctx.lr = 0x822ED45C;
	sub_82EA7A50(ctx, base);
	// 822ED45C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 822ED460: 419A000C  beq cr6, 0x822ed46c
	if ctx.cr[6].eq {
	pc = 0x822ED46C; continue 'dispatch;
	}
	// 822ED464: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822ED468: 48BDFBA1  bl 0x82ecd008
	ctx.lr = 0x822ED46C;
	sub_82ECD008(ctx, base);
	// 822ED46C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 822ED470: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822ED474: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822ED478: CBE1FFE0  lfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 822ED47C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 822ED480: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822ED484: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822ED488(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x822ED488 size=12
    let mut pc: u32 = 0x822ED488;
    'dispatch: loop {
        match pc {
            0x822ED488 => {
    //   block [0x822ED488..0x822ED494)
	// 822ED488: 38830100  addi r4, r3, 0x100
	ctx.r[4].s64 = ctx.r[3].s64 + 256;
	// 822ED48C: 80630004  lwz r3, 4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 822ED490: 48BDE960  b 0x82ecbdf0
	sub_82ECBDF0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822ED498(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822ED498 size=84
    let mut pc: u32 = 0x822ED498;
    'dispatch: loop {
        match pc {
            0x822ED498 => {
    //   block [0x822ED498..0x822ED4EC)
	// 822ED498: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822ED49C: 48EBACD1  bl 0x831a816c
	ctx.lr = 0x822ED4A0;
	sub_831A8130(ctx, base);
	// 822ED4A0: 9421FF10  stwu r1, -0xf0(r1)
	ea = ctx.r[1].u32.wrapping_add(-240 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822ED4A4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822ED4A8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 822ED4AC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 822ED4B0: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 822ED4B4: 386B00E0  addi r3, r11, 0xe0
	ctx.r[3].s64 = ctx.r[11].s64 + 224;
	// 822ED4B8: 48BB38D9  bl 0x82ea0d90
	ctx.lr = 0x822ED4BC;
	sub_82EA0D90(ctx, base);
	// 822ED4BC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 822ED4C0: 38610090  addi r3, r1, 0x90
	ctx.r[3].s64 = ctx.r[1].s64 + 144;
	// 822ED4C4: 83BF0004  lwz r29, 4(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 822ED4C8: 4BFFB3C1  bl 0x822e8888
	ctx.lr = 0x822ED4CC;
	sub_822E8888(ctx, base);
	// 822ED4CC: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 822ED4D0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 822ED4D4: 48BDEBC5  bl 0x82ecc098
	ctx.lr = 0x822ED4D8;
	sub_82ECC098(ctx, base);
	// 822ED4D8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 822ED4DC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822ED4E0: 4BFF9611  bl 0x822e6af0
	ctx.lr = 0x822ED4E4;
	sub_822E6AF0(ctx, base);
	// 822ED4E4: 382100F0  addi r1, r1, 0xf0
	ctx.r[1].s64 = ctx.r[1].s64 + 240;
	// 822ED4E8: 48EBACD4  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822ED4F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822ED4F0 size=180
    let mut pc: u32 = 0x822ED4F0;
    'dispatch: loop {
        match pc {
            0x822ED4F0 => {
    //   block [0x822ED4F0..0x822ED5A4)
	// 822ED4F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822ED4F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822ED4F8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 822ED4FC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822ED500: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822ED504: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822ED508: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 822ED50C: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 822ED510: 396BC148  addi r11, r11, -0x3eb8
	ctx.r[11].s64 = ctx.r[11].s64 + -16056;
	// 822ED514: 394AC13C  addi r10, r10, -0x3ec4
	ctx.r[10].s64 = ctx.r[10].s64 + -16068;
	// 822ED518: 807F00F4  lwz r3, 0xf4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(244 as u32) ) } as u64;
	// 822ED51C: 3BDF0050  addi r30, r31, 0x50
	ctx.r[30].s64 = ctx.r[31].s64 + 80;
	// 822ED520: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 822ED524: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 822ED528: 915F0050  stw r10, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 822ED52C: 419A000C  beq cr6, 0x822ed538
	if ctx.cr[6].eq {
	pc = 0x822ED538; continue 'dispatch;
	}
	// 822ED530: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 822ED534: 48B2500D  bl 0x82e12540
	ctx.lr = 0x822ED538;
	sub_82E12540(ctx, base);
	// 822ED538: 807F00F8  lwz r3, 0xf8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(248 as u32) ) } as u64;
	// 822ED53C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 822ED540: 419A0008  beq cr6, 0x822ed548
	if ctx.cr[6].eq {
	pc = 0x822ED548; continue 'dispatch;
	}
	// 822ED544: 4BFD334D  bl 0x822c0890
	ctx.lr = 0x822ED548;
	sub_822C0890(ctx, base);
	// 822ED548: 807F00F0  lwz r3, 0xf0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(240 as u32) ) } as u64;
	// 822ED54C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 822ED550: 419A0008  beq cr6, 0x822ed558
	if ctx.cr[6].eq {
	pc = 0x822ED558; continue 'dispatch;
	}
	// 822ED554: 4BFD333D  bl 0x822c0890
	ctx.lr = 0x822ED558;
	sub_822C0890(ctx, base);
	// 822ED558: 807F00E8  lwz r3, 0xe8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(232 as u32) ) } as u64;
	// 822ED55C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 822ED560: 419A0008  beq cr6, 0x822ed568
	if ctx.cr[6].eq {
	pc = 0x822ED568; continue 'dispatch;
	}
	// 822ED564: 4BFD332D  bl 0x822c0890
	ctx.lr = 0x822ED568;
	sub_822C0890(ctx, base);
	// 822ED568: 807F00E0  lwz r3, 0xe0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(224 as u32) ) } as u64;
	// 822ED56C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 822ED570: 419A0008  beq cr6, 0x822ed578
	if ctx.cr[6].eq {
	pc = 0x822ED578; continue 'dispatch;
	}
	// 822ED574: 4BFFACF5  bl 0x822e8268
	ctx.lr = 0x822ED578;
	sub_822E8268(ctx, base);
	// 822ED578: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 822ED57C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822ED580: 396BA980  addi r11, r11, -0x5680
	ctx.r[11].s64 = ctx.r[11].s64 + -22144;
	// 822ED584: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 822ED588: 4BFF94D9  bl 0x822e6a60
	ctx.lr = 0x822ED58C;
	sub_822E6A60(ctx, base);
	// 822ED58C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 822ED590: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822ED594: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822ED598: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 822ED59C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822ED5A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822ED5A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x822ED5A8 size=8
    let mut pc: u32 = 0x822ED5A8;
    'dispatch: loop {
        match pc {
            0x822ED5A8 => {
    //   block [0x822ED5A8..0x822ED5B0)
	// 822ED5A8: 3863FFB0  addi r3, r3, -0x50
	ctx.r[3].s64 = ctx.r[3].s64 + -80;
	// 822ED5AC: 480008B4  b 0x822ede60
	sub_822EDE60(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822ED5B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x822ED5B0 size=132
    let mut pc: u32 = 0x822ED5B0;
    'dispatch: loop {
        match pc {
            0x822ED5B0 => {
    //   block [0x822ED5B0..0x822ED634)
	// 822ED5B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822ED5B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822ED5B8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 822ED5BC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822ED5C0: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822ED5C4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822ED5C8: 4BFF9781  bl 0x822e6d48
	ctx.lr = 0x822ED5CC;
	sub_822E6D48(ctx, base);
	// 822ED5CC: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 822ED5D0: 3BDF0060  addi r30, r31, 0x60
	ctx.r[30].s64 = ctx.r[31].s64 + 96;
	// 822ED5D4: 386B00E0  addi r3, r11, 0xe0
	ctx.r[3].s64 = ctx.r[11].s64 + 224;
	// 822ED5D8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 822ED5DC: 48BB37B5  bl 0x82ea0d90
	ctx.lr = 0x822ED5E0;
	sub_82EA0D90(ctx, base);
	// 822ED5E0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 822ED5E4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 822ED5E8: 3BFF00A0  addi r31, r31, 0xa0
	ctx.r[31].s64 = ctx.r[31].s64 + 160;
	// 822ED5EC: 48B8E47D  bl 0x82e7ba68
	ctx.lr = 0x822ED5F0;
	sub_82E7BA68(ctx, base);
	// 822ED5F0: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 822ED5F4: 39400020  li r10, 0x20
	ctx.r[10].s64 = 32;
	// 822ED5F8: 13E01C07  vcmpneb. (lvlx128) v31, v0, v3
	tmp.u32 = ctx.r[3].u32;
	// load shuffled into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 822ED5FC: 39600030  li r11, 0x30
	ctx.r[11].s64 = 48;
	// 822ED600: 13C91C07  vcmpneb. (lvlx128) v30, v9, v3
	tmp.u32 = ctx.r[9].u32 + ctx.r[3].u32;
	// load shuffled into ctx.v[62] using VectorMaskL[(tmp.u32 & 0xF)]
	// 822ED604: 13AA1C07  vcmpneb. (lvlx128) v29, v10, v3
	tmp.u32 = ctx.r[10].u32 + ctx.r[3].u32;
	// load shuffled into ctx.v[61] using VectorMaskL[(tmp.u32 & 0xF)]
	// 822ED608: 138B1C07  vcmpneb. (lvlx128) v28, v11, v3
	tmp.u32 = ctx.r[11].u32 + ctx.r[3].u32;
	// load shuffled into ctx.v[60] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822ED638(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x822ED638 size=124
    let mut pc: u32 = 0x822ED638;
    'dispatch: loop {
        match pc {
            0x822ED638 => {
    //   block [0x822ED638..0x822ED6B4)
	// 822ED638: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822ED63C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822ED640: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 822ED644: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822ED648: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822ED64C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822ED650: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 822ED654: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 822ED658: 386B00E0  addi r3, r11, 0xe0
	ctx.r[3].s64 = ctx.r[11].s64 + 224;
	// 822ED65C: 48BB3735  bl 0x82ea0d90
	ctx.lr = 0x822ED660;
	sub_82EA0D90(ctx, base);
	// 822ED660: 38BF00A0  addi r5, r31, 0xa0
	ctx.r[5].s64 = ctx.r[31].s64 + 160;
	// 822ED664: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 822ED668: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 822ED66C: 4BFD7295  bl 0x822c4900
	ctx.lr = 0x822ED670;
	sub_822C4900(ctx, base);
	// 822ED670: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 822ED674: 39400020  li r10, 0x20
	ctx.r[10].s64 = 32;
	// 822ED678: 13E01C07  vcmpneb. (lvlx128) v31, v0, v3
	tmp.u32 = ctx.r[3].u32;
	// load shuffled into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 822ED67C: 39600030  li r11, 0x30
	ctx.r[11].s64 = 48;
	// 822ED680: 13C91C07  vcmpneb. (lvlx128) v30, v9, v3
	tmp.u32 = ctx.r[9].u32 + ctx.r[3].u32;
	// load shuffled into ctx.v[62] using VectorMaskL[(tmp.u32 & 0xF)]
	// 822ED684: 13AA1C07  vcmpneb. (lvlx128) v29, v10, v3
	tmp.u32 = ctx.r[10].u32 + ctx.r[3].u32;
	// load shuffled into ctx.v[61] using VectorMaskL[(tmp.u32 & 0xF)]
	// 822ED688: 138B1C07  vcmpneb. (lvlx128) v28, v11, v3
	tmp.u32 = ctx.r[11].u32 + ctx.r[3].u32;
	// load shuffled into ctx.v[60] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822ED6B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822ED6B8 size=100
    let mut pc: u32 = 0x822ED6B8;
    'dispatch: loop {
        match pc {
            0x822ED6B8 => {
    //   block [0x822ED6B8..0x822ED71C)
	// 822ED6B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822ED6BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822ED6C0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 822ED6C4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822ED6C8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822ED6CC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 822ED6D0: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 822ED6D4: 83EB0008  lwz r31, 8(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 822ED6D8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 822ED6DC: 419A000C  beq cr6, 0x822ed6e8
	if ctx.cr[6].eq {
	pc = 0x822ED6E8; continue 'dispatch;
	}
	// 822ED6E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822ED6E4: 48BE111D  bl 0x82ece800
	ctx.lr = 0x822ED6E8;
	sub_82ECE800(ctx, base);
	// 822ED6E8: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 822ED6EC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 822ED6F0: 8BCB00D8  lbz r30, 0xd8(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(216 as u32) ) } as u64;
	// 822ED6F4: 419A000C  beq cr6, 0x822ed700
	if ctx.cr[6].eq {
	pc = 0x822ED700; continue 'dispatch;
	}
	// 822ED6F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822ED6FC: 48BDF90D  bl 0x82ecd008
	ctx.lr = 0x822ED700;
	sub_82ECD008(ctx, base);
	// 822ED700: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 822ED704: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 822ED708: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822ED70C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822ED710: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 822ED714: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822ED718: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822ED720(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822ED720 size=108
    let mut pc: u32 = 0x822ED720;
    'dispatch: loop {
        match pc {
            0x822ED720 => {
    //   block [0x822ED720..0x822ED78C)
	// 822ED720: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822ED724: 48EBAA49  bl 0x831a816c
	ctx.lr = 0x822ED728;
	sub_831A8130(ctx, base);
	// 822ED728: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822ED72C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822ED730: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 822ED734: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 822ED738: 83AB0008  lwz r29, 8(r11)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 822ED73C: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 822ED740: 419A000C  beq cr6, 0x822ed74c
	if ctx.cr[6].eq {
	pc = 0x822ED74C; continue 'dispatch;
	}
	// 822ED744: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 822ED748: 48BE10B9  bl 0x82ece800
	ctx.lr = 0x822ED74C;
	sub_82ECE800(ctx, base);
	// 822ED74C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 822ED750: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 822ED754: 48BDE86D  bl 0x82ecbfc0
	ctx.lr = 0x822ED758;
	sub_82ECBFC0(ctx, base);
	// 822ED758: 817F00F4  lwz r11, 0xf4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(244 as u32) ) } as u64;
	// 822ED75C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 822ED760: 409A0014  bne cr6, 0x822ed774
	if !ctx.cr[6].eq {
	pc = 0x822ED774; continue 'dispatch;
	}
	// 822ED764: 807F00E4  lwz r3, 0xe4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(228 as u32) ) } as u64;
	// 822ED768: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 822ED76C: 419A0008  beq cr6, 0x822ed774
	if ctx.cr[6].eq {
	pc = 0x822ED774; continue 'dispatch;
	}
	// 822ED770: 48B24A49  bl 0x82e121b8
	ctx.lr = 0x822ED774;
	sub_82E121B8(ctx, base);
	// 822ED774: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 822ED778: 419A000C  beq cr6, 0x822ed784
	if ctx.cr[6].eq {
	pc = 0x822ED784; continue 'dispatch;
	}
	// 822ED77C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 822ED780: 48BDF889  bl 0x82ecd008
	ctx.lr = 0x822ED784;
	sub_82ECD008(ctx, base);
	// 822ED784: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 822ED788: 48EBAA34  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822ED790(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x822ED790 size=168
    let mut pc: u32 = 0x822ED790;
    'dispatch: loop {
        match pc {
            0x822ED790 => {
    //   block [0x822ED790..0x822ED838)
	// 822ED790: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822ED794: 48EBA9D9  bl 0x831a816c
	ctx.lr = 0x822ED798;
	sub_831A8130(ctx, base);
	// 822ED798: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822ED79C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822ED7A0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 822ED7A4: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 822ED7A8: 83AB0008  lwz r29, 8(r11)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 822ED7AC: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 822ED7B0: 419A000C  beq cr6, 0x822ed7bc
	if ctx.cr[6].eq {
	pc = 0x822ED7BC; continue 'dispatch;
	}
	// 822ED7B4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 822ED7B8: 48BE1049  bl 0x82ece800
	ctx.lr = 0x822ED7BC;
	sub_82ECE800(ctx, base);
	// 822ED7BC: 13E0F0C7  vcmpequd (lvx128) v31, v0, v30
	tmp.u32 = ctx.r[30].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 822ED7C0: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822ED838(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x822ED838 size=176
    let mut pc: u32 = 0x822ED838;
    'dispatch: loop {
        match pc {
            0x822ED838 => {
    //   block [0x822ED838..0x822ED8E8)
	// 822ED838: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822ED83C: 48EBA92D  bl 0x831a8168
	ctx.lr = 0x822ED840;
	sub_831A8130(ctx, base);
	// 822ED840: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822ED844: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822ED848: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 822ED84C: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 822ED850: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 822ED854: 838B0008  lwz r28, 8(r11)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 822ED858: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 822ED85C: 419A000C  beq cr6, 0x822ed868
	if ctx.cr[6].eq {
	pc = 0x822ED868; continue 'dispatch;
	}
	// 822ED860: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 822ED864: 48BE0F9D  bl 0x82ece800
	ctx.lr = 0x822ED868;
	sub_82ECE800(ctx, base);
	// 822ED868: 13E0F0C7  vcmpequd (lvx128) v31, v0, v30
	tmp.u32 = ctx.r[30].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 822ED86C: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822ED8E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x822ED8E8 size=52
    let mut pc: u32 = 0x822ED8E8;
    'dispatch: loop {
        match pc {
            0x822ED8E8 => {
    //   block [0x822ED8E8..0x822ED91C)
	// 822ED8E8: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 822ED8EC: 896B00D8  lbz r11, 0xd8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(216 as u32) ) } as u64;
	// 822ED8F0: 2B0B0007  cmplwi cr6, r11, 7
	ctx.cr[6].compare_u32(ctx.r[11].u32, 7 as u32, &mut ctx.xer);
	// 822ED8F4: 419A0010  beq cr6, 0x822ed904
	if ctx.cr[6].eq {
	pc = 0x822ED904; continue 'dispatch;
	}
	// 822ED8F8: 2B0B0006  cmplwi cr6, r11, 6
	ctx.cr[6].compare_u32(ctx.r[11].u32, 6 as u32, &mut ctx.xer);
	// 822ED8FC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 822ED900: 409A0008  bne cr6, 0x822ed908
	if !ctx.cr[6].eq {
	pc = 0x822ED908; continue 'dispatch;
	}
	// 822ED904: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 822ED908: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 822ED90C: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 822ED910: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 822ED914: 69630001  xori r3, r11, 1
	ctx.r[3].u64 = ctx.r[11].u64 ^ 1;
	// 822ED918: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822ED920(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822ED920 size=124
    let mut pc: u32 = 0x822ED920;
    'dispatch: loop {
        match pc {
            0x822ED920 => {
    //   block [0x822ED920..0x822ED99C)
	// 822ED920: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822ED924: 48EBA849  bl 0x831a816c
	ctx.lr = 0x822ED928;
	sub_831A8130(ctx, base);
	// 822ED928: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822ED92C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822ED930: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 822ED934: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 822ED938: 83AB0008  lwz r29, 8(r11)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 822ED93C: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 822ED940: 419A000C  beq cr6, 0x822ed94c
	if ctx.cr[6].eq {
	pc = 0x822ED94C; continue 'dispatch;
	}
	// 822ED944: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 822ED948: 48BE0EB9  bl 0x82ece800
	ctx.lr = 0x822ED94C;
	sub_82ECE800(ctx, base);
	// 822ED94C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 822ED950: 83DF0004  lwz r30, 4(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 822ED954: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 822ED958: 4BFFAF31  bl 0x822e8888
	ctx.lr = 0x822ED95C;
	sub_822E8888(ctx, base);
	// 822ED95C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 822ED960: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 822ED964: 48BDE735  bl 0x82ecc098
	ctx.lr = 0x822ED968;
	sub_82ECC098(ctx, base);
	// 822ED968: 817F00F4  lwz r11, 0xf4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(244 as u32) ) } as u64;
	// 822ED96C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 822ED970: 409A0014  bne cr6, 0x822ed984
	if !ctx.cr[6].eq {
	pc = 0x822ED984; continue 'dispatch;
	}
	// 822ED974: 807F00E4  lwz r3, 0xe4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(228 as u32) ) } as u64;
	// 822ED978: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 822ED97C: 419A0008  beq cr6, 0x822ed984
	if ctx.cr[6].eq {
	pc = 0x822ED984; continue 'dispatch;
	}
	// 822ED980: 48B24839  bl 0x82e121b8
	ctx.lr = 0x822ED984;
	sub_82E121B8(ctx, base);
	// 822ED984: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 822ED988: 419A000C  beq cr6, 0x822ed994
	if ctx.cr[6].eq {
	pc = 0x822ED994; continue 'dispatch;
	}
	// 822ED98C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 822ED990: 48BDF679  bl 0x82ecd008
	ctx.lr = 0x822ED994;
	sub_82ECD008(ctx, base);
	// 822ED994: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 822ED998: 48EBA824  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822ED9A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822ED9A0 size=96
    let mut pc: u32 = 0x822ED9A0;
    'dispatch: loop {
        match pc {
            0x822ED9A0 => {
    //   block [0x822ED9A0..0x822EDA00)
	// 822ED9A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822ED9A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822ED9A8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 822ED9AC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822ED9B0: 9421FED0  stwu r1, -0x130(r1)
	ea = ctx.r[1].u32.wrapping_add(-304 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822ED9B4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822ED9B8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 822ED9BC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 822ED9C0: 48BDF029  bl 0x82ecc9e8
	ctx.lr = 0x822ED9C4;
	sub_82ECC9E8(ctx, base);
	// 822ED9C4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 822ED9C8: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 822ED9CC: 48BDEBBD  bl 0x82ecc588
	ctx.lr = 0x822ED9D0;
	sub_82ECC588(ctx, base);
	// 822ED9D0: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 822ED9D4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 822ED9D8: 907F0000  stw r3, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 822ED9DC: 419A0008  beq cr6, 0x822ed9e4
	if ctx.cr[6].eq {
	pc = 0x822ED9E4; continue 'dispatch;
	}
	// 822ED9E0: 4BFFA869  bl 0x822e8248
	ctx.lr = 0x822ED9E4;
	sub_822E8248(ctx, base);
	// 822ED9E4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822ED9E8: 38210130  addi r1, r1, 0x130
	ctx.r[1].s64 = ctx.r[1].s64 + 304;
	// 822ED9EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822ED9F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822ED9F4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 822ED9F8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822ED9FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822EDA00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x822EDA00 size=52
    let mut pc: u32 = 0x822EDA00;
    'dispatch: loop {
        match pc {
            0x822EDA00 => {
    //   block [0x822EDA00..0x822EDA34)
	// 822EDA00: 39630060  addi r11, r3, 0x60
	ctx.r[11].s64 = ctx.r[3].s64 + 96;
	// 822EDA04: 39000010  li r8, 0x10
	ctx.r[8].s64 = 16;
	// 822EDA08: 39200020  li r9, 0x20
	ctx.r[9].s64 = 32;
	// 822EDA0C: 39400030  li r10, 0x30
	ctx.r[10].s64 = 48;
	// 822EDA10: 13E05C07  vcmpneb. (lvlx128) v31, v0, v11
	tmp.u32 = ctx.r[11].u32;
	// load shuffled into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 822EDA14: 13C85C07  vcmpneb. (lvlx128) v30, v8, v11
	tmp.u32 = ctx.r[8].u32 + ctx.r[11].u32;
	// load shuffled into ctx.v[62] using VectorMaskL[(tmp.u32 & 0xF)]
	// 822EDA18: 13A95C07  vcmpneb. (lvlx128) v29, v9, v11
	tmp.u32 = ctx.r[9].u32 + ctx.r[11].u32;
	// load shuffled into ctx.v[61] using VectorMaskL[(tmp.u32 & 0xF)]
	// 822EDA1C: 138A5C07  vcmpneb. (lvlx128) v28, v10, v11
	tmp.u32 = ctx.r[10].u32 + ctx.r[11].u32;
	// load shuffled into ctx.v[60] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822EDA38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822EDA38 size=224
    let mut pc: u32 = 0x822EDA38;
    'dispatch: loop {
        match pc {
            0x822EDA38 => {
    //   block [0x822EDA38..0x822EDB18)
	// 822EDA38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822EDA3C: 48EBA72D  bl 0x831a8168
	ctx.lr = 0x822EDA40;
	sub_831A8130(ctx, base);
	// 822EDA40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822EDA44: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 822EDA48: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 822EDA4C: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 822EDA50: 838B0008  lwz r28, 8(r11)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 822EDA54: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 822EDA58: 419A000C  beq cr6, 0x822eda64
	if ctx.cr[6].eq {
	pc = 0x822EDA64; continue 'dispatch;
	}
	// 822EDA5C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 822EDA60: 48BE0DA1  bl 0x82ece800
	ctx.lr = 0x822EDA64;
	sub_82ECE800(ctx, base);
	// 822EDA64: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 822EDA68: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 822EDA6C: 57EA103A  slwi r10, r31, 2
	ctx.r[10].u32 = ctx.r[31].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 822EDA70: 396BC17C  addi r11, r11, -0x3e84
	ctx.r[11].s64 = ctx.r[11].s64 + -16004;
	// 822EDA74: 892300D8  lbz r9, 0xd8(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(216 as u32) ) } as u64;
	// 822EDA78: 7FAA582E  lwzx r29, r10, r11
	ctx.r[29].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 822EDA7C: 7F09E800  cmpw cr6, r9, r29
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[29].s32, &mut ctx.xer);
	// 822EDA80: 409A000C  bne cr6, 0x822eda8c
	if !ctx.cr[6].eq {
	pc = 0x822EDA8C; continue 'dispatch;
	}
	// 822EDA84: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 822EDA88: 48000024  b 0x822edaac
	pc = 0x822EDAAC; continue 'dispatch;
	// 822EDA8C: 2F1D0001  cmpwi cr6, r29, 1
	ctx.cr[6].compare_i32(ctx.r[29].s32, 1, &mut ctx.xer);
	// 822EDA90: 409A0018  bne cr6, 0x822edaa8
	if !ctx.cr[6].eq {
	pc = 0x822EDAA8; continue 'dispatch;
	}
	// 822EDA94: 48BDE3AD  bl 0x82ecbe40
	ctx.lr = 0x822EDA98;
	sub_82ECBE40(ctx, base);
	// 822EDA98: 7C6B0034  cntlzw r11, r3
	ctx.r[11].u64 = if ctx.r[3].u32 == 0 { 32 } else { ctx.r[3].u32.leading_zeros() as u64 };
	// 822EDA9C: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 822EDAA0: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 822EDAA4: 48000008  b 0x822edaac
	pc = 0x822EDAAC; continue 'dispatch;
	// 822EDAA8: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 822EDAAC: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 822EDAB0: 41820040  beq 0x822edaf0
	if ctx.cr[0].eq {
	pc = 0x822EDAF0; continue 'dispatch;
	}
	// 822EDAB4: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 822EDAB8: 83EB0008  lwz r31, 8(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 822EDABC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 822EDAC0: 419A000C  beq cr6, 0x822edacc
	if ctx.cr[6].eq {
	pc = 0x822EDACC; continue 'dispatch;
	}
	// 822EDAC4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822EDAC8: 48BE0D39  bl 0x82ece800
	ctx.lr = 0x822EDACC;
	sub_82ECE800(ctx, base);
	// 822EDACC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 822EDAD0: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 822EDAD4: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 822EDAD8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 822EDADC: 48BDE0B5  bl 0x82ecbb90
	ctx.lr = 0x822EDAE0;
	sub_82ECBB90(ctx, base);
	// 822EDAE0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 822EDAE4: 419A000C  beq cr6, 0x822edaf0
	if ctx.cr[6].eq {
	pc = 0x822EDAF0; continue 'dispatch;
	}
	// 822EDAE8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822EDAEC: 48BDF51D  bl 0x82ecd008
	ctx.lr = 0x822EDAF0;
	sub_82ECD008(ctx, base);
	// 822EDAF0: 807E00E4  lwz r3, 0xe4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(228 as u32) ) } as u64;
	// 822EDAF4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 822EDAF8: 419A0008  beq cr6, 0x822edb00
	if ctx.cr[6].eq {
	pc = 0x822EDB00; continue 'dispatch;
	}
	// 822EDAFC: 48B246BD  bl 0x82e121b8
	ctx.lr = 0x822EDB00;
	sub_82E121B8(ctx, base);
	// 822EDB00: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 822EDB04: 419A000C  beq cr6, 0x822edb10
	if ctx.cr[6].eq {
	pc = 0x822EDB10; continue 'dispatch;
	}
	// 822EDB08: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 822EDB0C: 48BDF4FD  bl 0x82ecd008
	ctx.lr = 0x822EDB10;
	sub_82ECD008(ctx, base);
	// 822EDB10: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 822EDB14: 48EBA6A4  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822EDB18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x822EDB18 size=24
    let mut pc: u32 = 0x822EDB18;
    'dispatch: loop {
        match pc {
            0x822EDB18 => {
    //   block [0x822EDB18..0x822EDB30)
	// 822EDB18: 816400E4  lwz r11, 0xe4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(228 as u32) ) } as u64;
	// 822EDB1C: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 822EDB20: 816400E8  lwz r11, 0xe8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(232 as u32) ) } as u64;
	// 822EDB24: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 822EDB28: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 822EDB2C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822EDB30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x822EDB30 size=36
    let mut pc: u32 = 0x822EDB30;
    'dispatch: loop {
        match pc {
            0x822EDB30 => {
    //   block [0x822EDB30..0x822EDB54)
	// 822EDB30: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 822EDB34: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 822EDB38: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 822EDB3C: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 822EDB40: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 822EDB44: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 822EDB48: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 822EDB4C: 4082FFE8  bne 0x822edb34
	if !ctx.cr[0].eq {
	pc = 0x822EDB34; continue 'dispatch;
	}
	// 822EDB50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822EDB58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x822EDB58 size=40
    let mut pc: u32 = 0x822EDB58;
    'dispatch: loop {
        match pc {
            0x822EDB58 => {
    //   block [0x822EDB58..0x822EDB80)
	// 822EDB58: 80630004  lwz r3, 4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 822EDB5C: 896300D8  lbz r11, 0xd8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(216 as u32) ) } as u64;
	// 822EDB60: 2B0B0007  cmplwi cr6, r11, 7
	ctx.cr[6].compare_u32(ctx.r[11].u32, 7 as u32, &mut ctx.xer);
	// 822EDB64: 419A0010  beq cr6, 0x822edb74
	if ctx.cr[6].eq {
	pc = 0x822EDB74; continue 'dispatch;
	}
	// 822EDB68: 2B0B0006  cmplwi cr6, r11, 6
	ctx.cr[6].compare_u32(ctx.r[11].u32, 6 as u32, &mut ctx.xer);
	// 822EDB6C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 822EDB70: 409A0008  bne cr6, 0x822edb78
	if !ctx.cr[6].eq {
	pc = 0x822EDB78; continue 'dispatch;
	}
	// 822EDB74: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 822EDB78: 7D6B0775  extsb. r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 822EDB7C: 4C820020  bnelr
	if !ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822EDB80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x822EDB80 size=8
    let mut pc: u32 = 0x822EDB80;
    'dispatch: loop {
        match pc {
            0x822EDB80 => {
    //   block [0x822EDB80..0x822EDB88)
	// 822EDB80: 48BDD970  b 0x82ecb4f0
	sub_82ECB4F0(ctx, base);
	return;
	// 822EDB84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822EDB88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822EDB88 size=196
    let mut pc: u32 = 0x822EDB88;
    'dispatch: loop {
        match pc {
            0x822EDB88 => {
    //   block [0x822EDB88..0x822EDC4C)
	// 822EDB88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822EDB8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822EDB90: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 822EDB94: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822EDB98: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822EDB9C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 822EDBA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 822EDBA4: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 822EDBA8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 822EDBAC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 822EDBB0: 4BFD2D89  bl 0x822c0938
	ctx.lr = 0x822EDBB4;
	sub_822C0938(ctx, base);
	// 822EDBB4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 822EDBB8: 41820028  beq 0x822edbe0
	if ctx.cr[0].eq {
	pc = 0x822EDBE0; continue 'dispatch;
	}
	// 822EDBBC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 822EDBC0: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 822EDBC4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 822EDBC8: 392BC100  addi r9, r11, -0x3f00
	ctx.r[9].s64 = ctx.r[11].s64 + -16128;
	// 822EDBCC: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 822EDBD0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 822EDBD4: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 822EDBD8: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 822EDBDC: 48000008  b 0x822edbe4
	pc = 0x822EDBE4; continue 'dispatch;
	// 822EDBE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 822EDBE4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 822EDBE8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 822EDBEC: 409A0044  bne cr6, 0x822edc30
	if !ctx.cr[6].eq {
	pc = 0x822EDC30; continue 'dispatch;
	}
	// 822EDBF0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 822EDBF4: 419A001C  beq cr6, 0x822edc10
	if ctx.cr[6].eq {
	pc = 0x822EDC10; continue 'dispatch;
	}
	// 822EDBF8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 822EDBFC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 822EDC00: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822EDC04: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 822EDC08: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 822EDC0C: 4E800421  bctrl
	ctx.lr = 0x822EDC10;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 822EDC10: 3D608325  lis r11, -0x7cdb
	ctx.r[11].s64 = -2094727168;
	// 822EDC14: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 822EDC18: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 822EDC1C: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 822EDC20: 816B46E4  lwz r11, 0x46e4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(18148 as u32) ) } as u64;
	// 822EDC24: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 822EDC28: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 822EDC2C: 4BFD23D5  bl 0x822c0000
	ctx.lr = 0x822EDC30;
	sub_822C0000(ctx, base);
	// 822EDC30: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 822EDC34: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 822EDC38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822EDC3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822EDC40: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 822EDC44: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822EDC48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822EDC50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822EDC50 size=196
    let mut pc: u32 = 0x822EDC50;
    'dispatch: loop {
        match pc {
            0x822EDC50 => {
    //   block [0x822EDC50..0x822EDD14)
	// 822EDC50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822EDC54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822EDC58: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 822EDC5C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822EDC60: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822EDC64: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 822EDC68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 822EDC6C: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 822EDC70: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 822EDC74: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 822EDC78: 4BFD2CC1  bl 0x822c0938
	ctx.lr = 0x822EDC7C;
	sub_822C0938(ctx, base);
	// 822EDC7C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 822EDC80: 41820028  beq 0x822edca8
	if ctx.cr[0].eq {
	pc = 0x822EDCA8; continue 'dispatch;
	}
	// 822EDC84: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 822EDC88: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 822EDC8C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 822EDC90: 392BC114  addi r9, r11, -0x3eec
	ctx.r[9].s64 = ctx.r[11].s64 + -16108;
	// 822EDC94: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 822EDC98: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 822EDC9C: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 822EDCA0: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 822EDCA4: 48000008  b 0x822edcac
	pc = 0x822EDCAC; continue 'dispatch;
	// 822EDCA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 822EDCAC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 822EDCB0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 822EDCB4: 409A0044  bne cr6, 0x822edcf8
	if !ctx.cr[6].eq {
	pc = 0x822EDCF8; continue 'dispatch;
	}
	// 822EDCB8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 822EDCBC: 419A001C  beq cr6, 0x822edcd8
	if ctx.cr[6].eq {
	pc = 0x822EDCD8; continue 'dispatch;
	}
	// 822EDCC0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 822EDCC4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 822EDCC8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822EDCCC: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 822EDCD0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 822EDCD4: 4E800421  bctrl
	ctx.lr = 0x822EDCD8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 822EDCD8: 3D608325  lis r11, -0x7cdb
	ctx.r[11].s64 = -2094727168;
	// 822EDCDC: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 822EDCE0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 822EDCE4: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 822EDCE8: 816B46E4  lwz r11, 0x46e4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(18148 as u32) ) } as u64;
	// 822EDCEC: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 822EDCF0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 822EDCF4: 4BFD230D  bl 0x822c0000
	ctx.lr = 0x822EDCF8;
	sub_822C0000(ctx, base);
	// 822EDCF8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 822EDCFC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 822EDD00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822EDD04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822EDD08: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 822EDD0C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822EDD10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822EDD18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822EDD18 size=196
    let mut pc: u32 = 0x822EDD18;
    'dispatch: loop {
        match pc {
            0x822EDD18 => {
    //   block [0x822EDD18..0x822EDDDC)
	// 822EDD18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822EDD1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822EDD20: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 822EDD24: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822EDD28: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822EDD2C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 822EDD30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 822EDD34: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 822EDD38: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 822EDD3C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 822EDD40: 4BFD2BF9  bl 0x822c0938
	ctx.lr = 0x822EDD44;
	sub_822C0938(ctx, base);
	// 822EDD44: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 822EDD48: 41820028  beq 0x822edd70
	if ctx.cr[0].eq {
	pc = 0x822EDD70; continue 'dispatch;
	}
	// 822EDD4C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 822EDD50: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 822EDD54: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 822EDD58: 392BC128  addi r9, r11, -0x3ed8
	ctx.r[9].s64 = ctx.r[11].s64 + -16088;
	// 822EDD5C: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 822EDD60: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 822EDD64: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 822EDD68: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 822EDD6C: 48000008  b 0x822edd74
	pc = 0x822EDD74; continue 'dispatch;
	// 822EDD70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 822EDD74: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 822EDD78: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 822EDD7C: 409A0044  bne cr6, 0x822eddc0
	if !ctx.cr[6].eq {
	pc = 0x822EDDC0; continue 'dispatch;
	}
	// 822EDD80: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 822EDD84: 419A001C  beq cr6, 0x822edda0
	if ctx.cr[6].eq {
	pc = 0x822EDDA0; continue 'dispatch;
	}
	// 822EDD88: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 822EDD8C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 822EDD90: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822EDD94: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 822EDD98: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 822EDD9C: 4E800421  bctrl
	ctx.lr = 0x822EDDA0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 822EDDA0: 3D608325  lis r11, -0x7cdb
	ctx.r[11].s64 = -2094727168;
	// 822EDDA4: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 822EDDA8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 822EDDAC: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 822EDDB0: 816B46E4  lwz r11, 0x46e4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(18148 as u32) ) } as u64;
	// 822EDDB4: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 822EDDB8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 822EDDBC: 4BFD2245  bl 0x822c0000
	ctx.lr = 0x822EDDC0;
	sub_822C0000(ctx, base);
	// 822EDDC0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 822EDDC4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 822EDDC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822EDDCC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822EDDD0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 822EDDD4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822EDDD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822EDDE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822EDDE0 size=124
    let mut pc: u32 = 0x822EDDE0;
    'dispatch: loop {
        match pc {
            0x822EDDE0 => {
    //   block [0x822EDDE0..0x822EDE5C)
	// 822EDDE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822EDDE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822EDDE8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822EDDEC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822EDDF0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822EDDF4: 4BFF8EBD  bl 0x822e6cb0
	ctx.lr = 0x822EDDF8;
	sub_822E6CB0(ctx, base);
	// 822EDDF8: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 822EDDFC: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 822EDE00: 3D008201  lis r8, -0x7dff
	ctx.r[8].s64 = -2113863680;
	// 822EDE04: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 822EDE08: 394AA980  addi r10, r10, -0x5680
	ctx.r[10].s64 = ctx.r[10].s64 + -22144;
	// 822EDE0C: 3929C148  addi r9, r9, -0x3eb8
	ctx.r[9].s64 = ctx.r[9].s64 + -16056;
	// 822EDE10: 917F0054  stw r11, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 822EDE14: 3908C13C  addi r8, r8, -0x3ec4
	ctx.r[8].s64 = ctx.r[8].s64 + -16068;
	// 822EDE18: 915F0050  stw r10, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 822EDE1C: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 822EDE20: 395F0050  addi r10, r31, 0x50
	ctx.r[10].s64 = ctx.r[31].s64 + 80;
	// 822EDE24: 911F0050  stw r8, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[8].u32 ) };
	// 822EDE28: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822EDE2C: 917F00E0  stw r11, 0xe0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(224 as u32), ctx.r[11].u32 ) };
	// 822EDE30: 917F00E4  stw r11, 0xe4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(228 as u32), ctx.r[11].u32 ) };
	// 822EDE34: 917F00E8  stw r11, 0xe8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(232 as u32), ctx.r[11].u32 ) };
	// 822EDE38: 917F00EC  stw r11, 0xec(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(236 as u32), ctx.r[11].u32 ) };
	// 822EDE3C: 917F00F0  stw r11, 0xf0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(240 as u32), ctx.r[11].u32 ) };
	// 822EDE40: 917F00F4  stw r11, 0xf4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(244 as u32), ctx.r[11].u32 ) };
	// 822EDE44: 917F00F8  stw r11, 0xf8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(248 as u32), ctx.r[11].u32 ) };
	// 822EDE48: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 822EDE4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822EDE50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822EDE54: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822EDE58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822EDE60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822EDE60 size=76
    let mut pc: u32 = 0x822EDE60;
    'dispatch: loop {
        match pc {
            0x822EDE60 => {
    //   block [0x822EDE60..0x822EDEAC)
	// 822EDE60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822EDE64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822EDE68: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 822EDE6C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822EDE70: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822EDE74: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822EDE78: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 822EDE7C: 4BFFF675  bl 0x822ed4f0
	ctx.lr = 0x822EDE80;
	sub_822ED4F0(ctx, base);
	// 822EDE80: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 822EDE84: 4182000C  beq 0x822ede90
	if ctx.cr[0].eq {
	pc = 0x822EDE90; continue 'dispatch;
	}
	// 822EDE88: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822EDE8C: 48B0454D  bl 0x82df23d8
	ctx.lr = 0x822EDE90;
	sub_82DF23D8(ctx, base);
	// 822EDE90: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822EDE94: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 822EDE98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822EDE9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822EDEA0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 822EDEA4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822EDEA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822EDEB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x822EDEB0 size=176
    let mut pc: u32 = 0x822EDEB0;
    'dispatch: loop {
        match pc {
            0x822EDEB0 => {
    //   block [0x822EDEB0..0x822EDF60)
	// 822EDEB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822EDEB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822EDEB8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 822EDEBC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822EDEC0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822EDEC4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822EDEC8: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 822EDECC: 4BFF8DE5  bl 0x822e6cb0
	ctx.lr = 0x822EDED0;
	sub_822E6CB0(ctx, base);
	// 822EDED0: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 822EDED4: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 822EDED8: 3D008201  lis r8, -0x7dff
	ctx.r[8].s64 = -2113863680;
	// 822EDEDC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 822EDEE0: 394AA980  addi r10, r10, -0x5680
	ctx.r[10].s64 = ctx.r[10].s64 + -22144;
	// 822EDEE4: 3929C148  addi r9, r9, -0x3eb8
	ctx.r[9].s64 = ctx.r[9].s64 + -16056;
	// 822EDEE8: 917F0054  stw r11, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 822EDEEC: 3908C13C  addi r8, r8, -0x3ec4
	ctx.r[8].s64 = ctx.r[8].s64 + -16068;
	// 822EDEF0: 915F0050  stw r10, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 822EDEF4: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 822EDEF8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822EDEFC: 911F0050  stw r8, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[8].u32 ) };
	// 822EDF00: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 822EDF04: 917F00E0  stw r11, 0xe0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(224 as u32), ctx.r[11].u32 ) };
	// 822EDF08: 917F00E4  stw r11, 0xe4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(228 as u32), ctx.r[11].u32 ) };
	// 822EDF0C: 917F00E8  stw r11, 0xe8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(232 as u32), ctx.r[11].u32 ) };
	// 822EDF10: 917F00EC  stw r11, 0xec(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(236 as u32), ctx.r[11].u32 ) };
	// 822EDF14: 917F00F0  stw r11, 0xf0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(240 as u32), ctx.r[11].u32 ) };
	// 822EDF18: 917F00F4  stw r11, 0xf4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(244 as u32), ctx.r[11].u32 ) };
	// 822EDF1C: 917F00F8  stw r11, 0xf8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(248 as u32), ctx.r[11].u32 ) };
	// 822EDF20: 397F0050  addi r11, r31, 0x50
	ctx.r[11].s64 = ctx.r[31].s64 + 80;
	// 822EDF24: 4BFFF68D  bl 0x822ed5b0
	ctx.lr = 0x822EDF28;
	sub_822ED5B0(ctx, base);
	// 822EDF28: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 822EDF2C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 822EDF30: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 822EDF34: 386B018D  addi r3, r11, 0x18d
	ctx.r[3].s64 = ctx.r[11].s64 + 397;
	// 822EDF38: C00ABCA4  lfs f0, -0x435c(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-17244 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 822EDF3C: D0010050  stfs f0, 0x50(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 822EDF40: 48BB9B11  bl 0x82ea7a50
	ctx.lr = 0x822EDF44;
	sub_82EA7A50(ctx, base);
	// 822EDF44: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822EDF48: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 822EDF4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822EDF50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822EDF54: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 822EDF58: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822EDF5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822EDF60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x822EDF60 size=336
    let mut pc: u32 = 0x822EDF60;
    'dispatch: loop {
        match pc {
            0x822EDF60 => {
    //   block [0x822EDF60..0x822EE0B0)
	// 822EDF60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822EDF64: 48EBA205  bl 0x831a8168
	ctx.lr = 0x822EDF68;
	sub_831A8130(ctx, base);
	// 822EDF68: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822EDF6C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822EDF70: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 822EDF74: 3BBF00F4  addi r29, r31, 0xf4
	ctx.r[29].s64 = ctx.r[31].s64 + 244;
	// 822EDF78: 807F00F4  lwz r3, 0xf4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(244 as u32) ) } as u64;
	// 822EDF7C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 822EDF80: 419A000C  beq cr6, 0x822edf8c
	if ctx.cr[6].eq {
	pc = 0x822EDF8C; continue 'dispatch;
	}
	// 822EDF84: 389F0050  addi r4, r31, 0x50
	ctx.r[4].s64 = ctx.r[31].s64 + 80;
	// 822EDF88: 48B245B9  bl 0x82e12540
	ctx.lr = 0x822EDF8C;
	sub_82E12540(ctx, base);
	// 822EDF8C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 822EDF90: 3B9E0004  addi r28, r30, 4
	ctx.r[28].s64 = ctx.r[30].s64 + 4;
	// 822EDF94: 387D0004  addi r3, r29, 4
	ctx.r[3].s64 = ctx.r[29].s64 + 4;
	// 822EDF98: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 822EDF9C: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 822EDFA0: 4BFD64C1  bl 0x822c4460
	ctx.lr = 0x822EDFA4;
	sub_822C4460(ctx, base);
	// 822EDFA4: 807D0000  lwz r3, 0(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 822EDFA8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 822EDFAC: 419A002C  beq cr6, 0x822edfd8
	if ctx.cr[6].eq {
	pc = 0x822EDFD8; continue 'dispatch;
	}
	// 822EDFB0: 389F0050  addi r4, r31, 0x50
	ctx.r[4].s64 = ctx.r[31].s64 + 80;
	// 822EDFB4: 48B24B0D  bl 0x82e12ac0
	ctx.lr = 0x822EDFB8;
	sub_82E12AC0(ctx, base);
	// 822EDFB8: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 822EDFBC: 83CB0008  lwz r30, 8(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 822EDFC0: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 822EDFC4: 419A000C  beq cr6, 0x822edfd0
	if ctx.cr[6].eq {
	pc = 0x822EDFD0; continue 'dispatch;
	}
	// 822EDFC8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 822EDFCC: 48BE0835  bl 0x82ece800
	ctx.lr = 0x822EDFD0;
	sub_82ECE800(ctx, base);
	// 822EDFD0: 38800006  li r4, 6
	ctx.r[4].s64 = 6;
	// 822EDFD4: 48000020  b 0x822edff4
	pc = 0x822EDFF4; continue 'dispatch;
	// 822EDFD8: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 822EDFDC: 83CB0008  lwz r30, 8(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 822EDFE0: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 822EDFE4: 419A000C  beq cr6, 0x822edff0
	if ctx.cr[6].eq {
	pc = 0x822EDFF0; continue 'dispatch;
	}
	// 822EDFE8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 822EDFEC: 48BE0815  bl 0x82ece800
	ctx.lr = 0x822EDFF0;
	sub_82ECE800(ctx, base);
	// 822EDFF0: 38800007  li r4, 7
	ctx.r[4].s64 = 7;
	// 822EDFF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 822EDFF8: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 822EDFFC: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 822EE000: 48BDDB91  bl 0x82ecbb90
	ctx.lr = 0x822EE004;
	sub_82ECBB90(ctx, base);
	// 822EE004: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 822EE008: 419A000C  beq cr6, 0x822ee014
	if ctx.cr[6].eq {
	pc = 0x822EE014; continue 'dispatch;
	}
	// 822EE00C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 822EE010: 48BDEFF9  bl 0x82ecd008
	ctx.lr = 0x822EE014;
	sub_82ECD008(ctx, base);
	// 822EE014: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 822EE018: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 822EE01C: 419A007C  beq cr6, 0x822ee098
	if ctx.cr[6].eq {
	pc = 0x822EE098; continue 'dispatch;
	}
	// 822EE020: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 822EE024: 83CB0008  lwz r30, 8(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 822EE028: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 822EE02C: 419A000C  beq cr6, 0x822ee038
	if ctx.cr[6].eq {
	pc = 0x822EE038; continue 'dispatch;
	}
	// 822EE030: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 822EE034: 48BE07CD  bl 0x82ece800
	ctx.lr = 0x822EE038;
	sub_82ECE800(ctx, base);
	// 822EE038: 807D0000  lwz r3, 0(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 822EE03C: 83BF0004  lwz r29, 4(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 822EE040: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 822EE044: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 822EE048: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 822EE04C: 4E800421  bctrl
	ctx.lr = 0x822EE050;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 822EE050: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 822EE054: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 822EE058: 4BFFA831  bl 0x822e8888
	ctx.lr = 0x822EE05C;
	sub_822E8888(ctx, base);
	// 822EE05C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 822EE060: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 822EE064: 48BDE035  bl 0x82ecc098
	ctx.lr = 0x822EE068;
	sub_82ECC098(ctx, base);
	// 822EE068: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 822EE06C: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 822EE070: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 822EE074: 396BBC40  addi r11, r11, -0x43c0
	ctx.r[11].s64 = ctx.r[11].s64 + -17344;
	// 822EE078: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 822EE07C: 13E058C7  vcmpequd (lvx128) v31, v0, v11
	tmp.u32 = ctx.r[11].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822EE0B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822EE0B0 size=48
    let mut pc: u32 = 0x822EE0B0;
    'dispatch: loop {
        match pc {
            0x822EE0B0 => {
    //   block [0x822EE0B0..0x822EE0E0)
	// 822EE0B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822EE0B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822EE0B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822EE0BC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 822EE0C0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 822EE0C4: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 822EE0C8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 822EE0CC: 4BFFFE95  bl 0x822edf60
	ctx.lr = 0x822EE0D0;
	sub_822EDF60(ctx, base);
	// 822EE0D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 822EE0D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822EE0D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822EE0DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822EE0E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x822EE0E0 size=224
    let mut pc: u32 = 0x822EE0E0;
    'dispatch: loop {
        match pc {
            0x822EE0E0 => {
    //   block [0x822EE0E0..0x822EE1C0)
	// 822EE0E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822EE0E4: 48EBA085  bl 0x831a8168
	ctx.lr = 0x822EE0E8;
	sub_831A8130(ctx, base);
	// 822EE0E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822EE0EC: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 822EE0F0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 822EE0F4: 387D00E0  addi r3, r29, 0xe0
	ctx.r[3].s64 = ctx.r[29].s64 + 224;
	// 822EE0F8: 7CC43378  mr r4, r6
	ctx.r[4].u64 = ctx.r[6].u64;
	// 822EE0FC: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 822EE100: 4BFF8B51  bl 0x822e6c50
	ctx.lr = 0x822EE104;
	sub_822E6C50(ctx, base);
	// 822EE104: 807D0004  lwz r3, 4(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 822EE108: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 822EE10C: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 822EE110: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 822EE114: 4E800421  bctrl
	ctx.lr = 0x822EE118;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 822EE118: 90610050  stw r3, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 822EE11C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 822EE120: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 822EE124: 388BC198  addi r4, r11, -0x3e68
	ctx.r[4].s64 = ctx.r[11].s64 + -15976;
	// 822EE128: 38A000DF  li r5, 0xdf
	ctx.r[5].s64 = 223;
	// 822EE12C: 38600110  li r3, 0x110
	ctx.r[3].s64 = 272;
	// 822EE130: 48B042B9  bl 0x82df23e8
	ctx.lr = 0x822EE134;
	sub_82DF23E8(ctx, base);
	// 822EE134: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 822EE138: 41820018  beq 0x822ee150
	if ctx.cr[0].eq {
	pc = 0x822EE150; continue 'dispatch;
	}
	// 822EE13C: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 822EE140: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 822EE144: 4BFFFD6D  bl 0x822edeb0
	ctx.lr = 0x822EE148;
	sub_822EDEB0(ctx, base);
	// 822EE148: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822EE14C: 48000008  b 0x822ee154
	pc = 0x822EE154; continue 'dispatch;
	// 822EE150: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 822EE154: 93FE0000  stw r31, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 822EE158: 3B9E0004  addi r28, r30, 4
	ctx.r[28].s64 = ctx.r[30].s64 + 4;
	// 822EE15C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 822EE160: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 822EE164: 4BFFFA25  bl 0x822edb88
	ctx.lr = 0x822EE168;
	sub_822EDB88(ctx, base);
	// 822EE168: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 822EE16C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 822EE170: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 822EE174: 4BFD1E8D  bl 0x822c0000
	ctx.lr = 0x822EE178;
	sub_822C0000(ctx, base);
	// 822EE178: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 822EE17C: 83FE0000  lwz r31, 0(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 822EE180: 48222E61  bl 0x82510fe0
	ctx.lr = 0x822EE184;
	sub_82510FE0(ctx, base);
	// 822EE184: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 822EE188: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822EE18C: 481A1215  bl 0x8248f3a0
	ctx.lr = 0x822EE190;
	sub_8248F3A0(ctx, base);
	// 822EE190: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 822EE194: 39400100  li r10, 0x100
	ctx.r[10].s64 = 256;
	// 822EE198: 396B0100  addi r11, r11, 0x100
	ctx.r[11].s64 = ctx.r[11].s64 + 256;
	// 822EE19C: 13FD50C7  vcmpequd (lvx128) v31, v29, v10
	tmp.u32 = ctx.r[29].u32 + ctx.r[10].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822EE1C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822EE1C0 size=180
    let mut pc: u32 = 0x822EE1C0;
    'dispatch: loop {
        match pc {
            0x822EE1C0 => {
    //   block [0x822EE1C0..0x822EE274)
	// 822EE1C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822EE1C4: 48EB9FA9  bl 0x831a816c
	ctx.lr = 0x822EE1C8;
	sub_831A8130(ctx, base);
	// 822EE1C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822EE1CC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 822EE1D0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 822EE1D4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 822EE1D8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 822EE1DC: 388BC198  addi r4, r11, -0x3e68
	ctx.r[4].s64 = ctx.r[11].s64 + -15976;
	// 822EE1E0: 38A002F7  li r5, 0x2f7
	ctx.r[5].s64 = 759;
	// 822EE1E4: 386000EC  li r3, 0xec
	ctx.r[3].s64 = 236;
	// 822EE1E8: 48B04201  bl 0x82df23e8
	ctx.lr = 0x822EE1EC;
	sub_82DF23E8(ctx, base);
	// 822EE1EC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 822EE1F0: 41820050  beq 0x822ee240
	if ctx.cr[0].eq {
	pc = 0x822EE240; continue 'dispatch;
	}
	// 822EE1F4: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 822EE1F8: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 822EE1FC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 822EE200: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 822EE204: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 822EE208: 419A0024  beq cr6, 0x822ee22c
	if ctx.cr[6].eq {
	pc = 0x822EE22C; continue 'dispatch;
	}
	// 822EE20C: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 822EE210: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 822EE214: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 822EE218: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 822EE21C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 822EE220: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 822EE224: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 822EE228: 4082FFE8  bne 0x822ee210
	if !ctx.cr[0].eq {
	pc = 0x822EE210; continue 'dispatch;
	}
	// 822EE22C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 822EE230: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 822EE234: 481A017D  bl 0x8248e3b0
	ctx.lr = 0x822EE238;
	sub_8248E3B0(ctx, base);
	// 822EE238: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822EE23C: 48000008  b 0x822ee244
	pc = 0x822EE244; continue 'dispatch;
	// 822EE240: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 822EE244: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 822EE248: 3BDD0004  addi r30, r29, 4
	ctx.r[30].s64 = ctx.r[29].s64 + 4;
	// 822EE24C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 822EE250: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 822EE254: 4BFFF9FD  bl 0x822edc50
	ctx.lr = 0x822EE258;
	sub_822EDC50(ctx, base);
	// 822EE258: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 822EE25C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 822EE260: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 822EE264: 4BFD1D9D  bl 0x822c0000
	ctx.lr = 0x822EE268;
	sub_822C0000(ctx, base);
	// 822EE268: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 822EE26C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 822EE270: 48EB9F4C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822EE278(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822EE278 size=112
    let mut pc: u32 = 0x822EE278;
    'dispatch: loop {
        match pc {
            0x822EE278 => {
    //   block [0x822EE278..0x822EE2E8)
	// 822EE278: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822EE27C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822EE280: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 822EE284: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822EE288: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822EE28C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 822EE290: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822EE294: 93C10050  stw r30, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 822EE298: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 822EE29C: 4BFFFA7D  bl 0x822edd18
	ctx.lr = 0x822EE2A0;
	sub_822EDD18(ctx, base);
	// 822EE2A0: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 822EE2A4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 822EE2A8: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 822EE2AC: 4BFD1D55  bl 0x822c0000
	ctx.lr = 0x822EE2B0;
	sub_822C0000(ctx, base);
	// 822EE2B0: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 822EE2B4: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 822EE2B8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 822EE2BC: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 822EE2C0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 822EE2C4: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 822EE2C8: 419A0008  beq cr6, 0x822ee2d0
	if ctx.cr[6].eq {
	pc = 0x822EE2D0; continue 'dispatch;
	}
	// 822EE2CC: 4BFD25C5  bl 0x822c0890
	ctx.lr = 0x822EE2D0;
	sub_822C0890(ctx, base);
	// 822EE2D0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 822EE2D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822EE2D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822EE2DC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 822EE2E0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822EE2E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822EE2E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822EE2E8 size=164
    let mut pc: u32 = 0x822EE2E8;
    'dispatch: loop {
        match pc {
            0x822EE2E8 => {
    //   block [0x822EE2E8..0x822EE38C)
	// 822EE2E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822EE2EC: 48EB9E81  bl 0x831a816c
	ctx.lr = 0x822EE2F0;
	sub_831A8130(ctx, base);
	// 822EE2F0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822EE2F4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 822EE2F8: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 822EE2FC: 3BFE00E4  addi r31, r30, 0xe4
	ctx.r[31].s64 = ctx.r[30].s64 + 228;
	// 822EE300: 817E00E4  lwz r11, 0xe4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(228 as u32) ) } as u64;
	// 822EE304: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 822EE308: 409A0040  bne cr6, 0x822ee348
	if !ctx.cr[6].eq {
	pc = 0x822EE348; continue 'dispatch;
	}
	// 822EE30C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 822EE310: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 822EE314: 388BC198  addi r4, r11, -0x3e68
	ctx.r[4].s64 = ctx.r[11].s64 + -15976;
	// 822EE318: 38A0020A  li r5, 0x20a
	ctx.r[5].s64 = 522;
	// 822EE31C: 386000F0  li r3, 0xf0
	ctx.r[3].s64 = 240;
	// 822EE320: 48B040C9  bl 0x82df23e8
	ctx.lr = 0x822EE324;
	sub_82DF23E8(ctx, base);
	// 822EE324: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 822EE328: 41820014  beq 0x822ee33c
	if ctx.cr[0].eq {
	pc = 0x822EE33C; continue 'dispatch;
	}
	// 822EE32C: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 822EE330: 48005151  bl 0x822f3480
	ctx.lr = 0x822EE334;
	sub_822F3480(ctx, base);
	// 822EE334: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 822EE338: 48000008  b 0x822ee340
	pc = 0x822EE340; continue 'dispatch;
	// 822EE33C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 822EE340: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822EE344: 4BFFFF35  bl 0x822ee278
	ctx.lr = 0x822EE348;
	sub_822EE278(ctx, base);
	// 822EE348: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 822EE34C: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 822EE350: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 822EE354: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 822EE358: 917D0004  stw r11, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 822EE35C: 419A0024  beq cr6, 0x822ee380
	if ctx.cr[6].eq {
	pc = 0x822EE380; continue 'dispatch;
	}
	// 822EE360: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 822EE364: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 822EE368: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 822EE36C: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 822EE370: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 822EE374: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 822EE378: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 822EE37C: 4082FFE8  bne 0x822ee364
	if !ctx.cr[0].eq {
	pc = 0x822EE364; continue 'dispatch;
	}
	// 822EE380: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 822EE384: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 822EE388: 48EB9E34  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822EE390(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822EE390 size=184
    let mut pc: u32 = 0x822EE390;
    'dispatch: loop {
        match pc {
            0x822EE390 => {
    //   block [0x822EE390..0x822EE448)
	// 822EE390: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822EE394: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822EE398: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 822EE39C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822EE3A0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822EE3A4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 822EE3A8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 822EE3AC: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 822EE3B0: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 822EE3B4: 388BC198  addi r4, r11, -0x3e68
	ctx.r[4].s64 = ctx.r[11].s64 + -15976;
	// 822EE3B8: 38A00227  li r5, 0x227
	ctx.r[5].s64 = 551;
	// 822EE3BC: 386000E0  li r3, 0xe0
	ctx.r[3].s64 = 224;
	// 822EE3C0: 48B04029  bl 0x82df23e8
	ctx.lr = 0x822EE3C4;
	sub_82DF23E8(ctx, base);
	// 822EE3C4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 822EE3C8: 41820014  beq 0x822ee3dc
	if ctx.cr[0].eq {
	pc = 0x822EE3DC; continue 'dispatch;
	}
	// 822EE3CC: 389F00A0  addi r4, r31, 0xa0
	ctx.r[4].s64 = ctx.r[31].s64 + 160;
	// 822EE3D0: 48B24D21  bl 0x82e130f0
	ctx.lr = 0x822EE3D4;
	sub_82E130F0(ctx, base);
	// 822EE3D4: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 822EE3D8: 48000008  b 0x822ee3e0
	pc = 0x822EE3E0; continue 'dispatch;
	// 822EE3DC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 822EE3E0: 387F00EC  addi r3, r31, 0xec
	ctx.r[3].s64 = ctx.r[31].s64 + 236;
	// 822EE3E4: 4BFF396D  bl 0x822e1d50
	ctx.lr = 0x822EE3E8;
	sub_822E1D50(ctx, base);
	// 822EE3E8: 809F00EC  lwz r4, 0xec(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(236 as u32) ) } as u64;
	// 822EE3EC: 807F00E4  lwz r3, 0xe4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(228 as u32) ) } as u64;
	// 822EE3F0: 48B24A41  bl 0x82e12e30
	ctx.lr = 0x822EE3F4;
	sub_82E12E30(ctx, base);
	// 822EE3F4: 817F00EC  lwz r11, 0xec(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(236 as u32) ) } as u64;
	// 822EE3F8: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 822EE3FC: 817F00F0  lwz r11, 0xf0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(240 as u32) ) } as u64;
	// 822EE400: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 822EE404: 917E0004  stw r11, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 822EE408: 419A0024  beq cr6, 0x822ee42c
	if ctx.cr[6].eq {
	pc = 0x822EE42C; continue 'dispatch;
	}
	// 822EE40C: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 822EE410: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 822EE414: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 822EE418: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 822EE41C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 822EE420: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 822EE424: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 822EE428: 4082FFE8  bne 0x822ee410
	if !ctx.cr[0].eq {
	pc = 0x822EE410; continue 'dispatch;
	}
	// 822EE42C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 822EE430: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 822EE434: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822EE438: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822EE43C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 822EE440: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822EE444: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822EE448(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822EE448 size=112
    let mut pc: u32 = 0x822EE448;
    'dispatch: loop {
        match pc {
            0x822EE448 => {
    //   block [0x822EE448..0x822EE4B8)
	// 822EE448: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822EE44C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822EE450: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 822EE454: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822EE458: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822EE45C: 3D608325  lis r11, -0x7cdb
	ctx.r[11].s64 = -2094727168;
	// 822EE460: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822EE464: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 822EE468: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 822EE46C: 388B4224  addi r4, r11, 0x4224
	ctx.r[4].s64 = ctx.r[11].s64 + 16932;
	// 822EE470: 48B05599  bl 0x82df3a08
	ctx.lr = 0x822EE474;
	sub_82DF3A08(ctx, base);
	// 822EE474: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 822EE478: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 822EE47C: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 822EE480: 4BFFAC89  bl 0x822e9108
	ctx.lr = 0x822EE484;
	sub_822E9108(ctx, base);
	// 822EE484: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 822EE488: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822EE48C: 48B13FAD  bl 0x82e02438
	ctx.lr = 0x822EE490;
	sub_82E02438(ctx, base);
	// 822EE490: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 822EE494: 48B04F95  bl 0x82df3428
	ctx.lr = 0x822EE498;
	sub_82DF3428(ctx, base);
	// 822EE498: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 822EE49C: 48B04F8D  bl 0x82df3428
	ctx.lr = 0x822EE4A0;
	sub_82DF3428(ctx, base);
	// 822EE4A0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 822EE4A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822EE4A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822EE4AC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 822EE4B0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822EE4B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822EE4B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822EE4B8 size=88
    let mut pc: u32 = 0x822EE4B8;
    'dispatch: loop {
        match pc {
            0x822EE4B8 => {
    //   block [0x822EE4B8..0x822EE510)
	// 822EE4B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822EE4BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822EE4C0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 822EE4C4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822EE4C8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822EE4CC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 822EE4D0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 822EE4D4: 48000014  b 0x822ee4e8
	pc = 0x822EE4E8; continue 'dispatch;
	// 822EE4D8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 822EE4DC: 48B15B05  bl 0x82e03fe0
	ctx.lr = 0x822EE4E0;
	sub_82E03FE0(ctx, base);
	// 822EE4E0: 38600005  li r3, 5
	ctx.r[3].s64 = 5;
	// 822EE4E4: 48B04105  bl 0x82df25e8
	ctx.lr = 0x822EE4E8;
	sub_82DF25E8(ctx, base);
	// 822EE4E8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822EE4EC: 48B102F5  bl 0x82dfe7e0
	ctx.lr = 0x822EE4F0;
	sub_82DFE7E0(ctx, base);
	// 822EE4F0: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 822EE4F4: 4182FFE4  beq 0x822ee4d8
	if ctx.cr[0].eq {
	pc = 0x822EE4D8; continue 'dispatch;
	}
	// 822EE4F8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 822EE4FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822EE500: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822EE504: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 822EE508: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822EE50C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822EE510(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x822EE510 size=64
    let mut pc: u32 = 0x822EE510;
    'dispatch: loop {
        match pc {
            0x822EE510 => {
    //   block [0x822EE510..0x822EE550)
	// 822EE510: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 822EE514: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 822EE518: 91440000  stw r10, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 822EE51C: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 822EE520: 892A0015  lbz r9, 0x15(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(21 as u32) ) } as u64;
	// 822EE524: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 822EE528: 409A0008  bne cr6, 0x822ee530
	if !ctx.cr[6].eq {
	pc = 0x822EE530; continue 'dispatch;
	}
	// 822EE52C: 908A0004  stw r4, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 822EE530: 81440004  lwz r10, 4(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 822EE534: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 822EE538: 81430004  lwz r10, 4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 822EE53C: 812A0004  lwz r9, 4(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 822EE540: 7F044840  cmplw cr6, r4, r9
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[9].u32, &mut ctx.xer);
	// 822EE544: 409A000C  bne cr6, 0x822ee550
	if !ctx.cr[6].eq {
		sub_822EE550(ctx, base);
		return;
	}
	// 822EE548: 916A0004  stw r11, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 822EE54C: 48000020  b 0x822ee56c
	sub_822EE568(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822EE550(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x822EE550 size=24
    let mut pc: u32 = 0x822EE550;
    'dispatch: loop {
        match pc {
            0x822EE550 => {
    //   block [0x822EE550..0x822EE568)
	// 822EE550: 81440004  lwz r10, 4(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 822EE554: 812A0008  lwz r9, 8(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 822EE558: 7F044840  cmplw cr6, r4, r9
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[9].u32, &mut ctx.xer);
	// 822EE55C: 409A000C  bne cr6, 0x822ee568
	if !ctx.cr[6].eq {
		sub_822EE568(ctx, base);
		return;
	}
	// 822EE560: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 822EE564: 48000008  b 0x822ee56c
	sub_822EE568(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822EE568(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x822EE568 size=16
    let mut pc: u32 = 0x822EE568;
    'dispatch: loop {
        match pc {
            0x822EE568 => {
    //   block [0x822EE568..0x822EE578)
	// 822EE568: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 822EE56C: 908B0008  stw r4, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[4].u32 ) };
	// 822EE570: 91640004  stw r11, 4(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 822EE574: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822EE578(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822EE578 size=100
    let mut pc: u32 = 0x822EE578;
    'dispatch: loop {
        match pc {
            0x822EE578 => {
    //   block [0x822EE578..0x822EE5DC)
	// 822EE578: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822EE57C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822EE580: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822EE584: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822EE588: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822EE58C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 822EE590: 419A0038  beq cr6, 0x822ee5c8
	if ctx.cr[6].eq {
	pc = 0x822EE5C8; continue 'dispatch;
	}
	// 822EE594: 48B1024D  bl 0x82dfe7e0
	ctx.lr = 0x822EE598;
	sub_82DFE7E0(ctx, base);
	// 822EE598: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 822EE59C: 4082002C  bne 0x822ee5c8
	if !ctx.cr[0].eq {
	pc = 0x822EE5C8; continue 'dispatch;
	}
	// 822EE5A0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 822EE5A4: 481FC4E5  bl 0x824eaa88
	ctx.lr = 0x822EE5A8;
	sub_824EAA88(ctx, base);
	// 822EE5A8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 822EE5AC: 38C000F2  li r6, 0xf2
	ctx.r[6].s64 = 242;
	// 822EE5B0: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 822EE5B4: 38ABC1D0  addi r5, r11, -0x3e30
	ctx.r[5].s64 = ctx.r[11].s64 + -15920;
	// 822EE5B8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 822EE5BC: 48009DC5  bl 0x822f8380
	ctx.lr = 0x822EE5C0;
	sub_822F8380(ctx, base);
	// 822EE5C0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 822EE5C4: 48B036CD  bl 0x82df1c90
	ctx.lr = 0x822EE5C8;
	sub_82DF1C90(ctx, base);
	// 822EE5C8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 822EE5CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822EE5D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822EE5D4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822EE5D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822EE5E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822EE5E0 size=116
    let mut pc: u32 = 0x822EE5E0;
    'dispatch: loop {
        match pc {
            0x822EE5E0 => {
    //   block [0x822EE5E0..0x822EE654)
	// 822EE5E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822EE5E4: 48EB9B81  bl 0x831a8164
	ctx.lr = 0x822EE5E8;
	sub_831A8130(ctx, base);
	// 822EE5E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822EE5EC: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 822EE5F0: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 822EE5F4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 822EE5F8: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 822EE5FC: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 822EE600: 38C00018  li r6, 0x18
	ctx.r[6].s64 = 24;
	// 822EE604: 806B110C  lwz r3, 0x110c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4364 as u32) ) } as u64;
	// 822EE608: 388A08B0  addi r4, r10, 0x8b0
	ctx.r[4].s64 = ctx.r[10].s64 + 2224;
	// 822EE60C: 38A0002D  li r5, 0x2d
	ctx.r[5].s64 = 45;
	// 822EE610: 7CFF3B78  mr r31, r7
	ctx.r[31].u64 = ctx.r[7].u64;
	// 822EE614: 7D1B4378  mr r27, r8
	ctx.r[27].u64 = ctx.r[8].u64;
	// 822EE618: 48B03AB1  bl 0x82df20c8
	ctx.lr = 0x822EE61C;
	sub_82DF20C8(ctx, base);
	// 822EE61C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 822EE620: 4182002C  beq 0x822ee64c
	if ctx.cr[0].eq {
	pc = 0x822EE64C; continue 'dispatch;
	}
	// 822EE624: 93C30000  stw r30, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 822EE628: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 822EE62C: 93A30004  stw r29, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 822EE630: 93830008  stw r28, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[28].u32 ) };
	// 822EE634: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 822EE638: 9143000C  stw r10, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 822EE63C: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 822EE640: 91430010  stw r10, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 822EE644: 9B630014  stb r27, 0x14(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[27].u8 ) };
	// 822EE648: 99630015  stb r11, 0x15(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(21 as u32), ctx.r[11].u8 ) };
	// 822EE64C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 822EE650: 48EB9B64  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822EE658(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822EE658 size=124
    let mut pc: u32 = 0x822EE658;
    'dispatch: loop {
        match pc {
            0x822EE658 => {
    //   block [0x822EE658..0x822EE6D4)
	// 822EE658: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822EE65C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822EE660: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 822EE664: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822EE668: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822EE66C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 822EE670: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822EE674: 809E0000  lwz r4, 0(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 822EE678: 48B05391  bl 0x82df3a08
	ctx.lr = 0x822EE67C;
	sub_82DF3A08(ctx, base);
	// 822EE67C: 397F0004  addi r11, r31, 4
	ctx.r[11].s64 = ctx.r[31].s64 + 4;
	// 822EE680: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 822EE684: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 822EE688: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 822EE68C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 822EE690: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 822EE694: 419A0024  beq cr6, 0x822ee6b8
	if ctx.cr[6].eq {
	pc = 0x822EE6B8; continue 'dispatch;
	}
	// 822EE698: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 822EE69C: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 822EE6A0: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 822EE6A4: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 822EE6A8: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 822EE6AC: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 822EE6B0: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 822EE6B4: 4082FFE8  bne 0x822ee69c
	if !ctx.cr[0].eq {
	pc = 0x822EE69C; continue 'dispatch;
	}
	// 822EE6B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822EE6BC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 822EE6C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822EE6C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822EE6C8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 822EE6CC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822EE6D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822EE6D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822EE6D8 size=116
    let mut pc: u32 = 0x822EE6D8;
    'dispatch: loop {
        match pc {
            0x822EE6D8 => {
    //   block [0x822EE6D8..0x822EE74C)
	// 822EE6D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822EE6DC: 48EB9A8D  bl 0x831a8168
	ctx.lr = 0x822EE6E0;
	sub_831A8130(ctx, base);
	// 822EE6E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822EE6E4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 822EE6E8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 822EE6EC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822EE6F0: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 822EE6F4: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 822EE6F8: 4BFFFE81  bl 0x822ee578
	ctx.lr = 0x822EE6FC;
	sub_822EE578(ctx, base);
	// 822EE6FC: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 822EE700: 389F0010  addi r4, r31, 0x10
	ctx.r[4].s64 = ctx.r[31].s64 + 16;
	// 822EE704: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 822EE708: 485080C1  bl 0x827f67c8
	ctx.lr = 0x822EE70C;
	sub_827F67C8(ctx, base);
	// 822EE70C: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 822EE710: 815F0014  lwz r10, 0x14(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 822EE714: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 822EE718: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 822EE71C: 419A001C  beq cr6, 0x822ee738
	if ctx.cr[6].eq {
	pc = 0x822EE738; continue 'dispatch;
	}
	// 822EE720: 38DF000C  addi r6, r31, 0xc
	ctx.r[6].s64 = ctx.r[31].s64 + 12;
	// 822EE724: 808B0010  lwz r4, 0x10(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 822EE728: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 822EE72C: 4BFFF9B5  bl 0x822ee0e0
	ctx.lr = 0x822EE730;
	sub_822EE0E0(ctx, base);
	// 822EE730: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 822EE734: 48000010  b 0x822ee744
	pc = 0x822EE744; continue 'dispatch;
	// 822EE738: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 822EE73C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 822EE740: 917E0004  stw r11, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 822EE744: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 822EE748: 48EB9A70  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822EE750(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822EE750 size=136
    let mut pc: u32 = 0x822EE750;
    'dispatch: loop {
        match pc {
            0x822EE750 => {
    //   block [0x822EE750..0x822EE7D8)
	// 822EE750: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822EE754: 48EB9A15  bl 0x831a8168
	ctx.lr = 0x822EE758;
	sub_831A8130(ctx, base);
	// 822EE758: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822EE75C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 822EE760: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 822EE764: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822EE768: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 822EE76C: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 822EE770: 4BFFFE09  bl 0x822ee578
	ctx.lr = 0x822EE774;
	sub_822EE578(ctx, base);
	// 822EE774: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 822EE778: 389F0010  addi r4, r31, 0x10
	ctx.r[4].s64 = ctx.r[31].s64 + 16;
	// 822EE77C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 822EE780: 48508049  bl 0x827f67c8
	ctx.lr = 0x822EE784;
	sub_827F67C8(ctx, base);
	// 822EE784: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 822EE788: 83E10050  lwz r31, 0x50(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 822EE78C: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 822EE790: 419A0034  beq cr6, 0x822ee7c4
	if ctx.cr[6].eq {
	pc = 0x822EE7C4; continue 'dispatch;
	}
	// 822EE794: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 822EE798: 419A0010  beq cr6, 0x822ee7a8
	if ctx.cr[6].eq {
	pc = 0x822EE7A8; continue 'dispatch;
	}
	// 822EE79C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 822EE7A0: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 822EE7A4: 4BFFF25D  bl 0x822eda00
	ctx.lr = 0x822EE7A8;
	sub_822EDA00(ctx, base);
	// 822EE7A8: 809F0010  lwz r4, 0x10(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 822EE7AC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 822EE7B0: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 822EE7B4: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 822EE7B8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 822EE7BC: 4E800421  bctrl
	ctx.lr = 0x822EE7C0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 822EE7C0: 4800000C  b 0x822ee7cc
	pc = 0x822EE7CC; continue 'dispatch;
	// 822EE7C4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 822EE7C8: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 822EE7CC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 822EE7D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 822EE7D4: 48EB99E4  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822EE7D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822EE7D8 size=88
    let mut pc: u32 = 0x822EE7D8;
    'dispatch: loop {
        match pc {
            0x822EE7D8 => {
    //   block [0x822EE7D8..0x822EE830)
	// 822EE7D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822EE7DC: 48EB9991  bl 0x831a816c
	ctx.lr = 0x822EE7E0;
	sub_831A8130(ctx, base);
	// 822EE7E0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822EE7E4: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 822EE7E8: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 822EE7EC: 7F03E840  cmplw cr6, r3, r29
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[29].u32, &mut ctx.xer);
	// 822EE7F0: 419A0034  beq cr6, 0x822ee824
	if ctx.cr[6].eq {
	pc = 0x822EE824; continue 'dispatch;
	}
	// 822EE7F4: 7FDF1850  subf r30, r31, r3
	ctx.r[30].s64 = ctx.r[3].s64 - ctx.r[31].s64;
	// 822EE7F8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 822EE7FC: 419A0018  beq cr6, 0x822ee814
	if ctx.cr[6].eq {
	pc = 0x822EE814; continue 'dispatch;
	}
	// 822EE800: 7C7EF82E  lwzx r3, r30, r31
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 822EE804: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 822EE808: 907F0000  stw r3, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 822EE80C: 41820008  beq 0x822ee814
	if ctx.cr[0].eq {
	pc = 0x822EE814; continue 'dispatch;
	}
	// 822EE810: 4BFF9A39  bl 0x822e8248
	ctx.lr = 0x822EE814;
	sub_822E8248(ctx, base);
	// 822EE814: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 822EE818: 7D7EFA14  add r11, r30, r31
	ctx.r[11].u64 = ctx.r[30].u64 + ctx.r[31].u64;
	// 822EE81C: 7F0BE840  cmplw cr6, r11, r29
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[29].u32, &mut ctx.xer);
	// 822EE820: 409AFFD8  bne cr6, 0x822ee7f8
	if !ctx.cr[6].eq {
	pc = 0x822EE7F8; continue 'dispatch;
	}
	// 822EE824: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822EE828: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 822EE82C: 48EB9990  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822EE830(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822EE830 size=88
    let mut pc: u32 = 0x822EE830;
    'dispatch: loop {
        match pc {
            0x822EE830 => {
    //   block [0x822EE830..0x822EE888)
	// 822EE830: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822EE834: 48EB9931  bl 0x831a8164
	ctx.lr = 0x822EE838;
	sub_831A8130(ctx, base);
	// 822EE838: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822EE83C: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 822EE840: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 822EE844: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 822EE848: 7FFDFB78  mr r29, r31
	ctx.r[29].u64 = ctx.r[31].u64;
	// 822EE84C: 7F9EE378  mr r30, r28
	ctx.r[30].u64 = ctx.r[28].u64;
	// 822EE850: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 822EE854: 419A0024  beq cr6, 0x822ee878
	if ctx.cr[6].eq {
	pc = 0x822EE878; continue 'dispatch;
	}
	// 822EE858: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 822EE85C: 419A0010  beq cr6, 0x822ee86c
	if ctx.cr[6].eq {
	pc = 0x822EE86C; continue 'dispatch;
	}
	// 822EE860: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 822EE864: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 822EE868: 48B05399  bl 0x82df3c00
	ctx.lr = 0x822EE86C;
	sub_82DF3C00(ctx, base);
	// 822EE86C: 37BDFFFF  addic. r29, r29, -1
	ctx.xer.ca = (ctx.r[29].u32 > (!(-1 as u32)));
	ctx.r[29].s64 = ctx.r[29].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 822EE870: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 822EE874: 4082FFE4  bne 0x822ee858
	if !ctx.cr[0].eq {
	pc = 0x822EE858; continue 'dispatch;
	}
	// 822EE878: 57EB103A  slwi r11, r31, 2
	ctx.r[11].u32 = ctx.r[31].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 822EE87C: 7C6BE214  add r3, r11, r28
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[28].u64;
	// 822EE880: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 822EE884: 48EB9930  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822EE888(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822EE888 size=92
    let mut pc: u32 = 0x822EE888;
    'dispatch: loop {
        match pc {
            0x822EE888 => {
    //   block [0x822EE888..0x822EE8E4)
	// 822EE888: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822EE88C: 48EB98E1  bl 0x831a816c
	ctx.lr = 0x822EE890;
	sub_831A8130(ctx, base);
	// 822EE890: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822EE894: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 822EE898: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822EE89C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 822EE8A0: 2B1E0004  cmplwi cr6, r30, 4
	ctx.cr[6].compare_u32(ctx.r[30].u32, 4 as u32, &mut ctx.xer);
	// 822EE8A4: 41980008  blt cr6, 0x822ee8ac
	if ctx.cr[6].lt {
	pc = 0x822EE8AC; continue 'dispatch;
	}
	// 822EE8A8: 48031F61  bl 0x82320808
	ctx.lr = 0x822EE8AC;
	sub_82320808(ctx, base);
	// 822EE8AC: 57AB063F  clrlwi. r11, r29, 0x18
	ctx.r[11].u64 = ctx.r[29].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 822EE8B0: 57CBE8FA  rlwinm r11, r30, 0x1d, 3, 0x1d
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000007u64;
	// 822EE8B4: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 822EE8B8: 57CA06FE  clrlwi r10, r30, 0x1b
	ctx.r[10].u64 = ctx.r[30].u32 as u64 & 0x0000001Fu64;
	// 822EE8BC: 7D2A5030  slw r10, r9, r10
	if (ctx.r[10].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[9].u32) << ((ctx.r[10].u8 & 0x1F) as u32)) as u64;
	}
	// 822EE8C0: 7D2BF82E  lwzx r9, r11, r31
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 822EE8C4: 4182000C  beq 0x822ee8d0
	if ctx.cr[0].eq {
	pc = 0x822EE8D0; continue 'dispatch;
	}
	// 822EE8C8: 7D4A4B78  or r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[9].u64;
	// 822EE8CC: 48000008  b 0x822ee8d4
	pc = 0x822EE8D4; continue 'dispatch;
	// 822EE8D0: 7D2A5078  andc r10, r9, r10
	ctx.r[10].u64 = ctx.r[9].u64 & !ctx.r[10].u64;
	// 822EE8D4: 7D4BF92E  stwx r10, r11, r31
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32), ctx.r[10].u32) };
	// 822EE8D8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822EE8DC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 822EE8E0: 48EB98DC  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822EE8E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822EE8E8 size=100
    let mut pc: u32 = 0x822EE8E8;
    'dispatch: loop {
        match pc {
            0x822EE8E8 => {
    //   block [0x822EE8E8..0x822EE94C)
	// 822EE8E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822EE8EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822EE8F0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 822EE8F4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822EE8F8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822EE8FC: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 822EE900: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 822EE904: 2B1F0004  cmplwi cr6, r31, 4
	ctx.cr[6].compare_u32(ctx.r[31].u32, 4 as u32, &mut ctx.xer);
	// 822EE908: 41980008  blt cr6, 0x822ee910
	if ctx.cr[6].lt {
	pc = 0x822EE910; continue 'dispatch;
	}
	// 822EE90C: 48031EFD  bl 0x82320808
	ctx.lr = 0x822EE910;
	sub_82320808(ctx, base);
	// 822EE910: 57EBE8FA  rlwinm r11, r31, 0x1d, 3, 0x1d
	ctx.r[11].u64 = ctx.r[31].u32 as u64 & 0x00000007u64;
	// 822EE914: 57EA06FE  clrlwi r10, r31, 0x1b
	ctx.r[10].u64 = ctx.r[31].u32 as u64 & 0x0000001Fu64;
	// 822EE918: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 822EE91C: 7D2A5030  slw r10, r9, r10
	if (ctx.r[10].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[9].u32) << ((ctx.r[10].u8 & 0x1F) as u32)) as u64;
	}
	// 822EE920: 7D6BF02E  lwzx r11, r11, r30
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 822EE924: 7D4B5838  and r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 & ctx.r[11].u64;
	// 822EE928: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 822EE92C: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 822EE930: 69630001  xori r3, r11, 1
	ctx.r[3].u64 = ctx.r[11].u64 ^ 1;
	// 822EE934: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 822EE938: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822EE93C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822EE940: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 822EE944: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822EE948: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822EE950(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822EE950 size=872
    let mut pc: u32 = 0x822EE950;
    'dispatch: loop {
        match pc {
            0x822EE950 => {
    //   block [0x822EE950..0x822EECB8)
	// 822EE950: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822EE954: 48EB9801  bl 0x831a8154
	ctx.lr = 0x822EE958;
	sub_831A8130(ctx, base);
	// 822EE958: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822EE95C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822EE960: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 822EE964: 7CC43378  mr r4, r6
	ctx.r[4].u64 = ctx.r[6].u64;
	// 822EE968: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 822EE96C: 7CB72B78  mr r23, r5
	ctx.r[23].u64 = ctx.r[5].u64;
	// 822EE970: 48B05291  bl 0x82df3c00
	ctx.lr = 0x822EE974;
	sub_82DF3C00(ctx, base);
	// 822EE974: 813F0004  lwz r9, 4(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 822EE978: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 822EE97C: 409A000C  bne cr6, 0x822ee988
	if !ctx.cr[6].eq {
	pc = 0x822EE988; continue 'dispatch;
	}
	// 822EE980: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 822EE984: 48000010  b 0x822ee994
	pc = 0x822EE994; continue 'dispatch;
	// 822EE988: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 822EE98C: 7D695850  subf r11, r9, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 822EE990: 7D681670  srawi r8, r11, 2
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[8].s64 = (ctx.r[11].s32 >> 2) as i64;
	// 822EE994: 2B170000  cmplwi cr6, r23, 0
	ctx.cr[6].compare_u32(ctx.r[23].u32, 0 as u32, &mut ctx.xer);
	// 822EE998: 419A0310  beq cr6, 0x822eeca8
	if ctx.cr[6].eq {
	pc = 0x822EECA8; continue 'dispatch;
	}
	// 822EE99C: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 822EE9A0: 409A000C  bne cr6, 0x822ee9ac
	if !ctx.cr[6].eq {
	pc = 0x822EE9AC; continue 'dispatch;
	}
	// 822EE9A4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 822EE9A8: 48000010  b 0x822ee9b8
	pc = 0x822EE9B8; continue 'dispatch;
	// 822EE9AC: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 822EE9B0: 7D695850  subf r11, r9, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 822EE9B4: 7D6B1670  srawi r11, r11, 2
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 2) as i64;
	// 822EE9B8: 3D403FFF  lis r10, 0x3fff
	ctx.r[10].s64 = 1073676288;
	// 822EE9BC: 614AFFFF  ori r10, r10, 0xffff
	ctx.r[10].u64 = ctx.r[10].u64 | 65535;
	// 822EE9C0: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 822EE9C4: 7F0BB840  cmplw cr6, r11, r23
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[23].u32, &mut ctx.xer);
	// 822EE9C8: 4098000C  bge cr6, 0x822ee9d4
	if !ctx.cr[6].lt {
	pc = 0x822EE9D4; continue 'dispatch;
	}
	// 822EE9CC: 488C9355  bl 0x82bb7d20
	ctx.lr = 0x822EE9D0;
	sub_82BB7D20(ctx, base);
	// 822EE9D0: 480002D8  b 0x822eeca8
	pc = 0x822EECA8; continue 'dispatch;
	// 822EE9D4: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 822EE9D8: 409A000C  bne cr6, 0x822ee9e4
	if !ctx.cr[6].eq {
	pc = 0x822EE9E4; continue 'dispatch;
	}
	// 822EE9DC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 822EE9E0: 48000010  b 0x822ee9f0
	pc = 0x822EE9F0; continue 'dispatch;
	// 822EE9E4: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 822EE9E8: 7D695850  subf r11, r9, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 822EE9EC: 7D6B1670  srawi r11, r11, 2
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 2) as i64;
	// 822EE9F0: 7D6BBA14  add r11, r11, r23
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[23].u64;
	// 822EE9F4: 7F085840  cmplw cr6, r8, r11
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[11].u32, &mut ctx.xer);
	// 822EE9F8: 40980180  bge cr6, 0x822eeb78
	if !ctx.cr[6].lt {
	pc = 0x822EEB78; continue 'dispatch;
	}
	// 822EE9FC: 550BF87E  srwi r11, r8, 1
	ctx.r[11].u32 = ctx.r[8].u32.wrapping_shr(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 822EEA00: 7D4B5050  subf r10, r11, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 822EEA04: 7F0A4040  cmplw cr6, r10, r8
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[8].u32, &mut ctx.xer);
	// 822EEA08: 4098000C  bge cr6, 0x822eea14
	if !ctx.cr[6].lt {
	pc = 0x822EEA14; continue 'dispatch;
	}
	// 822EEA0C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 822EEA10: 48000008  b 0x822eea18
	pc = 0x822EEA18; continue 'dispatch;
	// 822EEA14: 7D6B4214  add r11, r11, r8
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[8].u64;
	// 822EEA18: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 822EEA1C: 409A000C  bne cr6, 0x822eea28
	if !ctx.cr[6].eq {
	pc = 0x822EEA28; continue 'dispatch;
	}
	// 822EEA20: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 822EEA24: 48000010  b 0x822eea34
	pc = 0x822EEA34; continue 'dispatch;
	// 822EEA28: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 822EEA2C: 7D495050  subf r10, r9, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[9].s64;
	// 822EEA30: 7D4A1670  srawi r10, r10, 2
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[10].s32 >> 2) as i64;
	// 822EEA34: 7D4ABA14  add r10, r10, r23
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[23].u64;
	// 822EEA38: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 822EEA3C: 40980024  bge cr6, 0x822eea60
	if !ctx.cr[6].lt {
	pc = 0x822EEA60; continue 'dispatch;
	}
	// 822EEA40: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 822EEA44: 409A000C  bne cr6, 0x822eea50
	if !ctx.cr[6].eq {
	pc = 0x822EEA50; continue 'dispatch;
	}
	// 822EEA48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 822EEA4C: 48000010  b 0x822eea5c
	pc = 0x822EEA5C; continue 'dispatch;
	// 822EEA50: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 822EEA54: 7D695850  subf r11, r9, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 822EEA58: 7D6B1670  srawi r11, r11, 2
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 2) as i64;
	// 822EEA5C: 7D6BBA14  add r11, r11, r23
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[23].u64;
	// 822EEA60: 5578103A  slwi r24, r11, 2
	ctx.r[24].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[24].u64 = ctx.r[24].u32 as u64;
	// 822EEA64: 3F208335  lis r25, -0x7ccb
	ctx.r[25].s64 = -2093678592;
	// 822EEA68: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 822EEA6C: 7F06C378  mr r6, r24
	ctx.r[6].u64 = ctx.r[24].u64;
	// 822EEA70: 388B08B0  addi r4, r11, 0x8b0
	ctx.r[4].s64 = ctx.r[11].s64 + 2224;
	// 822EEA74: 38A0002D  li r5, 0x2d
	ctx.r[5].s64 = 45;
	// 822EEA78: 8079110C  lwz r3, 0x110c(r25)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(4364 as u32) ) } as u64;
	// 822EEA7C: 48B0364D  bl 0x82df20c8
	ctx.lr = 0x822EEA80;
	sub_82DF20C8(ctx, base);
	// 822EEA80: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 822EEA84: 83DF0004  lwz r30, 4(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 822EEA88: 7F7DDB78  mr r29, r27
	ctx.r[29].u64 = ctx.r[27].u64;
	// 822EEA8C: 48000020  b 0x822eeaac
	pc = 0x822EEAAC; continue 'dispatch;
	// 822EEA90: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 822EEA94: 419A0010  beq cr6, 0x822eeaa4
	if ctx.cr[6].eq {
	pc = 0x822EEAA4; continue 'dispatch;
	}
	// 822EEA98: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 822EEA9C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 822EEAA0: 48B05161  bl 0x82df3c00
	ctx.lr = 0x822EEAA4;
	sub_82DF3C00(ctx, base);
	// 822EEAA4: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 822EEAA8: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 822EEAAC: 7F1ED040  cmplw cr6, r30, r26
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[26].u32, &mut ctx.xer);
	// 822EEAB0: 409AFFE0  bne cr6, 0x822eea90
	if !ctx.cr[6].eq {
	pc = 0x822EEA90; continue 'dispatch;
	}
	// 822EEAB4: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 822EEAB8: 7EE5BB78  mr r5, r23
	ctx.r[5].u64 = ctx.r[23].u64;
	// 822EEABC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 822EEAC0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822EEAC4: 4BFFFD6D  bl 0x822ee830
	ctx.lr = 0x822EEAC8;
	sub_822EE830(ctx, base);
	// 822EEAC8: 83BF0008  lwz r29, 8(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 822EEACC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 822EEAD0: 7F1AE840  cmplw cr6, r26, r29
	ctx.cr[6].compare_u32(ctx.r[26].u32, ctx.r[29].u32, &mut ctx.xer);
	// 822EEAD4: 419A002C  beq cr6, 0x822eeb00
	if ctx.cr[6].eq {
	pc = 0x822EEB00; continue 'dispatch;
	}
	// 822EEAD8: 7F9ED050  subf r28, r30, r26
	ctx.r[28].s64 = ctx.r[26].s64 - ctx.r[30].s64;
	// 822EEADC: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 822EEAE0: 419A0010  beq cr6, 0x822eeaf0
	if ctx.cr[6].eq {
	pc = 0x822EEAF0; continue 'dispatch;
	}
	// 822EEAE4: 7C9CF214  add r4, r28, r30
	ctx.r[4].u64 = ctx.r[28].u64 + ctx.r[30].u64;
	// 822EEAE8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 822EEAEC: 48B05115  bl 0x82df3c00
	ctx.lr = 0x822EEAF0;
	sub_82DF3C00(ctx, base);
	// 822EEAF0: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 822EEAF4: 7D7CF214  add r11, r28, r30
	ctx.r[11].u64 = ctx.r[28].u64 + ctx.r[30].u64;
	// 822EEAF8: 7F0BE840  cmplw cr6, r11, r29
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[29].u32, &mut ctx.xer);
	// 822EEAFC: 409AFFE0  bne cr6, 0x822eeadc
	if !ctx.cr[6].eq {
	pc = 0x822EEADC; continue 'dispatch;
	}
	// 822EEB00: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 822EEB04: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 822EEB08: 409A000C  bne cr6, 0x822eeb14
	if !ctx.cr[6].eq {
	pc = 0x822EEB14; continue 'dispatch;
	}
	// 822EEB0C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 822EEB10: 48000010  b 0x822eeb20
	pc = 0x822EEB20; continue 'dispatch;
	// 822EEB14: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 822EEB18: 7D4B5050  subf r10, r11, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 822EEB1C: 7D4A1670  srawi r10, r10, 2
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[10].s32 >> 2) as i64;
	// 822EEB20: 7F8ABA14  add r28, r10, r23
	ctx.r[28].u64 = ctx.r[10].u64 + ctx.r[23].u64;
	// 822EEB24: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 822EEB28: 419A0034  beq cr6, 0x822eeb5c
	if ctx.cr[6].eq {
	pc = 0x822EEB5C; continue 'dispatch;
	}
	// 822EEB2C: 83BF0008  lwz r29, 8(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 822EEB30: 7D7E5B78  mr r30, r11
	ctx.r[30].u64 = ctx.r[11].u64;
	// 822EEB34: 7F0BE840  cmplw cr6, r11, r29
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[29].u32, &mut ctx.xer);
	// 822EEB38: 419A0018  beq cr6, 0x822eeb50
	if ctx.cr[6].eq {
	pc = 0x822EEB50; continue 'dispatch;
	}
	// 822EEB3C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 822EEB40: 48B048E9  bl 0x82df3428
	ctx.lr = 0x822EEB44;
	sub_82DF3428(ctx, base);
	// 822EEB44: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 822EEB48: 7F1EE840  cmplw cr6, r30, r29
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[29].u32, &mut ctx.xer);
	// 822EEB4C: 409AFFF0  bne cr6, 0x822eeb3c
	if !ctx.cr[6].eq {
	pc = 0x822EEB3C; continue 'dispatch;
	}
	// 822EEB50: 8079110C  lwz r3, 0x110c(r25)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(4364 as u32) ) } as u64;
	// 822EEB54: 809F0004  lwz r4, 4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 822EEB58: 48B03631  bl 0x82df2188
	ctx.lr = 0x822EEB5C;
	sub_82DF2188(ctx, base);
	// 822EEB5C: 578B103A  slwi r11, r28, 2
	ctx.r[11].u32 = ctx.r[28].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 822EEB60: 937F0004  stw r27, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[27].u32 ) };
	// 822EEB64: 7D58DA14  add r10, r24, r27
	ctx.r[10].u64 = ctx.r[24].u64 + ctx.r[27].u64;
	// 822EEB68: 7D6BDA14  add r11, r11, r27
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[27].u64;
	// 822EEB6C: 915F000C  stw r10, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 822EEB70: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 822EEB74: 48000134  b 0x822eeca8
	pc = 0x822EECA8; continue 'dispatch;
	// 822EEB78: 83DF0008  lwz r30, 8(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 822EEB7C: 7D7AF050  subf r11, r26, r30
	ctx.r[11].s64 = ctx.r[30].s64 - ctx.r[26].s64;
	// 822EEB80: 7D6B1670  srawi r11, r11, 2
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 2) as i64;
	// 822EEB84: 7F0BB840  cmplw cr6, r11, r23
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[23].u32, &mut ctx.xer);
	// 822EEB88: 40980090  bge cr6, 0x822eec18
	if !ctx.cr[6].lt {
	pc = 0x822EEC18; continue 'dispatch;
	}
	// 822EEB8C: 56FD103A  slwi r29, r23, 2
	ctx.r[29].u32 = ctx.r[23].u32.wrapping_shl(2);
	ctx.r[29].u64 = ctx.r[29].u32 as u64;
	// 822EEB90: 7F1AF040  cmplw cr6, r26, r30
	ctx.cr[6].compare_u32(ctx.r[26].u32, ctx.r[30].u32, &mut ctx.xer);
	// 822EEB94: 7F9DD214  add r28, r29, r26
	ctx.r[28].u64 = ctx.r[29].u64 + ctx.r[26].u64;
	// 822EEB98: 419A002C  beq cr6, 0x822eebc4
	if ctx.cr[6].eq {
	pc = 0x822EEBC4; continue 'dispatch;
	}
	// 822EEB9C: 7F7DE050  subf r27, r29, r28
	ctx.r[27].s64 = ctx.r[28].s64 - ctx.r[29].s64;
	// 822EEBA0: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 822EEBA4: 419A0010  beq cr6, 0x822eebb4
	if ctx.cr[6].eq {
	pc = 0x822EEBB4; continue 'dispatch;
	}
	// 822EEBA8: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 822EEBAC: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 822EEBB0: 48B05051  bl 0x82df3c00
	ctx.lr = 0x822EEBB4;
	sub_82DF3C00(ctx, base);
	// 822EEBB4: 3B7B0004  addi r27, r27, 4
	ctx.r[27].s64 = ctx.r[27].s64 + 4;
	// 822EEBB8: 3B9C0004  addi r28, r28, 4
	ctx.r[28].s64 = ctx.r[28].s64 + 4;
	// 822EEBBC: 7F1BF040  cmplw cr6, r27, r30
	ctx.cr[6].compare_u32(ctx.r[27].u32, ctx.r[30].u32, &mut ctx.xer);
	// 822EEBC0: 409AFFE0  bne cr6, 0x822eeba0
	if !ctx.cr[6].eq {
	pc = 0x822EEBA0; continue 'dispatch;
	}
	// 822EEBC4: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 822EEBC8: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 822EEBCC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822EEBD0: 7D7A2050  subf r11, r26, r4
	ctx.r[11].s64 = ctx.r[4].s64 - ctx.r[26].s64;
	// 822EEBD4: 7D6B1670  srawi r11, r11, 2
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 2) as i64;
	// 822EEBD8: 7CABB850  subf r5, r11, r23
	ctx.r[5].s64 = ctx.r[23].s64 - ctx.r[11].s64;
	// 822EEBDC: 4BFFFC55  bl 0x822ee830
	ctx.lr = 0x822EEBE0;
	sub_822EE830(ctx, base);
	// 822EEBE0: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 822EEBE4: 7F5CD378  mr r28, r26
	ctx.r[28].u64 = ctx.r[26].u64;
	// 822EEBE8: 7D6BEA14  add r11, r11, r29
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 822EEBEC: 7FDD5850  subf r30, r29, r11
	ctx.r[30].s64 = ctx.r[11].s64 - ctx.r[29].s64;
	// 822EEBF0: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 822EEBF4: 7F1AF040  cmplw cr6, r26, r30
	ctx.cr[6].compare_u32(ctx.r[26].u32, ctx.r[30].u32, &mut ctx.xer);
	// 822EEBF8: 419A00B0  beq cr6, 0x822eeca8
	if ctx.cr[6].eq {
	pc = 0x822EECA8; continue 'dispatch;
	}
	// 822EEBFC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 822EEC00: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 822EEC04: 48B04FCD  bl 0x82df3bd0
	ctx.lr = 0x822EEC08;
	sub_82DF3BD0(ctx, base);
	// 822EEC08: 3B9C0004  addi r28, r28, 4
	ctx.r[28].s64 = ctx.r[28].s64 + 4;
	// 822EEC0C: 7F1CF040  cmplw cr6, r28, r30
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[30].u32, &mut ctx.xer);
	// 822EEC10: 409AFFEC  bne cr6, 0x822eebfc
	if !ctx.cr[6].eq {
	pc = 0x822EEBFC; continue 'dispatch;
	}
	// 822EEC14: 48000094  b 0x822eeca8
	pc = 0x822EECA8; continue 'dispatch;
	// 822EEC18: 56F9103A  slwi r25, r23, 2
	ctx.r[25].u32 = ctx.r[23].u32.wrapping_shl(2);
	ctx.r[25].u64 = ctx.r[25].u32 as u64;
	// 822EEC1C: 7FDCF378  mr r28, r30
	ctx.r[28].u64 = ctx.r[30].u64;
	// 822EEC20: 7FB9F050  subf r29, r25, r30
	ctx.r[29].s64 = ctx.r[30].s64 - ctx.r[25].s64;
	// 822EEC24: 7FBBEB78  mr r27, r29
	ctx.r[27].u64 = ctx.r[29].u64;
	// 822EEC28: 7F1DF040  cmplw cr6, r29, r30
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[30].u32, &mut ctx.xer);
	// 822EEC2C: 419A0028  beq cr6, 0x822eec54
	if ctx.cr[6].eq {
	pc = 0x822EEC54; continue 'dispatch;
	}
	// 822EEC30: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 822EEC34: 419A0010  beq cr6, 0x822eec44
	if ctx.cr[6].eq {
	pc = 0x822EEC44; continue 'dispatch;
	}
	// 822EEC38: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 822EEC3C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 822EEC40: 48B04FC1  bl 0x82df3c00
	ctx.lr = 0x822EEC44;
	sub_82DF3C00(ctx, base);
	// 822EEC44: 3B7B0004  addi r27, r27, 4
	ctx.r[27].s64 = ctx.r[27].s64 + 4;
	// 822EEC48: 3B9C0004  addi r28, r28, 4
	ctx.r[28].s64 = ctx.r[28].s64 + 4;
	// 822EEC4C: 7F1BF040  cmplw cr6, r27, r30
	ctx.cr[6].compare_u32(ctx.r[27].u32, ctx.r[30].u32, &mut ctx.xer);
	// 822EEC50: 409AFFE0  bne cr6, 0x822eec30
	if !ctx.cr[6].eq {
	pc = 0x822EEC30; continue 'dispatch;
	}
	// 822EEC54: 939F0008  stw r28, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[28].u32 ) };
	// 822EEC58: 7FBFEB78  mr r31, r29
	ctx.r[31].u64 = ctx.r[29].u64;
	// 822EEC5C: 7F1AE840  cmplw cr6, r26, r29
	ctx.cr[6].compare_u32(ctx.r[26].u32, ctx.r[29].u32, &mut ctx.xer);
	// 822EEC60: 419A0020  beq cr6, 0x822eec80
	if ctx.cr[6].eq {
	pc = 0x822EEC80; continue 'dispatch;
	}
	// 822EEC64: 7FDDF050  subf r30, r29, r30
	ctx.r[30].s64 = ctx.r[30].s64 - ctx.r[29].s64;
	// 822EEC68: 3BFFFFFC  addi r31, r31, -4
	ctx.r[31].s64 = ctx.r[31].s64 + -4;
	// 822EEC6C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 822EEC70: 7C7EFA14  add r3, r30, r31
	ctx.r[3].u64 = ctx.r[30].u64 + ctx.r[31].u64;
	// 822EEC74: 48B04F5D  bl 0x82df3bd0
	ctx.lr = 0x822EEC78;
	sub_82DF3BD0(ctx, base);
	// 822EEC78: 7F1FD040  cmplw cr6, r31, r26
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[26].u32, &mut ctx.xer);
	// 822EEC7C: 409AFFEC  bne cr6, 0x822eec68
	if !ctx.cr[6].eq {
	pc = 0x822EEC68; continue 'dispatch;
	}
	// 822EEC80: 7FD9D214  add r30, r25, r26
	ctx.r[30].u64 = ctx.r[25].u64 + ctx.r[26].u64;
	// 822EEC84: 7F5FD378  mr r31, r26
	ctx.r[31].u64 = ctx.r[26].u64;
	// 822EEC88: 7F1AF040  cmplw cr6, r26, r30
	ctx.cr[6].compare_u32(ctx.r[26].u32, ctx.r[30].u32, &mut ctx.xer);
	// 822EEC8C: 419A001C  beq cr6, 0x822eeca8
	if ctx.cr[6].eq {
	pc = 0x822EECA8; continue 'dispatch;
	}
	// 822EEC90: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 822EEC94: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822EEC98: 48B04F39  bl 0x82df3bd0
	ctx.lr = 0x822EEC9C;
	sub_82DF3BD0(ctx, base);
	// 822EEC9C: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 822EECA0: 7F1FF040  cmplw cr6, r31, r30
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[30].u32, &mut ctx.xer);
	// 822EECA4: 409AFFEC  bne cr6, 0x822eec90
	if !ctx.cr[6].eq {
	pc = 0x822EEC90; continue 'dispatch;
	}
	// 822EECA8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 822EECAC: 48B0477D  bl 0x82df3428
	ctx.lr = 0x822EECB0;
	sub_82DF3428(ctx, base);
	// 822EECB0: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 822EECB4: 48EB94F0  b 0x831a81a4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822EECB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822EECB8 size=108
    let mut pc: u32 = 0x822EECB8;
    'dispatch: loop {
        match pc {
            0x822EECB8 => {
    //   block [0x822EECB8..0x822EED24)
	// 822EECB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822EECBC: 48EB94B1  bl 0x831a816c
	ctx.lr = 0x822EECC0;
	sub_831A8130(ctx, base);
	// 822EECC0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822EECC4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 822EECC8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 822EECCC: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 822EECD0: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 822EECD4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 822EECD8: 419A0014  beq cr6, 0x822eecec
	if ctx.cr[6].eq {
	pc = 0x822EECEC; continue 'dispatch;
	}
	// 822EECDC: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 822EECE0: 7D4B5050  subf r10, r11, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 822EECE4: 7D4A1671  srawi. r10, r10, 2
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[10].s32 >> 2) as i64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 822EECE8: 4082000C  bne 0x822eecf4
	if !ctx.cr[0].eq {
	pc = 0x822EECF4; continue 'dispatch;
	}
	// 822EECEC: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 822EECF0: 4800000C  b 0x822eecfc
	pc = 0x822EECFC; continue 'dispatch;
	// 822EECF4: 7D6B2050  subf r11, r11, r4
	ctx.r[11].s64 = ctx.r[4].s64 - ctx.r[11].s64;
	// 822EECF8: 7D7D1670  srawi r29, r11, 2
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[29].s64 = (ctx.r[11].s32 >> 2) as i64;
	// 822EECFC: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 822EED00: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822EED04: 4BFFFC4D  bl 0x822ee950
	ctx.lr = 0x822EED08;
	sub_822EE950(ctx, base);
	// 822EED08: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 822EED0C: 57AB103A  slwi r11, r29, 2
	ctx.r[11].u32 = ctx.r[29].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 822EED10: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 822EED14: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 822EED18: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 822EED1C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 822EED20: 48EB949C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822EED28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822EED28 size=160
    let mut pc: u32 = 0x822EED28;
    'dispatch: loop {
        match pc {
            0x822EED28 => {
    //   block [0x822EED28..0x822EEDC8)
	// 822EED28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822EED2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822EED30: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 822EED34: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822EED38: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822EED3C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822EED40: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 822EED44: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 822EED48: 409A000C  bne cr6, 0x822eed54
	if !ctx.cr[6].eq {
	pc = 0x822EED54; continue 'dispatch;
	}
	// 822EED4C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 822EED50: 48000010  b 0x822eed60
	pc = 0x822EED60; continue 'dispatch;
	// 822EED54: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 822EED58: 7D4B5050  subf r10, r11, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 822EED5C: 7D4A1670  srawi r10, r10, 2
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[10].s32 >> 2) as i64;
	// 822EED60: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 822EED64: 419A0038  beq cr6, 0x822eed9c
	if ctx.cr[6].eq {
	pc = 0x822EED9C; continue 'dispatch;
	}
	// 822EED68: 813F000C  lwz r9, 0xc(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 822EED6C: 7D6B4850  subf r11, r11, r9
	ctx.r[11].s64 = ctx.r[9].s64 - ctx.r[11].s64;
	// 822EED70: 7D6B1670  srawi r11, r11, 2
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 2) as i64;
	// 822EED74: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 822EED78: 40980024  bge cr6, 0x822eed9c
	if !ctx.cr[6].lt {
	pc = 0x822EED9C; continue 'dispatch;
	}
	// 822EED7C: 83DF0008  lwz r30, 8(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 822EED80: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 822EED84: 419A000C  beq cr6, 0x822eed90
	if ctx.cr[6].eq {
	pc = 0x822EED90; continue 'dispatch;
	}
	// 822EED88: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 822EED8C: 48B04E75  bl 0x82df3c00
	ctx.lr = 0x822EED90;
	sub_82DF3C00(ctx, base);
	// 822EED90: 397E0004  addi r11, r30, 4
	ctx.r[11].s64 = ctx.r[30].s64 + 4;
	// 822EED94: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 822EED98: 48000018  b 0x822eedb0
	pc = 0x822EEDB0; continue 'dispatch;
	// 822EED9C: 7C862378  mr r6, r4
	ctx.r[6].u64 = ctx.r[4].u64;
	// 822EEDA0: 80BF0008  lwz r5, 8(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 822EEDA4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 822EEDA8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 822EEDAC: 4BFFFF0D  bl 0x822eecb8
	ctx.lr = 0x822EEDB0;
	sub_822EECB8(ctx, base);
	// 822EEDB0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 822EEDB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822EEDB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822EEDBC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 822EEDC0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822EEDC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822EEDC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822EEDC8 size=128
    let mut pc: u32 = 0x822EEDC8;
    'dispatch: loop {
        match pc {
            0x822EEDC8 => {
    //   block [0x822EEDC8..0x822EEE48)
	// 822EEDC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822EEDCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822EEDD0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822EEDD4: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822EEDD8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822EEDDC: 7CA92B78  mr r9, r5
	ctx.r[9].u64 = ctx.r[5].u64;
	// 822EEDE0: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 822EEDE4: 395F0004  addi r10, r31, 4
	ctx.r[10].s64 = ctx.r[31].s64 + 4;
	// 822EEDE8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 822EEDEC: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 822EEDF0: 910A0000  stw r8, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 822EEDF4: 394AFFFC  addi r10, r10, -4
	ctx.r[10].s64 = ctx.r[10].s64 + -4;
	// 822EEDF8: 4080FFF0  bge 0x822eede8
	if !ctx.cr[0].lt {
	pc = 0x822EEDE8; continue 'dispatch;
	}
	// 822EEDFC: 90810060  stw r4, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[4].u32 ) };
	// 822EEE00: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 822EEE04: 91210064  stw r9, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[9].u32 ) };
	// 822EEE08: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 822EEE0C: 90C10068  stw r6, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[6].u32 ) };
	// 822EEE10: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 822EEE14: 90E1006C  stw r7, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[7].u32 ) };
	// 822EEE18: 4BFF6041  bl 0x822e4e58
	ctx.lr = 0x822EEE1C;
	sub_822E4E58(ctx, base);
	// 822EEE1C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 822EEE20: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822EEE24: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 822EEE28: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 822EEE2C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 822EEE30: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 822EEE34: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 822EEE38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822EEE3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822EEE40: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822EEE44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822EEE48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822EEE48 size=104
    let mut pc: u32 = 0x822EEE48;
    'dispatch: loop {
        match pc {
            0x822EEE48 => {
    //   block [0x822EEE48..0x822EEEB0)
	// 822EEE48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822EEE4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822EEE50: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 822EEE54: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822EEE58: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822EEE5C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822EEE60: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 822EEE64: 4BFFF715  bl 0x822ee578
	ctx.lr = 0x822EEE68;
	sub_822EE578(ctx, base);
	// 822EEE68: 83FF0014  lwz r31, 0x14(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 822EEE6C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 822EEE70: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 822EEE74: 4800001C  b 0x822eee90
	pc = 0x822EEE90; continue 'dispatch;
	// 822EEE78: 388B000C  addi r4, r11, 0xc
	ctx.r[4].s64 = ctx.r[11].s64 + 12;
	// 822EEE7C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 822EEE80: 4BFFFEA9  bl 0x822eed28
	ctx.lr = 0x822EEE84;
	sub_822EED28(ctx, base);
	// 822EEE84: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 822EEE88: 480B2801  bl 0x823a1688
	ctx.lr = 0x822EEE8C;
	sub_823A1688(ctx, base);
	// 822EEE8C: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 822EEE90: 7F0BF840  cmplw cr6, r11, r31
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[31].u32, &mut ctx.xer);
	// 822EEE94: 409AFFE4  bne cr6, 0x822eee78
	if !ctx.cr[6].eq {
	pc = 0x822EEE78; continue 'dispatch;
	}
	// 822EEE98: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 822EEE9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822EEEA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822EEEA4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 822EEEA8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822EEEAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822EEEB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822EEEB0 size=104
    let mut pc: u32 = 0x822EEEB0;
    'dispatch: loop {
        match pc {
            0x822EEEB0 => {
    //   block [0x822EEEB0..0x822EEF18)
	// 822EEEB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822EEEB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822EEEB8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822EEEBC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822EEEC0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822EEEC4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 822EEEC8: 387F001C  addi r3, r31, 0x1c
	ctx.r[3].s64 = ctx.r[31].s64 + 28;
	// 822EEECC: 396BC214  addi r11, r11, -0x3dec
	ctx.r[11].s64 = ctx.r[11].s64 + -15852;
	// 822EEED0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 822EEED4: 4833E0ED  bl 0x8262cfc0
	ctx.lr = 0x822EEED8;
	sub_8262CFC0(ctx, base);
	// 822EEED8: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 822EEEDC: 482C1235  bl 0x825b0110
	ctx.lr = 0x822EEEE0;
	sub_825B0110(ctx, base);
	// 822EEEE0: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 822EEEE4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 822EEEE8: 419A0008  beq cr6, 0x822eeef0
	if ctx.cr[6].eq {
	pc = 0x822EEEF0; continue 'dispatch;
	}
	// 822EEEEC: 4BFF937D  bl 0x822e8268
	ctx.lr = 0x822EEEF0;
	sub_822E8268(ctx, base);
	// 822EEEF0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 822EEEF4: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 822EEEF8: 396B9B84  addi r11, r11, -0x647c
	ctx.r[11].s64 = ctx.r[11].s64 + -25724;
	// 822EEEFC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 822EEF00: 48B04529  bl 0x82df3428
	ctx.lr = 0x822EEF04;
	sub_82DF3428(ctx, base);
	// 822EEF04: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 822EEF08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822EEF0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822EEF10: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822EEF14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822EEF18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822EEF18 size=128
    let mut pc: u32 = 0x822EEF18;
    'dispatch: loop {
        match pc {
            0x822EEF18 => {
    //   block [0x822EEF18..0x822EEF98)
	// 822EEF18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822EEF1C: 48EB924D  bl 0x831a8168
	ctx.lr = 0x822EEF20;
	sub_831A8130(ctx, base);
	// 822EEF20: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822EEF24: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822EEF28: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 822EEF2C: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 822EEF30: 4BFFF649  bl 0x822ee578
	ctx.lr = 0x822EEF34;
	sub_822EE578(ctx, base);
	// 822EEF34: 83DF0014  lwz r30, 0x14(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 822EEF38: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 822EEF3C: 7F0BF040  cmplw cr6, r11, r30
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[30].u32, &mut ctx.xer);
	// 822EEF40: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 822EEF44: 419A004C  beq cr6, 0x822eef90
	if ctx.cr[6].eq {
	pc = 0x822EEF90; continue 'dispatch;
	}
	// 822EEF48: 3BFF000C  addi r31, r31, 0xc
	ctx.r[31].s64 = ctx.r[31].s64 + 12;
	// 822EEF4C: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 822EEF50: 808B0010  lwz r4, 0x10(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 822EEF54: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 822EEF58: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 822EEF5C: 4BFFF185  bl 0x822ee0e0
	ctx.lr = 0x822EEF60;
	sub_822EE0E0(ctx, base);
	// 822EEF60: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 822EEF64: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 822EEF68: 488C41A1  bl 0x82bb3108
	ctx.lr = 0x822EEF6C;
	sub_82BB3108(ctx, base);
	// 822EEF6C: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 822EEF70: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 822EEF74: 419A0008  beq cr6, 0x822eef7c
	if ctx.cr[6].eq {
	pc = 0x822EEF7C; continue 'dispatch;
	}
	// 822EEF78: 4BFD1919  bl 0x822c0890
	ctx.lr = 0x822EEF7C;
	sub_822C0890(ctx, base);
	// 822EEF7C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 822EEF80: 480B2709  bl 0x823a1688
	ctx.lr = 0x822EEF84;
	sub_823A1688(ctx, base);
	// 822EEF84: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 822EEF88: 7F0BF040  cmplw cr6, r11, r30
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[30].u32, &mut ctx.xer);
	// 822EEF8C: 409AFFC0  bne cr6, 0x822eef4c
	if !ctx.cr[6].eq {
	pc = 0x822EEF4C; continue 'dispatch;
	}
	// 822EEF90: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 822EEF94: 48EB9224  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822EEF98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822EEF98 size=96
    let mut pc: u32 = 0x822EEF98;
    'dispatch: loop {
        match pc {
            0x822EEF98 => {
    //   block [0x822EEF98..0x822EEFF8)
	// 822EEF98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822EEF9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822EEFA0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 822EEFA4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822EEFA8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822EEFAC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822EEFB0: 48B0F7E9  bl 0x82dfe798
	ctx.lr = 0x822EEFB4;
	sub_82DFE798(ctx, base);
	// 822EEFB4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 822EEFB8: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 822EEFBC: 396BC214  addi r11, r11, -0x3dec
	ctx.r[11].s64 = ctx.r[11].s64 + -15852;
	// 822EEFC0: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 822EEFC4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 822EEFC8: 93DF000C  stw r30, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[30].u32 ) };
	// 822EEFCC: 484ACC2D  bl 0x8279bbf8
	ctx.lr = 0x822EEFD0;
	sub_8279BBF8(ctx, base);
	// 822EEFD0: 93DF0020  stw r30, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[30].u32 ) };
	// 822EEFD4: 93DF0024  stw r30, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[30].u32 ) };
	// 822EEFD8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822EEFDC: 93DF0028  stw r30, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[30].u32 ) };
	// 822EEFE0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 822EEFE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822EEFE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822EEFEC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 822EEFF0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822EEFF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822EEFF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822EEFF8 size=76
    let mut pc: u32 = 0x822EEFF8;
    'dispatch: loop {
        match pc {
            0x822EEFF8 => {
    //   block [0x822EEFF8..0x822EF044)
	// 822EEFF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822EEFFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822EF000: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 822EF004: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822EF008: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822EF00C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822EF010: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 822EF014: 4BFFFE9D  bl 0x822eeeb0
	ctx.lr = 0x822EF018;
	sub_822EEEB0(ctx, base);
	// 822EF018: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 822EF01C: 4182000C  beq 0x822ef028
	if ctx.cr[0].eq {
	pc = 0x822EF028; continue 'dispatch;
	}
	// 822EF020: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822EF024: 48B033B5  bl 0x82df23d8
	ctx.lr = 0x822EF028;
	sub_82DF23D8(ctx, base);
	// 822EF028: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822EF02C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 822EF030: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822EF034: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822EF038: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 822EF03C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822EF040: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822EF048(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822EF048 size=1764
    let mut pc: u32 = 0x822EF048;
    'dispatch: loop {
        match pc {
            0x822EF048 => {
    //   block [0x822EF048..0x822EF72C)
	// 822EF048: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822EF04C: 48EB90ED  bl 0x831a8138
	ctx.lr = 0x822EF050;
	sub_831A8130(ctx, base);
	// 822EF050: 9421FEE0  stwu r1, -0x120(r1)
	ea = ctx.r[1].u32.wrapping_add(-288 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822EF054: 3FE08335  lis r31, -0x7ccb
	ctx.r[31].s64 = -2093678592;
	// 822EF058: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 822EF05C: 7C912378  mr r17, r4
	ctx.r[17].u64 = ctx.r[4].u64;
	// 822EF060: 3FC08335  lis r30, -0x7ccb
	ctx.r[30].s64 = -2093678592;
	// 822EF064: 817F17E4  lwz r11, 0x17e4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(6116 as u32) ) } as u64;
	// 822EF068: 556A07FF  clrlwi. r10, r11, 0x1f
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 822EF06C: 40820020  bne 0x822ef08c
	if !ctx.cr[0].eq {
	pc = 0x822EF08C; continue 'dispatch;
	}
	// 822EF070: 616B0001  ori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 | 1;
	// 822EF074: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 822EF078: 917F17E4  stw r11, 0x17e4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(6116 as u32), ctx.r[11].u32 ) };
	// 822EF07C: 386AC314  addi r3, r10, -0x3cec
	ctx.r[3].s64 = ctx.r[10].s64 + -15596;
	// 822EF080: 481A0099  bl 0x8248f118
	ctx.lr = 0x822EF084;
	sub_8248F118(ctx, base);
	// 822EF084: 817F17E4  lwz r11, 0x17e4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(6116 as u32) ) } as u64;
	// 822EF088: 907E17E0  stw r3, 0x17e0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(6112 as u32), ctx.r[3].u32 ) };
	// 822EF08C: 556A07BD  rlwinm. r10, r11, 0, 0x1e, 0x1e
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 822EF090: 3E608335  lis r19, -0x7ccb
	ctx.r[19].s64 = -2093678592;
	// 822EF094: 40820020  bne 0x822ef0b4
	if !ctx.cr[0].eq {
	pc = 0x822EF0B4; continue 'dispatch;
	}
	// 822EF098: 616B0002  ori r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u64 | 2;
	// 822EF09C: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 822EF0A0: 917F17E4  stw r11, 0x17e4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(6116 as u32), ctx.r[11].u32 ) };
	// 822EF0A4: 386AC2FC  addi r3, r10, -0x3d04
	ctx.r[3].s64 = ctx.r[10].s64 + -15620;
	// 822EF0A8: 481A0071  bl 0x8248f118
	ctx.lr = 0x822EF0AC;
	sub_8248F118(ctx, base);
	// 822EF0AC: 817F17E4  lwz r11, 0x17e4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(6116 as u32) ) } as u64;
	// 822EF0B0: 907317DC  stw r3, 0x17dc(r19)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[19].u32.wrapping_add(6108 as u32), ctx.r[3].u32 ) };
	// 822EF0B4: 556A077B  rlwinm. r10, r11, 0, 0x1d, 0x1d
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 822EF0B8: 3EA08335  lis r21, -0x7ccb
	ctx.r[21].s64 = -2093678592;
	// 822EF0BC: 40820020  bne 0x822ef0dc
	if !ctx.cr[0].eq {
	pc = 0x822EF0DC; continue 'dispatch;
	}
	// 822EF0C0: 616B0004  ori r11, r11, 4
	ctx.r[11].u64 = ctx.r[11].u64 | 4;
	// 822EF0C4: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 822EF0C8: 917F17E4  stw r11, 0x17e4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(6116 as u32), ctx.r[11].u32 ) };
	// 822EF0CC: 386AC2E0  addi r3, r10, -0x3d20
	ctx.r[3].s64 = ctx.r[10].s64 + -15648;
	// 822EF0D0: 481A0049  bl 0x8248f118
	ctx.lr = 0x822EF0D4;
	sub_8248F118(ctx, base);
	// 822EF0D4: 817F17E4  lwz r11, 0x17e4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(6116 as u32) ) } as u64;
	// 822EF0D8: 907517D8  stw r3, 0x17d8(r21)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[21].u32.wrapping_add(6104 as u32), ctx.r[3].u32 ) };
	// 822EF0DC: 556A0739  rlwinm. r10, r11, 0, 0x1c, 0x1c
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 822EF0E0: 3EC08335  lis r22, -0x7ccb
	ctx.r[22].s64 = -2093678592;
	// 822EF0E4: 40820020  bne 0x822ef104
	if !ctx.cr[0].eq {
	pc = 0x822EF104; continue 'dispatch;
	}
	// 822EF0E8: 616B0008  ori r11, r11, 8
	ctx.r[11].u64 = ctx.r[11].u64 | 8;
	// 822EF0EC: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 822EF0F0: 917F17E4  stw r11, 0x17e4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(6116 as u32), ctx.r[11].u32 ) };
	// 822EF0F4: 386AC2C4  addi r3, r10, -0x3d3c
	ctx.r[3].s64 = ctx.r[10].s64 + -15676;
	// 822EF0F8: 481A0021  bl 0x8248f118
	ctx.lr = 0x822EF0FC;
	sub_8248F118(ctx, base);
	// 822EF0FC: 817F17E4  lwz r11, 0x17e4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(6116 as u32) ) } as u64;
	// 822EF100: 907617D4  stw r3, 0x17d4(r22)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[22].u32.wrapping_add(6100 as u32), ctx.r[3].u32 ) };
	// 822EF104: 556A06F7  rlwinm. r10, r11, 0, 0x1b, 0x1b
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 822EF108: 3E808335  lis r20, -0x7ccb
	ctx.r[20].s64 = -2093678592;
	// 822EF10C: 40820020  bne 0x822ef12c
	if !ctx.cr[0].eq {
	pc = 0x822EF12C; continue 'dispatch;
	}
	// 822EF110: 616B0010  ori r11, r11, 0x10
	ctx.r[11].u64 = ctx.r[11].u64 | 16;
	// 822EF114: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 822EF118: 917F17E4  stw r11, 0x17e4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(6116 as u32), ctx.r[11].u32 ) };
	// 822EF11C: 386AC2A8  addi r3, r10, -0x3d58
	ctx.r[3].s64 = ctx.r[10].s64 + -15704;
	// 822EF120: 4819FFF9  bl 0x8248f118
	ctx.lr = 0x822EF124;
	sub_8248F118(ctx, base);
	// 822EF124: 817F17E4  lwz r11, 0x17e4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(6116 as u32) ) } as u64;
	// 822EF128: 907417D0  stw r3, 0x17d0(r20)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[20].u32.wrapping_add(6096 as u32), ctx.r[3].u32 ) };
	// 822EF12C: 556A06B5  rlwinm. r10, r11, 0, 0x1a, 0x1a
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 822EF130: 3E408335  lis r18, -0x7ccb
	ctx.r[18].s64 = -2093678592;
	// 822EF134: 40820020  bne 0x822ef154
	if !ctx.cr[0].eq {
	pc = 0x822EF154; continue 'dispatch;
	}
	// 822EF138: 616B0020  ori r11, r11, 0x20
	ctx.r[11].u64 = ctx.r[11].u64 | 32;
	// 822EF13C: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 822EF140: 917F17E4  stw r11, 0x17e4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(6116 as u32), ctx.r[11].u32 ) };
	// 822EF144: 386AC294  addi r3, r10, -0x3d6c
	ctx.r[3].s64 = ctx.r[10].s64 + -15724;
	// 822EF148: 4819FFD1  bl 0x8248f118
	ctx.lr = 0x822EF14C;
	sub_8248F118(ctx, base);
	// 822EF14C: 817F17E4  lwz r11, 0x17e4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(6116 as u32) ) } as u64;
	// 822EF150: 907217CC  stw r3, 0x17cc(r18)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[18].u32.wrapping_add(6092 as u32), ctx.r[3].u32 ) };
	// 822EF154: 556A0673  rlwinm. r10, r11, 0, 0x19, 0x19
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 822EF158: 3F008335  lis r24, -0x7ccb
	ctx.r[24].s64 = -2093678592;
	// 822EF15C: 4082001C  bne 0x822ef178
	if !ctx.cr[0].eq {
	pc = 0x822EF178; continue 'dispatch;
	}
	// 822EF160: 616B0040  ori r11, r11, 0x40
	ctx.r[11].u64 = ctx.r[11].u64 | 64;
	// 822EF164: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 822EF168: 917F17E4  stw r11, 0x17e4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(6116 as u32), ctx.r[11].u32 ) };
	// 822EF16C: 386AC280  addi r3, r10, -0x3d80
	ctx.r[3].s64 = ctx.r[10].s64 + -15744;
	// 822EF170: 4819FFA9  bl 0x8248f118
	ctx.lr = 0x822EF174;
	sub_8248F118(ctx, base);
	// 822EF174: 907817C8  stw r3, 0x17c8(r24)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[24].u32.wrapping_add(6088 as u32), ctx.r[3].u32 ) };
	// 822EF178: 3A000001  li r16, 1
	ctx.r[16].s64 = 1;
	// 822EF17C: 807A0000  lwz r3, 0(r26)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 822EF180: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 822EF184: 809E17E0  lwz r4, 0x17e0(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(6112 as u32) ) } as u64;
	// 822EF188: 92010058  stw r16, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[16].u32 ) };
	// 822EF18C: 481A00D5  bl 0x8248f260
	ctx.lr = 0x822EF190;
	sub_8248F260(ctx, base);
	// 822EF190: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 822EF194: 7E238B78  mr r3, r17
	ctx.r[3].u64 = ctx.r[17].u64;
	// 822EF198: 388BC278  addi r4, r11, -0x3d88
	ctx.r[4].s64 = ctx.r[11].s64 + -15752;
	// 822EF19C: 48B04E2D  bl 0x82df3fc8
	ctx.lr = 0x822EF1A0;
	sub_82DF3FC8(ctx, base);
	// 822EF1A0: 3EE08212  lis r23, -0x7dee
	ctx.r[23].s64 = -2112749568;
	// 822EF1A4: 80A10058  lwz r5, 0x58(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 822EF1A8: 8177B230  lwz r11, -0x4dd0(r23)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(-19920 as u32) ) } as u64;
	// 822EF1AC: 7F035800  cmpw cr6, r3, r11
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[11].s32, &mut ctx.xer);
	// 822EF1B0: 419A0018  beq cr6, 0x822ef1c8
	if ctx.cr[6].eq {
	pc = 0x822EF1C8; continue 'dispatch;
	}
	// 822EF1B4: 2F05000E  cmpwi cr6, r5, 0xe
	ctx.cr[6].compare_i32(ctx.r[5].s32, 14, &mut ctx.xer);
	// 822EF1B8: 419A0010  beq cr6, 0x822ef1c8
	if ctx.cr[6].eq {
	pc = 0x822EF1C8; continue 'dispatch;
	}
	// 822EF1BC: 2F05000B  cmpwi cr6, r5, 0xb
	ctx.cr[6].compare_i32(ctx.r[5].s32, 11, &mut ctx.xer);
	// 822EF1C0: 409A0008  bne cr6, 0x822ef1c8
	if !ctx.cr[6].eq {
	pc = 0x822EF1C8; continue 'dispatch;
	}
	// 822EF1C4: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 822EF1C8: 38802004  li r4, 0x2004
	ctx.r[4].s64 = 8196;
	// 822EF1CC: 807A0000  lwz r3, 0(r26)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 822EF1D0: 4819FF71  bl 0x8248f140
	ctx.lr = 0x822EF1D4;
	sub_8248F140(ctx, base);
	// 822EF1D4: 817F17E4  lwz r11, 0x17e4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(6116 as u32) ) } as u64;
	// 822EF1D8: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 822EF1DC: 55690631  rlwinm. r9, r11, 0, 0x18, 0x18
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 822EF1E0: 3BCA17A8  addi r30, r10, 0x17a8
	ctx.r[30].s64 = ctx.r[10].s64 + 6056;
	// 822EF1E4: 40820064  bne 0x822ef248
	if !ctx.cr[0].eq {
	pc = 0x822EF248; continue 'dispatch;
	}
	// 822EF1E8: 616B0080  ori r11, r11, 0x80
	ctx.r[11].u64 = ctx.r[11].u64 | 128;
	// 822EF1EC: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 822EF1F0: 917F17E4  stw r11, 0x17e4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(6116 as u32), ctx.r[11].u32 ) };
	// 822EF1F4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 822EF1F8: 808A6754  lwz r4, 0x6754(r10)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(26452 as u32) ) } as u64;
	// 822EF1FC: 4BFF5D35  bl 0x822e4f30
	ctx.lr = 0x822EF200;
	sub_822E4F30(ctx, base);
	// 822EF200: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 822EF204: 387E0008  addi r3, r30, 8
	ctx.r[3].s64 = ctx.r[30].s64 + 8;
	// 822EF208: 808B6758  lwz r4, 0x6758(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(26456 as u32) ) } as u64;
	// 822EF20C: 4BFF5D25  bl 0x822e4f30
	ctx.lr = 0x822EF210;
	sub_822E4F30(ctx, base);
	// 822EF210: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 822EF214: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 822EF218: 3D208335  lis r9, -0x7ccb
	ctx.r[9].s64 = -2093678592;
	// 822EF21C: 3D008335  lis r8, -0x7ccb
	ctx.r[8].s64 = -2093678592;
	// 822EF220: 387E0010  addi r3, r30, 0x10
	ctx.r[3].s64 = ctx.r[30].s64 + 16;
	// 822EF224: 80EB676C  lwz r7, 0x676c(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(26476 as u32) ) } as u64;
	// 822EF228: 80CA6780  lwz r6, 0x6780(r10)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(26496 as u32) ) } as u64;
	// 822EF22C: 80A9677C  lwz r5, 0x677c(r9)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(26492 as u32) ) } as u64;
	// 822EF230: 80886750  lwz r4, 0x6750(r8)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(26448 as u32) ) } as u64;
	// 822EF234: 4BFFFB95  bl 0x822eedc8
	ctx.lr = 0x822EF238;
	sub_822EEDC8(ctx, base);
	// 822EF238: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 822EF23C: 387E0018  addi r3, r30, 0x18
	ctx.r[3].s64 = ctx.r[30].s64 + 24;
	// 822EF240: 808B6770  lwz r4, 0x6770(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(26480 as u32) ) } as u64;
	// 822EF244: 4BFF5CED  bl 0x822e4f30
	ctx.lr = 0x822EF248;
	sub_822E4F30(ctx, base);
	// 822EF248: 3FA08335  lis r29, -0x7ccb
	ctx.r[29].s64 = -2093678592;
	// 822EF24C: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 822EF250: 3B200000  li r25, 0
	ctx.r[25].s64 = 0;
	// 822EF254: 3B8B1794  addi r28, r11, 0x1794
	ctx.r[28].s64 = ctx.r[11].s64 + 6036;
	// 822EF258: 897D17A4  lbz r11, 0x17a4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(6052 as u32) ) } as u64;
	// 822EF25C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 822EF260: 40820030  bne 0x822ef290
	if !ctx.cr[0].eq {
	pc = 0x822EF290; continue 'dispatch;
	}
	// 822EF264: 3D608325  lis r11, -0x7cdb
	ctx.r[11].s64 = -2094727168;
	// 822EF268: 7F3FCB78  mr r31, r25
	ctx.r[31].u64 = ctx.r[25].u64;
	// 822EF26C: 3B6B4800  addi r27, r11, 0x4800
	ctx.r[27].s64 = ctx.r[11].s64 + 18432;
	// 822EF270: 7C7FD82E  lwzx r3, r31, r27
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[27].u32)) } as u64;
	// 822EF274: 4819FEA5  bl 0x8248f118
	ctx.lr = 0x822EF278;
	sub_8248F118(ctx, base);
	// 822EF278: 7C7FE12E  stwx r3, r31, r28
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[28].u32), ctx.r[3].u32) };
	// 822EF27C: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 822EF280: 2B1F0010  cmplwi cr6, r31, 0x10
	ctx.cr[6].compare_u32(ctx.r[31].u32, 16 as u32, &mut ctx.xer);
	// 822EF284: 4198FFEC  blt cr6, 0x822ef270
	if ctx.cr[6].lt {
	pc = 0x822EF270; continue 'dispatch;
	}
	// 822EF288: 7E0B8378  mr r11, r16
	ctx.r[11].u64 = ctx.r[16].u64;
	// 822EF28C: 997D17A4  stb r11, 0x17a4(r29)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[29].u32.wrapping_add(6052 as u32), ctx.r[11].u8 ) };
	// 822EF290: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 822EF294: 4801A195  bl 0x82309428
	ctx.lr = 0x822EF298;
	sub_82309428(ctx, base);
	// 822EF298: 807A0000  lwz r3, 0(r26)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 822EF29C: 809C0000  lwz r4, 0(r28)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 822EF2A0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 822EF2A4: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 822EF2A8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 822EF2AC: 4E800421  bctrl
	ctx.lr = 0x822EF2B0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 822EF2B0: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 822EF2B4: 41820130  beq 0x822ef3e4
	if ctx.cr[0].eq {
	pc = 0x822EF3E4; continue 'dispatch;
	}
	// 822EF2B8: 93210058  stw r25, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[25].u32 ) };
	// 822EF2BC: 7F3FCB78  mr r31, r25
	ctx.r[31].u64 = ctx.r[25].u64;
	// 822EF2C0: 39610078  addi r11, r1, 0x78
	ctx.r[11].s64 = ctx.r[1].s64 + 120;
	// 822EF2C4: 809C0000  lwz r4, 0(r28)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 822EF2C8: 807A0000  lwz r3, 0(r26)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 822EF2CC: 7FBF5A14  add r29, r31, r11
	ctx.r[29].u64 = ctx.r[31].u64 + ctx.r[11].u64;
	// 822EF2D0: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 822EF2D4: 4819FDA5  bl 0x8248f078
	ctx.lr = 0x822EF2D8;
	sub_8248F078(ctx, base);
	// 822EF2D8: 897D0000  lbz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 822EF2DC: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 822EF2E0: 41820014  beq 0x822ef2f4
	if ctx.cr[0].eq {
	pc = 0x822EF2F4; continue 'dispatch;
	}
	// 822EF2E4: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 822EF2E8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 822EF2EC: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 822EF2F0: 4BFFF599  bl 0x822ee888
	ctx.lr = 0x822EF2F4;
	sub_822EE888(ctx, base);
	// 822EF2F4: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 822EF2F8: 3B9C0004  addi r28, r28, 4
	ctx.r[28].s64 = ctx.r[28].s64 + 4;
	// 822EF2FC: 2B1F0004  cmplwi cr6, r31, 4
	ctx.cr[6].compare_u32(ctx.r[31].u32, 4 as u32, &mut ctx.xer);
	// 822EF300: 4198FFC0  blt cr6, 0x822ef2c0
	if ctx.cr[6].lt {
	pc = 0x822EF2C0; continue 'dispatch;
	}
	// 822EF304: 83810058  lwz r28, 0x58(r1)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 822EF308: 38A10068  addi r5, r1, 0x68
	ctx.r[5].s64 = ctx.r[1].s64 + 104;
	// 822EF30C: 38810080  addi r4, r1, 0x80
	ctx.r[4].s64 = ctx.r[1].s64 + 128;
	// 822EF310: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 822EF314: 93810068  stw r28, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[28].u32 ) };
	// 822EF318: 4822CE91  bl 0x8251c1a8
	ctx.lr = 0x822EF31C;
	sub_8251C1A8(ctx, base);
	// 822EF31C: 81410084  lwz r10, 0x84(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 822EF320: 81610070  lwz r11, 0x70(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	// 822EF324: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 822EF328: 419A000C  beq cr6, 0x822ef334
	if ctx.cr[6].eq {
	pc = 0x822EF334; continue 'dispatch;
	}
	// 822EF32C: 808B0010  lwz r4, 0x10(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 822EF330: 480000E4  b 0x822ef414
	pc = 0x822EF414; continue 'dispatch;
	// 822EF334: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 822EF338: 4BFF5B89  bl 0x822e4ec0
	ctx.lr = 0x822EF33C;
	sub_822E4EC0(ctx, base);
	// 822EF33C: 3BFE0004  addi r31, r30, 4
	ctx.r[31].s64 = ctx.r[30].s64 + 4;
	// 822EF340: 7F3DCB78  mr r29, r25
	ctx.r[29].u64 = ctx.r[25].u64;
	// 822EF344: EBC10070  ld r30, 0x70(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) };
	// 822EF348: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 822EF34C: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 822EF350: 4BFFF599  bl 0x822ee8e8
	ctx.lr = 0x822EF354;
	sub_822EE8E8(ctx, base);
	// 822EF354: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 822EF358: 41820038  beq 0x822ef390
	if ctx.cr[0].eq {
	pc = 0x822EF390; continue 'dispatch;
	}
	// 822EF35C: FBC10070  std r30, 0x70(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[30].u64 ) };
	// 822EF360: 7E0A8378  mr r10, r16
	ctx.r[10].u64 = ctx.r[16].u64;
	// 822EF364: 39610074  addi r11, r1, 0x74
	ctx.r[11].s64 = ctx.r[1].s64 + 116;
	// 822EF368: 7FE9FB78  mr r9, r31
	ctx.r[9].u64 = ctx.r[31].u64;
	// 822EF36C: 810B0000  lwz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 822EF370: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 822EF374: 80E90000  lwz r7, 0(r9)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 822EF378: 3929FFFC  addi r9, r9, -4
	ctx.r[9].s64 = ctx.r[9].s64 + -4;
	// 822EF37C: 7CE84378  or r8, r7, r8
	ctx.r[8].u64 = ctx.r[7].u64 | ctx.r[8].u64;
	// 822EF380: 910B0000  stw r8, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 822EF384: 396BFFFC  addi r11, r11, -4
	ctx.r[11].s64 = ctx.r[11].s64 + -4;
	// 822EF388: 4080FFE4  bge 0x822ef36c
	if !ctx.cr[0].lt {
	pc = 0x822EF36C; continue 'dispatch;
	}
	// 822EF38C: EBC10070  ld r30, 0x70(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) };
	// 822EF390: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 822EF394: 3BFF0008  addi r31, r31, 8
	ctx.r[31].s64 = ctx.r[31].s64 + 8;
	// 822EF398: 2B1D0004  cmplwi cr6, r29, 4
	ctx.cr[6].compare_u32(ctx.r[29].u32, 4 as u32, &mut ctx.xer);
	// 822EF39C: 4198FFAC  blt cr6, 0x822ef348
	if ctx.cr[6].lt {
	pc = 0x822EF348; continue 'dispatch;
	}
	// 822EF3A0: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 822EF3A4: 4BFF5B1D  bl 0x822e4ec0
	ctx.lr = 0x822EF3A8;
	sub_822E4EC0(ctx, base);
	// 822EF3A8: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 822EF3AC: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 822EF3B0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 822EF3B4: E8AA0000  ld r5, 0(r10)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	// 822EF3B8: 806B6740  lwz r3, 0x6740(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(26432 as u32) ) } as u64;
	// 822EF3BC: 4819CFB5  bl 0x8248c370
	ctx.lr = 0x822EF3C0;
	sub_8248C370(ctx, base);
	// 822EF3C0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822EF3C4: 93810070  stw r28, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[28].u32 ) };
	// 822EF3C8: 38A10070  addi r5, r1, 0x70
	ctx.r[5].s64 = ctx.r[1].s64 + 112;
	// 822EF3CC: 93E10074  stw r31, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[31].u32 ) };
	// 822EF3D0: 38810080  addi r4, r1, 0x80
	ctx.r[4].s64 = ctx.r[1].s64 + 128;
	// 822EF3D4: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 822EF3D8: 4822D7B1  bl 0x8251cb88
	ctx.lr = 0x822EF3DC;
	sub_8251CB88(ctx, base);
	// 822EF3DC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 822EF3E0: 48000034  b 0x822ef414
	pc = 0x822EF414; continue 'dispatch;
	// 822EF3E4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 822EF3E8: 7E238B78  mr r3, r17
	ctx.r[3].u64 = ctx.r[17].u64;
	// 822EF3EC: 388BC26C  addi r4, r11, -0x3d94
	ctx.r[4].s64 = ctx.r[11].s64 + -15764;
	// 822EF3F0: 48B04BD9  bl 0x82df3fc8
	ctx.lr = 0x822EF3F4;
	sub_82DF3FC8(ctx, base);
	// 822EF3F4: 8177B230  lwz r11, -0x4dd0(r23)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(-19920 as u32) ) } as u64;
	// 822EF3F8: 7F035800  cmpw cr6, r3, r11
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[11].s32, &mut ctx.xer);
	// 822EF3FC: 419A0010  beq cr6, 0x822ef40c
	if ctx.cr[6].eq {
	pc = 0x822EF40C; continue 'dispatch;
	}
	// 822EF400: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 822EF404: 808B6738  lwz r4, 0x6738(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(26424 as u32) ) } as u64;
	// 822EF408: 4800000C  b 0x822ef414
	pc = 0x822EF414; continue 'dispatch;
	// 822EF40C: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 822EF410: 808B673C  lwz r4, 0x673c(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(26428 as u32) ) } as u64;
	// 822EF414: 807A0000  lwz r3, 0(r26)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 822EF418: 4819FF89  bl 0x8248f3a0
	ctx.lr = 0x822EF41C;
	sub_8248F3A0(ctx, base);
	// 822EF41C: 93210060  stw r25, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[25].u32 ) };
	// 822EF420: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 822EF424: 809817C8  lwz r4, 0x17c8(r24)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(6088 as u32) ) } as u64;
	// 822EF428: 807A0000  lwz r3, 0(r26)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 822EF42C: 4819FE35  bl 0x8248f260
	ctx.lr = 0x822EF430;
	sub_8248F260(ctx, base);
	// 822EF430: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 822EF434: 418200B4  beq 0x822ef4e8
	if ctx.cr[0].eq {
	pc = 0x822EF4E8; continue 'dispatch;
	}
	// 822EF438: 807A0000  lwz r3, 0(r26)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 822EF43C: 48221BA5  bl 0x82510fe0
	ctx.lr = 0x822EF440;
	sub_82510FE0(ctx, base);
	// 822EF440: 81610060  lwz r11, 0x60(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 822EF444: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822EF448: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 822EF44C: 419A0018  beq cr6, 0x822ef464
	if ctx.cr[6].eq {
	pc = 0x822EF464; continue 'dispatch;
	}
	// 822EF450: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 822EF454: 409A0088  bne cr6, 0x822ef4dc
	if !ctx.cr[6].eq {
	pc = 0x822EF4DC; continue 'dispatch;
	}
	// 822EF458: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 822EF45C: 83EB66D8  lwz r31, 0x66d8(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(26328 as u32) ) } as u64;
	// 822EF460: 4800006C  b 0x822ef4cc
	pc = 0x822EF4CC; continue 'dispatch;
	// 822EF464: 38610078  addi r3, r1, 0x78
	ctx.r[3].s64 = ctx.r[1].s64 + 120;
	// 822EF468: 4BFF5A59  bl 0x822e4ec0
	ctx.lr = 0x822EF46C;
	sub_822E4EC0(ctx, base);
	// 822EF46C: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 822EF470: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 822EF474: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 822EF478: 808B675C  lwz r4, 0x675c(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(26460 as u32) ) } as u64;
	// 822EF47C: 4BFF5AB5  bl 0x822e4f30
	ctx.lr = 0x822EF480;
	sub_822E4F30(ctx, base);
	// 822EF480: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 822EF484: E8BE0000  ld r5, 0(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) };
	// 822EF488: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822EF48C: E88B0000  ld r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 822EF490: 4819CFB1  bl 0x8248c440
	ctx.lr = 0x822EF494;
	sub_8248C440(ctx, base);
	// 822EF494: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822EF498: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 822EF49C: 4BFF5A25  bl 0x822e4ec0
	ctx.lr = 0x822EF4A0;
	sub_822E4EC0(ctx, base);
	// 822EF4A0: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 822EF4A4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 822EF4A8: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 822EF4AC: 808B6760  lwz r4, 0x6760(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(26464 as u32) ) } as u64;
	// 822EF4B0: 4BFF5A81  bl 0x822e4f30
	ctx.lr = 0x822EF4B4;
	sub_822E4F30(ctx, base);
	// 822EF4B4: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 822EF4B8: E8BE0000  ld r5, 0(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) };
	// 822EF4BC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822EF4C0: E88B0000  ld r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 822EF4C4: 4819CEAD  bl 0x8248c370
	ctx.lr = 0x822EF4C8;
	sub_8248C370(ctx, base);
	// 822EF4C8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822EF4CC: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 822EF4D0: 807A0000  lwz r3, 0(r26)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 822EF4D4: 38802009  li r4, 0x2009
	ctx.r[4].s64 = 8201;
	// 822EF4D8: 4819FCD9  bl 0x8248f1b0
	ctx.lr = 0x822EF4DC;
	sub_8248F1B0(ctx, base);
	// 822EF4DC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 822EF4E0: 807A0000  lwz r3, 0(r26)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 822EF4E4: 4819FEBD  bl 0x8248f3a0
	ctx.lr = 0x822EF4E8;
	sub_8248F3A0(ctx, base);
	// 822EF4E8: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 822EF4EC: 484BDCFD  bl 0x827ad1e8
	ctx.lr = 0x822EF4F0;
	sub_827AD1E8(ctx, base);
	// 822EF4F0: 9B210051  stb r25, 0x51(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(81 as u32), ctx.r[25].u8 ) };
	// 822EF4F4: 38A10051  addi r5, r1, 0x51
	ctx.r[5].s64 = ctx.r[1].s64 + 81;
	// 822EF4F8: 809317DC  lwz r4, 0x17dc(r19)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(6108 as u32) ) } as u64;
	// 822EF4FC: 807A0000  lwz r3, 0(r26)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 822EF500: 4819FB79  bl 0x8248f078
	ctx.lr = 0x822EF504;
	sub_8248F078(ctx, base);
	// 822EF504: 89610051  lbz r11, 0x51(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(81 as u32) ) } as u64;
	// 822EF508: 38802005  li r4, 0x2005
	ctx.r[4].s64 = 8197;
	// 822EF50C: 807A0000  lwz r3, 0(r26)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 822EF510: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 822EF514: 5565DFFE  rlwinm r5, r11, 0x1b, 0x1f, 0x1f
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 822EF518: 4819FC99  bl 0x8248f1b0
	ctx.lr = 0x822EF51C;
	sub_8248F1B0(ctx, base);
	// 822EF51C: 9B210052  stb r25, 0x52(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(82 as u32), ctx.r[25].u8 ) };
	// 822EF520: 38A10052  addi r5, r1, 0x52
	ctx.r[5].s64 = ctx.r[1].s64 + 82;
	// 822EF524: 809517D8  lwz r4, 0x17d8(r21)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(6104 as u32) ) } as u64;
	// 822EF528: 807A0000  lwz r3, 0(r26)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 822EF52C: 4819FB4D  bl 0x8248f078
	ctx.lr = 0x822EF530;
	sub_8248F078(ctx, base);
	// 822EF530: 38802007  li r4, 0x2007
	ctx.r[4].s64 = 8199;
	// 822EF534: 88A10052  lbz r5, 0x52(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(82 as u32) ) } as u64;
	// 822EF538: 807A0000  lwz r3, 0(r26)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 822EF53C: 4819FC75  bl 0x8248f1b0
	ctx.lr = 0x822EF540;
	sub_8248F1B0(ctx, base);
	// 822EF540: 9B210053  stb r25, 0x53(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(83 as u32), ctx.r[25].u8 ) };
	// 822EF544: 38A10053  addi r5, r1, 0x53
	ctx.r[5].s64 = ctx.r[1].s64 + 83;
	// 822EF548: 809617D4  lwz r4, 0x17d4(r22)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(6100 as u32) ) } as u64;
	// 822EF54C: 807A0000  lwz r3, 0(r26)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 822EF550: 4819FB29  bl 0x8248f078
	ctx.lr = 0x822EF554;
	sub_8248F078(ctx, base);
	// 822EF554: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 822EF558: 4182001C  beq 0x822ef574
	if ctx.cr[0].eq {
	pc = 0x822EF574; continue 'dispatch;
	}
	// 822EF55C: 89610053  lbz r11, 0x53(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(83 as u32) ) } as u64;
	// 822EF560: 38802006  li r4, 0x2006
	ctx.r[4].s64 = 8198;
	// 822EF564: 807A0000  lwz r3, 0(r26)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 822EF568: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 822EF56C: 5565DFFE  rlwinm r5, r11, 0x1b, 0x1f, 0x1f
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 822EF570: 4819FC41  bl 0x8248f1b0
	ctx.lr = 0x822EF574;
	sub_8248F1B0(ctx, base);
	// 822EF574: 9B210050  stb r25, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[25].u8 ) };
	// 822EF578: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 822EF57C: 7E238B78  mr r3, r17
	ctx.r[3].u64 = ctx.r[17].u64;
	// 822EF580: 388BC260  addi r4, r11, -0x3da0
	ctx.r[4].s64 = ctx.r[11].s64 + -15776;
	// 822EF584: 48B04A45  bl 0x82df3fc8
	ctx.lr = 0x822EF588;
	sub_82DF3FC8(ctx, base);
	// 822EF588: 8177B230  lwz r11, -0x4dd0(r23)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(-19920 as u32) ) } as u64;
	// 822EF58C: 7F035800  cmpw cr6, r3, r11
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[11].s32, &mut ctx.xer);
	// 822EF590: 807A0000  lwz r3, 0(r26)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 822EF594: 419A0018  beq cr6, 0x822ef5ac
	if ctx.cr[6].eq {
	pc = 0x822EF5AC; continue 'dispatch;
	}
	// 822EF598: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 822EF59C: 3880200C  li r4, 0x200c
	ctx.r[4].s64 = 8204;
	// 822EF5A0: 4819FC11  bl 0x8248f1b0
	ctx.lr = 0x822EF5A4;
	sub_8248F1B0(ctx, base);
	// 822EF5A4: 9A010050  stb r16, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[16].u8 ) };
	// 822EF5A8: 48000028  b 0x822ef5d0
	pc = 0x822EF5D0; continue 'dispatch;
	// 822EF5AC: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 822EF5B0: 809417D0  lwz r4, 0x17d0(r20)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(6096 as u32) ) } as u64;
	// 822EF5B4: 4819FAC5  bl 0x8248f078
	ctx.lr = 0x822EF5B8;
	sub_8248F078(ctx, base);
	// 822EF5B8: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 822EF5BC: 41820014  beq 0x822ef5d0
	if ctx.cr[0].eq {
	pc = 0x822EF5D0; continue 'dispatch;
	}
	// 822EF5C0: 3880200C  li r4, 0x200c
	ctx.r[4].s64 = 8204;
	// 822EF5C4: 88A10050  lbz r5, 0x50(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 822EF5C8: 807A0000  lwz r3, 0(r26)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 822EF5CC: 4819FBE5  bl 0x8248f1b0
	ctx.lr = 0x822EF5D0;
	sub_8248F1B0(ctx, base);
	// 822EF5D0: 89610050  lbz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 822EF5D4: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 822EF5D8: 4182004C  beq 0x822ef624
	if ctx.cr[0].eq {
	pc = 0x822EF624; continue 'dispatch;
	}
	// 822EF5DC: 807A0000  lwz r3, 0(r26)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 822EF5E0: 48221A01  bl 0x82510fe0
	ctx.lr = 0x822EF5E4;
	sub_82510FE0(ctx, base);
	// 822EF5E4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822EF5E8: 38610078  addi r3, r1, 0x78
	ctx.r[3].s64 = ctx.r[1].s64 + 120;
	// 822EF5EC: 4BFF58D5  bl 0x822e4ec0
	ctx.lr = 0x822EF5F0;
	sub_822E4EC0(ctx, base);
	// 822EF5F0: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 822EF5F4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 822EF5F8: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 822EF5FC: 808B684C  lwz r4, 0x684c(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(26700 as u32) ) } as u64;
	// 822EF600: 4BFF5931  bl 0x822e4f30
	ctx.lr = 0x822EF604;
	sub_822E4F30(ctx, base);
	// 822EF604: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 822EF608: E8BE0000  ld r5, 0(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) };
	// 822EF60C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822EF610: E88B0000  ld r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 822EF614: 4819CD5D  bl 0x8248c370
	ctx.lr = 0x822EF618;
	sub_8248C370(ctx, base);
	// 822EF618: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 822EF61C: 807A0000  lwz r3, 0(r26)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 822EF620: 4819FD81  bl 0x8248f3a0
	ctx.lr = 0x822EF624;
	sub_8248F3A0(ctx, base);
	// 822EF624: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 822EF628: 7E238B78  mr r3, r17
	ctx.r[3].u64 = ctx.r[17].u64;
	// 822EF62C: 388BC258  addi r4, r11, -0x3da8
	ctx.r[4].s64 = ctx.r[11].s64 + -15784;
	// 822EF630: 48B04999  bl 0x82df3fc8
	ctx.lr = 0x822EF634;
	sub_82DF3FC8(ctx, base);
	// 822EF634: 8177B230  lwz r11, -0x4dd0(r23)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(-19920 as u32) ) } as u64;
	// 822EF638: 7F035800  cmpw cr6, r3, r11
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[11].s32, &mut ctx.xer);
	// 822EF63C: 419A0014  beq cr6, 0x822ef650
	if ctx.cr[6].eq {
	pc = 0x822EF650; continue 'dispatch;
	}
	// 822EF640: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 822EF644: 807A0000  lwz r3, 0(r26)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 822EF648: 3880200D  li r4, 0x200d
	ctx.r[4].s64 = 8205;
	// 822EF64C: 4819FB65  bl 0x8248f1b0
	ctx.lr = 0x822EF650;
	sub_8248F1B0(ctx, base);
	// 822EF650: 9B210050  stb r25, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[25].u8 ) };
	// 822EF654: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 822EF658: 7E238B78  mr r3, r17
	ctx.r[3].u64 = ctx.r[17].u64;
	// 822EF65C: 388BC24C  addi r4, r11, -0x3db4
	ctx.r[4].s64 = ctx.r[11].s64 + -15796;
	// 822EF660: 48B04969  bl 0x82df3fc8
	ctx.lr = 0x822EF664;
	sub_82DF3FC8(ctx, base);
	// 822EF664: 8177B230  lwz r11, -0x4dd0(r23)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(-19920 as u32) ) } as u64;
	// 822EF668: 7F035800  cmpw cr6, r3, r11
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[11].s32, &mut ctx.xer);
	// 822EF66C: 807A0000  lwz r3, 0(r26)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 822EF670: 419A000C  beq cr6, 0x822ef67c
	if ctx.cr[6].eq {
	pc = 0x822EF67C; continue 'dispatch;
	}
	// 822EF674: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 822EF678: 48000020  b 0x822ef698
	pc = 0x822EF698; continue 'dispatch;
	// 822EF67C: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 822EF680: 809217CC  lwz r4, 0x17cc(r18)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[18].u32.wrapping_add(6092 as u32) ) } as u64;
	// 822EF684: 4819F9F5  bl 0x8248f078
	ctx.lr = 0x822EF688;
	sub_8248F078(ctx, base);
	// 822EF688: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 822EF68C: 41820014  beq 0x822ef6a0
	if ctx.cr[0].eq {
	pc = 0x822EF6A0; continue 'dispatch;
	}
	// 822EF690: 88A10050  lbz r5, 0x50(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 822EF694: 807A0000  lwz r3, 0(r26)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 822EF698: 38802010  li r4, 0x2010
	ctx.r[4].s64 = 8208;
	// 822EF69C: 4819FB15  bl 0x8248f1b0
	ctx.lr = 0x822EF6A0;
	sub_8248F1B0(ctx, base);
	// 822EF6A0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 822EF6A4: 7E238B78  mr r3, r17
	ctx.r[3].u64 = ctx.r[17].u64;
	// 822EF6A8: 388BC240  addi r4, r11, -0x3dc0
	ctx.r[4].s64 = ctx.r[11].s64 + -15808;
	// 822EF6AC: 48B0491D  bl 0x82df3fc8
	ctx.lr = 0x822EF6B0;
	sub_82DF3FC8(ctx, base);
	// 822EF6B0: 8177B230  lwz r11, -0x4dd0(r23)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(-19920 as u32) ) } as u64;
	// 822EF6B4: 7F035800  cmpw cr6, r3, r11
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[11].s32, &mut ctx.xer);
	// 822EF6B8: 419A0014  beq cr6, 0x822ef6cc
	if ctx.cr[6].eq {
	pc = 0x822EF6CC; continue 'dispatch;
	}
	// 822EF6BC: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 822EF6C0: 807A0000  lwz r3, 0(r26)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 822EF6C4: 38802011  li r4, 0x2011
	ctx.r[4].s64 = 8209;
	// 822EF6C8: 4819FAE9  bl 0x8248f1b0
	ctx.lr = 0x822EF6CC;
	sub_8248F1B0(ctx, base);
	// 822EF6CC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 822EF6D0: 7E238B78  mr r3, r17
	ctx.r[3].u64 = ctx.r[17].u64;
	// 822EF6D4: 388BC22C  addi r4, r11, -0x3dd4
	ctx.r[4].s64 = ctx.r[11].s64 + -15828;
	// 822EF6D8: 48B048F1  bl 0x82df3fc8
	ctx.lr = 0x822EF6DC;
	sub_82DF3FC8(ctx, base);
	// 822EF6DC: 8177B230  lwz r11, -0x4dd0(r23)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(-19920 as u32) ) } as u64;
	// 822EF6E0: 7F035800  cmpw cr6, r3, r11
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[11].s32, &mut ctx.xer);
	// 822EF6E4: 419A0014  beq cr6, 0x822ef6f8
	if ctx.cr[6].eq {
	pc = 0x822EF6F8; continue 'dispatch;
	}
	// 822EF6E8: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 822EF6EC: 807A0000  lwz r3, 0(r26)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 822EF6F0: 38802012  li r4, 0x2012
	ctx.r[4].s64 = 8210;
	// 822EF6F4: 4819FABD  bl 0x8248f1b0
	ctx.lr = 0x822EF6F8;
	sub_8248F1B0(ctx, base);
	// 822EF6F8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 822EF6FC: 7E238B78  mr r3, r17
	ctx.r[3].u64 = ctx.r[17].u64;
	// 822EF700: 388BC21C  addi r4, r11, -0x3de4
	ctx.r[4].s64 = ctx.r[11].s64 + -15844;
	// 822EF704: 48B048C5  bl 0x82df3fc8
	ctx.lr = 0x822EF708;
	sub_82DF3FC8(ctx, base);
	// 822EF708: 8177B230  lwz r11, -0x4dd0(r23)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(-19920 as u32) ) } as u64;
	// 822EF70C: 7F035800  cmpw cr6, r3, r11
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[11].s32, &mut ctx.xer);
	// 822EF710: 419A0014  beq cr6, 0x822ef724
	if ctx.cr[6].eq {
	pc = 0x822EF724; continue 'dispatch;
	}
	// 822EF714: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 822EF718: 807A0000  lwz r3, 0(r26)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 822EF71C: 38802013  li r4, 0x2013
	ctx.r[4].s64 = 8211;
	// 822EF720: 4819FA91  bl 0x8248f1b0
	ctx.lr = 0x822EF724;
	sub_8248F1B0(ctx, base);
	// 822EF724: 38210120  addi r1, r1, 0x120
	ctx.r[1].s64 = ctx.r[1].s64 + 288;
	// 822EF728: 48EB8A60  b 0x831a8188
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822EF730(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x822EF730 size=1300
    let mut pc: u32 = 0x822EF730;
    'dispatch: loop {
        match pc {
            0x822EF730 => {
    //   block [0x822EF730..0x822EFC44)
	// 822EF730: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822EF734: 48EB8A0D  bl 0x831a8140
	ctx.lr = 0x822EF738;
	sub_831A8130(ctx, base);
	// 822EF738: DBE1FF80  stfd f31, -0x80(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-128 as u32), ctx.f[31].u64 ) };
	// 822EF73C: 9421FEC0  stwu r1, -0x140(r1)
	ea = ctx.r[1].u32.wrapping_add(-320 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822EF740: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822EF744: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 822EF748: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 822EF74C: 80860000  lwz r4, 0(r6)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 822EF750: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 822EF754: 7CFB3B78  mr r27, r7
	ctx.r[27].u64 = ctx.r[7].u64;
	// 822EF758: 4BFF7AB1  bl 0x822e7208
	ctx.lr = 0x822EF75C;
	sub_822E7208(ctx, base);
	// 822EF75C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 822EF760: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 822EF764: 38810068  addi r4, r1, 0x68
	ctx.r[4].s64 = ctx.r[1].s64 + 104;
	// 822EF768: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 822EF76C: 4BFF7C25  bl 0x822e7390
	ctx.lr = 0x822EF770;
	sub_822E7390(ctx, base);
	// 822EF770: 80610058  lwz r3, 0x58(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 822EF774: 48B0F0F5  bl 0x82dfe868
	ctx.lr = 0x822EF778;
	sub_82DFE868(ctx, base);
	// 822EF778: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 822EF77C: 408204A4  bne 0x822efc20
	if !ctx.cr[0].eq {
	pc = 0x822EFC20; continue 'dispatch;
	}
	// 822EF780: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 822EF784: 419A049C  beq cr6, 0x822efc20
	if ctx.cr[6].eq {
	pc = 0x822EFC20; continue 'dispatch;
	}
	// 822EF788: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 822EF78C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 822EF790: 388BC368  addi r4, r11, -0x3c98
	ctx.r[4].s64 = ctx.r[11].s64 + -15512;
	// 822EF794: 48B04275  bl 0x82df3a08
	ctx.lr = 0x822EF798;
	sub_82DF3A08(ctx, base);
	// 822EF798: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 822EF79C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822EF7A0: 48B03B69  bl 0x82df3308
	ctx.lr = 0x822EF7A4;
	sub_82DF3308(ctx, base);
	// 822EF7A4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 822EF7A8: 48B03C81  bl 0x82df3428
	ctx.lr = 0x822EF7AC;
	sub_82DF3428(ctx, base);
	// 822EF7AC: 83ED0000  lwz r31, 0(r13)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 822EF7B0: 3BC00014  li r30, 0x14
	ctx.r[30].s64 = 20;
	// 822EF7B4: 38A0001A  li r5, 0x1a
	ctx.r[5].s64 = 26;
	// 822EF7B8: 3880001C  li r4, 0x1c
	ctx.r[4].s64 = 28;
	// 822EF7BC: 7C7EF82E  lwzx r3, r30, r31
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 822EF7C0: 48BB0F71  bl 0x82ea0730
	ctx.lr = 0x822EF7C4;
	sub_82EA0730(ctx, base);
	// 822EF7C4: 3960001C  li r11, 0x1c
	ctx.r[11].s64 = 28;
	// 822EF7C8: 38C00002  li r6, 2
	ctx.r[6].s64 = 2;
	// 822EF7CC: B1630004  sth r11, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 822EF7D0: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 822EF7D4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 822EF7D8: 48BB9091  bl 0x82ea8868
	ctx.lr = 0x822EF7DC;
	sub_82EA8868(ctx, base);
	// 822EF7DC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 822EF7E0: 7C721B78  mr r18, r3
	ctx.r[18].u64 = ctx.r[3].u64;
	// 822EF7E4: 7C7EF82E  lwzx r3, r30, r31
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 822EF7E8: 576B063F  clrlwi. r11, r27, 0x18
	ctx.r[11].u64 = ctx.r[27].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 822EF7EC: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 822EF7F0: 38A00006  li r5, 6
	ctx.r[5].s64 = 6;
	// 822EF7F4: 4182002C  beq 0x822ef820
	if ctx.cr[0].eq {
	pc = 0x822EF820; continue 'dispatch;
	}
	// 822EF7F8: 38800048  li r4, 0x48
	ctx.r[4].s64 = 72;
	// 822EF7FC: 48BB0F35  bl 0x82ea0730
	ctx.lr = 0x822EF800;
	sub_82EA0730(ctx, base);
	// 822EF800: 39600048  li r11, 0x48
	ctx.r[11].s64 = 72;
	// 822EF804: B1630004  sth r11, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 822EF808: 48BCBA59  bl 0x82ebb260
	ctx.lr = 0x822EF80C;
	sub_82EBB260(ctx, base);
	// 822EF80C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 822EF810: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 822EF814: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 822EF818: 480B6CA1  bl 0x823a64b8
	ctx.lr = 0x822EF81C;
	sub_823A64B8(ctx, base);
	// 822EF81C: 48000028  b 0x822ef844
	pc = 0x822EF844; continue 'dispatch;
	// 822EF820: 38800088  li r4, 0x88
	ctx.r[4].s64 = 136;
	// 822EF824: 48BB0F0D  bl 0x82ea0730
	ctx.lr = 0x822EF828;
	sub_82EA0730(ctx, base);
	// 822EF828: 39600088  li r11, 0x88
	ctx.r[11].s64 = 136;
	// 822EF82C: B1630004  sth r11, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 822EF830: 48BC8AD1  bl 0x82eb8300
	ctx.lr = 0x822EF834;
	sub_82EB8300(ctx, base);
	// 822EF834: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 822EF838: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 822EF83C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 822EF840: 480B6C79  bl 0x823a64b8
	ctx.lr = 0x822EF844;
	sub_823A64B8(ctx, base);
	// 822EF844: 82E10050  lwz r23, 0x50(r1)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 822EF848: 7E449378  mr r4, r18
	ctx.r[4].u64 = ctx.r[18].u64;
	// 822EF84C: 7EE3BB78  mr r3, r23
	ctx.r[3].u64 = ctx.r[23].u64;
	// 822EF850: 81770000  lwz r11, 0(r23)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(0 as u32) ) } as u64;
	// 822EF854: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 822EF858: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 822EF85C: 4E800421  bctrl
	ctx.lr = 0x822EF860;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 822EF860: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 822EF864: 7EE3BB78  mr r3, r23
	ctx.r[3].u64 = ctx.r[23].u64;
	// 822EF868: 808B71DC  lwz r4, 0x71dc(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(29148 as u32) ) } as u64;
	// 822EF86C: 48BC6295  bl 0x82eb5b00
	ctx.lr = 0x822EF870;
	sub_82EB5B00(ctx, base);
	// 822EF870: 81770000  lwz r11, 0(r23)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(0 as u32) ) } as u64;
	// 822EF874: 7EE3BB78  mr r3, r23
	ctx.r[3].u64 = ctx.r[23].u64;
	// 822EF878: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 822EF87C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 822EF880: 4E800421  bctrl
	ctx.lr = 0x822EF884;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 822EF884: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 822EF888: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 822EF88C: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 822EF890: 386B000C  addi r3, r11, 0xc
	ctx.r[3].s64 = ctx.r[11].s64 + 12;
	// 822EF894: 480B6C25  bl 0x823a64b8
	ctx.lr = 0x822EF898;
	sub_823A64B8(ctx, base);
	// 822EF898: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 822EF89C: 386B714C  addi r3, r11, 0x714c
	ctx.r[3].s64 = ctx.r[11].s64 + 29004;
	// 822EF8A0: 83F70000  lwz r31, 0(r23)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(0 as u32) ) } as u64;
	// 822EF8A4: 48BB8365  bl 0x82ea7c08
	ctx.lr = 0x822EF8A8;
	sub_82EA7C08(ctx, base);
	// 822EF8A8: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 822EF8AC: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 822EF8B0: 7EE3BB78  mr r3, r23
	ctx.r[3].u64 = ctx.r[23].u64;
	// 822EF8B4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 822EF8B8: 4E800421  bctrl
	ctx.lr = 0x822EF8BC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 822EF8BC: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 822EF8C0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822EF8C4: 386BA678  addi r3, r11, -0x5988
	ctx.r[3].s64 = ctx.r[11].s64 + -22920;
	// 822EF8C8: 48BB8341  bl 0x82ea7c08
	ctx.lr = 0x822EF8CC;
	sub_82EA7C08(ctx, base);
	// 822EF8CC: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 822EF8D0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822EF8D4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 822EF8D8: 48BC79E1  bl 0x82eb72b8
	ctx.lr = 0x822EF8DC;
	sub_82EB72B8(ctx, base);
	// 822EF8DC: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 822EF8E0: 41820320  beq 0x822efc00
	if ctx.cr[0].eq {
	pc = 0x822EFC00; continue 'dispatch;
	}
	// 822EF8E4: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 822EF8E8: 386B75CC  addi r3, r11, 0x75cc
	ctx.r[3].s64 = ctx.r[11].s64 + 30156;
	// 822EF8EC: 48BB831D  bl 0x82ea7c08
	ctx.lr = 0x822EF8F0;
	sub_82EA7C08(ctx, base);
	// 822EF8F0: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 822EF8F4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822EF8F8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 822EF8FC: 48BC79BD  bl 0x82eb72b8
	ctx.lr = 0x822EF900;
	sub_82EB72B8(ctx, base);
	// 822EF900: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 822EF904: 3B3E000C  addi r25, r30, 0xc
	ctx.r[25].s64 = ctx.r[30].s64 + 12;
	// 822EF908: 3AC00000  li r22, 0
	ctx.r[22].s64 = 0;
	// 822EF90C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 822EF910: 40990258  ble cr6, 0x822efb68
	if !ctx.cr[6].gt {
	pc = 0x822EFB68; continue 'dispatch;
	}
	// 822EF914: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 822EF918: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 822EF91C: 3B000000  li r24, 0
	ctx.r[24].s64 = 0;
	// 822EF920: 3E808335  lis r20, -0x7ccb
	ctx.r[20].s64 = -2093678592;
	// 822EF924: 3E608335  lis r19, -0x7ccb
	ctx.r[19].s64 = -2093678592;
	// 822EF928: C3EA08A4  lfs f31, 0x8a4(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2212 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 822EF92C: 3AABC1D0  addi r21, r11, -0x3e30
	ctx.r[21].s64 = ctx.r[11].s64 + -15920;
	// 822EF930: 81790000  lwz r11, 0(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 822EF934: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 822EF938: 7D78582E  lwzx r11, r24, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[24].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 822EF93C: 3B8B0008  addi r28, r11, 8
	ctx.r[28].s64 = ctx.r[11].s64 + 8;
	// 822EF940: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 822EF944: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 822EF948: 4099020C  ble cr6, 0x822efb54
	if !ctx.cr[6].gt {
	pc = 0x822EFB54; continue 'dispatch;
	}
	// 822EF94C: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 822EF950: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 822EF954: 39400160  li r10, 0x160
	ctx.r[10].s64 = 352;
	// 822EF958: 392100B0  addi r9, r1, 0xb0
	ctx.r[9].s64 = ctx.r[1].s64 + 176;
	// 822EF95C: 7FDB582E  lwzx r30, r27, r11
	ctx.r[30].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[27].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 822EF960: 13FE50C7  vcmpequd (lvx128) v31, v30, v10
	tmp.u32 = ctx.r[30].u32 + ctx.r[10].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822EFC48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x822EFC48 size=96
    let mut pc: u32 = 0x822EFC48;
    'dispatch: loop {
        match pc {
            0x822EFC48 => {
    //   block [0x822EFC48..0x822EFCA8)
	// 822EFC48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822EFC4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822EFC50: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822EFC54: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822EFC58: 7CA62B78  mr r6, r5
	ctx.r[6].u64 = ctx.r[5].u64;
	// 822EFC5C: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 822EFC60: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 822EFC64: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822EFC68: 4BFF6779  bl 0x822e63e0
	ctx.lr = 0x822EFC6C;
	sub_822E63E0(ctx, base);
	// 822EFC6C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 822EFC70: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 822EFC74: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 822EFC78: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822EFC7C: 3929C388  addi r9, r9, -0x3c78
	ctx.r[9].s64 = ctx.r[9].s64 + -15480;
	// 822EFC80: C00B9F78  lfs f0, -0x6088(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-24712 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 822EFC84: C1AA964C  lfs f13, -0x69b4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-27060 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 822EFC88: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 822EFC8C: D01F0058  stfs f0, 0x58(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), tmp.u32 ) };
	// 822EFC90: D1BF005C  stfs f13, 0x5c(r31)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), tmp.u32 ) };
	// 822EFC94: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 822EFC98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822EFC9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822EFCA0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822EFCA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822EFCA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x822EFCA8 size=4
    let mut pc: u32 = 0x822EFCA8;
    'dispatch: loop {
        match pc {
            0x822EFCA8 => {
    //   block [0x822EFCA8..0x822EFCAC)
	// 822EFCA8: 4BFF62B0  b 0x822e5f58
	sub_822E5F58(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822EFCB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x822EFCB0 size=4
    let mut pc: u32 = 0x822EFCB0;
    'dispatch: loop {
        match pc {
            0x822EFCB0 => {
    //   block [0x822EFCB0..0x822EFCB4)
	// 822EFCB0: 4BFF6300  b 0x822e5fb0
	sub_822E5FB0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822EFCB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x822EFCB8 size=4
    let mut pc: u32 = 0x822EFCB8;
    'dispatch: loop {
        match pc {
            0x822EFCB8 => {
    //   block [0x822EFCB8..0x822EFCBC)
	// 822EFCB8: 4BFF61C0  b 0x822e5e78
	sub_822E5E78(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822EFCC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x822EFCC0 size=4
    let mut pc: u32 = 0x822EFCC0;
    'dispatch: loop {
        match pc {
            0x822EFCC0 => {
    //   block [0x822EFCC0..0x822EFCC4)
	// 822EFCC0: 4BFF64E8  b 0x822e61a8
	sub_822E61A8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822EFCC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822EFCC8 size=108
    let mut pc: u32 = 0x822EFCC8;
    'dispatch: loop {
        match pc {
            0x822EFCC8 => {
    //   block [0x822EFCC8..0x822EFD34)
	// 822EFCC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822EFCCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822EFCD0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822EFCD4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822EFCD8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822EFCDC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 822EFCE0: 419A003C  beq cr6, 0x822efd1c
	if ctx.cr[6].eq {
	pc = 0x822EFD1C; continue 'dispatch;
	}
	// 822EFCE4: 89640010  lbz r11, 0x10(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(16 as u32) ) } as u64;
	// 822EFCE8: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 822EFCEC: 7D6B2214  add r11, r11, r4
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 822EFCF0: 806B000C  lwz r3, 0xc(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 822EFCF4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 822EFCF8: 419A001C  beq cr6, 0x822efd14
	if ctx.cr[6].eq {
	pc = 0x822EFD14; continue 'dispatch;
	}
	// 822EFCFC: 484EA2D5  bl 0x827d9fd0
	ctx.lr = 0x822EFD00;
	sub_827D9FD0(ctx, base);
	// 822EFD00: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 822EFD04: 41820018  beq 0x822efd1c
	if ctx.cr[0].eq {
	pc = 0x822EFD1C; continue 'dispatch;
	}
	// 822EFD08: 48D192B1  bl 0x83008fb8
	ctx.lr = 0x822EFD0C;
	sub_83008FB8(ctx, base);
	// 822EFD0C: 7F1F1840  cmplw cr6, r31, r3
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[3].u32, &mut ctx.xer);
	// 822EFD10: 409A000C  bne cr6, 0x822efd1c
	if !ctx.cr[6].eq {
	pc = 0x822EFD1C; continue 'dispatch;
	}
	// 822EFD14: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 822EFD18: 48000008  b 0x822efd20
	pc = 0x822EFD20; continue 'dispatch;
	// 822EFD1C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 822EFD20: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 822EFD24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822EFD28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822EFD2C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822EFD30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822EFD38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822EFD38 size=140
    let mut pc: u32 = 0x822EFD38;
    'dispatch: loop {
        match pc {
            0x822EFD38 => {
    //   block [0x822EFD38..0x822EFDC4)
	// 822EFD38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822EFD3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822EFD40: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822EFD44: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822EFD48: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822EFD4C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 822EFD50: 548A07FF  clrlwi. r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 822EFD54: 396BC3C0  addi r11, r11, -0x3c40
	ctx.r[11].s64 = ctx.r[11].s64 + -15424;
	// 822EFD58: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 822EFD5C: 41820050  beq 0x822efdac
	if ctx.cr[0].eq {
	pc = 0x822EFDAC; continue 'dispatch;
	}
	// 822EFD60: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 822EFD64: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 822EFD68: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 822EFD6C: 81630044  lwz r11, 0x44(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(68 as u32) ) } as u64;
	// 822EFD70: 81430034  lwz r10, 0x34(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(52 as u32) ) } as u64;
	// 822EFD74: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 822EFD78: 41980018  blt cr6, 0x822efd90
	if ctx.cr[6].lt {
	pc = 0x822EFD90; continue 'dispatch;
	}
	// 822EFD7C: 38C0001F  li r6, 0x1f
	ctx.r[6].s64 = 31;
	// 822EFD80: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 822EFD84: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 822EFD88: 48BB02D9  bl 0x82ea0060
	ctx.lr = 0x822EFD8C;
	sub_82EA0060(ctx, base);
	// 822EFD8C: 48000020  b 0x822efdac
	pc = 0x822EFDAC; continue 'dispatch;
	// 822EFD90: 81430044  lwz r10, 0x44(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(68 as u32) ) } as u64;
	// 822EFD94: 39630040  addi r11, r3, 0x40
	ctx.r[11].s64 = ctx.r[3].s64 + 64;
	// 822EFD98: 81630040  lwz r11, 0x40(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(64 as u32) ) } as u64;
	// 822EFD9C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 822EFDA0: 91430044  stw r10, 0x44(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(68 as u32), ctx.r[10].u32 ) };
	// 822EFDA4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 822EFDA8: 93E30040  stw r31, 0x40(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(64 as u32), ctx.r[31].u32 ) };
	// 822EFDAC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822EFDB0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 822EFDB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822EFDB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822EFDBC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822EFDC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822EFDC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822EFDC8 size=184
    let mut pc: u32 = 0x822EFDC8;
    'dispatch: loop {
        match pc {
            0x822EFDC8 => {
    //   block [0x822EFDC8..0x822EFE80)
	// 822EFDC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822EFDCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822EFDD0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 822EFDD4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822EFDD8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822EFDDC: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 822EFDE0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 822EFDE4: 815F0024  lwz r10, 0x24(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 822EFDE8: 816A000C  lwz r11, 0xc(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 822EFDEC: 4800000C  b 0x822efdf8
	pc = 0x822EFDF8; continue 'dispatch;
	// 822EFDF0: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 822EFDF4: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 822EFDF8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 822EFDFC: 409AFFF4  bne cr6, 0x822efdf0
	if !ctx.cr[6].eq {
	pc = 0x822EFDF0; continue 'dispatch;
	}
	// 822EFE00: 813F0020  lwz r9, 0x20(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 822EFE04: 808A001C  lwz r4, 0x1c(r10)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(28 as u32) ) } as u64;
	// 822EFE08: 8169000C  lwz r11, 0xc(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(12 as u32) ) } as u64;
	// 822EFE0C: 4800000C  b 0x822efe18
	pc = 0x822EFE18; continue 'dispatch;
	// 822EFE10: 7D695B78  mr r9, r11
	ctx.r[9].u64 = ctx.r[11].u64;
	// 822EFE14: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 822EFE18: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 822EFE1C: 409AFFF4  bne cr6, 0x822efe10
	if !ctx.cr[6].eq {
	pc = 0x822EFE10; continue 'dispatch;
	}
	// 822EFE20: 8069001C  lwz r3, 0x1c(r9)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(28 as u32) ) } as u64;
	// 822EFE24: 4819BCF5  bl 0x8248bb18
	ctx.lr = 0x822EFE28;
	sub_8248BB18(ctx, base);
	// 822EFE28: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 822EFE2C: 4182003C  beq 0x822efe68
	if ctx.cr[0].eq {
	pc = 0x822EFE68; continue 'dispatch;
	}
	// 822EFE30: 809F0024  lwz r4, 0x24(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 822EFE34: 8164000C  lwz r11, 0xc(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(12 as u32) ) } as u64;
	// 822EFE38: 4800000C  b 0x822efe44
	pc = 0x822EFE44; continue 'dispatch;
	// 822EFE3C: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 822EFE40: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 822EFE44: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 822EFE48: 409AFFF4  bne cr6, 0x822efe3c
	if !ctx.cr[6].eq {
	pc = 0x822EFE3C; continue 'dispatch;
	}
	// 822EFE4C: 807E0040  lwz r3, 0x40(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(64 as u32) ) } as u64;
	// 822EFE50: 4BFFFE79  bl 0x822efcc8
	ctx.lr = 0x822EFE54;
	sub_822EFCC8(ctx, base);
	// 822EFE54: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 822EFE58: 40820010  bne 0x822efe68
	if !ctx.cr[0].eq {
	pc = 0x822EFE68; continue 'dispatch;
	}
	// 822EFE5C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 822EFE60: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 822EFE64: 48C2E1AD  bl 0x82f1e010
	ctx.lr = 0x822EFE68;
	sub_82F1E010(ctx, base);
	// 822EFE68: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 822EFE6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822EFE70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822EFE74: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 822EFE78: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822EFE7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822EFE80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822EFE80 size=140
    let mut pc: u32 = 0x822EFE80;
    'dispatch: loop {
        match pc {
            0x822EFE80 => {
    //   block [0x822EFE80..0x822EFF0C)
	// 822EFE80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822EFE84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822EFE88: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822EFE8C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822EFE90: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822EFE94: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 822EFE98: 548A07FF  clrlwi. r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 822EFE9C: 396BBA80  addi r11, r11, -0x4580
	ctx.r[11].s64 = ctx.r[11].s64 + -17792;
	// 822EFEA0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 822EFEA4: 41820050  beq 0x822efef4
	if ctx.cr[0].eq {
	pc = 0x822EFEF4; continue 'dispatch;
	}
	// 822EFEA8: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 822EFEAC: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 822EFEB0: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 822EFEB4: 81630064  lwz r11, 0x64(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(100 as u32) ) } as u64;
	// 822EFEB8: 81430034  lwz r10, 0x34(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(52 as u32) ) } as u64;
	// 822EFEBC: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 822EFEC0: 41980018  blt cr6, 0x822efed8
	if ctx.cr[6].lt {
	pc = 0x822EFED8; continue 'dispatch;
	}
	// 822EFEC4: 38C0001F  li r6, 0x1f
	ctx.r[6].s64 = 31;
	// 822EFEC8: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 822EFECC: 38800005  li r4, 5
	ctx.r[4].s64 = 5;
	// 822EFED0: 48BB0191  bl 0x82ea0060
	ctx.lr = 0x822EFED4;
	sub_82EA0060(ctx, base);
	// 822EFED4: 48000020  b 0x822efef4
	pc = 0x822EFEF4; continue 'dispatch;
	// 822EFED8: 81430064  lwz r10, 0x64(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(100 as u32) ) } as u64;
	// 822EFEDC: 39630060  addi r11, r3, 0x60
	ctx.r[11].s64 = ctx.r[3].s64 + 96;
	// 822EFEE0: 81630060  lwz r11, 0x60(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(96 as u32) ) } as u64;
	// 822EFEE4: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 822EFEE8: 91430064  stw r10, 0x64(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(100 as u32), ctx.r[10].u32 ) };
	// 822EFEEC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 822EFEF0: 93E30060  stw r31, 0x60(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(96 as u32), ctx.r[31].u32 ) };
	// 822EFEF4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822EFEF8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 822EFEFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822EFF00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822EFF04: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822EFF08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822EFF10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x822EFF10 size=176
    let mut pc: u32 = 0x822EFF10;
    'dispatch: loop {
        match pc {
            0x822EFF10 => {
    //   block [0x822EFF10..0x822EFFC0)
	// 822EFF10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822EFF14: 48EB8259  bl 0x831a816c
	ctx.lr = 0x822EFF18;
	sub_831A8130(ctx, base);
	// 822EFF18: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822EFF1C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 822EFF20: 90A10090  stw r5, 0x90(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(144 as u32), ctx.r[5].u32 ) };
	// 822EFF24: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 822EFF28: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 822EFF2C: 394AC3DC  addi r10, r10, -0x3c24
	ctx.r[10].s64 = ctx.r[10].s64 + -15396;
	// 822EFF30: 91210080  stw r9, 0x80(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[9].u32 ) };
	// 822EFF34: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 822EFF38: C00BBA78  lfs f0, -0x4588(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-17800 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 822EFF3C: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 822EFF40: D001007C  stfs f0, 0x7c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), tmp.u32 ) };
	// 822EFF44: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 822EFF48: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 822EFF4C: 4BFF5E75  bl 0x822e5dc0
	ctx.lr = 0x822EFF50;
	sub_822E5DC0(ctx, base);
	// 822EFF50: 83E30008  lwz r31, 8(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 822EFF54: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 822EFF58: 419A000C  beq cr6, 0x822eff64
	if ctx.cr[6].eq {
	pc = 0x822EFF64; continue 'dispatch;
	}
	// 822EFF5C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822EFF60: 48BDE8A1  bl 0x82ece800
	ctx.lr = 0x822EFF64;
	sub_82ECE800(ctx, base);
	// 822EFF64: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 822EFF68: 4BFF5E59  bl 0x822e5dc0
	ctx.lr = 0x822EFF6C;
	sub_822E5DC0(ctx, base);
	// 822EFF6C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 822EFF70: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 822EFF74: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 822EFF78: 816B0038  lwz r11, 0x38(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) } as u64;
	// 822EFF7C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 822EFF80: 4E800421  bctrl
	ctx.lr = 0x822EFF84;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 822EFF84: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 822EFF88: 419A000C  beq cr6, 0x822eff94
	if ctx.cr[6].eq {
	pc = 0x822EFF94; continue 'dispatch;
	}
	// 822EFF8C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822EFF90: 48BDD079  bl 0x82ecd008
	ctx.lr = 0x822EFF94;
	sub_82ECD008(ctx, base);
	// 822EFF94: 81610080  lwz r11, 0x80(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(128 as u32) ) } as u64;
	// 822EFF98: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 822EFF9C: 419A0018  beq cr6, 0x822effb4
	if ctx.cr[6].eq {
	pc = 0x822EFFB4; continue 'dispatch;
	}
	// 822EFFA0: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 822EFFA4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 822EFFA8: 4BFF8661  bl 0x822e8608
	ctx.lr = 0x822EFFAC;
	sub_822E8608(ctx, base);
	// 822EFFAC: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 822EFFB0: 48000008  b 0x822effb8
	pc = 0x822EFFB8; continue 'dispatch;
	// 822EFFB4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 822EFFB8: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 822EFFBC: 48EB8200  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822EFFC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x822EFFC0 size=424
    let mut pc: u32 = 0x822EFFC0;
    'dispatch: loop {
        match pc {
            0x822EFFC0 => {
    //   block [0x822EFFC0..0x822F0168)
	// 822EFFC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822EFFC4: 48EB819D  bl 0x831a8160
	ctx.lr = 0x822EFFC8;
	sub_831A8130(ctx, base);
	// 822EFFC8: 9421FE70  stwu r1, -0x190(r1)
	ea = ctx.r[1].u32.wrapping_add(-400 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822EFFCC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822EFFD0: 910100D0  stw r8, 0xd0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(208 as u32), ctx.r[8].u32 ) };
	// 822EFFD4: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 822EFFD8: 91010140  stw r8, 0x140(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(320 as u32), ctx.r[8].u32 ) };
	// 822EFFDC: 7CFA3B78  mr r26, r7
	ctx.r[26].u64 = ctx.r[7].u64;
	// 822EFFE0: 394100E0  addi r10, r1, 0xe0
	ctx.r[10].s64 = ctx.r[1].s64 + 224;
	// 822EFFE4: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 822EFFE8: 3CE08201  lis r7, -0x7dff
	ctx.r[7].s64 = -2113863680;
	// 822EFFEC: C01F005C  lfs f0, 0x5c(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 822EFFF0: 3CC08201  lis r6, -0x7dff
	ctx.r[6].s64 = -2113863680;
	// 822EFFF4: C1BF0058  lfs f13, 0x58(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 822EFFF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 822EFFFC: 13E0E8C7  vcmpequd (lvx128) v31, v0, v29
	tmp.u32 = ctx.r[29].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 822F0000: 38C6C3DC  addi r6, r6, -0x3c24
	ctx.r[6].s64 = ctx.r[6].s64 + -15396;
	// 822F0004: ED80682A  fadds f12, f0, f13
	ctx.f[12].f64 = ((ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64;
	// 822F0008: C1A99528  lfs f13, -0x6ad8(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-27352 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 822F000C: 916100C0  stw r11, 0xc0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(192 as u32), ctx.r[11].u32 ) };
	// 822F0010: C007BA78  lfs f0, -0x4588(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(-17800 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 822F0014: 90C10090  stw r6, 0x90(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(144 as u32), ctx.r[6].u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822F0168(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822F0168 size=108
    let mut pc: u32 = 0x822F0168;
    'dispatch: loop {
        match pc {
            0x822F0168 => {
    //   block [0x822F0168..0x822F01D4)
	// 822F0168: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822F016C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822F0170: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822F0174: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 822F0178: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 822F017C: 38C00038  li r6, 0x38
	ctx.r[6].s64 = 56;
	// 822F0180: 388A08B0  addi r4, r10, 0x8b0
	ctx.r[4].s64 = ctx.r[10].s64 + 2224;
	// 822F0184: 38A0002D  li r5, 0x2d
	ctx.r[5].s64 = 45;
	// 822F0188: 806B110C  lwz r3, 0x110c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4364 as u32) ) } as u64;
	// 822F018C: 48B01F3D  bl 0x82df20c8
	ctx.lr = 0x822F0190;
	sub_82DF20C8(ctx, base);
	// 822F0190: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 822F0194: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 822F0198: 41820008  beq 0x822f01a0
	if ctx.cr[0].eq {
	pc = 0x822F01A0; continue 'dispatch;
	}
	// 822F019C: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 822F01A0: 35630004  addic. r11, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 822F01A4: 41820008  beq 0x822f01ac
	if ctx.cr[0].eq {
	pc = 0x822F01AC; continue 'dispatch;
	}
	// 822F01A8: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 822F01AC: 35630008  addic. r11, r3, 8
	ctx.xer.ca = (ctx.r[3].u32 > (!(8 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 8;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 822F01B0: 41820008  beq 0x822f01b8
	if ctx.cr[0].eq {
	pc = 0x822F01B8; continue 'dispatch;
	}
	// 822F01B4: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 822F01B8: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 822F01BC: 99430035  stb r10, 0x35(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(53 as u32), ctx.r[10].u8 ) };
	// 822F01C0: 99630034  stb r11, 0x34(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(52 as u32), ctx.r[11].u8 ) };
	// 822F01C4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 822F01C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822F01CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822F01D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822F01D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822F01D8 size=88
    let mut pc: u32 = 0x822F01D8;
    'dispatch: loop {
        match pc {
            0x822F01D8 => {
    //   block [0x822F01D8..0x822F0230)
	// 822F01D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822F01DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822F01E0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822F01E4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822F01E8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822F01EC: 4BFFFF7D  bl 0x822f0168
	ctx.lr = 0x822F01F0;
	sub_822F0168(ctx, base);
	// 822F01F0: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 822F01F4: 907F0004  stw r3, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 822F01F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 822F01FC: 99630035  stb r11, 0x35(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(53 as u32), ctx.r[11].u8 ) };
	// 822F0200: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 822F0204: 916B0004  stw r11, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 822F0208: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 822F020C: 916B0000  stw r11, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 822F0210: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 822F0214: 916B0008  stw r11, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 822F0218: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 822F021C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 822F0220: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822F0224: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822F0228: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822F022C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822F0230(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822F0230 size=104
    let mut pc: u32 = 0x822F0230;
    'dispatch: loop {
        match pc {
            0x822F0230 => {
    //   block [0x822F0230..0x822F0298)
	// 822F0230: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822F0234: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822F0238: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822F023C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822F0240: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822F0244: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 822F0248: 396BC3E8  addi r11, r11, -0x3c18
	ctx.r[11].s64 = ctx.r[11].s64 + -15384;
	// 822F024C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 822F0250: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 822F0254: 556A0001  rlwinm. r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 822F0258: 40820020  bne 0x822f0278
	if !ctx.cr[0].eq {
	pc = 0x822F0278; continue 'dispatch;
	}
	// 822F025C: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 822F0260: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 822F0264: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 822F0268: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 822F026C: 55652036  slwi r5, r11, 4
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 822F0270: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 822F0274: 48BB053D  bl 0x82ea07b0
	ctx.lr = 0x822F0278;
	sub_82EA07B0(ctx, base);
	// 822F0278: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 822F027C: 396BC3C0  addi r11, r11, -0x3c40
	ctx.r[11].s64 = ctx.r[11].s64 + -15424;
	// 822F0280: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 822F0284: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 822F0288: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822F028C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822F0290: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822F0294: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822F0298(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822F0298 size=108
    let mut pc: u32 = 0x822F0298;
    'dispatch: loop {
        match pc {
            0x822F0298 => {
    //   block [0x822F0298..0x822F0304)
	// 822F0298: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822F029C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822F02A0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822F02A4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822F02A8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822F02AC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 822F02B0: 396BC3F4  addi r11, r11, -0x3c0c
	ctx.r[11].s64 = ctx.r[11].s64 + -15372;
	// 822F02B4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 822F02B8: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 822F02BC: 556A0001  rlwinm. r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 822F02C0: 40820024  bne 0x822f02e4
	if !ctx.cr[0].eq {
	pc = 0x822F02E4; continue 'dispatch;
	}
	// 822F02C4: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 822F02C8: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 822F02CC: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 822F02D0: 809F0010  lwz r4, 0x10(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 822F02D4: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 822F02D8: 1CAB0030  mulli r5, r11, 0x30
	ctx.r[5].s64 = ctx.r[11].s64 * 48;
	// 822F02DC: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 822F02E0: 48BB04D1  bl 0x82ea07b0
	ctx.lr = 0x822F02E4;
	sub_82EA07B0(ctx, base);
	// 822F02E4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 822F02E8: 396BBA80  addi r11, r11, -0x4580
	ctx.r[11].s64 = ctx.r[11].s64 + -17792;
	// 822F02EC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 822F02F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 822F02F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822F02F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822F02FC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822F0300: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822F0308(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822F0308 size=168
    let mut pc: u32 = 0x822F0308;
    'dispatch: loop {
        match pc {
            0x822F0308 => {
    //   block [0x822F0308..0x822F03B0)
	// 822F0308: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822F030C: 48EB7E61  bl 0x831a816c
	ctx.lr = 0x822F0310;
	sub_831A8130(ctx, base);
	// 822F0310: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822F0314: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 822F0318: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 822F031C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 822F0320: 7FEAFB78  mr r10, r31
	ctx.r[10].u64 = ctx.r[31].u64;
	// 822F0324: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 822F0328: 4800000C  b 0x822f0334
	pc = 0x822F0334; continue 'dispatch;
	// 822F032C: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 822F0330: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 822F0334: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 822F0338: 409AFFF4  bne cr6, 0x822f032c
	if !ctx.cr[6].eq {
	pc = 0x822F032C; continue 'dispatch;
	}
	// 822F033C: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 822F0340: 7FC9F378  mr r9, r30
	ctx.r[9].u64 = ctx.r[30].u64;
	// 822F0344: 808A001C  lwz r4, 0x1c(r10)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(28 as u32) ) } as u64;
	// 822F0348: 4800000C  b 0x822f0354
	pc = 0x822F0354; continue 'dispatch;
	// 822F034C: 7D695B78  mr r9, r11
	ctx.r[9].u64 = ctx.r[11].u64;
	// 822F0350: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 822F0354: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 822F0358: 409AFFF4  bne cr6, 0x822f034c
	if !ctx.cr[6].eq {
	pc = 0x822F034C; continue 'dispatch;
	}
	// 822F035C: 8069001C  lwz r3, 0x1c(r9)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(28 as u32) ) } as u64;
	// 822F0360: 4819B7B9  bl 0x8248bb18
	ctx.lr = 0x822F0364;
	sub_8248BB18(ctx, base);
	// 822F0364: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 822F0368: 41820040  beq 0x822f03a8
	if ctx.cr[0].eq {
	pc = 0x822F03A8; continue 'dispatch;
	}
	// 822F036C: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 822F0370: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 822F0374: 4800000C  b 0x822f0380
	pc = 0x822F0380; continue 'dispatch;
	// 822F0378: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 822F037C: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 822F0380: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 822F0384: 409AFFF4  bne cr6, 0x822f0378
	if !ctx.cr[6].eq {
	pc = 0x822F0378; continue 'dispatch;
	}
	// 822F0388: 807D0114  lwz r3, 0x114(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(276 as u32) ) } as u64;
	// 822F038C: 4BFFF93D  bl 0x822efcc8
	ctx.lr = 0x822F0390;
	sub_822EFCC8(ctx, base);
	// 822F0390: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 822F0394: 40820014  bne 0x822f03a8
	if !ctx.cr[0].eq {
	pc = 0x822F03A8; continue 'dispatch;
	}
	// 822F0398: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 822F039C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 822F03A0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 822F03A4: 48C2DD0D  bl 0x82f1e0b0
	ctx.lr = 0x822F03A8;
	sub_82F1E0B0(ctx, base);
	// 822F03A8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 822F03AC: 48EB7E10  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822F03B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822F03B0 size=184
    let mut pc: u32 = 0x822F03B0;
    'dispatch: loop {
        match pc {
            0x822F03B0 => {
    //   block [0x822F03B0..0x822F0468)
	// 822F03B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822F03B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822F03B8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 822F03BC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822F03C0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822F03C4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 822F03C8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 822F03CC: 815F0024  lwz r10, 0x24(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 822F03D0: 816A000C  lwz r11, 0xc(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 822F03D4: 4800000C  b 0x822f03e0
	pc = 0x822F03E0; continue 'dispatch;
	// 822F03D8: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 822F03DC: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 822F03E0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 822F03E4: 409AFFF4  bne cr6, 0x822f03d8
	if !ctx.cr[6].eq {
	pc = 0x822F03D8; continue 'dispatch;
	}
	// 822F03E8: 813F0020  lwz r9, 0x20(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 822F03EC: 808A001C  lwz r4, 0x1c(r10)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(28 as u32) ) } as u64;
	// 822F03F0: 8169000C  lwz r11, 0xc(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(12 as u32) ) } as u64;
	// 822F03F4: 4800000C  b 0x822f0400
	pc = 0x822F0400; continue 'dispatch;
	// 822F03F8: 7D695B78  mr r9, r11
	ctx.r[9].u64 = ctx.r[11].u64;
	// 822F03FC: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 822F0400: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 822F0404: 409AFFF4  bne cr6, 0x822f03f8
	if !ctx.cr[6].eq {
	pc = 0x822F03F8; continue 'dispatch;
	}
	// 822F0408: 8069001C  lwz r3, 0x1c(r9)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(28 as u32) ) } as u64;
	// 822F040C: 4819B70D  bl 0x8248bb18
	ctx.lr = 0x822F0410;
	sub_8248BB18(ctx, base);
	// 822F0410: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 822F0414: 4182003C  beq 0x822f0450
	if ctx.cr[0].eq {
	pc = 0x822F0450; continue 'dispatch;
	}
	// 822F0418: 809F0024  lwz r4, 0x24(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 822F041C: 8164000C  lwz r11, 0xc(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(12 as u32) ) } as u64;
	// 822F0420: 4800000C  b 0x822f042c
	pc = 0x822F042C; continue 'dispatch;
	// 822F0424: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 822F0428: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 822F042C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 822F0430: 409AFFF4  bne cr6, 0x822f0424
	if !ctx.cr[6].eq {
	pc = 0x822F0424; continue 'dispatch;
	}
	// 822F0434: 807E01A0  lwz r3, 0x1a0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(416 as u32) ) } as u64;
	// 822F0438: 4BFFF891  bl 0x822efcc8
	ctx.lr = 0x822F043C;
	sub_822EFCC8(ctx, base);
	// 822F043C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 822F0440: 40820010  bne 0x822f0450
	if !ctx.cr[0].eq {
	pc = 0x822F0450; continue 'dispatch;
	}
	// 822F0444: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 822F0448: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 822F044C: 48C2DD15  bl 0x82f1e160
	ctx.lr = 0x822F0450;
	sub_82F1E160(ctx, base);
	// 822F0450: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 822F0454: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822F0458: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822F045C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 822F0460: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822F0464: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822F0468(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822F0468 size=152
    let mut pc: u32 = 0x822F0468;
    'dispatch: loop {
        match pc {
            0x822F0468 => {
    //   block [0x822F0468..0x822F0500)
	// 822F0468: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822F046C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822F0470: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 822F0474: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822F0478: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822F047C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822F0480: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 822F0484: 4BFFFDAD  bl 0x822f0230
	ctx.lr = 0x822F0488;
	sub_822F0230(ctx, base);
	// 822F0488: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 822F048C: 41820058  beq 0x822f04e4
	if ctx.cr[0].eq {
	pc = 0x822F04E4; continue 'dispatch;
	}
	// 822F0490: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 822F0494: 419A0050  beq cr6, 0x822f04e4
	if ctx.cr[6].eq {
	pc = 0x822F04E4; continue 'dispatch;
	}
	// 822F0498: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 822F049C: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 822F04A0: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 822F04A4: 81630094  lwz r11, 0x94(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(148 as u32) ) } as u64;
	// 822F04A8: 81430034  lwz r10, 0x34(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(52 as u32) ) } as u64;
	// 822F04AC: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 822F04B0: 41980018  blt cr6, 0x822f04c8
	if ctx.cr[6].lt {
	pc = 0x822F04C8; continue 'dispatch;
	}
	// 822F04B4: 38C0001F  li r6, 0x1f
	ctx.r[6].s64 = 31;
	// 822F04B8: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 822F04BC: 3880000B  li r4, 0xb
	ctx.r[4].s64 = 11;
	// 822F04C0: 48BAFBA1  bl 0x82ea0060
	ctx.lr = 0x822F04C4;
	sub_82EA0060(ctx, base);
	// 822F04C4: 48000020  b 0x822f04e4
	pc = 0x822F04E4; continue 'dispatch;
	// 822F04C8: 81430094  lwz r10, 0x94(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(148 as u32) ) } as u64;
	// 822F04CC: 39630090  addi r11, r3, 0x90
	ctx.r[11].s64 = ctx.r[3].s64 + 144;
	// 822F04D0: 81630090  lwz r11, 0x90(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(144 as u32) ) } as u64;
	// 822F04D4: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 822F04D8: 91430094  stw r10, 0x94(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(148 as u32), ctx.r[10].u32 ) };
	// 822F04DC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 822F04E0: 93E30090  stw r31, 0x90(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(144 as u32), ctx.r[31].u32 ) };
	// 822F04E4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822F04E8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 822F04EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822F04F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822F04F4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 822F04F8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822F04FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822F0500(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822F0500 size=152
    let mut pc: u32 = 0x822F0500;
    'dispatch: loop {
        match pc {
            0x822F0500 => {
    //   block [0x822F0500..0x822F0598)
	// 822F0500: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822F0504: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822F0508: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 822F050C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822F0510: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822F0514: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822F0518: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 822F051C: 4BFFFD7D  bl 0x822f0298
	ctx.lr = 0x822F0520;
	sub_822F0298(ctx, base);
	// 822F0520: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 822F0524: 41820058  beq 0x822f057c
	if ctx.cr[0].eq {
	pc = 0x822F057C; continue 'dispatch;
	}
	// 822F0528: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 822F052C: 419A0050  beq cr6, 0x822f057c
	if ctx.cr[6].eq {
	pc = 0x822F057C; continue 'dispatch;
	}
	// 822F0530: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 822F0534: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 822F0538: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 822F053C: 8163009C  lwz r11, 0x9c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(156 as u32) ) } as u64;
	// 822F0540: 81430034  lwz r10, 0x34(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(52 as u32) ) } as u64;
	// 822F0544: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 822F0548: 41980018  blt cr6, 0x822f0560
	if ctx.cr[6].lt {
	pc = 0x822F0560; continue 'dispatch;
	}
	// 822F054C: 38C0001F  li r6, 0x1f
	ctx.r[6].s64 = 31;
	// 822F0550: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 822F0554: 3880000C  li r4, 0xc
	ctx.r[4].s64 = 12;
	// 822F0558: 48BAFB09  bl 0x82ea0060
	ctx.lr = 0x822F055C;
	sub_82EA0060(ctx, base);
	// 822F055C: 48000020  b 0x822f057c
	pc = 0x822F057C; continue 'dispatch;
	// 822F0560: 8143009C  lwz r10, 0x9c(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(156 as u32) ) } as u64;
	// 822F0564: 39630098  addi r11, r3, 0x98
	ctx.r[11].s64 = ctx.r[3].s64 + 152;
	// 822F0568: 81630098  lwz r11, 0x98(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(152 as u32) ) } as u64;
	// 822F056C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 822F0570: 9143009C  stw r10, 0x9c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(156 as u32), ctx.r[10].u32 ) };
	// 822F0574: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 822F0578: 93E30098  stw r31, 0x98(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(152 as u32), ctx.r[31].u32 ) };
	// 822F057C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822F0580: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 822F0584: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822F0588: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822F058C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 822F0590: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822F0594: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822F0598(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822F0598 size=1016
    let mut pc: u32 = 0x822F0598;
    'dispatch: loop {
        match pc {
            0x822F0598 => {
    //   block [0x822F0598..0x822F0990)
	// 822F0598: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822F059C: 48EB7BBD  bl 0x831a8158
	ctx.lr = 0x822F05A0;
	sub_831A8130(ctx, base);
	// 822F05A0: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822F05A4: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 822F05A8: 7C781B78  mr r24, r3
	ctx.r[24].u64 = ctx.r[3].u64;
	// 822F05AC: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 822F05B0: 93E10104  stw r31, 0x104(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(260 as u32), ctx.r[31].u32 ) };
	// 822F05B4: 897F0035  lbz r11, 0x35(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(53 as u32) ) } as u64;
	// 822F05B8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 822F05BC: 419A0048  beq cr6, 0x822f0604
	if ctx.cr[6].eq {
	pc = 0x822F0604; continue 'dispatch;
	}
	// 822F05C0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 822F05C4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 822F05C8: 388B9620  addi r4, r11, -0x69e0
	ctx.r[4].s64 = ctx.r[11].s64 + -27104;
	// 822F05CC: 4BFD52FD  bl 0x822c58c8
	ctx.lr = 0x822F05D0;
	sub_822C58C8(ctx, base);
	// 822F05D0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 822F05D4: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 822F05D8: 4BFD98D9  bl 0x822c9eb0
	ctx.lr = 0x822F05DC;
	sub_822C9EB0(ctx, base);
	// 822F05DC: 4BFD3CD5  bl 0x822c42b0
	ctx.lr = 0x822F05E0;
	sub_822C42B0(ctx, base);
	// 822F05E0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 822F05E4: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 822F05E8: 396B9600  addi r11, r11, -0x6a00
	ctx.r[11].s64 = ctx.r[11].s64 + -27136;
	// 822F05EC: 91610070  stw r11, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[11].u32 ) };
	// 822F05F0: 4BFD4E81  bl 0x822c5470
	ctx.lr = 0x822F05F4;
	sub_822C5470(ctx, base);
	// 822F05F4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 822F05F8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 822F05FC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 822F0600: 4BFD46E1  bl 0x822c4ce0
	ctx.lr = 0x822F0604;
	sub_822C4CE0(ctx, base);
	// 822F0604: 38610104  addi r3, r1, 0x104
	ctx.r[3].s64 = ctx.r[1].s64 + 260;
	// 822F0608: 7FFAFB78  mr r26, r31
	ctx.r[26].u64 = ctx.r[31].u64;
	// 822F060C: 481F8BED  bl 0x824e91f8
	ctx.lr = 0x822F0610;
	sub_824E91F8(ctx, base);
	// 822F0610: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 822F0614: 894B0035  lbz r10, 0x35(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(53 as u32) ) } as u64;
	// 822F0618: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 822F061C: 83210104  lwz r25, 0x104(r1)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(260 as u32) ) } as u64;
	// 822F0620: 419A000C  beq cr6, 0x822f062c
	if ctx.cr[6].eq {
	pc = 0x822F062C; continue 'dispatch;
	}
	// 822F0624: 839A0008  lwz r28, 8(r26)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 822F0628: 48000028  b 0x822f0650
	pc = 0x822F0650; continue 'dispatch;
	// 822F062C: 815A0008  lwz r10, 8(r26)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 822F0630: 894A0035  lbz r10, 0x35(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(53 as u32) ) } as u64;
	// 822F0634: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 822F0638: 419A000C  beq cr6, 0x822f0644
	if ctx.cr[6].eq {
	pc = 0x822F0644; continue 'dispatch;
	}
	// 822F063C: 7D7C5B78  mr r28, r11
	ctx.r[28].u64 = ctx.r[11].u64;
	// 822F0640: 48000010  b 0x822f0650
	pc = 0x822F0650; continue 'dispatch;
	// 822F0644: 83990008  lwz r28, 8(r25)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(8 as u32) ) } as u64;
	// 822F0648: 7F19D040  cmplw cr6, r25, r26
	ctx.cr[6].compare_u32(ctx.r[25].u32, ctx.r[26].u32, &mut ctx.xer);
	// 822F064C: 409A00DC  bne cr6, 0x822f0728
	if !ctx.cr[6].eq {
	pc = 0x822F0728; continue 'dispatch;
	}
	// 822F0650: 897C0035  lbz r11, 0x35(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(53 as u32) ) } as u64;
	// 822F0654: 83FA0004  lwz r31, 4(r26)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 822F0658: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 822F065C: 409A0008  bne cr6, 0x822f0664
	if !ctx.cr[6].eq {
	pc = 0x822F0664; continue 'dispatch;
	}
	// 822F0660: 93FC0004  stw r31, 4(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 822F0664: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 822F0668: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 822F066C: 7F0AD040  cmplw cr6, r10, r26
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[26].u32, &mut ctx.xer);
	// 822F0670: 409A000C  bne cr6, 0x822f067c
	if !ctx.cr[6].eq {
	pc = 0x822F067C; continue 'dispatch;
	}
	// 822F0674: 938B0004  stw r28, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 822F0678: 4800001C  b 0x822f0694
	pc = 0x822F0694; continue 'dispatch;
	// 822F067C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 822F0680: 7F0BD040  cmplw cr6, r11, r26
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[26].u32, &mut ctx.xer);
	// 822F0684: 409A000C  bne cr6, 0x822f0690
	if !ctx.cr[6].eq {
	pc = 0x822F0690; continue 'dispatch;
	}
	// 822F0688: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 822F068C: 48000008  b 0x822f0694
	pc = 0x822F0694; continue 'dispatch;
	// 822F0690: 939F0008  stw r28, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[28].u32 ) };
	// 822F0694: 813B0004  lwz r9, 4(r27)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 822F0698: 81690000  lwz r11, 0(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 822F069C: 7F0BD040  cmplw cr6, r11, r26
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[26].u32, &mut ctx.xer);
	// 822F06A0: 409A003C  bne cr6, 0x822f06dc
	if !ctx.cr[6].eq {
	pc = 0x822F06DC; continue 'dispatch;
	}
	// 822F06A4: 897C0035  lbz r11, 0x35(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(53 as u32) ) } as u64;
	// 822F06A8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 822F06AC: 419A000C  beq cr6, 0x822f06b8
	if ctx.cr[6].eq {
	pc = 0x822F06B8; continue 'dispatch;
	}
	// 822F06B0: 7FEAFB78  mr r10, r31
	ctx.r[10].u64 = ctx.r[31].u64;
	// 822F06B4: 48000024  b 0x822f06d8
	pc = 0x822F06D8; continue 'dispatch;
	// 822F06B8: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 822F06BC: 7F8AE378  mr r10, r28
	ctx.r[10].u64 = ctx.r[28].u64;
	// 822F06C0: 4800000C  b 0x822f06cc
	pc = 0x822F06CC; continue 'dispatch;
	// 822F06C4: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 822F06C8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 822F06CC: 890B0035  lbz r8, 0x35(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(53 as u32) ) } as u64;
	// 822F06D0: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 822F06D4: 419AFFF0  beq cr6, 0x822f06c4
	if ctx.cr[6].eq {
	pc = 0x822F06C4; continue 'dispatch;
	}
	// 822F06D8: 91490000  stw r10, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 822F06DC: 813B0004  lwz r9, 4(r27)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 822F06E0: 81690008  lwz r11, 8(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) } as u64;
	// 822F06E4: 7F0BD040  cmplw cr6, r11, r26
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[26].u32, &mut ctx.xer);
	// 822F06E8: 409A00D4  bne cr6, 0x822f07bc
	if !ctx.cr[6].eq {
	pc = 0x822F07BC; continue 'dispatch;
	}
	// 822F06EC: 897C0035  lbz r11, 0x35(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(53 as u32) ) } as u64;
	// 822F06F0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 822F06F4: 419A000C  beq cr6, 0x822f0700
	if ctx.cr[6].eq {
	pc = 0x822F0700; continue 'dispatch;
	}
	// 822F06F8: 7FEAFB78  mr r10, r31
	ctx.r[10].u64 = ctx.r[31].u64;
	// 822F06FC: 48000024  b 0x822f0720
	pc = 0x822F0720; continue 'dispatch;
	// 822F0700: 817C0008  lwz r11, 8(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 822F0704: 7F8AE378  mr r10, r28
	ctx.r[10].u64 = ctx.r[28].u64;
	// 822F0708: 4800000C  b 0x822f0714
	pc = 0x822F0714; continue 'dispatch;
	// 822F070C: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 822F0710: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 822F0714: 890B0035  lbz r8, 0x35(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(53 as u32) ) } as u64;
	// 822F0718: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 822F071C: 419AFFF0  beq cr6, 0x822f070c
	if ctx.cr[6].eq {
	pc = 0x822F070C; continue 'dispatch;
	}
	// 822F0720: 91490008  stw r10, 8(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 822F0724: 48000098  b 0x822f07bc
	pc = 0x822F07BC; continue 'dispatch;
	// 822F0728: 932B0004  stw r25, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[25].u32 ) };
	// 822F072C: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 822F0730: 91790000  stw r11, 0(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 822F0734: 817A0008  lwz r11, 8(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 822F0738: 7F195840  cmplw cr6, r25, r11
	ctx.cr[6].compare_u32(ctx.r[25].u32, ctx.r[11].u32, &mut ctx.xer);
	// 822F073C: 409A000C  bne cr6, 0x822f0748
	if !ctx.cr[6].eq {
	pc = 0x822F0748; continue 'dispatch;
	}
	// 822F0740: 7F3FCB78  mr r31, r25
	ctx.r[31].u64 = ctx.r[25].u64;
	// 822F0744: 4800002C  b 0x822f0770
	pc = 0x822F0770; continue 'dispatch;
	// 822F0748: 897C0035  lbz r11, 0x35(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(53 as u32) ) } as u64;
	// 822F074C: 83F90004  lwz r31, 4(r25)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(4 as u32) ) } as u64;
	// 822F0750: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 822F0754: 409A0008  bne cr6, 0x822f075c
	if !ctx.cr[6].eq {
	pc = 0x822F075C; continue 'dispatch;
	}
	// 822F0758: 93FC0004  stw r31, 4(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 822F075C: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 822F0760: 817A0008  lwz r11, 8(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 822F0764: 91790008  stw r11, 8(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 822F0768: 817A0008  lwz r11, 8(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 822F076C: 932B0004  stw r25, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[25].u32 ) };
	// 822F0770: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 822F0774: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 822F0778: 7F0AD040  cmplw cr6, r10, r26
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[26].u32, &mut ctx.xer);
	// 822F077C: 409A000C  bne cr6, 0x822f0788
	if !ctx.cr[6].eq {
	pc = 0x822F0788; continue 'dispatch;
	}
	// 822F0780: 932B0004  stw r25, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[25].u32 ) };
	// 822F0784: 48000020  b 0x822f07a4
	pc = 0x822F07A4; continue 'dispatch;
	// 822F0788: 817A0004  lwz r11, 4(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 822F078C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 822F0790: 7F0AD040  cmplw cr6, r10, r26
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[26].u32, &mut ctx.xer);
	// 822F0794: 409A000C  bne cr6, 0x822f07a0
	if !ctx.cr[6].eq {
	pc = 0x822F07A0; continue 'dispatch;
	}
	// 822F0798: 932B0000  stw r25, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[25].u32 ) };
	// 822F079C: 48000008  b 0x822f07a4
	pc = 0x822F07A4; continue 'dispatch;
	// 822F07A0: 932B0008  stw r25, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[25].u32 ) };
	// 822F07A4: 817A0004  lwz r11, 4(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 822F07A8: 91790004  stw r11, 4(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 822F07AC: 897A0034  lbz r11, 0x34(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[26].u32.wrapping_add(52 as u32) ) } as u64;
	// 822F07B0: 89590034  lbz r10, 0x34(r25)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[25].u32.wrapping_add(52 as u32) ) } as u64;
	// 822F07B4: 99790034  stb r11, 0x34(r25)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[25].u32.wrapping_add(52 as u32), ctx.r[11].u8 ) };
	// 822F07B8: 995A0034  stb r10, 0x34(r26)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[26].u32.wrapping_add(52 as u32), ctx.r[10].u8 ) };
	// 822F07BC: 897A0034  lbz r11, 0x34(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[26].u32.wrapping_add(52 as u32) ) } as u64;
	// 822F07C0: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 822F07C4: 409A0198  bne cr6, 0x822f095c
	if !ctx.cr[6].eq {
	pc = 0x822F095C; continue 'dispatch;
	}
	// 822F07C8: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 822F07CC: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 822F07D0: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 822F07D4: 7F1C5840  cmplw cr6, r28, r11
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[11].u32, &mut ctx.xer);
	// 822F07D8: 419A0180  beq cr6, 0x822f0958
	if ctx.cr[6].eq {
	pc = 0x822F0958; continue 'dispatch;
	}
	// 822F07DC: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 822F07E0: 897C0034  lbz r11, 0x34(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(52 as u32) ) } as u64;
	// 822F07E4: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 822F07E8: 409A0170  bne cr6, 0x822f0958
	if !ctx.cr[6].eq {
	pc = 0x822F0958; continue 'dispatch;
	}
	// 822F07EC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 822F07F0: 7F1C5840  cmplw cr6, r28, r11
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[11].u32, &mut ctx.xer);
	// 822F07F4: 409A00A8  bne cr6, 0x822f089c
	if !ctx.cr[6].eq {
	pc = 0x822F089C; continue 'dispatch;
	}
	// 822F07F8: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 822F07FC: 894B0034  lbz r10, 0x34(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 822F0800: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 822F0804: 409A001C  bne cr6, 0x822f0820
	if !ctx.cr[6].eq {
	pc = 0x822F0820; continue 'dispatch;
	}
	// 822F0808: 9BCB0034  stb r30, 0x34(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(52 as u32), ctx.r[30].u8 ) };
	// 822F080C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 822F0810: 9BBF0034  stb r29, 0x34(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[29].u8 ) };
	// 822F0814: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 822F0818: 4BFF7FA1  bl 0x822e87b8
	ctx.lr = 0x822F081C;
	sub_822E87B8(ctx, base);
	// 822F081C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 822F0820: 894B0035  lbz r10, 0x35(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(53 as u32) ) } as u64;
	// 822F0824: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 822F0828: 409A00C8  bne cr6, 0x822f08f0
	if !ctx.cr[6].eq {
	pc = 0x822F08F0; continue 'dispatch;
	}
	// 822F082C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 822F0830: 894A0034  lbz r10, 0x34(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(52 as u32) ) } as u64;
	// 822F0834: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 822F0838: 409A0014  bne cr6, 0x822f084c
	if !ctx.cr[6].eq {
	pc = 0x822F084C; continue 'dispatch;
	}
	// 822F083C: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 822F0840: 894A0034  lbz r10, 0x34(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(52 as u32) ) } as u64;
	// 822F0844: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 822F0848: 419A00A4  beq cr6, 0x822f08ec
	if ctx.cr[6].eq {
	pc = 0x822F08EC; continue 'dispatch;
	}
	// 822F084C: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 822F0850: 894A0034  lbz r10, 0x34(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(52 as u32) ) } as u64;
	// 822F0854: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 822F0858: 409A0020  bne cr6, 0x822f0878
	if !ctx.cr[6].eq {
	pc = 0x822F0878; continue 'dispatch;
	}
	// 822F085C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 822F0860: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 822F0864: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 822F0868: 9BCA0034  stb r30, 0x34(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(52 as u32), ctx.r[30].u8 ) };
	// 822F086C: 9BAB0034  stb r29, 0x34(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(52 as u32), ctx.r[29].u8 ) };
	// 822F0870: 4BFF7FB1  bl 0x822e8820
	ctx.lr = 0x822F0874;
	sub_822E8820(ctx, base);
	// 822F0874: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 822F0878: 895F0034  lbz r10, 0x34(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 822F087C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 822F0880: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 822F0884: 994B0034  stb r10, 0x34(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(52 as u32), ctx.r[10].u8 ) };
	// 822F0888: 9BDF0034  stb r30, 0x34(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[30].u8 ) };
	// 822F088C: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 822F0890: 9BCB0034  stb r30, 0x34(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(52 as u32), ctx.r[30].u8 ) };
	// 822F0894: 4BFF7F25  bl 0x822e87b8
	ctx.lr = 0x822F0898;
	sub_822E87B8(ctx, base);
	// 822F0898: 480000C0  b 0x822f0958
	pc = 0x822F0958; continue 'dispatch;
	// 822F089C: 894B0034  lbz r10, 0x34(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 822F08A0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 822F08A4: 409A001C  bne cr6, 0x822f08c0
	if !ctx.cr[6].eq {
	pc = 0x822F08C0; continue 'dispatch;
	}
	// 822F08A8: 9BCB0034  stb r30, 0x34(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(52 as u32), ctx.r[30].u8 ) };
	// 822F08AC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 822F08B0: 9BBF0034  stb r29, 0x34(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[29].u8 ) };
	// 822F08B4: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 822F08B8: 4BFF7F69  bl 0x822e8820
	ctx.lr = 0x822F08BC;
	sub_822E8820(ctx, base);
	// 822F08BC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 822F08C0: 894B0035  lbz r10, 0x35(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(53 as u32) ) } as u64;
	// 822F08C4: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 822F08C8: 409A0028  bne cr6, 0x822f08f0
	if !ctx.cr[6].eq {
	pc = 0x822F08F0; continue 'dispatch;
	}
	// 822F08CC: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 822F08D0: 894A0034  lbz r10, 0x34(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(52 as u32) ) } as u64;
	// 822F08D4: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 822F08D8: 409A0034  bne cr6, 0x822f090c
	if !ctx.cr[6].eq {
	pc = 0x822F090C; continue 'dispatch;
	}
	// 822F08DC: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 822F08E0: 894A0034  lbz r10, 0x34(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(52 as u32) ) } as u64;
	// 822F08E4: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 822F08E8: 409A0024  bne cr6, 0x822f090c
	if !ctx.cr[6].eq {
	pc = 0x822F090C; continue 'dispatch;
	}
	// 822F08EC: 9BAB0034  stb r29, 0x34(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(52 as u32), ctx.r[29].u8 ) };
	// 822F08F0: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 822F08F4: 7FFCFB78  mr r28, r31
	ctx.r[28].u64 = ctx.r[31].u64;
	// 822F08F8: 83FF0004  lwz r31, 4(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 822F08FC: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 822F0900: 7F1C5840  cmplw cr6, r28, r11
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[11].u32, &mut ctx.xer);
	// 822F0904: 409AFEDC  bne cr6, 0x822f07e0
	if !ctx.cr[6].eq {
	pc = 0x822F07E0; continue 'dispatch;
	}
	// 822F0908: 48000050  b 0x822f0958
	pc = 0x822F0958; continue 'dispatch;
	// 822F090C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 822F0910: 894A0034  lbz r10, 0x34(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(52 as u32) ) } as u64;
	// 822F0914: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 822F0918: 409A0020  bne cr6, 0x822f0938
	if !ctx.cr[6].eq {
	pc = 0x822F0938; continue 'dispatch;
	}
	// 822F091C: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 822F0920: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 822F0924: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 822F0928: 9BCA0034  stb r30, 0x34(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(52 as u32), ctx.r[30].u8 ) };
	// 822F092C: 9BAB0034  stb r29, 0x34(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(52 as u32), ctx.r[29].u8 ) };
	// 822F0930: 4BFF7E89  bl 0x822e87b8
	ctx.lr = 0x822F0934;
	sub_822E87B8(ctx, base);
	// 822F0934: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 822F0938: 895F0034  lbz r10, 0x34(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 822F093C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 822F0940: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 822F0944: 994B0034  stb r10, 0x34(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(52 as u32), ctx.r[10].u8 ) };
	// 822F0948: 9BDF0034  stb r30, 0x34(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[30].u8 ) };
	// 822F094C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 822F0950: 9BCB0034  stb r30, 0x34(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(52 as u32), ctx.r[30].u8 ) };
	// 822F0954: 4BFF7ECD  bl 0x822e8820
	ctx.lr = 0x822F0958;
	sub_822E8820(ctx, base);
	// 822F0958: 9BDC0034  stb r30, 0x34(r28)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[28].u32.wrapping_add(52 as u32), ctx.r[30].u8 ) };
	// 822F095C: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 822F0960: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 822F0964: 806B110C  lwz r3, 0x110c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4364 as u32) ) } as u64;
	// 822F0968: 48B01821  bl 0x82df2188
	ctx.lr = 0x822F096C;
	sub_82DF2188(ctx, base);
	// 822F096C: 817B0008  lwz r11, 8(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 822F0970: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 822F0974: 419A000C  beq cr6, 0x822f0980
	if ctx.cr[6].eq {
	pc = 0x822F0980; continue 'dispatch;
	}
	// 822F0978: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 822F097C: 917B0008  stw r11, 8(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 822F0980: 93380000  stw r25, 0(r24)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[24].u32.wrapping_add(0 as u32), ctx.r[25].u32 ) };
	// 822F0984: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 822F0988: 382100E0  addi r1, r1, 0xe0
	ctx.r[1].s64 = ctx.r[1].s64 + 224;
	// 822F098C: 48EB781C  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822F0990(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822F0990 size=132
    let mut pc: u32 = 0x822F0990;
    'dispatch: loop {
        match pc {
            0x822F0990 => {
    //   block [0x822F0990..0x822F0A14)
	// 822F0990: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822F0994: 48EB77D5  bl 0x831a8168
	ctx.lr = 0x822F0998;
	sub_831A8130(ctx, base);
	// 822F0998: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822F099C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 822F09A0: 90A100A4  stw r5, 0xa4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(164 as u32), ctx.r[5].u32 ) };
	// 822F09A4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 822F09A8: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 822F09AC: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 822F09B0: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 822F09B4: 7F055040  cmplw cr6, r5, r10
	ctx.cr[6].compare_u32(ctx.r[5].u32, ctx.r[10].u32, &mut ctx.xer);
	// 822F09B8: 409A0044  bne cr6, 0x822f09fc
	if !ctx.cr[6].eq {
	pc = 0x822F09FC; continue 'dispatch;
	}
	// 822F09BC: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 822F09C0: 409A003C  bne cr6, 0x822f09fc
	if !ctx.cr[6].eq {
	pc = 0x822F09FC; continue 'dispatch;
	}
	// 822F09C4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822F09C8: 4BFF8359  bl 0x822e8d20
	ctx.lr = 0x822F09CC;
	sub_822E8D20(ctx, base);
	// 822F09CC: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 822F09D0: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 822F09D4: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 822F09D8: 48000030  b 0x822f0a08
	pc = 0x822F0A08; continue 'dispatch;
	// 822F09DC: 386100A4  addi r3, r1, 0xa4
	ctx.r[3].s64 = ctx.r[1].s64 + 164;
	// 822F09E0: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 822F09E4: 481F8815  bl 0x824e91f8
	ctx.lr = 0x822F09E8;
	sub_824E91F8(ctx, base);
	// 822F09E8: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 822F09EC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 822F09F0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 822F09F4: 4BFFFBA5  bl 0x822f0598
	ctx.lr = 0x822F09F8;
	sub_822F0598(ctx, base);
	// 822F09F8: 80A100A4  lwz r5, 0xa4(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(164 as u32) ) } as u64;
	// 822F09FC: 7F05F040  cmplw cr6, r5, r30
	ctx.cr[6].compare_u32(ctx.r[5].u32, ctx.r[30].u32, &mut ctx.xer);
	// 822F0A00: 409AFFDC  bne cr6, 0x822f09dc
	if !ctx.cr[6].eq {
	pc = 0x822F09DC; continue 'dispatch;
	}
	// 822F0A04: 90BD0000  stw r5, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[5].u32 ) };
	// 822F0A08: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 822F0A0C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 822F0A10: 48EB77A8  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822F0A18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822F0A18 size=88
    let mut pc: u32 = 0x822F0A18;
    'dispatch: loop {
        match pc {
            0x822F0A18 => {
    //   block [0x822F0A18..0x822F0A70)
	// 822F0A18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822F0A1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822F0A20: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822F0A24: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822F0A28: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822F0A2C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 822F0A30: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 822F0A34: 80DF0004  lwz r6, 4(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 822F0A38: 80A60000  lwz r5, 0(r6)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 822F0A3C: 4BFFFF55  bl 0x822f0990
	ctx.lr = 0x822F0A40;
	sub_822F0990(ctx, base);
	// 822F0A40: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 822F0A44: 809F0004  lwz r4, 4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 822F0A48: 806B110C  lwz r3, 0x110c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4364 as u32) ) } as u64;
	// 822F0A4C: 48B0173D  bl 0x82df2188
	ctx.lr = 0x822F0A50;
	sub_82DF2188(ctx, base);
	// 822F0A50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 822F0A54: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 822F0A58: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 822F0A5C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 822F0A60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822F0A64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822F0A68: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822F0A6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822F0A70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822F0A70 size=288
    let mut pc: u32 = 0x822F0A70;
    'dispatch: loop {
        match pc {
            0x822F0A70 => {
    //   block [0x822F0A70..0x822F0B90)
	// 822F0A70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822F0A74: 48EB76F1  bl 0x831a8164
	ctx.lr = 0x822F0A78;
	sub_831A8130(ctx, base);
	// 822F0A78: 9421FE50  stwu r1, -0x1b0(r1)
	ea = ctx.r[1].u32.wrapping_add(-432 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822F0A7C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 822F0A80: 90A10174  stw r5, 0x174(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(372 as u32), ctx.r[5].u32 ) };
	// 822F0A84: 3D408000  lis r10, -0x8000
	ctx.r[10].s64 = -2147483648;
	// 822F0A88: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 822F0A8C: 39210074  addi r9, r1, 0x74
	ctx.r[9].s64 = ctx.r[1].s64 + 116;
	// 822F0A90: 396BC400  addi r11, r11, -0x3c00
	ctx.r[11].s64 = ctx.r[11].s64 + -15360;
	// 822F0A94: 93A1006C  stw r29, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[29].u32 ) };
	// 822F0A98: 614A0010  ori r10, r10, 0x10
	ctx.r[10].u64 = ctx.r[10].u64 | 16;
	// 822F0A9C: 91210068  stw r9, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[9].u32 ) };
	// 822F0AA0: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 822F0AA4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 822F0AA8: 91410070  stw r10, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[10].u32 ) };
	// 822F0AAC: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 822F0AB0: 9BA10064  stb r29, 0x64(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[29].u8 ) };
	// 822F0AB4: 4BFF530D  bl 0x822e5dc0
	ctx.lr = 0x822F0AB8;
	sub_822E5DC0(ctx, base);
	// 822F0AB8: 83E30008  lwz r31, 8(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 822F0ABC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 822F0AC0: 419A000C  beq cr6, 0x822f0acc
	if ctx.cr[6].eq {
	pc = 0x822F0ACC; continue 'dispatch;
	}
	// 822F0AC4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822F0AC8: 48BDDD39  bl 0x82ece800
	ctx.lr = 0x822F0ACC;
	sub_82ECE800(ctx, base);
	// 822F0ACC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 822F0AD0: 4BFF52F1  bl 0x822e5dc0
	ctx.lr = 0x822F0AD4;
	sub_822E5DC0(ctx, base);
	// 822F0AD4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 822F0AD8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 822F0ADC: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 822F0AE0: 816B003C  lwz r11, 0x3c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 822F0AE4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 822F0AE8: 4E800421  bctrl
	ctx.lr = 0x822F0AEC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 822F0AEC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 822F0AF0: 419A000C  beq cr6, 0x822f0afc
	if ctx.cr[6].eq {
	pc = 0x822F0AFC; continue 'dispatch;
	}
	// 822F0AF4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822F0AF8: 48BDC511  bl 0x82ecd008
	ctx.lr = 0x822F0AFC;
	sub_82ECD008(ctx, base);
	// 822F0AFC: 8161006C  lwz r11, 0x6c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(108 as u32) ) } as u64;
	// 822F0B00: 7FBEEB78  mr r30, r29
	ctx.r[30].u64 = ctx.r[29].u64;
	// 822F0B04: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 822F0B08: 40990078  ble cr6, 0x822f0b80
	if !ctx.cr[6].gt {
	pc = 0x822F0B80; continue 'dispatch;
	}
	// 822F0B0C: 7FBFEB78  mr r31, r29
	ctx.r[31].u64 = ctx.r[29].u64;
	// 822F0B10: 81610068  lwz r11, 0x68(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 822F0B14: 7D7F5A14  add r11, r31, r11
	ctx.r[11].u64 = ctx.r[31].u64 + ctx.r[11].u64;
	// 822F0B18: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 822F0B1C: 894B0010  lbz r10, 0x10(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 822F0B20: 7D4A0774  extsb r10, r10
	ctx.r[10].s64 = ctx.r[10].s8 as i64;
	// 822F0B24: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 822F0B28: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 822F0B2C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 822F0B30: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 822F0B34: 419A0038  beq cr6, 0x822f0b6c
	if ctx.cr[6].eq {
	pc = 0x822F0B6C; continue 'dispatch;
	}
	// 822F0B38: 83BC0004  lwz r29, 4(r28)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 822F0B3C: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 822F0B40: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 822F0B44: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 822F0B48: 80BD0004  lwz r5, 4(r29)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 822F0B4C: 482691B5  bl 0x82559d00
	ctx.lr = 0x822F0B50;
	sub_82559D00(ctx, base);
	// 822F0B50: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 822F0B54: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 822F0B58: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 822F0B5C: 488D929D  bl 0x82bc9df8
	ctx.lr = 0x822F0B60;
	sub_82BC9DF8(ctx, base);
	// 822F0B60: 937D0004  stw r27, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[27].u32 ) };
	// 822F0B64: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 822F0B68: 936B0000  stw r27, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 822F0B6C: 8161006C  lwz r11, 0x6c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(108 as u32) ) } as u64;
	// 822F0B70: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 822F0B74: 3BFF0010  addi r31, r31, 0x10
	ctx.r[31].s64 = ctx.r[31].s64 + 16;
	// 822F0B78: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 822F0B7C: 4198FF94  blt cr6, 0x822f0b10
	if ctx.cr[6].lt {
	pc = 0x822F0B10; continue 'dispatch;
	}
	// 822F0B80: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 822F0B84: 4BFFF6AD  bl 0x822f0230
	ctx.lr = 0x822F0B88;
	sub_822F0230(ctx, base);
	// 822F0B88: 382101B0  addi r1, r1, 0x1b0
	ctx.r[1].s64 = ctx.r[1].s64 + 432;
	// 822F0B8C: 48EB7628  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822F0B90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822F0B90 size=256
    let mut pc: u32 = 0x822F0B90;
    'dispatch: loop {
        match pc {
            0x822F0B90 => {
    //   block [0x822F0B90..0x822F0C90)
	// 822F0B90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822F0B94: 48EB75D5  bl 0x831a8168
	ctx.lr = 0x822F0B98;
	sub_831A8130(ctx, base);
	// 822F0B98: 9421FE60  stwu r1, -0x1a0(r1)
	ea = ctx.r[1].u32.wrapping_add(-416 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822F0B9C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 822F0BA0: 90A10174  stw r5, 0x174(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(372 as u32), ctx.r[5].u32 ) };
	// 822F0BA4: 3D408000  lis r10, -0x8000
	ctx.r[10].s64 = -2147483648;
	// 822F0BA8: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 822F0BAC: 39210074  addi r9, r1, 0x74
	ctx.r[9].s64 = ctx.r[1].s64 + 116;
	// 822F0BB0: 396BC400  addi r11, r11, -0x3c00
	ctx.r[11].s64 = ctx.r[11].s64 + -15360;
	// 822F0BB4: 93A1006C  stw r29, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[29].u32 ) };
	// 822F0BB8: 614A0010  ori r10, r10, 0x10
	ctx.r[10].u64 = ctx.r[10].u64 | 16;
	// 822F0BBC: 91210068  stw r9, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[9].u32 ) };
	// 822F0BC0: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 822F0BC4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 822F0BC8: 91410070  stw r10, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[10].u32 ) };
	// 822F0BCC: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 822F0BD0: 9BA10064  stb r29, 0x64(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[29].u8 ) };
	// 822F0BD4: 4BFF51ED  bl 0x822e5dc0
	ctx.lr = 0x822F0BD8;
	sub_822E5DC0(ctx, base);
	// 822F0BD8: 83E30008  lwz r31, 8(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 822F0BDC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 822F0BE0: 419A000C  beq cr6, 0x822f0bec
	if ctx.cr[6].eq {
	pc = 0x822F0BEC; continue 'dispatch;
	}
	// 822F0BE4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822F0BE8: 48BDDC19  bl 0x82ece800
	ctx.lr = 0x822F0BEC;
	sub_82ECE800(ctx, base);
	// 822F0BEC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 822F0BF0: 4BFF51D1  bl 0x822e5dc0
	ctx.lr = 0x822F0BF4;
	sub_822E5DC0(ctx, base);
	// 822F0BF4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 822F0BF8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 822F0BFC: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 822F0C00: 816B003C  lwz r11, 0x3c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 822F0C04: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 822F0C08: 4E800421  bctrl
	ctx.lr = 0x822F0C0C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 822F0C0C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 822F0C10: 419A000C  beq cr6, 0x822f0c1c
	if ctx.cr[6].eq {
	pc = 0x822F0C1C; continue 'dispatch;
	}
	// 822F0C14: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822F0C18: 48BDC3F1  bl 0x82ecd008
	ctx.lr = 0x822F0C1C;
	sub_82ECD008(ctx, base);
	// 822F0C1C: 8161006C  lwz r11, 0x6c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(108 as u32) ) } as u64;
	// 822F0C20: 7FBEEB78  mr r30, r29
	ctx.r[30].u64 = ctx.r[29].u64;
	// 822F0C24: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 822F0C28: 40990058  ble cr6, 0x822f0c80
	if !ctx.cr[6].gt {
	pc = 0x822F0C80; continue 'dispatch;
	}
	// 822F0C2C: 7FBFEB78  mr r31, r29
	ctx.r[31].u64 = ctx.r[29].u64;
	// 822F0C30: 81610068  lwz r11, 0x68(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 822F0C34: 7D5F5A14  add r10, r31, r11
	ctx.r[10].u64 = ctx.r[31].u64 + ctx.r[11].u64;
	// 822F0C38: 816A0008  lwz r11, 8(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 822F0C3C: 892B0010  lbz r9, 0x10(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 822F0C40: 7D290774  extsb r9, r9
	ctx.r[9].s64 = ctx.r[9].s8 as i64;
	// 822F0C44: 7D695A14  add r11, r9, r11
	ctx.r[11].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 822F0C48: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 822F0C4C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 822F0C50: 419A001C  beq cr6, 0x822f0c6c
	if ctx.cr[6].eq {
	pc = 0x822F0C6C; continue 'dispatch;
	}
	// 822F0C54: 814A000C  lwz r10, 0xc(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 822F0C58: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 822F0C5C: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 822F0C60: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 822F0C64: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 822F0C68: 48834189  bl 0x82b24df0
	ctx.lr = 0x822F0C6C;
	sub_82B24DF0(ctx, base);
	// 822F0C6C: 8161006C  lwz r11, 0x6c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(108 as u32) ) } as u64;
	// 822F0C70: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 822F0C74: 3BFF0010  addi r31, r31, 0x10
	ctx.r[31].s64 = ctx.r[31].s64 + 16;
	// 822F0C78: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 822F0C7C: 4198FFB4  blt cr6, 0x822f0c30
	if ctx.cr[6].lt {
	pc = 0x822F0C30; continue 'dispatch;
	}
	// 822F0C80: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 822F0C84: 4BFFF5AD  bl 0x822f0230
	ctx.lr = 0x822F0C88;
	sub_822F0230(ctx, base);
	// 822F0C88: 382101A0  addi r1, r1, 0x1a0
	ctx.r[1].s64 = ctx.r[1].s64 + 416;
	// 822F0C8C: 48EB752C  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822F0C90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x822F0C90 size=496
    let mut pc: u32 = 0x822F0C90;
    'dispatch: loop {
        match pc {
            0x822F0C90 => {
    //   block [0x822F0C90..0x822F0E80)
	// 822F0C90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822F0C94: 48EB74C9  bl 0x831a815c
	ctx.lr = 0x822F0C98;
	sub_831A8130(ctx, base);
	// 822F0C98: DBE1FFB8  stfd f31, -0x48(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-72 as u32), ctx.f[31].u64 ) };
	// 822F0C9C: 9421FBB0  stwu r1, -0x450(r1)
	ea = ctx.r[1].u32.wrapping_add(-1104 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822F0CA0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822F0CA4: 91010240  stw r8, 0x240(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(576 as u32), ctx.r[8].u32 ) };
	// 822F0CA8: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 822F0CAC: 910103F0  stw r8, 0x3f0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(1008 as u32), ctx.r[8].u32 ) };
	// 822F0CB0: 7CF93B78  mr r25, r7
	ctx.r[25].u64 = ctx.r[7].u64;
	// 822F0CB4: 39410070  addi r10, r1, 0x70
	ctx.r[10].s64 = ctx.r[1].s64 + 112;
	// 822F0CB8: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 822F0CBC: 3CE08201  lis r7, -0x7dff
	ctx.r[7].s64 = -2113863680;
	// 822F0CC0: C01F005C  lfs f0, 0x5c(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 822F0CC4: C1BF0058  lfs f13, 0x58(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 822F0CC8: 3CC08201  lis r6, -0x7dff
	ctx.r[6].s64 = -2113863680;
	// 822F0CCC: 3D608000  lis r11, -0x8000
	ctx.r[11].s64 = -2147483648;
	// 822F0CD0: 13E0E8C7  vcmpequd (lvx128) v31, v0, v29
	tmp.u32 = ctx.r[29].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 822F0CD4: ED80682A  fadds f12, f0, f13
	ctx.f[12].f64 = ((ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822F0E80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x822F0E80 size=8
    let mut pc: u32 = 0x822F0E80;
    'dispatch: loop {
        match pc {
            0x822F0E80 => {
    //   block [0x822F0E80..0x822F0E88)
	// 822F0E80: 80630018  lwz r3, 0x18(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 822F0E84: 4800097C  b 0x822f1800
	sub_822F1800(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822F0E88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822F0E88 size=196
    let mut pc: u32 = 0x822F0E88;
    'dispatch: loop {
        match pc {
            0x822F0E88 => {
    //   block [0x822F0E88..0x822F0F4C)
	// 822F0E88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822F0E8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822F0E90: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 822F0E94: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822F0E98: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822F0E9C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 822F0EA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 822F0EA4: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 822F0EA8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 822F0EAC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 822F0EB0: 4BFCFA89  bl 0x822c0938
	ctx.lr = 0x822F0EB4;
	sub_822C0938(ctx, base);
	// 822F0EB4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 822F0EB8: 41820028  beq 0x822f0ee0
	if ctx.cr[0].eq {
	pc = 0x822F0EE0; continue 'dispatch;
	}
	// 822F0EBC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 822F0EC0: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 822F0EC4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 822F0EC8: 392BC418  addi r9, r11, -0x3be8
	ctx.r[9].s64 = ctx.r[11].s64 + -15336;
	// 822F0ECC: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 822F0ED0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 822F0ED4: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 822F0ED8: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 822F0EDC: 48000008  b 0x822f0ee4
	pc = 0x822F0EE4; continue 'dispatch;
	// 822F0EE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 822F0EE4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 822F0EE8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 822F0EEC: 409A0044  bne cr6, 0x822f0f30
	if !ctx.cr[6].eq {
	pc = 0x822F0F30; continue 'dispatch;
	}
	// 822F0EF0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 822F0EF4: 419A001C  beq cr6, 0x822f0f10
	if ctx.cr[6].eq {
	pc = 0x822F0F10; continue 'dispatch;
	}
	// 822F0EF8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 822F0EFC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 822F0F00: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822F0F04: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 822F0F08: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 822F0F0C: 4E800421  bctrl
	ctx.lr = 0x822F0F10;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 822F0F10: 3D608325  lis r11, -0x7cdb
	ctx.r[11].s64 = -2094727168;
	// 822F0F14: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 822F0F18: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 822F0F1C: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 822F0F20: 816B498C  lwz r11, 0x498c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(18828 as u32) ) } as u64;
	// 822F0F24: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 822F0F28: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 822F0F2C: 4BFCF0D5  bl 0x822c0000
	ctx.lr = 0x822F0F30;
	sub_822C0000(ctx, base);
	// 822F0F30: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 822F0F34: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 822F0F38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822F0F3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822F0F40: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 822F0F44: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822F0F48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822F0F50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822F0F50 size=196
    let mut pc: u32 = 0x822F0F50;
    'dispatch: loop {
        match pc {
            0x822F0F50 => {
    //   block [0x822F0F50..0x822F1014)
	// 822F0F50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822F0F54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822F0F58: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 822F0F5C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822F0F60: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822F0F64: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 822F0F68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 822F0F6C: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 822F0F70: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 822F0F74: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 822F0F78: 4BFCF9C1  bl 0x822c0938
	ctx.lr = 0x822F0F7C;
	sub_822C0938(ctx, base);
	// 822F0F7C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 822F0F80: 41820028  beq 0x822f0fa8
	if ctx.cr[0].eq {
	pc = 0x822F0FA8; continue 'dispatch;
	}
	// 822F0F84: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 822F0F88: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 822F0F8C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 822F0F90: 392BC42C  addi r9, r11, -0x3bd4
	ctx.r[9].s64 = ctx.r[11].s64 + -15316;
	// 822F0F94: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 822F0F98: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 822F0F9C: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 822F0FA0: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 822F0FA4: 48000008  b 0x822f0fac
	pc = 0x822F0FAC; continue 'dispatch;
	// 822F0FA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 822F0FAC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 822F0FB0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 822F0FB4: 409A0044  bne cr6, 0x822f0ff8
	if !ctx.cr[6].eq {
	pc = 0x822F0FF8; continue 'dispatch;
	}
	// 822F0FB8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 822F0FBC: 419A001C  beq cr6, 0x822f0fd8
	if ctx.cr[6].eq {
	pc = 0x822F0FD8; continue 'dispatch;
	}
	// 822F0FC0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 822F0FC4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 822F0FC8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822F0FCC: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 822F0FD0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 822F0FD4: 4E800421  bctrl
	ctx.lr = 0x822F0FD8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 822F0FD8: 3D608325  lis r11, -0x7cdb
	ctx.r[11].s64 = -2094727168;
	// 822F0FDC: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 822F0FE0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 822F0FE4: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 822F0FE8: 816B498C  lwz r11, 0x498c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(18828 as u32) ) } as u64;
	// 822F0FEC: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 822F0FF0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 822F0FF4: 4BFCF00D  bl 0x822c0000
	ctx.lr = 0x822F0FF8;
	sub_822C0000(ctx, base);
	// 822F0FF8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 822F0FFC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 822F1000: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822F1004: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822F1008: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 822F100C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822F1010: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822F1018(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822F1018 size=112
    let mut pc: u32 = 0x822F1018;
    'dispatch: loop {
        match pc {
            0x822F1018 => {
    //   block [0x822F1018..0x822F1088)
	// 822F1018: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822F101C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822F1020: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 822F1024: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822F1028: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822F102C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 822F1030: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822F1034: 93C10050  stw r30, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 822F1038: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 822F103C: 4BFFFF15  bl 0x822f0f50
	ctx.lr = 0x822F1040;
	sub_822F0F50(ctx, base);
	// 822F1040: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 822F1044: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 822F1048: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 822F104C: 4BFCEFB5  bl 0x822c0000
	ctx.lr = 0x822F1050;
	sub_822C0000(ctx, base);
	// 822F1050: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 822F1054: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 822F1058: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 822F105C: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 822F1060: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 822F1064: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 822F1068: 419A0008  beq cr6, 0x822f1070
	if ctx.cr[6].eq {
	pc = 0x822F1070; continue 'dispatch;
	}
	// 822F106C: 4BFCF825  bl 0x822c0890
	ctx.lr = 0x822F1070;
	sub_822C0890(ctx, base);
	// 822F1070: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 822F1074: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822F1078: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822F107C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 822F1080: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822F1084: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822F1088(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822F1088 size=76
    let mut pc: u32 = 0x822F1088;
    'dispatch: loop {
        match pc {
            0x822F1088 => {
    //   block [0x822F1088..0x822F10D4)
	// 822F1088: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822F108C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822F1090: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822F1094: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822F1098: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822F109C: 48B20E3D  bl 0x82e11ed8
	ctx.lr = 0x822F10A0;
	sub_82E11ED8(ctx, base);
	// 822F10A0: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 822F10A4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 822F10A8: 394AC440  addi r10, r10, -0x3bc0
	ctx.r[10].s64 = ctx.r[10].s64 + -15296;
	// 822F10AC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822F10B0: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 822F10B4: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 822F10B8: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 822F10BC: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 822F10C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 822F10C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822F10C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822F10CC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822F10D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822F10D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822F10D8 size=88
    let mut pc: u32 = 0x822F10D8;
    'dispatch: loop {
        match pc {
            0x822F10D8 => {
    //   block [0x822F10D8..0x822F1130)
	// 822F10D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822F10DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822F10E0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 822F10E4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822F10E8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822F10EC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822F10F0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 822F10F4: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 822F10F8: 4861F4B1  bl 0x829105a8
	ctx.lr = 0x822F10FC;
	sub_829105A8(ctx, base);
	// 822F10FC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822F1100: 48B20DF1  bl 0x82e11ef0
	ctx.lr = 0x822F1104;
	sub_82E11EF0(ctx, base);
	// 822F1104: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 822F1108: 4182000C  beq 0x822f1114
	if ctx.cr[0].eq {
	pc = 0x822F1114; continue 'dispatch;
	}
	// 822F110C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822F1110: 48B012C9  bl 0x82df23d8
	ctx.lr = 0x822F1114;
	sub_82DF23D8(ctx, base);
	// 822F1114: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822F1118: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 822F111C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822F1120: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822F1124: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 822F1128: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822F112C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822F1130(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822F1130 size=104
    let mut pc: u32 = 0x822F1130;
    'dispatch: loop {
        match pc {
            0x822F1130 => {
    //   block [0x822F1130..0x822F1198)
	// 822F1130: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822F1134: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822F1138: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 822F113C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822F1140: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822F1144: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822F1148: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 822F114C: 807F001C  lwz r3, 0x1c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 822F1150: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 822F1154: 419A0008  beq cr6, 0x822f115c
	if ctx.cr[6].eq {
	pc = 0x822F115C; continue 'dispatch;
	}
	// 822F1158: 4BFCF739  bl 0x822c0890
	ctx.lr = 0x822F115C;
	sub_822C0890(ctx, base);
	// 822F115C: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 822F1160: 4861F449  bl 0x829105a8
	ctx.lr = 0x822F1164;
	sub_829105A8(ctx, base);
	// 822F1164: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822F1168: 48B20D89  bl 0x82e11ef0
	ctx.lr = 0x822F116C;
	sub_82E11EF0(ctx, base);
	// 822F116C: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 822F1170: 4182000C  beq 0x822f117c
	if ctx.cr[0].eq {
	pc = 0x822F117C; continue 'dispatch;
	}
	// 822F1174: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822F1178: 48B01261  bl 0x82df23d8
	ctx.lr = 0x822F117C;
	sub_82DF23D8(ctx, base);
	// 822F117C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822F1180: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 822F1184: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822F1188: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822F118C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 822F1190: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822F1194: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822F1198(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822F1198 size=416
    let mut pc: u32 = 0x822F1198;
    'dispatch: loop {
        match pc {
            0x822F1198 => {
    //   block [0x822F1198..0x822F1338)
	// 822F1198: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822F119C: 48EB6FB1  bl 0x831a814c
	ctx.lr = 0x822F11A0;
	sub_831A8130(ctx, base);
	// 822F11A0: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822F11A4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 822F11A8: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 822F11AC: 3BABC460  addi r29, r11, -0x3ba0
	ctx.r[29].s64 = ctx.r[11].s64 + -15264;
	// 822F11B0: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 822F11B4: 7CB92B78  mr r25, r5
	ctx.r[25].u64 = ctx.r[5].u64;
	// 822F11B8: 7CD83378  mr r24, r6
	ctx.r[24].u64 = ctx.r[6].u64;
	// 822F11BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 822F11C0: 38A0001B  li r5, 0x1b
	ctx.r[5].s64 = 27;
	// 822F11C4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 822F11C8: 38600020  li r3, 0x20
	ctx.r[3].s64 = 32;
	// 822F11CC: 7CF73B78  mr r23, r7
	ctx.r[23].u64 = ctx.r[7].u64;
	// 822F11D0: 7D164378  mr r22, r8
	ctx.r[22].u64 = ctx.r[8].u64;
	// 822F11D4: 7D354B78  mr r21, r9
	ctx.r[21].u64 = ctx.r[9].u64;
	// 822F11D8: 7D5E5378  mr r30, r10
	ctx.r[30].u64 = ctx.r[10].u64;
	// 822F11DC: 48B0120D  bl 0x82df23e8
	ctx.lr = 0x822F11E0;
	sub_82DF23E8(ctx, base);
	// 822F11E0: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 822F11E4: 41820030  beq 0x822f1214
	if ctx.cr[0].eq {
	pc = 0x822F1214; continue 'dispatch;
	}
	// 822F11E8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822F11EC: 4BFFFE9D  bl 0x822f1088
	ctx.lr = 0x822F11F0;
	sub_822F1088(ctx, base);
	// 822F11F0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 822F11F4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 822F11F8: 392BC450  addi r9, r11, -0x3bb0
	ctx.r[9].s64 = ctx.r[11].s64 + -15280;
	// 822F11FC: 397F0018  addi r11, r31, 0x18
	ctx.r[11].s64 = ctx.r[31].s64 + 24;
	// 822F1200: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 822F1204: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 822F1208: 915F0018  stw r10, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[10].u32 ) };
	// 822F120C: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 822F1210: 48000008  b 0x822f1218
	pc = 0x822F1218; continue 'dispatch;
	// 822F1214: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 822F1218: 93FB0000  stw r31, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 822F121C: 3B9B0004  addi r28, r27, 4
	ctx.r[28].s64 = ctx.r[27].s64 + 4;
	// 822F1220: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 822F1224: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 822F1228: 4BFFFC61  bl 0x822f0e88
	ctx.lr = 0x822F122C;
	sub_822F0E88(ctx, base);
	// 822F122C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 822F1230: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 822F1234: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 822F1238: 4BFCEDC9  bl 0x822c0000
	ctx.lr = 0x822F123C;
	sub_822C0000(ctx, base);
	// 822F123C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 822F1240: 48EAA641  bl 0x8319b880
	ctx.lr = 0x822F1244;
	sub_8319B880(ctx, base);
	// 822F1244: 897E0000  lbz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 822F1248: 38610064  addi r3, r1, 0x64
	ctx.r[3].s64 = ctx.r[1].s64 + 100;
	// 822F124C: 2B0B00FF  cmplwi cr6, r11, 0xff
	ctx.cr[6].compare_u32(ctx.r[11].u32, 255 as u32, &mut ctx.xer);
	// 822F1250: 40980010  bge cr6, 0x822f1260
	if !ctx.cr[6].lt {
	pc = 0x822F1260; continue 'dispatch;
	}
	// 822F1254: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 822F1258: 388B9430  addi r4, r11, -0x6bd0
	ctx.r[4].s64 = ctx.r[11].s64 + -27600;
	// 822F125C: 4800000C  b 0x822f1268
	pc = 0x822F1268; continue 'dispatch;
	// 822F1260: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 822F1264: 388B9428  addi r4, r11, -0x6bd8
	ctx.r[4].s64 = ctx.r[11].s64 + -27608;
	// 822F1268: 48B082C9  bl 0x82df9530
	ctx.lr = 0x822F126C;
	sub_82DF9530(ctx, base);
	// 822F126C: 81610064  lwz r11, 0x64(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 822F1270: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 822F1274: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 822F1278: 38A00025  li r5, 0x25
	ctx.r[5].s64 = 37;
	// 822F127C: 38600070  li r3, 0x70
	ctx.r[3].s64 = 112;
	// 822F1280: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 822F1284: 48B01165  bl 0x82df23e8
	ctx.lr = 0x822F1288;
	sub_82DF23E8(ctx, base);
	// 822F1288: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 822F128C: 41820038  beq 0x822f12c4
	if ctx.cr[0].eq {
	pc = 0x822F12C4; continue 'dispatch;
	}
	// 822F1290: 81610124  lwz r11, 0x124(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(292 as u32) ) } as u64;
	// 822F1294: 7EAAAB78  mr r10, r21
	ctx.r[10].u64 = ctx.r[21].u64;
	// 822F1298: 7EC9B378  mr r9, r22
	ctx.r[9].u64 = ctx.r[22].u64;
	// 822F129C: 80E10060  lwz r7, 0x60(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 822F12A0: 7EE8BB78  mr r8, r23
	ctx.r[8].u64 = ctx.r[23].u64;
	// 822F12A4: 93C10054  stw r30, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[30].u32 ) };
	// 822F12A8: 7F06C378  mr r6, r24
	ctx.r[6].u64 = ctx.r[24].u64;
	// 822F12AC: 7F25CB78  mr r5, r25
	ctx.r[5].u64 = ctx.r[25].u64;
	// 822F12B0: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 822F12B4: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 822F12B8: 48001FD9  bl 0x822f3290
	ctx.lr = 0x822F12BC;
	sub_822F3290(ctx, base);
	// 822F12BC: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 822F12C0: 48000008  b 0x822f12c8
	pc = 0x822F12C8; continue 'dispatch;
	// 822F12C4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 822F12C8: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 822F12CC: 386B0018  addi r3, r11, 0x18
	ctx.r[3].s64 = ctx.r[11].s64 + 24;
	// 822F12D0: 4BFFFD49  bl 0x822f1018
	ctx.lr = 0x822F12D4;
	sub_822F1018(ctx, base);
	// 822F12D4: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 822F12D8: 814B0018  lwz r10, 0x18(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 822F12DC: 91410068  stw r10, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[10].u32 ) };
	// 822F12E0: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 822F12E4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 822F12E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 822F12EC: 419A0024  beq cr6, 0x822f1310
	if ctx.cr[6].eq {
	pc = 0x822F1310; continue 'dispatch;
	}
	// 822F12F0: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 822F12F4: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 822F12F8: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 822F12FC: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 822F1300: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 822F1304: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 822F1308: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 822F130C: 4082FFE8  bne 0x822f12f4
	if !ctx.cr[0].eq {
	pc = 0x822F12F4; continue 'dispatch;
	}
	// 822F1310: 38810068  addi r4, r1, 0x68
	ctx.r[4].s64 = ctx.r[1].s64 + 104;
	// 822F1314: 807B0000  lwz r3, 0(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 822F1318: 48B22181  bl 0x82e13498
	ctx.lr = 0x822F131C;
	sub_82E13498(ctx, base);
	// 822F131C: 8061006C  lwz r3, 0x6c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(108 as u32) ) } as u64;
	// 822F1320: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 822F1324: 419A0008  beq cr6, 0x822f132c
	if ctx.cr[6].eq {
	pc = 0x822F132C; continue 'dispatch;
	}
	// 822F1328: 4BFCF569  bl 0x822c0890
	ctx.lr = 0x822F132C;
	sub_822C0890(ctx, base);
	// 822F132C: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 822F1330: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 822F1334: 48EB6E68  b 0x831a819c
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822F1338(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x822F1338 size=352
    let mut pc: u32 = 0x822F1338;
    'dispatch: loop {
        match pc {
            0x822F1338 => {
    //   block [0x822F1338..0x822F1498)
	// 822F1338: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822F133C: 48EB6E31  bl 0x831a816c
	ctx.lr = 0x822F1340;
	sub_831A8130(ctx, base);
	// 822F1340: DBE1FFD8  stfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 822F1344: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822F1348: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 822F134C: C0060000  lfs f0, 0(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 822F1350: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 822F1354: C1A60004  lfs f13, 4(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 822F1358: 396BC4C0  addi r11, r11, -0x3b40
	ctx.r[11].s64 = ctx.r[11].s64 + -15168;
	// 822F135C: C1860008  lfs f12, 8(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(8 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 822F1360: 39210050  addi r9, r1, 0x50
	ctx.r[9].s64 = ctx.r[1].s64 + 80;
	// 822F1364: D0010050  stfs f0, 0x50(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 822F1368: D1A10054  stfs f13, 0x54(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 822F136C: 39010080  addi r8, r1, 0x80
	ctx.r[8].s64 = ctx.r[1].s64 + 128;
	// 822F1370: D1810058  stfs f12, 0x58(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), tmp.u32 ) };
	// 822F1374: 7CFD3B78  mr r29, r7
	ctx.r[29].u64 = ctx.r[7].u64;
	// 822F1378: C3EA08A4  lfs f31, 0x8a4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2212 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 822F137C: 3CE08209  lis r7, -0x7df7
	ctx.r[7].s64 = -2113339392;
	// 822F1380: 13E058C7  vcmpequd (lvx128) v31, v0, v11
	tmp.u32 = ctx.r[11].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 822F1384: 39410060  addi r10, r1, 0x60
	ctx.r[10].s64 = ctx.r[1].s64 + 96;
	// 822F1388: D3E1005C  stfs f31, 0x5c(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), tmp.u32 ) };
	// 822F138C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 822F1390: 13C048C7  vcmpequd (lvx128) v30, v0, v9
	tmp.u32 = ctx.r[9].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[62] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822F1498(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822F1498 size=220
    let mut pc: u32 = 0x822F1498;
    'dispatch: loop {
        match pc {
            0x822F1498 => {
    //   block [0x822F1498..0x822F1574)
	// 822F1498: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822F149C: 48EB6CC1  bl 0x831a815c
	ctx.lr = 0x822F14A0;
	sub_831A8130(ctx, base);
	// 822F14A0: 9421FD60  stwu r1, -0x2a0(r1)
	ea = ctx.r[1].u32.wrapping_add(-672 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822F14A4: 81660000  lwz r11, 0(r6)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 822F14A8: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 822F14AC: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 822F14B0: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 822F14B4: 7CFA3B78  mr r26, r7
	ctx.r[26].u64 = ctx.r[7].u64;
	// 822F14B8: 7D194378  mr r25, r8
	ctx.r[25].u64 = ctx.r[8].u64;
	// 822F14BC: 806B0034  lwz r3, 0x34(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 822F14C0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 822F14C4: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 822F14C8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 822F14CC: 4E800421  bctrl
	ctx.lr = 0x822F14D0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 822F14D0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822F14D4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 822F14D8: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 822F14DC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 822F14E0: 4E800421  bctrl
	ctx.lr = 0x822F14E4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 822F14E4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 822F14E8: 4800007C  b 0x822f1564
	pc = 0x822F1564; continue 'dispatch;
	// 822F14EC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 822F14F0: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 822F14F4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 822F14F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822F14FC: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 822F1500: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 822F1504: 4E800421  bctrl
	ctx.lr = 0x822F1508;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 822F1508: 90610050  stw r3, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 822F150C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 822F1510: 41820008  beq 0x822f1518
	if ctx.cr[0].eq {
	pc = 0x822F1518; continue 'dispatch;
	}
	// 822F1514: 4BFF6D35  bl 0x822e8248
	ctx.lr = 0x822F1518;
	sub_822E8248(ctx, base);
	// 822F1518: 7F28CB78  mr r8, r25
	ctx.r[8].u64 = ctx.r[25].u64;
	// 822F151C: 7F47D378  mr r7, r26
	ctx.r[7].u64 = ctx.r[26].u64;
	// 822F1520: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 822F1524: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 822F1528: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 822F152C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 822F1530: 48001981  bl 0x822f2eb0
	ctx.lr = 0x822F1534;
	sub_822F2EB0(ctx, base);
	// 822F1534: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 822F1538: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 822F153C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822F1540: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 822F1544: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 822F1548: 4E800421  bctrl
	ctx.lr = 0x822F154C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 822F154C: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 822F1550: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 822F1554: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 822F1558: 419A000C  beq cr6, 0x822f1564
	if ctx.cr[6].eq {
	pc = 0x822F1564; continue 'dispatch;
	}
	// 822F155C: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 822F1560: 4BFF6D09  bl 0x822e8268
	ctx.lr = 0x822F1564;
	sub_822E8268(ctx, base);
	// 822F1564: 2F1EFFFF  cmpwi cr6, r30, -1
	ctx.cr[6].compare_i32(ctx.r[30].s32, -1, &mut ctx.xer);
	// 822F1568: 409AFF84  bne cr6, 0x822f14ec
	if !ctx.cr[6].eq {
	pc = 0x822F14EC; continue 'dispatch;
	}
	// 822F156C: 382102A0  addi r1, r1, 0x2a0
	ctx.r[1].s64 = ctx.r[1].s64 + 672;
	// 822F1570: 48EB6C3C  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822F1578(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822F1578 size=212
    let mut pc: u32 = 0x822F1578;
    'dispatch: loop {
        match pc {
            0x822F1578 => {
    //   block [0x822F1578..0x822F164C)
	// 822F1578: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822F157C: 48EB6BE1  bl 0x831a815c
	ctx.lr = 0x822F1580;
	sub_831A8130(ctx, base);
	// 822F1580: 9421FD60  stwu r1, -0x2a0(r1)
	ea = ctx.r[1].u32.wrapping_add(-672 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822F1584: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 822F1588: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 822F158C: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 822F1590: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 822F1594: 7CFA3B78  mr r26, r7
	ctx.r[26].u64 = ctx.r[7].u64;
	// 822F1598: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 822F159C: 7D194378  mr r25, r8
	ctx.r[25].u64 = ctx.r[8].u64;
	// 822F15A0: 386B0010  addi r3, r11, 0x10
	ctx.r[3].s64 = ctx.r[11].s64 + 16;
	// 822F15A4: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 822F15A8: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 822F15AC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 822F15B0: 4E800421  bctrl
	ctx.lr = 0x822F15B4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 822F15B4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822F15B8: 48000084  b 0x822f163c
	pc = 0x822F163C; continue 'dispatch;
	// 822F15BC: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 822F15C0: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 822F15C4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 822F15C8: 386B0010  addi r3, r11, 0x10
	ctx.r[3].s64 = ctx.r[11].s64 + 16;
	// 822F15CC: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 822F15D0: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 822F15D4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 822F15D8: 4E800421  bctrl
	ctx.lr = 0x822F15DC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 822F15DC: 90610050  stw r3, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 822F15E0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 822F15E4: 41820008  beq 0x822f15ec
	if ctx.cr[0].eq {
	pc = 0x822F15EC; continue 'dispatch;
	}
	// 822F15E8: 4BFF6C61  bl 0x822e8248
	ctx.lr = 0x822F15EC;
	sub_822E8248(ctx, base);
	// 822F15EC: 7F28CB78  mr r8, r25
	ctx.r[8].u64 = ctx.r[25].u64;
	// 822F15F0: 7F47D378  mr r7, r26
	ctx.r[7].u64 = ctx.r[26].u64;
	// 822F15F4: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 822F15F8: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 822F15FC: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 822F1600: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 822F1604: 480018AD  bl 0x822f2eb0
	ctx.lr = 0x822F1608;
	sub_822F2EB0(ctx, base);
	// 822F1608: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 822F160C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 822F1610: 386B0010  addi r3, r11, 0x10
	ctx.r[3].s64 = ctx.r[11].s64 + 16;
	// 822F1614: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 822F1618: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 822F161C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 822F1620: 4E800421  bctrl
	ctx.lr = 0x822F1624;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 822F1624: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 822F1628: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822F162C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 822F1630: 419A000C  beq cr6, 0x822f163c
	if ctx.cr[6].eq {
	pc = 0x822F163C; continue 'dispatch;
	}
	// 822F1634: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 822F1638: 4BFF6C31  bl 0x822e8268
	ctx.lr = 0x822F163C;
	sub_822E8268(ctx, base);
	// 822F163C: 2F1FFFFF  cmpwi cr6, r31, -1
	ctx.cr[6].compare_i32(ctx.r[31].s32, -1, &mut ctx.xer);
	// 822F1640: 409AFF7C  bne cr6, 0x822f15bc
	if !ctx.cr[6].eq {
	pc = 0x822F15BC; continue 'dispatch;
	}
	// 822F1644: 382102A0  addi r1, r1, 0x2a0
	ctx.r[1].s64 = ctx.r[1].s64 + 672;
	// 822F1648: 48EB6B64  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822F1650(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822F1650 size=144
    let mut pc: u32 = 0x822F1650;
    'dispatch: loop {
        match pc {
            0x822F1650 => {
    //   block [0x822F1650..0x822F16E0)
	// 822F1650: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822F1654: 48EB6B0D  bl 0x831a8160
	ctx.lr = 0x822F1658;
	sub_831A8130(ctx, base);
	// 822F1658: 9421FEE0  stwu r1, -0x120(r1)
	ea = ctx.r[1].u32.wrapping_add(-288 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822F165C: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 822F1660: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 822F1664: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 822F1668: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 822F166C: 7CFB3B78  mr r27, r7
	ctx.r[27].u64 = ctx.r[7].u64;
	// 822F1670: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 822F1674: 7D1A4378  mr r26, r8
	ctx.r[26].u64 = ctx.r[8].u64;
	// 822F1678: 806B0014  lwz r3, 0x14(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 822F167C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 822F1680: 90610050  stw r3, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 822F1684: 419A0008  beq cr6, 0x822f168c
	if ctx.cr[6].eq {
	pc = 0x822F168C; continue 'dispatch;
	}
	// 822F1688: 4BFF6BC1  bl 0x822e8248
	ctx.lr = 0x822F168C;
	sub_822E8248(ctx, base);
	// 822F168C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 822F1690: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 822F1694: 388B0030  addi r4, r11, 0x30
	ctx.r[4].s64 = ctx.r[11].s64 + 48;
	// 822F1698: 4BFF7001  bl 0x822e8698
	ctx.lr = 0x822F169C;
	sub_822E8698(ctx, base);
	// 822F169C: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 822F16A0: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 822F16A4: 386100A0  addi r3, r1, 0xa0
	ctx.r[3].s64 = ctx.r[1].s64 + 160;
	// 822F16A8: 4BFD3259  bl 0x822c4900
	ctx.lr = 0x822F16AC;
	sub_822C4900(ctx, base);
	// 822F16AC: 7C681B78  mr r8, r3
	ctx.r[8].u64 = ctx.r[3].u64;
	// 822F16B0: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 822F16B4: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 822F16B8: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 822F16BC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 822F16C0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 822F16C4: 480017ED  bl 0x822f2eb0
	ctx.lr = 0x822F16C8;
	sub_822F2EB0(ctx, base);
	// 822F16C8: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 822F16CC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 822F16D0: 419A0008  beq cr6, 0x822f16d8
	if ctx.cr[6].eq {
	pc = 0x822F16D8; continue 'dispatch;
	}
	// 822F16D4: 4BFF6B95  bl 0x822e8268
	ctx.lr = 0x822F16D8;
	sub_822E8268(ctx, base);
	// 822F16D8: 38210120  addi r1, r1, 0x120
	ctx.r[1].s64 = ctx.r[1].s64 + 288;
	// 822F16DC: 48EB6AD4  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822F16E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822F16E0 size=144
    let mut pc: u32 = 0x822F16E0;
    'dispatch: loop {
        match pc {
            0x822F16E0 => {
    //   block [0x822F16E0..0x822F1770)
	// 822F16E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822F16E4: 48EB6A7D  bl 0x831a8160
	ctx.lr = 0x822F16E8;
	sub_831A8130(ctx, base);
	// 822F16E8: 9421FEE0  stwu r1, -0x120(r1)
	ea = ctx.r[1].u32.wrapping_add(-288 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822F16EC: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 822F16F0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 822F16F4: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 822F16F8: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 822F16FC: 7CFB3B78  mr r27, r7
	ctx.r[27].u64 = ctx.r[7].u64;
	// 822F1700: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 822F1704: 7D1A4378  mr r26, r8
	ctx.r[26].u64 = ctx.r[8].u64;
	// 822F1708: 806B0018  lwz r3, 0x18(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 822F170C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 822F1710: 90610050  stw r3, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 822F1714: 419A0008  beq cr6, 0x822f171c
	if ctx.cr[6].eq {
	pc = 0x822F171C; continue 'dispatch;
	}
	// 822F1718: 4BFF6B31  bl 0x822e8248
	ctx.lr = 0x822F171C;
	sub_822E8248(ctx, base);
	// 822F171C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 822F1720: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 822F1724: 388B0020  addi r4, r11, 0x20
	ctx.r[4].s64 = ctx.r[11].s64 + 32;
	// 822F1728: 4BFF6F71  bl 0x822e8698
	ctx.lr = 0x822F172C;
	sub_822E8698(ctx, base);
	// 822F172C: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 822F1730: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 822F1734: 386100A0  addi r3, r1, 0xa0
	ctx.r[3].s64 = ctx.r[1].s64 + 160;
	// 822F1738: 4BFD31C9  bl 0x822c4900
	ctx.lr = 0x822F173C;
	sub_822C4900(ctx, base);
	// 822F173C: 7C681B78  mr r8, r3
	ctx.r[8].u64 = ctx.r[3].u64;
	// 822F1740: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 822F1744: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 822F1748: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 822F174C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 822F1750: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 822F1754: 4800175D  bl 0x822f2eb0
	ctx.lr = 0x822F1758;
	sub_822F2EB0(ctx, base);
	// 822F1758: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 822F175C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 822F1760: 419A0008  beq cr6, 0x822f1768
	if ctx.cr[6].eq {
	pc = 0x822F1768; continue 'dispatch;
	}
	// 822F1764: 4BFF6B05  bl 0x822e8268
	ctx.lr = 0x822F1768;
	sub_822E8268(ctx, base);
	// 822F1768: 38210120  addi r1, r1, 0x120
	ctx.r[1].s64 = ctx.r[1].s64 + 288;
	// 822F176C: 48EB6A44  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822F1770(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822F1770 size=144
    let mut pc: u32 = 0x822F1770;
    'dispatch: loop {
        match pc {
            0x822F1770 => {
    //   block [0x822F1770..0x822F1800)
	// 822F1770: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822F1774: 48EB69ED  bl 0x831a8160
	ctx.lr = 0x822F1778;
	sub_831A8130(ctx, base);
	// 822F1778: 9421FEE0  stwu r1, -0x120(r1)
	ea = ctx.r[1].u32.wrapping_add(-288 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822F177C: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 822F1780: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 822F1784: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 822F1788: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 822F178C: 7CFB3B78  mr r27, r7
	ctx.r[27].u64 = ctx.r[7].u64;
	// 822F1790: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 822F1794: 7D1A4378  mr r26, r8
	ctx.r[26].u64 = ctx.r[8].u64;
	// 822F1798: 806B0018  lwz r3, 0x18(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 822F179C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 822F17A0: 90610050  stw r3, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 822F17A4: 419A0008  beq cr6, 0x822f17ac
	if ctx.cr[6].eq {
	pc = 0x822F17AC; continue 'dispatch;
	}
	// 822F17A8: 4BFF6AA1  bl 0x822e8248
	ctx.lr = 0x822F17AC;
	sub_822E8248(ctx, base);
	// 822F17AC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 822F17B0: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 822F17B4: 388B0020  addi r4, r11, 0x20
	ctx.r[4].s64 = ctx.r[11].s64 + 32;
	// 822F17B8: 48B8A821  bl 0x82e7bfd8
	ctx.lr = 0x822F17BC;
	sub_82E7BFD8(ctx, base);
	// 822F17BC: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 822F17C0: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 822F17C4: 386100A0  addi r3, r1, 0xa0
	ctx.r[3].s64 = ctx.r[1].s64 + 160;
	// 822F17C8: 4BFD3139  bl 0x822c4900
	ctx.lr = 0x822F17CC;
	sub_822C4900(ctx, base);
	// 822F17CC: 7C681B78  mr r8, r3
	ctx.r[8].u64 = ctx.r[3].u64;
	// 822F17D0: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 822F17D4: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 822F17D8: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 822F17DC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 822F17E0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 822F17E4: 480016CD  bl 0x822f2eb0
	ctx.lr = 0x822F17E8;
	sub_822F2EB0(ctx, base);
	// 822F17E8: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 822F17EC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 822F17F0: 419A0008  beq cr6, 0x822f17f8
	if ctx.cr[6].eq {
	pc = 0x822F17F8; continue 'dispatch;
	}
	// 822F17F4: 4BFF6A75  bl 0x822e8268
	ctx.lr = 0x822F17F8;
	sub_822E8268(ctx, base);
	// 822F17F8: 38210120  addi r1, r1, 0x120
	ctx.r[1].s64 = ctx.r[1].s64 + 288;
	// 822F17FC: 48EB69B4  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822F1800(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x822F1800 size=236
    let mut pc: u32 = 0x822F1800;
    'dispatch: loop {
        match pc {
            0x822F1800 => {
    //   block [0x822F1800..0x822F18EC)
	// 822F1800: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822F1804: 48EB6969  bl 0x831a816c
	ctx.lr = 0x822F1808;
	sub_831A8130(ctx, base);
	// 822F1808: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822F180C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822F1810: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 822F1814: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 822F1818: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 822F181C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 822F1820: 419A00B0  beq cr6, 0x822f18d0
	if ctx.cr[6].eq {
	pc = 0x822F18D0; continue 'dispatch;
	}
	// 822F1824: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 822F1828: 488ECF99  bl 0x82bde7c0
	ctx.lr = 0x822F182C;
	sub_82BDE7C0(ctx, base);
	// 822F182C: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 822F1830: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 822F1834: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 822F1838: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 822F183C: 488ED94D  bl 0x82bdf188
	ctx.lr = 0x822F1840;
	sub_82BDF188(ctx, base);
	// 822F1840: 81610070  lwz r11, 0x70(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	// 822F1844: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 822F1848: 38E00020  li r7, 0x20
	ctx.r[7].s64 = 32;
	// 822F184C: 5566D97E  srwi r6, r11, 5
	ctx.r[6].u32 = ctx.r[11].u32.wrapping_shr(5);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 822F1850: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 822F1854: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 822F1858: 481A2821  bl 0x82494078
	ctx.lr = 0x822F185C;
	sub_82494078(ctx, base);
	// 822F185C: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 822F1860: 488ED979  bl 0x82bdf1d8
	ctx.lr = 0x822F1864;
	sub_82BDF1D8(ctx, base);
	// 822F1864: 807F002C  lwz r3, 0x2c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 822F1868: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 822F186C: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 822F1870: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 822F1874: 4E800421  bctrl
	ctx.lr = 0x822F1878;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 822F1878: 39600010  li r11, 0x10
	ctx.r[11].s64 = 16;
	// 822F187C: 39400020  li r10, 0x20
	ctx.r[10].s64 = 32;
	// 822F1880: 13E01C07  vcmpneb. (lvlx128) v31, v0, v3
	tmp.u32 = ctx.r[3].u32;
	// load shuffled into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 822F1884: 39200030  li r9, 0x30
	ctx.r[9].s64 = 48;
	// 822F1888: 39010080  addi r8, r1, 0x80
	ctx.r[8].s64 = ctx.r[1].s64 + 128;
	// 822F188C: 38E10090  addi r7, r1, 0x90
	ctx.r[7].s64 = ctx.r[1].s64 + 144;
	// 822F1890: 13CB1C07  vcmpneb. (lvlx128) v30, v11, v3
	tmp.u32 = ctx.r[11].u32 + ctx.r[3].u32;
	// load shuffled into ctx.v[62] using VectorMaskL[(tmp.u32 & 0xF)]
	// 822F1894: 38C100A0  addi r6, r1, 0xa0
	ctx.r[6].s64 = ctx.r[1].s64 + 160;
	// 822F1898: 396100B0  addi r11, r1, 0xb0
	ctx.r[11].s64 = ctx.r[1].s64 + 176;
	// 822F189C: 13AA1C07  vcmpneb. (lvlx128) v29, v10, v3
	tmp.u32 = ctx.r[10].u32 + ctx.r[3].u32;
	// load shuffled into ctx.v[61] using VectorMaskL[(tmp.u32 & 0xF)]
	// 822F18A0: 13891C07  vcmpneb. (lvlx128) v28, v9, v3
	tmp.u32 = ctx.r[9].u32 + ctx.r[3].u32;
	// load shuffled into ctx.v[60] using VectorMaskL[(tmp.u32 & 0xF)]
	// 822F18A4: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822F18F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822F18F0 size=88
    let mut pc: u32 = 0x822F18F0;
    'dispatch: loop {
        match pc {
            0x822F18F0 => {
    //   block [0x822F18F0..0x822F1948)
	// 822F18F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822F18F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822F18F8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822F18FC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822F1900: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822F1904: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 822F1908: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 822F190C: 419A0008  beq cr6, 0x822f1914
	if ctx.cr[6].eq {
	pc = 0x822F1914; continue 'dispatch;
	}
	// 822F1910: 4BFCEF81  bl 0x822c0890
	ctx.lr = 0x822F1914;
	sub_822C0890(ctx, base);
	// 822F1914: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 822F1918: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 822F191C: 419A0008  beq cr6, 0x822f1924
	if ctx.cr[6].eq {
	pc = 0x822F1924; continue 'dispatch;
	}
	// 822F1920: 4BFCEF71  bl 0x822c0890
	ctx.lr = 0x822F1924;
	sub_822C0890(ctx, base);
	// 822F1924: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 822F1928: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 822F192C: 419A0008  beq cr6, 0x822f1934
	if ctx.cr[6].eq {
	pc = 0x822F1934; continue 'dispatch;
	}
	// 822F1930: 4BFCEF61  bl 0x822c0890
	ctx.lr = 0x822F1934;
	sub_822C0890(ctx, base);
	// 822F1934: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 822F1938: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822F193C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822F1940: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822F1944: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822F1948(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x822F1948 size=736
    let mut pc: u32 = 0x822F1948;
    'dispatch: loop {
        match pc {
            0x822F1948 => {
    //   block [0x822F1948..0x822F1C28)
	// 822F1948: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822F194C: 48EB6811  bl 0x831a815c
	ctx.lr = 0x822F1950;
	sub_831A8130(ctx, base);
	// 822F1950: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822F1954: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822F1958: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 822F195C: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 822F1960: 7CFA3B78  mr r26, r7
	ctx.r[26].u64 = ctx.r[7].u64;
	// 822F1964: 7D194378  mr r25, r8
	ctx.r[25].u64 = ctx.r[8].u64;
	// 822F1968: 913F0044  stw r9, 0x44(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(68 as u32), ctx.r[9].u32 ) };
	// 822F196C: 7D5E5378  mr r30, r10
	ctx.r[30].u64 = ctx.r[10].u64;
	// 822F1970: 48B39179  bl 0x82e2aae8
	ctx.lr = 0x822F1974;
	sub_82E2AAE8(ctx, base);
	// 822F1974: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 822F1978: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 822F197C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 822F1980: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 822F1984: 48B3CFDD  bl 0x82e2e960
	ctx.lr = 0x822F1988;
	sub_82E2E960(ctx, base);
	// 822F1988: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 822F198C: 395F0014  addi r10, r31, 0x14
	ctx.r[10].s64 = ctx.r[31].s64 + 20;
	// 822F1990: 388B0004  addi r4, r11, 4
	ctx.r[4].s64 = ctx.r[11].s64 + 4;
	// 822F1994: 386A0004  addi r3, r10, 4
	ctx.r[3].s64 = ctx.r[10].s64 + 4;
	// 822F1998: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 822F199C: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 822F19A0: 4BFD2AC1  bl 0x822c4460
	ctx.lr = 0x822F19A4;
	sub_822C4460(ctx, base);
	// 822F19A4: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 822F19A8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 822F19AC: 419A0008  beq cr6, 0x822f19b4
	if ctx.cr[6].eq {
	pc = 0x822F19B4; continue 'dispatch;
	}
	// 822F19B0: 4BFCEEE1  bl 0x822c0890
	ctx.lr = 0x822F19B4;
	sub_822C0890(ctx, base);
	// 822F19B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 822F19B8: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 822F19BC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 822F19C0: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 822F19C4: 48B3CE4D  bl 0x82e2e810
	ctx.lr = 0x822F19C8;
	sub_82E2E810(ctx, base);
	// 822F19C8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 822F19CC: 395F001C  addi r10, r31, 0x1c
	ctx.r[10].s64 = ctx.r[31].s64 + 28;
	// 822F19D0: 388B0004  addi r4, r11, 4
	ctx.r[4].s64 = ctx.r[11].s64 + 4;
	// 822F19D4: 386A0004  addi r3, r10, 4
	ctx.r[3].s64 = ctx.r[10].s64 + 4;
	// 822F19D8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 822F19DC: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 822F19E0: 4BFD2A81  bl 0x822c4460
	ctx.lr = 0x822F19E4;
	sub_822C4460(ctx, base);
	// 822F19E4: 80610064  lwz r3, 0x64(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 822F19E8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 822F19EC: 419A0008  beq cr6, 0x822f19f4
	if ctx.cr[6].eq {
	pc = 0x822F19F4; continue 'dispatch;
	}
	// 822F19F0: 4BFCEEA1  bl 0x822c0890
	ctx.lr = 0x822F19F4;
	sub_822C0890(ctx, base);
	// 822F19F4: 815D0000  lwz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 822F19F8: 397F002C  addi r11, r31, 0x2c
	ctx.r[11].s64 = ctx.r[31].s64 + 44;
	// 822F19FC: 389D0004  addi r4, r29, 4
	ctx.r[4].s64 = ctx.r[29].s64 + 4;
	// 822F1A00: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 822F1A04: 915F002C  stw r10, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[10].u32 ) };
	// 822F1A08: 4BFD2A59  bl 0x822c4460
	ctx.lr = 0x822F1A0C;
	sub_822C4460(ctx, base);
	// 822F1A0C: 817A0004  lwz r11, 4(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 822F1A10: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 822F1A14: 419A0010  beq cr6, 0x822f1a24
	if ctx.cr[6].eq {
	pc = 0x822F1A24; continue 'dispatch;
	}
	// 822F1A18: 815A0008  lwz r10, 8(r26)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 822F1A1C: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 822F1A20: 7D6B2E70  srawi r11, r11, 5
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 5) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 5) as i64;
	// 822F1A24: 557E2835  rlwinm. r30, r11, 5, 0, 0x1a
	ctx.r[30].u64 = ctx.r[11].u32 as u64 & 0x07FFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 822F1A28: 41820074  beq 0x822f1a9c
	if ctx.cr[0].eq {
	pc = 0x822F1A9C; continue 'dispatch;
	}
	// 822F1A2C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 822F1A30: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 822F1A34: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 822F1A38: 488ED689  bl 0x82bdf0c0
	ctx.lr = 0x822F1A3C;
	sub_82BDF0C0(ctx, base);
	// 822F1A3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 822F1A40: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 822F1A44: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 822F1A48: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 822F1A4C: 488ED73D  bl 0x82bdf188
	ctx.lr = 0x822F1A50;
	sub_82BDF188(ctx, base);
	// 822F1A50: 839A0008  lwz r28, 8(r26)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 822F1A54: 83DA0004  lwz r30, 4(r26)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 822F1A58: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 822F1A5C: 4800001C  b 0x822f1a78
	pc = 0x822F1A78; continue 'dispatch;
	// 822F1A60: 38A00020  li r5, 0x20
	ctx.r[5].s64 = 32;
	// 822F1A64: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 822F1A68: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 822F1A6C: 48EB6AA5  bl 0x831a8510
	ctx.lr = 0x822F1A70;
	sub_831A8510(ctx, base);
	// 822F1A70: 3BDE0020  addi r30, r30, 0x20
	ctx.r[30].s64 = ctx.r[30].s64 + 32;
	// 822F1A74: 3BBD0020  addi r29, r29, 0x20
	ctx.r[29].s64 = ctx.r[29].s64 + 32;
	// 822F1A78: 7F1EE040  cmplw cr6, r30, r28
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[28].u32, &mut ctx.xer);
	// 822F1A7C: 409AFFE4  bne cr6, 0x822f1a60
	if !ctx.cr[6].eq {
	pc = 0x822F1A60; continue 'dispatch;
	}
	// 822F1A80: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 822F1A84: 488ED755  bl 0x82bdf1d8
	ctx.lr = 0x822F1A88;
	sub_82BDF1D8(ctx, base);
	// 822F1A88: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 822F1A8C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 822F1A90: 419A0008  beq cr6, 0x822f1a98
	if ctx.cr[6].eq {
	pc = 0x822F1A98; continue 'dispatch;
	}
	// 822F1A94: 488ED545  bl 0x82bdefd8
	ctx.lr = 0x822F1A98;
	sub_82BDEFD8(ctx, base);
	// 822F1A98: 937F0008  stw r27, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[27].u32 ) };
	// 822F1A9C: 81790004  lwz r11, 4(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(4 as u32) ) } as u64;
	// 822F1AA0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 822F1AA4: 419A0010  beq cr6, 0x822f1ab4
	if ctx.cr[6].eq {
	pc = 0x822F1AB4; continue 'dispatch;
	}
	// 822F1AA8: 81590008  lwz r10, 8(r25)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(8 as u32) ) } as u64;
	// 822F1AAC: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 822F1AB0: 7D6B1670  srawi r11, r11, 2
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 2) as i64;
	// 822F1AB4: 557E103B  rlwinm. r30, r11, 2, 0, 0x1d
	ctx.r[30].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 822F1AB8: 4182006C  beq 0x822f1b24
	if ctx.cr[0].eq {
	pc = 0x822F1B24; continue 'dispatch;
	}
	// 822F1ABC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 822F1AC0: 38A00006  li r5, 6
	ctx.r[5].s64 = 6;
	// 822F1AC4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 822F1AC8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 822F1ACC: 488ED71D  bl 0x82bdf1e8
	ctx.lr = 0x822F1AD0;
	sub_82BDF1E8(ctx, base);
	// 822F1AD0: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 822F1AD4: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 822F1AD8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 822F1ADC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 822F1AE0: 488ED7B9  bl 0x82bdf298
	ctx.lr = 0x822F1AE4;
	sub_82BDF298(ctx, base);
	// 822F1AE4: 81590008  lwz r10, 8(r25)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(8 as u32) ) } as u64;
	// 822F1AE8: 81790004  lwz r11, 4(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(4 as u32) ) } as u64;
	// 822F1AEC: 48000014  b 0x822f1b00
	pc = 0x822F1B00; continue 'dispatch;
	// 822F1AF0: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 822F1AF4: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 822F1AF8: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 822F1AFC: 38630004  addi r3, r3, 4
	ctx.r[3].s64 = ctx.r[3].s64 + 4;
	// 822F1B00: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 822F1B04: 409AFFEC  bne cr6, 0x822f1af0
	if !ctx.cr[6].eq {
	pc = 0x822F1AF0; continue 'dispatch;
	}
	// 822F1B08: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 822F1B0C: 488ED7D5  bl 0x82bdf2e0
	ctx.lr = 0x822F1B10;
	sub_82BDF2E0(ctx, base);
	// 822F1B10: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 822F1B14: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 822F1B18: 419A0008  beq cr6, 0x822f1b20
	if ctx.cr[6].eq {
	pc = 0x822F1B20; continue 'dispatch;
	}
	// 822F1B1C: 488ED4BD  bl 0x82bdefd8
	ctx.lr = 0x822F1B20;
	sub_82BDEFD8(ctx, base);
	// 822F1B20: 93BF000C  stw r29, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[29].u32 ) };
	// 822F1B24: 817A0004  lwz r11, 4(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 822F1B28: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 822F1B2C: 419A0010  beq cr6, 0x822f1b3c
	if ctx.cr[6].eq {
	pc = 0x822F1B3C; continue 'dispatch;
	}
	// 822F1B30: 815A0008  lwz r10, 8(r26)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 822F1B34: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 822F1B38: 7D6B2E70  srawi r11, r11, 5
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 5) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 5) as i64;
	// 822F1B3C: 917F0034  stw r11, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 822F1B40: 81790004  lwz r11, 4(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(4 as u32) ) } as u64;
	// 822F1B44: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 822F1B48: 419A0010  beq cr6, 0x822f1b58
	if ctx.cr[6].eq {
	pc = 0x822F1B58; continue 'dispatch;
	}
	// 822F1B4C: 81590008  lwz r10, 8(r25)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(8 as u32) ) } as u64;
	// 822F1B50: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 822F1B54: 7D6B1670  srawi r11, r11, 2
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 2) as i64;
	// 822F1B58: 917F0038  stw r11, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 822F1B5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 822F1B60: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 822F1B64: 80A10114  lwz r5, 0x114(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(276 as u32) ) } as u64;
	// 822F1B68: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 822F1B6C: 48B3A075  bl 0x82e2bbe0
	ctx.lr = 0x822F1B70;
	sub_82E2BBE0(ctx, base);
	// 822F1B70: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 822F1B74: 395F003C  addi r10, r31, 0x3c
	ctx.r[10].s64 = ctx.r[31].s64 + 60;
	// 822F1B78: 388B0004  addi r4, r11, 4
	ctx.r[4].s64 = ctx.r[11].s64 + 4;
	// 822F1B7C: 386A0004  addi r3, r10, 4
	ctx.r[3].s64 = ctx.r[10].s64 + 4;
	// 822F1B80: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 822F1B84: 917F003C  stw r11, 0x3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[11].u32 ) };
	// 822F1B88: 4BFD28D9  bl 0x822c4460
	ctx.lr = 0x822F1B8C;
	sub_822C4460(ctx, base);
	// 822F1B8C: 8061006C  lwz r3, 0x6c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(108 as u32) ) } as u64;
	// 822F1B90: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 822F1B94: 419A0008  beq cr6, 0x822f1b9c
	if ctx.cr[6].eq {
	pc = 0x822F1B9C; continue 'dispatch;
	}
	// 822F1B98: 4BFCECF9  bl 0x822c0890
	ctx.lr = 0x822F1B9C;
	sub_822C0890(ctx, base);
	// 822F1B9C: 8161011C  lwz r11, 0x11c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(284 as u32) ) } as u64;
	// 822F1BA0: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 822F1BA4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 822F1BA8: 892B0001  lbz r9, 1(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(1 as u32) ) } as u64;
	// 822F1BAC: C00A94AC  lfs f0, -0x6b54(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-27476 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 822F1BB0: 894B0000  lbz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 822F1BB4: 890B0003  lbz r8, 3(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(3 as u32) ) } as u64;
	// 822F1BB8: 896B0002  lbz r11, 2(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(2 as u32) ) } as u64;
	// 822F1BBC: F9210070  std r9, 0x70(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[9].u64 ) };
	// 822F1BC0: C9A10070  lfd f13, 0x70(r1)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) };
	// 822F1BC4: F9410078  std r10, 0x78(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[10].u64 ) };
	// 822F1BC8: C9810078  lfd f12, 0x78(r1)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(120 as u32) ) };
	// 822F1BCC: F9010070  std r8, 0x70(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[8].u64 ) };
	// 822F1BD0: C9610070  lfd f11, 0x70(r1)
	ctx.f[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) };
	// 822F1BD4: F9610078  std r11, 0x78(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[11].u64 ) };
	// 822F1BD8: C9410078  lfd f10, 0x78(r1)
	ctx.f[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(120 as u32) ) };
	// 822F1BDC: FD40569C  fcfid f10, f10
	ctx.f[10].f64 = (ctx.f[10].s64 as f64);
	// 822F1BE0: FD605E9C  fcfid f11, f11
	ctx.f[11].f64 = (ctx.f[11].s64 as f64);
	// 822F1BE4: FD80669C  fcfid f12, f12
	ctx.f[12].f64 = (ctx.f[12].s64 as f64);
	// 822F1BE8: FDA06E9C  fcfid f13, f13
	ctx.f[13].f64 = (ctx.f[13].s64 as f64);
	// 822F1BEC: FD405018  frsp f10, f10
	ctx.f[10].f64 = (ctx.f[10].f64 as f32) as f64;
	// 822F1BF0: FD605818  frsp f11, f11
	ctx.f[11].f64 = (ctx.f[11].f64 as f32) as f64;
	// 822F1BF4: FD806018  frsp f12, f12
	ctx.f[12].f64 = (ctx.f[12].f64 as f32) as f64;
	// 822F1BF8: FDA06818  frsp f13, f13
	ctx.f[13].f64 = (ctx.f[13].f64 as f32) as f64;
	// 822F1BFC: ED4A0032  fmuls f10, f10, f0
	ctx.f[10].f64 = (((ctx.f[10].f64 * ctx.f[0].f64) as f32) as f64);
	// 822F1C00: D15F0054  stfs f10, 0x54(r31)
	tmp.f32 = (ctx.f[10].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 822F1C04: ED6B0032  fmuls f11, f11, f0
	ctx.f[11].f64 = (((ctx.f[11].f64 * ctx.f[0].f64) as f32) as f64);
	// 822F1C08: D17F0058  stfs f11, 0x58(r31)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), tmp.u32 ) };
	// 822F1C0C: ED8C0032  fmuls f12, f12, f0
	ctx.f[12].f64 = (((ctx.f[12].f64 * ctx.f[0].f64) as f32) as f64);
	// 822F1C10: D19F005C  stfs f12, 0x5c(r31)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), tmp.u32 ) };
	// 822F1C14: EC0D0032  fmuls f0, f13, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 822F1C18: D01F0050  stfs f0, 0x50(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 822F1C1C: 48B38EE5  bl 0x82e2ab00
	ctx.lr = 0x822F1C20;
	sub_82E2AB00(ctx, base);
	// 822F1C20: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 822F1C24: 48EB6588  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822F1C28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x822F1C28 size=976
    let mut pc: u32 = 0x822F1C28;
    'dispatch: loop {
        match pc {
            0x822F1C28 => {
    //   block [0x822F1C28..0x822F1FF8)
	// 822F1C28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822F1C2C: 48EB6531  bl 0x831a815c
	ctx.lr = 0x822F1C30;
	sub_831A8130(ctx, base);
	// 822F1C30: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822F1C34: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 822F1C38: 7CB92B78  mr r25, r5
	ctx.r[25].u64 = ctx.r[5].u64;
	// 822F1C3C: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 822F1C40: 817E0044  lwz r11, 0x44(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(68 as u32) ) } as u64;
	// 822F1C44: 7F195840  cmplw cr6, r25, r11
	ctx.cr[6].compare_u32(ctx.r[25].u32, ctx.r[11].u32, &mut ctx.xer);
	// 822F1C48: 409A03A8  bne cr6, 0x822f1ff0
	if !ctx.cr[6].eq {
	pc = 0x822F1FF0; continue 'dispatch;
	}
	// 822F1C4C: 3FA08335  lis r29, -0x7ccb
	ctx.r[29].s64 = -2093678592;
	// 822F1C50: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 822F1C54: 3B4B17F8  addi r26, r11, 0x17f8
	ctx.r[26].s64 = ctx.r[11].s64 + 6136;
	// 822F1C58: 817D17FC  lwz r11, 0x17fc(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(6140 as u32) ) } as u64;
	// 822F1C5C: 556A07FF  clrlwi. r10, r11, 0x1f
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 822F1C60: 40820020  bne 0x822f1c80
	if !ctx.cr[0].eq {
	pc = 0x822F1C80; continue 'dispatch;
	}
	// 822F1C64: 616B0001  ori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 | 1;
	// 822F1C68: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 822F1C6C: 917D17FC  stw r11, 0x17fc(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(6140 as u32), ctx.r[11].u32 ) };
	// 822F1C70: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 822F1C74: 388A9428  addi r4, r10, -0x6bd8
	ctx.r[4].s64 = ctx.r[10].s64 + -27608;
	// 822F1C78: 48B078B9  bl 0x82df9530
	ctx.lr = 0x822F1C7C;
	sub_82DF9530(ctx, base);
	// 822F1C7C: 817D17FC  lwz r11, 0x17fc(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(6140 as u32) ) } as u64;
	// 822F1C80: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 822F1C84: 556907BD  rlwinm. r9, r11, 0, 0x1e, 0x1e
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 822F1C88: 3B8A17F4  addi r28, r10, 0x17f4
	ctx.r[28].s64 = ctx.r[10].s64 + 6132;
	// 822F1C8C: 40820020  bne 0x822f1cac
	if !ctx.cr[0].eq {
	pc = 0x822F1CAC; continue 'dispatch;
	}
	// 822F1C90: 616B0002  ori r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u64 | 2;
	// 822F1C94: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 822F1C98: 917D17FC  stw r11, 0x17fc(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(6140 as u32), ctx.r[11].u32 ) };
	// 822F1C9C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 822F1CA0: 388A9430  addi r4, r10, -0x6bd0
	ctx.r[4].s64 = ctx.r[10].s64 + -27600;
	// 822F1CA4: 48B0788D  bl 0x82df9530
	ctx.lr = 0x822F1CA8;
	sub_82DF9530(ctx, base);
	// 822F1CA8: 817D17FC  lwz r11, 0x17fc(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(6140 as u32) ) } as u64;
	// 822F1CAC: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 822F1CB0: 5569077B  rlwinm. r9, r11, 0, 0x1d, 0x1d
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 822F1CB4: 3BEA17F0  addi r31, r10, 0x17f0
	ctx.r[31].s64 = ctx.r[10].s64 + 6128;
	// 822F1CB8: 4082001C  bne 0x822f1cd4
	if !ctx.cr[0].eq {
	pc = 0x822F1CD4; continue 'dispatch;
	}
	// 822F1CBC: 616B0004  ori r11, r11, 4
	ctx.r[11].u64 = ctx.r[11].u64 | 4;
	// 822F1CC0: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 822F1CC4: 917D17FC  stw r11, 0x17fc(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(6140 as u32), ctx.r[11].u32 ) };
	// 822F1CC8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822F1CCC: 388AC508  addi r4, r10, -0x3af8
	ctx.r[4].s64 = ctx.r[10].s64 + -15096;
	// 822F1CD0: 48B07861  bl 0x82df9530
	ctx.lr = 0x822F1CD4;
	sub_82DF9530(ctx, base);
	// 822F1CD4: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 822F1CD8: 7F195840  cmplw cr6, r25, r11
	ctx.cr[6].compare_u32(ctx.r[25].u32, ctx.r[11].u32, &mut ctx.xer);
	// 822F1CDC: 409A0020  bne cr6, 0x822f1cfc
	if !ctx.cr[6].eq {
	pc = 0x822F1CFC; continue 'dispatch;
	}
	// 822F1CE0: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 822F1CE4: 807B0000  lwz r3, 0(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 822F1CE8: 38800030  li r4, 0x30
	ctx.r[4].s64 = 48;
	// 822F1CEC: 4BFF35FD  bl 0x822e52e8
	ctx.lr = 0x822F1CF0;
	sub_822E52E8(ctx, base);
	// 822F1CF0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 822F1CF4: 38800060  li r4, 0x60
	ctx.r[4].s64 = 96;
	// 822F1CF8: 480000A0  b 0x822f1d98
	pc = 0x822F1D98; continue 'dispatch;
	// 822F1CFC: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 822F1D00: 7F195840  cmplw cr6, r25, r11
	ctx.cr[6].compare_u32(ctx.r[25].u32, ctx.r[11].u32, &mut ctx.xer);
	// 822F1D04: 409A0050  bne cr6, 0x822f1d54
	if !ctx.cr[6].eq {
	pc = 0x822F1D54; continue 'dispatch;
	}
	// 822F1D08: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 822F1D0C: 807B0000  lwz r3, 0(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 822F1D10: 38800030  li r4, 0x30
	ctx.r[4].s64 = 48;
	// 822F1D14: 4BFF35D5  bl 0x822e52e8
	ctx.lr = 0x822F1D18;
	sub_822E52E8(ctx, base);
	// 822F1D18: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 822F1D1C: 38800060  li r4, 0x60
	ctx.r[4].s64 = 96;
	// 822F1D20: 807B0000  lwz r3, 0(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 822F1D24: 4BFF35C5  bl 0x822e52e8
	ctx.lr = 0x822F1D28;
	sub_822E52E8(ctx, base);
	// 822F1D28: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 822F1D2C: 3880003C  li r4, 0x3c
	ctx.r[4].s64 = 60;
	// 822F1D30: 807B0000  lwz r3, 0(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 822F1D34: 4BFF35B5  bl 0x822e52e8
	ctx.lr = 0x822F1D38;
	sub_822E52E8(ctx, base);
	// 822F1D38: 38A00006  li r5, 6
	ctx.r[5].s64 = 6;
	// 822F1D3C: 38800048  li r4, 0x48
	ctx.r[4].s64 = 72;
	// 822F1D40: 807B0000  lwz r3, 0(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 822F1D44: 4BFF35A5  bl 0x822e52e8
	ctx.lr = 0x822F1D48;
	sub_822E52E8(ctx, base);
	// 822F1D48: 38A00007  li r5, 7
	ctx.r[5].s64 = 7;
	// 822F1D4C: 3880004C  li r4, 0x4c
	ctx.r[4].s64 = 76;
	// 822F1D50: 48000058  b 0x822f1da8
	pc = 0x822F1DA8; continue 'dispatch;
	// 822F1D54: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 822F1D58: 7F195840  cmplw cr6, r25, r11
	ctx.cr[6].compare_u32(ctx.r[25].u32, ctx.r[11].u32, &mut ctx.xer);
	// 822F1D5C: 409A0054  bne cr6, 0x822f1db0
	if !ctx.cr[6].eq {
	pc = 0x822F1DB0; continue 'dispatch;
	}
	// 822F1D60: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 822F1D64: 807B0000  lwz r3, 0(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 822F1D68: 38800030  li r4, 0x30
	ctx.r[4].s64 = 48;
	// 822F1D6C: 4BFF357D  bl 0x822e52e8
	ctx.lr = 0x822F1D70;
	sub_822E52E8(ctx, base);
	// 822F1D70: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 822F1D74: 38800060  li r4, 0x60
	ctx.r[4].s64 = 96;
	// 822F1D78: 807B0000  lwz r3, 0(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 822F1D7C: 4BFF356D  bl 0x822e52e8
	ctx.lr = 0x822F1D80;
	sub_822E52E8(ctx, base);
	// 822F1D80: 38A00080  li r5, 0x80
	ctx.r[5].s64 = 128;
	// 822F1D84: 38800064  li r4, 0x64
	ctx.r[4].s64 = 100;
	// 822F1D88: 807B0000  lwz r3, 0(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 822F1D8C: 4BFF355D  bl 0x822e52e8
	ctx.lr = 0x822F1D90;
	sub_822E52E8(ctx, base);
	// 822F1D90: 38A00006  li r5, 6
	ctx.r[5].s64 = 6;
	// 822F1D94: 38800068  li r4, 0x68
	ctx.r[4].s64 = 104;
	// 822F1D98: 807B0000  lwz r3, 0(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 822F1D9C: 4BFF354D  bl 0x822e52e8
	ctx.lr = 0x822F1DA0;
	sub_822E52E8(ctx, base);
	// 822F1DA0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 822F1DA4: 3880003C  li r4, 0x3c
	ctx.r[4].s64 = 60;
	// 822F1DA8: 807B0000  lwz r3, 0(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 822F1DAC: 4BFF353D  bl 0x822e52e8
	ctx.lr = 0x822F1DB0;
	sub_822E52E8(ctx, base);
	// 822F1DB0: 817E0060  lwz r11, 0x60(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(96 as u32) ) } as u64;
	// 822F1DB4: 3BFE0060  addi r31, r30, 0x60
	ctx.r[31].s64 = ctx.r[30].s64 + 96;
	// 822F1DB8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 822F1DBC: 409A0018  bne cr6, 0x822f1dd4
	if !ctx.cr[6].eq {
	pc = 0x822F1DD4; continue 'dispatch;
	}
	// 822F1DC0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 822F1DC4: 807B0000  lwz r3, 0(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 822F1DC8: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 822F1DCC: 388BC4D8  addi r4, r11, -0x3b28
	ctx.r[4].s64 = ctx.r[11].s64 + -15144;
	// 822F1DD0: 48B1D399  bl 0x82e0f168
	ctx.lr = 0x822F1DD4;
	sub_82E0F168(ctx, base);
	// 822F1DD4: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 822F1DD8: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 822F1DDC: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 822F1DE0: 7F0A2040  cmplw cr6, r10, r4
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[4].u32, &mut ctx.xer);
	// 822F1DE4: 419A0010  beq cr6, 0x822f1df4
	if ctx.cr[6].eq {
	pc = 0x822F1DF4; continue 'dispatch;
	}
	// 822F1DE8: 908B0004  stw r4, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 822F1DEC: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 822F1DF0: 488E6AE1  bl 0x82bd88d0
	ctx.lr = 0x822F1DF4;
	sub_82BD88D0(ctx, base);
	// 822F1DF4: 807E002C  lwz r3, 0x2c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(44 as u32) ) } as u64;
	// 822F1DF8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 822F1DFC: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 822F1E00: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 822F1E04: 4E800421  bctrl
	ctx.lr = 0x822F1E08;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 822F1E08: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 822F1E0C: 807B0004  lwz r3, 4(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 822F1E10: 48B1D6E9  bl 0x82e0f4f8
	ctx.lr = 0x822F1E14;
	sub_82E0F4F8(ctx, base);
	// 822F1E14: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 822F1E18: 807B0004  lwz r3, 4(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 822F1E1C: 4BFD25ED  bl 0x822c4408
	ctx.lr = 0x822F1E20;
	sub_822C4408(ctx, base);
	// 822F1E20: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 822F1E24: 3B8B17EC  addi r28, r11, 0x17ec
	ctx.r[28].s64 = ctx.r[11].s64 + 6124;
	// 822F1E28: 817D17FC  lwz r11, 0x17fc(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(6140 as u32) ) } as u64;
	// 822F1E2C: 556A0739  rlwinm. r10, r11, 0, 0x1c, 0x1c
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 822F1E30: 4082001C  bne 0x822f1e4c
	if !ctx.cr[0].eq {
	pc = 0x822F1E4C; continue 'dispatch;
	}
	// 822F1E34: 616B0008  ori r11, r11, 8
	ctx.r[11].u64 = ctx.r[11].u64 | 8;
	// 822F1E38: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 822F1E3C: 917D17FC  stw r11, 0x17fc(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(6140 as u32), ctx.r[11].u32 ) };
	// 822F1E40: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 822F1E44: 388AAC50  addi r4, r10, -0x53b0
	ctx.r[4].s64 = ctx.r[10].s64 + -21424;
	// 822F1E48: 48B076E9  bl 0x82df9530
	ctx.lr = 0x822F1E4C;
	sub_82DF9530(ctx, base);
	// 822F1E4C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 822F1E50: 4BFF36F9  bl 0x822e5548
	ctx.lr = 0x822F1E54;
	sub_822E5548(ctx, base);
	// 822F1E54: 397E003C  addi r11, r30, 0x3c
	ctx.r[11].s64 = ctx.r[30].s64 + 60;
	// 822F1E58: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 822F1E5C: 388B0004  addi r4, r11, 4
	ctx.r[4].s64 = ctx.r[11].s64 + 4;
	// 822F1E60: 817E003C  lwz r11, 0x3c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(60 as u32) ) } as u64;
	// 822F1E64: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 822F1E68: 4BFD25F9  bl 0x822c4460
	ctx.lr = 0x822F1E6C;
	sub_822C4460(ctx, base);
	// 822F1E6C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 822F1E70: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 822F1E74: 807E001C  lwz r3, 0x1c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 822F1E78: 91610070  stw r11, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[11].u32 ) };
	// 822F1E7C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 822F1E80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 822F1E84: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 822F1E88: 91610078  stw r11, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[11].u32 ) };
	// 822F1E8C: 3BFE001C  addi r31, r30, 0x1c
	ctx.r[31].s64 = ctx.r[30].s64 + 28;
	// 822F1E90: 91410068  stw r10, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[10].u32 ) };
	// 822F1E94: 9141006C  stw r10, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[10].u32 ) };
	// 822F1E98: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 822F1E9C: 9161007C  stw r11, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[11].u32 ) };
	// 822F1EA0: 48B1F559  bl 0x82e113f8
	ctx.lr = 0x822F1EA4;
	sub_82E113F8(ctx, base);
	// 822F1EA4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 822F1EA8: 4BFF3651  bl 0x822e54f8
	ctx.lr = 0x822F1EAC;
	sub_822E54F8(ctx, base);
	// 822F1EAC: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 822F1EB0: 3B8B17E8  addi r28, r11, 0x17e8
	ctx.r[28].s64 = ctx.r[11].s64 + 6120;
	// 822F1EB4: 817D17FC  lwz r11, 0x17fc(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(6140 as u32) ) } as u64;
	// 822F1EB8: 556A06F7  rlwinm. r10, r11, 0, 0x1b, 0x1b
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 822F1EBC: 4082001C  bne 0x822f1ed8
	if !ctx.cr[0].eq {
	pc = 0x822F1ED8; continue 'dispatch;
	}
	// 822F1EC0: 616B0010  ori r11, r11, 0x10
	ctx.r[11].u64 = ctx.r[11].u64 | 16;
	// 822F1EC4: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 822F1EC8: 917D17FC  stw r11, 0x17fc(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(6140 as u32), ctx.r[11].u32 ) };
	// 822F1ECC: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 822F1ED0: 388AC4D0  addi r4, r10, -0x3b30
	ctx.r[4].s64 = ctx.r[10].s64 + -15152;
	// 822F1ED4: 48B0765D  bl 0x82df9530
	ctx.lr = 0x822F1ED8;
	sub_82DF9530(ctx, base);
	// 822F1ED8: 38BE0050  addi r5, r30, 0x50
	ctx.r[5].s64 = ctx.r[30].s64 + 80;
	// 822F1EDC: 809C0000  lwz r4, 0(r28)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 822F1EE0: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 822F1EE4: 48B1FD3D  bl 0x82e11c20
	ctx.lr = 0x822F1EE8;
	sub_82E11C20(ctx, base);
	// 822F1EE8: 389E0014  addi r4, r30, 0x14
	ctx.r[4].s64 = ctx.r[30].s64 + 20;
	// 822F1EEC: 83BB0000  lwz r29, 0(r27)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 822F1EF0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 822F1EF4: 48B1CC35  bl 0x82e0eb28
	ctx.lr = 0x822F1EF8;
	sub_82E0EB28(ctx, base);
	// 822F1EF8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 822F1EFC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 822F1F00: 48B1CD21  bl 0x82e0ec20
	ctx.lr = 0x822F1F04;
	sub_82E0EC20(ctx, base);
	// 822F1F04: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 822F1F08: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 822F1F0C: 80BE0008  lwz r5, 8(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 822F1F10: 38E00020  li r7, 0x20
	ctx.r[7].s64 = 32;
	// 822F1F14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 822F1F18: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 822F1F1C: 83EB0000  lwz r31, 0(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 822F1F20: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822F1F24: 488E37C5  bl 0x82bd56e8
	ctx.lr = 0x822F1F28;
	sub_82BD56E8(ctx, base);
	// 822F1F28: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822F1F2C: 809E000C  lwz r4, 0xc(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 822F1F30: 488E38D9  bl 0x82bd5808
	ctx.lr = 0x822F1F34;
	sub_82BD5808(ctx, base);
	// 822F1F34: 38800038  li r4, 0x38
	ctx.r[4].s64 = 56;
	// 822F1F38: 83BB0000  lwz r29, 0(r27)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 822F1F3C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 822F1F40: 897D0026  lbz r11, 0x26(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(38 as u32) ) } as u64;
	// 822F1F44: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 822F1F48: 4082000C  bne 0x822f1f54
	if !ctx.cr[0].eq {
	pc = 0x822F1F54; continue 'dispatch;
	}
	// 822F1F4C: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 822F1F50: 48000028  b 0x822f1f78
	pc = 0x822F1F78; continue 'dispatch;
	// 822F1F54: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 822F1F58: 4BFF3391  bl 0x822e52e8
	ctx.lr = 0x822F1F5C;
	sub_822E52E8(ctx, base);
	// 822F1F5C: 388000D0  li r4, 0xd0
	ctx.r[4].s64 = 208;
	// 822F1F60: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 822F1F64: 80BD0030  lwz r5, 0x30(r29)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(48 as u32) ) } as u64;
	// 822F1F68: 4BFF3381  bl 0x822e52e8
	ctx.lr = 0x822F1F6C;
	sub_822E52E8(ctx, base);
	// 822F1F6C: 80BD0034  lwz r5, 0x34(r29)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(52 as u32) ) } as u64;
	// 822F1F70: 388000CC  li r4, 0xcc
	ctx.r[4].s64 = 204;
	// 822F1F74: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 822F1F78: 4BFF3371  bl 0x822e52e8
	ctx.lr = 0x822F1F7C;
	sub_822E52E8(ctx, base);
	// 822F1F7C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 822F1F80: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 822F1F84: 807B0000  lwz r3, 0(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 822F1F88: C04B08A4  lfs f2, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 822F1F8C: C02AAC40  lfs f1, -0x53c0(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-21440 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 822F1F90: 4BFF3239  bl 0x822e51c8
	ctx.lr = 0x822F1F94;
	sub_822E51C8(ctx, base);
	// 822F1F94: 815E0038  lwz r10, 0x38(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(56 as u32) ) } as u64;
	// 822F1F98: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 822F1F9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 822F1FA0: 7D6A5B96  divwu r11, r10, r11
	ctx.r[11].u32 = ctx.r[10].u32 / ctx.r[11].u32;
	// 822F1FA4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 822F1FA8: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 822F1FAC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822F1FB0: 1CEB0003  mulli r7, r11, 3
	ctx.r[7].s64 = ctx.r[11].s64 * 3;
	// 822F1FB4: 488EC32D  bl 0x82bde2e0
	ctx.lr = 0x822F1FB8;
	sub_82BDE2E0(ctx, base);
	// 822F1FB8: 388000D0  li r4, 0xd0
	ctx.r[4].s64 = 208;
	// 822F1FBC: 83DB0000  lwz r30, 0(r27)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 822F1FC0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 822F1FC4: 48B1C8CD  bl 0x82e0e890
	ctx.lr = 0x822F1FC8;
	sub_82E0E890(ctx, base);
	// 822F1FC8: 388000CC  li r4, 0xcc
	ctx.r[4].s64 = 204;
	// 822F1FCC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 822F1FD0: 48B1C8C1  bl 0x82e0e890
	ctx.lr = 0x822F1FD4;
	sub_82E0E890(ctx, base);
	// 822F1FD4: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 822F1FD8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 822F1FDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 822F1FE0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 822F1FE4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 822F1FE8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822F1FEC: 488E36FD  bl 0x82bd56e8
	ctx.lr = 0x822F1FF0;
	sub_82BD56E8(ctx, base);
	// 822F1FF0: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 822F1FF4: 48EB61B8  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822F1FF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822F1FF8 size=948
    let mut pc: u32 = 0x822F1FF8;
    'dispatch: loop {
        match pc {
            0x822F1FF8 => {
    //   block [0x822F1FF8..0x822F23AC)
	// 822F1FF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822F1FFC: 48EB6155  bl 0x831a8150
	ctx.lr = 0x822F2000;
	sub_831A8130(ctx, base);
	// 822F2000: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822F2004: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 822F2008: 83060000  lwz r24, 0(r6)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 822F200C: 82E60004  lwz r23, 4(r6)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(4 as u32) ) } as u64;
	// 822F2010: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 822F2014: 82C60008  lwz r22, 8(r6)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(8 as u32) ) } as u64;
	// 822F2018: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 822F201C: 3B20000C  li r25, 0xc
	ctx.r[25].s64 = 12;
	// 822F2020: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 822F2024: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 822F2028: 409A000C  bne cr6, 0x822f2034
	if !ctx.cr[6].eq {
	pc = 0x822F2034; continue 'dispatch;
	}
	// 822F202C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 822F2030: 48000010  b 0x822f2040
	pc = 0x822F2040; continue 'dispatch;
	// 822F2034: 815E000C  lwz r10, 0xc(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 822F2038: 7D4B5050  subf r10, r11, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 822F203C: 7D0ACBD6  divw r8, r10, r25
	ctx.r[8].s32 = ctx.r[10].s32 / ctx.r[25].s32;
	// 822F2040: 2B1A0000  cmplwi cr6, r26, 0
	ctx.cr[6].compare_u32(ctx.r[26].u32, 0 as u32, &mut ctx.xer);
	// 822F2044: 419A0360  beq cr6, 0x822f23a4
	if ctx.cr[6].eq {
	pc = 0x822F23A4; continue 'dispatch;
	}
	// 822F2048: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 822F204C: 409A000C  bne cr6, 0x822f2058
	if !ctx.cr[6].eq {
	pc = 0x822F2058; continue 'dispatch;
	}
	// 822F2050: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 822F2054: 48000010  b 0x822f2064
	pc = 0x822F2064; continue 'dispatch;
	// 822F2058: 815E0008  lwz r10, 8(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 822F205C: 7D4B5050  subf r10, r11, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 822F2060: 7D4ACBD6  divw r10, r10, r25
	ctx.r[10].s32 = ctx.r[10].s32 / ctx.r[25].s32;
	// 822F2064: 3D201555  lis r9, 0x1555
	ctx.r[9].s64 = 357892096;
	// 822F2068: 61295555  ori r9, r9, 0x5555
	ctx.r[9].u64 = ctx.r[9].u64 | 21845;
	// 822F206C: 7D4A4850  subf r10, r10, r9
	ctx.r[10].s64 = ctx.r[9].s64 - ctx.r[10].s64;
	// 822F2070: 7F0AD040  cmplw cr6, r10, r26
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[26].u32, &mut ctx.xer);
	// 822F2074: 4098000C  bge cr6, 0x822f2080
	if !ctx.cr[6].lt {
	pc = 0x822F2080; continue 'dispatch;
	}
	// 822F2078: 488C5CA9  bl 0x82bb7d20
	ctx.lr = 0x822F207C;
	sub_82BB7D20(ctx, base);
	// 822F207C: 48000328  b 0x822f23a4
	pc = 0x822F23A4; continue 'dispatch;
	// 822F2080: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 822F2084: 409A000C  bne cr6, 0x822f2090
	if !ctx.cr[6].eq {
	pc = 0x822F2090; continue 'dispatch;
	}
	// 822F2088: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 822F208C: 48000010  b 0x822f209c
	pc = 0x822F209C; continue 'dispatch;
	// 822F2090: 815E0008  lwz r10, 8(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 822F2094: 7D4B5050  subf r10, r11, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 822F2098: 7D4ACBD6  divw r10, r10, r25
	ctx.r[10].s32 = ctx.r[10].s32 / ctx.r[25].s32;
	// 822F209C: 7D4AD214  add r10, r10, r26
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[26].u64;
	// 822F20A0: 7F085040  cmplw cr6, r8, r10
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[10].u32, &mut ctx.xer);
	// 822F20A4: 40980190  bge cr6, 0x822f2234
	if !ctx.cr[6].lt {
	pc = 0x822F2234; continue 'dispatch;
	}
	// 822F20A8: 550AF87E  srwi r10, r8, 1
	ctx.r[10].u32 = ctx.r[8].u32.wrapping_shr(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 822F20AC: 7D2A4850  subf r9, r10, r9
	ctx.r[9].s64 = ctx.r[9].s64 - ctx.r[10].s64;
	// 822F20B0: 7F094040  cmplw cr6, r9, r8
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[8].u32, &mut ctx.xer);
	// 822F20B4: 4098000C  bge cr6, 0x822f20c0
	if !ctx.cr[6].lt {
	pc = 0x822F20C0; continue 'dispatch;
	}
	// 822F20B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 822F20BC: 48000008  b 0x822f20c4
	pc = 0x822F20C4; continue 'dispatch;
	// 822F20C0: 7D4A4214  add r10, r10, r8
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[8].u64;
	// 822F20C4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 822F20C8: 409A000C  bne cr6, 0x822f20d4
	if !ctx.cr[6].eq {
	pc = 0x822F20D4; continue 'dispatch;
	}
	// 822F20CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 822F20D0: 48000010  b 0x822f20e0
	pc = 0x822F20E0; continue 'dispatch;
	// 822F20D4: 813E0008  lwz r9, 8(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 822F20D8: 7D2B4850  subf r9, r11, r9
	ctx.r[9].s64 = ctx.r[9].s64 - ctx.r[11].s64;
	// 822F20DC: 7D29CBD6  divw r9, r9, r25
	ctx.r[9].s32 = ctx.r[9].s32 / ctx.r[25].s32;
	// 822F20E0: 7D29D214  add r9, r9, r26
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[26].u64;
	// 822F20E4: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 822F20E8: 4098001C  bge cr6, 0x822f2104
	if !ctx.cr[6].lt {
	pc = 0x822F2104; continue 'dispatch;
	}
	// 822F20EC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 822F20F0: 419A0010  beq cr6, 0x822f2100
	if ctx.cr[6].eq {
	pc = 0x822F2100; continue 'dispatch;
	}
	// 822F20F4: 815E0008  lwz r10, 8(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 822F20F8: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 822F20FC: 7D6BCBD6  divw r11, r11, r25
	ctx.r[11].s32 = ctx.r[11].s32 / ctx.r[25].s32;
	// 822F2100: 7D4BD214  add r10, r11, r26
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[26].u64;
	// 822F2104: 3F808335  lis r28, -0x7ccb
	ctx.r[28].s64 = -2093678592;
	// 822F2108: 1F6A000C  mulli r27, r10, 0xc
	ctx.r[27].s64 = ctx.r[10].s64 * 12;
	// 822F210C: 807C110C  lwz r3, 0x110c(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4364 as u32) ) } as u64;
	// 822F2110: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 822F2114: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 822F2118: 388B08B0  addi r4, r11, 0x8b0
	ctx.r[4].s64 = ctx.r[11].s64 + 2224;
	// 822F211C: 38A0002D  li r5, 0x2d
	ctx.r[5].s64 = 45;
	// 822F2120: 48AFFFA9  bl 0x82df20c8
	ctx.lr = 0x822F2124;
	sub_82DF20C8(ctx, base);
	// 822F2124: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 822F2128: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 822F212C: 7FAAEB78  mr r10, r29
	ctx.r[10].u64 = ctx.r[29].u64;
	// 822F2130: 4800002C  b 0x822f215c
	pc = 0x822F215C; continue 'dispatch;
	// 822F2134: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 822F2138: 419A001C  beq cr6, 0x822f2154
	if ctx.cr[6].eq {
	pc = 0x822F2154; continue 'dispatch;
	}
	// 822F213C: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 822F2140: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 822F2144: 812B0004  lwz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 822F2148: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 822F214C: 812B0008  lwz r9, 8(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 822F2150: 912A0008  stw r9, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 822F2154: 396B000C  addi r11, r11, 0xc
	ctx.r[11].s64 = ctx.r[11].s64 + 12;
	// 822F2158: 394A000C  addi r10, r10, 0xc
	ctx.r[10].s64 = ctx.r[10].s64 + 12;
	// 822F215C: 7F0BF840  cmplw cr6, r11, r31
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[31].u32, &mut ctx.xer);
	// 822F2160: 409AFFD4  bne cr6, 0x822f2134
	if !ctx.cr[6].eq {
	pc = 0x822F2134; continue 'dispatch;
	}
	// 822F2164: 7F49D378  mr r9, r26
	ctx.r[9].u64 = ctx.r[26].u64;
	// 822F2168: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 822F216C: 2B1A0000  cmplwi cr6, r26, 0
	ctx.cr[6].compare_u32(ctx.r[26].u32, 0 as u32, &mut ctx.xer);
	// 822F2170: 419A0024  beq cr6, 0x822f2194
	if ctx.cr[6].eq {
	pc = 0x822F2194; continue 'dispatch;
	}
	// 822F2174: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 822F2178: 419A0010  beq cr6, 0x822f2188
	if ctx.cr[6].eq {
	pc = 0x822F2188; continue 'dispatch;
	}
	// 822F217C: 930B0000  stw r24, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[24].u32 ) };
	// 822F2180: 92EB0004  stw r23, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[23].u32 ) };
	// 822F2184: 92CB0008  stw r22, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[22].u32 ) };
	// 822F2188: 3529FFFF  addic. r9, r9, -1
	ctx.xer.ca = (ctx.r[9].u32 > (!(-1 as u32)));
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 822F218C: 396B000C  addi r11, r11, 0xc
	ctx.r[11].s64 = ctx.r[11].s64 + 12;
	// 822F2190: 4082FFE4  bne 0x822f2174
	if !ctx.cr[0].eq {
	pc = 0x822F2174; continue 'dispatch;
	}
	// 822F2194: 811E0008  lwz r8, 8(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 822F2198: 1D3A000C  mulli r9, r26, 0xc
	ctx.r[9].s64 = ctx.r[26].s64 * 12;
	// 822F219C: 7D695214  add r11, r9, r10
	ctx.r[11].u64 = ctx.r[9].u64 + ctx.r[10].u64;
	// 822F21A0: 7F1F4040  cmplw cr6, r31, r8
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[8].u32, &mut ctx.xer);
	// 822F21A4: 419A0040  beq cr6, 0x822f21e4
	if ctx.cr[6].eq {
	pc = 0x822F21E4; continue 'dispatch;
	}
	// 822F21A8: 7D295850  subf r9, r9, r11
	ctx.r[9].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 822F21AC: 7D4A4850  subf r10, r10, r9
	ctx.r[10].s64 = ctx.r[9].s64 - ctx.r[10].s64;
	// 822F21B0: 7D4AFA14  add r10, r10, r31
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[31].u64;
	// 822F21B4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 822F21B8: 419A001C  beq cr6, 0x822f21d4
	if ctx.cr[6].eq {
	pc = 0x822F21D4; continue 'dispatch;
	}
	// 822F21BC: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 822F21C0: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 822F21C4: 812A0004  lwz r9, 4(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 822F21C8: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 822F21CC: 812A0008  lwz r9, 8(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 822F21D0: 912B0008  stw r9, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 822F21D4: 394A000C  addi r10, r10, 0xc
	ctx.r[10].s64 = ctx.r[10].s64 + 12;
	// 822F21D8: 396B000C  addi r11, r11, 0xc
	ctx.r[11].s64 = ctx.r[11].s64 + 12;
	// 822F21DC: 7F0A4040  cmplw cr6, r10, r8
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[8].u32, &mut ctx.xer);
	// 822F21E0: 409AFFD4  bne cr6, 0x822f21b4
	if !ctx.cr[6].eq {
	pc = 0x822F21B4; continue 'dispatch;
	}
	// 822F21E4: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 822F21E8: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 822F21EC: 409A000C  bne cr6, 0x822f21f8
	if !ctx.cr[6].eq {
	pc = 0x822F21F8; continue 'dispatch;
	}
	// 822F21F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 822F21F4: 48000010  b 0x822f2204
	pc = 0x822F2204; continue 'dispatch;
	// 822F21F8: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 822F21FC: 7D645850  subf r11, r4, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[4].s64;
	// 822F2200: 7D6BCBD6  divw r11, r11, r25
	ctx.r[11].s32 = ctx.r[11].s32 / ctx.r[25].s32;
	// 822F2204: 7FEBD214  add r31, r11, r26
	ctx.r[31].u64 = ctx.r[11].u64 + ctx.r[26].u64;
	// 822F2208: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 822F220C: 419A000C  beq cr6, 0x822f2218
	if ctx.cr[6].eq {
	pc = 0x822F2218; continue 'dispatch;
	}
	// 822F2210: 807C110C  lwz r3, 0x110c(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4364 as u32) ) } as u64;
	// 822F2214: 48AFFF75  bl 0x82df2188
	ctx.lr = 0x822F2218;
	sub_82DF2188(ctx, base);
	// 822F2218: 1D7F000C  mulli r11, r31, 0xc
	ctx.r[11].s64 = ctx.r[31].s64 * 12;
	// 822F221C: 93BE0004  stw r29, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 822F2220: 7D5BEA14  add r10, r27, r29
	ctx.r[10].u64 = ctx.r[27].u64 + ctx.r[29].u64;
	// 822F2224: 7D6BEA14  add r11, r11, r29
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 822F2228: 915E000C  stw r10, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 822F222C: 917E0008  stw r11, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 822F2230: 48000174  b 0x822f23a4
	pc = 0x822F23A4; continue 'dispatch;
	// 822F2234: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 822F2238: 7D5F5850  subf r10, r31, r11
	ctx.r[10].s64 = ctx.r[11].s64 - ctx.r[31].s64;
	// 822F223C: 7D4ACBD6  divw r10, r10, r25
	ctx.r[10].s32 = ctx.r[10].s32 / ctx.r[25].s32;
	// 822F2240: 7F0AD040  cmplw cr6, r10, r26
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[26].u32, &mut ctx.xer);
	// 822F2244: 409800B4  bge cr6, 0x822f22f8
	if !ctx.cr[6].lt {
	pc = 0x822F22F8; continue 'dispatch;
	}
	// 822F2248: 1D1A000C  mulli r8, r26, 0xc
	ctx.r[8].s64 = ctx.r[26].s64 * 12;
	// 822F224C: 7D48FA14  add r10, r8, r31
	ctx.r[10].u64 = ctx.r[8].u64 + ctx.r[31].u64;
	// 822F2250: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 822F2254: 419A0038  beq cr6, 0x822f228c
	if ctx.cr[6].eq {
	pc = 0x822F228C; continue 'dispatch;
	}
	// 822F2258: 7D285050  subf r9, r8, r10
	ctx.r[9].s64 = ctx.r[10].s64 - ctx.r[8].s64;
	// 822F225C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 822F2260: 419A001C  beq cr6, 0x822f227c
	if ctx.cr[6].eq {
	pc = 0x822F227C; continue 'dispatch;
	}
	// 822F2264: 80E90000  lwz r7, 0(r9)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 822F2268: 90EA0000  stw r7, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 822F226C: 80E90004  lwz r7, 4(r9)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 822F2270: 90EA0004  stw r7, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 822F2274: 80E90008  lwz r7, 8(r9)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) } as u64;
	// 822F2278: 90EA0008  stw r7, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[7].u32 ) };
	// 822F227C: 3929000C  addi r9, r9, 0xc
	ctx.r[9].s64 = ctx.r[9].s64 + 12;
	// 822F2280: 394A000C  addi r10, r10, 0xc
	ctx.r[10].s64 = ctx.r[10].s64 + 12;
	// 822F2284: 7F095840  cmplw cr6, r9, r11
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[11].u32, &mut ctx.xer);
	// 822F2288: 409AFFD4  bne cr6, 0x822f225c
	if !ctx.cr[6].eq {
	pc = 0x822F225C; continue 'dispatch;
	}
	// 822F228C: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 822F2290: 7D5F5850  subf r10, r31, r11
	ctx.r[10].s64 = ctx.r[11].s64 - ctx.r[31].s64;
	// 822F2294: 7D4ACBD6  divw r10, r10, r25
	ctx.r[10].s32 = ctx.r[10].s32 / ctx.r[25].s32;
	// 822F2298: 7D4AD051  subf. r10, r10, r26
	ctx.r[10].s64 = ctx.r[26].s64 - ctx.r[10].s64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 822F229C: 41820024  beq 0x822f22c0
	if ctx.cr[0].eq {
	pc = 0x822F22C0; continue 'dispatch;
	}
	// 822F22A0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 822F22A4: 419A0010  beq cr6, 0x822f22b4
	if ctx.cr[6].eq {
	pc = 0x822F22B4; continue 'dispatch;
	}
	// 822F22A8: 930B0000  stw r24, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[24].u32 ) };
	// 822F22AC: 92EB0004  stw r23, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[23].u32 ) };
	// 822F22B0: 92CB0008  stw r22, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[22].u32 ) };
	// 822F22B4: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 822F22B8: 396B000C  addi r11, r11, 0xc
	ctx.r[11].s64 = ctx.r[11].s64 + 12;
	// 822F22BC: 4082FFE4  bne 0x822f22a0
	if !ctx.cr[0].eq {
	pc = 0x822F22A0; continue 'dispatch;
	}
	// 822F22C0: 815E0008  lwz r10, 8(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 822F22C4: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 822F22C8: 7D485214  add r10, r8, r10
	ctx.r[10].u64 = ctx.r[8].u64 + ctx.r[10].u64;
	// 822F22CC: 7D285050  subf r9, r8, r10
	ctx.r[9].s64 = ctx.r[10].s64 - ctx.r[8].s64;
	// 822F22D0: 915E0008  stw r10, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 822F22D4: 7F1F4840  cmplw cr6, r31, r9
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[9].u32, &mut ctx.xer);
	// 822F22D8: 419A00CC  beq cr6, 0x822f23a4
	if ctx.cr[6].eq {
	pc = 0x822F23A4; continue 'dispatch;
	}
	// 822F22DC: 930B0000  stw r24, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[24].u32 ) };
	// 822F22E0: 92EB0004  stw r23, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[23].u32 ) };
	// 822F22E4: 92CB0008  stw r22, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[22].u32 ) };
	// 822F22E8: 396B000C  addi r11, r11, 0xc
	ctx.r[11].s64 = ctx.r[11].s64 + 12;
	// 822F22EC: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 822F22F0: 409AFFEC  bne cr6, 0x822f22dc
	if !ctx.cr[6].eq {
	pc = 0x822F22DC; continue 'dispatch;
	}
	// 822F22F4: 480000B0  b 0x822f23a4
	pc = 0x822F23A4; continue 'dispatch;
	// 822F22F8: 1CFA000C  mulli r7, r26, 0xc
	ctx.r[7].s64 = ctx.r[26].s64 * 12;
	// 822F22FC: 7D275850  subf r9, r7, r11
	ctx.r[9].s64 = ctx.r[11].s64 - ctx.r[7].s64;
	// 822F2300: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 822F2304: 7D284B78  mr r8, r9
	ctx.r[8].u64 = ctx.r[9].u64;
	// 822F2308: 7F095840  cmplw cr6, r9, r11
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[11].u32, &mut ctx.xer);
	// 822F230C: 419A0034  beq cr6, 0x822f2340
	if ctx.cr[6].eq {
	pc = 0x822F2340; continue 'dispatch;
	}
	// 822F2310: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 822F2314: 419A001C  beq cr6, 0x822f2330
	if ctx.cr[6].eq {
	pc = 0x822F2330; continue 'dispatch;
	}
	// 822F2318: 80C80000  lwz r6, 0(r8)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 822F231C: 90CA0000  stw r6, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[6].u32 ) };
	// 822F2320: 80C80004  lwz r6, 4(r8)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 822F2324: 90CA0004  stw r6, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[6].u32 ) };
	// 822F2328: 80C80008  lwz r6, 8(r8)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(8 as u32) ) } as u64;
	// 822F232C: 90CA0008  stw r6, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[6].u32 ) };
	// 822F2330: 3908000C  addi r8, r8, 0xc
	ctx.r[8].s64 = ctx.r[8].s64 + 12;
	// 822F2334: 394A000C  addi r10, r10, 0xc
	ctx.r[10].s64 = ctx.r[10].s64 + 12;
	// 822F2338: 7F085840  cmplw cr6, r8, r11
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[11].u32, &mut ctx.xer);
	// 822F233C: 409AFFD4  bne cr6, 0x822f2310
	if !ctx.cr[6].eq {
	pc = 0x822F2310; continue 'dispatch;
	}
	// 822F2340: 915E0008  stw r10, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 822F2344: 7D2B4B78  mr r11, r9
	ctx.r[11].u64 = ctx.r[9].u64;
	// 822F2348: 7F1F4840  cmplw cr6, r31, r9
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[9].u32, &mut ctx.xer);
	// 822F234C: 419A0030  beq cr6, 0x822f237c
	if ctx.cr[6].eq {
	pc = 0x822F237C; continue 'dispatch;
	}
	// 822F2350: 7D475A14  add r10, r7, r11
	ctx.r[10].u64 = ctx.r[7].u64 + ctx.r[11].u64;
	// 822F2354: 396BFFF4  addi r11, r11, -0xc
	ctx.r[11].s64 = ctx.r[11].s64 + -12;
	// 822F2358: 394AFFF4  addi r10, r10, -0xc
	ctx.r[10].s64 = ctx.r[10].s64 + -12;
	// 822F235C: 7F0BF840  cmplw cr6, r11, r31
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[31].u32, &mut ctx.xer);
	// 822F2360: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 822F2364: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 822F2368: 812B0004  lwz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 822F236C: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 822F2370: 812B0008  lwz r9, 8(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 822F2374: 912A0008  stw r9, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 822F2378: 409AFFDC  bne cr6, 0x822f2354
	if !ctx.cr[6].eq {
	pc = 0x822F2354; continue 'dispatch;
	}
	// 822F237C: 7D47FA14  add r10, r7, r31
	ctx.r[10].u64 = ctx.r[7].u64 + ctx.r[31].u64;
	// 822F2380: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 822F2384: 7F1F5040  cmplw cr6, r31, r10
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[10].u32, &mut ctx.xer);
	// 822F2388: 419A001C  beq cr6, 0x822f23a4
	if ctx.cr[6].eq {
	pc = 0x822F23A4; continue 'dispatch;
	}
	// 822F238C: 930B0000  stw r24, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[24].u32 ) };
	// 822F2390: 92EB0004  stw r23, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[23].u32 ) };
	// 822F2394: 92CB0008  stw r22, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[22].u32 ) };
	// 822F2398: 396B000C  addi r11, r11, 0xc
	ctx.r[11].s64 = ctx.r[11].s64 + 12;
	// 822F239C: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 822F23A0: 409AFFEC  bne cr6, 0x822f238c
	if !ctx.cr[6].eq {
	pc = 0x822F238C; continue 'dispatch;
	}
	// 822F23A4: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 822F23A8: 48EB5DF8  b 0x831a81a0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822F23B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822F23B0 size=232
    let mut pc: u32 = 0x822F23B0;
    'dispatch: loop {
        match pc {
            0x822F23B0 => {
    //   block [0x822F23B0..0x822F2498)
	// 822F23B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822F23B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822F23B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822F23BC: 81430004  lwz r10, 4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 822F23C0: 7C882378  mr r8, r4
	ctx.r[8].u64 = ctx.r[4].u64;
	// 822F23C4: F8A10080  std r5, 0x80(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[5].u64 ) };
	// 822F23C8: 3920000C  li r9, 0xc
	ctx.r[9].s64 = 12;
	// 822F23CC: F8C10088  std r6, 0x88(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(136 as u32), ctx.r[6].u64 ) };
	// 822F23D0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 822F23D4: 409A000C  bne cr6, 0x822f23e0
	if !ctx.cr[6].eq {
	pc = 0x822F23E0; continue 'dispatch;
	}
	// 822F23D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 822F23DC: 48000010  b 0x822f23ec
	pc = 0x822F23EC; continue 'dispatch;
	// 822F23E0: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 822F23E4: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 822F23E8: 7D6B4BD6  divw r11, r11, r9
	ctx.r[11].s32 = ctx.r[11].s32 / ctx.r[9].s32;
	// 822F23EC: 7F0B4040  cmplw cr6, r11, r8
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[8].u32, &mut ctx.xer);
	// 822F23F0: 40980034  bge cr6, 0x822f2424
	if !ctx.cr[6].lt {
	pc = 0x822F2424; continue 'dispatch;
	}
	// 822F23F4: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 822F23F8: 409A000C  bne cr6, 0x822f2404
	if !ctx.cr[6].eq {
	pc = 0x822F2404; continue 'dispatch;
	}
	// 822F23FC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 822F2400: 48000010  b 0x822f2410
	pc = 0x822F2410; continue 'dispatch;
	// 822F2404: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 822F2408: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 822F240C: 7D6B4BD6  divw r11, r11, r9
	ctx.r[11].s32 = ctx.r[11].s32 / ctx.r[9].s32;
	// 822F2410: 38C10080  addi r6, r1, 0x80
	ctx.r[6].s64 = ctx.r[1].s64 + 128;
	// 822F2414: 80830008  lwz r4, 8(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 822F2418: 7CAB4050  subf r5, r11, r8
	ctx.r[5].s64 = ctx.r[8].s64 - ctx.r[11].s64;
	// 822F241C: 4BFFFBDD  bl 0x822f1ff8
	ctx.lr = 0x822F2420;
	sub_822F1FF8(ctx, base);
	// 822F2420: 48000068  b 0x822f2488
	pc = 0x822F2488; continue 'dispatch;
	// 822F2424: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 822F2428: 419A0060  beq cr6, 0x822f2488
	if ctx.cr[6].eq {
	pc = 0x822F2488; continue 'dispatch;
	}
	// 822F242C: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 822F2430: 7CEA5850  subf r7, r10, r11
	ctx.r[7].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 822F2434: 7D274BD6  divw r9, r7, r9
	ctx.r[9].s32 = ctx.r[7].s32 / ctx.r[9].s32;
	// 822F2438: 7F084840  cmplw cr6, r8, r9
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[9].u32, &mut ctx.xer);
	// 822F243C: 4098004C  bge cr6, 0x822f2488
	if !ctx.cr[6].lt {
	pc = 0x822F2488; continue 'dispatch;
	}
	// 822F2440: 1D28000C  mulli r9, r8, 0xc
	ctx.r[9].s64 = ctx.r[8].s64 * 12;
	// 822F2444: 7D295214  add r9, r9, r10
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[10].u64;
	// 822F2448: 7F095840  cmplw cr6, r9, r11
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[11].u32, &mut ctx.xer);
	// 822F244C: 419A003C  beq cr6, 0x822f2488
	if ctx.cr[6].eq {
	pc = 0x822F2488; continue 'dispatch;
	}
	// 822F2450: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 822F2454: 7F0B5840  cmplw cr6, r11, r11
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[11].u32, &mut ctx.xer);
	// 822F2458: 419A002C  beq cr6, 0x822f2484
	if ctx.cr[6].eq {
	pc = 0x822F2484; continue 'dispatch;
	}
	// 822F245C: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 822F2460: 91090000  stw r8, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 822F2464: 810A0004  lwz r8, 4(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 822F2468: 91090004  stw r8, 4(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 822F246C: 810A0008  lwz r8, 8(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 822F2470: 394A000C  addi r10, r10, 0xc
	ctx.r[10].s64 = ctx.r[10].s64 + 12;
	// 822F2474: 91090008  stw r8, 8(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), ctx.r[8].u32 ) };
	// 822F2478: 3929000C  addi r9, r9, 0xc
	ctx.r[9].s64 = ctx.r[9].s64 + 12;
	// 822F247C: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 822F2480: 409AFFDC  bne cr6, 0x822f245c
	if !ctx.cr[6].eq {
	pc = 0x822F245C; continue 'dispatch;
	}
	// 822F2484: 91230008  stw r9, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 822F2488: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 822F248C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822F2490: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822F2494: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822F2498(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x822F2498 size=368
    let mut pc: u32 = 0x822F2498;
    'dispatch: loop {
        match pc {
            0x822F2498 => {
    //   block [0x822F2498..0x822F2608)
	// 822F2498: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822F249C: 48EB5CC1  bl 0x831a815c
	ctx.lr = 0x822F24A0;
	sub_831A8130(ctx, base);
	// 822F24A0: DBE1FFB8  stfd f31, -0x48(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-72 as u32), ctx.f[31].u64 ) };
	// 822F24A4: 9421FF00  stwu r1, -0x100(r1)
	ea = ctx.r[1].u32.wrapping_add(-256 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822F24A8: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 822F24AC: 39400030  li r10, 0x30
	ctx.r[10].s64 = 48;
	// 822F24B0: 3BE00020  li r31, 0x20
	ctx.r[31].s64 = 32;
	// 822F24B4: 39200040  li r9, 0x40
	ctx.r[9].s64 = 64;
	// 822F24B8: 7D194378  mr r25, r8
	ctx.r[25].u64 = ctx.r[8].u64;
	// 822F24BC: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 822F24C0: 39010070  addi r8, r1, 0x70
	ctx.r[8].s64 = ctx.r[1].s64 + 112;
	// 822F24C4: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 822F24C8: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 822F24CC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 822F24D0: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 822F24D4: 13CB50C7  vcmpequd (lvx128) v30, v11, v10
	tmp.u32 = ctx.r[11].u32 + ctx.r[10].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[62] using VectorMaskL[(tmp.u32 & 0xF)]
	// 822F24D8: 7CFA3B78  mr r26, r7
	ctx.r[26].u64 = ctx.r[7].u64;
	// 822F24DC: 13EBF8C7  vcmpequd (lvx128) v31, v11, v31
	tmp.u32 = ctx.r[11].u32 + ctx.r[31].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 822F24E0: 13AB48C7  vcmpequd (lvx128) v29, v11, v9
	tmp.u32 = ctx.r[11].u32 + ctx.r[9].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[61] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822F2608(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x822F2608 size=960
    let mut pc: u32 = 0x822F2608;
    'dispatch: loop {
        match pc {
            0x822F2608 => {
    //   block [0x822F2608..0x822F29C8)
	// 822F2608: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822F260C: 48EB5B41  bl 0x831a814c
	ctx.lr = 0x822F2610;
	sub_831A8130(ctx, base);
	// 822F2610: DBE1FF98  stfd f31, -0x68(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-104 as u32), ctx.f[31].u64 ) };
	// 822F2614: 9421FE40  stwu r1, -0x1c0(r1)
	ea = ctx.r[1].u32.wrapping_add(-448 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822F2618: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 822F261C: 81460000  lwz r10, 0(r6)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 822F2620: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 822F2624: 39200020  li r9, 0x20
	ctx.r[9].s64 = 32;
	// 822F2628: 7CB72B78  mr r23, r5
	ctx.r[23].u64 = ctx.r[5].u64;
	// 822F262C: 938100A4  stw r28, 0xa4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(164 as u32), ctx.r[28].u32 ) };
	// 822F2630: 938100A8  stw r28, 0xa8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(168 as u32), ctx.r[28].u32 ) };
	// 822F2634: 7D1A4378  mr r26, r8
	ctx.r[26].u64 = ctx.r[8].u64;
	// 822F2638: FB8B0000  std r28, 0(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[28].u64 ) };
	// 822F263C: 938B0008  stw r28, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[28].u32 ) };
	// 822F2640: 39010070  addi r8, r1, 0x70
	ctx.r[8].s64 = ctx.r[1].s64 + 112;
	// 822F2644: 938100AC  stw r28, 0xac(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(172 as u32), ctx.r[28].u32 ) };
	// 822F2648: 7C761B78  mr r22, r3
	ctx.r[22].u64 = ctx.r[3].u64;
	// 822F264C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 822F2650: 38800008  li r4, 8
	ctx.r[4].s64 = 8;
	// 822F2654: 386100A0  addi r3, r1, 0xa0
	ctx.r[3].s64 = ctx.r[1].s64 + 160;
	// 822F2658: 7CF53B78  mr r21, r7
	ctx.r[21].u64 = ctx.r[7].u64;
	// 822F265C: 13EA48C7  vcmpequd (lvx128) v31, v10, v9
	tmp.u32 = ctx.r[10].u32 + ctx.r[9].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 822F2660: E8A10060  ld r5, 0x60(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) };
	// 822F2664: 81610068  lwz r11, 0x68(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 822F2668: 796607E6  rldicr r6, r11, 0x20, 0x3f
	ctx.r[6].u64 = (ctx.r[11].u64).rotate_left(32) & 0xFFFFFFFFFFFFFFFF;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822F29C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x822F29C8 size=1256
    let mut pc: u32 = 0x822F29C8;
    'dispatch: loop {
        match pc {
            0x822F29C8 => {
    //   block [0x822F29C8..0x822F2EB0)
	// 822F29C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822F29CC: 48EB5765  bl 0x831a8130
	ctx.lr = 0x822F29D0;
	sub_831A8130(ctx, base);
	// 822F29D0: 3981FF68  addi r12, r1, -0x98
	ctx.r[12].s64 = ctx.r[1].s64 + -152;
	// 822F29D4: 48EB60A5  bl 0x831a8a78
	ctx.lr = 0x822F29D8;
	sub_831A8A40(ctx, base);
	// 822F29D8: 9421FE20  stwu r1, -0x1e0(r1)
	ea = ctx.r[1].u32.wrapping_add(-480 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822F29DC: 3AC00000  li r22, 0
	ctx.r[22].s64 = 0;
	// 822F29E0: 9101021C  stw r8, 0x21c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(540 as u32), ctx.r[8].u32 ) };
	// 822F29E4: 3D608000  lis r11, -0x8000
	ctx.r[11].s64 = -2147483648;
	// 822F29E8: 92C10058  stw r22, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[22].u32 ) };
	// 822F29EC: 7CD13378  mr r17, r6
	ctx.r[17].u64 = ctx.r[6].u64;
	// 822F29F0: 92C1005C  stw r22, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[22].u32 ) };
	// 822F29F4: 7C922378  mr r18, r4
	ctx.r[18].u64 = ctx.r[4].u64;
	// 822F29F8: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 822F29FC: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 822F2A00: 7C6F1B78  mr r15, r3
	ctx.r[15].u64 = ctx.r[3].u64;
	// 822F2A04: 7CB02B78  mr r16, r5
	ctx.r[16].u64 = ctx.r[5].u64;
	// 822F2A08: 80710000  lwz r3, 0(r17)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[17].u32.wrapping_add(0 as u32) ) } as u64;
	// 822F2A0C: 7CEE3B78  mr r14, r7
	ctx.r[14].u64 = ctx.r[7].u64;
	// 822F2A10: 48C28611  bl 0x82f1b020
	ctx.lr = 0x822F2A14;
	sub_82F1B020(ctx, base);
	// 822F2A14: 80710000  lwz r3, 0(r17)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[17].u32.wrapping_add(0 as u32) ) } as u64;
	// 822F2A18: 48C27E91  bl 0x82f1a8a8
	ctx.lr = 0x822F2A1C;
	sub_82F1A8A8(ctx, base);
	// 822F2A1C: 7C741B78  mr r20, r3
	ctx.r[20].u64 = ctx.r[3].u64;
	// 822F2A20: 81740004  lwz r11, 4(r20)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(4 as u32) ) } as u64;
	// 822F2A24: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 822F2A28: 419A0450  beq cr6, 0x822f2e78
	if ctx.cr[6].eq {
	pc = 0x822F2E78; continue 'dispatch;
	}
	// 822F2A2C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 822F2A30: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 822F2A34: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 822F2A38: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 822F2A3C: 7ED3B378  mr r19, r22
	ctx.r[19].u64 = ctx.r[22].u64;
	// 822F2A40: C38B08A4  lfs f28, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[28].f64 = (tmp.f32 as f64);
	// 822F2A44: C3AA08A8  lfs f29, 0x8a8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2216 as u32) ) };
	ctx.f[29].f64 = (tmp.f32 as f64);
	// 822F2A48: C3C99528  lfs f30, -0x6ad8(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-27352 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 822F2A4C: 81710000  lwz r11, 0(r17)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[17].u32.wrapping_add(0 as u32) ) } as u64;
	// 822F2A50: 7EDBB378  mr r27, r22
	ctx.r[27].u64 = ctx.r[22].u64;
	// 822F2A54: 7ECAB378  mr r10, r22
	ctx.r[10].u64 = ctx.r[22].u64;
	// 822F2A58: 83C1005C  lwz r30, 0x5c(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 822F2A5C: 936100A4  stw r27, 0xa4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(164 as u32), ctx.r[27].u32 ) };
	// 822F2A60: 914100A8  stw r10, 0xa8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(168 as u32), ctx.r[10].u32 ) };
	// 822F2A64: 92C100AC  stw r22, 0xac(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(172 as u32), ctx.r[22].u32 ) };
	// 822F2A68: C00B0010  lfs f0, 0x10(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 822F2A6C: EFE0F02A  fadds f31, f0, f30
	ctx.f[31].f64 = ((ctx.f[0].f64 + ctx.f[30].f64) as f32) as f64;
	// 822F2A70: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 822F2A74: 419A0014  beq cr6, 0x822f2a88
	if ctx.cr[6].eq {
	pc = 0x822F2A88; continue 'dispatch;
	}
	// 822F2A78: 7D7B5050  subf r11, r27, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[27].s64;
	// 822F2A7C: 7D6B1670  srawi r11, r11, 2
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 2) as i64;
	// 822F2A80: 2B0B0003  cmplwi cr6, r11, 3
	ctx.cr[6].compare_u32(ctx.r[11].u32, 3 as u32, &mut ctx.xer);
	// 822F2A84: 409800AC  bge cr6, 0x822f2b30
	if !ctx.cr[6].lt {
	pc = 0x822F2B30; continue 'dispatch;
	}
	// 822F2A88: 7EDFB378  mr r31, r22
	ctx.r[31].u64 = ctx.r[22].u64;
	// 822F2A8C: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 822F2A90: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 822F2A94: 419A0090  beq cr6, 0x822f2b24
	if ctx.cr[6].eq {
	pc = 0x822F2B24; continue 'dispatch;
	}
	// 822F2A98: 7F6BDB78  mr r11, r27
	ctx.r[11].u64 = ctx.r[27].u64;
	// 822F2A9C: 7F1B5040  cmplw cr6, r27, r10
	ctx.cr[6].compare_u32(ctx.r[27].u32, ctx.r[10].u32, &mut ctx.xer);
	// 822F2AA0: 419A0024  beq cr6, 0x822f2ac4
	if ctx.cr[6].eq {
	pc = 0x822F2AC4; continue 'dispatch;
	}
	// 822F2AA4: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 822F2AA8: 7F09F840  cmplw cr6, r9, r31
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[31].u32, &mut ctx.xer);
	// 822F2AAC: 419A0010  beq cr6, 0x822f2abc
	if ctx.cr[6].eq {
	pc = 0x822F2ABC; continue 'dispatch;
	}
	// 822F2AB0: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 822F2AB4: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 822F2AB8: 409AFFEC  bne cr6, 0x822f2aa4
	if !ctx.cr[6].eq {
	pc = 0x822F2AA4; continue 'dispatch;
	}
	// 822F2ABC: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 822F2AC0: 409A0054  bne cr6, 0x822f2b14
	if !ctx.cr[6].eq {
	pc = 0x822F2B14; continue 'dispatch;
	}
	// 822F2AC4: 81740000  lwz r11, 0(r20)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(0 as u32) ) } as u64;
	// 822F2AC8: 57E92036  slwi r9, r31, 4
	ctx.r[9].u32 = ctx.r[31].u32.wrapping_shl(4);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 822F2ACC: 81010058  lwz r8, 0x58(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 822F2AD0: 38E10068  addi r7, r1, 0x68
	ctx.r[7].s64 = ctx.r[1].s64 + 104;
	// 822F2AD4: 7D735A14  add r11, r19, r11
	ctx.r[11].u64 = ctx.r[19].u64 + ctx.r[11].u64;
	// 822F2AD8: 13E940C7  vcmpequd (lvx128) v31, v9, v8
	tmp.u32 = ctx.r[9].u32 + ctx.r[8].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 822F2ADC: 13C058C7  vcmpequd (lvx128) v30, v0, v11
	tmp.u32 = ctx.r[11].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[62] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822F2EB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822F2EB0 size=776
    let mut pc: u32 = 0x822F2EB0;
    'dispatch: loop {
        match pc {
            0x822F2EB0 => {
    //   block [0x822F2EB0..0x822F31B8)
	// 822F2EB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822F2EB4: 48EB52AD  bl 0x831a8160
	ctx.lr = 0x822F2EB8;
	sub_831A8130(ctx, base);
	// 822F2EB8: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822F2EBC: 81660000  lwz r11, 0(r6)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 822F2EC0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822F2EC4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 822F2EC8: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 822F2ECC: 7CFC3B78  mr r28, r7
	ctx.r[28].u64 = ctx.r[7].u64;
	// 822F2ED0: 7D1B4378  mr r27, r8
	ctx.r[27].u64 = ctx.r[8].u64;
	// 822F2ED4: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 822F2ED8: 2F0A000C  cmpwi cr6, r10, 0xc
	ctx.cr[6].compare_i32(ctx.r[10].s32, 12, &mut ctx.xer);
	// 822F2EDC: 41990180  bgt cr6, 0x822f305c
	if ctx.cr[6].gt {
	pc = 0x822F305C; continue 'dispatch;
	}
	// 822F2EE0: 419A0144  beq cr6, 0x822f3024
	if ctx.cr[6].eq {
	pc = 0x822F3024; continue 'dispatch;
	}
	// 822F2EE4: 2F0A0004  cmpwi cr6, r10, 4
	ctx.cr[6].compare_i32(ctx.r[10].s32, 4, &mut ctx.xer);
	// 822F2EE8: 419A0104  beq cr6, 0x822f2fec
	if ctx.cr[6].eq {
	pc = 0x822F2FEC; continue 'dispatch;
	}
	// 822F2EEC: 2F0A0005  cmpwi cr6, r10, 5
	ctx.cr[6].compare_i32(ctx.r[10].s32, 5, &mut ctx.xer);
	// 822F2EF0: 419A00C4  beq cr6, 0x822f2fb4
	if ctx.cr[6].eq {
	pc = 0x822F2FB4; continue 'dispatch;
	}
	// 822F2EF4: 2F0A0007  cmpwi cr6, r10, 7
	ctx.cr[6].compare_i32(ctx.r[10].s32, 7, &mut ctx.xer);
	// 822F2EF8: 419A0084  beq cr6, 0x822f2f7c
	if ctx.cr[6].eq {
	pc = 0x822F2F7C; continue 'dispatch;
	}
	// 822F2EFC: 2F0A000A  cmpwi cr6, r10, 0xa
	ctx.cr[6].compare_i32(ctx.r[10].s32, 10, &mut ctx.xer);
	// 822F2F00: 419A0044  beq cr6, 0x822f2f44
	if ctx.cr[6].eq {
	pc = 0x822F2F44; continue 'dispatch;
	}
	// 822F2F04: 2F0A000B  cmpwi cr6, r10, 0xb
	ctx.cr[6].compare_i32(ctx.r[10].s32, 11, &mut ctx.xer);
	// 822F2F08: 409A02A8  bne cr6, 0x822f31b0
	if !ctx.cr[6].eq {
	pc = 0x822F31B0; continue 'dispatch;
	}
	// 822F2F0C: 7D7A5B78  mr r26, r11
	ctx.r[26].u64 = ctx.r[11].u64;
	// 822F2F10: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 822F2F14: 93410058  stw r26, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[26].u32 ) };
	// 822F2F18: 4182000C  beq 0x822f2f24
	if ctx.cr[0].eq {
	pc = 0x822F2F24; continue 'dispatch;
	}
	// 822F2F1C: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 822F2F20: 4BFF5329  bl 0x822e8248
	ctx.lr = 0x822F2F24;
	sub_822E8248(ctx, base);
	// 822F2F24: 7F68DB78  mr r8, r27
	ctx.r[8].u64 = ctx.r[27].u64;
	// 822F2F28: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 822F2F2C: 38C10058  addi r6, r1, 0x58
	ctx.r[6].s64 = ctx.r[1].s64 + 88;
	// 822F2F30: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 822F2F34: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 822F2F38: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822F2F3C: 4BFFE55D  bl 0x822f1498
	ctx.lr = 0x822F2F40;
	sub_822F1498(ctx, base);
	// 822F2F40: 48000260  b 0x822f31a0
	pc = 0x822F31A0; continue 'dispatch;
	// 822F2F44: 7D7A5B78  mr r26, r11
	ctx.r[26].u64 = ctx.r[11].u64;
	// 822F2F48: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 822F2F4C: 93410058  stw r26, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[26].u32 ) };
	// 822F2F50: 4182000C  beq 0x822f2f5c
	if ctx.cr[0].eq {
	pc = 0x822F2F5C; continue 'dispatch;
	}
	// 822F2F54: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 822F2F58: 4BFF52F1  bl 0x822e8248
	ctx.lr = 0x822F2F5C;
	sub_822E8248(ctx, base);
	// 822F2F5C: 7F68DB78  mr r8, r27
	ctx.r[8].u64 = ctx.r[27].u64;
	// 822F2F60: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 822F2F64: 38C10058  addi r6, r1, 0x58
	ctx.r[6].s64 = ctx.r[1].s64 + 88;
	// 822F2F68: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 822F2F6C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 822F2F70: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822F2F74: 4BFFE605  bl 0x822f1578
	ctx.lr = 0x822F2F78;
	sub_822F1578(ctx, base);
	// 822F2F78: 48000228  b 0x822f31a0
	pc = 0x822F31A0; continue 'dispatch;
	// 822F2F7C: 7D7A5B78  mr r26, r11
	ctx.r[26].u64 = ctx.r[11].u64;
	// 822F2F80: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 822F2F84: 93410058  stw r26, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[26].u32 ) };
	// 822F2F88: 4182000C  beq 0x822f2f94
	if ctx.cr[0].eq {
	pc = 0x822F2F94; continue 'dispatch;
	}
	// 822F2F8C: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 822F2F90: 4BFF52B9  bl 0x822e8248
	ctx.lr = 0x822F2F94;
	sub_822E8248(ctx, base);
	// 822F2F94: 7F68DB78  mr r8, r27
	ctx.r[8].u64 = ctx.r[27].u64;
	// 822F2F98: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 822F2F9C: 38C10058  addi r6, r1, 0x58
	ctx.r[6].s64 = ctx.r[1].s64 + 88;
	// 822F2FA0: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 822F2FA4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 822F2FA8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822F2FAC: 4BFFFA1D  bl 0x822f29c8
	ctx.lr = 0x822F2FB0;
	sub_822F29C8(ctx, base);
	// 822F2FB0: 480001F0  b 0x822f31a0
	pc = 0x822F31A0; continue 'dispatch;
	// 822F2FB4: 7D7A5B78  mr r26, r11
	ctx.r[26].u64 = ctx.r[11].u64;
	// 822F2FB8: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 822F2FBC: 93410058  stw r26, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[26].u32 ) };
	// 822F2FC0: 4182000C  beq 0x822f2fcc
	if ctx.cr[0].eq {
	pc = 0x822F2FCC; continue 'dispatch;
	}
	// 822F2FC4: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 822F2FC8: 4BFF5281  bl 0x822e8248
	ctx.lr = 0x822F2FCC;
	sub_822E8248(ctx, base);
	// 822F2FCC: 7F68DB78  mr r8, r27
	ctx.r[8].u64 = ctx.r[27].u64;
	// 822F2FD0: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 822F2FD4: 38C10058  addi r6, r1, 0x58
	ctx.r[6].s64 = ctx.r[1].s64 + 88;
	// 822F2FD8: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 822F2FDC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 822F2FE0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822F2FE4: 4BFFF625  bl 0x822f2608
	ctx.lr = 0x822F2FE8;
	sub_822F2608(ctx, base);
	// 822F2FE8: 480001B8  b 0x822f31a0
	pc = 0x822F31A0; continue 'dispatch;
	// 822F2FEC: 7D7A5B78  mr r26, r11
	ctx.r[26].u64 = ctx.r[11].u64;
	// 822F2FF0: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 822F2FF4: 93410058  stw r26, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[26].u32 ) };
	// 822F2FF8: 4182000C  beq 0x822f3004
	if ctx.cr[0].eq {
	pc = 0x822F3004; continue 'dispatch;
	}
	// 822F2FFC: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 822F3000: 4BFF5249  bl 0x822e8248
	ctx.lr = 0x822F3004;
	sub_822E8248(ctx, base);
	// 822F3004: 7F68DB78  mr r8, r27
	ctx.r[8].u64 = ctx.r[27].u64;
	// 822F3008: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 822F300C: 38C10058  addi r6, r1, 0x58
	ctx.r[6].s64 = ctx.r[1].s64 + 88;
	// 822F3010: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 822F3014: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 822F3018: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822F301C: 4BFFF47D  bl 0x822f2498
	ctx.lr = 0x822F3020;
	sub_822F2498(ctx, base);
	// 822F3020: 48000180  b 0x822f31a0
	pc = 0x822F31A0; continue 'dispatch;
	// 822F3024: 7D7A5B78  mr r26, r11
	ctx.r[26].u64 = ctx.r[11].u64;
	// 822F3028: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 822F302C: 93410058  stw r26, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[26].u32 ) };
	// 822F3030: 4182000C  beq 0x822f303c
	if ctx.cr[0].eq {
	pc = 0x822F303C; continue 'dispatch;
	}
	// 822F3034: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 822F3038: 4BFF5211  bl 0x822e8248
	ctx.lr = 0x822F303C;
	sub_822E8248(ctx, base);
	// 822F303C: 7F68DB78  mr r8, r27
	ctx.r[8].u64 = ctx.r[27].u64;
	// 822F3040: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 822F3044: 38C10058  addi r6, r1, 0x58
	ctx.r[6].s64 = ctx.r[1].s64 + 88;
	// 822F3048: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 822F304C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 822F3050: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822F3054: 4BFFE71D  bl 0x822f1770
	ctx.lr = 0x822F3058;
	sub_822F1770(ctx, base);
	// 822F3058: 48000148  b 0x822f31a0
	pc = 0x822F31A0; continue 'dispatch;
	// 822F305C: 2F0A000D  cmpwi cr6, r10, 0xd
	ctx.cr[6].compare_i32(ctx.r[10].s32, 13, &mut ctx.xer);
	// 822F3060: 419A010C  beq cr6, 0x822f316c
	if ctx.cr[6].eq {
	pc = 0x822F316C; continue 'dispatch;
	}
	// 822F3064: 2F0A000F  cmpwi cr6, r10, 0xf
	ctx.cr[6].compare_i32(ctx.r[10].s32, 15, &mut ctx.xer);
	// 822F3068: 419A00CC  beq cr6, 0x822f3134
	if ctx.cr[6].eq {
	pc = 0x822F3134; continue 'dispatch;
	}
	// 822F306C: 2F0A0016  cmpwi cr6, r10, 0x16
	ctx.cr[6].compare_i32(ctx.r[10].s32, 22, &mut ctx.xer);
	// 822F3070: 419A008C  beq cr6, 0x822f30fc
	if ctx.cr[6].eq {
	pc = 0x822F30FC; continue 'dispatch;
	}
	// 822F3074: 2F0A001A  cmpwi cr6, r10, 0x1a
	ctx.cr[6].compare_i32(ctx.r[10].s32, 26, &mut ctx.xer);
	// 822F3078: 419A0044  beq cr6, 0x822f30bc
	if ctx.cr[6].eq {
	pc = 0x822F30BC; continue 'dispatch;
	}
	// 822F307C: 2F0A001C  cmpwi cr6, r10, 0x1c
	ctx.cr[6].compare_i32(ctx.r[10].s32, 28, &mut ctx.xer);
	// 822F3080: 409A0130  bne cr6, 0x822f31b0
	if !ctx.cr[6].eq {
	pc = 0x822F31B0; continue 'dispatch;
	}
	// 822F3084: 7D7A5B78  mr r26, r11
	ctx.r[26].u64 = ctx.r[11].u64;
	// 822F3088: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 822F308C: 93410058  stw r26, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[26].u32 ) };
	// 822F3090: 4182000C  beq 0x822f309c
	if ctx.cr[0].eq {
	pc = 0x822F309C; continue 'dispatch;
	}
	// 822F3094: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 822F3098: 4BFF51B1  bl 0x822e8248
	ctx.lr = 0x822F309C;
	sub_822E8248(ctx, base);
	// 822F309C: 7F68DB78  mr r8, r27
	ctx.r[8].u64 = ctx.r[27].u64;
	// 822F30A0: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 822F30A4: 38C10058  addi r6, r1, 0x58
	ctx.r[6].s64 = ctx.r[1].s64 + 88;
	// 822F30A8: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 822F30AC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 822F30B0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822F30B4: 4BFFE59D  bl 0x822f1650
	ctx.lr = 0x822F30B8;
	sub_822F1650(ctx, base);
	// 822F30B8: 480000E8  b 0x822f31a0
	pc = 0x822F31A0; continue 'dispatch;
	// 822F30BC: 806B0010  lwz r3, 0x10(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 822F30C0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 822F30C4: 90610050  stw r3, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 822F30C8: 419A0008  beq cr6, 0x822f30d0
	if ctx.cr[6].eq {
	pc = 0x822F30D0; continue 'dispatch;
	}
	// 822F30CC: 4BFF517D  bl 0x822e8248
	ctx.lr = 0x822F30D0;
	sub_822E8248(ctx, base);
	// 822F30D0: 7F68DB78  mr r8, r27
	ctx.r[8].u64 = ctx.r[27].u64;
	// 822F30D4: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 822F30D8: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 822F30DC: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 822F30E0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 822F30E4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822F30E8: 4BFFFDC9  bl 0x822f2eb0
	ctx.lr = 0x822F30EC;
	sub_822F2EB0(ctx, base);
	// 822F30EC: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 822F30F0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 822F30F4: 419A00BC  beq cr6, 0x822f31b0
	if ctx.cr[6].eq {
	pc = 0x822F31B0; continue 'dispatch;
	}
	// 822F30F8: 480000B4  b 0x822f31ac
	pc = 0x822F31AC; continue 'dispatch;
	// 822F30FC: 7D7A5B78  mr r26, r11
	ctx.r[26].u64 = ctx.r[11].u64;
	// 822F3100: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 822F3104: 93410058  stw r26, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[26].u32 ) };
	// 822F3108: 4182000C  beq 0x822f3114
	if ctx.cr[0].eq {
	pc = 0x822F3114; continue 'dispatch;
	}
	// 822F310C: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 822F3110: 4BFF5139  bl 0x822e8248
	ctx.lr = 0x822F3114;
	sub_822E8248(ctx, base);
	// 822F3114: 7F68DB78  mr r8, r27
	ctx.r[8].u64 = ctx.r[27].u64;
	// 822F3118: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 822F311C: 38C10058  addi r6, r1, 0x58
	ctx.r[6].s64 = ctx.r[1].s64 + 88;
	// 822F3120: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 822F3124: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 822F3128: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822F312C: 4BFFE44D  bl 0x822f1578
	ctx.lr = 0x822F3130;
	sub_822F1578(ctx, base);
	// 822F3130: 48000070  b 0x822f31a0
	pc = 0x822F31A0; continue 'dispatch;
	// 822F3134: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 822F3138: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 822F313C: 90610054  stw r3, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 822F3140: 41820008  beq 0x822f3148
	if ctx.cr[0].eq {
	pc = 0x822F3148; continue 'dispatch;
	}
	// 822F3144: 4BFF5105  bl 0x822e8248
	ctx.lr = 0x822F3148;
	sub_822E8248(ctx, base);
	// 822F3148: 7F68DB78  mr r8, r27
	ctx.r[8].u64 = ctx.r[27].u64;
	// 822F314C: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 822F3150: 38C10054  addi r6, r1, 0x54
	ctx.r[6].s64 = ctx.r[1].s64 + 84;
	// 822F3154: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 822F3158: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 822F315C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822F3160: 48000059  bl 0x822f31b8
	ctx.lr = 0x822F3164;
	sub_822F31B8(ctx, base);
	// 822F3164: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 822F3168: 4BFFFF88  b 0x822f30f0
	pc = 0x822F30F0; continue 'dispatch;
	// 822F316C: 7D7A5B78  mr r26, r11
	ctx.r[26].u64 = ctx.r[11].u64;
	// 822F3170: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 822F3174: 93410058  stw r26, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[26].u32 ) };
	// 822F3178: 4182000C  beq 0x822f3184
	if ctx.cr[0].eq {
	pc = 0x822F3184; continue 'dispatch;
	}
	// 822F317C: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 822F3180: 4BFF50C9  bl 0x822e8248
	ctx.lr = 0x822F3184;
	sub_822E8248(ctx, base);
	// 822F3184: 7F68DB78  mr r8, r27
	ctx.r[8].u64 = ctx.r[27].u64;
	// 822F3188: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 822F318C: 38C10058  addi r6, r1, 0x58
	ctx.r[6].s64 = ctx.r[1].s64 + 88;
	// 822F3190: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 822F3194: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 822F3198: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822F319C: 4BFFE545  bl 0x822f16e0
	ctx.lr = 0x822F31A0;
	sub_822F16E0(ctx, base);
	// 822F31A0: 2B1A0000  cmplwi cr6, r26, 0
	ctx.cr[6].compare_u32(ctx.r[26].u32, 0 as u32, &mut ctx.xer);
	// 822F31A4: 419A000C  beq cr6, 0x822f31b0
	if ctx.cr[6].eq {
	pc = 0x822F31B0; continue 'dispatch;
	}
	// 822F31A8: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 822F31AC: 4BFF50BD  bl 0x822e8268
	ctx.lr = 0x822F31B0;
	sub_822E8268(ctx, base);
	// 822F31B0: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 822F31B4: 48EB4FFC  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822F31B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822F31B8 size=212
    let mut pc: u32 = 0x822F31B8;
    'dispatch: loop {
        match pc {
            0x822F31B8 => {
    //   block [0x822F31B8..0x822F328C)
	// 822F31B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822F31BC: 48EB4F9D  bl 0x831a8158
	ctx.lr = 0x822F31C0;
	sub_831A8130(ctx, base);
	// 822F31C0: 9421FD50  stwu r1, -0x2b0(r1)
	ea = ctx.r[1].u32.wrapping_add(-688 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822F31C4: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 822F31C8: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 822F31CC: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 822F31D0: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 822F31D4: 7CF93B78  mr r25, r7
	ctx.r[25].u64 = ctx.r[7].u64;
	// 822F31D8: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 822F31DC: 7D184378  mr r24, r8
	ctx.r[24].u64 = ctx.r[8].u64;
	// 822F31E0: 386B0010  addi r3, r11, 0x10
	ctx.r[3].s64 = ctx.r[11].s64 + 16;
	// 822F31E4: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 822F31E8: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 822F31EC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 822F31F0: 4E800421  bctrl
	ctx.lr = 0x822F31F4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 822F31F4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822F31F8: 48000084  b 0x822f327c
	pc = 0x822F327C; continue 'dispatch;
	// 822F31FC: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 822F3200: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 822F3204: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 822F3208: 386B0010  addi r3, r11, 0x10
	ctx.r[3].s64 = ctx.r[11].s64 + 16;
	// 822F320C: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 822F3210: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 822F3214: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 822F3218: 4E800421  bctrl
	ctx.lr = 0x822F321C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 822F321C: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 822F3220: 93A10050  stw r29, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[29].u32 ) };
	// 822F3224: 4182000C  beq 0x822f3230
	if ctx.cr[0].eq {
	pc = 0x822F3230; continue 'dispatch;
	}
	// 822F3228: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 822F322C: 4BFF501D  bl 0x822e8248
	ctx.lr = 0x822F3230;
	sub_822E8248(ctx, base);
	// 822F3230: 7F08C378  mr r8, r24
	ctx.r[8].u64 = ctx.r[24].u64;
	// 822F3234: 7F27CB78  mr r7, r25
	ctx.r[7].u64 = ctx.r[25].u64;
	// 822F3238: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 822F323C: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 822F3240: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 822F3244: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 822F3248: 4BFFFC69  bl 0x822f2eb0
	ctx.lr = 0x822F324C;
	sub_822F2EB0(ctx, base);
	// 822F324C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 822F3250: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 822F3254: 386B0010  addi r3, r11, 0x10
	ctx.r[3].s64 = ctx.r[11].s64 + 16;
	// 822F3258: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 822F325C: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 822F3260: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 822F3264: 4E800421  bctrl
	ctx.lr = 0x822F3268;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 822F3268: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822F326C: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 822F3270: 419A000C  beq cr6, 0x822f327c
	if ctx.cr[6].eq {
	pc = 0x822F327C; continue 'dispatch;
	}
	// 822F3274: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 822F3278: 4BFF4FF1  bl 0x822e8268
	ctx.lr = 0x822F327C;
	sub_822E8268(ctx, base);
	// 822F327C: 2F1FFFFF  cmpwi cr6, r31, -1
	ctx.cr[6].compare_i32(ctx.r[31].s32, -1, &mut ctx.xer);
	// 822F3280: 409AFF7C  bne cr6, 0x822f31fc
	if !ctx.cr[6].eq {
	pc = 0x822F31FC; continue 'dispatch;
	}
	// 822F3284: 382102B0  addi r1, r1, 0x2b0
	ctx.r[1].s64 = ctx.r[1].s64 + 688;
	// 822F3288: 48EB4F20  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822F3290(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822F3290 size=268
    let mut pc: u32 = 0x822F3290;
    'dispatch: loop {
        match pc {
            0x822F3290 => {
    //   block [0x822F3290..0x822F339C)
	// 822F3290: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822F3294: 48EB4EC1  bl 0x831a8154
	ctx.lr = 0x822F3298;
	sub_831A8130(ctx, base);
	// 822F3298: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822F329C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822F32A0: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 822F32A4: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 822F32A8: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 822F32AC: 7CFA3B78  mr r26, r7
	ctx.r[26].u64 = ctx.r[7].u64;
	// 822F32B0: 7D194378  mr r25, r8
	ctx.r[25].u64 = ctx.r[8].u64;
	// 822F32B4: 7D384B78  mr r24, r9
	ctx.r[24].u64 = ctx.r[9].u64;
	// 822F32B8: 7D575378  mr r23, r10
	ctx.r[23].u64 = ctx.r[10].u64;
	// 822F32BC: 48B1EC1D  bl 0x82e11ed8
	ctx.lr = 0x822F32C0;
	sub_82E11ED8(ctx, base);
	// 822F32C0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 822F32C4: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 822F32C8: 396BC51C  addi r11, r11, -0x3ae4
	ctx.r[11].s64 = ctx.r[11].s64 + -15076;
	// 822F32CC: 387F0044  addi r3, r31, 0x44
	ctx.r[3].s64 = ctx.r[31].s64 + 68;
	// 822F32D0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 822F32D4: 397F0014  addi r11, r31, 0x14
	ctx.r[11].s64 = ctx.r[31].s64 + 20;
	// 822F32D8: 93DF0008  stw r30, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 822F32DC: 93DF000C  stw r30, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[30].u32 ) };
	// 822F32E0: 93DF0010  stw r30, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[30].u32 ) };
	// 822F32E4: 93DF0014  stw r30, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[30].u32 ) };
	// 822F32E8: 93DF0018  stw r30, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[30].u32 ) };
	// 822F32EC: 93DF001C  stw r30, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[30].u32 ) };
	// 822F32F0: 93DF0020  stw r30, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[30].u32 ) };
	// 822F32F4: 93DF0024  stw r30, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[30].u32 ) };
	// 822F32F8: 93DF0028  stw r30, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[30].u32 ) };
	// 822F32FC: 93DF002C  stw r30, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[30].u32 ) };
	// 822F3300: 93DF0030  stw r30, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[30].u32 ) };
	// 822F3304: 93DF003C  stw r30, 0x3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[30].u32 ) };
	// 822F3308: 93DF0040  stw r30, 0x40(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), ctx.r[30].u32 ) };
	// 822F330C: 48EA8575  bl 0x8319b880
	ctx.lr = 0x822F3310;
	sub_8319B880(ctx, base);
	// 822F3310: 93DF0060  stw r30, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[30].u32 ) };
	// 822F3314: 93DF0064  stw r30, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[30].u32 ) };
	// 822F3318: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 822F331C: 93C10074  stw r30, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[30].u32 ) };
	// 822F3320: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822F3324: 93C10078  stw r30, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[30].u32 ) };
	// 822F3328: 390B6880  addi r8, r11, 0x6880
	ctx.r[8].s64 = ctx.r[11].s64 + 26752;
	// 822F332C: 93C1007C  stw r30, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[30].u32 ) };
	// 822F3330: 7EE7BB78  mr r7, r23
	ctx.r[7].u64 = ctx.r[23].u64;
	// 822F3334: 93C10064  stw r30, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[30].u32 ) };
	// 822F3338: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 822F333C: 93C10068  stw r30, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[30].u32 ) };
	// 822F3340: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 822F3344: 93C1006C  stw r30, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[30].u32 ) };
	// 822F3348: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 822F334C: 4BFFFB65  bl 0x822f2eb0
	ctx.lr = 0x822F3350;
	sub_822F2EB0(ctx, base);
	// 822F3350: 81610124  lwz r11, 0x124(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(292 as u32) ) } as u64;
	// 822F3354: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822F3358: 80C1012C  lwz r6, 0x12c(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(300 as u32) ) } as u64;
	// 822F335C: 7F2ACB78  mr r10, r25
	ctx.r[10].u64 = ctx.r[25].u64;
	// 822F3360: 93010054  stw r24, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[24].u32 ) };
	// 822F3364: 7F49D378  mr r9, r26
	ctx.r[9].u64 = ctx.r[26].u64;
	// 822F3368: 39010060  addi r8, r1, 0x60
	ctx.r[8].s64 = ctx.r[1].s64 + 96;
	// 822F336C: 38E10070  addi r7, r1, 0x70
	ctx.r[7].s64 = ctx.r[1].s64 + 112;
	// 822F3370: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 822F3374: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 822F3378: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 822F337C: 4BFFE5CD  bl 0x822f1948
	ctx.lr = 0x822F3380;
	sub_822F1948(ctx, base);
	// 822F3380: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 822F3384: 48176C7D  bl 0x8246a000
	ctx.lr = 0x822F3388;
	sub_8246A000(ctx, base);
	// 822F3388: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 822F338C: 48176C75  bl 0x8246a000
	ctx.lr = 0x822F3390;
	sub_8246A000(ctx, base);
	// 822F3390: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822F3394: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 822F3398: 48EB4E0C  b 0x831a81a4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822F33A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822F33A0 size=144
    let mut pc: u32 = 0x822F33A0;
    'dispatch: loop {
        match pc {
            0x822F33A0 => {
    //   block [0x822F33A0..0x822F3430)
	// 822F33A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822F33A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822F33A8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822F33AC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822F33B0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822F33B4: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 822F33B8: 48B1FEE1  bl 0x82e13298
	ctx.lr = 0x822F33BC;
	sub_82E13298(ctx, base);
	// 822F33BC: 807F0040  lwz r3, 0x40(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as u64;
	// 822F33C0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 822F33C4: 419A0008  beq cr6, 0x822f33cc
	if ctx.cr[6].eq {
	pc = 0x822F33CC; continue 'dispatch;
	}
	// 822F33C8: 4BFCD4C9  bl 0x822c0890
	ctx.lr = 0x822F33CC;
	sub_822C0890(ctx, base);
	// 822F33CC: 807F0030  lwz r3, 0x30(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 822F33D0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 822F33D4: 419A0008  beq cr6, 0x822f33dc
	if ctx.cr[6].eq {
	pc = 0x822F33DC; continue 'dispatch;
	}
	// 822F33D8: 4BFCD4B9  bl 0x822c0890
	ctx.lr = 0x822F33DC;
	sub_822C0890(ctx, base);
	// 822F33DC: 387F0014  addi r3, r31, 0x14
	ctx.r[3].s64 = ctx.r[31].s64 + 20;
	// 822F33E0: 4BFFE511  bl 0x822f18f0
	ctx.lr = 0x822F33E4;
	sub_822F18F0(ctx, base);
	// 822F33E4: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 822F33E8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 822F33EC: 419A0008  beq cr6, 0x822f33f4
	if ctx.cr[6].eq {
	pc = 0x822F33F4; continue 'dispatch;
	}
	// 822F33F0: 488EBBE9  bl 0x82bdefd8
	ctx.lr = 0x822F33F4;
	sub_82BDEFD8(ctx, base);
	// 822F33F4: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 822F33F8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 822F33FC: 419A0008  beq cr6, 0x822f3404
	if ctx.cr[6].eq {
	pc = 0x822F3404; continue 'dispatch;
	}
	// 822F3400: 488EBBD9  bl 0x82bdefd8
	ctx.lr = 0x822F3404;
	sub_82BDEFD8(ctx, base);
	// 822F3404: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 822F3408: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 822F340C: 419A0008  beq cr6, 0x822f3414
	if ctx.cr[6].eq {
	pc = 0x822F3414; continue 'dispatch;
	}
	// 822F3410: 488EBBC9  bl 0x82bdefd8
	ctx.lr = 0x822F3414;
	sub_82BDEFD8(ctx, base);
	// 822F3414: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822F3418: 48B1EAD9  bl 0x82e11ef0
	ctx.lr = 0x822F341C;
	sub_82E11EF0(ctx, base);
	// 822F341C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 822F3420: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822F3424: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822F3428: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822F342C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822F3430(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822F3430 size=76
    let mut pc: u32 = 0x822F3430;
    'dispatch: loop {
        match pc {
            0x822F3430 => {
    //   block [0x822F3430..0x822F347C)
	// 822F3430: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822F3434: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822F3438: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 822F343C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822F3440: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822F3444: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822F3448: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 822F344C: 4BFFFF55  bl 0x822f33a0
	ctx.lr = 0x822F3450;
	sub_822F33A0(ctx, base);
	// 822F3450: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 822F3454: 4182000C  beq 0x822f3460
	if ctx.cr[0].eq {
	pc = 0x822F3460; continue 'dispatch;
	}
	// 822F3458: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822F345C: 48AFEF7D  bl 0x82df23d8
	ctx.lr = 0x822F3460;
	sub_82DF23D8(ctx, base);
	// 822F3460: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822F3464: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 822F3468: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822F346C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822F3470: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 822F3474: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822F3478: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822F3480(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x822F3480 size=152
    let mut pc: u32 = 0x822F3480;
    'dispatch: loop {
        match pc {
            0x822F3480 => {
    //   block [0x822F3480..0x822F3518)
	// 822F3480: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822F3484: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822F3488: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 822F348C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822F3490: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822F3494: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822F3498: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 822F349C: 48B1F845  bl 0x82e12ce0
	ctx.lr = 0x822F34A0;
	sub_82E12CE0(ctx, base);
	// 822F34A0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 822F34A4: 57C3003E  slwi r3, r30, 0
	ctx.r[3].u32 = ctx.r[30].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 822F34A8: 396BC52C  addi r11, r11, -0x3ad4
	ctx.r[11].s64 = ctx.r[11].s64 + -15060;
	// 822F34AC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 822F34B0: 93DF0060  stw r30, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[30].u32 ) };
	// 822F34B4: 48BE1C35  bl 0x82ed50e8
	ctx.lr = 0x822F34B8;
	sub_82ED50E8(ctx, base);
	// 822F34B8: 817F0060  lwz r11, 0x60(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 822F34BC: 3BDF0070  addi r30, r31, 0x70
	ctx.r[30].s64 = ctx.r[31].s64 + 112;
	// 822F34C0: 386B00E0  addi r3, r11, 0xe0
	ctx.r[3].s64 = ctx.r[11].s64 + 224;
	// 822F34C4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 822F34C8: 48BAD8C9  bl 0x82ea0d90
	ctx.lr = 0x822F34CC;
	sub_82EA0D90(ctx, base);
	// 822F34CC: 39000010  li r8, 0x10
	ctx.r[8].s64 = 16;
	// 822F34D0: 39200020  li r9, 0x20
	ctx.r[9].s64 = 32;
	// 822F34D4: 1380F407  vcmpneb. (lvlx128) v28, v0, v30
	tmp.u32 = ctx.r[30].u32;
	// load shuffled into ctx.v[60] using VectorMaskL[(tmp.u32 & 0xF)]
	// 822F34D8: 39400030  li r10, 0x30
	ctx.r[10].s64 = 48;
	// 822F34DC: 397F00B0  addi r11, r31, 0xb0
	ctx.r[11].s64 = ctx.r[31].s64 + 176;
	// 822F34E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822F34E4: 13E8F407  vcmpneb. (lvlx128) v31, v8, v30
	tmp.u32 = ctx.r[8].u32 + ctx.r[30].u32;
	// load shuffled into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 822F34E8: 13C9F407  vcmpneb. (lvlx128) v30, v9, v30
	tmp.u32 = ctx.r[9].u32 + ctx.r[30].u32;
	// load shuffled into ctx.v[62] using VectorMaskL[(tmp.u32 & 0xF)]
	// 822F34EC: 13AAF407  vcmpneb. (lvlx128) v29, v10, v30
	tmp.u32 = ctx.r[10].u32 + ctx.r[30].u32;
	// load shuffled into ctx.v[61] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822F3518(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822F3518 size=124
    let mut pc: u32 = 0x822F3518;
    'dispatch: loop {
        match pc {
            0x822F3518 => {
    //   block [0x822F3518..0x822F3594)
	// 822F3518: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822F351C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822F3520: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 822F3524: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822F3528: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822F352C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822F3530: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 822F3534: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 822F3538: 396BC52C  addi r11, r11, -0x3ad4
	ctx.r[11].s64 = ctx.r[11].s64 + -15060;
	// 822F353C: 815F0060  lwz r10, 0x60(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 822F3540: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 822F3544: 5543003E  slwi r3, r10, 0
	ctx.r[3].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 822F3548: A16A0006  lhz r11, 6(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(6 as u32) ) } as u64;
	// 822F354C: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 822F3550: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 822F3554: 419A000C  beq cr6, 0x822f3560
	if ctx.cr[6].eq {
	pc = 0x822F3560; continue 'dispatch;
	}
	// 822F3558: 48BE1BB1  bl 0x82ed5108
	ctx.lr = 0x822F355C;
	sub_82ED5108(ctx, base);
	// 822F355C: 93DF0060  stw r30, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[30].u32 ) };
	// 822F3560: 807F0060  lwz r3, 0x60(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 822F3564: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 822F3568: 419A000C  beq cr6, 0x822f3574
	if ctx.cr[6].eq {
	pc = 0x822F3574; continue 'dispatch;
	}
	// 822F356C: 48BE1B9D  bl 0x82ed5108
	ctx.lr = 0x822F3570;
	sub_82ED5108(ctx, base);
	// 822F3570: 93DF0060  stw r30, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[30].u32 ) };
	// 822F3574: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822F3578: 48B1F801  bl 0x82e12d78
	ctx.lr = 0x822F357C;
	sub_82E12D78(ctx, base);
	// 822F357C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 822F3580: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822F3584: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822F3588: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 822F358C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822F3590: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822F3598(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x822F3598 size=232
    let mut pc: u32 = 0x822F3598;
    'dispatch: loop {
        match pc {
            0x822F3598 => {
    //   block [0x822F3598..0x822F3680)
	// 822F3598: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822F359C: 48EB4BCD  bl 0x831a8168
	ctx.lr = 0x822F35A0;
	sub_831A8130(ctx, base);
	// 822F35A0: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822F35A4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822F35A8: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 822F35AC: 817F0060  lwz r11, 0x60(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 822F35B0: 894B00D8  lbz r10, 0xd8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(216 as u32) ) } as u64;
	// 822F35B4: 2B0A0007  cmplwi cr6, r10, 7
	ctx.cr[6].compare_u32(ctx.r[10].u32, 7 as u32, &mut ctx.xer);
	// 822F35B8: 419A0010  beq cr6, 0x822f35c8
	if ctx.cr[6].eq {
	pc = 0x822F35C8; continue 'dispatch;
	}
	// 822F35BC: 389F0070  addi r4, r31, 0x70
	ctx.r[4].s64 = ctx.r[31].s64 + 112;
	// 822F35C0: 386B00E0  addi r3, r11, 0xe0
	ctx.r[3].s64 = ctx.r[11].s64 + 224;
	// 822F35C4: 48BAD7CD  bl 0x82ea0d90
	ctx.lr = 0x822F35C8;
	sub_82EA0D90(ctx, base);
	// 822F35C8: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 822F35CC: 419A0060  beq cr6, 0x822f362c
	if ctx.cr[6].eq {
	pc = 0x822F362C; continue 'dispatch;
	}
	// 822F35D0: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 822F35D4: 3BDF00B0  addi r30, r31, 0xb0
	ctx.r[30].s64 = ctx.r[31].s64 + 176;
	// 822F35D8: 3B9F0070  addi r28, r31, 0x70
	ctx.r[28].s64 = ctx.r[31].s64 + 112;
	// 822F35DC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 822F35E0: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 822F35E4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 822F35E8: 4E800421  bctrl
	ctx.lr = 0x822F35EC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 822F35EC: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 822F35F0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 822F35F4: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 822F35F8: 4BFD1309  bl 0x822c4900
	ctx.lr = 0x822F35FC;
	sub_822C4900(ctx, base);
	// 822F35FC: 38E00010  li r7, 0x10
	ctx.r[7].s64 = 16;
	// 822F3600: 39000020  li r8, 0x20
	ctx.r[8].s64 = 32;
	// 822F3604: 13E01C07  vcmpneb. (lvlx128) v31, v0, v3
	tmp.u32 = ctx.r[3].u32;
	// load shuffled into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 822F3608: 39200030  li r9, 0x30
	ctx.r[9].s64 = 48;
	// 822F360C: 13C71C07  vcmpneb. (lvlx128) v30, v7, v3
	tmp.u32 = ctx.r[7].u32 + ctx.r[3].u32;
	// load shuffled into ctx.v[62] using VectorMaskL[(tmp.u32 & 0xF)]
	// 822F3610: 13A81C07  vcmpneb. (lvlx128) v29, v8, v3
	tmp.u32 = ctx.r[8].u32 + ctx.r[3].u32;
	// load shuffled into ctx.v[61] using VectorMaskL[(tmp.u32 & 0xF)]
	// 822F3614: 13891C07  vcmpneb. (lvlx128) v28, v9, v3
	tmp.u32 = ctx.r[9].u32 + ctx.r[3].u32;
	// load shuffled into ctx.v[60] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822F3680(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822F3680 size=76
    let mut pc: u32 = 0x822F3680;
    'dispatch: loop {
        match pc {
            0x822F3680 => {
    //   block [0x822F3680..0x822F36CC)
	// 822F3680: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822F3684: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822F3688: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 822F368C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822F3690: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822F3694: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822F3698: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 822F369C: 4BFFFE7D  bl 0x822f3518
	ctx.lr = 0x822F36A0;
	sub_822F3518(ctx, base);
	// 822F36A0: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 822F36A4: 4182000C  beq 0x822f36b0
	if ctx.cr[0].eq {
	pc = 0x822F36B0; continue 'dispatch;
	}
	// 822F36A8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822F36AC: 48AFED2D  bl 0x82df23d8
	ctx.lr = 0x822F36B0;
	sub_82DF23D8(ctx, base);
	// 822F36B0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822F36B4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 822F36B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822F36BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822F36C0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 822F36C4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822F36C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822F36D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x822F36D0 size=8
    let mut pc: u32 = 0x822F36D0;
    'dispatch: loop {
        match pc {
            0x822F36D0 => {
    //   block [0x822F36D0..0x822F36D8)
	// 822F36D0: D023015C  stfs f1, 0x15c(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(348 as u32), tmp.u32 ) };
	// 822F36D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822F36D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822F36D8 size=80
    let mut pc: u32 = 0x822F36D8;
    'dispatch: loop {
        match pc {
            0x822F36D8 => {
    //   block [0x822F36D8..0x822F3728)
	// 822F36D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822F36DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822F36E0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 822F36E4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822F36E8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822F36EC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822F36F0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 822F36F4: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 822F36F8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 822F36FC: 419A0010  beq cr6, 0x822f370c
	if ctx.cr[6].eq {
	pc = 0x822F370C; continue 'dispatch;
	}
	// 822F3700: 48BE1A09  bl 0x82ed5108
	ctx.lr = 0x822F3704;
	sub_82ED5108(ctx, base);
	// 822F3704: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 822F3708: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 822F370C: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 822F3710: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 822F3714: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822F3718: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822F371C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 822F3720: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822F3724: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822F3728(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x822F3728 size=80
    let mut pc: u32 = 0x822F3728;
    'dispatch: loop {
        match pc {
            0x822F3728 => {
    //   block [0x822F3728..0x822F3778)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822F3778(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822F3778 size=68
    let mut pc: u32 = 0x822F3778;
    'dispatch: loop {
        match pc {
            0x822F3778 => {
    //   block [0x822F3778..0x822F37BC)
	// 822F3778: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822F377C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822F3780: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822F3784: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822F3788: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822F378C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 822F3790: 548A07FF  clrlwi. r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 822F3794: 396BC540  addi r11, r11, -0x3ac0
	ctx.r[11].s64 = ctx.r[11].s64 + -15040;
	// 822F3798: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 822F379C: 41820008  beq 0x822f37a4
	if ctx.cr[0].eq {
	pc = 0x822F37A4; continue 'dispatch;
	}
	// 822F37A0: 4BFCCAC9  bl 0x822c0268
	ctx.lr = 0x822F37A4;
	sub_822C0268(ctx, base);
	// 822F37A4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822F37A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 822F37AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822F37B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822F37B4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822F37B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822F37C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x822F37C0 size=8
    let mut pc: u32 = 0x822F37C0;
    'dispatch: loop {
        match pc {
            0x822F37C0 => {
    //   block [0x822F37C0..0x822F37C8)
	// 822F37C0: 3863FFF8  addi r3, r3, -8
	ctx.r[3].s64 = ctx.r[3].s64 + -8;
	// 822F37C4: 4800035C  b 0x822f3b20
	sub_822F3B20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822F37C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x822F37C8 size=248
    let mut pc: u32 = 0x822F37C8;
    'dispatch: loop {
        match pc {
            0x822F37C8 => {
    //   block [0x822F37C8..0x822F38C0)
	// 822F37C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822F37CC: 48EB499D  bl 0x831a8168
	ctx.lr = 0x822F37D0;
	sub_831A8130(ctx, base);
	// 822F37D0: 9421FEE0  stwu r1, -0x120(r1)
	ea = ctx.r[1].u32.wrapping_add(-288 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822F37D4: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 822F37D8: 13C038C7  vcmpequd (lvx128) v30, v0, v7
	tmp.u32 = ctx.r[7].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[62] using VectorMaskL[(tmp.u32 & 0xF)]
	// 822F37DC: 7D3C4B78  mr r28, r9
	ctx.r[28].u64 = ctx.r[9].u64;
	// 822F37E0: 13E030C7  vcmpequd (lvx128) v31, v0, v6
	tmp.u32 = ctx.r[6].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 822F37E4: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 822F37E8: 39210060  addi r9, r1, 0x60
	ctx.r[9].s64 = ctx.r[1].s64 + 96;
	// 822F37EC: 7D1D4378  mr r29, r8
	ctx.r[29].u64 = ctx.r[8].u64;
	// 822F37F0: 3D008200  lis r8, -0x7e00
	ctx.r[8].s64 = -2113929216;
	// 822F37F4: 816B671C  lwz r11, 0x671c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(26396 as u32) ) } as u64;
	// 822F37F8: 3CE08201  lis r7, -0x7dff
	ctx.r[7].s64 = -2113863680;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822F38C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822F38C0 size=100
    let mut pc: u32 = 0x822F38C0;
    'dispatch: loop {
        match pc {
            0x822F38C0 => {
    //   block [0x822F38C0..0x822F3924)
	// 822F38C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822F38C4: 48EB48A9  bl 0x831a816c
	ctx.lr = 0x822F38C8;
	sub_831A8130(ctx, base);
	// 822F38C8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822F38CC: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 822F38D0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822F38D4: 83DD0000  lwz r30, 0(r29)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 822F38D8: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 822F38DC: 419A000C  beq cr6, 0x822f38e8
	if ctx.cr[6].eq {
	pc = 0x822F38E8; continue 'dispatch;
	}
	// 822F38E0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 822F38E4: 48BDAF1D  bl 0x82ece800
	ctx.lr = 0x822F38E8;
	sub_82ECE800(ctx, base);
	// 822F38E8: 809F00C4  lwz r4, 0xc4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(196 as u32) ) } as u64;
	// 822F38EC: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 822F38F0: 419A000C  beq cr6, 0x822f38fc
	if ctx.cr[6].eq {
	pc = 0x822F38FC; continue 'dispatch;
	}
	// 822F38F4: 807D0000  lwz r3, 0(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 822F38F8: 48BD9C79  bl 0x82ecd570
	ctx.lr = 0x822F38FC;
	sub_82ECD570(ctx, base);
	// 822F38FC: 807F00A8  lwz r3, 0xa8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(168 as u32) ) } as u64;
	// 822F3900: 48BD4791  bl 0x82ec8090
	ctx.lr = 0x822F3904;
	sub_82EC8090(ctx, base);
	// 822F3904: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 822F3908: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 822F390C: 917F00E0  stw r11, 0xe0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(224 as u32), ctx.r[11].u32 ) };
	// 822F3910: 419A000C  beq cr6, 0x822f391c
	if ctx.cr[6].eq {
	pc = 0x822F391C; continue 'dispatch;
	}
	// 822F3914: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 822F3918: 48BD96F1  bl 0x82ecd008
	ctx.lr = 0x822F391C;
	sub_82ECD008(ctx, base);
	// 822F391C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 822F3920: 48EB489C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822F3928(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822F3928 size=152
    let mut pc: u32 = 0x822F3928;
    'dispatch: loop {
        match pc {
            0x822F3928 => {
    //   block [0x822F3928..0x822F39C0)
	// 822F3928: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822F392C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822F3930: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 822F3934: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822F3938: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822F393C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822F3940: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 822F3944: 48BD5E2D  bl 0x82ec9770
	ctx.lr = 0x822F3948;
	sub_82EC9770(ctx, base);
	// 822F3948: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 822F394C: 41820058  beq 0x822f39a4
	if ctx.cr[0].eq {
	pc = 0x822F39A4; continue 'dispatch;
	}
	// 822F3950: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 822F3954: 419A0050  beq cr6, 0x822f39a4
	if ctx.cr[6].eq {
	pc = 0x822F39A4; continue 'dispatch;
	}
	// 822F3958: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 822F395C: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 822F3960: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 822F3964: 8163005C  lwz r11, 0x5c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(92 as u32) ) } as u64;
	// 822F3968: 81430034  lwz r10, 0x34(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(52 as u32) ) } as u64;
	// 822F396C: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 822F3970: 41980018  blt cr6, 0x822f3988
	if ctx.cr[6].lt {
	pc = 0x822F3988; continue 'dispatch;
	}
	// 822F3974: 38C00013  li r6, 0x13
	ctx.r[6].s64 = 19;
	// 822F3978: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 822F397C: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 822F3980: 48BAC6E1  bl 0x82ea0060
	ctx.lr = 0x822F3984;
	sub_82EA0060(ctx, base);
	// 822F3984: 48000020  b 0x822f39a4
	pc = 0x822F39A4; continue 'dispatch;
	// 822F3988: 8143005C  lwz r10, 0x5c(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(92 as u32) ) } as u64;
	// 822F398C: 39630058  addi r11, r3, 0x58
	ctx.r[11].s64 = ctx.r[3].s64 + 88;
	// 822F3990: 81630058  lwz r11, 0x58(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(88 as u32) ) } as u64;
	// 822F3994: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 822F3998: 9143005C  stw r10, 0x5c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(92 as u32), ctx.r[10].u32 ) };
	// 822F399C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 822F39A0: 93E30058  stw r31, 0x58(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(88 as u32), ctx.r[31].u32 ) };
	// 822F39A4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822F39A8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 822F39AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822F39B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822F39B4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 822F39B8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822F39BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822F39C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822F39C0 size=196
    let mut pc: u32 = 0x822F39C0;
    'dispatch: loop {
        match pc {
            0x822F39C0 => {
    //   block [0x822F39C0..0x822F3A84)
	// 822F39C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822F39C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822F39C8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 822F39CC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822F39D0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822F39D4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 822F39D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 822F39DC: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 822F39E0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 822F39E4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 822F39E8: 4BFCCF51  bl 0x822c0938
	ctx.lr = 0x822F39EC;
	sub_822C0938(ctx, base);
	// 822F39EC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 822F39F0: 41820028  beq 0x822f3a18
	if ctx.cr[0].eq {
	pc = 0x822F3A18; continue 'dispatch;
	}
	// 822F39F4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 822F39F8: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 822F39FC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 822F3A00: 392BC574  addi r9, r11, -0x3a8c
	ctx.r[9].s64 = ctx.r[11].s64 + -14988;
	// 822F3A04: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 822F3A08: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 822F3A0C: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 822F3A10: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 822F3A14: 48000008  b 0x822f3a1c
	pc = 0x822F3A1C; continue 'dispatch;
	// 822F3A18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 822F3A1C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 822F3A20: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 822F3A24: 409A0044  bne cr6, 0x822f3a68
	if !ctx.cr[6].eq {
	pc = 0x822F3A68; continue 'dispatch;
	}
	// 822F3A28: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 822F3A2C: 419A001C  beq cr6, 0x822f3a48
	if ctx.cr[6].eq {
	pc = 0x822F3A48; continue 'dispatch;
	}
	// 822F3A30: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 822F3A34: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 822F3A38: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822F3A3C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 822F3A40: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 822F3A44: 4E800421  bctrl
	ctx.lr = 0x822F3A48;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 822F3A48: 3D608325  lis r11, -0x7cdb
	ctx.r[11].s64 = -2094727168;
	// 822F3A4C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 822F3A50: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 822F3A54: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 822F3A58: 816B4AF4  lwz r11, 0x4af4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(19188 as u32) ) } as u64;
	// 822F3A5C: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 822F3A60: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 822F3A64: 4BFCC59D  bl 0x822c0000
	ctx.lr = 0x822F3A68;
	sub_822C0000(ctx, base);
	// 822F3A68: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 822F3A6C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 822F3A70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822F3A74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822F3A78: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 822F3A7C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822F3A80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822F3A88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822F3A88 size=152
    let mut pc: u32 = 0x822F3A88;
    'dispatch: loop {
        match pc {
            0x822F3A88 => {
    //   block [0x822F3A88..0x822F3B20)
	// 822F3A88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822F3A8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822F3A90: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 822F3A94: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822F3A98: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822F3A9C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822F3AA0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 822F3AA4: 48BD6735  bl 0x82eca1d8
	ctx.lr = 0x822F3AA8;
	sub_82ECA1D8(ctx, base);
	// 822F3AA8: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 822F3AAC: 41820058  beq 0x822f3b04
	if ctx.cr[0].eq {
	pc = 0x822F3B04; continue 'dispatch;
	}
	// 822F3AB0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 822F3AB4: 419A0050  beq cr6, 0x822f3b04
	if ctx.cr[6].eq {
	pc = 0x822F3B04; continue 'dispatch;
	}
	// 822F3AB8: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 822F3ABC: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 822F3AC0: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 822F3AC4: 8163006C  lwz r11, 0x6c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(108 as u32) ) } as u64;
	// 822F3AC8: 81430034  lwz r10, 0x34(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(52 as u32) ) } as u64;
	// 822F3ACC: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 822F3AD0: 41980018  blt cr6, 0x822f3ae8
	if ctx.cr[6].lt {
	pc = 0x822F3AE8; continue 'dispatch;
	}
	// 822F3AD4: 38C0000E  li r6, 0xe
	ctx.r[6].s64 = 14;
	// 822F3AD8: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 822F3ADC: 38800006  li r4, 6
	ctx.r[4].s64 = 6;
	// 822F3AE0: 48BAC581  bl 0x82ea0060
	ctx.lr = 0x822F3AE4;
	sub_82EA0060(ctx, base);
	// 822F3AE4: 48000020  b 0x822f3b04
	pc = 0x822F3B04; continue 'dispatch;
	// 822F3AE8: 8143006C  lwz r10, 0x6c(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(108 as u32) ) } as u64;
	// 822F3AEC: 39630068  addi r11, r3, 0x68
	ctx.r[11].s64 = ctx.r[3].s64 + 104;
	// 822F3AF0: 81630068  lwz r11, 0x68(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(104 as u32) ) } as u64;
	// 822F3AF4: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 822F3AF8: 9143006C  stw r10, 0x6c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(108 as u32), ctx.r[10].u32 ) };
	// 822F3AFC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 822F3B00: 93E30068  stw r31, 0x68(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(104 as u32), ctx.r[31].u32 ) };
	// 822F3B04: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822F3B08: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 822F3B0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822F3B10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822F3B14: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 822F3B18: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822F3B1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822F3B20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822F3B20 size=120
    let mut pc: u32 = 0x822F3B20;
    'dispatch: loop {
        match pc {
            0x822F3B20 => {
    //   block [0x822F3B20..0x822F3B98)
	// 822F3B20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822F3B24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822F3B28: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822F3B2C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822F3B30: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822F3B34: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 822F3B38: 397F0008  addi r11, r31, 8
	ctx.r[11].s64 = ctx.r[31].s64 + 8;
	// 822F3B3C: 409A0008  bne cr6, 0x822f3b44
	if !ctx.cr[6].eq {
	pc = 0x822F3B44; continue 'dispatch;
	}
	// 822F3B40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 822F3B44: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 822F3B48: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 822F3B4C: 394AC540  addi r10, r10, -0x3ac0
	ctx.r[10].s64 = ctx.r[10].s64 + -15040;
	// 822F3B50: 39299EAC  addi r9, r9, -0x6154
	ctx.r[9].s64 = ctx.r[9].s64 + -24916;
	// 822F3B54: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 822F3B58: 548807FF  clrlwi. r8, r4, 0x1f
	ctx.r[8].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 822F3B5C: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 822F3B60: 41820020  beq 0x822f3b80
	if ctx.cr[0].eq {
	pc = 0x822F3B80; continue 'dispatch;
	}
	// 822F3B64: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 822F3B68: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 822F3B6C: 38C00013  li r6, 0x13
	ctx.r[6].s64 = 19;
	// 822F3B70: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 822F3B74: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 822F3B78: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 822F3B7C: 48BACC35  bl 0x82ea07b0
	ctx.lr = 0x822F3B80;
	sub_82EA07B0(ctx, base);
	// 822F3B80: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822F3B84: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 822F3B88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822F3B8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822F3B90: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822F3B94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822F3B98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822F3B98 size=208
    let mut pc: u32 = 0x822F3B98;
    'dispatch: loop {
        match pc {
            0x822F3B98 => {
    //   block [0x822F3B98..0x822F3C68)
	// 822F3B98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822F3B9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822F3BA0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822F3BA4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822F3BA8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822F3BAC: 817F0034  lwz r11, 0x34(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 822F3BB0: 556A0001  rlwinm. r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 822F3BB4: 40820020  bne 0x822f3bd4
	if !ctx.cr[0].eq {
	pc = 0x822F3BD4; continue 'dispatch;
	}
	// 822F3BB8: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 822F3BBC: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 822F3BC0: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 822F3BC4: 809F002C  lwz r4, 0x2c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 822F3BC8: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 822F3BCC: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 822F3BD0: 48BACBE1  bl 0x82ea07b0
	ctx.lr = 0x822F3BD4;
	sub_82EA07B0(ctx, base);
	// 822F3BD4: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 822F3BD8: 556A0001  rlwinm. r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 822F3BDC: 40820020  bne 0x822f3bfc
	if !ctx.cr[0].eq {
	pc = 0x822F3BFC; continue 'dispatch;
	}
	// 822F3BE0: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 822F3BE4: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 822F3BE8: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 822F3BEC: 809F001C  lwz r4, 0x1c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 822F3BF0: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 822F3BF4: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 822F3BF8: 48BACBB9  bl 0x82ea07b0
	ctx.lr = 0x822F3BFC;
	sub_82EA07B0(ctx, base);
	// 822F3BFC: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 822F3C00: 556A0001  rlwinm. r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 822F3C04: 40820024  bne 0x822f3c28
	if !ctx.cr[0].eq {
	pc = 0x822F3C28; continue 'dispatch;
	}
	// 822F3C08: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 822F3C0C: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 822F3C10: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 822F3C14: 809F0010  lwz r4, 0x10(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 822F3C18: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 822F3C1C: 1CAB0030  mulli r5, r11, 0x30
	ctx.r[5].s64 = ctx.r[11].s64 * 48;
	// 822F3C20: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 822F3C24: 48BACB8D  bl 0x82ea07b0
	ctx.lr = 0x822F3C28;
	sub_82EA07B0(ctx, base);
	// 822F3C28: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 822F3C2C: 556A0001  rlwinm. r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 822F3C30: 40820024  bne 0x822f3c54
	if !ctx.cr[0].eq {
	pc = 0x822F3C54; continue 'dispatch;
	}
	// 822F3C34: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 822F3C38: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 822F3C3C: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 822F3C40: 809F0004  lwz r4, 4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 822F3C44: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 822F3C48: 1CAB0030  mulli r5, r11, 0x30
	ctx.r[5].s64 = ctx.r[11].s64 * 48;
	// 822F3C4C: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 822F3C50: 48BACB61  bl 0x82ea07b0
	ctx.lr = 0x822F3C54;
	sub_82EA07B0(ctx, base);
	// 822F3C54: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 822F3C58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822F3C5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822F3C60: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822F3C64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822F3C68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x822F3C68 size=32
    let mut pc: u32 = 0x822F3C68;
    'dispatch: loop {
        match pc {
            0x822F3C68 => {
    //   block [0x822F3C68..0x822F3C88)
	// 822F3C68: 816400A8  lwz r11, 0xa8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(168 as u32) ) } as u64;
	// 822F3C6C: 54AA103A  slwi r10, r5, 2
	ctx.r[10].u32 = ctx.r[5].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 822F3C70: 812B0020  lwz r9, 0x20(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 822F3C74: 7D49502E  lwzx r10, r9, r10
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 822F3C78: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 822F3C7C: 4098000C  bge cr6, 0x822f3c88
	if !ctx.cr[6].lt {
		sub_822F3C88(ctx, base);
		return;
	}
	// 822F3C80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 822F3C84: 48000010  b 0x822f3c94
	sub_822F3C88(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


