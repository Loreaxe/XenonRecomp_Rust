pub fn sub_82409A60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82409A60 size=124
    let mut pc: u32 = 0x82409A60;
    'dispatch: loop {
        match pc {
            0x82409A60 => {
    //   block [0x82409A60..0x82409AA4)
	// 82409A60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82409A64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82409A68: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82409A6C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82409A70: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82409A74: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82409A78: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82409A7C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82409A80: 4BFFCEA1  bl 0x82406920
	ctx.lr = 0x82409A84;
	sub_82406920(ctx, base);
	// 82409A84: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82409A88: 4BFFD3D9  bl 0x82406e60
	ctx.lr = 0x82409A8C;
	sub_82406E60(ctx, base);
	// 82409A8C: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82409A90: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 82409A94: 409A0010  bne cr6, 0x82409aa4
	if !ctx.cr[6].eq {
	pc = 0x82409AA4; continue 'dispatch;
	}
	// 82409A98: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82409A9C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82409AA0: 4BFFFD69  bl 0x82409808
	ctx.lr = 0x82409AA4;
	sub_82409808(ctx, base);
	pc = 0x82409AA4; continue 'dispatch;
            }
            0x82409AA4 => {
    //   block [0x82409AA4..0x82409ADC)
	// 82409AA4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82409AA8: 807E1C3C  lwz r3, 0x1c3c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(7228 as u32) ) } as u64;
	// 82409AAC: 480016E5  bl 0x8240b190
	ctx.lr = 0x82409AB0;
	sub_8240B190(ctx, base);
	// 82409AB0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82409AB4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82409AB8: 4BFFECB9  bl 0x82408770
	ctx.lr = 0x82409ABC;
	sub_82408770(ctx, base);
	// 82409ABC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82409AC0: 93FE0000  stw r31, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82409AC4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82409AC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82409ACC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82409AD0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82409AD4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82409AD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82409AE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82409AE0 size=96
    let mut pc: u32 = 0x82409AE0;
    'dispatch: loop {
        match pc {
            0x82409AE0 => {
    //   block [0x82409AE0..0x82409B20)
	// 82409AE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82409AE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82409AE8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82409AEC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82409AF0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82409AF4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82409AF8: 807E1C4C  lwz r3, 0x1c4c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(7244 as u32) ) } as u64;
	// 82409AFC: 48005315  bl 0x8240ee10
	ctx.lr = 0x82409B00;
	sub_8240EE10(ctx, base);
	// 82409B00: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82409B04: 4182001C  beq 0x82409b20
	if ctx.cr[0].eq {
	pc = 0x82409B20; continue 'dispatch;
	}
	// 82409B08: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82409B0C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82409B10: 386BCECC  addi r3, r11, -0x3134
	ctx.r[3].s64 = ctx.r[11].s64 + -12596;
	// 82409B14: 4BEA946D  bl 0x822b2f80
	ctx.lr = 0x82409B18;
	sub_822B2F80(ctx, base);
	// 82409B18: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82409B1C: 4800000C  b 0x82409b28
	pc = 0x82409B28; continue 'dispatch;
            }
            0x82409B20 => {
    //   block [0x82409B20..0x82409B28)
	// 82409B20: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82409B24: 4BFFEE1D  bl 0x82408940
	ctx.lr = 0x82409B28;
	sub_82408940(ctx, base);
	pc = 0x82409B28; continue 'dispatch;
            }
            0x82409B28 => {
    //   block [0x82409B28..0x82409B40)
	// 82409B28: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82409B2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82409B30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82409B34: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82409B38: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82409B3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82409B40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82409B40 size=704
    let mut pc: u32 = 0x82409B40;
    'dispatch: loop {
        match pc {
            0x82409B40 => {
    //   block [0x82409B40..0x82409BA8)
	// 82409B40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82409B44: 4812B549  bl 0x8253508c
	ctx.lr = 0x82409B48;
	sub_82535080(ctx, base);
	// 82409B48: DBC1FF70  stfd f30, -0x90(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-144 as u32), ctx.f[30].u64 ) };
	// 82409B4C: DBE1FF78  stfd f31, -0x88(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-136 as u32), ctx.f[31].u64 ) };
	// 82409B50: 9421FEE0  stwu r1, -0x120(r1)
	ea = ctx.r[1].u32.wrapping_add(-288 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82409B54: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82409B58: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 82409B5C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82409B60: FFC01090  fmr f30, f2
	ctx.f[30].f64 = ctx.f[2].f64;
	// 82409B64: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 82409B68: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82409B6C: 7CD53378  mr r21, r6
	ctx.r[21].u64 = ctx.r[6].u64;
	// 82409B70: C00B1FF8  lfs f0, 0x1ff8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82409B74: 7D344B78  mr r20, r9
	ctx.r[20].u64 = ctx.r[9].u64;
	// 82409B78: 7D565378  mr r22, r10
	ctx.r[22].u64 = ctx.r[10].u64;
	// 82409B7C: 3A200000  li r17, 0
	ctx.r[17].s64 = 0;
	// 82409B80: FF1F0000  fcmpu cr6, f31, f0
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[0].f64);
	// 82409B84: 40980024  bge cr6, 0x82409ba8
	if !ctx.cr[6].lt {
	pc = 0x82409BA8; continue 'dispatch;
	}
	// 82409B88: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82409B8C: D8210018  stfd f1, 0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(24 as u32), ctx.f[1].u64 ) };
	// 82409B90: E8810018  ld r4, 0x18(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(24 as u32) ) };
	// 82409B94: 386BD028  addi r3, r11, -0x2fd8
	ctx.r[3].s64 = ctx.r[11].s64 + -12248;
	// 82409B98: 4BEA93E9  bl 0x822b2f80
	ctx.lr = 0x82409B9C;
	sub_822B2F80(ctx, base);
	// 82409B9C: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 82409BA0: 60630018  ori r3, r3, 0x18
	ctx.r[3].u64 = ctx.r[3].u64 | 24;
	// 82409BA4: 4800024C  b 0x82409df0
	pc = 0x82409DF0; continue 'dispatch;
            }
            0x82409BA8 => {
    //   block [0x82409BA8..0x82409BC0)
	// 82409BA8: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82409BAC: 3B400001  li r26, 1
	ctx.r[26].s64 = 1;
	// 82409BB0: 356B0001  addic. r11, r11, 1
	ctx.xer.ca = (ctx.r[11].u32 > (!(1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82409BB4: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82409BB8: 40820008  bne 0x82409bc0
	if !ctx.cr[0].eq {
	pc = 0x82409BC0; continue 'dispatch;
	}
	// 82409BBC: 935F0004  stw r26, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[26].u32 ) };
	pc = 0x82409BC0; continue 'dispatch;
            }
            0x82409BC0 => {
    //   block [0x82409BC0..0x82409C04)
	// 82409BC0: 8161017C  lwz r11, 0x17c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(380 as u32) ) } as u64;
	// 82409BC4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82409BC8: 419A0214  beq cr6, 0x82409ddc
	if ctx.cr[6].eq {
	pc = 0x82409DDC; continue 'dispatch;
	}
	// 82409BCC: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82409BD0: 38A10080  addi r5, r1, 0x80
	ctx.r[5].s64 = ctx.r[1].s64 + 128;
	// 82409BD4: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82409BD8: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82409BDC: 807F1C38  lwz r3, 0x1c38(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(7224 as u32) ) } as u64;
	// 82409BE0: 480028A1  bl 0x8240c480
	ctx.lr = 0x82409BE4;
	sub_8240C480(ctx, base);
	// 82409BE4: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82409BE8: 4182001C  beq 0x82409c04
	if ctx.cr[0].eq {
	pc = 0x82409C04; continue 'dispatch;
	}
	// 82409BEC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82409BF0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82409BF4: 386BCFE0  addi r3, r11, -0x3020
	ctx.r[3].s64 = ctx.r[11].s64 + -12320;
	// 82409BF8: 4BEA9389  bl 0x822b2f80
	ctx.lr = 0x82409BFC;
	sub_822B2F80(ctx, base);
	// 82409BFC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82409C00: 480001F0  b 0x82409df0
	pc = 0x82409DF0; continue 'dispatch;
            }
            0x82409C04 => {
    //   block [0x82409C04..0x82409C20)
	// 82409C04: 83010174  lwz r24, 0x174(r1)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(372 as u32) ) } as u64;
	// 82409C08: 7FBEEB78  mr r30, r29
	ctx.r[30].u64 = ctx.r[29].u64;
	// 82409C0C: 3B80FFFF  li r28, -1
	ctx.r[28].s64 = -1;
	// 82409C10: 2B180000  cmplwi cr6, r24, 0
	ctx.cr[6].compare_u32(ctx.r[24].u32, 0 as u32, &mut ctx.xer);
	// 82409C14: 419A000C  beq cr6, 0x82409c20
	if ctx.cr[6].eq {
	pc = 0x82409C20; continue 'dispatch;
	}
	// 82409C18: 2B160000  cmplwi cr6, r22, 0
	ctx.cr[6].compare_u32(ctx.r[22].u32, 0 as u32, &mut ctx.xer);
	// 82409C1C: 409A0008  bne cr6, 0x82409c24
	if !ctx.cr[6].eq {
	pc = 0x82409C24; continue 'dispatch;
	}
	pc = 0x82409C20; continue 'dispatch;
            }
            0x82409C20 => {
    //   block [0x82409C20..0x82409C24)
	// 82409C20: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	pc = 0x82409C24; continue 'dispatch;
            }
            0x82409C24 => {
    //   block [0x82409C24..0x82409C54)
	// 82409C24: 3D400008  lis r10, 8
	ctx.r[10].s64 = 524288;
	// 82409C28: 817F1C38  lwz r11, 0x1c38(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(7224 as u32) ) } as u64;
	// 82409C2C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82409C30: 80810080  lwz r4, 0x80(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(128 as u32) ) } as u64;
	// 82409C34: 6157030C  ori r23, r10, 0x30c
	ctx.r[23].u64 = ctx.r[10].u64 | 780;
	// 82409C38: 7C6BBA14  add r3, r11, r23
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[23].u64;
	// 82409C3C: 4BFFD00D  bl 0x82406c48
	ctx.lr = 0x82409C40;
	sub_82406C48(ctx, base);
	// 82409C40: 7C731B79  or. r19, r3, r3
	ctx.r[19].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[19].s32, 0, &mut ctx.xer);
	// 82409C44: 40820024  bne 0x82409c68
	if !ctx.cr[0].eq {
	pc = 0x82409C68; continue 'dispatch;
	}
	// 82409C48: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82409C4C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82409C50: 386BCF90  addi r3, r11, -0x3070
	ctx.r[3].s64 = ctx.r[11].s64 + -12400;
	pc = 0x82409C54; continue 'dispatch;
            }
            0x82409C54 => {
    //   block [0x82409C54..0x82409C68)
	// 82409C54: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82409C58: 4BEA9329  bl 0x822b2f80
	ctx.lr = 0x82409C5C;
	sub_822B2F80(ctx, base);
	// 82409C5C: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 82409C60: 6063000F  ori r3, r3, 0xf
	ctx.r[3].u64 = ctx.r[3].u64 | 15;
	// 82409C64: 4800018C  b 0x82409df0
	pc = 0x82409DF0; continue 'dispatch;
            }
            0x82409C68 => {
    //   block [0x82409C68..0x82409C7C)
	// 82409C68: 8173002C  lwz r11, 0x2c(r19)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(44 as u32) ) } as u64;
	// 82409C6C: 3A400000  li r18, 0
	ctx.r[18].s64 = 0;
	// 82409C70: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82409C74: 4198007C  blt cr6, 0x82409cf0
	if ctx.cr[6].lt {
	pc = 0x82409CF0; continue 'dispatch;
	}
	// 82409C78: 3B3F0218  addi r25, r31, 0x218
	ctx.r[25].s64 = ctx.r[31].s64 + 536;
	pc = 0x82409C7C; continue 'dispatch;
            }
            0x82409C7C => {
    //   block [0x82409C7C..0x82409CF0)
	// 82409C7C: 817F1C38  lwz r11, 0x1c38(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(7224 as u32) ) } as u64;
	// 82409C80: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82409C84: 80810080  lwz r4, 0x80(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(128 as u32) ) } as u64;
	// 82409C88: 7C6BBA14  add r3, r11, r23
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[23].u64;
	// 82409C8C: 4BFFCF7D  bl 0x82406c08
	ctx.lr = 0x82409C90;
	sub_82406C08(ctx, base);
	// 82409C90: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82409C94: 41820120  beq 0x82409db4
	if ctx.cr[0].eq {
	pc = 0x82409DB4; continue 'dispatch;
	}
	// 82409C98: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82409C9C: 7F88E378  mr r8, r28
	ctx.r[8].u64 = ctx.r[28].u64;
	// 82409CA0: 7EA7AB78  mr r7, r21
	ctx.r[7].u64 = ctx.r[21].u64;
	// 82409CA4: 80810080  lwz r4, 0x80(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(128 as u32) ) } as u64;
	// 82409CA8: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82409CAC: 93010074  stw r24, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[24].u32 ) };
	// 82409CB0: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82409CB4: 92C1006C  stw r22, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[22].u32 ) };
	// 82409CB8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82409CBC: 93410064  stw r26, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[26].u32 ) };
	// 82409CC0: 92810054  stw r20, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[20].u32 ) };
	// 82409CC4: FC40F090  fmr f2, f30
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[2].f64 = ctx.f[30].f64;
	// 82409CC8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82409CCC: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82409CD0: 4BFFEDB9  bl 0x82408a88
	ctx.lr = 0x82409CD4;
	sub_82408A88(ctx, base);
	// 82409CD4: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82409CD8: 40820028  bne 0x82409d00
	if !ctx.cr[0].eq {
	pc = 0x82409D00; continue 'dispatch;
	}
	// 82409CDC: 817D0048  lwz r11, 0x48(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(72 as u32) ) } as u64;
	// 82409CE0: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82409CE4: 419A000C  beq cr6, 0x82409cf0
	if ctx.cr[6].eq {
	pc = 0x82409CF0; continue 'dispatch;
	}
	// 82409CE8: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 82409CEC: 409A008C  bne cr6, 0x82409d78
	if !ctx.cr[6].eq {
	pc = 0x82409D78; continue 'dispatch;
	}
	pc = 0x82409CF0; continue 'dispatch;
            }
            0x82409CF0 => {
    //   block [0x82409CF0..0x82409D00)
	// 82409CF0: 81610184  lwz r11, 0x184(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(388 as u32) ) } as u64;
	// 82409CF4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82409CF8: 922B0000  stw r17, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[17].u32 ) };
	// 82409CFC: 480000F4  b 0x82409df0
	pc = 0x82409DF0; continue 'dispatch;
            }
            0x82409D00 => {
    //   block [0x82409D00..0x82409D54)
	// 82409D00: 2F110020  cmpwi cr6, r17, 0x20
	ctx.cr[6].compare_i32(ctx.r[17].s32, 32, &mut ctx.xer);
	// 82409D04: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82409D08: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82409D0C: 409800B8  bge cr6, 0x82409dc4
	if !ctx.cr[6].lt {
	pc = 0x82409DC4; continue 'dispatch;
	}
	// 82409D10: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82409D14: 7E8AA378  mr r10, r20
	ctx.r[10].u64 = ctx.r[20].u64;
	// 82409D18: 7EA7AB78  mr r7, r21
	ctx.r[7].u64 = ctx.r[21].u64;
	// 82409D1C: 80810080  lwz r4, 0x80(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(128 as u32) ) } as u64;
	// 82409D20: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82409D24: 93210074  stw r25, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[25].u32 ) };
	// 82409D28: 9301006C  stw r24, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[24].u32 ) };
	// 82409D2C: FC40F090  fmr f2, f30
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[2].f64 = ctx.f[30].f64;
	// 82409D30: 92C10064  stw r22, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[22].u32 ) };
	// 82409D34: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82409D38: 9341005C  stw r26, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[26].u32 ) };
	// 82409D3C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82409D40: 4BFFEEA1  bl 0x82408be0
	ctx.lr = 0x82409D44;
	sub_82408BE0(ctx, base);
	// 82409D44: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82409D48: 4082000C  bne 0x82409d54
	if !ctx.cr[0].eq {
	pc = 0x82409D54; continue 'dispatch;
	}
	// 82409D4C: 3A310001  addi r17, r17, 1
	ctx.r[17].s64 = ctx.r[17].s64 + 1;
	// 82409D50: 3B3900D0  addi r25, r25, 0xd0
	ctx.r[25].s64 = ctx.r[25].s64 + 208;
	pc = 0x82409D54; continue 'dispatch;
            }
            0x82409D54 => {
    //   block [0x82409D54..0x82409D68)
	// 82409D54: 817D0048  lwz r11, 0x48(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(72 as u32) ) } as u64;
	// 82409D58: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82409D5C: 409A000C  bne cr6, 0x82409d68
	if !ctx.cr[6].eq {
	pc = 0x82409D68; continue 'dispatch;
	}
	// 82409D60: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82409D64: 419AFF8C  beq cr6, 0x82409cf0
	if ctx.cr[6].eq {
	pc = 0x82409CF0; continue 'dispatch;
	}
	pc = 0x82409D68; continue 'dispatch;
            }
            0x82409D68 => {
    //   block [0x82409D68..0x82409D78)
	// 82409D68: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 82409D6C: 409A000C  bne cr6, 0x82409d78
	if !ctx.cr[6].eq {
	pc = 0x82409D78; continue 'dispatch;
	}
	// 82409D70: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82409D74: 409AFF7C  bne cr6, 0x82409cf0
	if !ctx.cr[6].eq {
	pc = 0x82409CF0; continue 'dispatch;
	}
	pc = 0x82409D78; continue 'dispatch;
            }
            0x82409D78 => {
    //   block [0x82409D78..0x82409D88)
	// 82409D78: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 82409D7C: 7FDCF378  mr r28, r30
	ctx.r[28].u64 = ctx.r[30].u64;
	// 82409D80: 419A0008  beq cr6, 0x82409d88
	if ctx.cr[6].eq {
	pc = 0x82409D88; continue 'dispatch;
	}
	// 82409D84: 3B80FFFF  li r28, -1
	ctx.r[28].s64 = -1;
	pc = 0x82409D88; continue 'dispatch;
            }
            0x82409D88 => {
    //   block [0x82409D88..0x82409D9C)
	// 82409D88: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82409D8C: 2C0B0000  cmpwi r11, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82409D90: 4180000C  blt 0x82409d9c
	if ctx.cr[0].lt {
	pc = 0x82409D9C; continue 'dispatch;
	}
	// 82409D94: 7D7E5B78  mr r30, r11
	ctx.r[30].u64 = ctx.r[11].u64;
	// 82409D98: 48000008  b 0x82409da0
	pc = 0x82409DA0; continue 'dispatch;
            }
            0x82409D9C => {
    //   block [0x82409D9C..0x82409DA0)
	// 82409D9C: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	pc = 0x82409DA0; continue 'dispatch;
            }
            0x82409DA0 => {
    //   block [0x82409DA0..0x82409DB4)
	// 82409DA0: 8173002C  lwz r11, 0x2c(r19)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(44 as u32) ) } as u64;
	// 82409DA4: 3A520001  addi r18, r18, 1
	ctx.r[18].s64 = ctx.r[18].s64 + 1;
	// 82409DA8: 7F125800  cmpw cr6, r18, r11
	ctx.cr[6].compare_i32(ctx.r[18].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82409DAC: 4099FED0  ble cr6, 0x82409c7c
	if !ctx.cr[6].gt {
	pc = 0x82409C7C; continue 'dispatch;
	}
	// 82409DB0: 4BFFFF40  b 0x82409cf0
	pc = 0x82409CF0; continue 'dispatch;
            }
            0x82409DB4 => {
    //   block [0x82409DB4..0x82409DC4)
	// 82409DB4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82409DB8: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82409DBC: 386BCDB8  addi r3, r11, -0x3248
	ctx.r[3].s64 = ctx.r[11].s64 + -12872;
	// 82409DC0: 4BFFFE94  b 0x82409c54
	pc = 0x82409C54; continue 'dispatch;
            }
            0x82409DC4 => {
    //   block [0x82409DC4..0x82409DDC)
	// 82409DC4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82409DC8: 7E248B78  mr r4, r17
	ctx.r[4].u64 = ctx.r[17].u64;
	// 82409DCC: 386BCF40  addi r3, r11, -0x30c0
	ctx.r[3].s64 = ctx.r[11].s64 + -12480;
	// 82409DD0: 4BEA91B1  bl 0x822b2f80
	ctx.lr = 0x82409DD4;
	sub_822B2F80(ctx, base);
	// 82409DD4: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 82409DD8: 48000018  b 0x82409df0
	pc = 0x82409DF0; continue 'dispatch;
            }
            0x82409DDC => {
    //   block [0x82409DDC..0x82409DF0)
	// 82409DDC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82409DE0: 386BCF08  addi r3, r11, -0x30f8
	ctx.r[3].s64 = ctx.r[11].s64 + -12536;
	// 82409DE4: 4BEA919D  bl 0x822b2f80
	ctx.lr = 0x82409DE8;
	sub_822B2F80(ctx, base);
	// 82409DE8: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 82409DEC: 60630015  ori r3, r3, 0x15
	ctx.r[3].u64 = ctx.r[3].u64 | 21;
	pc = 0x82409DF0; continue 'dispatch;
            }
            0x82409DF0 => {
    //   block [0x82409DF0..0x82409E00)
	// 82409DF0: 38210120  addi r1, r1, 0x120
	ctx.r[1].s64 = ctx.r[1].s64 + 288;
	// 82409DF4: CBC1FF70  lfd f30, -0x90(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-144 as u32) ) };
	// 82409DF8: CBE1FF78  lfd f31, -0x88(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-136 as u32) ) };
	// 82409DFC: 4812B2E0  b 0x825350dc
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82409E00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82409E00 size=104
    let mut pc: u32 = 0x82409E00;
    'dispatch: loop {
        match pc {
            0x82409E00 => {
    //   block [0x82409E00..0x82409E48)
	// 82409E00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82409E04: 4812B2B9  bl 0x825350bc
	ctx.lr = 0x82409E08;
	sub_82535080(ctx, base);
	// 82409E08: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82409E0C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82409E10: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82409E14: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82409E18: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82409E1C: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82409E20: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82409E24: 4BFFFCBD  bl 0x82409ae0
	ctx.lr = 0x82409E28;
	sub_82409AE0(ctx, base);
	// 82409E28: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82409E2C: 4182001C  beq 0x82409e48
	if ctx.cr[0].eq {
	pc = 0x82409E48; continue 'dispatch;
	}
	// 82409E30: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82409E34: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82409E38: 386BD058  addi r3, r11, -0x2fa8
	ctx.r[3].s64 = ctx.r[11].s64 + -12200;
	// 82409E3C: 4BEA9145  bl 0x822b2f80
	ctx.lr = 0x82409E40;
	sub_822B2F80(ctx, base);
	// 82409E40: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82409E44: 4800001C  b 0x82409e60
	pc = 0x82409E60; continue 'dispatch;
            }
            0x82409E48 => {
    //   block [0x82409E48..0x82409E60)
	// 82409E48: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82409E4C: 807F1C38  lwz r3, 0x1c38(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(7224 as u32) ) } as u64;
	// 82409E50: 48002059  bl 0x8240bea8
	ctx.lr = 0x82409E54;
	sub_8240BEA8(ctx, base);
	// 82409E54: 807F1C50  lwz r3, 0x1c50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(7248 as u32) ) } as u64;
	// 82409E58: 48004E91  bl 0x8240ece8
	ctx.lr = 0x82409E5C;
	sub_8240ECE8(ctx, base);
	// 82409E5C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	pc = 0x82409E60; continue 'dispatch;
            }
            0x82409E60 => {
    //   block [0x82409E60..0x82409E68)
	// 82409E60: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82409E64: 4812B2A8  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82409E68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82409E68 size=72
    let mut pc: u32 = 0x82409E68;
    'dispatch: loop {
        match pc {
            0x82409E68 => {
    //   block [0x82409E68..0x82409EB0)
	// 82409E68: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 82409E6C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82409E70: C00A1FF8  lfs f0, 0x1ff8(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8184 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82409E74: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 82409E78: D003002C  stfs f0, 0x2c(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), tmp.u32 ) };
	// 82409E7C: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82409E80: D0030030  stfs f0, 0x30(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(48 as u32), tmp.u32 ) };
	// 82409E84: 91630028  stw r11, 0x28(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 82409E88: 91630018  stw r11, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82409E8C: 91630020  stw r11, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82409E90: 91430010  stw r10, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 82409E94: 91630024  stw r11, 0x24(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 82409E98: 9163001C  stw r11, 0x1c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82409E9C: 91430034  stw r10, 0x34(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(52 as u32), ctx.r[10].u32 ) };
	// 82409EA0: 91430014  stw r10, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 82409EA4: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82409EA8: 9963000C  stb r11, 0xc(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u8 ) };
	// 82409EAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82409EB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82409EB0 size=68
    let mut pc: u32 = 0x82409EB0;
    'dispatch: loop {
        match pc {
            0x82409EB0 => {
    //   block [0x82409EB0..0x82409EF4)
	// 82409EB0: 81240000  lwz r9, 0(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82409EB4: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82409EB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82409EBC: 3900FFFF  li r8, -1
	ctx.r[8].s64 = -1;
	// 82409EC0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82409EC4: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82409EC8: 914B0028  stw r10, 0x28(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(40 as u32), ctx.r[10].u32 ) };
	// 82409ECC: 914B0018  stw r10, 0x18(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[10].u32 ) };
	// 82409ED0: 914B0020  stw r10, 0x20(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 82409ED4: 914B0024  stw r10, 0x24(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(36 as u32), ctx.r[10].u32 ) };
	// 82409ED8: 914B001C  stw r10, 0x1c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(28 as u32), ctx.r[10].u32 ) };
	// 82409EDC: 910B0014  stw r8, 0x14(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[8].u32 ) };
	// 82409EE0: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82409EE4: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82409EE8: 994B000C  stb r10, 0xc(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u8 ) };
	// 82409EEC: 90AB0010  stw r5, 0x10(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[5].u32 ) };
	// 82409EF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82409EF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82409EF8 size=20
    let mut pc: u32 = 0x82409EF8;
    'dispatch: loop {
        match pc {
            0x82409EF8 => {
    //   block [0x82409EF8..0x82409F0C)
	// 82409EF8: 81630028  lwz r11, 0x28(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 82409EFC: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82409F00: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82409F04: 5563DFFE  rlwinm r3, r11, 0x1b, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82409F08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82409F10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82409F10 size=104
    let mut pc: u32 = 0x82409F10;
    'dispatch: loop {
        match pc {
            0x82409F10 => {
    //   block [0x82409F10..0x82409F4C)
	// 82409F10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82409F14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82409F18: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82409F1C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82409F20: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82409F24: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82409F28: 817F0028  lwz r11, 0x28(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82409F2C: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 82409F30: 409A0030  bne cr6, 0x82409f60
	if !ctx.cr[6].eq {
	pc = 0x82409F60; continue 'dispatch;
	}
	// 82409F34: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82409F38: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82409F3C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82409F40: 4182000C  beq 0x82409f4c
	if ctx.cr[0].eq {
	pc = 0x82409F4C; continue 'dispatch;
	}
	// 82409F44: 48179C65  bl 0x82583ba8
	ctx.lr = 0x82409F48;
	sub_82583BA8(ctx, base);
	// 82409F48: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	pc = 0x82409F4C; continue 'dispatch;
            }
            0x82409F4C => {
    //   block [0x82409F4C..0x82409F60)
	// 82409F4C: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82409F50: 93DF0008  stw r30, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 82409F54: 9BDF000C  stb r30, 0xc(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[30].u8 ) };
	// 82409F58: 93DF0028  stw r30, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[30].u32 ) };
	// 82409F5C: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	pc = 0x82409F60; continue 'dispatch;
            }
            0x82409F60 => {
    //   block [0x82409F60..0x82409F78)
	// 82409F60: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82409F64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82409F68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82409F6C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82409F70: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82409F74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82409F78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82409F78 size=100
    let mut pc: u32 = 0x82409F78;
    'dispatch: loop {
        match pc {
            0x82409F78 => {
    //   block [0x82409F78..0x82409F98)
	// 82409F78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82409F7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82409F80: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82409F84: 81630028  lwz r11, 0x28(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 82409F88: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 82409F8C: 419A000C  beq cr6, 0x82409f98
	if ctx.cr[6].eq {
	pc = 0x82409F98; continue 'dispatch;
	}
	// 82409F90: 3860FFFE  li r3, -2
	ctx.r[3].s64 = -2;
	// 82409F94: 48000038  b 0x82409fcc
	pc = 0x82409FCC; continue 'dispatch;
            }
            0x82409F98 => {
    //   block [0x82409F98..0x82409FC8)
	// 82409F98: 80630004  lwz r3, 4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82409F9C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82409FA0: 41820028  beq 0x82409fc8
	if ctx.cr[0].eq {
	pc = 0x82409FC8; continue 'dispatch;
	}
	// 82409FA4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82409FA8: 48179EF9  bl 0x82583ea0
	ctx.lr = 0x82409FAC;
	sub_82583EA0(ctx, base);
	// 82409FAC: 7C641B79  or. r4, r3, r3
	ctx.r[4].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 82409FB0: 40800018  bge 0x82409fc8
	if !ctx.cr[0].lt {
	pc = 0x82409FC8; continue 'dispatch;
	}
	// 82409FB4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82409FB8: 386BD090  addi r3, r11, -0x2f70
	ctx.r[3].s64 = ctx.r[11].s64 + -12144;
	// 82409FBC: 4BEA8FC5  bl 0x822b2f80
	ctx.lr = 0x82409FC0;
	sub_822B2F80(ctx, base);
	// 82409FC0: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82409FC4: 48000008  b 0x82409fcc
	pc = 0x82409FCC; continue 'dispatch;
            }
            0x82409FC8 => {
    //   block [0x82409FC8..0x82409FCC)
	// 82409FC8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	pc = 0x82409FCC; continue 'dispatch;
            }
            0x82409FCC => {
    //   block [0x82409FCC..0x82409FDC)
	// 82409FCC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82409FD0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82409FD4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82409FD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82409FE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82409FE0 size=24
    let mut pc: u32 = 0x82409FE0;
    'dispatch: loop {
        match pc {
            0x82409FE0 => {
    //   block [0x82409FE0..0x82409FF8)
	// 82409FE0: 81630028  lwz r11, 0x28(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 82409FE4: 3860FFFE  li r3, -2
	ctx.r[3].s64 = -2;
	// 82409FE8: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 82409FEC: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
	// 82409FF0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82409FF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82409FF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82409FF8 size=92
    let mut pc: u32 = 0x82409FF8;
    'dispatch: loop {
        match pc {
            0x82409FF8 => {
    //   block [0x82409FF8..0x8240A020)
	// 82409FF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82409FFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240A000: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8240A004: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240A008: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8240A00C: 817F0028  lwz r11, 0x28(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 8240A010: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 8240A014: 419A000C  beq cr6, 0x8240a020
	if ctx.cr[6].eq {
	pc = 0x8240A020; continue 'dispatch;
	}
	// 8240A018: 3860FFFE  li r3, -2
	ctx.r[3].s64 = -2;
	// 8240A01C: 48000024  b 0x8240a040
	pc = 0x8240A040; continue 'dispatch;
            }
            0x8240A020 => {
    //   block [0x8240A020..0x8240A034)
	// 8240A020: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8240A024: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8240A028: 4182000C  beq 0x8240a034
	if ctx.cr[0].eq {
	pc = 0x8240A034; continue 'dispatch;
	}
	// 8240A02C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8240A030: 48179EB9  bl 0x82583ee8
	ctx.lr = 0x8240A034;
	sub_82583EE8(ctx, base);
	pc = 0x8240A034; continue 'dispatch;
            }
            0x8240A034 => {
    //   block [0x8240A034..0x8240A040)
	// 8240A034: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8240A038: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8240A03C: 917F0024  stw r11, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	pc = 0x8240A040; continue 'dispatch;
            }
            0x8240A040 => {
    //   block [0x8240A040..0x8240A054)
	// 8240A040: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8240A044: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240A048: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240A04C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8240A050: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240A058(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8240A058 size=112
    let mut pc: u32 = 0x8240A058;
    'dispatch: loop {
        match pc {
            0x8240A058 => {
    //   block [0x8240A058..0x8240A080)
	// 8240A058: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240A05C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240A060: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8240A064: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240A068: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8240A06C: 817F0028  lwz r11, 0x28(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 8240A070: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 8240A074: 419A000C  beq cr6, 0x8240a080
	if ctx.cr[6].eq {
	pc = 0x8240A080; continue 'dispatch;
	}
	// 8240A078: 3860FFFE  li r3, -2
	ctx.r[3].s64 = -2;
	// 8240A07C: 48000038  b 0x8240a0b4
	pc = 0x8240A0B4; continue 'dispatch;
            }
            0x8240A080 => {
    //   block [0x8240A080..0x8240A094)
	// 8240A080: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 8240A084: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 8240A088: 409A000C  bne cr6, 0x8240a094
	if !ctx.cr[6].eq {
	pc = 0x8240A094; continue 'dispatch;
	}
	// 8240A08C: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 8240A090: 48000024  b 0x8240a0b4
	pc = 0x8240A0B4; continue 'dispatch;
            }
            0x8240A094 => {
    //   block [0x8240A094..0x8240A0A8)
	// 8240A094: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8240A098: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8240A09C: 4182000C  beq 0x8240a0a8
	if ctx.cr[0].eq {
	pc = 0x8240A0A8; continue 'dispatch;
	}
	// 8240A0A0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8240A0A4: 48179E45  bl 0x82583ee8
	ctx.lr = 0x8240A0A8;
	sub_82583EE8(ctx, base);
	pc = 0x8240A0A8; continue 'dispatch;
            }
            0x8240A0A8 => {
    //   block [0x8240A0A8..0x8240A0B4)
	// 8240A0A8: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8240A0AC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8240A0B0: 917F0024  stw r11, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	pc = 0x8240A0B4; continue 'dispatch;
            }
            0x8240A0B4 => {
    //   block [0x8240A0B4..0x8240A0C8)
	// 8240A0B4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8240A0B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240A0BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240A0C0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8240A0C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240A0C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8240A0C8 size=168
    let mut pc: u32 = 0x8240A0C8;
    'dispatch: loop {
        match pc {
            0x8240A0C8 => {
    //   block [0x8240A0C8..0x8240A0FC)
	// 8240A0C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240A0CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240A0D0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8240A0D4: DBC1FFE0  stfd f30, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[30].u64 ) };
	// 8240A0D8: DBE1FFE8  stfd f31, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.f[31].u64 ) };
	// 8240A0DC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240A0E0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8240A0E4: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 8240A0E8: 817F0028  lwz r11, 0x28(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 8240A0EC: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 8240A0F0: 419A000C  beq cr6, 0x8240a0fc
	if ctx.cr[6].eq {
	pc = 0x8240A0FC; continue 'dispatch;
	}
	// 8240A0F4: 3860FFFE  li r3, -2
	ctx.r[3].s64 = -2;
	// 8240A0F8: 4800005C  b 0x8240a154
	pc = 0x8240A154; continue 'dispatch;
            }
            0x8240A0FC => {
    //   block [0x8240A0FC..0x8240A140)
	// 8240A0FC: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8240A100: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8240A104: 419A004C  beq cr6, 0x8240a150
	if ctx.cr[6].eq {
	pc = 0x8240A150; continue 'dispatch;
	}
	// 8240A108: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8240A10C: C3CB1FF8  lfs f30, 0x1ff8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 8240A110: FF1FF000  fcmpu cr6, f31, f30
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[30].f64);
	// 8240A114: 4198002C  blt cr6, 0x8240a140
	if ctx.cr[6].lt {
	pc = 0x8240A140; continue 'dispatch;
	}
	// 8240A118: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 8240A11C: 4812A13D  bl 0x82534258
	ctx.lr = 0x8240A120;
	sub_82534258(ctx, base);
	// 8240A120: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8240A124: 4082001C  bne 0x8240a140
	if !ctx.cr[0].eq {
	pc = 0x8240A140; continue 'dispatch;
	}
	// 8240A128: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8240A12C: C00B1850  lfs f0, 0x1850(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(6224 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240A130: FF1F0000  fcmpu cr6, f31, f0
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[0].f64);
	// 8240A134: 40990010  ble cr6, 0x8240a144
	if !ctx.cr[6].gt {
	pc = 0x8240A144; continue 'dispatch;
	}
	// 8240A138: FFE00090  fmr f31, f0
	ctx.f[31].f64 = ctx.f[0].f64;
	// 8240A13C: 48000008  b 0x8240a144
	pc = 0x8240A144; continue 'dispatch;
            }
            0x8240A140 => {
    //   block [0x8240A140..0x8240A144)
	// 8240A140: FFE0F090  fmr f31, f30
	ctx.f[31].f64 = ctx.f[30].f64;
	pc = 0x8240A144; continue 'dispatch;
            }
            0x8240A144 => {
    //   block [0x8240A144..0x8240A150)
	// 8240A144: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8240A148: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 8240A14C: 48179EB5  bl 0x82584000
	ctx.lr = 0x8240A150;
	sub_82584000(ctx, base);
	pc = 0x8240A150; continue 'dispatch;
            }
            0x8240A150 => {
    //   block [0x8240A150..0x8240A154)
	// 8240A150: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	pc = 0x8240A154; continue 'dispatch;
            }
            0x8240A154 => {
    //   block [0x8240A154..0x8240A170)
	// 8240A154: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8240A158: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240A15C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240A160: CBC1FFE0  lfd f30, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 8240A164: CBE1FFE8  lfd f31, -0x18(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8240A168: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8240A16C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240A170(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8240A170 size=116
    let mut pc: u32 = 0x8240A170;
    'dispatch: loop {
        match pc {
            0x8240A170 => {
    //   block [0x8240A170..0x8240A190)
	// 8240A170: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240A174: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240A178: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240A17C: 81630028  lwz r11, 0x28(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 8240A180: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 8240A184: 419A000C  beq cr6, 0x8240a190
	if ctx.cr[6].eq {
	pc = 0x8240A190; continue 'dispatch;
	}
	// 8240A188: 3860FFFE  li r3, -2
	ctx.r[3].s64 = -2;
	// 8240A18C: 48000048  b 0x8240a1d4
	pc = 0x8240A1D4; continue 'dispatch;
            }
            0x8240A190 => {
    //   block [0x8240A190..0x8240A1BC)
	// 8240A190: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8240A194: C00B2698  lfs f0, 0x2698(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(9880 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240A198: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8240A19C: EC210032  fmuls f1, f1, f0
	ctx.f[1].f64 = (((ctx.f[1].f64 * ctx.f[0].f64) as f32) as f64);
	// 8240A1A0: C00B2068  lfs f0, 0x2068(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8296 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240A1A4: FF010000  fcmpu cr6, f1, f0
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 8240A1A8: 41990014  bgt cr6, 0x8240a1bc
	if ctx.cr[6].gt {
	pc = 0x8240A1BC; continue 'dispatch;
	}
	// 8240A1AC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240A1B0: C00BC808  lfs f0, -0x37f8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-14328 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240A1B4: FF010000  fcmpu cr6, f1, f0
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 8240A1B8: 40980008  bge cr6, 0x8240a1c0
	if !ctx.cr[6].lt {
	pc = 0x8240A1C0; continue 'dispatch;
	}
	pc = 0x8240A1BC; continue 'dispatch;
            }
            0x8240A1BC => {
    //   block [0x8240A1BC..0x8240A1C0)
	// 8240A1BC: FC200090  fmr f1, f0
	ctx.f[1].f64 = ctx.f[0].f64;
	pc = 0x8240A1C0; continue 'dispatch;
            }
            0x8240A1C0 => {
    //   block [0x8240A1C0..0x8240A1D0)
	// 8240A1C0: 80630004  lwz r3, 4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8240A1C4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8240A1C8: 41820008  beq 0x8240a1d0
	if ctx.cr[0].eq {
	pc = 0x8240A1D0; continue 'dispatch;
	}
	// 8240A1CC: 48179EAD  bl 0x82584078
	ctx.lr = 0x8240A1D0;
	sub_82584078(ctx, base);
	pc = 0x8240A1D0; continue 'dispatch;
            }
            0x8240A1D0 => {
    //   block [0x8240A1D0..0x8240A1D4)
	// 8240A1D0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	pc = 0x8240A1D4; continue 'dispatch;
            }
            0x8240A1D4 => {
    //   block [0x8240A1D4..0x8240A1E4)
	// 8240A1D4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8240A1D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240A1DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240A1E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240A1E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8240A1E8 size=376
    let mut pc: u32 = 0x8240A1E8;
    'dispatch: loop {
        match pc {
            0x8240A1E8 => {
    //   block [0x8240A1E8..0x8240A218)
	// 8240A1E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240A1EC: 4812AEB9  bl 0x825350a4
	ctx.lr = 0x8240A1F0;
	sub_82535080(ctx, base);
	// 8240A1F0: DBA1FF98  stfd f29, -0x68(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-104 as u32), ctx.f[29].u64 ) };
	// 8240A1F4: DBC1FFA0  stfd f30, -0x60(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-96 as u32), ctx.f[30].u64 ) };
	// 8240A1F8: DBE1FFA8  stfd f31, -0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-88 as u32), ctx.f[31].u64 ) };
	// 8240A1FC: 9421FDC0  stwu r1, -0x240(r1)
	ea = ctx.r[1].u32.wrapping_add(-576 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240A200: 7C771B78  mr r23, r3
	ctx.r[23].u64 = ctx.r[3].u64;
	// 8240A204: 81770028  lwz r11, 0x28(r23)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(40 as u32) ) } as u64;
	// 8240A208: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 8240A20C: 419A000C  beq cr6, 0x8240a218
	if ctx.cr[6].eq {
	pc = 0x8240A218; continue 'dispatch;
	}
	// 8240A210: 3860FFFE  li r3, -2
	ctx.r[3].s64 = -2;
	// 8240A214: 48000138  b 0x8240a34c
	pc = 0x8240A34C; continue 'dispatch;
            }
            0x8240A218 => {
    //   block [0x8240A218..0x8240A228)
	// 8240A218: 7CD93378  mr r25, r6
	ctx.r[25].u64 = ctx.r[6].u64;
	// 8240A21C: 2F060006  cmpwi cr6, r6, 6
	ctx.cr[6].compare_i32(ctx.r[6].s32, 6, &mut ctx.xer);
	// 8240A220: 40990008  ble cr6, 0x8240a228
	if !ctx.cr[6].gt {
	pc = 0x8240A228; continue 'dispatch;
	}
	// 8240A224: 3B200006  li r25, 6
	ctx.r[25].s64 = 6;
	pc = 0x8240A228; continue 'dispatch;
            }
            0x8240A228 => {
    //   block [0x8240A228..0x8240A254)
	// 8240A228: 2F190000  cmpwi cr6, r25, 0
	ctx.cr[6].compare_i32(ctx.r[25].s32, 0, &mut ctx.xer);
	// 8240A22C: 4099008C  ble cr6, 0x8240a2b8
	if !ctx.cr[6].gt {
	pc = 0x8240A2B8; continue 'dispatch;
	}
	// 8240A230: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 8240A234: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8240A238: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8240A23C: 3BE100B4  addi r31, r1, 0xb4
	ctx.r[31].s64 = ctx.r[1].s64 + 180;
	// 8240A240: 54B8103A  slwi r24, r5, 2
	ctx.r[24].u32 = ctx.r[5].u32.wrapping_shl(2);
	ctx.r[24].u64 = ctx.r[24].u32 as u64;
	// 8240A244: C3AA1850  lfs f29, 0x1850(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(6224 as u32) ) };
	ctx.f[29].f64 = (tmp.f32 as f64);
	// 8240A248: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8240A24C: C3CB1FF8  lfs f30, 0x1ff8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 8240A250: 7F3ACB78  mr r26, r25
	ctx.r[26].u64 = ctx.r[25].u64;
	pc = 0x8240A254; continue 'dispatch;
            }
            0x8240A254 => {
    //   block [0x8240A254..0x8240A25C)
	// 8240A254: 7FBBEB78  mr r27, r29
	ctx.r[27].u64 = ctx.r[29].u64;
	// 8240A258: 3B800006  li r28, 6
	ctx.r[28].s64 = 6;
	pc = 0x8240A25C; continue 'dispatch;
            }
            0x8240A25C => {
    //   block [0x8240A25C..0x8240A278)
	// 8240A25C: C3FB0000  lfs f31, 0(r27)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 8240A260: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 8240A264: 48129FF5  bl 0x82534258
	ctx.lr = 0x8240A268;
	sub_82534258(ctx, base);
	// 8240A268: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8240A26C: 4082000C  bne 0x8240a278
	if !ctx.cr[0].eq {
	pc = 0x8240A278; continue 'dispatch;
	}
	// 8240A270: FF1FF000  fcmpu cr6, f31, f30
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[30].f64);
	// 8240A274: 4098000C  bge cr6, 0x8240a280
	if !ctx.cr[6].lt {
	pc = 0x8240A280; continue 'dispatch;
	}
	pc = 0x8240A278; continue 'dispatch;
            }
            0x8240A278 => {
    //   block [0x8240A278..0x8240A280)
	// 8240A278: FFE0F090  fmr f31, f30
	ctx.f[31].f64 = ctx.f[30].f64;
	// 8240A27C: 48000010  b 0x8240a28c
	pc = 0x8240A28C; continue 'dispatch;
            }
            0x8240A280 => {
    //   block [0x8240A280..0x8240A28C)
	// 8240A280: FF1FE800  fcmpu cr6, f31, f29
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[29].f64);
	// 8240A284: 40990008  ble cr6, 0x8240a28c
	if !ctx.cr[6].gt {
	pc = 0x8240A28C; continue 'dispatch;
	}
	// 8240A288: FFE0E890  fmr f31, f29
	ctx.f[31].f64 = ctx.f[29].f64;
	pc = 0x8240A28C; continue 'dispatch;
            }
            0x8240A28C => {
    //   block [0x8240A28C..0x8240A2B8)
	// 8240A28C: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 8240A290: D3FF0000  stfs f31, 0(r31)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 8240A294: 379CFFFF  addic. r28, r28, -1
	ctx.xer.ca = (ctx.r[28].u32 > (!(-1 as u32)));
	ctx.r[28].s64 = ctx.r[28].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 8240A298: 3B7B0004  addi r27, r27, 4
	ctx.r[27].s64 = ctx.r[27].s64 + 4;
	// 8240A29C: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 8240A2A0: 997FFFFC  stb r11, -4(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(-4 as u32), ctx.r[11].u8 ) };
	// 8240A2A4: 3BFF0008  addi r31, r31, 8
	ctx.r[31].s64 = ctx.r[31].s64 + 8;
	// 8240A2A8: 4082FFB4  bne 0x8240a25c
	if !ctx.cr[0].eq {
	pc = 0x8240A25C; continue 'dispatch;
	}
	// 8240A2AC: 375AFFFF  addic. r26, r26, -1
	ctx.xer.ca = (ctx.r[26].u32 > (!(-1 as u32)));
	ctx.r[26].s64 = ctx.r[26].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[26].s32, 0, &mut ctx.xer);
	// 8240A2B0: 7FB8EA14  add r29, r24, r29
	ctx.r[29].u64 = ctx.r[24].u64 + ctx.r[29].u64;
	// 8240A2B4: 4082FFA0  bne 0x8240a254
	if !ctx.cr[0].eq {
	pc = 0x8240A254; continue 'dispatch;
	}
	pc = 0x8240A2B8; continue 'dispatch;
            }
            0x8240A2B8 => {
    //   block [0x8240A2B8..0x8240A2EC)
	// 8240A2B8: 1D790006  mulli r11, r25, 6
	ctx.r[11].s32 = ((ctx.r[25].s32 as i64 * 6 as i64) as i32);
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 8240A2BC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8240A2C0: 99610060  stb r11, 0x60(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u8 ) };
	// 8240A2C4: 396100B0  addi r11, r1, 0xb0
	ctx.r[11].s64 = ctx.r[1].s64 + 176;
	// 8240A2C8: 2F190000  cmpwi cr6, r25, 0
	ctx.cr[6].compare_i32(ctx.r[25].s32, 0, &mut ctx.xer);
	// 8240A2CC: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8240A2D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8240A2D4: 99610070  stb r11, 0x70(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[11].u8 ) };
	// 8240A2D8: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 8240A2DC: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8240A2E0: 40990028  ble cr6, 0x8240a308
	if !ctx.cr[6].gt {
	pc = 0x8240A308; continue 'dispatch;
	}
	// 8240A2E4: C017002C  lfs f0, 0x2c(r23)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(44 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240A2E8: 39610084  addi r11, r1, 0x84
	ctx.r[11].s64 = ctx.r[1].s64 + 132;
	pc = 0x8240A2EC; continue 'dispatch;
            }
            0x8240A2EC => {
    //   block [0x8240A2EC..0x8240A308)
	// 8240A2EC: 7D495378  mr r9, r10
	ctx.r[9].u64 = ctx.r[10].u64;
	// 8240A2F0: D00B0000  stfs f0, 0(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 8240A2F4: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8240A2F8: 7F0AC800  cmpw cr6, r10, r25
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[25].s32, &mut ctx.xer);
	// 8240A2FC: 992BFFFC  stb r9, -4(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(-4 as u32), ctx.r[9].u8 ) };
	// 8240A300: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 8240A304: 4198FFE8  blt cr6, 0x8240a2ec
	if ctx.cr[6].lt {
	pc = 0x8240A2EC; continue 'dispatch;
	}
	pc = 0x8240A308; continue 'dispatch;
            }
            0x8240A308 => {
    //   block [0x8240A308..0x8240A348)
	// 8240A308: 39610080  addi r11, r1, 0x80
	ctx.r[11].s64 = ctx.r[1].s64 + 128;
	// 8240A30C: 80770004  lwz r3, 4(r23)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(4 as u32) ) } as u64;
	// 8240A310: 9B210058  stb r25, 0x58(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[25].u8 ) };
	// 8240A314: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8240A318: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8240A31C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8240A320: 99610078  stb r11, 0x78(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[11].u8 ) };
	// 8240A324: 39610058  addi r11, r1, 0x58
	ctx.r[11].s64 = ctx.r[1].s64 + 88;
	// 8240A328: 9161007C  stw r11, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[11].u32 ) };
	// 8240A32C: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 8240A330: 99610050  stb r11, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u8 ) };
	// 8240A334: 39610070  addi r11, r1, 0x70
	ctx.r[11].s64 = ctx.r[1].s64 + 112;
	// 8240A338: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8240A33C: 4182000C  beq 0x8240a348
	if ctx.cr[0].eq {
	pc = 0x8240A348; continue 'dispatch;
	}
	// 8240A340: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8240A344: 48179995  bl 0x82583cd8
	ctx.lr = 0x8240A348;
	sub_82583CD8(ctx, base);
	pc = 0x8240A348; continue 'dispatch;
            }
            0x8240A348 => {
    //   block [0x8240A348..0x8240A34C)
	// 8240A348: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	pc = 0x8240A34C; continue 'dispatch;
            }
            0x8240A34C => {
    //   block [0x8240A34C..0x8240A360)
	// 8240A34C: 38210240  addi r1, r1, 0x240
	ctx.r[1].s64 = ctx.r[1].s64 + 576;
	// 8240A350: CBA1FF98  lfd f29, -0x68(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[29].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-104 as u32) ) };
	// 8240A354: CBC1FFA0  lfd f30, -0x60(r1)
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-96 as u32) ) };
	// 8240A358: CBE1FFA8  lfd f31, -0x58(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-88 as u32) ) };
	// 8240A35C: 4812AD98  b 0x825350f4
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240A360(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8240A360 size=24
    let mut pc: u32 = 0x8240A360;
    'dispatch: loop {
        match pc {
            0x8240A360 => {
    //   block [0x8240A360..0x8240A378)
	// 8240A360: 8963000C  lbz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 8240A364: 556B07BE  clrlwi r11, r11, 0x1e
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000003u64;
	// 8240A368: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 8240A36C: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8240A370: 69630001  xori r3, r11, 1
	ctx.r[3].u64 = ctx.r[11].u64 ^ 1;
	// 8240A374: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240A378(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8240A378 size=252
    let mut pc: u32 = 0x8240A378;
    'dispatch: loop {
        match pc {
            0x8240A378 => {
    //   block [0x8240A378..0x8240A3CC)
	// 8240A378: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240A37C: 4812AD3D  bl 0x825350b8
	ctx.lr = 0x8240A380;
	sub_82535080(ctx, base);
	// 8240A380: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240A384: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8240A388: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 8240A38C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8240A390: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8240A394: 38A00054  li r5, 0x54
	ctx.r[5].s64 = 84;
	// 8240A398: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8240A39C: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8240A3A0: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8240A3A4: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 8240A3A8: 4812AE29  bl 0x825351d0
	ctx.lr = 0x8240A3AC;
	sub_825351D0(ctx, base);
	// 8240A3AC: 57EB057F  clrlwi. r11, r31, 0x15
	ctx.r[11].u64 = ctx.r[31].u32 as u64 & 0x000007FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8240A3B0: 4182001C  beq 0x8240a3cc
	if ctx.cr[0].eq {
	pc = 0x8240A3CC; continue 'dispatch;
	}
	// 8240A3B4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240A3B8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8240A3BC: 386BD118  addi r3, r11, -0x2ee8
	ctx.r[3].s64 = ctx.r[11].s64 + -12008;
	// 8240A3C0: 4BEA8BC1  bl 0x822b2f80
	ctx.lr = 0x8240A3C4;
	sub_822B2F80(ctx, base);
	// 8240A3C4: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 8240A3C8: 480000A4  b 0x8240a46c
	pc = 0x8240A46C; continue 'dispatch;
            }
            0x8240A3CC => {
    //   block [0x8240A3CC..0x8240A3E0)
	// 8240A3CC: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8240A3D0: 409A0018  bne cr6, 0x8240a3e8
	if !ctx.cr[6].eq {
	pc = 0x8240A3E8; continue 'dispatch;
	}
	// 8240A3D4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240A3D8: 386BD0E0  addi r3, r11, -0x2f20
	ctx.r[3].s64 = ctx.r[11].s64 + -12064;
	// 8240A3DC: 4BEA8BA5  bl 0x822b2f80
	ctx.lr = 0x8240A3E0;
	sub_822B2F80(ctx, base);
	pc = 0x8240A3E0; continue 'dispatch;
            }
            0x8240A3E0 => {
    //   block [0x8240A3E0..0x8240A3E8)
	// 8240A3E0: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 8240A3E4: 48000088  b 0x8240a46c
	pc = 0x8240A46C; continue 'dispatch;
            }
            0x8240A3E8 => {
    //   block [0x8240A3E8..0x8240A40C)
	// 8240A3E8: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 8240A3EC: A13E0008  lhz r9, 8(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8240A3F0: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 8240A3F4: 93A10054  stw r29, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[29].u32 ) };
	// 8240A3F8: 2C090000  cmpwi r9, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8240A3FC: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 8240A400: 40810040  ble 0x8240a440
	if !ctx.cr[0].gt {
	pc = 0x8240A440; continue 'dispatch;
	}
	// 8240A404: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 8240A408: 395E001C  addi r10, r30, 0x1c
	ctx.r[10].s64 = ctx.r[30].s64 + 28;
	pc = 0x8240A40C; continue 'dispatch;
            }
            0x8240A40C => {
    //   block [0x8240A40C..0x8240A440)
	// 8240A40C: 80EAFFF8  lwz r7, -8(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240A410: 3529FFFF  addic. r9, r9, -1
	ctx.xer.ca = (ctx.r[9].u32 > (!(-1 as u32)));
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8240A414: 890A0000  lbz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8240A418: 80CAFFFC  lwz r6, -4(r10)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-4 as u32) ) } as u64;
	// 8240A41C: 394A0014  addi r10, r10, 0x14
	ctx.r[10].s64 = ctx.r[10].s64 + 20;
	// 8240A420: 90EBFFFC  stw r7, -4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-4 as u32), ctx.r[7].u32 ) };
	// 8240A424: 5507E13E  srwi r7, r8, 4
	ctx.r[7].u32 = ctx.r[8].u32.wrapping_shr(4);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 8240A428: 5508073E  clrlwi r8, r8, 0x1c
	ctx.r[8].u64 = ctx.r[8].u32 as u64 & 0x0000000Fu64;
	// 8240A42C: 90CB0000  stw r6, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[6].u32 ) };
	// 8240A430: 98EB0004  stb r7, 4(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[7].u8 ) };
	// 8240A434: 990B0005  stb r8, 5(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(5 as u32), ctx.r[8].u8 ) };
	// 8240A438: 396B000C  addi r11, r11, 0xc
	ctx.r[11].s64 = ctx.r[11].s64 + 12;
	// 8240A43C: 4082FFD0  bne 0x8240a40c
	if !ctx.cr[0].eq {
	pc = 0x8240A40C; continue 'dispatch;
	}
	pc = 0x8240A440; continue 'dispatch;
            }
            0x8240A440 => {
    //   block [0x8240A440..0x8240A468)
	// 8240A440: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8240A444: 807C0004  lwz r3, 4(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 8240A448: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8240A44C: 4817993D  bl 0x82583d88
	ctx.lr = 0x8240A450;
	sub_82583D88(ctx, base);
	// 8240A450: 7C641B79  or. r4, r3, r3
	ctx.r[4].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 8240A454: 40800014  bge 0x8240a468
	if !ctx.cr[0].lt {
	pc = 0x8240A468; continue 'dispatch;
	}
	// 8240A458: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240A45C: 386BD0B4  addi r3, r11, -0x2f4c
	ctx.r[3].s64 = ctx.r[11].s64 + -12108;
	// 8240A460: 4BEA8B21  bl 0x822b2f80
	ctx.lr = 0x8240A464;
	sub_822B2F80(ctx, base);
	// 8240A464: 4BFFFF7C  b 0x8240a3e0
	pc = 0x8240A3E0; continue 'dispatch;
            }
            0x8240A468 => {
    //   block [0x8240A468..0x8240A46C)
	// 8240A468: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	pc = 0x8240A46C; continue 'dispatch;
            }
            0x8240A46C => {
    //   block [0x8240A46C..0x8240A474)
	// 8240A46C: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 8240A470: 4812AC98  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240A478(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8240A478 size=196
    let mut pc: u32 = 0x8240A478;
    'dispatch: loop {
        match pc {
            0x8240A478 => {
    //   block [0x8240A478..0x8240A4CC)
	// 8240A478: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240A47C: 4812AC3D  bl 0x825350b8
	ctx.lr = 0x8240A480;
	sub_82535080(ctx, base);
	// 8240A480: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240A484: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8240A488: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8240A48C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8240A490: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 8240A494: 38A0001C  li r5, 0x1c
	ctx.r[5].s64 = 28;
	// 8240A498: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8240A49C: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8240A4A0: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8240A4A4: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 8240A4A8: 4812AD29  bl 0x825351d0
	ctx.lr = 0x8240A4AC;
	sub_825351D0(ctx, base);
	// 8240A4AC: 57CB057F  clrlwi. r11, r30, 0x15
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x000007FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8240A4B0: 4182001C  beq 0x8240a4cc
	if ctx.cr[0].eq {
	pc = 0x8240A4CC; continue 'dispatch;
	}
	// 8240A4B4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240A4B8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8240A4BC: 386BD1D8  addi r3, r11, -0x2e28
	ctx.r[3].s64 = ctx.r[11].s64 + -11816;
	// 8240A4C0: 4BEA8AC1  bl 0x822b2f80
	ctx.lr = 0x8240A4C4;
	sub_822B2F80(ctx, base);
	// 8240A4C4: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 8240A4C8: 4800006C  b 0x8240a534
	pc = 0x8240A534; continue 'dispatch;
            }
            0x8240A4CC => {
    //   block [0x8240A4CC..0x8240A4E0)
	// 8240A4CC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8240A4D0: 409A0018  bne cr6, 0x8240a4e8
	if !ctx.cr[6].eq {
	pc = 0x8240A4E8; continue 'dispatch;
	}
	// 8240A4D4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240A4D8: 386BD198  addi r3, r11, -0x2e68
	ctx.r[3].s64 = ctx.r[11].s64 + -11880;
	// 8240A4DC: 4BEA8AA5  bl 0x822b2f80
	ctx.lr = 0x8240A4E0;
	sub_822B2F80(ctx, base);
	pc = 0x8240A4E0; continue 'dispatch;
            }
            0x8240A4E0 => {
    //   block [0x8240A4E0..0x8240A4E8)
	// 8240A4E0: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 8240A4E4: 48000050  b 0x8240a534
	pc = 0x8240A534; continue 'dispatch;
            }
            0x8240A4E8 => {
    //   block [0x8240A4E8..0x8240A530)
	// 8240A4E8: 897F0003  lbz r11, 3(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(3 as u32) ) } as u64;
	// 8240A4EC: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8240A4F0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8240A4F4: 807D0004  lwz r3, 4(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 8240A4F8: 93C10050  stw r30, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 8240A4FC: 93810054  stw r28, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[28].u32 ) };
	// 8240A500: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 8240A504: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8240A508: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8240A50C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8240A510: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 8240A514: 4817992D  bl 0x82583e40
	ctx.lr = 0x8240A518;
	sub_82583E40(ctx, base);
	// 8240A518: 7C641B79  or. r4, r3, r3
	ctx.r[4].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 8240A51C: 40800014  bge 0x8240a530
	if !ctx.cr[0].lt {
	pc = 0x8240A530; continue 'dispatch;
	}
	// 8240A520: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240A524: 386BD168  addi r3, r11, -0x2e98
	ctx.r[3].s64 = ctx.r[11].s64 + -11928;
	// 8240A528: 4BEA8A59  bl 0x822b2f80
	ctx.lr = 0x8240A52C;
	sub_822B2F80(ctx, base);
	// 8240A52C: 4BFFFFB4  b 0x8240a4e0
	pc = 0x8240A4E0; continue 'dispatch;
            }
            0x8240A530 => {
    //   block [0x8240A530..0x8240A534)
	// 8240A530: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	pc = 0x8240A534; continue 'dispatch;
            }
            0x8240A534 => {
    //   block [0x8240A534..0x8240A53C)
	// 8240A534: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 8240A538: 4812ABD0  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240A540(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8240A540 size=664
    let mut pc: u32 = 0x8240A540;
    'dispatch: loop {
        match pc {
            0x8240A540 => {
    //   block [0x8240A540..0x8240A7C4)
	// 8240A540: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240A544: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240A548: 9421FE20  stwu r1, -0x1e0(r1)
	ea = ctx.r[1].u32.wrapping_add(-480 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240A54C: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 8240A550: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8240A554: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 8240A558: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 8240A55C: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8240A560: C00A1FF8  lfs f0, 0x1ff8(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8184 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240A564: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 8240A568: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 8240A56C: D0010084  stfs f0, 0x84(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), tmp.u32 ) };
	// 8240A570: D001008C  stfs f0, 0x8c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(140 as u32), tmp.u32 ) };
	// 8240A574: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 8240A578: D0010094  stfs f0, 0x94(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(148 as u32), tmp.u32 ) };
	// 8240A57C: 99610080  stb r11, 0x80(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[11].u8 ) };
	// 8240A580: D001009C  stfs f0, 0x9c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(156 as u32), tmp.u32 ) };
	// 8240A584: 99610081  stb r11, 0x81(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(129 as u32), ctx.r[11].u8 ) };
	// 8240A588: D00100A4  stfs f0, 0xa4(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(164 as u32), tmp.u32 ) };
	// 8240A58C: 99610088  stb r11, 0x88(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(136 as u32), ctx.r[11].u8 ) };
	// 8240A590: D00100AC  stfs f0, 0xac(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(172 as u32), tmp.u32 ) };
	// 8240A594: 98C10089  stb r6, 0x89(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(137 as u32), ctx.r[6].u8 ) };
	// 8240A598: D00100B4  stfs f0, 0xb4(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(180 as u32), tmp.u32 ) };
	// 8240A59C: 99610090  stb r11, 0x90(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(144 as u32), ctx.r[11].u8 ) };
	// 8240A5A0: D00100BC  stfs f0, 0xbc(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(188 as u32), tmp.u32 ) };
	// 8240A5A4: 99410091  stb r10, 0x91(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(145 as u32), ctx.r[10].u8 ) };
	// 8240A5A8: D00100C4  stfs f0, 0xc4(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(196 as u32), tmp.u32 ) };
	// 8240A5AC: 99610098  stb r11, 0x98(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(152 as u32), ctx.r[11].u8 ) };
	// 8240A5B0: D00100CC  stfs f0, 0xcc(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(204 as u32), tmp.u32 ) };
	// 8240A5B4: 98E10099  stb r7, 0x99(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(153 as u32), ctx.r[7].u8 ) };
	// 8240A5B8: D00100D4  stfs f0, 0xd4(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(212 as u32), tmp.u32 ) };
	// 8240A5BC: 996100A0  stb r11, 0xa0(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(160 as u32), ctx.r[11].u8 ) };
	// 8240A5C0: D00100DC  stfs f0, 0xdc(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(220 as u32), tmp.u32 ) };
	// 8240A5C4: 990100A1  stb r8, 0xa1(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(161 as u32), ctx.r[8].u8 ) };
	// 8240A5C8: D00100E4  stfs f0, 0xe4(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(228 as u32), tmp.u32 ) };
	// 8240A5CC: 996100A8  stb r11, 0xa8(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(168 as u32), ctx.r[11].u8 ) };
	// 8240A5D0: D00100EC  stfs f0, 0xec(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(236 as u32), tmp.u32 ) };
	// 8240A5D4: 992100A9  stb r9, 0xa9(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(169 as u32), ctx.r[9].u8 ) };
	// 8240A5D8: D00100F4  stfs f0, 0xf4(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(244 as u32), tmp.u32 ) };
	// 8240A5DC: 98C100B0  stb r6, 0xb0(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(176 as u32), ctx.r[6].u8 ) };
	// 8240A5E0: D00100FC  stfs f0, 0xfc(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(252 as u32), tmp.u32 ) };
	// 8240A5E4: 996100B1  stb r11, 0xb1(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(177 as u32), ctx.r[11].u8 ) };
	// 8240A5E8: D0010104  stfs f0, 0x104(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(260 as u32), tmp.u32 ) };
	// 8240A5EC: 98C100B8  stb r6, 0xb8(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(184 as u32), ctx.r[6].u8 ) };
	// 8240A5F0: D001010C  stfs f0, 0x10c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(268 as u32), tmp.u32 ) };
	// 8240A5F4: 98C100B9  stb r6, 0xb9(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(185 as u32), ctx.r[6].u8 ) };
	// 8240A5F8: D0010114  stfs f0, 0x114(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(276 as u32), tmp.u32 ) };
	// 8240A5FC: 98C100C0  stb r6, 0xc0(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(192 as u32), ctx.r[6].u8 ) };
	// 8240A600: D001011C  stfs f0, 0x11c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(284 as u32), tmp.u32 ) };
	// 8240A604: 994100C1  stb r10, 0xc1(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(193 as u32), ctx.r[10].u8 ) };
	// 8240A608: D0010124  stfs f0, 0x124(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(292 as u32), tmp.u32 ) };
	// 8240A60C: 98C100C8  stb r6, 0xc8(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(200 as u32), ctx.r[6].u8 ) };
	// 8240A610: D001012C  stfs f0, 0x12c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(300 as u32), tmp.u32 ) };
	// 8240A614: 98E100C9  stb r7, 0xc9(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(201 as u32), ctx.r[7].u8 ) };
	// 8240A618: D0010134  stfs f0, 0x134(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(308 as u32), tmp.u32 ) };
	// 8240A61C: 98C100D0  stb r6, 0xd0(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(208 as u32), ctx.r[6].u8 ) };
	// 8240A620: D001013C  stfs f0, 0x13c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(316 as u32), tmp.u32 ) };
	// 8240A624: 990100D1  stb r8, 0xd1(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(209 as u32), ctx.r[8].u8 ) };
	// 8240A628: D0010144  stfs f0, 0x144(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(324 as u32), tmp.u32 ) };
	// 8240A62C: 98C100D8  stb r6, 0xd8(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(216 as u32), ctx.r[6].u8 ) };
	// 8240A630: D001014C  stfs f0, 0x14c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(332 as u32), tmp.u32 ) };
	// 8240A634: 992100D9  stb r9, 0xd9(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(217 as u32), ctx.r[9].u8 ) };
	// 8240A638: D0010154  stfs f0, 0x154(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(340 as u32), tmp.u32 ) };
	// 8240A63C: 994100E0  stb r10, 0xe0(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(224 as u32), ctx.r[10].u8 ) };
	// 8240A640: 996100E1  stb r11, 0xe1(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(225 as u32), ctx.r[11].u8 ) };
	// 8240A644: 994100E8  stb r10, 0xe8(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(232 as u32), ctx.r[10].u8 ) };
	// 8240A648: 98C100E9  stb r6, 0xe9(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(233 as u32), ctx.r[6].u8 ) };
	// 8240A64C: 994100F0  stb r10, 0xf0(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(240 as u32), ctx.r[10].u8 ) };
	// 8240A650: 994100F1  stb r10, 0xf1(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(241 as u32), ctx.r[10].u8 ) };
	// 8240A654: 994100F8  stb r10, 0xf8(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(248 as u32), ctx.r[10].u8 ) };
	// 8240A658: 98E100F9  stb r7, 0xf9(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(249 as u32), ctx.r[7].u8 ) };
	// 8240A65C: 99410100  stb r10, 0x100(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(256 as u32), ctx.r[10].u8 ) };
	// 8240A660: 99010101  stb r8, 0x101(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(257 as u32), ctx.r[8].u8 ) };
	// 8240A664: 99410108  stb r10, 0x108(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(264 as u32), ctx.r[10].u8 ) };
	// 8240A668: 99210109  stb r9, 0x109(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(265 as u32), ctx.r[9].u8 ) };
	// 8240A66C: 98E10110  stb r7, 0x110(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(272 as u32), ctx.r[7].u8 ) };
	// 8240A670: 99610111  stb r11, 0x111(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(273 as u32), ctx.r[11].u8 ) };
	// 8240A674: 98E10118  stb r7, 0x118(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(280 as u32), ctx.r[7].u8 ) };
	// 8240A678: 98C10119  stb r6, 0x119(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(281 as u32), ctx.r[6].u8 ) };
	// 8240A67C: 98E10120  stb r7, 0x120(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(288 as u32), ctx.r[7].u8 ) };
	// 8240A680: 99410121  stb r10, 0x121(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(289 as u32), ctx.r[10].u8 ) };
	// 8240A684: 98E10128  stb r7, 0x128(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(296 as u32), ctx.r[7].u8 ) };
	// 8240A688: 98E10129  stb r7, 0x129(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(297 as u32), ctx.r[7].u8 ) };
	// 8240A68C: 98E10130  stb r7, 0x130(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(304 as u32), ctx.r[7].u8 ) };
	// 8240A690: 99210139  stb r9, 0x139(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(313 as u32), ctx.r[9].u8 ) };
	// 8240A694: D001015C  stfs f0, 0x15c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(348 as u32), tmp.u32 ) };
	// 8240A698: 99210169  stb r9, 0x169(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(361 as u32), ctx.r[9].u8 ) };
	// 8240A69C: D0010164  stfs f0, 0x164(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(356 as u32), tmp.u32 ) };
	// 8240A6A0: 99210170  stb r9, 0x170(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(368 as u32), ctx.r[9].u8 ) };
	// 8240A6A4: D001016C  stfs f0, 0x16c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(364 as u32), tmp.u32 ) };
	// 8240A6A8: 99210178  stb r9, 0x178(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(376 as u32), ctx.r[9].u8 ) };
	// 8240A6AC: D0010174  stfs f0, 0x174(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(372 as u32), tmp.u32 ) };
	// 8240A6B0: 99210180  stb r9, 0x180(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(384 as u32), ctx.r[9].u8 ) };
	// 8240A6B4: D001017C  stfs f0, 0x17c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(380 as u32), tmp.u32 ) };
	// 8240A6B8: 99210188  stb r9, 0x188(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(392 as u32), ctx.r[9].u8 ) };
	// 8240A6BC: D0010184  stfs f0, 0x184(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(388 as u32), tmp.u32 ) };
	// 8240A6C0: 99210190  stb r9, 0x190(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(400 as u32), ctx.r[9].u8 ) };
	// 8240A6C4: D001018C  stfs f0, 0x18c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(396 as u32), tmp.u32 ) };
	// 8240A6C8: 99210198  stb r9, 0x198(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(408 as u32), ctx.r[9].u8 ) };
	// 8240A6CC: D0010194  stfs f0, 0x194(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(404 as u32), tmp.u32 ) };
	// 8240A6D0: 99210199  stb r9, 0x199(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(409 as u32), ctx.r[9].u8 ) };
	// 8240A6D4: D001019C  stfs f0, 0x19c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(412 as u32), tmp.u32 ) };
	// 8240A6D8: 99210078  stb r9, 0x78(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[9].u8 ) };
	// 8240A6DC: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 8240A6E0: 99610141  stb r11, 0x141(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(321 as u32), ctx.r[11].u8 ) };
	// 8240A6E4: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 8240A6E8: 99610171  stb r11, 0x171(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(369 as u32), ctx.r[11].u8 ) };
	// 8240A6EC: D001005C  stfs f0, 0x5c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), tmp.u32 ) };
	// 8240A6F0: 99610050  stb r11, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u8 ) };
	// 8240A6F4: D0010064  stfs f0, 0x64(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), tmp.u32 ) };
	// 8240A6F8: 99610051  stb r11, 0x51(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(81 as u32), ctx.r[11].u8 ) };
	// 8240A6FC: D001006C  stfs f0, 0x6c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), tmp.u32 ) };
	// 8240A700: 992101A8  stb r9, 0x1a8(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(424 as u32), ctx.r[9].u8 ) };
	// 8240A704: 39210050  addi r9, r1, 0x50
	ctx.r[9].s64 = ctx.r[1].s64 + 80;
	// 8240A708: 99610059  stb r11, 0x59(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(89 as u32), ctx.r[11].u8 ) };
	// 8240A70C: D0010074  stfs f0, 0x74(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), tmp.u32 ) };
	// 8240A710: 99610061  stb r11, 0x61(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(97 as u32), ctx.r[11].u8 ) };
	// 8240A714: D001007C  stfs f0, 0x7c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), tmp.u32 ) };
	// 8240A718: 99610069  stb r11, 0x69(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(105 as u32), ctx.r[11].u8 ) };
	// 8240A71C: 99610071  stb r11, 0x71(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(113 as u32), ctx.r[11].u8 ) };
	// 8240A720: 99610079  stb r11, 0x79(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(121 as u32), ctx.r[11].u8 ) };
	// 8240A724: 912101AC  stw r9, 0x1ac(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(428 as u32), ctx.r[9].u32 ) };
	// 8240A728: 39200024  li r9, 0x24
	ctx.r[9].s64 = 36;
	// 8240A72C: 916101C0  stw r11, 0x1c0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(448 as u32), ctx.r[11].u32 ) };
	// 8240A730: 396101B0  addi r11, r1, 0x1b0
	ctx.r[11].s64 = ctx.r[1].s64 + 432;
	// 8240A734: 80650004  lwz r3, 4(r5)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) } as u64;
	// 8240A738: 99410151  stb r10, 0x151(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(337 as u32), ctx.r[10].u8 ) };
	// 8240A73C: 99410181  stb r10, 0x181(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(385 as u32), ctx.r[10].u8 ) };
	// 8240A740: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8240A744: 99410060  stb r10, 0x60(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[10].u8 ) };
	// 8240A748: 992101B0  stb r9, 0x1b0(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(432 as u32), ctx.r[9].u8 ) };
	// 8240A74C: 39210080  addi r9, r1, 0x80
	ctx.r[9].s64 = ctx.r[1].s64 + 128;
	// 8240A750: 994101A0  stb r10, 0x1a0(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(416 as u32), ctx.r[10].u8 ) };
	// 8240A754: 394101C0  addi r10, r1, 0x1c0
	ctx.r[10].s64 = ctx.r[1].s64 + 448;
	// 8240A758: 916101C4  stw r11, 0x1c4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(452 as u32), ctx.r[11].u32 ) };
	// 8240A75C: 396101A8  addi r11, r1, 0x1a8
	ctx.r[11].s64 = ctx.r[1].s64 + 424;
	// 8240A760: 80A50008  lwz r5, 8(r5)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(8 as u32) ) } as u64;
	// 8240A764: 99010131  stb r8, 0x131(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(305 as u32), ctx.r[8].u8 ) };
	// 8240A768: 98E10138  stb r7, 0x138(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(312 as u32), ctx.r[7].u8 ) };
	// 8240A76C: 99010140  stb r8, 0x140(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(320 as u32), ctx.r[8].u8 ) };
	// 8240A770: 99010148  stb r8, 0x148(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(328 as u32), ctx.r[8].u8 ) };
	// 8240A774: 98C10149  stb r6, 0x149(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(329 as u32), ctx.r[6].u8 ) };
	// 8240A778: 99010150  stb r8, 0x150(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(336 as u32), ctx.r[8].u8 ) };
	// 8240A77C: 99010158  stb r8, 0x158(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(344 as u32), ctx.r[8].u8 ) };
	// 8240A780: 90A101C8  stw r5, 0x1c8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(456 as u32), ctx.r[5].u32 ) };
	// 8240A784: 98E10159  stb r7, 0x159(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(345 as u32), ctx.r[7].u8 ) };
	// 8240A788: 99010160  stb r8, 0x160(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(352 as u32), ctx.r[8].u8 ) };
	// 8240A78C: 99010161  stb r8, 0x161(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(353 as u32), ctx.r[8].u8 ) };
	// 8240A790: 99010168  stb r8, 0x168(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(360 as u32), ctx.r[8].u8 ) };
	// 8240A794: 98C10179  stb r6, 0x179(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(377 as u32), ctx.r[6].u8 ) };
	// 8240A798: 98E10189  stb r7, 0x189(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(393 as u32), ctx.r[7].u8 ) };
	// 8240A79C: 99010191  stb r8, 0x191(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(401 as u32), ctx.r[8].u8 ) };
	// 8240A7A0: 98C10058  stb r6, 0x58(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[6].u8 ) };
	// 8240A7A4: 98E10068  stb r7, 0x68(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[7].u8 ) };
	// 8240A7A8: 99010070  stb r8, 0x70(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[8].u8 ) };
	// 8240A7AC: 912101B4  stw r9, 0x1b4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(436 as u32), ctx.r[9].u32 ) };
	// 8240A7B0: 914101A4  stw r10, 0x1a4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(420 as u32), ctx.r[10].u32 ) };
	// 8240A7B4: 916101CC  stw r11, 0x1cc(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(460 as u32), ctx.r[11].u32 ) };
	// 8240A7B8: 4182000C  beq 0x8240a7c4
	if ctx.cr[0].eq {
	pc = 0x8240A7C4; continue 'dispatch;
	}
	// 8240A7BC: 388101A0  addi r4, r1, 0x1a0
	ctx.r[4].s64 = ctx.r[1].s64 + 416;
	// 8240A7C0: 481794C1  bl 0x82583c80
	ctx.lr = 0x8240A7C4;
	sub_82583C80(ctx, base);
	pc = 0x8240A7C4; continue 'dispatch;
            }
            0x8240A7C4 => {
    //   block [0x8240A7C4..0x8240A7D8)
	// 8240A7C4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8240A7C8: 382101E0  addi r1, r1, 0x1e0
	ctx.r[1].s64 = ctx.r[1].s64 + 480;
	// 8240A7CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240A7D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240A7D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240A7D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8240A7D8 size=68
    let mut pc: u32 = 0x8240A7D8;
    'dispatch: loop {
        match pc {
            0x8240A7D8 => {
    //   block [0x8240A7D8..0x8240A800)
	// 8240A7D8: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8240A7DC: D023002C  stfs f1, 0x2c(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), tmp.u32 ) };
	// 8240A7E0: D0430030  stfs f2, 0x30(r3)
	tmp.f32 = (ctx.f[2].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(48 as u32), tmp.u32 ) };
	// 8240A7E4: C00B1FF8  lfs f0, 0x1ff8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240A7E8: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8240A7EC: FF010000  fcmpu cr6, f1, f0
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 8240A7F0: C1AB1850  lfs f13, 0x1850(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(6224 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8240A7F4: 4098000C  bge cr6, 0x8240a800
	if !ctx.cr[6].lt {
	pc = 0x8240A800; continue 'dispatch;
	}
	// 8240A7F8: D003002C  stfs f0, 0x2c(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), tmp.u32 ) };
	// 8240A7FC: 48000010  b 0x8240a80c
	pc = 0x8240A80C; continue 'dispatch;
            }
            0x8240A800 => {
    //   block [0x8240A800..0x8240A80C)
	// 8240A800: FF016800  fcmpu cr6, f1, f13
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[13].f64);
	// 8240A804: 40990008  ble cr6, 0x8240a80c
	if !ctx.cr[6].gt {
	pc = 0x8240A80C; continue 'dispatch;
	}
	// 8240A808: D1A3002C  stfs f13, 0x2c(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), tmp.u32 ) };
	pc = 0x8240A80C; continue 'dispatch;
            }
            0x8240A80C => {
    //   block [0x8240A80C..0x8240A81C)
	// 8240A80C: FF020000  fcmpu cr6, f2, f0
	ctx.cr[6].compare_f64(ctx.f[2].f64, ctx.f[0].f64);
	// 8240A810: 4098000C  bge cr6, 0x8240a81c
	if !ctx.cr[6].lt {
		crate::recompiler::externs::call(ctx, base, 0x8240A81C);
		return;
	}
	// 8240A814: D0030030  stfs f0, 0x30(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(48 as u32), tmp.u32 ) };
	// 8240A818: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240A830(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8240A830 size=88
    let mut pc: u32 = 0x8240A830;
    'dispatch: loop {
        match pc {
            0x8240A830 => {
    //   block [0x8240A830..0x8240A858)
	// 8240A830: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240A834: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240A838: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8240A83C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240A840: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8240A844: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 8240A848: 40980010  bge cr6, 0x8240a858
	if !ctx.cr[6].lt {
	pc = 0x8240A858; continue 'dispatch;
	}
	// 8240A84C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8240A850: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 8240A854: 4800001C  b 0x8240a870
	pc = 0x8240A870; continue 'dispatch;
            }
            0x8240A858 => {
    //   block [0x8240A858..0x8240A870)
	// 8240A858: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8240A85C: 909F0034  stw r4, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[4].u32 ) };
	// 8240A860: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8240A864: 4BFFD0BD  bl 0x82407920
	ctx.lr = 0x8240A868;
	sub_82407920(ctx, base);
	// 8240A868: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8240A86C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	pc = 0x8240A870; continue 'dispatch;
            }
            0x8240A870 => {
    //   block [0x8240A870..0x8240A888)
	// 8240A870: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8240A874: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8240A878: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240A87C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240A880: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8240A884: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240A888(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8240A888 size=4
    let mut pc: u32 = 0x8240A888;
    'dispatch: loop {
        match pc {
            0x8240A888 => {
    //   block [0x8240A888..0x8240A88C)
	// 8240A888: 4BFFF688  b 0x82409f10
	sub_82409F10(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240A890(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8240A890 size=84
    let mut pc: u32 = 0x8240A890;
    'dispatch: loop {
        match pc {
            0x8240A890 => {
    //   block [0x8240A890..0x8240A8D0)
	// 8240A890: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240A894: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240A898: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8240A89C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240A8A0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8240A8A4: 817F0028  lwz r11, 0x28(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 8240A8A8: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 8240A8AC: 409A0024  bne cr6, 0x8240a8d0
	if !ctx.cr[6].eq {
	pc = 0x8240A8D0; continue 'dispatch;
	}
	// 8240A8B0: 389F000C  addi r4, r31, 0xc
	ctx.r[4].s64 = ctx.r[31].s64 + 12;
	// 8240A8B4: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8240A8B8: 48179479  bl 0x82583d30
	ctx.lr = 0x8240A8BC;
	sub_82583D30(ctx, base);
	// 8240A8BC: 897F000C  lbz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8240A8C0: 556B06B5  rlwinm. r11, r11, 0, 0x1a, 0x1a
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8240A8C4: 4182000C  beq 0x8240a8d0
	if ctx.cr[0].eq {
	pc = 0x8240A8D0; continue 'dispatch;
	}
	// 8240A8C8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8240A8CC: 4BFFF645  bl 0x82409f10
	ctx.lr = 0x8240A8D0;
	sub_82409F10(ctx, base);
	pc = 0x8240A8D0; continue 'dispatch;
            }
            0x8240A8D0 => {
    //   block [0x8240A8D0..0x8240A8E4)
	// 8240A8D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8240A8D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240A8D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240A8DC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8240A8E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240A8E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8240A8E8 size=352
    let mut pc: u32 = 0x8240A8E8;
    'dispatch: loop {
        match pc {
            0x8240A8E8 => {
    //   block [0x8240A8E8..0x8240A95C)
	// 8240A8E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240A8EC: 4812A7B5  bl 0x825350a0
	ctx.lr = 0x8240A8F0;
	sub_82535080(ctx, base);
	// 8240A8F0: 9421FEA0  stwu r1, -0x160(r1)
	ea = ctx.r[1].u32.wrapping_add(-352 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240A8F4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8240A8F8: 7CB72B78  mr r23, r5
	ctx.r[23].u64 = ctx.r[5].u64;
	// 8240A8FC: 7CD63378  mr r22, r6
	ctx.r[22].u64 = ctx.r[6].u64;
	// 8240A900: 7CF83B78  mr r24, r7
	ctx.r[24].u64 = ctx.r[7].u64;
	// 8240A904: 2B040FFF  cmplwi cr6, r4, 0xfff
	ctx.cr[6].compare_u32(ctx.r[4].u32, 4095 as u32, &mut ctx.xer);
	// 8240A908: 41990128  bgt cr6, 0x8240aa30
	if ctx.cr[6].gt {
	pc = 0x8240AA30; continue 'dispatch;
	}
	// 8240A90C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8240A910: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 8240A914: 4800527D  bl 0x8240fb90
	ctx.lr = 0x8240A918;
	sub_8240FB90(ctx, base);
	// 8240A918: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8240A91C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8240A920: 38A00058  li r5, 0x58
	ctx.r[5].s64 = 88;
	// 8240A924: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8240A928: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8240A92C: 838B0010  lwz r28, 0x10(r11)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 8240A930: 836B000C  lwz r27, 0xc(r11)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8240A934: 83EB0014  lwz r31, 0x14(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 8240A938: 99410050  stb r10, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u8 ) };
	// 8240A93C: 4812A895  bl 0x825351d0
	ctx.lr = 0x8240A940;
	sub_825351D0(ctx, base);
	// 8240A940: A17F000E  lhz r11, 0xe(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(14 as u32) ) } as u64;
	// 8240A944: 3B200001  li r25, 1
	ctx.r[25].s64 = 1;
	// 8240A948: 3B400002  li r26, 2
	ctx.r[26].s64 = 2;
	// 8240A94C: 2B0B0010  cmplwi cr6, r11, 0x10
	ctx.cr[6].compare_u32(ctx.r[11].u32, 16 as u32, &mut ctx.xer);
	// 8240A950: 409A000C  bne cr6, 0x8240a95c
	if !ctx.cr[6].eq {
	pc = 0x8240A95C; continue 'dispatch;
	}
	// 8240A954: 9B210050  stb r25, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[25].u8 ) };
	// 8240A958: 48000020  b 0x8240a978
	pc = 0x8240A978; continue 'dispatch;
            }
            0x8240A95C => {
    //   block [0x8240A95C..0x8240A96C)
	// 8240A95C: 2B0B0008  cmplwi cr6, r11, 8
	ctx.cr[6].compare_u32(ctx.r[11].u32, 8 as u32, &mut ctx.xer);
	// 8240A960: 409A000C  bne cr6, 0x8240a96c
	if !ctx.cr[6].eq {
	pc = 0x8240A96C; continue 'dispatch;
	}
	// 8240A964: 9B410050  stb r26, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[26].u8 ) };
	// 8240A968: 48000010  b 0x8240a978
	pc = 0x8240A978; continue 'dispatch;
            }
            0x8240A96C => {
    //   block [0x8240A96C..0x8240A978)
	// 8240A96C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240A970: 386BD2B0  addi r3, r11, -0x2d50
	ctx.r[3].s64 = ctx.r[11].s64 + -11600;
	// 8240A974: 4BEA860D  bl 0x822b2f80
	ctx.lr = 0x8240A978;
	sub_822B2F80(ctx, base);
	pc = 0x8240A978; continue 'dispatch;
            }
            0x8240A978 => {
    //   block [0x8240A978..0x8240A9CC)
	// 8240A978: A17F0002  lhz r11, 2(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(2 as u32) ) } as u64;
	// 8240A97C: 3957FFFF  addi r10, r23, -1
	ctx.r[10].s64 = ctx.r[23].s64 + -1;
	// 8240A980: 813F0004  lwz r9, 4(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8240A984: 3BBE0004  addi r29, r30, 4
	ctx.r[29].s64 = ctx.r[30].s64 + 4;
	// 8240A988: 7D4A0034  cntlzw r10, r10
	ctx.r[10].u64 = if ctx.r[10].u32 == 0 { 32 } else { ctx.r[10].u32.leading_zeros() as u64 };
	// 8240A98C: 9B410089  stb r26, 0x89(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(137 as u32), ctx.r[26].u8 ) };
	// 8240A990: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8240A994: 9B21008B  stb r25, 0x8b(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(139 as u32), ctx.r[25].u8 ) };
	// 8240A998: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8240A99C: 99610054  stb r11, 0x54(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u8 ) };
	// 8240A9A0: 554BDFFE  rlwinm r11, r10, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0x0000001Fu64;
	// 8240A9A4: 91210058  stw r9, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[9].u32 ) };
	// 8240A9A8: 39200024  li r9, 0x24
	ctx.r[9].s64 = 36;
	// 8240A9AC: 99610088  stb r11, 0x88(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(136 as u32), ctx.r[11].u8 ) };
	// 8240A9B0: 9921008A  stb r9, 0x8a(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(138 as u32), ctx.r[9].u8 ) };
	// 8240A9B4: 48179915  bl 0x825842c8
	ctx.lr = 0x8240A9B8;
	sub_825842C8(ctx, base);
	// 8240A9B8: 7C641B79  or. r4, r3, r3
	ctx.r[4].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 8240A9BC: 40800010  bge 0x8240a9cc
	if !ctx.cr[0].lt {
	pc = 0x8240A9CC; continue 'dispatch;
	}
	// 8240A9C0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240A9C4: 386BD278  addi r3, r11, -0x2d88
	ctx.r[3].s64 = ctx.r[11].s64 + -11656;
	// 8240A9C8: 4BEA85B9  bl 0x822b2f80
	ctx.lr = 0x8240A9CC;
	sub_822B2F80(ctx, base);
	pc = 0x8240A9CC; continue 'dispatch;
            }
            0x8240A9CC => {
    //   block [0x8240A9CC..0x8240AA28)
	// 8240A9CC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8240A9D0: A09F0002  lhz r4, 2(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(2 as u32) ) } as u64;
	// 8240A9D4: 4BFFFB6D  bl 0x8240a540
	ctx.lr = 0x8240A9D8;
	sub_8240A540(ctx, base);
	// 8240A9D8: 38A00054  li r5, 0x54
	ctx.r[5].s64 = 84;
	// 8240A9DC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8240A9E0: 386100B4  addi r3, r1, 0xb4
	ctx.r[3].s64 = ctx.r[1].s64 + 180;
	// 8240A9E4: 4812A7ED  bl 0x825351d0
	ctx.lr = 0x8240A9E8;
	sub_825351D0(ctx, base);
	// 8240A9E8: 21780000  subfic r11, r24, 0
	ctx.xer.ca = ctx.r[24].u32 <= 0 as u32;
	ctx.r[11].s64 = (0 as i64) - ctx.r[24].s64;
	// 8240A9EC: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8240A9F0: 807D0000  lwz r3, 0(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8240A9F4: 7D6B5910  subfe r11, r11, r11
	let x = (!ctx.r[11].u32);
	let y = ctx.r[11].u32;
	let s = x.wrapping_add(y);
	let res = s.wrapping_add(ctx.xer.ca as u32);
	tmp.u8 = (s < x) as u8 | (res < s) as u8;
	ctx.r[11].u32 = res;
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	ctx.xer.ca = (tmp.u8 != 0);
	// 8240A9F8: 938100B0  stw r28, 0xb0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(176 as u32), ctx.r[28].u32 ) };
	// 8240A9FC: 388100B0  addi r4, r1, 0xb0
	ctx.r[4].s64 = ctx.r[1].s64 + 176;
	// 8240AA00: 936100B4  stw r27, 0xb4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(180 as u32), ctx.r[27].u32 ) };
	// 8240AA04: 92C100BC  stw r22, 0xbc(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(188 as u32), ctx.r[22].u32 ) };
	// 8240AA08: 930100C0  stw r24, 0xc0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(192 as u32), ctx.r[24].u32 ) };
	// 8240AA0C: 916100B8  stw r11, 0xb8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(184 as u32), ctx.r[11].u32 ) };
	// 8240AA10: 48179379  bl 0x82583d88
	ctx.lr = 0x8240AA14;
	sub_82583D88(ctx, base);
	// 8240AA14: 7C641B79  or. r4, r3, r3
	ctx.r[4].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 8240AA18: 40800010  bge 0x8240aa28
	if !ctx.cr[0].lt {
	pc = 0x8240AA28; continue 'dispatch;
	}
	// 8240AA1C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240AA20: 386BD0B4  addi r3, r11, -0x2f4c
	ctx.r[3].s64 = ctx.r[11].s64 + -12108;
	// 8240AA24: 4BEA855D  bl 0x822b2f80
	ctx.lr = 0x8240AA28;
	sub_822B2F80(ctx, base);
	pc = 0x8240AA28; continue 'dispatch;
            }
            0x8240AA28 => {
    //   block [0x8240AA28..0x8240AA30)
	// 8240AA28: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8240AA2C: 48000014  b 0x8240aa40
	pc = 0x8240AA40; continue 'dispatch;
            }
            0x8240AA30 => {
    //   block [0x8240AA30..0x8240AA40)
	// 8240AA30: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240AA34: 386BD230  addi r3, r11, -0x2dd0
	ctx.r[3].s64 = ctx.r[11].s64 + -11728;
	// 8240AA38: 4BEA8549  bl 0x822b2f80
	ctx.lr = 0x8240AA3C;
	sub_822B2F80(ctx, base);
	// 8240AA3C: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	pc = 0x8240AA40; continue 'dispatch;
            }
            0x8240AA40 => {
    //   block [0x8240AA40..0x8240AA48)
	// 8240AA40: 38210160  addi r1, r1, 0x160
	ctx.r[1].s64 = ctx.r[1].s64 + 352;
	// 8240AA44: 4812A6AC  b 0x825350f0
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240AA48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8240AA48 size=300
    let mut pc: u32 = 0x8240AA48;
    'dispatch: loop {
        match pc {
            0x8240AA48 => {
    //   block [0x8240AA48..0x8240AA90)
	// 8240AA48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240AA4C: 4812A66D  bl 0x825350b8
	ctx.lr = 0x8240AA50;
	sub_82535080(ctx, base);
	// 8240AA50: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240AA54: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8240AA58: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8240AA5C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8240AA60: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 8240AA64: 38A00058  li r5, 0x58
	ctx.r[5].s64 = 88;
	// 8240AA68: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8240AA6C: 99610050  stb r11, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u8 ) };
	// 8240AA70: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8240AA74: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 8240AA78: 4812A759  bl 0x825351d0
	ctx.lr = 0x8240AA7C;
	sub_825351D0(ctx, base);
	// 8240AA7C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8240AA80: 409A0018  bne cr6, 0x8240aa98
	if !ctx.cr[6].eq {
	pc = 0x8240AA98; continue 'dispatch;
	}
	// 8240AA84: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240AA88: 386BD2D0  addi r3, r11, -0x2d30
	ctx.r[3].s64 = ctx.r[11].s64 + -11568;
	// 8240AA8C: 4BEA84F5  bl 0x822b2f80
	ctx.lr = 0x8240AA90;
	sub_822B2F80(ctx, base);
	pc = 0x8240AA90; continue 'dispatch;
            }
            0x8240AA90 => {
    //   block [0x8240AA90..0x8240AA98)
	// 8240AA90: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 8240AA94: 480000D8  b 0x8240ab6c
	pc = 0x8240AB6C; continue 'dispatch;
            }
            0x8240AA98 => {
    //   block [0x8240AA98..0x8240AAA8)
	// 8240AA98: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8240AA9C: 409A000C  bne cr6, 0x8240aaa8
	if !ctx.cr[6].eq {
	pc = 0x8240AAA8; continue 'dispatch;
	}
	// 8240AAA0: A17F0008  lhz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8240AAA4: 48000008  b 0x8240aaac
	pc = 0x8240AAAC; continue 'dispatch;
            }
            0x8240AAA8 => {
    //   block [0x8240AAA8..0x8240AAAC)
	// 8240AAA8: 897F0001  lbz r11, 1(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(1 as u32) ) } as u64;
	pc = 0x8240AAAC; continue 'dispatch;
            }
            0x8240AAAC => {
    //   block [0x8240AAAC..0x8240AAF0)
	// 8240AAAC: 5569043F  clrlwi. r9, r11, 0x10
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8240AAB0: 99610054  stb r11, 0x54(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u8 ) };
	// 8240AAB4: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 8240AAB8: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 8240AABC: 99610089  stb r11, 0x89(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(137 as u32), ctx.r[11].u8 ) };
	// 8240AAC0: 39600024  li r11, 0x24
	ctx.r[11].s64 = 36;
	// 8240AAC4: C00A2068  lfs f0, 0x2068(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8296 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240AAC8: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 8240AACC: D001008C  stfs f0, 0x8c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(140 as u32), tmp.u32 ) };
	// 8240AAD0: 9961008A  stb r11, 0x8a(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(138 as u32), ctx.r[11].u8 ) };
	// 8240AAD4: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8240AAD8: 99410050  stb r10, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u8 ) };
	// 8240AADC: 9961008B  stb r11, 0x8b(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(139 as u32), ctx.r[11].u8 ) };
	// 8240AAE0: 40810054  ble 0x8240ab34
	if !ctx.cr[0].gt {
	pc = 0x8240AB34; continue 'dispatch;
	}
	// 8240AAE4: 3961005C  addi r11, r1, 0x5c
	ctx.r[11].s64 = ctx.r[1].s64 + 92;
	// 8240AAE8: 38FF0028  addi r7, r31, 0x28
	ctx.r[7].s64 = ctx.r[31].s64 + 40;
	// 8240AAEC: 395F001D  addi r10, r31, 0x1d
	ctx.r[10].s64 = ctx.r[31].s64 + 29;
	pc = 0x8240AAF0; continue 'dispatch;
            }
            0x8240AAF0 => {
    //   block [0x8240AAF0..0x8240AB00)
	// 8240AAF0: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8240AAF4: 409A000C  bne cr6, 0x8240ab00
	if !ctx.cr[6].eq {
	pc = 0x8240AB00; continue 'dispatch;
	}
	// 8240AAF8: 810AFFF3  lwz r8, -0xd(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-13 as u32) ) } as u64;
	// 8240AAFC: 48000008  b 0x8240ab04
	pc = 0x8240AB04; continue 'dispatch;
            }
            0x8240AB00 => {
    //   block [0x8240AB00..0x8240AB04)
	// 8240AB00: 811F000C  lwz r8, 0xc(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	pc = 0x8240AB04; continue 'dispatch;
            }
            0x8240AB04 => {
    //   block [0x8240AB04..0x8240AB18)
	// 8240AB04: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8240AB08: 910BFFFC  stw r8, -4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-4 as u32), ctx.r[8].u32 ) };
	// 8240AB0C: 409A000C  bne cr6, 0x8240ab18
	if !ctx.cr[6].eq {
	pc = 0x8240AB18; continue 'dispatch;
	}
	// 8240AB10: 890A0000  lbz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8240AB14: 48000008  b 0x8240ab1c
	pc = 0x8240AB1C; continue 'dispatch;
            }
            0x8240AB18 => {
    //   block [0x8240AB18..0x8240AB1C)
	// 8240AB18: 89070000  lbz r8, 0(r7)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	pc = 0x8240AB1C; continue 'dispatch;
            }
            0x8240AB1C => {
    //   block [0x8240AB1C..0x8240AB34)
	// 8240AB1C: 990B0000  stb r8, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u8 ) };
	// 8240AB20: 3529FFFF  addic. r9, r9, -1
	ctx.xer.ca = (ctx.r[9].u32 > (!(-1 as u32)));
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8240AB24: 38E70004  addi r7, r7, 4
	ctx.r[7].s64 = ctx.r[7].s64 + 4;
	// 8240AB28: 394A0014  addi r10, r10, 0x14
	ctx.r[10].s64 = ctx.r[10].s64 + 20;
	// 8240AB2C: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 8240AB30: 4082FFC0  bne 0x8240aaf0
	if !ctx.cr[0].eq {
	pc = 0x8240AAF0; continue 'dispatch;
	}
	pc = 0x8240AB34; continue 'dispatch;
            }
            0x8240AB34 => {
    //   block [0x8240AB34..0x8240AB68)
	// 8240AB34: 397CFFFF  addi r11, r28, -1
	ctx.r[11].s64 = ctx.r[28].s64 + -1;
	// 8240AB38: 389D0004  addi r4, r29, 4
	ctx.r[4].s64 = ctx.r[29].s64 + 4;
	// 8240AB3C: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 8240AB40: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8240AB44: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8240AB48: 99610088  stb r11, 0x88(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(136 as u32), ctx.r[11].u8 ) };
	// 8240AB4C: 4817977D  bl 0x825842c8
	ctx.lr = 0x8240AB50;
	sub_825842C8(ctx, base);
	// 8240AB50: 7C641B79  or. r4, r3, r3
	ctx.r[4].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 8240AB54: 40800014  bge 0x8240ab68
	if !ctx.cr[0].lt {
	pc = 0x8240AB68; continue 'dispatch;
	}
	// 8240AB58: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240AB5C: 386BD278  addi r3, r11, -0x2d88
	ctx.r[3].s64 = ctx.r[11].s64 + -11656;
	// 8240AB60: 4BEA8421  bl 0x822b2f80
	ctx.lr = 0x8240AB64;
	sub_822B2F80(ctx, base);
	// 8240AB64: 4BFFFF2C  b 0x8240aa90
	pc = 0x8240AA90; continue 'dispatch;
            }
            0x8240AB68 => {
    //   block [0x8240AB68..0x8240AB6C)
	// 8240AB68: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	pc = 0x8240AB6C; continue 'dispatch;
            }
            0x8240AB6C => {
    //   block [0x8240AB6C..0x8240AB74)
	// 8240AB6C: 382100E0  addi r1, r1, 0xe0
	ctx.r[1].s64 = ctx.r[1].s64 + 224;
	// 8240AB70: 4812A598  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240AB78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8240AB78 size=132
    let mut pc: u32 = 0x8240AB78;
    'dispatch: loop {
        match pc {
            0x8240AB78 => {
    //   block [0x8240AB78..0x8240ABE4)
	// 8240AB78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240AB7C: 4812A541  bl 0x825350bc
	ctx.lr = 0x8240AB80;
	sub_82535080(ctx, base);
	// 8240AB80: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240AB84: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8240AB88: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8240AB8C: 2B040FFF  cmplwi cr6, r4, 0xfff
	ctx.cr[6].compare_u32(ctx.r[4].u32, 4095 as u32, &mut ctx.xer);
	// 8240AB90: 41990054  bgt cr6, 0x8240abe4
	if ctx.cr[6].gt {
	pc = 0x8240ABE4; continue 'dispatch;
	}
	// 8240AB94: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8240AB98: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 8240AB9C: 48004FF5  bl 0x8240fb90
	ctx.lr = 0x8240ABA0;
	sub_8240FB90(ctx, base);
	// 8240ABA0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8240ABA4: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8240ABA8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8240ABAC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8240ABB0: 83BF0014  lwz r29, 0x14(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 8240ABB4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8240ABB8: 4BFFFE91  bl 0x8240aa48
	ctx.lr = 0x8240ABBC;
	sub_8240AA48(ctx, base);
	// 8240ABBC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8240ABC0: A09D0008  lhz r4, 8(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 8240ABC4: 4BFFF97D  bl 0x8240a540
	ctx.lr = 0x8240ABC8;
	sub_8240A540(ctx, base);
	// 8240ABC8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8240ABCC: 80DF0014  lwz r6, 0x14(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 8240ABD0: 80BF000C  lwz r5, 0xc(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8240ABD4: 809F0010  lwz r4, 0x10(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8240ABD8: 4BFFF7A1  bl 0x8240a378
	ctx.lr = 0x8240ABDC;
	sub_8240A378(ctx, base);
	// 8240ABDC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8240ABE0: 48000014  b 0x8240abf4
	pc = 0x8240ABF4; continue 'dispatch;
            }
            0x8240ABE4 => {
    //   block [0x8240ABE4..0x8240ABF4)
	// 8240ABE4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240ABE8: 386BD318  addi r3, r11, -0x2ce8
	ctx.r[3].s64 = ctx.r[11].s64 + -11496;
	// 8240ABEC: 4BEA8395  bl 0x822b2f80
	ctx.lr = 0x8240ABF0;
	sub_822B2F80(ctx, base);
	// 8240ABF0: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	pc = 0x8240ABF4; continue 'dispatch;
            }
            0x8240ABF4 => {
    //   block [0x8240ABF4..0x8240ABFC)
	// 8240ABF4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8240ABF8: 4812A514  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240AC00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8240AC00 size=132
    let mut pc: u32 = 0x8240AC00;
    'dispatch: loop {
        match pc {
            0x8240AC00 => {
    //   block [0x8240AC00..0x8240AC6C)
	// 8240AC00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240AC04: 4812A4B9  bl 0x825350bc
	ctx.lr = 0x8240AC08;
	sub_82535080(ctx, base);
	// 8240AC08: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240AC0C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8240AC10: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8240AC14: 2B040FFF  cmplwi cr6, r4, 0xfff
	ctx.cr[6].compare_u32(ctx.r[4].u32, 4095 as u32, &mut ctx.xer);
	// 8240AC18: 41990054  bgt cr6, 0x8240ac6c
	if ctx.cr[6].gt {
	pc = 0x8240AC6C; continue 'dispatch;
	}
	// 8240AC1C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8240AC20: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 8240AC24: 48004F6D  bl 0x8240fb90
	ctx.lr = 0x8240AC28;
	sub_8240FB90(ctx, base);
	// 8240AC28: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8240AC2C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8240AC30: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 8240AC34: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8240AC38: 83BF0014  lwz r29, 0x14(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 8240AC3C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8240AC40: 4BFFFE09  bl 0x8240aa48
	ctx.lr = 0x8240AC44;
	sub_8240AA48(ctx, base);
	// 8240AC44: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8240AC48: 889D0001  lbz r4, 1(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(1 as u32) ) } as u64;
	// 8240AC4C: 4BFFF8F5  bl 0x8240a540
	ctx.lr = 0x8240AC50;
	sub_8240A540(ctx, base);
	// 8240AC50: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8240AC54: 80DF0014  lwz r6, 0x14(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 8240AC58: 80BF000C  lwz r5, 0xc(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8240AC5C: 809F0010  lwz r4, 0x10(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8240AC60: 4BFFF819  bl 0x8240a478
	ctx.lr = 0x8240AC64;
	sub_8240A478(ctx, base);
	// 8240AC64: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8240AC68: 48000014  b 0x8240ac7c
	pc = 0x8240AC7C; continue 'dispatch;
            }
            0x8240AC6C => {
    //   block [0x8240AC6C..0x8240AC7C)
	// 8240AC6C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240AC70: 386BD318  addi r3, r11, -0x2ce8
	ctx.r[3].s64 = ctx.r[11].s64 + -11496;
	// 8240AC74: 4BEA830D  bl 0x822b2f80
	ctx.lr = 0x8240AC78;
	sub_822B2F80(ctx, base);
	// 8240AC78: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	pc = 0x8240AC7C; continue 'dispatch;
            }
            0x8240AC7C => {
    //   block [0x8240AC7C..0x8240AC84)
	// 8240AC7C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8240AC80: 4812A48C  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240AC88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8240AC88 size=296
    let mut pc: u32 = 0x8240AC88;
    'dispatch: loop {
        match pc {
            0x8240AC88 => {
    //   block [0x8240AC88..0x8240ACD0)
	// 8240AC88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240AC8C: 4812A421  bl 0x825350ac
	ctx.lr = 0x8240AC90;
	sub_82535080(ctx, base);
	// 8240AC90: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240AC94: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8240AC98: 7C992378  mr r25, r4
	ctx.r[25].u64 = ctx.r[4].u64;
	// 8240AC9C: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 8240ACA0: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 8240ACA4: 7CFC3B78  mr r28, r7
	ctx.r[28].u64 = ctx.r[7].u64;
	// 8240ACA8: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8240ACAC: 7D1B4378  mr r27, r8
	ctx.r[27].u64 = ctx.r[8].u64;
	// 8240ACB0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8240ACB4: 419A001C  beq cr6, 0x8240acd0
	if ctx.cr[6].eq {
	pc = 0x8240ACD0; continue 'dispatch;
	}
	// 8240ACB8: 4BFFF341  bl 0x82409ff8
	ctx.lr = 0x8240ACBC;
	sub_82409FF8(ctx, base);
	// 8240ACBC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8240ACC0: 4BFFF251  bl 0x82409f10
	ctx.lr = 0x8240ACC4;
	sub_82409F10(ctx, base);
	// 8240ACC4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8240ACC8: 809F0034  lwz r4, 0x34(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 8240ACCC: 4BFFFB65  bl 0x8240a830
	ctx.lr = 0x8240ACD0;
	sub_8240A830(ctx, base);
	pc = 0x8240ACD0; continue 'dispatch;
            }
            0x8240ACD0 => {
    //   block [0x8240ACD0..0x8240AD28)
	// 8240ACD0: 2B1E0FFF  cmplwi cr6, r30, 0xfff
	ctx.cr[6].compare_u32(ctx.r[30].u32, 4095 as u32, &mut ctx.xer);
	// 8240ACD4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8240ACD8: 419900C0  bgt cr6, 0x8240ad98
	if ctx.cr[6].gt {
	pc = 0x8240AD98; continue 'dispatch;
	}
	// 8240ACDC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8240ACE0: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 8240ACE4: 48004EAD  bl 0x8240fb90
	ctx.lr = 0x8240ACE8;
	sub_8240FB90(ctx, base);
	// 8240ACE8: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 8240ACEC: 817A0008  lwz r11, 8(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 8240ACF0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8240ACF4: 409A004C  bne cr6, 0x8240ad40
	if !ctx.cr[6].eq {
	pc = 0x8240AD40; continue 'dispatch;
	}
	// 8240ACF8: 2B190001  cmplwi cr6, r25, 1
	ctx.cr[6].compare_u32(ctx.r[25].u32, 1 as u32, &mut ctx.xer);
	// 8240ACFC: 41980088  blt cr6, 0x8240ad84
	if ctx.cr[6].lt {
	pc = 0x8240AD84; continue 'dispatch;
	}
	// 8240AD00: 419A0068  beq cr6, 0x8240ad68
	if ctx.cr[6].eq {
	pc = 0x8240AD68; continue 'dispatch;
	}
	// 8240AD04: 2B190004  cmplwi cr6, r25, 4
	ctx.cr[6].compare_u32(ctx.r[25].u32, 4 as u32, &mut ctx.xer);
	// 8240AD08: 419A0020  beq cr6, 0x8240ad28
	if ctx.cr[6].eq {
	pc = 0x8240AD28; continue 'dispatch;
	}
	// 8240AD0C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240AD10: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8240AD14: 386BD3A8  addi r3, r11, -0x2c58
	ctx.r[3].s64 = ctx.r[11].s64 + -11352;
	// 8240AD18: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 8240AD1C: 4BEA8265  bl 0x822b2f80
	ctx.lr = 0x8240AD20;
	sub_822B2F80(ctx, base);
	// 8240AD20: 3860FFFD  li r3, -3
	ctx.r[3].s64 = -3;
	// 8240AD24: 48000084  b 0x8240ada8
	pc = 0x8240ADA8; continue 'dispatch;
            }
            0x8240AD28 => {
    //   block [0x8240AD28..0x8240AD38)
	// 8240AD28: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8240AD2C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8240AD30: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8240AD34: 4BFFFECD  bl 0x8240ac00
	ctx.lr = 0x8240AD38;
	sub_8240AC00(ctx, base);
	pc = 0x8240AD38; continue 'dispatch;
            }
            0x8240AD38 => {
    //   block [0x8240AD38..0x8240AD40)
	// 8240AD38: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8240AD3C: 41800068  blt 0x8240ada4
	if ctx.cr[0].lt {
	pc = 0x8240ADA4; continue 'dispatch;
	}
	pc = 0x8240AD40; continue 'dispatch;
            }
            0x8240AD40 => {
    //   block [0x8240AD40..0x8240AD68)
	// 8240AD40: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8240AD44: 933F0014  stw r25, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[25].u32 ) };
	// 8240AD48: 93DF0018  stw r30, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[30].u32 ) };
	// 8240AD4C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8240AD50: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8240AD54: 917F0028  stw r11, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 8240AD58: 817A0008  lwz r11, 8(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 8240AD5C: 915F0024  stw r10, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[10].u32 ) };
	// 8240AD60: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 8240AD64: 48000044  b 0x8240ada8
	pc = 0x8240ADA8; continue 'dispatch;
            }
            0x8240AD68 => {
    //   block [0x8240AD68..0x8240AD84)
	// 8240AD68: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 8240AD6C: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 8240AD70: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8240AD74: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8240AD78: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8240AD7C: 4BFFFB6D  bl 0x8240a8e8
	ctx.lr = 0x8240AD80;
	sub_8240A8E8(ctx, base);
	// 8240AD80: 4BFFFFB8  b 0x8240ad38
	pc = 0x8240AD38; continue 'dispatch;
            }
            0x8240AD84 => {
    //   block [0x8240AD84..0x8240AD98)
	// 8240AD84: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8240AD88: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8240AD8C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8240AD90: 4BFFFDE9  bl 0x8240ab78
	ctx.lr = 0x8240AD94;
	sub_8240AB78(ctx, base);
	// 8240AD94: 4BFFFFA4  b 0x8240ad38
	pc = 0x8240AD38; continue 'dispatch;
            }
            0x8240AD98 => {
    //   block [0x8240AD98..0x8240ADA4)
	// 8240AD98: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240AD9C: 386BD360  addi r3, r11, -0x2ca0
	ctx.r[3].s64 = ctx.r[11].s64 + -11424;
	// 8240ADA0: 4BEA81E1  bl 0x822b2f80
	ctx.lr = 0x8240ADA4;
	sub_822B2F80(ctx, base);
	pc = 0x8240ADA4; continue 'dispatch;
            }
            0x8240ADA4 => {
    //   block [0x8240ADA4..0x8240ADA8)
	// 8240ADA4: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	pc = 0x8240ADA8; continue 'dispatch;
            }
            0x8240ADA8 => {
    //   block [0x8240ADA8..0x8240ADB0)
	// 8240ADA8: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8240ADAC: 4812A350  b 0x825350fc
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240ADB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8240ADB0 size=72
    let mut pc: u32 = 0x8240ADB0;
    'dispatch: loop {
        match pc {
            0x8240ADB0 => {
    //   block [0x8240ADB0..0x8240ADCC)
	// 8240ADB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240ADB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240ADB8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8240ADBC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8240ADC0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240ADC4: 3BE32A04  addi r31, r3, 0x2a04
	ctx.r[31].s64 = ctx.r[3].s64 + 10756;
	// 8240ADC8: 3BC000BF  li r30, 0xbf
	ctx.r[30].s64 = 191;
	pc = 0x8240ADCC; continue 'dispatch;
            }
            0x8240ADCC => {
    //   block [0x8240ADCC..0x8240ADF8)
	// 8240ADCC: 3BFFFFC8  addi r31, r31, -0x38
	ctx.r[31].s64 = ctx.r[31].s64 + -56;
	// 8240ADD0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8240ADD4: 4BFFFAB5  bl 0x8240a888
	ctx.lr = 0x8240ADD8;
	sub_8240A888(ctx, base);
	// 8240ADD8: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8240ADDC: 4080FFF0  bge 0x8240adcc
	if !ctx.cr[0].lt {
	pc = 0x8240ADCC; continue 'dispatch;
	}
	// 8240ADE0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8240ADE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240ADE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240ADEC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8240ADF0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8240ADF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240ADF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8240ADF8 size=120
    let mut pc: u32 = 0x8240ADF8;
    'dispatch: loop {
        match pc {
            0x8240ADF8 => {
    //   block [0x8240ADF8..0x8240AE24)
	// 8240ADF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240ADFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240AE00: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8240AE04: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8240AE08: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240AE0C: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 8240AE10: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8240AE14: 3BC30004  addi r30, r3, 4
	ctx.r[30].s64 = ctx.r[3].s64 + 4;
	// 8240AE18: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8240AE1C: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 8240AE20: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	pc = 0x8240AE24; continue 'dispatch;
            }
            0x8240AE24 => {
    //   block [0x8240AE24..0x8240AE50)
	// 8240AE24: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8240AE28: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8240AE2C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8240AE30: 4BFFF081  bl 0x82409eb0
	ctx.lr = 0x8240AE34;
	sub_82409EB0(ctx, base);
	// 8240AE34: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 8240AE38: 419A0030  beq cr6, 0x8240ae68
	if ctx.cr[6].eq {
	pc = 0x8240AE68; continue 'dispatch;
	}
	// 8240AE3C: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 8240AE40: 3BDE0038  addi r30, r30, 0x38
	ctx.r[30].s64 = ctx.r[30].s64 + 56;
	// 8240AE44: 2F1F00C0  cmpwi cr6, r31, 0xc0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 192, &mut ctx.xer);
	// 8240AE48: 4198FFDC  blt cr6, 0x8240ae24
	if ctx.cr[6].lt {
	pc = 0x8240AE24; continue 'dispatch;
	}
	// 8240AE4C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	pc = 0x8240AE50; continue 'dispatch;
            }
            0x8240AE50 => {
    //   block [0x8240AE50..0x8240AE68)
	// 8240AE50: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8240AE54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240AE58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240AE5C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8240AE60: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8240AE64: 4E800020  blr
	return;
            }
            0x8240AE68 => {
    //   block [0x8240AE68..0x8240AE70)
	// 8240AE68: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 8240AE6C: 4BFFFFE4  b 0x8240ae50
	pc = 0x8240AE50; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240AE70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8240AE70 size=60
    let mut pc: u32 = 0x8240AE70;
    'dispatch: loop {
        match pc {
            0x8240AE70 => {
    //   block [0x8240AE70..0x8240AE88)
	// 8240AE70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240AE74: 4812A249  bl 0x825350bc
	ctx.lr = 0x8240AE78;
	sub_82535080(ctx, base);
	// 8240AE78: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240AE7C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8240AE80: 3BC30004  addi r30, r3, 4
	ctx.r[30].s64 = ctx.r[3].s64 + 4;
	// 8240AE84: 3BE000C0  li r31, 0xc0
	ctx.r[31].s64 = 192;
	pc = 0x8240AE88; continue 'dispatch;
            }
            0x8240AE88 => {
    //   block [0x8240AE88..0x8240AEAC)
	// 8240AE88: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8240AE8C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8240AE90: 4BFFFA01  bl 0x8240a890
	ctx.lr = 0x8240AE94;
	sub_8240A890(ctx, base);
	// 8240AE94: 37FFFFFF  addic. r31, r31, -1
	ctx.xer.ca = (ctx.r[31].u32 > (!(-1 as u32)));
	ctx.r[31].s64 = ctx.r[31].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 8240AE98: 3BDE0038  addi r30, r30, 0x38
	ctx.r[30].s64 = ctx.r[30].s64 + 56;
	// 8240AE9C: 4082FFEC  bne 0x8240ae88
	if !ctx.cr[0].eq {
	pc = 0x8240AE88; continue 'dispatch;
	}
	// 8240AEA0: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8240AEA4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8240AEA8: 4812A264  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240AEB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8240AEB0 size=16
    let mut pc: u32 = 0x8240AEB0;
    'dispatch: loop {
        match pc {
            0x8240AEB0 => {
    //   block [0x8240AEB0..0x8240AEC0)
	// 8240AEB0: 2B0400C0  cmplwi cr6, r4, 0xc0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 192 as u32, &mut ctx.xer);
	// 8240AEB4: 4198000C  blt cr6, 0x8240aec0
	if ctx.cr[6].lt {
		crate::recompiler::externs::call(ctx, base, 0x8240AEC0);
		return;
	}
	// 8240AEB8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8240AEBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240AED0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8240AED0 size=56
    let mut pc: u32 = 0x8240AED0;
    'dispatch: loop {
        match pc {
            0x8240AED0 => {
    //   block [0x8240AED0..0x8240AEE8)
	// 8240AED0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240AED4: 4812A1E9  bl 0x825350bc
	ctx.lr = 0x8240AED8;
	sub_82535080(ctx, base);
	// 8240AED8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240AEDC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8240AEE0: 3BE000BF  li r31, 0xbf
	ctx.r[31].s64 = 191;
	// 8240AEE4: 3BDD0004  addi r30, r29, 4
	ctx.r[30].s64 = ctx.r[29].s64 + 4;
	pc = 0x8240AEE8; continue 'dispatch;
            }
            0x8240AEE8 => {
    //   block [0x8240AEE8..0x8240AF08)
	// 8240AEE8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8240AEEC: 4BFFEF7D  bl 0x82409e68
	ctx.lr = 0x8240AEF0;
	sub_82409E68(ctx, base);
	// 8240AEF0: 37FFFFFF  addic. r31, r31, -1
	ctx.xer.ca = (ctx.r[31].u32 > (!(-1 as u32)));
	ctx.r[31].s64 = ctx.r[31].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 8240AEF4: 3BDE0038  addi r30, r30, 0x38
	ctx.r[30].s64 = ctx.r[30].s64 + 56;
	// 8240AEF8: 4080FFF0  bge 0x8240aee8
	if !ctx.cr[0].lt {
	pc = 0x8240AEE8; continue 'dispatch;
	}
	// 8240AEFC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8240AF00: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8240AF04: 4812A208  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240AF08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8240AF08 size=204
    let mut pc: u32 = 0x8240AF08;
    'dispatch: loop {
        match pc {
            0x8240AF08 => {
    //   block [0x8240AF08..0x8240AF88)
	// 8240AF08: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 8240AF0C: 1D640164  mulli r11, r4, 0x164
	ctx.r[11].s32 = ((ctx.r[4].s32 as i64 * 356 as i64) as i32);
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 8240AF10: C00A1FF8  lfs f0, 0x1ff8(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8184 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240AF14: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 8240AF18: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 8240AF1C: 38C0FFFF  li r6, -1
	ctx.r[6].s64 = -1;
	// 8240AF20: 3CE03F80  lis r7, 0x3f80
	ctx.r[7].s64 = 1065353216;
	// 8240AF24: 39200024  li r9, 0x24
	ctx.r[9].s64 = 36;
	// 8240AF28: 390B0050  addi r8, r11, 0x50
	ctx.r[8].s64 = ctx.r[11].s64 + 80;
	// 8240AF2C: C1AA1850  lfs f13, 0x1850(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(6224 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8240AF30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8240AF34: D00B001C  stfs f0, 0x1c(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 8240AF38: 90CB0004  stw r6, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[6].u32 ) };
	// 8240AF3C: D00B0020  stfs f0, 0x20(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), tmp.u32 ) };
	// 8240AF40: D00B0024  stfs f0, 0x24(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(36 as u32), tmp.u32 ) };
	// 8240AF44: D1AB0028  stfs f13, 0x28(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(40 as u32), tmp.u32 ) };
	// 8240AF48: D00B002C  stfs f0, 0x2c(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(44 as u32), tmp.u32 ) };
	// 8240AF4C: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8240AF50: D00B003C  stfs f0, 0x3c(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(60 as u32), tmp.u32 ) };
	// 8240AF54: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8240AF58: D00B0040  stfs f0, 0x40(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(64 as u32), tmp.u32 ) };
	// 8240AF5C: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 8240AF60: D00B0044  stfs f0, 0x44(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(68 as u32), tmp.u32 ) };
	// 8240AF64: 914B0010  stw r10, 0x10(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 8240AF68: 914B0014  stw r10, 0x14(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 8240AF6C: 914B0018  stw r10, 0x18(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[10].u32 ) };
	// 8240AF70: 914B0030  stw r10, 0x30(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(48 as u32), ctx.r[10].u32 ) };
	// 8240AF74: 914B0038  stw r10, 0x38(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(56 as u32), ctx.r[10].u32 ) };
	// 8240AF78: 914B0034  stw r10, 0x34(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(52 as u32), ctx.r[10].u32 ) };
	// 8240AF7C: 914B0048  stw r10, 0x48(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(72 as u32), ctx.r[10].u32 ) };
	// 8240AF80: 914B004C  stw r10, 0x4c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(76 as u32), ctx.r[10].u32 ) };
	// 8240AF84: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	pc = 0x8240AF88; continue 'dispatch;
            }
            0x8240AF88 => {
    //   block [0x8240AF88..0x8240AFD4)
	// 8240AF88: 90E80000  stw r7, 0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 8240AF8C: 39080004  addi r8, r8, 4
	ctx.r[8].s64 = ctx.r[8].s64 + 4;
	// 8240AF90: 4200FFF8  bdnz 0x8240af88
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x8240AF88; continue 'dispatch;
	}
	// 8240AF94: D00B00E0  stfs f0, 0xe0(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(224 as u32), tmp.u32 ) };
	// 8240AF98: 90CB012C  stw r6, 0x12c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(300 as u32), ctx.r[6].u32 ) };
	// 8240AF9C: D1AB00E4  stfs f13, 0xe4(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(228 as u32), tmp.u32 ) };
	// 8240AFA0: 914B0154  stw r10, 0x154(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(340 as u32), ctx.r[10].u32 ) };
	// 8240AFA4: D1AB00E8  stfs f13, 0xe8(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(232 as u32), tmp.u32 ) };
	// 8240AFA8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8240AFAC: D1AB00EC  stfs f13, 0xec(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(236 as u32), tmp.u32 ) };
	// 8240AFB0: D1AB00F0  stfs f13, 0xf0(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(240 as u32), tmp.u32 ) };
	// 8240AFB4: D1AB00F4  stfs f13, 0xf4(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(244 as u32), tmp.u32 ) };
	// 8240AFB8: D00B0148  stfs f0, 0x148(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(328 as u32), tmp.u32 ) };
	// 8240AFBC: D1AB014C  stfs f13, 0x14c(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(332 as u32), tmp.u32 ) };
	// 8240AFC0: D00B0150  stfs f0, 0x150(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(336 as u32), tmp.u32 ) };
	// 8240AFC4: D00B0158  stfs f0, 0x158(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(344 as u32), tmp.u32 ) };
	// 8240AFC8: D1AB015C  stfs f13, 0x15c(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(348 as u32), tmp.u32 ) };
	// 8240AFCC: D1AB0160  stfs f13, 0x160(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(352 as u32), tmp.u32 ) };
	// 8240AFD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240AFD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8240AFD8 size=116
    let mut pc: u32 = 0x8240AFD8;
    'dispatch: loop {
        match pc {
            0x8240AFD8 => {
    //   block [0x8240AFD8..0x8240B01C)
	// 8240AFD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240AFDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240AFE0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240AFE4: 2B0400BF  cmplwi cr6, r4, 0xbf
	ctx.cr[6].compare_u32(ctx.r[4].u32, 191 as u32, &mut ctx.xer);
	// 8240AFE8: 41990040  bgt cr6, 0x8240b028
	if ctx.cr[6].gt {
	pc = 0x8240B028; continue 'dispatch;
	}
	// 8240AFEC: 1D640164  mulli r11, r4, 0x164
	ctx.r[11].s32 = ((ctx.r[4].s32 as i64 * 356 as i64) as i32);
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 8240AFF0: 7D4B1A14  add r10, r11, r3
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 8240AFF4: 816A0008  lwz r11, 8(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 8240AFF8: 2C0B0000  cmpwi r11, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8240AFFC: 41820020  beq 0x8240b01c
	if ctx.cr[0].eq {
	pc = 0x8240B01C; continue 'dispatch;
	}
	// 8240B000: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 8240B004: 419A0018  beq cr6, 0x8240b01c
	if ctx.cr[6].eq {
	pc = 0x8240B01C; continue 'dispatch;
	}
	// 8240B008: 2F0B0006  cmpwi cr6, r11, 6
	ctx.cr[6].compare_i32(ctx.r[11].s32, 6, &mut ctx.xer);
	// 8240B00C: 419A0010  beq cr6, 0x8240b01c
	if ctx.cr[6].eq {
	pc = 0x8240B01C; continue 'dispatch;
	}
	// 8240B010: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 8240B014: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8240B018: 48000008  b 0x8240b020
	pc = 0x8240B020; continue 'dispatch;
            }
            0x8240B01C => {
    //   block [0x8240B01C..0x8240B020)
	// 8240B01C: 4BFFFEED  bl 0x8240af08
	ctx.lr = 0x8240B020;
	sub_8240AF08(ctx, base);
	pc = 0x8240B020; continue 'dispatch;
            }
            0x8240B020 => {
    //   block [0x8240B020..0x8240B028)
	// 8240B020: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8240B024: 48000018  b 0x8240b03c
	pc = 0x8240B03C; continue 'dispatch;
            }
            0x8240B028 => {
    //   block [0x8240B028..0x8240B03C)
	// 8240B028: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240B02C: 386BD3F4  addi r3, r11, -0x2c0c
	ctx.r[3].s64 = ctx.r[11].s64 + -11276;
	// 8240B030: 4BEA7F51  bl 0x822b2f80
	ctx.lr = 0x8240B034;
	sub_822B2F80(ctx, base);
	// 8240B034: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 8240B038: 60630016  ori r3, r3, 0x16
	ctx.r[3].u64 = ctx.r[3].u64 | 22;
	pc = 0x8240B03C; continue 'dispatch;
            }
            0x8240B03C => {
    //   block [0x8240B03C..0x8240B04C)
	// 8240B03C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8240B040: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240B044: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240B048: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240B050(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8240B050 size=68
    let mut pc: u32 = 0x8240B050;
    'dispatch: loop {
        match pc {
            0x8240B050 => {
    //   block [0x8240B050..0x8240B070)
	// 8240B050: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240B054: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240B058: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240B05C: 2B0400BF  cmplwi cr6, r4, 0xbf
	ctx.cr[6].compare_u32(ctx.r[4].u32, 191 as u32, &mut ctx.xer);
	// 8240B060: 41990010  bgt cr6, 0x8240b070
	if ctx.cr[6].gt {
	pc = 0x8240B070; continue 'dispatch;
	}
	// 8240B064: 4BFFFEA5  bl 0x8240af08
	ctx.lr = 0x8240B068;
	sub_8240AF08(ctx, base);
	// 8240B068: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8240B06C: 48000018  b 0x8240b084
	pc = 0x8240B084; continue 'dispatch;
            }
            0x8240B070 => {
    //   block [0x8240B070..0x8240B084)
	// 8240B070: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240B074: 386BD438  addi r3, r11, -0x2bc8
	ctx.r[3].s64 = ctx.r[11].s64 + -11208;
	// 8240B078: 4BEA7F09  bl 0x822b2f80
	ctx.lr = 0x8240B07C;
	sub_822B2F80(ctx, base);
	// 8240B07C: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 8240B080: 60630016  ori r3, r3, 0x16
	ctx.r[3].u64 = ctx.r[3].u64 | 22;
	pc = 0x8240B084; continue 'dispatch;
            }
            0x8240B084 => {
    //   block [0x8240B084..0x8240B094)
	// 8240B084: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8240B088: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240B08C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240B090: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240B098(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8240B098 size=84
    let mut pc: u32 = 0x8240B098;
    'dispatch: loop {
        match pc {
            0x8240B098 => {
    //   block [0x8240B098..0x8240B0B4)
	// 8240B098: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240B09C: 4812A021  bl 0x825350bc
	ctx.lr = 0x8240B0A0;
	sub_82535080(ctx, base);
	// 8240B0A0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240B0A4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240B0A8: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8240B0AC: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8240B0B0: 3BCBD438  addi r30, r11, -0x2bc8
	ctx.r[30].s64 = ctx.r[11].s64 + -11208;
	pc = 0x8240B0B4; continue 'dispatch;
            }
            0x8240B0B4 => {
    //   block [0x8240B0B4..0x8240B0CC)
	// 8240B0B4: 2B1F00BF  cmplwi cr6, r31, 0xbf
	ctx.cr[6].compare_u32(ctx.r[31].u32, 191 as u32, &mut ctx.xer);
	// 8240B0B8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8240B0BC: 41990010  bgt cr6, 0x8240b0cc
	if ctx.cr[6].gt {
	pc = 0x8240B0CC; continue 'dispatch;
	}
	// 8240B0C0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8240B0C4: 4BFFFE45  bl 0x8240af08
	ctx.lr = 0x8240B0C8;
	sub_8240AF08(ctx, base);
	// 8240B0C8: 4800000C  b 0x8240b0d4
	pc = 0x8240B0D4; continue 'dispatch;
            }
            0x8240B0CC => {
    //   block [0x8240B0CC..0x8240B0D4)
	// 8240B0CC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8240B0D0: 4BEA7EB1  bl 0x822b2f80
	ctx.lr = 0x8240B0D4;
	sub_822B2F80(ctx, base);
	pc = 0x8240B0D4; continue 'dispatch;
            }
            0x8240B0D4 => {
    //   block [0x8240B0D4..0x8240B0EC)
	// 8240B0D4: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 8240B0D8: 2F1F00C0  cmpwi cr6, r31, 0xc0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 192, &mut ctx.xer);
	// 8240B0DC: 4198FFD8  blt cr6, 0x8240b0b4
	if ctx.cr[6].lt {
	pc = 0x8240B0B4; continue 'dispatch;
	}
	// 8240B0E0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8240B0E4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8240B0E8: 4812A024  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240B0F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8240B0F0 size=156
    let mut pc: u32 = 0x8240B0F0;
    'dispatch: loop {
        match pc {
            0x8240B0F0 => {
    //   block [0x8240B0F0..0x8240B13C)
	// 8240B0F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240B0F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240B0F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240B0FC: 2B0400BF  cmplwi cr6, r4, 0xbf
	ctx.cr[6].compare_u32(ctx.r[4].u32, 191 as u32, &mut ctx.xer);
	// 8240B100: 41990068  bgt cr6, 0x8240b168
	if ctx.cr[6].gt {
	pc = 0x8240B168; continue 'dispatch;
	}
	// 8240B104: 1D640164  mulli r11, r4, 0x164
	ctx.r[11].s32 = ((ctx.r[4].s32 as i64 * 356 as i64) as i32);
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 8240B108: 7D4B1A14  add r10, r11, r3
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 8240B10C: 816A0008  lwz r11, 8(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 8240B110: 2F0B0006  cmpwi cr6, r11, 6
	ctx.cr[6].compare_i32(ctx.r[11].s32, 6, &mut ctx.xer);
	// 8240B114: 419A004C  beq cr6, 0x8240b160
	if ctx.cr[6].eq {
	pc = 0x8240B160; continue 'dispatch;
	}
	// 8240B118: 2B060000  cmplwi cr6, r6, 0
	ctx.cr[6].compare_u32(ctx.r[6].u32, 0 as u32, &mut ctx.xer);
	// 8240B11C: 419A0030  beq cr6, 0x8240b14c
	if ctx.cr[6].eq {
	pc = 0x8240B14C; continue 'dispatch;
	}
	// 8240B120: 2B050001  cmplwi cr6, r5, 1
	ctx.cr[6].compare_u32(ctx.r[5].u32, 1 as u32, &mut ctx.xer);
	// 8240B124: 409A0018  bne cr6, 0x8240b13c
	if !ctx.cr[6].eq {
	pc = 0x8240B13C; continue 'dispatch;
	}
	// 8240B128: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 8240B12C: 409A0034  bne cr6, 0x8240b160
	if !ctx.cr[6].eq {
	pc = 0x8240B160; continue 'dispatch;
	}
	// 8240B130: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8240B134: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8240B138: 48000028  b 0x8240b160
	pc = 0x8240B160; continue 'dispatch;
            }
            0x8240B13C => {
    //   block [0x8240B13C..0x8240B14C)
	// 8240B13C: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 8240B140: 419A0020  beq cr6, 0x8240b160
	if ctx.cr[6].eq {
	pc = 0x8240B160; continue 'dispatch;
	}
	// 8240B144: 2F0B0005  cmpwi cr6, r11, 5
	ctx.cr[6].compare_i32(ctx.r[11].s32, 5, &mut ctx.xer);
	// 8240B148: 419A0018  beq cr6, 0x8240b160
	if ctx.cr[6].eq {
	pc = 0x8240B160; continue 'dispatch;
	}
	pc = 0x8240B14C; continue 'dispatch;
            }
            0x8240B14C => {
    //   block [0x8240B14C..0x8240B160)
	// 8240B14C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8240B150: 419A0010  beq cr6, 0x8240b160
	if ctx.cr[6].eq {
	pc = 0x8240B160; continue 'dispatch;
	}
	// 8240B154: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8240B158: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8240B15C: 4BFFFDAD  bl 0x8240af08
	ctx.lr = 0x8240B160;
	sub_8240AF08(ctx, base);
	pc = 0x8240B160; continue 'dispatch;
            }
            0x8240B160 => {
    //   block [0x8240B160..0x8240B168)
	// 8240B160: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8240B164: 48000018  b 0x8240b17c
	pc = 0x8240B17C; continue 'dispatch;
            }
            0x8240B168 => {
    //   block [0x8240B168..0x8240B17C)
	// 8240B168: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240B16C: 386BD480  addi r3, r11, -0x2b80
	ctx.r[3].s64 = ctx.r[11].s64 + -11136;
	// 8240B170: 4BEA7E11  bl 0x822b2f80
	ctx.lr = 0x8240B174;
	sub_822B2F80(ctx, base);
	// 8240B174: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 8240B178: 60630016  ori r3, r3, 0x16
	ctx.r[3].u64 = ctx.r[3].u64 | 22;
	pc = 0x8240B17C; continue 'dispatch;
            }
            0x8240B17C => {
    //   block [0x8240B17C..0x8240B18C)
	// 8240B17C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8240B180: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240B184: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240B188: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240B190(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8240B190 size=72
    let mut pc: u32 = 0x8240B190;
    'dispatch: loop {
        match pc {
            0x8240B190 => {
    //   block [0x8240B190..0x8240B198)
	// 8240B190: 39630008  addi r11, r3, 8
	ctx.r[11].s64 = ctx.r[3].s64 + 8;
	// 8240B194: 390000C0  li r8, 0xc0
	ctx.r[8].s64 = 192;
	pc = 0x8240B198; continue 'dispatch;
            }
            0x8240B198 => {
    //   block [0x8240B198..0x8240B1B0)
	// 8240B198: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8240B19C: 2F090001  cmpwi cr6, r9, 1
	ctx.cr[6].compare_i32(ctx.r[9].s32, 1, &mut ctx.xer);
	// 8240B1A0: 409A0010  bne cr6, 0x8240b1b0
	if !ctx.cr[6].eq {
	pc = 0x8240B1B0; continue 'dispatch;
	}
	// 8240B1A4: 814B0040  lwz r10, 0x40(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(64 as u32) ) } as u64;
	// 8240B1A8: 7D4A2214  add r10, r10, r4
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[4].u64;
	// 8240B1AC: 914B0040  stw r10, 0x40(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(64 as u32), ctx.r[10].u32 ) };
	pc = 0x8240B1B0; continue 'dispatch;
            }
            0x8240B1B0 => {
    //   block [0x8240B1B0..0x8240B1C4)
	// 8240B1B0: 2F090004  cmpwi cr6, r9, 4
	ctx.cr[6].compare_i32(ctx.r[9].s32, 4, &mut ctx.xer);
	// 8240B1B4: 409A0010  bne cr6, 0x8240b1c4
	if !ctx.cr[6].eq {
	pc = 0x8240B1C4; continue 'dispatch;
	}
	// 8240B1B8: 814B0044  lwz r10, 0x44(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(68 as u32) ) } as u64;
	// 8240B1BC: 7D4A2214  add r10, r10, r4
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[4].u64;
	// 8240B1C0: 914B0044  stw r10, 0x44(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(68 as u32), ctx.r[10].u32 ) };
	pc = 0x8240B1C4; continue 'dispatch;
            }
            0x8240B1C4 => {
    //   block [0x8240B1C4..0x8240B1D8)
	// 8240B1C4: 3508FFFF  addic. r8, r8, -1
	ctx.xer.ca = (ctx.r[8].u32 > (!(-1 as u32)));
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 8240B1C8: 396B0164  addi r11, r11, 0x164
	ctx.r[11].s64 = ctx.r[11].s64 + 356;
	// 8240B1CC: 4082FFCC  bne 0x8240b198
	if !ctx.cr[0].eq {
	pc = 0x8240B198; continue 'dispatch;
	}
	// 8240B1D0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8240B1D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240B1D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8240B1D8 size=68
    let mut pc: u32 = 0x8240B1D8;
    'dispatch: loop {
        match pc {
            0x8240B1D8 => {
    //   block [0x8240B1D8..0x8240B1FC)
	// 8240B1D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240B1DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240B1E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240B1E4: 2B0400BF  cmplwi cr6, r4, 0xbf
	ctx.cr[6].compare_u32(ctx.r[4].u32, 191 as u32, &mut ctx.xer);
	// 8240B1E8: 41990014  bgt cr6, 0x8240b1fc
	if ctx.cr[6].gt {
	pc = 0x8240B1FC; continue 'dispatch;
	}
	// 8240B1EC: 1D640164  mulli r11, r4, 0x164
	ctx.r[11].s32 = ((ctx.r[4].s32 as i64 * 356 as i64) as i32);
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 8240B1F0: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 8240B1F4: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8240B1F8: 48000014  b 0x8240b20c
	pc = 0x8240B20C; continue 'dispatch;
            }
            0x8240B1FC => {
    //   block [0x8240B1FC..0x8240B20C)
	// 8240B1FC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240B200: 386BD4C4  addi r3, r11, -0x2b3c
	ctx.r[3].s64 = ctx.r[11].s64 + -11068;
	// 8240B204: 4BEA7D7D  bl 0x822b2f80
	ctx.lr = 0x8240B208;
	sub_822B2F80(ctx, base);
	// 8240B208: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	pc = 0x8240B20C; continue 'dispatch;
            }
            0x8240B20C => {
    //   block [0x8240B20C..0x8240B21C)
	// 8240B20C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8240B210: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240B214: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240B218: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240B220(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8240B220 size=80
    let mut pc: u32 = 0x8240B220;
    'dispatch: loop {
        match pc {
            0x8240B220 => {
    //   block [0x8240B220..0x8240B24C)
	// 8240B220: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240B224: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240B228: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240B22C: 2B0400BF  cmplwi cr6, r4, 0xbf
	ctx.cr[6].compare_u32(ctx.r[4].u32, 191 as u32, &mut ctx.xer);
	// 8240B230: 4199001C  bgt cr6, 0x8240b24c
	if ctx.cr[6].gt {
	pc = 0x8240B24C; continue 'dispatch;
	}
	// 8240B234: 1D640164  mulli r11, r4, 0x164
	ctx.r[11].s32 = ((ctx.r[4].s32 as i64 * 356 as i64) as i32);
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 8240B238: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 8240B23C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8240B240: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8240B244: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8240B248: 48000018  b 0x8240b260
	pc = 0x8240B260; continue 'dispatch;
            }
            0x8240B24C => {
    //   block [0x8240B24C..0x8240B260)
	// 8240B24C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240B250: 386BD500  addi r3, r11, -0x2b00
	ctx.r[3].s64 = ctx.r[11].s64 + -11008;
	// 8240B254: 4BEA7D2D  bl 0x822b2f80
	ctx.lr = 0x8240B258;
	sub_822B2F80(ctx, base);
	// 8240B258: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 8240B25C: 60630016  ori r3, r3, 0x16
	ctx.r[3].u64 = ctx.r[3].u64 | 22;
	pc = 0x8240B260; continue 'dispatch;
            }
            0x8240B260 => {
    //   block [0x8240B260..0x8240B270)
	// 8240B260: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8240B264: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240B268: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240B26C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240B270(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8240B270 size=80
    let mut pc: u32 = 0x8240B270;
    'dispatch: loop {
        match pc {
            0x8240B270 => {
    //   block [0x8240B270..0x8240B29C)
	// 8240B270: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240B274: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240B278: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240B27C: 2B0400BF  cmplwi cr6, r4, 0xbf
	ctx.cr[6].compare_u32(ctx.r[4].u32, 191 as u32, &mut ctx.xer);
	// 8240B280: 4199001C  bgt cr6, 0x8240b29c
	if ctx.cr[6].gt {
	pc = 0x8240B29C; continue 'dispatch;
	}
	// 8240B284: 1D640164  mulli r11, r4, 0x164
	ctx.r[11].s32 = ((ctx.r[4].s32 as i64 * 356 as i64) as i32);
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 8240B288: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 8240B28C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8240B290: 816B0030  lwz r11, 0x30(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 8240B294: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8240B298: 48000018  b 0x8240b2b0
	pc = 0x8240B2B0; continue 'dispatch;
            }
            0x8240B29C => {
    //   block [0x8240B29C..0x8240B2B0)
	// 8240B29C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240B2A0: 386BD540  addi r3, r11, -0x2ac0
	ctx.r[3].s64 = ctx.r[11].s64 + -10944;
	// 8240B2A4: 4BEA7CDD  bl 0x822b2f80
	ctx.lr = 0x8240B2A8;
	sub_822B2F80(ctx, base);
	// 8240B2A8: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 8240B2AC: 60630016  ori r3, r3, 0x16
	ctx.r[3].u64 = ctx.r[3].u64 | 22;
	pc = 0x8240B2B0; continue 'dispatch;
            }
            0x8240B2B0 => {
    //   block [0x8240B2B0..0x8240B2C0)
	// 8240B2B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8240B2B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240B2B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240B2BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240B2C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8240B2C0 size=68
    let mut pc: u32 = 0x8240B2C0;
    'dispatch: loop {
        match pc {
            0x8240B2C0 => {
    //   block [0x8240B2C0..0x8240B2E4)
	// 8240B2C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240B2C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240B2C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240B2CC: 2B0400BF  cmplwi cr6, r4, 0xbf
	ctx.cr[6].compare_u32(ctx.r[4].u32, 191 as u32, &mut ctx.xer);
	// 8240B2D0: 41990014  bgt cr6, 0x8240b2e4
	if ctx.cr[6].gt {
	pc = 0x8240B2E4; continue 'dispatch;
	}
	// 8240B2D4: 1D640164  mulli r11, r4, 0x164
	ctx.r[11].s32 = ((ctx.r[4].s32 as i64 * 356 as i64) as i32);
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 8240B2D8: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 8240B2DC: 806B0014  lwz r3, 0x14(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 8240B2E0: 48000014  b 0x8240b2f4
	pc = 0x8240B2F4; continue 'dispatch;
            }
            0x8240B2E4 => {
    //   block [0x8240B2E4..0x8240B2F4)
	// 8240B2E4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240B2E8: 386BD580  addi r3, r11, -0x2a80
	ctx.r[3].s64 = ctx.r[11].s64 + -10880;
	// 8240B2EC: 4BEA7C95  bl 0x822b2f80
	ctx.lr = 0x8240B2F0;
	sub_822B2F80(ctx, base);
	// 8240B2F0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	pc = 0x8240B2F4; continue 'dispatch;
            }
            0x8240B2F4 => {
    //   block [0x8240B2F4..0x8240B304)
	// 8240B2F4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8240B2F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240B2FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240B300: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240B308(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8240B308 size=88
    let mut pc: u32 = 0x8240B308;
    'dispatch: loop {
        match pc {
            0x8240B308 => {
    //   block [0x8240B308..0x8240B33C)
	// 8240B308: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240B30C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240B310: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240B314: 7CAA2B78  mr r10, r5
	ctx.r[10].u64 = ctx.r[5].u64;
	// 8240B318: 2B0400BF  cmplwi cr6, r4, 0xbf
	ctx.cr[6].compare_u32(ctx.r[4].u32, 191 as u32, &mut ctx.xer);
	// 8240B31C: 41990020  bgt cr6, 0x8240b33c
	if ctx.cr[6].gt {
	pc = 0x8240B33C; continue 'dispatch;
	}
	// 8240B320: 1D640164  mulli r11, r4, 0x164
	ctx.r[11].s32 = ((ctx.r[4].s32 as i64 * 356 as i64) as i32);
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 8240B324: 7C6B1A14  add r3, r11, r3
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 8240B328: 38A00164  li r5, 0x164
	ctx.r[5].s64 = 356;
	// 8240B32C: 7D445378  mr r4, r10
	ctx.r[4].u64 = ctx.r[10].u64;
	// 8240B330: 4BFB58D1  bl 0x823c0c00
	ctx.lr = 0x8240B334;
	sub_823C0C00(ctx, base);
	// 8240B334: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8240B338: 48000018  b 0x8240b350
	pc = 0x8240B350; continue 'dispatch;
            }
            0x8240B33C => {
    //   block [0x8240B33C..0x8240B350)
	// 8240B33C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240B340: 386BD5C8  addi r3, r11, -0x2a38
	ctx.r[3].s64 = ctx.r[11].s64 + -10808;
	// 8240B344: 4BEA7C3D  bl 0x822b2f80
	ctx.lr = 0x8240B348;
	sub_822B2F80(ctx, base);
	// 8240B348: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 8240B34C: 60630016  ori r3, r3, 0x16
	ctx.r[3].u64 = ctx.r[3].u64 | 22;
	pc = 0x8240B350; continue 'dispatch;
            }
            0x8240B350 => {
    //   block [0x8240B350..0x8240B360)
	// 8240B350: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8240B354: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240B358: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240B35C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240B360(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8240B360 size=88
    let mut pc: u32 = 0x8240B360;
    'dispatch: loop {
        match pc {
            0x8240B360 => {
    //   block [0x8240B360..0x8240B394)
	// 8240B360: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240B364: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240B368: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240B36C: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 8240B370: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 8240B374: 2B0400BF  cmplwi cr6, r4, 0xbf
	ctx.cr[6].compare_u32(ctx.r[4].u32, 191 as u32, &mut ctx.xer);
	// 8240B378: 4199001C  bgt cr6, 0x8240b394
	if ctx.cr[6].gt {
	pc = 0x8240B394; continue 'dispatch;
	}
	// 8240B37C: 1D640164  mulli r11, r4, 0x164
	ctx.r[11].s32 = ((ctx.r[4].s32 as i64 * 356 as i64) as i32);
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 8240B380: 7C8B5214  add r4, r11, r10
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8240B384: 38A00164  li r5, 0x164
	ctx.r[5].s64 = 356;
	// 8240B388: 4BFB5879  bl 0x823c0c00
	ctx.lr = 0x8240B38C;
	sub_823C0C00(ctx, base);
	// 8240B38C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8240B390: 48000018  b 0x8240b3a8
	pc = 0x8240B3A8; continue 'dispatch;
            }
            0x8240B394 => {
    //   block [0x8240B394..0x8240B3A8)
	// 8240B394: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240B398: 386BD608  addi r3, r11, -0x29f8
	ctx.r[3].s64 = ctx.r[11].s64 + -10744;
	// 8240B39C: 4BEA7BE5  bl 0x822b2f80
	ctx.lr = 0x8240B3A0;
	sub_822B2F80(ctx, base);
	// 8240B3A0: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 8240B3A4: 60630016  ori r3, r3, 0x16
	ctx.r[3].u64 = ctx.r[3].u64 | 22;
	pc = 0x8240B3A8; continue 'dispatch;
            }
            0x8240B3A8 => {
    //   block [0x8240B3A8..0x8240B3B8)
	// 8240B3A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8240B3AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240B3B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240B3B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240B3B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8240B3B8 size=80
    let mut pc: u32 = 0x8240B3B8;
    'dispatch: loop {
        match pc {
            0x8240B3B8 => {
    //   block [0x8240B3B8..0x8240B3E4)
	// 8240B3B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240B3BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240B3C0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240B3C4: 2B0400BF  cmplwi cr6, r4, 0xbf
	ctx.cr[6].compare_u32(ctx.r[4].u32, 191 as u32, &mut ctx.xer);
	// 8240B3C8: 4199001C  bgt cr6, 0x8240b3e4
	if ctx.cr[6].gt {
	pc = 0x8240B3E4; continue 'dispatch;
	}
	// 8240B3CC: 1D640164  mulli r11, r4, 0x164
	ctx.r[11].s32 = ((ctx.r[4].s32 as i64 * 356 as i64) as i32);
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 8240B3D0: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 8240B3D4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8240B3D8: C00B014C  lfs f0, 0x14c(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(332 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240B3DC: D0050000  stfs f0, 0(r5)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 8240B3E0: 48000018  b 0x8240b3f8
	pc = 0x8240B3F8; continue 'dispatch;
            }
            0x8240B3E4 => {
    //   block [0x8240B3E4..0x8240B3F8)
	// 8240B3E4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240B3E8: 386BD648  addi r3, r11, -0x29b8
	ctx.r[3].s64 = ctx.r[11].s64 + -10680;
	// 8240B3EC: 4BEA7B95  bl 0x822b2f80
	ctx.lr = 0x8240B3F0;
	sub_822B2F80(ctx, base);
	// 8240B3F0: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 8240B3F4: 60630016  ori r3, r3, 0x16
	ctx.r[3].u64 = ctx.r[3].u64 | 22;
	pc = 0x8240B3F8; continue 'dispatch;
            }
            0x8240B3F8 => {
    //   block [0x8240B3F8..0x8240B408)
	// 8240B3F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8240B3FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240B400: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240B404: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240B408(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8240B408 size=80
    let mut pc: u32 = 0x8240B408;
    'dispatch: loop {
        match pc {
            0x8240B408 => {
    //   block [0x8240B408..0x8240B434)
	// 8240B408: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240B40C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240B410: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240B414: 2B0400BF  cmplwi cr6, r4, 0xbf
	ctx.cr[6].compare_u32(ctx.r[4].u32, 191 as u32, &mut ctx.xer);
	// 8240B418: 4199001C  bgt cr6, 0x8240b434
	if ctx.cr[6].gt {
	pc = 0x8240B434; continue 'dispatch;
	}
	// 8240B41C: 1D640164  mulli r11, r4, 0x164
	ctx.r[11].s32 = ((ctx.r[4].s32 as i64 * 356 as i64) as i32);
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 8240B420: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 8240B424: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8240B428: 816B0154  lwz r11, 0x154(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(340 as u32) ) } as u64;
	// 8240B42C: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8240B430: 48000018  b 0x8240b448
	pc = 0x8240B448; continue 'dispatch;
            }
            0x8240B434 => {
    //   block [0x8240B434..0x8240B448)
	// 8240B434: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240B438: 386BD688  addi r3, r11, -0x2978
	ctx.r[3].s64 = ctx.r[11].s64 + -10616;
	// 8240B43C: 4BEA7B45  bl 0x822b2f80
	ctx.lr = 0x8240B440;
	sub_822B2F80(ctx, base);
	// 8240B440: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 8240B444: 60630016  ori r3, r3, 0x16
	ctx.r[3].u64 = ctx.r[3].u64 | 22;
	pc = 0x8240B448; continue 'dispatch;
            }
            0x8240B448 => {
    //   block [0x8240B448..0x8240B458)
	// 8240B448: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8240B44C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240B450: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240B454: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240B458(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8240B458 size=80
    let mut pc: u32 = 0x8240B458;
    'dispatch: loop {
        match pc {
            0x8240B458 => {
    //   block [0x8240B458..0x8240B484)
	// 8240B458: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240B45C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240B460: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240B464: 2B0400BF  cmplwi cr6, r4, 0xbf
	ctx.cr[6].compare_u32(ctx.r[4].u32, 191 as u32, &mut ctx.xer);
	// 8240B468: 4199001C  bgt cr6, 0x8240b484
	if ctx.cr[6].gt {
	pc = 0x8240B484; continue 'dispatch;
	}
	// 8240B46C: 1D640164  mulli r11, r4, 0x164
	ctx.r[11].s32 = ((ctx.r[4].s32 as i64 * 356 as i64) as i32);
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 8240B470: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 8240B474: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8240B478: C00B0160  lfs f0, 0x160(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(352 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240B47C: D0050000  stfs f0, 0(r5)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 8240B480: 48000018  b 0x8240b498
	pc = 0x8240B498; continue 'dispatch;
            }
            0x8240B484 => {
    //   block [0x8240B484..0x8240B498)
	// 8240B484: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240B488: 386BD6D0  addi r3, r11, -0x2930
	ctx.r[3].s64 = ctx.r[11].s64 + -10544;
	// 8240B48C: 4BEA7AF5  bl 0x822b2f80
	ctx.lr = 0x8240B490;
	sub_822B2F80(ctx, base);
	// 8240B490: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 8240B494: 60630016  ori r3, r3, 0x16
	ctx.r[3].u64 = ctx.r[3].u64 | 22;
	pc = 0x8240B498; continue 'dispatch;
            }
            0x8240B498 => {
    //   block [0x8240B498..0x8240B4A8)
	// 8240B498: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8240B49C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240B4A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240B4A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240B4A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8240B4A8 size=80
    let mut pc: u32 = 0x8240B4A8;
    'dispatch: loop {
        match pc {
            0x8240B4A8 => {
    //   block [0x8240B4A8..0x8240B4D4)
	// 8240B4A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240B4AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240B4B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240B4B4: 2B0400BF  cmplwi cr6, r4, 0xbf
	ctx.cr[6].compare_u32(ctx.r[4].u32, 191 as u32, &mut ctx.xer);
	// 8240B4B8: 4199001C  bgt cr6, 0x8240b4d4
	if ctx.cr[6].gt {
	pc = 0x8240B4D4; continue 'dispatch;
	}
	// 8240B4BC: 1D640164  mulli r11, r4, 0x164
	ctx.r[11].s32 = ((ctx.r[4].s32 as i64 * 356 as i64) as i32);
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 8240B4C0: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 8240B4C4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8240B4C8: C00B015C  lfs f0, 0x15c(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(348 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240B4CC: D0050000  stfs f0, 0(r5)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 8240B4D0: 48000018  b 0x8240b4e8
	pc = 0x8240B4E8; continue 'dispatch;
            }
            0x8240B4D4 => {
    //   block [0x8240B4D4..0x8240B4E8)
	// 8240B4D4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240B4D8: 386BD720  addi r3, r11, -0x28e0
	ctx.r[3].s64 = ctx.r[11].s64 + -10464;
	// 8240B4DC: 4BEA7AA5  bl 0x822b2f80
	ctx.lr = 0x8240B4E0;
	sub_822B2F80(ctx, base);
	// 8240B4E0: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 8240B4E4: 60630016  ori r3, r3, 0x16
	ctx.r[3].u64 = ctx.r[3].u64 | 22;
	pc = 0x8240B4E8; continue 'dispatch;
            }
            0x8240B4E8 => {
    //   block [0x8240B4E8..0x8240B4F8)
	// 8240B4E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8240B4EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240B4F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240B4F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240B4F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8240B4F8 size=124
    let mut pc: u32 = 0x8240B4F8;
    'dispatch: loop {
        match pc {
            0x8240B4F8 => {
    //   block [0x8240B4F8..0x8240B53C)
	// 8240B4F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240B4FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240B500: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240B504: 2B0400BF  cmplwi cr6, r4, 0xbf
	ctx.cr[6].compare_u32(ctx.r[4].u32, 191 as u32, &mut ctx.xer);
	// 8240B508: 41990048  bgt cr6, 0x8240b550
	if ctx.cr[6].gt {
	pc = 0x8240B550; continue 'dispatch;
	}
	// 8240B50C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8240B510: C00B1FF8  lfs f0, 0x1ff8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240B514: FF010000  fcmpu cr6, f1, f0
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 8240B518: 40980024  bge cr6, 0x8240b53c
	if !ctx.cr[6].lt {
	pc = 0x8240B53C; continue 'dispatch;
	}
	// 8240B51C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240B520: D8210018  stfd f1, 0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(24 as u32), ctx.f[1].u64 ) };
	// 8240B524: E8810018  ld r4, 0x18(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(24 as u32) ) };
	// 8240B528: 386BD7B0  addi r3, r11, -0x2850
	ctx.r[3].s64 = ctx.r[11].s64 + -10320;
	// 8240B52C: 4BEA7A55  bl 0x822b2f80
	ctx.lr = 0x8240B530;
	sub_822B2F80(ctx, base);
	// 8240B530: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 8240B534: 60630018  ori r3, r3, 0x18
	ctx.r[3].u64 = ctx.r[3].u64 | 24;
	// 8240B538: 4800002C  b 0x8240b564
	pc = 0x8240B564; continue 'dispatch;
            }
            0x8240B53C => {
    //   block [0x8240B53C..0x8240B550)
	// 8240B53C: 1D640164  mulli r11, r4, 0x164
	ctx.r[11].s32 = ((ctx.r[4].s32 as i64 * 356 as i64) as i32);
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 8240B540: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 8240B544: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8240B548: D02B014C  stfs f1, 0x14c(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(332 as u32), tmp.u32 ) };
	// 8240B54C: 48000018  b 0x8240b564
	pc = 0x8240B564; continue 'dispatch;
            }
            0x8240B550 => {
    //   block [0x8240B550..0x8240B564)
	// 8240B550: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240B554: 386BD76C  addi r3, r11, -0x2894
	ctx.r[3].s64 = ctx.r[11].s64 + -10388;
	// 8240B558: 4BEA7A29  bl 0x822b2f80
	ctx.lr = 0x8240B55C;
	sub_822B2F80(ctx, base);
	// 8240B55C: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 8240B560: 60630016  ori r3, r3, 0x16
	ctx.r[3].u64 = ctx.r[3].u64 | 22;
	pc = 0x8240B564; continue 'dispatch;
            }
            0x8240B564 => {
    //   block [0x8240B564..0x8240B574)
	// 8240B564: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8240B568: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240B56C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240B570: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240B578(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8240B578 size=76
    let mut pc: u32 = 0x8240B578;
    'dispatch: loop {
        match pc {
            0x8240B578 => {
    //   block [0x8240B578..0x8240B5A0)
	// 8240B578: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240B57C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240B580: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240B584: 2B0400BF  cmplwi cr6, r4, 0xbf
	ctx.cr[6].compare_u32(ctx.r[4].u32, 191 as u32, &mut ctx.xer);
	// 8240B588: 41990018  bgt cr6, 0x8240b5a0
	if ctx.cr[6].gt {
	pc = 0x8240B5A0; continue 'dispatch;
	}
	// 8240B58C: 1D640164  mulli r11, r4, 0x164
	ctx.r[11].s32 = ((ctx.r[4].s32 as i64 * 356 as i64) as i32);
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 8240B590: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 8240B594: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8240B598: 90AB0154  stw r5, 0x154(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(340 as u32), ctx.r[5].u32 ) };
	// 8240B59C: 48000018  b 0x8240b5b4
	pc = 0x8240B5B4; continue 'dispatch;
            }
            0x8240B5A0 => {
    //   block [0x8240B5A0..0x8240B5B4)
	// 8240B5A0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240B5A4: 386BD7F0  addi r3, r11, -0x2810
	ctx.r[3].s64 = ctx.r[11].s64 + -10256;
	// 8240B5A8: 4BEA79D9  bl 0x822b2f80
	ctx.lr = 0x8240B5AC;
	sub_822B2F80(ctx, base);
	// 8240B5AC: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 8240B5B0: 60630016  ori r3, r3, 0x16
	ctx.r[3].u64 = ctx.r[3].u64 | 22;
	pc = 0x8240B5B4; continue 'dispatch;
            }
            0x8240B5B4 => {
    //   block [0x8240B5B4..0x8240B5C4)
	// 8240B5B4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8240B5B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240B5BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240B5C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240B5C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8240B5C8 size=76
    let mut pc: u32 = 0x8240B5C8;
    'dispatch: loop {
        match pc {
            0x8240B5C8 => {
    //   block [0x8240B5C8..0x8240B5F0)
	// 8240B5C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240B5CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240B5D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240B5D4: 2B0400BF  cmplwi cr6, r4, 0xbf
	ctx.cr[6].compare_u32(ctx.r[4].u32, 191 as u32, &mut ctx.xer);
	// 8240B5D8: 41990018  bgt cr6, 0x8240b5f0
	if ctx.cr[6].gt {
	pc = 0x8240B5F0; continue 'dispatch;
	}
	// 8240B5DC: 1D640164  mulli r11, r4, 0x164
	ctx.r[11].s32 = ((ctx.r[4].s32 as i64 * 356 as i64) as i32);
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 8240B5E0: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 8240B5E4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8240B5E8: D02B0150  stfs f1, 0x150(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(336 as u32), tmp.u32 ) };
	// 8240B5EC: 48000018  b 0x8240b604
	pc = 0x8240B604; continue 'dispatch;
            }
            0x8240B5F0 => {
    //   block [0x8240B5F0..0x8240B604)
	// 8240B5F0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240B5F4: 386BD834  addi r3, r11, -0x27cc
	ctx.r[3].s64 = ctx.r[11].s64 + -10188;
	// 8240B5F8: 4BEA7989  bl 0x822b2f80
	ctx.lr = 0x8240B5FC;
	sub_822B2F80(ctx, base);
	// 8240B5FC: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 8240B600: 60630016  ori r3, r3, 0x16
	ctx.r[3].u64 = ctx.r[3].u64 | 22;
	pc = 0x8240B604; continue 'dispatch;
            }
            0x8240B604 => {
    //   block [0x8240B604..0x8240B614)
	// 8240B604: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8240B608: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240B60C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240B610: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240B618(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8240B618 size=80
    let mut pc: u32 = 0x8240B618;
    'dispatch: loop {
        match pc {
            0x8240B618 => {
    //   block [0x8240B618..0x8240B648)
	// 8240B618: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240B61C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240B620: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240B624: 2B0400BF  cmplwi cr6, r4, 0xbf
	ctx.cr[6].compare_u32(ctx.r[4].u32, 191 as u32, &mut ctx.xer);
	// 8240B628: 41990020  bgt cr6, 0x8240b648
	if ctx.cr[6].gt {
	pc = 0x8240B648; continue 'dispatch;
	}
	// 8240B62C: 1D640164  mulli r11, r4, 0x164
	ctx.r[11].s32 = ((ctx.r[4].s32 as i64 * 356 as i64) as i32);
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 8240B630: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 8240B634: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8240B638: 396BFFFB  addi r11, r11, -5
	ctx.r[11].s64 = ctx.r[11].s64 + -5;
	// 8240B63C: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 8240B640: 5563DFFE  rlwinm r3, r11, 0x1b, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8240B644: 48000014  b 0x8240b658
	pc = 0x8240B658; continue 'dispatch;
            }
            0x8240B648 => {
    //   block [0x8240B648..0x8240B658)
	// 8240B648: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240B64C: 386BD874  addi r3, r11, -0x278c
	ctx.r[3].s64 = ctx.r[11].s64 + -10124;
	// 8240B650: 4BEA7931  bl 0x822b2f80
	ctx.lr = 0x8240B654;
	sub_822B2F80(ctx, base);
	// 8240B654: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	pc = 0x8240B658; continue 'dispatch;
            }
            0x8240B658 => {
    //   block [0x8240B658..0x8240B668)
	// 8240B658: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8240B65C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240B660: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240B664: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240B668(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8240B668 size=168
    let mut pc: u32 = 0x8240B668;
    'dispatch: loop {
        match pc {
            0x8240B668 => {
    //   block [0x8240B668..0x8240B6AC)
	// 8240B668: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240B66C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240B670: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240B674: 7C671B78  mr r7, r3
	ctx.r[7].u64 = ctx.r[3].u64;
	// 8240B678: 7CAB2B78  mr r11, r5
	ctx.r[11].u64 = ctx.r[5].u64;
	// 8240B67C: 2B060040  cmplwi cr6, r6, 0x40
	ctx.cr[6].compare_u32(ctx.r[6].u32, 64 as u32, &mut ctx.xer);
	// 8240B680: 41990068  bgt cr6, 0x8240b6e8
	if ctx.cr[6].gt {
	pc = 0x8240B6E8; continue 'dispatch;
	}
	// 8240B684: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 8240B688: 419A0058  beq cr6, 0x8240b6e0
	if ctx.cr[6].eq {
	pc = 0x8240B6E0; continue 'dispatch;
	}
	// 8240B68C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8240B690: 419A0050  beq cr6, 0x8240b6e0
	if ctx.cr[6].eq {
	pc = 0x8240B6E0; continue 'dispatch;
	}
	// 8240B694: 2F060000  cmpwi cr6, r6, 0
	ctx.cr[6].compare_i32(ctx.r[6].s32, 0, &mut ctx.xer);
	// 8240B698: 40990034  ble cr6, 0x8240b6cc
	if !ctx.cr[6].gt {
	pc = 0x8240B6CC; continue 'dispatch;
	}
	// 8240B69C: 3D470001  addis r10, r7, 1
	ctx.r[10].s64 = ctx.r[7].s64 + 65536;
	// 8240B6A0: 7D0B2050  subf r8, r11, r4
	ctx.r[8].s64 = ctx.r[4].s64 - ctx.r[11].s64;
	// 8240B6A4: 394A0C04  addi r10, r10, 0xc04
	ctx.r[10].s64 = ctx.r[10].s64 + 3076;
	// 8240B6A8: 7CC93378  mr r9, r6
	ctx.r[9].u64 = ctx.r[6].u64;
	pc = 0x8240B6AC; continue 'dispatch;
            }
            0x8240B6AC => {
    //   block [0x8240B6AC..0x8240B6CC)
	// 8240B6AC: 7CA8582E  lwzx r5, r8, r11
	ctx.r[5].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8240B6B0: 3529FFFF  addic. r9, r9, -1
	ctx.xer.ca = (ctx.r[9].u32 > (!(-1 as u32)));
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8240B6B4: 90AAFF00  stw r5, -0x100(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-256 as u32), ctx.r[5].u32 ) };
	// 8240B6B8: 80AB0000  lwz r5, 0(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8240B6BC: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8240B6C0: 90AA0000  stw r5, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[5].u32 ) };
	// 8240B6C4: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 8240B6C8: 4082FFE4  bne 0x8240b6ac
	if !ctx.cr[0].eq {
	pc = 0x8240B6AC; continue 'dispatch;
	}
	pc = 0x8240B6CC; continue 'dispatch;
            }
            0x8240B6CC => {
    //   block [0x8240B6CC..0x8240B6E0)
	// 8240B6CC: 3D600001  lis r11, 1
	ctx.r[11].s64 = 65536;
	// 8240B6D0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8240B6D4: 616B0D04  ori r11, r11, 0xd04
	ctx.r[11].u64 = ctx.r[11].u64 | 3332;
	// 8240B6D8: 7CC7592E  stwx r6, r7, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[7].u32.wrapping_add(ctx.r[11].u32), ctx.r[6].u32) };
	// 8240B6DC: 48000024  b 0x8240b700
	pc = 0x8240B700; continue 'dispatch;
            }
            0x8240B6E0 => {
    //   block [0x8240B6E0..0x8240B6E8)
	// 8240B6E0: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 8240B6E4: 4800001C  b 0x8240b700
	pc = 0x8240B700; continue 'dispatch;
            }
            0x8240B6E8 => {
    //   block [0x8240B6E8..0x8240B700)
	// 8240B6E8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240B6EC: 7CC43378  mr r4, r6
	ctx.r[4].u64 = ctx.r[6].u64;
	// 8240B6F0: 386BD8B8  addi r3, r11, -0x2748
	ctx.r[3].s64 = ctx.r[11].s64 + -10056;
	// 8240B6F4: 4BEA788D  bl 0x822b2f80
	ctx.lr = 0x8240B6F8;
	sub_822B2F80(ctx, base);
	// 8240B6F8: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 8240B6FC: 60630014  ori r3, r3, 0x14
	ctx.r[3].u64 = ctx.r[3].u64 | 20;
	pc = 0x8240B700; continue 'dispatch;
            }
            0x8240B700 => {
    //   block [0x8240B700..0x8240B710)
	// 8240B700: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8240B704: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240B708: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240B70C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240B710(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8240B710 size=120
    let mut pc: u32 = 0x8240B710;
    'dispatch: loop {
        match pc {
            0x8240B710 => {
    //   block [0x8240B710..0x8240B764)
	// 8240B710: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240B714: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240B718: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240B71C: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 8240B720: 41980044  blt cr6, 0x8240b764
	if ctx.cr[6].lt {
	pc = 0x8240B764; continue 'dispatch;
	}
	// 8240B724: 3D600001  lis r11, 1
	ctx.r[11].s64 = 65536;
	// 8240B728: 616B0D04  ori r11, r11, 0xd04
	ctx.r[11].u64 = ctx.r[11].u64 | 3332;
	// 8240B72C: 7D63582E  lwzx r11, r3, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[3].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8240B730: 7F045800  cmpw cr6, r4, r11
	ctx.cr[6].compare_i32(ctx.r[4].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8240B734: 40980030  bge cr6, 0x8240b764
	if !ctx.cr[6].lt {
	pc = 0x8240B764; continue 'dispatch;
	}
	// 8240B738: 396442C1  addi r11, r4, 0x42c1
	ctx.r[11].s64 = ctx.r[4].s64 + 17089;
	// 8240B73C: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8240B740: 7C6B182E  lwzx r3, r11, r3
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[3].u32)) } as u64;
	// 8240B744: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8240B748: 41810030  bgt 0x8240b778
	if ctx.cr[0].gt {
	pc = 0x8240B778; continue 'dispatch;
	}
	// 8240B74C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240B750: 386BD948  addi r3, r11, -0x26b8
	ctx.r[3].s64 = ctx.r[11].s64 + -9912;
	// 8240B754: 4BEA782D  bl 0x822b2f80
	ctx.lr = 0x8240B758;
	sub_822B2F80(ctx, base);
	// 8240B758: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 8240B75C: 60630011  ori r3, r3, 0x11
	ctx.r[3].u64 = ctx.r[3].u64 | 17;
	// 8240B760: 48000018  b 0x8240b778
	pc = 0x8240B778; continue 'dispatch;
            }
            0x8240B764 => {
    //   block [0x8240B764..0x8240B778)
	// 8240B764: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240B768: 386BD900  addi r3, r11, -0x2700
	ctx.r[3].s64 = ctx.r[11].s64 + -9984;
	// 8240B76C: 4BEA7815  bl 0x822b2f80
	ctx.lr = 0x8240B770;
	sub_822B2F80(ctx, base);
	// 8240B770: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 8240B774: 60630014  ori r3, r3, 0x14
	ctx.r[3].u64 = ctx.r[3].u64 | 20;
	pc = 0x8240B778; continue 'dispatch;
            }
            0x8240B778 => {
    //   block [0x8240B778..0x8240B788)
	// 8240B778: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8240B77C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240B780: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240B784: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240B788(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8240B788 size=92
    let mut pc: u32 = 0x8240B788;
    'dispatch: loop {
        match pc {
            0x8240B788 => {
    //   block [0x8240B788..0x8240B7C0)
	// 8240B788: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240B78C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240B790: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240B794: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 8240B798: 41980028  blt cr6, 0x8240b7c0
	if ctx.cr[6].lt {
	pc = 0x8240B7C0; continue 'dispatch;
	}
	// 8240B79C: 3D600001  lis r11, 1
	ctx.r[11].s64 = 65536;
	// 8240B7A0: 616B0D04  ori r11, r11, 0xd04
	ctx.r[11].u64 = ctx.r[11].u64 | 3332;
	// 8240B7A4: 7D63582E  lwzx r11, r3, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[3].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8240B7A8: 7F045800  cmpw cr6, r4, r11
	ctx.cr[6].compare_i32(ctx.r[4].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8240B7AC: 40980014  bge cr6, 0x8240b7c0
	if !ctx.cr[6].lt {
	pc = 0x8240B7C0; continue 'dispatch;
	}
	// 8240B7B0: 39644301  addi r11, r4, 0x4301
	ctx.r[11].s64 = ctx.r[4].s64 + 17153;
	// 8240B7B4: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8240B7B8: 7C6B182E  lwzx r3, r11, r3
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[3].u32)) } as u64;
	// 8240B7BC: 48000018  b 0x8240b7d4
	pc = 0x8240B7D4; continue 'dispatch;
            }
            0x8240B7C0 => {
    //   block [0x8240B7C0..0x8240B7D4)
	// 8240B7C0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240B7C4: 386BD9B0  addi r3, r11, -0x2650
	ctx.r[3].s64 = ctx.r[11].s64 + -9808;
	// 8240B7C8: 4BEA77B9  bl 0x822b2f80
	ctx.lr = 0x8240B7CC;
	sub_822B2F80(ctx, base);
	// 8240B7CC: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 8240B7D0: 60630014  ori r3, r3, 0x14
	ctx.r[3].u64 = ctx.r[3].u64 | 20;
	pc = 0x8240B7D4; continue 'dispatch;
            }
            0x8240B7D4 => {
    //   block [0x8240B7D4..0x8240B7E4)
	// 8240B7D4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8240B7D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240B7DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240B7E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240B7E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8240B7E8 size=128
    let mut pc: u32 = 0x8240B7E8;
    'dispatch: loop {
        match pc {
            0x8240B7E8 => {
    //   block [0x8240B7E8..0x8240B83C)
	// 8240B7E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240B7EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240B7F0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240B7F4: 2B0400BF  cmplwi cr6, r4, 0xbf
	ctx.cr[6].compare_u32(ctx.r[4].u32, 191 as u32, &mut ctx.xer);
	// 8240B7F8: 4199004C  bgt cr6, 0x8240b844
	if ctx.cr[6].gt {
	pc = 0x8240B844; continue 'dispatch;
	}
	// 8240B7FC: 1D640164  mulli r11, r4, 0x164
	ctx.r[11].s32 = ((ctx.r[4].s32 as i64 * 356 as i64) as i32);
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 8240B800: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 8240B804: 814B0014  lwz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 8240B808: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8240B80C: 41820030  beq 0x8240b83c
	if ctx.cr[0].eq {
	pc = 0x8240B83C; continue 'dispatch;
	}
	// 8240B810: 812B0008  lwz r9, 8(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8240B814: 2F090006  cmpwi cr6, r9, 6
	ctx.cr[6].compare_i32(ctx.r[9].s32, 6, &mut ctx.xer);
	// 8240B818: 419A0024  beq cr6, 0x8240b83c
	if ctx.cr[6].eq {
	pc = 0x8240B83C; continue 'dispatch;
	}
	// 8240B81C: 814A004C  lwz r10, 0x4c(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(76 as u32) ) } as u64;
	// 8240B820: 554A07FF  clrlwi. r10, r10, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8240B824: 41820018  beq 0x8240b83c
	if ctx.cr[0].eq {
	pc = 0x8240B83C; continue 'dispatch;
	}
	// 8240B828: 39400005  li r10, 5
	ctx.r[10].s64 = 5;
	// 8240B82C: 912B000C  stw r9, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 8240B830: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8240B834: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8240B838: 48000020  b 0x8240b858
	pc = 0x8240B858; continue 'dispatch;
            }
            0x8240B83C => {
    //   block [0x8240B83C..0x8240B844)
	// 8240B83C: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 8240B840: 48000018  b 0x8240b858
	pc = 0x8240B858; continue 'dispatch;
            }
            0x8240B844 => {
    //   block [0x8240B844..0x8240B858)
	// 8240B844: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240B848: 386BD9FC  addi r3, r11, -0x2604
	ctx.r[3].s64 = ctx.r[11].s64 + -9732;
	// 8240B84C: 4BEA7735  bl 0x822b2f80
	ctx.lr = 0x8240B850;
	sub_822B2F80(ctx, base);
	// 8240B850: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 8240B854: 60630016  ori r3, r3, 0x16
	ctx.r[3].u64 = ctx.r[3].u64 | 22;
	pc = 0x8240B858; continue 'dispatch;
            }
            0x8240B858 => {
    //   block [0x8240B858..0x8240B868)
	// 8240B858: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8240B85C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240B860: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240B864: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240B868(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8240B868 size=60
    let mut pc: u32 = 0x8240B868;
    'dispatch: loop {
        match pc {
            0x8240B868 => {
    //   block [0x8240B868..0x8240B870)
	// 8240B868: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8240B86C: 39630008  addi r11, r3, 8
	ctx.r[11].s64 = ctx.r[3].s64 + 8;
	pc = 0x8240B870; continue 'dispatch;
            }
            0x8240B870 => {
    //   block [0x8240B870..0x8240B88C)
	// 8240B870: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8240B874: 2C0A0000  cmpwi r10, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8240B878: 41820014  beq 0x8240b88c
	if ctx.cr[0].eq {
	pc = 0x8240B88C; continue 'dispatch;
	}
	// 8240B87C: 2F0A0003  cmpwi cr6, r10, 3
	ctx.cr[6].compare_i32(ctx.r[10].s32, 3, &mut ctx.xer);
	// 8240B880: 419A000C  beq cr6, 0x8240b88c
	if ctx.cr[6].eq {
	pc = 0x8240B88C; continue 'dispatch;
	}
	// 8240B884: 2F0A0006  cmpwi cr6, r10, 6
	ctx.cr[6].compare_i32(ctx.r[10].s32, 6, &mut ctx.xer);
	// 8240B888: 409A001C  bne cr6, 0x8240b8a4
	if !ctx.cr[6].eq {
		crate::recompiler::externs::call(ctx, base, 0x8240B8A4);
		return;
	}
	pc = 0x8240B88C; continue 'dispatch;
            }
            0x8240B88C => {
    //   block [0x8240B88C..0x8240B8A4)
	// 8240B88C: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 8240B890: 396B0164  addi r11, r11, 0x164
	ctx.r[11].s64 = ctx.r[11].s64 + 356;
	// 8240B894: 2F0900C0  cmpwi cr6, r9, 0xc0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 192, &mut ctx.xer);
	// 8240B898: 4198FFD8  blt cr6, 0x8240b870
	if ctx.cr[6].lt {
	pc = 0x8240B870; continue 'dispatch;
	}
	// 8240B89C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8240B8A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240B8B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8240B8B0 size=704
    let mut pc: u32 = 0x8240B8B0;
    'dispatch: loop {
        match pc {
            0x8240B8B0 => {
    //   block [0x8240B8B0..0x8240B8F0)
	// 8240B8B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240B8B4: 481297F1  bl 0x825350a4
	ctx.lr = 0x8240B8B8;
	sub_82535080(ctx, base);
	// 8240B8B8: 9421F350  stwu r1, -0xcb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-3248 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240B8BC: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 8240B8C0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8240B8C4: 3B000000  li r24, 0
	ctx.r[24].s64 = 0;
	// 8240B8C8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8240B8CC: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 8240B8D0: 7CB92B78  mr r25, r5
	ctx.r[25].u64 = ctx.r[5].u64;
	// 8240B8D4: 7CD73378  mr r23, r6
	ctx.r[23].u64 = ctx.r[6].u64;
	// 8240B8D8: 7F1DC378  mr r29, r24
	ctx.r[29].u64 = ctx.r[24].u64;
	// 8240B8DC: 4BFFFE35  bl 0x8240b710
	ctx.lr = 0x8240B8E0;
	sub_8240B710(ctx, base);
	// 8240B8E0: 7C7C1B79  or. r28, r3, r3
	ctx.r[28].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 8240B8E4: 4080000C  bge 0x8240b8f0
	if !ctx.cr[0].lt {
	pc = 0x8240B8F0; continue 'dispatch;
	}
	// 8240B8E8: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8240B8EC: 4800027C  b 0x8240bb68
	pc = 0x8240BB68; continue 'dispatch;
            }
            0x8240B8F0 => {
    //   block [0x8240B8F0..0x8240B908)
	// 8240B8F0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8240B8F4: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 8240B8F8: 4BFFFE91  bl 0x8240b788
	ctx.lr = 0x8240B8FC;
	sub_8240B788(ctx, base);
	// 8240B8FC: 393B0030  addi r9, r27, 0x30
	ctx.r[9].s64 = ctx.r[27].s64 + 48;
	// 8240B900: 7F0BC378  mr r11, r24
	ctx.r[11].u64 = ctx.r[24].u64;
	// 8240B904: 3B400001  li r26, 1
	ctx.r[26].s64 = 1;
	pc = 0x8240B908; continue 'dispatch;
            }
            0x8240B908 => {
    //   block [0x8240B908..0x8240B930)
	// 8240B908: 39010660  addi r8, r1, 0x660
	ctx.r[8].s64 = ctx.r[1].s64 + 1632;
	// 8240B90C: 8149FFD8  lwz r10, -0x28(r9)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-40 as u32) ) } as u64;
	// 8240B910: 2C0A0000  cmpwi r10, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8240B914: 7F0B412E  stwx r24, r11, r8
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[8].u32), ctx.r[24].u32) };
	// 8240B918: 40810038  ble 0x8240b950
	if !ctx.cr[0].gt {
	pc = 0x8240B950; continue 'dispatch;
	}
	// 8240B91C: 2F0A0002  cmpwi cr6, r10, 2
	ctx.cr[6].compare_i32(ctx.r[10].s32, 2, &mut ctx.xer);
	// 8240B920: 40990010  ble cr6, 0x8240b930
	if !ctx.cr[6].gt {
	pc = 0x8240B930; continue 'dispatch;
	}
	// 8240B924: 394AFFFC  addi r10, r10, -4
	ctx.r[10].s64 = ctx.r[10].s64 + -4;
	// 8240B928: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 8240B92C: 41990024  bgt cr6, 0x8240b950
	if ctx.cr[6].gt {
	pc = 0x8240B950; continue 'dispatch;
	}
	pc = 0x8240B930; continue 'dispatch;
            }
            0x8240B930 => {
    //   block [0x8240B930..0x8240B950)
	// 8240B930: 39410060  addi r10, r1, 0x60
	ctx.r[10].s64 = ctx.r[1].s64 + 96;
	// 8240B934: 80E90000  lwz r7, 0(r9)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 8240B938: 7F1E3800  cmpw cr6, r30, r7
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[7].s32, &mut ctx.xer);
	// 8240B93C: 7F4B512E  stwx r26, r11, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[26].u32) };
	// 8240B940: 409A0018  bne cr6, 0x8240b958
	if !ctx.cr[6].eq {
	pc = 0x8240B958; continue 'dispatch;
	}
	// 8240B944: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 8240B948: 7F4B412E  stwx r26, r11, r8
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[8].u32), ctx.r[26].u32) };
	// 8240B94C: 4800000C  b 0x8240b958
	pc = 0x8240B958; continue 'dispatch;
            }
            0x8240B950 => {
    //   block [0x8240B950..0x8240B958)
	// 8240B950: 39410060  addi r10, r1, 0x60
	ctx.r[10].s64 = ctx.r[1].s64 + 96;
	// 8240B954: 7F0B512E  stwx r24, r11, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[24].u32) };
	pc = 0x8240B958; continue 'dispatch;
            }
            0x8240B958 => {
    //   block [0x8240B958..0x8240B984)
	// 8240B958: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8240B95C: 39290164  addi r9, r9, 0x164
	ctx.r[9].s64 = ctx.r[9].s64 + 356;
	// 8240B960: 2F0B0300  cmpwi cr6, r11, 0x300
	ctx.cr[6].compare_i32(ctx.r[11].s32, 768, &mut ctx.xer);
	// 8240B964: 4198FFA4  blt cr6, 0x8240b908
	if ctx.cr[6].lt {
	pc = 0x8240B908; continue 'dispatch;
	}
	// 8240B968: 7F1DE000  cmpw cr6, r29, r28
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[28].s32, &mut ctx.xer);
	// 8240B96C: 409A0020  bne cr6, 0x8240b98c
	if !ctx.cr[6].eq {
	pc = 0x8240B98C; continue 'dispatch;
	}
	// 8240B970: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8240B974: 419A0010  beq cr6, 0x8240b984
	if ctx.cr[6].eq {
	pc = 0x8240B984; continue 'dispatch;
	}
	// 8240B978: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 8240B97C: 60630043  ori r3, r3, 0x43
	ctx.r[3].u64 = ctx.r[3].u64 | 67;
	// 8240B980: 480001E8  b 0x8240bb68
	pc = 0x8240BB68; continue 'dispatch;
            }
            0x8240B984 => {
    //   block [0x8240B984..0x8240B98C)
	// 8240B984: 38810660  addi r4, r1, 0x660
	ctx.r[4].s64 = ctx.r[1].s64 + 1632;
	// 8240B988: 48000048  b 0x8240b9d0
	pc = 0x8240B9D0; continue 'dispatch;
            }
            0x8240B98C => {
    //   block [0x8240B98C..0x8240B9A8)
	// 8240B98C: 409801C0  bge cr6, 0x8240bb4c
	if !ctx.cr[6].lt {
	pc = 0x8240BB4C; continue 'dispatch;
	}
	// 8240B990: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8240B994: 7F1FC800  cmpw cr6, r31, r25
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[25].s32, &mut ctx.xer);
	// 8240B998: 41990034  bgt cr6, 0x8240b9cc
	if ctx.cr[6].gt {
	pc = 0x8240B9CC; continue 'dispatch;
	}
	// 8240B99C: 1D7F0164  mulli r11, r31, 0x164
	ctx.r[11].s32 = ((ctx.r[31].s32 as i64 * 356 as i64) as i32);
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 8240B9A0: 7D6BDA14  add r11, r11, r27
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[27].u64;
	// 8240B9A4: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	pc = 0x8240B9A8; continue 'dispatch;
            }
            0x8240B9A8 => {
    //   block [0x8240B9A8..0x8240B9CC)
	// 8240B9A8: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8240B9AC: 2C0A0000  cmpwi r10, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8240B9B0: 418201B8  beq 0x8240bb68
	if ctx.cr[0].eq {
	pc = 0x8240BB68; continue 'dispatch;
	}
	// 8240B9B4: 2F0A0003  cmpwi cr6, r10, 3
	ctx.cr[6].compare_i32(ctx.r[10].s32, 3, &mut ctx.xer);
	// 8240B9B8: 419A01B0  beq cr6, 0x8240bb68
	if ctx.cr[6].eq {
	pc = 0x8240BB68; continue 'dispatch;
	}
	// 8240B9BC: 38630001  addi r3, r3, 1
	ctx.r[3].s64 = ctx.r[3].s64 + 1;
	// 8240B9C0: 396B0164  addi r11, r11, 0x164
	ctx.r[11].s64 = ctx.r[11].s64 + 356;
	// 8240B9C4: 7F03C800  cmpw cr6, r3, r25
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[25].s32, &mut ctx.xer);
	// 8240B9C8: 4099FFE0  ble cr6, 0x8240b9a8
	if !ctx.cr[6].gt {
	pc = 0x8240B9A8; continue 'dispatch;
	}
	pc = 0x8240B9CC; continue 'dispatch;
            }
            0x8240B9CC => {
    //   block [0x8240B9CC..0x8240B9D0)
	// 8240B9CC: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	pc = 0x8240B9D0; continue 'dispatch;
            }
            0x8240B9D0 => {
    //   block [0x8240B9D0..0x8240BA00)
	// 8240B9D0: 38A00300  li r5, 0x300
	ctx.r[5].s64 = 768;
	// 8240B9D4: 38610360  addi r3, r1, 0x360
	ctx.r[3].s64 = ctx.r[1].s64 + 864;
	// 8240B9D8: 4BFB5229  bl 0x823c0c00
	ctx.lr = 0x8240B9DC;
	sub_823C0C00(ctx, base);
	// 8240B9DC: 3880FFFF  li r4, -1
	ctx.r[4].s64 = -1;
	// 8240B9E0: 7FE8FB78  mr r8, r31
	ctx.r[8].u64 = ctx.r[31].u64;
	// 8240B9E4: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8240B9E8: 7F1FC800  cmpw cr6, r31, r25
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[25].s32, &mut ctx.xer);
	// 8240B9EC: 419900BC  bgt cr6, 0x8240baa8
	if ctx.cr[6].gt {
	pc = 0x8240BAA8; continue 'dispatch;
	}
	// 8240B9F0: 1D7F0164  mulli r11, r31, 0x164
	ctx.r[11].s32 = ((ctx.r[31].s32 as i64 * 356 as i64) as i32);
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 8240B9F4: 7D4BDA14  add r10, r11, r27
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[27].u64;
	// 8240B9F8: 57EB103A  slwi r11, r31, 2
	ctx.r[11].u32 = ctx.r[31].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8240B9FC: 38CA0034  addi r6, r10, 0x34
	ctx.r[6].s64 = ctx.r[10].s64 + 52;
	pc = 0x8240BA00; continue 'dispatch;
            }
            0x8240BA00 => {
    //   block [0x8240BA00..0x8240BA64)
	// 8240BA00: 39210360  addi r9, r1, 0x360
	ctx.r[9].s64 = ctx.r[1].s64 + 864;
	// 8240BA04: 8146FFE0  lwz r10, -0x20(r6)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-32 as u32) ) } as u64;
	// 8240BA08: 38E10960  addi r7, r1, 0x960
	ctx.r[7].s64 = ctx.r[1].s64 + 2400;
	// 8240BA0C: 7D2B482E  lwzx r9, r11, r9
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 8240BA10: 7F0B392E  stwx r24, r11, r7
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[7].u32), ctx.r[24].u32) };
	// 8240BA14: 2B090001  cmplwi cr6, r9, 1
	ctx.cr[6].compare_u32(ctx.r[9].u32, 1 as u32, &mut ctx.xer);
	// 8240BA18: 409A007C  bne cr6, 0x8240ba94
	if !ctx.cr[6].eq {
	pc = 0x8240BA94; continue 'dispatch;
	}
	// 8240BA1C: 81260000  lwz r9, 0(r6)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 8240BA20: 2B090001  cmplwi cr6, r9, 1
	ctx.cr[6].compare_u32(ctx.r[9].u32, 1 as u32, &mut ctx.xer);
	// 8240BA24: 409A0040  bne cr6, 0x8240ba64
	if !ctx.cr[6].eq {
	pc = 0x8240BA64; continue 'dispatch;
	}
	// 8240BA28: 812A0040  lwz r9, 0x40(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(64 as u32) ) } as u64;
	// 8240BA2C: 2F0903E8  cmpwi cr6, r9, 0x3e8
	ctx.cr[6].compare_i32(ctx.r[9].s32, 1000, &mut ctx.xer);
	// 8240BA30: 40980034  bge cr6, 0x8240ba64
	if !ctx.cr[6].lt {
	pc = 0x8240BA64; continue 'dispatch;
	}
	// 8240BA34: 7D2A07B4  extsw r10, r9
	ctx.r[10].s64 = ctx.r[9].s32 as i64;
	// 8240BA38: C00600B8  lfs f0, 0xb8(r6)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(184 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240BA3C: 39210050  addi r9, r1, 0x50
	ctx.r[9].s64 = ctx.r[1].s64 + 80;
	// 8240BA40: F9410058  std r10, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u64 ) };
	// 8240BA44: C9A10058  lfd f13, 0x58(r1)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 8240BA48: FDA06E9C  fcfid f13, f13
	ctx.f[13].f64 = (ctx.f[13].s64 as f64);
	// 8240BA4C: FDA06818  frsp f13, f13
	ctx.f[13].f64 = (ctx.f[13].f64 as f32) as f64;
	// 8240BA50: EC000372  fmuls f0, f0, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[13].f64) as f32) as f64);
	// 8240BA54: FC00001E  fctiwz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[0].f64.trunc() as i32 as i64 };
	// 8240BA58: 7C004FAE  stfiwx f0, 0, r9
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32, tmp.u32) };
	// 8240BA5C: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8240BA60: 4800000C  b 0x8240ba6c
	pc = 0x8240BA6C; continue 'dispatch;
            }
            0x8240BA64 => {
    //   block [0x8240BA64..0x8240BA6C)
	// 8240BA64: 814A0040  lwz r10, 0x40(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(64 as u32) ) } as u64;
	// 8240BA68: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	pc = 0x8240BA6C; continue 'dispatch;
            }
            0x8240BA6C => {
    //   block [0x8240BA6C..0x8240BA88)
	// 8240BA6C: 7F0A2000  cmpw cr6, r10, r4
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[4].s32, &mut ctx.xer);
	// 8240BA70: 41980018  blt cr6, 0x8240ba88
	if ctx.cr[6].lt {
	pc = 0x8240BA88; continue 'dispatch;
	}
	// 8240BA74: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 8240BA78: 41980010  blt cr6, 0x8240ba88
	if ctx.cr[6].lt {
	pc = 0x8240BA88; continue 'dispatch;
	}
	// 8240BA7C: 7F0A2000  cmpw cr6, r10, r4
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[4].s32, &mut ctx.xer);
	// 8240BA80: 409A0014  bne cr6, 0x8240ba94
	if !ctx.cr[6].eq {
	pc = 0x8240BA94; continue 'dispatch;
	}
	// 8240BA84: 4800000C  b 0x8240ba90
	pc = 0x8240BA90; continue 'dispatch;
            }
            0x8240BA88 => {
    //   block [0x8240BA88..0x8240BA90)
	// 8240BA88: 7D445378  mr r4, r10
	ctx.r[4].u64 = ctx.r[10].u64;
	// 8240BA8C: 7CA82B78  mr r8, r5
	ctx.r[8].u64 = ctx.r[5].u64;
	pc = 0x8240BA90; continue 'dispatch;
            }
            0x8240BA90 => {
    //   block [0x8240BA90..0x8240BA94)
	// 8240BA90: 7F4B392E  stwx r26, r11, r7
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[7].u32), ctx.r[26].u32) };
	pc = 0x8240BA94; continue 'dispatch;
            }
            0x8240BA94 => {
    //   block [0x8240BA94..0x8240BAA8)
	// 8240BA94: 38A50001  addi r5, r5, 1
	ctx.r[5].s64 = ctx.r[5].s64 + 1;
	// 8240BA98: 38C60164  addi r6, r6, 0x164
	ctx.r[6].s64 = ctx.r[6].s64 + 356;
	// 8240BA9C: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8240BAA0: 7F05C800  cmpw cr6, r5, r25
	ctx.cr[6].compare_i32(ctx.r[5].s32, ctx.r[25].s32, &mut ctx.xer);
	// 8240BAA4: 4099FF5C  ble cr6, 0x8240ba00
	if !ctx.cr[6].gt {
	pc = 0x8240BA00; continue 'dispatch;
	}
	pc = 0x8240BAA8; continue 'dispatch;
            }
            0x8240BAA8 => {
    //   block [0x8240BAA8..0x8240BACC)
	// 8240BAA8: 7F172000  cmpw cr6, r23, r4
	ctx.cr[6].compare_i32(ctx.r[23].s32, ctx.r[4].s32, &mut ctx.xer);
	// 8240BAAC: 40980020  bge cr6, 0x8240bacc
	if !ctx.cr[6].lt {
	pc = 0x8240BACC; continue 'dispatch;
	}
	// 8240BAB0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240BAB4: 7EE4BB78  mr r4, r23
	ctx.r[4].u64 = ctx.r[23].u64;
	// 8240BAB8: 386BDAE8  addi r3, r11, -0x2518
	ctx.r[3].s64 = ctx.r[11].s64 + -9496;
	// 8240BABC: 4BEA74C5  bl 0x822b2f80
	ctx.lr = 0x8240BAC0;
	sub_822B2F80(ctx, base);
	// 8240BAC0: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 8240BAC4: 60630010  ori r3, r3, 0x10
	ctx.r[3].u64 = ctx.r[3].u64 | 16;
	// 8240BAC8: 480000A0  b 0x8240bb68
	pc = 0x8240BB68; continue 'dispatch;
            }
            0x8240BACC => {
    //   block [0x8240BACC..0x8240BAF0)
	// 8240BACC: 3880FFFF  li r4, -1
	ctx.r[4].s64 = -1;
	// 8240BAD0: 7F07C378  mr r7, r24
	ctx.r[7].u64 = ctx.r[24].u64;
	// 8240BAD4: 7F08C800  cmpw cr6, r8, r25
	ctx.cr[6].compare_i32(ctx.r[8].s32, ctx.r[25].s32, &mut ctx.xer);
	// 8240BAD8: 4199005C  bgt cr6, 0x8240bb34
	if ctx.cr[6].gt {
	pc = 0x8240BB34; continue 'dispatch;
	}
	// 8240BADC: 1D680164  mulli r11, r8, 0x164
	ctx.r[11].s32 = ((ctx.r[8].s32 as i64 * 356 as i64) as i32);
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 8240BAE0: 550A103A  slwi r10, r8, 2
	ctx.r[10].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8240BAE4: 39210960  addi r9, r1, 0x960
	ctx.r[9].s64 = ctx.r[1].s64 + 2400;
	// 8240BAE8: 7D6BDA14  add r11, r11, r27
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[27].u64;
	// 8240BAEC: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	pc = 0x8240BAF0; continue 'dispatch;
            }
            0x8240BAF0 => {
    //   block [0x8240BAF0..0x8240BB10)
	// 8240BAF0: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8240BAF4: 2F090001  cmpwi cr6, r9, 1
	ctx.cr[6].compare_i32(ctx.r[9].s32, 1, &mut ctx.xer);
	// 8240BAF8: 409A0020  bne cr6, 0x8240bb18
	if !ctx.cr[6].eq {
	pc = 0x8240BB18; continue 'dispatch;
	}
	// 8240BAFC: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 8240BB00: 41980010  blt cr6, 0x8240bb10
	if ctx.cr[6].lt {
	pc = 0x8240BB10; continue 'dispatch;
	}
	// 8240BB04: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8240BB08: 7F074840  cmplw cr6, r7, r9
	ctx.cr[6].compare_u32(ctx.r[7].u32, ctx.r[9].u32, &mut ctx.xer);
	// 8240BB0C: 4099000C  ble cr6, 0x8240bb18
	if !ctx.cr[6].gt {
	pc = 0x8240BB18; continue 'dispatch;
	}
	pc = 0x8240BB10; continue 'dispatch;
            }
            0x8240BB10 => {
    //   block [0x8240BB10..0x8240BB18)
	// 8240BB10: 80EB0000  lwz r7, 0(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8240BB14: 7D044378  mr r4, r8
	ctx.r[4].u64 = ctx.r[8].u64;
	pc = 0x8240BB18; continue 'dispatch;
            }
            0x8240BB18 => {
    //   block [0x8240BB18..0x8240BB34)
	// 8240BB18: 39080001  addi r8, r8, 1
	ctx.r[8].s64 = ctx.r[8].s64 + 1;
	// 8240BB1C: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 8240BB20: 396B0164  addi r11, r11, 0x164
	ctx.r[11].s64 = ctx.r[11].s64 + 356;
	// 8240BB24: 7F08C800  cmpw cr6, r8, r25
	ctx.cr[6].compare_i32(ctx.r[8].s32, ctx.r[25].s32, &mut ctx.xer);
	// 8240BB28: 4099FFC8  ble cr6, 0x8240baf0
	if !ctx.cr[6].gt {
	pc = 0x8240BAF0; continue 'dispatch;
	}
	// 8240BB2C: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 8240BB30: 40980014  bge cr6, 0x8240bb44
	if !ctx.cr[6].lt {
	pc = 0x8240BB44; continue 'dispatch;
	}
	pc = 0x8240BB34; continue 'dispatch;
            }
            0x8240BB34 => {
    //   block [0x8240BB34..0x8240BB44)
	// 8240BB34: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240BB38: 386BDAA0  addi r3, r11, -0x2560
	ctx.r[3].s64 = ctx.r[11].s64 + -9568;
	// 8240BB3C: 4BEA7445  bl 0x822b2f80
	ctx.lr = 0x8240BB40;
	sub_822B2F80(ctx, base);
	// 8240BB40: 48000024  b 0x8240bb64
	pc = 0x8240BB64; continue 'dispatch;
            }
            0x8240BB44 => {
    //   block [0x8240BB44..0x8240BB4C)
	// 8240BB44: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 8240BB48: 48000020  b 0x8240bb68
	pc = 0x8240BB68; continue 'dispatch;
            }
            0x8240BB4C => {
    //   block [0x8240BB4C..0x8240BB64)
	// 8240BB4C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240BB50: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 8240BB54: 386BDA40  addi r3, r11, -0x25c0
	ctx.r[3].s64 = ctx.r[11].s64 + -9664;
	// 8240BB58: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 8240BB5C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8240BB60: 4BEA7421  bl 0x822b2f80
	ctx.lr = 0x8240BB64;
	sub_822B2F80(ctx, base);
	pc = 0x8240BB64; continue 'dispatch;
            }
            0x8240BB64 => {
    //   block [0x8240BB64..0x8240BB68)
	// 8240BB64: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	pc = 0x8240BB68; continue 'dispatch;
            }
            0x8240BB68 => {
    //   block [0x8240BB68..0x8240BB70)
	// 8240BB68: 38210CB0  addi r1, r1, 0xcb0
	ctx.r[1].s64 = ctx.r[1].s64 + 3248;
	// 8240BB6C: 48129588  b 0x825350f4
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240BB70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8240BB70 size=244
    let mut pc: u32 = 0x8240BB70;
    'dispatch: loop {
        match pc {
            0x8240BB70 => {
    //   block [0x8240BB70..0x8240BB9C)
	// 8240BB70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240BB74: 4812953D  bl 0x825350b0
	ctx.lr = 0x8240BB78;
	sub_82535080(ctx, base);
	// 8240BB78: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240BB7C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8240BB80: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 8240BB84: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 8240BB88: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 8240BB8C: 2F07FFFF  cmpwi cr6, r7, -1
	ctx.cr[6].compare_i32(ctx.r[7].s32, -1, &mut ctx.xer);
	// 8240BB90: 7CFD3B78  mr r29, r7
	ctx.r[29].u64 = ctx.r[7].u64;
	// 8240BB94: 409A0008  bne cr6, 0x8240bb9c
	if !ctx.cr[6].eq {
	pc = 0x8240BB9C; continue 'dispatch;
	}
	// 8240BB98: 83BE0034  lwz r29, 0x34(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(52 as u32) ) } as u64;
	pc = 0x8240BB9C; continue 'dispatch;
            }
            0x8240BB9C => {
    //   block [0x8240BB9C..0x8240BBD4)
	// 8240BB9C: 817E0040  lwz r11, 0x40(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(64 as u32) ) } as u64;
	// 8240BBA0: 2F0B03E8  cmpwi cr6, r11, 0x3e8
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1000, &mut ctx.xer);
	// 8240BBA4: 40980030  bge cr6, 0x8240bbd4
	if !ctx.cr[6].lt {
	pc = 0x8240BBD4; continue 'dispatch;
	}
	// 8240BBA8: 7D6B07B4  extsw r11, r11
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 8240BBAC: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 8240BBB0: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 8240BBB4: C8010050  lfd f0, 0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 8240BBB8: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 8240BBBC: FC000018  frsp f0, f0
	ctx.f[0].f64 = (ctx.f[0].f64 as f32) as f64;
	// 8240BBC0: EC000072  fmuls f0, f0, f1
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[1].f64) as f32) as f64);
	// 8240BBC4: FC00001E  fctiwz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[0].f64.trunc() as i32 as i64 };
	// 8240BBC8: 7C0057AE  stfiwx f0, 0, r10
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32, tmp.u32) };
	// 8240BBCC: 83810050  lwz r28, 0x50(r1)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8240BBD0: 48000008  b 0x8240bbd8
	pc = 0x8240BBD8; continue 'dispatch;
            }
            0x8240BBD4 => {
    //   block [0x8240BBD4..0x8240BBD8)
	// 8240BBD4: 7D7C5B78  mr r28, r11
	ctx.r[28].u64 = ctx.r[11].u64;
	pc = 0x8240BBD8; continue 'dispatch;
            }
            0x8240BBD8 => {
    //   block [0x8240BBD8..0x8240BC10)
	// 8240BBD8: 817E0044  lwz r11, 0x44(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(68 as u32) ) } as u64;
	// 8240BBDC: 2F0B0064  cmpwi cr6, r11, 0x64
	ctx.cr[6].compare_i32(ctx.r[11].s32, 100, &mut ctx.xer);
	// 8240BBE0: 419A0030  beq cr6, 0x8240bc10
	if ctx.cr[6].eq {
	pc = 0x8240BC10; continue 'dispatch;
	}
	// 8240BBE4: 480028FD  bl 0x8240e4e0
	ctx.lr = 0x8240BBE8;
	sub_8240E4E0(ctx, base);
	// 8240BBE8: 39400064  li r10, 0x64
	ctx.r[10].s64 = 100;
	// 8240BBEC: 817E0044  lwz r11, 0x44(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(68 as u32) ) } as u64;
	// 8240BBF0: 7D435396  divwu r10, r3, r10
	ctx.r[10].u32 = ctx.r[3].u32 / ctx.r[10].u32;
	// 8240BBF4: 1D4A0064  mulli r10, r10, 0x64
	ctx.r[10].s32 = ((ctx.r[10].s32 as i64 * 100 as i64) as i32);
	ctx.r[10].s64 = ctx.r[10].s32 as i64;
	// 8240BBF8: 7D4A1850  subf r10, r10, r3
	ctx.r[10].s64 = ctx.r[3].s64 - ctx.r[10].s64;
	// 8240BBFC: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 8240BC00: 41990010  bgt cr6, 0x8240bc10
	if ctx.cr[6].gt {
	pc = 0x8240BC10; continue 'dispatch;
	}
	// 8240BC04: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 8240BC08: 60630013  ori r3, r3, 0x13
	ctx.r[3].u64 = ctx.r[3].u64 | 19;
	// 8240BC0C: 48000050  b 0x8240bc5c
	pc = 0x8240BC5C; continue 'dispatch;
            }
            0x8240BC10 => {
    //   block [0x8240BC10..0x8240BC58)
	// 8240BC10: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 8240BC14: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 8240BC18: 38A000BF  li r5, 0xbf
	ctx.r[5].s64 = 191;
	// 8240BC1C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8240BC20: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8240BC24: 4BFFFC8D  bl 0x8240b8b0
	ctx.lr = 0x8240BC28;
	sub_8240B8B0(ctx, base);
	// 8240BC28: 7C641B79  or. r4, r3, r3
	ctx.r[4].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 8240BC2C: 4180002C  blt 0x8240bc58
	if ctx.cr[0].lt {
	pc = 0x8240BC58; continue 'dispatch;
	}
	// 8240BC30: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8240BC34: 4BFFF2D5  bl 0x8240af08
	ctx.lr = 0x8240BC38;
	sub_8240AF08(ctx, base);
	// 8240BC38: 1D640164  mulli r11, r4, 0x164
	ctx.r[11].s32 = ((ctx.r[4].s32 as i64 * 356 as i64) as i32);
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 8240BC3C: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 8240BC40: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 8240BC44: 936B0000  stw r27, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 8240BC48: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8240BC4C: 93CB0014  stw r30, 0x14(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[30].u32 ) };
	// 8240BC50: 934B0018  stw r26, 0x18(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[26].u32 ) };
	// 8240BC54: 93AB0030  stw r29, 0x30(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(48 as u32), ctx.r[29].u32 ) };
	pc = 0x8240BC58; continue 'dispatch;
            }
            0x8240BC58 => {
    //   block [0x8240BC58..0x8240BC5C)
	// 8240BC58: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	pc = 0x8240BC5C; continue 'dispatch;
            }
            0x8240BC5C => {
    //   block [0x8240BC5C..0x8240BC64)
	// 8240BC5C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8240BC60: 481294A0  b 0x82535100
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240BC68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8240BC68 size=180
    let mut pc: u32 = 0x8240BC68;
    'dispatch: loop {
        match pc {
            0x8240BC68 => {
    //   block [0x8240BC68..0x8240BC94)
	// 8240BC68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240BC6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240BC70: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240BC74: 3CE0820D  lis r7, -0x7df3
	ctx.r[7].s64 = -2113077248;
	// 8240BC78: 3D008200  lis r8, -0x7e00
	ctx.r[8].s64 = -2113929216;
	// 8240BC7C: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 8240BC80: 394000BF  li r10, 0xbf
	ctx.r[10].s64 = 191;
	// 8240BC84: 39650154  addi r11, r5, 0x154
	ctx.r[11].s64 = ctx.r[5].s64 + 340;
	// 8240BC88: C1A71FF8  lfs f13, 0x1ff8(r7)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(8184 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8240BC8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8240BC90: C0081850  lfs f0, 0x1850(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(6224 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	pc = 0x8240BC94; continue 'dispatch;
            }
            0x8240BC94 => {
    //   block [0x8240BC94..0x8240BCDC)
	// 8240BC94: D00BFFF8  stfs f0, -8(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-8 as u32), tmp.u32 ) };
	// 8240BC98: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8240BC9C: D1ABFFFC  stfs f13, -4(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-4 as u32), tmp.u32 ) };
	// 8240BCA0: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8240BCA4: D1AB0004  stfs f13, 4(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 8240BCA8: D00B0008  stfs f0, 8(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 8240BCAC: D00B000C  stfs f0, 0xc(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 8240BCB0: 396B0164  addi r11, r11, 0x164
	ctx.r[11].s64 = ctx.r[11].s64 + 356;
	// 8240BCB4: 4080FFE0  bge 0x8240bc94
	if !ctx.cr[0].lt {
	pc = 0x8240BC94; continue 'dispatch;
	}
	// 8240BCB8: 3D600001  lis r11, 1
	ctx.r[11].s64 = 65536;
	// 8240BCBC: 3D400001  lis r10, 1
	ctx.r[10].s64 = 65536;
	// 8240BCC0: 616B0B00  ori r11, r11, 0xb00
	ctx.r[11].u64 = ctx.r[11].u64 | 2816;
	// 8240BCC4: 61480D04  ori r8, r10, 0xd04
	ctx.r[8].u64 = ctx.r[10].u64 | 3332;
	// 8240BCC8: 39400040  li r10, 0x40
	ctx.r[10].s64 = 64;
	// 8240BCCC: 7D25592E  stwx r9, r5, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[5].u32.wrapping_add(ctx.r[11].u32), ctx.r[9].u32) };
	// 8240BCD0: 3D650001  addis r11, r5, 1
	ctx.r[11].s64 = ctx.r[5].s64 + 65536;
	// 8240BCD4: 396B0C04  addi r11, r11, 0xc04
	ctx.r[11].s64 = ctx.r[11].s64 + 3076;
	// 8240BCD8: 7D45412E  stwx r10, r5, r8
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[5].u32.wrapping_add(ctx.r[8].u32), ctx.r[10].u32) };
	pc = 0x8240BCDC; continue 'dispatch;
            }
            0x8240BCDC => {
    //   block [0x8240BCDC..0x8240BCF4)
	// 8240BCDC: 912BFF00  stw r9, -0x100(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-256 as u32), ctx.r[9].u32 ) };
	// 8240BCE0: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8240BCE4: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8240BCE8: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8240BCEC: 4082FFF0  bne 0x8240bcdc
	if !ctx.cr[0].eq {
	pc = 0x8240BCDC; continue 'dispatch;
	}
	// 8240BCF0: 7D244B78  mr r4, r9
	ctx.r[4].u64 = ctx.r[9].u64;
	pc = 0x8240BCF4; continue 'dispatch;
            }
            0x8240BCF4 => {
    //   block [0x8240BCF4..0x8240BD1C)
	// 8240BCF4: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 8240BCF8: 4BFFF211  bl 0x8240af08
	ctx.lr = 0x8240BCFC;
	sub_8240AF08(ctx, base);
	// 8240BCFC: 38840001  addi r4, r4, 1
	ctx.r[4].s64 = ctx.r[4].s64 + 1;
	// 8240BD00: 2F0400C0  cmpwi cr6, r4, 0xc0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 192, &mut ctx.xer);
	// 8240BD04: 4198FFF0  blt cr6, 0x8240bcf4
	if ctx.cr[6].lt {
	pc = 0x8240BCF4; continue 'dispatch;
	}
	// 8240BD08: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 8240BD0C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8240BD10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240BD14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240BD18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240BD20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8240BD20 size=12
    let mut pc: u32 = 0x8240BD20;
    'dispatch: loop {
        match pc {
            0x8240BD20 => {
    //   block [0x8240BD20..0x8240BD2C)
	// 8240BD20: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8240BD24: C02B1850  lfs f1, 0x1850(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(6224 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8240BD28: 4BFFFE48  b 0x8240bb70
	sub_8240BB70(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240BD30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8240BD30 size=4
    let mut pc: u32 = 0x8240BD30;
    'dispatch: loop {
        match pc {
            0x8240BD30 => {
    //   block [0x8240BD30..0x8240BD34)
	// 8240BD30: 4BFFFE40  b 0x8240bb70
	sub_8240BB70(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240BD38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8240BD38 size=172
    let mut pc: u32 = 0x8240BD38;
    'dispatch: loop {
        match pc {
            0x8240BD38 => {
    //   block [0x8240BD38..0x8240BD84)
	// 8240BD38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240BD3C: 4812937D  bl 0x825350b8
	ctx.lr = 0x8240BD40;
	sub_82535080(ctx, base);
	// 8240BD40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240BD44: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8240BD48: 2B04003F  cmplwi cr6, r4, 0x3f
	ctx.cr[6].compare_u32(ctx.r[4].u32, 63 as u32, &mut ctx.xer);
	// 8240BD4C: 4199007C  bgt cr6, 0x8240bdc8
	if ctx.cr[6].gt {
	pc = 0x8240BDC8; continue 'dispatch;
	}
	// 8240BD50: 1D642008  mulli r11, r4, 0x2008
	ctx.r[11].s32 = ((ctx.r[4].s32 as i64 * 8200 as i64) as i32);
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 8240BD54: 7D6BEA14  add r11, r11, r29
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 8240BD58: 3BCB0108  addi r30, r11, 0x108
	ctx.r[30].s64 = ctx.r[11].s64 + 264;
	// 8240BD5C: 809E0000  lwz r4, 0(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8240BD60: 28040000  cmplwi r4, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8240BD64: 41820050  beq 0x8240bdb4
	if ctx.cr[0].eq {
	pc = 0x8240BDB4; continue 'dispatch;
	}
	// 8240BD68: 3C7D0008  addis r3, r29, 8
	ctx.r[3].s64 = ctx.r[29].s64 + 524288;
	// 8240BD6C: 3863030C  addi r3, r3, 0x30c
	ctx.r[3].s64 = ctx.r[3].s64 + 780;
	// 8240BD70: 4BFFAD01  bl 0x82406a70
	ctx.lr = 0x8240BD74;
	sub_82406A70(ctx, base);
	// 8240BD74: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8240BD78: 4081003C  ble 0x8240bdb4
	if !ctx.cr[0].gt {
	pc = 0x8240BDB4; continue 'dispatch;
	}
	// 8240BD7C: 3BFE0008  addi r31, r30, 8
	ctx.r[31].s64 = ctx.r[30].s64 + 8;
	// 8240BD80: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	pc = 0x8240BD84; continue 'dispatch;
            }
            0x8240BD84 => {
    //   block [0x8240BD84..0x8240BDA8)
	// 8240BD84: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8240BD88: 2C040000  cmpwi r4, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8240BD8C: 4180001C  blt 0x8240bda8
	if ctx.cr[0].lt {
	pc = 0x8240BDA8; continue 'dispatch;
	}
	// 8240BD90: 3D600008  lis r11, 8
	ctx.r[11].s64 = 524288;
	// 8240BD94: 616B0308  ori r11, r11, 0x308
	ctx.r[11].u64 = ctx.r[11].u64 | 776;
	// 8240BD98: 7C7D582E  lwzx r3, r29, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8240BD9C: 48003DE5  bl 0x8240fb80
	ctx.lr = 0x8240BDA0;
	sub_8240FB80(ctx, base);
	// 8240BDA0: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 8240BDA4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	pc = 0x8240BDA8; continue 'dispatch;
            }
            0x8240BDA8 => {
    //   block [0x8240BDA8..0x8240BDB4)
	// 8240BDA8: 379CFFFF  addic. r28, r28, -1
	ctx.xer.ca = (ctx.r[28].u32 > (!(-1 as u32)));
	ctx.r[28].s64 = ctx.r[28].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 8240BDAC: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 8240BDB0: 4082FFD4  bne 0x8240bd84
	if !ctx.cr[0].eq {
	pc = 0x8240BD84; continue 'dispatch;
	}
	pc = 0x8240BDB4; continue 'dispatch;
            }
            0x8240BDB4 => {
    //   block [0x8240BDB4..0x8240BDC8)
	// 8240BDB4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8240BDB8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8240BDBC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8240BDC0: 917E0004  stw r11, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8240BDC4: 48000018  b 0x8240bddc
	pc = 0x8240BDDC; continue 'dispatch;
            }
            0x8240BDC8 => {
    //   block [0x8240BDC8..0x8240BDDC)
	// 8240BDC8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240BDCC: 386BDB38  addi r3, r11, -0x24c8
	ctx.r[3].s64 = ctx.r[11].s64 + -9416;
	// 8240BDD0: 4BEA71B1  bl 0x822b2f80
	ctx.lr = 0x8240BDD4;
	sub_822B2F80(ctx, base);
	// 8240BDD4: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 8240BDD8: 6063001E  ori r3, r3, 0x1e
	ctx.r[3].u64 = ctx.r[3].u64 | 30;
	pc = 0x8240BDDC; continue 'dispatch;
            }
            0x8240BDDC => {
    //   block [0x8240BDDC..0x8240BDE4)
	// 8240BDDC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8240BDE0: 48129328  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240BDE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8240BDE8 size=76
    let mut pc: u32 = 0x8240BDE8;
    'dispatch: loop {
        match pc {
            0x8240BDE8 => {
    //   block [0x8240BDE8..0x8240BE04)
	// 8240BDE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240BDEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240BDF0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8240BDF4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8240BDF8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240BDFC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8240BE00: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	pc = 0x8240BE04; continue 'dispatch;
            }
            0x8240BE04 => {
    //   block [0x8240BE04..0x8240BE34)
	// 8240BE04: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8240BE08: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8240BE0C: 4BFFFF2D  bl 0x8240bd38
	ctx.lr = 0x8240BE10;
	sub_8240BD38(ctx, base);
	// 8240BE10: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 8240BE14: 2F1F0040  cmpwi cr6, r31, 0x40
	ctx.cr[6].compare_i32(ctx.r[31].s32, 64, &mut ctx.xer);
	// 8240BE18: 4198FFEC  blt cr6, 0x8240be04
	if ctx.cr[6].lt {
	pc = 0x8240BE04; continue 'dispatch;
	}
	// 8240BE1C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8240BE20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240BE24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240BE28: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8240BE2C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8240BE30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240BE38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8240BE38 size=108
    let mut pc: u32 = 0x8240BE38;
    'dispatch: loop {
        match pc {
            0x8240BE38 => {
    //   block [0x8240BE38..0x8240BE44)
	// 8240BE38: 39630108  addi r11, r3, 0x108
	ctx.r[11].s64 = ctx.r[3].s64 + 264;
	// 8240BE3C: 38C0003F  li r6, 0x3f
	ctx.r[6].s64 = 63;
	// 8240BE40: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	pc = 0x8240BE44; continue 'dispatch;
            }
            0x8240BE44 => {
    //   block [0x8240BE44..0x8240BE5C)
	// 8240BE44: 392B0008  addi r9, r11, 8
	ctx.r[9].s64 = ctx.r[11].s64 + 8;
	// 8240BE48: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8240BE4C: 38E0FFFF  li r7, -1
	ctx.r[7].s64 = -1;
	// 8240BE50: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8240BE54: 39000800  li r8, 0x800
	ctx.r[8].s64 = 2048;
	// 8240BE58: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	pc = 0x8240BE5C; continue 'dispatch;
            }
            0x8240BE5C => {
    //   block [0x8240BE5C..0x8240BE88)
	// 8240BE5C: 90E90000  stw r7, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 8240BE60: 39290004  addi r9, r9, 4
	ctx.r[9].s64 = ctx.r[9].s64 + 4;
	// 8240BE64: 4200FFF8  bdnz 0x8240be5c
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x8240BE5C; continue 'dispatch;
	}
	// 8240BE68: 34C6FFFF  addic. r6, r6, -1
	ctx.xer.ca = (ctx.r[6].u32 > (!(-1 as u32)));
	ctx.r[6].s64 = ctx.r[6].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[6].s32, 0, &mut ctx.xer);
	// 8240BE6C: 396B2008  addi r11, r11, 0x2008
	ctx.r[11].s64 = ctx.r[11].s64 + 8200;
	// 8240BE70: 4080FFD4  bge 0x8240be44
	if !ctx.cr[0].lt {
	pc = 0x8240BE44; continue 'dispatch;
	}
	// 8240BE74: 39630008  addi r11, r3, 8
	ctx.r[11].s64 = ctx.r[3].s64 + 8;
	// 8240BE78: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8240BE7C: 39200040  li r9, 0x40
	ctx.r[9].s64 = 64;
	// 8240BE80: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8240BE84: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	pc = 0x8240BE88; continue 'dispatch;
            }
            0x8240BE88 => {
    //   block [0x8240BE88..0x8240BEA4)
	// 8240BE88: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8240BE8C: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8240BE90: 4200FFF8  bdnz 0x8240be88
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x8240BE88; continue 'dispatch;
	}
	// 8240BE94: 3D600008  lis r11, 8
	ctx.r[11].s64 = 524288;
	// 8240BE98: 616B0308  ori r11, r11, 0x308
	ctx.r[11].u64 = ctx.r[11].u64 | 776;
	// 8240BE9C: 7D43592E  stwx r10, r3, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[3].u32.wrapping_add(ctx.r[11].u32), ctx.r[10].u32) };
	// 8240BEA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240BEA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8240BEA8 size=100
    let mut pc: u32 = 0x8240BEA8;
    'dispatch: loop {
        match pc {
            0x8240BEA8 => {
    //   block [0x8240BEA8..0x8240BED8)
	// 8240BEA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240BEAC: 48129211  bl 0x825350bc
	ctx.lr = 0x8240BEB0;
	sub_82535080(ctx, base);
	// 8240BEB0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240BEB4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8240BEB8: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8240BEBC: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8240BEC0: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 8240BEC4: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 8240BEC8: 4BFFFF21  bl 0x8240bde8
	ctx.lr = 0x8240BECC;
	sub_8240BDE8(ctx, base);
	// 8240BECC: 397F0008  addi r11, r31, 8
	ctx.r[11].s64 = ctx.r[31].s64 + 8;
	// 8240BED0: 39400040  li r10, 0x40
	ctx.r[10].s64 = 64;
	// 8240BED4: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	pc = 0x8240BED8; continue 'dispatch;
            }
            0x8240BED8 => {
    //   block [0x8240BED8..0x8240BF00)
	// 8240BED8: 93CB0000  stw r30, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 8240BEDC: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8240BEE0: 4200FFF8  bdnz 0x8240bed8
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x8240BED8; continue 'dispatch;
	}
	// 8240BEE4: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 8240BEE8: 419A0018  beq cr6, 0x8240bf00
	if ctx.cr[6].eq {
	pc = 0x8240BF00; continue 'dispatch;
	}
	// 8240BEEC: 3D600008  lis r11, 8
	ctx.r[11].s64 = 524288;
	// 8240BEF0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8240BEF4: 616B0308  ori r11, r11, 0x308
	ctx.r[11].u64 = ctx.r[11].u64 | 776;
	// 8240BEF8: 7FBF592E  stwx r29, r31, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32), ctx.r[29].u32) };
	// 8240BEFC: 48000008  b 0x8240bf04
	pc = 0x8240BF04; continue 'dispatch;
            }
            0x8240BF00 => {
    //   block [0x8240BF00..0x8240BF04)
	// 8240BF00: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	pc = 0x8240BF04; continue 'dispatch;
            }
            0x8240BF04 => {
    //   block [0x8240BF04..0x8240BF0C)
	// 8240BF04: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8240BF08: 48129204  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240BF10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8240BF10 size=808
    let mut pc: u32 = 0x8240BF10;
    'dispatch: loop {
        match pc {
            0x8240BF10 => {
    //   block [0x8240BF10..0x8240BF60)
	// 8240BF10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240BF14: 48129181  bl 0x82535094
	ctx.lr = 0x8240BF18;
	sub_82535080(ctx, base);
	// 8240BF18: 9421FF10  stwu r1, -0xf0(r1)
	ea = ctx.r[1].u32.wrapping_add(-240 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240BF1C: 7C982378  mr r24, r4
	ctx.r[24].u64 = ctx.r[4].u64;
	// 8240BF20: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 8240BF24: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 8240BF28: 7CD53378  mr r21, r6
	ctx.r[21].u64 = ctx.r[6].u64;
	// 8240BF2C: 2B18003F  cmplwi cr6, r24, 0x3f
	ctx.cr[6].compare_u32(ctx.r[24].u32, 63 as u32, &mut ctx.xer);
	// 8240BF30: 40990030  ble cr6, 0x8240bf60
	if !ctx.cr[6].gt {
	pc = 0x8240BF60; continue 'dispatch;
	}
	// 8240BF34: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240BF38: 386BDB88  addi r3, r11, -0x2478
	ctx.r[3].s64 = ctx.r[11].s64 + -9336;
	// 8240BF3C: 4BEA7045  bl 0x822b2f80
	ctx.lr = 0x8240BF40;
	sub_822B2F80(ctx, base);
	// 8240BF40: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240BF44: 3C808000  lis r4, -0x8000
	ctx.r[4].s64 = -2147483648;
	// 8240BF48: 386BDE28  addi r3, r11, -0x21d8
	ctx.r[3].s64 = ctx.r[11].s64 + -8664;
	// 8240BF4C: 6084001E  ori r4, r4, 0x1e
	ctx.r[4].u64 = ctx.r[4].u64 | 30;
	// 8240BF50: 4BEA7031  bl 0x822b2f80
	ctx.lr = 0x8240BF54;
	sub_822B2F80(ctx, base);
	// 8240BF54: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 8240BF58: 6063001E  ori r3, r3, 0x1e
	ctx.r[3].u64 = ctx.r[3].u64 | 30;
	// 8240BF5C: 480002D4  b 0x8240c230
	pc = 0x8240C230; continue 'dispatch;
            }
            0x8240BF60 => {
    //   block [0x8240BF60..0x8240BF90)
	// 8240BF60: 1D782008  mulli r11, r24, 0x2008
	ctx.r[11].s32 = ((ctx.r[24].s32 as i64 * 8200 as i64) as i32);
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 8240BF64: 7E6BE214  add r19, r11, r28
	ctx.r[19].u64 = ctx.r[11].u64 + ctx.r[28].u64;
	// 8240BF68: 81730108  lwz r11, 0x108(r19)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(264 as u32) ) } as u64;
	// 8240BF6C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8240BF70: 419A0020  beq cr6, 0x8240bf90
	if ctx.cr[6].eq {
	pc = 0x8240BF90; continue 'dispatch;
	}
	// 8240BF74: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240BF78: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 8240BF7C: 386BDDD8  addi r3, r11, -0x2228
	ctx.r[3].s64 = ctx.r[11].s64 + -8744;
	// 8240BF80: 4BEA7001  bl 0x822b2f80
	ctx.lr = 0x8240BF84;
	sub_822B2F80(ctx, base);
	// 8240BF84: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 8240BF88: 60630021  ori r3, r3, 0x21
	ctx.r[3].u64 = ctx.r[3].u64 | 33;
	// 8240BF8C: 480002A4  b 0x8240c230
	pc = 0x8240C230; continue 'dispatch;
            }
            0x8240BF90 => {
    //   block [0x8240BF90..0x8240BFB0)
	// 8240BF90: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8240BF94: 409A001C  bne cr6, 0x8240bfb0
	if !ctx.cr[6].eq {
	pc = 0x8240BFB0; continue 'dispatch;
	}
	// 8240BF98: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240BF9C: 386BDD90  addi r3, r11, -0x2270
	ctx.r[3].s64 = ctx.r[11].s64 + -8816;
	// 8240BFA0: 4BEA6FE1  bl 0x822b2f80
	ctx.lr = 0x8240BFA4;
	sub_822B2F80(ctx, base);
	// 8240BFA4: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 8240BFA8: 60630024  ori r3, r3, 0x24
	ctx.r[3].u64 = ctx.r[3].u64 | 36;
	// 8240BFAC: 48000284  b 0x8240c230
	pc = 0x8240C230; continue 'dispatch;
            }
            0x8240BFB0 => {
    //   block [0x8240BFB0..0x8240C018)
	// 8240BFB0: 3FDC0008  addis r30, r28, 8
	ctx.r[30].s64 = ctx.r[28].s64 + 524288;
	// 8240BFB4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8240BFB8: 3BDE030C  addi r30, r30, 0x30c
	ctx.r[30].s64 = ctx.r[30].s64 + 780;
	// 8240BFBC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8240BFC0: 4BFFAA31  bl 0x824069f0
	ctx.lr = 0x8240BFC4;
	sub_824069F0(ctx, base);
	// 8240BFC4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8240BFC8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8240BFCC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8240BFD0: 4BFFA9E1  bl 0x824069b0
	ctx.lr = 0x8240BFD4;
	sub_824069B0(ctx, base);
	// 8240BFD4: 7C640774  extsb r4, r3
	ctx.r[4].s64 = ctx.r[3].s8 as i64;
	// 8240BFD8: 2F040002  cmpwi cr6, r4, 2
	ctx.cr[6].compare_i32(ctx.r[4].s32, 2, &mut ctx.xer);
	// 8240BFDC: 409A023C  bne cr6, 0x8240c218
	if !ctx.cr[6].eq {
	pc = 0x8240C218; continue 'dispatch;
	}
	// 8240BFE0: 7FAB0774  extsb r11, r29
	ctx.r[11].s64 = ctx.r[29].s8 as i64;
	// 8240BFE4: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8240BFE8: 409A0230  bne cr6, 0x8240c218
	if !ctx.cr[6].eq {
	pc = 0x8240C218; continue 'dispatch;
	}
	// 8240BFEC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8240BFF0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8240BFF4: 4BFFA97D  bl 0x82406970
	ctx.lr = 0x8240BFF8;
	sub_82406970(ctx, base);
	// 8240BFF8: 7C6B0775  extsb. r11, r3
	ctx.r[11].s64 = ctx.r[3].s8 as i64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8240BFFC: 4182001C  beq 0x8240c018
	if ctx.cr[0].eq {
	pc = 0x8240C018; continue 'dispatch;
	}
	// 8240C000: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240C004: 386BDD48  addi r3, r11, -0x22b8
	ctx.r[3].s64 = ctx.r[11].s64 + -8888;
	// 8240C008: 4BEA6F79  bl 0x822b2f80
	ctx.lr = 0x8240C00C;
	sub_822B2F80(ctx, base);
	// 8240C00C: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 8240C010: 60630042  ori r3, r3, 0x42
	ctx.r[3].u64 = ctx.r[3].u64 | 66;
	// 8240C014: 4800021C  b 0x8240c230
	pc = 0x8240C230; continue 'dispatch;
            }
            0x8240C018 => {
    //   block [0x8240C018..0x8240C030)
	// 8240C018: 2B150000  cmplwi cr6, r21, 0
	ctx.cr[6].compare_u32(ctx.r[21].u32, 0 as u32, &mut ctx.xer);
	// 8240C01C: 409A0014  bne cr6, 0x8240c030
	if !ctx.cr[6].eq {
	pc = 0x8240C030; continue 'dispatch;
	}
	// 8240C020: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8240C024: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8240C028: 4BFFAA89  bl 0x82406ab0
	ctx.lr = 0x8240C02C;
	sub_82406AB0(ctx, base);
	// 8240C02C: 7EA3FA14  add r21, r3, r31
	ctx.r[21].u64 = ctx.r[3].u64 + ctx.r[31].u64;
	pc = 0x8240C030; continue 'dispatch;
            }
            0x8240C030 => {
    //   block [0x8240C030..0x8240C06C)
	// 8240C030: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8240C034: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8240C038: 4BFFAA39  bl 0x82406a70
	ctx.lr = 0x8240C03C;
	sub_82406A70(ctx, base);
	// 8240C03C: 7C741B78  mr r20, r3
	ctx.r[20].u64 = ctx.r[3].u64;
	// 8240C040: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8240C044: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8240C048: 4BFFAAA9  bl 0x82406af0
	ctx.lr = 0x8240C04C;
	sub_82406AF0(ctx, base);
	// 8240C04C: 3D600008  lis r11, 8
	ctx.r[11].s64 = 524288;
	// 8240C050: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 8240C054: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 8240C058: 61760308  ori r22, r11, 0x308
	ctx.r[22].u64 = ctx.r[11].u64 | 776;
	// 8240C05C: 2F140000  cmpwi cr6, r20, 0
	ctx.cr[6].compare_i32(ctx.r[20].s32, 0, &mut ctx.xer);
	// 8240C060: 409900D8  ble cr6, 0x8240c138
	if !ctx.cr[6].gt {
	pc = 0x8240C138; continue 'dispatch;
	}
	// 8240C064: 3B530110  addi r26, r19, 0x110
	ctx.r[26].s64 = ctx.r[19].s64 + 272;
	// 8240C068: 7EFCB214  add r23, r28, r22
	ctx.r[23].u64 = ctx.r[28].u64 + ctx.r[22].u64;
	pc = 0x8240C06C; continue 'dispatch;
            }
            0x8240C06C => {
    //   block [0x8240C06C..0x8240C138)
	// 8240C06C: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 8240C070: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8240C074: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8240C078: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8240C07C: 4BFFAC55  bl 0x82406cd0
	ctx.lr = 0x8240C080;
	sub_82406CD0(ctx, base);
	// 8240C080: 38C10054  addi r6, r1, 0x54
	ctx.r[6].s64 = ctx.r[1].s64 + 84;
	// 8240C084: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8240C088: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8240C08C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8240C090: 4BFFACC1  bl 0x82406d50
	ctx.lr = 0x8240C094;
	sub_82406D50(ctx, base);
	// 8240C094: 38C10058  addi r6, r1, 0x58
	ctx.r[6].s64 = ctx.r[1].s64 + 88;
	// 8240C098: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8240C09C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8240C0A0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8240C0A4: 4BFFAD2D  bl 0x82406dd0
	ctx.lr = 0x8240C0A8;
	sub_82406DD0(ctx, base);
	// 8240C0A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8240C0AC: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8240C0B0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8240C0B4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8240C0B8: 91610068  stw r11, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 8240C0BC: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8240C0C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8240C0C4: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8240C0C8: 7D6BAA14  add r11, r11, r21
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[21].u64;
	// 8240C0CC: 91610070  stw r11, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[11].u32 ) };
	// 8240C0D0: 4BFFABB1  bl 0x82406c80
	ctx.lr = 0x8240C0D4;
	sub_82406C80(ctx, base);
	// 8240C0D4: 90610074  stw r3, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[3].u32 ) };
	// 8240C0D8: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8240C0DC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8240C0E0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8240C0E4: 4BFFAB65  bl 0x82406c48
	ctx.lr = 0x8240C0E8;
	sub_82406C48(ctx, base);
	// 8240C0E8: 90610078  stw r3, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[3].u32 ) };
	// 8240C0EC: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8240C0F0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8240C0F4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8240C0F8: 4BFFAB11  bl 0x82406c08
	ctx.lr = 0x8240C0FC;
	sub_82406C08(ctx, base);
	// 8240C0FC: 8161006C  lwz r11, 0x6c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(108 as u32) ) } as u64;
	// 8240C100: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 8240C104: 9061007C  stw r3, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[3].u32 ) };
	// 8240C108: 80770000  lwz r3, 0(r23)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(0 as u32) ) } as u64;
	// 8240C10C: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 8240C110: 81610068  lwz r11, 0x68(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 8240C114: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8240C118: 480039F1  bl 0x8240fb08
	ctx.lr = 0x8240C11C;
	sub_8240FB08(ctx, base);
	// 8240C11C: 7C7B1B79  or. r27, r3, r3
	ctx.r[27].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 8240C120: 937A0000  stw r27, 0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 8240C124: 41800060  blt 0x8240c184
	if ctx.cr[0].lt {
	pc = 0x8240C184; continue 'dispatch;
	}
	// 8240C128: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 8240C12C: 3B5A0004  addi r26, r26, 4
	ctx.r[26].s64 = ctx.r[26].s64 + 4;
	// 8240C130: 7F1DA000  cmpw cr6, r29, r20
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[20].s32, &mut ctx.xer);
	// 8240C134: 4198FF38  blt cr6, 0x8240c06c
	if ctx.cr[6].lt {
	pc = 0x8240C06C; continue 'dispatch;
	}
	pc = 0x8240C138; continue 'dispatch;
            }
            0x8240C138 => {
    //   block [0x8240C138..0x8240C184)
	// 8240C138: 7FBCB214  add r29, r28, r22
	ctx.r[29].u64 = ctx.r[28].u64 + ctx.r[22].u64;
	// 8240C13C: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 8240C140: 807D0000  lwz r3, 0(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8240C144: 48003A5D  bl 0x8240fba0
	ctx.lr = 0x8240C148;
	sub_8240FBA0(ctx, base);
	// 8240C148: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8240C14C: 40800060  bge 0x8240c1ac
	if !ctx.cr[0].lt {
	pc = 0x8240C1AC; continue 'dispatch;
	}
	// 8240C150: 815D0000  lwz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8240C154: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240C158: 38A00020  li r5, 0x20
	ctx.r[5].s64 = 32;
	// 8240C15C: 386BDCF0  addi r3, r11, -0x2310
	ctx.r[3].s64 = ctx.r[11].s64 + -8976;
	// 8240C160: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 8240C164: 7C8BCA14  add r4, r11, r25
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[25].u64;
	// 8240C168: 4BEA6E19  bl 0x822b2f80
	ctx.lr = 0x8240C16C;
	sub_822B2F80(ctx, base);
	// 8240C16C: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 8240C170: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8240C174: 4BFFFBC5  bl 0x8240bd38
	ctx.lr = 0x8240C178;
	sub_8240BD38(ctx, base);
	// 8240C178: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 8240C17C: 60630027  ori r3, r3, 0x27
	ctx.r[3].u64 = ctx.r[3].u64 | 39;
	// 8240C180: 480000B0  b 0x8240c230
	pc = 0x8240C230; continue 'dispatch;
            }
            0x8240C184 => {
    //   block [0x8240C184..0x8240C1AC)
	// 8240C184: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 8240C188: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8240C18C: 4BFFFBAD  bl 0x8240bd38
	ctx.lr = 0x8240C190;
	sub_8240BD38(ctx, base);
	// 8240C190: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240C194: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8240C198: 386BDC98  addi r3, r11, -0x2368
	ctx.r[3].s64 = ctx.r[11].s64 + -9064;
	// 8240C19C: 4BEA6DE5  bl 0x822b2f80
	ctx.lr = 0x8240C1A0;
	sub_822B2F80(ctx, base);
	// 8240C1A0: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 8240C1A4: 60630026  ori r3, r3, 0x26
	ctx.r[3].u64 = ctx.r[3].u64 | 38;
	// 8240C1A8: 48000088  b 0x8240c230
	pc = 0x8240C230; continue 'dispatch;
            }
            0x8240C1AC => {
    //   block [0x8240C1AC..0x8240C218)
	// 8240C1AC: 39780002  addi r11, r24, 2
	ctx.r[11].s64 = ctx.r[24].s64 + 2;
	// 8240C1B0: 93F30108  stw r31, 0x108(r19)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[19].u32.wrapping_add(264 as u32), ctx.r[31].u32 ) };
	// 8240C1B4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8240C1B8: 92B3010C  stw r21, 0x10c(r19)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[19].u32.wrapping_add(268 as u32), ctx.r[21].u32 ) };
	// 8240C1BC: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8240C1C0: 7C695670  srawi r9, r3, 0xa
	ctx.xer.ca = (ctx.r[3].s32 < 0) && ((ctx.r[3].u32 & ((1u32 << 10) - 1)) != 0);
	ctx.r[9].s64 = (ctx.r[3].s32 >> 10) as i64;
	// 8240C1C4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8240C1C8: 7FA90194  addze r29, r9
	tmp.s64 = ctx.r[9].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[9].u32);
	ctx.r[29].s64 = tmp.s64;
	// 8240C1CC: 7F295670  srawi r9, r25, 0xa
	ctx.xer.ca = (ctx.r[25].s32 < 0) && ((ctx.r[25].u32 & ((1u32 << 10) - 1)) != 0);
	ctx.r[9].s64 = (ctx.r[25].s32 >> 10) as i64;
	// 8240C1D0: 7D4BE12E  stwx r10, r11, r28
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[28].u32), ctx.r[10].u32) };
	// 8240C1D4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8240C1D8: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 8240C1DC: 7FC90194  addze r30, r9
	tmp.s64 = ctx.r[9].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[9].u32);
	ctx.r[30].s64 = tmp.s64;
	// 8240C1E0: 3BEB0001  addi r31, r11, 1
	ctx.r[31].s64 = ctx.r[11].s64 + 1;
	// 8240C1E4: 93FC0000  stw r31, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 8240C1E8: 4BFFA849  bl 0x82406a30
	ctx.lr = 0x8240C1EC;
	sub_82406A30(ctx, base);
	// 8240C1EC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240C1F0: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 8240C1F4: 396BDC30  addi r11, r11, -0x23d0
	ctx.r[11].s64 = ctx.r[11].s64 + -9168;
	// 8240C1F8: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 8240C1FC: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 8240C200: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 8240C204: 7FC7F378  mr r7, r30
	ctx.r[7].u64 = ctx.r[30].u64;
	// 8240C208: 7FA8EB78  mr r8, r29
	ctx.r[8].u64 = ctx.r[29].u64;
	// 8240C20C: 4BEA6D75  bl 0x822b2f80
	ctx.lr = 0x8240C210;
	sub_822B2F80(ctx, base);
	// 8240C210: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8240C214: 4800001C  b 0x8240c230
	pc = 0x8240C230; continue 'dispatch;
            }
            0x8240C218 => {
    //   block [0x8240C218..0x8240C230)
	// 8240C218: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240C21C: 7FA50774  extsb r5, r29
	ctx.r[5].s64 = ctx.r[29].s8 as i64;
	// 8240C220: 386BDBD8  addi r3, r11, -0x2428
	ctx.r[3].s64 = ctx.r[11].s64 + -9256;
	// 8240C224: 4BEA6D5D  bl 0x822b2f80
	ctx.lr = 0x8240C228;
	sub_822B2F80(ctx, base);
	// 8240C228: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 8240C22C: 6063001D  ori r3, r3, 0x1d
	ctx.r[3].u64 = ctx.r[3].u64 | 29;
	pc = 0x8240C230; continue 'dispatch;
            }
            0x8240C230 => {
    //   block [0x8240C230..0x8240C238)
	// 8240C230: 382100F0  addi r1, r1, 0xf0
	ctx.r[1].s64 = ctx.r[1].s64 + 240;
	// 8240C234: 48128EB0  b 0x825350e4
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240C238(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8240C238 size=264
    let mut pc: u32 = 0x8240C238;
    'dispatch: loop {
        match pc {
            0x8240C238 => {
    //   block [0x8240C238..0x8240C280)
	// 8240C238: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240C23C: 48128E7D  bl 0x825350b8
	ctx.lr = 0x8240C240;
	sub_82535080(ctx, base);
	// 8240C240: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240C244: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8240C248: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8240C24C: 2B1E003F  cmplwi cr6, r30, 0x3f
	ctx.cr[6].compare_u32(ctx.r[30].u32, 63 as u32, &mut ctx.xer);
	// 8240C250: 40990030  ble cr6, 0x8240c280
	if !ctx.cr[6].gt {
	pc = 0x8240C280; continue 'dispatch;
	}
	// 8240C254: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240C258: 386BDB88  addi r3, r11, -0x2478
	ctx.r[3].s64 = ctx.r[11].s64 + -9336;
	// 8240C25C: 4BEA6D25  bl 0x822b2f80
	ctx.lr = 0x8240C260;
	sub_822B2F80(ctx, base);
	// 8240C260: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240C264: 3C808000  lis r4, -0x8000
	ctx.r[4].s64 = -2147483648;
	// 8240C268: 386BDF18  addi r3, r11, -0x20e8
	ctx.r[3].s64 = ctx.r[11].s64 + -8424;
	// 8240C26C: 6084001E  ori r4, r4, 0x1e
	ctx.r[4].u64 = ctx.r[4].u64 | 30;
	// 8240C270: 4BEA6D11  bl 0x822b2f80
	ctx.lr = 0x8240C274;
	sub_822B2F80(ctx, base);
	// 8240C274: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 8240C278: 6063001E  ori r3, r3, 0x1e
	ctx.r[3].u64 = ctx.r[3].u64 | 30;
	// 8240C27C: 480000BC  b 0x8240c338
	pc = 0x8240C338; continue 'dispatch;
            }
            0x8240C280 => {
    //   block [0x8240C280..0x8240C2B0)
	// 8240C280: 1D7E2008  mulli r11, r30, 0x2008
	ctx.r[11].s32 = ((ctx.r[30].s32 as i64 * 8200 as i64) as i32);
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 8240C284: 7F8BFA14  add r28, r11, r31
	ctx.r[28].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 8240C288: 817C0108  lwz r11, 0x108(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(264 as u32) ) } as u64;
	// 8240C28C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8240C290: 409A0020  bne cr6, 0x8240c2b0
	if !ctx.cr[6].eq {
	pc = 0x8240C2B0; continue 'dispatch;
	}
	// 8240C294: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240C298: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8240C29C: 386BDEC8  addi r3, r11, -0x2138
	ctx.r[3].s64 = ctx.r[11].s64 + -8504;
	// 8240C2A0: 4BEA6CE1  bl 0x822b2f80
	ctx.lr = 0x8240C2A4;
	sub_822B2F80(ctx, base);
	// 8240C2A4: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 8240C2A8: 60630020  ori r3, r3, 0x20
	ctx.r[3].u64 = ctx.r[3].u64 | 32;
	// 8240C2AC: 4800008C  b 0x8240c338
	pc = 0x8240C338; continue 'dispatch;
            }
            0x8240C2B0 => {
    //   block [0x8240C2B0..0x8240C338)
	// 8240C2B0: 3FBF0008  addis r29, r31, 8
	ctx.r[29].s64 = ctx.r[31].s64 + 524288;
	// 8240C2B4: 809C0108  lwz r4, 0x108(r28)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(264 as u32) ) } as u64;
	// 8240C2B8: 3BBD030C  addi r29, r29, 0x30c
	ctx.r[29].s64 = ctx.r[29].s64 + 780;
	// 8240C2BC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8240C2C0: 4BFFA7B1  bl 0x82406a70
	ctx.lr = 0x8240C2C4;
	sub_82406A70(ctx, base);
	// 8240C2C4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8240C2C8: 809C0108  lwz r4, 0x108(r28)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(264 as u32) ) } as u64;
	// 8240C2CC: 4BFFA825  bl 0x82406af0
	ctx.lr = 0x8240C2D0;
	sub_82406AF0(ctx, base);
	// 8240C2D0: 3D600008  lis r11, 8
	ctx.r[11].s64 = 524288;
	// 8240C2D4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8240C2D8: 616B0308  ori r11, r11, 0x308
	ctx.r[11].u64 = ctx.r[11].u64 | 776;
	// 8240C2DC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8240C2E0: 7C7F582E  lwzx r3, r31, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8240C2E4: 480038ED  bl 0x8240fbd0
	ctx.lr = 0x8240C2E8;
	sub_8240FBD0(ctx, base);
	// 8240C2E8: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 8240C2EC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8240C2F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8240C2F4: 4BFFFA45  bl 0x8240bd38
	ctx.lr = 0x8240C2F8;
	sub_8240BD38(ctx, base);
	// 8240C2F8: 397E0002  addi r11, r30, 2
	ctx.r[11].s64 = ctx.r[30].s64 + 2;
	// 8240C2FC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8240C300: 5569103A  slwi r9, r11, 2
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 8240C304: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240C308: 7F885670  srawi r8, r28, 0xa
	ctx.xer.ca = (ctx.r[28].s32 < 0) && ((ctx.r[28].u32 & ((1u32 << 10) - 1)) != 0);
	ctx.r[8].s64 = (ctx.r[28].s32 >> 10) as i64;
	// 8240C30C: 386BDE70  addi r3, r11, -0x2190
	ctx.r[3].s64 = ctx.r[11].s64 + -8592;
	// 8240C310: 7CE80194  addze r7, r8
	tmp.s64 = ctx.r[8].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[8].u32);
	ctx.r[7].s64 = tmp.s64;
	// 8240C314: 7D49F92E  stwx r10, r9, r31
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[31].u32), ctx.r[10].u32) };
	// 8240C318: 7FA85670  srawi r8, r29, 0xa
	ctx.xer.ca = (ctx.r[29].s32 < 0) && ((ctx.r[29].u32 & ((1u32 << 10) - 1)) != 0);
	ctx.r[8].s64 = (ctx.r[29].s32 >> 10) as i64;
	// 8240C31C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8240C320: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8240C324: 7CC80194  addze r6, r8
	tmp.s64 = ctx.r[8].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[8].u32);
	ctx.r[6].s64 = tmp.s64;
	// 8240C328: 38ABFFFF  addi r5, r11, -1
	ctx.r[5].s64 = ctx.r[11].s64 + -1;
	// 8240C32C: 90BF0000  stw r5, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[5].u32 ) };
	// 8240C330: 4BEA6C51  bl 0x822b2f80
	ctx.lr = 0x8240C334;
	sub_822B2F80(ctx, base);
	// 8240C334: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	pc = 0x8240C338; continue 'dispatch;
            }
            0x8240C338 => {
    //   block [0x8240C338..0x8240C340)
	// 8240C338: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8240C33C: 48128DCC  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240C340(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8240C340 size=152
    let mut pc: u32 = 0x8240C340;
    'dispatch: loop {
        match pc {
            0x8240C340 => {
    //   block [0x8240C340..0x8240C364)
	// 8240C340: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240C344: 48128D75  bl 0x825350b8
	ctx.lr = 0x8240C348;
	sub_82535080(ctx, base);
	// 8240C348: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240C34C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8240C350: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 8240C354: 3BDF0108  addi r30, r31, 0x108
	ctx.r[30].s64 = ctx.r[31].s64 + 264;
	// 8240C358: 3BA00040  li r29, 0x40
	ctx.r[29].s64 = 64;
	// 8240C35C: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 8240C360: 939F0004  stw r28, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	pc = 0x8240C364; continue 'dispatch;
            }
            0x8240C364 => {
    //   block [0x8240C364..0x8240C390)
	// 8240C364: 809E0000  lwz r4, 0(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8240C368: 28040000  cmplwi r4, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8240C36C: 41820024  beq 0x8240c390
	if ctx.cr[0].eq {
	pc = 0x8240C390; continue 'dispatch;
	}
	// 8240C370: 3C7F0008  addis r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 524288;
	// 8240C374: 3863030C  addi r3, r3, 0x30c
	ctx.r[3].s64 = ctx.r[3].s64 + 780;
	// 8240C378: 4BFFA779  bl 0x82406af0
	ctx.lr = 0x8240C37C;
	sub_82406AF0(ctx, base);
	// 8240C37C: 3D600008  lis r11, 8
	ctx.r[11].s64 = 524288;
	// 8240C380: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8240C384: 616B0308  ori r11, r11, 0x308
	ctx.r[11].u64 = ctx.r[11].u64 | 776;
	// 8240C388: 7C7F582E  lwzx r3, r31, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8240C38C: 48003845  bl 0x8240fbd0
	ctx.lr = 0x8240C390;
	sub_8240FBD0(ctx, base);
	pc = 0x8240C390; continue 'dispatch;
            }
            0x8240C390 => {
    //   block [0x8240C390..0x8240C3B4)
	// 8240C390: 37BDFFFF  addic. r29, r29, -1
	ctx.xer.ca = (ctx.r[29].u32 > (!(-1 as u32)));
	ctx.r[29].s64 = ctx.r[29].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 8240C394: 3BDE2008  addi r30, r30, 0x2008
	ctx.r[30].s64 = ctx.r[30].s64 + 8200;
	// 8240C398: 4082FFCC  bne 0x8240c364
	if !ctx.cr[0].eq {
	pc = 0x8240C364; continue 'dispatch;
	}
	// 8240C39C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8240C3A0: 4BFFFA49  bl 0x8240bde8
	ctx.lr = 0x8240C3A4;
	sub_8240BDE8(ctx, base);
	// 8240C3A4: 397F0008  addi r11, r31, 8
	ctx.r[11].s64 = ctx.r[31].s64 + 8;
	// 8240C3A8: 7F89E378  mr r9, r28
	ctx.r[9].u64 = ctx.r[28].u64;
	// 8240C3AC: 39400040  li r10, 0x40
	ctx.r[10].s64 = 64;
	// 8240C3B0: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	pc = 0x8240C3B4; continue 'dispatch;
            }
            0x8240C3B4 => {
    //   block [0x8240C3B4..0x8240C3D8)
	// 8240C3B4: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8240C3B8: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8240C3BC: 4200FFF8  bdnz 0x8240c3b4
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x8240C3B4; continue 'dispatch;
	}
	// 8240C3C0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240C3C4: 386BDF5C  addi r3, r11, -0x20a4
	ctx.r[3].s64 = ctx.r[11].s64 + -8356;
	// 8240C3C8: 4BEA6BB9  bl 0x822b2f80
	ctx.lr = 0x8240C3CC;
	sub_822B2F80(ctx, base);
	// 8240C3CC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8240C3D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8240C3D4: 48128D34  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240C3D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8240C3D8 size=168
    let mut pc: u32 = 0x8240C3D8;
    'dispatch: loop {
        match pc {
            0x8240C3D8 => {
    //   block [0x8240C3D8..0x8240C40C)
	// 8240C3D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240C3DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240C3E0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8240C3E4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240C3E8: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 8240C3EC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8240C3F0: 409A001C  bne cr6, 0x8240c40c
	if !ctx.cr[6].eq {
	pc = 0x8240C40C; continue 'dispatch;
	}
	// 8240C3F4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240C3F8: 386BDFD0  addi r3, r11, -0x2030
	ctx.r[3].s64 = ctx.r[11].s64 + -8240;
	// 8240C3FC: 4BEA6B85  bl 0x822b2f80
	ctx.lr = 0x8240C400;
	sub_822B2F80(ctx, base);
	// 8240C400: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 8240C404: 60630022  ori r3, r3, 0x22
	ctx.r[3].u64 = ctx.r[3].u64 | 34;
	// 8240C408: 48000064  b 0x8240c46c
	pc = 0x8240C46C; continue 'dispatch;
            }
            0x8240C40C => {
    //   block [0x8240C40C..0x8240C444)
	// 8240C40C: 2B04003F  cmplwi cr6, r4, 0x3f
	ctx.cr[6].compare_u32(ctx.r[4].u32, 63 as u32, &mut ctx.xer);
	// 8240C410: 40990034  ble cr6, 0x8240c444
	if !ctx.cr[6].gt {
	pc = 0x8240C444; continue 'dispatch;
	}
	// 8240C414: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240C418: 386BDB88  addi r3, r11, -0x2478
	ctx.r[3].s64 = ctx.r[11].s64 + -9336;
	// 8240C41C: 4BEA6B65  bl 0x822b2f80
	ctx.lr = 0x8240C420;
	sub_822B2F80(ctx, base);
	// 8240C420: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240C424: 3C808000  lis r4, -0x8000
	ctx.r[4].s64 = -2147483648;
	// 8240C428: 386BDF88  addi r3, r11, -0x2078
	ctx.r[3].s64 = ctx.r[11].s64 + -8312;
	// 8240C42C: 6084001E  ori r4, r4, 0x1e
	ctx.r[4].u64 = ctx.r[4].u64 | 30;
	// 8240C430: 4BEA6B51  bl 0x822b2f80
	ctx.lr = 0x8240C434;
	sub_822B2F80(ctx, base);
	// 8240C434: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 8240C438: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 8240C43C: 6063001E  ori r3, r3, 0x1e
	ctx.r[3].u64 = ctx.r[3].u64 | 30;
	// 8240C440: 48000028  b 0x8240c468
	pc = 0x8240C468; continue 'dispatch;
            }
            0x8240C444 => {
    //   block [0x8240C444..0x8240C460)
	// 8240C444: 39640002  addi r11, r4, 2
	ctx.r[11].s64 = ctx.r[4].s64 + 2;
	// 8240C448: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8240C44C: 7D4B182E  lwzx r10, r11, r3
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[3].u32)) } as u64;
	// 8240C450: 2F0A0001  cmpwi cr6, r10, 1
	ctx.cr[6].compare_i32(ctx.r[10].s32, 1, &mut ctx.xer);
	// 8240C454: 409A000C  bne cr6, 0x8240c460
	if !ctx.cr[6].eq {
	pc = 0x8240C460; continue 'dispatch;
	}
	// 8240C458: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 8240C45C: 7D4B192E  stwx r10, r11, r3
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[3].u32), ctx.r[10].u32) };
	pc = 0x8240C460; continue 'dispatch;
            }
            0x8240C460 => {
    //   block [0x8240C460..0x8240C468)
	// 8240C460: 7D6B182E  lwzx r11, r11, r3
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[3].u32)) } as u64;
	// 8240C464: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	pc = 0x8240C468; continue 'dispatch;
            }
            0x8240C468 => {
    //   block [0x8240C468..0x8240C46C)
	// 8240C468: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	pc = 0x8240C46C; continue 'dispatch;
            }
            0x8240C46C => {
    //   block [0x8240C46C..0x8240C480)
	// 8240C46C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8240C470: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240C474: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240C478: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8240C47C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240C480(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8240C480 size=180
    let mut pc: u32 = 0x8240C480;
    'dispatch: loop {
        match pc {
            0x8240C480 => {
    //   block [0x8240C480..0x8240C4B4)
	// 8240C480: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240C484: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240C488: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8240C48C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240C490: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8240C494: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 8240C498: 409A001C  bne cr6, 0x8240c4b4
	if !ctx.cr[6].eq {
	pc = 0x8240C4B4; continue 'dispatch;
	}
	// 8240C49C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240C4A0: 386BE0C0  addi r3, r11, -0x1f40
	ctx.r[3].s64 = ctx.r[11].s64 + -8000;
	// 8240C4A4: 4BEA6ADD  bl 0x822b2f80
	ctx.lr = 0x8240C4A8;
	sub_822B2F80(ctx, base);
	// 8240C4A8: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 8240C4AC: 60630024  ori r3, r3, 0x24
	ctx.r[3].u64 = ctx.r[3].u64 | 36;
	// 8240C4B0: 48000070  b 0x8240c520
	pc = 0x8240C520; continue 'dispatch;
            }
            0x8240C4B4 => {
    //   block [0x8240C4B4..0x8240C4E8)
	// 8240C4B4: 2B1F003F  cmplwi cr6, r31, 0x3f
	ctx.cr[6].compare_u32(ctx.r[31].u32, 63 as u32, &mut ctx.xer);
	// 8240C4B8: 40990030  ble cr6, 0x8240c4e8
	if !ctx.cr[6].gt {
	pc = 0x8240C4E8; continue 'dispatch;
	}
	// 8240C4BC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240C4C0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8240C4C4: 386BDB88  addi r3, r11, -0x2478
	ctx.r[3].s64 = ctx.r[11].s64 + -9336;
	// 8240C4C8: 4BEA6AB9  bl 0x822b2f80
	ctx.lr = 0x8240C4CC;
	sub_822B2F80(ctx, base);
	// 8240C4CC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240C4D0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8240C4D4: 386BE070  addi r3, r11, -0x1f90
	ctx.r[3].s64 = ctx.r[11].s64 + -8080;
	// 8240C4D8: 4BEA6AA9  bl 0x822b2f80
	ctx.lr = 0x8240C4DC;
	sub_822B2F80(ctx, base);
	// 8240C4DC: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 8240C4E0: 6063001E  ori r3, r3, 0x1e
	ctx.r[3].u64 = ctx.r[3].u64 | 30;
	// 8240C4E4: 4800003C  b 0x8240c520
	pc = 0x8240C520; continue 'dispatch;
            }
            0x8240C4E8 => {
    //   block [0x8240C4E8..0x8240C518)
	// 8240C4E8: 1D7F2008  mulli r11, r31, 0x2008
	ctx.r[11].s32 = ((ctx.r[31].s32 as i64 * 8200 as i64) as i32);
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 8240C4EC: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 8240C4F0: 816B0108  lwz r11, 0x108(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(264 as u32) ) } as u64;
	// 8240C4F4: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8240C4F8: 40820020  bne 0x8240c518
	if !ctx.cr[0].eq {
	pc = 0x8240C518; continue 'dispatch;
	}
	// 8240C4FC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240C500: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8240C504: 386BE018  addi r3, r11, -0x1fe8
	ctx.r[3].s64 = ctx.r[11].s64 + -8168;
	// 8240C508: 4BEA6A79  bl 0x822b2f80
	ctx.lr = 0x8240C50C;
	sub_822B2F80(ctx, base);
	// 8240C50C: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 8240C510: 60630020  ori r3, r3, 0x20
	ctx.r[3].u64 = ctx.r[3].u64 | 32;
	// 8240C514: 4800000C  b 0x8240c520
	pc = 0x8240C520; continue 'dispatch;
            }
            0x8240C518 => {
    //   block [0x8240C518..0x8240C520)
	// 8240C518: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8240C51C: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	pc = 0x8240C520; continue 'dispatch;
            }
            0x8240C520 => {
    //   block [0x8240C520..0x8240C534)
	// 8240C520: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8240C524: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240C528: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240C52C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8240C530: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240C538(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8240C538 size=136
    let mut pc: u32 = 0x8240C538;
    'dispatch: loop {
        match pc {
            0x8240C538 => {
    //   block [0x8240C538..0x8240C574)
	// 8240C538: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240C53C: 48128B81  bl 0x825350bc
	ctx.lr = 0x8240C540;
	sub_82535080(ctx, base);
	// 8240C540: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240C544: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 8240C548: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8240C54C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8240C550: 4BFFFF31  bl 0x8240c480
	ctx.lr = 0x8240C554;
	sub_8240C480(ctx, base);
	// 8240C554: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 8240C558: 4182001C  beq 0x8240c574
	if ctx.cr[0].eq {
	pc = 0x8240C574; continue 'dispatch;
	}
	// 8240C55C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240C560: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8240C564: 386BE168  addi r3, r11, -0x1e98
	ctx.r[3].s64 = ctx.r[11].s64 + -7832;
	// 8240C568: 4BEA6A19  bl 0x822b2f80
	ctx.lr = 0x8240C56C;
	sub_822B2F80(ctx, base);
	// 8240C56C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8240C570: 48000048  b 0x8240c5b8
	pc = 0x8240C5B8; continue 'dispatch;
            }
            0x8240C574 => {
    //   block [0x8240C574..0x8240C5A0)
	// 8240C574: 3C7D0008  addis r3, r29, 8
	ctx.r[3].s64 = ctx.r[29].s64 + 524288;
	// 8240C578: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8240C57C: 3863030C  addi r3, r3, 0x30c
	ctx.r[3].s64 = ctx.r[3].s64 + 780;
	// 8240C580: 4BFFA4F1  bl 0x82406a70
	ctx.lr = 0x8240C584;
	sub_82406A70(ctx, base);
	// 8240C584: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 8240C588: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8240C58C: 41980014  blt cr6, 0x8240c5a0
	if ctx.cr[6].lt {
	pc = 0x8240C5A0; continue 'dispatch;
	}
	// 8240C590: 7F1E2800  cmpw cr6, r30, r5
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[5].s32, &mut ctx.xer);
	// 8240C594: 4098000C  bge cr6, 0x8240c5a0
	if !ctx.cr[6].lt {
	pc = 0x8240C5A0; continue 'dispatch;
	}
	// 8240C598: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8240C59C: 4800001C  b 0x8240c5b8
	pc = 0x8240C5B8; continue 'dispatch;
            }
            0x8240C5A0 => {
    //   block [0x8240C5A0..0x8240C5B8)
	// 8240C5A0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240C5A4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8240C5A8: 386BE110  addi r3, r11, -0x1ef0
	ctx.r[3].s64 = ctx.r[11].s64 + -7920;
	// 8240C5AC: 4BEA69D5  bl 0x822b2f80
	ctx.lr = 0x8240C5B0;
	sub_822B2F80(ctx, base);
	// 8240C5B0: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 8240C5B4: 6063000F  ori r3, r3, 0xf
	ctx.r[3].u64 = ctx.r[3].u64 | 15;
	pc = 0x8240C5B8; continue 'dispatch;
            }
            0x8240C5B8 => {
    //   block [0x8240C5B8..0x8240C5C0)
	// 8240C5B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8240C5BC: 48128B50  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240C5C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8240C5C0 size=180
    let mut pc: u32 = 0x8240C5C0;
    'dispatch: loop {
        match pc {
            0x8240C5C0 => {
    //   block [0x8240C5C0..0x8240C5E0)
	// 8240C5C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240C5C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240C5C8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8240C5CC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8240C5D0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240C5D4: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 8240C5D8: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8240C5DC: 39630108  addi r11, r3, 0x108
	ctx.r[11].s64 = ctx.r[3].s64 + 264;
	pc = 0x8240C5E0; continue 'dispatch;
            }
            0x8240C5E0 => {
    //   block [0x8240C5E0..0x8240C600)
	// 8240C5E0: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8240C5E4: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8240C5E8: 419A0018  beq cr6, 0x8240c600
	if ctx.cr[6].eq {
	pc = 0x8240C600; continue 'dispatch;
	}
	// 8240C5EC: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 8240C5F0: 396B2008  addi r11, r11, 0x2008
	ctx.r[11].s64 = ctx.r[11].s64 + 8200;
	// 8240C5F4: 2F1F0040  cmpwi cr6, r31, 0x40
	ctx.cr[6].compare_i32(ctx.r[31].s32, 64, &mut ctx.xer);
	// 8240C5F8: 4198FFE8  blt cr6, 0x8240c5e0
	if ctx.cr[6].lt {
	pc = 0x8240C5E0; continue 'dispatch;
	}
	// 8240C5FC: 4800000C  b 0x8240c608
	pc = 0x8240C608; continue 'dispatch;
            }
            0x8240C600 => {
    //   block [0x8240C600..0x8240C608)
	// 8240C600: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 8240C604: 40980020  bge cr6, 0x8240c624
	if !ctx.cr[6].lt {
	pc = 0x8240C624; continue 'dispatch;
	}
	pc = 0x8240C608; continue 'dispatch;
            }
            0x8240C608 => {
    //   block [0x8240C608..0x8240C624)
	// 8240C608: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240C60C: 38800040  li r4, 0x40
	ctx.r[4].s64 = 64;
	// 8240C610: 386BE1E8  addi r3, r11, -0x1e18
	ctx.r[3].s64 = ctx.r[11].s64 + -7704;
	// 8240C614: 4BEA696D  bl 0x822b2f80
	ctx.lr = 0x8240C618;
	sub_822B2F80(ctx, base);
	// 8240C618: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 8240C61C: 6063001B  ori r3, r3, 0x1b
	ctx.r[3].u64 = ctx.r[3].u64 | 27;
	// 8240C620: 4800003C  b 0x8240c65c
	pc = 0x8240C65C; continue 'dispatch;
            }
            0x8240C624 => {
    //   block [0x8240C624..0x8240C648)
	// 8240C624: 7CA62B78  mr r6, r5
	ctx.r[6].u64 = ctx.r[5].u64;
	// 8240C628: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 8240C62C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8240C630: 4BFFF8E1  bl 0x8240bf10
	ctx.lr = 0x8240C634;
	sub_8240BF10(ctx, base);
	// 8240C634: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8240C638: 40820024  bne 0x8240c65c
	if !ctx.cr[0].eq {
	pc = 0x8240C65C; continue 'dispatch;
	}
	// 8240C63C: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8240C640: 419A0008  beq cr6, 0x8240c648
	if ctx.cr[6].eq {
	pc = 0x8240C648; continue 'dispatch;
	}
	// 8240C644: 93FE0000  stw r31, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	pc = 0x8240C648; continue 'dispatch;
            }
            0x8240C648 => {
    //   block [0x8240C648..0x8240C65C)
	// 8240C648: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240C64C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8240C650: 386BE1B0  addi r3, r11, -0x1e50
	ctx.r[3].s64 = ctx.r[11].s64 + -7760;
	// 8240C654: 4BEA692D  bl 0x822b2f80
	ctx.lr = 0x8240C658;
	sub_822B2F80(ctx, base);
	// 8240C658: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	pc = 0x8240C65C; continue 'dispatch;
            }
            0x8240C65C => {
    //   block [0x8240C65C..0x8240C674)
	// 8240C65C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8240C660: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240C664: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240C668: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8240C66C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8240C670: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240C678(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8240C678 size=60
    let mut pc: u32 = 0x8240C678;
    'dispatch: loop {
        match pc {
            0x8240C678 => {
    //   block [0x8240C678..0x8240C6B4)
	// 8240C678: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240C67C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240C680: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8240C684: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240C688: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8240C68C: 4BFFFCB5  bl 0x8240c340
	ctx.lr = 0x8240C690;
	sub_8240C340(ctx, base);
	// 8240C690: 3D600008  lis r11, 8
	ctx.r[11].s64 = 524288;
	// 8240C694: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8240C698: 616B0308  ori r11, r11, 0x308
	ctx.r[11].u64 = ctx.r[11].u64 | 776;
	// 8240C69C: 7D5F592E  stwx r10, r31, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32), ctx.r[10].u32) };
	// 8240C6A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8240C6A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240C6A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240C6AC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8240C6B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240C6B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8240C6B8 size=64
    let mut pc: u32 = 0x8240C6B8;
    'dispatch: loop {
        match pc {
            0x8240C6B8 => {
    //   block [0x8240C6B8..0x8240C6E4)
	// 8240C6B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240C6BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240C6C0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240C6C4: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 8240C6C8: 409A001C  bne cr6, 0x8240c6e4
	if !ctx.cr[6].eq {
	pc = 0x8240C6E4; continue 'dispatch;
	}
	// 8240C6CC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240C6D0: 386BE238  addi r3, r11, -0x1dc8
	ctx.r[3].s64 = ctx.r[11].s64 + -7624;
	// 8240C6D4: 4BEA68AD  bl 0x822b2f80
	ctx.lr = 0x8240C6D8;
	sub_822B2F80(ctx, base);
	// 8240C6D8: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 8240C6DC: 60630025  ori r3, r3, 0x25
	ctx.r[3].u64 = ctx.r[3].u64 | 37;
	// 8240C6E0: 48000008  b 0x8240c6e8
	pc = 0x8240C6E8; continue 'dispatch;
            }
            0x8240C6E4 => {
    //   block [0x8240C6E4..0x8240C6E8)
	// 8240C6E4: 4BFFFEDD  bl 0x8240c5c0
	ctx.lr = 0x8240C6E8;
	sub_8240C5C0(ctx, base);
	pc = 0x8240C6E8; continue 'dispatch;
            }
            0x8240C6E8 => {
    //   block [0x8240C6E8..0x8240C6F8)
	// 8240C6E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8240C6EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240C6F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240C6F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240C6F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8240C6F8 size=136
    let mut pc: u32 = 0x8240C6F8;
    'dispatch: loop {
        match pc {
            0x8240C6F8 => {
    //   block [0x8240C6F8..0x8240C72C)
	// 8240C6F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240C6FC: 481289C1  bl 0x825350bc
	ctx.lr = 0x8240C700;
	sub_82535080(ctx, base);
	// 8240C700: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240C704: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 8240C708: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8240C70C: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8240C710: 409A001C  bne cr6, 0x8240c72c
	if !ctx.cr[6].eq {
	pc = 0x8240C72C; continue 'dispatch;
	}
	// 8240C714: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240C718: 386BE2C8  addi r3, r11, -0x1d38
	ctx.r[3].s64 = ctx.r[11].s64 + -7480;
	// 8240C71C: 4BEA6865  bl 0x822b2f80
	ctx.lr = 0x8240C720;
	sub_822B2F80(ctx, base);
	// 8240C720: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 8240C724: 60630023  ori r3, r3, 0x23
	ctx.r[3].u64 = ctx.r[3].u64 | 35;
	// 8240C728: 48000050  b 0x8240c778
	pc = 0x8240C778; continue 'dispatch;
            }
            0x8240C72C => {
    //   block [0x8240C72C..0x8240C75C)
	// 8240C72C: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8240C730: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8240C734: 4BFFFD4D  bl 0x8240c480
	ctx.lr = 0x8240C738;
	sub_8240C480(ctx, base);
	// 8240C738: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 8240C73C: 41820020  beq 0x8240c75c
	if ctx.cr[0].eq {
	pc = 0x8240C75C; continue 'dispatch;
	}
	// 8240C740: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240C744: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8240C748: 386BE280  addi r3, r11, -0x1d80
	ctx.r[3].s64 = ctx.r[11].s64 + -7552;
	// 8240C74C: 4BEA6835  bl 0x822b2f80
	ctx.lr = 0x8240C750;
	sub_822B2F80(ctx, base);
	// 8240C750: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 8240C754: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8240C758: 4800001C  b 0x8240c774
	pc = 0x8240C774; continue 'dispatch;
            }
            0x8240C75C => {
    //   block [0x8240C75C..0x8240C774)
	// 8240C75C: 3C7D0008  addis r3, r29, 8
	ctx.r[3].s64 = ctx.r[29].s64 + 524288;
	// 8240C760: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8240C764: 3863030C  addi r3, r3, 0x30c
	ctx.r[3].s64 = ctx.r[3].s64 + 780;
	// 8240C768: 4BFFA2C9  bl 0x82406a30
	ctx.lr = 0x8240C76C;
	sub_82406A30(ctx, base);
	// 8240C76C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8240C770: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	pc = 0x8240C774; continue 'dispatch;
            }
            0x8240C774 => {
    //   block [0x8240C774..0x8240C778)
	// 8240C774: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	pc = 0x8240C778; continue 'dispatch;
            }
            0x8240C778 => {
    //   block [0x8240C778..0x8240C780)
	// 8240C778: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8240C77C: 48128990  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240C780(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8240C780 size=88
    let mut pc: u32 = 0x8240C780;
    'dispatch: loop {
        match pc {
            0x8240C780 => {
    //   block [0x8240C780..0x8240C7BC)
	// 8240C780: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240C784: 48128935  bl 0x825350b8
	ctx.lr = 0x8240C788;
	sub_82535080(ctx, base);
	// 8240C788: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240C78C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8240C790: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8240C794: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 8240C798: 4BFFFDA1  bl 0x8240c538
	ctx.lr = 0x8240C79C;
	sub_8240C538(ctx, base);
	// 8240C79C: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 8240C7A0: 4182001C  beq 0x8240c7bc
	if ctx.cr[0].eq {
	pc = 0x8240C7BC; continue 'dispatch;
	}
	// 8240C7A4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240C7A8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8240C7AC: 386BE310  addi r3, r11, -0x1cf0
	ctx.r[3].s64 = ctx.r[11].s64 + -7408;
	// 8240C7B0: 4BEA67D1  bl 0x822b2f80
	ctx.lr = 0x8240C7B4;
	sub_822B2F80(ctx, base);
	// 8240C7B4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8240C7B8: 48000018  b 0x8240c7d0
	pc = 0x8240C7D0; continue 'dispatch;
            }
            0x8240C7BC => {
    //   block [0x8240C7BC..0x8240C7D0)
	// 8240C7BC: 1D7D0802  mulli r11, r29, 0x802
	ctx.r[11].s32 = ((ctx.r[29].s32 as i64 * 2050 as i64) as i32);
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 8240C7C0: 7D6BE214  add r11, r11, r28
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[28].u64;
	// 8240C7C4: 396B0044  addi r11, r11, 0x44
	ctx.r[11].s64 = ctx.r[11].s64 + 68;
	// 8240C7C8: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8240C7CC: 7C6BF02E  lwzx r3, r11, r30
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	pc = 0x8240C7D0; continue 'dispatch;
            }
            0x8240C7D0 => {
    //   block [0x8240C7D0..0x8240C7D8)
	// 8240C7D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8240C7D4: 48128934  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240C7D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8240C7D8 size=64
    let mut pc: u32 = 0x8240C7D8;
    'dispatch: loop {
        match pc {
            0x8240C7D8 => {
    //   block [0x8240C7D8..0x8240C818)
	// 8240C7D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240C7DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240C7E0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8240C7E4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240C7E8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8240C7EC: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 8240C7F0: 48003411  bl 0x8240fc00
	ctx.lr = 0x8240C7F4;
	sub_8240FC00(ctx, base);
	// 8240C7F4: 3C7F0002  addis r3, r31, 2
	ctx.r[3].s64 = ctx.r[31].s64 + 131072;
	// 8240C7F8: 3863000C  addi r3, r3, 0xc
	ctx.r[3].s64 = ctx.r[3].s64 + 12;
	// 8240C7FC: 4BFFE6D5  bl 0x8240aed0
	ctx.lr = 0x8240C800;
	sub_8240AED0(ctx, base);
	// 8240C800: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8240C804: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8240C808: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240C80C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240C810: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8240C814: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240C818(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8240C818 size=60
    let mut pc: u32 = 0x8240C818;
    'dispatch: loop {
        match pc {
            0x8240C818 => {
    //   block [0x8240C818..0x8240C854)
	// 8240C818: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240C81C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240C820: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8240C824: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240C828: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8240C82C: 3C7F0002  addis r3, r31, 2
	ctx.r[3].s64 = ctx.r[31].s64 + 131072;
	// 8240C830: 3863000C  addi r3, r3, 0xc
	ctx.r[3].s64 = ctx.r[3].s64 + 12;
	// 8240C834: 4BFFE57D  bl 0x8240adb0
	ctx.lr = 0x8240C838;
	sub_8240ADB0(ctx, base);
	// 8240C838: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 8240C83C: 4800321D  bl 0x8240fa58
	ctx.lr = 0x8240C840;
	sub_8240FA58(ctx, base);
	// 8240C840: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8240C844: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240C848: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240C84C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8240C850: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240C858(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8240C858 size=88
    let mut pc: u32 = 0x8240C858;
    'dispatch: loop {
        match pc {
            0x8240C858 => {
    //   block [0x8240C858..0x8240C8B0)
	// 8240C858: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240C85C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240C860: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8240C864: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240C868: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 8240C86C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8240C870: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8240C874: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 8240C878: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 8240C87C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8240C880: 48003229  bl 0x8240faa8
	ctx.lr = 0x8240C884;
	sub_8240FAA8(ctx, base);
	// 8240C884: 3C7F0002  addis r3, r31, 2
	ctx.r[3].s64 = ctx.r[31].s64 + 131072;
	// 8240C888: 93E10054  stw r31, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[31].u32 ) };
	// 8240C88C: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 8240C890: 3863000C  addi r3, r3, 0xc
	ctx.r[3].s64 = ctx.r[3].s64 + 12;
	// 8240C894: 4BFFE565  bl 0x8240adf8
	ctx.lr = 0x8240C898;
	sub_8240ADF8(ctx, base);
	// 8240C898: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8240C89C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8240C8A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240C8A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240C8A8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8240C8AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240C8B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8240C8B0 size=60
    let mut pc: u32 = 0x8240C8B0;
    'dispatch: loop {
        match pc {
            0x8240C8B0 => {
    //   block [0x8240C8B0..0x8240C8EC)
	// 8240C8B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240C8B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240C8B8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8240C8BC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240C8C0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8240C8C4: 3C7F0002  addis r3, r31, 2
	ctx.r[3].s64 = ctx.r[31].s64 + 131072;
	// 8240C8C8: 3863000C  addi r3, r3, 0xc
	ctx.r[3].s64 = ctx.r[3].s64 + 12;
	// 8240C8CC: 4BEA66B5  bl 0x822b2f80
	ctx.lr = 0x8240C8D0;
	sub_822B2F80(ctx, base);
	// 8240C8D0: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 8240C8D4: 4BEA66AD  bl 0x822b2f80
	ctx.lr = 0x8240C8D8;
	sub_822B2F80(ctx, base);
	// 8240C8D8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8240C8DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240C8E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240C8E4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8240C8E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240C8F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8240C8F0 size=12
    let mut pc: u32 = 0x8240C8F0;
    'dispatch: loop {
        match pc {
            0x8240C8F0 => {
    //   block [0x8240C8F0..0x8240C8FC)
	// 8240C8F0: 3C630002  addis r3, r3, 2
	ctx.r[3].s64 = ctx.r[3].s64 + 131072;
	// 8240C8F4: 3863000C  addi r3, r3, 0xc
	ctx.r[3].s64 = ctx.r[3].s64 + 12;
	// 8240C8F8: 4BFFE578  b 0x8240ae70
	sub_8240AE70(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240C900(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8240C900 size=228
    let mut pc: u32 = 0x8240C900;
    'dispatch: loop {
        match pc {
            0x8240C900 => {
    //   block [0x8240C900..0x8240C9AC)
	// 8240C900: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240C904: 481287B5  bl 0x825350b8
	ctx.lr = 0x8240C908;
	sub_82535080(ctx, base);
	// 8240C908: 3981FFD8  addi r12, r1, -0x28
	ctx.r[12].s64 = ctx.r[1].s64 + -40;
	// 8240C90C: 481296DD  bl 0x82535fe8
	ctx.lr = 0x8240C910;
	sub_82535FB0(ctx, base);
	// 8240C910: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240C914: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8240C918: FFE01090  fmr f31, f2
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].f64 = ctx.f[2].f64;
	// 8240C91C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8240C920: FFC01890  fmr f30, f3
	ctx.f[30].f64 = ctx.f[3].f64;
	// 8240C924: FFA02090  fmr f29, f4
	ctx.f[29].f64 = ctx.f[4].f64;
	// 8240C928: 2B1D0009  cmplwi cr6, r29, 9
	ctx.cr[6].compare_u32(ctx.r[29].u32, 9 as u32, &mut ctx.xer);
	// 8240C92C: FF802890  fmr f28, f5
	ctx.f[28].f64 = ctx.f[5].f64;
	// 8240C930: 4199009C  bgt cr6, 0x8240c9cc
	if ctx.cr[6].gt {
	pc = 0x8240C9CC; continue 'dispatch;
	}
	// 8240C934: 397D0003  addi r11, r29, 3
	ctx.r[11].s64 = ctx.r[29].s64 + 3;
	// 8240C938: 1F8B0018  mulli r28, r11, 0x18
	ctx.r[28].s32 = ((ctx.r[11].s32 as i64 * 24 as i64) as i32);
	ctx.r[28].s64 = ctx.r[28].s32 as i64;
	// 8240C93C: 480034E5  bl 0x8240fe20
	ctx.lr = 0x8240C940;
	sub_8240FE20(ctx, base);
	// 8240C940: 7C3CF52E  stfsx f1, r28, r30
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[28].u32.wrapping_add(ctx.r[30].u32), tmp.u32) };
	// 8240C944: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 8240C948: 1D7D0018  mulli r11, r29, 0x18
	ctx.r[11].s32 = ((ctx.r[29].s32 as i64 * 24 as i64) as i32);
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 8240C94C: 7FEBF214  add r31, r11, r30
	ctx.r[31].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 8240C950: 480034D1  bl 0x8240fe20
	ctx.lr = 0x8240C954;
	sub_8240FE20(ctx, base);
	// 8240C954: D03F004C  stfs f1, 0x4c(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(76 as u32), tmp.u32 ) };
	// 8240C958: FC20F090  fmr f1, f30
	ctx.f[1].f64 = ctx.f[30].f64;
	// 8240C95C: 480034C5  bl 0x8240fe20
	ctx.lr = 0x8240C960;
	sub_8240FE20(ctx, base);
	// 8240C960: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8240C964: D03F0050  stfs f1, 0x50(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 8240C968: C02B1FF8  lfs f1, 0x1ff8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8240C96C: 480034B5  bl 0x8240fe20
	ctx.lr = 0x8240C970;
	sub_8240FE20(ctx, base);
	// 8240C970: D03F0054  stfs f1, 0x54(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 8240C974: FC20E890  fmr f1, f29
	ctx.f[1].f64 = ctx.f[29].f64;
	// 8240C978: 480034A9  bl 0x8240fe20
	ctx.lr = 0x8240C97C;
	sub_8240FE20(ctx, base);
	// 8240C97C: D03F0058  stfs f1, 0x58(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), tmp.u32 ) };
	// 8240C980: FC20E090  fmr f1, f28
	ctx.f[1].f64 = ctx.f[28].f64;
	// 8240C984: 4800349D  bl 0x8240fe20
	ctx.lr = 0x8240C988;
	sub_8240FE20(ctx, base);
	// 8240C988: 397D000D  addi r11, r29, 0xd
	ctx.r[11].s64 = ctx.r[29].s64 + 13;
	// 8240C98C: 815F0148  lwz r10, 0x148(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(328 as u32) ) } as u64;
	// 8240C990: D03F005C  stfs f1, 0x5c(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), tmp.u32 ) };
	// 8240C994: 1D6B0018  mulli r11, r11, 0x18
	ctx.r[11].s32 = ((ctx.r[11].s32 as i64 * 24 as i64) as i32);
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 8240C998: 7D6BF02E  lwzx r11, r11, r30
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 8240C99C: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 8240C9A0: 409A000C  bne cr6, 0x8240c9ac
	if !ctx.cr[6].eq {
	pc = 0x8240C9AC; continue 'dispatch;
	}
	// 8240C9A4: 7C1CF42E  lfsx f0, r28, r30
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[28].u32.wrapping_add(ctx.r[30].u32)) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240C9A8: D01F0058  stfs f0, 0x58(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), tmp.u32 ) };
	pc = 0x8240C9AC; continue 'dispatch;
            }
            0x8240C9AC => {
    //   block [0x8240C9AC..0x8240C9C4)
	// 8240C9AC: 817F013C  lwz r11, 0x13c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(316 as u32) ) } as u64;
	// 8240C9B0: 815F014C  lwz r10, 0x14c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(332 as u32) ) } as u64;
	// 8240C9B4: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 8240C9B8: 409A000C  bne cr6, 0x8240c9c4
	if !ctx.cr[6].eq {
	pc = 0x8240C9C4; continue 'dispatch;
	}
	// 8240C9BC: C01F004C  lfs f0, 0x4c(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240C9C0: D01F005C  stfs f0, 0x5c(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), tmp.u32 ) };
	pc = 0x8240C9C4; continue 'dispatch;
            }
            0x8240C9C4 => {
    //   block [0x8240C9C4..0x8240C9CC)
	// 8240C9C4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8240C9C8: 4800000C  b 0x8240c9d4
	pc = 0x8240C9D4; continue 'dispatch;
            }
            0x8240C9CC => {
    //   block [0x8240C9CC..0x8240C9D4)
	// 8240C9CC: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 8240C9D0: 6063002B  ori r3, r3, 0x2b
	ctx.r[3].u64 = ctx.r[3].u64 | 43;
	pc = 0x8240C9D4; continue 'dispatch;
            }
            0x8240C9D4 => {
    //   block [0x8240C9D4..0x8240C9E4)
	// 8240C9D4: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 8240C9D8: 3981FFD8  addi r12, r1, -0x28
	ctx.r[12].s64 = ctx.r[1].s64 + -40;
	// 8240C9DC: 48129659  bl 0x82536034
	ctx.lr = 0x8240C9E0;
	sub_82535FFC(ctx, base);
	// 8240C9E0: 48128728  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240C9E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8240C9E8 size=60
    let mut pc: u32 = 0x8240C9E8;
    'dispatch: loop {
        match pc {
            0x8240C9E8 => {
    //   block [0x8240C9E8..0x8240CA24)
	// 8240C9E8: 2B040009  cmplwi cr6, r4, 9
	ctx.cr[6].compare_u32(ctx.r[4].u32, 9 as u32, &mut ctx.xer);
	// 8240C9EC: 41990038  bgt cr6, 0x8240ca24
	if ctx.cr[6].gt {
		sub_8240CA24(ctx, base);
		return;
	}
	// 8240C9F0: 3944000D  addi r10, r4, 0xd
	ctx.r[10].s64 = ctx.r[4].s64 + 13;
	// 8240C9F4: 1D640018  mulli r11, r4, 0x18
	ctx.r[11].s32 = ((ctx.r[4].s32 as i64 * 24 as i64) as i32);
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 8240C9F8: 1D4A0018  mulli r10, r10, 0x18
	ctx.r[10].s32 = ((ctx.r[10].s32 as i64 * 24 as i64) as i32);
	ctx.r[10].s64 = ctx.r[10].s32 as i64;
	// 8240C9FC: 7CAA192E  stwx r5, r10, r3
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[3].u32), ctx.r[5].u32) };
	// 8240CA00: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 8240CA04: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8240CA08: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8240CA0C: 90CB013C  stw r6, 0x13c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(316 as u32), ctx.r[6].u32 ) };
	// 8240CA10: 90EB0140  stw r7, 0x140(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(320 as u32), ctx.r[7].u32 ) };
	// 8240CA14: 914B0144  stw r10, 0x144(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(324 as u32), ctx.r[10].u32 ) };
	// 8240CA18: 910B0148  stw r8, 0x148(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(328 as u32), ctx.r[8].u32 ) };
	// 8240CA1C: 912B014C  stw r9, 0x14c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(332 as u32), ctx.r[9].u32 ) };
	// 8240CA20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240CA24(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8240CA24 size=12
    let mut pc: u32 = 0x8240CA24;
    'dispatch: loop {
        match pc {
            0x8240CA24 => {
    //   block [0x8240CA24..0x8240CA30)
	// 8240CA24: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 8240CA28: 6063002B  ori r3, r3, 0x2b
	ctx.r[3].u64 = ctx.r[3].u64 | 43;
	// 8240CA2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240CA30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8240CA30 size=232
    let mut pc: u32 = 0x8240CA30;
    'dispatch: loop {
        match pc {
            0x8240CA30 => {
    //   block [0x8240CA30..0x8240CB18)
	// 8240CA30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240CA34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240CA38: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8240CA3C: DBE1FFE8  stfd f31, -0x18(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.f[31].u64 ) };
	// 8240CA40: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240CA44: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8240CA48: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8240CA4C: C3EBBFFC  lfs f31, -0x4004(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16388 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 8240CA50: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8240CA54: C1BF0004  lfs f13, 4(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8240CA58: C01F0000  lfs f0, 0(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240CA5C: C19F0014  lfs f12, 0x14(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 8240CA60: D1BF0040  stfs f13, 0x40(r31)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), tmp.u32 ) };
	// 8240CA64: C14B210C  lfs f10, 0x210c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8460 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 8240CA68: ED2D502A  fadds f9, f13, f10
	ctx.f[9].f64 = ((ctx.f[13].f64 + ctx.f[10].f64) as f32) as f64;
	// 8240CA6C: C17F0010  lfs f11, 0x10(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 8240CA70: ED4A0028  fsubs f10, f10, f0
	ctx.f[10].f64 = (((ctx.f[10].f64 - ctx.f[0].f64) as f32) as f64);
	// 8240CA74: D15F0038  stfs f10, 0x38(r31)
	tmp.f32 = (ctx.f[10].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), tmp.u32 ) };
	// 8240CA78: ED4C6828  fsubs f10, f12, f13
	ctx.f[10].f64 = (((ctx.f[12].f64 - ctx.f[13].f64) as f32) as f64);
	// 8240CA7C: D15F0020  stfs f10, 0x20(r31)
	tmp.f32 = (ctx.f[10].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), tmp.u32 ) };
	// 8240CA80: ED8B6028  fsubs f12, f11, f12
	ctx.f[12].f64 = (((ctx.f[11].f64 - ctx.f[12].f64) as f32) as f64);
	// 8240CA84: D19F0028  stfs f12, 0x28(r31)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), tmp.u32 ) };
	// 8240CA88: ED805828  fsubs f12, f0, f11
	ctx.f[12].f64 = (((ctx.f[0].f64 - ctx.f[11].f64) as f32) as f64);
	// 8240CA8C: D19F0030  stfs f12, 0x30(r31)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), tmp.u32 ) };
	// 8240CA90: EDA90028  fsubs f13, f9, f0
	ctx.f[13].f64 = (((ctx.f[9].f64 - ctx.f[0].f64) as f32) as f64);
	// 8240CA94: D1BF0018  stfs f13, 0x18(r31)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 8240CA98: EC2D07FA  fmadds f1, f13, f31, f0
	ctx.f[1].f64 = (((ctx.f[13].f64 * ctx.f[31].f64 + ctx.f[0].f64) as f32) as f64);
	// 8240CA9C: 48003385  bl 0x8240fe20
	ctx.lr = 0x8240CAA0;
	sub_8240FE20(ctx, base);
	// 8240CAA0: C01F0020  lfs f0, 0x20(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240CAA4: C1BF0004  lfs f13, 4(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8240CAA8: D03F001C  stfs f1, 0x1c(r31)
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 8240CAAC: EC206FFA  fmadds f1, f0, f31, f13
	ctx.f[1].f64 = (((ctx.f[0].f64 * ctx.f[31].f64 + ctx.f[13].f64) as f32) as f64);
	// 8240CAB0: 48003371  bl 0x8240fe20
	ctx.lr = 0x8240CAB4;
	sub_8240FE20(ctx, base);
	// 8240CAB4: C01F0028  lfs f0, 0x28(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240CAB8: C1BF0014  lfs f13, 0x14(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8240CABC: D03F0024  stfs f1, 0x24(r31)
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), tmp.u32 ) };
	// 8240CAC0: EC206FFA  fmadds f1, f0, f31, f13
	ctx.f[1].f64 = (((ctx.f[0].f64 * ctx.f[31].f64 + ctx.f[13].f64) as f32) as f64);
	// 8240CAC4: 4800335D  bl 0x8240fe20
	ctx.lr = 0x8240CAC8;
	sub_8240FE20(ctx, base);
	// 8240CAC8: C01F0030  lfs f0, 0x30(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240CACC: C1BF0010  lfs f13, 0x10(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8240CAD0: D03F002C  stfs f1, 0x2c(r31)
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), tmp.u32 ) };
	// 8240CAD4: EC206FFA  fmadds f1, f0, f31, f13
	ctx.f[1].f64 = (((ctx.f[0].f64 * ctx.f[31].f64 + ctx.f[13].f64) as f32) as f64);
	// 8240CAD8: 48003349  bl 0x8240fe20
	ctx.lr = 0x8240CADC;
	sub_8240FE20(ctx, base);
	// 8240CADC: C01F0038  lfs f0, 0x38(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240CAE0: C1BF0000  lfs f13, 0(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8240CAE4: D03F0034  stfs f1, 0x34(r31)
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), tmp.u32 ) };
	// 8240CAE8: EC206FFA  fmadds f1, f0, f31, f13
	ctx.f[1].f64 = (((ctx.f[0].f64 * ctx.f[31].f64 + ctx.f[13].f64) as f32) as f64);
	// 8240CAEC: 48003335  bl 0x8240fe20
	ctx.lr = 0x8240CAF0;
	sub_8240FE20(ctx, base);
	// 8240CAF0: C01F0040  lfs f0, 0x40(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240CAF4: EC0007F2  fmuls f0, f0, f31
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[31].f64) as f32) as f64);
	// 8240CAF8: D03F003C  stfs f1, 0x3c(r31)
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), tmp.u32 ) };
	// 8240CAFC: D01F0044  stfs f0, 0x44(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(68 as u32), tmp.u32 ) };
	// 8240CB00: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8240CB04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240CB08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240CB0C: CBE1FFE8  lfd f31, -0x18(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8240CB10: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8240CB14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240CB18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8240CB18 size=116
    let mut pc: u32 = 0x8240CB18;
    'dispatch: loop {
        match pc {
            0x8240CB18 => {
    //   block [0x8240CB18..0x8240CB50)
	// 8240CB18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240CB1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240CB20: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8240CB24: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240CB28: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8240CB2C: 480032F5  bl 0x8240fe20
	ctx.lr = 0x8240CB30;
	sub_8240FE20(ctx, base);
	// 8240CB30: C01F0004  lfs f0, 4(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240CB34: FF000800  fcmpu cr6, f0, f1
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[1].f64);
	// 8240CB38: 40980024  bge cr6, 0x8240cb5c
	if !ctx.cr[6].lt {
	pc = 0x8240CB5C; continue 'dispatch;
	}
	// 8240CB3C: C01F0014  lfs f0, 0x14(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240CB40: FF000800  fcmpu cr6, f0, f1
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[1].f64);
	// 8240CB44: 4099000C  ble cr6, 0x8240cb50
	if !ctx.cr[6].gt {
	pc = 0x8240CB50; continue 'dispatch;
	}
	// 8240CB48: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8240CB4C: 4800002C  b 0x8240cb78
	pc = 0x8240CB78; continue 'dispatch;
            }
            0x8240CB50 => {
    //   block [0x8240CB50..0x8240CB5C)
	// 8240CB50: C01F0000  lfs f0, 0(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240CB54: FF000800  fcmpu cr6, f0, f1
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[1].f64);
	// 8240CB58: 4199000C  bgt cr6, 0x8240cb64
	if ctx.cr[6].gt {
	pc = 0x8240CB64; continue 'dispatch;
	}
	pc = 0x8240CB5C; continue 'dispatch;
            }
            0x8240CB5C => {
    //   block [0x8240CB5C..0x8240CB64)
	// 8240CB5C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8240CB60: 48000018  b 0x8240cb78
	pc = 0x8240CB78; continue 'dispatch;
            }
            0x8240CB64 => {
    //   block [0x8240CB64..0x8240CB78)
	// 8240CB64: C01F0010  lfs f0, 0x10(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240CB68: 38600003  li r3, 3
	ctx.r[3].s64 = 3;
	// 8240CB6C: FF000800  fcmpu cr6, f0, f1
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[1].f64);
	// 8240CB70: 41980008  blt cr6, 0x8240cb78
	if ctx.cr[6].lt {
	pc = 0x8240CB78; continue 'dispatch;
	}
	// 8240CB74: 38600002  li r3, 2
	ctx.r[3].s64 = 2;
	pc = 0x8240CB78; continue 'dispatch;
            }
            0x8240CB78 => {
    //   block [0x8240CB78..0x8240CB8C)
	// 8240CB78: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8240CB7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240CB80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240CB84: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8240CB88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240CB90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8240CB90 size=212
    let mut pc: u32 = 0x8240CB90;
    'dispatch: loop {
        match pc {
            0x8240CB90 => {
    //   block [0x8240CB90..0x8240CBCC)
	// 8240CB90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240CB94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240CB98: DBC1FFE8  stfd f30, -0x18(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.f[30].u64 ) };
	// 8240CB9C: DBE1FFF0  stfd f31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.f[31].u64 ) };
	// 8240CBA0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240CBA4: FC000890  fmr f0, f1
	ctx.f[0].f64 = ctx.f[1].f64;
	// 8240CBA8: 2B040001  cmplwi cr6, r4, 1
	ctx.cr[6].compare_u32(ctx.r[4].u32, 1 as u32, &mut ctx.xer);
	// 8240CBAC: 41980050  blt cr6, 0x8240cbfc
	if ctx.cr[6].lt {
	pc = 0x8240CBFC; continue 'dispatch;
	}
	// 8240CBB0: 419A003C  beq cr6, 0x8240cbec
	if ctx.cr[6].eq {
	pc = 0x8240CBEC; continue 'dispatch;
	}
	// 8240CBB4: 2B040003  cmplwi cr6, r4, 3
	ctx.cr[6].compare_u32(ctx.r[4].u32, 3 as u32, &mut ctx.xer);
	// 8240CBB8: 41980024  blt cr6, 0x8240cbdc
	if ctx.cr[6].lt {
	pc = 0x8240CBDC; continue 'dispatch;
	}
	// 8240CBBC: 419A0010  beq cr6, 0x8240cbcc
	if ctx.cr[6].eq {
	pc = 0x8240CBCC; continue 'dispatch;
	}
	// 8240CBC0: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8240CBC4: C02B1FF8  lfs f1, 0x1ff8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8240CBC8: 48000084  b 0x8240cc4c
	pc = 0x8240CC4C; continue 'dispatch;
            }
            0x8240CBCC => {
    //   block [0x8240CBCC..0x8240CBDC)
	// 8240CBCC: C1A30000  lfs f13, 0(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8240CBD0: 2F050004  cmpwi cr6, r5, 4
	ctx.cr[6].compare_i32(ctx.r[5].s32, 4, &mut ctx.xer);
	// 8240CBD4: C1830010  lfs f12, 0x10(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 8240CBD8: 48000030  b 0x8240cc08
	pc = 0x8240CC08; continue 'dispatch;
            }
            0x8240CBDC => {
    //   block [0x8240CBDC..0x8240CBEC)
	// 8240CBDC: C1A30010  lfs f13, 0x10(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8240CBE0: 2F050005  cmpwi cr6, r5, 5
	ctx.cr[6].compare_i32(ctx.r[5].s32, 5, &mut ctx.xer);
	// 8240CBE4: C1830014  lfs f12, 0x14(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 8240CBE8: 48000020  b 0x8240cc08
	pc = 0x8240CC08; continue 'dispatch;
            }
            0x8240CBEC => {
    //   block [0x8240CBEC..0x8240CBFC)
	// 8240CBEC: C1A30014  lfs f13, 0x14(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8240CBF0: 2F050001  cmpwi cr6, r5, 1
	ctx.cr[6].compare_i32(ctx.r[5].s32, 1, &mut ctx.xer);
	// 8240CBF4: C1830004  lfs f12, 4(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 8240CBF8: 48000010  b 0x8240cc08
	pc = 0x8240CC08; continue 'dispatch;
            }
            0x8240CBFC => {
    //   block [0x8240CBFC..0x8240CC08)
	// 8240CBFC: C1A30004  lfs f13, 4(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8240CC00: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 8240CC04: C1830000  lfs f12, 0(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	pc = 0x8240CC08; continue 'dispatch;
            }
            0x8240CC08 => {
    //   block [0x8240CC08..0x8240CC18)
	// 8240CC08: EC2D6028  fsubs f1, f13, f12
	ctx.f[1].f64 = (((ctx.f[13].f64 - ctx.f[12].f64) as f32) as f64);
	// 8240CC0C: 409A000C  bne cr6, 0x8240cc18
	if !ctx.cr[6].eq {
	pc = 0x8240CC18; continue 'dispatch;
	}
	// 8240CC10: EFE06028  fsubs f31, f0, f12
	ctx.f[31].f64 = (((ctx.f[0].f64 - ctx.f[12].f64) as f32) as f64);
	// 8240CC14: 48000008  b 0x8240cc1c
	pc = 0x8240CC1C; continue 'dispatch;
            }
            0x8240CC18 => {
    //   block [0x8240CC18..0x8240CC1C)
	// 8240CC18: EFED0028  fsubs f31, f13, f0
	ctx.f[31].f64 = (((ctx.f[13].f64 - ctx.f[0].f64) as f32) as f64);
	pc = 0x8240CC1C; continue 'dispatch;
            }
            0x8240CC1C => {
    //   block [0x8240CC1C..0x8240CC44)
	// 8240CC1C: 48003205  bl 0x8240fe20
	ctx.lr = 0x8240CC20;
	sub_8240FE20(ctx, base);
	// 8240CC20: FFC00890  fmr f30, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[30].f64 = ctx.f[1].f64;
	// 8240CC24: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 8240CC28: 480031F9  bl 0x8240fe20
	ctx.lr = 0x8240CC2C;
	sub_8240FE20(ctx, base);
	// 8240CC2C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8240CC30: C00B1FF8  lfs f0, 0x1ff8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240CC34: FF1E0000  fcmpu cr6, f30, f0
	ctx.cr[6].compare_f64(ctx.f[30].f64, ctx.f[0].f64);
	// 8240CC38: 409A000C  bne cr6, 0x8240cc44
	if !ctx.cr[6].eq {
	pc = 0x8240CC44; continue 'dispatch;
	}
	// 8240CC3C: FC200090  fmr f1, f0
	ctx.f[1].f64 = ctx.f[0].f64;
	// 8240CC40: 4800000C  b 0x8240cc4c
	pc = 0x8240CC4C; continue 'dispatch;
            }
            0x8240CC44 => {
    //   block [0x8240CC44..0x8240CC4C)
	// 8240CC44: EC1E0828  fsubs f0, f30, f1
	ctx.f[0].f64 = (((ctx.f[30].f64 - ctx.f[1].f64) as f32) as f64);
	// 8240CC48: EC20F024  fdivs f1, f0, f30
	ctx.f[1].f64 = ((ctx.f[0].f64 / ctx.f[30].f64) as f32) as f64;
	pc = 0x8240CC4C; continue 'dispatch;
            }
            0x8240CC4C => {
    //   block [0x8240CC4C..0x8240CC64)
	// 8240CC4C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8240CC50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240CC54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240CC58: CBC1FFE8  lfd f30, -0x18(r1)
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8240CC5C: CBE1FFF0  lfd f31, -0x10(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8240CC60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240CC68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8240CC68 size=320
    let mut pc: u32 = 0x8240CC68;
    'dispatch: loop {
        match pc {
            0x8240CC68 => {
    //   block [0x8240CC68..0x8240CD00)
	// 8240CC68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240CC6C: 48128451  bl 0x825350bc
	ctx.lr = 0x8240CC70;
	sub_82535080(ctx, base);
	// 8240CC70: 3981FFE0  addi r12, r1, -0x20
	ctx.r[12].s64 = ctx.r[1].s64 + -32;
	// 8240CC74: 48129371  bl 0x82535fe4
	ctx.lr = 0x8240CC78;
	sub_82535FB0(ctx, base);
	// 8240CC78: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240CC7C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8240CC80: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8240CC84: C3CB2F80  lfs f30, 0x2f80(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12160 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 8240CC88: FC20F090  fmr f1, f30
	ctx.f[1].f64 = ctx.f[30].f64;
	// 8240CC8C: 48003195  bl 0x8240fe20
	ctx.lr = 0x8240CC90;
	sub_8240FE20(ctx, base);
	// 8240CC90: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8240CC94: D03F0000  stfs f1, 0(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 8240CC98: C3AB76F4  lfs f29, 0x76f4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(30452 as u32) ) };
	ctx.f[29].f64 = (tmp.f32 as f64);
	// 8240CC9C: FC20E890  fmr f1, f29
	ctx.f[1].f64 = ctx.f[29].f64;
	// 8240CCA0: 48003181  bl 0x8240fe20
	ctx.lr = 0x8240CCA4;
	sub_8240FE20(ctx, base);
	// 8240CCA4: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8240CCA8: D03F0004  stfs f1, 4(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 8240CCAC: C3EB1FF8  lfs f31, 0x1ff8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 8240CCB0: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 8240CCB4: 4800316D  bl 0x8240fe20
	ctx.lr = 0x8240CCB8;
	sub_8240FE20(ctx, base);
	// 8240CCB8: D03F0008  stfs f1, 8(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 8240CCBC: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 8240CCC0: 48003161  bl 0x8240fe20
	ctx.lr = 0x8240CCC4;
	sub_8240FE20(ctx, base);
	// 8240CCC4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240CCC8: D03F000C  stfs f1, 0xc(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 8240CCCC: C38BC10C  lfs f28, -0x3ef4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16116 as u32) ) };
	ctx.f[28].f64 = (tmp.f32 as f64);
	// 8240CCD0: FC20E090  fmr f1, f28
	ctx.f[1].f64 = ctx.f[28].f64;
	// 8240CCD4: 4800314D  bl 0x8240fe20
	ctx.lr = 0x8240CCD8;
	sub_8240FE20(ctx, base);
	// 8240CCD8: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8240CCDC: D03F0010  stfs f1, 0x10(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 8240CCE0: C36B236C  lfs f27, 0x236c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(9068 as u32) ) };
	ctx.f[27].f64 = (tmp.f32 as f64);
	// 8240CCE4: FC20D890  fmr f1, f27
	ctx.f[1].f64 = ctx.f[27].f64;
	// 8240CCE8: 48003139  bl 0x8240fe20
	ctx.lr = 0x8240CCEC;
	sub_8240FE20(ctx, base);
	// 8240CCEC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8240CCF0: D03F0014  stfs f1, 0x14(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 8240CCF4: 4BFFFD3D  bl 0x8240ca30
	ctx.lr = 0x8240CCF8;
	sub_8240CA30(ctx, base);
	// 8240CCF8: 3BDF004C  addi r30, r31, 0x4c
	ctx.r[30].s64 = ctx.r[31].s64 + 76;
	// 8240CCFC: 3BA0000A  li r29, 0xa
	ctx.r[29].s64 = 10;
	pc = 0x8240CD00; continue 'dispatch;
            }
            0x8240CD00 => {
    //   block [0x8240CD00..0x8240CD60)
	// 8240CD00: FC20F090  fmr f1, f30
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[30].f64;
	// 8240CD04: 4800311D  bl 0x8240fe20
	ctx.lr = 0x8240CD08;
	sub_8240FE20(ctx, base);
	// 8240CD08: D03EFFFC  stfs f1, -4(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(-4 as u32), tmp.u32 ) };
	// 8240CD0C: FC20E890  fmr f1, f29
	ctx.f[1].f64 = ctx.f[29].f64;
	// 8240CD10: 48003111  bl 0x8240fe20
	ctx.lr = 0x8240CD14;
	sub_8240FE20(ctx, base);
	// 8240CD14: D03E0000  stfs f1, 0(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 8240CD18: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 8240CD1C: 48003105  bl 0x8240fe20
	ctx.lr = 0x8240CD20;
	sub_8240FE20(ctx, base);
	// 8240CD20: D03E0004  stfs f1, 4(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 8240CD24: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 8240CD28: 480030F9  bl 0x8240fe20
	ctx.lr = 0x8240CD2C;
	sub_8240FE20(ctx, base);
	// 8240CD2C: D03E0008  stfs f1, 8(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 8240CD30: FC20E090  fmr f1, f28
	ctx.f[1].f64 = ctx.f[28].f64;
	// 8240CD34: 480030ED  bl 0x8240fe20
	ctx.lr = 0x8240CD38;
	sub_8240FE20(ctx, base);
	// 8240CD38: D03E000C  stfs f1, 0xc(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 8240CD3C: FC20D890  fmr f1, f27
	ctx.f[1].f64 = ctx.f[27].f64;
	// 8240CD40: 480030E1  bl 0x8240fe20
	ctx.lr = 0x8240CD44;
	sub_8240FE20(ctx, base);
	// 8240CD44: D03E0010  stfs f1, 0x10(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 8240CD48: 37BDFFFF  addic. r29, r29, -1
	ctx.xer.ca = (ctx.r[29].u32 > (!(-1 as u32)));
	ctx.r[29].s64 = ctx.r[29].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 8240CD4C: 3BDE0018  addi r30, r30, 0x18
	ctx.r[30].s64 = ctx.r[30].s64 + 24;
	// 8240CD50: 4082FFB0  bne 0x8240cd00
	if !ctx.cr[0].eq {
	pc = 0x8240CD00; continue 'dispatch;
	}
	// 8240CD54: 397F013C  addi r11, r31, 0x13c
	ctx.r[11].s64 = ctx.r[31].s64 + 316;
	// 8240CD58: 3940000A  li r10, 0xa
	ctx.r[10].s64 = 10;
	// 8240CD5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	pc = 0x8240CD60; continue 'dispatch;
            }
            0x8240CD60 => {
    //   block [0x8240CD60..0x8240CDA8)
	// 8240CD60: 3900FFE2  li r8, -0x1e
	ctx.r[8].s64 = -30;
	// 8240CD64: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 8240CD68: 38E0001E  li r7, 0x1e
	ctx.r[7].s64 = 30;
	// 8240CD6C: 912B0008  stw r9, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 8240CD70: 38C0FF92  li r6, -0x6e
	ctx.r[6].s64 = -110;
	// 8240CD74: 38A0006E  li r5, 0x6e
	ctx.r[5].s64 = 110;
	// 8240CD78: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8240CD7C: 910BFFFC  stw r8, -4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-4 as u32), ctx.r[8].u32 ) };
	// 8240CD80: 90EB0000  stw r7, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 8240CD84: 90CB000C  stw r6, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[6].u32 ) };
	// 8240CD88: 90AB0010  stw r5, 0x10(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[5].u32 ) };
	// 8240CD8C: 396B0018  addi r11, r11, 0x18
	ctx.r[11].s64 = ctx.r[11].s64 + 24;
	// 8240CD90: 4082FFD0  bne 0x8240cd60
	if !ctx.cr[0].eq {
	pc = 0x8240CD60; continue 'dispatch;
	}
	// 8240CD94: 913F0228  stw r9, 0x228(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(552 as u32), ctx.r[9].u32 ) };
	// 8240CD98: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 8240CD9C: 3981FFE0  addi r12, r1, -0x20
	ctx.r[12].s64 = ctx.r[1].s64 + -32;
	// 8240CDA0: 48129291  bl 0x82536030
	ctx.lr = 0x8240CDA4;
	sub_82535FFC(ctx, base);
	// 8240CDA4: 48128368  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240CDA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8240CDA8 size=156
    let mut pc: u32 = 0x8240CDA8;
    'dispatch: loop {
        match pc {
            0x8240CDA8 => {
    //   block [0x8240CDA8..0x8240CE44)
	// 8240CDA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240CDAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240CDB0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8240CDB4: 3981FFF0  addi r12, r1, -0x10
	ctx.r[12].s64 = ctx.r[1].s64 + -16;
	// 8240CDB8: 48129231  bl 0x82535fe8
	ctx.lr = 0x8240CDBC;
	sub_82535FB0(ctx, base);
	// 8240CDBC: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240CDC0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8240CDC4: FFE01090  fmr f31, f2
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].f64 = ctx.f[2].f64;
	// 8240CDC8: FFC01890  fmr f30, f3
	ctx.f[30].f64 = ctx.f[3].f64;
	// 8240CDCC: FFA02090  fmr f29, f4
	ctx.f[29].f64 = ctx.f[4].f64;
	// 8240CDD0: FF802890  fmr f28, f5
	ctx.f[28].f64 = ctx.f[5].f64;
	// 8240CDD4: 4800304D  bl 0x8240fe20
	ctx.lr = 0x8240CDD8;
	sub_8240FE20(ctx, base);
	// 8240CDD8: D03F0000  stfs f1, 0(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 8240CDDC: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 8240CDE0: 48003041  bl 0x8240fe20
	ctx.lr = 0x8240CDE4;
	sub_8240FE20(ctx, base);
	// 8240CDE4: D03F0004  stfs f1, 4(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 8240CDE8: FC20F090  fmr f1, f30
	ctx.f[1].f64 = ctx.f[30].f64;
	// 8240CDEC: 48003035  bl 0x8240fe20
	ctx.lr = 0x8240CDF0;
	sub_8240FE20(ctx, base);
	// 8240CDF0: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8240CDF4: D03F0008  stfs f1, 8(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 8240CDF8: C02B1FF8  lfs f1, 0x1ff8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8240CDFC: 48003025  bl 0x8240fe20
	ctx.lr = 0x8240CE00;
	sub_8240FE20(ctx, base);
	// 8240CE00: D03F000C  stfs f1, 0xc(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 8240CE04: FC20E890  fmr f1, f29
	ctx.f[1].f64 = ctx.f[29].f64;
	// 8240CE08: 48003019  bl 0x8240fe20
	ctx.lr = 0x8240CE0C;
	sub_8240FE20(ctx, base);
	// 8240CE0C: D03F0010  stfs f1, 0x10(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 8240CE10: FC20E090  fmr f1, f28
	ctx.f[1].f64 = ctx.f[28].f64;
	// 8240CE14: 4800300D  bl 0x8240fe20
	ctx.lr = 0x8240CE18;
	sub_8240FE20(ctx, base);
	// 8240CE18: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8240CE1C: D03F0014  stfs f1, 0x14(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 8240CE20: 4BFFFC11  bl 0x8240ca30
	ctx.lr = 0x8240CE24;
	sub_8240CA30(ctx, base);
	// 8240CE24: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8240CE28: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8240CE2C: 3981FFF0  addi r12, r1, -0x10
	ctx.r[12].s64 = ctx.r[1].s64 + -16;
	// 8240CE30: 48129205  bl 0x82536034
	ctx.lr = 0x8240CE34;
	sub_82535FFC(ctx, base);
	// 8240CE34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240CE38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240CE3C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8240CE40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240CE48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8240CE48 size=48
    let mut pc: u32 = 0x8240CE48;
    'dispatch: loop {
        match pc {
            0x8240CE48 => {
    //   block [0x8240CE48..0x8240CE78)
	// 8240CE48: 2B040009  cmplwi cr6, r4, 9
	ctx.cr[6].compare_u32(ctx.r[4].u32, 9 as u32, &mut ctx.xer);
	// 8240CE4C: 4199002C  bgt cr6, 0x8240ce78
	if ctx.cr[6].gt {
		sub_8240CE78(ctx, base);
		return;
	}
	// 8240CE50: 39640003  addi r11, r4, 3
	ctx.r[11].s64 = ctx.r[4].s64 + 3;
	// 8240CE54: 90830228  stw r4, 0x228(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(552 as u32), ctx.r[4].u32 ) };
	// 8240CE58: 1D6B0018  mulli r11, r11, 0x18
	ctx.r[11].s32 = ((ctx.r[11].s32 as i64 * 24 as i64) as i32);
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 8240CE5C: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 8240CE60: C0AB0014  lfs f5, 0x14(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) };
	ctx.f[5].f64 = (tmp.f32 as f64);
	// 8240CE64: C08B0010  lfs f4, 0x10(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) };
	ctx.f[4].f64 = (tmp.f32 as f64);
	// 8240CE68: C06B0008  lfs f3, 8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	ctx.f[3].f64 = (tmp.f32 as f64);
	// 8240CE6C: C04B0004  lfs f2, 4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 8240CE70: C02B0000  lfs f1, 0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8240CE74: 4BFFFF34  b 0x8240cda8
	sub_8240CDA8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240CE78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8240CE78 size=12
    let mut pc: u32 = 0x8240CE78;
    'dispatch: loop {
        match pc {
            0x8240CE78 => {
    //   block [0x8240CE78..0x8240CE84)
	// 8240CE78: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 8240CE7C: 6063002B  ori r3, r3, 0x2b
	ctx.r[3].u64 = ctx.r[3].u64 | 43;
	// 8240CE80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240CE88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8240CE88 size=1460
    let mut pc: u32 = 0x8240CE88;
    'dispatch: loop {
        match pc {
            0x8240CE88 => {
    //   block [0x8240CE88..0x8240CF04)
	// 8240CE88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240CE8C: 48128231  bl 0x825350bc
	ctx.lr = 0x8240CE90;
	sub_82535080(ctx, base);
	// 8240CE90: 3981FFE0  addi r12, r1, -0x20
	ctx.r[12].s64 = ctx.r[1].s64 + -32;
	// 8240CE94: 48129145  bl 0x82535fd8
	ctx.lr = 0x8240CE98;
	sub_82535FB0(ctx, base);
	// 8240CE98: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240CE9C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8240CEA0: FFC01090  fmr f30, f2
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[30].f64 = ctx.f[2].f64;
	// 8240CEA4: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 8240CEA8: FF001890  fmr f24, f3
	ctx.f[24].f64 = ctx.f[3].f64;
	// 8240CEAC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8240CEB0: C34B1FF8  lfs f26, 0x1ff8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[26].f64 = (tmp.f32 as f64);
	// 8240CEB4: D35E000C  stfs f26, 0xc(r30)
	tmp.f32 = (ctx.f[26].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 8240CEB8: 817F0228  lwz r11, 0x228(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(552 as u32) ) } as u64;
	// 8240CEBC: 394B000D  addi r10, r11, 0xd
	ctx.r[10].s64 = ctx.r[11].s64 + 13;
	// 8240CEC0: 1D4A0018  mulli r10, r10, 0x18
	ctx.r[10].s32 = ((ctx.r[10].s32 as i64 * 24 as i64) as i32);
	ctx.r[10].s64 = ctx.r[10].s32 as i64;
	// 8240CEC4: 7D4AF82E  lwzx r10, r10, r31
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 8240CEC8: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8240CECC: 409A0038  bne cr6, 0x8240cf04
	if !ctx.cr[6].eq {
	pc = 0x8240CF04; continue 'dispatch;
	}
	// 8240CED0: 1D6B0018  mulli r11, r11, 0x18
	ctx.r[11].s32 = ((ctx.r[11].s32 as i64 * 24 as i64) as i32);
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 8240CED4: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 8240CED8: 816B013C  lwz r11, 0x13c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(316 as u32) ) } as u64;
	// 8240CEDC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8240CEE0: 409A0024  bne cr6, 0x8240cf04
	if !ctx.cr[6].eq {
	pc = 0x8240CF04; continue 'dispatch;
	}
	// 8240CEE4: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8240CEE8: D35E0010  stfs f26, 0x10(r30)
	tmp.f32 = (ctx.f[26].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 8240CEEC: D35E0014  stfs f26, 0x14(r30)
	tmp.f32 = (ctx.f[26].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 8240CEF0: D35E0008  stfs f26, 8(r30)
	tmp.f32 = (ctx.f[26].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 8240CEF4: C00B2AD4  lfs f0, 0x2ad4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(10964 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240CEF8: D01E0000  stfs f0, 0(r30)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 8240CEFC: D01E0004  stfs f0, 4(r30)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 8240CF00: 4800052C  b 0x8240d42c
	pc = 0x8240D42C; continue 'dispatch;
            }
            0x8240CF04 => {
    //   block [0x8240CF04..0x8240CF38)
	// 8240CF04: 48002F1D  bl 0x8240fe20
	ctx.lr = 0x8240CF08;
	sub_8240FE20(ctx, base);
	// 8240CF08: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8240CF0C: FFE00890  fmr f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].f64 = ctx.f[1].f64;
	// 8240CF10: FF1ED000  fcmpu cr6, f30, f26
	ctx.cr[6].compare_f64(ctx.f[30].f64, ctx.f[26].f64);
	// 8240CF14: C3AB213C  lfs f29, 0x213c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8508 as u32) ) };
	ctx.f[29].f64 = (tmp.f32 as f64);
	// 8240CF18: 40980024  bge cr6, 0x8240cf3c
	if !ctx.cr[6].lt {
	pc = 0x8240CF3C; continue 'dispatch;
	}
	// 8240CF1C: FFC0F050  fneg f30, f30
	ctx.f[30].u64 = ctx.f[30].u64 ^ 0x8000_0000_0000_0000u64;
	// 8240CF20: FF1FE800  fcmpu cr6, f31, f29
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[29].f64);
	// 8240CF24: 40990014  ble cr6, 0x8240cf38
	if !ctx.cr[6].gt {
	pc = 0x8240CF38; continue 'dispatch;
	}
	// 8240CF28: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8240CF2C: C00B2C68  lfs f0, 0x2c68(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(11368 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240CF30: EFE0F828  fsubs f31, f0, f31
	ctx.f[31].f64 = (((ctx.f[0].f64 - ctx.f[31].f64) as f32) as f64);
	// 8240CF34: 48000008  b 0x8240cf3c
	pc = 0x8240CF3C; continue 'dispatch;
            }
            0x8240CF38 => {
    //   block [0x8240CF38..0x8240CF3C)
	// 8240CF38: EFFDF828  fsubs f31, f29, f31
	ctx.f[31].f64 = (((ctx.f[29].f64 - ctx.f[31].f64) as f32) as f64);
	pc = 0x8240CF3C; continue 'dispatch;
            }
            0x8240CF3C => {
    //   block [0x8240CF3C..0x8240CF74)
	// 8240CF3C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8240CF40: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 8240CF44: 4BFFFBD5  bl 0x8240cb18
	ctx.lr = 0x8240CF48;
	sub_8240CB18(ctx, base);
	// 8240CF48: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8240CF4C: FF1FE800  fcmpu cr6, f31, f29
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[29].f64);
	// 8240CF50: 57AB1838  slwi r11, r29, 3
	ctx.r[11].u32 = ctx.r[29].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8240CF54: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 8240CF58: C00B001C  lfs f0, 0x1c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240CF5C: 40990018  ble cr6, 0x8240cf74
	if !ctx.cr[6].gt {
	pc = 0x8240CF74; continue 'dispatch;
	}
	// 8240CF60: FF00E800  fcmpu cr6, f0, f29
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[29].f64);
	// 8240CF64: 40980010  bge cr6, 0x8240cf74
	if !ctx.cr[6].lt {
	pc = 0x8240CF74; continue 'dispatch;
	}
	// 8240CF68: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8240CF6C: C1AB210C  lfs f13, 0x210c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8460 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8240CF70: EC00682A  fadds f0, f0, f13
	ctx.f[0].f64 = ((ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64;
	pc = 0x8240CF74; continue 'dispatch;
            }
            0x8240CF74 => {
    //   block [0x8240CF74..0x8240CFC0)
	// 8240CF74: EC20F828  fsubs f1, f0, f31
	ctx.f[1].f64 = (((ctx.f[0].f64 - ctx.f[31].f64) as f32) as f64);
	// 8240CF78: 48002F01  bl 0x8240fe78
	ctx.lr = 0x8240CF7C;
	sub_8240FE78(ctx, base);
	// 8240CF7C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8240CF80: C3AB1850  lfs f29, 0x1850(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(6224 as u32) ) };
	ctx.f[29].f64 = (tmp.f32 as f64);
	// 8240CF84: EC1DF028  fsubs f0, f29, f30
	ctx.f[0].f64 = (((ctx.f[29].f64 - ctx.f[30].f64) as f32) as f64);
	// 8240CF88: EC20F87A  fmadds f1, f0, f1, f31
	ctx.f[1].f64 = (((ctx.f[0].f64 * ctx.f[1].f64 + ctx.f[31].f64) as f32) as f64);
	// 8240CF8C: 48002E95  bl 0x8240fe20
	ctx.lr = 0x8240CF90;
	sub_8240FE20(ctx, base);
	// 8240CF90: EDBEE82A  fadds f13, f30, f29
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[13].f64 = ((ctx.f[30].f64 + ctx.f[29].f64) as f32) as f64;
	// 8240CF94: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8240CF98: FFC00890  fmr f30, f1
	ctx.f[30].f64 = ctx.f[1].f64;
	// 8240CF9C: C00BBFFC  lfs f0, -0x4004(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16388 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240CFA0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240CFA4: 396BE358  addi r11, r11, -0x1ca8
	ctx.r[11].s64 = ctx.r[11].s64 + -7336;
	// 8240CFA8: EC0D0032  fmuls f0, f13, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 8240CFAC: C32B0000  lfs f25, 0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[25].f64 = (tmp.f32 as f64);
	// 8240CFB0: FF00E800  fcmpu cr6, f0, f29
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[29].f64);
	// 8240CFB4: 4198000C  blt cr6, 0x8240cfc0
	if ctx.cr[6].lt {
	pc = 0x8240CFC0; continue 'dispatch;
	}
	// 8240CFB8: FF80E890  fmr f28, f29
	ctx.f[28].f64 = ctx.f[29].f64;
	// 8240CFBC: 48000028  b 0x8240cfe4
	pc = 0x8240CFE4; continue 'dispatch;
            }
            0x8240CFC0 => {
    //   block [0x8240CFC0..0x8240CFD0)
	// 8240CFC0: FF00D000  fcmpu cr6, f0, f26
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[26].f64);
	// 8240CFC4: 4199000C  bgt cr6, 0x8240cfd0
	if ctx.cr[6].gt {
	pc = 0x8240CFD0; continue 'dispatch;
	}
	// 8240CFC8: FF80D090  fmr f28, f26
	ctx.f[28].f64 = ctx.f[26].f64;
	// 8240CFCC: 48000028  b 0x8240cff4
	pc = 0x8240CFF4; continue 'dispatch;
            }
            0x8240CFD0 => {
    //   block [0x8240CFD0..0x8240CFE4)
	// 8240CFD0: EC200672  fmuls f1, f0, f25
	ctx.f[1].f64 = (((ctx.f[0].f64 * ctx.f[25].f64) as f32) as f64);
	// 8240CFD4: 48002C6D  bl 0x8240fc40
	ctx.lr = 0x8240CFD8;
	sub_8240FC40(ctx, base);
	// 8240CFD8: FF800890  fmr f28, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[28].f64 = ctx.f[1].f64;
	// 8240CFDC: FF1CE800  fcmpu cr6, f28, f29
	ctx.cr[6].compare_f64(ctx.f[28].f64, ctx.f[29].f64);
	// 8240CFE0: 4198000C  blt cr6, 0x8240cfec
	if ctx.cr[6].lt {
	pc = 0x8240CFEC; continue 'dispatch;
	}
	pc = 0x8240CFE4; continue 'dispatch;
            }
            0x8240CFE4 => {
    //   block [0x8240CFE4..0x8240CFEC)
	// 8240CFE4: FF60D090  fmr f27, f26
	ctx.f[27].f64 = ctx.f[26].f64;
	// 8240CFE8: 48000020  b 0x8240d008
	pc = 0x8240D008; continue 'dispatch;
            }
            0x8240CFEC => {
    //   block [0x8240CFEC..0x8240CFF4)
	// 8240CFEC: FF1CD000  fcmpu cr6, f28, f26
	ctx.cr[6].compare_f64(ctx.f[28].f64, ctx.f[26].f64);
	// 8240CFF0: 4199000C  bgt cr6, 0x8240cffc
	if ctx.cr[6].gt {
	pc = 0x8240CFFC; continue 'dispatch;
	}
	pc = 0x8240CFF4; continue 'dispatch;
            }
            0x8240CFF4 => {
    //   block [0x8240CFF4..0x8240CFFC)
	// 8240CFF4: FF60E890  fmr f27, f29
	ctx.f[27].f64 = ctx.f[29].f64;
	// 8240CFF8: 48000010  b 0x8240d008
	pc = 0x8240D008; continue 'dispatch;
            }
            0x8240CFFC => {
    //   block [0x8240CFFC..0x8240D008)
	// 8240CFFC: EC3CEF3C  fnmsubs f1, f28, f28, f29
	ctx.f[1].f64 = -(((ctx.f[28].f64 * ctx.f[28].f64 - ctx.f[29].f64) as f32) as f64);
	// 8240D000: 48002EB9  bl 0x8240feb8
	ctx.lr = 0x8240D004;
	sub_8240FEB8(ctx, base);
	// 8240D004: FF600890  fmr f27, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[27].f64 = ctx.f[1].f64;
	pc = 0x8240D008; continue 'dispatch;
            }
            0x8240D008 => {
    //   block [0x8240D008..0x8240D044)
	// 8240D008: 2B1D0001  cmplwi cr6, r29, 1
	ctx.cr[6].compare_u32(ctx.r[29].u32, 1 as u32, &mut ctx.xer);
	// 8240D00C: 419801EC  blt cr6, 0x8240d1f8
	if ctx.cr[6].lt {
	pc = 0x8240D1F8; continue 'dispatch;
	}
	// 8240D010: 419A0148  beq cr6, 0x8240d158
	if ctx.cr[6].eq {
	pc = 0x8240D158; continue 'dispatch;
	}
	// 8240D014: 2B1D0003  cmplwi cr6, r29, 3
	ctx.cr[6].compare_u32(ctx.r[29].u32, 3 as u32, &mut ctx.xer);
	// 8240D018: 419800A0  blt cr6, 0x8240d0b8
	if ctx.cr[6].lt {
	pc = 0x8240D0B8; continue 'dispatch;
	}
	// 8240D01C: 409A0278  bne cr6, 0x8240d294
	if !ctx.cr[6].eq {
	pc = 0x8240D294; continue 'dispatch;
	}
	// 8240D020: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8240D024: FC20F090  fmr f1, f30
	ctx.f[1].f64 = ctx.f[30].f64;
	// 8240D028: 38800003  li r4, 3
	ctx.r[4].s64 = 3;
	// 8240D02C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8240D030: 4BFFFB61  bl 0x8240cb90
	ctx.lr = 0x8240D034;
	sub_8240CB90(ctx, base);
	// 8240D034: FF01E800  fcmpu cr6, f1, f29
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[29].f64);
	// 8240D038: 4198000C  blt cr6, 0x8240d044
	if ctx.cr[6].lt {
	pc = 0x8240D044; continue 'dispatch;
	}
	// 8240D03C: FFE0E890  fmr f31, f29
	ctx.f[31].f64 = ctx.f[29].f64;
	// 8240D040: 48000020  b 0x8240d060
	pc = 0x8240D060; continue 'dispatch;
            }
            0x8240D044 => {
    //   block [0x8240D044..0x8240D054)
	// 8240D044: FF01D000  fcmpu cr6, f1, f26
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[26].f64);
	// 8240D048: 4199000C  bgt cr6, 0x8240d054
	if ctx.cr[6].gt {
	pc = 0x8240D054; continue 'dispatch;
	}
	// 8240D04C: FFE0D090  fmr f31, f26
	ctx.f[31].f64 = ctx.f[26].f64;
	// 8240D050: 48000010  b 0x8240d060
	pc = 0x8240D060; continue 'dispatch;
            }
            0x8240D054 => {
    //   block [0x8240D054..0x8240D060)
	// 8240D054: EC210672  fmuls f1, f1, f25
	ctx.f[1].f64 = (((ctx.f[1].f64 * ctx.f[25].f64) as f32) as f64);
	// 8240D058: 48002BE9  bl 0x8240fc40
	ctx.lr = 0x8240D05C;
	sub_8240FC40(ctx, base);
	// 8240D05C: FFE00890  fmr f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].f64 = ctx.f[1].f64;
	pc = 0x8240D060; continue 'dispatch;
            }
            0x8240D060 => {
    //   block [0x8240D060..0x8240D084)
	// 8240D060: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 8240D064: FC20F090  fmr f1, f30
	ctx.f[1].f64 = ctx.f[30].f64;
	// 8240D068: 38800003  li r4, 3
	ctx.r[4].s64 = 3;
	// 8240D06C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8240D070: 4BFFFB21  bl 0x8240cb90
	ctx.lr = 0x8240D074;
	sub_8240CB90(ctx, base);
	// 8240D074: FF01E800  fcmpu cr6, f1, f29
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[29].f64);
	// 8240D078: 4198000C  blt cr6, 0x8240d084
	if ctx.cr[6].lt {
	pc = 0x8240D084; continue 'dispatch;
	}
	// 8240D07C: FC20E890  fmr f1, f29
	ctx.f[1].f64 = ctx.f[29].f64;
	// 8240D080: 4800001C  b 0x8240d09c
	pc = 0x8240D09C; continue 'dispatch;
            }
            0x8240D084 => {
    //   block [0x8240D084..0x8240D094)
	// 8240D084: FF01D000  fcmpu cr6, f1, f26
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[26].f64);
	// 8240D088: 4199000C  bgt cr6, 0x8240d094
	if ctx.cr[6].gt {
	pc = 0x8240D094; continue 'dispatch;
	}
	// 8240D08C: FC20D090  fmr f1, f26
	ctx.f[1].f64 = ctx.f[26].f64;
	// 8240D090: 4800000C  b 0x8240d09c
	pc = 0x8240D09C; continue 'dispatch;
            }
            0x8240D094 => {
    //   block [0x8240D094..0x8240D09C)
	// 8240D094: EC210672  fmuls f1, f1, f25
	ctx.f[1].f64 = (((ctx.f[1].f64 * ctx.f[25].f64) as f32) as f64);
	// 8240D098: 48002BA9  bl 0x8240fc40
	ctx.lr = 0x8240D09C;
	sub_8240FC40(ctx, base);
	pc = 0x8240D09C; continue 'dispatch;
            }
            0x8240D09C => {
    //   block [0x8240D09C..0x8240D0B8)
	// 8240D09C: EC1F0732  fmuls f0, f31, f28
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = (((ctx.f[31].f64 * ctx.f[28].f64) as f32) as f64);
	// 8240D0A0: D01E0000  stfs f0, 0(r30)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 8240D0A4: EC010732  fmuls f0, f1, f28
	ctx.f[0].f64 = (((ctx.f[1].f64 * ctx.f[28].f64) as f32) as f64);
	// 8240D0A8: D01E0010  stfs f0, 0x10(r30)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 8240D0AC: EC1F06F2  fmuls f0, f31, f27
	ctx.f[0].f64 = (((ctx.f[31].f64 * ctx.f[27].f64) as f32) as f64);
	// 8240D0B0: D01E0004  stfs f0, 4(r30)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 8240D0B4: 480001D8  b 0x8240d28c
	pc = 0x8240D28C; continue 'dispatch;
            }
            0x8240D0B8 => {
    //   block [0x8240D0B8..0x8240D0DC)
	// 8240D0B8: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 8240D0BC: FC20F090  fmr f1, f30
	ctx.f[1].f64 = ctx.f[30].f64;
	// 8240D0C0: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 8240D0C4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8240D0C8: 4BFFFAC9  bl 0x8240cb90
	ctx.lr = 0x8240D0CC;
	sub_8240CB90(ctx, base);
	// 8240D0CC: FF01E800  fcmpu cr6, f1, f29
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[29].f64);
	// 8240D0D0: 4198000C  blt cr6, 0x8240d0dc
	if ctx.cr[6].lt {
	pc = 0x8240D0DC; continue 'dispatch;
	}
	// 8240D0D4: FFE0E890  fmr f31, f29
	ctx.f[31].f64 = ctx.f[29].f64;
	// 8240D0D8: 48000020  b 0x8240d0f8
	pc = 0x8240D0F8; continue 'dispatch;
            }
            0x8240D0DC => {
    //   block [0x8240D0DC..0x8240D0EC)
	// 8240D0DC: FF01D000  fcmpu cr6, f1, f26
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[26].f64);
	// 8240D0E0: 4199000C  bgt cr6, 0x8240d0ec
	if ctx.cr[6].gt {
	pc = 0x8240D0EC; continue 'dispatch;
	}
	// 8240D0E4: FFE0D090  fmr f31, f26
	ctx.f[31].f64 = ctx.f[26].f64;
	// 8240D0E8: 48000010  b 0x8240d0f8
	pc = 0x8240D0F8; continue 'dispatch;
            }
            0x8240D0EC => {
    //   block [0x8240D0EC..0x8240D0F8)
	// 8240D0EC: EC210672  fmuls f1, f1, f25
	ctx.f[1].f64 = (((ctx.f[1].f64 * ctx.f[25].f64) as f32) as f64);
	// 8240D0F0: 48002B51  bl 0x8240fc40
	ctx.lr = 0x8240D0F4;
	sub_8240FC40(ctx, base);
	// 8240D0F4: FFE00890  fmr f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].f64 = ctx.f[1].f64;
	pc = 0x8240D0F8; continue 'dispatch;
            }
            0x8240D0F8 => {
    //   block [0x8240D0F8..0x8240D11C)
	// 8240D0F8: 38A00005  li r5, 5
	ctx.r[5].s64 = 5;
	// 8240D0FC: FC20F090  fmr f1, f30
	ctx.f[1].f64 = ctx.f[30].f64;
	// 8240D100: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 8240D104: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8240D108: 4BFFFA89  bl 0x8240cb90
	ctx.lr = 0x8240D10C;
	sub_8240CB90(ctx, base);
	// 8240D10C: FF01E800  fcmpu cr6, f1, f29
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[29].f64);
	// 8240D110: 4198000C  blt cr6, 0x8240d11c
	if ctx.cr[6].lt {
	pc = 0x8240D11C; continue 'dispatch;
	}
	// 8240D114: FC20E890  fmr f1, f29
	ctx.f[1].f64 = ctx.f[29].f64;
	// 8240D118: 4800001C  b 0x8240d134
	pc = 0x8240D134; continue 'dispatch;
            }
            0x8240D11C => {
    //   block [0x8240D11C..0x8240D12C)
	// 8240D11C: FF01D000  fcmpu cr6, f1, f26
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[26].f64);
	// 8240D120: 4199000C  bgt cr6, 0x8240d12c
	if ctx.cr[6].gt {
	pc = 0x8240D12C; continue 'dispatch;
	}
	// 8240D124: FC20D090  fmr f1, f26
	ctx.f[1].f64 = ctx.f[26].f64;
	// 8240D128: 4800000C  b 0x8240d134
	pc = 0x8240D134; continue 'dispatch;
            }
            0x8240D12C => {
    //   block [0x8240D12C..0x8240D134)
	// 8240D12C: EC210672  fmuls f1, f1, f25
	ctx.f[1].f64 = (((ctx.f[1].f64 * ctx.f[25].f64) as f32) as f64);
	// 8240D130: 48002B11  bl 0x8240fc40
	ctx.lr = 0x8240D134;
	sub_8240FC40(ctx, base);
	pc = 0x8240D134; continue 'dispatch;
            }
            0x8240D134 => {
    //   block [0x8240D134..0x8240D158)
	// 8240D134: EC1F0732  fmuls f0, f31, f28
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = (((ctx.f[31].f64 * ctx.f[28].f64) as f32) as f64);
	// 8240D138: D01E0010  stfs f0, 0x10(r30)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 8240D13C: EC010732  fmuls f0, f1, f28
	ctx.f[0].f64 = (((ctx.f[1].f64 * ctx.f[28].f64) as f32) as f64);
	// 8240D140: D01E0014  stfs f0, 0x14(r30)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 8240D144: EC1F06F2  fmuls f0, f31, f27
	ctx.f[0].f64 = (((ctx.f[31].f64 * ctx.f[27].f64) as f32) as f64);
	// 8240D148: D01E0000  stfs f0, 0(r30)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 8240D14C: EC0106F2  fmuls f0, f1, f27
	ctx.f[0].f64 = (((ctx.f[1].f64 * ctx.f[27].f64) as f32) as f64);
	// 8240D150: D01E0004  stfs f0, 4(r30)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 8240D154: 48000140  b 0x8240d294
	pc = 0x8240D294; continue 'dispatch;
            }
            0x8240D158 => {
    //   block [0x8240D158..0x8240D17C)
	// 8240D158: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8240D15C: FC20F090  fmr f1, f30
	ctx.f[1].f64 = ctx.f[30].f64;
	// 8240D160: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8240D164: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8240D168: 4BFFFA29  bl 0x8240cb90
	ctx.lr = 0x8240D16C;
	sub_8240CB90(ctx, base);
	// 8240D16C: FF01E800  fcmpu cr6, f1, f29
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[29].f64);
	// 8240D170: 4198000C  blt cr6, 0x8240d17c
	if ctx.cr[6].lt {
	pc = 0x8240D17C; continue 'dispatch;
	}
	// 8240D174: FFE0E890  fmr f31, f29
	ctx.f[31].f64 = ctx.f[29].f64;
	// 8240D178: 48000020  b 0x8240d198
	pc = 0x8240D198; continue 'dispatch;
            }
            0x8240D17C => {
    //   block [0x8240D17C..0x8240D18C)
	// 8240D17C: FF01D000  fcmpu cr6, f1, f26
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[26].f64);
	// 8240D180: 4199000C  bgt cr6, 0x8240d18c
	if ctx.cr[6].gt {
	pc = 0x8240D18C; continue 'dispatch;
	}
	// 8240D184: FFE0D090  fmr f31, f26
	ctx.f[31].f64 = ctx.f[26].f64;
	// 8240D188: 48000010  b 0x8240d198
	pc = 0x8240D198; continue 'dispatch;
            }
            0x8240D18C => {
    //   block [0x8240D18C..0x8240D198)
	// 8240D18C: EC210672  fmuls f1, f1, f25
	ctx.f[1].f64 = (((ctx.f[1].f64 * ctx.f[25].f64) as f32) as f64);
	// 8240D190: 48002AB1  bl 0x8240fc40
	ctx.lr = 0x8240D194;
	sub_8240FC40(ctx, base);
	// 8240D194: FFE00890  fmr f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].f64 = ctx.f[1].f64;
	pc = 0x8240D198; continue 'dispatch;
            }
            0x8240D198 => {
    //   block [0x8240D198..0x8240D1BC)
	// 8240D198: 38A00005  li r5, 5
	ctx.r[5].s64 = 5;
	// 8240D19C: FC20F090  fmr f1, f30
	ctx.f[1].f64 = ctx.f[30].f64;
	// 8240D1A0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8240D1A4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8240D1A8: 4BFFF9E9  bl 0x8240cb90
	ctx.lr = 0x8240D1AC;
	sub_8240CB90(ctx, base);
	// 8240D1AC: FF01E800  fcmpu cr6, f1, f29
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[29].f64);
	// 8240D1B0: 4198000C  blt cr6, 0x8240d1bc
	if ctx.cr[6].lt {
	pc = 0x8240D1BC; continue 'dispatch;
	}
	// 8240D1B4: FC20E890  fmr f1, f29
	ctx.f[1].f64 = ctx.f[29].f64;
	// 8240D1B8: 4800001C  b 0x8240d1d4
	pc = 0x8240D1D4; continue 'dispatch;
            }
            0x8240D1BC => {
    //   block [0x8240D1BC..0x8240D1CC)
	// 8240D1BC: FF01D000  fcmpu cr6, f1, f26
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[26].f64);
	// 8240D1C0: 4199000C  bgt cr6, 0x8240d1cc
	if ctx.cr[6].gt {
	pc = 0x8240D1CC; continue 'dispatch;
	}
	// 8240D1C4: FC20D090  fmr f1, f26
	ctx.f[1].f64 = ctx.f[26].f64;
	// 8240D1C8: 4800000C  b 0x8240d1d4
	pc = 0x8240D1D4; continue 'dispatch;
            }
            0x8240D1CC => {
    //   block [0x8240D1CC..0x8240D1D4)
	// 8240D1CC: EC210672  fmuls f1, f1, f25
	ctx.f[1].f64 = (((ctx.f[1].f64 * ctx.f[25].f64) as f32) as f64);
	// 8240D1D0: 48002A71  bl 0x8240fc40
	ctx.lr = 0x8240D1D4;
	sub_8240FC40(ctx, base);
	pc = 0x8240D1D4; continue 'dispatch;
            }
            0x8240D1D4 => {
    //   block [0x8240D1D4..0x8240D1F8)
	// 8240D1D4: EC1F0732  fmuls f0, f31, f28
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = (((ctx.f[31].f64 * ctx.f[28].f64) as f32) as f64);
	// 8240D1D8: D01E0004  stfs f0, 4(r30)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 8240D1DC: EC010732  fmuls f0, f1, f28
	ctx.f[0].f64 = (((ctx.f[1].f64 * ctx.f[28].f64) as f32) as f64);
	// 8240D1E0: D01E0014  stfs f0, 0x14(r30)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 8240D1E4: EC1F06F2  fmuls f0, f31, f27
	ctx.f[0].f64 = (((ctx.f[31].f64 * ctx.f[27].f64) as f32) as f64);
	// 8240D1E8: D01E0000  stfs f0, 0(r30)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 8240D1EC: EC0106F2  fmuls f0, f1, f27
	ctx.f[0].f64 = (((ctx.f[1].f64 * ctx.f[27].f64) as f32) as f64);
	// 8240D1F0: D01E0010  stfs f0, 0x10(r30)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 8240D1F4: 480000A0  b 0x8240d294
	pc = 0x8240D294; continue 'dispatch;
            }
            0x8240D1F8 => {
    //   block [0x8240D1F8..0x8240D21C)
	// 8240D1F8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8240D1FC: FC20F090  fmr f1, f30
	ctx.f[1].f64 = ctx.f[30].f64;
	// 8240D200: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8240D204: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8240D208: 4BFFF989  bl 0x8240cb90
	ctx.lr = 0x8240D20C;
	sub_8240CB90(ctx, base);
	// 8240D20C: FF01E800  fcmpu cr6, f1, f29
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[29].f64);
	// 8240D210: 4198000C  blt cr6, 0x8240d21c
	if ctx.cr[6].lt {
	pc = 0x8240D21C; continue 'dispatch;
	}
	// 8240D214: FFE0E890  fmr f31, f29
	ctx.f[31].f64 = ctx.f[29].f64;
	// 8240D218: 48000020  b 0x8240d238
	pc = 0x8240D238; continue 'dispatch;
            }
            0x8240D21C => {
    //   block [0x8240D21C..0x8240D22C)
	// 8240D21C: FF01D000  fcmpu cr6, f1, f26
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[26].f64);
	// 8240D220: 4199000C  bgt cr6, 0x8240d22c
	if ctx.cr[6].gt {
	pc = 0x8240D22C; continue 'dispatch;
	}
	// 8240D224: FFE0D090  fmr f31, f26
	ctx.f[31].f64 = ctx.f[26].f64;
	// 8240D228: 48000010  b 0x8240d238
	pc = 0x8240D238; continue 'dispatch;
            }
            0x8240D22C => {
    //   block [0x8240D22C..0x8240D238)
	// 8240D22C: EC210672  fmuls f1, f1, f25
	ctx.f[1].f64 = (((ctx.f[1].f64 * ctx.f[25].f64) as f32) as f64);
	// 8240D230: 48002A11  bl 0x8240fc40
	ctx.lr = 0x8240D234;
	sub_8240FC40(ctx, base);
	// 8240D234: FFE00890  fmr f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].f64 = ctx.f[1].f64;
	pc = 0x8240D238; continue 'dispatch;
            }
            0x8240D238 => {
    //   block [0x8240D238..0x8240D25C)
	// 8240D238: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8240D23C: FC20F090  fmr f1, f30
	ctx.f[1].f64 = ctx.f[30].f64;
	// 8240D240: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8240D244: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8240D248: 4BFFF949  bl 0x8240cb90
	ctx.lr = 0x8240D24C;
	sub_8240CB90(ctx, base);
	// 8240D24C: FF01E800  fcmpu cr6, f1, f29
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[29].f64);
	// 8240D250: 4198000C  blt cr6, 0x8240d25c
	if ctx.cr[6].lt {
	pc = 0x8240D25C; continue 'dispatch;
	}
	// 8240D254: FC20E890  fmr f1, f29
	ctx.f[1].f64 = ctx.f[29].f64;
	// 8240D258: 4800001C  b 0x8240d274
	pc = 0x8240D274; continue 'dispatch;
            }
            0x8240D25C => {
    //   block [0x8240D25C..0x8240D26C)
	// 8240D25C: FF01D000  fcmpu cr6, f1, f26
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[26].f64);
	// 8240D260: 4199000C  bgt cr6, 0x8240d26c
	if ctx.cr[6].gt {
	pc = 0x8240D26C; continue 'dispatch;
	}
	// 8240D264: FC20D090  fmr f1, f26
	ctx.f[1].f64 = ctx.f[26].f64;
	// 8240D268: 4800000C  b 0x8240d274
	pc = 0x8240D274; continue 'dispatch;
            }
            0x8240D26C => {
    //   block [0x8240D26C..0x8240D274)
	// 8240D26C: EC210672  fmuls f1, f1, f25
	ctx.f[1].f64 = (((ctx.f[1].f64 * ctx.f[25].f64) as f32) as f64);
	// 8240D270: 480029D1  bl 0x8240fc40
	ctx.lr = 0x8240D274;
	sub_8240FC40(ctx, base);
	pc = 0x8240D274; continue 'dispatch;
            }
            0x8240D274 => {
    //   block [0x8240D274..0x8240D28C)
	// 8240D274: EC1F0732  fmuls f0, f31, f28
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = (((ctx.f[31].f64 * ctx.f[28].f64) as f32) as f64);
	// 8240D278: D01E0000  stfs f0, 0(r30)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 8240D27C: EC010732  fmuls f0, f1, f28
	ctx.f[0].f64 = (((ctx.f[1].f64 * ctx.f[28].f64) as f32) as f64);
	// 8240D280: D01E0004  stfs f0, 4(r30)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 8240D284: EC1F06F2  fmuls f0, f31, f27
	ctx.f[0].f64 = (((ctx.f[31].f64 * ctx.f[27].f64) as f32) as f64);
	// 8240D288: D01E0010  stfs f0, 0x10(r30)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), tmp.u32 ) };
	pc = 0x8240D28C; continue 'dispatch;
            }
            0x8240D28C => {
    //   block [0x8240D28C..0x8240D294)
	// 8240D28C: EC0106F2  fmuls f0, f1, f27
	ctx.f[0].f64 = (((ctx.f[1].f64 * ctx.f[27].f64) as f32) as f64);
	// 8240D290: D01E0014  stfs f0, 0x14(r30)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), tmp.u32 ) };
	pc = 0x8240D294; continue 'dispatch;
            }
            0x8240D294 => {
    //   block [0x8240D294..0x8240D2EC)
	// 8240D294: 4BFF80FD  bl 0x82405390
	ctx.lr = 0x8240D298;
	sub_82405390(ctx, base);
	// 8240D298: 817F0228  lwz r11, 0x228(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(552 as u32) ) } as u64;
	// 8240D29C: FFC00890  fmr f30, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[30].f64 = ctx.f[1].f64;
	// 8240D2A0: 394B000D  addi r10, r11, 0xd
	ctx.r[10].s64 = ctx.r[11].s64 + 13;
	// 8240D2A4: 1D6B0018  mulli r11, r11, 0x18
	ctx.r[11].s32 = ((ctx.r[11].s32 as i64 * 24 as i64) as i32);
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 8240D2A8: 1D4A0018  mulli r10, r10, 0x18
	ctx.r[10].s32 = ((ctx.r[10].s32 as i64 * 24 as i64) as i32);
	ctx.r[10].s64 = ctx.r[10].s32 as i64;
	// 8240D2AC: 7D4AF82E  lwzx r10, r10, r31
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 8240D2B0: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 8240D2B4: 816B0148  lwz r11, 0x148(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(328 as u32) ) } as u64;
	// 8240D2B8: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8240D2BC: 409A0044  bne cr6, 0x8240d300
	if !ctx.cr[6].eq {
	pc = 0x8240D300; continue 'dispatch;
	}
	// 8240D2C0: C03E0010  lfs f1, 0x10(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8240D2C4: 4800124D  bl 0x8240e510
	ctx.lr = 0x8240D2C8;
	sub_8240E510(ctx, base);
	// 8240D2C8: FFE00890  fmr f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].f64 = ctx.f[1].f64;
	// 8240D2CC: 4BFF80ED  bl 0x824053b8
	ctx.lr = 0x8240D2D0;
	sub_824053B8(ctx, base);
	// 8240D2D0: FC000890  fmr f0, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = ctx.f[1].f64;
	// 8240D2D4: C03E0000  lfs f1, 0(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8240D2D8: EFFF002A  fadds f31, f31, f0
	ctx.f[31].f64 = ((ctx.f[31].f64 + ctx.f[0].f64) as f32) as f64;
	// 8240D2DC: 48001235  bl 0x8240e510
	ctx.lr = 0x8240D2E0;
	sub_8240E510(ctx, base);
	// 8240D2E0: FF1FF000  fcmpu cr6, f31, f30
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[30].f64);
	// 8240D2E4: 40980008  bge cr6, 0x8240d2ec
	if !ctx.cr[6].lt {
	pc = 0x8240D2EC; continue 'dispatch;
	}
	// 8240D2E8: FFE0F090  fmr f31, f30
	ctx.f[31].f64 = ctx.f[30].f64;
	pc = 0x8240D2EC; continue 'dispatch;
            }
            0x8240D2EC => {
    //   block [0x8240D2EC..0x8240D300)
	// 8240D2EC: FC40F890  fmr f2, f31
	ctx.f[2].f64 = ctx.f[31].f64;
	// 8240D2F0: 480012C1  bl 0x8240e5b0
	ctx.lr = 0x8240D2F4;
	sub_8240E5B0(ctx, base);
	// 8240D2F4: 48001265  bl 0x8240e558
	ctx.lr = 0x8240D2F8;
	sub_8240E558(ctx, base);
	// 8240D2F8: D03E0000  stfs f1, 0(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 8240D2FC: D35E0010  stfs f26, 0x10(r30)
	tmp.f32 = (ctx.f[26].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), tmp.u32 ) };
	pc = 0x8240D300; continue 'dispatch;
            }
            0x8240D300 => {
    //   block [0x8240D300..0x8240D348)
	// 8240D300: 817F0228  lwz r11, 0x228(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(552 as u32) ) } as u64;
	// 8240D304: 1D6B0018  mulli r11, r11, 0x18
	ctx.r[11].s32 = ((ctx.r[11].s32 as i64 * 24 as i64) as i32);
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 8240D308: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 8240D30C: 814B013C  lwz r10, 0x13c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(316 as u32) ) } as u64;
	// 8240D310: 816B014C  lwz r11, 0x14c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(332 as u32) ) } as u64;
	// 8240D314: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8240D318: 409A0044  bne cr6, 0x8240d35c
	if !ctx.cr[6].eq {
	pc = 0x8240D35C; continue 'dispatch;
	}
	// 8240D31C: C03E0014  lfs f1, 0x14(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8240D320: 480011F1  bl 0x8240e510
	ctx.lr = 0x8240D324;
	sub_8240E510(ctx, base);
	// 8240D324: FFE00890  fmr f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].f64 = ctx.f[1].f64;
	// 8240D328: 4BFF8091  bl 0x824053b8
	ctx.lr = 0x8240D32C;
	sub_824053B8(ctx, base);
	// 8240D32C: FC000890  fmr f0, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = ctx.f[1].f64;
	// 8240D330: C03E0004  lfs f1, 4(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8240D334: EFFF002A  fadds f31, f31, f0
	ctx.f[31].f64 = ((ctx.f[31].f64 + ctx.f[0].f64) as f32) as f64;
	// 8240D338: 480011D9  bl 0x8240e510
	ctx.lr = 0x8240D33C;
	sub_8240E510(ctx, base);
	// 8240D33C: FF1FF000  fcmpu cr6, f31, f30
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[30].f64);
	// 8240D340: 40980008  bge cr6, 0x8240d348
	if !ctx.cr[6].lt {
	pc = 0x8240D348; continue 'dispatch;
	}
	// 8240D344: FFE0F090  fmr f31, f30
	ctx.f[31].f64 = ctx.f[30].f64;
	pc = 0x8240D348; continue 'dispatch;
            }
            0x8240D348 => {
    //   block [0x8240D348..0x8240D35C)
	// 8240D348: FC40F890  fmr f2, f31
	ctx.f[2].f64 = ctx.f[31].f64;
	// 8240D34C: 48001265  bl 0x8240e5b0
	ctx.lr = 0x8240D350;
	sub_8240E5B0(ctx, base);
	// 8240D350: 48001209  bl 0x8240e558
	ctx.lr = 0x8240D354;
	sub_8240E558(ctx, base);
	// 8240D354: D03E0004  stfs f1, 4(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 8240D358: D35E0014  stfs f26, 0x14(r30)
	tmp.f32 = (ctx.f[26].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), tmp.u32 ) };
	pc = 0x8240D35C; continue 'dispatch;
            }
            0x8240D35C => {
    //   block [0x8240D35C..0x8240D3B0)
	// 8240D35C: 817F0228  lwz r11, 0x228(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(552 as u32) ) } as u64;
	// 8240D360: 394B000D  addi r10, r11, 0xd
	ctx.r[10].s64 = ctx.r[11].s64 + 13;
	// 8240D364: 1D6B0018  mulli r11, r11, 0x18
	ctx.r[11].s32 = ((ctx.r[11].s32 as i64 * 24 as i64) as i32);
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 8240D368: 1D4A0018  mulli r10, r10, 0x18
	ctx.r[10].s32 = ((ctx.r[10].s32 as i64 * 24 as i64) as i32);
	ctx.r[10].s64 = ctx.r[10].s32 as i64;
	// 8240D36C: 7D4AF82E  lwzx r10, r10, r31
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 8240D370: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 8240D374: 816B0148  lwz r11, 0x148(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(328 as u32) ) } as u64;
	// 8240D378: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8240D37C: 419A008C  beq cr6, 0x8240d408
	if ctx.cr[6].eq {
	pc = 0x8240D408; continue 'dispatch;
	}
	// 8240D380: FF18D000  fcmpu cr6, f24, f26
	ctx.cr[6].compare_f64(ctx.f[24].f64, ctx.f[26].f64);
	// 8240D384: 40990084  ble cr6, 0x8240d408
	if !ctx.cr[6].gt {
	pc = 0x8240D408; continue 'dispatch;
	}
	// 8240D388: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 8240D38C: 409A007C  bne cr6, 0x8240d408
	if !ctx.cr[6].eq {
	pc = 0x8240D408; continue 'dispatch;
	}
	// 8240D390: C1BE0000  lfs f13, 0(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8240D394: FF18E800  fcmpu cr6, f24, f29
	ctx.cr[6].compare_f64(ctx.f[24].f64, ctx.f[29].f64);
	// 8240D398: EDAD0372  fmuls f13, f13, f13
	ctx.f[13].f64 = (((ctx.f[13].f64 * ctx.f[13].f64) as f32) as f64);
	// 8240D39C: C01E0004  lfs f0, 4(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240D3A0: EFC0683A  fmadds f30, f0, f0, f13
	ctx.f[30].f64 = (((ctx.f[0].f64 * ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64);
	// 8240D3A4: 4198000C  blt cr6, 0x8240d3b0
	if ctx.cr[6].lt {
	pc = 0x8240D3B0; continue 'dispatch;
	}
	// 8240D3A8: FFE0E890  fmr f31, f29
	ctx.f[31].f64 = ctx.f[29].f64;
	// 8240D3AC: 48000018  b 0x8240d3c4
	pc = 0x8240D3C4; continue 'dispatch;
            }
            0x8240D3B0 => {
    //   block [0x8240D3B0..0x8240D3C4)
	// 8240D3B0: EC380672  fmuls f1, f24, f25
	ctx.f[1].f64 = (((ctx.f[24].f64 * ctx.f[25].f64) as f32) as f64);
	// 8240D3B4: 4800288D  bl 0x8240fc40
	ctx.lr = 0x8240D3B8;
	sub_8240FC40(ctx, base);
	// 8240D3B8: FFE00890  fmr f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].f64 = ctx.f[1].f64;
	// 8240D3BC: FF1FE800  fcmpu cr6, f31, f29
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[29].f64);
	// 8240D3C0: 4198000C  blt cr6, 0x8240d3cc
	if ctx.cr[6].lt {
	pc = 0x8240D3CC; continue 'dispatch;
	}
	pc = 0x8240D3C4; continue 'dispatch;
            }
            0x8240D3C4 => {
    //   block [0x8240D3C4..0x8240D3CC)
	// 8240D3C4: FC20D090  fmr f1, f26
	ctx.f[1].f64 = ctx.f[26].f64;
	// 8240D3C8: 4800001C  b 0x8240d3e4
	pc = 0x8240D3E4; continue 'dispatch;
            }
            0x8240D3CC => {
    //   block [0x8240D3CC..0x8240D3DC)
	// 8240D3CC: FF1FD000  fcmpu cr6, f31, f26
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[26].f64);
	// 8240D3D0: 4199000C  bgt cr6, 0x8240d3dc
	if ctx.cr[6].gt {
	pc = 0x8240D3DC; continue 'dispatch;
	}
	// 8240D3D4: FC20E890  fmr f1, f29
	ctx.f[1].f64 = ctx.f[29].f64;
	// 8240D3D8: 4800000C  b 0x8240d3e4
	pc = 0x8240D3E4; continue 'dispatch;
            }
            0x8240D3DC => {
    //   block [0x8240D3DC..0x8240D3E4)
	// 8240D3DC: EC3FEFFC  fnmsubs f1, f31, f31, f29
	ctx.f[1].f64 = -(((ctx.f[31].f64 * ctx.f[31].f64 - ctx.f[29].f64) as f32) as f64);
	// 8240D3E0: 48002AD9  bl 0x8240feb8
	ctx.lr = 0x8240D3E4;
	sub_8240FEB8(ctx, base);
	pc = 0x8240D3E4; continue 'dispatch;
            }
            0x8240D3E4 => {
    //   block [0x8240D3E4..0x8240D408)
	// 8240D3E4: C01E0000  lfs f0, 0(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240D3E8: EDBE07F2  fmuls f13, f30, f31
	ctx.f[13].f64 = (((ctx.f[30].f64 * ctx.f[31].f64) as f32) as f64);
	// 8240D3EC: C19E0004  lfs f12, 4(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 8240D3F0: EC000072  fmuls f0, f0, f1
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[1].f64) as f32) as f64);
	// 8240D3F4: ED8C0072  fmuls f12, f12, f1
	ctx.f[12].f64 = (((ctx.f[12].f64 * ctx.f[1].f64) as f32) as f64);
	// 8240D3F8: D01E0000  stfs f0, 0(r30)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 8240D3FC: D19E0004  stfs f12, 4(r30)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 8240D400: D1BE0008  stfs f13, 8(r30)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 8240D404: 48000008  b 0x8240d40c
	pc = 0x8240D40C; continue 'dispatch;
            }
            0x8240D408 => {
    //   block [0x8240D408..0x8240D40C)
	// 8240D408: D35E0008  stfs f26, 8(r30)
	tmp.f32 = (ctx.f[26].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), tmp.u32 ) };
	pc = 0x8240D40C; continue 'dispatch;
            }
            0x8240D40C => {
    //   block [0x8240D40C..0x8240D410)
	// 8240D40C: 39600006  li r11, 6
	ctx.r[11].s64 = 6;
	pc = 0x8240D410; continue 'dispatch;
            }
            0x8240D410 => {
    //   block [0x8240D410..0x8240D420)
	// 8240D410: C01E0000  lfs f0, 0(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240D414: FF00D000  fcmpu cr6, f0, f26
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[26].f64);
	// 8240D418: 40980008  bge cr6, 0x8240d420
	if !ctx.cr[6].lt {
	pc = 0x8240D420; continue 'dispatch;
	}
	// 8240D41C: D35E0000  stfs f26, 0(r30)
	tmp.f32 = (ctx.f[26].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), tmp.u32 ) };
	pc = 0x8240D420; continue 'dispatch;
            }
            0x8240D420 => {
    //   block [0x8240D420..0x8240D42C)
	// 8240D420: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8240D424: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 8240D428: 4082FFE8  bne 0x8240d410
	if !ctx.cr[0].eq {
	pc = 0x8240D410; continue 'dispatch;
	}
	pc = 0x8240D42C; continue 'dispatch;
            }
            0x8240D42C => {
    //   block [0x8240D42C..0x8240D43C)
	// 8240D42C: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 8240D430: 3981FFE0  addi r12, r1, -0x20
	ctx.r[12].s64 = ctx.r[1].s64 + -32;
	// 8240D434: 48128BF1  bl 0x82536024
	ctx.lr = 0x8240D438;
	sub_82535FFC(ctx, base);
	// 8240D438: 48127CD4  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240D440(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8240D440 size=48
    let mut pc: u32 = 0x8240D440;
    'dispatch: loop {
        match pc {
            0x8240D440 => {
    //   block [0x8240D440..0x8240D470)
	// 8240D440: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240D444: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240D448: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8240D44C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240D450: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8240D454: 4BFFF815  bl 0x8240cc68
	ctx.lr = 0x8240D458;
	sub_8240CC68(ctx, base);
	// 8240D458: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8240D45C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8240D460: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240D464: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240D468: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8240D46C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240D470(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8240D470 size=304
    let mut pc: u32 = 0x8240D470;
    'dispatch: loop {
        match pc {
            0x8240D470 => {
    //   block [0x8240D470..0x8240D508)
	// 8240D470: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240D474: 48127C3D  bl 0x825350b0
	ctx.lr = 0x8240D478;
	sub_82535080(ctx, base);
	// 8240D478: 3981FFC8  addi r12, r1, -0x38
	ctx.r[12].s64 = ctx.r[1].s64 + -56;
	// 8240D47C: 48128B61  bl 0x82535fdc
	ctx.lr = 0x8240D480;
	sub_82535FB0(ctx, base);
	// 8240D480: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240D484: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8240D488: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8240D48C: 3C808201  lis r4, -0x7dff
	ctx.r[4].s64 = -2113863680;
	// 8240D490: 3CA0820D  lis r5, -0x7df3
	ctx.r[5].s64 = -2113077248;
	// 8240D494: 3CC0820D  lis r6, -0x7df3
	ctx.r[6].s64 = -2113077248;
	// 8240D498: C00B1850  lfs f0, 0x1850(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(6224 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240D49C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8240D4A0: 3CE08201  lis r7, -0x7dff
	ctx.r[7].s64 = -2113863680;
	// 8240D4A4: D01F0000  stfs f0, 0(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 8240D4A8: 3D008200  lis r8, -0x7e00
	ctx.r[8].s64 = -2113929216;
	// 8240D4AC: C344C808  lfs f26, -0x37f8(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(-14328 as u32) ) };
	ctx.f[26].f64 = (tmp.f32 as f64);
	// 8240D4B0: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 8240D4B4: C3651FF8  lfs f27, 0x1ff8(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(8184 as u32) ) };
	ctx.f[27].f64 = (tmp.f32 as f64);
	// 8240D4B8: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 8240D4BC: C3862074  lfs f28, 0x2074(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(8308 as u32) ) };
	ctx.f[28].f64 = (tmp.f32 as f64);
	// 8240D4C0: C1AB2068  lfs f13, 0x2068(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8296 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8240D4C4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240D4C8: D1BF0004  stfs f13, 4(r31)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 8240D4CC: 3B9F0130  addi r28, r31, 0x130
	ctx.r[28].s64 = ctx.r[31].s64 + 304;
	// 8240D4D0: C3C876F4  lfs f30, 0x76f4(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(30452 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 8240D4D4: 3B400020  li r26, 0x20
	ctx.r[26].s64 = 32;
	// 8240D4D8: C3E9294C  lfs f31, 0x294c(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(10572 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 8240D4DC: C32A2140  lfs f25, 0x2140(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8512 as u32) ) };
	ctx.f[25].f64 = (tmp.f32 as f64);
	// 8240D4E0: C18BE35C  lfs f12, -0x1ca4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-7332 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 8240D4E4: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8240D4E8: C3A7E360  lfs f29, -0x1ca0(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(-7328 as u32) ) };
	ctx.f[29].f64 = (tmp.f32 as f64);
	// 8240D4EC: D19F0008  stfs f12, 8(r31)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 8240D4F0: D01F000C  stfs f0, 0xc(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 8240D4F4: D01F0010  stfs f0, 0x10(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 8240D4F8: C1AB184C  lfs f13, 0x184c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(6220 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8240D4FC: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 8240D500: D1BF0014  stfs f13, 0x14(r31)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 8240D504: 3B6B3808  addi r27, r11, 0x3808
	ctx.r[27].s64 = ctx.r[11].s64 + 14344;
	pc = 0x8240D508; continue 'dispatch;
            }
            0x8240D508 => {
    //   block [0x8240D508..0x8240D510)
	// 8240D508: 3BBCFEEC  addi r29, r28, -0x114
	ctx.r[29].s64 = ctx.r[28].s64 + -276;
	// 8240D50C: 3BDB0004  addi r30, r27, 4
	ctx.r[30].s64 = ctx.r[27].s64 + 4;
	pc = 0x8240D510; continue 'dispatch;
            }
            0x8240D510 => {
    //   block [0x8240D510..0x8240D5A0)
	// 8240D510: C01EFFFC  lfs f0, -4(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240D514: D01DFFFC  stfs f0, -4(r29)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(-4 as u32), tmp.u32 ) };
	// 8240D518: C03E0000  lfs f1, 0(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8240D51C: 4800103D  bl 0x8240e558
	ctx.lr = 0x8240D520;
	sub_8240E558(ctx, base);
	// 8240D520: D03D0000  stfs f1, 0(r29)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 8240D524: C01E0004  lfs f0, 4(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240D528: EC00C82A  fadds f0, f0, f25
	ctx.f[0].f64 = ((ctx.f[0].f64 + ctx.f[25].f64) as f32) as f64;
	// 8240D52C: D01D0004  stfs f0, 4(r29)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 8240D530: C03E0008  lfs f1, 8(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8240D534: 48001025  bl 0x8240e558
	ctx.lr = 0x8240D538;
	sub_8240E558(ctx, base);
	// 8240D538: D03D0008  stfs f1, 8(r29)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 8240D53C: 397B011C  addi r11, r27, 0x11c
	ctx.r[11].s64 = ctx.r[27].s64 + 284;
	// 8240D540: C01E000C  lfs f0, 0xc(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240D544: D01D000C  stfs f0, 0xc(r29)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 8240D548: C01E0010  lfs f0, 0x10(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240D54C: D01D0010  stfs f0, 0x10(r29)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 8240D550: C01E0014  lfs f0, 0x14(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240D554: 3BDE001C  addi r30, r30, 0x1c
	ctx.r[30].s64 = ctx.r[30].s64 + 28;
	// 8240D558: D01D0014  stfs f0, 0x14(r29)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 8240D55C: 3BBD001C  addi r29, r29, 0x1c
	ctx.r[29].s64 = ctx.r[29].s64 + 28;
	// 8240D560: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8240D564: 4198FFAC  blt cr6, 0x8240d510
	if ctx.cr[6].lt {
	pc = 0x8240D510; continue 'dispatch;
	}
	// 8240D568: D3FC0000  stfs f31, 0(r28)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 8240D56C: 375AFFFF  addic. r26, r26, -1
	ctx.xer.ca = (ctx.r[26].u32 > (!(-1 as u32)));
	ctx.r[26].s64 = ctx.r[26].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[26].s32, 0, &mut ctx.xer);
	// 8240D570: D3DC0004  stfs f30, 4(r28)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 8240D574: D3BC0008  stfs f29, 8(r28)
	tmp.f32 = (ctx.f[29].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 8240D578: D39C000C  stfs f28, 0xc(r28)
	tmp.f32 = (ctx.f[28].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 8240D57C: D37C0010  stfs f27, 0x10(r28)
	tmp.f32 = (ctx.f[27].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 8240D580: D35C0014  stfs f26, 0x14(r28)
	tmp.f32 = (ctx.f[26].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 8240D584: 3B9C0130  addi r28, r28, 0x130
	ctx.r[28].s64 = ctx.r[28].s64 + 304;
	// 8240D588: 4082FF80  bne 0x8240d508
	if !ctx.cr[0].eq {
	pc = 0x8240D508; continue 'dispatch;
	}
	// 8240D58C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8240D590: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 8240D594: 3981FFC8  addi r12, r1, -0x38
	ctx.r[12].s64 = ctx.r[1].s64 + -56;
	// 8240D598: 48128A91  bl 0x82536028
	ctx.lr = 0x8240D59C;
	sub_82535FFC(ctx, base);
	// 8240D59C: 48127B64  b 0x82535100
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240D5A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8240D5A0 size=236
    let mut pc: u32 = 0x8240D5A0;
    'dispatch: loop {
        match pc {
            0x8240D5A0 => {
    //   block [0x8240D5A0..0x8240D5E0)
	// 8240D5A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240D5A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240D5A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240D5AC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8240D5B0: 2B04001F  cmplwi cr6, r4, 0x1f
	ctx.cr[6].compare_u32(ctx.r[4].u32, 31 as u32, &mut ctx.xer);
	// 8240D5B4: C16B1FF8  lfs f11, 0x1ff8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 8240D5B8: D1670000  stfs f11, 0(r7)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 8240D5BC: 419900AC  bgt cr6, 0x8240d668
	if ctx.cr[6].gt {
	pc = 0x8240D668; continue 'dispatch;
	}
	// 8240D5C0: 2F060000  cmpwi cr6, r6, 0
	ctx.cr[6].compare_i32(ctx.r[6].s32, 0, &mut ctx.xer);
	// 8240D5C4: 4098001C  bge cr6, 0x8240d5e0
	if !ctx.cr[6].lt {
	pc = 0x8240D5E0; continue 'dispatch;
	}
	// 8240D5C8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240D5CC: 7CC43378  mr r4, r6
	ctx.r[4].u64 = ctx.r[6].u64;
	// 8240D5D0: 386BE3B8  addi r3, r11, -0x1c48
	ctx.r[3].s64 = ctx.r[11].s64 + -7240;
	// 8240D5D4: 4BEA59AD  bl 0x822b2f80
	ctx.lr = 0x8240D5D8;
	sub_822B2F80(ctx, base);
	// 8240D5D8: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 8240D5DC: 480000A0  b 0x8240d67c
	pc = 0x8240D67C; continue 'dispatch;
            }
            0x8240D5E0 => {
    //   block [0x8240D5E0..0x8240D620)
	// 8240D5E0: 1D640130  mulli r11, r4, 0x130
	ctx.r[11].s32 = ((ctx.r[4].s32 as i64 * 304 as i64) as i32);
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 8240D5E4: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 8240D5E8: 2F060000  cmpwi cr6, r6, 0
	ctx.cr[6].compare_i32(ctx.r[6].s32, 0, &mut ctx.xer);
	// 8240D5EC: 396B0018  addi r11, r11, 0x18
	ctx.r[11].s64 = ctx.r[11].s64 + 24;
	// 8240D5F0: 419A0054  beq cr6, 0x8240d644
	if ctx.cr[6].eq {
	pc = 0x8240D644; continue 'dispatch;
	}
	// 8240D5F4: 2F06000A  cmpwi cr6, r6, 0xa
	ctx.cr[6].compare_i32(ctx.r[6].s32, 10, &mut ctx.xer);
	// 8240D5F8: 419A0048  beq cr6, 0x8240d640
	if ctx.cr[6].eq {
	pc = 0x8240D640; continue 'dispatch;
	}
	// 8240D5FC: 1D46001C  mulli r10, r6, 0x1c
	ctx.r[10].s32 = ((ctx.r[6].s32 as i64 * 28 as i64) as i32);
	ctx.r[10].s64 = ctx.r[10].s32 as i64;
	// 8240D600: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 8240D604: C1AB0000  lfs f13, 0(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8240D608: C00BFFE4  lfs f0, -0x1c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-28 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240D60C: C18BFFE8  lfs f12, -0x18(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-24 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 8240D610: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 8240D614: 409A000C  bne cr6, 0x8240d620
	if !ctx.cr[6].eq {
	pc = 0x8240D620; continue 'dispatch;
	}
	// 8240D618: FC006890  fmr f0, f13
	ctx.f[0].f64 = ctx.f[13].f64;
	// 8240D61C: 48000034  b 0x8240d650
	pc = 0x8240D650; continue 'dispatch;
            }
            0x8240D620 => {
    //   block [0x8240D620..0x8240D640)
	// 8240D620: C14B0004  lfs f10, 4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 8240D624: ED210028  fsubs f9, f1, f0
	ctx.f[9].f64 = (((ctx.f[1].f64 - ctx.f[0].f64) as f32) as f64);
	// 8240D628: ED4A6028  fsubs f10, f10, f12
	ctx.f[10].f64 = (((ctx.f[10].f64 - ctx.f[12].f64) as f32) as f64);
	// 8240D62C: EC0D0028  fsubs f0, f13, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 - ctx.f[0].f64) as f32) as f64);
	// 8240D630: EDAA0272  fmuls f13, f10, f9
	ctx.f[13].f64 = (((ctx.f[10].f64 * ctx.f[9].f64) as f32) as f64);
	// 8240D634: EC0D0024  fdivs f0, f13, f0
	ctx.f[0].f64 = ((ctx.f[13].f64 / ctx.f[0].f64) as f32) as f64;
	// 8240D638: EC00602A  fadds f0, f0, f12
	ctx.f[0].f64 = ((ctx.f[0].f64 + ctx.f[12].f64) as f32) as f64;
	// 8240D63C: 48000014  b 0x8240d650
	pc = 0x8240D650; continue 'dispatch;
            }
            0x8240D640 => {
    //   block [0x8240D640..0x8240D644)
	// 8240D640: 38C00009  li r6, 9
	ctx.r[6].s64 = 9;
	pc = 0x8240D644; continue 'dispatch;
            }
            0x8240D644 => {
    //   block [0x8240D644..0x8240D650)
	// 8240D644: 1D46001C  mulli r10, r6, 0x1c
	ctx.r[10].s32 = ((ctx.r[6].s32 as i64 * 28 as i64) as i32);
	ctx.r[10].s64 = ctx.r[10].s32 as i64;
	// 8240D648: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 8240D64C: C00B0004  lfs f0, 4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	pc = 0x8240D650; continue 'dispatch;
            }
            0x8240D650 => {
    //   block [0x8240D650..0x8240D65C)
	// 8240D650: FF005800  fcmpu cr6, f0, f11
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[11].f64);
	// 8240D654: 40980008  bge cr6, 0x8240d65c
	if !ctx.cr[6].lt {
	pc = 0x8240D65C; continue 'dispatch;
	}
	// 8240D658: FC005890  fmr f0, f11
	ctx.f[0].f64 = ctx.f[11].f64;
	pc = 0x8240D65C; continue 'dispatch;
            }
            0x8240D65C => {
    //   block [0x8240D65C..0x8240D668)
	// 8240D65C: D0070000  stfs f0, 0(r7)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 8240D660: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8240D664: 48000018  b 0x8240d67c
	pc = 0x8240D67C; continue 'dispatch;
            }
            0x8240D668 => {
    //   block [0x8240D668..0x8240D67C)
	// 8240D668: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240D66C: 386BE368  addi r3, r11, -0x1c98
	ctx.r[3].s64 = ctx.r[11].s64 + -7320;
	// 8240D670: 4BEA5911  bl 0x822b2f80
	ctx.lr = 0x8240D674;
	sub_822B2F80(ctx, base);
	// 8240D674: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 8240D678: 60630034  ori r3, r3, 0x34
	ctx.r[3].u64 = ctx.r[3].u64 | 52;
	pc = 0x8240D67C; continue 'dispatch;
            }
            0x8240D67C => {
    //   block [0x8240D67C..0x8240D68C)
	// 8240D67C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8240D680: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240D684: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240D688: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240D690(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8240D690 size=216
    let mut pc: u32 = 0x8240D690;
    'dispatch: loop {
        match pc {
            0x8240D690 => {
    //   block [0x8240D690..0x8240D6E0)
	// 8240D690: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240D694: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240D698: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8240D69C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8240D6A0: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 8240D6A4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240D6A8: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 8240D6AC: 2B04001F  cmplwi cr6, r4, 0x1f
	ctx.cr[6].compare_u32(ctx.r[4].u32, 31 as u32, &mut ctx.xer);
	// 8240D6B0: 41990088  bgt cr6, 0x8240d738
	if ctx.cr[6].gt {
	pc = 0x8240D738; continue 'dispatch;
	}
	// 8240D6B4: 1D640130  mulli r11, r4, 0x130
	ctx.r[11].s32 = ((ctx.r[4].s32 as i64 * 304 as i64) as i32);
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 8240D6B8: 7FEB1A14  add r31, r11, r3
	ctx.r[31].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 8240D6BC: 480027BD  bl 0x8240fe78
	ctx.lr = 0x8240D6C0;
	sub_8240FE78(ctx, base);
	// 8240D6C0: FFE00890  fmr f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].f64 = ctx.f[1].f64;
	// 8240D6C4: C03F0140  lfs f1, 0x140(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(320 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8240D6C8: 480027B1  bl 0x8240fe78
	ctx.lr = 0x8240D6CC;
	sub_8240FE78(ctx, base);
	// 8240D6CC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8240D6D0: C1AB1FF8  lfs f13, 0x1ff8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8240D6D4: FF1F6800  fcmpu cr6, f31, f13
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[13].f64);
	// 8240D6D8: 40980008  bge cr6, 0x8240d6e0
	if !ctx.cr[6].lt {
	pc = 0x8240D6E0; continue 'dispatch;
	}
	// 8240D6DC: FFE0F850  fneg f31, f31
	ctx.f[31].u64 = ctx.f[31].u64 ^ 0x8000_0000_0000_0000u64;
	pc = 0x8240D6E0; continue 'dispatch;
            }
            0x8240D6E0 => {
    //   block [0x8240D6E0..0x8240D6EC)
	// 8240D6E0: FF016800  fcmpu cr6, f1, f13
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[13].f64);
	// 8240D6E4: 40980008  bge cr6, 0x8240d6ec
	if !ctx.cr[6].lt {
	pc = 0x8240D6EC; continue 'dispatch;
	}
	// 8240D6E8: FC200850  fneg f1, f1
	ctx.f[1].u64 = ctx.f[1].u64 ^ 0x8000_0000_0000_0000u64;
	pc = 0x8240D6EC; continue 'dispatch;
            }
            0x8240D6EC => {
    //   block [0x8240D6EC..0x8240D728)
	// 8240D6EC: ED81F82A  fadds f12, f1, f31
	ctx.f[12].f64 = ((ctx.f[1].f64 + ctx.f[31].f64) as f32) as f64;
	// 8240D6F0: C01F0134  lfs f0, 0x134(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(308 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240D6F4: FF0C0000  fcmpu cr6, f12, f0
	ctx.cr[6].compare_f64(ctx.f[12].f64, ctx.f[0].f64);
	// 8240D6F8: 40990034  ble cr6, 0x8240d72c
	if !ctx.cr[6].gt {
	pc = 0x8240D72C; continue 'dispatch;
	}
	// 8240D6FC: C1BF0138  lfs f13, 0x138(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(312 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8240D700: FF0C6800  fcmpu cr6, f12, f13
	ctx.cr[6].compare_f64(ctx.f[12].f64, ctx.f[13].f64);
	// 8240D704: 40980024  bge cr6, 0x8240d728
	if !ctx.cr[6].lt {
	pc = 0x8240D728; continue 'dispatch;
	}
	// 8240D708: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 8240D70C: 419A0020  beq cr6, 0x8240d72c
	if ctx.cr[6].eq {
	pc = 0x8240D72C; continue 'dispatch;
	}
	// 8240D710: ED8C0028  fsubs f12, f12, f0
	ctx.f[12].f64 = (((ctx.f[12].f64 - ctx.f[0].f64) as f32) as f64);
	// 8240D714: C17F013C  lfs f11, 0x13c(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(316 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 8240D718: EC0D0028  fsubs f0, f13, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 - ctx.f[0].f64) as f32) as f64);
	// 8240D71C: EDAC02F2  fmuls f13, f12, f11
	ctx.f[13].f64 = (((ctx.f[12].f64 * ctx.f[11].f64) as f32) as f64);
	// 8240D720: EDAD0024  fdivs f13, f13, f0
	ctx.f[13].f64 = ((ctx.f[13].f64 / ctx.f[0].f64) as f32) as f64;
	// 8240D724: 48000008  b 0x8240d72c
	pc = 0x8240D72C; continue 'dispatch;
            }
            0x8240D728 => {
    //   block [0x8240D728..0x8240D72C)
	// 8240D728: C1BF013C  lfs f13, 0x13c(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(316 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	pc = 0x8240D72C; continue 'dispatch;
            }
            0x8240D72C => {
    //   block [0x8240D72C..0x8240D738)
	// 8240D72C: D1BE0000  stfs f13, 0(r30)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 8240D730: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8240D734: 48000018  b 0x8240d74c
	pc = 0x8240D74C; continue 'dispatch;
            }
            0x8240D738 => {
    //   block [0x8240D738..0x8240D74C)
	// 8240D738: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240D73C: 386BE408  addi r3, r11, -0x1bf8
	ctx.r[3].s64 = ctx.r[11].s64 + -7160;
	// 8240D740: 4BEA5841  bl 0x822b2f80
	ctx.lr = 0x8240D744;
	sub_822B2F80(ctx, base);
	// 8240D744: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 8240D748: 60630034  ori r3, r3, 0x34
	ctx.r[3].u64 = ctx.r[3].u64 | 52;
	pc = 0x8240D74C; continue 'dispatch;
            }
            0x8240D74C => {
    //   block [0x8240D74C..0x8240D768)
	// 8240D74C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8240D750: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240D754: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240D758: CBE1FFE0  lfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 8240D75C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8240D760: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8240D764: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240D768(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8240D768 size=88
    let mut pc: u32 = 0x8240D768;
    'dispatch: loop {
        match pc {
            0x8240D768 => {
    //   block [0x8240D768..0x8240D7A8)
	// 8240D768: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240D76C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240D770: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240D774: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8240D778: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 8240D77C: C00B1FF8  lfs f0, 0x1ff8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240D780: FF010000  fcmpu cr6, f1, f0
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 8240D784: 41990024  bgt cr6, 0x8240d7a8
	if ctx.cr[6].gt {
	pc = 0x8240D7A8; continue 'dispatch;
	}
	// 8240D788: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240D78C: D8210018  stfd f1, 0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(24 as u32), ctx.f[1].u64 ) };
	// 8240D790: E8810018  ld r4, 0x18(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(24 as u32) ) };
	// 8240D794: 386BE458  addi r3, r11, -0x1ba8
	ctx.r[3].s64 = ctx.r[11].s64 + -7080;
	// 8240D798: 4BEA57E9  bl 0x822b2f80
	ctx.lr = 0x8240D79C;
	sub_822B2F80(ctx, base);
	// 8240D79C: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 8240D7A0: 6063003C  ori r3, r3, 0x3c
	ctx.r[3].u64 = ctx.r[3].u64 | 60;
	// 8240D7A4: 4800000C  b 0x8240d7b0
	pc = 0x8240D7B0; continue 'dispatch;
            }
            0x8240D7A8 => {
    //   block [0x8240D7A8..0x8240D7B0)
	// 8240D7A8: D02A000C  stfs f1, 0xc(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 8240D7AC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	pc = 0x8240D7B0; continue 'dispatch;
            }
            0x8240D7B0 => {
    //   block [0x8240D7B0..0x8240D7C0)
	// 8240D7B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8240D7B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240D7B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240D7BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240D7C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8240D7C0 size=88
    let mut pc: u32 = 0x8240D7C0;
    'dispatch: loop {
        match pc {
            0x8240D7C0 => {
    //   block [0x8240D7C0..0x8240D800)
	// 8240D7C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240D7C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240D7C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240D7CC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8240D7D0: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 8240D7D4: C00B1FF8  lfs f0, 0x1ff8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240D7D8: FF010000  fcmpu cr6, f1, f0
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 8240D7DC: 41990024  bgt cr6, 0x8240d800
	if ctx.cr[6].gt {
	pc = 0x8240D800; continue 'dispatch;
	}
	// 8240D7E0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240D7E4: D8210018  stfd f1, 0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(24 as u32), ctx.f[1].u64 ) };
	// 8240D7E8: E8810018  ld r4, 0x18(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(24 as u32) ) };
	// 8240D7EC: 386BE4A0  addi r3, r11, -0x1b60
	ctx.r[3].s64 = ctx.r[11].s64 + -7008;
	// 8240D7F0: 4BEA5791  bl 0x822b2f80
	ctx.lr = 0x8240D7F4;
	sub_822B2F80(ctx, base);
	// 8240D7F4: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 8240D7F8: 6063003D  ori r3, r3, 0x3d
	ctx.r[3].u64 = ctx.r[3].u64 | 61;
	// 8240D7FC: 4800000C  b 0x8240d808
	pc = 0x8240D808; continue 'dispatch;
            }
            0x8240D800 => {
    //   block [0x8240D800..0x8240D808)
	// 8240D800: D02A0008  stfs f1, 8(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 8240D804: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	pc = 0x8240D808; continue 'dispatch;
            }
            0x8240D808 => {
    //   block [0x8240D808..0x8240D818)
	// 8240D808: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8240D80C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240D810: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240D814: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240D818(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8240D818 size=100
    let mut pc: u32 = 0x8240D818;
    'dispatch: loop {
        match pc {
            0x8240D818 => {
    //   block [0x8240D818..0x8240D850)
	// 8240D818: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240D81C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240D820: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240D824: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8240D828: C00B1850  lfs f0, 0x1850(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(6224 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240D82C: FF010000  fcmpu cr6, f1, f0
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 8240D830: 41980020  blt cr6, 0x8240d850
	if ctx.cr[6].lt {
	pc = 0x8240D850; continue 'dispatch;
	}
	// 8240D834: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8240D838: C00B3034  lfs f0, 0x3034(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12340 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240D83C: FF010000  fcmpu cr6, f1, f0
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 8240D840: 41990010  bgt cr6, 0x8240d850
	if ctx.cr[6].gt {
	pc = 0x8240D850; continue 'dispatch;
	}
	// 8240D844: D0230004  stfs f1, 4(r3)
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 8240D848: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8240D84C: 48000020  b 0x8240d86c
	pc = 0x8240D86C; continue 'dispatch;
            }
            0x8240D850 => {
    //   block [0x8240D850..0x8240D86C)
	// 8240D850: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240D854: D8210018  stfd f1, 0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(24 as u32), ctx.f[1].u64 ) };
	// 8240D858: E8810018  ld r4, 0x18(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(24 as u32) ) };
	// 8240D85C: 386BE4E8  addi r3, r11, -0x1b18
	ctx.r[3].s64 = ctx.r[11].s64 + -6936;
	// 8240D860: 4BEA5721  bl 0x822b2f80
	ctx.lr = 0x8240D864;
	sub_822B2F80(ctx, base);
	// 8240D864: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 8240D868: 6063003E  ori r3, r3, 0x3e
	ctx.r[3].u64 = ctx.r[3].u64 | 62;
	pc = 0x8240D86C; continue 'dispatch;
            }
            0x8240D86C => {
    //   block [0x8240D86C..0x8240D87C)
	// 8240D86C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8240D870: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240D874: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240D878: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240D880(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8240D880 size=100
    let mut pc: u32 = 0x8240D880;
    'dispatch: loop {
        match pc {
            0x8240D880 => {
    //   block [0x8240D880..0x8240D8B8)
	// 8240D880: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240D884: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240D888: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240D88C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8240D890: C00B1FF8  lfs f0, 0x1ff8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240D894: FF010000  fcmpu cr6, f1, f0
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 8240D898: 41980020  blt cr6, 0x8240d8b8
	if ctx.cr[6].lt {
	pc = 0x8240D8B8; continue 'dispatch;
	}
	// 8240D89C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240D8A0: C00BE57C  lfs f0, -0x1a84(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-6788 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240D8A4: FF010000  fcmpu cr6, f1, f0
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 8240D8A8: 41990010  bgt cr6, 0x8240d8b8
	if ctx.cr[6].gt {
	pc = 0x8240D8B8; continue 'dispatch;
	}
	// 8240D8AC: D0230010  stfs f1, 0x10(r3)
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 8240D8B0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8240D8B4: 48000020  b 0x8240d8d4
	pc = 0x8240D8D4; continue 'dispatch;
            }
            0x8240D8B8 => {
    //   block [0x8240D8B8..0x8240D8D4)
	// 8240D8B8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240D8BC: D8210018  stfd f1, 0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(24 as u32), ctx.f[1].u64 ) };
	// 8240D8C0: E8810018  ld r4, 0x18(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(24 as u32) ) };
	// 8240D8C4: 386BE530  addi r3, r11, -0x1ad0
	ctx.r[3].s64 = ctx.r[11].s64 + -6864;
	// 8240D8C8: 4BEA56B9  bl 0x822b2f80
	ctx.lr = 0x8240D8CC;
	sub_822B2F80(ctx, base);
	// 8240D8CC: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 8240D8D0: 6063003F  ori r3, r3, 0x3f
	ctx.r[3].u64 = ctx.r[3].u64 | 63;
	pc = 0x8240D8D4; continue 'dispatch;
            }
            0x8240D8D4 => {
    //   block [0x8240D8D4..0x8240D8E4)
	// 8240D8D4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8240D8D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240D8DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240D8E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240D8E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8240D8E8 size=192
    let mut pc: u32 = 0x8240D8E8;
    'dispatch: loop {
        match pc {
            0x8240D8E8 => {
    //   block [0x8240D8E8..0x8240D954)
	// 8240D8E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240D8EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240D8F0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8240D8F4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8240D8F8: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240D8FC: C0040000  lfs f0, 0(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240D900: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8240D904: C1A50000  lfs f13, 0(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8240D908: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 8240D90C: EDA06828  fsubs f13, f0, f13
	ctx.f[13].f64 = (((ctx.f[0].f64 - ctx.f[13].f64) as f32) as f64);
	// 8240D910: C1840004  lfs f12, 4(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 8240D914: C1650004  lfs f11, 4(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 8240D918: 7CFF3B78  mr r31, r7
	ctx.r[31].u64 = ctx.r[7].u64;
	// 8240D91C: D1A10050  stfs f13, 0x50(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 8240D920: C00B1FF8  lfs f0, 0x1ff8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240D924: C1440008  lfs f10, 8(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 8240D928: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 8240D92C: EDAC5828  fsubs f13, f12, f11
	ctx.f[13].f64 = (((ctx.f[12].f64 - ctx.f[11].f64) as f32) as f64);
	// 8240D930: C1850008  lfs f12, 8(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(8 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 8240D934: ED8A6028  fsubs f12, f10, f12
	ctx.f[12].f64 = (((ctx.f[10].f64 - ctx.f[12].f64) as f32) as f64);
	// 8240D938: D1A10054  stfs f13, 0x54(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 8240D93C: D1810058  stfs f12, 0x58(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), tmp.u32 ) };
	// 8240D940: 409A0014  bne cr6, 0x8240d954
	if !ctx.cr[6].eq {
	pc = 0x8240D954; continue 'dispatch;
	}
	// 8240D944: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 8240D948: 409A000C  bne cr6, 0x8240d954
	if !ctx.cr[6].eq {
	pc = 0x8240D954; continue 'dispatch;
	}
	// 8240D94C: FF0C0000  fcmpu cr6, f12, f0
	ctx.cr[6].compare_f64(ctx.f[12].f64, ctx.f[0].f64);
	// 8240D950: 419A0038  beq cr6, 0x8240d988
	if ctx.cr[6].eq {
	pc = 0x8240D988; continue 'dispatch;
	}
	pc = 0x8240D954; continue 'dispatch;
            }
            0x8240D954 => {
    //   block [0x8240D954..0x8240D988)
	// 8240D954: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8240D958: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 8240D95C: 48002305  bl 0x8240fc60
	ctx.lr = 0x8240D960;
	sub_8240FC60(ctx, base);
	// 8240D960: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8240D964: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8240D968: 480022F9  bl 0x8240fc60
	ctx.lr = 0x8240D96C;
	sub_8240FC60(ctx, base);
	// 8240D96C: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 8240D970: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 8240D974: 480022FD  bl 0x8240fc70
	ctx.lr = 0x8240D978;
	sub_8240FC70(ctx, base);
	// 8240D978: 480022D9  bl 0x8240fc50
	ctx.lr = 0x8240D97C;
	sub_8240FC50(ctx, base);
	// 8240D97C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240D980: C00BE580  lfs f0, -0x1a80(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-6784 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240D984: EC010032  fmuls f0, f1, f0
	ctx.f[0].f64 = (((ctx.f[1].f64 * ctx.f[0].f64) as f32) as f64);
	pc = 0x8240D988; continue 'dispatch;
            }
            0x8240D988 => {
    //   block [0x8240D988..0x8240D9A8)
	// 8240D988: D01F0000  stfs f0, 0(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 8240D98C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8240D990: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 8240D994: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240D998: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240D99C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8240D9A0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8240D9A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240D9A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8240D9A8 size=312
    let mut pc: u32 = 0x8240D9A8;
    'dispatch: loop {
        match pc {
            0x8240D9A8 => {
    //   block [0x8240D9A8..0x8240D9F8)
	// 8240D9A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240D9AC: 4812770D  bl 0x825350b8
	ctx.lr = 0x8240D9B0;
	sub_82535080(ctx, base);
	// 8240D9B0: DBE1FFD0  stfd f31, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[31].u64 ) };
	// 8240D9B4: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240D9B8: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 8240D9BC: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 8240D9C0: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 8240D9C4: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 8240D9C8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8240D9CC: 419A00F4  beq cr6, 0x8240dac0
	if ctx.cr[6].eq {
	pc = 0x8240DAC0; continue 'dispatch;
	}
	// 8240D9D0: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 8240D9D4: 419A00EC  beq cr6, 0x8240dac0
	if ctx.cr[6].eq {
	pc = 0x8240DAC0; continue 'dispatch;
	}
	// 8240D9D8: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 8240D9DC: 409A001C  bne cr6, 0x8240d9f8
	if !ctx.cr[6].eq {
	pc = 0x8240D9F8; continue 'dispatch;
	}
	// 8240D9E0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240D9E4: 386BE5D0  addi r3, r11, -0x1a30
	ctx.r[3].s64 = ctx.r[11].s64 + -6704;
	// 8240D9E8: 4BEA5599  bl 0x822b2f80
	ctx.lr = 0x8240D9EC;
	sub_822B2F80(ctx, base);
	// 8240D9EC: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 8240D9F0: 60630033  ori r3, r3, 0x33
	ctx.r[3].u64 = ctx.r[3].u64 | 51;
	// 8240D9F4: 480000E0  b 0x8240dad4
	pc = 0x8240DAD4; continue 'dispatch;
            }
            0x8240D9F8 => {
    //   block [0x8240D9F8..0x8240DA60)
	// 8240D9F8: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8240D9FC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8240DA00: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8240DA04: 48002255  bl 0x8240fc58
	ctx.lr = 0x8240DA08;
	sub_8240FC58(ctx, base);
	// 8240DA08: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8240DA0C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8240DA10: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8240DA14: 48002245  bl 0x8240fc58
	ctx.lr = 0x8240DA18;
	sub_8240FC58(ctx, base);
	// 8240DA18: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8240DA1C: 4800225D  bl 0x8240fc78
	ctx.lr = 0x8240DA20;
	sub_8240FC78(ctx, base);
	// 8240DA20: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 8240DA24: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8240DA28: FFE00890  fmr f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].f64 = ctx.f[1].f64;
	// 8240DA2C: 48002245  bl 0x8240fc70
	ctx.lr = 0x8240DA30;
	sub_8240FC70(ctx, base);
	// 8240DA30: EC1F07F2  fmuls f0, f31, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = (((ctx.f[31].f64 * ctx.f[31].f64) as f32) as f64);
	// 8240DA34: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8240DA38: C1AB1FF8  lfs f13, 0x1ff8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8240DA3C: EC010024  fdivs f0, f1, f0
	ctx.f[0].f64 = ((ctx.f[1].f64 / ctx.f[0].f64) as f32) as f64;
	// 8240DA40: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 8240DA44: 4098001C  bge cr6, 0x8240da60
	if !ctx.cr[6].lt {
	pc = 0x8240DA60; continue 'dispatch;
	}
	// 8240DA48: C01F0000  lfs f0, 0(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240DA4C: D01E0000  stfs f0, 0(r30)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 8240DA50: C01F0004  lfs f0, 4(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240DA54: D01E0004  stfs f0, 4(r30)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 8240DA58: C01F0008  lfs f0, 8(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240DA5C: 48000058  b 0x8240dab4
	pc = 0x8240DAB4; continue 'dispatch;
            }
            0x8240DA60 => {
    //   block [0x8240DA60..0x8240DA88)
	// 8240DA60: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8240DA64: C1AB1850  lfs f13, 0x1850(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(6224 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8240DA68: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 8240DA6C: 4099001C  ble cr6, 0x8240da88
	if !ctx.cr[6].gt {
	pc = 0x8240DA88; continue 'dispatch;
	}
	// 8240DA70: C01D0000  lfs f0, 0(r29)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240DA74: D01E0000  stfs f0, 0(r30)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 8240DA78: C01D0004  lfs f0, 4(r29)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240DA7C: D01E0004  stfs f0, 4(r30)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 8240DA80: C01D0008  lfs f0, 8(r29)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240DA84: 48000030  b 0x8240dab4
	pc = 0x8240DAB4; continue 'dispatch;
            }
            0x8240DA88 => {
    //   block [0x8240DA88..0x8240DAB4)
	// 8240DA88: C19F0000  lfs f12, 0(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 8240DA8C: C1A10050  lfs f13, 0x50(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8240DA90: EDAD603A  fmadds f13, f13, f0, f12
	ctx.f[13].f64 = (((ctx.f[13].f64 * ctx.f[0].f64 + ctx.f[12].f64) as f32) as f64);
	// 8240DA94: D1BE0000  stfs f13, 0(r30)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 8240DA98: C1BF0004  lfs f13, 4(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8240DA9C: C1610054  lfs f11, 0x54(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 8240DAA0: EDAB683A  fmadds f13, f11, f0, f13
	ctx.f[13].f64 = (((ctx.f[11].f64 * ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64);
	// 8240DAA4: D1BE0004  stfs f13, 4(r30)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 8240DAA8: C1410058  lfs f10, 0x58(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 8240DAAC: C1BF0008  lfs f13, 8(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8240DAB0: EC0A683A  fmadds f0, f10, f0, f13
	ctx.f[0].f64 = (((ctx.f[10].f64 * ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64);
	pc = 0x8240DAB4; continue 'dispatch;
            }
            0x8240DAB4 => {
    //   block [0x8240DAB4..0x8240DAC0)
	// 8240DAB4: D01E0008  stfs f0, 8(r30)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 8240DAB8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8240DABC: 48000018  b 0x8240dad4
	pc = 0x8240DAD4; continue 'dispatch;
            }
            0x8240DAC0 => {
    //   block [0x8240DAC0..0x8240DAD4)
	// 8240DAC0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240DAC4: 386BE588  addi r3, r11, -0x1a78
	ctx.r[3].s64 = ctx.r[11].s64 + -6776;
	// 8240DAC8: 4BEA54B9  bl 0x822b2f80
	ctx.lr = 0x8240DACC;
	sub_822B2F80(ctx, base);
	// 8240DACC: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 8240DAD0: 60630031  ori r3, r3, 0x31
	ctx.r[3].u64 = ctx.r[3].u64 | 49;
	pc = 0x8240DAD4; continue 'dispatch;
            }
            0x8240DAD4 => {
    //   block [0x8240DAD4..0x8240DAE0)
	// 8240DAD4: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 8240DAD8: CBE1FFD0  lfd f31, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-48 as u32) ) };
	// 8240DADC: 4812762C  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240DB08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8240DB08 size=224
    let mut pc: u32 = 0x8240DB08;
    'dispatch: loop {
        match pc {
            0x8240DB08 => {
    //   block [0x8240DB08..0x8240DB40)
	// 8240DB08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240DB0C: 481275A9  bl 0x825350b4
	ctx.lr = 0x8240DB10;
	sub_82535080(ctx, base);
	// 8240DB10: DBE1FFC8  stfd f31, -0x38(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-56 as u32), ctx.f[31].u64 ) };
	// 8240DB14: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240DB18: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 8240DB1C: 2B04001F  cmplwi cr6, r4, 0x1f
	ctx.cr[6].compare_u32(ctx.r[4].u32, 31 as u32, &mut ctx.xer);
	// 8240DB20: 419900AC  bgt cr6, 0x8240dbcc
	if ctx.cr[6].gt {
	pc = 0x8240DBCC; continue 'dispatch;
	}
	// 8240DB24: 1D640130  mulli r11, r4, 0x130
	ctx.r[11].s32 = ((ctx.r[4].s32 as i64 * 304 as i64) as i32);
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 8240DB28: 7FAB1A14  add r29, r11, r3
	ctx.r[29].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 8240DB2C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8240DB30: 3BFC0008  addi r31, r28, 8
	ctx.r[31].s64 = ctx.r[28].s64 + 8;
	// 8240DB34: 3B60000A  li r27, 0xa
	ctx.r[27].s64 = 10;
	// 8240DB38: 3BDD001C  addi r30, r29, 0x1c
	ctx.r[30].s64 = ctx.r[29].s64 + 28;
	// 8240DB3C: C3EB2140  lfs f31, 0x2140(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8512 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	pc = 0x8240DB40; continue 'dispatch;
            }
            0x8240DB40 => {
    //   block [0x8240DB40..0x8240DBCC)
	// 8240DB40: C01FFFF8  lfs f0, -8(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240DB44: D01EFFFC  stfs f0, -4(r30)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(-4 as u32), tmp.u32 ) };
	// 8240DB48: C03FFFFC  lfs f1, -4(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-4 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8240DB4C: 48000A0D  bl 0x8240e558
	ctx.lr = 0x8240DB50;
	sub_8240E558(ctx, base);
	// 8240DB50: D03E0000  stfs f1, 0(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 8240DB54: C01F0000  lfs f0, 0(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240DB58: EC00F82A  fadds f0, f0, f31
	ctx.f[0].f64 = ((ctx.f[0].f64 + ctx.f[31].f64) as f32) as f64;
	// 8240DB5C: D01E0004  stfs f0, 4(r30)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 8240DB60: C03F0004  lfs f1, 4(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8240DB64: 480009F5  bl 0x8240e558
	ctx.lr = 0x8240DB68;
	sub_8240E558(ctx, base);
	// 8240DB68: D03E0008  stfs f1, 8(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 8240DB6C: 377BFFFF  addic. r27, r27, -1
	ctx.xer.ca = (ctx.r[27].u32 > (!(-1 as u32)));
	ctx.r[27].s64 = ctx.r[27].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 8240DB70: C01F0008  lfs f0, 8(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240DB74: D01E000C  stfs f0, 0xc(r30)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 8240DB78: C01F000C  lfs f0, 0xc(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240DB7C: D01E0010  stfs f0, 0x10(r30)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 8240DB80: C01F0010  lfs f0, 0x10(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240DB84: 3BFF001C  addi r31, r31, 0x1c
	ctx.r[31].s64 = ctx.r[31].s64 + 28;
	// 8240DB88: D01E0014  stfs f0, 0x14(r30)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 8240DB8C: 3BDE001C  addi r30, r30, 0x1c
	ctx.r[30].s64 = ctx.r[30].s64 + 28;
	// 8240DB90: 4082FFB0  bne 0x8240db40
	if !ctx.cr[0].eq {
	pc = 0x8240DB40; continue 'dispatch;
	}
	// 8240DB94: C01C0118  lfs f0, 0x118(r28)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(280 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240DB98: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8240DB9C: D01D0130  stfs f0, 0x130(r29)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(304 as u32), tmp.u32 ) };
	// 8240DBA0: C01C011C  lfs f0, 0x11c(r28)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(284 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240DBA4: D01D0134  stfs f0, 0x134(r29)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(308 as u32), tmp.u32 ) };
	// 8240DBA8: C01C0120  lfs f0, 0x120(r28)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(288 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240DBAC: D01D0138  stfs f0, 0x138(r29)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(312 as u32), tmp.u32 ) };
	// 8240DBB0: C01C0124  lfs f0, 0x124(r28)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(292 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240DBB4: D01D013C  stfs f0, 0x13c(r29)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(316 as u32), tmp.u32 ) };
	// 8240DBB8: C01C0128  lfs f0, 0x128(r28)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(296 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240DBBC: D01D0140  stfs f0, 0x140(r29)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(320 as u32), tmp.u32 ) };
	// 8240DBC0: C01C012C  lfs f0, 0x12c(r28)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(300 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240DBC4: D01D0144  stfs f0, 0x144(r29)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(324 as u32), tmp.u32 ) };
	// 8240DBC8: 48000014  b 0x8240dbdc
	pc = 0x8240DBDC; continue 'dispatch;
            }
            0x8240DBCC => {
    //   block [0x8240DBCC..0x8240DBDC)
	// 8240DBCC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240DBD0: 386BE618  addi r3, r11, -0x19e8
	ctx.r[3].s64 = ctx.r[11].s64 + -6632;
	// 8240DBD4: 4BEA53AD  bl 0x822b2f80
	ctx.lr = 0x8240DBD8;
	sub_822B2F80(ctx, base);
	// 8240DBD8: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	pc = 0x8240DBDC; continue 'dispatch;
            }
            0x8240DBDC => {
    //   block [0x8240DBDC..0x8240DBE8)
	// 8240DBDC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8240DBE0: CBE1FFC8  lfd f31, -0x38(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-56 as u32) ) };
	// 8240DBE4: 48127520  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240DBE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8240DBE8 size=440
    let mut pc: u32 = 0x8240DBE8;
    'dispatch: loop {
        match pc {
            0x8240DBE8 => {
    //   block [0x8240DBE8..0x8240DC64)
	// 8240DBE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240DBEC: 481274BD  bl 0x825350a8
	ctx.lr = 0x8240DBF0;
	sub_82535080(ctx, base);
	// 8240DBF0: DBC1FFA8  stfd f30, -0x58(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-88 as u32), ctx.f[30].u64 ) };
	// 8240DBF4: DBE1FFB0  stfd f31, -0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-80 as u32), ctx.f[31].u64 ) };
	// 8240DBF8: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240DBFC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8240DC00: 7CF83B78  mr r24, r7
	ctx.r[24].u64 = ctx.r[7].u64;
	// 8240DC04: 7D194378  mr r25, r8
	ctx.r[25].u64 = ctx.r[8].u64;
	// 8240DC08: 7D3A4B78  mr r26, r9
	ctx.r[26].u64 = ctx.r[9].u64;
	// 8240DC0C: 3940000A  li r10, 0xa
	ctx.r[10].s64 = 10;
	// 8240DC10: C3EB1FF8  lfs f31, 0x1ff8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 8240DC14: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 8240DC18: D3F80000  stfs f31, 0(r24)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[24].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 8240DC1C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8240DC20: D3F90000  stfs f31, 0(r25)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 8240DC24: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8240DC28: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 8240DC2C: 2B1B001F  cmplwi cr6, r27, 0x1f
	ctx.cr[6].compare_u32(ctx.r[27].u32, 31 as u32, &mut ctx.xer);
	// 8240DC30: 915A0000  stw r10, 0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8240DC34: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8240DC38: 41990144  bgt cr6, 0x8240dd7c
	if ctx.cr[6].gt {
	pc = 0x8240DD7C; continue 'dispatch;
	}
	// 8240DC3C: 1D7B0130  mulli r11, r27, 0x130
	ctx.r[11].s32 = ((ctx.r[27].s32 as i64 * 304 as i64) as i32);
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 8240DC40: C03F0018  lfs f1, 0x18(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8240DC44: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 8240DC48: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 8240DC4C: 3B8B0018  addi r28, r11, 0x18
	ctx.r[28].s64 = ctx.r[11].s64 + 24;
	// 8240DC50: 4BFFFA41  bl 0x8240d690
	ctx.lr = 0x8240DC54;
	sub_8240D690(ctx, base);
	// 8240DC54: 4BFF773D  bl 0x82405390
	ctx.lr = 0x8240DC58;
	sub_82405390(ctx, base);
	// 8240DC58: C3C10050  lfs f30, 0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 8240DC5C: FF1E0800  fcmpu cr6, f30, f1
	ctx.cr[6].compare_f64(ctx.f[30].f64, ctx.f[1].f64);
	// 8240DC60: 41990010  bgt cr6, 0x8240dc70
	if ctx.cr[6].gt {
	pc = 0x8240DC70; continue 'dispatch;
	}
	pc = 0x8240DC64; continue 'dispatch;
            }
            0x8240DC64 => {
    //   block [0x8240DC64..0x8240DC68)
	// 8240DC64: D3F80000  stfs f31, 0(r24)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[24].u32.wrapping_add(0 as u32), tmp.u32 ) };
	pc = 0x8240DC68; continue 'dispatch;
            }
            0x8240DC68 => {
    //   block [0x8240DC68..0x8240DC70)
	// 8240DC68: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8240DC6C: 48000124  b 0x8240dd90
	pc = 0x8240DD90; continue 'dispatch;
            }
            0x8240DC70 => {
    //   block [0x8240DC70..0x8240DCAC)
	// 8240DC70: C1BD0004  lfs f13, 4(r29)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8240DC74: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8240DC78: C01F0004  lfs f0, 4(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240DC7C: 7F8AE378  mr r10, r28
	ctx.r[10].u64 = ctx.r[28].u64;
	// 8240DC80: EC006828  fsubs f0, f0, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 - ctx.f[13].f64) as f32) as f64);
	// 8240DC84: C19F0008  lfs f12, 8(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 8240DC88: C1BD0008  lfs f13, 8(r29)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8240DC8C: EDAC6828  fsubs f13, f12, f13
	ctx.f[13].f64 = (((ctx.f[12].f64 - ctx.f[13].f64) as f32) as f64);
	// 8240DC90: C17D0000  lfs f11, 0(r29)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 8240DC94: C19F0000  lfs f12, 0(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 8240DC98: ED8C5828  fsubs f12, f12, f11
	ctx.f[12].f64 = (((ctx.f[12].f64 - ctx.f[11].f64) as f32) as f64);
	// 8240DC9C: C17E000C  lfs f11, 0xc(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 8240DCA0: EC000032  fmuls f0, f0, f0
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[0].f64) as f32) as f64);
	// 8240DCA4: EC0D037A  fmadds f0, f13, f13, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[13].f64 + ctx.f[0].f64) as f32) as f64);
	// 8240DCA8: EC2C033A  fmadds f1, f12, f12, f0
	ctx.f[1].f64 = (((ctx.f[12].f64 * ctx.f[12].f64 + ctx.f[0].f64) as f32) as f64);
	pc = 0x8240DCAC; continue 'dispatch;
            }
            0x8240DCAC => {
    //   block [0x8240DCAC..0x8240DCD4)
	// 8240DCAC: C00A0000  lfs f0, 0(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240DCB0: EC0B0032  fmuls f0, f11, f0
	ctx.f[0].f64 = (((ctx.f[11].f64 * ctx.f[0].f64) as f32) as f64);
	// 8240DCB4: EC000032  fmuls f0, f0, f0
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[0].f64) as f32) as f64);
	// 8240DCB8: FF010000  fcmpu cr6, f1, f0
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 8240DCBC: 41980018  blt cr6, 0x8240dcd4
	if ctx.cr[6].lt {
	pc = 0x8240DCD4; continue 'dispatch;
	}
	// 8240DCC0: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8240DCC4: 394A001C  addi r10, r10, 0x1c
	ctx.r[10].s64 = ctx.r[10].s64 + 28;
	// 8240DCC8: 2F0B000A  cmpwi cr6, r11, 0xa
	ctx.cr[6].compare_i32(ctx.r[11].s32, 10, &mut ctx.xer);
	// 8240DCCC: 4198FFE0  blt cr6, 0x8240dcac
	if ctx.cr[6].lt {
	pc = 0x8240DCAC; continue 'dispatch;
	}
	// 8240DCD0: 48000008  b 0x8240dcd8
	pc = 0x8240DCD8; continue 'dispatch;
            }
            0x8240DCD4 => {
    //   block [0x8240DCD4..0x8240DCD8)
	// 8240DCD4: 917A0000  stw r11, 0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	pc = 0x8240DCD8; continue 'dispatch;
            }
            0x8240DCD8 => {
    //   block [0x8240DCD8..0x8240DCEC)
	// 8240DCD8: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 8240DCDC: 2C0B0000  cmpwi r11, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8240DCE0: 4082000C  bne 0x8240dcec
	if !ctx.cr[0].eq {
	pc = 0x8240DCEC; continue 'dispatch;
	}
	// 8240DCE4: C01C0004  lfs f0, 4(r28)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240DCE8: 48000010  b 0x8240dcf8
	pc = 0x8240DCF8; continue 'dispatch;
            }
            0x8240DCEC => {
    //   block [0x8240DCEC..0x8240DCF8)
	// 8240DCEC: 2F0B000A  cmpwi cr6, r11, 0xa
	ctx.cr[6].compare_i32(ctx.r[11].s32, 10, &mut ctx.xer);
	// 8240DCF0: 409A0010  bne cr6, 0x8240dd00
	if !ctx.cr[6].eq {
	pc = 0x8240DD00; continue 'dispatch;
	}
	// 8240DCF4: C01C0100  lfs f0, 0x100(r28)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(256 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	pc = 0x8240DCF8; continue 'dispatch;
            }
            0x8240DCF8 => {
    //   block [0x8240DCF8..0x8240DD00)
	// 8240DCF8: FDA0F890  fmr f13, f31
	ctx.f[13].f64 = ctx.f[31].f64;
	// 8240DCFC: 48000014  b 0x8240dd10
	pc = 0x8240DD10; continue 'dispatch;
            }
            0x8240DD00 => {
    //   block [0x8240DD00..0x8240DD10)
	// 8240DD00: 1D6B001C  mulli r11, r11, 0x1c
	ctx.r[11].s32 = ((ctx.r[11].s32 as i64 * 28 as i64) as i32);
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 8240DD04: 7D6BE214  add r11, r11, r28
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[28].u64;
	// 8240DD08: C00BFFE8  lfs f0, -0x18(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-24 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240DD0C: C1AB0004  lfs f13, 4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	pc = 0x8240DD10; continue 'dispatch;
            }
            0x8240DD10 => {
    //   block [0x8240DD10..0x8240DD20)
	// 8240DD10: FF00F800  fcmpu cr6, f0, f31
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[31].f64);
	// 8240DD14: 4199000C  bgt cr6, 0x8240dd20
	if ctx.cr[6].gt {
	pc = 0x8240DD20; continue 'dispatch;
	}
	// 8240DD18: FF0DF800  fcmpu cr6, f13, f31
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[31].f64);
	// 8240DD1C: 4099FF48  ble cr6, 0x8240dc64
	if !ctx.cr[6].gt {
	pc = 0x8240DC64; continue 'dispatch;
	}
	pc = 0x8240DD20; continue 'dispatch;
            }
            0x8240DD20 => {
    //   block [0x8240DD20..0x8240DD68)
	// 8240DD20: 48002199  bl 0x8240feb8
	ctx.lr = 0x8240DD24;
	sub_8240FEB8(ctx, base);
	// 8240DD24: C01E000C  lfs f0, 0xc(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240DD28: EC210024  fdivs f1, f1, f0
	ctx.f[1].f64 = ((ctx.f[1].f64 / ctx.f[0].f64) as f32) as f64;
	// 8240DD2C: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 8240DD30: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8240DD34: D0390000  stfs f1, 0(r25)
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 8240DD38: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8240DD3C: 80DA0000  lwz r6, 0(r26)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 8240DD40: 4BFFF861  bl 0x8240d5a0
	ctx.lr = 0x8240DD44;
	sub_8240D5A0(ctx, base);
	// 8240DD44: C0210050  lfs f1, 0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8240DD48: 480007C9  bl 0x8240e510
	ctx.lr = 0x8240DD4C;
	sub_8240E510(ctx, base);
	// 8240DD4C: EC21F02A  fadds f1, f1, f30
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ((ctx.f[1].f64 + ctx.f[30].f64) as f32) as f64;
	// 8240DD50: C01C012C  lfs f0, 0x12c(r28)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(300 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240DD54: FF010000  fcmpu cr6, f1, f0
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 8240DD58: 40990010  ble cr6, 0x8240dd68
	if !ctx.cr[6].gt {
	pc = 0x8240DD68; continue 'dispatch;
	}
	// 8240DD5C: ED810028  fsubs f12, f1, f0
	ctx.f[12].f64 = (((ctx.f[1].f64 - ctx.f[0].f64) as f32) as f64);
	// 8240DD60: C1BE0010  lfs f13, 0x10(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8240DD64: EC2C037A  fmadds f1, f12, f13, f0
	ctx.f[1].f64 = (((ctx.f[12].f64 * ctx.f[13].f64 + ctx.f[0].f64) as f32) as f64);
	pc = 0x8240DD68; continue 'dispatch;
            }
            0x8240DD68 => {
    //   block [0x8240DD68..0x8240DD7C)
	// 8240DD68: 480007F1  bl 0x8240e558
	ctx.lr = 0x8240DD6C;
	sub_8240E558(ctx, base);
	// 8240DD6C: D0380000  stfs f1, 0(r24)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[24].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 8240DD70: FF01F800  fcmpu cr6, f1, f31
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[31].f64);
	// 8240DD74: 4098FEF4  bge cr6, 0x8240dc68
	if !ctx.cr[6].lt {
	pc = 0x8240DC68; continue 'dispatch;
	}
	// 8240DD78: 4BFFFEEC  b 0x8240dc64
	pc = 0x8240DC64; continue 'dispatch;
            }
            0x8240DD7C => {
    //   block [0x8240DD7C..0x8240DD90)
	// 8240DD7C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240DD80: 386BE660  addi r3, r11, -0x19a0
	ctx.r[3].s64 = ctx.r[11].s64 + -6560;
	// 8240DD84: 4BEA51FD  bl 0x822b2f80
	ctx.lr = 0x8240DD88;
	sub_822B2F80(ctx, base);
	// 8240DD88: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 8240DD8C: 60630034  ori r3, r3, 0x34
	ctx.r[3].u64 = ctx.r[3].u64 | 52;
	pc = 0x8240DD90; continue 'dispatch;
            }
            0x8240DD90 => {
    //   block [0x8240DD90..0x8240DDA0)
	// 8240DD90: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 8240DD94: CBC1FFA8  lfd f30, -0x58(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-88 as u32) ) };
	// 8240DD98: CBE1FFB0  lfd f31, -0x50(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-80 as u32) ) };
	// 8240DD9C: 4812735C  b 0x825350f8
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240DDA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8240DDA0 size=1676
    let mut pc: u32 = 0x8240DDA0;
    'dispatch: loop {
        match pc {
            0x8240DDA0 => {
    //   block [0x8240DDA0..0x8240DE1C)
	// 8240DDA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240DDA4: 48127301  bl 0x825350a4
	ctx.lr = 0x8240DDA8;
	sub_82535080(ctx, base);
	// 8240DDA8: 3981FFB0  addi r12, r1, -0x50
	ctx.r[12].s64 = ctx.r[1].s64 + -80;
	// 8240DDAC: 48128231  bl 0x82535fdc
	ctx.lr = 0x8240DDB0;
	sub_82535FB0(ctx, base);
	// 8240DDB0: 9421FE30  stwu r1, -0x1d0(r1)
	ea = ctx.r[1].u32.wrapping_add(-464 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240DDB4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8240DDB8: FF200890  fmr f25, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[25].f64 = ctx.f[1].f64;
	// 8240DDBC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8240DDC0: FFA01090  fmr f29, f2
	ctx.f[29].f64 = ctx.f[2].f64;
	// 8240DDC4: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 8240DDC8: FF401890  fmr f26, f3
	ctx.f[26].f64 = ctx.f[3].f64;
	// 8240DDCC: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 8240DDD0: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 8240DDD4: C01E0018  lfs f0, 0x18(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240DDD8: 7D5B5378  mr r27, r10
	ctx.r[27].u64 = ctx.r[10].u64;
	// 8240DDDC: C38B1FF8  lfs f28, 0x1ff8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[28].f64 = (tmp.f32 as f64);
	// 8240DDE0: 3BFE0018  addi r31, r30, 0x18
	ctx.r[31].s64 = ctx.r[30].s64 + 24;
	// 8240DDE4: FF00E000  fcmpu cr6, f0, f28
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[28].f64);
	// 8240DDE8: 409A0034  bne cr6, 0x8240de1c
	if !ctx.cr[6].eq {
	pc = 0x8240DE1C; continue 'dispatch;
	}
	// 8240DDEC: C01E001C  lfs f0, 0x1c(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240DDF0: FF00E000  fcmpu cr6, f0, f28
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[28].f64);
	// 8240DDF4: 409A0028  bne cr6, 0x8240de1c
	if !ctx.cr[6].eq {
	pc = 0x8240DE1C; continue 'dispatch;
	}
	// 8240DDF8: C01E0020  lfs f0, 0x20(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240DDFC: FF00E000  fcmpu cr6, f0, f28
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[28].f64);
	// 8240DE00: 409A001C  bne cr6, 0x8240de1c
	if !ctx.cr[6].eq {
	pc = 0x8240DE1C; continue 'dispatch;
	}
	// 8240DE04: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240DE08: 386BE850  addi r3, r11, -0x17b0
	ctx.r[3].s64 = ctx.r[11].s64 + -6064;
	// 8240DE0C: 4BEA5175  bl 0x822b2f80
	ctx.lr = 0x8240DE10;
	sub_822B2F80(ctx, base);
	// 8240DE10: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 8240DE14: 6063003B  ori r3, r3, 0x3b
	ctx.r[3].u64 = ctx.r[3].u64 | 59;
	// 8240DE18: 48000604  b 0x8240e41c
	pc = 0x8240E41C; continue 'dispatch;
            }
            0x8240DE1C => {
    //   block [0x8240DE1C..0x8240DE50)
	// 8240DE1C: C01E0024  lfs f0, 0x24(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240DE20: 3B9E0024  addi r28, r30, 0x24
	ctx.r[28].s64 = ctx.r[30].s64 + 36;
	// 8240DE24: FF00E000  fcmpu cr6, f0, f28
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[28].f64);
	// 8240DE28: 409A0028  bne cr6, 0x8240de50
	if !ctx.cr[6].eq {
	pc = 0x8240DE50; continue 'dispatch;
	}
	// 8240DE2C: C01E0028  lfs f0, 0x28(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240DE30: FF00E000  fcmpu cr6, f0, f28
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[28].f64);
	// 8240DE34: 409A001C  bne cr6, 0x8240de50
	if !ctx.cr[6].eq {
	pc = 0x8240DE50; continue 'dispatch;
	}
	// 8240DE38: C01E002C  lfs f0, 0x2c(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(44 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240DE3C: FF00E000  fcmpu cr6, f0, f28
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[28].f64);
	// 8240DE40: 409A0010  bne cr6, 0x8240de50
	if !ctx.cr[6].eq {
	pc = 0x8240DE50; continue 'dispatch;
	}
	// 8240DE44: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240DE48: 386BE808  addi r3, r11, -0x17f8
	ctx.r[3].s64 = ctx.r[11].s64 + -6136;
	// 8240DE4C: 480005C8  b 0x8240e414
	pc = 0x8240E414; continue 'dispatch;
            }
            0x8240DE50 => {
    //   block [0x8240DE50..0x8240DF4C)
	// 8240DE50: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8240DE54: 4800206D  bl 0x8240fec0
	ctx.lr = 0x8240DE58;
	sub_8240FEC0(ctx, base);
	// 8240DE58: 2B030001  cmplwi cr6, r3, 1
	ctx.cr[6].compare_u32(ctx.r[3].u32, 1 as u32, &mut ctx.xer);
	// 8240DE5C: 419A05B0  beq cr6, 0x8240e40c
	if ctx.cr[6].eq {
	pc = 0x8240E40C; continue 'dispatch;
	}
	// 8240DE60: 3AFE000C  addi r23, r30, 0xc
	ctx.r[23].s64 = ctx.r[30].s64 + 12;
	// 8240DE64: 7EE3BB78  mr r3, r23
	ctx.r[3].u64 = ctx.r[23].u64;
	// 8240DE68: 48002059  bl 0x8240fec0
	ctx.lr = 0x8240DE6C;
	sub_8240FEC0(ctx, base);
	// 8240DE6C: 2B030001  cmplwi cr6, r3, 1
	ctx.cr[6].compare_u32(ctx.r[3].u32, 1 as u32, &mut ctx.xer);
	// 8240DE70: 419A059C  beq cr6, 0x8240e40c
	if ctx.cr[6].eq {
	pc = 0x8240E40C; continue 'dispatch;
	}
	// 8240DE74: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8240DE78: 48002049  bl 0x8240fec0
	ctx.lr = 0x8240DE7C;
	sub_8240FEC0(ctx, base);
	// 8240DE7C: 2B030001  cmplwi cr6, r3, 1
	ctx.cr[6].compare_u32(ctx.r[3].u32, 1 as u32, &mut ctx.xer);
	// 8240DE80: 419A058C  beq cr6, 0x8240e40c
	if ctx.cr[6].eq {
	pc = 0x8240E40C; continue 'dispatch;
	}
	// 8240DE84: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8240DE88: 48002039  bl 0x8240fec0
	ctx.lr = 0x8240DE8C;
	sub_8240FEC0(ctx, base);
	// 8240DE8C: 2B030001  cmplwi cr6, r3, 1
	ctx.cr[6].compare_u32(ctx.r[3].u32, 1 as u32, &mut ctx.xer);
	// 8240DE90: 419A057C  beq cr6, 0x8240e40c
	if ctx.cr[6].eq {
	pc = 0x8240E40C; continue 'dispatch;
	}
	// 8240DE94: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 8240DE98: 48002029  bl 0x8240fec0
	ctx.lr = 0x8240DE9C;
	sub_8240FEC0(ctx, base);
	// 8240DE9C: 2B030001  cmplwi cr6, r3, 1
	ctx.cr[6].compare_u32(ctx.r[3].u32, 1 as u32, &mut ctx.xer);
	// 8240DEA0: 419A0560  beq cr6, 0x8240e400
	if ctx.cr[6].eq {
	pc = 0x8240E400; continue 'dispatch;
	}
	// 8240DEA4: 3B1A000C  addi r24, r26, 0xc
	ctx.r[24].s64 = ctx.r[26].s64 + 12;
	// 8240DEA8: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 8240DEAC: 48002015  bl 0x8240fec0
	ctx.lr = 0x8240DEB0;
	sub_8240FEC0(ctx, base);
	// 8240DEB0: 2B030001  cmplwi cr6, r3, 1
	ctx.cr[6].compare_u32(ctx.r[3].u32, 1 as u32, &mut ctx.xer);
	// 8240DEB4: 419A054C  beq cr6, 0x8240e400
	if ctx.cr[6].eq {
	pc = 0x8240E400; continue 'dispatch;
	}
	// 8240DEB8: 2B1D001F  cmplwi cr6, r29, 0x1f
	ctx.cr[6].compare_u32(ctx.r[29].u32, 31 as u32, &mut ctx.xer);
	// 8240DEBC: 41990528  bgt cr6, 0x8240e3e4
	if ctx.cr[6].gt {
	pc = 0x8240E3E4; continue 'dispatch;
	}
	// 8240DEC0: C1BF0000  lfs f13, 0(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8240DEC4: 1D7D0130  mulli r11, r29, 0x130
	ctx.r[11].s32 = ((ctx.r[29].s32 as i64 * 304 as i64) as i32);
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 8240DEC8: C01E0000  lfs f0, 0(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240DECC: 7D6BCA14  add r11, r11, r25
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[25].u64;
	// 8240DED0: EC00682A  fadds f0, f0, f13
	ctx.f[0].f64 = ((ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64;
	// 8240DED4: D0010050  stfs f0, 0x50(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 8240DED8: C19E001C  lfs f12, 0x1c(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 8240DEDC: C01E0004  lfs f0, 4(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240DEE0: EC0C002A  fadds f0, f12, f0
	ctx.f[0].f64 = ((ctx.f[12].f64 + ctx.f[0].f64) as f32) as f64;
	// 8240DEE4: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 8240DEE8: C1BE0020  lfs f13, 0x20(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8240DEEC: C01E0008  lfs f0, 8(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240DEF0: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 8240DEF4: EC0D002A  fadds f0, f13, f0
	ctx.f[0].f64 = ((ctx.f[13].f64 + ctx.f[0].f64) as f32) as f64;
	// 8240DEF8: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8240DEFC: D0010058  stfs f0, 0x58(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), tmp.u32 ) };
	// 8240DF00: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8240DF04: 38610100  addi r3, r1, 0x100
	ctx.r[3].s64 = ctx.r[1].s64 + 256;
	// 8240DF08: 3B8B0018  addi r28, r11, 0x18
	ctx.r[28].s64 = ctx.r[11].s64 + 24;
	// 8240DF0C: 48001D7D  bl 0x8240fc88
	ctx.lr = 0x8240DF10;
	sub_8240FC88(ctx, base);
	// 8240DF10: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8240DF14: 386100C0  addi r3, r1, 0xc0
	ctx.r[3].s64 = ctx.r[1].s64 + 192;
	// 8240DF18: 38A00040  li r5, 0x40
	ctx.r[5].s64 = 64;
	// 8240DF1C: 48126C35  bl 0x82534b50
	ctx.lr = 0x8240DF20;
	sub_82534B50(ctx, base);
	// 8240DF20: 38A100C0  addi r5, r1, 0xc0
	ctx.r[5].s64 = ctx.r[1].s64 + 192;
	// 8240DF24: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 8240DF28: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 8240DF2C: 48001D55  bl 0x8240fc80
	ctx.lr = 0x8240DF30;
	sub_8240FC80(ctx, base);
	// 8240DF30: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 8240DF34: 48001FF5  bl 0x8240ff28
	ctx.lr = 0x8240DF38;
	sub_8240FF28(ctx, base);
	// 8240DF38: 2B030001  cmplwi cr6, r3, 1
	ctx.cr[6].compare_u32(ctx.r[3].u32, 1 as u32, &mut ctx.xer);
	// 8240DF3C: 409A0010  bne cr6, 0x8240df4c
	if !ctx.cr[6].eq {
	pc = 0x8240DF4C; continue 'dispatch;
	}
	// 8240DF40: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240DF44: 386BE7C0  addi r3, r11, -0x1840
	ctx.r[3].s64 = ctx.r[11].s64 + -6208;
	// 8240DF48: 480004CC  b 0x8240e414
	pc = 0x8240E414; continue 'dispatch;
            }
            0x8240DF4C => {
    //   block [0x8240DF4C..0x8240DF8C)
	// 8240DF4C: 83E10224  lwz r31, 0x224(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(548 as u32) ) } as u64;
	// 8240DF50: 39210060  addi r9, r1, 0x60
	ctx.r[9].s64 = ctx.r[1].s64 + 96;
	// 8240DF54: 3901005C  addi r8, r1, 0x5c
	ctx.r[8].s64 = ctx.r[1].s64 + 92;
	// 8240DF58: 38FF00A4  addi r7, r31, 0xa4
	ctx.r[7].s64 = ctx.r[31].s64 + 164;
	// 8240DF5C: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 8240DF60: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 8240DF64: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8240DF68: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 8240DF6C: 4BFFFC7D  bl 0x8240dbe8
	ctx.lr = 0x8240DF70;
	sub_8240DBE8(ctx, base);
	// 8240DF70: C01F00A4  lfs f0, 0xa4(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240DF74: FF00E000  fcmpu cr6, f0, f28
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[28].f64);
	// 8240DF78: 41990050  bgt cr6, 0x8240dfc8
	if ctx.cr[6].gt {
	pc = 0x8240DFC8; continue 'dispatch;
	}
	// 8240DF7C: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 8240DF80: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8240DF84: 39400024  li r10, 0x24
	ctx.r[10].s64 = 36;
	// 8240DF88: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	pc = 0x8240DF8C; continue 'dispatch;
            }
            0x8240DF8C => {
    //   block [0x8240DF8C..0x8240DFBC)
	// 8240DF8C: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8240DF90: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8240DF94: 4200FFF8  bdnz 0x8240df8c
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x8240DF8C; continue 'dispatch;
	}
	// 8240DF98: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8240DF9C: D39F0094  stfs f28, 0x94(r31)
	tmp.f32 = (ctx.f[28].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), tmp.u32 ) };
	// 8240DFA0: D39F0098  stfs f28, 0x98(r31)
	tmp.f32 = (ctx.f[28].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(152 as u32), tmp.u32 ) };
	// 8240DFA4: D39F009C  stfs f28, 0x9c(r31)
	tmp.f32 = (ctx.f[28].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(156 as u32), tmp.u32 ) };
	// 8240DFA8: D39F00A8  stfs f28, 0xa8(r31)
	tmp.f32 = (ctx.f[28].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(168 as u32), tmp.u32 ) };
	// 8240DFAC: C00B2268  lfs f0, 0x2268(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8808 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240DFB0: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8240DFB4: D01F0090  stfs f0, 0x90(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(144 as u32), tmp.u32 ) };
	// 8240DFB8: C00B1850  lfs f0, 0x1850(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(6224 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	pc = 0x8240DFBC; continue 'dispatch;
            }
            0x8240DFBC => {
    //   block [0x8240DFBC..0x8240DFC0)
	// 8240DFBC: D01F00A0  stfs f0, 0xa0(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(160 as u32), tmp.u32 ) };
	pc = 0x8240DFC0; continue 'dispatch;
            }
            0x8240DFC0 => {
    //   block [0x8240DFC0..0x8240DFC8)
	// 8240DFC0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8240DFC4: 48000458  b 0x8240e41c
	pc = 0x8240E41C; continue 'dispatch;
            }
            0x8240DFC8 => {
    //   block [0x8240DFC8..0x8240DFF8)
	// 8240DFC8: C0010078  lfs f0, 0x78(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(120 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240DFCC: C1A10070  lfs f13, 0x70(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8240DFD0: FF00E000  fcmpu cr6, f0, f28
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[28].f64);
	// 8240DFD4: 4099003C  ble cr6, 0x8240e010
	if !ctx.cr[6].gt {
	pc = 0x8240E010; continue 'dispatch;
	}
	// 8240DFD8: FF0DE000  fcmpu cr6, f13, f28
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[28].f64);
	// 8240DFDC: EC2D0024  fdivs f1, f13, f0
	ctx.f[1].f64 = ((ctx.f[13].f64 / ctx.f[0].f64) as f32) as f64;
	// 8240DFE0: 40990018  ble cr6, 0x8240dff8
	if !ctx.cr[6].gt {
	pc = 0x8240DFF8; continue 'dispatch;
	}
	// 8240DFE4: 48001CED  bl 0x8240fcd0
	ctx.lr = 0x8240DFE8;
	sub_8240FCD0(ctx, base);
	// 8240DFE8: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8240DFEC: C00B2930  lfs f0, 0x2930(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(10544 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240DFF0: EC000828  fsubs f0, f0, f1
	ctx.f[0].f64 = (((ctx.f[0].f64 - ctx.f[1].f64) as f32) as f64);
	// 8240DFF4: 48000078  b 0x8240e06c
	pc = 0x8240E06C; continue 'dispatch;
            }
            0x8240DFF8 => {
    //   block [0x8240DFF8..0x8240E010)
	// 8240DFF8: 48001CD9  bl 0x8240fcd0
	ctx.lr = 0x8240DFFC;
	sub_8240FCD0(ctx, base);
	// 8240DFFC: FDA00850  fneg f13, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[13].u64 = ctx.f[1].u64 ^ 0x8000_0000_0000_0000u64;
	// 8240E000: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8240E004: C00B2930  lfs f0, 0x2930(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(10544 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240E008: EC0D0028  fsubs f0, f13, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 - ctx.f[0].f64) as f32) as f64);
	// 8240E00C: 48000060  b 0x8240e06c
	pc = 0x8240E06C; continue 'dispatch;
            }
            0x8240E010 => {
    //   block [0x8240E010..0x8240E02C)
	// 8240E010: FF00E000  fcmpu cr6, f0, f28
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[28].f64);
	// 8240E014: 409A0028  bne cr6, 0x8240e03c
	if !ctx.cr[6].eq {
	pc = 0x8240E03C; continue 'dispatch;
	}
	// 8240E018: FF0DE000  fcmpu cr6, f13, f28
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[28].f64);
	// 8240E01C: 40990010  ble cr6, 0x8240e02c
	if !ctx.cr[6].gt {
	pc = 0x8240E02C; continue 'dispatch;
	}
	// 8240E020: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240E024: C00BE358  lfs f0, -0x1ca8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-7336 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240E028: 48000044  b 0x8240e06c
	pc = 0x8240E06C; continue 'dispatch;
            }
            0x8240E02C => {
    //   block [0x8240E02C..0x8240E03C)
	// 8240E02C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240E030: C00BE7B8  lfs f0, -0x1848(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-6216 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240E034: FC0D072E  fsel f0, f13, f28, f0
	ctx.f[0].f64 = if ctx.f[13].f64 >= 0.0 { ctx.f[28].f64 } else { ctx.f[0].f64 };
	// 8240E038: 48000034  b 0x8240e06c
	pc = 0x8240E06C; continue 'dispatch;
            }
            0x8240E03C => {
    //   block [0x8240E03C..0x8240E058)
	// 8240E03C: FF0DE000  fcmpu cr6, f13, f28
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[28].f64);
	// 8240E040: 409A0020  bne cr6, 0x8240e060
	if !ctx.cr[6].eq {
	pc = 0x8240E060; continue 'dispatch;
	}
	// 8240E044: FF00E000  fcmpu cr6, f0, f28
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[28].f64);
	// 8240E048: 40990010  ble cr6, 0x8240e058
	if !ctx.cr[6].gt {
	pc = 0x8240E058; continue 'dispatch;
	}
	// 8240E04C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8240E050: C00B2930  lfs f0, 0x2930(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(10544 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240E054: 48000018  b 0x8240e06c
	pc = 0x8240E06C; continue 'dispatch;
            }
            0x8240E058 => {
    //   block [0x8240E058..0x8240E060)
	// 8240E058: FC00E090  fmr f0, f28
	ctx.f[0].f64 = ctx.f[28].f64;
	// 8240E05C: 48000010  b 0x8240e06c
	pc = 0x8240E06C; continue 'dispatch;
            }
            0x8240E060 => {
    //   block [0x8240E060..0x8240E06C)
	// 8240E060: EC2D0024  fdivs f1, f13, f0
	ctx.f[1].f64 = ((ctx.f[13].f64 / ctx.f[0].f64) as f32) as f64;
	// 8240E064: 48001C6D  bl 0x8240fcd0
	ctx.lr = 0x8240E068;
	sub_8240FCD0(ctx, base);
	// 8240E068: FC000850  fneg f0, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = ctx.f[1].u64 ^ 0x8000_0000_0000_0000u64;
	pc = 0x8240E06C; continue 'dispatch;
            }
            0x8240E06C => {
    //   block [0x8240E06C..0x8240E094)
	// 8240E06C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240E070: C1ABE580  lfs f13, -0x1a80(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-6784 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8240E074: EFE00372  fmuls f31, f0, f13
	ctx.f[31].f64 = (((ctx.f[0].f64 * ctx.f[13].f64) as f32) as f64);
	// 8240E078: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 8240E07C: 48001F25  bl 0x8240ffa0
	ctx.lr = 0x8240E080;
	sub_8240FFA0(ctx, base);
	// 8240E080: 2B030001  cmplwi cr6, r3, 1
	ctx.cr[6].compare_u32(ctx.r[3].u32, 1 as u32, &mut ctx.xer);
	// 8240E084: 409A0010  bne cr6, 0x8240e094
	if !ctx.cr[6].eq {
	pc = 0x8240E094; continue 'dispatch;
	}
	// 8240E088: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240E08C: 386BE778  addi r3, r11, -0x1888
	ctx.r[3].s64 = ctx.r[11].s64 + -6280;
	// 8240E090: 48000384  b 0x8240e414
	pc = 0x8240E414; continue 'dispatch;
            }
            0x8240E094 => {
    //   block [0x8240E094..0x8240E0B4)
	// 8240E094: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8240E098: C01C0118  lfs f0, 0x118(r28)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(280 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240E09C: C3C1005C  lfs f30, 0x5c(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 8240E0A0: FF00E000  fcmpu cr6, f0, f28
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[28].f64);
	// 8240E0A4: C36B1850  lfs f27, 0x1850(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(6224 as u32) ) };
	ctx.f[27].f64 = (tmp.f32 as f64);
	// 8240E0A8: 4099000C  ble cr6, 0x8240e0b4
	if !ctx.cr[6].gt {
	pc = 0x8240E0B4; continue 'dispatch;
	}
	// 8240E0AC: EC5E0024  fdivs f2, f30, f0
	ctx.f[2].f64 = ((ctx.f[30].f64 / ctx.f[0].f64) as f32) as f64;
	// 8240E0B0: 48000018  b 0x8240e0c8
	pc = 0x8240E0C8; continue 'dispatch;
            }
            0x8240E0B4 => {
    //   block [0x8240E0B4..0x8240E0C8)
	// 8240E0B4: FF00E000  fcmpu cr6, f0, f28
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[28].f64);
	// 8240E0B8: 40980020  bge cr6, 0x8240e0d8
	if !ctx.cr[6].lt {
	pc = 0x8240E0D8; continue 'dispatch;
	}
	// 8240E0BC: EC1E0024  fdivs f0, f30, f0
	ctx.f[0].f64 = ((ctx.f[30].f64 / ctx.f[0].f64) as f32) as f64;
	// 8240E0C0: FC000050  fneg f0, f0
	ctx.f[0].u64 = ctx.f[0].u64 ^ 0x8000_0000_0000_0000u64;
	// 8240E0C4: EC5B0028  fsubs f2, f27, f0
	ctx.f[2].f64 = (((ctx.f[27].f64 - ctx.f[0].f64) as f32) as f64);
	pc = 0x8240E0C8; continue 'dispatch;
            }
            0x8240E0C8 => {
    //   block [0x8240E0C8..0x8240E0D8)
	// 8240E0C8: FF02D800  fcmpu cr6, f2, f27
	ctx.cr[6].compare_f64(ctx.f[2].f64, ctx.f[27].f64);
	// 8240E0CC: 41980010  blt cr6, 0x8240e0dc
	if ctx.cr[6].lt {
	pc = 0x8240E0DC; continue 'dispatch;
	}
	// 8240E0D0: FC40D890  fmr f2, f27
	ctx.f[2].f64 = ctx.f[27].f64;
	// 8240E0D4: 48000008  b 0x8240e0dc
	pc = 0x8240E0DC; continue 'dispatch;
            }
            0x8240E0D8 => {
    //   block [0x8240E0D8..0x8240E0DC)
	// 8240E0D8: FC40E890  fmr f2, f29
	ctx.f[2].f64 = ctx.f[29].f64;
	pc = 0x8240E0DC; continue 'dispatch;
            }
            0x8240E0DC => {
    //   block [0x8240E0DC..0x8240E0FC)
	// 8240E0DC: 2F1B0001  cmpwi cr6, r27, 1
	ctx.cr[6].compare_i32(ctx.r[27].s32, 1, &mut ctx.xer);
	// 8240E0E0: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 8240E0E4: 409A004C  bne cr6, 0x8240e130
	if !ctx.cr[6].eq {
	pc = 0x8240E130; continue 'dispatch;
	}
	// 8240E0E8: 38C100A0  addi r6, r1, 0xa0
	ctx.r[6].s64 = ctx.r[1].s64 + 160;
	// 8240E0EC: FC60D090  fmr f3, f26
	ctx.f[3].f64 = ctx.f[26].f64;
	// 8240E0F0: 4BFF8069  bl 0x82406158
	ctx.lr = 0x8240E0F4;
	sub_82406158(ctx, base);
	// 8240E0F4: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 8240E0F8: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	pc = 0x8240E0FC; continue 'dispatch;
            }
            0x8240E0FC => {
    //   block [0x8240E0FC..0x8240E10C)
	// 8240E0FC: 394100A0  addi r10, r1, 0xa0
	ctx.r[10].s64 = ctx.r[1].s64 + 160;
	// 8240E100: 7D695B78  mr r9, r11
	ctx.r[9].u64 = ctx.r[11].u64;
	// 8240E104: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 8240E108: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	pc = 0x8240E10C; continue 'dispatch;
            }
            0x8240E10C => {
    //   block [0x8240E10C..0x8240E130)
	// 8240E10C: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8240E110: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 8240E114: 91090000  stw r8, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 8240E118: 39290004  addi r9, r9, 4
	ctx.r[9].s64 = ctx.r[9].s64 + 4;
	// 8240E11C: 4200FFF0  bdnz 0x8240e10c
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x8240E10C; continue 'dispatch;
	}
	// 8240E120: 34E7FFFF  addic. r7, r7, -1
	ctx.xer.ca = (ctx.r[7].u32 > (!(-1 as u32)));
	ctx.r[7].s64 = ctx.r[7].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[7].s32, 0, &mut ctx.xer);
	// 8240E124: 396B0018  addi r11, r11, 0x18
	ctx.r[11].s64 = ctx.r[11].s64 + 24;
	// 8240E128: 4082FFD4  bne 0x8240e0fc
	if !ctx.cr[0].eq {
	pc = 0x8240E0FC; continue 'dispatch;
	}
	// 8240E12C: 48000030  b 0x8240e15c
	pc = 0x8240E15C; continue 'dispatch;
            }
            0x8240E130 => {
    //   block [0x8240E130..0x8240E150)
	// 8240E130: 2F1B0002  cmpwi cr6, r27, 2
	ctx.cr[6].compare_i32(ctx.r[27].s32, 2, &mut ctx.xer);
	// 8240E134: 409A001C  bne cr6, 0x8240e150
	if !ctx.cr[6].eq {
	pc = 0x8240E150; continue 'dispatch;
	}
	// 8240E138: 7FE7FB78  mr r7, r31
	ctx.r[7].u64 = ctx.r[31].u64;
	// 8240E13C: FC601090  fmr f3, f2
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[3].f64 = ctx.f[2].f64;
	// 8240E140: FC80D090  fmr f4, f26
	ctx.f[4].f64 = ctx.f[26].f64;
	// 8240E144: FC40C890  fmr f2, f25
	ctx.f[2].f64 = ctx.f[25].f64;
	// 8240E148: 4BFF8121  bl 0x82406268
	ctx.lr = 0x8240E14C;
	sub_82406268(ctx, base);
	// 8240E14C: 48000010  b 0x8240e15c
	pc = 0x8240E15C; continue 'dispatch;
            }
            0x8240E150 => {
    //   block [0x8240E150..0x8240E15C)
	// 8240E150: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 8240E154: FC60D090  fmr f3, f26
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[3].f64 = ctx.f[26].f64;
	// 8240E158: 4BFF8031  bl 0x82406188
	ctx.lr = 0x8240E15C;
	sub_82406188(ctx, base);
	pc = 0x8240E15C; continue 'dispatch;
            }
            0x8240E15C => {
    //   block [0x8240E15C..0x8240E1AC)
	// 8240E15C: 81610060  lwz r11, 0x60(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 8240E160: D3FF00A8  stfs f31, 0xa8(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(168 as u32), tmp.u32 ) };
	// 8240E164: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8240E168: 419A0134  beq cr6, 0x8240e29c
	if ctx.cr[6].eq {
	pc = 0x8240E29C; continue 'dispatch;
	}
	// 8240E16C: 2F0B000A  cmpwi cr6, r11, 0xa
	ctx.cr[6].compare_i32(ctx.r[11].s32, 10, &mut ctx.xer);
	// 8240E170: 419A0128  beq cr6, 0x8240e298
	if ctx.cr[6].eq {
	pc = 0x8240E298; continue 'dispatch;
	}
	// 8240E174: 1D6B001C  mulli r11, r11, 0x1c
	ctx.r[11].s32 = ((ctx.r[11].s32 as i64 * 28 as i64) as i32);
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 8240E178: 7D6BE214  add r11, r11, r28
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[28].u64;
	// 8240E17C: C1AB0000  lfs f13, 0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8240E180: C00BFFE4  lfs f0, -0x1c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-28 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240E184: C18BFFF0  lfs f12, -0x10(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 8240E188: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 8240E18C: 419A0020  beq cr6, 0x8240e1ac
	if ctx.cr[6].eq {
	pc = 0x8240E1AC; continue 'dispatch;
	}
	// 8240E190: C16B000C  lfs f11, 0xc(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 8240E194: ED5E0028  fsubs f10, f30, f0
	ctx.f[10].f64 = (((ctx.f[30].f64 - ctx.f[0].f64) as f32) as f64);
	// 8240E198: ED6B6028  fsubs f11, f11, f12
	ctx.f[11].f64 = (((ctx.f[11].f64 - ctx.f[12].f64) as f32) as f64);
	// 8240E19C: EC0D0028  fsubs f0, f13, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 - ctx.f[0].f64) as f32) as f64);
	// 8240E1A0: EDAB02B2  fmuls f13, f11, f10
	ctx.f[13].f64 = (((ctx.f[11].f64 * ctx.f[10].f64) as f32) as f64);
	// 8240E1A4: EC0D0024  fdivs f0, f13, f0
	ctx.f[0].f64 = ((ctx.f[13].f64 / ctx.f[0].f64) as f32) as f64;
	// 8240E1A8: EDA0602A  fadds f13, f0, f12
	ctx.f[13].f64 = ((ctx.f[0].f64 + ctx.f[12].f64) as f32) as f64;
	pc = 0x8240E1AC; continue 'dispatch;
            }
            0x8240E1AC => {
    //   block [0x8240E1AC..0x8240E1B4)
	// 8240E1AC: 393F000C  addi r9, r31, 0xc
	ctx.r[9].s64 = ctx.r[31].s64 + 12;
	// 8240E1B0: 39400006  li r10, 6
	ctx.r[10].s64 = 6;
	pc = 0x8240E1B4; continue 'dispatch;
            }
            0x8240E1B4 => {
    //   block [0x8240E1B4..0x8240E1F4)
	// 8240E1B4: D1A90000  stfs f13, 0(r9)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 8240E1B8: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8240E1BC: 39290018  addi r9, r9, 0x18
	ctx.r[9].s64 = ctx.r[9].s64 + 24;
	// 8240E1C0: 4082FFF4  bne 0x8240e1b4
	if !ctx.cr[0].eq {
	pc = 0x8240E1B4; continue 'dispatch;
	}
	// 8240E1C4: C1AB0000  lfs f13, 0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8240E1C8: C00BFFE4  lfs f0, -0x1c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-28 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240E1CC: C18BFFEC  lfs f12, -0x14(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-20 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 8240E1D0: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 8240E1D4: 419A0020  beq cr6, 0x8240e1f4
	if ctx.cr[6].eq {
	pc = 0x8240E1F4; continue 'dispatch;
	}
	// 8240E1D8: C16B0008  lfs f11, 8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 8240E1DC: ED5E0028  fsubs f10, f30, f0
	ctx.f[10].f64 = (((ctx.f[30].f64 - ctx.f[0].f64) as f32) as f64);
	// 8240E1E0: ED6B6028  fsubs f11, f11, f12
	ctx.f[11].f64 = (((ctx.f[11].f64 - ctx.f[12].f64) as f32) as f64);
	// 8240E1E4: EC0D0028  fsubs f0, f13, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 - ctx.f[0].f64) as f32) as f64);
	// 8240E1E8: EDAB02B2  fmuls f13, f11, f10
	ctx.f[13].f64 = (((ctx.f[11].f64 * ctx.f[10].f64) as f32) as f64);
	// 8240E1EC: EC0D0024  fdivs f0, f13, f0
	ctx.f[0].f64 = ((ctx.f[13].f64 / ctx.f[0].f64) as f32) as f64;
	// 8240E1F0: EDA0602A  fadds f13, f0, f12
	ctx.f[13].f64 = ((ctx.f[0].f64 + ctx.f[12].f64) as f32) as f64;
	pc = 0x8240E1F4; continue 'dispatch;
            }
            0x8240E1F4 => {
    //   block [0x8240E1F4..0x8240E228)
	// 8240E1F4: D1BF0090  stfs f13, 0x90(r31)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(144 as u32), tmp.u32 ) };
	// 8240E1F8: C1AB0000  lfs f13, 0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8240E1FC: C00BFFE4  lfs f0, -0x1c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-28 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240E200: C18BFFF4  lfs f12, -0xc(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-12 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 8240E204: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 8240E208: 419A0020  beq cr6, 0x8240e228
	if ctx.cr[6].eq {
	pc = 0x8240E228; continue 'dispatch;
	}
	// 8240E20C: C16B0010  lfs f11, 0x10(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 8240E210: ED5E0028  fsubs f10, f30, f0
	ctx.f[10].f64 = (((ctx.f[30].f64 - ctx.f[0].f64) as f32) as f64);
	// 8240E214: ED6B6028  fsubs f11, f11, f12
	ctx.f[11].f64 = (((ctx.f[11].f64 - ctx.f[12].f64) as f32) as f64);
	// 8240E218: EC0D0028  fsubs f0, f13, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 - ctx.f[0].f64) as f32) as f64);
	// 8240E21C: EDAB02B2  fmuls f13, f11, f10
	ctx.f[13].f64 = (((ctx.f[11].f64 * ctx.f[10].f64) as f32) as f64);
	// 8240E220: EC0D0024  fdivs f0, f13, f0
	ctx.f[0].f64 = ((ctx.f[13].f64 / ctx.f[0].f64) as f32) as f64;
	// 8240E224: EDA0602A  fadds f13, f0, f12
	ctx.f[13].f64 = ((ctx.f[0].f64 + ctx.f[12].f64) as f32) as f64;
	pc = 0x8240E228; continue 'dispatch;
            }
            0x8240E228 => {
    //   block [0x8240E228..0x8240E25C)
	// 8240E228: D1BF0094  stfs f13, 0x94(r31)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), tmp.u32 ) };
	// 8240E22C: C1AB0000  lfs f13, 0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8240E230: C00BFFE4  lfs f0, -0x1c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-28 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240E234: C18BFFF8  lfs f12, -8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-8 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 8240E238: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 8240E23C: 419A0020  beq cr6, 0x8240e25c
	if ctx.cr[6].eq {
	pc = 0x8240E25C; continue 'dispatch;
	}
	// 8240E240: C16B0014  lfs f11, 0x14(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 8240E244: ED5E0028  fsubs f10, f30, f0
	ctx.f[10].f64 = (((ctx.f[30].f64 - ctx.f[0].f64) as f32) as f64);
	// 8240E248: ED6B6028  fsubs f11, f11, f12
	ctx.f[11].f64 = (((ctx.f[11].f64 - ctx.f[12].f64) as f32) as f64);
	// 8240E24C: EC0D0028  fsubs f0, f13, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 - ctx.f[0].f64) as f32) as f64);
	// 8240E250: EDAB02B2  fmuls f13, f11, f10
	ctx.f[13].f64 = (((ctx.f[11].f64 * ctx.f[10].f64) as f32) as f64);
	// 8240E254: EC0D0024  fdivs f0, f13, f0
	ctx.f[0].f64 = ((ctx.f[13].f64 / ctx.f[0].f64) as f32) as f64;
	// 8240E258: EDA0602A  fadds f13, f0, f12
	ctx.f[13].f64 = ((ctx.f[0].f64 + ctx.f[12].f64) as f32) as f64;
	pc = 0x8240E25C; continue 'dispatch;
            }
            0x8240E25C => {
    //   block [0x8240E25C..0x8240E290)
	// 8240E25C: D1BF0098  stfs f13, 0x98(r31)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(152 as u32), tmp.u32 ) };
	// 8240E260: C1AB0000  lfs f13, 0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8240E264: C00BFFE4  lfs f0, -0x1c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-28 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240E268: C18BFFFC  lfs f12, -4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-4 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 8240E26C: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 8240E270: 419A0020  beq cr6, 0x8240e290
	if ctx.cr[6].eq {
	pc = 0x8240E290; continue 'dispatch;
	}
	// 8240E274: C16B0018  lfs f11, 0x18(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 8240E278: ED5E0028  fsubs f10, f30, f0
	ctx.f[10].f64 = (((ctx.f[30].f64 - ctx.f[0].f64) as f32) as f64);
	// 8240E27C: ED6B6028  fsubs f11, f11, f12
	ctx.f[11].f64 = (((ctx.f[11].f64 - ctx.f[12].f64) as f32) as f64);
	// 8240E280: EC0D0028  fsubs f0, f13, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 - ctx.f[0].f64) as f32) as f64);
	// 8240E284: EDAB02B2  fmuls f13, f11, f10
	ctx.f[13].f64 = (((ctx.f[11].f64 * ctx.f[10].f64) as f32) as f64);
	// 8240E288: EC0D0024  fdivs f0, f13, f0
	ctx.f[0].f64 = ((ctx.f[13].f64 / ctx.f[0].f64) as f32) as f64;
	// 8240E28C: EDA0602A  fadds f13, f0, f12
	ctx.f[13].f64 = ((ctx.f[0].f64 + ctx.f[12].f64) as f32) as f64;
	pc = 0x8240E290; continue 'dispatch;
            }
            0x8240E290 => {
    //   block [0x8240E290..0x8240E298)
	// 8240E290: D1BF009C  stfs f13, 0x9c(r31)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(156 as u32), tmp.u32 ) };
	// 8240E294: 4800004C  b 0x8240e2e0
	pc = 0x8240E2E0; continue 'dispatch;
            }
            0x8240E298 => {
    //   block [0x8240E298..0x8240E29C)
	// 8240E298: 39600009  li r11, 9
	ctx.r[11].s64 = 9;
	pc = 0x8240E29C; continue 'dispatch;
            }
            0x8240E29C => {
    //   block [0x8240E29C..0x8240E2AC)
	// 8240E29C: 1D6B001C  mulli r11, r11, 0x1c
	ctx.r[11].s32 = ((ctx.r[11].s32 as i64 * 28 as i64) as i32);
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 8240E2A0: 7D6BE214  add r11, r11, r28
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[28].u64;
	// 8240E2A4: 393F000C  addi r9, r31, 0xc
	ctx.r[9].s64 = ctx.r[31].s64 + 12;
	// 8240E2A8: 39400006  li r10, 6
	ctx.r[10].s64 = 6;
	pc = 0x8240E2AC; continue 'dispatch;
            }
            0x8240E2AC => {
    //   block [0x8240E2AC..0x8240E2E0)
	// 8240E2AC: C00B000C  lfs f0, 0xc(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240E2B0: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8240E2B4: D0090000  stfs f0, 0(r9)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 8240E2B8: 39290018  addi r9, r9, 0x18
	ctx.r[9].s64 = ctx.r[9].s64 + 24;
	// 8240E2BC: 4082FFF0  bne 0x8240e2ac
	if !ctx.cr[0].eq {
	pc = 0x8240E2AC; continue 'dispatch;
	}
	// 8240E2C0: C00B0008  lfs f0, 8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240E2C4: D01F0090  stfs f0, 0x90(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(144 as u32), tmp.u32 ) };
	// 8240E2C8: C00B0010  lfs f0, 0x10(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240E2CC: D01F0094  stfs f0, 0x94(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), tmp.u32 ) };
	// 8240E2D0: C00B0014  lfs f0, 0x14(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240E2D4: D01F0098  stfs f0, 0x98(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(152 as u32), tmp.u32 ) };
	// 8240E2D8: C00B0018  lfs f0, 0x18(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240E2DC: D01F009C  stfs f0, 0x9c(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(156 as u32), tmp.u32 ) };
	pc = 0x8240E2E0; continue 'dispatch;
            }
            0x8240E2E0 => {
    //   block [0x8240E2E0..0x8240E3B4)
	// 8240E2E0: 7EE5BB78  mr r5, r23
	ctx.r[5].u64 = ctx.r[23].u64;
	// 8240E2E4: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 8240E2E8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8240E2EC: 4800196D  bl 0x8240fc58
	ctx.lr = 0x8240E2F0;
	sub_8240FC58(ctx, base);
	// 8240E2F0: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8240E2F4: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 8240E2F8: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 8240E2FC: 4800195D  bl 0x8240fc58
	ctx.lr = 0x8240E300;
	sub_8240FC58(ctx, base);
	// 8240E300: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8240E304: 48001975  bl 0x8240fc78
	ctx.lr = 0x8240E308;
	sub_8240FC78(ctx, base);
	// 8240E308: FF01E000  fcmpu cr6, f1, f28
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[28].f64);
	// 8240E30C: 409900A8  ble cr6, 0x8240e3b4
	if !ctx.cr[6].gt {
	pc = 0x8240E3B4; continue 'dispatch;
	}
	// 8240E310: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 8240E314: 48001965  bl 0x8240fc78
	ctx.lr = 0x8240E318;
	sub_8240FC78(ctx, base);
	// 8240E318: FF01E000  fcmpu cr6, f1, f28
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[28].f64);
	// 8240E31C: 40990098  ble cr6, 0x8240e3b4
	if !ctx.cr[6].gt {
	pc = 0x8240E3B4; continue 'dispatch;
	}
	// 8240E320: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8240E324: 386100A0  addi r3, r1, 0xa0
	ctx.r[3].s64 = ctx.r[1].s64 + 160;
	// 8240E328: 48001939  bl 0x8240fc60
	ctx.lr = 0x8240E32C;
	sub_8240FC60(ctx, base);
	// 8240E32C: 38810080  addi r4, r1, 0x80
	ctx.r[4].s64 = ctx.r[1].s64 + 128;
	// 8240E330: 38610090  addi r3, r1, 0x90
	ctx.r[3].s64 = ctx.r[1].s64 + 144;
	// 8240E334: 4800192D  bl 0x8240fc60
	ctx.lr = 0x8240E338;
	sub_8240FC60(ctx, base);
	// 8240E338: 38810090  addi r4, r1, 0x90
	ctx.r[4].s64 = ctx.r[1].s64 + 144;
	// 8240E33C: 386100A0  addi r3, r1, 0xa0
	ctx.r[3].s64 = ctx.r[1].s64 + 160;
	// 8240E340: 48001931  bl 0x8240fc70
	ctx.lr = 0x8240E344;
	sub_8240FC70(ctx, base);
	// 8240E344: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8240E348: C00B2074  lfs f0, 0x2074(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8308 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240E34C: FF010000  fcmpu cr6, f1, f0
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 8240E350: 419800C8  blt cr6, 0x8240e418
	if ctx.cr[6].lt {
	pc = 0x8240E418; continue 'dispatch;
	}
	// 8240E354: FF01D800  fcmpu cr6, f1, f27
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[27].f64);
	// 8240E358: 419900C0  bgt cr6, 0x8240e418
	if ctx.cr[6].gt {
	pc = 0x8240E418; continue 'dispatch;
	}
	// 8240E35C: 480018F5  bl 0x8240fc50
	ctx.lr = 0x8240E360;
	sub_8240FC50(ctx, base);
	// 8240E360: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8240E364: FFE00890  fmr f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].f64 = ctx.f[1].f64;
	// 8240E368: 48001911  bl 0x8240fc78
	ctx.lr = 0x8240E36C;
	sub_8240FC78(ctx, base);
	// 8240E36C: FFC00890  fmr f30, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[30].f64 = ctx.f[1].f64;
	// 8240E370: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 8240E374: 480018D5  bl 0x8240fc48
	ctx.lr = 0x8240E378;
	sub_8240FC48(ctx, base);
	// 8240E378: C1B9000C  lfs f13, 0xc(r25)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(12 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8240E37C: ED6107B2  fmuls f11, f1, f30
	ctx.f[11].f64 = (((ctx.f[1].f64 * ctx.f[30].f64) as f32) as f64);
	// 8240E380: C0190014  lfs f0, 0x14(r25)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(20 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240E384: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8240E388: EC000372  fmuls f0, f0, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[13].f64) as f32) as f64);
	// 8240E38C: C1B90008  lfs f13, 8(r25)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(8 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8240E390: C1990000  lfs f12, 0(r25)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 8240E394: ED8B0332  fmuls f12, f11, f12
	ctx.f[12].f64 = (((ctx.f[11].f64 * ctx.f[12].f64) as f32) as f64);
	// 8240E398: EDA00372  fmuls f13, f0, f13
	ctx.f[13].f64 = (((ctx.f[0].f64 * ctx.f[13].f64) as f32) as f64);
	// 8240E39C: C00B2238  lfs f0, 0x2238(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8760 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240E3A0: EC0D0032  fmuls f0, f13, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 8240E3A4: EC0C0024  fdivs f0, f12, f0
	ctx.f[0].f64 = ((ctx.f[12].f64 / ctx.f[0].f64) as f32) as f64;
	// 8240E3A8: EC1B0028  fsubs f0, f27, f0
	ctx.f[0].f64 = (((ctx.f[27].f64 - ctx.f[0].f64) as f32) as f64);
	// 8240E3AC: D01F00A0  stfs f0, 0xa0(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(160 as u32), tmp.u32 ) };
	// 8240E3B0: 48000008  b 0x8240e3b8
	pc = 0x8240E3B8; continue 'dispatch;
            }
            0x8240E3B4 => {
    //   block [0x8240E3B4..0x8240E3B8)
	// 8240E3B4: D37F00A0  stfs f27, 0xa0(r31)
	tmp.f32 = (ctx.f[27].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(160 as u32), tmp.u32 ) };
	pc = 0x8240E3B8; continue 'dispatch;
            }
            0x8240E3B8 => {
    //   block [0x8240E3B8..0x8240E3CC)
	// 8240E3B8: C0190004  lfs f0, 4(r25)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240E3BC: C1BF00A0  lfs f13, 0xa0(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(160 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8240E3C0: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 8240E3C4: 40990008  ble cr6, 0x8240e3cc
	if !ctx.cr[6].gt {
	pc = 0x8240E3CC; continue 'dispatch;
	}
	// 8240E3C8: D01F00A0  stfs f0, 0xa0(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(160 as u32), tmp.u32 ) };
	pc = 0x8240E3CC; continue 'dispatch;
            }
            0x8240E3CC => {
    //   block [0x8240E3CC..0x8240E3E4)
	// 8240E3CC: C0190004  lfs f0, 4(r25)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240E3D0: EC1B0024  fdivs f0, f27, f0
	ctx.f[0].f64 = ((ctx.f[27].f64 / ctx.f[0].f64) as f32) as f64;
	// 8240E3D4: C1BF00A0  lfs f13, 0xa0(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(160 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8240E3D8: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 8240E3DC: 4098FBE4  bge cr6, 0x8240dfc0
	if !ctx.cr[6].lt {
	pc = 0x8240DFC0; continue 'dispatch;
	}
	// 8240E3E0: 4BFFFBDC  b 0x8240dfbc
	pc = 0x8240DFBC; continue 'dispatch;
            }
            0x8240E3E4 => {
    //   block [0x8240E3E4..0x8240E400)
	// 8240E3E4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240E3E8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8240E3EC: 386BE730  addi r3, r11, -0x18d0
	ctx.r[3].s64 = ctx.r[11].s64 + -6352;
	// 8240E3F0: 4BEA4B91  bl 0x822b2f80
	ctx.lr = 0x8240E3F4;
	sub_822B2F80(ctx, base);
	// 8240E3F4: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 8240E3F8: 60630034  ori r3, r3, 0x34
	ctx.r[3].u64 = ctx.r[3].u64 | 52;
	// 8240E3FC: 48000020  b 0x8240e41c
	pc = 0x8240E41C; continue 'dispatch;
            }
            0x8240E400 => {
    //   block [0x8240E400..0x8240E40C)
	// 8240E400: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240E404: 386BE6E8  addi r3, r11, -0x1918
	ctx.r[3].s64 = ctx.r[11].s64 + -6424;
	// 8240E408: 4800000C  b 0x8240e414
	pc = 0x8240E414; continue 'dispatch;
            }
            0x8240E40C => {
    //   block [0x8240E40C..0x8240E414)
	// 8240E40C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240E410: 386BE6A0  addi r3, r11, -0x1960
	ctx.r[3].s64 = ctx.r[11].s64 + -6496;
	pc = 0x8240E414; continue 'dispatch;
            }
            0x8240E414 => {
    //   block [0x8240E414..0x8240E418)
	// 8240E414: 4BEA4B6D  bl 0x822b2f80
	ctx.lr = 0x8240E418;
	sub_822B2F80(ctx, base);
	pc = 0x8240E418; continue 'dispatch;
            }
            0x8240E418 => {
    //   block [0x8240E418..0x8240E41C)
	// 8240E418: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	pc = 0x8240E41C; continue 'dispatch;
            }
            0x8240E41C => {
    //   block [0x8240E41C..0x8240E42C)
	// 8240E41C: 382101D0  addi r1, r1, 0x1d0
	ctx.r[1].s64 = ctx.r[1].s64 + 464;
	// 8240E420: 3981FFB0  addi r12, r1, -0x50
	ctx.r[12].s64 = ctx.r[1].s64 + -80;
	// 8240E424: 48127C05  bl 0x82536028
	ctx.lr = 0x8240E428;
	sub_82535FFC(ctx, base);
	// 8240E428: 48126CCC  b 0x825350f4
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240E430(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8240E430 size=56
    let mut pc: u32 = 0x8240E430;
    'dispatch: loop {
        match pc {
            0x8240E430 => {
    //   block [0x8240E430..0x8240E468)
	// 8240E430: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240E434: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240E438: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240E43C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8240E440: FC601090  fmr f3, f2
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[3].f64 = ctx.f[2].f64;
	// 8240E444: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8240E448: FC400890  fmr f2, f1
	ctx.f[2].f64 = ctx.f[1].f64;
	// 8240E44C: 91210054  stw r9, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[9].u32 ) };
	// 8240E450: C02B1FF8  lfs f1, 0x1ff8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8240E454: 4BFFF94D  bl 0x8240dda0
	ctx.lr = 0x8240E458;
	sub_8240DDA0(ctx, base);
	// 8240E458: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8240E45C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240E460: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240E464: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240E468(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8240E468 size=44
    let mut pc: u32 = 0x8240E468;
    'dispatch: loop {
        match pc {
            0x8240E468 => {
    //   block [0x8240E468..0x8240E494)
	// 8240E468: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240E46C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240E470: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240E474: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 8240E478: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 8240E47C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8240E480: 4BFFF921  bl 0x8240dda0
	ctx.lr = 0x8240E484;
	sub_8240DDA0(ctx, base);
	// 8240E484: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8240E488: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240E48C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240E490: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240E498(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8240E498 size=56
    let mut pc: u32 = 0x8240E498;
    'dispatch: loop {
        match pc {
            0x8240E498 => {
    //   block [0x8240E498..0x8240E4D0)
	// 8240E498: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240E49C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240E4A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240E4A4: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8240E4A8: FC601090  fmr f3, f2
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[3].f64 = ctx.f[2].f64;
	// 8240E4AC: 39400006  li r10, 6
	ctx.r[10].s64 = 6;
	// 8240E4B0: FC400890  fmr f2, f1
	ctx.f[2].f64 = ctx.f[1].f64;
	// 8240E4B4: 91210054  stw r9, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[9].u32 ) };
	// 8240E4B8: C02B1FF8  lfs f1, 0x1ff8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8240E4BC: 4BFFF8E5  bl 0x8240dda0
	ctx.lr = 0x8240E4C0;
	sub_8240DDA0(ctx, base);
	// 8240E4C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8240E4C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240E4C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240E4CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240E4D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8240E4D0 size=8
    let mut pc: u32 = 0x8240E4D0;
    'dispatch: loop {
        match pc {
            0x8240E4D0 => {
    //   block [0x8240E4D0..0x8240E4D8)
	// 8240E4D0: 48001AD8  b 0x8240ffa8
	crate::recompiler::externs::call(ctx, base, 0x8240FFA8);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240E4D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8240E4D8 size=8
    let mut pc: u32 = 0x8240E4D8;
    'dispatch: loop {
        match pc {
            0x8240E4D8 => {
    //   block [0x8240E4D8..0x8240E4E0)
	// 8240E4D8: 48001AD8  b 0x8240ffb0
	crate::recompiler::externs::call(ctx, base, 0x8240FFB0);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240E4E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8240E4E0 size=40
    let mut pc: u32 = 0x8240E4E0;
    'dispatch: loop {
        match pc {
            0x8240E4E0 => {
    //   block [0x8240E4E0..0x8240E508)
	// 8240E4E0: 3D600696  lis r11, 0x696
	ctx.r[11].s64 = 110493696;
	// 8240E4E4: 6169542D  ori r9, r11, 0x542d
	ctx.r[9].u64 = ctx.r[11].u64 | 21549;
	// 8240E4E8: 3D608289  lis r11, -0x7d77
	ctx.r[11].s64 = -2104950784;
	// 8240E4EC: 814BF208  lwz r10, -0xdf8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-3576 as u32) ) } as u64;
	// 8240E4F0: 7D4A49D6  mullw r10, r10, r9
	ctx.r[10].s32 = ((ctx.r[10].s32 as i64 * ctx.r[9].s32 as i64) as i32);
	ctx.r[10].s64 = ctx.r[10].s32 as i64;
	// 8240E4F4: 394A3039  addi r10, r10, 0x3039
	ctx.r[10].s64 = ctx.r[10].s64 + 12345;
	// 8240E4F8: 914BF208  stw r10, -0xdf8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-3576 as u32), ctx.r[10].u32 ) };
	// 8240E4FC: A16BF208  lhz r11, -0xdf8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(-3576 as u32) ) } as u64;
	// 8240E500: 5563047E  clrlwi r3, r11, 0x11
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x00007FFFu64;
	// 8240E504: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240E508(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8240E508 size=8
    let mut pc: u32 = 0x8240E508;
    'dispatch: loop {
        match pc {
            0x8240E508 => {
    //   block [0x8240E508..0x8240E510)
	// 8240E508: 38607FFF  li r3, 0x7fff
	ctx.r[3].s64 = 32767;
	// 8240E50C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240E510(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8240E510 size=68
    let mut pc: u32 = 0x8240E510;
    'dispatch: loop {
        match pc {
            0x8240E510 => {
    //   block [0x8240E510..0x8240E540)
	// 8240E510: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240E514: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240E518: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240E51C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8240E520: C00B1FF8  lfs f0, 0x1ff8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240E524: FF010000  fcmpu cr6, f1, f0
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 8240E528: 40990018  ble cr6, 0x8240e540
	if !ctx.cr[6].gt {
	pc = 0x8240E540; continue 'dispatch;
	}
	// 8240E52C: 48001A95  bl 0x8240ffc0
	ctx.lr = 0x8240E530;
	sub_8240FFC0(ctx, base);
	// 8240E530: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8240E534: C00B2938  lfs f0, 0x2938(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(10552 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240E538: EC210032  fmuls f1, f1, f0
	ctx.f[1].f64 = (((ctx.f[1].f64 * ctx.f[0].f64) as f32) as f64);
	// 8240E53C: 48000008  b 0x8240e544
	pc = 0x8240E544; continue 'dispatch;
            }
            0x8240E540 => {
    //   block [0x8240E540..0x8240E544)
	// 8240E540: 4BFF6E51  bl 0x82405390
	ctx.lr = 0x8240E544;
	sub_82405390(ctx, base);
	pc = 0x8240E544; continue 'dispatch;
            }
            0x8240E544 => {
    //   block [0x8240E544..0x8240E554)
	// 8240E544: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8240E548: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240E54C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240E550: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240E558(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8240E558 size=88
    let mut pc: u32 = 0x8240E558;
    'dispatch: loop {
        match pc {
            0x8240E558 => {
    //   block [0x8240E558..0x8240E584)
	// 8240E558: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240E55C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240E560: DBE1FFF0  stfd f31, -0x10(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.f[31].u64 ) };
	// 8240E564: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240E568: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 8240E56C: 4BFF6E25  bl 0x82405390
	ctx.lr = 0x8240E570;
	sub_82405390(ctx, base);
	// 8240E570: FF1F0800  fcmpu cr6, f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[1].f64);
	// 8240E574: 41990010  bgt cr6, 0x8240e584
	if ctx.cr[6].gt {
	pc = 0x8240E584; continue 'dispatch;
	}
	// 8240E578: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8240E57C: C02B1FF8  lfs f1, 0x1ff8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8240E580: 4800001C  b 0x8240e59c
	pc = 0x8240E59C; continue 'dispatch;
            }
            0x8240E584 => {
    //   block [0x8240E584..0x8240E59C)
	// 8240E584: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8240E588: C00B2954  lfs f0, 0x2954(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(10580 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240E58C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8240E590: EC5F0032  fmuls f2, f31, f0
	ctx.f[2].f64 = (((ctx.f[31].f64 * ctx.f[0].f64) as f32) as f64);
	// 8240E594: C02B2934  lfs f1, 0x2934(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(10548 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8240E598: 48001A79  bl 0x82410010
	ctx.lr = 0x8240E59C;
	sub_82410010(ctx, base);
	pc = 0x8240E59C; continue 'dispatch;
            }
            0x8240E59C => {
    //   block [0x8240E59C..0x8240E5B0)
	// 8240E59C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8240E5A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240E5A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240E5A8: CBE1FFF0  lfd f31, -0x10(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8240E5AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240E5B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8240E5B0 size=172
    let mut pc: u32 = 0x8240E5B0;
    'dispatch: loop {
        match pc {
            0x8240E5B0 => {
    //   block [0x8240E5B0..0x8240E5F8)
	// 8240E5B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240E5B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240E5B8: 3981FFF8  addi r12, r1, -8
	ctx.r[12].s64 = ctx.r[1].s64 + -8;
	// 8240E5BC: 48127A29  bl 0x82535fe4
	ctx.lr = 0x8240E5C0;
	sub_82535FB0(ctx, base);
	// 8240E5C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240E5C4: FFE00890  fmr f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].f64 = ctx.f[1].f64;
	// 8240E5C8: FF801090  fmr f28, f2
	ctx.f[28].f64 = ctx.f[2].f64;
	// 8240E5CC: 4BFF6DC5  bl 0x82405390
	ctx.lr = 0x8240E5D0;
	sub_82405390(ctx, base);
	// 8240E5D0: 3D20820D  lis r9, -0x7df3
	ctx.r[9].s64 = -2113077248;
	// 8240E5D4: FF1F0800  fcmpu cr6, f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[1].f64);
	// 8240E5D8: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 8240E5DC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240E5E0: C3691FF8  lfs f27, 0x1ff8(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8184 as u32) ) };
	ctx.f[27].f64 = (tmp.f32 as f64);
	// 8240E5E4: C3AA2934  lfs f29, 0x2934(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(10548 as u32) ) };
	ctx.f[29].f64 = (tmp.f32 as f64);
	// 8240E5E8: C3CB8E30  lfs f30, -0x71d0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-29136 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 8240E5EC: 4199000C  bgt cr6, 0x8240e5f8
	if ctx.cr[6].gt {
	pc = 0x8240E5F8; continue 'dispatch;
	}
	// 8240E5F0: FFE0D890  fmr f31, f27
	ctx.f[31].f64 = ctx.f[27].f64;
	// 8240E5F4: 48000014  b 0x8240e608
	pc = 0x8240E608; continue 'dispatch;
            }
            0x8240E5F8 => {
    //   block [0x8240E5F8..0x8240E608)
	// 8240E5F8: EC5F07B2  fmuls f2, f31, f30
	ctx.f[2].f64 = (((ctx.f[31].f64 * ctx.f[30].f64) as f32) as f64);
	// 8240E5FC: FC20E890  fmr f1, f29
	ctx.f[1].f64 = ctx.f[29].f64;
	// 8240E600: 48001A11  bl 0x82410010
	ctx.lr = 0x8240E604;
	sub_82410010(ctx, base);
	// 8240E604: FFE00890  fmr f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].f64 = ctx.f[1].f64;
	pc = 0x8240E608; continue 'dispatch;
            }
            0x8240E608 => {
    //   block [0x8240E608..0x8240E61C)
	// 8240E608: 4BFF6D89  bl 0x82405390
	ctx.lr = 0x8240E60C;
	sub_82405390(ctx, base);
	// 8240E60C: FF1C0800  fcmpu cr6, f28, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.cr[6].compare_f64(ctx.f[28].f64, ctx.f[1].f64);
	// 8240E610: 4199000C  bgt cr6, 0x8240e61c
	if ctx.cr[6].gt {
	pc = 0x8240E61C; continue 'dispatch;
	}
	// 8240E614: FC20D890  fmr f1, f27
	ctx.f[1].f64 = ctx.f[27].f64;
	// 8240E618: 48000010  b 0x8240e628
	pc = 0x8240E628; continue 'dispatch;
            }
            0x8240E61C => {
    //   block [0x8240E61C..0x8240E628)
	// 8240E61C: EC5C07B2  fmuls f2, f28, f30
	ctx.f[2].f64 = (((ctx.f[28].f64 * ctx.f[30].f64) as f32) as f64);
	// 8240E620: FC20E890  fmr f1, f29
	ctx.f[1].f64 = ctx.f[29].f64;
	// 8240E624: 480019ED  bl 0x82410010
	ctx.lr = 0x8240E628;
	sub_82410010(ctx, base);
	pc = 0x8240E628; continue 'dispatch;
            }
            0x8240E628 => {
    //   block [0x8240E628..0x8240E640)
	// 8240E628: EC21F82A  fadds f1, f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ((ctx.f[1].f64 + ctx.f[31].f64) as f32) as f64;
	// 8240E62C: FF01D800  fcmpu cr6, f1, f27
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[27].f64);
	// 8240E630: 40990010  ble cr6, 0x8240e640
	if !ctx.cr[6].gt {
	pc = 0x8240E640; continue 'dispatch;
	}
	// 8240E634: 4800198D  bl 0x8240ffc0
	ctx.lr = 0x8240E638;
	sub_8240FFC0(ctx, base);
	// 8240E638: EC210772  fmuls f1, f1, f29
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = (((ctx.f[1].f64 * ctx.f[29].f64) as f32) as f64);
	// 8240E63C: 48000008  b 0x8240e644
	pc = 0x8240E644; continue 'dispatch;
            }
            0x8240E640 => {
    //   block [0x8240E640..0x8240E644)
	// 8240E640: 4BFF6D51  bl 0x82405390
	ctx.lr = 0x8240E644;
	sub_82405390(ctx, base);
	pc = 0x8240E644; continue 'dispatch;
            }
            0x8240E644 => {
    //   block [0x8240E644..0x8240E65C)
	// 8240E644: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8240E648: 3981FFF8  addi r12, r1, -8
	ctx.r[12].s64 = ctx.r[1].s64 + -8;
	// 8240E64C: 481279E5  bl 0x82536030
	ctx.lr = 0x8240E650;
	sub_82535FFC(ctx, base);
	// 8240E650: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240E654: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240E658: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240E660(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8240E660 size=56
    let mut pc: u32 = 0x8240E660;
    'dispatch: loop {
        match pc {
            0x8240E660 => {
    //   block [0x8240E660..0x8240E698)
	// 8240E660: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240E664: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240E668: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240E66C: 4800197D  bl 0x8240ffe8
	ctx.lr = 0x8240E670;
	sub_8240FFE8(ctx, base);
	// 8240E670: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240E674: C00BE8B4  lfs f0, -0x174c(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-5964 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240E678: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240E67C: EDA10032  fmuls f13, f1, f0
	ctx.f[13].f64 = (((ctx.f[1].f64 * ctx.f[0].f64) as f32) as f64);
	// 8240E680: C00BE8B8  lfs f0, -0x1748(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-5960 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240E684: EC2D0032  fmuls f1, f13, f0
	ctx.f[1].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 8240E688: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8240E68C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240E690: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240E694: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240E698(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8240E698 size=144
    let mut pc: u32 = 0x8240E698;
    'dispatch: loop {
        match pc {
            0x8240E698 => {
    //   block [0x8240E698..0x8240E6A4)
	// 8240E698: 39632204  addi r11, r3, 0x2204
	ctx.r[11].s64 = ctx.r[3].s64 + 8708;
	// 8240E69C: 39230014  addi r9, r3, 0x14
	ctx.r[9].s64 = ctx.r[3].s64 + 20;
	// 8240E6A0: 39000040  li r8, 0x40
	ctx.r[8].s64 = 64;
	pc = 0x8240E6A4; continue 'dispatch;
            }
            0x8240E6A4 => {
    //   block [0x8240E6A4..0x8240E6D0)
	// 8240E6A4: 8149FFEC  lwz r10, -0x14(r9)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-20 as u32) ) } as u64;
	// 8240E6A8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8240E6AC: 419A0060  beq cr6, 0x8240e70c
	if ctx.cr[6].eq {
	pc = 0x8240E70C; continue 'dispatch;
	}
	// 8240E6B0: 81490000  lwz r10, 0(r9)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 8240E6B4: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8240E6B8: 41820054  beq 0x8240e70c
	if ctx.cr[0].eq {
	pc = 0x8240E70C; continue 'dispatch;
	}
	// 8240E6BC: 2B050001  cmplwi cr6, r5, 1
	ctx.cr[6].compare_u32(ctx.r[5].u32, 1 as u32, &mut ctx.xer);
	// 8240E6C0: 409A0010  bne cr6, 0x8240e6d0
	if !ctx.cr[6].eq {
	pc = 0x8240E6D0; continue 'dispatch;
	}
	// 8240E6C4: 814A004C  lwz r10, 0x4c(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(76 as u32) ) } as u64;
	// 8240E6C8: 554A07FF  clrlwi. r10, r10, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8240E6CC: 40820040  bne 0x8240e70c
	if !ctx.cr[0].eq {
	pc = 0x8240E70C; continue 'dispatch;
	}
	pc = 0x8240E6D0; continue 'dispatch;
            }
            0x8240E6D0 => {
    //   block [0x8240E6D0..0x8240E6EC)
	// 8240E6D0: 814BFFFC  lwz r10, -4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-4 as u32) ) } as u64;
	// 8240E6D4: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8240E6D8: 41980020  blt cr6, 0x8240e6f8
	if ctx.cr[6].lt {
	pc = 0x8240E6F8; continue 'dispatch;
	}
	// 8240E6DC: 81460000  lwz r10, 0(r6)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 8240E6E0: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 8240E6E4: 409A0008  bne cr6, 0x8240e6ec
	if !ctx.cr[6].eq {
	pc = 0x8240E6EC; continue 'dispatch;
	}
	// 8240E6E8: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	pc = 0x8240E6EC; continue 'dispatch;
            }
            0x8240E6EC => {
    //   block [0x8240E6EC..0x8240E6F8)
	// 8240E6EC: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8240E6F0: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 8240E6F4: 409A0018  bne cr6, 0x8240e70c
	if !ctx.cr[6].eq {
	pc = 0x8240E70C; continue 'dispatch;
	}
	pc = 0x8240E6F8; continue 'dispatch;
            }
            0x8240E6F8 => {
    //   block [0x8240E6F8..0x8240E70C)
	// 8240E6F8: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8240E6FC: 2C0A0000  cmpwi r10, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8240E700: 4081000C  ble 0x8240e70c
	if !ctx.cr[0].gt {
	pc = 0x8240E70C; continue 'dispatch;
	}
	// 8240E704: 7D445050  subf r10, r4, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[4].s64;
	// 8240E708: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	pc = 0x8240E70C; continue 'dispatch;
            }
            0x8240E70C => {
    //   block [0x8240E70C..0x8240E728)
	// 8240E70C: 3508FFFF  addic. r8, r8, -1
	ctx.xer.ca = (ctx.r[8].u32 > (!(-1 as u32)));
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 8240E710: 39290088  addi r9, r9, 0x88
	ctx.r[9].s64 = ctx.r[9].s64 + 136;
	// 8240E714: 38C60004  addi r6, r6, 4
	ctx.r[6].s64 = ctx.r[6].s64 + 4;
	// 8240E718: 396B000C  addi r11, r11, 0xc
	ctx.r[11].s64 = ctx.r[11].s64 + 12;
	// 8240E71C: 4082FF88  bne 0x8240e6a4
	if !ctx.cr[0].eq {
	pc = 0x8240E6A4; continue 'dispatch;
	}
	// 8240E720: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8240E724: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240E728(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8240E728 size=100
    let mut pc: u32 = 0x8240E728;
    'dispatch: loop {
        match pc {
            0x8240E728 => {
    //   block [0x8240E728..0x8240E76C)
	// 8240E728: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240E72C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240E730: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240E734: 2B04003F  cmplwi cr6, r4, 0x3f
	ctx.cr[6].compare_u32(ctx.r[4].u32, 63 as u32, &mut ctx.xer);
	// 8240E738: 41990034  bgt cr6, 0x8240e76c
	if ctx.cr[6].gt {
	pc = 0x8240E76C; continue 'dispatch;
	}
	// 8240E73C: 1D640088  mulli r11, r4, 0x88
	ctx.r[11].s32 = ((ctx.r[4].s32 as i64 * 136 as i64) as i32);
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 8240E740: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 8240E744: 1D44000C  mulli r10, r4, 0xc
	ctx.r[10].s32 = ((ctx.r[4].s32 as i64 * 12 as i64) as i32);
	ctx.r[10].s64 = ctx.r[10].s32 as i64;
	// 8240E748: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8240E74C: 7D4A1A14  add r10, r10, r3
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[3].u64;
	// 8240E750: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8240E754: 91250000  stw r9, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8240E758: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8240E75C: 91650004  stw r11, 4(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8240E760: 816A2200  lwz r11, 0x2200(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8704 as u32) ) } as u64;
	// 8240E764: 91650008  stw r11, 8(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8240E768: 48000014  b 0x8240e77c
	pc = 0x8240E77C; continue 'dispatch;
            }
            0x8240E76C => {
    //   block [0x8240E76C..0x8240E77C)
	// 8240E76C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240E770: 386BE8BC  addi r3, r11, -0x1744
	ctx.r[3].s64 = ctx.r[11].s64 + -5956;
	// 8240E774: 4BEA480D  bl 0x822b2f80
	ctx.lr = 0x8240E778;
	sub_822B2F80(ctx, base);
	// 8240E778: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	pc = 0x8240E77C; continue 'dispatch;
            }
            0x8240E77C => {
    //   block [0x8240E77C..0x8240E78C)
	// 8240E77C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8240E780: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240E784: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240E788: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240E790(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8240E790 size=140
    let mut pc: u32 = 0x8240E790;
    'dispatch: loop {
        match pc {
            0x8240E790 => {
    //   block [0x8240E790..0x8240E7B0)
	// 8240E790: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240E794: 48126925  bl 0x825350b8
	ctx.lr = 0x8240E798;
	sub_82535080(ctx, base);
	// 8240E798: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240E79C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8240E7A0: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8240E7A4: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 8240E7A8: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8240E7AC: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	pc = 0x8240E7B0; continue 'dispatch;
            }
            0x8240E7B0 => {
    //   block [0x8240E7B0..0x8240E7D0)
	// 8240E7B0: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8240E7B4: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8240E7B8: 419A0018  beq cr6, 0x8240e7d0
	if ctx.cr[6].eq {
	pc = 0x8240E7D0; continue 'dispatch;
	}
	// 8240E7BC: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 8240E7C0: 396B0088  addi r11, r11, 0x88
	ctx.r[11].s64 = ctx.r[11].s64 + 136;
	// 8240E7C4: 2F1F0040  cmpwi cr6, r31, 0x40
	ctx.cr[6].compare_i32(ctx.r[31].s32, 64, &mut ctx.xer);
	// 8240E7C8: 4198FFE8  blt cr6, 0x8240e7b0
	if ctx.cr[6].lt {
	pc = 0x8240E7B0; continue 'dispatch;
	}
	// 8240E7CC: 4800001C  b 0x8240e7e8
	pc = 0x8240E7E8; continue 'dispatch;
            }
            0x8240E7D0 => {
    //   block [0x8240E7D0..0x8240E7E8)
	// 8240E7D0: 1D7F0088  mulli r11, r31, 0x88
	ctx.r[11].s32 = ((ctx.r[31].s32 as i64 * 136 as i64) as i32);
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 8240E7D4: 7C6BF214  add r3, r11, r30
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 8240E7D8: 38A00088  li r5, 0x88
	ctx.r[5].s64 = 136;
	// 8240E7DC: 48126375  bl 0x82534b50
	ctx.lr = 0x8240E7E0;
	sub_82534B50(ctx, base);
	// 8240E7E0: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 8240E7E4: 4098000C  bge cr6, 0x8240e7f0
	if !ctx.cr[6].lt {
	pc = 0x8240E7F0; continue 'dispatch;
	}
	pc = 0x8240E7E8; continue 'dispatch;
            }
            0x8240E7E8 => {
    //   block [0x8240E7E8..0x8240E7F0)
	// 8240E7E8: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 8240E7EC: 48000028  b 0x8240e814
	pc = 0x8240E814; continue 'dispatch;
            }
            0x8240E7F0 => {
    //   block [0x8240E7F0..0x8240E814)
	// 8240E7F0: 1D7F000C  mulli r11, r31, 0xc
	ctx.r[11].s32 = ((ctx.r[31].s32 as i64 * 12 as i64) as i32);
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 8240E7F4: 395F02D6  addi r10, r31, 0x2d6
	ctx.r[10].s64 = ctx.r[31].s64 + 726;
	// 8240E7F8: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 8240E7FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8240E800: 1D4A000C  mulli r10, r10, 0xc
	ctx.r[10].s32 = ((ctx.r[10].s32 as i64 * 12 as i64) as i32);
	ctx.r[10].s64 = ctx.r[10].s32 as i64;
	// 8240E804: 93AB2204  stw r29, 0x2204(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8708 as u32), ctx.r[29].u32 ) };
	// 8240E808: 7D2AF12E  stwx r9, r10, r30
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[30].u32), ctx.r[9].u32) };
	// 8240E80C: 938B2200  stw r28, 0x2200(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8704 as u32), ctx.r[28].u32 ) };
	// 8240E810: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	pc = 0x8240E814; continue 'dispatch;
            }
            0x8240E814 => {
    //   block [0x8240E814..0x8240E81C)
	// 8240E814: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8240E818: 481268F0  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240E820(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8240E820 size=44
    let mut pc: u32 = 0x8240E820;
    'dispatch: loop {
        match pc {
            0x8240E820 => {
    //   block [0x8240E820..0x8240E828)
	// 8240E820: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8240E824: 39400040  li r10, 0x40
	ctx.r[10].s64 = 64;
	pc = 0x8240E828; continue 'dispatch;
            }
            0x8240E828 => {
    //   block [0x8240E828..0x8240E838)
	// 8240E828: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8240E82C: 7F044840  cmplw cr6, r4, r9
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[9].u32, &mut ctx.xer);
	// 8240E830: 409A0008  bne cr6, 0x8240e838
	if !ctx.cr[6].eq {
	pc = 0x8240E838; continue 'dispatch;
	}
	// 8240E834: D02B0070  stfs f1, 0x70(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(112 as u32), tmp.u32 ) };
	pc = 0x8240E838; continue 'dispatch;
            }
            0x8240E838 => {
    //   block [0x8240E838..0x8240E84C)
	// 8240E838: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8240E83C: 396B0088  addi r11, r11, 0x88
	ctx.r[11].s64 = ctx.r[11].s64 + 136;
	// 8240E840: 4082FFE8  bne 0x8240e828
	if !ctx.cr[0].eq {
	pc = 0x8240E828; continue 'dispatch;
	}
	// 8240E844: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8240E848: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240E850(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8240E850 size=44
    let mut pc: u32 = 0x8240E850;
    'dispatch: loop {
        match pc {
            0x8240E850 => {
    //   block [0x8240E850..0x8240E858)
	// 8240E850: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8240E854: 39400040  li r10, 0x40
	ctx.r[10].s64 = 64;
	pc = 0x8240E858; continue 'dispatch;
            }
            0x8240E858 => {
    //   block [0x8240E858..0x8240E868)
	// 8240E858: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8240E85C: 7F044840  cmplw cr6, r4, r9
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[9].u32, &mut ctx.xer);
	// 8240E860: 409A0008  bne cr6, 0x8240e868
	if !ctx.cr[6].eq {
	pc = 0x8240E868; continue 'dispatch;
	}
	// 8240E864: D02B0074  stfs f1, 0x74(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(116 as u32), tmp.u32 ) };
	pc = 0x8240E868; continue 'dispatch;
            }
            0x8240E868 => {
    //   block [0x8240E868..0x8240E87C)
	// 8240E868: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8240E86C: 396B0088  addi r11, r11, 0x88
	ctx.r[11].s64 = ctx.r[11].s64 + 136;
	// 8240E870: 4082FFE8  bne 0x8240e858
	if !ctx.cr[0].eq {
	pc = 0x8240E858; continue 'dispatch;
	}
	// 8240E874: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8240E878: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240E880(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8240E880 size=44
    let mut pc: u32 = 0x8240E880;
    'dispatch: loop {
        match pc {
            0x8240E880 => {
    //   block [0x8240E880..0x8240E888)
	// 8240E880: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8240E884: 39400040  li r10, 0x40
	ctx.r[10].s64 = 64;
	pc = 0x8240E888; continue 'dispatch;
            }
            0x8240E888 => {
    //   block [0x8240E888..0x8240E898)
	// 8240E888: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8240E88C: 7F044840  cmplw cr6, r4, r9
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[9].u32, &mut ctx.xer);
	// 8240E890: 409A0008  bne cr6, 0x8240e898
	if !ctx.cr[6].eq {
	pc = 0x8240E898; continue 'dispatch;
	}
	// 8240E894: 90AB0078  stw r5, 0x78(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(120 as u32), ctx.r[5].u32 ) };
	pc = 0x8240E898; continue 'dispatch;
            }
            0x8240E898 => {
    //   block [0x8240E898..0x8240E8AC)
	// 8240E898: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8240E89C: 396B0088  addi r11, r11, 0x88
	ctx.r[11].s64 = ctx.r[11].s64 + 136;
	// 8240E8A0: 4082FFE8  bne 0x8240e888
	if !ctx.cr[0].eq {
	pc = 0x8240E888; continue 'dispatch;
	}
	// 8240E8A4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8240E8A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240E8B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8240E8B0 size=80
    let mut pc: u32 = 0x8240E8B0;
    'dispatch: loop {
        match pc {
            0x8240E8B0 => {
    //   block [0x8240E8B0..0x8240E8CC)
	// 8240E8B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240E8B4: 48126805  bl 0x825350b8
	ctx.lr = 0x8240E8B8;
	sub_82535080(ctx, base);
	// 8240E8B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240E8BC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8240E8C0: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8240E8C4: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 8240E8C8: 3BC00040  li r30, 0x40
	ctx.r[30].s64 = 64;
	pc = 0x8240E8CC; continue 'dispatch;
            }
            0x8240E8CC => {
    //   block [0x8240E8CC..0x8240E8E8)
	// 8240E8CC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8240E8D0: 7F1D5840  cmplw cr6, r29, r11
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8240E8D4: 409A0014  bne cr6, 0x8240e8e8
	if !ctx.cr[6].eq {
	pc = 0x8240E8E8; continue 'dispatch;
	}
	// 8240E8D8: 387F0070  addi r3, r31, 0x70
	ctx.r[3].s64 = ctx.r[31].s64 + 112;
	// 8240E8DC: 38A00018  li r5, 0x18
	ctx.r[5].s64 = 24;
	// 8240E8E0: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8240E8E4: 4812626D  bl 0x82534b50
	ctx.lr = 0x8240E8E8;
	sub_82534B50(ctx, base);
	pc = 0x8240E8E8; continue 'dispatch;
            }
            0x8240E8E8 => {
    //   block [0x8240E8E8..0x8240E900)
	// 8240E8E8: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8240E8EC: 3BFF0088  addi r31, r31, 0x88
	ctx.r[31].s64 = ctx.r[31].s64 + 136;
	// 8240E8F0: 4082FFDC  bne 0x8240e8cc
	if !ctx.cr[0].eq {
	pc = 0x8240E8CC; continue 'dispatch;
	}
	// 8240E8F4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8240E8F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8240E8FC: 4812680C  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240E900(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8240E900 size=116
    let mut pc: u32 = 0x8240E900;
    'dispatch: loop {
        match pc {
            0x8240E900 => {
    //   block [0x8240E900..0x8240E920)
	// 8240E900: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240E904: 481267B1  bl 0x825350b4
	ctx.lr = 0x8240E908;
	sub_82535080(ctx, base);
	// 8240E908: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240E90C: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 8240E910: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8240E914: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 8240E918: 3BE30054  addi r31, r3, 0x54
	ctx.r[31].s64 = ctx.r[3].s64 + 84;
	// 8240E91C: 3BC00040  li r30, 0x40
	ctx.r[30].s64 = 64;
	pc = 0x8240E920; continue 'dispatch;
            }
            0x8240E920 => {
    //   block [0x8240E920..0x8240E944)
	// 8240E920: 817FFFAC  lwz r11, -0x54(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-84 as u32) ) } as u64;
	// 8240E924: 7F1B5840  cmplw cr6, r27, r11
	ctx.cr[6].compare_u32(ctx.r[27].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8240E928: 409A0034  bne cr6, 0x8240e95c
	if !ctx.cr[6].eq {
	pc = 0x8240E95C; continue 'dispatch;
	}
	// 8240E92C: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 8240E930: 419A0014  beq cr6, 0x8240e944
	if ctx.cr[6].eq {
	pc = 0x8240E944; continue 'dispatch;
	}
	// 8240E934: 38A00034  li r5, 0x34
	ctx.r[5].s64 = 52;
	// 8240E938: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8240E93C: 387FFFCC  addi r3, r31, -0x34
	ctx.r[3].s64 = ctx.r[31].s64 + -52;
	// 8240E940: 4BFB22C1  bl 0x823c0c00
	ctx.lr = 0x8240E944;
	sub_823C0C00(ctx, base);
	pc = 0x8240E944; continue 'dispatch;
            }
            0x8240E944 => {
    //   block [0x8240E944..0x8240E95C)
	// 8240E944: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 8240E948: 419A0014  beq cr6, 0x8240e95c
	if ctx.cr[6].eq {
	pc = 0x8240E95C; continue 'dispatch;
	}
	// 8240E94C: 38A0001C  li r5, 0x1c
	ctx.r[5].s64 = 28;
	// 8240E950: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8240E954: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8240E958: 4BFB22A9  bl 0x823c0c00
	ctx.lr = 0x8240E95C;
	sub_823C0C00(ctx, base);
	pc = 0x8240E95C; continue 'dispatch;
            }
            0x8240E95C => {
    //   block [0x8240E95C..0x8240E974)
	// 8240E95C: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8240E960: 3BFF0088  addi r31, r31, 0x88
	ctx.r[31].s64 = ctx.r[31].s64 + 136;
	// 8240E964: 4082FFBC  bne 0x8240e920
	if !ctx.cr[0].eq {
	pc = 0x8240E920; continue 'dispatch;
	}
	// 8240E968: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8240E96C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8240E970: 48126794  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240E978(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8240E978 size=44
    let mut pc: u32 = 0x8240E978;
    'dispatch: loop {
        match pc {
            0x8240E978 => {
    //   block [0x8240E978..0x8240E984)
	// 8240E978: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8240E97C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8240E980: 39400040  li r10, 0x40
	ctx.r[10].s64 = 64;
	pc = 0x8240E984; continue 'dispatch;
            }
            0x8240E984 => {
    //   block [0x8240E984..0x8240E994)
	// 8240E984: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8240E988: 7F044840  cmplw cr6, r4, r9
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[9].u32, &mut ctx.xer);
	// 8240E98C: 409A0008  bne cr6, 0x8240e994
	if !ctx.cr[6].eq {
	pc = 0x8240E994; continue 'dispatch;
	}
	// 8240E990: 38630001  addi r3, r3, 1
	ctx.r[3].s64 = ctx.r[3].s64 + 1;
	pc = 0x8240E994; continue 'dispatch;
            }
            0x8240E994 => {
    //   block [0x8240E994..0x8240E9A4)
	// 8240E994: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8240E998: 396B0088  addi r11, r11, 0x88
	ctx.r[11].s64 = ctx.r[11].s64 + 136;
	// 8240E99C: 4082FFE8  bne 0x8240e984
	if !ctx.cr[0].eq {
	pc = 0x8240E984; continue 'dispatch;
	}
	// 8240E9A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240E9A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8240E9A8 size=180
    let mut pc: u32 = 0x8240E9A8;
    'dispatch: loop {
        match pc {
            0x8240E9A8 => {
    //   block [0x8240E9A8..0x8240EA3C)
	// 8240E9A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240E9AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240E9B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240E9B4: 7C681B78  mr r8, r3
	ctx.r[8].u64 = ctx.r[3].u64;
	// 8240E9B8: 2B04003F  cmplwi cr6, r4, 0x3f
	ctx.cr[6].compare_u32(ctx.r[4].u32, 63 as u32, &mut ctx.xer);
	// 8240E9BC: 41990080  bgt cr6, 0x8240ea3c
	if ctx.cr[6].gt {
	pc = 0x8240EA3C; continue 'dispatch;
	}
	// 8240E9C0: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 8240E9C4: 1D44000C  mulli r10, r4, 0xc
	ctx.r[10].s32 = ((ctx.r[4].s32 as i64 * 12 as i64) as i32);
	ctx.r[10].s64 = ctx.r[10].s32 as i64;
	// 8240E9C8: C0091850  lfs f0, 0x1850(r9)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(6224 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240E9CC: 3D20820D  lis r9, -0x7df3
	ctx.r[9].s64 = -2113077248;
	// 8240E9D0: 1D640088  mulli r11, r4, 0x88
	ctx.r[11].s32 = ((ctx.r[4].s32 as i64 * 136 as i64) as i32);
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 8240E9D4: C1A91FF8  lfs f13, 0x1ff8(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8184 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8240E9D8: 38C402D6  addi r6, r4, 0x2d6
	ctx.r[6].s64 = ctx.r[4].s64 + 726;
	// 8240E9DC: 7CEA4214  add r7, r10, r8
	ctx.r[7].u64 = ctx.r[10].u64 + ctx.r[8].u64;
	// 8240E9E0: 7D6B4214  add r11, r11, r8
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[8].u64;
	// 8240E9E4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8240E9E8: 3920FFFF  li r9, -1
	ctx.r[9].s64 = -1;
	// 8240E9EC: 1CC6000C  mulli r6, r6, 0xc
	ctx.r[6].s32 = ((ctx.r[6].s32 as i64 * 12 as i64) as i32);
	ctx.r[6].s64 = ctx.r[6].s32 as i64;
	// 8240E9F0: D00B0070  stfs f0, 0x70(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(112 as u32), tmp.u32 ) };
	// 8240E9F4: D1AB0074  stfs f13, 0x74(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(116 as u32), tmp.u32 ) };
	// 8240E9F8: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8240E9FC: D1AB007C  stfs f13, 0x7c(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(124 as u32), tmp.u32 ) };
	// 8240EA00: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 8240EA04: D00B0080  stfs f0, 0x80(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(128 as u32), tmp.u32 ) };
	// 8240EA08: 912B0008  stw r9, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 8240EA0C: D00B0084  stfs f0, 0x84(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(132 as u32), tmp.u32 ) };
	// 8240EA10: 912B000C  stw r9, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 8240EA14: 914B0010  stw r10, 0x10(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 8240EA18: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8240EA1C: 914B0014  stw r10, 0x14(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 8240EA20: 912B0018  stw r9, 0x18(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[9].u32 ) };
	// 8240EA24: 914B001C  stw r10, 0x1c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(28 as u32), ctx.r[10].u32 ) };
	// 8240EA28: 914B0078  stw r10, 0x78(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(120 as u32), ctx.r[10].u32 ) };
	// 8240EA2C: 91272200  stw r9, 0x2200(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(8704 as u32), ctx.r[9].u32 ) };
	// 8240EA30: 91472204  stw r10, 0x2204(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(8708 as u32), ctx.r[10].u32 ) };
	// 8240EA34: 7D46412E  stwx r10, r6, r8
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[6].u32.wrapping_add(ctx.r[8].u32), ctx.r[10].u32) };
	// 8240EA38: 48000014  b 0x8240ea4c
	pc = 0x8240EA4C; continue 'dispatch;
            }
            0x8240EA3C => {
    //   block [0x8240EA3C..0x8240EA4C)
	// 8240EA3C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240EA40: 386BE8F4  addi r3, r11, -0x170c
	ctx.r[3].s64 = ctx.r[11].s64 + -5900;
	// 8240EA44: 4BEA453D  bl 0x822b2f80
	ctx.lr = 0x8240EA48;
	sub_822B2F80(ctx, base);
	// 8240EA48: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	pc = 0x8240EA4C; continue 'dispatch;
            }
            0x8240EA4C => {
    //   block [0x8240EA4C..0x8240EA5C)
	// 8240EA4C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8240EA50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240EA54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240EA58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240EA60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8240EA60 size=176
    let mut pc: u32 = 0x8240EA60;
    'dispatch: loop {
        match pc {
            0x8240EA60 => {
    //   block [0x8240EA60..0x8240EA84)
	// 8240EA60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240EA64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240EA68: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8240EA6C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8240EA70: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240EA74: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8240EA78: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8240EA7C: 7FCAF378  mr r10, r30
	ctx.r[10].u64 = ctx.r[30].u64;
	// 8240EA80: 397E2200  addi r11, r30, 0x2200
	ctx.r[11].s64 = ctx.r[30].s64 + 8704;
	pc = 0x8240EA84; continue 'dispatch;
            }
            0x8240EA84 => {
    //   block [0x8240EA84..0x8240EAB4)
	// 8240EA84: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8240EA88: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 8240EA8C: 419A0028  beq cr6, 0x8240eab4
	if ctx.cr[6].eq {
	pc = 0x8240EAB4; continue 'dispatch;
	}
	// 8240EA90: 812B0004  lwz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8240EA94: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8240EA98: 4199001C  bgt cr6, 0x8240eab4
	if ctx.cr[6].gt {
	pc = 0x8240EAB4; continue 'dispatch;
	}
	// 8240EA9C: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8240EAA0: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8240EAA4: 41980040  blt cr6, 0x8240eae4
	if ctx.cr[6].lt {
	pc = 0x8240EAE4; continue 'dispatch;
	}
	// 8240EAA8: 812B0008  lwz r9, 8(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8240EAAC: 2B090001  cmplwi cr6, r9, 1
	ctx.cr[6].compare_u32(ctx.r[9].u32, 1 as u32, &mut ctx.xer);
	// 8240EAB0: 419A0034  beq cr6, 0x8240eae4
	if ctx.cr[6].eq {
	pc = 0x8240EAE4; continue 'dispatch;
	}
	pc = 0x8240EAB4; continue 'dispatch;
            }
            0x8240EAB4 => {
    //   block [0x8240EAB4..0x8240EACC)
	// 8240EAB4: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 8240EAB8: 394A0088  addi r10, r10, 0x88
	ctx.r[10].s64 = ctx.r[10].s64 + 136;
	// 8240EABC: 396B000C  addi r11, r11, 0xc
	ctx.r[11].s64 = ctx.r[11].s64 + 12;
	// 8240EAC0: 2F1F0040  cmpwi cr6, r31, 0x40
	ctx.cr[6].compare_i32(ctx.r[31].s32, 64, &mut ctx.xer);
	// 8240EAC4: 4198FFC0  blt cr6, 0x8240ea84
	if ctx.cr[6].lt {
	pc = 0x8240EA84; continue 'dispatch;
	}
	// 8240EAC8: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	pc = 0x8240EACC; continue 'dispatch;
            }
            0x8240EACC => {
    //   block [0x8240EACC..0x8240EAE4)
	// 8240EACC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8240EAD0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240EAD4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240EAD8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8240EADC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8240EAE0: 4E800020  blr
	return;
            }
            0x8240EAE4 => {
    //   block [0x8240EAE4..0x8240EB10)
	// 8240EAE4: 1D7F0088  mulli r11, r31, 0x88
	ctx.r[11].s32 = ((ctx.r[31].s32 as i64 * 136 as i64) as i32);
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 8240EAE8: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 8240EAEC: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 8240EAF0: 38A00088  li r5, 0x88
	ctx.r[5].s64 = 136;
	// 8240EAF4: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 8240EAF8: 48126059  bl 0x82534b50
	ctx.lr = 0x8240EAFC;
	sub_82534B50(ctx, base);
	// 8240EAFC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8240EB00: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8240EB04: 4BFFFEA5  bl 0x8240e9a8
	ctx.lr = 0x8240EB08;
	sub_8240E9A8(ctx, base);
	// 8240EB08: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8240EB0C: 4BFFFFC0  b 0x8240eacc
	pc = 0x8240EACC; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240EB10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8240EB10 size=112
    let mut pc: u32 = 0x8240EB10;
    'dispatch: loop {
        match pc {
            0x8240EB10 => {
    //   block [0x8240EB10..0x8240EB30)
	// 8240EB10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240EB14: 481265A1  bl 0x825350b4
	ctx.lr = 0x8240EB18;
	sub_82535080(ctx, base);
	// 8240EB18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240EB1C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8240EB20: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 8240EB24: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8240EB28: 7FDDF378  mr r29, r30
	ctx.r[29].u64 = ctx.r[30].u64;
	// 8240EB2C: 3B9E2208  addi r28, r30, 0x2208
	ctx.r[28].s64 = ctx.r[30].s64 + 8712;
	pc = 0x8240EB30; continue 'dispatch;
            }
            0x8240EB30 => {
    //   block [0x8240EB30..0x8240EB54)
	// 8240EB30: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8240EB34: 7F1B5840  cmplw cr6, r27, r11
	ctx.cr[6].compare_u32(ctx.r[27].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8240EB38: 409A0028  bne cr6, 0x8240eb60
	if !ctx.cr[6].eq {
	pc = 0x8240EB60; continue 'dispatch;
	}
	// 8240EB3C: 817CFFF8  lwz r11, -8(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240EB40: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8240EB44: 41980010  blt cr6, 0x8240eb54
	if ctx.cr[6].lt {
	pc = 0x8240EB54; continue 'dispatch;
	}
	// 8240EB48: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8240EB4C: 917C0000  stw r11, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8240EB50: 48000010  b 0x8240eb60
	pc = 0x8240EB60; continue 'dispatch;
            }
            0x8240EB54 => {
    //   block [0x8240EB54..0x8240EB60)
	// 8240EB54: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8240EB58: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8240EB5C: 4BFFFE4D  bl 0x8240e9a8
	ctx.lr = 0x8240EB60;
	sub_8240E9A8(ctx, base);
	pc = 0x8240EB60; continue 'dispatch;
            }
            0x8240EB60 => {
    //   block [0x8240EB60..0x8240EB80)
	// 8240EB60: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 8240EB64: 3BBD0088  addi r29, r29, 0x88
	ctx.r[29].s64 = ctx.r[29].s64 + 136;
	// 8240EB68: 3B9C000C  addi r28, r28, 0xc
	ctx.r[28].s64 = ctx.r[28].s64 + 12;
	// 8240EB6C: 2F1F0040  cmpwi cr6, r31, 0x40
	ctx.cr[6].compare_i32(ctx.r[31].s32, 64, &mut ctx.xer);
	// 8240EB70: 4198FFC0  blt cr6, 0x8240eb30
	if ctx.cr[6].lt {
	pc = 0x8240EB30; continue 'dispatch;
	}
	// 8240EB74: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8240EB78: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8240EB7C: 48126588  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240EB80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8240EB80 size=80
    let mut pc: u32 = 0x8240EB80;
    'dispatch: loop {
        match pc {
            0x8240EB80 => {
    //   block [0x8240EB80..0x8240EB9C)
	// 8240EB80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240EB84: 48126535  bl 0x825350b8
	ctx.lr = 0x8240EB88;
	sub_82535080(ctx, base);
	// 8240EB88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240EB8C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8240EB90: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 8240EB94: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8240EB98: 3BDD0018  addi r30, r29, 0x18
	ctx.r[30].s64 = ctx.r[29].s64 + 24;
	pc = 0x8240EB9C; continue 'dispatch;
            }
            0x8240EB9C => {
    //   block [0x8240EB9C..0x8240EBB4)
	// 8240EB9C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8240EBA0: 7F1C5800  cmpw cr6, r28, r11
	ctx.cr[6].compare_i32(ctx.r[28].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8240EBA4: 409A0010  bne cr6, 0x8240ebb4
	if !ctx.cr[6].eq {
	pc = 0x8240EBB4; continue 'dispatch;
	}
	// 8240EBA8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8240EBAC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8240EBB0: 4BFFFDF9  bl 0x8240e9a8
	ctx.lr = 0x8240EBB4;
	sub_8240E9A8(ctx, base);
	pc = 0x8240EBB4; continue 'dispatch;
            }
            0x8240EBB4 => {
    //   block [0x8240EBB4..0x8240EBD0)
	// 8240EBB4: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 8240EBB8: 3BDE0088  addi r30, r30, 0x88
	ctx.r[30].s64 = ctx.r[30].s64 + 136;
	// 8240EBBC: 2F1F0040  cmpwi cr6, r31, 0x40
	ctx.cr[6].compare_i32(ctx.r[31].s32, 64, &mut ctx.xer);
	// 8240EBC0: 4198FFDC  blt cr6, 0x8240eb9c
	if ctx.cr[6].lt {
	pc = 0x8240EB9C; continue 'dispatch;
	}
	// 8240EBC4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8240EBC8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8240EBCC: 4812653C  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240EBD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8240EBD0 size=80
    let mut pc: u32 = 0x8240EBD0;
    'dispatch: loop {
        match pc {
            0x8240EBD0 => {
    //   block [0x8240EBD0..0x8240EBEC)
	// 8240EBD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240EBD4: 481264E5  bl 0x825350b8
	ctx.lr = 0x8240EBD8;
	sub_82535080(ctx, base);
	// 8240EBD8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240EBDC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8240EBE0: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 8240EBE4: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8240EBE8: 3BDD0004  addi r30, r29, 4
	ctx.r[30].s64 = ctx.r[29].s64 + 4;
	pc = 0x8240EBEC; continue 'dispatch;
            }
            0x8240EBEC => {
    //   block [0x8240EBEC..0x8240EC04)
	// 8240EBEC: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8240EBF0: 7F1C5800  cmpw cr6, r28, r11
	ctx.cr[6].compare_i32(ctx.r[28].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8240EBF4: 409A0010  bne cr6, 0x8240ec04
	if !ctx.cr[6].eq {
	pc = 0x8240EC04; continue 'dispatch;
	}
	// 8240EBF8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8240EBFC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8240EC00: 4BFFFDA9  bl 0x8240e9a8
	ctx.lr = 0x8240EC04;
	sub_8240E9A8(ctx, base);
	pc = 0x8240EC04; continue 'dispatch;
            }
            0x8240EC04 => {
    //   block [0x8240EC04..0x8240EC20)
	// 8240EC04: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 8240EC08: 3BDE0088  addi r30, r30, 0x88
	ctx.r[30].s64 = ctx.r[30].s64 + 136;
	// 8240EC0C: 2F1F0040  cmpwi cr6, r31, 0x40
	ctx.cr[6].compare_i32(ctx.r[31].s32, 64, &mut ctx.xer);
	// 8240EC10: 4198FFDC  blt cr6, 0x8240ebec
	if ctx.cr[6].lt {
	pc = 0x8240EBEC; continue 'dispatch;
	}
	// 8240EC14: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8240EC18: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8240EC1C: 481264EC  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240EC20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8240EC20 size=80
    let mut pc: u32 = 0x8240EC20;
    'dispatch: loop {
        match pc {
            0x8240EC20 => {
    //   block [0x8240EC20..0x8240EC3C)
	// 8240EC20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240EC24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240EC28: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8240EC2C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8240EC30: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240EC34: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8240EC38: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	pc = 0x8240EC3C; continue 'dispatch;
            }
            0x8240EC3C => {
    //   block [0x8240EC3C..0x8240EC70)
	// 8240EC3C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8240EC40: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8240EC44: 4BFFFD65  bl 0x8240e9a8
	ctx.lr = 0x8240EC48;
	sub_8240E9A8(ctx, base);
	// 8240EC48: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 8240EC4C: 2F1F0040  cmpwi cr6, r31, 0x40
	ctx.cr[6].compare_i32(ctx.r[31].s32, 64, &mut ctx.xer);
	// 8240EC50: 4198FFEC  blt cr6, 0x8240ec3c
	if ctx.cr[6].lt {
	pc = 0x8240EC3C; continue 'dispatch;
	}
	// 8240EC54: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8240EC58: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8240EC5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240EC60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240EC64: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8240EC68: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8240EC6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240EC70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8240EC70 size=116
    let mut pc: u32 = 0x8240EC70;
    'dispatch: loop {
        match pc {
            0x8240EC70 => {
    //   block [0x8240EC70..0x8240EC9C)
	// 8240EC70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240EC74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240EC78: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8240EC7C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240EC80: 3D00820D  lis r8, -0x7df3
	ctx.r[8].s64 = -2113077248;
	// 8240EC84: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 8240EC88: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8240EC8C: 3940003F  li r10, 0x3f
	ctx.r[10].s64 = 63;
	// 8240EC90: 397F0078  addi r11, r31, 0x78
	ctx.r[11].s64 = ctx.r[31].s64 + 120;
	// 8240EC94: C1A81FF8  lfs f13, 0x1ff8(r8)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(8184 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8240EC98: C0091850  lfs f0, 0x1850(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(6224 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	pc = 0x8240EC9C; continue 'dispatch;
            }
            0x8240EC9C => {
    //   block [0x8240EC9C..0x8240ECE4)
	// 8240EC9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8240ECA0: D00BFFF8  stfs f0, -8(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-8 as u32), tmp.u32 ) };
	// 8240ECA4: D1ABFFFC  stfs f13, -4(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-4 as u32), tmp.u32 ) };
	// 8240ECA8: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8240ECAC: D1AB0004  stfs f13, 4(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 8240ECB0: D00B0008  stfs f0, 8(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 8240ECB4: D00B000C  stfs f0, 0xc(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 8240ECB8: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8240ECBC: 396B0088  addi r11, r11, 0x88
	ctx.r[11].s64 = ctx.r[11].s64 + 136;
	// 8240ECC0: 4080FFDC  bge 0x8240ec9c
	if !ctx.cr[0].lt {
	pc = 0x8240EC9C; continue 'dispatch;
	}
	// 8240ECC4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8240ECC8: 4BFFFF59  bl 0x8240ec20
	ctx.lr = 0x8240ECCC;
	sub_8240EC20(ctx, base);
	// 8240ECCC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8240ECD0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8240ECD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240ECD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240ECDC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8240ECE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240ECE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8240ECE8 size=36
    let mut pc: u32 = 0x8240ECE8;
    'dispatch: loop {
        match pc {
            0x8240ECE8 => {
    //   block [0x8240ECE8..0x8240ED0C)
	// 8240ECE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240ECEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240ECF0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240ECF4: 4BFFFF2D  bl 0x8240ec20
	ctx.lr = 0x8240ECF8;
	sub_8240EC20(ctx, base);
	// 8240ECF8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8240ECFC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8240ED00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240ED04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240ED08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240ED18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8240ED18 size=200
    let mut pc: u32 = 0x8240ED18;
    'dispatch: loop {
        match pc {
            0x8240ED18 => {
    //   block [0x8240ED18..0x8240EDE0)
	// 8240ED18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240ED1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240ED20: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8240ED24: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240ED28: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8240ED2C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8240ED30: 38A00020  li r5, 0x20
	ctx.r[5].s64 = 32;
	// 8240ED34: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8240ED38: 387F001C  addi r3, r31, 0x1c
	ctx.r[3].s64 = ctx.r[31].s64 + 28;
	// 8240ED3C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8240ED40: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8240ED44: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8240ED48: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 8240ED4C: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 8240ED50: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 8240ED54: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 8240ED58: 4BFB2809  bl 0x823c1560
	ctx.lr = 0x8240ED5C;
	sub_823C1560(ctx, base);
	// 8240ED5C: 38A00400  li r5, 0x400
	ctx.r[5].s64 = 1024;
	// 8240ED60: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8240ED64: 387F003C  addi r3, r31, 0x3c
	ctx.r[3].s64 = ctx.r[31].s64 + 60;
	// 8240ED68: 4BFB27F9  bl 0x823c1560
	ctx.lr = 0x8240ED6C;
	sub_823C1560(ctx, base);
	// 8240ED6C: 38A00040  li r5, 0x40
	ctx.r[5].s64 = 64;
	// 8240ED70: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8240ED74: 387F043C  addi r3, r31, 0x43c
	ctx.r[3].s64 = ctx.r[31].s64 + 1084;
	// 8240ED78: 4BFB27E9  bl 0x823c1560
	ctx.lr = 0x8240ED7C;
	sub_823C1560(ctx, base);
	// 8240ED7C: 38A00200  li r5, 0x200
	ctx.r[5].s64 = 512;
	// 8240ED80: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8240ED84: 387F047C  addi r3, r31, 0x47c
	ctx.r[3].s64 = ctx.r[31].s64 + 1148;
	// 8240ED88: 4BFB27D9  bl 0x823c1560
	ctx.lr = 0x8240ED8C;
	sub_823C1560(ctx, base);
	// 8240ED8C: 38A00400  li r5, 0x400
	ctx.r[5].s64 = 1024;
	// 8240ED90: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8240ED94: 387F067C  addi r3, r31, 0x67c
	ctx.r[3].s64 = ctx.r[31].s64 + 1660;
	// 8240ED98: 4BFB27C9  bl 0x823c1560
	ctx.lr = 0x8240ED9C;
	sub_823C1560(ctx, base);
	// 8240ED9C: 38A00140  li r5, 0x140
	ctx.r[5].s64 = 320;
	// 8240EDA0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8240EDA4: 387F0A7C  addi r3, r31, 0xa7c
	ctx.r[3].s64 = ctx.r[31].s64 + 2684;
	// 8240EDA8: 4BFB27B9  bl 0x823c1560
	ctx.lr = 0x8240EDAC;
	sub_823C1560(ctx, base);
	// 8240EDAC: 38A00100  li r5, 0x100
	ctx.r[5].s64 = 256;
	// 8240EDB0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8240EDB4: 387F0BBC  addi r3, r31, 0xbbc
	ctx.r[3].s64 = ctx.r[31].s64 + 3004;
	// 8240EDB8: 4BFB27A9  bl 0x823c1560
	ctx.lr = 0x8240EDBC;
	sub_823C1560(ctx, base);
	// 8240EDBC: 38A00100  li r5, 0x100
	ctx.r[5].s64 = 256;
	// 8240EDC0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8240EDC4: 387F0CBC  addi r3, r31, 0xcbc
	ctx.r[3].s64 = ctx.r[31].s64 + 3260;
	// 8240EDC8: 4BFB2799  bl 0x823c1560
	ctx.lr = 0x8240EDCC;
	sub_823C1560(ctx, base);
	// 8240EDCC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8240EDD0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240EDD4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240EDD8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8240EDDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240EDE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8240EDE0 size=48
    let mut pc: u32 = 0x8240EDE0;
    'dispatch: loop {
        match pc {
            0x8240EDE0 => {
    //   block [0x8240EDE0..0x8240EE10)
	// 8240EDE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240EDE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240EDE8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8240EDEC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240EDF0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8240EDF4: 4BFFFF25  bl 0x8240ed18
	ctx.lr = 0x8240EDF8;
	sub_8240ED18(ctx, base);
	// 8240EDF8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8240EDFC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8240EE00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240EE04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240EE08: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8240EE0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240EE10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8240EE10 size=372
    let mut pc: u32 = 0x8240EE10;
    'dispatch: loop {
        match pc {
            0x8240EE10 => {
    //   block [0x8240EE10..0x8240EE4C)
	// 8240EE10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240EE14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240EE18: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8240EE1C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8240EE20: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240EE24: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8240EE28: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8240EE2C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8240EE30: 409A001C  bne cr6, 0x8240ee4c
	if !ctx.cr[6].eq {
	pc = 0x8240EE4C; continue 'dispatch;
	}
	// 8240EE34: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240EE38: 386BE970  addi r3, r11, -0x1690
	ctx.r[3].s64 = ctx.r[11].s64 + -5776;
	// 8240EE3C: 4BEA4145  bl 0x822b2f80
	ctx.lr = 0x8240EE40;
	sub_822B2F80(ctx, base);
	// 8240EE40: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 8240EE44: 60630041  ori r3, r3, 0x41
	ctx.r[3].u64 = ctx.r[3].u64 | 65;
	// 8240EE48: 48000124  b 0x8240ef6c
	pc = 0x8240EF6C; continue 'dispatch;
            }
            0x8240EE4C => {
    //   block [0x8240EE4C..0x8240EE74)
	// 8240EE4C: 889F0005  lbz r4, 5(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(5 as u32) ) } as u64;
	// 8240EE50: 2B040002  cmplwi cr6, r4, 2
	ctx.cr[6].compare_u32(ctx.r[4].u32, 2 as u32, &mut ctx.xer);
	// 8240EE54: 419A0020  beq cr6, 0x8240ee74
	if ctx.cr[6].eq {
	pc = 0x8240EE74; continue 'dispatch;
	}
	// 8240EE58: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240EE5C: 88BF0006  lbz r5, 6(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(6 as u32) ) } as u64;
	// 8240EE60: 386BE928  addi r3, r11, -0x16d8
	ctx.r[3].s64 = ctx.r[11].s64 + -5848;
	// 8240EE64: 4BEA411D  bl 0x822b2f80
	ctx.lr = 0x8240EE68;
	sub_822B2F80(ctx, base);
	// 8240EE68: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 8240EE6C: 60630040  ori r3, r3, 0x40
	ctx.r[3].u64 = ctx.r[3].u64 | 64;
	// 8240EE70: 480000FC  b 0x8240ef6c
	pc = 0x8240EF6C; continue 'dispatch;
            }
            0x8240EE74 => {
    //   block [0x8240EE74..0x8240EF6C)
	// 8240EE74: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8240EE78: 38A00020  li r5, 0x20
	ctx.r[5].s64 = 32;
	// 8240EE7C: 387E001C  addi r3, r30, 0x1c
	ctx.r[3].s64 = ctx.r[30].s64 + 28;
	// 8240EE80: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8240EE84: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 8240EE88: 917E0004  stw r11, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8240EE8C: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 8240EE90: 917E0008  stw r11, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8240EE94: 817F0028  lwz r11, 0x28(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 8240EE98: 917E000C  stw r11, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 8240EE9C: 817F0030  lwz r11, 0x30(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 8240EEA0: 917E0010  stw r11, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 8240EEA4: 817F0038  lwz r11, 0x38(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 8240EEA8: 917E0014  stw r11, 0x14(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 8240EEAC: 817F0040  lwz r11, 0x40(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as u64;
	// 8240EEB0: 917E0018  stw r11, 0x18(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 8240EEB4: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8240EEB8: 7C8BFA14  add r4, r11, r31
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 8240EEBC: 4BFB1D45  bl 0x823c0c00
	ctx.lr = 0x8240EEC0;
	sub_823C0C00(ctx, base);
	// 8240EEC0: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8240EEC4: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 8240EEC8: 387E003C  addi r3, r30, 0x3c
	ctx.r[3].s64 = ctx.r[30].s64 + 60;
	// 8240EECC: 55452036  slwi r5, r10, 4
	ctx.r[5].u32 = ctx.r[10].u32.wrapping_shl(4);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 8240EED0: 7C8BFA14  add r4, r11, r31
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 8240EED4: 4BFB1D2D  bl 0x823c0c00
	ctx.lr = 0x8240EED8;
	sub_823C0C00(ctx, base);
	// 8240EED8: 815F0018  lwz r10, 0x18(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 8240EEDC: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 8240EEE0: 387E043C  addi r3, r30, 0x43c
	ctx.r[3].s64 = ctx.r[30].s64 + 1084;
	// 8240EEE4: 55452036  slwi r5, r10, 4
	ctx.r[5].u32 = ctx.r[10].u32.wrapping_shl(4);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 8240EEE8: 7C8BFA14  add r4, r11, r31
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 8240EEEC: 4BFB1D15  bl 0x823c0c00
	ctx.lr = 0x8240EEF0;
	sub_823C0C00(ctx, base);
	// 8240EEF0: 815F0020  lwz r10, 0x20(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 8240EEF4: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 8240EEF8: 387E047C  addi r3, r30, 0x47c
	ctx.r[3].s64 = ctx.r[30].s64 + 1148;
	// 8240EEFC: 55452036  slwi r5, r10, 4
	ctx.r[5].u32 = ctx.r[10].u32.wrapping_shl(4);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 8240EF00: 7C8BFA14  add r4, r11, r31
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 8240EF04: 4BFB1CFD  bl 0x823c0c00
	ctx.lr = 0x8240EF08;
	sub_823C0C00(ctx, base);
	// 8240EF08: 815F0028  lwz r10, 0x28(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 8240EF0C: 817F002C  lwz r11, 0x2c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 8240EF10: 387E067C  addi r3, r30, 0x67c
	ctx.r[3].s64 = ctx.r[30].s64 + 1660;
	// 8240EF14: 55452036  slwi r5, r10, 4
	ctx.r[5].u32 = ctx.r[10].u32.wrapping_shl(4);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 8240EF18: 7C8BFA14  add r4, r11, r31
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 8240EF1C: 4BFB1CE5  bl 0x823c0c00
	ctx.lr = 0x8240EF20;
	sub_823C0C00(ctx, base);
	// 8240EF20: 815F0030  lwz r10, 0x30(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 8240EF24: 817F0034  lwz r11, 0x34(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 8240EF28: 387E0A7C  addi r3, r30, 0xa7c
	ctx.r[3].s64 = ctx.r[30].s64 + 2684;
	// 8240EF2C: 55452834  slwi r5, r10, 5
	ctx.r[5].u32 = ctx.r[10].u32.wrapping_shl(5);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 8240EF30: 7C8BFA14  add r4, r11, r31
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 8240EF34: 4BFB1CCD  bl 0x823c0c00
	ctx.lr = 0x8240EF38;
	sub_823C0C00(ctx, base);
	// 8240EF38: 815F0038  lwz r10, 0x38(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 8240EF3C: 817F003C  lwz r11, 0x3c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) } as u64;
	// 8240EF40: 387E0BBC  addi r3, r30, 0xbbc
	ctx.r[3].s64 = ctx.r[30].s64 + 3004;
	// 8240EF44: 55452834  slwi r5, r10, 5
	ctx.r[5].u32 = ctx.r[10].u32.wrapping_shl(5);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 8240EF48: 7C8BFA14  add r4, r11, r31
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 8240EF4C: 4BFB1CB5  bl 0x823c0c00
	ctx.lr = 0x8240EF50;
	sub_823C0C00(ctx, base);
	// 8240EF50: 815F0040  lwz r10, 0x40(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as u64;
	// 8240EF54: 817F0044  lwz r11, 0x44(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(68 as u32) ) } as u64;
	// 8240EF58: 387E0CBC  addi r3, r30, 0xcbc
	ctx.r[3].s64 = ctx.r[30].s64 + 3260;
	// 8240EF5C: 55452834  slwi r5, r10, 5
	ctx.r[5].u32 = ctx.r[10].u32.wrapping_shl(5);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 8240EF60: 7C8BFA14  add r4, r11, r31
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 8240EF64: 4BFB1C9D  bl 0x823c0c00
	ctx.lr = 0x8240EF68;
	sub_823C0C00(ctx, base);
	// 8240EF68: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	pc = 0x8240EF6C; continue 'dispatch;
            }
            0x8240EF6C => {
    //   block [0x8240EF6C..0x8240EF84)
	// 8240EF6C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8240EF70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240EF74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240EF78: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8240EF7C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8240EF80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240EF88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8240EF88 size=904
    let mut pc: u32 = 0x8240EF88;
    'dispatch: loop {
        match pc {
            0x8240EF88 => {
    //   block [0x8240EF88..0x8240EFDC)
	// 8240EF88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240EF8C: 48126101  bl 0x8253508c
	ctx.lr = 0x8240EF90;
	sub_82535080(ctx, base);
	// 8240EF90: 3981FF80  addi r12, r1, -0x80
	ctx.r[12].s64 = ctx.r[1].s64 + -128;
	// 8240EF94: 48127055  bl 0x82535fe8
	ctx.lr = 0x8240EF98;
	sub_82535FB0(ctx, base);
	// 8240EF98: 9421FF00  stwu r1, -0x100(r1)
	ea = ctx.r[1].u32.wrapping_add(-256 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240EF9C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8240EFA0: 7CF53B78  mr r21, r7
	ctx.r[21].u64 = ctx.r[7].u64;
	// 8240EFA4: 3A600000  li r19, 0
	ctx.r[19].s64 = 0;
	// 8240EFA8: 7C741B78  mr r20, r3
	ctx.r[20].u64 = ctx.r[3].u64;
	// 8240EFAC: 7C912378  mr r17, r4
	ctx.r[17].u64 = ctx.r[4].u64;
	// 8240EFB0: C3CB1FF8  lfs f30, 0x1ff8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 8240EFB4: 7CB72B78  mr r23, r5
	ctx.r[23].u64 = ctx.r[5].u64;
	// 8240EFB8: 7CD63378  mr r22, r6
	ctx.r[22].u64 = ctx.r[6].u64;
	// 8240EFBC: 7E7C9B78  mr r28, r19
	ctx.r[28].u64 = ctx.r[19].u64;
	// 8240EFC0: 3A400001  li r18, 1
	ctx.r[18].s64 = 1;
	// 8240EFC4: 2F150000  cmpwi cr6, r21, 0
	ctx.cr[6].compare_i32(ctx.r[21].s32, 0, &mut ctx.xer);
	// 8240EFC8: 409900D4  ble cr6, 0x8240f09c
	if !ctx.cr[6].gt {
	pc = 0x8240F09C; continue 'dispatch;
	}
	// 8240EFCC: 3B340008  addi r25, r20, 8
	ctx.r[25].s64 = ctx.r[20].s64 + 8;
	// 8240EFD0: 3B160014  addi r24, r22, 0x14
	ctx.r[24].s64 = ctx.r[22].s64 + 20;
	// 8240EFD4: 7F3FCB78  mr r31, r25
	ctx.r[31].u64 = ctx.r[25].u64;
	// 8240EFD8: 7F1DC378  mr r29, r24
	ctx.r[29].u64 = ctx.r[24].u64;
	pc = 0x8240EFDC; continue 'dispatch;
            }
            0x8240EFDC => {
    //   block [0x8240EFDC..0x8240F028)
	// 8240EFDC: 817DFFEC  lwz r11, -0x14(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(-20 as u32) ) } as u64;
	// 8240EFE0: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8240EFE4: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8240EFE8: 7C0BBC2E  lfsx f0, r11, r23
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[23].u32)) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240EFEC: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8240EFF0: D01FFFFC  stfs f0, -4(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-4 as u32), tmp.u32 ) };
	// 8240EFF4: C01D0000  lfs f0, 0(r29)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240EFF8: FF00F000  fcmpu cr6, f0, f30
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[30].f64);
	// 8240EFFC: 419A007C  beq cr6, 0x8240f078
	if ctx.cr[6].eq {
	pc = 0x8240F078; continue 'dispatch;
	}
	// 8240F000: C01FFFFC  lfs f0, -4(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240F004: C1BDFFF4  lfs f13, -0xc(r29)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(-12 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8240F008: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 8240F00C: 4198006C  blt cr6, 0x8240f078
	if ctx.cr[6].lt {
	pc = 0x8240F078; continue 'dispatch;
	}
	// 8240F010: 7E7A9B78  mr r26, r19
	ctx.r[26].u64 = ctx.r[19].u64;
	// 8240F014: 925F0000  stw r18, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[18].u32 ) };
	// 8240F018: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 8240F01C: 40990060  ble cr6, 0x8240f07c
	if !ctx.cr[6].gt {
	pc = 0x8240F07C; continue 'dispatch;
	}
	// 8240F020: 7F1EC378  mr r30, r24
	ctx.r[30].u64 = ctx.r[24].u64;
	// 8240F024: 7F3BCB78  mr r27, r25
	ctx.r[27].u64 = ctx.r[25].u64;
	pc = 0x8240F028; continue 'dispatch;
            }
            0x8240F028 => {
    //   block [0x8240F028..0x8240F060)
	// 8240F028: 817DFFF0  lwz r11, -0x10(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(-16 as u32) ) } as u64;
	// 8240F02C: 815EFFF0  lwz r10, -0x10(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-16 as u32) ) } as u64;
	// 8240F030: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 8240F034: 409A002C  bne cr6, 0x8240f060
	if !ctx.cr[6].eq {
	pc = 0x8240F060; continue 'dispatch;
	}
	// 8240F038: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 8240F03C: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 8240F040: 409A0020  bne cr6, 0x8240f060
	if !ctx.cr[6].eq {
	pc = 0x8240F060; continue 'dispatch;
	}
	// 8240F044: C03D0000  lfs f1, 0(r29)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8240F048: 4BFFF489  bl 0x8240e4d0
	ctx.lr = 0x8240F04C;
	sub_8240E4D0(ctx, base);
	// 8240F04C: FFE00890  fmr f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].f64 = ctx.f[1].f64;
	// 8240F050: C03E0000  lfs f1, 0(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8240F054: 4BFFF47D  bl 0x8240e4d0
	ctx.lr = 0x8240F058;
	sub_8240E4D0(ctx, base);
	// 8240F058: FF1F0800  fcmpu cr6, f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[1].f64);
	// 8240F05C: 4198001C  blt cr6, 0x8240f078
	if ctx.cr[6].lt {
	pc = 0x8240F078; continue 'dispatch;
	}
	pc = 0x8240F060; continue 'dispatch;
            }
            0x8240F060 => {
    //   block [0x8240F060..0x8240F078)
	// 8240F060: 3B5A0001  addi r26, r26, 1
	ctx.r[26].s64 = ctx.r[26].s64 + 1;
	// 8240F064: 3B7B001C  addi r27, r27, 0x1c
	ctx.r[27].s64 = ctx.r[27].s64 + 28;
	// 8240F068: 3BDE0020  addi r30, r30, 0x20
	ctx.r[30].s64 = ctx.r[30].s64 + 32;
	// 8240F06C: 7F1AE000  cmpw cr6, r26, r28
	ctx.cr[6].compare_i32(ctx.r[26].s32, ctx.r[28].s32, &mut ctx.xer);
	// 8240F070: 4198FFB8  blt cr6, 0x8240f028
	if ctx.cr[6].lt {
	pc = 0x8240F028; continue 'dispatch;
	}
	// 8240F074: 48000008  b 0x8240f07c
	pc = 0x8240F07C; continue 'dispatch;
            }
            0x8240F078 => {
    //   block [0x8240F078..0x8240F07C)
	// 8240F078: 927F0000  stw r19, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[19].u32 ) };
	pc = 0x8240F07C; continue 'dispatch;
            }
            0x8240F07C => {
    //   block [0x8240F07C..0x8240F09C)
	// 8240F07C: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 8240F080: 927F000C  stw r19, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[19].u32 ) };
	// 8240F084: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 8240F088: 3BBD0020  addi r29, r29, 0x20
	ctx.r[29].s64 = ctx.r[29].s64 + 32;
	// 8240F08C: 7F1CA800  cmpw cr6, r28, r21
	ctx.cr[6].compare_i32(ctx.r[28].s32, ctx.r[21].s32, &mut ctx.xer);
	// 8240F090: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 8240F094: 3BFF001C  addi r31, r31, 0x1c
	ctx.r[31].s64 = ctx.r[31].s64 + 28;
	// 8240F098: 4198FF44  blt cr6, 0x8240efdc
	if ctx.cr[6].lt {
	pc = 0x8240EFDC; continue 'dispatch;
	}
	pc = 0x8240F09C; continue 'dispatch;
            }
            0x8240F09C => {
    //   block [0x8240F09C..0x8240F0B8)
	// 8240F09C: 7E7D9B78  mr r29, r19
	ctx.r[29].u64 = ctx.r[19].u64;
	// 8240F0A0: 2F150000  cmpwi cr6, r21, 0
	ctx.cr[6].compare_i32(ctx.r[21].s32, 0, &mut ctx.xer);
	// 8240F0A4: 409901D8  ble cr6, 0x8240f27c
	if !ctx.cr[6].gt {
	pc = 0x8240F27C; continue 'dispatch;
	}
	// 8240F0A8: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8240F0AC: 3BF40010  addi r31, r20, 0x10
	ctx.r[31].s64 = ctx.r[20].s64 + 16;
	// 8240F0B0: 3BD60014  addi r30, r22, 0x14
	ctx.r[30].s64 = ctx.r[22].s64 + 20;
	// 8240F0B4: C3AB204C  lfs f29, 0x204c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8268 as u32) ) };
	ctx.f[29].f64 = (tmp.f32 as f64);
	pc = 0x8240F0B8; continue 'dispatch;
            }
            0x8240F0B8 => {
    //   block [0x8240F0B8..0x8240F0F8)
	// 8240F0B8: 817FFFF8  lwz r11, -8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240F0BC: C19FFFF0  lfs f12, -0x10(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-16 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 8240F0C0: FFE06090  fmr f31, f12
	ctx.f[31].f64 = ctx.f[12].f64;
	// 8240F0C4: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 8240F0C8: 409A00B0  bne cr6, 0x8240f178
	if !ctx.cr[6].eq {
	pc = 0x8240F178; continue 'dispatch;
	}
	// 8240F0CC: 925F0004  stw r18, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[18].u32 ) };
	// 8240F0D0: 927F0000  stw r19, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[19].u32 ) };
	// 8240F0D4: C01E0000  lfs f0, 0(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240F0D8: FF0C0000  fcmpu cr6, f12, f0
	ctx.cr[6].compare_f64(ctx.f[12].f64, ctx.f[0].f64);
	// 8240F0DC: 419A0170  beq cr6, 0x8240f24c
	if ctx.cr[6].eq {
	pc = 0x8240F24C; continue 'dispatch;
	}
	// 8240F0E0: C1BEFFF8  lfs f13, -8(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-8 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8240F0E4: FF0DF000  fcmpu cr6, f13, f30
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[30].f64);
	// 8240F0E8: 40990010  ble cr6, 0x8240f0f8
	if !ctx.cr[6].gt {
	pc = 0x8240F0F8; continue 'dispatch;
	}
	// 8240F0EC: EDAD0772  fmuls f13, f13, f29
	ctx.f[13].f64 = (((ctx.f[13].f64 * ctx.f[29].f64) as f32) as f64);
	// 8240F0F0: EDA06824  fdivs f13, f0, f13
	ctx.f[13].f64 = ((ctx.f[0].f64 / ctx.f[13].f64) as f32) as f64;
	// 8240F0F4: 48000008  b 0x8240f0fc
	pc = 0x8240F0FC; continue 'dispatch;
            }
            0x8240F0F8 => {
    //   block [0x8240F0F8..0x8240F0FC)
	// 8240F0F8: FDA00090  fmr f13, f0
	ctx.f[13].f64 = ctx.f[0].f64;
	pc = 0x8240F0FC; continue 'dispatch;
            }
            0x8240F0FC => {
    //   block [0x8240F0FC..0x8240F110)
	// 8240F0FC: FF00F000  fcmpu cr6, f0, f30
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[30].f64);
	// 8240F100: 40980010  bge cr6, 0x8240f110
	if !ctx.cr[6].lt {
	pc = 0x8240F110; continue 'dispatch;
	}
	// 8240F104: FF0C0000  fcmpu cr6, f12, f0
	ctx.cr[6].compare_f64(ctx.f[12].f64, ctx.f[0].f64);
	// 8240F108: 40980014  bge cr6, 0x8240f11c
	if !ctx.cr[6].lt {
	pc = 0x8240F11C; continue 'dispatch;
	}
	// 8240F10C: 4800000C  b 0x8240f118
	pc = 0x8240F118; continue 'dispatch;
            }
            0x8240F110 => {
    //   block [0x8240F110..0x8240F118)
	// 8240F110: FF0C0000  fcmpu cr6, f12, f0
	ctx.cr[6].compare_f64(ctx.f[12].f64, ctx.f[0].f64);
	// 8240F114: 40990008  ble cr6, 0x8240f11c
	if !ctx.cr[6].gt {
	pc = 0x8240F11C; continue 'dispatch;
	}
	pc = 0x8240F118; continue 'dispatch;
            }
            0x8240F118 => {
    //   block [0x8240F118..0x8240F11C)
	// 8240F118: FDA06850  fneg f13, f13
	ctx.f[13].u64 = ctx.f[13].u64 ^ 0x8000_0000_0000_0000u64;
	pc = 0x8240F11C; continue 'dispatch;
            }
            0x8240F11C => {
    //   block [0x8240F11C..0x8240F144)
	// 8240F11C: 7A2B0020  clrldi r11, r17, 0x20
	ctx.r[11].u64 = ctx.r[17].u64 & 0x00000000FFFFFFFFu64;
	// 8240F120: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 8240F124: C9610050  lfd f11, 0x50(r1)
	ctx.f[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 8240F128: FD605E9C  fcfid f11, f11
	ctx.f[11].f64 = (ctx.f[11].s64 as f64);
	// 8240F12C: FD605818  frsp f11, f11
	ctx.f[11].f64 = (ctx.f[11].f64 as f32) as f64;
	// 8240F130: EFEB637A  fmadds f31, f11, f13, f12
	ctx.f[31].f64 = (((ctx.f[11].f64 * ctx.f[13].f64 + ctx.f[12].f64) as f32) as f64);
	// 8240F134: FF1FF000  fcmpu cr6, f31, f30
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[30].f64);
	// 8240F138: 4099000C  ble cr6, 0x8240f144
	if !ctx.cr[6].gt {
	pc = 0x8240F144; continue 'dispatch;
	}
	// 8240F13C: FF00F000  fcmpu cr6, f0, f30
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[30].f64);
	// 8240F140: 41990014  bgt cr6, 0x8240f154
	if ctx.cr[6].gt {
	pc = 0x8240F154; continue 'dispatch;
	}
	pc = 0x8240F144; continue 'dispatch;
            }
            0x8240F144 => {
    //   block [0x8240F144..0x8240F154)
	// 8240F144: FF1FF000  fcmpu cr6, f31, f30
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[30].f64);
	// 8240F148: 40980104  bge cr6, 0x8240f24c
	if !ctx.cr[6].lt {
	pc = 0x8240F24C; continue 'dispatch;
	}
	// 8240F14C: FF00F000  fcmpu cr6, f0, f30
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[30].f64);
	// 8240F150: 409800FC  bge cr6, 0x8240f24c
	if !ctx.cr[6].lt {
	pc = 0x8240F24C; continue 'dispatch;
	}
	pc = 0x8240F154; continue 'dispatch;
            }
            0x8240F154 => {
    //   block [0x8240F154..0x8240F178)
	// 8240F154: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 8240F158: 4BFFF379  bl 0x8240e4d0
	ctx.lr = 0x8240F15C;
	sub_8240E4D0(ctx, base);
	// 8240F15C: FF800890  fmr f28, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[28].f64 = ctx.f[1].f64;
	// 8240F160: C03E0000  lfs f1, 0(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8240F164: 4BFFF36D  bl 0x8240e4d0
	ctx.lr = 0x8240F168;
	sub_8240E4D0(ctx, base);
	// 8240F168: FF1C0800  fcmpu cr6, f28, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.cr[6].compare_f64(ctx.f[28].f64, ctx.f[1].f64);
	// 8240F16C: 419800E0  blt cr6, 0x8240f24c
	if ctx.cr[6].lt {
	pc = 0x8240F24C; continue 'dispatch;
	}
	// 8240F170: C3FE0000  lfs f31, 0(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 8240F174: 480000D8  b 0x8240f24c
	pc = 0x8240F24C; continue 'dispatch;
            }
            0x8240F178 => {
    //   block [0x8240F178..0x8240F184)
	// 8240F178: 7E6B9B78  mr r11, r19
	ctx.r[11].u64 = ctx.r[19].u64;
	// 8240F17C: 39340008  addi r9, r20, 8
	ctx.r[9].s64 = ctx.r[20].s64 + 8;
	// 8240F180: 39560004  addi r10, r22, 4
	ctx.r[10].s64 = ctx.r[22].s64 + 4;
	pc = 0x8240F184; continue 'dispatch;
            }
            0x8240F184 => {
    //   block [0x8240F184..0x8240F1A8)
	// 8240F184: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8240F188: 419A0020  beq cr6, 0x8240f1a8
	if ctx.cr[6].eq {
	pc = 0x8240F1A8; continue 'dispatch;
	}
	// 8240F18C: 811EFFF0  lwz r8, -0x10(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-16 as u32) ) } as u64;
	// 8240F190: 80EA0000  lwz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8240F194: 7F083800  cmpw cr6, r8, r7
	ctx.cr[6].compare_i32(ctx.r[8].s32, ctx.r[7].s32, &mut ctx.xer);
	// 8240F198: 409A0010  bne cr6, 0x8240f1a8
	if !ctx.cr[6].eq {
	pc = 0x8240F1A8; continue 'dispatch;
	}
	// 8240F19C: 81090000  lwz r8, 0(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 8240F1A0: 2B080001  cmplwi cr6, r8, 1
	ctx.cr[6].compare_u32(ctx.r[8].u32, 1 as u32, &mut ctx.xer);
	// 8240F1A4: 419A001C  beq cr6, 0x8240f1c0
	if ctx.cr[6].eq {
	pc = 0x8240F1C0; continue 'dispatch;
	}
	pc = 0x8240F1A8; continue 'dispatch;
            }
            0x8240F1A8 => {
    //   block [0x8240F1A8..0x8240F1C0)
	// 8240F1A8: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8240F1AC: 394A0020  addi r10, r10, 0x20
	ctx.r[10].s64 = ctx.r[10].s64 + 32;
	// 8240F1B0: 3929001C  addi r9, r9, 0x1c
	ctx.r[9].s64 = ctx.r[9].s64 + 28;
	// 8240F1B4: 7F0BA800  cmpw cr6, r11, r21
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[21].s32, &mut ctx.xer);
	// 8240F1B8: 4198FFCC  blt cr6, 0x8240f184
	if ctx.cr[6].lt {
	pc = 0x8240F184; continue 'dispatch;
	}
	// 8240F1BC: 4800000C  b 0x8240f1c8
	pc = 0x8240F1C8; continue 'dispatch;
            }
            0x8240F1C0 => {
    //   block [0x8240F1C0..0x8240F1C8)
	// 8240F1C0: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8240F1C4: 927F0000  stw r19, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[19].u32 ) };
	pc = 0x8240F1C8; continue 'dispatch;
            }
            0x8240F1C8 => {
    //   block [0x8240F1C8..0x8240F1E0)
	// 8240F1C8: 817FFFFC  lwz r11, -4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-4 as u32) ) } as u64;
	// 8240F1CC: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 8240F1D0: 419A0010  beq cr6, 0x8240f1e0
	if ctx.cr[6].eq {
	pc = 0x8240F1E0; continue 'dispatch;
	}
	// 8240F1D4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8240F1D8: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 8240F1DC: 409A0070  bne cr6, 0x8240f24c
	if !ctx.cr[6].eq {
	pc = 0x8240F24C; continue 'dispatch;
	}
	pc = 0x8240F1E0; continue 'dispatch;
            }
            0x8240F1E0 => {
    //   block [0x8240F1E0..0x8240F204)
	// 8240F1E0: 925F0004  stw r18, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[18].u32 ) };
	// 8240F1E4: C01EFFFC  lfs f0, -4(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240F1E8: FF00F000  fcmpu cr6, f0, f30
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[30].f64);
	// 8240F1EC: 40990018  ble cr6, 0x8240f204
	if !ctx.cr[6].gt {
	pc = 0x8240F204; continue 'dispatch;
	}
	// 8240F1F0: EC000772  fmuls f0, f0, f29
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[29].f64) as f32) as f64);
	// 8240F1F4: C1BE0000  lfs f13, 0(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8240F1F8: EC0D0024  fdivs f0, f13, f0
	ctx.f[0].f64 = ((ctx.f[13].f64 / ctx.f[0].f64) as f32) as f64;
	// 8240F1FC: FC000050  fneg f0, f0
	ctx.f[0].u64 = ctx.f[0].u64 ^ 0x8000_0000_0000_0000u64;
	// 8240F200: 48000008  b 0x8240f208
	pc = 0x8240F208; continue 'dispatch;
            }
            0x8240F204 => {
    //   block [0x8240F204..0x8240F208)
	// 8240F204: FC00F090  fmr f0, f30
	ctx.f[0].f64 = ctx.f[30].f64;
	pc = 0x8240F208; continue 'dispatch;
            }
            0x8240F208 => {
    //   block [0x8240F208..0x8240F218)
	// 8240F208: EDA00332  fmuls f13, f0, f12
	ctx.f[13].f64 = (((ctx.f[0].f64 * ctx.f[12].f64) as f32) as f64);
	// 8240F20C: FF0DF000  fcmpu cr6, f13, f30
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[30].f64);
	// 8240F210: 40990008  ble cr6, 0x8240f218
	if !ctx.cr[6].gt {
	pc = 0x8240F218; continue 'dispatch;
	}
	// 8240F214: FC000050  fneg f0, f0
	ctx.f[0].u64 = ctx.f[0].u64 ^ 0x8000_0000_0000_0000u64;
	pc = 0x8240F218; continue 'dispatch;
            }
            0x8240F218 => {
    //   block [0x8240F218..0x8240F248)
	// 8240F218: 7A2B0020  clrldi r11, r17, 0x20
	ctx.r[11].u64 = ctx.r[17].u64 & 0x00000000FFFFFFFFu64;
	// 8240F21C: F9610058  std r11, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u64 ) };
	// 8240F220: C9A10058  lfd f13, 0x58(r1)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 8240F224: FDA06E9C  fcfid f13, f13
	ctx.f[13].f64 = (ctx.f[13].s64 as f64);
	// 8240F228: FDA06818  frsp f13, f13
	ctx.f[13].f64 = (ctx.f[13].f64 as f32) as f64;
	// 8240F22C: EFED603A  fmadds f31, f13, f0, f12
	ctx.f[31].f64 = (((ctx.f[13].f64 * ctx.f[0].f64 + ctx.f[12].f64) as f32) as f64);
	// 8240F230: EC1F0332  fmuls f0, f31, f12
	ctx.f[0].f64 = (((ctx.f[31].f64 * ctx.f[12].f64) as f32) as f64);
	// 8240F234: FF00F000  fcmpu cr6, f0, f30
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[30].f64);
	// 8240F238: 41990010  bgt cr6, 0x8240f248
	if ctx.cr[6].gt {
	pc = 0x8240F248; continue 'dispatch;
	}
	// 8240F23C: FFE0F090  fmr f31, f30
	ctx.f[31].f64 = ctx.f[30].f64;
	// 8240F240: 927F0000  stw r19, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[19].u32 ) };
	// 8240F244: 48000008  b 0x8240f24c
	pc = 0x8240F24C; continue 'dispatch;
            }
            0x8240F248 => {
    //   block [0x8240F248..0x8240F24C)
	// 8240F248: 925F0000  stw r18, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[18].u32 ) };
	pc = 0x8240F24C; continue 'dispatch;
            }
            0x8240F24C => {
    //   block [0x8240F24C..0x8240F268)
	// 8240F24C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8240F250: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 8240F254: 409A0014  bne cr6, 0x8240f268
	if !ctx.cr[6].eq {
	pc = 0x8240F268; continue 'dispatch;
	}
	// 8240F258: D3FFFFF0  stfs f31, -0x10(r31)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 8240F25C: 807EFFF0  lwz r3, -0x10(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-16 as u32) ) } as u64;
	// 8240F260: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 8240F264: 4BFF7DE5  bl 0x82407048
	ctx.lr = 0x8240F268;
	sub_82407048(ctx, base);
	pc = 0x8240F268; continue 'dispatch;
            }
            0x8240F268 => {
    //   block [0x8240F268..0x8240F27C)
	// 8240F268: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 8240F26C: 3BFF001C  addi r31, r31, 0x1c
	ctx.r[31].s64 = ctx.r[31].s64 + 28;
	// 8240F270: 3BDE0020  addi r30, r30, 0x20
	ctx.r[30].s64 = ctx.r[30].s64 + 32;
	// 8240F274: 7F1DA800  cmpw cr6, r29, r21
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[21].s32, &mut ctx.xer);
	// 8240F278: 4198FE40  blt cr6, 0x8240f0b8
	if ctx.cr[6].lt {
	pc = 0x8240F0B8; continue 'dispatch;
	}
	pc = 0x8240F27C; continue 'dispatch;
            }
            0x8240F27C => {
    //   block [0x8240F27C..0x8240F294)
	// 8240F27C: 7E659B78  mr r5, r19
	ctx.r[5].u64 = ctx.r[19].u64;
	// 8240F280: 2F150000  cmpwi cr6, r21, 0
	ctx.cr[6].compare_i32(ctx.r[21].s32, 0, &mut ctx.xer);
	// 8240F284: 4099007C  ble cr6, 0x8240f300
	if !ctx.cr[6].gt {
	pc = 0x8240F300; continue 'dispatch;
	}
	// 8240F288: 38F60004  addi r7, r22, 4
	ctx.r[7].s64 = ctx.r[22].s64 + 4;
	// 8240F28C: 7E88A378  mr r8, r20
	ctx.r[8].u64 = ctx.r[20].u64;
	// 8240F290: 7CE63B78  mr r6, r7
	ctx.r[6].u64 = ctx.r[7].u64;
	pc = 0x8240F294; continue 'dispatch;
            }
            0x8240F294 => {
    //   block [0x8240F294..0x8240F2AC)
	// 8240F294: 81680014  lwz r11, 0x14(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(20 as u32) ) } as u64;
	// 8240F298: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8240F29C: 409A0050  bne cr6, 0x8240f2ec
	if !ctx.cr[6].eq {
	pc = 0x8240F2EC; continue 'dispatch;
	}
	// 8240F2A0: 7E6B9B78  mr r11, r19
	ctx.r[11].u64 = ctx.r[19].u64;
	// 8240F2A4: 7E8AA378  mr r10, r20
	ctx.r[10].u64 = ctx.r[20].u64;
	// 8240F2A8: 7CE93B78  mr r9, r7
	ctx.r[9].u64 = ctx.r[7].u64;
	pc = 0x8240F2AC; continue 'dispatch;
            }
            0x8240F2AC => {
    //   block [0x8240F2AC..0x8240F2D8)
	// 8240F2AC: 7F055800  cmpw cr6, r5, r11
	ctx.cr[6].compare_i32(ctx.r[5].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8240F2B0: 419A0028  beq cr6, 0x8240f2d8
	if ctx.cr[6].eq {
	pc = 0x8240F2D8; continue 'dispatch;
	}
	// 8240F2B4: 80860000  lwz r4, 0(r6)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 8240F2B8: 80690000  lwz r3, 0(r9)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 8240F2BC: 7F041800  cmpw cr6, r4, r3
	ctx.cr[6].compare_i32(ctx.r[4].s32, ctx.r[3].s32, &mut ctx.xer);
	// 8240F2C0: 409A0018  bne cr6, 0x8240f2d8
	if !ctx.cr[6].eq {
	pc = 0x8240F2D8; continue 'dispatch;
	}
	// 8240F2C4: 808A0014  lwz r4, 0x14(r10)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 8240F2C8: 2B040001  cmplwi cr6, r4, 1
	ctx.cr[6].compare_u32(ctx.r[4].u32, 1 as u32, &mut ctx.xer);
	// 8240F2CC: 409A000C  bne cr6, 0x8240f2d8
	if !ctx.cr[6].eq {
	pc = 0x8240F2D8; continue 'dispatch;
	}
	// 8240F2D0: C00A0000  lfs f0, 0(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240F2D4: D0080000  stfs f0, 0(r8)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), tmp.u32 ) };
	pc = 0x8240F2D8; continue 'dispatch;
            }
            0x8240F2D8 => {
    //   block [0x8240F2D8..0x8240F2EC)
	// 8240F2D8: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8240F2DC: 39290020  addi r9, r9, 0x20
	ctx.r[9].s64 = ctx.r[9].s64 + 32;
	// 8240F2E0: 394A001C  addi r10, r10, 0x1c
	ctx.r[10].s64 = ctx.r[10].s64 + 28;
	// 8240F2E4: 7F0BA800  cmpw cr6, r11, r21
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[21].s32, &mut ctx.xer);
	// 8240F2E8: 4198FFC4  blt cr6, 0x8240f2ac
	if ctx.cr[6].lt {
	pc = 0x8240F2AC; continue 'dispatch;
	}
	pc = 0x8240F2EC; continue 'dispatch;
            }
            0x8240F2EC => {
    //   block [0x8240F2EC..0x8240F300)
	// 8240F2EC: 38A50001  addi r5, r5, 1
	ctx.r[5].s64 = ctx.r[5].s64 + 1;
	// 8240F2F0: 3908001C  addi r8, r8, 0x1c
	ctx.r[8].s64 = ctx.r[8].s64 + 28;
	// 8240F2F4: 38C60020  addi r6, r6, 0x20
	ctx.r[6].s64 = ctx.r[6].s64 + 32;
	// 8240F2F8: 7F05A800  cmpw cr6, r5, r21
	ctx.cr[6].compare_i32(ctx.r[5].s32, ctx.r[21].s32, &mut ctx.xer);
	// 8240F2FC: 4198FF98  blt cr6, 0x8240f294
	if ctx.cr[6].lt {
	pc = 0x8240F294; continue 'dispatch;
	}
	pc = 0x8240F300; continue 'dispatch;
            }
            0x8240F300 => {
    //   block [0x8240F300..0x8240F310)
	// 8240F300: 38210100  addi r1, r1, 0x100
	ctx.r[1].s64 = ctx.r[1].s64 + 256;
	// 8240F304: 3981FF80  addi r12, r1, -0x80
	ctx.r[12].s64 = ctx.r[1].s64 + -128;
	// 8240F308: 48126D2D  bl 0x82536034
	ctx.lr = 0x8240F30C;
	sub_82535FFC(ctx, base);
	// 8240F30C: 48125DD0  b 0x825350dc
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240F310(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8240F310 size=96
    let mut pc: u32 = 0x8240F310;
    'dispatch: loop {
        match pc {
            0x8240F310 => {
    //   block [0x8240F310..0x8240F334)
	// 8240F310: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240F314: 48125DA9  bl 0x825350bc
	ctx.lr = 0x8240F318;
	sub_82535080(ctx, base);
	// 8240F318: DBE1FFD8  stfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 8240F31C: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240F320: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8240F324: 3BE30008  addi r31, r3, 8
	ctx.r[31].s64 = ctx.r[3].s64 + 8;
	// 8240F328: 3BA00008  li r29, 8
	ctx.r[29].s64 = 8;
	// 8240F32C: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8240F330: C3EB1FF8  lfs f31, 0x1ff8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	pc = 0x8240F334; continue 'dispatch;
            }
            0x8240F334 => {
    //   block [0x8240F334..0x8240F370)
	// 8240F334: D3FFFFF8  stfs f31, -8(r31)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-8 as u32), tmp.u32 ) };
	// 8240F338: 4BFF6059  bl 0x82405390
	ctx.lr = 0x8240F33C;
	sub_82405390(ctx, base);
	// 8240F33C: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 8240F340: D03FFFFC  stfs f1, -4(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-4 as u32), tmp.u32 ) };
	// 8240F344: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 8240F348: 37BDFFFF  addic. r29, r29, -1
	ctx.xer.ca = (ctx.r[29].u32 > (!(-1 as u32)));
	ctx.r[29].s64 = ctx.r[29].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 8240F34C: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 8240F350: 93DF0008  stw r30, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 8240F354: 93DF000C  stw r30, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[30].u32 ) };
	// 8240F358: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 8240F35C: 3BFF001C  addi r31, r31, 0x1c
	ctx.r[31].s64 = ctx.r[31].s64 + 28;
	// 8240F360: 4082FFD4  bne 0x8240f334
	if !ctx.cr[0].eq {
	pc = 0x8240F334; continue 'dispatch;
	}
	// 8240F364: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8240F368: CBE1FFD8  lfd f31, -0x28(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-40 as u32) ) };
	// 8240F36C: 48125DA0  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240F370(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8240F370 size=48
    let mut pc: u32 = 0x8240F370;
    'dispatch: loop {
        match pc {
            0x8240F370 => {
    //   block [0x8240F370..0x8240F3A0)
	// 8240F370: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240F374: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240F378: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8240F37C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240F380: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8240F384: 4BFFFF8D  bl 0x8240f310
	ctx.lr = 0x8240F388;
	sub_8240F310(ctx, base);
	// 8240F388: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8240F38C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8240F390: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240F394: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240F398: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8240F39C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240F3A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8240F3A0 size=120
    let mut pc: u32 = 0x8240F3A0;
    'dispatch: loop {
        match pc {
            0x8240F3A0 => {
    //   block [0x8240F3A0..0x8240F3D8)
	// 8240F3A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240F3A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240F3A8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8240F3AC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8240F3B0: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 8240F3B4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240F3B8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8240F3BC: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 8240F3C0: 4BFFFF51  bl 0x8240f310
	ctx.lr = 0x8240F3C4;
	sub_8240F310(ctx, base);
	// 8240F3C4: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 8240F3C8: 40990034  ble cr6, 0x8240f3fc
	if !ctx.cr[6].gt {
	pc = 0x8240F3FC; continue 'dispatch;
	}
	// 8240F3CC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8240F3D0: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 8240F3D4: C3EB1FF8  lfs f31, 0x1ff8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	pc = 0x8240F3D8; continue 'dispatch;
            }
            0x8240F3D8 => {
    //   block [0x8240F3D8..0x8240F3F0)
	// 8240F3D8: C01E0010  lfs f0, 0x10(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240F3DC: FF00F800  fcmpu cr6, f0, f31
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[31].f64);
	// 8240F3E0: 419A0010  beq cr6, 0x8240f3f0
	if ctx.cr[6].eq {
	pc = 0x8240F3F0; continue 'dispatch;
	}
	// 8240F3E4: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8240F3E8: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 8240F3EC: 4BFF7C5D  bl 0x82407048
	ctx.lr = 0x8240F3F0;
	sub_82407048(ctx, base);
	pc = 0x8240F3F0; continue 'dispatch;
            }
            0x8240F3F0 => {
    //   block [0x8240F3F0..0x8240F3FC)
	// 8240F3F0: 37FFFFFF  addic. r31, r31, -1
	ctx.xer.ca = (ctx.r[31].u32 > (!(-1 as u32)));
	ctx.r[31].s64 = ctx.r[31].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 8240F3F4: 3BDE0020  addi r30, r30, 0x20
	ctx.r[30].s64 = ctx.r[30].s64 + 32;
	// 8240F3F8: 4082FFE0  bne 0x8240f3d8
	if !ctx.cr[0].eq {
	pc = 0x8240F3D8; continue 'dispatch;
	}
	pc = 0x8240F3FC; continue 'dispatch;
            }
            0x8240F3FC => {
    //   block [0x8240F3FC..0x8240F418)
	// 8240F3FC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8240F400: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240F404: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240F408: CBE1FFE0  lfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 8240F40C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8240F410: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8240F414: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240F418(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8240F418 size=24
    let mut pc: u32 = 0x8240F418;
    'dispatch: loop {
        match pc {
            0x8240F418 => {
    //   block [0x8240F418..0x8240F430)
	// 8240F418: EC01102A  fadds f0, f1, f2
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = ((ctx.f[1].f64 + ctx.f[2].f64) as f32) as f64;
	// 8240F41C: EC20182A  fadds f1, f0, f3
	ctx.f[1].f64 = ((ctx.f[0].f64 + ctx.f[3].f64) as f32) as f64;
	// 8240F420: FF012800  fcmpu cr6, f1, f5
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[5].f64);
	// 8240F424: 4C990020  blelr cr6
	if !ctx.cr[6].gt { return; }
	// 8240F428: EC21202A  fadds f1, f1, f4
	ctx.f[1].f64 = ((ctx.f[1].f64 + ctx.f[4].f64) as f32) as f64;
	// 8240F42C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240F430(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8240F430 size=128
    let mut pc: u32 = 0x8240F430;
    'dispatch: loop {
        match pc {
            0x8240F430 => {
    //   block [0x8240F430..0x8240F490)
	// 8240F430: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240F434: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240F438: DBC1FFE8  stfd f30, -0x18(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.f[30].u64 ) };
	// 8240F43C: DBE1FFF0  stfd f31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.f[31].u64 ) };
	// 8240F440: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240F444: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8240F448: FFC00890  fmr f30, f1
	ctx.f[30].f64 = ctx.f[1].f64;
	// 8240F44C: 7D453214  add r10, r5, r6
	ctx.r[10].u64 = ctx.r[5].u64 + ctx.r[6].u64;
	// 8240F450: C00B1FF8  lfs f0, 0x1ff8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240F454: 7D4B07B4  extsw r11, r10
	ctx.r[11].s64 = ctx.r[10].s32 as i64;
	// 8240F458: FF030000  fcmpu cr6, f3, f0
	ctx.cr[6].compare_f64(ctx.f[3].f64, ctx.f[0].f64);
	// 8240F45C: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 8240F460: C8010050  lfd f0, 0x50(r1)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 8240F464: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 8240F468: FC000018  frsp f0, f0
	ctx.f[0].f64 = (ctx.f[0].f64 as f32) as f64;
	// 8240F46C: EFE0102A  fadds f31, f0, f2
	ctx.f[31].f64 = ((ctx.f[0].f64 + ctx.f[2].f64) as f32) as f64;
	// 8240F470: 40980020  bge cr6, 0x8240f490
	if !ctx.cr[6].lt {
	pc = 0x8240F490; continue 'dispatch;
	}
	// 8240F474: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240F478: FC201890  fmr f1, f3
	ctx.f[1].f64 = ctx.f[3].f64;
	// 8240F47C: D8210018  stfd f1, 0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(24 as u32), ctx.f[1].u64 ) };
	// 8240F480: E8810018  ld r4, 0x18(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(24 as u32) ) };
	// 8240F484: 386BE9A8  addi r3, r11, -0x1658
	ctx.r[3].s64 = ctx.r[11].s64 + -5720;
	// 8240F488: 4BEA3AF9  bl 0x822b2f80
	ctx.lr = 0x8240F48C;
	sub_822B2F80(ctx, base);
	// 8240F48C: 48000008  b 0x8240f494
	pc = 0x8240F494; continue 'dispatch;
            }
            0x8240F490 => {
    //   block [0x8240F490..0x8240F494)
	// 8240F490: EFFF00F2  fmuls f31, f31, f3
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].f64 = (((ctx.f[31].f64 * ctx.f[3].f64) as f32) as f64);
	pc = 0x8240F494; continue 'dispatch;
            }
            0x8240F494 => {
    //   block [0x8240F494..0x8240F4B0)
	// 8240F494: EC3FF02A  fadds f1, f31, f30
	ctx.f[1].f64 = ((ctx.f[31].f64 + ctx.f[30].f64) as f32) as f64;
	// 8240F498: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8240F49C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240F4A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240F4A4: CBC1FFE8  lfd f30, -0x18(r1)
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8240F4A8: CBE1FFF0  lfd f31, -0x10(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8240F4AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240F4B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8240F4B0 size=12
    let mut pc: u32 = 0x8240F4B0;
    'dispatch: loop {
        match pc {
            0x8240F4B0 => {
    //   block [0x8240F4B0..0x8240F4BC)
	// 8240F4B0: EC01102A  fadds f0, f1, f2
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = ((ctx.f[1].f64 + ctx.f[2].f64) as f32) as f64;
	// 8240F4B4: EC20182A  fadds f1, f0, f3
	ctx.f[1].f64 = ((ctx.f[0].f64 + ctx.f[3].f64) as f32) as f64;
	// 8240F4B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240F4C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8240F4C0 size=144
    let mut pc: u32 = 0x8240F4C0;
    'dispatch: loop {
        match pc {
            0x8240F4C0 => {
    //   block [0x8240F4C0..0x8240F4D4)
	// 8240F4C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240F4C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240F4C8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240F4CC: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 8240F4D0: 409A0010  bne cr6, 0x8240f4e0
	if !ctx.cr[6].eq {
	pc = 0x8240F4E0; continue 'dispatch;
	}
	pc = 0x8240F4D4; continue 'dispatch;
            }
            0x8240F4D4 => {
    //   block [0x8240F4D4..0x8240F4E0)
	// 8240F4D4: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8240F4D8: C02B1850  lfs f1, 0x1850(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(6224 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8240F4DC: 48000064  b 0x8240f540
	pc = 0x8240F540; continue 'dispatch;
            }
            0x8240F4E0 => {
    //   block [0x8240F4E0..0x8240F4F4)
	// 8240F4E0: 40980014  bge cr6, 0x8240f4f4
	if !ctx.cr[6].lt {
	pc = 0x8240F4F4; continue 'dispatch;
	}
	// 8240F4E4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240F4E8: 386BE9EC  addi r3, r11, -0x1614
	ctx.r[3].s64 = ctx.r[11].s64 + -5652;
	// 8240F4EC: 4BEA3A95  bl 0x822b2f80
	ctx.lr = 0x8240F4F0;
	sub_822B2F80(ctx, base);
	// 8240F4F0: 4BFFFFE4  b 0x8240f4d4
	pc = 0x8240F4D4; continue 'dispatch;
            }
            0x8240F4F4 => {
    //   block [0x8240F4F4..0x8240F540)
	// 8240F4F4: 7F042800  cmpw cr6, r4, r5
	ctx.cr[6].compare_i32(ctx.r[4].s32, ctx.r[5].s32, &mut ctx.xer);
	// 8240F4F8: 4198FFDC  blt cr6, 0x8240f4d4
	if ctx.cr[6].lt {
	pc = 0x8240F4D4; continue 'dispatch;
	}
	// 8240F4FC: 7C8B07B4  extsw r11, r4
	ctx.r[11].s64 = ctx.r[4].s32 as i64;
	// 8240F500: 7CAA07B4  extsw r10, r5
	ctx.r[10].s64 = ctx.r[5].s32 as i64;
	// 8240F504: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 8240F508: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8240F50C: F9410058  std r10, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u64 ) };
	// 8240F510: C8010050  lfd f0, 0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 8240F514: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 8240F518: C9A10058  lfd f13, 0x58(r1)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 8240F51C: FDA06E9C  fcfid f13, f13
	ctx.f[13].f64 = (ctx.f[13].s64 as f64);
	// 8240F520: FD800018  frsp f12, f0
	ctx.f[12].f64 = (ctx.f[0].f64 as f32) as f64;
	// 8240F524: C00B1850  lfs f0, 0x1850(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(6224 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240F528: FDA06818  frsp f13, f13
	ctx.f[13].f64 = (ctx.f[13].f64 as f32) as f64;
	// 8240F52C: ED806024  fdivs f12, f0, f12
	ctx.f[12].f64 = ((ctx.f[0].f64 / ctx.f[12].f64) as f32) as f64;
	// 8240F530: EC2C0372  fmuls f1, f12, f13
	ctx.f[1].f64 = (((ctx.f[12].f64 * ctx.f[13].f64) as f32) as f64);
	// 8240F534: FF010000  fcmpu cr6, f1, f0
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 8240F538: 41980008  blt cr6, 0x8240f540
	if ctx.cr[6].lt {
	pc = 0x8240F540; continue 'dispatch;
	}
	// 8240F53C: FC200090  fmr f1, f0
	ctx.f[1].f64 = ctx.f[0].f64;
	pc = 0x8240F540; continue 'dispatch;
            }
            0x8240F540 => {
    //   block [0x8240F540..0x8240F550)
	// 8240F540: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8240F544: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240F548: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240F54C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240F550(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8240F550 size=144
    let mut pc: u32 = 0x8240F550;
    'dispatch: loop {
        match pc {
            0x8240F550 => {
    //   block [0x8240F550..0x8240F564)
	// 8240F550: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240F554: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240F558: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240F55C: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 8240F560: 409A0010  bne cr6, 0x8240f570
	if !ctx.cr[6].eq {
	pc = 0x8240F570; continue 'dispatch;
	}
	pc = 0x8240F564; continue 'dispatch;
            }
            0x8240F564 => {
    //   block [0x8240F564..0x8240F570)
	// 8240F564: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8240F568: C02B1FF8  lfs f1, 0x1ff8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8240F56C: 48000064  b 0x8240f5d0
	pc = 0x8240F5D0; continue 'dispatch;
            }
            0x8240F570 => {
    //   block [0x8240F570..0x8240F584)
	// 8240F570: 40980014  bge cr6, 0x8240f584
	if !ctx.cr[6].lt {
	pc = 0x8240F584; continue 'dispatch;
	}
	// 8240F574: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240F578: 386BEA2C  addi r3, r11, -0x15d4
	ctx.r[3].s64 = ctx.r[11].s64 + -5588;
	// 8240F57C: 4BEA3A05  bl 0x822b2f80
	ctx.lr = 0x8240F580;
	sub_822B2F80(ctx, base);
	// 8240F580: 4BFFFFE4  b 0x8240f564
	pc = 0x8240F564; continue 'dispatch;
            }
            0x8240F584 => {
    //   block [0x8240F584..0x8240F5D0)
	// 8240F584: 7F042800  cmpw cr6, r4, r5
	ctx.cr[6].compare_i32(ctx.r[4].s32, ctx.r[5].s32, &mut ctx.xer);
	// 8240F588: 4198FFDC  blt cr6, 0x8240f564
	if ctx.cr[6].lt {
	pc = 0x8240F564; continue 'dispatch;
	}
	// 8240F58C: 7C8B07B4  extsw r11, r4
	ctx.r[11].s64 = ctx.r[4].s32 as i64;
	// 8240F590: 7CAA07B4  extsw r10, r5
	ctx.r[10].s64 = ctx.r[5].s32 as i64;
	// 8240F594: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 8240F598: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8240F59C: F9410058  std r10, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u64 ) };
	// 8240F5A0: C8010050  lfd f0, 0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 8240F5A4: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 8240F5A8: C9A10058  lfd f13, 0x58(r1)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 8240F5AC: FDA06E9C  fcfid f13, f13
	ctx.f[13].f64 = (ctx.f[13].s64 as f64);
	// 8240F5B0: FD800018  frsp f12, f0
	ctx.f[12].f64 = (ctx.f[0].f64 as f32) as f64;
	// 8240F5B4: C00B1850  lfs f0, 0x1850(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(6224 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240F5B8: FDA06818  frsp f13, f13
	ctx.f[13].f64 = (ctx.f[13].f64 as f32) as f64;
	// 8240F5BC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8240F5C0: ED806024  fdivs f12, f0, f12
	ctx.f[12].f64 = ((ctx.f[0].f64 / ctx.f[12].f64) as f32) as f64;
	// 8240F5C4: EC0C037C  fnmsubs f0, f12, f13, f0
	ctx.f[0].f64 = -(((ctx.f[12].f64 * ctx.f[13].f64 - ctx.f[0].f64) as f32) as f64);
	// 8240F5C8: C1AB1FF8  lfs f13, 0x1ff8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8240F5CC: FC20682E  fsel f1, f0, f0, f13
	ctx.f[1].f64 = if ctx.f[0].f64 >= 0.0 { ctx.f[0].f64 } else { ctx.f[13].f64 };
	pc = 0x8240F5D0; continue 'dispatch;
            }
            0x8240F5D0 => {
    //   block [0x8240F5D0..0x8240F5E0)
	// 8240F5D0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8240F5D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240F5D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240F5DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240F5E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8240F5E0 size=188
    let mut pc: u32 = 0x8240F5E0;
    'dispatch: loop {
        match pc {
            0x8240F5E0 => {
    //   block [0x8240F5E0..0x8240F668)
	// 8240F5E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240F5E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240F5E8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8240F5EC: 3981FFF0  addi r12, r1, -0x10
	ctx.r[12].s64 = ctx.r[1].s64 + -16;
	// 8240F5F0: 481269F9  bl 0x82535fe8
	ctx.lr = 0x8240F5F4;
	sub_82535FB0(ctx, base);
	// 8240F5F4: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240F5F8: FFC00890  fmr f30, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[30].f64 = ctx.f[1].f64;
	// 8240F5FC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8240F600: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 8240F604: C38B1FF8  lfs f28, 0x1ff8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[28].f64 = (tmp.f32 as f64);
	// 8240F608: FF1EE000  fcmpu cr6, f30, f28
	ctx.cr[6].compare_f64(ctx.f[30].f64, ctx.f[28].f64);
	// 8240F60C: 419A0074  beq cr6, 0x8240f680
	if ctx.cr[6].eq {
	pc = 0x8240F680; continue 'dispatch;
	}
	// 8240F610: 4BFFEEC1  bl 0x8240e4d0
	ctx.lr = 0x8240F614;
	sub_8240E4D0(ctx, base);
	// 8240F614: FFE00890  fmr f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].f64 = ctx.f[1].f64;
	// 8240F618: 4BFFEEC9  bl 0x8240e4e0
	ctx.lr = 0x8240F61C;
	sub_8240E4E0(ctx, base);
	// 8240F61C: 786B0020  clrldi r11, r3, 0x20
	ctx.r[11].u64 = ctx.r[3].u64 & 0x00000000FFFFFFFFu64;
	// 8240F620: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 8240F624: C8010050  lfd f0, 0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 8240F628: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 8240F62C: FFA00018  frsp f29, f0
	ctx.f[29].f64 = (ctx.f[0].f64 as f32) as f64;
	// 8240F630: 4BFFEED9  bl 0x8240e508
	ctx.lr = 0x8240F634;
	sub_8240E508(ctx, base);
	// 8240F634: 786B0020  clrldi r11, r3, 0x20
	ctx.r[11].u64 = ctx.r[3].u64 & 0x00000000FFFFFFFFu64;
	// 8240F638: 2B1F0001  cmplwi cr6, r31, 1
	ctx.cr[6].compare_u32(ctx.r[31].u32, 1 as u32, &mut ctx.xer);
	// 8240F63C: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 8240F640: C8010050  lfd f0, 0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 8240F644: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 8240F648: FC000018  frsp f0, f0
	ctx.f[0].f64 = (ctx.f[0].f64 as f32) as f64;
	// 8240F64C: EC1D0024  fdivs f0, f29, f0
	ctx.f[0].f64 = ((ctx.f[29].f64 / ctx.f[0].f64) as f32) as f64;
	// 8240F650: EFE007F2  fmuls f31, f0, f31
	ctx.f[31].f64 = (((ctx.f[0].f64 * ctx.f[31].f64) as f32) as f64);
	// 8240F654: 409A0014  bne cr6, 0x8240f668
	if !ctx.cr[6].eq {
	pc = 0x8240F668; continue 'dispatch;
	}
	// 8240F658: 4BFFEE89  bl 0x8240e4e0
	ctx.lr = 0x8240F65C;
	sub_8240E4E0(ctx, base);
	// 8240F65C: 546B07FF  clrlwi. r11, r3, 0x1f
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8240F660: 4182001C  beq 0x8240f67c
	if ctx.cr[0].eq {
	pc = 0x8240F67C; continue 'dispatch;
	}
	// 8240F664: 4800000C  b 0x8240f670
	pc = 0x8240F670; continue 'dispatch;
            }
            0x8240F668 => {
    //   block [0x8240F668..0x8240F670)
	// 8240F668: FF1EE000  fcmpu cr6, f30, f28
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.cr[6].compare_f64(ctx.f[30].f64, ctx.f[28].f64);
	// 8240F66C: 40980010  bge cr6, 0x8240f67c
	if !ctx.cr[6].lt {
	pc = 0x8240F67C; continue 'dispatch;
	}
	pc = 0x8240F670; continue 'dispatch;
            }
            0x8240F670 => {
    //   block [0x8240F670..0x8240F67C)
	// 8240F670: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8240F674: C00B2074  lfs f0, 0x2074(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8308 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240F678: EFFF0032  fmuls f31, f31, f0
	ctx.f[31].f64 = (((ctx.f[31].f64 * ctx.f[0].f64) as f32) as f64);
	pc = 0x8240F67C; continue 'dispatch;
            }
            0x8240F67C => {
    //   block [0x8240F67C..0x8240F680)
	// 8240F67C: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	pc = 0x8240F680; continue 'dispatch;
            }
            0x8240F680 => {
    //   block [0x8240F680..0x8240F69C)
	// 8240F680: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8240F684: 3981FFF0  addi r12, r1, -0x10
	ctx.r[12].s64 = ctx.r[1].s64 + -16;
	// 8240F688: 481269AD  bl 0x82536034
	ctx.lr = 0x8240F68C;
	sub_82535FFC(ctx, base);
	// 8240F68C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240F690: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240F694: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8240F698: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240F6A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8240F6A0 size=116
    let mut pc: u32 = 0x8240F6A0;
    'dispatch: loop {
        match pc {
            0x8240F6A0 => {
    //   block [0x8240F6A0..0x8240F6C4)
	// 8240F6A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240F6A4: 48125A19  bl 0x825350bc
	ctx.lr = 0x8240F6A8;
	sub_82535080(ctx, base);
	// 8240F6A8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240F6AC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8240F6B0: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8240F6B4: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8240F6B8: 409A000C  bne cr6, 0x8240f6c4
	if !ctx.cr[6].eq {
	pc = 0x8240F6C4; continue 'dispatch;
	}
	// 8240F6BC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8240F6C0: 4800004C  b 0x8240f70c
	pc = 0x8240F70C; continue 'dispatch;
            }
            0x8240F6C4 => {
    //   block [0x8240F6C4..0x8240F6FC)
	// 8240F6C4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8240F6C8: 4BFFEE11  bl 0x8240e4d8
	ctx.lr = 0x8240F6CC;
	sub_8240E4D8(ctx, base);
	// 8240F6CC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8240F6D0: 4BFFEE11  bl 0x8240e4e0
	ctx.lr = 0x8240F6D4;
	sub_8240E4E0(ctx, base);
	// 8240F6D4: 7D63FB96  divwu r11, r3, r31
	ctx.r[11].u32 = ctx.r[3].u32 / ctx.r[31].u32;
	// 8240F6D8: 2B1D0001  cmplwi cr6, r29, 1
	ctx.cr[6].compare_u32(ctx.r[29].u32, 1 as u32, &mut ctx.xer);
	// 8240F6DC: 7D6BF9D6  mullw r11, r11, r31
	ctx.r[11].s32 = ((ctx.r[11].s32 as i64 * ctx.r[31].s32 as i64) as i32);
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 8240F6E0: 7D6B1850  subf r11, r11, r3
	ctx.r[11].s64 = ctx.r[3].s64 - ctx.r[11].s64;
	// 8240F6E4: 3BEB0001  addi r31, r11, 1
	ctx.r[31].s64 = ctx.r[11].s64 + 1;
	// 8240F6E8: 409A0014  bne cr6, 0x8240f6fc
	if !ctx.cr[6].eq {
	pc = 0x8240F6FC; continue 'dispatch;
	}
	// 8240F6EC: 4BFFEDF5  bl 0x8240e4e0
	ctx.lr = 0x8240F6F0;
	sub_8240E4E0(ctx, base);
	// 8240F6F0: 546B07FF  clrlwi. r11, r3, 0x1f
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8240F6F4: 41820014  beq 0x8240f708
	if ctx.cr[0].eq {
	pc = 0x8240F708; continue 'dispatch;
	}
	// 8240F6F8: 4800000C  b 0x8240f704
	pc = 0x8240F704; continue 'dispatch;
            }
            0x8240F6FC => {
    //   block [0x8240F6FC..0x8240F704)
	// 8240F6FC: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8240F700: 40980008  bge cr6, 0x8240f708
	if !ctx.cr[6].lt {
	pc = 0x8240F708; continue 'dispatch;
	}
	pc = 0x8240F704; continue 'dispatch;
            }
            0x8240F704 => {
    //   block [0x8240F704..0x8240F708)
	// 8240F704: 7FFF00D0  neg r31, r31
	ctx.r[31].s64 = -ctx.r[31].s64;
	pc = 0x8240F708; continue 'dispatch;
            }
            0x8240F708 => {
    //   block [0x8240F708..0x8240F70C)
	// 8240F708: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	pc = 0x8240F70C; continue 'dispatch;
            }
            0x8240F70C => {
    //   block [0x8240F70C..0x8240F714)
	// 8240F70C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8240F710: 481259FC  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240F718(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8240F718 size=76
    let mut pc: u32 = 0x8240F718;
    'dispatch: loop {
        match pc {
            0x8240F718 => {
    //   block [0x8240F718..0x8240F740)
	// 8240F718: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240F71C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240F720: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8240F724: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240F728: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8240F72C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8240F730: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8240F734: 556B07BD  rlwinm. r11, r11, 0, 0x1e, 0x1e
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8240F738: 41820008  beq 0x8240f740
	if ctx.cr[0].eq {
	pc = 0x8240F740; continue 'dispatch;
	}
	// 8240F73C: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	pc = 0x8240F740; continue 'dispatch;
            }
            0x8240F740 => {
    //   block [0x8240F740..0x8240F764)
	// 8240F740: C03F009C  lfs f1, 0x9c(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(156 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8240F744: 4BFFFE9D  bl 0x8240f5e0
	ctx.lr = 0x8240F748;
	sub_8240F5E0(ctx, base);
	// 8240F748: C01F0034  lfs f0, 0x34(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240F74C: EC20082A  fadds f1, f0, f1
	ctx.f[1].f64 = ((ctx.f[0].f64 + ctx.f[1].f64) as f32) as f64;
	// 8240F750: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8240F754: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240F758: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240F75C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8240F760: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240F768(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8240F768 size=168
    let mut pc: u32 = 0x8240F768;
    'dispatch: loop {
        match pc {
            0x8240F768 => {
    //   block [0x8240F768..0x8240F794)
	// 8240F768: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240F76C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240F770: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8240F774: DBE1FFE8  stfd f31, -0x18(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.f[31].u64 ) };
	// 8240F778: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240F77C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8240F780: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8240F784: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8240F788: 556B07FF  clrlwi. r11, r11, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8240F78C: 41820008  beq 0x8240f794
	if ctx.cr[0].eq {
	pc = 0x8240F794; continue 'dispatch;
	}
	// 8240F790: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	pc = 0x8240F794; continue 'dispatch;
            }
            0x8240F794 => {
    //   block [0x8240F794..0x8240F7DC)
	// 8240F794: 809F0098  lwz r4, 0x98(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(152 as u32) ) } as u64;
	// 8240F798: 4BFFFF09  bl 0x8240f6a0
	ctx.lr = 0x8240F79C;
	sub_8240F6A0(ctx, base);
	// 8240F79C: 7C6B07B4  extsw r11, r3
	ctx.r[11].s64 = ctx.r[3].s32 as i64;
	// 8240F7A0: C03F00A8  lfs f1, 0xa8(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(168 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8240F7A4: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 8240F7A8: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8240F7AC: C00B1FF8  lfs f0, 0x1ff8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240F7B0: FF010000  fcmpu cr6, f1, f0
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 8240F7B4: C8010050  lfd f0, 0x50(r1)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 8240F7B8: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 8240F7BC: FFE00018  frsp f31, f0
	ctx.f[31].f64 = (ctx.f[0].f64 as f32) as f64;
	// 8240F7C0: 4098001C  bge cr6, 0x8240f7dc
	if !ctx.cr[6].lt {
	pc = 0x8240F7DC; continue 'dispatch;
	}
	// 8240F7C4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240F7C8: D8210018  stfd f1, 0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(24 as u32), ctx.f[1].u64 ) };
	// 8240F7CC: E8810018  ld r4, 0x18(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(24 as u32) ) };
	// 8240F7D0: 386BEA70  addi r3, r11, -0x1590
	ctx.r[3].s64 = ctx.r[11].s64 + -5520;
	// 8240F7D4: 4BEA37AD  bl 0x822b2f80
	ctx.lr = 0x8240F7D8;
	sub_822B2F80(ctx, base);
	// 8240F7D8: 48000008  b 0x8240f7e0
	pc = 0x8240F7E0; continue 'dispatch;
            }
            0x8240F7DC => {
    //   block [0x8240F7DC..0x8240F7E0)
	// 8240F7DC: EFE107F2  fmuls f31, f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].f64 = (((ctx.f[1].f64 * ctx.f[31].f64) as f32) as f64);
	pc = 0x8240F7E0; continue 'dispatch;
            }
            0x8240F7E0 => {
    //   block [0x8240F7E0..0x8240F810)
	// 8240F7E0: E97F003E  lwa r11, 0x3c(r31)
	ctx.r[11].s64 = (unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) } as i32) as i64;
	// 8240F7E4: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 8240F7E8: C8010050  lfd f0, 0x50(r1)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 8240F7EC: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 8240F7F0: FC000018  frsp f0, f0
	ctx.f[0].f64 = (ctx.f[0].f64 as f32) as f64;
	// 8240F7F4: EC20F82A  fadds f1, f0, f31
	ctx.f[1].f64 = ((ctx.f[0].f64 + ctx.f[31].f64) as f32) as f64;
	// 8240F7F8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8240F7FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240F800: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240F804: CBE1FFE8  lfd f31, -0x18(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8240F808: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8240F80C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240F810(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8240F810 size=76
    let mut pc: u32 = 0x8240F810;
    'dispatch: loop {
        match pc {
            0x8240F810 => {
    //   block [0x8240F810..0x8240F838)
	// 8240F810: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240F814: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240F818: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8240F81C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240F820: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8240F824: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8240F828: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8240F82C: 556B077B  rlwinm. r11, r11, 0, 0x1d, 0x1d
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8240F830: 41820008  beq 0x8240f838
	if ctx.cr[0].eq {
	pc = 0x8240F838; continue 'dispatch;
	}
	// 8240F834: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	pc = 0x8240F838; continue 'dispatch;
            }
            0x8240F838 => {
    //   block [0x8240F838..0x8240F85C)
	// 8240F838: C03F00A0  lfs f1, 0xa0(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(160 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8240F83C: 4BFFFDA5  bl 0x8240f5e0
	ctx.lr = 0x8240F840;
	sub_8240F5E0(ctx, base);
	// 8240F840: C01F0040  lfs f0, 0x40(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240F844: EC20082A  fadds f1, f0, f1
	ctx.f[1].f64 = ((ctx.f[0].f64 + ctx.f[1].f64) as f32) as f64;
	// 8240F848: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8240F84C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240F850: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240F854: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8240F858: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240F860(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8240F860 size=112
    let mut pc: u32 = 0x8240F860;
    'dispatch: loop {
        match pc {
            0x8240F860 => {
    //   block [0x8240F860..0x8240F888)
	// 8240F860: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240F864: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240F868: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8240F86C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240F870: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8240F874: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8240F878: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8240F87C: 556B0739  rlwinm. r11, r11, 0, 0x1c, 0x1c
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8240F880: 41820008  beq 0x8240f888
	if ctx.cr[0].eq {
	pc = 0x8240F888; continue 'dispatch;
	}
	// 8240F884: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	pc = 0x8240F888; continue 'dispatch;
            }
            0x8240F888 => {
    //   block [0x8240F888..0x8240F8B8)
	// 8240F888: C03F00A4  lfs f1, 0xa4(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8240F88C: 4BFFFD55  bl 0x8240f5e0
	ctx.lr = 0x8240F890;
	sub_8240F5E0(ctx, base);
	// 8240F890: C01F0048  lfs f0, 0x48(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240F894: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8240F898: EC20082A  fadds f1, f0, f1
	ctx.f[1].f64 = ((ctx.f[0].f64 + ctx.f[1].f64) as f32) as f64;
	// 8240F89C: C00B2074  lfs f0, 0x2074(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8308 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240F8A0: FF010000  fcmpu cr6, f1, f0
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 8240F8A4: 41980014  blt cr6, 0x8240f8b8
	if ctx.cr[6].lt {
	pc = 0x8240F8B8; continue 'dispatch;
	}
	// 8240F8A8: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8240F8AC: C00B1850  lfs f0, 0x1850(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(6224 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240F8B0: FF010000  fcmpu cr6, f1, f0
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 8240F8B4: 40990008  ble cr6, 0x8240f8bc
	if !ctx.cr[6].gt {
	pc = 0x8240F8BC; continue 'dispatch;
	}
	pc = 0x8240F8B8; continue 'dispatch;
            }
            0x8240F8B8 => {
    //   block [0x8240F8B8..0x8240F8BC)
	// 8240F8B8: FC200090  fmr f1, f0
	ctx.f[1].f64 = ctx.f[0].f64;
	pc = 0x8240F8BC; continue 'dispatch;
            }
            0x8240F8BC => {
    //   block [0x8240F8BC..0x8240F8D0)
	// 8240F8BC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8240F8C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240F8C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240F8C8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8240F8CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240F8D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8240F8D0 size=256
    let mut pc: u32 = 0x8240F8D0;
    'dispatch: loop {
        match pc {
            0x8240F8D0 => {
    //   block [0x8240F8D0..0x8240F940)
	// 8240F8D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240F8D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240F8D8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8240F8DC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8240F8E0: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 8240F8E4: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240F8E8: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 8240F8EC: FFE01090  fmr f31, f2
	ctx.f[31].f64 = ctx.f[2].f64;
	// 8240F8F0: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 8240F8F4: 48000585  bl 0x8240fe78
	ctx.lr = 0x8240F8F8;
	sub_8240FE78(ctx, base);
	// 8240F8F8: 4BFFEBD9  bl 0x8240e4d0
	ctx.lr = 0x8240F8FC;
	sub_8240E4D0(ctx, base);
	// 8240F8FC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8240F900: 419A00B0  beq cr6, 0x8240f9b0
	if ctx.cr[6].eq {
	pc = 0x8240F9B0; continue 'dispatch;
	}
	// 8240F904: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8240F908: 419A00A8  beq cr6, 0x8240f9b0
	if ctx.cr[6].eq {
	pc = 0x8240F9B0; continue 'dispatch;
	}
	// 8240F90C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8240F910: C00B212C  lfs f0, 0x212c(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8492 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240F914: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 8240F918: EC010032  fmuls f0, f1, f0
	ctx.f[0].f64 = (((ctx.f[1].f64 * ctx.f[0].f64) as f32) as f64);
	// 8240F91C: FC00001E  fctiwz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[0].f64.trunc() as i32 as i64 };
	// 8240F920: 7C005FAE  stfiwx f0, 0, r11
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32, tmp.u32) };
	// 8240F924: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8240F928: 2F0A0007  cmpwi cr6, r10, 7
	ctx.cr[6].compare_i32(ctx.r[10].s32, 7, &mut ctx.xer);
	// 8240F92C: 40980084  bge cr6, 0x8240f9b0
	if !ctx.cr[6].lt {
	pc = 0x8240F9B0; continue 'dispatch;
	}
	// 8240F930: 2F0A0006  cmpwi cr6, r10, 6
	ctx.cr[6].compare_i32(ctx.r[10].s32, 6, &mut ctx.xer);
	// 8240F934: 409A000C  bne cr6, 0x8240f940
	if !ctx.cr[6].eq {
	pc = 0x8240F940; continue 'dispatch;
	}
	// 8240F938: C17F0018  lfs f11, 0x18(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 8240F93C: 48000064  b 0x8240f9a0
	pc = 0x8240F9A0; continue 'dispatch;
            }
            0x8240F940 => {
    //   block [0x8240F940..0x8240F9A0)
	// 8240F940: 1D6A001E  mulli r11, r10, 0x1e
	ctx.r[11].s32 = ((ctx.r[10].s32 as i64 * 30 as i64) as i32);
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 8240F944: 392B001E  addi r9, r11, 0x1e
	ctx.r[9].s64 = ctx.r[11].s64 + 30;
	// 8240F948: 7D6B07B4  extsw r11, r11
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 8240F94C: 7D2907B4  extsw r9, r9
	ctx.r[9].s64 = ctx.r[9].s32 as i64;
	// 8240F950: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8240F954: 7D4AFA14  add r10, r10, r31
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[31].u64;
	// 8240F958: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 8240F95C: C8010050  lfd f0, 0x50(r1)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 8240F960: F9210058  std r9, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[9].u64 ) };
	// 8240F964: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 8240F968: C16A0004  lfs f11, 4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 8240F96C: C1AA0000  lfs f13, 0(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8240F970: FC000018  frsp f0, f0
	ctx.f[0].f64 = (ctx.f[0].f64 as f32) as f64;
	// 8240F974: C9810058  lfd f12, 0x58(r1)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 8240F978: FD80669C  fcfid f12, f12
	ctx.f[12].f64 = (ctx.f[12].s64 as f64);
	// 8240F97C: FD806018  frsp f12, f12
	ctx.f[12].f64 = (ctx.f[12].f64 as f32) as f64;
	// 8240F980: FF006000  fcmpu cr6, f0, f12
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[12].f64);
	// 8240F984: 419A001C  beq cr6, 0x8240f9a0
	if ctx.cr[6].eq {
	pc = 0x8240F9A0; continue 'dispatch;
	}
	// 8240F988: ED410028  fsubs f10, f1, f0
	ctx.f[10].f64 = (((ctx.f[1].f64 - ctx.f[0].f64) as f32) as f64);
	// 8240F98C: ED6B6828  fsubs f11, f11, f13
	ctx.f[11].f64 = (((ctx.f[11].f64 - ctx.f[13].f64) as f32) as f64);
	// 8240F990: EC0C0028  fsubs f0, f12, f0
	ctx.f[0].f64 = (((ctx.f[12].f64 - ctx.f[0].f64) as f32) as f64);
	// 8240F994: ED8B02B2  fmuls f12, f11, f10
	ctx.f[12].f64 = (((ctx.f[11].f64 * ctx.f[10].f64) as f32) as f64);
	// 8240F998: EC0C0024  fdivs f0, f12, f0
	ctx.f[0].f64 = ((ctx.f[12].f64 / ctx.f[0].f64) as f32) as f64;
	// 8240F99C: ED60682A  fadds f11, f0, f13
	ctx.f[11].f64 = ((ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64;
	pc = 0x8240F9A0; continue 'dispatch;
            }
            0x8240F9A0 => {
    //   block [0x8240F9A0..0x8240F9B0)
	// 8240F9A0: EC0BF82A  fadds f0, f11, f31
	ctx.f[0].f64 = ((ctx.f[11].f64 + ctx.f[31].f64) as f32) as f64;
	// 8240F9A4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8240F9A8: D01E0000  stfs f0, 0(r30)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 8240F9AC: 48000008  b 0x8240f9b4
	pc = 0x8240F9B4; continue 'dispatch;
            }
            0x8240F9B0 => {
    //   block [0x8240F9B0..0x8240F9B4)
	// 8240F9B0: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	pc = 0x8240F9B4; continue 'dispatch;
            }
            0x8240F9B4 => {
    //   block [0x8240F9B4..0x8240F9D0)
	// 8240F9B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8240F9B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240F9BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240F9C0: CBE1FFE0  lfd f31, -0x20(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 8240F9C4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8240F9C8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8240F9CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240F9D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8240F9D0 size=16
    let mut pc: u32 = 0x8240F9D0;
    'dispatch: loop {
        match pc {
            0x8240F9D0 => {
    //   block [0x8240F9D0..0x8240F9E0)
	// 8240F9D0: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 8240F9D4: 409A000C  bne cr6, 0x8240f9e0
	if !ctx.cr[6].eq {
		sub_8240F9E0(ctx, base);
		return;
	}
	// 8240F9D8: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 8240F9DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240F9E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8240F9E0 size=120
    let mut pc: u32 = 0x8240F9E0;
    'dispatch: loop {
        match pc {
            0x8240F9E0 => {
    //   block [0x8240F9E0..0x8240FA08)
	// 8240F9E0: 2B030001  cmplwi cr6, r3, 1
	ctx.cr[6].compare_u32(ctx.r[3].u32, 1 as u32, &mut ctx.xer);
	// 8240F9E4: 41980044  blt cr6, 0x8240fa28
	if ctx.cr[6].lt {
	pc = 0x8240FA28; continue 'dispatch;
	}
	// 8240F9E8: 419A0038  beq cr6, 0x8240fa20
	if ctx.cr[6].eq {
	pc = 0x8240FA20; continue 'dispatch;
	}
	// 8240F9EC: 2B030004  cmplwi cr6, r3, 4
	ctx.cr[6].compare_u32(ctx.r[3].u32, 4 as u32, &mut ctx.xer);
	// 8240F9F0: 409AFFE8  bne cr6, 0x8240f9d8
	if !ctx.cr[6].eq {
		sub_8240F9D0(ctx, base);
		return;
	}
	// 8240F9F4: 89640001  lbz r11, 1(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(1 as u32) ) } as u64;
	// 8240F9F8: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8240F9FC: 2C0B0000  cmpwi r11, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8240FA00: 40810050  ble 0x8240fa50
	if !ctx.cr[0].gt {
	pc = 0x8240FA50; continue 'dispatch;
	}
	// 8240FA04: 39440028  addi r10, r4, 0x28
	ctx.r[10].s64 = ctx.r[4].s64 + 40;
	pc = 0x8240FA08; continue 'dispatch;
            }
            0x8240FA08 => {
    //   block [0x8240FA08..0x8240FA20)
	// 8240FA08: 890A0000  lbz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8240FA0C: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8240FA10: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 8240FA14: 7D284A14  add r9, r8, r9
	ctx.r[9].u64 = ctx.r[8].u64 + ctx.r[9].u64;
	// 8240FA18: 4082FFF0  bne 0x8240fa08
	if !ctx.cr[0].eq {
	pc = 0x8240FA08; continue 'dispatch;
	}
	// 8240FA1C: 48000034  b 0x8240fa50
	pc = 0x8240FA50; continue 'dispatch;
            }
            0x8240FA20 => {
    //   block [0x8240FA20..0x8240FA28)
	// 8240FA20: A1240002  lhz r9, 2(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(2 as u32) ) } as u64;
	// 8240FA24: 4800002C  b 0x8240fa50
	pc = 0x8240FA50; continue 'dispatch;
            }
            0x8240FA28 => {
    //   block [0x8240FA28..0x8240FA3C)
	// 8240FA28: A1640008  lhz r11, 8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 8240FA2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8240FA30: 2C0B0000  cmpwi r11, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8240FA34: 4081001C  ble 0x8240fa50
	if !ctx.cr[0].gt {
	pc = 0x8240FA50; continue 'dispatch;
	}
	// 8240FA38: 3944001D  addi r10, r4, 0x1d
	ctx.r[10].s64 = ctx.r[4].s64 + 29;
	pc = 0x8240FA3C; continue 'dispatch;
            }
            0x8240FA3C => {
    //   block [0x8240FA3C..0x8240FA50)
	// 8240FA3C: 890A0000  lbz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8240FA40: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8240FA44: 394A0014  addi r10, r10, 0x14
	ctx.r[10].s64 = ctx.r[10].s64 + 20;
	// 8240FA48: 7D284A14  add r9, r8, r9
	ctx.r[9].u64 = ctx.r[8].u64 + ctx.r[9].u64;
	// 8240FA4C: 4082FFF0  bne 0x8240fa3c
	if !ctx.cr[0].eq {
	pc = 0x8240FA3C; continue 'dispatch;
	}
	pc = 0x8240FA50; continue 'dispatch;
            }
            0x8240FA50 => {
    //   block [0x8240FA50..0x8240FA58)
	// 8240FA50: 7D234B78  mr r3, r9
	ctx.r[3].u64 = ctx.r[9].u64;
	// 8240FA54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240FA58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8240FA58 size=76
    let mut pc: u32 = 0x8240FA58;
    'dispatch: loop {
        match pc {
            0x8240FA58 => {
    //   block [0x8240FA58..0x8240FA78)
	// 8240FA58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240FA5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240FA60: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8240FA64: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8240FA68: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240FA6C: 3FE30002  addis r31, r3, 2
	ctx.r[31].s64 = ctx.r[3].s64 + 131072;
	// 8240FA70: 3BC00FFF  li r30, 0xfff
	ctx.r[30].s64 = 4095;
	// 8240FA74: 3BFF0008  addi r31, r31, 8
	ctx.r[31].s64 = ctx.r[31].s64 + 8;
	pc = 0x8240FA78; continue 'dispatch;
            }
            0x8240FA78 => {
    //   block [0x8240FA78..0x8240FAA4)
	// 8240FA78: 3BFFFFE0  addi r31, r31, -0x20
	ctx.r[31].s64 = ctx.r[31].s64 + -32;
	// 8240FA7C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8240FA80: 4BEA3501  bl 0x822b2f80
	ctx.lr = 0x8240FA84;
	sub_822B2F80(ctx, base);
	// 8240FA84: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8240FA88: 4080FFF0  bge 0x8240fa78
	if !ctx.cr[0].lt {
	pc = 0x8240FA78; continue 'dispatch;
	}
	// 8240FA8C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8240FA90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240FA94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240FA98: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8240FA9C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8240FAA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240FAA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8240FAA8 size=92
    let mut pc: u32 = 0x8240FAA8;
    'dispatch: loop {
        match pc {
            0x8240FAA8 => {
    //   block [0x8240FAA8..0x8240FAD0)
	// 8240FAA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240FAAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240FAB0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8240FAB4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8240FAB8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240FABC: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 8240FAC0: 3BC30008  addi r30, r3, 8
	ctx.r[30].s64 = ctx.r[3].s64 + 8;
	// 8240FAC4: 3BE01000  li r31, 0x1000
	ctx.r[31].s64 = 4096;
	// 8240FAC8: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8240FACC: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	pc = 0x8240FAD0; continue 'dispatch;
            }
            0x8240FAD0 => {
    //   block [0x8240FAD0..0x8240FB04)
	// 8240FAD0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8240FAD4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8240FAD8: 48000571  bl 0x82410048
	ctx.lr = 0x8240FADC;
	sub_82410048(ctx, base);
	// 8240FADC: 37FFFFFF  addic. r31, r31, -1
	ctx.xer.ca = (ctx.r[31].u32 > (!(-1 as u32)));
	ctx.r[31].s64 = ctx.r[31].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 8240FAE0: 3BDE0020  addi r30, r30, 0x20
	ctx.r[30].s64 = ctx.r[30].s64 + 32;
	// 8240FAE4: 4082FFEC  bne 0x8240fad0
	if !ctx.cr[0].eq {
	pc = 0x8240FAD0; continue 'dispatch;
	}
	// 8240FAE8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8240FAEC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8240FAF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240FAF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240FAF8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8240FAFC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8240FB00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240FB08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8240FB08 size=116
    let mut pc: u32 = 0x8240FB08;
    'dispatch: loop {
        match pc {
            0x8240FB08 => {
    //   block [0x8240FB08..0x8240FB24)
	// 8240FB08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240FB0C: 481255AD  bl 0x825350b8
	ctx.lr = 0x8240FB10;
	sub_82535080(ctx, base);
	// 8240FB10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240FB14: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8240FB18: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 8240FB1C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8240FB20: 3BBE0008  addi r29, r30, 8
	ctx.r[29].s64 = ctx.r[30].s64 + 8;
	pc = 0x8240FB24; continue 'dispatch;
            }
            0x8240FB24 => {
    //   block [0x8240FB24..0x8240FB48)
	// 8240FB24: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8240FB28: 48000551  bl 0x82410078
	ctx.lr = 0x8240FB2C;
	sub_82410078(ctx, base);
	// 8240FB2C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8240FB30: 41820020  beq 0x8240fb50
	if ctx.cr[0].eq {
	pc = 0x8240FB50; continue 'dispatch;
	}
	// 8240FB34: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 8240FB38: 3BBD0020  addi r29, r29, 0x20
	ctx.r[29].s64 = ctx.r[29].s64 + 32;
	// 8240FB3C: 2F1F1000  cmpwi cr6, r31, 0x1000
	ctx.cr[6].compare_i32(ctx.r[31].s32, 4096, &mut ctx.xer);
	// 8240FB40: 4198FFE4  blt cr6, 0x8240fb24
	if ctx.cr[6].lt {
	pc = 0x8240FB24; continue 'dispatch;
	}
	// 8240FB44: 3860FFFD  li r3, -3
	ctx.r[3].s64 = -3;
	pc = 0x8240FB48; continue 'dispatch;
            }
            0x8240FB48 => {
    //   block [0x8240FB48..0x8240FB50)
	// 8240FB48: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8240FB4C: 481255BC  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            0x8240FB50 => {
    //   block [0x8240FB50..0x8240FB74)
	// 8240FB50: 57EB2834  slwi r11, r31, 5
	ctx.r[11].u32 = ctx.r[31].u32.wrapping_shl(5);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8240FB54: 389C0008  addi r4, r28, 8
	ctx.r[4].s64 = ctx.r[28].s64 + 8;
	// 8240FB58: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 8240FB5C: 386B0008  addi r3, r11, 8
	ctx.r[3].s64 = ctx.r[11].s64 + 8;
	// 8240FB60: 48000531  bl 0x82410090
	ctx.lr = 0x8240FB64;
	sub_82410090(ctx, base);
	// 8240FB64: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8240FB68: 4082000C  bne 0x8240fb74
	if !ctx.cr[0].eq {
	pc = 0x8240FB74; continue 'dispatch;
	}
	// 8240FB6C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8240FB70: 4BFFFFD8  b 0x8240fb48
	pc = 0x8240FB48; continue 'dispatch;
            }
            0x8240FB74 => {
    //   block [0x8240FB74..0x8240FB7C)
	// 8240FB74: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 8240FB78: 4BFFFFD0  b 0x8240fb48
	pc = 0x8240FB48; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240FB80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8240FB80 size=16
    let mut pc: u32 = 0x8240FB80;
    'dispatch: loop {
        match pc {
            0x8240FB80 => {
    //   block [0x8240FB80..0x8240FB90)
	// 8240FB80: 548B2834  slwi r11, r4, 5
	ctx.r[11].u32 = ctx.r[4].u32.wrapping_shl(5);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8240FB84: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 8240FB88: 386B0008  addi r3, r11, 8
	ctx.r[3].s64 = ctx.r[11].s64 + 8;
	// 8240FB8C: 48000564  b 0x824100f0
	sub_824100F0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240FB90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8240FB90 size=16
    let mut pc: u32 = 0x8240FB90;
    'dispatch: loop {
        match pc {
            0x8240FB90 => {
    //   block [0x8240FB90..0x8240FBA0)
	// 8240FB90: 548B2834  slwi r11, r4, 5
	ctx.r[11].u32 = ctx.r[4].u32.wrapping_shl(5);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8240FB94: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 8240FB98: 386B0008  addi r3, r11, 8
	ctx.r[3].s64 = ctx.r[11].s64 + 8;
	// 8240FB9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240FBA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8240FBA0 size=20
    let mut pc: u32 = 0x8240FBA0;
    'dispatch: loop {
        match pc {
            0x8240FBA0 => {
    //   block [0x8240FBA0..0x8240FBB4)
	// 8240FBA0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8240FBA4: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 8240FBA8: 4098000C  bge cr6, 0x8240fbb4
	if !ctx.cr[6].lt {
		crate::recompiler::externs::call(ctx, base, 0x8240FBB4);
		return;
	}
	// 8240FBAC: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 8240FBB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240FBD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8240FBD0 size=44
    let mut pc: u32 = 0x8240FBD0;
    'dispatch: loop {
        match pc {
            0x8240FBD0 => {
    //   block [0x8240FBD0..0x8240FBF4)
	// 8240FBD0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8240FBD4: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 8240FBD8: 4198001C  blt cr6, 0x8240fbf4
	if ctx.cr[6].lt {
	pc = 0x8240FBF4; continue 'dispatch;
	}
	// 8240FBDC: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8240FBE0: 7C645051  subf. r3, r4, r10
	ctx.r[3].s64 = ctx.r[10].s64 - ctx.r[4].s64;
	ctx.cr[0].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8240FBE4: 906B0004  stw r3, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 8240FBE8: 4C800020  bgelr
	if !ctx.cr[0].lt { return; }
	// 8240FBEC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8240FBF0: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	pc = 0x8240FBF4; continue 'dispatch;
            }
            0x8240FBF4 => {
    //   block [0x8240FBF4..0x8240FBFC)
	// 8240FBF4: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 8240FBF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240FC00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8240FC00 size=64
    let mut pc: u32 = 0x8240FC00;
    'dispatch: loop {
        match pc {
            0x8240FC00 => {
    //   block [0x8240FC00..0x8240FC18)
	// 8240FC00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240FC04: 481254B9  bl 0x825350bc
	ctx.lr = 0x8240FC08;
	sub_82535080(ctx, base);
	// 8240FC08: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240FC0C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8240FC10: 3BE00FFF  li r31, 0xfff
	ctx.r[31].s64 = 4095;
	// 8240FC14: 3BDD0008  addi r30, r29, 8
	ctx.r[30].s64 = ctx.r[29].s64 + 8;
	pc = 0x8240FC18; continue 'dispatch;
            }
            0x8240FC18 => {
    //   block [0x8240FC18..0x8240FC40)
	// 8240FC18: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8240FC1C: 4800041D  bl 0x82410038
	ctx.lr = 0x8240FC20;
	sub_82410038(ctx, base);
	// 8240FC20: 37FFFFFF  addic. r31, r31, -1
	ctx.xer.ca = (ctx.r[31].u32 > (!(-1 as u32)));
	ctx.r[31].s64 = ctx.r[31].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 8240FC24: 3BDE0020  addi r30, r30, 0x20
	ctx.r[30].s64 = ctx.r[30].s64 + 32;
	// 8240FC28: 4080FFF0  bge 0x8240fc18
	if !ctx.cr[0].lt {
	pc = 0x8240FC18; continue 'dispatch;
	}
	// 8240FC2C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8240FC30: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8240FC34: 917D0004  stw r11, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8240FC38: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8240FC3C: 481254D0  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240FC40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8240FC40 size=8
    let mut pc: u32 = 0x8240FC40;
    'dispatch: loop {
        match pc {
            0x8240FC40 => {
    //   block [0x8240FC40..0x8240FC48)
	// 8240FC40: 48000598  b 0x824101d8
	sub_824101D8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240FC48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8240FC48 size=8
    let mut pc: u32 = 0x8240FC48;
    'dispatch: loop {
        match pc {
            0x8240FC48 => {
    //   block [0x8240FC48..0x8240FC50)
	// 8240FC48: 48000608  b 0x82410250
	sub_82410250(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240FC50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8240FC50 size=8
    let mut pc: u32 = 0x8240FC50;
    'dispatch: loop {
        match pc {
            0x8240FC50 => {
    //   block [0x8240FC50..0x8240FC58)
	// 8240FC50: 48000678  b 0x824102c8
	sub_824102C8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240FC58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8240FC58 size=8
    let mut pc: u32 = 0x8240FC58;
    'dispatch: loop {
        match pc {
            0x8240FC58 => {
    //   block [0x8240FC58..0x8240FC60)
	// 8240FC58: 48000678  b 0x824102d0
	sub_824102D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240FC60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8240FC60 size=8
    let mut pc: u32 = 0x8240FC60;
    'dispatch: loop {
        match pc {
            0x8240FC60 => {
    //   block [0x8240FC60..0x8240FC68)
	// 8240FC60: 480006A8  b 0x82410308
	sub_82410308(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240FC68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8240FC68 size=8
    let mut pc: u32 = 0x8240FC68;
    'dispatch: loop {
        match pc {
            0x8240FC68 => {
    //   block [0x8240FC68..0x8240FC70)
	// 8240FC68: 48000710  b 0x82410378
	sub_82410378(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240FC70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8240FC70 size=8
    let mut pc: u32 = 0x8240FC70;
    'dispatch: loop {
        match pc {
            0x8240FC70 => {
    //   block [0x8240FC70..0x8240FC78)
	// 8240FC70: 48000760  b 0x824103d0
	sub_824103D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240FC78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8240FC78 size=8
    let mut pc: u32 = 0x8240FC78;
    'dispatch: loop {
        match pc {
            0x8240FC78 => {
    //   block [0x8240FC78..0x8240FC80)
	// 8240FC78: 48000780  b 0x824103f8
	sub_824103F8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240FC80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8240FC80 size=8
    let mut pc: u32 = 0x8240FC80;
    'dispatch: loop {
        match pc {
            0x8240FC80 => {
    //   block [0x8240FC80..0x8240FC88)
	// 8240FC80: 48000798  b 0x82410418
	sub_82410418(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240FC88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8240FC88 size=68
    let mut pc: u32 = 0x8240FC88;
    'dispatch: loop {
        match pc {
            0x8240FC88 => {
    //   block [0x8240FC88..0x8240FCCC)
	// 8240FC88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240FC8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240FC90: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8240FC94: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240FC98: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8240FC9C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8240FCA0: 48000831  bl 0x824104d0
	ctx.lr = 0x8240FCA4;
	sub_824104D0(ctx, base);
	// 8240FCA4: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8240FCA8: 38A00040  li r5, 0x40
	ctx.r[5].s64 = 64;
	// 8240FCAC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8240FCB0: 48124EA1  bl 0x82534b50
	ctx.lr = 0x8240FCB4;
	sub_82534B50(ctx, base);
	// 8240FCB4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8240FCB8: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 8240FCBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240FCC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240FCC4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8240FCC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240FCD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8240FCD0 size=336
    let mut pc: u32 = 0x8240FCD0;
    'dispatch: loop {
        match pc {
            0x8240FCD0 => {
    //   block [0x8240FCD0..0x8240FD74)
	// 8240FCD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240FCD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240FCD8: DBE1FFF0  stfd f31, -0x10(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.f[31].u64 ) };
	// 8240FCDC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240FCE0: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 8240FCE4: 4BFFE7ED  bl 0x8240e4d0
	ctx.lr = 0x8240FCE8;
	sub_8240E4D0(ctx, base);
	// 8240FCE8: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8240FCEC: C00B1850  lfs f0, 0x1850(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(6224 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240FCF0: FF010000  fcmpu cr6, f1, f0
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 8240FCF4: 41990080  bgt cr6, 0x8240fd74
	if ctx.cr[6].gt {
	pc = 0x8240FD74; continue 'dispatch;
	}
	// 8240FCF8: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8240FCFC: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 8240FD00: C00B294C  lfs f0, 0x294c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(10572 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240FD04: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8240FD08: EDA10032  fmuls f13, f1, f0
	ctx.f[13].f64 = (((ctx.f[1].f64 * ctx.f[0].f64) as f32) as f64);
	// 8240FD0C: C00B2038  lfs f0, 0x2038(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8248 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240FD10: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240FD14: 396BEAB8  addi r11, r11, -0x1548
	ctx.r[11].s64 = ctx.r[11].s64 + -5448;
	// 8240FD18: FDA0681E  fctiwz f13, f13
	ctx.f[13].s64 = if ctx.f[13].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[13].f64.trunc() as i32 as i64 };
	// 8240FD1C: 7DA057AE  stfiwx f13, 0, r10
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32, tmp.u32) };
	// 8240FD20: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8240FD24: 7D4907B4  extsw r9, r10
	ctx.r[9].s64 = ctx.r[10].s32 as i64;
	// 8240FD28: 1D4A0018  mulli r10, r10, 0x18
	ctx.r[10].s32 = ((ctx.r[10].s32 as i64 * 24 as i64) as i32);
	ctx.r[10].s64 = ctx.r[10].s32 as i64;
	// 8240FD2C: F9210050  std r9, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u64 ) };
	// 8240FD30: C9A10050  lfd f13, 0x50(r1)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 8240FD34: FDA06E9C  fcfid f13, f13
	ctx.f[13].f64 = (ctx.f[13].s64 as f64);
	// 8240FD38: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 8240FD3C: C18B0000  lfs f12, 0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 8240FD40: C16B0004  lfs f11, 4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 8240FD44: FDA06818  frsp f13, f13
	ctx.f[13].f64 = (ctx.f[13].f64 as f32) as f64;
	// 8240FD48: C14B0008  lfs f10, 8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 8240FD4C: C12B000C  lfs f9, 0xc(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) };
	ctx.f[9].f64 = (tmp.f32 as f64);
	// 8240FD50: C10B0010  lfs f8, 0x10(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) };
	ctx.f[8].f64 = (tmp.f32 as f64);
	// 8240FD54: C0EB0014  lfs f7, 0x14(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) };
	ctx.f[7].f64 = (tmp.f32 as f64);
	// 8240FD58: EC0D083C  fnmsubs f0, f13, f0, f1
	ctx.f[0].f64 = -(((ctx.f[13].f64 * ctx.f[0].f64 - ctx.f[1].f64) as f32) as f64);
	// 8240FD5C: EDAC583A  fmadds f13, f12, f0, f11
	ctx.f[13].f64 = (((ctx.f[12].f64 * ctx.f[0].f64 + ctx.f[11].f64) as f32) as f64);
	// 8240FD60: EDAD503A  fmadds f13, f13, f0, f10
	ctx.f[13].f64 = (((ctx.f[13].f64 * ctx.f[0].f64 + ctx.f[10].f64) as f32) as f64);
	// 8240FD64: EDAD483A  fmadds f13, f13, f0, f9
	ctx.f[13].f64 = (((ctx.f[13].f64 * ctx.f[0].f64 + ctx.f[9].f64) as f32) as f64);
	// 8240FD68: EDAD403A  fmadds f13, f13, f0, f8
	ctx.f[13].f64 = (((ctx.f[13].f64 * ctx.f[0].f64 + ctx.f[8].f64) as f32) as f64);
	// 8240FD6C: EC0D383A  fmadds f0, f13, f0, f7
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[0].f64 + ctx.f[7].f64) as f32) as f64);
	// 8240FD70: 48000094  b 0x8240fe04
	pc = 0x8240FE04; continue 'dispatch;
            }
            0x8240FD74 => {
    //   block [0x8240FD74..0x8240FE04)
	// 8240FD74: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8240FD78: EC000824  fdivs f0, f0, f1
	ctx.f[0].f64 = ((ctx.f[0].f64 / ctx.f[1].f64) as f32) as f64;
	// 8240FD7C: 39210050  addi r9, r1, 0x50
	ctx.r[9].s64 = ctx.r[1].s64 + 80;
	// 8240FD80: C1AB2038  lfs f13, 0x2038(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8248 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8240FD84: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240FD88: 394BEAB8  addi r10, r11, -0x1548
	ctx.r[10].s64 = ctx.r[11].s64 + -5448;
	// 8240FD8C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8240FD90: C18B294C  lfs f12, 0x294c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(10572 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 8240FD94: ED800332  fmuls f12, f0, f12
	ctx.f[12].f64 = (((ctx.f[0].f64 * ctx.f[12].f64) as f32) as f64);
	// 8240FD98: FD80601E  fctiwz f12, f12
	ctx.f[12].s64 = if ctx.f[12].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[12].f64.trunc() as i32 as i64 };
	// 8240FD9C: 7D804FAE  stfiwx f12, 0, r9
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32, tmp.u32) };
	// 8240FDA0: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8240FDA4: 7D6907B4  extsw r9, r11
	ctx.r[9].s64 = ctx.r[11].s32 as i64;
	// 8240FDA8: 1D6B0018  mulli r11, r11, 0x18
	ctx.r[11].s32 = ((ctx.r[11].s32 as i64 * 24 as i64) as i32);
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 8240FDAC: F9210050  std r9, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u64 ) };
	// 8240FDB0: C9810050  lfd f12, 0x50(r1)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 8240FDB4: FD80669C  fcfid f12, f12
	ctx.f[12].f64 = (ctx.f[12].s64 as f64);
	// 8240FDB8: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8240FDBC: C16B0000  lfs f11, 0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 8240FDC0: C14B0004  lfs f10, 4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 8240FDC4: FD806018  frsp f12, f12
	ctx.f[12].f64 = (ctx.f[12].f64 as f32) as f64;
	// 8240FDC8: C12B0008  lfs f9, 8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	ctx.f[9].f64 = (tmp.f32 as f64);
	// 8240FDCC: C10B000C  lfs f8, 0xc(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) };
	ctx.f[8].f64 = (tmp.f32 as f64);
	// 8240FDD0: C0EB0010  lfs f7, 0x10(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) };
	ctx.f[7].f64 = (tmp.f32 as f64);
	// 8240FDD4: C0CB0014  lfs f6, 0x14(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) };
	ctx.f[6].f64 = (tmp.f32 as f64);
	// 8240FDD8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240FDDC: EC0C037C  fnmsubs f0, f12, f13, f0
	ctx.f[0].f64 = -(((ctx.f[12].f64 * ctx.f[13].f64 - ctx.f[0].f64) as f32) as f64);
	// 8240FDE0: EDA052FA  fmadds f13, f0, f11, f10
	ctx.f[13].f64 = (((ctx.f[0].f64 * ctx.f[11].f64 + ctx.f[10].f64) as f32) as f64);
	// 8240FDE4: EDAD483A  fmadds f13, f13, f0, f9
	ctx.f[13].f64 = (((ctx.f[13].f64 * ctx.f[0].f64 + ctx.f[9].f64) as f32) as f64);
	// 8240FDE8: EDAD403A  fmadds f13, f13, f0, f8
	ctx.f[13].f64 = (((ctx.f[13].f64 * ctx.f[0].f64 + ctx.f[8].f64) as f32) as f64);
	// 8240FDEC: EDAD383A  fmadds f13, f13, f0, f7
	ctx.f[13].f64 = (((ctx.f[13].f64 * ctx.f[0].f64 + ctx.f[7].f64) as f32) as f64);
	// 8240FDF0: EDAD303A  fmadds f13, f13, f0, f6
	ctx.f[13].f64 = (((ctx.f[13].f64 * ctx.f[0].f64 + ctx.f[6].f64) as f32) as f64);
	// 8240FDF4: C00A007C  lfs f0, 0x7c(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(124 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240FDF8: EDAD002A  fadds f13, f13, f0
	ctx.f[13].f64 = ((ctx.f[13].f64 + ctx.f[0].f64) as f32) as f64;
	// 8240FDFC: C00BEB38  lfs f0, -0x14c8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-5320 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240FE00: EC006828  fsubs f0, f0, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 - ctx.f[13].f64) as f32) as f64);
	pc = 0x8240FE04; continue 'dispatch;
            }
            0x8240FE04 => {
    //   block [0x8240FE04..0x8240FE20)
	// 8240FE04: FDA00050  fneg f13, f0
	ctx.f[13].u64 = ctx.f[0].u64 ^ 0x8000_0000_0000_0000u64;
	// 8240FE08: FC3F682E  fsel f1, f31, f0, f13
	ctx.f[1].f64 = if ctx.f[31].f64 >= 0.0 { ctx.f[0].f64 } else { ctx.f[13].f64 };
	// 8240FE0C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8240FE10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240FE14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240FE18: CBE1FFF0  lfd f31, -0x10(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8240FE1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240FE20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8240FE20 size=84
    let mut pc: u32 = 0x8240FE20;
    'dispatch: loop {
        match pc {
            0x8240FE20 => {
    //   block [0x8240FE20..0x8240FE74)
	// 8240FE20: 3961FFF0  addi r11, r1, -0x10
	ctx.r[11].s64 = ctx.r[1].s64 + -16;
	// 8240FE24: FC00081E  fctiwz f0, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].s64 = if ctx.f[1].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[1].f64.trunc() as i32 as i64 };
	// 8240FE28: 39400168  li r10, 0x168
	ctx.r[10].s64 = 360;
	// 8240FE2C: 7C005FAE  stfiwx f0, 0, r11
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32, tmp.u32) };
	// 8240FE30: 8161FFF0  lwz r11, -0x10(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) } as u64;
	// 8240FE34: 7D6B53D6  divw r11, r11, r10
	ctx.r[11].s32 = ctx.r[11].s32 / ctx.r[10].s32;
	// 8240FE38: 1D6B0168  mulli r11, r11, 0x168
	ctx.r[11].s32 = ((ctx.r[11].s32 as i64 * 360 as i64) as i32);
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 8240FE3C: 7D6B07B4  extsw r11, r11
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 8240FE40: F961FFF0  std r11, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[11].u64 ) };
	// 8240FE44: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8240FE48: C801FFF0  lfd f0, -0x10(r1)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8240FE4C: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 8240FE50: FC000018  frsp f0, f0
	ctx.f[0].f64 = (ctx.f[0].f64 as f32) as f64;
	// 8240FE54: EC210028  fsubs f1, f1, f0
	ctx.f[1].f64 = (((ctx.f[1].f64 - ctx.f[0].f64) as f32) as f64);
	// 8240FE58: C00B1FF8  lfs f0, 0x1ff8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240FE5C: FF010000  fcmpu cr6, f1, f0
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 8240FE60: 4C980020  bgelr cr6
	if !ctx.cr[6].lt { return; }
	// 8240FE64: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8240FE68: C00B210C  lfs f0, 0x210c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8460 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240FE6C: EC21002A  fadds f1, f1, f0
	ctx.f[1].f64 = ((ctx.f[1].f64 + ctx.f[0].f64) as f32) as f64;
	// 8240FE70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240FE78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8240FE78 size=60
    let mut pc: u32 = 0x8240FE78;
    'dispatch: loop {
        match pc {
            0x8240FE78 => {
    //   block [0x8240FE78..0x8240FEA4)
	// 8240FE78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240FE7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240FE80: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240FE84: 4BFFFF9D  bl 0x8240fe20
	ctx.lr = 0x8240FE88;
	sub_8240FE20(ctx, base);
	// 8240FE88: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8240FE8C: C00B213C  lfs f0, 0x213c(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8508 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240FE90: FF010000  fcmpu cr6, f1, f0
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 8240FE94: 40990010  ble cr6, 0x8240fea4
	if !ctx.cr[6].gt {
	pc = 0x8240FEA4; continue 'dispatch;
	}
	// 8240FE98: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8240FE9C: C00B210C  lfs f0, 0x210c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8460 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240FEA0: EC210028  fsubs f1, f1, f0
	ctx.f[1].f64 = (((ctx.f[1].f64 - ctx.f[0].f64) as f32) as f64);
	pc = 0x8240FEA4; continue 'dispatch;
            }
            0x8240FEA4 => {
    //   block [0x8240FEA4..0x8240FEB4)
	// 8240FEA4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8240FEA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240FEAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240FEB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240FEB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8240FEB8 size=8
    let mut pc: u32 = 0x8240FEB8;
    'dispatch: loop {
        match pc {
            0x8240FEB8 => {
    //   block [0x8240FEB8..0x8240FEC0)
	// 8240FEB8: EC20082C  fsqrts f1, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ((ctx.f[1].f64).sqrt() as f32) as f64;
	// 8240FEBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240FEC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8240FEC0 size=104
    let mut pc: u32 = 0x8240FEC0;
    'dispatch: loop {
        match pc {
            0x8240FEC0 => {
    //   block [0x8240FEC0..0x8240FF10)
	// 8240FEC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240FEC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240FEC8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8240FECC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240FED0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8240FED4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8240FED8: 419A003C  beq cr6, 0x8240ff14
	if ctx.cr[6].eq {
	pc = 0x8240FF14; continue 'dispatch;
	}
	// 8240FEDC: C03F0000  lfs f1, 0(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8240FEE0: 48000711  bl 0x824105f0
	ctx.lr = 0x8240FEE4;
	sub_824105F0(ctx, base);
	// 8240FEE4: 2B030001  cmplwi cr6, r3, 1
	ctx.cr[6].compare_u32(ctx.r[3].u32, 1 as u32, &mut ctx.xer);
	// 8240FEE8: 419A0028  beq cr6, 0x8240ff10
	if ctx.cr[6].eq {
	pc = 0x8240FF10; continue 'dispatch;
	}
	// 8240FEEC: C03F0004  lfs f1, 4(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8240FEF0: 48000701  bl 0x824105f0
	ctx.lr = 0x8240FEF4;
	sub_824105F0(ctx, base);
	// 8240FEF4: 2B030001  cmplwi cr6, r3, 1
	ctx.cr[6].compare_u32(ctx.r[3].u32, 1 as u32, &mut ctx.xer);
	// 8240FEF8: 419A0018  beq cr6, 0x8240ff10
	if ctx.cr[6].eq {
	pc = 0x8240FF10; continue 'dispatch;
	}
	// 8240FEFC: C03F0008  lfs f1, 8(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8240FF00: 480006F1  bl 0x824105f0
	ctx.lr = 0x8240FF04;
	sub_824105F0(ctx, base);
	// 8240FF04: 2B030001  cmplwi cr6, r3, 1
	ctx.cr[6].compare_u32(ctx.r[3].u32, 1 as u32, &mut ctx.xer);
	// 8240FF08: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8240FF0C: 409A0008  bne cr6, 0x8240ff14
	if !ctx.cr[6].eq {
	pc = 0x8240FF14; continue 'dispatch;
	}
	pc = 0x8240FF10; continue 'dispatch;
            }
            0x8240FF10 => {
    //   block [0x8240FF10..0x8240FF14)
	// 8240FF10: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	pc = 0x8240FF14; continue 'dispatch;
            }
            0x8240FF14 => {
    //   block [0x8240FF14..0x8240FF28)
	// 8240FF14: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8240FF18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240FF1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240FF20: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8240FF24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240FF28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8240FF28 size=120
    let mut pc: u32 = 0x8240FF28;
    'dispatch: loop {
        match pc {
            0x8240FF28 => {
    //   block [0x8240FF28..0x8240FF88)
	// 8240FF28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240FF2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240FF30: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8240FF34: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240FF38: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8240FF3C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8240FF40: 419A004C  beq cr6, 0x8240ff8c
	if ctx.cr[6].eq {
	pc = 0x8240FF8C; continue 'dispatch;
	}
	// 8240FF44: C03F0000  lfs f1, 0(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8240FF48: 480006A9  bl 0x824105f0
	ctx.lr = 0x8240FF4C;
	sub_824105F0(ctx, base);
	// 8240FF4C: 2B030001  cmplwi cr6, r3, 1
	ctx.cr[6].compare_u32(ctx.r[3].u32, 1 as u32, &mut ctx.xer);
	// 8240FF50: 419A0038  beq cr6, 0x8240ff88
	if ctx.cr[6].eq {
	pc = 0x8240FF88; continue 'dispatch;
	}
	// 8240FF54: C03F0004  lfs f1, 4(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8240FF58: 48000699  bl 0x824105f0
	ctx.lr = 0x8240FF5C;
	sub_824105F0(ctx, base);
	// 8240FF5C: 2B030001  cmplwi cr6, r3, 1
	ctx.cr[6].compare_u32(ctx.r[3].u32, 1 as u32, &mut ctx.xer);
	// 8240FF60: 419A0028  beq cr6, 0x8240ff88
	if ctx.cr[6].eq {
	pc = 0x8240FF88; continue 'dispatch;
	}
	// 8240FF64: C03F0008  lfs f1, 8(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8240FF68: 48000689  bl 0x824105f0
	ctx.lr = 0x8240FF6C;
	sub_824105F0(ctx, base);
	// 8240FF6C: 2B030001  cmplwi cr6, r3, 1
	ctx.cr[6].compare_u32(ctx.r[3].u32, 1 as u32, &mut ctx.xer);
	// 8240FF70: 419A0018  beq cr6, 0x8240ff88
	if ctx.cr[6].eq {
	pc = 0x8240FF88; continue 'dispatch;
	}
	// 8240FF74: C03F000C  lfs f1, 0xc(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8240FF78: 48000679  bl 0x824105f0
	ctx.lr = 0x8240FF7C;
	sub_824105F0(ctx, base);
	// 8240FF7C: 2B030001  cmplwi cr6, r3, 1
	ctx.cr[6].compare_u32(ctx.r[3].u32, 1 as u32, &mut ctx.xer);
	// 8240FF80: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8240FF84: 409A0008  bne cr6, 0x8240ff8c
	if !ctx.cr[6].eq {
	pc = 0x8240FF8C; continue 'dispatch;
	}
	pc = 0x8240FF88; continue 'dispatch;
            }
            0x8240FF88 => {
    //   block [0x8240FF88..0x8240FF8C)
	// 8240FF88: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	pc = 0x8240FF8C; continue 'dispatch;
            }
            0x8240FF8C => {
    //   block [0x8240FF8C..0x8240FFA0)
	// 8240FF8C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8240FF90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240FF94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240FF98: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8240FF9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240FFA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8240FFA0 size=8
    let mut pc: u32 = 0x8240FFA0;
    'dispatch: loop {
        match pc {
            0x8240FFA0 => {
    //   block [0x8240FFA0..0x8240FFA8)
	// 8240FFA0: 48000650  b 0x824105f0
	sub_824105F0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240FFC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8240FFC0 size=36
    let mut pc: u32 = 0x8240FFC0;
    'dispatch: loop {
        match pc {
            0x8240FFC0 => {
    //   block [0x8240FFC0..0x8240FFE4)
	// 8240FFC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240FFC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240FFC8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240FFCC: 4812318D  bl 0x82533158
	ctx.lr = 0x8240FFD0;
	sub_82533158(ctx, base);
	// 8240FFD0: FC200818  frsp f1, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = (ctx.f[1].f64 as f32) as f64;
	// 8240FFD4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8240FFD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240FFDC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240FFE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240FFE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8240FFE8 size=36
    let mut pc: u32 = 0x8240FFE8;
    'dispatch: loop {
        match pc {
            0x8240FFE8 => {
    //   block [0x8240FFE8..0x8241000C)
	// 8240FFE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240FFEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240FFF0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240FFF4: 48122FE5  bl 0x82532fd8
	ctx.lr = 0x8240FFF8;
	sub_82532FD8(ctx, base);
	// 8240FFF8: FC200818  frsp f1, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = (ctx.f[1].f64 as f32) as f64;
	// 8240FFFC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82410000: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82410004: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82410008: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82410010(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82410010 size=36
    let mut pc: u32 = 0x82410010;
    'dispatch: loop {
        match pc {
            0x82410010 => {
    //   block [0x82410010..0x82410034)
	// 82410010: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82410014: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82410018: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241001C: 48123755  bl 0x82533770
	ctx.lr = 0x82410020;
	sub_82533770(ctx, base);
	// 82410020: FC200818  frsp f1, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = (ctx.f[1].f64 as f32) as f64;
	// 82410024: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82410028: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8241002C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82410030: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82410038(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82410038 size=12
    let mut pc: u32 = 0x82410038;
    'dispatch: loop {
        match pc {
            0x82410038 => {
    //   block [0x82410038..0x82410044)
	// 82410038: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8241003C: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82410040: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82410048(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82410048 size=44
    let mut pc: u32 = 0x82410048;
    'dispatch: loop {
        match pc {
            0x82410048 => {
    //   block [0x82410048..0x82410074)
	// 82410048: 81440000  lwz r10, 0(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241004C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82410050: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82410054: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82410058: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8241005C: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82410060: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82410064: 91630014  stw r11, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82410068: 91630018  stw r11, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 8241006C: 9163001C  stw r11, 0x1c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82410070: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82410078(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82410078 size=20
    let mut pc: u32 = 0x82410078;
    'dispatch: loop {
        match pc {
            0x82410078 => {
    //   block [0x82410078..0x8241008C)
	// 82410078: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241007C: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82410080: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82410084: 69630001  xori r3, r11, 1
	ctx.r[3].u64 = ctx.r[11].u64 ^ 1;
	// 82410088: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82410090(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82410090 size=92
    let mut pc: u32 = 0x82410090;
    'dispatch: loop {
        match pc {
            0x82410090 => {
    //   block [0x82410090..0x824100B0)
	// 82410090: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82410094: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82410098: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8241009C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824100A0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824100A4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824100A8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824100AC: 419A000C  beq cr6, 0x824100b8
	if ctx.cr[6].eq {
	pc = 0x824100B8; continue 'dispatch;
	}
	pc = 0x824100B0; continue 'dispatch;
            }
            0x824100B0 => {
    //   block [0x824100B0..0x824100B8)
	// 824100B0: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 824100B4: 48000024  b 0x824100d8
	pc = 0x824100D8; continue 'dispatch;
            }
            0x824100B8 => {
    //   block [0x824100B8..0x824100D8)
	// 824100B8: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 824100BC: 419AFFF4  beq cr6, 0x824100b0
	if ctx.cr[6].eq {
	pc = 0x824100B0; continue 'dispatch;
	}
	// 824100C0: 38A00018  li r5, 0x18
	ctx.r[5].s64 = 24;
	// 824100C4: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 824100C8: 4BFB0B39  bl 0x823c0c00
	ctx.lr = 0x824100CC;
	sub_823C0C00(ctx, base);
	// 824100CC: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 824100D0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824100D4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	pc = 0x824100D8; continue 'dispatch;
            }
            0x824100D8 => {
    //   block [0x824100D8..0x824100EC)
	// 824100D8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824100DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824100E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824100E4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824100E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824100F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824100F0 size=32
    let mut pc: u32 = 0x824100F0;
    'dispatch: loop {
        match pc {
            0x824100F0 => {
    //   block [0x824100F0..0x82410110)
	// 824100F0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 824100F4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824100F8: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824100FC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82410100: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 82410104: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82410108: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8241010C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82410110(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82410110 size=200
    let mut pc: u32 = 0x82410110;
    'dispatch: loop {
        match pc {
            0x82410110 => {
    //   block [0x82410110..0x824101D8)
	// 82410110: D0210014  stfs f1, 0x14(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 82410114: D021FFE8  stfs f1, -0x18(r1)
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), tmp.u32 ) };
	// 82410118: D021FFEC  stfs f1, -0x14(r1)
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-20 as u32), tmp.u32 ) };
	// 8241011C: 81610014  lwz r11, 0x14(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(20 as u32) ) } as u64;
	// 82410120: 556B007E  clrlwi r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x7FFFFFFFu64;
	// 82410124: 9161FFF0  stw r11, -0x10(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[11].u32 ) };
	// 82410128: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8241012C: C1A1FFF0  lfs f13, -0x10(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82410130: C00B1850  lfs f0, 0x1850(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(6224 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82410134: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 82410138: D001FFD4  stfs f0, -0x2c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-44 as u32), tmp.u32 ) };
	// 8241013C: 396BDE10  addi r11, r11, -0x21f0
	ctx.r[11].s64 = ctx.r[11].s64 + -8688;
	// 82410140: D001FFDC  stfs f0, -0x24(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-36 as u32), tmp.u32 ) };
	// 82410144: EC0D0072  fmuls f0, f13, f1
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[1].f64) as f32) as f64);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824101D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x824101D8 size=120
    let mut pc: u32 = 0x824101D8;
    'dispatch: loop {
        match pc {
            0x824101D8 => {
    //   block [0x824101D8..0x82410250)
	// 824101D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824101DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824101E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824101E4: D0210050  stfs f1, 0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 824101E8: 3D600124  lis r11, 0x124
	ctx.r[11].s64 = 19136512;
	// 824101EC: 616A3F6D  ori r10, r11, 0x3f6d
	ctx.r[10].u64 = ctx.r[11].u64 | 16237;
	// 824101F0: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 824101F4: C00B2204  lfs f0, 0x2204(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8708 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824101F8: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 824101FC: 39210050  addi r9, r1, 0x50
	ctx.r[9].s64 = ctx.r[1].s64 + 80;
	// 82410200: 514BF07E  rlwimi r11, r10, 0x1e, 1, 0x1f
	ctx.r[11].u64 = (((ctx.r[10].u32).rotate_left(30) as u64) & 0x000000007FFFFFFF) | (ctx.r[11].u64 & 0xFFFFFFFF80000000);
	// 82410204: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82410208: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8241020C: C1A10050  lfs f13, 0x50(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82410210: ED8D082A  fadds f12, f13, f1
	ctx.f[12].f64 = ((ctx.f[13].f64 + ctx.f[1].f64) as f32) as f64;
	// 82410214: C1AB2200  lfs f13, 0x2200(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8704 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82410218: EDAC0372  fmuls f13, f12, f13
	ctx.f[13].f64 = (((ctx.f[12].f64 * ctx.f[13].f64) as f32) as f64);
	// 8241021C: FDA0681E  fctiwz f13, f13
	ctx.f[13].s64 = if ctx.f[13].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[13].f64.trunc() as i32 as i64 };
	// 82410220: 7DA04FAE  stfiwx f13, 0, r9
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32, tmp.u32) };
	// 82410224: E9610052  lwa r11, 0x50(r1)
	ctx.r[11].s64 = (unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as i32) as i64;
	// 82410228: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 8241022C: C9A10050  lfd f13, 0x50(r1)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82410230: FDA06E9C  fcfid f13, f13
	ctx.f[13].f64 = (ctx.f[13].s64 as f64);
	// 82410234: FDA06818  frsp f13, f13
	ctx.f[13].f64 = (ctx.f[13].f64 as f32) as f64;
	// 82410238: EC2D083C  fnmsubs f1, f13, f0, f1
	ctx.f[1].f64 = -(((ctx.f[13].f64 * ctx.f[0].f64 - ctx.f[1].f64) as f32) as f64);
	// 8241023C: 4BD0D28D  bl 0x8211d4c8
	ctx.lr = 0x82410240;
	sub_8211D4C8(ctx, base);
	// 82410240: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82410244: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82410248: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8241024C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82410250(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82410250 size=120
    let mut pc: u32 = 0x82410250;
    'dispatch: loop {
        match pc {
            0x82410250 => {
    //   block [0x82410250..0x824102C8)
	// 82410250: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82410254: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82410258: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241025C: D0210050  stfs f1, 0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 82410260: 3D600124  lis r11, 0x124
	ctx.r[11].s64 = 19136512;
	// 82410264: 616A3F6D  ori r10, r11, 0x3f6d
	ctx.r[10].u64 = ctx.r[11].u64 | 16237;
	// 82410268: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8241026C: C00B2204  lfs f0, 0x2204(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8708 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82410270: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82410274: 39210050  addi r9, r1, 0x50
	ctx.r[9].s64 = ctx.r[1].s64 + 80;
	// 82410278: 514BF07E  rlwimi r11, r10, 0x1e, 1, 0x1f
	ctx.r[11].u64 = (((ctx.r[10].u32).rotate_left(30) as u64) & 0x000000007FFFFFFF) | (ctx.r[11].u64 & 0xFFFFFFFF80000000);
	// 8241027C: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82410280: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 82410284: C1A10050  lfs f13, 0x50(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82410288: ED8D082A  fadds f12, f13, f1
	ctx.f[12].f64 = ((ctx.f[13].f64 + ctx.f[1].f64) as f32) as f64;
	// 8241028C: C1AB2200  lfs f13, 0x2200(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8704 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82410290: EDAC0372  fmuls f13, f12, f13
	ctx.f[13].f64 = (((ctx.f[12].f64 * ctx.f[13].f64) as f32) as f64);
	// 82410294: FDA0681E  fctiwz f13, f13
	ctx.f[13].s64 = if ctx.f[13].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[13].f64.trunc() as i32 as i64 };
	// 82410298: 7DA04FAE  stfiwx f13, 0, r9
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32, tmp.u32) };
	// 8241029C: E9610052  lwa r11, 0x50(r1)
	ctx.r[11].s64 = (unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as i32) as i64;
	// 824102A0: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 824102A4: C9A10050  lfd f13, 0x50(r1)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 824102A8: FDA06E9C  fcfid f13, f13
	ctx.f[13].f64 = (ctx.f[13].s64 as f64);
	// 824102AC: FDA06818  frsp f13, f13
	ctx.f[13].f64 = (ctx.f[13].f64 as f32) as f64;
	// 824102B0: EC2D083C  fnmsubs f1, f13, f0, f1
	ctx.f[1].f64 = -(((ctx.f[13].f64 * ctx.f[0].f64 - ctx.f[1].f64) as f32) as f64);
	// 824102B4: 4BD43005  bl 0x821532b8
	ctx.lr = 0x824102B8;
	sub_821532B8(ctx, base);
	// 824102B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824102BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824102C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824102C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824102C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824102C8 size=4
    let mut pc: u32 = 0x824102C8;
    'dispatch: loop {
        match pc {
            0x824102C8 => {
    //   block [0x824102C8..0x824102CC)
	// 824102C8: 4BFFFE48  b 0x82410110
	sub_82410110(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824102D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x824102D0 size=52
    let mut pc: u32 = 0x824102D0;
    'dispatch: loop {
        match pc {
            0x824102D0 => {
    //   block [0x824102D0..0x82410304)
	// 824102D0: C1A50000  lfs f13, 0(r5)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 824102D4: C0040000  lfs f0, 0(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824102D8: EC006828  fsubs f0, f0, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 - ctx.f[13].f64) as f32) as f64);
	// 824102DC: D0030000  stfs f0, 0(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 824102E0: C1A50004  lfs f13, 4(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 824102E4: C0040004  lfs f0, 4(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824102E8: EC006828  fsubs f0, f0, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 - ctx.f[13].f64) as f32) as f64);
	// 824102EC: D0030004  stfs f0, 4(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 824102F0: C0040008  lfs f0, 8(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824102F4: C1A50008  lfs f13, 8(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(8 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 824102F8: EC006828  fsubs f0, f0, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 - ctx.f[13].f64) as f32) as f64);
	// 824102FC: D0030008  stfs f0, 8(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82410300: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82410308(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82410308 size=112
    let mut pc: u32 = 0x82410308;
    'dispatch: loop {
        match pc {
            0x82410308 => {
    //   block [0x82410308..0x82410378)
	// 82410308: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241030C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82410310: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82410314: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82410318: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241031C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82410320: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82410324: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82410328: 4BFFF951  bl 0x8240fc78
	ctx.lr = 0x8241032C;
	sub_8240FC78(ctx, base);
	// 8241032C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82410330: C1BE0000  lfs f13, 0(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82410334: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82410338: C00B1850  lfs f0, 0x1850(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(6224 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8241033C: EC000824  fdivs f0, f0, f1
	ctx.f[0].f64 = ((ctx.f[0].f64 / ctx.f[1].f64) as f32) as f64;
	// 82410340: EDAD0032  fmuls f13, f13, f0
	ctx.f[13].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 82410344: D1BF0000  stfs f13, 0(r31)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82410348: C1BE0004  lfs f13, 4(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8241034C: EDAD0032  fmuls f13, f13, f0
	ctx.f[13].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 82410350: D1BF0004  stfs f13, 4(r31)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82410354: C1BE0008  lfs f13, 8(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82410358: EC0D0032  fmuls f0, f13, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 8241035C: D01F0008  stfs f0, 8(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82410360: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82410364: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82410368: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8241036C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82410370: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82410374: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82410378(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82410378 size=88
    let mut pc: u32 = 0x82410378;
    'dispatch: loop {
        match pc {
            0x82410378 => {
    //   block [0x82410378..0x824103D0)
	// 82410378: C1A40008  lfs f13, 8(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8241037C: C0050004  lfs f0, 4(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82410380: EC000372  fmuls f0, f0, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[13].f64) as f32) as f64);
	// 82410384: C1840004  lfs f12, 4(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82410388: C1A50008  lfs f13, 8(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(8 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8241038C: EC0D0338  fmsubs f0, f13, f12, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[12].f64 - ctx.f[0].f64) as f32) as f64);
	// 82410390: D0030000  stfs f0, 0(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82410394: C1A40000  lfs f13, 0(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82410398: C0050008  lfs f0, 8(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8241039C: EC000372  fmuls f0, f0, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[13].f64) as f32) as f64);
	// 824103A0: C1840008  lfs f12, 8(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 824103A4: C1A50000  lfs f13, 0(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 824103A8: EC0D0338  fmsubs f0, f13, f12, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[12].f64 - ctx.f[0].f64) as f32) as f64);
	// 824103AC: D0030004  stfs f0, 4(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 824103B0: C1A40004  lfs f13, 4(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 824103B4: C0050000  lfs f0, 0(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824103B8: EC000372  fmuls f0, f0, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[13].f64) as f32) as f64);
	// 824103BC: C1840000  lfs f12, 0(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 824103C0: C1A50004  lfs f13, 4(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 824103C4: EC0D0338  fmsubs f0, f13, f12, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[12].f64 - ctx.f[0].f64) as f32) as f64);
	// 824103C8: D0030008  stfs f0, 8(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 824103CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824103D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x824103D0 size=40
    let mut pc: u32 = 0x824103D0;
    'dispatch: loop {
        match pc {
            0x824103D0 => {
    //   block [0x824103D0..0x824103F8)
	// 824103D0: C1A40004  lfs f13, 4(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 824103D4: C0030004  lfs f0, 4(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824103D8: EC000372  fmuls f0, f0, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[13].f64) as f32) as f64);
	// 824103DC: C1840008  lfs f12, 8(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 824103E0: C1A30008  lfs f13, 8(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 824103E4: C1630000  lfs f11, 0(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 824103E8: C1440000  lfs f10, 0(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 824103EC: EC0D033A  fmadds f0, f13, f12, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[12].f64 + ctx.f[0].f64) as f32) as f64);
	// 824103F0: EC2B02BA  fmadds f1, f11, f10, f0
	ctx.f[1].f64 = (((ctx.f[11].f64 * ctx.f[10].f64 + ctx.f[0].f64) as f32) as f64);
	// 824103F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824103F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x824103F8 size=28
    let mut pc: u32 = 0x824103F8;
    'dispatch: loop {
        match pc {
            0x824103F8 => {
    //   block [0x824103F8..0x82410414)
	// 824103F8: C1A30004  lfs f13, 4(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 824103FC: EDAD0372  fmuls f13, f13, f13
	ctx.f[13].f64 = (((ctx.f[13].f64 * ctx.f[13].f64) as f32) as f64);
	// 82410400: C0030000  lfs f0, 0(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82410404: C1830008  lfs f12, 8(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82410408: EC00683A  fmadds f0, f0, f0, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64);
	// 8241040C: EC2C033A  fmadds f1, f12, f12, f0
	ctx.f[1].f64 = (((ctx.f[12].f64 * ctx.f[12].f64 + ctx.f[0].f64) as f32) as f64);
	// 82410410: 4BFFFAA8  b 0x8240feb8
	sub_8240FEB8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82410418(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82410418 size=184
    let mut pc: u32 = 0x82410418;
    'dispatch: loop {
        match pc {
            0x82410418 => {
    //   block [0x82410418..0x824104D0)
	// 82410418: 81640004  lwz r11, 4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 8241041C: C1A50010  lfs f13, 0x10(r5)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(16 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82410420: C1450020  lfs f10, 0x20(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(32 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 82410424: C1250000  lfs f9, 0(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) };
	ctx.f[9].f64 = (tmp.f32 as f64);
	// 82410428: C1050030  lfs f8, 0x30(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(48 as u32) ) };
	ctx.f[8].f64 = (tmp.f32 as f64);
	// 8241042C: 9161FFF4  stw r11, -0xc(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-12 as u32), ctx.r[11].u32 ) };
	// 82410430: C001FFF4  lfs f0, -0xc(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82410434: ED6D0032  fmuls f11, f13, f0
	ctx.f[11].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 82410438: 81640008  lwz r11, 8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 8241043C: 9161FFF8  stw r11, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[11].u32 ) };
	// 82410440: C1A1FFF8  lfs f13, -8(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82410444: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82410448: ED6A5B7A  fmadds f11, f10, f13, f11
	ctx.f[11].f64 = (((ctx.f[10].f64 * ctx.f[13].f64 + ctx.f[11].f64) as f32) as f64);
	// 8241044C: 9161FFF0  stw r11, -0x10(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[11].u32 ) };
	// 82410450: C181FFF0  lfs f12, -0x10(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82410454: ED695B3A  fmadds f11, f9, f12, f11
	ctx.f[11].f64 = (((ctx.f[9].f64 * ctx.f[12].f64 + ctx.f[11].f64) as f32) as f64);
	// 82410458: ED6B402A  fadds f11, f11, f8
	ctx.f[11].f64 = ((ctx.f[11].f64 + ctx.f[8].f64) as f32) as f64;
	// 8241045C: D1630000  stfs f11, 0(r3)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82410460: C1650014  lfs f11, 0x14(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(20 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82410464: ED6B0032  fmuls f11, f11, f0
	ctx.f[11].f64 = (((ctx.f[11].f64 * ctx.f[0].f64) as f32) as f64);
	// 82410468: C1450024  lfs f10, 0x24(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(36 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 8241046C: C1250004  lfs f9, 4(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) };
	ctx.f[9].f64 = (tmp.f32 as f64);
	// 82410470: C1050034  lfs f8, 0x34(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(52 as u32) ) };
	ctx.f[8].f64 = (tmp.f32 as f64);
	// 82410474: ED6A5B7A  fmadds f11, f10, f13, f11
	ctx.f[11].f64 = (((ctx.f[10].f64 * ctx.f[13].f64 + ctx.f[11].f64) as f32) as f64);
	// 82410478: ED695B3A  fmadds f11, f9, f12, f11
	ctx.f[11].f64 = (((ctx.f[9].f64 * ctx.f[12].f64 + ctx.f[11].f64) as f32) as f64);
	// 8241047C: ED6B402A  fadds f11, f11, f8
	ctx.f[11].f64 = ((ctx.f[11].f64 + ctx.f[8].f64) as f32) as f64;
	// 82410480: D1630004  stfs f11, 4(r3)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82410484: C1650018  lfs f11, 0x18(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(24 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82410488: ED6B0032  fmuls f11, f11, f0
	ctx.f[11].f64 = (((ctx.f[11].f64 * ctx.f[0].f64) as f32) as f64);
	// 8241048C: C1450028  lfs f10, 0x28(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(40 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 82410490: C1250008  lfs f9, 8(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(8 as u32) ) };
	ctx.f[9].f64 = (tmp.f32 as f64);
	// 82410494: C1050038  lfs f8, 0x38(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(56 as u32) ) };
	ctx.f[8].f64 = (tmp.f32 as f64);
	// 82410498: ED6A5B7A  fmadds f11, f10, f13, f11
	ctx.f[11].f64 = (((ctx.f[10].f64 * ctx.f[13].f64 + ctx.f[11].f64) as f32) as f64);
	// 8241049C: ED695B3A  fmadds f11, f9, f12, f11
	ctx.f[11].f64 = (((ctx.f[9].f64 * ctx.f[12].f64 + ctx.f[11].f64) as f32) as f64);
	// 824104A0: ED6B402A  fadds f11, f11, f8
	ctx.f[11].f64 = ((ctx.f[11].f64 + ctx.f[8].f64) as f32) as f64;
	// 824104A4: D1630008  stfs f11, 8(r3)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 824104A8: C165001C  lfs f11, 0x1c(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(28 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 824104AC: EC0B0032  fmuls f0, f11, f0
	ctx.f[0].f64 = (((ctx.f[11].f64 * ctx.f[0].f64) as f32) as f64);
	// 824104B0: C165002C  lfs f11, 0x2c(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(44 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 824104B4: C145000C  lfs f10, 0xc(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(12 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 824104B8: C125003C  lfs f9, 0x3c(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(60 as u32) ) };
	ctx.f[9].f64 = (tmp.f32 as f64);
	// 824104BC: EC0B037A  fmadds f0, f11, f13, f0
	ctx.f[0].f64 = (((ctx.f[11].f64 * ctx.f[13].f64 + ctx.f[0].f64) as f32) as f64);
	// 824104C0: EC0A033A  fmadds f0, f10, f12, f0
	ctx.f[0].f64 = (((ctx.f[10].f64 * ctx.f[12].f64 + ctx.f[0].f64) as f32) as f64);
	// 824104C4: EC00482A  fadds f0, f0, f9
	ctx.f[0].f64 = ((ctx.f[0].f64 + ctx.f[9].f64) as f32) as f64;
	// 824104C8: D003000C  stfs f0, 0xc(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 824104CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824104D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x824104D0 size=284
    let mut pc: u32 = 0x824104D0;
    'dispatch: loop {
        match pc {
            0x824104D0 => {
    //   block [0x824104D0..0x824105EC)
	// 824104D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824104D4: 48124BE9  bl 0x825350bc
	ctx.lr = 0x824104D8;
	sub_82535080(ctx, base);
	// 824104D8: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824104DC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824104E0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 824104E4: 7CC43378  mr r4, r6
	ctx.r[4].u64 = ctx.r[6].u64;
	// 824104E8: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 824104EC: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 824104F0: 4BFFF771  bl 0x8240fc60
	ctx.lr = 0x824104F4;
	sub_8240FC60(ctx, base);
	// 824104F4: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 824104F8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 824104FC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82410500: 4BFFF759  bl 0x8240fc58
	ctx.lr = 0x82410504;
	sub_8240FC58(ctx, base);
	// 82410504: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82410508: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8241050C: 4BFFF755  bl 0x8240fc60
	ctx.lr = 0x82410510;
	sub_8240FC60(ctx, base);
	// 82410510: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82410514: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 82410518: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8241051C: 4BFFF74D  bl 0x8240fc68
	ctx.lr = 0x82410520;
	sub_8240FC68(ctx, base);
	// 82410520: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82410524: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82410528: 4BFFF739  bl 0x8240fc60
	ctx.lr = 0x8241052C;
	sub_8240FC60(ctx, base);
	// 8241052C: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 82410530: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82410534: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82410538: 4BFFF731  bl 0x8240fc68
	ctx.lr = 0x8241053C;
	sub_8240FC68(ctx, base);
	// 8241053C: C0010060  lfs f0, 0x60(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82410540: D01F0000  stfs f0, 0(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82410544: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 82410548: C1A10064  lfs f13, 0x64(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8241054C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82410550: C0010068  lfs f0, 0x68(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82410554: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82410558: D1BF0010  stfs f13, 0x10(r31)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 8241055C: D01F0020  stfs f0, 0x20(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), tmp.u32 ) };
	// 82410560: C1A10070  lfs f13, 0x70(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82410564: C0010074  lfs f0, 0x74(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82410568: D1BF0004  stfs f13, 4(r31)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 8241056C: D01F0014  stfs f0, 0x14(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 82410570: C1A10078  lfs f13, 0x78(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(120 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82410574: C0010050  lfs f0, 0x50(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82410578: D1BF0024  stfs f13, 0x24(r31)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), tmp.u32 ) };
	// 8241057C: D01F0008  stfs f0, 8(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82410580: C00B1FF8  lfs f0, 0x1ff8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82410584: C1A10054  lfs f13, 0x54(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82410588: C1810058  lfs f12, 0x58(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 8241058C: D1BF0018  stfs f13, 0x18(r31)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 82410590: D19F0028  stfs f12, 0x28(r31)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), tmp.u32 ) };
	// 82410594: D01F000C  stfs f0, 0xc(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 82410598: D01F001C  stfs f0, 0x1c(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 8241059C: D01F002C  stfs f0, 0x2c(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), tmp.u32 ) };
	// 824105A0: 4BFFF6D1  bl 0x8240fc70
	ctx.lr = 0x824105A4;
	sub_8240FC70(ctx, base);
	// 824105A4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 824105A8: FC000850  fneg f0, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = ctx.f[1].u64 ^ 0x8000_0000_0000_0000u64;
	// 824105AC: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 824105B0: D01F0030  stfs f0, 0x30(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), tmp.u32 ) };
	// 824105B4: 4BFFF6BD  bl 0x8240fc70
	ctx.lr = 0x824105B8;
	sub_8240FC70(ctx, base);
	// 824105B8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 824105BC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 824105C0: FC000850  fneg f0, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = ctx.f[1].u64 ^ 0x8000_0000_0000_0000u64;
	// 824105C4: D01F0034  stfs f0, 0x34(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), tmp.u32 ) };
	// 824105C8: 4BFFF6A9  bl 0x8240fc70
	ctx.lr = 0x824105CC;
	sub_8240FC70(ctx, base);
	// 824105CC: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 824105D0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824105D4: C00B1850  lfs f0, 0x1850(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(6224 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824105D8: D01F003C  stfs f0, 0x3c(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), tmp.u32 ) };
	// 824105DC: FC000850  fneg f0, f1
	ctx.f[0].u64 = ctx.f[1].u64 ^ 0x8000_0000_0000_0000u64;
	// 824105E0: D01F0038  stfs f0, 0x38(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), tmp.u32 ) };
	// 824105E4: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 824105E8: 48124B24  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824105F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824105F0 size=44
    let mut pc: u32 = 0x824105F0;
    'dispatch: loop {
        match pc {
            0x824105F0 => {
    //   block [0x824105F0..0x8241061C)
	// 824105F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824105F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824105F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824105FC: 48123C5D  bl 0x82534258
	ctx.lr = 0x82410600;
	sub_82534258(ctx, base);
	// 82410600: 7C6B0034  cntlzw r11, r3
	ctx.r[11].u64 = if ctx.r[3].u32 == 0 { 32 } else { ctx.r[3].u32.leading_zeros() as u64 };
	// 82410604: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82410608: 69630001  xori r3, r11, 1
	ctx.r[3].u64 = ctx.r[11].u64 ^ 1;
	// 8241060C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82410610: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82410614: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82410618: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82410620(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82410620 size=120
    let mut pc: u32 = 0x82410620;
    'dispatch: loop {
        match pc {
            0x82410620 => {
    //   block [0x82410620..0x82410678)
	// 82410620: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82410624: 48124A95  bl 0x825350b8
	ctx.lr = 0x82410628;
	sub_82535080(ctx, base);
	// 82410628: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241062C: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82410630: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82410634: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82410638: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 8241063C: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82410640: 917C0000  stw r11, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82410644: 4800017D  bl 0x824107c0
	ctx.lr = 0x82410648;
	sub_824107C0(ctx, base);
	// 82410648: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241064C: 3880001C  li r4, 0x1c
	ctx.r[4].s64 = 28;
	// 82410650: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82410654: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82410658: 4E800421  bctrl
	ctx.lr = 0x8241065C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8241065C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82410660: 41820018  beq 0x82410678
	if ctx.cr[0].eq {
	pc = 0x82410678; continue 'dispatch;
	}
	// 82410664: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82410668: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8241066C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82410670: 48000679  bl 0x82410ce8
	ctx.lr = 0x82410674;
	sub_82410CE8(ctx, base);
	// 82410674: 48000008  b 0x8241067c
	pc = 0x8241067C; continue 'dispatch;
            }
            0x82410678 => {
    //   block [0x82410678..0x8241067C)
	// 82410678: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	pc = 0x8241067C; continue 'dispatch;
            }
            0x8241067C => {
    //   block [0x8241067C..0x82410688)
	// 8241067C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82410680: 907C0000  stw r3, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82410684: 409A0008  bne cr6, 0x8241068c
	if !ctx.cr[6].eq {
	pc = 0x8241068C; continue 'dispatch;
	}
	pc = 0x82410688; continue 'dispatch;
            }
            0x82410688 => {
    //   block [0x82410688..0x8241068C)
	// 82410688: 48000000  b 0x82410688
	pc = 0x82410688; continue 'dispatch;
            }
            0x8241068C => {
    //   block [0x8241068C..0x82410698)
	// 8241068C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82410690: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82410694: 48124A74  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82410698(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82410698 size=92
    let mut pc: u32 = 0x82410698;
    'dispatch: loop {
        match pc {
            0x82410698 => {
    //   block [0x82410698..0x824106D8)
	// 82410698: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241069C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824106A0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 824106A4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824106A8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824106AC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824106B0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 824106B4: 480006BD  bl 0x82410d70
	ctx.lr = 0x824106B8;
	sub_82410D70(ctx, base);
	// 824106B8: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824106BC: 4182001C  beq 0x824106d8
	if ctx.cr[0].eq {
	pc = 0x824106D8; continue 'dispatch;
	}
	// 824106C0: 48000101  bl 0x824107c0
	ctx.lr = 0x824106C4;
	sub_824107C0(ctx, base);
	// 824106C4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824106C8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 824106CC: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 824106D0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824106D4: 4E800421  bctrl
	ctx.lr = 0x824106D8;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x824106D8 => {
    //   block [0x824106D8..0x824106F4)
	// 824106D8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824106DC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824106E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824106E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824106E8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 824106EC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824106F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824106F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824106F8 size=8
    let mut pc: u32 = 0x824106F8;
    'dispatch: loop {
        match pc {
            0x824106F8 => {
    //   block [0x824106F8..0x82410700)
	// 824106F8: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824106FC: 480006C4  b 0x82410dc0
	sub_82410DC0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82410700(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82410700 size=8
    let mut pc: u32 = 0x82410700;
    'dispatch: loop {
        match pc {
            0x82410700 => {
    //   block [0x82410700..0x82410708)
	// 82410700: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82410704: 48000184  b 0x82410888
	sub_82410888(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82410708(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82410708 size=8
    let mut pc: u32 = 0x82410708;
    'dispatch: loop {
        match pc {
            0x82410708 => {
    //   block [0x82410708..0x82410710)
	// 82410708: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241070C: 480001D4  b 0x824108e0
	sub_824108E0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82410710(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82410710 size=8
    let mut pc: u32 = 0x82410710;
    'dispatch: loop {
        match pc {
            0x82410710 => {
    //   block [0x82410710..0x82410718)
	// 82410710: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82410714: 4800028C  b 0x824109a0
	sub_824109A0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82410718(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82410718 size=8
    let mut pc: u32 = 0x82410718;
    'dispatch: loop {
        match pc {
            0x82410718 => {
    //   block [0x82410718..0x82410720)
	// 82410718: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241071C: 4800034C  b 0x82410a68
	sub_82410A68(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82410720(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82410720 size=8
    let mut pc: u32 = 0x82410720;
    'dispatch: loop {
        match pc {
            0x82410720 => {
    //   block [0x82410720..0x82410728)
	// 82410720: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82410724: 480002A4  b 0x824109c8
	sub_824109C8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82410728(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82410728 size=8
    let mut pc: u32 = 0x82410728;
    'dispatch: loop {
        match pc {
            0x82410728 => {
    //   block [0x82410728..0x82410730)
	// 82410728: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241072C: 48000394  b 0x82410ac0
	sub_82410AC0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82410730(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82410730 size=8
    let mut pc: u32 = 0x82410730;
    'dispatch: loop {
        match pc {
            0x82410730 => {
    //   block [0x82410730..0x82410738)
	// 82410730: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82410734: 480002AC  b 0x824109e0
	sub_824109E0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82410738(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82410738 size=8
    let mut pc: u32 = 0x82410738;
    'dispatch: loop {
        match pc {
            0x82410738 => {
    //   block [0x82410738..0x82410740)
	// 82410738: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241073C: 480002BC  b 0x824109f8
	sub_824109F8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82410740(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82410740 size=8
    let mut pc: u32 = 0x82410740;
    'dispatch: loop {
        match pc {
            0x82410740 => {
    //   block [0x82410740..0x82410748)
	// 82410740: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82410744: 480002D4  b 0x82410a18
	sub_82410A18(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82410748(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82410748 size=8
    let mut pc: u32 = 0x82410748;
    'dispatch: loop {
        match pc {
            0x82410748 => {
    //   block [0x82410748..0x82410750)
	// 82410748: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241074C: 480002E4  b 0x82410a30
	sub_82410A30(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82410750(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82410750 size=8
    let mut pc: u32 = 0x82410750;
    'dispatch: loop {
        match pc {
            0x82410750 => {
    //   block [0x82410750..0x82410758)
	// 82410750: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82410754: 480002F4  b 0x82410a48
	sub_82410A48(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82410758(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82410758 size=68
    let mut pc: u32 = 0x82410758;
    'dispatch: loop {
        match pc {
            0x82410758 => {
    //   block [0x82410758..0x82410780)
	// 82410758: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241075C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82410760: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82410764: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82410768: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8241076C: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82410770: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82410774: 4182000C  beq 0x82410780
	if ctx.cr[0].eq {
	pc = 0x82410780; continue 'dispatch;
	}
	// 82410778: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8241077C: 4BFFFF1D  bl 0x82410698
	ctx.lr = 0x82410780;
	sub_82410698(ctx, base);
	pc = 0x82410780; continue 'dispatch;
            }
            0x82410780 => {
    //   block [0x82410780..0x8241079C)
	// 82410780: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82410784: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82410788: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8241078C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82410790: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82410794: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82410798: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824107A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824107A0 size=20
    let mut pc: u32 = 0x824107A0;
    'dispatch: loop {
        match pc {
            0x824107A0 => {
    //   block [0x824107A0..0x824107B0)
	// 824107A0: 3D608289  lis r11, -0x7d77
	ctx.r[11].s64 = -2104950784;
	// 824107A4: 814BF20C  lwz r10, -0xdf4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-3572 as u32) ) } as u64;
	// 824107A8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 824107AC: 419A0008  beq cr6, 0x824107b4
	if ctx.cr[6].eq {
		crate::recompiler::externs::call(ctx, base, 0x824107B4);
		return;
	}
	pc = 0x824107B0; continue 'dispatch;
            }
            0x824107B0 => {
    //   block [0x824107B0..0x824107B4)
	// 824107B0: 48000000  b 0x824107b0
	pc = 0x824107B0; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824107C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824107C0 size=20
    let mut pc: u32 = 0x824107C0;
    'dispatch: loop {
        match pc {
            0x824107C0 => {
    //   block [0x824107C0..0x824107D0)
	// 824107C0: 3D608289  lis r11, -0x7d77
	ctx.r[11].s64 = -2104950784;
	// 824107C4: 806BF20C  lwz r3, -0xdf4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-3572 as u32) ) } as u64;
	// 824107C8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 824107CC: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
	pc = 0x824107D0; continue 'dispatch;
            }
            0x824107D0 => {
    //   block [0x824107D0..0x824107D4)
	// 824107D0: 48000000  b 0x824107d0
	pc = 0x824107D0; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824107D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824107D8 size=80
    let mut pc: u32 = 0x824107D8;
    'dispatch: loop {
        match pc {
            0x824107D8 => {
    //   block [0x824107D8..0x82410814)
	// 824107D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824107DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824107E0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824107E4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824107E8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824107EC: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824107F0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 824107F4: 41820020  beq 0x82410814
	if ctx.cr[0].eq {
	pc = 0x82410814; continue 'dispatch;
	}
	// 824107F8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824107FC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82410800: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82410804: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82410808: 4E800421  bctrl
	ctx.lr = 0x8241080C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8241080C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82410810: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
            }
            0x82410814 => {
    //   block [0x82410814..0x82410828)
	// 82410814: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82410818: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8241081C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82410820: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82410824: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82410828(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82410828 size=92
    let mut pc: u32 = 0x82410828;
    'dispatch: loop {
        match pc {
            0x82410828 => {
    //   block [0x82410828..0x82410868)
	// 82410828: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241082C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82410830: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82410834: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82410838: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8241083C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82410840: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82410844: 48000AF5  bl 0x82411338
	ctx.lr = 0x82410848;
	sub_82411338(ctx, base);
	// 82410848: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8241084C: 4182001C  beq 0x82410868
	if ctx.cr[0].eq {
	pc = 0x82410868; continue 'dispatch;
	}
	// 82410850: 4BFFFF71  bl 0x824107c0
	ctx.lr = 0x82410854;
	sub_824107C0(ctx, base);
	// 82410854: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82410858: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8241085C: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82410860: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82410864: 4E800421  bctrl
	ctx.lr = 0x82410868;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x82410868 => {
    //   block [0x82410868..0x82410884)
	// 82410868: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8241086C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82410870: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82410874: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82410878: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8241087C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82410880: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82410888(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82410888 size=84
    let mut pc: u32 = 0x82410888;
    'dispatch: loop {
        match pc {
            0x82410888 => {
    //   block [0x82410888..0x824108A8)
	// 82410888: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8241088C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82410890: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82410894: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82410898: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8241089C: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 824108A0: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 824108A4: 4082000C  bne 0x824108b0
	if !ctx.cr[0].eq {
	pc = 0x824108B0; continue 'dispatch;
	}
	pc = 0x824108A8; continue 'dispatch;
            }
            0x824108A8 => {
    //   block [0x824108A8..0x824108B0)
	// 824108A8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824108AC: 4800001C  b 0x824108c8
	pc = 0x824108C8; continue 'dispatch;
            }
            0x824108B0 => {
    //   block [0x824108B0..0x824108C8)
	// 824108B0: 48000D11  bl 0x824115c0
	ctx.lr = 0x824108B4;
	sub_824115C0(ctx, base);
	// 824108B4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 824108B8: 4182FFF0  beq 0x824108a8
	if ctx.cr[0].eq {
	pc = 0x824108A8; continue 'dispatch;
	}
	// 824108BC: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 824108C0: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 824108C4: 48000A6D  bl 0x82411330
	ctx.lr = 0x824108C8;
	sub_82411330(ctx, base);
	pc = 0x824108C8; continue 'dispatch;
            }
            0x824108C8 => {
    //   block [0x824108C8..0x824108DC)
	// 824108C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824108CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824108D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824108D4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824108D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824108E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x824108E0 size=188
    let mut pc: u32 = 0x824108E0;
    'dispatch: loop {
        match pc {
            0x824108E0 => {
    //   block [0x824108E0..0x82410904)
	// 824108E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824108E4: 481247D9  bl 0x825350bc
	ctx.lr = 0x824108E8;
	sub_82535080(ctx, base);
	// 824108E8: DBE1FFD8  stfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 824108EC: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824108F0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824108F4: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 824108F8: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 824108FC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82410900: 409A000C  bne cr6, 0x8241090c
	if !ctx.cr[6].eq {
	pc = 0x8241090C; continue 'dispatch;
	}
	pc = 0x82410904; continue 'dispatch;
            }
            0x82410904 => {
    //   block [0x82410904..0x8241090C)
	// 82410904: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82410908: 48000088  b 0x82410990
	pc = 0x82410990; continue 'dispatch;
            }
            0x8241090C => {
    //   block [0x8241090C..0x82410944)
	// 8241090C: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82410910: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82410914: 480005B5  bl 0x82410ec8
	ctx.lr = 0x82410918;
	sub_82410EC8(ctx, base);
	// 82410918: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8241091C: 4182FFE8  beq 0x82410904
	if ctx.cr[0].eq {
	pc = 0x82410904; continue 'dispatch;
	}
	// 82410920: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82410924: 48000755  bl 0x82411078
	ctx.lr = 0x82410928;
	sub_82411078(ctx, base);
	// 82410928: 7F1E1840  cmplw cr6, r30, r3
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[3].u32, &mut ctx.xer);
	// 8241092C: 409A0018  bne cr6, 0x82410944
	if !ctx.cr[6].eq {
	pc = 0x82410944; continue 'dispatch;
	}
	// 82410930: C01F0014  lfs f0, 0x14(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82410934: FF00F800  fcmpu cr6, f0, f31
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[31].f64);
	// 82410938: 409A000C  bne cr6, 0x82410944
	if !ctx.cr[6].eq {
	pc = 0x82410944; continue 'dispatch;
	}
	// 8241093C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82410940: 48000050  b 0x82410990
	pc = 0x82410990; continue 'dispatch;
            }
            0x82410944 => {
    //   block [0x82410944..0x8241098C)
	// 82410944: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82410948: 48000731  bl 0x82411078
	ctx.lr = 0x8241094C;
	sub_82411078(ctx, base);
	// 8241094C: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82410950: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82410954: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82410958: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 8241095C: 480009D5  bl 0x82411330
	ctx.lr = 0x82410960;
	sub_82411330(ctx, base);
	// 82410960: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82410964: 41820028  beq 0x8241098c
	if ctx.cr[0].eq {
	pc = 0x8241098C; continue 'dispatch;
	}
	// 82410968: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8241096C: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82410970: D3FF0014  stfs f31, 0x14(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 82410974: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82410978: 48000701  bl 0x82411078
	ctx.lr = 0x8241097C;
	sub_82411078(ctx, base);
	// 8241097C: 7F03E840  cmplw cr6, r3, r29
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[29].u32, &mut ctx.xer);
	// 82410980: 409A000C  bne cr6, 0x8241098c
	if !ctx.cr[6].eq {
	pc = 0x8241098C; continue 'dispatch;
	}
	// 82410984: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82410988: 48000721  bl 0x824110a8
	ctx.lr = 0x8241098C;
	sub_824110A8(ctx, base);
	pc = 0x8241098C; continue 'dispatch;
            }
            0x8241098C => {
    //   block [0x8241098C..0x82410990)
	// 8241098C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	pc = 0x82410990; continue 'dispatch;
            }
            0x82410990 => {
    //   block [0x82410990..0x8241099C)
	// 82410990: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82410994: CBE1FFD8  lfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-40 as u32) ) };
	// 82410998: 48124774  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824109A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824109A0 size=20
    let mut pc: u32 = 0x824109A0;
    'dispatch: loop {
        match pc {
            0x824109A0 => {
    //   block [0x824109A0..0x824109B4)
	// 824109A0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 824109A4: 806B000C  lwz r3, 0xc(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 824109A8: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 824109AC: 40820008  bne 0x824109b4
	if !ctx.cr[0].eq {
		sub_824109B4(ctx, base);
		return;
	}
	// 824109B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824109B4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x824109B4 size=16
    let mut pc: u32 = 0x824109B4;
    'dispatch: loop {
        match pc {
            0x824109B4 => {
    //   block [0x824109B4..0x824109C4)
	// 824109B4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 824109B8: D02B0014  stfs f1, 0x14(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 824109BC: 914B0018  stw r10, 0x18(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[10].u32 ) };
	// 824109C0: 48000D80  b 0x82411740
	sub_82411740(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824109C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824109C8 size=16
    let mut pc: u32 = 0x824109C8;
    'dispatch: loop {
        match pc {
            0x824109C8 => {
    //   block [0x824109C8..0x824109D8)
	// 824109C8: 8063000C  lwz r3, 0xc(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 824109CC: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 824109D0: 40820008  bne 0x824109d8
	if !ctx.cr[0].eq {
		sub_824109D8(ctx, base);
		return;
	}
	// 824109D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824109D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824109D8 size=4
    let mut pc: u32 = 0x824109D8;
    'dispatch: loop {
        match pc {
            0x824109D8 => {
    //   block [0x824109D8..0x824109DC)
	// 824109D8: 48000BD0  b 0x824115a8
	crate::recompiler::externs::call(ctx, base, 0x824115A8);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824109E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824109E0 size=16
    let mut pc: u32 = 0x824109E0;
    'dispatch: loop {
        match pc {
            0x824109E0 => {
    //   block [0x824109E0..0x824109F0)
	// 824109E0: 8063000C  lwz r3, 0xc(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 824109E4: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 824109E8: 40820008  bne 0x824109f0
	if !ctx.cr[0].eq {
		sub_824109F0(ctx, base);
		return;
	}
	// 824109EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824109F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824109F0 size=4
    let mut pc: u32 = 0x824109F0;
    'dispatch: loop {
        match pc {
            0x824109F0 => {
    //   block [0x824109F0..0x824109F4)
	// 824109F0: 48000B28  b 0x82411518
	sub_82411518(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824109F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824109F8 size=16
    let mut pc: u32 = 0x824109F8;
    'dispatch: loop {
        match pc {
            0x824109F8 => {
    //   block [0x824109F8..0x82410A08)
	// 824109F8: 8063000C  lwz r3, 0xc(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 824109FC: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82410A00: 40820008  bne 0x82410a08
	if !ctx.cr[0].eq {
		sub_82410A08(ctx, base);
		return;
	}
	// 82410A04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82410A08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82410A08 size=16
    let mut pc: u32 = 0x82410A08;
    'dispatch: loop {
        match pc {
            0x82410A08 => {
    //   block [0x82410A08..0x82410A18)
	// 82410A08: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82410A0C: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82410A10: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82410A14: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82410A18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82410A18 size=16
    let mut pc: u32 = 0x82410A18;
    'dispatch: loop {
        match pc {
            0x82410A18 => {
    //   block [0x82410A18..0x82410A28)
	// 82410A18: 8063000C  lwz r3, 0xc(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82410A1C: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82410A20: 40820008  bne 0x82410a28
	if !ctx.cr[0].eq {
		sub_82410A28(ctx, base);
		return;
	}
	// 82410A24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82410A28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82410A28 size=4
    let mut pc: u32 = 0x82410A28;
    'dispatch: loop {
        match pc {
            0x82410A28 => {
    //   block [0x82410A28..0x82410A2C)
	// 82410A28: 48000E70  b 0x82411898
	crate::recompiler::externs::call(ctx, base, 0x82411898);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82410A30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82410A30 size=16
    let mut pc: u32 = 0x82410A30;
    'dispatch: loop {
        match pc {
            0x82410A30 => {
    //   block [0x82410A30..0x82410A40)
	// 82410A30: 8063000C  lwz r3, 0xc(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82410A34: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82410A38: 40820008  bne 0x82410a40
	if !ctx.cr[0].eq {
		sub_82410A40(ctx, base);
		return;
	}
	// 82410A3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82410A40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82410A40 size=4
    let mut pc: u32 = 0x82410A40;
    'dispatch: loop {
        match pc {
            0x82410A40 => {
    //   block [0x82410A40..0x82410A44)
	// 82410A40: 48000E70  b 0x824118b0
	crate::recompiler::externs::call(ctx, base, 0x824118B0);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82410A48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82410A48 size=16
    let mut pc: u32 = 0x82410A48;
    'dispatch: loop {
        match pc {
            0x82410A48 => {
    //   block [0x82410A48..0x82410A58)
	// 82410A48: 8063000C  lwz r3, 0xc(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82410A4C: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82410A50: 40820008  bne 0x82410a58
	if !ctx.cr[0].eq {
		sub_82410A58(ctx, base);
		return;
	}
	// 82410A54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82410A58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82410A58 size=16
    let mut pc: u32 = 0x82410A58;
    'dispatch: loop {
        match pc {
            0x82410A58 => {
    //   block [0x82410A58..0x82410A68)
	// 82410A58: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82410A5C: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82410A60: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82410A64: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82410A68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82410A68 size=88
    let mut pc: u32 = 0x82410A68;
    'dispatch: loop {
        match pc {
            0x82410A68 => {
    //   block [0x82410A68..0x82410A90)
	// 82410A68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82410A6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82410A70: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82410A74: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82410A78: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82410A7C: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82410A80: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82410A84: 409A000C  bne cr6, 0x82410a90
	if !ctx.cr[6].eq {
	pc = 0x82410A90; continue 'dispatch;
	}
	// 82410A88: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82410A8C: 48000020  b 0x82410aac
	pc = 0x82410AAC; continue 'dispatch;
            }
            0x82410A90 => {
    //   block [0x82410A90..0x82410AAC)
	// 82410A90: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82410A94: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82410A98: 48000899  bl 0x82411330
	ctx.lr = 0x82410A9C;
	sub_82411330(ctx, base);
	// 82410A9C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82410AA0: 4182000C  beq 0x82410aac
	if ctx.cr[0].eq {
	pc = 0x82410AAC; continue 'dispatch;
	}
	// 82410AA4: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82410AA8: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	pc = 0x82410AAC; continue 'dispatch;
            }
            0x82410AAC => {
    //   block [0x82410AAC..0x82410AC0)
	// 82410AAC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82410AB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82410AB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82410AB8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82410ABC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82410AC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82410AC0 size=356
    //   switch @ 0x82410B24: r3 with 13 label(s)
    //       case  0 → 0x82410B34
    //       case  1 → 0x82410B40
    //       case  2 → 0x82410B54
    //       case  3 → 0x82410B84
    //       case  4 → 0x82410B60
    //       case  5 → 0x82410B78
    //       case  6 → 0x82410B70
    //       case  7 → 0x82410B78
    //       case  8 → 0x82410B8C
    //       case  9 → 0x82410B60
    //       case 10 → 0x82410B78
    //       case 11 → 0x82410B60
    //       case 12 → 0x82410B78
    let mut pc: u32 = 0x82410AC0;
    'dispatch: loop {
        match pc {
            0x82410AC0 => {
    //   block [0x82410AC0..0x82410AE4)
	// 82410AC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82410AC4: 481245F9  bl 0x825350bc
	ctx.lr = 0x82410AC8;
	sub_82535080(ctx, base);
	// 82410AC8: DBE1FFD8  stfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 82410ACC: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82410AD0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82410AD4: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82410AD8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82410ADC: 409A0014  bne cr6, 0x82410af0
	if !ctx.cr[6].eq {
	pc = 0x82410AF0; continue 'dispatch;
	}
	// 82410AE0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	pc = 0x82410AE4; continue 'dispatch;
            }
            0x82410AE4 => {
    //   block [0x82410AE4..0x82410AF0)
	// 82410AE4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82410AE8: CBE1FFD8  lfd f31, -0x28(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-40 as u32) ) };
	// 82410AEC: 48124620  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            0x82410AF0 => {
    //   block [0x82410AF0..0x82410AFC)
	// 82410AF0: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 82410AF4: 3BA00001  li r29, 1
	ctx.r[29].s64 = 1;
	// 82410AF8: C3EB1FF8  lfs f31, 0x1ff8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	pc = 0x82410AFC; continue 'dispatch;
            }
            0x82410AFC => {
    //   block [0x82410AFC..0x82410B34)
	// 82410AFC: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82410B00: 48000579  bl 0x82411078
	ctx.lr = 0x82410B04;
	sub_82411078(ctx, base);
	// 82410B04: 2B03000C  cmplwi cr6, r3, 0xc
	ctx.cr[6].compare_u32(ctx.r[3].u32, 12 as u32, &mut ctx.xer);
	// 82410B08: 41990118  bgt cr6, 0x82410c20
	if ctx.cr[6].gt {
	pc = 0x82410C20; continue 'dispatch;
	}
	// 82410B0C: 3D808201  lis r12, -0x7dff
	ctx.r[12].s64 = -2113863680;
	// 82410B10: 398CEB40  addi r12, r12, -0x14c0
	ctx.r[12].s64 = ctx.r[12].s64 + -5312;
	// 82410B14: 7C0C18AE  lbzx r0, r12, r3
	ctx.r[0].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[12].u32.wrapping_add(ctx.r[3].u32)) } as u64;
	// 82410B18: 3D808241  lis r12, -0x7dbf
	ctx.r[12].s64 = -2109669376;
	// 82410B1C: 398C0B34  addi r12, r12, 0xb34
	ctx.r[12].s64 = ctx.r[12].s64 + 2868;
	// 82410B20: 7D8C0214  add r12, r12, r0
	ctx.r[12].u64 = ctx.r[12].u64 + ctx.r[0].u64;
	// 82410B24: 7D8903A6  mtctr r12
	ctx.ctr.u64 = ctx.r[12].u64;
	// 82410B28: 60000000  nop
	// 82410B2C: 60000000  nop
	// 82410B30: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            0x82410B34 => {
    //   block [0x82410B34..0x82410B40)
	// 82410B34: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82410B38: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82410B3C: 48000060  b 0x82410b9c
	pc = 0x82410B9C; continue 'dispatch;
            }
            0x82410B40 => {
    //   block [0x82410B40..0x82410B4C)
	// 82410B40: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82410B44: 809F0018  lwz r4, 0x18(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82410B48: 48000E59  bl 0x824119a0
	ctx.lr = 0x82410B4C;
	sub_824119A0(ctx, base);
	pc = 0x82410B4C; continue 'dispatch;
            }
            0x82410B4C => {
    //   block [0x82410B4C..0x82410B54)
	// 82410B4C: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82410B50: 48000048  b 0x82410b98
	pc = 0x82410B98; continue 'dispatch;
            }
            0x82410B54 => {
    //   block [0x82410B54..0x82410B60)
	// 82410B54: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82410B58: 48000AC9  bl 0x82411620
	ctx.lr = 0x82410B5C;
	sub_82411620(ctx, base);
	// 82410B5C: 4BFFFFF0  b 0x82410b4c
	pc = 0x82410B4C; continue 'dispatch;
            }
            0x82410B60 => {
    //   block [0x82410B60..0x82410B64)
	// 82410B60: C03F0014  lfs f1, 0x14(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	pc = 0x82410B64; continue 'dispatch;
            }
            0x82410B64 => {
    //   block [0x82410B64..0x82410B70)
	// 82410B64: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82410B68: 48000BD9  bl 0x82411740
	ctx.lr = 0x82410B6C;
	sub_82411740(ctx, base);
	// 82410B6C: 4800002C  b 0x82410b98
	pc = 0x82410B98; continue 'dispatch;
            }
            0x82410B70 => {
    //   block [0x82410B70..0x82410B78)
	// 82410B70: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82410B74: 4BFFFFF0  b 0x82410b64
	pc = 0x82410B64; continue 'dispatch;
            }
            0x82410B78 => {
    //   block [0x82410B78..0x82410B84)
	// 82410B78: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82410B7C: 48000C95  bl 0x82411810
	ctx.lr = 0x82410B80;
	sub_82411810(ctx, base);
	// 82410B80: 4BFFFFCC  b 0x82410b4c
	pc = 0x82410B4C; continue 'dispatch;
            }
            0x82410B84 => {
    //   block [0x82410B84..0x82410B8C)
	// 82410B84: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82410B88: 48000008  b 0x82410b90
	pc = 0x82410B90; continue 'dispatch;
            }
            0x82410B8C => {
    //   block [0x82410B8C..0x82410B90)
	// 82410B8C: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	pc = 0x82410B90; continue 'dispatch;
            }
            0x82410B90 => {
    //   block [0x82410B90..0x82410B98)
	// 82410B90: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82410B94: 4800098D  bl 0x82411520
	ctx.lr = 0x82410B98;
	sub_82411520(ctx, base);
	pc = 0x82410B98; continue 'dispatch;
            }
            0x82410B98 => {
    //   block [0x82410B98..0x82410B9C)
	// 82410B98: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	pc = 0x82410B9C; continue 'dispatch;
            }
            0x82410B9C => {
    //   block [0x82410B9C..0x82410BBC)
	// 82410B9C: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82410BA0: 480004D9  bl 0x82411078
	ctx.lr = 0x82410BA4;
	sub_82411078(ctx, base);
	// 82410BA4: 2B030005  cmplwi cr6, r3, 5
	ctx.cr[6].compare_u32(ctx.r[3].u32, 5 as u32, &mut ctx.xer);
	// 82410BA8: 419A0014  beq cr6, 0x82410bbc
	if ctx.cr[6].eq {
	pc = 0x82410BBC; continue 'dispatch;
	}
	// 82410BAC: 2B03000A  cmplwi cr6, r3, 0xa
	ctx.cr[6].compare_u32(ctx.r[3].u32, 10 as u32, &mut ctx.xer);
	// 82410BB0: 419A000C  beq cr6, 0x82410bbc
	if ctx.cr[6].eq {
	pc = 0x82410BBC; continue 'dispatch;
	}
	// 82410BB4: 2B03000C  cmplwi cr6, r3, 0xc
	ctx.cr[6].compare_u32(ctx.r[3].u32, 12 as u32, &mut ctx.xer);
	// 82410BB8: 409A0038  bne cr6, 0x82410bf0
	if !ctx.cr[6].eq {
	pc = 0x82410BF0; continue 'dispatch;
	}
	pc = 0x82410BBC; continue 'dispatch;
            }
            0x82410BBC => {
    //   block [0x82410BBC..0x82410BD8)
	// 82410BBC: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82410BC0: 419A0030  beq cr6, 0x82410bf0
	if ctx.cr[6].eq {
	pc = 0x82410BF0; continue 'dispatch;
	}
	// 82410BC4: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82410BC8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82410BCC: 409A000C  bne cr6, 0x82410bd8
	if !ctx.cr[6].eq {
	pc = 0x82410BD8; continue 'dispatch;
	}
	// 82410BD0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82410BD4: 48000010  b 0x82410be4
	pc = 0x82410BE4; continue 'dispatch;
            }
            0x82410BD8 => {
    //   block [0x82410BD8..0x82410BE4)
	// 82410BD8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82410BDC: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82410BE0: 48000751  bl 0x82411330
	ctx.lr = 0x82410BE4;
	sub_82411330(ctx, base);
	pc = 0x82410BE4; continue 'dispatch;
            }
            0x82410BE4 => {
    //   block [0x82410BE4..0x82410BF0)
	// 82410BE4: 3BA00001  li r29, 1
	ctx.r[29].s64 = 1;
	// 82410BE8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82410BEC: 419A0030  beq cr6, 0x82410c1c
	if ctx.cr[6].eq {
	pc = 0x82410C1C; continue 'dispatch;
	}
	pc = 0x82410BF0; continue 'dispatch;
            }
            0x82410BF0 => {
    //   block [0x82410BF0..0x82410C1C)
	// 82410BF0: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82410BF4: 480004B5  bl 0x824110a8
	ctx.lr = 0x82410BF8;
	sub_824110A8(ctx, base);
	// 82410BF8: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82410BFC: 409AFF00  bne cr6, 0x82410afc
	if !ctx.cr[6].eq {
	pc = 0x82410AFC; continue 'dispatch;
	}
	// 82410C00: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82410C04: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82410C08: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82410C0C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82410C10: 4E800421  bctrl
	ctx.lr = 0x82410C14;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82410C14: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82410C18: 4BFFFECC  b 0x82410ae4
	pc = 0x82410AE4; continue 'dispatch;
            }
            0x82410C1C => {
    //   block [0x82410C1C..0x82410C20)
	// 82410C1C: 48000000  b 0x82410c1c
	pc = 0x82410C1C; continue 'dispatch;
            }
            0x82410C20 => {
    //   block [0x82410C20..0x82410C24)
	// 82410C20: 48000000  b 0x82410c20
	pc = 0x82410C20; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82410C28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82410C28 size=104
    let mut pc: u32 = 0x82410C28;
    'dispatch: loop {
        match pc {
            0x82410C28 => {
    //   block [0x82410C28..0x82410C58)
	// 82410C28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82410C2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82410C30: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82410C34: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82410C38: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82410C3C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82410C40: 3BDF000C  addi r30, r31, 0xc
	ctx.r[30].s64 = ctx.r[31].s64 + 12;
	// 82410C44: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82410C48: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82410C4C: 409A000C  bne cr6, 0x82410c58
	if !ctx.cr[6].eq {
	pc = 0x82410C58; continue 'dispatch;
	}
	// 82410C50: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82410C54: 48000024  b 0x82410c78
	pc = 0x82410C78; continue 'dispatch;
            }
            0x82410C58 => {
    //   block [0x82410C58..0x82410C78)
	// 82410C58: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82410C5C: 4BFFFB7D  bl 0x824107d8
	ctx.lr = 0x82410C60;
	sub_824107D8(ctx, base);
	// 82410C60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82410C64: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82410C68: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82410C6C: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82410C70: 480006D1  bl 0x82411340
	ctx.lr = 0x82410C74;
	sub_82411340(ctx, base);
	// 82410C74: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	pc = 0x82410C78; continue 'dispatch;
            }
            0x82410C78 => {
    //   block [0x82410C78..0x82410C90)
	// 82410C78: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82410C7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82410C80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82410C84: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82410C88: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82410C8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82410C90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82410C90 size=84
    let mut pc: u32 = 0x82410C90;
    'dispatch: loop {
        match pc {
            0x82410C90 => {
    //   block [0x82410C90..0x82410CC8)
	// 82410C90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82410C94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82410C98: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82410C9C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82410CA0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82410CA4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82410CA8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82410CAC: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82410CB0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82410CB4: 41820014  beq 0x82410cc8
	if ctx.cr[0].eq {
	pc = 0x82410CC8; continue 'dispatch;
	}
	// 82410CB8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82410CBC: 4BFFFB6D  bl 0x82410828
	ctx.lr = 0x82410CC0;
	sub_82410828(ctx, base);
	// 82410CC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82410CC4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	pc = 0x82410CC8; continue 'dispatch;
            }
            0x82410CC8 => {
    //   block [0x82410CC8..0x82410CE4)
	// 82410CC8: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82410CCC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82410CD0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82410CD4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82410CD8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82410CDC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82410CE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82410CE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82410CE8 size=132
    let mut pc: u32 = 0x82410CE8;
    'dispatch: loop {
        match pc {
            0x82410CE8 => {
    //   block [0x82410CE8..0x82410D54)
	// 82410CE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82410CEC: 481243D1  bl 0x825350bc
	ctx.lr = 0x82410CF0;
	sub_82535080(ctx, base);
	// 82410CF0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82410CF4: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 82410CF8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82410CFC: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82410D00: 3BBF0010  addi r29, r31, 0x10
	ctx.r[29].s64 = ctx.r[31].s64 + 16;
	// 82410D04: C00B1FF8  lfs f0, 0x1ff8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82410D08: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82410D0C: 909F0000  stw r4, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 82410D10: 90BF0004  stw r5, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82410D14: 90DF0008  stw r6, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[6].u32 ) };
	// 82410D18: 93DF000C  stw r30, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[30].u32 ) };
	// 82410D1C: 93DF0010  stw r30, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[30].u32 ) };
	// 82410D20: D01F0014  stfs f0, 0x14(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 82410D24: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82410D28: 4BFFFA99  bl 0x824107c0
	ctx.lr = 0x82410D2C;
	sub_824107C0(ctx, base);
	// 82410D2C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82410D30: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 82410D34: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82410D38: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82410D3C: 4E800421  bctrl
	ctx.lr = 0x82410D40;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82410D40: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82410D44: 41820010  beq 0x82410d54
	if ctx.cr[0].eq {
	pc = 0x82410D54; continue 'dispatch;
	}
	// 82410D48: 480006D9  bl 0x82411420
	ctx.lr = 0x82410D4C;
	sub_82411420(ctx, base);
	// 82410D4C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82410D50: 48000008  b 0x82410d58
	pc = 0x82410D58; continue 'dispatch;
            }
            0x82410D54 => {
    //   block [0x82410D54..0x82410D58)
	// 82410D54: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	pc = 0x82410D58; continue 'dispatch;
            }
            0x82410D58 => {
    //   block [0x82410D58..0x82410D6C)
	// 82410D58: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82410D5C: 4BFFFF35  bl 0x82410c90
	ctx.lr = 0x82410D60;
	sub_82410C90(ctx, base);
	// 82410D60: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82410D64: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82410D68: 481243A4  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82410D70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82410D70 size=80
    let mut pc: u32 = 0x82410D70;
    'dispatch: loop {
        match pc {
            0x82410D70 => {
    //   block [0x82410D70..0x82410DA4)
	// 82410D70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82410D74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82410D78: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82410D7C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82410D80: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82410D84: 4BFFFEA5  bl 0x82410c28
	ctx.lr = 0x82410D88;
	sub_82410C28(ctx, base);
	// 82410D88: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82410D8C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82410D90: 41820014  beq 0x82410da4
	if ctx.cr[0].eq {
	pc = 0x82410DA4; continue 'dispatch;
	}
	// 82410D94: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82410D98: 4BFFFA91  bl 0x82410828
	ctx.lr = 0x82410D9C;
	sub_82410828(ctx, base);
	// 82410D9C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82410DA0: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	pc = 0x82410DA4; continue 'dispatch;
            }
            0x82410DA4 => {
    //   block [0x82410DA4..0x82410DC0)
	// 82410DA4: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 82410DA8: 4BFFFA31  bl 0x824107d8
	ctx.lr = 0x82410DAC;
	sub_824107D8(ctx, base);
	// 82410DAC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82410DB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82410DB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82410DB8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82410DBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82410DC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82410DC0 size=136
    let mut pc: u32 = 0x82410DC0;
    'dispatch: loop {
        match pc {
            0x82410DC0 => {
    //   block [0x82410DC0..0x82410E1C)
	// 82410DC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82410DC4: 481242F9  bl 0x825350bc
	ctx.lr = 0x82410DC8;
	sub_82535080(ctx, base);
	// 82410DC8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82410DCC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82410DD0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82410DD4: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82410DD8: 4BFFFE51  bl 0x82410c28
	ctx.lr = 0x82410DDC;
	sub_82410C28(ctx, base);
	// 82410DDC: 4BFFF9E5  bl 0x824107c0
	ctx.lr = 0x82410DE0;
	sub_824107C0(ctx, base);
	// 82410DE0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82410DE4: 38800034  li r4, 0x34
	ctx.r[4].s64 = 52;
	// 82410DE8: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82410DEC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82410DF0: 4E800421  bctrl
	ctx.lr = 0x82410DF4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82410DF4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82410DF8: 41820024  beq 0x82410e1c
	if ctx.cr[0].eq {
	pc = 0x82410E1C; continue 'dispatch;
	}
	// 82410DFC: 7FA8EB78  mr r8, r29
	ctx.r[8].u64 = ctx.r[29].u64;
	// 82410E00: 80DF0008  lwz r6, 8(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82410E04: 7FC7F378  mr r7, r30
	ctx.r[7].u64 = ctx.r[30].u64;
	// 82410E08: 80BF0004  lwz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82410E0C: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82410E10: 48000FB9  bl 0x82411dc8
	ctx.lr = 0x82410E14;
	sub_82411DC8(ctx, base);
	// 82410E14: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82410E18: 48000008  b 0x82410e20
	pc = 0x82410E20; continue 'dispatch;
            }
            0x82410E1C => {
    //   block [0x82410E1C..0x82410E20)
	// 82410E1C: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	pc = 0x82410E20; continue 'dispatch;
            }
            0x82410E20 => {
    //   block [0x82410E20..0x82410E48)
	// 82410E20: 3BDF000C  addi r30, r31, 0xc
	ctx.r[30].s64 = ctx.r[31].s64 + 12;
	// 82410E24: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82410E28: 4BFFF9B1  bl 0x824107d8
	ctx.lr = 0x82410E2C;
	sub_824107D8(ctx, base);
	// 82410E2C: 93BE0000  stw r29, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82410E30: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82410E34: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82410E38: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82410E3C: 69630001  xori r3, r11, 1
	ctx.r[3].u64 = ctx.r[11].u64 ^ 1;
	// 82410E40: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82410E44: 481242C8  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82410E48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82410E48 size=128
    let mut pc: u32 = 0x82410E48;
    'dispatch: loop {
        match pc {
            0x82410E48 => {
    //   block [0x82410E48..0x82410E88)
	// 82410E48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82410E4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82410E50: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82410E54: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82410E58: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82410E5C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82410E60: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82410E64: 3BDF0004  addi r30, r31, 4
	ctx.r[30].s64 = ctx.r[31].s64 + 4;
	// 82410E68: 38A00034  li r5, 0x34
	ctx.r[5].s64 = 52;
	// 82410E6C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82410E70: 48123CE1  bl 0x82534b50
	ctx.lr = 0x82410E74;
	sub_82534B50(ctx, base);
	// 82410E74: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82410E78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82410E7C: 2C0A0000  cmpwi r10, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82410E80: 41800030  blt 0x82410eb0
	if ctx.cr[0].lt {
	pc = 0x82410EB0; continue 'dispatch;
	}
	// 82410E84: 7FC9F378  mr r9, r30
	ctx.r[9].u64 = ctx.r[30].u64;
	pc = 0x82410E88; continue 'dispatch;
            }
            0x82410E88 => {
    //   block [0x82410E88..0x82410EB0)
	// 82410E88: 7D4B5050  subf r10, r11, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 82410E8C: 39010050  addi r8, r1, 0x50
	ctx.r[8].s64 = ctx.r[1].s64 + 80;
	// 82410E90: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82410E94: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82410E98: 7D4A402E  lwzx r10, r10, r8
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 82410E9C: 91490000  stw r10, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82410EA0: 39290004  addi r9, r9, 4
	ctx.r[9].s64 = ctx.r[9].s64 + 4;
	// 82410EA4: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82410EA8: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82410EAC: 4099FFDC  ble cr6, 0x82410e88
	if !ctx.cr[6].gt {
	pc = 0x82410E88; continue 'dispatch;
	}
	pc = 0x82410EB0; continue 'dispatch;
            }
            0x82410EB0 => {
    //   block [0x82410EB0..0x82410EC8)
	// 82410EB0: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82410EB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82410EB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82410EBC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82410EC0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82410EC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82410EC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82410EC8 size=24
    let mut pc: u32 = 0x82410EC8;
    'dispatch: loop {
        match pc {
            0x82410EC8 => {
    //   block [0x82410EC8..0x82410EE0)
	// 82410EC8: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82410ECC: C00B1850  lfs f0, 0x1850(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(6224 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82410ED0: FF010000  fcmpu cr6, f1, f0
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 82410ED4: 409A000C  bne cr6, 0x82410ee0
	if !ctx.cr[6].eq {
		sub_82410EE0(ctx, base);
		return;
	}
	// 82410ED8: 38600005  li r3, 5
	ctx.r[3].s64 = 5;
	// 82410EDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82410EE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82410EE0 size=24
    let mut pc: u32 = 0x82410EE0;
    'dispatch: loop {
        match pc {
            0x82410EE0 => {
    //   block [0x82410EE0..0x82410EF8)
	// 82410EE0: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 82410EE4: C00B1FF8  lfs f0, 0x1ff8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82410EE8: FF010000  fcmpu cr6, f1, f0
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 82410EEC: 409A000C  bne cr6, 0x82410ef8
	if !ctx.cr[6].eq {
		sub_82410EF8(ctx, base);
		return;
	}
	// 82410EF0: 38600007  li r3, 7
	ctx.r[3].s64 = 7;
	// 82410EF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82410EF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82410EF8 size=16
    let mut pc: u32 = 0x82410EF8;
    'dispatch: loop {
        match pc {
            0x82410EF8 => {
    //   block [0x82410EF8..0x82410F08)
	// 82410EF8: FF010000  fcmpu cr6, f1, f0
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 82410EFC: 4099000C  ble cr6, 0x82410f08
	if !ctx.cr[6].gt {
		sub_82410F08(ctx, base);
		return;
	}
	// 82410F00: 3860000A  li r3, 0xa
	ctx.r[3].s64 = 10;
	// 82410F04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82410F08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82410F08 size=20
    let mut pc: u32 = 0x82410F08;
    'dispatch: loop {
        match pc {
            0x82410F08 => {
    //   block [0x82410F08..0x82410F1C)
	// 82410F08: FF010000  fcmpu cr6, f1, f0
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 82410F0C: 3860000C  li r3, 0xc
	ctx.r[3].s64 = 12;
	// 82410F10: 4D980020  bltlr cr6
	if ctx.cr[6].lt { return; }
	// 82410F14: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82410F18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82410F20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82410F20 size=340
    let mut pc: u32 = 0x82410F20;
    'dispatch: loop {
        match pc {
            0x82410F20 => {
    //   block [0x82410F20..0x82410F50)
	// 82410F20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82410F24: 4812418D  bl 0x825350b0
	ctx.lr = 0x82410F28;
	sub_82535080(ctx, base);
	// 82410F28: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82410F2C: 81450000  lwz r10, 0(r5)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 82410F30: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 82410F34: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 82410F38: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 82410F3C: 7F1B5040  cmplw cr6, r27, r10
	ctx.cr[6].compare_u32(ctx.r[27].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82410F40: 409A003C  bne cr6, 0x82410f7c
	if !ctx.cr[6].eq {
	pc = 0x82410F7C; continue 'dispatch;
	}
	// 82410F44: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82410F48: 2F0BFFFF  cmpwi cr6, r11, -1
	ctx.cr[6].compare_i32(ctx.r[11].s32, -1, &mut ctx.xer);
	// 82410F4C: 40980008  bge cr6, 0x82410f54
	if !ctx.cr[6].lt {
	pc = 0x82410F54; continue 'dispatch;
	}
	pc = 0x82410F50; continue 'dispatch;
            }
            0x82410F50 => {
    //   block [0x82410F50..0x82410F54)
	// 82410F50: 48000000  b 0x82410f50
	pc = 0x82410F50; continue 'dispatch;
            }
            0x82410F54 => {
    //   block [0x82410F54..0x82410F5C)
	// 82410F54: 2F0B000C  cmpwi cr6, r11, 0xc
	ctx.cr[6].compare_i32(ctx.r[11].s32, 12, &mut ctx.xer);
	// 82410F58: 41980008  blt cr6, 0x82410f60
	if ctx.cr[6].lt {
	pc = 0x82410F60; continue 'dispatch;
	}
	pc = 0x82410F5C; continue 'dispatch;
            }
            0x82410F5C => {
    //   block [0x82410F5C..0x82410F60)
	// 82410F5C: 48000000  b 0x82410f5c
	pc = 0x82410F5C; continue 'dispatch;
            }
            0x82410F60 => {
    //   block [0x82410F60..0x82410F7C)
	// 82410F60: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82410F64: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82410F68: 392B0001  addi r9, r11, 1
	ctx.r[9].s64 = ctx.r[11].s64 + 1;
	// 82410F6C: 5529103A  slwi r9, r9, 2
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82410F70: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82410F74: 7D49F92E  stwx r10, r9, r31
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[31].u32), ctx.r[10].u32) };
	// 82410F78: 480000F4  b 0x8241106c
	pc = 0x8241106C; continue 'dispatch;
            }
            0x82410F7C => {
    //   block [0x82410F7C..0x82410F90)
	// 82410F7C: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82410F80: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82410F84: 2F090001  cmpwi cr6, r9, 1
	ctx.cr[6].compare_i32(ctx.r[9].s32, 1, &mut ctx.xer);
	// 82410F88: 41980028  blt cr6, 0x82410fb0
	if ctx.cr[6].lt {
	pc = 0x82410FB0; continue 'dispatch;
	}
	// 82410F8C: 397F0008  addi r11, r31, 8
	ctx.r[11].s64 = ctx.r[31].s64 + 8;
	pc = 0x82410F90; continue 'dispatch;
            }
            0x82410F90 => {
    //   block [0x82410F90..0x82410FB0)
	// 82410F90: 80EB0000  lwz r7, 0(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82410F94: 7F0A3840  cmplw cr6, r10, r7
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[7].u32, &mut ctx.xer);
	// 82410F98: 419A00D0  beq cr6, 0x82411068
	if ctx.cr[6].eq {
	pc = 0x82411068; continue 'dispatch;
	}
	// 82410F9C: 80FF0000  lwz r7, 0(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82410FA0: 39080001  addi r8, r8, 1
	ctx.r[8].s64 = ctx.r[8].s64 + 1;
	// 82410FA4: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82410FA8: 7F083800  cmpw cr6, r8, r7
	ctx.cr[6].compare_i32(ctx.r[8].s32, ctx.r[7].s32, &mut ctx.xer);
	// 82410FAC: 4099FFE4  ble cr6, 0x82410f90
	if !ctx.cr[6].gt {
	pc = 0x82410F90; continue 'dispatch;
	}
	pc = 0x82410FB0; continue 'dispatch;
            }
            0x82410FB0 => {
    //   block [0x82410FB0..0x82410FB8)
	// 82410FB0: 2F09FFFF  cmpwi cr6, r9, -1
	ctx.cr[6].compare_i32(ctx.r[9].s32, -1, &mut ctx.xer);
	// 82410FB4: 40980008  bge cr6, 0x82410fbc
	if !ctx.cr[6].lt {
	pc = 0x82410FBC; continue 'dispatch;
	}
	pc = 0x82410FB8; continue 'dispatch;
            }
            0x82410FB8 => {
    //   block [0x82410FB8..0x82410FBC)
	// 82410FB8: 48000000  b 0x82410fb8
	pc = 0x82410FB8; continue 'dispatch;
            }
            0x82410FBC => {
    //   block [0x82410FBC..0x82410FC4)
	// 82410FBC: 2F09000C  cmpwi cr6, r9, 0xc
	ctx.cr[6].compare_i32(ctx.r[9].s32, 12, &mut ctx.xer);
	// 82410FC0: 41980008  blt cr6, 0x82410fc8
	if ctx.cr[6].lt {
	pc = 0x82410FC8; continue 'dispatch;
	}
	pc = 0x82410FC4; continue 'dispatch;
            }
            0x82410FC4 => {
    //   block [0x82410FC4..0x82410FC8)
	// 82410FC4: 48000000  b 0x82410fc4
	pc = 0x82410FC4; continue 'dispatch;
            }
            0x82410FC8 => {
    //   block [0x82410FC8..0x82410FEC)
	// 82410FC8: 39690001  addi r11, r9, 1
	ctx.r[11].s64 = ctx.r[9].s64 + 1;
	// 82410FCC: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82410FD0: 392B0001  addi r9, r11, 1
	ctx.r[9].s64 = ctx.r[11].s64 + 1;
	// 82410FD4: 3BC50004  addi r30, r5, 4
	ctx.r[30].s64 = ctx.r[5].s64 + 4;
	// 82410FD8: 5529103A  slwi r9, r9, 2
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82410FDC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82410FE0: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 82410FE4: 3B8B3920  addi r28, r11, 0x3920
	ctx.r[28].s64 = ctx.r[11].s64 + 14624;
	// 82410FE8: 7D49F92E  stwx r10, r9, r31
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[31].u32), ctx.r[10].u32) };
	pc = 0x82410FEC; continue 'dispatch;
            }
            0x82410FEC => {
    //   block [0x82410FEC..0x82411028)
	// 82410FEC: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82410FF0: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82410FF4: 41820034  beq 0x82411028
	if ctx.cr[0].eq {
	pc = 0x82411028; continue 'dispatch;
	}
	// 82410FF8: 1D6B0038  mulli r11, r11, 0x38
	ctx.r[11].s32 = ((ctx.r[11].s32 as i64 * 56 as i64) as i32);
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 82410FFC: 7CABE214  add r5, r11, r28
	ctx.r[5].u64 = ctx.r[11].u64 + ctx.r[28].u64;
	// 82411000: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 82411004: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82411008: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 8241100C: 4BFFFF15  bl 0x82410f20
	ctx.lr = 0x82411010;
	sub_82410F20(ctx, base);
	// 82411010: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82411014: 40820024  bne 0x82411038
	if !ctx.cr[0].eq {
	pc = 0x82411038; continue 'dispatch;
	}
	// 82411018: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 8241101C: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82411020: 2B1D000D  cmplwi cr6, r29, 0xd
	ctx.cr[6].compare_u32(ctx.r[29].u32, 13 as u32, &mut ctx.xer);
	// 82411024: 4198FFC8  blt cr6, 0x82410fec
	if ctx.cr[6].lt {
	pc = 0x82410FEC; continue 'dispatch;
	}
	pc = 0x82411028; continue 'dispatch;
            }
            0x82411028 => {
    //   block [0x82411028..0x82411034)
	// 82411028: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241102C: 2C0B0000  cmpwi r11, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82411030: 40800010  bge 0x82411040
	if !ctx.cr[0].lt {
	pc = 0x82411040; continue 'dispatch;
	}
	pc = 0x82411034; continue 'dispatch;
            }
            0x82411034 => {
    //   block [0x82411034..0x82411038)
	// 82411034: 48000000  b 0x82411034
	pc = 0x82411034; continue 'dispatch;
            }
            0x82411038 => {
    //   block [0x82411038..0x82411040)
	// 82411038: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8241103C: 48000030  b 0x8241106c
	pc = 0x8241106C; continue 'dispatch;
            }
            0x82411040 => {
    //   block [0x82411040..0x82411048)
	// 82411040: 2F0B000D  cmpwi cr6, r11, 0xd
	ctx.cr[6].compare_i32(ctx.r[11].s32, 13, &mut ctx.xer);
	// 82411044: 41980008  blt cr6, 0x8241104c
	if ctx.cr[6].lt {
	pc = 0x8241104C; continue 'dispatch;
	}
	pc = 0x82411048; continue 'dispatch;
            }
            0x82411048 => {
    //   block [0x82411048..0x8241104C)
	// 82411048: 48000000  b 0x82411048
	pc = 0x82411048; continue 'dispatch;
            }
            0x8241104C => {
    //   block [0x8241104C..0x82411068)
	// 8241104C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82411050: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82411054: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82411058: 7D4BF92E  stwx r10, r11, r31
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32), ctx.r[10].u32) };
	// 8241105C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82411060: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82411064: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	pc = 0x82411068; continue 'dispatch;
            }
            0x82411068 => {
    //   block [0x82411068..0x8241106C)
	// 82411068: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	pc = 0x8241106C; continue 'dispatch;
            }
            0x8241106C => {
    //   block [0x8241106C..0x82411074)
	// 8241106C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82411070: 48124090  b 0x82535100
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82411078(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82411078 size=20
    let mut pc: u32 = 0x82411078;
    'dispatch: loop {
        match pc {
            0x82411078 => {
    //   block [0x82411078..0x82411088)
	// 82411078: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8241107C: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82411080: 2C0B0000  cmpwi r11, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82411084: 40800008  bge 0x8241108c
	if !ctx.cr[0].lt {
		crate::recompiler::externs::call(ctx, base, 0x8241108C);
		return;
	}
	pc = 0x82411088; continue 'dispatch;
            }
            0x82411088 => {
    //   block [0x82411088..0x8241108C)
	// 82411088: 48000000  b 0x82411088
	pc = 0x82411088; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


