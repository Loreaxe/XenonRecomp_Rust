pub fn sub_82507100(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82507100 size=100
    let mut pc: u32 = 0x82507100;
    'dispatch: loop {
        match pc {
            0x82507100 => {
    //   block [0x82507100..0x82507164)
	// 82507100: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82507104: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82507108: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8250710C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82507110: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82507114: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82507118: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8250711C: 4BFFF38D  bl 0x825064a8
	ctx.lr = 0x82507120;
	sub_825064A8(ctx, base);
	// 82507120: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82507124: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82507128: 419A0020  beq cr6, 0x82507148
	if ctx.cr[6].eq {
	pc = 0x82507148; continue 'dispatch;
	}
	// 8250712C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82507130: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 82507134: 38C00027  li r6, 0x27
	ctx.r[6].s64 = 39;
	// 82507138: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8250713C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82507140: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82507144: 4BF5CF75  bl 0x824640b8
	ctx.lr = 0x82507148;
	sub_824640B8(ctx, base);
	// 82507148: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8250714C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82507150: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82507154: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82507158: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8250715C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82507160: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82507168(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82507168 size=316
    let mut pc: u32 = 0x82507168;
    'dispatch: loop {
        match pc {
            0x82507168 => {
    //   block [0x82507168..0x825072A4)
	// 82507168: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250716C: 4802DF4D  bl 0x825350b8
	ctx.lr = 0x82507170;
	sub_82535080(ctx, base);
	// 82507170: DBE1FFD0  stfd f31, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[31].u64 ) };
	// 82507174: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82507178: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8250717C: 3D608283  lis r11, -0x7d7d
	ctx.r[11].s64 = -2105344000;
	// 82507180: 38C10060  addi r6, r1, 0x60
	ctx.r[6].s64 = ctx.r[1].s64 + 96;
	// 82507184: 3B8B2690  addi r28, r11, 0x2690
	ctx.r[28].s64 = ctx.r[11].s64 + 9872;
	// 82507188: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8250718C: 815F0018  lwz r10, 0x18(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82507190: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82507194: C3EB1FF8  lfs f31, 0x1ff8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82507198: 806A0000  lwz r3, 0(r10)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250719C: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 825071A0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825071A4: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 825071A8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825071AC: 4E800421  bctrl
	ctx.lr = 0x825071B0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825071B0: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 825071B4: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 825071B8: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 825071BC: 4099007C  ble cr6, 0x82507238
	if !ctx.cr[6].gt {
	pc = 0x82507238; continue 'dispatch;
	}
	// 825071C0: 3BA00010  li r29, 0x10
	ctx.r[29].s64 = 16;
	// 825071C4: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 825071C8: 38C10080  addi r6, r1, 0x80
	ctx.r[6].s64 = ctx.r[1].s64 + 128;
	// 825071CC: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 825071D0: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 825071D4: 7C7D582E  lwzx r3, r29, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 825071D8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825071DC: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 825071E0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825071E4: 4E800421  bctrl
	ctx.lr = 0x825071E8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825071E8: 39410070  addi r10, r1, 0x70
	ctx.r[10].s64 = ctx.r[1].s64 + 112;
	// 825071EC: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 825071F0: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 825071F4: 3BBD0010  addi r29, r29, 0x10
	ctx.r[29].s64 = ctx.r[29].s64 + 16;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825072A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825072A8 size=16
    let mut pc: u32 = 0x825072A8;
    'dispatch: loop {
        match pc {
            0x825072A8 => {
    //   block [0x825072A8..0x825072B8)
	// 825072A8: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 825072AC: 548A2036  slwi r10, r4, 4
	ctx.r[10].u32 = ctx.r[4].u32.wrapping_shl(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 825072B0: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 825072B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825072B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825072B8 size=20
    let mut pc: u32 = 0x825072B8;
    'dispatch: loop {
        match pc {
            0x825072B8 => {
    //   block [0x825072B8..0x825072CC)
	// 825072B8: 81430008  lwz r10, 8(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 825072BC: 548B2036  slwi r11, r4, 4
	ctx.r[11].u32 = ctx.r[4].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 825072C0: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 825072C4: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 825072C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825072D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825072D0 size=20
    let mut pc: u32 = 0x825072D0;
    'dispatch: loop {
        match pc {
            0x825072D0 => {
    //   block [0x825072D0..0x825072E4)
	// 825072D0: 81430018  lwz r10, 0x18(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 825072D4: 548B2036  slwi r11, r4, 4
	ctx.r[11].u32 = ctx.r[4].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 825072D8: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 825072DC: 90AB0004  stw r5, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 825072E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825072E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825072E8 size=192
    let mut pc: u32 = 0x825072E8;
    'dispatch: loop {
        match pc {
            0x825072E8 => {
    //   block [0x825072E8..0x825073A8)
	// 825072E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825072EC: 4802DDC9  bl 0x825350b4
	ctx.lr = 0x825072F0;
	sub_82535080(ctx, base);
	// 825072F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825072F4: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 825072F8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825072FC: 897C0000  lbz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82507300: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82507304: 419A001C  beq cr6, 0x82507320
	if ctx.cr[6].eq {
	pc = 0x82507320; continue 'dispatch;
	}
	// 82507308: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 8250730C: 2F0B00FC  cmpwi cr6, r11, 0xfc
	ctx.cr[6].compare_i32(ctx.r[11].s32, 252, &mut ctx.xer);
	// 82507310: 40990010  ble cr6, 0x82507320
	if !ctx.cr[6].gt {
	pc = 0x82507320; continue 'dispatch;
	}
	// 82507314: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82507318: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8250731C: 4802DDE8  b 0x82535104
	sub_825350D0(ctx, base);
	return;
	// 82507320: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82507324: 2F0B0FFF  cmpwi cr6, r11, 0xfff
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4095, &mut ctx.xer);
	// 82507328: 4098FFEC  bge cr6, 0x82507314
	if !ctx.cr[6].lt {
	pc = 0x82507314; continue 'dispatch;
	}
	// 8250732C: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82507330: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82507334: 40990068  ble cr6, 0x8250739c
	if !ctx.cr[6].gt {
	pc = 0x8250739C; continue 'dispatch;
	}
	// 82507338: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8250733C: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82507340: 38A00100  li r5, 0x100
	ctx.r[5].s64 = 256;
	// 82507344: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82507348: 7FABF214  add r29, r11, r30
	ctx.r[29].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 8250734C: 807D0000  lwz r3, 0(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82507350: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82507354: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82507358: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8250735C: 4E800421  bctrl
	ctx.lr = 0x82507360;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82507360: 907D0008  stw r3, 8(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(8 as u32), ctx.r[3].u32 ) };
	// 82507364: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82507368: 815F001C  lwz r10, 0x1c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 8250736C: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82507370: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 82507374: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82507378: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 8250737C: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82507380: 2F0BFFFF  cmpwi cr6, r11, -1
	ctx.cr[6].compare_i32(ctx.r[11].s32, -1, &mut ctx.xer);
	// 82507384: 419AFF90  beq cr6, 0x82507314
	if ctx.cr[6].eq {
	pc = 0x82507314; continue 'dispatch;
	}
	// 82507388: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 8250738C: 3B7B0001  addi r27, r27, 1
	ctx.r[27].s64 = ctx.r[27].s64 + 1;
	// 82507390: 3BDE0010  addi r30, r30, 0x10
	ctx.r[30].s64 = ctx.r[30].s64 + 16;
	// 82507394: 7F1B5800  cmpw cr6, r27, r11
	ctx.cr[6].compare_i32(ctx.r[27].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82507398: 4198FFA4  blt cr6, 0x8250733c
	if ctx.cr[6].lt {
	pc = 0x8250733C; continue 'dispatch;
	}
	// 8250739C: 38600070  li r3, 0x70
	ctx.r[3].s64 = 112;
	// 825073A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825073A4: 4802DD60  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825073A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825073A8 size=56
    let mut pc: u32 = 0x825073A8;
    'dispatch: loop {
        match pc {
            0x825073A8 => {
    //   block [0x825073A8..0x825073E0)
	// 825073A8: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 825073AC: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 825073B0: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 825073B4: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 825073B8: 396B2A44  addi r11, r11, 0x2a44
	ctx.r[11].s64 = ctx.r[11].s64 + 10820;
	// 825073BC: 394A4828  addi r10, r10, 0x4828
	ctx.r[10].s64 = ctx.r[10].s64 + 18472;
	// 825073C0: 39294804  addi r9, r9, 0x4804
	ctx.r[9].s64 = ctx.r[9].s64 + 18436;
	// 825073C4: 38E0000A  li r7, 0xa
	ctx.r[7].s64 = 10;
	// 825073C8: B1030006  sth r8, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[8].u16 ) };
	// 825073CC: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 825073D0: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 825073D4: 91230010  stw r9, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[9].u32 ) };
	// 825073D8: 90E3000C  stw r7, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[7].u32 ) };
	// 825073DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825073E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825073E0 size=220
    let mut pc: u32 = 0x825073E0;
    'dispatch: loop {
        match pc {
            0x825073E0 => {
    //   block [0x825073E0..0x825074BC)
	// 825073E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825073E4: 4802DCD9  bl 0x825350bc
	ctx.lr = 0x825073E8;
	sub_82535080(ctx, base);
	// 825073E8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825073EC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825073F0: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 825073F4: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 825073F8: 396B4828  addi r11, r11, 0x4828
	ctx.r[11].s64 = ctx.r[11].s64 + 18472;
	// 825073FC: 394A4804  addi r10, r10, 0x4804
	ctx.r[10].s64 = ctx.r[10].s64 + 18436;
	// 82507400: 813F001C  lwz r9, 0x1c(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82507404: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82507408: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8250740C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82507410: 915F0010  stw r10, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 82507414: 4099005C  ble cr6, 0x82507470
	if !ctx.cr[6].gt {
	pc = 0x82507470; continue 'dispatch;
	}
	// 82507418: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8250741C: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82507420: 7C7E582E  lwzx r3, r30, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82507424: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82507428: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8250742C: 419A0030  beq cr6, 0x8250745c
	if ctx.cr[6].eq {
	pc = 0x8250745C; continue 'dispatch;
	}
	// 82507430: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 82507434: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82507438: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 8250743C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82507440: B1630006  sth r11, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 82507444: 409A0018  bne cr6, 0x8250745c
	if !ctx.cr[6].eq {
	pc = 0x8250745C; continue 'dispatch;
	}
	// 82507448: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250744C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82507450: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82507454: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82507458: 4E800421  bctrl
	ctx.lr = 0x8250745C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8250745C: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82507460: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82507464: 3BDE0010  addi r30, r30, 0x10
	ctx.r[30].s64 = ctx.r[30].s64 + 16;
	// 82507468: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8250746C: 4198FFB0  blt cr6, 0x8250741c
	if ctx.cr[6].lt {
	pc = 0x8250741C; continue 'dispatch;
	}
	// 82507470: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82507474: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82507478: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8250747C: 409A0020  bne cr6, 0x8250749c
	if !ctx.cr[6].eq {
	pc = 0x8250749C; continue 'dispatch;
	}
	// 82507480: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82507484: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 82507488: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 8250748C: 809F0018  lwz r4, 0x18(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82507490: 55652036  slwi r5, r11, 4
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82507494: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82507498: 4BF5CC21  bl 0x824640b8
	ctx.lr = 0x8250749C;
	sub_824640B8(ctx, base);
	// 8250749C: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 825074A0: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 825074A4: 396B2A44  addi r11, r11, 0x2a44
	ctx.r[11].s64 = ctx.r[11].s64 + 10820;
	// 825074A8: 394A6DD0  addi r10, r10, 0x6dd0
	ctx.r[10].s64 = ctx.r[10].s64 + 28112;
	// 825074AC: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 825074B0: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 825074B4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825074B8: 4802DC54  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825074C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825074C0 size=252
    let mut pc: u32 = 0x825074C0;
    'dispatch: loop {
        match pc {
            0x825074C0 => {
    //   block [0x825074C0..0x825075BC)
	// 825074C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825074C4: 4802DBF1  bl 0x825350b4
	ctx.lr = 0x825074C8;
	sub_82535080(ctx, base);
	// 825074C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825074CC: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 825074D0: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 825074D4: 3BFB0018  addi r31, r27, 0x18
	ctx.r[31].s64 = ctx.r[27].s64 + 24;
	// 825074D8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825074DC: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 825074E0: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 825074E4: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 825074E8: 7F0BE800  cmpw cr6, r11, r29
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[29].s32, &mut ctx.xer);
	// 825074EC: 40980024  bge cr6, 0x82507510
	if !ctx.cr[6].lt {
	pc = 0x82507510; continue 'dispatch;
	}
	// 825074F0: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 825074F4: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 825074F8: 41980008  blt cr6, 0x82507500
	if ctx.cr[6].lt {
	pc = 0x82507500; continue 'dispatch;
	}
	// 825074FC: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 82507500: 38A00010  li r5, 0x10
	ctx.r[5].s64 = 16;
	// 82507504: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82507508: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8250750C: 4BF66DBD  bl 0x8246e2c8
	ctx.lr = 0x82507510;
	sub_8246E2C8(ctx, base);
	// 82507510: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82507514: 93BF0004  stw r29, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82507518: 40990094  ble cr6, 0x825075ac
	if !ctx.cr[6].gt {
	pc = 0x825075AC; continue 'dispatch;
	}
	// 8250751C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82507520: 7FC8F378  mr r8, r30
	ctx.r[8].u64 = ctx.r[30].u64;
	// 82507524: 7CCB3378  mr r11, r6
	ctx.r[11].u64 = ctx.r[6].u64;
	// 82507528: 7CBEE050  subf r5, r30, r28
	ctx.r[5].s64 = ctx.r[28].s64 - ctx.r[30].s64;
	// 8250752C: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 82507530: 81480000  lwz r10, 0(r8)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 82507534: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82507538: 419A0060  beq cr6, 0x82507598
	if ctx.cr[6].eq {
	pc = 0x82507598; continue 'dispatch;
	}
	// 8250753C: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82507540: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 82507544: 7D4B492E  stwx r10, r11, r9
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32), ctx.r[10].u32) };
	// 82507548: 419A000C  beq cr6, 0x82507554
	if ctx.cr[6].eq {
	pc = 0x82507554; continue 'dispatch;
	}
	// 8250754C: 7D25402E  lwzx r9, r5, r8
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[5].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 82507550: 48000008  b 0x82507558
	pc = 0x82507558; continue 'dispatch;
	// 82507554: 7CC93378  mr r9, r6
	ctx.r[9].u64 = ctx.r[6].u64;
	// 82507558: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250755C: 7D4B5214  add r10, r11, r10
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82507560: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82507564: 81480000  lwz r10, 0(r8)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 82507568: A12A0004  lhz r9, 4(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 8250756C: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82507570: 419A0010  beq cr6, 0x82507580
	if ctx.cr[6].eq {
	pc = 0x82507580; continue 'dispatch;
	}
	// 82507574: A12A0006  lhz r9, 6(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(6 as u32) ) } as u64;
	// 82507578: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 8250757C: B12A0006  sth r9, 6(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 82507580: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82507584: 7D4B5214  add r10, r11, r10
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82507588: 93AA000C  stw r29, 0xc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(12 as u32), ctx.r[29].u32 ) };
	// 8250758C: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82507590: 7D4B5214  add r10, r11, r10
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82507594: 90CA0008  stw r6, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[6].u32 ) };
	// 82507598: 38E7FFFF  addi r7, r7, -1
	ctx.r[7].s64 = ctx.r[7].s64 + -1;
	// 8250759C: 39080004  addi r8, r8, 4
	ctx.r[8].s64 = ctx.r[8].s64 + 4;
	// 825075A0: 396B0010  addi r11, r11, 0x10
	ctx.r[11].s64 = ctx.r[11].s64 + 16;
	// 825075A4: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 825075A8: 409AFF88  bne cr6, 0x82507530
	if !ctx.cr[6].eq {
	pc = 0x82507530; continue 'dispatch;
	}
	// 825075AC: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 825075B0: 4BFFFBB9  bl 0x82507168
	ctx.lr = 0x825075B4;
	sub_82507168(ctx, base);
	// 825075B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825075B8: 4802DB4C  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825075C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825075C0 size=536
    let mut pc: u32 = 0x825075C0;
    'dispatch: loop {
        match pc {
            0x825075C0 => {
    //   block [0x825075C0..0x825077D8)
	// 825075C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825075C4: 4802DAD9  bl 0x8253509c
	ctx.lr = 0x825075C8;
	sub_82535080(ctx, base);
	// 825075C8: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825075CC: 82ED0000  lwz r23, 0(r13)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 825075D0: 3B000014  li r24, 0x14
	ctx.r[24].s64 = 20;
	// 825075D4: 7C751B78  mr r21, r3
	ctx.r[21].u64 = ctx.r[3].u64;
	// 825075D8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 825075DC: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 825075E0: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 825075E4: 7D78B82E  lwzx r11, r24, r23
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[24].u32.wrapping_add(ctx.r[23].u32)) } as u64;
	// 825075E8: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 825075EC: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 825075F0: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 825075F4: 40980020  bge cr6, 0x82507614
	if !ctx.cr[6].lt {
	pc = 0x82507614; continue 'dispatch;
	}
	// 825075F8: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 825075FC: 3929484C  addi r9, r9, 0x484c
	ctx.r[9].s64 = ctx.r[9].s64 + 18508;
	// 82507600: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82507604: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82507608: 390A000C  addi r8, r10, 0xc
	ctx.r[8].s64 = ctx.r[10].s64 + 12;
	// 8250760C: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82507610: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82507614: 817A0040  lwz r11, 0x40(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(64 as u32) ) } as u64;
	// 82507618: 3AC0FFFF  li r22, -1
	ctx.r[22].s64 = -1;
	// 8250761C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82507620: 7ED9B378  mr r25, r22
	ctx.r[25].u64 = ctx.r[22].u64;
	// 82507624: 917A0040  stw r11, 0x40(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(64 as u32), ctx.r[11].u32 ) };
	// 82507628: 817C0024  lwz r11, 0x24(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(36 as u32) ) } as u64;
	// 8250762C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82507630: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82507634: 409A0084  bne cr6, 0x825076b8
	if !ctx.cr[6].eq {
	pc = 0x825076B8; continue 'dispatch;
	}
	// 82507638: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 8250763C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82507640: 40990128  ble cr6, 0x82507768
	if !ctx.cr[6].gt {
	pc = 0x82507768; continue 'dispatch;
	}
	// 82507644: 3B7F0010  addi r27, r31, 0x10
	ctx.r[27].s64 = ctx.r[31].s64 + 16;
	// 82507648: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8250764C: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82507650: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82507654: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82507658: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 8250765C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82507660: 4E800421  bctrl
	ctx.lr = 0x82507664;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82507664: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82507668: 419A0038  beq cr6, 0x825076a0
	if ctx.cr[6].eq {
	pc = 0x825076A0; continue 'dispatch;
	}
	// 8250766C: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82507670: 7F46D378  mr r6, r26
	ctx.r[6].u64 = ctx.r[26].u64;
	// 82507674: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82507678: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8250767C: 7C8BF02E  lwzx r4, r11, r30
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82507680: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82507684: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82507688: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8250768C: 4E800421  bctrl
	ctx.lr = 0x82507690;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82507690: 89610050  lbz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82507694: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82507698: 419A0008  beq cr6, 0x825076a0
	if ctx.cr[6].eq {
	pc = 0x825076A0; continue 'dispatch;
	}
	// 8250769C: 7FB9EB78  mr r25, r29
	ctx.r[25].u64 = ctx.r[29].u64;
	// 825076A0: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 825076A4: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 825076A8: 3BDE0010  addi r30, r30, 0x10
	ctx.r[30].s64 = ctx.r[30].s64 + 16;
	// 825076AC: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 825076B0: 4198FF9C  blt cr6, 0x8250764c
	if ctx.cr[6].lt {
	pc = 0x8250764C; continue 'dispatch;
	}
	// 825076B4: 480000B4  b 0x82507768
	pc = 0x82507768; continue 'dispatch;
	// 825076B8: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 825076BC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825076C0: 409900A8  ble cr6, 0x82507768
	if !ctx.cr[6].gt {
	pc = 0x82507768; continue 'dispatch;
	}
	// 825076C4: 3B7F0010  addi r27, r31, 0x10
	ctx.r[27].s64 = ctx.r[31].s64 + 16;
	// 825076C8: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 825076CC: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 825076D0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 825076D4: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 825076D8: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 825076DC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825076E0: 4E800421  bctrl
	ctx.lr = 0x825076E4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825076E4: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 825076E8: 419A006C  beq cr6, 0x82507754
	if ctx.cr[6].eq {
	pc = 0x82507754; continue 'dispatch;
	}
	// 825076EC: 809C0024  lwz r4, 0x24(r28)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(36 as u32) ) } as u64;
	// 825076F0: 7FC8F378  mr r8, r30
	ctx.r[8].u64 = ctx.r[30].u64;
	// 825076F4: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 825076F8: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 825076FC: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82507700: 38610052  addi r3, r1, 0x52
	ctx.r[3].s64 = ctx.r[1].s64 + 82;
	// 82507704: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82507708: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250770C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82507710: 4E800421  bctrl
	ctx.lr = 0x82507714;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82507714: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82507718: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8250771C: 419A0038  beq cr6, 0x82507754
	if ctx.cr[6].eq {
	pc = 0x82507754; continue 'dispatch;
	}
	// 82507720: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82507724: 7F46D378  mr r6, r26
	ctx.r[6].u64 = ctx.r[26].u64;
	// 82507728: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 8250772C: 38610051  addi r3, r1, 0x51
	ctx.r[3].s64 = ctx.r[1].s64 + 81;
	// 82507730: 7C8BE82E  lwzx r4, r11, r29
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 82507734: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82507738: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 8250773C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82507740: 4E800421  bctrl
	ctx.lr = 0x82507744;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82507744: 89610051  lbz r11, 0x51(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(81 as u32) ) } as u64;
	// 82507748: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8250774C: 419A0008  beq cr6, 0x82507754
	if ctx.cr[6].eq {
	pc = 0x82507754; continue 'dispatch;
	}
	// 82507750: 7FD9F378  mr r25, r30
	ctx.r[25].u64 = ctx.r[30].u64;
	// 82507754: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82507758: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 8250775C: 3BBD0010  addi r29, r29, 0x10
	ctx.r[29].s64 = ctx.r[29].s64 + 16;
	// 82507760: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82507764: 4198FF68  blt cr6, 0x825076cc
	if ctx.cr[6].lt {
	pc = 0x825076CC; continue 'dispatch;
	}
	// 82507768: 817A0040  lwz r11, 0x40(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(64 as u32) ) } as u64;
	// 8250776C: 2F19FFFF  cmpwi cr6, r25, -1
	ctx.cr[6].compare_i32(ctx.r[25].s32, -1, &mut ctx.xer);
	// 82507770: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82507774: 917A0040  stw r11, 0x40(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(64 as u32), ctx.r[11].u32 ) };
	// 82507778: 419A0010  beq cr6, 0x82507788
	if ctx.cr[6].eq {
	pc = 0x82507788; continue 'dispatch;
	}
	// 8250777C: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82507780: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82507784: 7F2BD12E  stwx r25, r11, r26
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[26].u32), ctx.r[25].u32) };
	// 82507788: 7D58B82E  lwzx r10, r24, r23
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[24].u32.wrapping_add(ctx.r[23].u32)) } as u64;
	// 8250778C: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82507790: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82507794: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82507798: 40980020  bge cr6, 0x825077b8
	if !ctx.cr[6].lt {
	pc = 0x825077B8; continue 'dispatch;
	}
	// 8250779C: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 825077A0: 392974DC  addi r9, r9, 0x74dc
	ctx.r[9].s64 = ctx.r[9].s64 + 29916;
	// 825077A4: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 825077A8: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 825077AC: 390B000C  addi r8, r11, 0xc
	ctx.r[8].s64 = ctx.r[11].s64 + 12;
	// 825077B0: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 825077B4: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 825077B8: 7D79B050  subf r11, r25, r22
	ctx.r[11].s64 = ctx.r[22].s64 - ctx.r[25].s64;
	// 825077BC: 7EA3AB78  mr r3, r21
	ctx.r[3].u64 = ctx.r[21].u64;
	// 825077C0: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 825077C4: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 825077C8: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 825077CC: 99750000  stb r11, 0(r21)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[21].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 825077D0: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 825077D4: 4802D918  b 0x825350ec
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825077D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825077D8 size=260
    let mut pc: u32 = 0x825077D8;
    'dispatch: loop {
        match pc {
            0x825077D8 => {
    //   block [0x825077D8..0x825078DC)
	// 825077D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825077DC: 4802D8D5  bl 0x825350b0
	ctx.lr = 0x825077E0;
	sub_82535080(ctx, base);
	// 825077E0: DBE1FFC0  stfd f31, -0x40(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-64 as u32), ctx.f[31].u64 ) };
	// 825077E4: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825077E8: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 825077EC: 11BF038C  vspltisw v13, -1
	for i in 0..4 {
		ctx.v[13].u32[i] = 4294967295;
	}
	// 825077F0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 825077F4: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 825077F8: 39400030  li r10, 0x30
	ctx.r[10].s64 = 48;
	// 825077FC: 396B9F50  addi r11, r11, -0x60b0
	ctx.r[11].s64 = ctx.r[11].s64 + -24752;
	// 82507800: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 82507804: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825078E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825078E0 size=1996
    let mut pc: u32 = 0x825078E0;
    'dispatch: loop {
        match pc {
            0x825078E0 => {
    //   block [0x825078E0..0x825080AC)
	// 825078E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825078E4: 4802D79D  bl 0x82535080
	ctx.lr = 0x825078E8;
	sub_82535080(ctx, base);
	// 825078E8: DBC1FF58  stfd f30, -0xa8(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-168 as u32), ctx.f[30].u64 ) };
	// 825078EC: DBE1FF60  stfd f31, -0xa0(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-160 as u32), ctx.f[31].u64 ) };
	// 825078F0: 3981FF50  addi r12, r1, -0xb0
	ctx.r[12].s64 = ctx.r[1].s64 + -176;
	// 825078F4: 48031AB1  bl 0x825393a4
	ctx.lr = 0x825078F8;
	sub_82539130(ctx, base);
	// 825078F8: 9421FC10  stwu r1, -0x3f0(r1)
	ea = ctx.r[1].u32.wrapping_add(-1008 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825078FC: 39010060  addi r8, r1, 0x60
	ctx.r[8].s64 = ctx.r[1].s64 + 96;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825080B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x825080B0 size=1004
    let mut pc: u32 = 0x825080B0;
    'dispatch: loop {
        match pc {
            0x825080B0 => {
    //   block [0x825080B0..0x8250849C)
	// 825080B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825080B4: 4802CFCD  bl 0x82535080
	ctx.lr = 0x825080B8;
	sub_82535080(ctx, base);
	// 825080B8: DBC1FF58  stfd f30, -0xa8(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-168 as u32), ctx.f[30].u64 ) };
	// 825080BC: DBE1FF60  stfd f31, -0xa0(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-160 as u32), ctx.f[31].u64 ) };
	// 825080C0: 9421FC90  stwu r1, -0x370(r1)
	ea = ctx.r[1].u32.wrapping_add(-880 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825080C4: 7D575378  mr r23, r10
	ctx.r[23].u64 = ctx.r[10].u64;
	// 825080C8: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 825080CC: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 825080D0: 7D3C4B78  mr r28, r9
	ctx.r[28].u64 = ctx.r[9].u64;
	// 825080D4: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 825080D8: 3B5C0010  addi r26, r28, 0x10
	ctx.r[26].s64 = ctx.r[28].s64 + 16;
	// 825080DC: 3A4B0010  addi r18, r11, 0x10
	ctx.r[18].s64 = ctx.r[11].s64 + 16;
	// 825080E0: C00A8CB4  lfs f0, -0x734c(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-29516 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825080E4: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 825080E8: D01C0000  stfs f0, 0(r28)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 825080EC: 7C962378  mr r22, r4
	ctx.r[22].u64 = ctx.r[4].u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825084A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825084A0 size=232
    let mut pc: u32 = 0x825084A0;
    'dispatch: loop {
        match pc {
            0x825084A0 => {
    //   block [0x825084A0..0x82508588)
	// 825084A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825084A4: 4802CC11  bl 0x825350b4
	ctx.lr = 0x825084A8;
	sub_82535080(ctx, base);
	// 825084A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825084AC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825084B0: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 825084B4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825084B8: 388B4870  addi r4, r11, 0x4870
	ctx.r[4].s64 = ctx.r[11].s64 + 18544;
	// 825084BC: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 825084C0: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 825084C4: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 825084C8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825084CC: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 825084D0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825084D4: 4E800421  bctrl
	ctx.lr = 0x825084D8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825084D8: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 825084DC: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 825084E0: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 825084E4: 409A0034  bne cr6, 0x82508518
	if !ctx.cr[6].eq {
	pc = 0x82508518; continue 'dispatch;
	}
	// 825084E8: 813E0000  lwz r9, 0(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 825084EC: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 825084F0: 80FF001C  lwz r7, 0x1c(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 825084F4: 55682036  slwi r8, r11, 4
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 825084F8: 80DF0018  lwz r6, 0x18(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 825084FC: 388A4864  addi r4, r10, 0x4864
	ctx.r[4].s64 = ctx.r[10].s64 + 18532;
	// 82508500: 54E72036  slwi r7, r7, 4
	ctx.r[7].u32 = ctx.r[7].u32.wrapping_shl(4);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82508504: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82508508: 81690008  lwz r11, 8(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250850C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82508510: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82508514: 4E800421  bctrl
	ctx.lr = 0x82508518;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82508518: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 8250851C: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82508520: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82508524: 40990048  ble cr6, 0x8250856c
	if !ctx.cr[6].gt {
	pc = 0x8250856C; continue 'dispatch;
	}
	// 82508528: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8250852C: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82508530: 3B6B485C  addi r27, r11, 0x485c
	ctx.r[27].s64 = ctx.r[11].s64 + 18524;
	// 82508534: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82508538: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8250853C: 815F0018  lwz r10, 0x18(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82508540: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82508544: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82508548: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8250854C: 7CCAE82E  lwzx r6, r10, r29
	ctx.r[6].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 82508550: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82508554: 4E800421  bctrl
	ctx.lr = 0x82508558;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82508558: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 8250855C: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 82508560: 3BBD0010  addi r29, r29, 0x10
	ctx.r[29].s64 = ctx.r[29].s64 + 16;
	// 82508564: 7F1C5800  cmpw cr6, r28, r11
	ctx.cr[6].compare_i32(ctx.r[28].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82508568: 4198FFCC  blt cr6, 0x82508534
	if ctx.cr[6].lt {
	pc = 0x82508534; continue 'dispatch;
	}
	// 8250856C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82508570: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82508574: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82508578: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8250857C: 4E800421  bctrl
	ctx.lr = 0x82508580;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82508580: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82508584: 4802CB80  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82508588(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82508588 size=192
    let mut pc: u32 = 0x82508588;
    'dispatch: loop {
        match pc {
            0x82508588 => {
    //   block [0x82508588..0x82508648)
	// 82508588: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250858C: 4802CB31  bl 0x825350bc
	ctx.lr = 0x82508590;
	sub_82535080(ctx, base);
	// 82508590: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82508594: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82508598: 3880000A  li r4, 0xa
	ctx.r[4].s64 = 10;
	// 8250859C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825085A0: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 825085A4: 48002D1D  bl 0x8250b2c0
	ctx.lr = 0x825085A8;
	sub_8250B2C0(ctx, base);
	// 825085A8: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 825085AC: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 825085B0: 396B4828  addi r11, r11, 0x4828
	ctx.r[11].s64 = ctx.r[11].s64 + 18472;
	// 825085B4: 3D008000  lis r8, -0x8000
	ctx.r[8].s64 = -2147483648;
	// 825085B8: 394A4804  addi r10, r10, 0x4804
	ctx.r[10].s64 = ctx.r[10].s64 + 18436;
	// 825085BC: 387F0018  addi r3, r31, 0x18
	ctx.r[3].s64 = ctx.r[31].s64 + 24;
	// 825085C0: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825085C4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825085C8: 550B003E  slwi r11, r8, 0
	ctx.r[11].u32 = ctx.r[8].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 825085CC: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 825085D0: 915F0010  stw r10, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 825085D4: 91030008  stw r8, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[8].u32 ) };
	// 825085D8: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 825085DC: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 825085E0: 91230004  stw r9, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 825085E4: 40980020  bge cr6, 0x82508604
	if !ctx.cr[6].lt {
	pc = 0x82508604; continue 'dispatch;
	}
	// 825085E8: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 825085EC: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 825085F0: 41990008  bgt cr6, 0x825085f8
	if ctx.cr[6].gt {
	pc = 0x825085F8; continue 'dispatch;
	}
	// 825085F4: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 825085F8: 38A00010  li r5, 0x10
	ctx.r[5].s64 = 16;
	// 825085FC: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82508600: 4BF65CC9  bl 0x8246e2c8
	ctx.lr = 0x82508604;
	sub_8246E2C8(ctx, base);
	// 82508604: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82508608: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8250860C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82508610: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82508614: 4BFFEEAD  bl 0x825074c0
	ctx.lr = 0x82508618;
	sub_825074C0(ctx, base);
	// 82508618: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8250861C: 4BFFEB4D  bl 0x82507168
	ctx.lr = 0x82508620;
	sub_82507168(ctx, base);
	// 82508620: 397F0050  addi r11, r31, 0x50
	ctx.r[11].s64 = ctx.r[31].s64 + 80;
	// 82508624: 3920FFFF  li r9, -1
	ctx.r[9].s64 = -1;
	// 82508628: 39400008  li r10, 8
	ctx.r[10].s64 = 8;
	// 8250862C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82508630: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82508634: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82508638: 4200FFF8  bdnz 0x82508630
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82508630; continue 'dispatch;
	}
	// 8250863C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82508640: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82508644: 4802CAC8  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82508648(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82508648 size=44
    let mut pc: u32 = 0x82508648;
    'dispatch: loop {
        match pc {
            0x82508648 => {
    //   block [0x82508648..0x82508674)
	// 82508648: 2B040100  cmplwi cr6, r4, 0x100
	ctx.cr[6].compare_u32(ctx.r[4].u32, 256 as u32, &mut ctx.xer);
	// 8250864C: 40980028  bge cr6, 0x82508674
	if !ctx.cr[6].lt {
		sub_82508674(ctx, base);
		return;
	}
	// 82508650: 548A06FE  clrlwi r10, r4, 0x1b
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x0000001Fu64;
	// 82508654: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82508658: 548BD97E  srwi r11, r4, 5
	ctx.r[11].u32 = ctx.r[4].u32.wrapping_shr(5);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8250865C: 396B0010  addi r11, r11, 0x10
	ctx.r[11].s64 = ctx.r[11].s64 + 16;
	// 82508660: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82508664: 7D6B182E  lwzx r11, r11, r3
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[3].u32)) } as u64;
	// 82508668: 7D2A5030  slw r10, r9, r10
	if (ctx.r[10].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[9].u32) << ((ctx.r[10].u8 & 0x1F) as u32)) as u64;
	}
	// 8250866C: 7D435838  and r3, r10, r11
	ctx.r[3].u64 = ctx.r[10].u64 & ctx.r[11].u64;
	// 82508670: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82508674(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82508674 size=8
    let mut pc: u32 = 0x82508674;
    'dispatch: loop {
        match pc {
            0x82508674 => {
    //   block [0x82508674..0x8250867C)
	// 82508674: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82508678: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82508680(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82508680 size=8
    let mut pc: u32 = 0x82508680;
    'dispatch: loop {
        match pc {
            0x82508680 => {
    //   block [0x82508680..0x82508688)
	// 82508680: 3863FFF0  addi r3, r3, -0x10
	ctx.r[3].s64 = ctx.r[3].s64 + -16;
	// 82508684: 48000024  b 0x825086a8
	sub_825086A8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82508688(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82508688 size=20
    let mut pc: u32 = 0x82508688;
    'dispatch: loop {
        match pc {
            0x82508688 => {
    //   block [0x82508688..0x8250869C)
	// 82508688: 8143000C  lwz r10, 0xc(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 8250868C: 39640001  addi r11, r4, 1
	ctx.r[11].s64 = ctx.r[4].s64 + 1;
	// 82508690: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82508694: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82508698: 4D980020  bltlr cr6
	if ctx.cr[6].lt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250869C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8250869C size=8
    let mut pc: u32 = 0x8250869C;
    'dispatch: loop {
        match pc {
            0x8250869C => {
    //   block [0x8250869C..0x825086A4)
	// 8250869C: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 825086A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825086A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825086A8 size=100
    let mut pc: u32 = 0x825086A8;
    'dispatch: loop {
        match pc {
            0x825086A8 => {
    //   block [0x825086A8..0x8250870C)
	// 825086A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825086AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825086B0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825086B4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825086B8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825086BC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825086C0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825086C4: 4BFFED1D  bl 0x825073e0
	ctx.lr = 0x825086C8;
	sub_825073E0(ctx, base);
	// 825086C8: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 825086CC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825086D0: 419A0020  beq cr6, 0x825086f0
	if ctx.cr[6].eq {
	pc = 0x825086F0; continue 'dispatch;
	}
	// 825086D4: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 825086D8: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 825086DC: 38C00027  li r6, 0x27
	ctx.r[6].s64 = 39;
	// 825086E0: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 825086E4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825086E8: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 825086EC: 4BF5B9CD  bl 0x824640b8
	ctx.lr = 0x825086F0;
	sub_824640B8(ctx, base);
	// 825086F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825086F4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825086F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825086FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82508700: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82508704: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82508708: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82508710(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82508710 size=24
    let mut pc: u32 = 0x82508710;
    'dispatch: loop {
        match pc {
            0x82508710 => {
    //   block [0x82508710..0x82508728)
	// 82508710: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 82508714: 419A0020  beq cr6, 0x82508734
	if ctx.cr[6].eq {
		sub_82508734(ctx, base);
		return;
	}
	// 82508718: 2F030002  cmpwi cr6, r3, 2
	ctx.cr[6].compare_i32(ctx.r[3].s32, 2, &mut ctx.xer);
	// 8250871C: 419A000C  beq cr6, 0x82508728
	if ctx.cr[6].eq {
		sub_82508728(ctx, base);
		return;
	}
	// 82508720: 5483103A  slwi r3, r4, 2
	ctx.r[3].u32 = ctx.r[4].u32.wrapping_shl(2);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82508724: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82508728(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82508728 size=12
    let mut pc: u32 = 0x82508728;
    'dispatch: loop {
        match pc {
            0x82508728 => {
    //   block [0x82508728..0x82508734)
	// 82508728: 548B083C  slwi r11, r4, 1
	ctx.r[11].u32 = ctx.r[4].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8250872C: 386B0001  addi r3, r11, 1
	ctx.r[3].s64 = ctx.r[11].s64 + 1;
	// 82508730: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82508734(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82508734 size=8
    let mut pc: u32 = 0x82508734;
    'dispatch: loop {
        match pc {
            0x82508734 => {
    //   block [0x82508734..0x8250873C)
	// 82508734: 38640002  addi r3, r4, 2
	ctx.r[3].s64 = ctx.r[4].s64 + 2;
	// 82508738: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82508740(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82508740 size=24
    let mut pc: u32 = 0x82508740;
    'dispatch: loop {
        match pc {
            0x82508740 => {
    //   block [0x82508740..0x82508758)
	// 82508740: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82508744: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82508748: 396B4880  addi r11, r11, 0x4880
	ctx.r[11].s64 = ctx.r[11].s64 + 18560;
	// 8250874C: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82508750: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82508754: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82508758(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82508758 size=104
    let mut pc: u32 = 0x82508758;
    'dispatch: loop {
        match pc {
            0x82508758 => {
    //   block [0x82508758..0x825087C0)
	// 82508758: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250875C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82508760: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82508764: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82508768: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8250876C: 4BFFC29D  bl 0x82504a08
	ctx.lr = 0x82508770;
	sub_82504A08(ctx, base);
	// 82508770: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82508774: 3D008000  lis r8, -0x8000
	ctx.r[8].s64 = -2147483648;
	// 82508778: 394B48B4  addi r10, r11, 0x48b4
	ctx.r[10].s64 = ctx.r[11].s64 + 18612;
	// 8250877C: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82508780: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82508784: 392B4890  addi r9, r11, 0x4890
	ctx.r[9].s64 = ctx.r[11].s64 + 18576;
	// 82508788: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8250878C: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82508790: 913F0010  stw r9, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[9].u32 ) };
	// 82508794: 917F00C0  stw r11, 0xc0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(192 as u32), ctx.r[11].u32 ) };
	// 82508798: 917F00C4  stw r11, 0xc4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(196 as u32), ctx.r[11].u32 ) };
	// 8250879C: 911F00C8  stw r8, 0xc8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(200 as u32), ctx.r[8].u32 ) };
	// 825087A0: 917F00CC  stw r11, 0xcc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(204 as u32), ctx.r[11].u32 ) };
	// 825087A4: 917F00D0  stw r11, 0xd0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(208 as u32), ctx.r[11].u32 ) };
	// 825087A8: 911F00D4  stw r8, 0xd4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(212 as u32), ctx.r[8].u32 ) };
	// 825087AC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825087B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825087B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825087B8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825087BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825087C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825087C0 size=512
    let mut pc: u32 = 0x825087C0;
    'dispatch: loop {
        match pc {
            0x825087C0 => {
    //   block [0x825087C0..0x825089C0)
	// 825087C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825087C4: 4802C8ED  bl 0x825350b0
	ctx.lr = 0x825087C8;
	sub_82535080(ctx, base);
	// 825087C8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825087CC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 825087D0: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 825087D4: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 825087D8: 396B48B4  addi r11, r11, 0x48b4
	ctx.r[11].s64 = ctx.r[11].s64 + 18612;
	// 825087DC: 394A4890  addi r10, r10, 0x4890
	ctx.r[10].s64 = ctx.r[10].s64 + 18576;
	// 825087E0: 813D00C4  lwz r9, 0xc4(r29)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(196 as u32) ) } as u64;
	// 825087E4: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 825087E8: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 825087EC: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825087F0: 915D0010  stw r10, 0x10(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 825087F4: 4099005C  ble cr6, 0x82508850
	if !ctx.cr[6].gt {
	pc = 0x82508850; continue 'dispatch;
	}
	// 825087F8: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 825087FC: 817D00C0  lwz r11, 0xc0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(192 as u32) ) } as u64;
	// 82508800: 7C6BF82E  lwzx r3, r11, r31
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82508804: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82508808: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8250880C: 419A0030  beq cr6, 0x8250883c
	if ctx.cr[6].eq {
	pc = 0x8250883C; continue 'dispatch;
	}
	// 82508810: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 82508814: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82508818: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 8250881C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82508820: B1630006  sth r11, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 82508824: 409A0018  bne cr6, 0x8250883c
	if !ctx.cr[6].eq {
	pc = 0x8250883C; continue 'dispatch;
	}
	// 82508828: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250882C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82508830: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82508834: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82508838: 4E800421  bctrl
	ctx.lr = 0x8250883C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8250883C: 817D00C4  lwz r11, 0xc4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(196 as u32) ) } as u64;
	// 82508840: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82508844: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 82508848: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8250884C: 4198FFB0  blt cr6, 0x825087fc
	if ctx.cr[6].lt {
	pc = 0x825087FC; continue 'dispatch;
	}
	// 82508850: 817D005C  lwz r11, 0x5c(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(92 as u32) ) } as u64;
	// 82508854: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 82508858: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8250885C: 4099008C  ble cr6, 0x825088e8
	if !ctx.cr[6].gt {
	pc = 0x825088E8; continue 'dispatch;
	}
	// 82508860: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82508864: 817D0058  lwz r11, 0x58(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(88 as u32) ) } as u64;
	// 82508868: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 8250886C: 7FFB5A14  add r31, r27, r11
	ctx.r[31].u64 = ctx.r[27].u64 + ctx.r[11].u64;
	// 82508870: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82508874: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82508878: 4099005C  ble cr6, 0x825088d4
	if !ctx.cr[6].gt {
	pc = 0x825088D4; continue 'dispatch;
	}
	// 8250887C: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82508880: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82508884: 7C7E582E  lwzx r3, r30, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82508888: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8250888C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82508890: 419A0030  beq cr6, 0x825088c0
	if ctx.cr[6].eq {
	pc = 0x825088C0; continue 'dispatch;
	}
	// 82508894: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 82508898: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8250889C: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 825088A0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825088A4: B1630006  sth r11, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 825088A8: 409A0018  bne cr6, 0x825088c0
	if !ctx.cr[6].eq {
	pc = 0x825088C0; continue 'dispatch;
	}
	// 825088AC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825088B0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 825088B4: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825088B8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825088BC: 4E800421  bctrl
	ctx.lr = 0x825088C0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825088C0: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 825088C4: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 825088C8: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 825088CC: 7F1C5800  cmpw cr6, r28, r11
	ctx.cr[6].compare_i32(ctx.r[28].s32, ctx.r[11].s32, &mut ctx.xer);
	// 825088D0: 4198FFB0  blt cr6, 0x82508880
	if ctx.cr[6].lt {
	pc = 0x82508880; continue 'dispatch;
	}
	// 825088D4: 817D005C  lwz r11, 0x5c(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(92 as u32) ) } as u64;
	// 825088D8: 3B5A0001  addi r26, r26, 1
	ctx.r[26].s64 = ctx.r[26].s64 + 1;
	// 825088DC: 3B7B0040  addi r27, r27, 0x40
	ctx.r[27].s64 = ctx.r[27].s64 + 64;
	// 825088E0: 7F1A5800  cmpw cr6, r26, r11
	ctx.cr[6].compare_i32(ctx.r[26].s32, ctx.r[11].s32, &mut ctx.xer);
	// 825088E4: 4198FF80  blt cr6, 0x82508864
	if ctx.cr[6].lt {
	pc = 0x82508864; continue 'dispatch;
	}
	// 825088E8: 817D00D0  lwz r11, 0xd0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(208 as u32) ) } as u64;
	// 825088EC: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 825088F0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825088F4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825088F8: 917D005C  stw r11, 0x5c(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 825088FC: 4099005C  ble cr6, 0x82508958
	if !ctx.cr[6].gt {
	pc = 0x82508958; continue 'dispatch;
	}
	// 82508900: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82508904: 817D00CC  lwz r11, 0xcc(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(204 as u32) ) } as u64;
	// 82508908: 7C7F582E  lwzx r3, r31, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8250890C: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82508910: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82508914: 419A0030  beq cr6, 0x82508944
	if ctx.cr[6].eq {
	pc = 0x82508944; continue 'dispatch;
	}
	// 82508918: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 8250891C: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82508920: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 82508924: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82508928: B1630006  sth r11, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 8250892C: 409A0018  bne cr6, 0x82508944
	if !ctx.cr[6].eq {
	pc = 0x82508944; continue 'dispatch;
	}
	// 82508930: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82508934: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82508938: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250893C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82508940: 4E800421  bctrl
	ctx.lr = 0x82508944;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82508944: 817D00D0  lwz r11, 0xd0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(208 as u32) ) } as u64;
	// 82508948: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 8250894C: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 82508950: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82508954: 4198FFB0  blt cr6, 0x82508904
	if ctx.cr[6].lt {
	pc = 0x82508904; continue 'dispatch;
	}
	// 82508958: 817D00D4  lwz r11, 0xd4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(212 as u32) ) } as u64;
	// 8250895C: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82508960: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82508964: 409A0020  bne cr6, 0x82508984
	if !ctx.cr[6].eq {
	pc = 0x82508984; continue 'dispatch;
	}
	// 82508968: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250896C: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 82508970: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82508974: 809D00CC  lwz r4, 0xcc(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(204 as u32) ) } as u64;
	// 82508978: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 8250897C: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82508980: 4BF5B739  bl 0x824640b8
	ctx.lr = 0x82508984;
	sub_824640B8(ctx, base);
	// 82508984: 817D00C8  lwz r11, 0xc8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(200 as u32) ) } as u64;
	// 82508988: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 8250898C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82508990: 409A0020  bne cr6, 0x825089b0
	if !ctx.cr[6].eq {
	pc = 0x825089B0; continue 'dispatch;
	}
	// 82508994: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82508998: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 8250899C: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 825089A0: 809D00C0  lwz r4, 0xc0(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(192 as u32) ) } as u64;
	// 825089A4: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 825089A8: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 825089AC: 4BF5B70D  bl 0x824640b8
	ctx.lr = 0x825089B0;
	sub_824640B8(ctx, base);
	// 825089B0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 825089B4: 4BFFC4BD  bl 0x82504e70
	ctx.lr = 0x825089B8;
	sub_82504E70(ctx, base);
	// 825089B8: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 825089BC: 4802C744  b 0x82535100
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825089C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825089C0 size=24
    let mut pc: u32 = 0x825089C0;
    'dispatch: loop {
        match pc {
            0x825089C0 => {
    //   block [0x825089C0..0x825089D8)
	// 825089C0: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 825089C4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825089C8: 396B48E4  addi r11, r11, 0x48e4
	ctx.r[11].s64 = ctx.r[11].s64 + 18660;
	// 825089CC: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 825089D0: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825089D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825089D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825089D8 size=396
    let mut pc: u32 = 0x825089D8;
    'dispatch: loop {
        match pc {
            0x825089D8 => {
    //   block [0x825089D8..0x82508B64)
	// 825089D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825089DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825089E0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825089E4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825089E8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825089EC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825089F0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825089F4: 4BFFC5ED  bl 0x82504fe0
	ctx.lr = 0x825089F8;
	sub_82504FE0(ctx, base);
	// 825089F8: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 825089FC: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 82508A00: 396B48B4  addi r11, r11, 0x48b4
	ctx.r[11].s64 = ctx.r[11].s64 + 18612;
	// 82508A04: 394A4890  addi r10, r10, 0x4890
	ctx.r[10].s64 = ctx.r[10].s64 + 18576;
	// 82508A08: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82508A0C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82508A10: 915F0010  stw r10, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 82508A14: 419A0134  beq cr6, 0x82508b48
	if ctx.cr[6].eq {
	pc = 0x82508B48; continue 'dispatch;
	}
	// 82508A18: 817F0054  lwz r11, 0x54(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82508A1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82508A20: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82508A24: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82508A28: 4099009C  ble cr6, 0x82508ac4
	if !ctx.cr[6].gt {
	pc = 0x82508AC4; continue 'dispatch;
	}
	// 82508A2C: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82508A30: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82508A34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82508A38: 38CB48E4  addi r6, r11, 0x48e4
	ctx.r[6].s64 = ctx.r[11].s64 + 18660;
	// 82508A3C: 815F00C0  lwz r10, 0xc0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(192 as u32) ) } as u64;
	// 82508A40: 817F0050  lwz r11, 0x50(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82508A44: 7D685A14  add r11, r8, r11
	ctx.r[11].u64 = ctx.r[8].u64 + ctx.r[11].u64;
	// 82508A48: 7D49502E  lwzx r10, r9, r10
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82508A4C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82508A50: 419A000C  beq cr6, 0x82508a5c
	if ctx.cr[6].eq {
	pc = 0x82508A5C; continue 'dispatch;
	}
	// 82508A54: B0AA0006  sth r5, 6(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(6 as u32), ctx.r[5].u16 ) };
	// 82508A58: 90CA0000  stw r6, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[6].u32 ) };
	// 82508A5C: 815F00C0  lwz r10, 0xc0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(192 as u32) ) } as u64;
	// 82508A60: 888B0038  lbz r4, 0x38(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) } as u64;
	// 82508A64: 2B040001  cmplwi cr6, r4, 1
	ctx.cr[6].compare_u32(ctx.r[4].u32, 1 as u32, &mut ctx.xer);
	// 82508A68: 7D49502E  lwzx r10, r9, r10
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82508A6C: 808A0008  lwz r4, 8(r10)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 82508A70: 908B0014  stw r4, 0x14(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[4].u32 ) };
	// 82508A74: 409A000C  bne cr6, 0x82508a80
	if !ctx.cr[6].eq {
	pc = 0x82508A80; continue 'dispatch;
	}
	// 82508A78: 808A0014  lwz r4, 0x14(r10)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 82508A7C: 48000008  b 0x82508a84
	pc = 0x82508A84; continue 'dispatch;
	// 82508A80: 808A0020  lwz r4, 0x20(r10)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(32 as u32) ) } as u64;
	// 82508A84: 908B0030  stw r4, 0x30(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(48 as u32), ctx.r[4].u32 ) };
	// 82508A88: 888B0001  lbz r4, 1(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(1 as u32) ) } as u64;
	// 82508A8C: 2B040001  cmplwi cr6, r4, 1
	ctx.cr[6].compare_u32(ctx.r[4].u32, 1 as u32, &mut ctx.xer);
	// 82508A90: 409A000C  bne cr6, 0x82508a9c
	if !ctx.cr[6].eq {
	pc = 0x82508A9C; continue 'dispatch;
	}
	// 82508A94: 808A002C  lwz r4, 0x2c(r10)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(44 as u32) ) } as u64;
	// 82508A98: 48000008  b 0x82508aa0
	pc = 0x82508AA0; continue 'dispatch;
	// 82508A9C: 808A0044  lwz r4, 0x44(r10)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(68 as u32) ) } as u64;
	// 82508AA0: 908B0004  stw r4, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 82508AA4: 38E70001  addi r7, r7, 1
	ctx.r[7].s64 = ctx.r[7].s64 + 1;
	// 82508AA8: 814A0038  lwz r10, 0x38(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(56 as u32) ) } as u64;
	// 82508AAC: 39080040  addi r8, r8, 0x40
	ctx.r[8].s64 = ctx.r[8].s64 + 64;
	// 82508AB0: 39290004  addi r9, r9, 4
	ctx.r[9].s64 = ctx.r[9].s64 + 4;
	// 82508AB4: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 82508AB8: 817F0054  lwz r11, 0x54(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82508ABC: 7F075800  cmpw cr6, r7, r11
	ctx.cr[6].compare_i32(ctx.r[7].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82508AC0: 4198FF7C  blt cr6, 0x82508a3c
	if ctx.cr[6].lt {
	pc = 0x82508A3C; continue 'dispatch;
	}
	// 82508AC4: 817F005C  lwz r11, 0x5c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 82508AC8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82508ACC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82508AD0: 40990078  ble cr6, 0x82508b48
	if !ctx.cr[6].gt {
	pc = 0x82508B48; continue 'dispatch;
	}
	// 82508AD4: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82508AD8: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82508ADC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82508AE0: 38CB4880  addi r6, r11, 0x4880
	ctx.r[6].s64 = ctx.r[11].s64 + 18560;
	// 82508AE4: 809F00CC  lwz r4, 0xcc(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(204 as u32) ) } as u64;
	// 82508AE8: 817F0058  lwz r11, 0x58(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 82508AEC: 7D485A14  add r10, r8, r11
	ctx.r[10].u64 = ctx.r[8].u64 + ctx.r[11].u64;
	// 82508AF0: 7D69202E  lwzx r11, r9, r4
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[4].u32)) } as u64;
	// 82508AF4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82508AF8: 419A000C  beq cr6, 0x82508b04
	if ctx.cr[6].eq {
	pc = 0x82508B04; continue 'dispatch;
	}
	// 82508AFC: B0AB0006  sth r5, 6(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(6 as u32), ctx.r[5].u16 ) };
	// 82508B00: 90CB0000  stw r6, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[6].u32 ) };
	// 82508B04: 817F00CC  lwz r11, 0xcc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(204 as u32) ) } as u64;
	// 82508B08: 888A0001  lbz r4, 1(r10)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(1 as u32) ) } as u64;
	// 82508B0C: 2B040001  cmplwi cr6, r4, 1
	ctx.cr[6].compare_u32(ctx.r[4].u32, 1 as u32, &mut ctx.xer);
	// 82508B10: 7D69582E  lwzx r11, r9, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82508B14: 409A000C  bne cr6, 0x82508b20
	if !ctx.cr[6].eq {
	pc = 0x82508B20; continue 'dispatch;
	}
	// 82508B18: 808B0014  lwz r4, 0x14(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82508B1C: 48000008  b 0x82508b24
	pc = 0x82508B24; continue 'dispatch;
	// 82508B20: 808B002C  lwz r4, 0x2c(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 82508B24: 908A0004  stw r4, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 82508B28: 38E70001  addi r7, r7, 1
	ctx.r[7].s64 = ctx.r[7].s64 + 1;
	// 82508B2C: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82508B30: 39080040  addi r8, r8, 0x40
	ctx.r[8].s64 = ctx.r[8].s64 + 64;
	// 82508B34: 39290004  addi r9, r9, 4
	ctx.r[9].s64 = ctx.r[9].s64 + 4;
	// 82508B38: 916A000C  stw r11, 0xc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82508B3C: 817F005C  lwz r11, 0x5c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 82508B40: 7F075800  cmpw cr6, r7, r11
	ctx.cr[6].compare_i32(ctx.r[7].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82508B44: 4198FFA0  blt cr6, 0x82508ae4
	if ctx.cr[6].lt {
	pc = 0x82508AE4; continue 'dispatch;
	}
	// 82508B48: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82508B4C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82508B50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82508B54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82508B58: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82508B5C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82508B60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82508B68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82508B68 size=1940
    let mut pc: u32 = 0x82508B68;
    'dispatch: loop {
        match pc {
            0x82508B68 => {
    //   block [0x82508B68..0x825092FC)
	// 82508B68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82508B6C: 4802C531  bl 0x8253509c
	ctx.lr = 0x82508B70;
	sub_82535080(ctx, base);
	// 82508B70: 9421FF00  stwu r1, -0x100(r1)
	ea = ctx.r[1].u32.wrapping_add(-256 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82508B74: 7C771B78  mr r23, r3
	ctx.r[23].u64 = ctx.r[3].u64;
	// 82508B78: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82508B7C: 4BFFCD45  bl 0x825058c0
	ctx.lr = 0x82508B80;
	sub_825058C0(ctx, base);
	// 82508B80: 39410080  addi r10, r1, 0x80
	ctx.r[10].s64 = ctx.r[1].s64 + 128;
	// 82508B84: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 82508B88: 3B000001  li r24, 1
	ctx.r[24].s64 = 1;
	// 82508B8C: 39210080  addi r9, r1, 0x80
	ctx.r[9].s64 = ctx.r[1].s64 + 128;
	// 82508B90: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82509300(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82509300 size=1268
    let mut pc: u32 = 0x82509300;
    'dispatch: loop {
        match pc {
            0x82509300 => {
    //   block [0x82509300..0x825097F4)
	// 82509300: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82509304: 4802BD9D  bl 0x825350a0
	ctx.lr = 0x82509308;
	sub_82535080(ctx, base);
	// 82509308: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250930C: 7C761B78  mr r22, r3
	ctx.r[22].u64 = ctx.r[3].u64;
	// 82509310: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82509314: 4BFFC66D  bl 0x82505980
	ctx.lr = 0x82509318;
	sub_82505980(ctx, base);
	// 82509318: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250931C: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 82509320: 38A00025  li r5, 0x25
	ctx.r[5].s64 = 37;
	// 82509324: 38800038  li r4, 0x38
	ctx.r[4].s64 = 56;
	// 82509328: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 8250932C: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82509330: 4BF5AD09  bl 0x82464038
	ctx.lr = 0x82509334;
	sub_82464038(ctx, base);
	// 82509334: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82509338: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8250933C: 394B4880  addi r10, r11, 0x4880
	ctx.r[10].s64 = ctx.r[11].s64 + 18560;
	// 82509340: 39200038  li r9, 0x38
	ctx.r[9].s64 = 56;
	// 82509344: 3AE00001  li r23, 1
	ctx.r[23].s64 = 1;
	// 82509348: 3B200000  li r25, 0
	ctx.r[25].s64 = 0;
	// 8250934C: 3D608000  lis r11, -0x8000
	ctx.r[11].s64 = -2147483648;
	// 82509350: 3BB600CC  addi r29, r22, 0xcc
	ctx.r[29].s64 = ctx.r[22].s64 + 204;
	// 82509354: 915E0000  stw r10, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82509358: B13E0004  sth r9, 4(r30)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[9].u16 ) };
	// 8250935C: B2FE0006  sth r23, 6(r30)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[30].u32.wrapping_add(6 as u32), ctx.r[23].u16 ) };
	// 82509360: 933E0008  stw r25, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[25].u32 ) };
	// 82509364: 917E0010  stw r11, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82509368: 933E000C  stw r25, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[25].u32 ) };
	// 8250936C: 917E001C  stw r11, 0x1c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82509370: 933E0014  stw r25, 0x14(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), ctx.r[25].u32 ) };
	// 82509374: 933E0018  stw r25, 0x18(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), ctx.r[25].u32 ) };
	// 82509378: 917E0028  stw r11, 0x28(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 8250937C: 933E0020  stw r25, 0x20(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(32 as u32), ctx.r[25].u32 ) };
	// 82509380: 933E0024  stw r25, 0x24(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(36 as u32), ctx.r[25].u32 ) };
	// 82509384: 917E0034  stw r11, 0x34(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 82509388: 933E002C  stw r25, 0x2c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(44 as u32), ctx.r[25].u32 ) };
	// 8250938C: 933E0030  stw r25, 0x30(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(48 as u32), ctx.r[25].u32 ) };
	// 82509390: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82509394: 815D0004  lwz r10, 4(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82509398: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 8250939C: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 825093A0: 409A0010  bne cr6, 0x825093b0
	if !ctx.cr[6].eq {
	pc = 0x825093B0; continue 'dispatch;
	}
	// 825093A4: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 825093A8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 825093AC: 4BF64FA5  bl 0x8246e350
	ctx.lr = 0x825093B0;
	sub_8246E350(ctx, base);
	// 825093B0: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 825093B4: 39000020  li r8, 0x20
	ctx.r[8].s64 = 32;
	// 825093B8: 815D0000  lwz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 825093BC: 39200030  li r9, 0x30
	ctx.r[9].s64 = 48;
	// 825093C0: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 825093C4: 7F3ACB78  mr r26, r25
	ctx.r[26].u64 = ctx.r[25].u64;
	// 825093C8: 7FCB512E  stwx r30, r11, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[30].u32) };
	// 825093CC: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 825093D0: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 825093D4: 917D0004  stw r11, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 825093D8: 556B003E  slwi r11, r11, 0
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 825093DC: 815D0000  lwz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 825093E0: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 825093E4: 80FF0000  lwz r7, 0(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825093E8: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 825093EC: 830BFFFC  lwz r24, -4(r11)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-4 as u32) ) } as u64;
	// 825093F0: 90FC0000  stw r7, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 825093F4: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 825093F8: 917C0004  stw r11, 4(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 825093FC: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82509400: 917C0008  stw r11, 8(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82509404: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82509408: 917C000C  stw r11, 0xc(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 8250940C: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82509410: 917C0010  stw r11, 0x10(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82509414: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82509418: 917C0014  stw r11, 0x14(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 8250941C: 897F0018  lbz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82509420: 997C0018  stb r11, 0x18(r28)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[28].u32.wrapping_add(24 as u32), ctx.r[11].u8 ) };
	// 82509424: 897F0019  lbz r11, 0x19(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(25 as u32) ) } as u64;
	// 82509428: 997C0019  stb r11, 0x19(r28)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[28].u32.wrapping_add(25 as u32), ctx.r[11].u8 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825097F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x825097F8 size=428
    let mut pc: u32 = 0x825097F8;
    'dispatch: loop {
        match pc {
            0x825097F8 => {
    //   block [0x825097F8..0x825099A4)
	// 825097F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825097FC: 4802B8B9  bl 0x825350b4
	ctx.lr = 0x82509800;
	sub_82535080(ctx, base);
	// 82509800: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82509804: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82509808: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8250980C: 80BE0018  lwz r5, 0x18(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 82509810: C03E00B0  lfs f1, 0xb0(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(176 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82509814: 4BFFB1F5  bl 0x82504a08
	ctx.lr = 0x82509818;
	sub_82504A08(ctx, base);
	// 82509818: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8250981C: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 82509820: 396B48B4  addi r11, r11, 0x48b4
	ctx.r[11].s64 = ctx.r[11].s64 + 18612;
	// 82509824: 394A4890  addi r10, r10, 0x4890
	ctx.r[10].s64 = ctx.r[10].s64 + 18576;
	// 82509828: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 8250982C: 3D208000  lis r9, -0x8000
	ctx.r[9].s64 = -2147483648;
	// 82509830: 7F7DDB78  mr r29, r27
	ctx.r[29].u64 = ctx.r[27].u64;
	// 82509834: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82509838: 39600020  li r11, 0x20
	ctx.r[11].s64 = 32;
	// 8250983C: 915F0010  stw r10, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 82509840: 937F00C0  stw r27, 0xc0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(192 as u32), ctx.r[27].u32 ) };
	// 82509844: 937F00C4  stw r27, 0xc4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(196 as u32), ctx.r[27].u32 ) };
	// 82509848: 913F00C8  stw r9, 0xc8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(200 as u32), ctx.r[9].u32 ) };
	// 8250984C: 937F00CC  stw r27, 0xcc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(204 as u32), ctx.r[27].u32 ) };
	// 82509850: 937F00D0  stw r27, 0xd0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(208 as u32), ctx.r[27].u32 ) };
	// 82509854: 913F00D4  stw r9, 0xd4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(212 as u32), ctx.r[9].u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825099A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825099A8 size=240
    let mut pc: u32 = 0x825099A8;
    'dispatch: loop {
        match pc {
            0x825099A8 => {
    //   block [0x825099A8..0x82509A98)
	// 825099A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825099AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825099B0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825099B4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825099B8: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 825099BC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825099C0: 396B4880  addi r11, r11, 0x4880
	ctx.r[11].s64 = ctx.r[11].s64 + 18560;
	// 825099C4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825099C8: 817F0034  lwz r11, 0x34(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 825099CC: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 825099D0: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 825099D4: 409A0020  bne cr6, 0x825099f4
	if !ctx.cr[6].eq {
	pc = 0x825099F4; continue 'dispatch;
	}
	// 825099D8: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 825099DC: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 825099E0: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 825099E4: 809F002C  lwz r4, 0x2c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 825099E8: 5565087C  rlwinm r5, r11, 1, 1, 0x1e
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x7FFFFFFFu64;
	// 825099EC: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 825099F0: 4BF5A6C9  bl 0x824640b8
	ctx.lr = 0x825099F4;
	sub_824640B8(ctx, base);
	// 825099F4: 817F0028  lwz r11, 0x28(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 825099F8: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 825099FC: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82509A00: 409A0020  bne cr6, 0x82509a20
	if !ctx.cr[6].eq {
	pc = 0x82509A20; continue 'dispatch;
	}
	// 82509A04: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82509A08: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 82509A0C: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82509A10: 809F0020  lwz r4, 0x20(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82509A14: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82509A18: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82509A1C: 4BF5A69D  bl 0x824640b8
	ctx.lr = 0x82509A20;
	sub_824640B8(ctx, base);
	// 82509A20: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82509A24: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82509A28: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82509A2C: 409A0020  bne cr6, 0x82509a4c
	if !ctx.cr[6].eq {
	pc = 0x82509A4C; continue 'dispatch;
	}
	// 82509A30: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82509A34: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 82509A38: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82509A3C: 809F0014  lwz r4, 0x14(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82509A40: 556500BE  clrlwi r5, r11, 2
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82509A44: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82509A48: 4BF5A671  bl 0x824640b8
	ctx.lr = 0x82509A4C;
	sub_824640B8(ctx, base);
	// 82509A4C: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82509A50: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82509A54: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82509A58: 409A0020  bne cr6, 0x82509a78
	if !ctx.cr[6].eq {
	pc = 0x82509A78; continue 'dispatch;
	}
	// 82509A5C: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82509A60: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 82509A64: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82509A68: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82509A6C: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82509A70: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82509A74: 4BF5A645  bl 0x824640b8
	ctx.lr = 0x82509A78;
	sub_824640B8(ctx, base);
	// 82509A78: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82509A7C: 396B6DD0  addi r11, r11, 0x6dd0
	ctx.r[11].s64 = ctx.r[11].s64 + 28112;
	// 82509A80: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82509A84: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82509A88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82509A8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82509A90: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82509A94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82509A98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82509A98 size=100
    let mut pc: u32 = 0x82509A98;
    'dispatch: loop {
        match pc {
            0x82509A98 => {
    //   block [0x82509A98..0x82509AFC)
	// 82509A98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82509A9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82509AA0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82509AA4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82509AA8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82509AAC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82509AB0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82509AB4: 4BFFFEF5  bl 0x825099a8
	ctx.lr = 0x82509AB8;
	sub_825099A8(ctx, base);
	// 82509AB8: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82509ABC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82509AC0: 419A0020  beq cr6, 0x82509ae0
	if ctx.cr[6].eq {
	pc = 0x82509AE0; continue 'dispatch;
	}
	// 82509AC4: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82509AC8: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 82509ACC: 38C00025  li r6, 0x25
	ctx.r[6].s64 = 37;
	// 82509AD0: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82509AD4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82509AD8: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82509ADC: 4BF5A5DD  bl 0x824640b8
	ctx.lr = 0x82509AE0;
	sub_824640B8(ctx, base);
	// 82509AE0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82509AE4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82509AE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82509AEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82509AF0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82509AF4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82509AF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82509B00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82509B00 size=8
    let mut pc: u32 = 0x82509B00;
    'dispatch: loop {
        match pc {
            0x82509B00 => {
    //   block [0x82509B00..0x82509B08)
	// 82509B00: 3863FFF0  addi r3, r3, -0x10
	ctx.r[3].s64 = ctx.r[3].s64 + -16;
	// 82509B04: 480001B4  b 0x82509cb8
	sub_82509CB8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82509B08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82509B08 size=328
    let mut pc: u32 = 0x82509B08;
    'dispatch: loop {
        match pc {
            0x82509B08 => {
    //   block [0x82509B08..0x82509C50)
	// 82509B08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82509B0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82509B10: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82509B14: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82509B18: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82509B1C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82509B20: 396B48E4  addi r11, r11, 0x48e4
	ctx.r[11].s64 = ctx.r[11].s64 + 18660;
	// 82509B24: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82509B28: 817F004C  lwz r11, 0x4c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 82509B2C: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82509B30: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82509B34: 409A0020  bne cr6, 0x82509b54
	if !ctx.cr[6].eq {
	pc = 0x82509B54; continue 'dispatch;
	}
	// 82509B38: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82509B3C: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 82509B40: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82509B44: 809F0044  lwz r4, 0x44(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(68 as u32) ) } as u64;
	// 82509B48: 5565087C  rlwinm r5, r11, 1, 1, 0x1e
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x7FFFFFFFu64;
	// 82509B4C: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82509B50: 4BF5A569  bl 0x824640b8
	ctx.lr = 0x82509B54;
	sub_824640B8(ctx, base);
	// 82509B54: 817F0040  lwz r11, 0x40(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as u64;
	// 82509B58: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82509B5C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82509B60: 409A0020  bne cr6, 0x82509b80
	if !ctx.cr[6].eq {
	pc = 0x82509B80; continue 'dispatch;
	}
	// 82509B64: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82509B68: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 82509B6C: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82509B70: 809F0038  lwz r4, 0x38(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 82509B74: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82509B78: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82509B7C: 4BF5A53D  bl 0x824640b8
	ctx.lr = 0x82509B80;
	sub_824640B8(ctx, base);
	// 82509B80: 817F0034  lwz r11, 0x34(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 82509B84: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82509B88: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82509B8C: 409A0020  bne cr6, 0x82509bac
	if !ctx.cr[6].eq {
	pc = 0x82509BAC; continue 'dispatch;
	}
	// 82509B90: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82509B94: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 82509B98: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82509B9C: 809F002C  lwz r4, 0x2c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 82509BA0: 556500BE  clrlwi r5, r11, 2
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82509BA4: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82509BA8: 4BF5A511  bl 0x824640b8
	ctx.lr = 0x82509BAC;
	sub_824640B8(ctx, base);
	// 82509BAC: 817F0028  lwz r11, 0x28(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82509BB0: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82509BB4: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82509BB8: 409A0020  bne cr6, 0x82509bd8
	if !ctx.cr[6].eq {
	pc = 0x82509BD8; continue 'dispatch;
	}
	// 82509BBC: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82509BC0: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 82509BC4: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82509BC8: 809F0020  lwz r4, 0x20(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82509BCC: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82509BD0: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82509BD4: 4BF5A4E5  bl 0x824640b8
	ctx.lr = 0x82509BD8;
	sub_824640B8(ctx, base);
	// 82509BD8: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82509BDC: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82509BE0: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82509BE4: 409A0020  bne cr6, 0x82509c04
	if !ctx.cr[6].eq {
	pc = 0x82509C04; continue 'dispatch;
	}
	// 82509BE8: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82509BEC: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 82509BF0: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82509BF4: 809F0014  lwz r4, 0x14(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82509BF8: 5565087C  rlwinm r5, r11, 1, 1, 0x1e
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x7FFFFFFFu64;
	// 82509BFC: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82509C00: 4BF5A4B9  bl 0x824640b8
	ctx.lr = 0x82509C04;
	sub_824640B8(ctx, base);
	// 82509C04: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82509C08: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82509C0C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82509C10: 409A0020  bne cr6, 0x82509c30
	if !ctx.cr[6].eq {
	pc = 0x82509C30; continue 'dispatch;
	}
	// 82509C14: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82509C18: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 82509C1C: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82509C20: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82509C24: 55652036  slwi r5, r11, 4
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82509C28: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82509C2C: 4BF5A48D  bl 0x824640b8
	ctx.lr = 0x82509C30;
	sub_824640B8(ctx, base);
	// 82509C30: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82509C34: 396B6DD0  addi r11, r11, 0x6dd0
	ctx.r[11].s64 = ctx.r[11].s64 + 28112;
	// 82509C38: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82509C3C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82509C40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82509C44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82509C48: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82509C4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82509C50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82509C50 size=100
    let mut pc: u32 = 0x82509C50;
    'dispatch: loop {
        match pc {
            0x82509C50 => {
    //   block [0x82509C50..0x82509CB4)
	// 82509C50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82509C54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82509C58: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82509C5C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82509C60: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82509C64: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82509C68: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82509C6C: 4BFFFE9D  bl 0x82509b08
	ctx.lr = 0x82509C70;
	sub_82509B08(ctx, base);
	// 82509C70: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82509C74: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82509C78: 419A0020  beq cr6, 0x82509c98
	if ctx.cr[6].eq {
	pc = 0x82509C98; continue 'dispatch;
	}
	// 82509C7C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82509C80: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 82509C84: 38C00025  li r6, 0x25
	ctx.r[6].s64 = 37;
	// 82509C88: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82509C8C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82509C90: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82509C94: 4BF5A425  bl 0x824640b8
	ctx.lr = 0x82509C98;
	sub_824640B8(ctx, base);
	// 82509C98: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82509C9C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82509CA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82509CA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82509CA8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82509CAC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82509CB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82509CB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82509CB8 size=100
    let mut pc: u32 = 0x82509CB8;
    'dispatch: loop {
        match pc {
            0x82509CB8 => {
    //   block [0x82509CB8..0x82509D1C)
	// 82509CB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82509CBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82509CC0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82509CC4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82509CC8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82509CCC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82509CD0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82509CD4: 4BFFEAED  bl 0x825087c0
	ctx.lr = 0x82509CD8;
	sub_825087C0(ctx, base);
	// 82509CD8: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82509CDC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82509CE0: 419A0020  beq cr6, 0x82509d00
	if ctx.cr[6].eq {
	pc = 0x82509D00; continue 'dispatch;
	}
	// 82509CE4: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82509CE8: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 82509CEC: 38C00027  li r6, 0x27
	ctx.r[6].s64 = 39;
	// 82509CF0: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82509CF4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82509CF8: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82509CFC: 4BF5A3BD  bl 0x824640b8
	ctx.lr = 0x82509D00;
	sub_824640B8(ctx, base);
	// 82509D00: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82509D04: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82509D08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82509D0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82509D10: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82509D14: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82509D18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82509D20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82509D20 size=144
    let mut pc: u32 = 0x82509D20;
    'dispatch: loop {
        match pc {
            0x82509D20 => {
    //   block [0x82509D20..0x82509DB0)
	// 82509D20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82509D24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82509D28: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82509D2C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82509D30: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82509D34: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82509D38: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82509D3C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82509D40: 388B48F0  addi r4, r11, 0x48f0
	ctx.r[4].s64 = ctx.r[11].s64 + 18672;
	// 82509D44: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82509D48: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82509D4C: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82509D50: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82509D54: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82509D58: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82509D5C: 4E800421  bctrl
	ctx.lr = 0x82509D60;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82509D60: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82509D64: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82509D68: 80DE0018  lwz r6, 0x18(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 82509D6C: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82509D70: 388B485C  addi r4, r11, 0x485c
	ctx.r[4].s64 = ctx.r[11].s64 + 18524;
	// 82509D74: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82509D78: 816A000C  lwz r11, 0xc(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82509D7C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82509D80: 4E800421  bctrl
	ctx.lr = 0x82509D84;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82509D84: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82509D88: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82509D8C: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82509D90: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82509D94: 4E800421  bctrl
	ctx.lr = 0x82509D98;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82509D98: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82509D9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82509DA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82509DA4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82509DA8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82509DAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82509DB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82509DB0 size=8
    let mut pc: u32 = 0x82509DB0;
    'dispatch: loop {
        match pc {
            0x82509DB0 => {
    //   block [0x82509DB0..0x82509DB8)
	// 82509DB0: 38630014  addi r3, r3, 0x14
	ctx.r[3].s64 = ctx.r[3].s64 + 20;
	// 82509DB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82509DB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82509DB8 size=132
    let mut pc: u32 = 0x82509DB8;
    'dispatch: loop {
        match pc {
            0x82509DB8 => {
    //   block [0x82509DB8..0x82509E3C)
	// 82509DB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82509DBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82509DC0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82509DC4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82509DC8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82509DCC: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82509DD0: 396B30EC  addi r11, r11, 0x30ec
	ctx.r[11].s64 = ctx.r[11].s64 + 12524;
	// 82509DD4: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82509DD8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82509DDC: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82509DE0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82509DE4: 419A0030  beq cr6, 0x82509e14
	if ctx.cr[6].eq {
	pc = 0x82509E14; continue 'dispatch;
	}
	// 82509DE8: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 82509DEC: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82509DF0: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 82509DF4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82509DF8: B1630006  sth r11, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 82509DFC: 409A0018  bne cr6, 0x82509e14
	if !ctx.cr[6].eq {
	pc = 0x82509E14; continue 'dispatch;
	}
	// 82509E00: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82509E04: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82509E08: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82509E0C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82509E10: 4E800421  bctrl
	ctx.lr = 0x82509E14;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82509E14: 387F0014  addi r3, r31, 0x14
	ctx.r[3].s64 = ctx.r[31].s64 + 20;
	// 82509E18: 4BFEF8D9  bl 0x824f96f0
	ctx.lr = 0x82509E1C;
	sub_824F96F0(ctx, base);
	// 82509E1C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82509E20: 396B6DD0  addi r11, r11, 0x6dd0
	ctx.r[11].s64 = ctx.r[11].s64 + 28112;
	// 82509E24: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82509E28: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82509E2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82509E30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82509E34: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82509E38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82509E40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82509E40 size=20
    let mut pc: u32 = 0x82509E40;
    'dispatch: loop {
        match pc {
            0x82509E40 => {
    //   block [0x82509E40..0x82509E54)
	// 82509E40: 80630010  lwz r3, 0x10(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82509E44: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82509E48: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82509E4C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82509E50: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82509E58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82509E58 size=96
    let mut pc: u32 = 0x82509E58;
    'dispatch: loop {
        match pc {
            0x82509E58 => {
    //   block [0x82509E58..0x82509EB8)
	// 82509E58: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82509E5C: 90830010  stw r4, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[4].u32 ) };
	// 82509E60: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 82509E64: 396B30EC  addi r11, r11, 0x30ec
	ctx.r[11].s64 = ctx.r[11].s64 + 12524;
	// 82509E68: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82509E6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82509E70: 38E0001A  li r7, 0x1a
	ctx.r[7].s64 = 26;
	// 82509E74: 394A2A68  addi r10, r10, 0x2a68
	ctx.r[10].s64 = ctx.r[10].s64 + 10856;
	// 82509E78: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82509E7C: B1230006  sth r9, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 82509E80: 91030008  stw r8, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[8].u32 ) };
	// 82509E84: 90E3000C  stw r7, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[7].u32 ) };
	// 82509E88: 91430014  stw r10, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 82509E8C: 90A30018  stw r5, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[5].u32 ) };
	// 82509E90: A1650004  lhz r11, 4(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) } as u64;
	// 82509E94: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82509E98: 419A0010  beq cr6, 0x82509ea8
	if ctx.cr[6].eq {
	pc = 0x82509EA8; continue 'dispatch;
	}
	// 82509E9C: A1650006  lhz r11, 6(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[5].u32.wrapping_add(6 as u32) ) } as u64;
	// 82509EA0: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82509EA4: B1650006  sth r11, 6(r5)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[5].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 82509EA8: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82509EAC: A14B0004  lhz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82509EB0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82509EB4: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82509EB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82509EB8 size=16
    let mut pc: u32 = 0x82509EB8;
    'dispatch: loop {
        match pc {
            0x82509EB8 => {
    //   block [0x82509EB8..0x82509EC8)
	// 82509EB8: A14B0006  lhz r10, 6(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(6 as u32) ) } as u64;
	// 82509EBC: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82509EC0: B14B0006  sth r10, 6(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82509EC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82509EC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82509EC8 size=216
    let mut pc: u32 = 0x82509EC8;
    'dispatch: loop {
        match pc {
            0x82509EC8 => {
    //   block [0x82509EC8..0x82509FA0)
	// 82509EC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82509ECC: 4802B1ED  bl 0x825350b8
	ctx.lr = 0x82509ED0;
	sub_82535080(ctx, base);
	// 82509ED0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82509ED4: 838D0000  lwz r28, 0(r13)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82509ED8: 3BA00014  li r29, 0x14
	ctx.r[29].s64 = 20;
	// 82509EDC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82509EE0: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 82509EE4: 7D7DE02E  lwzx r11, r29, r28
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 82509EE8: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82509EEC: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82509EF0: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82509EF4: 40980020  bge cr6, 0x82509f14
	if !ctx.cr[6].lt {
	pc = 0x82509F14; continue 'dispatch;
	}
	// 82509EF8: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 82509EFC: 392948F8  addi r9, r9, 0x48f8
	ctx.r[9].s64 = ctx.r[9].s64 + 18680;
	// 82509F00: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82509F04: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82509F08: 390A000C  addi r8, r10, 0xc
	ctx.r[8].s64 = ctx.r[10].s64 + 12;
	// 82509F0C: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82509F10: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82509F14: 817F0040  lwz r11, 0x40(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as u64;
	// 82509F18: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 82509F1C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82509F20: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82509F24: 917F0040  stw r11, 0x40(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), ctx.r[11].u32 ) };
	// 82509F28: 80840018  lwz r4, 0x18(r4)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(24 as u32) ) } as u64;
	// 82509F2C: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82509F30: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82509F34: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82509F38: 4E800421  bctrl
	ctx.lr = 0x82509F3C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82509F3C: 817F0040  lwz r11, 0x40(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as u64;
	// 82509F40: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82509F44: 917F0040  stw r11, 0x40(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), ctx.r[11].u32 ) };
	// 82509F48: 895E0000  lbz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82509F4C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82509F50: 419A0014  beq cr6, 0x82509f64
	if ctx.cr[6].eq {
	pc = 0x82509F64; continue 'dispatch;
	}
	// 82509F54: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82509F58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82509F5C: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82509F60: 7D4BF92E  stwx r10, r11, r31
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32), ctx.r[10].u32) };
	// 82509F64: 7D5DE02E  lwzx r10, r29, r28
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 82509F68: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82509F6C: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82509F70: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82509F74: 40980020  bge cr6, 0x82509f94
	if !ctx.cr[6].lt {
	pc = 0x82509F94; continue 'dispatch;
	}
	// 82509F78: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 82509F7C: 392974DC  addi r9, r9, 0x74dc
	ctx.r[9].s64 = ctx.r[9].s64 + 29916;
	// 82509F80: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82509F84: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82509F88: 390B000C  addi r8, r11, 0xc
	ctx.r[8].s64 = ctx.r[11].s64 + 12;
	// 82509F8C: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82509F90: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82509F94: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82509F98: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82509F9C: 4802B16C  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82509FA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82509FA0 size=196
    let mut pc: u32 = 0x82509FA0;
    'dispatch: loop {
        match pc {
            0x82509FA0 => {
    //   block [0x82509FA0..0x8250A064)
	// 82509FA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82509FA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82509FA8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82509FAC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82509FB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82509FB4: 83CD0000  lwz r30, 0(r13)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82509FB8: 3BE00014  li r31, 0x14
	ctx.r[31].s64 = 20;
	// 82509FBC: 7D7FF02E  lwzx r11, r31, r30
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82509FC0: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82509FC4: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82509FC8: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82509FCC: 40980020  bge cr6, 0x82509fec
	if !ctx.cr[6].lt {
	pc = 0x82509FEC; continue 'dispatch;
	}
	// 82509FD0: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 82509FD4: 392948F8  addi r9, r9, 0x48f8
	ctx.r[9].s64 = ctx.r[9].s64 + 18680;
	// 82509FD8: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82509FDC: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82509FE0: 390A000C  addi r8, r10, 0xc
	ctx.r[8].s64 = ctx.r[10].s64 + 12;
	// 82509FE4: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82509FE8: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82509FEC: 81650008  lwz r11, 8(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(8 as u32) ) } as u64;
	// 82509FF0: 80630018  lwz r3, 0x18(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 82509FF4: 90A1005C  stw r5, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[5].u32 ) };
	// 82509FF8: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82509FFC: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 8250A000: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8250A004: 90610050  stw r3, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 8250A008: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8250A00C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250A010: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 8250A014: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8250A018: 4E800421  bctrl
	ctx.lr = 0x8250A01C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8250A01C: 7D7FF02E  lwzx r11, r31, r30
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 8250A020: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8250A024: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8250A028: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 8250A02C: 40980020  bge cr6, 0x8250a04c
	if !ctx.cr[6].lt {
	pc = 0x8250A04C; continue 'dispatch;
	}
	// 8250A030: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 8250A034: 392974DC  addi r9, r9, 0x74dc
	ctx.r[9].s64 = ctx.r[9].s64 + 29916;
	// 8250A038: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8250A03C: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 8250A040: 390A000C  addi r8, r10, 0xc
	ctx.r[8].s64 = ctx.r[10].s64 + 12;
	// 8250A044: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 8250A048: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 8250A04C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8250A050: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8250A054: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8250A058: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8250A05C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8250A060: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250A068(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8250A068 size=96
    let mut pc: u32 = 0x8250A068;
    'dispatch: loop {
        match pc {
            0x8250A068 => {
    //   block [0x8250A068..0x8250A0C8)
	// 8250A068: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250A06C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8250A070: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8250A074: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250A078: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8250A07C: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8250A080: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 8250A084: 388B4904  addi r4, r11, 0x4904
	ctx.r[4].s64 = ctx.r[11].s64 + 18692;
	// 8250A088: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8250A08C: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250A090: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8250A094: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 8250A098: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8250A09C: 4E800421  bctrl
	ctx.lr = 0x8250A0A0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8250A0A0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250A0A4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8250A0A8: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 8250A0AC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8250A0B0: 4E800421  bctrl
	ctx.lr = 0x8250A0B4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8250A0B4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8250A0B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8250A0BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8250A0C0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8250A0C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250A0C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8250A0C8 size=28
    let mut pc: u32 = 0x8250A0C8;
    'dispatch: loop {
        match pc {
            0x8250A0C8 => {
    //   block [0x8250A0C8..0x8250A0E4)
	// 8250A0C8: 548B103A  slwi r11, r4, 2
	ctx.r[11].u32 = ctx.r[4].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8250A0CC: 546A043E  clrlwi r10, r3, 0x10
	ctx.r[10].u64 = ctx.r[3].u32 as u64 & 0x0000FFFFu64;
	// 8250A0D0: 7D645A14  add r11, r4, r11
	ctx.r[11].u64 = ctx.r[4].u64 + ctx.r[11].u64;
	// 8250A0D4: 556B043E  clrlwi r11, r11, 0x10
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 8250A0D8: 7D4B5C30  srw r11, r10, r11
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[11].u64 = 0;
	} else {
		ctx.r[11].u64 = ((ctx.r[10].u32) >> ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	// 8250A0DC: 556306FE  clrlwi r3, r11, 0x1b
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8250A0E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250A0E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8250A0E8 size=20
    let mut pc: u32 = 0x8250A0E8;
    'dispatch: loop {
        match pc {
            0x8250A0E8 => {
    //   block [0x8250A0E8..0x8250A0FC)
	// 8250A0E8: 89630017  lbz r11, 0x17(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(23 as u32) ) } as u64;
	// 8250A0EC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8250A0F0: 556A083C  slwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8250A0F4: 7C6B5214  add r3, r11, r10
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8250A0F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250A100(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8250A100 size=16
    let mut pc: u32 = 0x8250A100;
    'dispatch: loop {
        match pc {
            0x8250A100 => {
    //   block [0x8250A100..0x8250A110)
	// 8250A100: 39600020  li r11, 0x20
	ctx.r[11].s64 = 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250A110(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8250A110 size=72
    let mut pc: u32 = 0x8250A110;
    'dispatch: loop {
        match pc {
            0x8250A110 => {
    //   block [0x8250A110..0x8250A158)
	// 8250A110: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 8250A114: 39630030  addi r11, r3, 0x30
	ctx.r[11].s64 = ctx.r[3].s64 + 48;
	// 8250A118: 3921FFF0  addi r9, r1, -0x10
	ctx.r[9].s64 = ctx.r[1].s64 + -16;
	// 8250A11C: C00A2128  lfs f0, 0x2128(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8488 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8250A120: 39400020  li r10, 0x20
	ctx.r[10].s64 = 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250A158(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8250A158 size=108
    let mut pc: u32 = 0x8250A158;
    'dispatch: loop {
        match pc {
            0x8250A158 => {
    //   block [0x8250A158..0x8250A1C4)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250A1C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8250A1C8 size=224
    let mut pc: u32 = 0x8250A1C8;
    'dispatch: loop {
        match pc {
            0x8250A1C8 => {
    //   block [0x8250A1C8..0x8250A2A8)
	// 8250A1C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250A1CC: 4802AEF1  bl 0x825350bc
	ctx.lr = 0x8250A1D0;
	sub_82535080(ctx, base);
	// 8250A1D0: 3980FFD0  li r12, -0x30
	ctx.r[12].s64 = -48;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250A2A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8250A2A8 size=92
    let mut pc: u32 = 0x8250A2A8;
    'dispatch: loop {
        match pc {
            0x8250A2A8 => {
    //   block [0x8250A2A8..0x8250A304)
	// 8250A2A8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8250A2AC: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 8250A2B0: 390B0020  addi r8, r11, 0x20
	ctx.r[8].s64 = ctx.r[11].s64 + 32;
	// 8250A2B4: 38EB0030  addi r7, r11, 0x30
	ctx.r[7].s64 = ctx.r[11].s64 + 48;
	// 8250A2B8: 39430010  addi r10, r3, 0x10
	ctx.r[10].s64 = ctx.r[3].s64 + 16;
	// 8250A2BC: 38CB0040  addi r6, r11, 0x40
	ctx.r[6].s64 = ctx.r[11].s64 + 64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250A304(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8250A304 size=88
    let mut pc: u32 = 0x8250A304;
    'dispatch: loop {
        match pc {
            0x8250A304 => {
    //   block [0x8250A304..0x8250A35C)
	// 8250A304: 394B0050  addi r10, r11, 0x50
	ctx.r[10].s64 = ctx.r[11].s64 + 80;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250A360(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8250A360 size=96
    let mut pc: u32 = 0x8250A360;
    'dispatch: loop {
        match pc {
            0x8250A360 => {
    //   block [0x8250A360..0x8250A3C0)
	// 8250A360: 39630020  addi r11, r3, 0x20
	ctx.r[11].s64 = ctx.r[3].s64 + 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250A3C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8250A3C0 size=112
    let mut pc: u32 = 0x8250A3C0;
    'dispatch: loop {
        match pc {
            0x8250A3C0 => {
    //   block [0x8250A3C0..0x8250A430)
	// 8250A3C0: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8250A3C4: FC006890  fmr f0, f13
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = ctx.f[13].f64;
	// 8250A3C8: C1A1FFF0  lfs f13, -0x10(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8250A3CC: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 8250A3D0: 40990008  ble cr6, 0x8250a3d8
	if !ctx.cr[6].gt {
	pc = 0x8250A3D8; continue 'dispatch;
	}
	// 8250A3D4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8250A3D8: 394B0002  addi r10, r11, 2
	ctx.r[10].s64 = ctx.r[11].s64 + 2;
	// 8250A3DC: 39200050  li r9, 0x50
	ctx.r[9].s64 = 80;
	// 8250A3E0: 554A2036  slwi r10, r10, 4
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8250A3E4: 3901FFE0  addi r8, r1, -0x20
	ctx.r[8].s64 = ctx.r[1].s64 + -32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250A430(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8250A430 size=12
    let mut pc: u32 = 0x8250A430;
    'dispatch: loop {
        match pc {
            0x8250A430 => {
    //   block [0x8250A430..0x8250A43C)
	// 8250A430: 3965FFFF  addi r11, r5, -1
	ctx.r[11].s64 = ctx.r[5].s64 + -1;
	// 8250A434: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8250A438: 4D980020  bltlr cr6
	if ctx.cr[6].lt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250A43C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8250A43C size=112
    let mut pc: u32 = 0x8250A43C;
    'dispatch: loop {
        match pc {
            0x8250A43C => {
    //   block [0x8250A43C..0x8250A4AC)
	// 8250A43C: 3D408282  lis r10, -0x7d7e
	ctx.r[10].s64 = -2105409536;
	// 8250A440: 38E30050  addi r7, r3, 0x50
	ctx.r[7].s64 = ctx.r[3].s64 + 80;
	// 8250A444: 392AD3D0  addi r9, r10, -0x2c30
	ctx.r[9].s64 = ctx.r[10].s64 + -11312;
	// 8250A448: 3D408282  lis r10, -0x7d7e
	ctx.r[10].s64 = -2105409536;
	// 8250A44C: 390AD3FC  addi r8, r10, -0x2c04
	ctx.r[8].s64 = ctx.r[10].s64 + -11268;
	// 8250A450: A1440000  lhz r10, 0(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250A454: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8250A458: 554A103E  rotlwi r10, r10, 2
	ctx.r[10].u64 = ((ctx.r[10].u32).rotate_left(2)) as u64;
	// 8250A45C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8250A460: 7D4A402E  lwzx r10, r10, r8
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 8250A464: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 8250A468: 554A2036  slwi r10, r10, 4
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250A4B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8250A4B0 size=1296
    let mut pc: u32 = 0x8250A4B0;
    'dispatch: loop {
        match pc {
            0x8250A4B0 => {
    //   block [0x8250A4B0..0x8250A9C0)
	// 8250A4B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250A4B4: 4802ABF5  bl 0x825350a8
	ctx.lr = 0x8250A4B8;
	sub_82535080(ctx, base);
	// 8250A4B8: DBE1FFB0  stfd f31, -0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-80 as u32), ctx.f[31].u64 ) };
	// 8250A4BC: 3980FF90  li r12, -0x70
	ctx.r[12].s64 = -112;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250A9C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8250A9C0 size=724
    let mut pc: u32 = 0x8250A9C0;
    'dispatch: loop {
        match pc {
            0x8250A9C0 => {
    //   block [0x8250A9C0..0x8250AC94)
	// 8250A9C0: FBC1FFF0  std r30, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[30].u64 ) };
	// 8250A9C4: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 8250A9C8: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250A9CC: 39600014  li r11, 0x14
	ctx.r[11].s64 = 20;
	// 8250A9D0: 7D0B5214  add r8, r11, r10
	ctx.r[8].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8250A9D4: 81680000  lwz r11, 0(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250A9D8: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8250A9DC: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8250A9E0: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 8250A9E4: 40980020  bge cr6, 0x8250aa04
	if !ctx.cr[6].lt {
	pc = 0x8250AA04; continue 'dispatch;
	}
	// 8250A9E8: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 8250A9EC: 39294914  addi r9, r9, 0x4914
	ctx.r[9].s64 = ctx.r[9].s64 + 18708;
	// 8250A9F0: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8250A9F4: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 8250A9F8: 38EA000C  addi r7, r10, 0xc
	ctx.r[7].s64 = ctx.r[10].s64 + 12;
	// 8250A9FC: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 8250AA00: 90EB0004  stw r7, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 8250AA04: 39640020  addi r11, r4, 0x20
	ctx.r[11].s64 = ctx.r[4].s64 + 32;
	// 8250AA08: 39440030  addi r10, r4, 0x30
	ctx.r[10].s64 = ctx.r[4].s64 + 48;
	// 8250AA0C: 39240040  addi r9, r4, 0x40
	ctx.r[9].s64 = ctx.r[4].s64 + 64;
	// 8250AA10: 38E1FFD0  addi r7, r1, -0x30
	ctx.r[7].s64 = ctx.r[1].s64 + -48;
	// 8250AA14: 3881FFD4  addi r4, r1, -0x2c
	ctx.r[4].s64 = ctx.r[1].s64 + -44;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250AC98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8250AC98 size=96
    let mut pc: u32 = 0x8250AC98;
    'dispatch: loop {
        match pc {
            0x8250AC98 => {
    //   block [0x8250AC98..0x8250ACF8)
	// 8250AC98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250AC9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8250ACA0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8250ACA4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250ACA8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8250ACAC: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8250ACB0: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 8250ACB4: 388B4924  addi r4, r11, 0x4924
	ctx.r[4].s64 = ctx.r[11].s64 + 18724;
	// 8250ACB8: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8250ACBC: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250ACC0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8250ACC4: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 8250ACC8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8250ACCC: 4E800421  bctrl
	ctx.lr = 0x8250ACD0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8250ACD0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250ACD4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8250ACD8: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 8250ACDC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8250ACE0: 4E800421  bctrl
	ctx.lr = 0x8250ACE4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8250ACE4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8250ACE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8250ACEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8250ACF0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8250ACF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250ACF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8250ACF8 size=172
    let mut pc: u32 = 0x8250ACF8;
    'dispatch: loop {
        match pc {
            0x8250ACF8 => {
    //   block [0x8250ACF8..0x8250ADA4)
	// 8250ACF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250ACFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8250AD00: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8250AD04: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8250AD08: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250AD0C: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 8250AD10: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250AD14: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 8250AD18: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 8250AD1C: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 8250AD20: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8250AD24: 91410074  stw r10, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[10].u32 ) };
	// 8250AD28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8250AD2C: C01F0004  lfs f0, 4(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8250AD30: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 8250AD34: D0010070  stfs f0, 0x70(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), tmp.u32 ) };
	// 8250AD38: 38C10060  addi r6, r1, 0x60
	ctx.r[6].s64 = ctx.r[1].s64 + 96;
	// 8250AD3C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8250AD40: 914100A0  stw r10, 0xa0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(160 as u32), ctx.r[10].u32 ) };
	// 8250AD44: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 8250AD48: 91410080  stw r10, 0x80(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[10].u32 ) };
	// 8250AD4C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8250AD50: 4E800421  bctrl
	ctx.lr = 0x8250AD54;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8250AD54: 89610050  lbz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8250AD58: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8250AD5C: 419A0030  beq cr6, 0x8250ad8c
	if ctx.cr[6].eq {
	pc = 0x8250AD8C; continue 'dispatch;
	}
	// 8250AD60: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 8250AD64: 809E0008  lwz r4, 8(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250AD68: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8250AD6C: 480A0E6D  bl 0x825abbd8
	ctx.lr = 0x8250AD70;
	sub_825ABBD8(ctx, base);
	// 8250AD70: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250AD74: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 8250AD78: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8250AD7C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8250AD80: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250AD84: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8250AD88: 4E800421  bctrl
	ctx.lr = 0x8250AD8C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8250AD8C: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 8250AD90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8250AD94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8250AD98: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8250AD9C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8250ADA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250ADA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8250ADA8 size=360
    let mut pc: u32 = 0x8250ADA8;
    'dispatch: loop {
        match pc {
            0x8250ADA8 => {
    //   block [0x8250ADA8..0x8250AF10)
	// 8250ADA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250ADAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8250ADB0: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250ADB4: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8250ADB8: 7C8A2378  mr r10, r4
	ctx.r[10].u64 = ctx.r[4].u64;
	// 8250ADBC: 392B0010  addi r9, r11, 0x10
	ctx.r[9].s64 = ctx.r[11].s64 + 16;
	// 8250ADC0: 39010050  addi r8, r1, 0x50
	ctx.r[8].s64 = ctx.r[1].s64 + 80;
	// 8250ADC4: 38E00010  li r7, 0x10
	ctx.r[7].s64 = 16;
	// 8250ADC8: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250AF10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8250AF10 size=24
    let mut pc: u32 = 0x8250AF10;
    'dispatch: loop {
        match pc {
            0x8250AF10 => {
    //   block [0x8250AF10..0x8250AF28)
	// 8250AF10: 81240004  lwz r9, 4(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 8250AF14: 7CAA2B78  mr r10, r5
	ctx.r[10].u64 = ctx.r[5].u64;
	// 8250AF18: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250AF1C: 3929FFFF  addi r9, r9, -1
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	// 8250AF20: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8250AF24: 4D980020  bltlr cr6
	if ctx.cr[6].lt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250AF28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8250AF28 size=80
    let mut pc: u32 = 0x8250AF28;
    'dispatch: loop {
        match pc {
            0x8250AF28 => {
    //   block [0x8250AF28..0x8250AF78)
	// 8250AF28: 39030010  addi r8, r3, 0x10
	ctx.r[8].s64 = ctx.r[3].s64 + 16;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250AF78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8250AF78 size=176
    let mut pc: u32 = 0x8250AF78;
    'dispatch: loop {
        match pc {
            0x8250AF78 => {
    //   block [0x8250AF78..0x8250B028)
	// 8250AF78: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 8250AF7C: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 8250AF80: 3901FFF0  addi r8, r1, -0x10
	ctx.r[8].s64 = ctx.r[1].s64 + -16;
	// 8250AF84: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 8250AF88: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 8250AF8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8250AF90: C00ABFFC  lfs f0, -0x4004(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-16388 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8250AF94: 392931C4  addi r9, r9, 0x31c4
	ctx.r[9].s64 = ctx.r[9].s64 + 12740;
	// 8250AF98: D001FFF0  stfs f0, -0x10(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 8250AF9C: 3BE0001B  li r31, 0x1b
	ctx.r[31].s64 = 27;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250B028(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8250B028 size=108
    let mut pc: u32 = 0x8250B028;
    'dispatch: loop {
        match pc {
            0x8250B028 => {
    //   block [0x8250B028..0x8250B094)
	// 8250B028: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8250B02C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8250B030: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8250B034: 3900001B  li r8, 0x1b
	ctx.r[8].s64 = 27;
	// 8250B038: 396B31C4  addi r11, r11, 0x31c4
	ctx.r[11].s64 = ctx.r[11].s64 + 12740;
	// 8250B03C: 38E00010  li r7, 0x10
	ctx.r[7].s64 = 16;
	// 8250B040: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 8250B044: 3941FFF0  addi r10, r1, -0x10
	ctx.r[10].s64 = ctx.r[1].s64 + -16;
	// 8250B048: 91230008  stw r9, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 8250B04C: 39200020  li r9, 0x20
	ctx.r[9].s64 = 32;
	// 8250B050: 9103000C  stw r8, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 8250B054: 39000030  li r8, 0x30
	ctx.r[8].s64 = 48;
	// 8250B058: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250B098(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8250B098 size=24
    let mut pc: u32 = 0x8250B098;
    'dispatch: loop {
        match pc {
            0x8250B098 => {
    //   block [0x8250B098..0x8250B0B0)
	// 8250B098: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8250B09C: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 8250B0A0: 7CC73378  mr r7, r6
	ctx.r[7].u64 = ctx.r[6].u64;
	// 8250B0A4: 38AB0020  addi r5, r11, 0x20
	ctx.r[5].s64 = ctx.r[11].s64 + 32;
	// 8250B0A8: 388B0030  addi r4, r11, 0x30
	ctx.r[4].s64 = ctx.r[11].s64 + 48;
	// 8250B0AC: 4BFF18A4  b 0x824fc950
	sub_824FC950(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250B0B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8250B0B0 size=408
    let mut pc: u32 = 0x8250B0B0;
    'dispatch: loop {
        match pc {
            0x8250B0B0 => {
    //   block [0x8250B0B0..0x8250B248)
	// 8250B0B0: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 8250B0B4: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250B0B8: 39600014  li r11, 0x14
	ctx.r[11].s64 = 20;
	// 8250B0BC: 7D2B5214  add r9, r11, r10
	ctx.r[9].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8250B0C0: 81690000  lwz r11, 0(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250B0C4: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8250B0C8: 810B000C  lwz r8, 0xc(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8250B0CC: 7F0A4040  cmplw cr6, r10, r8
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[8].u32, &mut ctx.xer);
	// 8250B0D0: 40980020  bge cr6, 0x8250b0f0
	if !ctx.cr[6].lt {
	pc = 0x8250B0F0; continue 'dispatch;
	}
	// 8250B0D4: 3D008206  lis r8, -0x7dfa
	ctx.r[8].s64 = -2113536000;
	// 8250B0D8: 39084930  addi r8, r8, 0x4930
	ctx.r[8].s64 = ctx.r[8].s64 + 18736;
	// 8250B0DC: 910A0000  stw r8, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 8250B0E0: 7D0C42E6  mftb r8, 0x10c
	ctx.r[8].u64 = crate::rt::rdtsc_u64();
	// 8250B0E4: 38EA000C  addi r7, r10, 0xc
	ctx.r[7].s64 = ctx.r[10].s64 + 12;
	// 8250B0E8: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 8250B0EC: 90EB0004  stw r7, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 8250B0F0: 39440010  addi r10, r4, 0x10
	ctx.r[10].s64 = ctx.r[4].s64 + 16;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250B248(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8250B248 size=12
    let mut pc: u32 = 0x8250B248;
    'dispatch: loop {
        match pc {
            0x8250B248 => {
    //   block [0x8250B248..0x8250B254)
	// 8250B248: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8250B24C: 38630010  addi r3, r3, 0x10
	ctx.r[3].s64 = ctx.r[3].s64 + 16;
	// 8250B250: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250B254(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8250B254 size=8
    let mut pc: u32 = 0x8250B254;
    'dispatch: loop {
        match pc {
            0x8250B254 => {
    //   block [0x8250B254..0x8250B25C)
	// 8250B254: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8250B258: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250B260(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8250B260 size=96
    let mut pc: u32 = 0x8250B260;
    'dispatch: loop {
        match pc {
            0x8250B260 => {
    //   block [0x8250B260..0x8250B2C0)
	// 8250B260: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250B264: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8250B268: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8250B26C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250B270: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8250B274: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8250B278: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 8250B27C: 388B493C  addi r4, r11, 0x493c
	ctx.r[4].s64 = ctx.r[11].s64 + 18748;
	// 8250B280: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8250B284: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250B288: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8250B28C: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 8250B290: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8250B294: 4E800421  bctrl
	ctx.lr = 0x8250B298;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8250B298: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250B29C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8250B2A0: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 8250B2A4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8250B2A8: 4E800421  bctrl
	ctx.lr = 0x8250B2AC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8250B2AC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8250B2B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8250B2B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8250B2B8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8250B2BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250B2C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8250B2C0 size=64
    let mut pc: u32 = 0x8250B2C0;
    'dispatch: loop {
        match pc {
            0x8250B2C0 => {
    //   block [0x8250B2C0..0x8250B300)
	// 8250B2C0: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8250B2C4: 9083000C  stw r4, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[4].u32 ) };
	// 8250B2C8: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 8250B2CC: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 8250B2D0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8250B2D4: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 8250B2D8: 396B2A44  addi r11, r11, 0x2a44
	ctx.r[11].s64 = ctx.r[11].s64 + 10820;
	// 8250B2DC: 394A3260  addi r10, r10, 0x3260
	ctx.r[10].s64 = ctx.r[10].s64 + 12896;
	// 8250B2E0: 3929323C  addi r9, r9, 0x323c
	ctx.r[9].s64 = ctx.r[9].s64 + 12860;
	// 8250B2E4: 91030008  stw r8, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[8].u32 ) };
	// 8250B2E8: B0E30006  sth r7, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[7].u16 ) };
	// 8250B2EC: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 8250B2F0: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8250B2F4: 91230010  stw r9, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[9].u32 ) };
	// 8250B2F8: 99030014  stb r8, 0x14(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[8].u8 ) };
	// 8250B2FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250B300(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8250B300 size=560
    let mut pc: u32 = 0x8250B300;
    'dispatch: loop {
        match pc {
            0x8250B300 => {
    //   block [0x8250B300..0x8250B530)
	// 8250B300: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250B304: 48029D9D  bl 0x825350a0
	ctx.lr = 0x8250B308;
	sub_82535080(ctx, base);
	// 8250B308: 9421FD40  stwu r1, -0x2c0(r1)
	ea = ctx.r[1].u32.wrapping_add(-704 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250B30C: 830D0000  lwz r24, 0(r13)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250B310: 3B200014  li r25, 0x14
	ctx.r[25].s64 = 20;
	// 8250B314: 7C761B78  mr r22, r3
	ctx.r[22].u64 = ctx.r[3].u64;
	// 8250B318: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 8250B31C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8250B320: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 8250B324: 7D79C02E  lwzx r11, r25, r24
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[25].u32.wrapping_add(ctx.r[24].u32)) } as u64;
	// 8250B328: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8250B32C: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8250B330: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 8250B334: 40980020  bge cr6, 0x8250b354
	if !ctx.cr[6].lt {
	pc = 0x8250B354; continue 'dispatch;
	}
	// 8250B338: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 8250B33C: 39294948  addi r9, r9, 0x4948
	ctx.r[9].s64 = ctx.r[9].s64 + 18760;
	// 8250B340: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8250B344: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 8250B348: 390A000C  addi r8, r10, 0xc
	ctx.r[8].s64 = ctx.r[10].s64 + 12;
	// 8250B34C: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 8250B350: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 8250B354: 817C0040  lwz r11, 0x40(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(64 as u32) ) } as u64;
	// 8250B358: 3AE0FFFF  li r23, -1
	ctx.r[23].s64 = -1;
	// 8250B35C: 3BDB0010  addi r30, r27, 0x10
	ctx.r[30].s64 = ctx.r[27].s64 + 16;
	// 8250B360: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8250B364: 7EFABB78  mr r26, r23
	ctx.r[26].u64 = ctx.r[23].u64;
	// 8250B368: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8250B36C: 917C0040  stw r11, 0x40(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(64 as u32), ctx.r[11].u32 ) };
	// 8250B370: 817D0024  lwz r11, 0x24(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(36 as u32) ) } as u64;
	// 8250B374: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8250B378: 817B0010  lwz r11, 0x10(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(16 as u32) ) } as u64;
	// 8250B37C: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250B380: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8250B384: 409A0088  bne cr6, 0x8250b40c
	if !ctx.cr[6].eq {
	pc = 0x8250B40C; continue 'dispatch;
	}
	// 8250B388: 4E800421  bctrl
	ctx.lr = 0x8250B38C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8250B38C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8250B390: 2F1FFFFF  cmpwi cr6, r31, -1
	ctx.cr[6].compare_i32(ctx.r[31].s32, -1, &mut ctx.xer);
	// 8250B394: 419A012C  beq cr6, 0x8250b4c0
	if ctx.cr[6].eq {
	pc = 0x8250B4C0; continue 'dispatch;
	}
	// 8250B398: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250B39C: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 8250B3A0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8250B3A4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8250B3A8: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 8250B3AC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8250B3B0: 4E800421  bctrl
	ctx.lr = 0x8250B3B4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8250B3B4: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8250B3B8: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 8250B3BC: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8250B3C0: 38610051  addi r3, r1, 0x51
	ctx.r[3].s64 = ctx.r[1].s64 + 81;
	// 8250B3C4: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250B3C8: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 8250B3CC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8250B3D0: 4E800421  bctrl
	ctx.lr = 0x8250B3D4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8250B3D4: 89610051  lbz r11, 0x51(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(81 as u32) ) } as u64;
	// 8250B3D8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8250B3DC: 419A0008  beq cr6, 0x8250b3e4
	if ctx.cr[6].eq {
	pc = 0x8250B3E4; continue 'dispatch;
	}
	// 8250B3E0: 7FFAFB78  mr r26, r31
	ctx.r[26].u64 = ctx.r[31].u64;
	// 8250B3E4: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250B3E8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8250B3EC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8250B3F0: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8250B3F4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8250B3F8: 4E800421  bctrl
	ctx.lr = 0x8250B3FC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8250B3FC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8250B400: 2F1FFFFF  cmpwi cr6, r31, -1
	ctx.cr[6].compare_i32(ctx.r[31].s32, -1, &mut ctx.xer);
	// 8250B404: 409AFF94  bne cr6, 0x8250b398
	if !ctx.cr[6].eq {
	pc = 0x8250B398; continue 'dispatch;
	}
	// 8250B408: 480000B8  b 0x8250b4c0
	pc = 0x8250B4C0; continue 'dispatch;
	// 8250B40C: 4E800421  bctrl
	ctx.lr = 0x8250B410;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8250B410: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8250B414: 2F1FFFFF  cmpwi cr6, r31, -1
	ctx.cr[6].compare_i32(ctx.r[31].s32, -1, &mut ctx.xer);
	// 8250B418: 419A00A8  beq cr6, 0x8250b4c0
	if ctx.cr[6].eq {
	pc = 0x8250B4C0; continue 'dispatch;
	}
	// 8250B41C: 809D0024  lwz r4, 0x24(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(36 as u32) ) } as u64;
	// 8250B420: 7FE8FB78  mr r8, r31
	ctx.r[8].u64 = ctx.r[31].u64;
	// 8250B424: 7FC7F378  mr r7, r30
	ctx.r[7].u64 = ctx.r[30].u64;
	// 8250B428: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 8250B42C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8250B430: 38610052  addi r3, r1, 0x52
	ctx.r[3].s64 = ctx.r[1].s64 + 82;
	// 8250B434: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250B438: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250B43C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8250B440: 4E800421  bctrl
	ctx.lr = 0x8250B444;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8250B444: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250B448: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8250B44C: 419A0050  beq cr6, 0x8250b49c
	if ctx.cr[6].eq {
	pc = 0x8250B49C; continue 'dispatch;
	}
	// 8250B450: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250B454: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 8250B458: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8250B45C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8250B460: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 8250B464: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8250B468: 4E800421  bctrl
	ctx.lr = 0x8250B46C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8250B46C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8250B470: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 8250B474: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8250B478: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8250B47C: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250B480: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 8250B484: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8250B488: 4E800421  bctrl
	ctx.lr = 0x8250B48C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8250B48C: 89610050  lbz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8250B490: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8250B494: 419A0008  beq cr6, 0x8250b49c
	if ctx.cr[6].eq {
	pc = 0x8250B49C; continue 'dispatch;
	}
	// 8250B498: 7FFAFB78  mr r26, r31
	ctx.r[26].u64 = ctx.r[31].u64;
	// 8250B49C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250B4A0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8250B4A4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8250B4A8: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8250B4AC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8250B4B0: 4E800421  bctrl
	ctx.lr = 0x8250B4B4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8250B4B4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8250B4B8: 2F1FFFFF  cmpwi cr6, r31, -1
	ctx.cr[6].compare_i32(ctx.r[31].s32, -1, &mut ctx.xer);
	// 8250B4BC: 409AFF60  bne cr6, 0x8250b41c
	if !ctx.cr[6].eq {
	pc = 0x8250B41C; continue 'dispatch;
	}
	// 8250B4C0: 817C0040  lwz r11, 0x40(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(64 as u32) ) } as u64;
	// 8250B4C4: 2F1AFFFF  cmpwi cr6, r26, -1
	ctx.cr[6].compare_i32(ctx.r[26].s32, -1, &mut ctx.xer);
	// 8250B4C8: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8250B4CC: 917C0040  stw r11, 0x40(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(64 as u32), ctx.r[11].u32 ) };
	// 8250B4D0: 419A0010  beq cr6, 0x8250b4e0
	if ctx.cr[6].eq {
	pc = 0x8250B4E0; continue 'dispatch;
	}
	// 8250B4D4: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 8250B4D8: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8250B4DC: 7F4BE12E  stwx r26, r11, r28
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[28].u32), ctx.r[26].u32) };
	// 8250B4E0: 7D59C02E  lwzx r10, r25, r24
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[25].u32.wrapping_add(ctx.r[24].u32)) } as u64;
	// 8250B4E4: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 8250B4E8: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 8250B4EC: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 8250B4F0: 40980020  bge cr6, 0x8250b510
	if !ctx.cr[6].lt {
	pc = 0x8250B510; continue 'dispatch;
	}
	// 8250B4F4: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 8250B4F8: 392974DC  addi r9, r9, 0x74dc
	ctx.r[9].s64 = ctx.r[9].s64 + 29916;
	// 8250B4FC: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8250B500: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 8250B504: 390B000C  addi r8, r11, 0xc
	ctx.r[8].s64 = ctx.r[11].s64 + 12;
	// 8250B508: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 8250B50C: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 8250B510: 7D7AB850  subf r11, r26, r23
	ctx.r[11].s64 = ctx.r[23].s64 - ctx.r[26].s64;
	// 8250B514: 7EC3B378  mr r3, r22
	ctx.r[3].u64 = ctx.r[22].u64;
	// 8250B518: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 8250B51C: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8250B520: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 8250B524: 99760000  stb r11, 0(r22)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[22].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 8250B528: 382102C0  addi r1, r1, 0x2c0
	ctx.r[1].s64 = ctx.r[1].s64 + 704;
	// 8250B52C: 48029BC4  b 0x825350f0
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250B530(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8250B530 size=368
    let mut pc: u32 = 0x8250B530;
    'dispatch: loop {
        match pc {
            0x8250B530 => {
    //   block [0x8250B530..0x8250B6A0)
	// 8250B530: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250B534: 48029B79  bl 0x825350ac
	ctx.lr = 0x8250B538;
	sub_82535080(ctx, base);
	// 8250B538: DBE1FFB8  stfd f31, -0x48(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-72 as u32), ctx.f[31].u64 ) };
	// 8250B53C: 9421FD40  stwu r1, -0x2c0(r1)
	ea = ctx.r[1].u32.wrapping_add(-704 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250B540: 832D0000  lwz r25, 0(r13)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250B544: 3B400014  li r26, 0x14
	ctx.r[26].s64 = 20;
	// 8250B548: 7C691B78  mr r9, r3
	ctx.r[9].u64 = ctx.r[3].u64;
	// 8250B54C: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 8250B550: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 8250B554: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 8250B558: 7D5AC82E  lwzx r10, r26, r25
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[25].u32)) } as u64;
	// 8250B55C: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 8250B560: 810A000C  lwz r8, 0xc(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 8250B564: 7F0B4040  cmplw cr6, r11, r8
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[8].u32, &mut ctx.xer);
	// 8250B568: 40980020  bge cr6, 0x8250b588
	if !ctx.cr[6].lt {
	pc = 0x8250B588; continue 'dispatch;
	}
	// 8250B56C: 3D008206  lis r8, -0x7dfa
	ctx.r[8].s64 = -2113536000;
	// 8250B570: 39084958  addi r8, r8, 0x4958
	ctx.r[8].s64 = ctx.r[8].s64 + 18776;
	// 8250B574: 910B0000  stw r8, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 8250B578: 7D0C42E6  mftb r8, 0x10c
	ctx.r[8].u64 = crate::rt::rdtsc_u64();
	// 8250B57C: 38EB000C  addi r7, r11, 0xc
	ctx.r[7].s64 = ctx.r[11].s64 + 12;
	// 8250B580: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 8250B584: 90EA0004  stw r7, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 8250B588: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8250B58C: 3B9D0010  addi r28, r29, 0x10
	ctx.r[28].s64 = ctx.r[29].s64 + 16;
	// 8250B590: 3BC90010  addi r30, r9, 0x10
	ctx.r[30].s64 = ctx.r[9].s64 + 16;
	// 8250B594: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8250B598: C00B8CB4  lfs f0, -0x734c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-29516 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8250B59C: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8250B5A0: D01D0000  stfs f0, 0(r29)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), tmp.u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250B6A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8250B6A0 size=276
    let mut pc: u32 = 0x8250B6A0;
    'dispatch: loop {
        match pc {
            0x8250B6A0 => {
    //   block [0x8250B6A0..0x8250B7B4)
	// 8250B6A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250B6A4: 48029A11  bl 0x825350b4
	ctx.lr = 0x8250B6A8;
	sub_82535080(ctx, base);
	// 8250B6A8: DBE1FFC8  stfd f31, -0x38(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-56 as u32), ctx.f[31].u64 ) };
	// 8250B6AC: 9421FD70  stwu r1, -0x290(r1)
	ea = ctx.r[1].u32.wrapping_add(-656 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250B6B0: 836D0000  lwz r27, 0(r13)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250B6B4: 3B800014  li r28, 0x14
	ctx.r[28].s64 = 20;
	// 8250B6B8: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8250B6BC: 7D5CD82E  lwzx r10, r28, r27
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[28].u32.wrapping_add(ctx.r[27].u32)) } as u64;
	// 8250B6C0: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 8250B6C4: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 8250B6C8: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 8250B6CC: 40980020  bge cr6, 0x8250b6ec
	if !ctx.cr[6].lt {
	pc = 0x8250B6EC; continue 'dispatch;
	}
	// 8250B6D0: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 8250B6D4: 39294978  addi r9, r9, 0x4978
	ctx.r[9].s64 = ctx.r[9].s64 + 18808;
	// 8250B6D8: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8250B6DC: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 8250B6E0: 390B000C  addi r8, r11, 0xc
	ctx.r[8].s64 = ctx.r[11].s64 + 12;
	// 8250B6E4: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 8250B6E8: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 8250B6EC: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8250B6F0: 81430010  lwz r10, 0x10(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 8250B6F4: 3BC30010  addi r30, r3, 0x10
	ctx.r[30].s64 = ctx.r[3].s64 + 16;
	// 8250B6F8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8250B6FC: C3EB0DA0  lfs f31, 0xda0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3488 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 8250B700: 816A0008  lwz r11, 8(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250B704: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8250B708: 4E800421  bctrl
	ctx.lr = 0x8250B70C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8250B70C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8250B710: 2F1FFFFF  cmpwi cr6, r31, -1
	ctx.cr[6].compare_i32(ctx.r[31].s32, -1, &mut ctx.xer);
	// 8250B714: 419A0060  beq cr6, 0x8250b774
	if ctx.cr[6].eq {
	pc = 0x8250B774; continue 'dispatch;
	}
	// 8250B718: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250B71C: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8250B720: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8250B724: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8250B728: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 8250B72C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8250B730: 4E800421  bctrl
	ctx.lr = 0x8250B734;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8250B734: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250B738: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8250B73C: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8250B740: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8250B744: 4E800421  bctrl
	ctx.lr = 0x8250B748;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8250B748: EC1F0828  fsubs f0, f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = (((ctx.f[31].f64 - ctx.f[1].f64) as f32) as f64);
	// 8250B74C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250B750: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8250B754: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8250B758: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8250B75C: FFE00FEE  fsel f31, f0, f31, f1
	ctx.f[31].f64 = if ctx.f[0].f64 >= 0.0 { ctx.f[31].f64 } else { ctx.f[1].f64 };
	// 8250B760: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8250B764: 4E800421  bctrl
	ctx.lr = 0x8250B768;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8250B768: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8250B76C: 2F1FFFFF  cmpwi cr6, r31, -1
	ctx.cr[6].compare_i32(ctx.r[31].s32, -1, &mut ctx.xer);
	// 8250B770: 409AFFA8  bne cr6, 0x8250b718
	if !ctx.cr[6].eq {
	pc = 0x8250B718; continue 'dispatch;
	}
	// 8250B774: 7D5CD82E  lwzx r10, r28, r27
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[28].u32.wrapping_add(ctx.r[27].u32)) } as u64;
	// 8250B778: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 8250B77C: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 8250B780: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 8250B784: 40980020  bge cr6, 0x8250b7a4
	if !ctx.cr[6].lt {
	pc = 0x8250B7A4; continue 'dispatch;
	}
	// 8250B788: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 8250B78C: 392974DC  addi r9, r9, 0x74dc
	ctx.r[9].s64 = ctx.r[9].s64 + 29916;
	// 8250B790: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8250B794: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 8250B798: 390B000C  addi r8, r11, 0xc
	ctx.r[8].s64 = ctx.r[11].s64 + 12;
	// 8250B79C: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 8250B7A0: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 8250B7A4: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 8250B7A8: 38210290  addi r1, r1, 0x290
	ctx.r[1].s64 = ctx.r[1].s64 + 656;
	// 8250B7AC: CBE1FFC8  lfd f31, -0x38(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-56 as u32) ) };
	// 8250B7B0: 48029954  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250B7B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8250B7B8 size=484
    let mut pc: u32 = 0x8250B7B8;
    'dispatch: loop {
        match pc {
            0x8250B7B8 => {
    //   block [0x8250B7B8..0x8250B99C)
	// 8250B7B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250B7BC: 480298ED  bl 0x825350a8
	ctx.lr = 0x8250B7C0;
	sub_82535080(ctx, base);
	// 8250B7C0: 9421FD30  stwu r1, -0x2d0(r1)
	ea = ctx.r[1].u32.wrapping_add(-720 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250B7C4: 830D0000  lwz r24, 0(r13)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250B7C8: 3B200014  li r25, 0x14
	ctx.r[25].s64 = 20;
	// 8250B7CC: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 8250B7D0: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8250B7D4: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 8250B7D8: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 8250B7DC: 7D79C02E  lwzx r11, r25, r24
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[25].u32.wrapping_add(ctx.r[24].u32)) } as u64;
	// 8250B7E0: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8250B7E4: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8250B7E8: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 8250B7EC: 40980020  bge cr6, 0x8250b80c
	if !ctx.cr[6].lt {
	pc = 0x8250B80C; continue 'dispatch;
	}
	// 8250B7F0: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 8250B7F4: 39294948  addi r9, r9, 0x4948
	ctx.r[9].s64 = ctx.r[9].s64 + 18760;
	// 8250B7F8: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8250B7FC: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 8250B800: 390A000C  addi r8, r10, 0xc
	ctx.r[8].s64 = ctx.r[10].s64 + 12;
	// 8250B804: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 8250B808: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 8250B80C: 817D0024  lwz r11, 0x24(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(36 as u32) ) } as u64;
	// 8250B810: 3BDB0010  addi r30, r27, 0x10
	ctx.r[30].s64 = ctx.r[27].s64 + 16;
	// 8250B814: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8250B818: 817B0010  lwz r11, 0x10(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(16 as u32) ) } as u64;
	// 8250B81C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8250B820: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250B824: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8250B828: 409A0088  bne cr6, 0x8250b8b0
	if !ctx.cr[6].eq {
	pc = 0x8250B8B0; continue 'dispatch;
	}
	// 8250B82C: 4E800421  bctrl
	ctx.lr = 0x8250B830;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8250B830: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8250B834: 2F1FFFFF  cmpwi cr6, r31, -1
	ctx.cr[6].compare_i32(ctx.r[31].s32, -1, &mut ctx.xer);
	// 8250B838: 419A012C  beq cr6, 0x8250b964
	if ctx.cr[6].eq {
	pc = 0x8250B964; continue 'dispatch;
	}
	// 8250B83C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250B840: 38A10080  addi r5, r1, 0x80
	ctx.r[5].s64 = ctx.r[1].s64 + 128;
	// 8250B844: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8250B848: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8250B84C: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 8250B850: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8250B854: 4E800421  bctrl
	ctx.lr = 0x8250B858;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8250B858: 817C0008  lwz r11, 8(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250B85C: 9381006C  stw r28, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[28].u32 ) };
	// 8250B860: 7F46D378  mr r6, r26
	ctx.r[6].u64 = ctx.r[26].u64;
	// 8250B864: 93E10064  stw r31, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[31].u32 ) };
	// 8250B868: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 8250B86C: 90610060  stw r3, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[3].u32 ) };
	// 8250B870: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8250B874: 91610068  stw r11, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 8250B878: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250B87C: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 8250B880: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8250B884: 4E800421  bctrl
	ctx.lr = 0x8250B888;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8250B888: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250B88C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8250B890: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8250B894: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8250B898: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8250B89C: 4E800421  bctrl
	ctx.lr = 0x8250B8A0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8250B8A0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8250B8A4: 2F1FFFFF  cmpwi cr6, r31, -1
	ctx.cr[6].compare_i32(ctx.r[31].s32, -1, &mut ctx.xer);
	// 8250B8A8: 409AFF94  bne cr6, 0x8250b83c
	if !ctx.cr[6].eq {
	pc = 0x8250B83C; continue 'dispatch;
	}
	// 8250B8AC: 480000B8  b 0x8250b964
	pc = 0x8250B964; continue 'dispatch;
	// 8250B8B0: 4E800421  bctrl
	ctx.lr = 0x8250B8B4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8250B8B4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8250B8B8: 2F1FFFFF  cmpwi cr6, r31, -1
	ctx.cr[6].compare_i32(ctx.r[31].s32, -1, &mut ctx.xer);
	// 8250B8BC: 419A00A8  beq cr6, 0x8250b964
	if ctx.cr[6].eq {
	pc = 0x8250B964; continue 'dispatch;
	}
	// 8250B8C0: 809D0024  lwz r4, 0x24(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(36 as u32) ) } as u64;
	// 8250B8C4: 7FE8FB78  mr r8, r31
	ctx.r[8].u64 = ctx.r[31].u64;
	// 8250B8C8: 7FC7F378  mr r7, r30
	ctx.r[7].u64 = ctx.r[30].u64;
	// 8250B8CC: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 8250B8D0: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8250B8D4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8250B8D8: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250B8DC: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250B8E0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8250B8E4: 4E800421  bctrl
	ctx.lr = 0x8250B8E8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8250B8E8: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250B8EC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8250B8F0: 419A0050  beq cr6, 0x8250b940
	if ctx.cr[6].eq {
	pc = 0x8250B940; continue 'dispatch;
	}
	// 8250B8F4: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250B8F8: 38A10080  addi r5, r1, 0x80
	ctx.r[5].s64 = ctx.r[1].s64 + 128;
	// 8250B8FC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8250B900: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8250B904: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 8250B908: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8250B90C: 4E800421  bctrl
	ctx.lr = 0x8250B910;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8250B910: 817C0008  lwz r11, 8(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250B914: 9381007C  stw r28, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[28].u32 ) };
	// 8250B918: 7F46D378  mr r6, r26
	ctx.r[6].u64 = ctx.r[26].u64;
	// 8250B91C: 93E10074  stw r31, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[31].u32 ) };
	// 8250B920: 38A10070  addi r5, r1, 0x70
	ctx.r[5].s64 = ctx.r[1].s64 + 112;
	// 8250B924: 90610070  stw r3, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[3].u32 ) };
	// 8250B928: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8250B92C: 91610078  stw r11, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[11].u32 ) };
	// 8250B930: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250B934: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 8250B938: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8250B93C: 4E800421  bctrl
	ctx.lr = 0x8250B940;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8250B940: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250B944: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8250B948: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8250B94C: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8250B950: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8250B954: 4E800421  bctrl
	ctx.lr = 0x8250B958;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8250B958: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8250B95C: 2F1FFFFF  cmpwi cr6, r31, -1
	ctx.cr[6].compare_i32(ctx.r[31].s32, -1, &mut ctx.xer);
	// 8250B960: 409AFF60  bne cr6, 0x8250b8c0
	if !ctx.cr[6].eq {
	pc = 0x8250B8C0; continue 'dispatch;
	}
	// 8250B964: 7D59C02E  lwzx r10, r25, r24
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[25].u32.wrapping_add(ctx.r[24].u32)) } as u64;
	// 8250B968: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 8250B96C: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 8250B970: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 8250B974: 40980020  bge cr6, 0x8250b994
	if !ctx.cr[6].lt {
	pc = 0x8250B994; continue 'dispatch;
	}
	// 8250B978: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 8250B97C: 392974DC  addi r9, r9, 0x74dc
	ctx.r[9].s64 = ctx.r[9].s64 + 29916;
	// 8250B980: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8250B984: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 8250B988: 390B000C  addi r8, r11, 0xc
	ctx.r[8].s64 = ctx.r[11].s64 + 12;
	// 8250B98C: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 8250B990: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 8250B994: 382102D0  addi r1, r1, 0x2d0
	ctx.r[1].s64 = ctx.r[1].s64 + 720;
	// 8250B998: 48029760  b 0x825350f8
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250B9A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8250B9A0 size=8
    let mut pc: u32 = 0x8250B9A0;
    'dispatch: loop {
        match pc {
            0x8250B9A0 => {
    //   block [0x8250B9A0..0x8250B9A8)
	// 8250B9A0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8250B9A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250B9A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8250B9A8 size=20
    let mut pc: u32 = 0x8250B9A8;
    'dispatch: loop {
        match pc {
            0x8250B9A8 => {
    //   block [0x8250B9A8..0x8250B9BC)
	// 8250B9A8: 548B07FE  clrlwi r11, r4, 0x1f
	ctx.r[11].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	// 8250B9AC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8250B9B0: 409A000C  bne cr6, 0x8250b9bc
	if !ctx.cr[6].eq {
		sub_8250B9BC(ctx, base);
		return;
	}
	// 8250B9B4: 60830001  ori r3, r4, 1
	ctx.r[3].u64 = ctx.r[4].u64 | 1;
	// 8250B9B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250B9BC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8250B9BC size=64
    let mut pc: u32 = 0x8250B9BC;
    'dispatch: loop {
        match pc {
            0x8250B9BC => {
    //   block [0x8250B9BC..0x8250B9FC)
	// 8250B9BC: 81230008  lwz r9, 8(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250B9C0: 548AFC7E  rlwinm r10, r4, 0x1f, 0x11, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	// 8250B9C4: 548B843E  srwi r11, r4, 0x10
	ctx.r[11].u32 = ctx.r[4].u32.wrapping_shr(16);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8250B9C8: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8250B9CC: 81090010  lwz r8, 0x10(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(16 as u32) ) } as u64;
	// 8250B9D0: 3908FFFF  addi r8, r8, -1
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	// 8250B9D4: 7F0A4000  cmpw cr6, r10, r8
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[8].s32, &mut ctx.xer);
	// 8250B9D8: 409A0024  bne cr6, 0x8250b9fc
	if !ctx.cr[6].eq {
		sub_8250B9FC(ctx, base);
		return;
	}
	// 8250B9DC: 81290014  lwz r9, 0x14(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(20 as u32) ) } as u64;
	// 8250B9E0: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8250B9E4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8250B9E8: 3929FFFF  addi r9, r9, -1
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	// 8250B9EC: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 8250B9F0: 409A000C  bne cr6, 0x8250b9fc
	if !ctx.cr[6].eq {
		sub_8250B9FC(ctx, base);
		return;
	}
	// 8250B9F4: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 8250B9F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250B9FC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8250B9FC size=16
    let mut pc: u32 = 0x8250B9FC;
    'dispatch: loop {
        match pc {
            0x8250B9FC => {
    //   block [0x8250B9FC..0x8250BA0C)
	// 8250B9FC: 556B7820  slwi r11, r11, 0xf
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(15);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8250BA00: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8250BA04: 5563083C  slwi r3, r11, 1
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 8250BA08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250BA10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8250BA10 size=144
    let mut pc: u32 = 0x8250BA10;
    'dispatch: loop {
        match pc {
            0x8250BA10 => {
    //   block [0x8250BA10..0x8250BAA0)
	// 8250BA10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250BA14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8250BA18: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8250BA1C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8250BA20: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250BA24: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8250BA28: 3BC5FFDC  addi r30, r5, -0x24
	ctx.r[30].s64 = ctx.r[5].s64 + -36;
	// 8250BA2C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8250BA30: 807F0018  lwz r3, 0x18(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 8250BA34: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250BA38: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 8250BA3C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8250BA40: 4E800421  bctrl
	ctx.lr = 0x8250BA44;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8250BA44: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8250BA48: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8250BA4C: 41980038  blt cr6, 0x8250ba84
	if ctx.cr[6].lt {
	pc = 0x8250BA84; continue 'dispatch;
	}
	// 8250BA50: 7F0BF000  cmpw cr6, r11, r30
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[30].s32, &mut ctx.xer);
	// 8250BA54: 41990030  bgt cr6, 0x8250ba84
	if ctx.cr[6].gt {
	pc = 0x8250BA84; continue 'dispatch;
	}
	// 8250BA58: 815F0018  lwz r10, 0x18(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 8250BA5C: 393F0024  addi r9, r31, 0x24
	ctx.r[9].s64 = ctx.r[31].s64 + 36;
	// 8250BA60: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 8250BA64: 409A0014  bne cr6, 0x8250ba78
	if !ctx.cr[6].eq {
	pc = 0x8250BA78; continue 'dispatch;
	}
	// 8250BA68: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8250BA6C: 386B0024  addi r3, r11, 0x24
	ctx.r[3].s64 = ctx.r[11].s64 + 36;
	// 8250BA70: 915F001C  stw r10, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[10].u32 ) };
	// 8250BA74: 48000014  b 0x8250ba88
	pc = 0x8250BA88; continue 'dispatch;
	// 8250BA78: 38600030  li r3, 0x30
	ctx.r[3].s64 = 48;
	// 8250BA7C: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 8250BA80: 48000008  b 0x8250ba88
	pc = 0x8250BA88; continue 'dispatch;
	// 8250BA84: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 8250BA88: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8250BA8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8250BA90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8250BA94: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8250BA98: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8250BA9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250BAA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8250BAA0 size=8
    let mut pc: u32 = 0x8250BAA0;
    'dispatch: loop {
        match pc {
            0x8250BAA0 => {
    //   block [0x8250BAA0..0x8250BAA8)
	// 8250BAA0: 80630018  lwz r3, 0x18(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 8250BAA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250BAA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8250BAA8 size=132
    let mut pc: u32 = 0x8250BAA8;
    'dispatch: loop {
        match pc {
            0x8250BAA8 => {
    //   block [0x8250BAA8..0x8250BB2C)
	// 8250BAA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250BAAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8250BAB0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8250BAB4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8250BAB8: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 8250BABC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250BAC0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8250BAC4: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 8250BAC8: 38800008  li r4, 8
	ctx.r[4].s64 = 8;
	// 8250BACC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8250BAD0: 4BFFF7F1  bl 0x8250b2c0
	ctx.lr = 0x8250BAD4;
	sub_8250B2C0(ctx, base);
	// 8250BAD4: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8250BAD8: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 8250BADC: D3FF0020  stfs f31, 0x20(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), tmp.u32 ) };
	// 8250BAE0: 396B32AC  addi r11, r11, 0x32ac
	ctx.r[11].s64 = ctx.r[11].s64 + 12972;
	// 8250BAE4: 93DF0018  stw r30, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[30].u32 ) };
	// 8250BAE8: 394A3288  addi r10, r10, 0x3288
	ctx.r[10].s64 = ctx.r[10].s64 + 12936;
	// 8250BAEC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8250BAF0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8250BAF4: 915F0010  stw r10, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 8250BAF8: A17E0004  lhz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8250BAFC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8250BB00: 419A0010  beq cr6, 0x8250bb10
	if ctx.cr[6].eq {
	pc = 0x8250BB10; continue 'dispatch;
	}
	// 8250BB04: A17E0006  lhz r11, 6(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(6 as u32) ) } as u64;
	// 8250BB08: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8250BB0C: B17E0006  sth r11, 6(r30)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[30].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 8250BB10: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8250BB14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8250BB18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8250BB1C: CBE1FFE0  lfd f31, -0x20(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 8250BB20: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8250BB24: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8250BB28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250BB30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8250BB30 size=148
    let mut pc: u32 = 0x8250BB30;
    'dispatch: loop {
        match pc {
            0x8250BB30 => {
    //   block [0x8250BB30..0x8250BBC4)
	// 8250BB30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250BB34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8250BB38: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8250BB3C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250BB40: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8250BB44: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8250BB48: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 8250BB4C: 396B32AC  addi r11, r11, 0x32ac
	ctx.r[11].s64 = ctx.r[11].s64 + 12972;
	// 8250BB50: 394A3288  addi r10, r10, 0x3288
	ctx.r[10].s64 = ctx.r[10].s64 + 12936;
	// 8250BB54: 807F0018  lwz r3, 0x18(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 8250BB58: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8250BB5C: 915F0010  stw r10, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 8250BB60: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8250BB64: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8250BB68: 419A0030  beq cr6, 0x8250bb98
	if ctx.cr[6].eq {
	pc = 0x8250BB98; continue 'dispatch;
	}
	// 8250BB6C: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 8250BB70: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8250BB74: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 8250BB78: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8250BB7C: B1630006  sth r11, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 8250BB80: 409A0018  bne cr6, 0x8250bb98
	if !ctx.cr[6].eq {
	pc = 0x8250BB98; continue 'dispatch;
	}
	// 8250BB84: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250BB88: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8250BB8C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250BB90: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8250BB94: 4E800421  bctrl
	ctx.lr = 0x8250BB98;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8250BB98: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8250BB9C: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8250BBA0: 396B2A44  addi r11, r11, 0x2a44
	ctx.r[11].s64 = ctx.r[11].s64 + 10820;
	// 8250BBA4: 394A6DD0  addi r10, r10, 0x6dd0
	ctx.r[10].s64 = ctx.r[10].s64 + 28112;
	// 8250BBA8: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 8250BBAC: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8250BBB0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8250BBB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8250BBB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8250BBBC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8250BBC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250BBC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8250BBC8 size=32
    let mut pc: u32 = 0x8250BBC8;
    'dispatch: loop {
        match pc {
            0x8250BBC8 => {
    //   block [0x8250BBC8..0x8250BBE8)
	// 8250BBC8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8250BBCC: 806B0018  lwz r3, 0x18(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 8250BBD0: C00B0020  lfs f0, 0x20(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8250BBD4: EC20082A  fadds f1, f0, f1
	ctx.f[1].f64 = ((ctx.f[0].f64 + ctx.f[1].f64) as f32) as f64;
	// 8250BBD8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250BBDC: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 8250BBE0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8250BBE4: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250BBE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8250BBE8 size=612
    let mut pc: u32 = 0x8250BBE8;
    'dispatch: loop {
        match pc {
            0x8250BBE8 => {
    //   block [0x8250BBE8..0x8250BE4C)
	// 8250BBE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250BBEC: 480294C1  bl 0x825350ac
	ctx.lr = 0x8250BBF0;
	sub_82535080(ctx, base);
	// 8250BBF0: 3981FFC0  addi r12, r1, -0x40
	ctx.r[12].s64 = ctx.r[1].s64 + -64;
	// 8250BBF4: 4802A3F5  bl 0x82535fe8
	ctx.lr = 0x8250BBF8;
	sub_82535FB0(ctx, base);
	// 8250BBF8: 3981FFA0  addi r12, r1, -0x60
	ctx.r[12].s64 = ctx.r[1].s64 + -96;
	// 8250BBFC: 4802D7A9  bl 0x825393a4
	ctx.lr = 0x8250BC00;
	sub_82539130(ctx, base);
	// 8250BC00: 9421FEE0  stwu r1, -0x120(r1)
	ea = ctx.r[1].u32.wrapping_add(-288 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250BC04: 7C992378  mr r25, r4
	ctx.r[25].u64 = ctx.r[4].u64;
	// 8250BC08: 83E30008  lwz r31, 8(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250BC0C: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 8250BC10: 573DFC7E  rlwinm r29, r25, 0x1f, 0x11, 0x1f
	ctx.r[29].u64 = ctx.r[25].u32 as u64 & 0x00000001u64;
	// 8250BC14: 573C843E  srwi r28, r25, 0x10
	ctx.r[28].u32 = ctx.r[25].u32.wrapping_shr(16);
	ctx.r[28].u64 = ctx.r[28].u32 as u64;
	// 8250BC18: 419A0058  beq cr6, 0x8250bc70
	if ctx.cr[6].eq {
	pc = 0x8250BC70; continue 'dispatch;
	}
	// 8250BC1C: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 8250BC20: C0030010  lfs f0, 0x10(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8250BC24: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8250BC28: D0050010  stfs f0, 0x10(r5)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 8250BC2C: 392A3134  addi r9, r10, 0x3134
	ctx.r[9].s64 = ctx.r[10].s64 + 12596;
	// 8250BC30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8250BC34: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 8250BC38: 38C00006  li r6, 6
	ctx.r[6].s64 = 6;
	// 8250BC3C: 39650050  addi r11, r5, 0x50
	ctx.r[11].s64 = ctx.r[5].s64 + 80;
	// 8250BC40: B1050006  sth r8, 6(r5)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[5].u32.wrapping_add(6 as u32), ctx.r[8].u16 ) };
	// 8250BC44: 91250000  stw r9, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8250BC48: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 8250BC4C: 91450008  stw r10, 8(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8250BC50: 90E5000C  stw r7, 0xc(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(12 as u32), ctx.r[7].u32 ) };
	// 8250BC54: B1450014  sth r10, 0x14(r5)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[5].u32.wrapping_add(20 as u32), ctx.r[10].u16 ) };
	// 8250BC58: 98C50016  stb r6, 0x16(r5)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[5].u32.wrapping_add(22 as u32), ctx.r[6].u8 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250BE50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8250BE50 size=8
    let mut pc: u32 = 0x8250BE50;
    'dispatch: loop {
        match pc {
            0x8250BE50 => {
    //   block [0x8250BE50..0x8250BE58)
	// 8250BE50: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8250BE54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250BE58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8250BE58 size=100
    let mut pc: u32 = 0x8250BE58;
    'dispatch: loop {
        match pc {
            0x8250BE58 => {
    //   block [0x8250BE58..0x8250BEBC)
	// 8250BE58: 3D600842  lis r11, 0x842
	ctx.r[11].s64 = 138543104;
	// 8250BE5C: 81450000  lwz r10, 0(r5)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250BE60: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8250BE64: 61691085  ori r9, r11, 0x1085
	ctx.r[9].u64 = ctx.r[11].u64 | 4229;
	// 8250BE68: 3960001F  li r11, 0x1f
	ctx.r[11].s64 = 31;
	// 8250BE6C: 7D665B96  divwu r11, r6, r11
	ctx.r[11].u32 = ctx.r[6].u32 / ctx.r[11].u32;
	// 8250BE70: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8250BE74: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8250BE78: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8250BE7C: 7CEB282E  lwzx r7, r11, r5
	ctx.r[7].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[5].u32)) } as u64;
	// 8250BE80: 7D664816  mulhwu r11, r6, r9
	ctx.r[11].u64 = ((ctx.r[6].u32 as u64 * ctx.r[9].u32 as u64) >> 32);
	// 8250BE84: 7D4B3050  subf r10, r11, r6
	ctx.r[10].s64 = ctx.r[6].s64 - ctx.r[11].s64;
	// 8250BE88: 554AF87E  srwi r10, r10, 1
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shr(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8250BE8C: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 8250BE90: 556BE13E  srwi r11, r11, 4
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shr(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8250BE94: 556A2834  slwi r10, r11, 5
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(5);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8250BE98: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 8250BE9C: 7D6B3050  subf r11, r11, r6
	ctx.r[11].s64 = ctx.r[6].s64 - ctx.r[11].s64;
	// 8250BEA0: 7D0B5830  slw r11, r8, r11
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[11].u64 = 0;
	} else {
		ctx.r[11].u64 = ((ctx.r[8].u32) << ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	// 8250BEA4: 7D6B3838  and r11, r11, r7
	ctx.r[11].u64 = ctx.r[11].u64 & ctx.r[7].u64;
	// 8250BEA8: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 8250BEAC: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8250BEB0: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 8250BEB4: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 8250BEB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250BEC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8250BEC0 size=144
    let mut pc: u32 = 0x8250BEC0;
    'dispatch: loop {
        match pc {
            0x8250BEC0 => {
    //   block [0x8250BEC0..0x8250BF50)
	// 8250BEC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250BEC4: 480291F9  bl 0x825350bc
	ctx.lr = 0x8250BEC8;
	sub_82535080(ctx, base);
	// 8250BEC8: DBE1FFD8  stfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 8250BECC: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250BED0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8250BED4: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 8250BED8: 38800008  li r4, 8
	ctx.r[4].s64 = 8;
	// 8250BEDC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8250BEE0: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8250BEE4: 4BFFF3DD  bl 0x8250b2c0
	ctx.lr = 0x8250BEE8;
	sub_8250B2C0(ctx, base);
	// 8250BEE8: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8250BEEC: D3FF0020  stfs f31, 0x20(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), tmp.u32 ) };
	// 8250BEF0: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 8250BEF4: 93BF0018  stw r29, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[29].u32 ) };
	// 8250BEF8: 396B3340  addi r11, r11, 0x3340
	ctx.r[11].s64 = ctx.r[11].s64 + 13120;
	// 8250BEFC: 93DF001C  stw r30, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[30].u32 ) };
	// 8250BF00: 394A331C  addi r10, r10, 0x331c
	ctx.r[10].s64 = ctx.r[10].s64 + 13084;
	// 8250BF04: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8250BF08: 915F0010  stw r10, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 8250BF0C: A17E0004  lhz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8250BF10: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8250BF14: 419A0010  beq cr6, 0x8250bf24
	if ctx.cr[6].eq {
	pc = 0x8250BF24; continue 'dispatch;
	}
	// 8250BF18: A17E0006  lhz r11, 6(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(6 as u32) ) } as u64;
	// 8250BF1C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8250BF20: B17E0006  sth r11, 6(r30)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[30].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 8250BF24: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 8250BF28: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8250BF2C: A14B0004  lhz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8250BF30: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8250BF34: 419A0010  beq cr6, 0x8250bf44
	if ctx.cr[6].eq {
	pc = 0x8250BF44; continue 'dispatch;
	}
	// 8250BF38: A14B0006  lhz r10, 6(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(6 as u32) ) } as u64;
	// 8250BF3C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8250BF40: B14B0006  sth r10, 6(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 8250BF44: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8250BF48: CBE1FFD8  lfd f31, -0x28(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-40 as u32) ) };
	// 8250BF4C: 480291C0  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250BF50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8250BF50 size=208
    let mut pc: u32 = 0x8250BF50;
    'dispatch: loop {
        match pc {
            0x8250BF50 => {
    //   block [0x8250BF50..0x8250C020)
	// 8250BF50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250BF54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8250BF58: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8250BF5C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250BF60: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8250BF64: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8250BF68: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 8250BF6C: 396B3340  addi r11, r11, 0x3340
	ctx.r[11].s64 = ctx.r[11].s64 + 13120;
	// 8250BF70: 394A331C  addi r10, r10, 0x331c
	ctx.r[10].s64 = ctx.r[10].s64 + 13084;
	// 8250BF74: 807F001C  lwz r3, 0x1c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 8250BF78: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8250BF7C: 915F0010  stw r10, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 8250BF80: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8250BF84: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8250BF88: 419A0030  beq cr6, 0x8250bfb8
	if ctx.cr[6].eq {
	pc = 0x8250BFB8; continue 'dispatch;
	}
	// 8250BF8C: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 8250BF90: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8250BF94: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 8250BF98: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8250BF9C: B1630006  sth r11, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 8250BFA0: 409A0018  bne cr6, 0x8250bfb8
	if !ctx.cr[6].eq {
	pc = 0x8250BFB8; continue 'dispatch;
	}
	// 8250BFA4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250BFA8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8250BFAC: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250BFB0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8250BFB4: 4E800421  bctrl
	ctx.lr = 0x8250BFB8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8250BFB8: 807F0018  lwz r3, 0x18(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 8250BFBC: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8250BFC0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8250BFC4: 419A0030  beq cr6, 0x8250bff4
	if ctx.cr[6].eq {
	pc = 0x8250BFF4; continue 'dispatch;
	}
	// 8250BFC8: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 8250BFCC: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8250BFD0: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 8250BFD4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8250BFD8: B1630006  sth r11, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 8250BFDC: 409A0018  bne cr6, 0x8250bff4
	if !ctx.cr[6].eq {
	pc = 0x8250BFF4; continue 'dispatch;
	}
	// 8250BFE0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250BFE4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8250BFE8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250BFEC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8250BFF0: 4E800421  bctrl
	ctx.lr = 0x8250BFF4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8250BFF4: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8250BFF8: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8250BFFC: 396B2A44  addi r11, r11, 0x2a44
	ctx.r[11].s64 = ctx.r[11].s64 + 10820;
	// 8250C000: 394A6DD0  addi r10, r10, 0x6dd0
	ctx.r[10].s64 = ctx.r[10].s64 + 28112;
	// 8250C004: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 8250C008: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8250C00C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8250C010: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8250C014: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8250C018: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8250C01C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250C020(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8250C020 size=28
    let mut pc: u32 = 0x8250C020;
    'dispatch: loop {
        match pc {
            0x8250C020 => {
    //   block [0x8250C020..0x8250C03C)
	// 8250C020: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250C024: 38640001  addi r3, r4, 1
	ctx.r[3].s64 = ctx.r[4].s64 + 1;
	// 8250C028: 814B0024  lwz r10, 0x24(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 8250C02C: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 8250C030: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 8250C034: 7F035800  cmpw cr6, r3, r11
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8250C038: 4D980020  bltlr cr6
	if ctx.cr[6].lt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250C03C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8250C03C size=8
    let mut pc: u32 = 0x8250C03C;
    'dispatch: loop {
        match pc {
            0x8250C03C => {
    //   block [0x8250C03C..0x8250C044)
	// 8250C03C: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 8250C040: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250C048(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8250C048 size=704
    let mut pc: u32 = 0x8250C048;
    'dispatch: loop {
        match pc {
            0x8250C048 => {
    //   block [0x8250C048..0x8250C308)
	// 8250C048: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250C04C: 48029049  bl 0x82535094
	ctx.lr = 0x8250C050;
	sub_82535080(ctx, base);
	// 8250C050: 9421FD40  stwu r1, -0x2c0(r1)
	ea = ctx.r[1].u32.wrapping_add(-704 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250C054: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 8250C058: 7C781B78  mr r24, r3
	ctx.r[24].u64 = ctx.r[3].u64;
	// 8250C05C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8250C060: 3AA00000  li r21, 0
	ctx.r[21].s64 = 0;
	// 8250C064: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8250C068: 419A0018  beq cr6, 0x8250c080
	if ctx.cr[6].eq {
	pc = 0x8250C080; continue 'dispatch;
	}
	// 8250C06C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8250C070: C0380010  lfs f1, 0x10(r24)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(16 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8250C074: 4801E32D  bl 0x8252a3a0
	ctx.lr = 0x8250C078;
	sub_8252A3A0(ctx, base);
	// 8250C078: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8250C07C: 48000008  b 0x8250c084
	pc = 0x8250C084; continue 'dispatch;
	// 8250C080: 7EBFAB78  mr r31, r21
	ctx.r[31].u64 = ctx.r[21].u64;
	// 8250C084: 8178000C  lwz r11, 0xc(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(12 as u32) ) } as u64;
	// 8250C088: 395E003B  addi r10, r30, 0x3b
	ctx.r[10].s64 = ctx.r[30].s64 + 59;
	// 8250C08C: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 8250C090: 81780008  lwz r11, 8(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250C094: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 8250C098: 7F1D5840  cmplw cr6, r29, r11
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8250C09C: 409801DC  bge cr6, 0x8250c278
	if !ctx.cr[6].lt {
	pc = 0x8250C278; continue 'dispatch;
	}
	// 8250C0A0: 81780008  lwz r11, 8(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250C0A4: 57A9103A  slwi r9, r29, 2
	ctx.r[9].u32 = ctx.r[29].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 8250C0A8: 55590036  rlwinm r25, r10, 0, 0, 0x1b
	ctx.r[25].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 8250C0AC: 810B0014  lwz r8, 0x14(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 8250C0B0: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250C0B4: 7D68482E  lwzx r11, r8, r9
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 8250C0B8: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8250C0BC: 7FCB5214  add r30, r11, r10
	ctx.r[30].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8250C0C0: 395E0004  addi r10, r30, 4
	ctx.r[10].s64 = ctx.r[30].s64 + 4;
	// 8250C0C4: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250C0C8: 933F0018  stw r25, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[25].u32 ) };
	// 8250C0CC: 92BF001C  stw r21, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[21].u32 ) };
	// 8250C0D0: 915F0024  stw r10, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[10].u32 ) };
	// 8250C0D4: 917F0028  stw r11, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 8250C0D8: 556B003E  slwi r11, r11, 0
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8250C0DC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8250C0E0: 4099021C  ble cr6, 0x8250c2fc
	if !ctx.cr[6].gt {
	pc = 0x8250C2FC; continue 'dispatch;
	}
	// 8250C0E4: 3D600842  lis r11, 0x842
	ctx.r[11].s64 = 138543104;
	// 8250C0E8: 7EB7AB78  mr r23, r21
	ctx.r[23].u64 = ctx.r[21].u64;
	// 8250C0EC: 3BA00002  li r29, 2
	ctx.r[29].s64 = 2;
	// 8250C0F0: 617C1085  ori r28, r11, 0x1085
	ctx.r[28].u64 = ctx.r[11].u64 | 4229;
	// 8250C0F4: 3B400001  li r26, 1
	ctx.r[26].s64 = 1;
	// 8250C0F8: 3B60001F  li r27, 0x1f
	ctx.r[27].s64 = 31;
	// 8250C0FC: 3AC00020  li r22, 0x20
	ctx.r[22].s64 = 32;
	// 8250C100: 3A600030  li r19, 0x30
	ctx.r[19].s64 = 48;
	// 8250C104: 3A800040  li r20, 0x40
	ctx.r[20].s64 = 64;
	// 8250C108: 8178000C  lwz r11, 0xc(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(12 as u32) ) } as u64;
	// 8250C10C: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8250C110: 815F0024  lwz r10, 0x24(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 8250C114: 386B0010  addi r3, r11, 0x10
	ctx.r[3].s64 = ctx.r[11].s64 + 16;
	// 8250C118: 7C8AB82E  lwzx r4, r10, r23
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[23].u32)) } as u64;
	// 8250C11C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250C120: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 8250C124: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8250C128: 4E800421  bctrl
	ctx.lr = 0x8250C12C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8250C12C: 397DFFFE  addi r11, r29, -2
	ctx.r[11].s64 = ctx.r[29].s64 + -2;
	// 8250C130: 813E0000  lwz r9, 0(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250C134: 7D4BDB96  divwu r10, r11, r27
	ctx.r[10].u32 = ctx.r[11].u32 / ctx.r[27].u32;
	// 8250C138: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 8250C13C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8250C140: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8250C144: 7D0AF02E  lwzx r8, r10, r30
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 8250C148: 7D4BE016  mulhwu r10, r11, r28
	ctx.r[10].u64 = ((ctx.r[11].u32 as u64 * ctx.r[28].u32 as u64) >> 32);
	// 8250C14C: 7D2A5850  subf r9, r10, r11
	ctx.r[9].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 8250C150: 5529F87E  srwi r9, r9, 1
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shr(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 8250C154: 7D495214  add r10, r9, r10
	ctx.r[10].u64 = ctx.r[9].u64 + ctx.r[10].u64;
	// 8250C158: 554AE13E  srwi r10, r10, 4
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shr(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8250C15C: 55492834  slwi r9, r10, 5
	ctx.r[9].u32 = ctx.r[10].u32.wrapping_shl(5);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 8250C160: 7D4A4850  subf r10, r10, r9
	ctx.r[10].s64 = ctx.r[9].s64 - ctx.r[10].s64;
	// 8250C164: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 8250C168: 7F4B5830  slw r11, r26, r11
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[11].u64 = 0;
	} else {
		ctx.r[11].u64 = ((ctx.r[26].u32) << ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	// 8250C16C: 7D6B4038  and r11, r11, r8
	ctx.r[11].u64 = ctx.r[11].u64 & ctx.r[8].u64;
	// 8250C170: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8250C174: 419A001C  beq cr6, 0x8250c190
	if ctx.cr[6].eq {
	pc = 0x8250C190; continue 'dispatch;
	}
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250C308(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8250C308 size=52
    let mut pc: u32 = 0x8250C308;
    'dispatch: loop {
        match pc {
            0x8250C308 => {
    //   block [0x8250C308..0x8250C33C)
	// 8250C308: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250C30C: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 8250C310: 7F045840  cmplw cr6, r4, r11
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8250C314: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250C318: 40980024  bge cr6, 0x8250c33c
	if !ctx.cr[6].lt {
		sub_8250C33C(ctx, base);
		return;
	}
	// 8250C31C: 812B0014  lwz r9, 0x14(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 8250C320: 548A103A  slwi r10, r4, 2
	ctx.r[10].u32 = ctx.r[4].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8250C324: 810B0008  lwz r8, 8(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250C328: 7D69502E  lwzx r11, r9, r10
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 8250C32C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8250C330: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8250C334: 7C8B402E  lwzx r4, r11, r8
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 8250C338: 48000018  b 0x8250c350
	sub_8250C33C(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250C33C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8250C33C size=44
    let mut pc: u32 = 0x8250C33C;
    'dispatch: loop {
        match pc {
            0x8250C33C => {
    //   block [0x8250C33C..0x8250C368)
	// 8250C33C: 814B0018  lwz r10, 0x18(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 8250C340: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 8250C344: 7D4A2050  subf r10, r10, r4
	ctx.r[10].s64 = ctx.r[4].s64 - ctx.r[10].s64;
	// 8250C348: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8250C34C: 7C8A582E  lwzx r4, r10, r11
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8250C350: 8163000C  lwz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 8250C354: 386B0010  addi r3, r11, 0x10
	ctx.r[3].s64 = ctx.r[11].s64 + 16;
	// 8250C358: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250C35C: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 8250C360: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8250C364: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250C368(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8250C368 size=296
    let mut pc: u32 = 0x8250C368;
    'dispatch: loop {
        match pc {
            0x8250C368 => {
    //   block [0x8250C368..0x8250C490)
	// 8250C368: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250C36C: 48028D49  bl 0x825350b4
	ctx.lr = 0x8250C370;
	sub_82535080(ctx, base);
	// 8250C370: DBE1FFC8  stfd f31, -0x38(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-56 as u32), ctx.f[31].u64 ) };
	// 8250C374: 9421FD40  stwu r1, -0x2c0(r1)
	ea = ctx.r[1].u32.wrapping_add(-704 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250C378: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8250C37C: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 8250C380: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 8250C384: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8250C388: 3B9D0010  addi r28, r29, 0x10
	ctx.r[28].s64 = ctx.r[29].s64 + 16;
	// 8250C38C: 3BCB0010  addi r30, r11, 0x10
	ctx.r[30].s64 = ctx.r[11].s64 + 16;
	// 8250C390: C00A8CB4  lfs f0, -0x734c(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-29516 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8250C394: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 8250C398: D0010050  stfs f0, 0x50(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 8250C39C: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 8250C3A0: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 8250C3A4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8250C3A8: D0010058  stfs f0, 0x58(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), tmp.u32 ) };
	// 8250C3AC: C1AA1FF8  lfs f13, 0x1ff8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8184 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8250C3B0: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 8250C3B4: D1A1005C  stfs f13, 0x5c(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), tmp.u32 ) };
	// 8250C3B8: D1A1006C  stfs f13, 0x6c(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), tmp.u32 ) };
	// 8250C3BC: C00A0DA0  lfs f0, 0xda0(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(3488 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8250C3C0: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 8250C3C4: D0010060  stfs f0, 0x60(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), tmp.u32 ) };
	// 8250C3C8: D0010064  stfs f0, 0x64(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), tmp.u32 ) };
	// 8250C3CC: D0010068  stfs f0, 0x68(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), tmp.u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250C490(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8250C490 size=220
    let mut pc: u32 = 0x8250C490;
    'dispatch: loop {
        match pc {
            0x8250C490 => {
    //   block [0x8250C490..0x8250C56C)
	// 8250C490: 81630018  lwz r11, 0x18(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 8250C494: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8250C498: 81240000  lwz r9, 0(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250C49C: 81440004  lwz r10, 4(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 8250C4A0: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 8250C4A4: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 8250C4A8: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8250C4AC: 81630018  lwz r11, 0x18(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 8250C4B0: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 8250C4B4: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8250C4B8: 91640004  stw r11, 4(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8250C4BC: 81630018  lwz r11, 0x18(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 8250C4C0: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 8250C4C4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8250C4C8: 40990058  ble cr6, 0x8250c520
	if !ctx.cr[6].gt {
	pc = 0x8250C520; continue 'dispatch;
	}
	// 8250C4CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8250C4D0: 81630018  lwz r11, 0x18(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 8250C4D4: 81040000  lwz r8, 0(r4)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250C4D8: 8144000C  lwz r10, 0xc(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(12 as u32) ) } as u64;
	// 8250C4DC: 80CB0014  lwz r6, 0x14(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 8250C4E0: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250C4E4: 7CC6482E  lwzx r6, r6, r9
	ctx.r[6].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[6].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 8250C4E8: 54C6103A  slwi r6, r6, 2
	ctx.r[6].u32 = ctx.r[6].u32.wrapping_shl(2);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 8250C4EC: 7D66582E  lwzx r11, r6, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[6].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8250C4F0: 7D085A14  add r8, r8, r11
	ctx.r[8].u64 = ctx.r[8].u64 + ctx.r[11].u64;
	// 8250C4F4: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8250C4F8: 91040000  stw r8, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 8250C4FC: 40990008  ble cr6, 0x8250c504
	if !ctx.cr[6].gt {
	pc = 0x8250C504; continue 'dispatch;
	}
	// 8250C500: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 8250C504: 9164000C  stw r11, 0xc(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 8250C508: 38E70001  addi r7, r7, 1
	ctx.r[7].s64 = ctx.r[7].s64 + 1;
	// 8250C50C: 81630018  lwz r11, 0x18(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 8250C510: 39290004  addi r9, r9, 4
	ctx.r[9].s64 = ctx.r[9].s64 + 4;
	// 8250C514: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 8250C518: 7F075800  cmpw cr6, r7, r11
	ctx.cr[6].compare_i32(ctx.r[7].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8250C51C: 4198FFB4  blt cr6, 0x8250c4d0
	if ctx.cr[6].lt {
	pc = 0x8250C4D0; continue 'dispatch;
	}
	// 8250C520: 81240000  lwz r9, 0(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250C524: 81630018  lwz r11, 0x18(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 8250C528: 7D2907B4  extsw r9, r9
	ctx.r[9].s64 = ctx.r[9].s32 as i64;
	// 8250C52C: 81440004  lwz r10, 4(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 8250C530: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 8250C534: F921FFF0  std r9, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[9].u64 ) };
	// 8250C538: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8250C53C: 7D6A07B4  extsw r10, r11
	ctx.r[10].s64 = ctx.r[11].s32 as i64;
	// 8250C540: 91640004  stw r11, 4(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8250C544: C801FFF0  lfd f0, -0x10(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8250C548: F941FFF0  std r10, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[10].u64 ) };
	// 8250C54C: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 8250C550: FC000018  frsp f0, f0
	ctx.f[0].f64 = (ctx.f[0].f64 as f32) as f64;
	// 8250C554: C9A1FFF0  lfd f13, -0x10(r1)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8250C558: FDA06E9C  fcfid f13, f13
	ctx.f[13].f64 = (ctx.f[13].s64 as f64);
	// 8250C55C: FDA06818  frsp f13, f13
	ctx.f[13].f64 = (ctx.f[13].f64 as f32) as f64;
	// 8250C560: EC006824  fdivs f0, f0, f13
	ctx.f[0].f64 = ((ctx.f[0].f64 / ctx.f[13].f64) as f32) as f64;
	// 8250C564: D0040008  stfs f0, 8(r4)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 8250C568: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250C570(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8250C570 size=320
    let mut pc: u32 = 0x8250C570;
    'dispatch: loop {
        match pc {
            0x8250C570 => {
    //   block [0x8250C570..0x8250C6B0)
	// 8250C570: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250C574: 48028B49  bl 0x825350bc
	ctx.lr = 0x8250C578;
	sub_82535080(ctx, base);
	// 8250C578: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250C57C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8250C580: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8250C584: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8250C588: 388B49D4  addi r4, r11, 0x49d4
	ctx.r[4].s64 = ctx.r[11].s64 + 18900;
	// 8250C58C: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 8250C590: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250C594: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8250C598: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8250C59C: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 8250C5A0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8250C5A4: 4E800421  bctrl
	ctx.lr = 0x8250C5A8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8250C5A8: 817E0018  lwz r11, 0x18(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 8250C5AC: 814B001C  lwz r10, 0x1c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 8250C5B0: 55490000  rlwinm r9, r10, 0, 0, 0
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 8250C5B4: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8250C5B8: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 8250C5BC: 3BA949CC  addi r29, r9, 0x49cc
	ctx.r[29].s64 = ctx.r[9].s64 + 18892;
	// 8250C5C0: 409A0030  bne cr6, 0x8250c5f0
	if !ctx.cr[6].eq {
	pc = 0x8250C5F0; continue 'dispatch;
	}
	// 8250C5C4: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250C5C8: 5548103A  slwi r8, r10, 2
	ctx.r[8].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 8250C5CC: 80EB0018  lwz r7, 0x18(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 8250C5D0: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8250C5D4: 80CB0014  lwz r6, 0x14(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 8250C5D8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8250C5DC: 54E7103A  slwi r7, r7, 2
	ctx.r[7].u32 = ctx.r[7].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 8250C5E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8250C5E4: 81690008  lwz r11, 8(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250C5E8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8250C5EC: 4E800421  bctrl
	ctx.lr = 0x8250C5F0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8250C5F0: 817E0018  lwz r11, 0x18(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 8250C5F4: 814B0028  lwz r10, 0x28(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 8250C5F8: 55490000  rlwinm r9, r10, 0, 0, 0
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 8250C5FC: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8250C600: 409A0030  bne cr6, 0x8250c630
	if !ctx.cr[6].eq {
	pc = 0x8250C630; continue 'dispatch;
	}
	// 8250C604: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250C608: 5548103A  slwi r8, r10, 2
	ctx.r[8].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 8250C60C: 80EB0024  lwz r7, 0x24(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 8250C610: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8250C614: 80CB0020  lwz r6, 0x20(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 8250C618: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8250C61C: 54E7103A  slwi r7, r7, 2
	ctx.r[7].u32 = ctx.r[7].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 8250C620: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8250C624: 81690008  lwz r11, 8(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250C628: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8250C62C: 4E800421  bctrl
	ctx.lr = 0x8250C630;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8250C630: 817E0018  lwz r11, 0x18(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 8250C634: 814B0010  lwz r10, 0x10(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 8250C638: 55490000  rlwinm r9, r10, 0, 0, 0
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 8250C63C: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8250C640: 409A0030  bne cr6, 0x8250c670
	if !ctx.cr[6].eq {
	pc = 0x8250C670; continue 'dispatch;
	}
	// 8250C644: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250C648: 5548103A  slwi r8, r10, 2
	ctx.r[8].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 8250C64C: 80EB000C  lwz r7, 0xc(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8250C650: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8250C654: 80CB0008  lwz r6, 8(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250C658: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8250C65C: 54E7103A  slwi r7, r7, 2
	ctx.r[7].u32 = ctx.r[7].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 8250C660: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8250C664: 81690008  lwz r11, 8(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250C668: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8250C66C: 4E800421  bctrl
	ctx.lr = 0x8250C670;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8250C670: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250C674: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8250C678: 80DE001C  lwz r6, 0x1c(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 8250C67C: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8250C680: 388B49C0  addi r4, r11, 0x49c0
	ctx.r[4].s64 = ctx.r[11].s64 + 18880;
	// 8250C684: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8250C688: 816A000C  lwz r11, 0xc(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 8250C68C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8250C690: 4E800421  bctrl
	ctx.lr = 0x8250C694;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8250C694: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250C698: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8250C69C: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 8250C6A0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8250C6A4: 4E800421  bctrl
	ctx.lr = 0x8250C6A8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8250C6A8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8250C6AC: 48028A60  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250C6B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8250C6B0 size=4
    let mut pc: u32 = 0x8250C6B0;
    'dispatch: loop {
        match pc {
            0x8250C6B0 => {
    //   block [0x8250C6B0..0x8250C6B4)
	// 8250C6B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250C6B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8250C6B8 size=12
    let mut pc: u32 = 0x8250C6B8;
    'dispatch: loop {
        match pc {
            0x8250C6B8 => {
    //   block [0x8250C6B8..0x8250C6C4)
	// 8250C6B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8250C6BC: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 8250C6C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250C6C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8250C6C8 size=364
    let mut pc: u32 = 0x8250C6C8;
    'dispatch: loop {
        match pc {
            0x8250C6C8 => {
    //   block [0x8250C6C8..0x8250C834)
	// 8250C6C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250C6CC: 480289E1  bl 0x825350ac
	ctx.lr = 0x8250C6D0;
	sub_82535080(ctx, base);
	// 8250C6D0: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250C6D4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8250C6D8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8250C6DC: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8250C6E0: 39000017  li r8, 0x17
	ctx.r[8].s64 = 23;
	// 8250C6E4: 394B35A8  addi r10, r11, 0x35a8
	ctx.r[10].s64 = ctx.r[11].s64 + 13736;
	// 8250C6E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8250C6EC: B13E0006  sth r9, 6(r30)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[30].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 8250C6F0: 3BFE0010  addi r31, r30, 0x10
	ctx.r[31].s64 = ctx.r[30].s64 + 16;
	// 8250C6F4: 3D208000  lis r9, -0x8000
	ctx.r[9].s64 = -2147483648;
	// 8250C6F8: 911E000C  stw r8, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 8250C6FC: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 8250C700: 915E0000  stw r10, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8250C704: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8250C708: 917E0008  stw r11, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8250C70C: 3B3E001C  addi r25, r30, 0x1c
	ctx.r[25].s64 = ctx.r[30].s64 + 28;
	// 8250C710: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8250C714: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8250C718: 913F0008  stw r9, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 8250C71C: D03E001C  stfs f1, 0x1c(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 8250C720: 835F0004  lwz r26, 4(r31)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8250C724: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250C728: 7F9ADA14  add r28, r26, r27
	ctx.r[28].u64 = ctx.r[26].u64 + ctx.r[27].u64;
	// 8250C72C: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 8250C730: 7F0BE000  cmpw cr6, r11, r28
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[28].s32, &mut ctx.xer);
	// 8250C734: 40980024  bge cr6, 0x8250c758
	if !ctx.cr[6].lt {
	pc = 0x8250C758; continue 'dispatch;
	}
	// 8250C738: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8250C73C: 7F1C5800  cmpw cr6, r28, r11
	ctx.cr[6].compare_i32(ctx.r[28].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8250C740: 41980008  blt cr6, 0x8250c748
	if ctx.cr[6].lt {
	pc = 0x8250C748; continue 'dispatch;
	}
	// 8250C744: 7F8BE378  mr r11, r28
	ctx.r[11].u64 = ctx.r[28].u64;
	// 8250C748: 38A00020  li r5, 0x20
	ctx.r[5].s64 = 32;
	// 8250C74C: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 8250C750: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8250C754: 4BF61B75  bl 0x8246e2c8
	ctx.lr = 0x8250C758;
	sub_8246E2C8(ctx, base);
	// 8250C758: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250C75C: 395BFFFF  addi r10, r27, -1
	ctx.r[10].s64 = ctx.r[27].s64 + -1;
	// 8250C760: 574B2834  slwi r11, r26, 5
	ctx.r[11].u32 = ctx.r[26].u32.wrapping_shl(5);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8250C764: 939F0004  stw r28, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 8250C768: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8250C76C: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 8250C770: 419800B8  blt cr6, 0x8250c828
	if ctx.cr[6].lt {
	pc = 0x8250C828; continue 'dispatch;
	}
	// 8250C774: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 8250C778: 39000010  li r8, 0x10
	ctx.r[8].s64 = 16;
	// 8250C77C: C0091850  lfs f0, 0x1850(r9)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(6224 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250C838(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8250C838 size=172
    let mut pc: u32 = 0x8250C838;
    'dispatch: loop {
        match pc {
            0x8250C838 => {
    //   block [0x8250C838..0x8250C8E4)
	// 8250C838: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250C83C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8250C840: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8250C844: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8250C848: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250C84C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8250C850: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8250C854: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8250C858: 388B4A08  addi r4, r11, 0x4a08
	ctx.r[4].s64 = ctx.r[11].s64 + 18952;
	// 8250C85C: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 8250C860: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250C864: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8250C868: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8250C86C: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 8250C870: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8250C874: 4E800421  bctrl
	ctx.lr = 0x8250C878;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8250C878: 817E0018  lwz r11, 0x18(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 8250C87C: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 8250C880: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8250C884: 409A0034  bne cr6, 0x8250c8b8
	if !ctx.cr[6].eq {
	pc = 0x8250C8B8; continue 'dispatch;
	}
	// 8250C888: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250C88C: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 8250C890: 80FE0014  lwz r7, 0x14(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 8250C894: 55682834  slwi r8, r11, 5
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shl(5);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 8250C898: 80DE0010  lwz r6, 0x10(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 8250C89C: 388A4A00  addi r4, r10, 0x4a00
	ctx.r[4].s64 = ctx.r[10].s64 + 18944;
	// 8250C8A0: 54E72834  slwi r7, r7, 5
	ctx.r[7].u32 = ctx.r[7].u32.wrapping_shl(5);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 8250C8A4: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8250C8A8: 81690008  lwz r11, 8(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250C8AC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8250C8B0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8250C8B4: 4E800421  bctrl
	ctx.lr = 0x8250C8B8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8250C8B8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250C8BC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8250C8C0: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 8250C8C4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8250C8C8: 4E800421  bctrl
	ctx.lr = 0x8250C8CC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8250C8CC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8250C8D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8250C8D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8250C8D8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8250C8DC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8250C8E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250C8E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8250C8E8 size=476
    let mut pc: u32 = 0x8250C8E8;
    'dispatch: loop {
        match pc {
            0x8250C8E8 => {
    //   block [0x8250C8E8..0x8250CAC4)
	// 8250C8E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250C8EC: 480287C1  bl 0x825350ac
	ctx.lr = 0x8250C8F0;
	sub_82535080(ctx, base);
	// 8250C8F0: 9421FD20  stwu r1, -0x2e0(r1)
	ea = ctx.r[1].u32.wrapping_add(-736 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250C8F4: 396100A0  addi r11, r1, 0xa0
	ctx.r[11].s64 = ctx.r[1].s64 + 160;
	// 8250C8F8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8250C8FC: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8250C900: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 8250C904: 91610090  stw r11, 0x90(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(144 as u32), ctx.r[11].u32 ) };
	// 8250C908: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8250C90C: 83BE0014  lwz r29, 0x14(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 8250C910: 2F1D0010  cmpwi cr6, r29, 0x10
	ctx.cr[6].compare_i32(ctx.r[29].s32, 16, &mut ctx.xer);
	// 8250C914: 91610094  stw r11, 0x94(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(148 as u32), ctx.r[11].u32 ) };
	// 8250C918: 3D608000  lis r11, -0x8000
	ctx.r[11].s64 = -2147483648;
	// 8250C91C: 616B0010  ori r11, r11, 0x10
	ctx.r[11].u64 = ctx.r[11].u64 | 16;
	// 8250C920: 91610098  stw r11, 0x98(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(152 as u32), ctx.r[11].u32 ) };
	// 8250C924: 40990020  ble cr6, 0x8250c944
	if !ctx.cr[6].gt {
	pc = 0x8250C944; continue 'dispatch;
	}
	// 8250C928: 2F1D0020  cmpwi cr6, r29, 0x20
	ctx.cr[6].compare_i32(ctx.r[29].s32, 32, &mut ctx.xer);
	// 8250C92C: 38800020  li r4, 0x20
	ctx.r[4].s64 = 32;
	// 8250C930: 41980008  blt cr6, 0x8250c938
	if ctx.cr[6].lt {
	pc = 0x8250C938; continue 'dispatch;
	}
	// 8250C934: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8250C938: 38A00020  li r5, 0x20
	ctx.r[5].s64 = 32;
	// 8250C93C: 38610090  addi r3, r1, 0x90
	ctx.r[3].s64 = ctx.r[1].s64 + 144;
	// 8250C940: 4BF61989  bl 0x8246e2c8
	ctx.lr = 0x8250C944;
	sub_8246E2C8(ctx, base);
	// 8250C944: 391F0030  addi r8, r31, 0x30
	ctx.r[8].s64 = ctx.r[31].s64 + 48;
	// 8250C948: 93A10094  stw r29, 0x94(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(148 as u32), ctx.r[29].u32 ) };
	// 8250C94C: EBBF0000  ld r29, 0(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) };
	// 8250C950: 395F0010  addi r10, r31, 0x10
	ctx.r[10].s64 = ctx.r[31].s64 + 16;
	// 8250C954: 393F0020  addi r9, r31, 0x20
	ctx.r[9].s64 = ctx.r[31].s64 + 32;
	// 8250C958: EBFF0008  ld r31, 8(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) };
	// 8250C95C: 38E10070  addi r7, r1, 0x70
	ctx.r[7].s64 = ctx.r[1].s64 + 112;
	// 8250C960: 817E0014  lwz r11, 0x14(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 8250C964: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 8250C968: 83DE0010  lwz r30, 0x10(r30)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 8250C96C: EB280000  ld r25, 0(r8)
	ctx.r[25].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) };
	// 8250C970: 38C10080  addi r6, r1, 0x80
	ctx.r[6].s64 = ctx.r[1].s64 + 128;
	// 8250C974: E9080008  ld r8, 8(r8)
	ctx.r[8].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[8].u32.wrapping_add(8 as u32) ) };
	// 8250C978: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8250C97C: EB6A0000  ld r27, 0(r10)
	ctx.r[27].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	// 8250C980: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8250C984: FBA70000  std r29, 0(r7)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), ctx.r[29].u64 ) };
	// 8250C988: FBE70008  std r31, 8(r7)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[7].u32.wrapping_add(8 as u32), ctx.r[31].u64 ) };
	// 8250C98C: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8250C990: FB240000  std r25, 0(r4)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[25].u64 ) };
	// 8250C994: F9040008  std r8, 8(r4)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[4].u32.wrapping_add(8 as u32), ctx.r[8].u64 ) };
	// 8250C998: 39010070  addi r8, r1, 0x70
	ctx.r[8].s64 = ctx.r[1].s64 + 112;
	// 8250C99C: E94A0008  ld r10, 8(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) };
	// 8250C9A0: 55632036  slwi r3, r11, 4
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 8250C9A4: FB660000  std r27, 0(r6)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[27].u64 ) };
	// 8250C9A8: EB490000  ld r26, 0(r9)
	ctx.r[26].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) };
	// 8250C9AC: E9290008  ld r9, 8(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250CAC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8250CAC8 size=124
    let mut pc: u32 = 0x8250CAC8;
    'dispatch: loop {
        match pc {
            0x8250CAC8 => {
    //   block [0x8250CAC8..0x8250CB44)
	// 8250CAC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250CACC: 480285E5  bl 0x825350b0
	ctx.lr = 0x8250CAD0;
	sub_82535080(ctx, base);
	// 8250CAD0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250CAD4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8250CAD8: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 8250CADC: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8250CAE0: 815D0028  lwz r10, 0x28(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(40 as u32) ) } as u64;
	// 8250CAE4: 98BD0040  stb r5, 0x40(r29)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[29].u32.wrapping_add(64 as u32), ctx.r[5].u8 ) };
	// 8250CAE8: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8250CAEC: 40990048  ble cr6, 0x8250cb34
	if !ctx.cr[6].gt {
	pc = 0x8250CB34; continue 'dispatch;
	}
	// 8250CAF0: 3965FFFB  addi r11, r5, -5
	ctx.r[11].s64 = ctx.r[5].s64 + -5;
	// 8250CAF4: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8250CAF8: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 8250CAFC: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8250CB00: 697B0001  xori r27, r11, 1
	ctx.r[27].u64 = ctx.r[11].u64 ^ 1;
	// 8250CB04: 817D0024  lwz r11, 0x24(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(36 as u32) ) } as u64;
	// 8250CB08: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 8250CB0C: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 8250CB10: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8250CB14: 7F9E5A14  add r28, r30, r11
	ctx.r[28].u64 = ctx.r[30].u64 + ctx.r[11].u64;
	// 8250CB18: 4801D2D9  bl 0x82529df0
	ctx.lr = 0x8250CB1C;
	sub_82529DF0(ctx, base);
	// 8250CB1C: B07C000C  sth r3, 0xc(r28)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[28].u32.wrapping_add(12 as u32), ctx.r[3].u16 ) };
	// 8250CB20: 817D0028  lwz r11, 0x28(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(40 as u32) ) } as u64;
	// 8250CB24: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 8250CB28: 3BDE0010  addi r30, r30, 0x10
	ctx.r[30].s64 = ctx.r[30].s64 + 16;
	// 8250CB2C: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8250CB30: 4198FFD4  blt cr6, 0x8250cb04
	if ctx.cr[6].lt {
	pc = 0x8250CB04; continue 'dispatch;
	}
	// 8250CB34: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8250CB38: 997D0014  stb r11, 0x14(r29)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[29].u32.wrapping_add(20 as u32), ctx.r[11].u8 ) };
	// 8250CB3C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8250CB40: 480285C0  b 0x82535100
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250CB48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8250CB48 size=268
    let mut pc: u32 = 0x8250CB48;
    'dispatch: loop {
        match pc {
            0x8250CB48 => {
    //   block [0x8250CB48..0x8250CC54)
	// 8250CB48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250CB4C: 48028565  bl 0x825350b0
	ctx.lr = 0x8250CB50;
	sub_82535080(ctx, base);
	// 8250CB50: DBE1FFC0  stfd f31, -0x40(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-64 as u32), ctx.f[31].u64 ) };
	// 8250CB54: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250CB58: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8250CB5C: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 8250CB60: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 8250CB64: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 8250CB68: 3BDF0010  addi r30, r31, 0x10
	ctx.r[30].s64 = ctx.r[31].s64 + 16;
	// 8250CB6C: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 8250CB70: C00B8CB4  lfs f0, -0x734c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-29516 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8250CB74: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8250CB78: D0010050  stfs f0, 0x50(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 8250CB7C: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 8250CB80: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 8250CB84: D0010058  stfs f0, 0x58(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), tmp.u32 ) };
	// 8250CB88: C1AB1FF8  lfs f13, 0x1ff8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8250CB8C: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8250CB90: D1A1005C  stfs f13, 0x5c(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), tmp.u32 ) };
	// 8250CB94: D1A1006C  stfs f13, 0x6c(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), tmp.u32 ) };
	// 8250CB98: C00B0DA0  lfs f0, 0xda0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3488 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8250CB9C: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 8250CBA0: D0010060  stfs f0, 0x60(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), tmp.u32 ) };
	// 8250CBA4: D0010064  stfs f0, 0x64(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), tmp.u32 ) };
	// 8250CBA8: D0010068  stfs f0, 0x68(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), tmp.u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250CC58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8250CC58 size=156
    let mut pc: u32 = 0x8250CC58;
    'dispatch: loop {
        match pc {
            0x8250CC58 => {
    //   block [0x8250CC58..0x8250CCF4)
	// 8250CC58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250CC5C: 4802845D  bl 0x825350b8
	ctx.lr = 0x8250CC60;
	sub_82535080(ctx, base);
	// 8250CC60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250CC64: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8250CC68: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 8250CC6C: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 8250CC70: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8250CC74: 40990068  ble cr6, 0x8250ccdc
	if !ctx.cr[6].gt {
	pc = 0x8250CCDC; continue 'dispatch;
	}
	// 8250CC78: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8250CC7C: 3FA08282  lis r29, -0x7d7e
	ctx.r[29].s64 = -2105409536;
	// 8250CC80: 815F0014  lwz r10, 0x14(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 8250CC84: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8250CC88: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250CC8C: C03DE098  lfs f1, -0x1f68(r29)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(-8040 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8250CC90: 7D5E5214  add r10, r30, r10
	ctx.r[10].u64 = ctx.r[30].u64 + ctx.r[10].u64;
	// 8250CC94: 812A0008  lwz r9, 8(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250CC98: 80EA0004  lwz r7, 4(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 8250CC9C: 814A0000  lwz r10, 0(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250CCA0: 55282036  slwi r8, r9, 4
	ctx.r[8].u32 = ctx.r[9].u32.wrapping_shl(4);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 8250CCA4: 54E92036  slwi r9, r7, 4
	ctx.r[9].u32 = ctx.r[7].u32.wrapping_shl(4);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 8250CCA8: 554A2036  slwi r10, r10, 4
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8250CCAC: 7CC85A14  add r6, r8, r11
	ctx.r[6].u64 = ctx.r[8].u64 + ctx.r[11].u64;
	// 8250CCB0: 7CA95A14  add r5, r9, r11
	ctx.r[5].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 8250CCB4: 7C8A5A14  add r4, r10, r11
	ctx.r[4].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 8250CCB8: 4801D489  bl 0x8252a140
	ctx.lr = 0x8250CCBC;
	sub_8252A140(ctx, base);
	// 8250CCBC: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250CCC0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8250CCC4: 419A0024  beq cr6, 0x8250cce8
	if ctx.cr[6].eq {
	pc = 0x8250CCE8; continue 'dispatch;
	}
	// 8250CCC8: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 8250CCCC: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 8250CCD0: 3BDE0010  addi r30, r30, 0x10
	ctx.r[30].s64 = ctx.r[30].s64 + 16;
	// 8250CCD4: 7F1C5800  cmpw cr6, r28, r11
	ctx.cr[6].compare_i32(ctx.r[28].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8250CCD8: 4198FFA8  blt cr6, 0x8250cc80
	if ctx.cr[6].lt {
	pc = 0x8250CC80; continue 'dispatch;
	}
	// 8250CCDC: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 8250CCE0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8250CCE4: 48028424  b 0x82535108
	sub_825350D0(ctx, base);
	return;
	// 8250CCE8: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8250CCEC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8250CCF0: 48028418  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250CCF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8250CCF8 size=156
    let mut pc: u32 = 0x8250CCF8;
    'dispatch: loop {
        match pc {
            0x8250CCF8 => {
    //   block [0x8250CCF8..0x8250CD94)
	// 8250CCF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250CCFC: 480283BD  bl 0x825350b8
	ctx.lr = 0x8250CD00;
	sub_82535080(ctx, base);
	// 8250CD00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250CD04: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8250CD08: 3B840001  addi r28, r4, 1
	ctx.r[28].s64 = ctx.r[4].s64 + 1;
	// 8250CD0C: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 8250CD10: 7F1C5800  cmpw cr6, r28, r11
	ctx.cr[6].compare_i32(ctx.r[28].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8250CD14: 40980068  bge cr6, 0x8250cd7c
	if !ctx.cr[6].lt {
	pc = 0x8250CD7C; continue 'dispatch;
	}
	// 8250CD18: 579E2036  slwi r30, r28, 4
	ctx.r[30].u32 = ctx.r[28].u32.wrapping_shl(4);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 8250CD1C: 3FA08282  lis r29, -0x7d7e
	ctx.r[29].s64 = -2105409536;
	// 8250CD20: 815F0014  lwz r10, 0x14(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 8250CD24: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8250CD28: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250CD2C: C03DE098  lfs f1, -0x1f68(r29)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(-8040 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8250CD30: 7D4AF214  add r10, r10, r30
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[30].u64;
	// 8250CD34: 812A0008  lwz r9, 8(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250CD38: 80EA0004  lwz r7, 4(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 8250CD3C: 814A0000  lwz r10, 0(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250CD40: 55282036  slwi r8, r9, 4
	ctx.r[8].u32 = ctx.r[9].u32.wrapping_shl(4);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 8250CD44: 54E92036  slwi r9, r7, 4
	ctx.r[9].u32 = ctx.r[7].u32.wrapping_shl(4);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 8250CD48: 554A2036  slwi r10, r10, 4
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8250CD4C: 7CC85A14  add r6, r8, r11
	ctx.r[6].u64 = ctx.r[8].u64 + ctx.r[11].u64;
	// 8250CD50: 7CA95A14  add r5, r9, r11
	ctx.r[5].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 8250CD54: 7C8A5A14  add r4, r10, r11
	ctx.r[4].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 8250CD58: 4801D3E9  bl 0x8252a140
	ctx.lr = 0x8250CD5C;
	sub_8252A140(ctx, base);
	// 8250CD5C: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250CD60: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8250CD64: 419A0024  beq cr6, 0x8250cd88
	if ctx.cr[6].eq {
	pc = 0x8250CD88; continue 'dispatch;
	}
	// 8250CD68: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 8250CD6C: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 8250CD70: 3BDE0010  addi r30, r30, 0x10
	ctx.r[30].s64 = ctx.r[30].s64 + 16;
	// 8250CD74: 7F1C5800  cmpw cr6, r28, r11
	ctx.cr[6].compare_i32(ctx.r[28].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8250CD78: 4198FFA8  blt cr6, 0x8250cd20
	if ctx.cr[6].lt {
	pc = 0x8250CD20; continue 'dispatch;
	}
	// 8250CD7C: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 8250CD80: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8250CD84: 48028384  b 0x82535108
	sub_825350D0(ctx, base);
	return;
	// 8250CD88: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8250CD8C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8250CD90: 48028378  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250CD98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8250CD98 size=144
    let mut pc: u32 = 0x8250CD98;
    'dispatch: loop {
        match pc {
            0x8250CD98 => {
    //   block [0x8250CD98..0x8250CE28)
	// 8250CD98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250CD9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8250CDA0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8250CDA4: DBE1FFE8  stfd f31, -0x18(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.f[31].u64 ) };
	// 8250CDA8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250CDAC: 38800016  li r4, 0x16
	ctx.r[4].s64 = 22;
	// 8250CDB0: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 8250CDB4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8250CDB8: 4BFFE509  bl 0x8250b2c0
	ctx.lr = 0x8250CDBC;
	sub_8250B2C0(ctx, base);
	// 8250CDBC: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8250CDC0: 3D408000  lis r10, -0x8000
	ctx.r[10].s64 = -2147483648;
	// 8250CDC4: 392B3670  addi r9, r11, 0x3670
	ctx.r[9].s64 = ctx.r[11].s64 + 13936;
	// 8250CDC8: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8250CDCC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8250CDD0: 390B364C  addi r8, r11, 0x364c
	ctx.r[8].s64 = ctx.r[11].s64 + 13900;
	// 8250CDD4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8250CDD8: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8250CDDC: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 8250CDE0: 911F0010  stw r8, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[8].u32 ) };
	// 8250CDE4: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 8250CDE8: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 8250CDEC: 915F0020  stw r10, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 8250CDF0: 917F0024  stw r11, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 8250CDF4: 917F0028  stw r11, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 8250CDF8: 915F002C  stw r10, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[10].u32 ) };
	// 8250CDFC: 917F0030  stw r11, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 8250CE00: 917F0034  stw r11, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 8250CE04: 915F0038  stw r10, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[10].u32 ) };
	// 8250CE08: D3FF003C  stfs f31, 0x3c(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), tmp.u32 ) };
	// 8250CE0C: 993F0040  stb r9, 0x40(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), ctx.r[9].u8 ) };
	// 8250CE10: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8250CE14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8250CE18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8250CE1C: CBE1FFE8  lfd f31, -0x18(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8250CE20: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8250CE24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250CE28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8250CE28 size=224
    let mut pc: u32 = 0x8250CE28;
    'dispatch: loop {
        match pc {
            0x8250CE28 => {
    //   block [0x8250CE28..0x8250CF08)
	// 8250CE28: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 8250CE2C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8250CE30: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 8250CE34: 419A0068  beq cr6, 0x8250ce9c
	if ctx.cr[6].eq {
	pc = 0x8250CE9C; continue 'dispatch;
	}
	// 8250CE38: 80EB0014  lwz r7, 0x14(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 8250CE3C: 54882036  slwi r8, r4, 4
	ctx.r[8].u32 = ctx.r[4].u32.wrapping_shl(4);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 8250CE40: 892B0030  lbz r9, 0x30(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 8250CE44: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8250CE48: 7CE83A14  add r7, r8, r7
	ctx.r[7].u64 = ctx.r[8].u64 + ctx.r[7].u64;
	// 8250CE4C: C00B002C  lfs f0, 0x2c(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8250CE50: 3D008206  lis r8, -0x7dfa
	ctx.r[8].s64 = -2113536000;
	// 8250CE54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8250CE58: 39083134  addi r8, r8, 0x3134
	ctx.r[8].s64 = ctx.r[8].s64 + 12596;
	// 8250CE5C: 3BE00004  li r31, 4
	ctx.r[31].s64 = 4;
	// 8250CE60: A0E7000C  lhz r7, 0xc(r7)
	ctx.r[7].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[7].u32.wrapping_add(12 as u32) ) } as u64;
	// 8250CE64: 39450050  addi r10, r5, 0x50
	ctx.r[10].s64 = ctx.r[5].s64 + 80;
	// 8250CE68: B0650006  sth r3, 6(r5)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[5].u32.wrapping_add(6 as u32), ctx.r[3].u16 ) };
	// 8250CE6C: D0050010  stfs f0, 0x10(r5)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 8250CE70: 90C50008  stw r6, 8(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(8 as u32), ctx.r[6].u32 ) };
	// 8250CE74: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 8250CE78: 91050000  stw r8, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 8250CE7C: 93E5000C  stw r31, 0xc(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 8250CE80: 99250016  stb r9, 0x16(r5)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[5].u32.wrapping_add(22 as u32), ctx.r[9].u8 ) };
	// 8250CE84: B0E50014  sth r7, 0x14(r5)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[5].u32.wrapping_add(20 as u32), ctx.r[7].u16 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250CF08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8250CF08 size=296
    let mut pc: u32 = 0x8250CF08;
    'dispatch: loop {
        match pc {
            0x8250CF08 => {
    //   block [0x8250CF08..0x8250D030)
	// 8250CF08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250CF0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8250CF10: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8250CF14: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8250CF18: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250CF1C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8250CF20: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8250CF24: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8250CF28: 388B4A38  addi r4, r11, 0x4a38
	ctx.r[4].s64 = ctx.r[11].s64 + 19000;
	// 8250CF2C: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 8250CF30: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250CF34: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8250CF38: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8250CF3C: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 8250CF40: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8250CF44: 4E800421  bctrl
	ctx.lr = 0x8250CF48;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8250CF48: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 8250CF4C: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 8250CF50: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8250CF54: 409A0034  bne cr6, 0x8250cf88
	if !ctx.cr[6].eq {
	pc = 0x8250CF88; continue 'dispatch;
	}
	// 8250CF58: 813E0000  lwz r9, 0(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250CF5C: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 8250CF60: 80FF001C  lwz r7, 0x1c(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 8250CF64: 55682036  slwi r8, r11, 4
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 8250CF68: 80DF0018  lwz r6, 0x18(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 8250CF6C: 388A4A2C  addi r4, r10, 0x4a2c
	ctx.r[4].s64 = ctx.r[10].s64 + 18988;
	// 8250CF70: 54E72036  slwi r7, r7, 4
	ctx.r[7].u32 = ctx.r[7].u32.wrapping_shl(4);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 8250CF74: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8250CF78: 81690008  lwz r11, 8(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250CF7C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8250CF80: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8250CF84: 4E800421  bctrl
	ctx.lr = 0x8250CF88;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8250CF88: 815F002C  lwz r10, 0x2c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 8250CF8C: 554B0000  rlwinm r11, r10, 0, 0, 0
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 8250CF90: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8250CF94: 409A0034  bne cr6, 0x8250cfc8
	if !ctx.cr[6].eq {
	pc = 0x8250CFC8; continue 'dispatch;
	}
	// 8250CF98: 813E0000  lwz r9, 0(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250CF9C: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8250CFA0: 80FF0028  lwz r7, 0x28(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 8250CFA4: 55482036  slwi r8, r10, 4
	ctx.r[8].u32 = ctx.r[10].u32.wrapping_shl(4);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 8250CFA8: 388B4A20  addi r4, r11, 0x4a20
	ctx.r[4].s64 = ctx.r[11].s64 + 18976;
	// 8250CFAC: 80DF0024  lwz r6, 0x24(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 8250CFB0: 54E72036  slwi r7, r7, 4
	ctx.r[7].u32 = ctx.r[7].u32.wrapping_shl(4);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 8250CFB4: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8250CFB8: 81690008  lwz r11, 8(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250CFBC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8250CFC0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8250CFC4: 4E800421  bctrl
	ctx.lr = 0x8250CFC8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8250CFC8: 815F0038  lwz r10, 0x38(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 8250CFCC: 554B0000  rlwinm r11, r10, 0, 0, 0
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 8250CFD0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8250CFD4: 409A0030  bne cr6, 0x8250d004
	if !ctx.cr[6].eq {
	pc = 0x8250D004; continue 'dispatch;
	}
	// 8250CFD8: 813E0000  lwz r9, 0(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250CFDC: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8250CFE0: 80FF0034  lwz r7, 0x34(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 8250CFE4: 554800BE  clrlwi r8, r10, 2
	ctx.r[8].u64 = ctx.r[10].u32 as u64 & 0x3FFFFFFFu64;
	// 8250CFE8: 388B4A14  addi r4, r11, 0x4a14
	ctx.r[4].s64 = ctx.r[11].s64 + 18964;
	// 8250CFEC: 80DF0030  lwz r6, 0x30(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 8250CFF0: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8250CFF4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8250CFF8: 81690008  lwz r11, 8(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250CFFC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8250D000: 4E800421  bctrl
	ctx.lr = 0x8250D004;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8250D004: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250D008: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8250D00C: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 8250D010: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8250D014: 4E800421  bctrl
	ctx.lr = 0x8250D018;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8250D018: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8250D01C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8250D020: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8250D024: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8250D028: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8250D02C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250D030(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8250D030 size=144
    let mut pc: u32 = 0x8250D030;
    'dispatch: loop {
        match pc {
            0x8250D030 => {
    //   block [0x8250D030..0x8250D0C0)
	// 8250D030: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250D034: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8250D038: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8250D03C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8250D040: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250D044: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8250D048: 3BC5FFD0  addi r30, r5, -0x30
	ctx.r[30].s64 = ctx.r[5].s64 + -48;
	// 8250D04C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8250D050: 807F0018  lwz r3, 0x18(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 8250D054: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250D058: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 8250D05C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8250D060: 4E800421  bctrl
	ctx.lr = 0x8250D064;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8250D064: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8250D068: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8250D06C: 4098000C  bge cr6, 0x8250d078
	if !ctx.cr[6].lt {
	pc = 0x8250D078; continue 'dispatch;
	}
	// 8250D070: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 8250D074: 48000034  b 0x8250d0a8
	pc = 0x8250D0A8; continue 'dispatch;
	// 8250D078: 7F0BF000  cmpw cr6, r11, r30
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[30].s32, &mut ctx.xer);
	// 8250D07C: 4199FFF4  bgt cr6, 0x8250d070
	if ctx.cr[6].gt {
	pc = 0x8250D070; continue 'dispatch;
	}
	// 8250D080: 815F0018  lwz r10, 0x18(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 8250D084: 393F0030  addi r9, r31, 0x30
	ctx.r[9].s64 = ctx.r[31].s64 + 48;
	// 8250D088: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 8250D08C: 409A0014  bne cr6, 0x8250d0a0
	if !ctx.cr[6].eq {
	pc = 0x8250D0A0; continue 'dispatch;
	}
	// 8250D090: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8250D094: 386B0030  addi r3, r11, 0x30
	ctx.r[3].s64 = ctx.r[11].s64 + 48;
	// 8250D098: 915F001C  stw r10, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[10].u32 ) };
	// 8250D09C: 4800000C  b 0x8250d0a8
	pc = 0x8250D0A8; continue 'dispatch;
	// 8250D0A0: 38600030  li r3, 0x30
	ctx.r[3].s64 = 48;
	// 8250D0A4: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 8250D0A8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8250D0AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8250D0B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8250D0B4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8250D0B8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8250D0BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250D0C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8250D0C0 size=20
    let mut pc: u32 = 0x8250D0C0;
    'dispatch: loop {
        match pc {
            0x8250D0C0 => {
    //   block [0x8250D0C0..0x8250D0D4)
	// 8250D0C0: 80630018  lwz r3, 0x18(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 8250D0C4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250D0C8: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 8250D0CC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8250D0D0: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250D0D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8250D0D8 size=144
    let mut pc: u32 = 0x8250D0D8;
    'dispatch: loop {
        match pc {
            0x8250D0D8 => {
    //   block [0x8250D0D8..0x8250D168)
	// 8250D0D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250D0DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8250D0E0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8250D0E4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8250D0E8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250D0EC: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8250D0F0: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8250D0F4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8250D0F8: 388B4A44  addi r4, r11, 0x4a44
	ctx.r[4].s64 = ctx.r[11].s64 + 19012;
	// 8250D0FC: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 8250D100: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250D104: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8250D108: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8250D10C: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 8250D110: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8250D114: 4E800421  bctrl
	ctx.lr = 0x8250D118;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8250D118: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250D11C: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8250D120: 80DE0018  lwz r6, 0x18(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 8250D124: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8250D128: 388B485C  addi r4, r11, 0x485c
	ctx.r[4].s64 = ctx.r[11].s64 + 18524;
	// 8250D12C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8250D130: 816A000C  lwz r11, 0xc(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 8250D134: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8250D138: 4E800421  bctrl
	ctx.lr = 0x8250D13C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8250D13C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250D140: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8250D144: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 8250D148: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8250D14C: 4E800421  bctrl
	ctx.lr = 0x8250D150;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8250D150: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8250D154: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8250D158: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8250D15C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8250D160: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8250D164: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250D168(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8250D168 size=8
    let mut pc: u32 = 0x8250D168;
    'dispatch: loop {
        match pc {
            0x8250D168 => {
    //   block [0x8250D168..0x8250D170)
	// 8250D168: 38630014  addi r3, r3, 0x14
	ctx.r[3].s64 = ctx.r[3].s64 + 20;
	// 8250D16C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250D170(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8250D170 size=104
    let mut pc: u32 = 0x8250D170;
    'dispatch: loop {
        match pc {
            0x8250D170 => {
    //   block [0x8250D170..0x8250D1D8)
	// 8250D170: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250D174: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8250D178: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8250D17C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8250D180: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250D184: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8250D188: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8250D18C: 807F0018  lwz r3, 0x18(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 8250D190: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250D194: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8250D198: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8250D19C: 4E800421  bctrl
	ctx.lr = 0x8250D1A0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8250D1A0: 39600020  li r11, 0x20
	ctx.r[11].s64 = 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250D1D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8250D1D8 size=164
    let mut pc: u32 = 0x8250D1D8;
    'dispatch: loop {
        match pc {
            0x8250D1D8 => {
    //   block [0x8250D1D8..0x8250D27C)
	// 8250D1D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250D1DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8250D1E0: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250D1E4: 39410060  addi r10, r1, 0x60
	ctx.r[10].s64 = ctx.r[1].s64 + 96;
	// 8250D1E8: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 8250D1EC: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 8250D1F0: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 8250D1F4: E92B0000  ld r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 8250D1F8: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 8250D1FC: F92A0000  std r9, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u64 ) };
	// 8250D200: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 8250D204: 4200FFF0  bdnz 0x8250d1f4
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x8250D1F4; continue 'dispatch;
	}
	// 8250D208: 39400020  li r10, 0x20
	ctx.r[10].s64 = 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250D280(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8250D280 size=92
    let mut pc: u32 = 0x8250D280;
    'dispatch: loop {
        match pc {
            0x8250D280 => {
    //   block [0x8250D280..0x8250D2DC)
	// 8250D280: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250D284: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8250D288: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8250D28C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8250D290: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250D294: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8250D298: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 8250D29C: 807E0018  lwz r3, 0x18(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 8250D2A0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250D2A4: 816B0030  lwz r11, 0x30(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 8250D2A8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8250D2AC: 4E800421  bctrl
	ctx.lr = 0x8250D2B0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8250D2B0: 39600020  li r11, 0x20
	ctx.r[11].s64 = 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250D2E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8250D2E0 size=104
    let mut pc: u32 = 0x8250D2E0;
    'dispatch: loop {
        match pc {
            0x8250D2E0 => {
    //   block [0x8250D2E0..0x8250D348)
	// 8250D2E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250D2E4: 48027DD9  bl 0x825350bc
	ctx.lr = 0x8250D2E8;
	sub_82535080(ctx, base);
	// 8250D2E8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250D2EC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8250D2F0: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 8250D2F4: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 8250D2F8: 807E0018  lwz r3, 0x18(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 8250D2FC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250D300: 816B0034  lwz r11, 0x34(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 8250D304: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8250D308: 4E800421  bctrl
	ctx.lr = 0x8250D30C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8250D30C: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 8250D310: 40990030  ble cr6, 0x8250d340
	if !ctx.cr[6].gt {
	pc = 0x8250D340; continue 'dispatch;
	}
	// 8250D314: 393E0020  addi r9, r30, 0x20
	ctx.r[9].s64 = ctx.r[30].s64 + 32;
	// 8250D318: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 8250D31C: 7FEAFB78  mr r10, r31
	ctx.r[10].u64 = ctx.r[31].u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250D348(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8250D348 size=92
    let mut pc: u32 = 0x8250D348;
    'dispatch: loop {
        match pc {
            0x8250D348 => {
    //   block [0x8250D348..0x8250D3A4)
	// 8250D348: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250D34C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8250D350: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8250D354: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8250D358: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250D35C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8250D360: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8250D364: 807E0018  lwz r3, 0x18(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 8250D368: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250D36C: 816B003C  lwz r11, 0x3c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 8250D370: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8250D374: 4E800421  bctrl
	ctx.lr = 0x8250D378;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8250D378: 39600020  li r11, 0x20
	ctx.r[11].s64 = 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250D3A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8250D3A8 size=92
    let mut pc: u32 = 0x8250D3A8;
    'dispatch: loop {
        match pc {
            0x8250D3A8 => {
    //   block [0x8250D3A8..0x8250D404)
	// 8250D3A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250D3AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8250D3B0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8250D3B4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8250D3B8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250D3BC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8250D3C0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8250D3C4: 807E0018  lwz r3, 0x18(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 8250D3C8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250D3CC: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 8250D3D0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8250D3D4: 4E800421  bctrl
	ctx.lr = 0x8250D3D8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8250D3D8: 39600020  li r11, 0x20
	ctx.r[11].s64 = 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250D408(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8250D408 size=136
    let mut pc: u32 = 0x8250D408;
    'dispatch: loop {
        match pc {
            0x8250D408 => {
    //   block [0x8250D408..0x8250D490)
	// 8250D408: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250D40C: 48027CAD  bl 0x825350b8
	ctx.lr = 0x8250D410;
	sub_82535080(ctx, base);
	// 8250D410: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250D414: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8250D418: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 8250D41C: 807D0018  lwz r3, 0x18(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(24 as u32) ) } as u64;
	// 8250D420: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250D424: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 8250D428: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8250D42C: 4E800421  bctrl
	ctx.lr = 0x8250D430;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8250D430: 817D0018  lwz r11, 0x18(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(24 as u32) ) } as u64;
	// 8250D434: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8250D438: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 8250D43C: 7F9EE378  mr r30, r28
	ctx.r[30].u64 = ctx.r[28].u64;
	// 8250D440: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250D444: 816A0024  lwz r11, 0x24(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(36 as u32) ) } as u64;
	// 8250D448: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8250D44C: 4E800421  bctrl
	ctx.lr = 0x8250D450;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8250D450: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8250D454: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8250D458: 4099002C  ble cr6, 0x8250d484
	if !ctx.cr[6].gt {
	pc = 0x8250D484; continue 'dispatch;
	}
	// 8250D45C: 395D0020  addi r10, r29, 0x20
	ctx.r[10].s64 = ctx.r[29].s64 + 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250D490(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8250D490 size=116
    let mut pc: u32 = 0x8250D490;
    'dispatch: loop {
        match pc {
            0x8250D490 => {
    //   block [0x8250D490..0x8250D504)
	// 8250D490: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8250D494: C0040010  lfs f0, 0x10(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(16 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8250D498: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 8250D49C: D0030010  stfs f0, 0x10(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 8250D4A0: 396B36E4  addi r11, r11, 0x36e4
	ctx.r[11].s64 = ctx.r[11].s64 + 14052;
	// 8250D4A4: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8250D4A8: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8250D4AC: 38E0000C  li r7, 0xc
	ctx.r[7].s64 = 12;
	// 8250D4B0: 394A2A68  addi r10, r10, 0x2a68
	ctx.r[10].s64 = ctx.r[10].s64 + 10856;
	// 8250D4B4: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8250D4B8: B1030006  sth r8, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[8].u16 ) };
	// 8250D4BC: 91230008  stw r9, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 8250D4C0: 90E3000C  stw r7, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[7].u32 ) };
	// 8250D4C4: 91430014  stw r10, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 8250D4C8: 90830018  stw r4, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[4].u32 ) };
	// 8250D4CC: A1640004  lhz r11, 4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 8250D4D0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8250D4D4: 419A0010  beq cr6, 0x8250d4e4
	if ctx.cr[6].eq {
	pc = 0x8250D4E4; continue 'dispatch;
	}
	// 8250D4D8: A1640006  lhz r11, 6(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(6 as u32) ) } as u64;
	// 8250D4DC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8250D4E0: B1640006  sth r11, 6(r4)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[4].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 8250D4E4: 39400020  li r10, 0x20
	ctx.r[10].s64 = 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250D508(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8250D508 size=132
    let mut pc: u32 = 0x8250D508;
    'dispatch: loop {
        match pc {
            0x8250D508 => {
    //   block [0x8250D508..0x8250D58C)
	// 8250D508: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250D50C: 48027BB1  bl 0x825350bc
	ctx.lr = 0x8250D510;
	sub_82535080(ctx, base);
	// 8250D510: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250D514: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8250D518: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8250D51C: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 8250D520: 807D0018  lwz r3, 0x18(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(24 as u32) ) } as u64;
	// 8250D524: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250D528: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 8250D52C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8250D530: 4E800421  bctrl
	ctx.lr = 0x8250D534;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8250D534: 395D0020  addi r10, r29, 0x20
	ctx.r[10].s64 = ctx.r[29].s64 + 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250D590(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8250D590 size=304
    let mut pc: u32 = 0x8250D590;
    'dispatch: loop {
        match pc {
            0x8250D590 => {
    //   block [0x8250D590..0x8250D6C0)
	// 8250D590: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250D594: 48027B25  bl 0x825350b8
	ctx.lr = 0x8250D598;
	sub_82535080(ctx, base);
	// 8250D598: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250D59C: 838D0000  lwz r28, 0(r13)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250D5A0: 3BA00014  li r29, 0x14
	ctx.r[29].s64 = 20;
	// 8250D5A4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8250D5A8: 7CA92B78  mr r9, r5
	ctx.r[9].u64 = ctx.r[5].u64;
	// 8250D5AC: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 8250D5B0: 7D7DE02E  lwzx r11, r29, r28
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 8250D5B4: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8250D5B8: 810B000C  lwz r8, 0xc(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8250D5BC: 7F0A4040  cmplw cr6, r10, r8
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[8].u32, &mut ctx.xer);
	// 8250D5C0: 40980020  bge cr6, 0x8250d5e0
	if !ctx.cr[6].lt {
	pc = 0x8250D5E0; continue 'dispatch;
	}
	// 8250D5C4: 3D008206  lis r8, -0x7dfa
	ctx.r[8].s64 = -2113536000;
	// 8250D5C8: 39084A54  addi r8, r8, 0x4a54
	ctx.r[8].s64 = ctx.r[8].s64 + 19028;
	// 8250D5CC: 910A0000  stw r8, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 8250D5D0: 7D0C42E6  mftb r8, 0x10c
	ctx.r[8].u64 = crate::rt::rdtsc_u64();
	// 8250D5D4: 38EA000C  addi r7, r10, 0xc
	ctx.r[7].s64 = ctx.r[10].s64 + 12;
	// 8250D5D8: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 8250D5DC: 90EB0004  stw r7, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 8250D5E0: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 8250D5E4: 7D2B4B78  mr r11, r9
	ctx.r[11].u64 = ctx.r[9].u64;
	// 8250D5E8: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 8250D5EC: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	// 8250D5F0: E90B0000  ld r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 8250D5F4: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 8250D5F8: F90A0000  std r8, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u64 ) };
	// 8250D5FC: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 8250D600: 4200FFF0  bdnz 0x8250d5f0
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x8250D5F0; continue 'dispatch;
	}
	// 8250D604: 39400020  li r10, 0x20
	ctx.r[10].s64 = 32;
	// 8250D608: 817F0040  lwz r11, 0x40(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250D6C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8250D6C0 size=8
    let mut pc: u32 = 0x8250D6C0;
    'dispatch: loop {
        match pc {
            0x8250D6C0 => {
    //   block [0x8250D6C0..0x8250D6C8)
	// 8250D6C0: 88630040  lbz r3, 0x40(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(64 as u32) ) } as u64;
	// 8250D6C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250D6C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8250D6C8 size=8
    let mut pc: u32 = 0x8250D6C8;
    'dispatch: loop {
        match pc {
            0x8250D6C8 => {
    //   block [0x8250D6C8..0x8250D6D0)
	// 8250D6C8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8250D6CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250D6D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8250D6D0 size=8
    let mut pc: u32 = 0x8250D6D0;
    'dispatch: loop {
        match pc {
            0x8250D6D0 => {
    //   block [0x8250D6D0..0x8250D6D8)
	// 8250D6D0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8250D6D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250D6D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8250D6D8 size=12
    let mut pc: u32 = 0x8250D6D8;
    'dispatch: loop {
        match pc {
            0x8250D6D8 => {
    //   block [0x8250D6D8..0x8250D6E4)
	// 8250D6D8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8250D6DC: 38630014  addi r3, r3, 0x14
	ctx.r[3].s64 = ctx.r[3].s64 + 20;
	// 8250D6E0: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250D6E4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8250D6E4 size=8
    let mut pc: u32 = 0x8250D6E4;
    'dispatch: loop {
        match pc {
            0x8250D6E4 => {
    //   block [0x8250D6E4..0x8250D6EC)
	// 8250D6E4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8250D6E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250D6F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8250D6F0 size=144
    let mut pc: u32 = 0x8250D6F0;
    'dispatch: loop {
        match pc {
            0x8250D6F0 => {
    //   block [0x8250D6F0..0x8250D780)
	// 8250D6F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250D6F4: 480279C1  bl 0x825350b4
	ctx.lr = 0x8250D6F8;
	sub_82535080(ctx, base);
	// 8250D6F8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250D6FC: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 8250D700: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8250D704: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 8250D708: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 8250D70C: 4099006C  ble cr6, 0x8250d778
	if !ctx.cr[6].gt {
	pc = 0x8250D778; continue 'dispatch;
	}
	// 8250D710: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 8250D714: A17D0000  lhz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250D718: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 8250D71C: 815B0044  lwz r10, 0x44(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(68 as u32) ) } as u64;
	// 8250D720: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8250D724: 557CC23E  srwi r28, r11, 8
	ctx.r[28].u32 = ctx.r[11].u32.wrapping_shr(8);
	ctx.r[28].u64 = ctx.r[28].u32 as u64;
	// 8250D728: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 8250D72C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8250D730: B1610050  sth r11, 0x50(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u16 ) };
	// 8250D734: 578B103A  slwi r11, r28, 2
	ctx.r[11].u32 = ctx.r[28].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8250D738: 7C6B502E  lwzx r3, r11, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 8250D73C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250D740: 816B0034  lwz r11, 0x34(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 8250D744: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8250D748: 4E800421  bctrl
	ctx.lr = 0x8250D74C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8250D74C: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8250D750: 578A402E  slwi r10, r28, 8
	ctx.r[10].u32 = ctx.r[28].u32.wrapping_shl(8);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8250D754: 556B0202  rlwinm r11, r11, 0, 8, 1
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 8250D758: 3BDEFFFF  addi r30, r30, -1
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	// 8250D75C: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8250D760: 3BBD0002  addi r29, r29, 2
	ctx.r[29].s64 = ctx.r[29].s64 + 2;
	// 8250D764: 656B3F00  oris r11, r11, 0x3f00
	ctx.r[11].u64 = ctx.r[11].u64 | 1056964608;
	// 8250D768: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8250D76C: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 8250D770: 3BFF0010  addi r31, r31, 0x10
	ctx.r[31].s64 = ctx.r[31].s64 + 16;
	// 8250D774: 409AFFA0  bne cr6, 0x8250d714
	if !ctx.cr[6].eq {
	pc = 0x8250D714; continue 'dispatch;
	}
	// 8250D778: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8250D77C: 48027988  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250D780(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8250D780 size=24
    let mut pc: u32 = 0x8250D780;
    'dispatch: loop {
        match pc {
            0x8250D780 => {
    //   block [0x8250D780..0x8250D798)
	// 8250D780: 81630044  lwz r11, 0x44(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(68 as u32) ) } as u64;
	// 8250D784: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250D788: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250D78C: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 8250D790: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8250D794: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250D798(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8250D798 size=100
    let mut pc: u32 = 0x8250D798;
    'dispatch: loop {
        match pc {
            0x8250D798 => {
    //   block [0x8250D798..0x8250D7FC)
	// 8250D798: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250D79C: 4802791D  bl 0x825350b8
	ctx.lr = 0x8250D7A0;
	sub_82535080(ctx, base);
	// 8250D7A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250D7A4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8250D7A8: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 8250D7AC: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8250D7B0: 817D0048  lwz r11, 0x48(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(72 as u32) ) } as u64;
	// 8250D7B4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8250D7B8: 40990038  ble cr6, 0x8250d7f0
	if !ctx.cr[6].gt {
	pc = 0x8250D7F0; continue 'dispatch;
	}
	// 8250D7BC: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8250D7C0: 817D0044  lwz r11, 0x44(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(68 as u32) ) } as u64;
	// 8250D7C4: 7C7E582E  lwzx r3, r30, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8250D7C8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250D7CC: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 8250D7D0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8250D7D4: 4E800421  bctrl
	ctx.lr = 0x8250D7D8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8250D7D8: 817D0048  lwz r11, 0x48(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(72 as u32) ) } as u64;
	// 8250D7DC: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 8250D7E0: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 8250D7E4: 7F83E214  add r28, r3, r28
	ctx.r[28].u64 = ctx.r[3].u64 + ctx.r[28].u64;
	// 8250D7E8: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8250D7EC: 4198FFD4  blt cr6, 0x8250d7c0
	if ctx.cr[6].lt {
	pc = 0x8250D7C0; continue 'dispatch;
	}
	// 8250D7F0: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8250D7F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8250D7F8: 48027910  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250D800(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8250D800 size=136
    let mut pc: u32 = 0x8250D800;
    'dispatch: loop {
        match pc {
            0x8250D800 => {
    //   block [0x8250D800..0x8250D888)
	// 8250D800: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250D804: 480278AD  bl 0x825350b0
	ctx.lr = 0x8250D808;
	sub_82535080(ctx, base);
	// 8250D808: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250D80C: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 8250D810: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 8250D814: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8250D818: 7F5DD378  mr r29, r26
	ctx.r[29].u64 = ctx.r[26].u64;
	// 8250D81C: 817B0048  lwz r11, 0x48(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(72 as u32) ) } as u64;
	// 8250D820: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8250D824: 40990058  ble cr6, 0x8250d87c
	if !ctx.cr[6].gt {
	pc = 0x8250D87C; continue 'dispatch;
	}
	// 8250D828: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 8250D82C: 817B0044  lwz r11, 0x44(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(68 as u32) ) } as u64;
	// 8250D830: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8250D834: 7FFC582E  lwzx r31, r28, r11
	ctx.r[31].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[28].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8250D838: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8250D83C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250D840: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 8250D844: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8250D848: 4E800421  bctrl
	ctx.lr = 0x8250D84C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8250D84C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250D850: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8250D854: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 8250D858: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8250D85C: 4E800421  bctrl
	ctx.lr = 0x8250D860;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8250D860: 815B0048  lwz r10, 0x48(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(72 as u32) ) } as u64;
	// 8250D864: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 8250D868: 546B2036  slwi r11, r3, 4
	ctx.r[11].u32 = ctx.r[3].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8250D86C: 3B9C0004  addi r28, r28, 4
	ctx.r[28].s64 = ctx.r[28].s64 + 4;
	// 8250D870: 7FABEA14  add r29, r11, r29
	ctx.r[29].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 8250D874: 7F1E5000  cmpw cr6, r30, r10
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[10].s32, &mut ctx.xer);
	// 8250D878: 4198FFB4  blt cr6, 0x8250d82c
	if ctx.cr[6].lt {
	pc = 0x8250D82C; continue 'dispatch;
	}
	// 8250D87C: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 8250D880: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8250D884: 4802787C  b 0x82535100
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250D888(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8250D888 size=328
    let mut pc: u32 = 0x8250D888;
    'dispatch: loop {
        match pc {
            0x8250D888 => {
    //   block [0x8250D888..0x8250D9D0)
	// 8250D888: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250D88C: 4802782D  bl 0x825350b8
	ctx.lr = 0x8250D890;
	sub_82535080(ctx, base);
	// 8250D890: DBE1FFD0  stfd f31, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[31].u64 ) };
	// 8250D894: 3980FFB0  li r12, -0x50
	ctx.r[12].s64 = -80;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250D9D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8250D9D0 size=8
    let mut pc: u32 = 0x8250D9D0;
    'dispatch: loop {
        match pc {
            0x8250D9D0 => {
    //   block [0x8250D9D0..0x8250D9D8)
	// 8250D9D0: 80630034  lwz r3, 0x34(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(52 as u32) ) } as u64;
	// 8250D9D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250D9D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8250D9D8 size=20
    let mut pc: u32 = 0x8250D9D8;
    'dispatch: loop {
        match pc {
            0x8250D9D8 => {
    //   block [0x8250D9D8..0x8250D9EC)
	// 8250D9D8: 81430034  lwz r10, 0x34(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(52 as u32) ) } as u64;
	// 8250D9DC: 39640001  addi r11, r4, 1
	ctx.r[11].s64 = ctx.r[4].s64 + 1;
	// 8250D9E0: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 8250D9E4: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 8250D9E8: 4D980020  bltlr cr6
	if ctx.cr[6].lt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250D9EC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8250D9EC size=8
    let mut pc: u32 = 0x8250D9EC;
    'dispatch: loop {
        match pc {
            0x8250D9EC => {
    //   block [0x8250D9EC..0x8250D9F4)
	// 8250D9EC: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 8250D9F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250D9F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8250D9F8 size=16
    let mut pc: u32 = 0x8250D9F8;
    'dispatch: loop {
        match pc {
            0x8250D9F8 => {
    //   block [0x8250D9F8..0x8250DA08)
	// 8250D9F8: 81630030  lwz r11, 0x30(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) } as u64;
	// 8250D9FC: 548A103A  slwi r10, r4, 2
	ctx.r[10].u32 = ctx.r[4].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8250DA00: 7C6B502E  lwzx r3, r11, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 8250DA04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250DA08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8250DA08 size=220
    let mut pc: u32 = 0x8250DA08;
    'dispatch: loop {
        match pc {
            0x8250DA08 => {
    //   block [0x8250DA08..0x8250DAE4)
	// 8250DA08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250DA0C: 480276B1  bl 0x825350bc
	ctx.lr = 0x8250DA10;
	sub_82535080(ctx, base);
	// 8250DA10: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250DA14: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8250DA18: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8250DA1C: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 8250DA20: 396B38EC  addi r11, r11, 0x38ec
	ctx.r[11].s64 = ctx.r[11].s64 + 14572;
	// 8250DA24: 394A38C4  addi r10, r10, 0x38c4
	ctx.r[10].s64 = ctx.r[10].s64 + 14532;
	// 8250DA28: 813F0048  lwz r9, 0x48(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 8250DA2C: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 8250DA30: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8250DA34: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8250DA38: 915F0014  stw r10, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 8250DA3C: 4099005C  ble cr6, 0x8250da98
	if !ctx.cr[6].gt {
	pc = 0x8250DA98; continue 'dispatch;
	}
	// 8250DA40: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8250DA44: 817F0044  lwz r11, 0x44(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(68 as u32) ) } as u64;
	// 8250DA48: 7C6BF02E  lwzx r3, r11, r30
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 8250DA4C: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8250DA50: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8250DA54: 419A0030  beq cr6, 0x8250da84
	if ctx.cr[6].eq {
	pc = 0x8250DA84; continue 'dispatch;
	}
	// 8250DA58: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 8250DA5C: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8250DA60: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 8250DA64: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8250DA68: B1630006  sth r11, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 8250DA6C: 409A0018  bne cr6, 0x8250da84
	if !ctx.cr[6].eq {
	pc = 0x8250DA84; continue 'dispatch;
	}
	// 8250DA70: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250DA74: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8250DA78: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250DA7C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8250DA80: 4E800421  bctrl
	ctx.lr = 0x8250DA84;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8250DA84: 817F0048  lwz r11, 0x48(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 8250DA88: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 8250DA8C: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 8250DA90: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8250DA94: 4198FFB0  blt cr6, 0x8250da44
	if ctx.cr[6].lt {
	pc = 0x8250DA44; continue 'dispatch;
	}
	// 8250DA98: 817F004C  lwz r11, 0x4c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 8250DA9C: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 8250DAA0: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8250DAA4: 409A0020  bne cr6, 0x8250dac4
	if !ctx.cr[6].eq {
	pc = 0x8250DAC4; continue 'dispatch;
	}
	// 8250DAA8: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250DAAC: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 8250DAB0: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 8250DAB4: 809F0044  lwz r4, 0x44(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(68 as u32) ) } as u64;
	// 8250DAB8: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 8250DABC: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 8250DAC0: 4BF565F9  bl 0x824640b8
	ctx.lr = 0x8250DAC4;
	sub_824640B8(ctx, base);
	// 8250DAC4: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8250DAC8: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8250DACC: 396B2A44  addi r11, r11, 0x2a44
	ctx.r[11].s64 = ctx.r[11].s64 + 10820;
	// 8250DAD0: 394A6DD0  addi r10, r10, 0x6dd0
	ctx.r[10].s64 = ctx.r[10].s64 + 28112;
	// 8250DAD4: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 8250DAD8: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8250DADC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8250DAE0: 4802762C  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250DAE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8250DAE8 size=200
    let mut pc: u32 = 0x8250DAE8;
    'dispatch: loop {
        match pc {
            0x8250DAE8 => {
    //   block [0x8250DAE8..0x8250DBB0)
	// 8250DAE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250DAEC: 480275C1  bl 0x825350ac
	ctx.lr = 0x8250DAF0;
	sub_82535080(ctx, base);
	// 8250DAF0: DBE1FFB8  stfd f31, -0x48(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-72 as u32), ctx.f[31].u64 ) };
	// 8250DAF4: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250DAF8: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8250DAFC: 81430048  lwz r10, 0x48(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(72 as u32) ) } as u64;
	// 8250DB00: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8250DB04: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 8250DB08: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 8250DB0C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8250DB10: C3EB0DA0  lfs f31, 0xda0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3488 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 8250DB14: 3B230048  addi r25, r3, 0x48
	ctx.r[25].s64 = ctx.r[3].s64 + 72;
	// 8250DB18: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8250DB1C: 40990070  ble cr6, 0x8250db8c
	if !ctx.cr[6].gt {
	pc = 0x8250DB8C; continue 'dispatch;
	}
	// 8250DB20: 3B430044  addi r26, r3, 0x44
	ctx.r[26].s64 = ctx.r[3].s64 + 68;
	// 8250DB24: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8250DB28: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250DB2C: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 8250DB30: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8250DB34: 7C6BF02E  lwzx r3, r11, r30
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 8250DB38: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250DB3C: 816B0030  lwz r11, 0x30(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 8250DB40: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8250DB44: 4E800421  bctrl
	ctx.lr = 0x8250DB48;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8250DB48: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250DBB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8250DBB0 size=168
    let mut pc: u32 = 0x8250DBB0;
    'dispatch: loop {
        match pc {
            0x8250DBB0 => {
    //   block [0x8250DBB0..0x8250DC58)
	// 8250DBB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250DBB4: 48027505  bl 0x825350b8
	ctx.lr = 0x8250DBB8;
	sub_82535080(ctx, base);
	// 8250DBB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250DBBC: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 8250DBC0: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 8250DBC4: 3BBC0044  addi r29, r28, 0x44
	ctx.r[29].s64 = ctx.r[28].s64 + 68;
	// 8250DBC8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8250DBCC: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250DBD0: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 8250DBD4: 7F0BF800  cmpw cr6, r11, r31
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[31].s32, &mut ctx.xer);
	// 8250DBD8: 40980024  bge cr6, 0x8250dbfc
	if !ctx.cr[6].lt {
	pc = 0x8250DBFC; continue 'dispatch;
	}
	// 8250DBDC: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8250DBE0: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8250DBE4: 41980008  blt cr6, 0x8250dbec
	if ctx.cr[6].lt {
	pc = 0x8250DBEC; continue 'dispatch;
	}
	// 8250DBE8: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 8250DBEC: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 8250DBF0: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 8250DBF4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8250DBF8: 4BF606D1  bl 0x8246e2c8
	ctx.lr = 0x8250DBFC;
	sub_8246E2C8(ctx, base);
	// 8250DBFC: 93FD0004  stw r31, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 8250DC00: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 8250DC04: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250DC08: C00B0010  lfs f0, 0x10(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8250DC0C: D01C0010  stfs f0, 0x10(r28)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 8250DC10: 40990040  ble cr6, 0x8250dc50
	if !ctx.cr[6].gt {
	pc = 0x8250DC50; continue 'dispatch;
	}
	// 8250DC14: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8250DC18: 815D0000  lwz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250DC1C: 7D2BF02E  lwzx r9, r11, r30
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 8250DC20: 7D2B512E  stwx r9, r11, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[9].u32) };
	// 8250DC24: 7D4BF02E  lwzx r10, r11, r30
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 8250DC28: A12A0004  lhz r9, 4(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 8250DC2C: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 8250DC30: 419A0010  beq cr6, 0x8250dc40
	if ctx.cr[6].eq {
	pc = 0x8250DC40; continue 'dispatch;
	}
	// 8250DC34: A12A0006  lhz r9, 6(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(6 as u32) ) } as u64;
	// 8250DC38: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 8250DC3C: B12A0006  sth r9, 6(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 8250DC40: 3BFFFFFF  addi r31, r31, -1
	ctx.r[31].s64 = ctx.r[31].s64 + -1;
	// 8250DC44: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8250DC48: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8250DC4C: 409AFFCC  bne cr6, 0x8250dc18
	if !ctx.cr[6].eq {
	pc = 0x8250DC18; continue 'dispatch;
	}
	// 8250DC50: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8250DC54: 480274B4  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250DC58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8250DC58 size=252
    let mut pc: u32 = 0x8250DC58;
    'dispatch: loop {
        match pc {
            0x8250DC58 => {
    //   block [0x8250DC58..0x8250DD54)
	// 8250DC58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250DC5C: 48027455  bl 0x825350b0
	ctx.lr = 0x8250DC60;
	sub_82535080(ctx, base);
	// 8250DC60: DBE1FFC0  stfd f31, -0x40(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-64 as u32), ctx.f[31].u64 ) };
	// 8250DC64: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250DC68: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8250DC6C: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 8250DC70: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 8250DC74: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 8250DC78: 897F0040  lbz r11, 0x40(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as u64;
	// 8250DC7C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8250DC80: 419A002C  beq cr6, 0x8250dcac
	if ctx.cr[6].eq {
	pc = 0x8250DCAC; continue 'dispatch;
	}
	// 8250DC84: C01F0010  lfs f0, 0x10(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8250DC88: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 8250DC8C: 38BF0030  addi r5, r31, 0x30
	ctx.r[5].s64 = ctx.r[31].s64 + 48;
	// 8250DC90: EC20F82A  fadds f1, f0, f31
	ctx.f[1].f64 = ((ctx.f[0].f64 + ctx.f[31].f64) as f32) as f64;
	// 8250DC94: 389F0020  addi r4, r31, 0x20
	ctx.r[4].s64 = ctx.r[31].s64 + 32;
	// 8250DC98: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 8250DC9C: 4BFEECB5  bl 0x824fc950
	ctx.lr = 0x8250DCA0;
	sub_824FC950(ctx, base);
	// 8250DCA0: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 8250DCA4: CBE1FFC0  lfd f31, -0x40(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-64 as u32) ) };
	// 8250DCA8: 48027458  b 0x82535100
	sub_825350D0(ctx, base);
	return;
	// 8250DCAC: 817F0044  lwz r11, 0x44(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(68 as u32) ) } as u64;
	// 8250DCB0: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 8250DCB4: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 8250DCB8: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 8250DCBC: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250DCC0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250DCC4: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 8250DCC8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8250DCCC: 4E800421  bctrl
	ctx.lr = 0x8250DCD0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8250DCD0: 817F0048  lwz r11, 0x48(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 8250DCD4: 3BA00001  li r29, 1
	ctx.r[29].s64 = 1;
	// 8250DCD8: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8250DCDC: 4099006C  ble cr6, 0x8250dd48
	if !ctx.cr[6].gt {
	pc = 0x8250DD48; continue 'dispatch;
	}
	// 8250DCE0: 3B7C0010  addi r27, r28, 0x10
	ctx.r[27].s64 = ctx.r[28].s64 + 16;
	// 8250DCE4: 3BC00004  li r30, 4
	ctx.r[30].s64 = 4;
	// 8250DCE8: 817F0044  lwz r11, 0x44(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(68 as u32) ) } as u64;
	// 8250DCEC: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 8250DCF0: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 8250DCF4: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 8250DCF8: 7C6BF02E  lwzx r3, r11, r30
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 8250DCFC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250DD00: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 8250DD04: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8250DD08: 4E800421  bctrl
	ctx.lr = 0x8250DD0C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8250DD0C: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250DD58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8250DD58 size=232
    let mut pc: u32 = 0x8250DD58;
    'dispatch: loop {
        match pc {
            0x8250DD58 => {
    //   block [0x8250DD58..0x8250DE40)
	// 8250DD58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250DD5C: 48027359  bl 0x825350b4
	ctx.lr = 0x8250DD60;
	sub_82535080(ctx, base);
	// 8250DD60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250DD64: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8250DD68: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8250DD6C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8250DD70: 388B4A64  addi r4, r11, 0x4a64
	ctx.r[4].s64 = ctx.r[11].s64 + 19044;
	// 8250DD74: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 8250DD78: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250DD7C: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8250DD80: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8250DD84: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 8250DD88: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8250DD8C: 4E800421  bctrl
	ctx.lr = 0x8250DD90;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8250DD90: 817F004C  lwz r11, 0x4c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 8250DD94: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 8250DD98: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8250DD9C: 409A0034  bne cr6, 0x8250ddd0
	if !ctx.cr[6].eq {
	pc = 0x8250DDD0; continue 'dispatch;
	}
	// 8250DDA0: 813E0000  lwz r9, 0(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250DDA4: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 8250DDA8: 80FF0048  lwz r7, 0x48(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 8250DDAC: 5568103A  slwi r8, r11, 2
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 8250DDB0: 80DF0044  lwz r6, 0x44(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(68 as u32) ) } as u64;
	// 8250DDB4: 388A4864  addi r4, r10, 0x4864
	ctx.r[4].s64 = ctx.r[10].s64 + 18532;
	// 8250DDB8: 54E7103A  slwi r7, r7, 2
	ctx.r[7].u32 = ctx.r[7].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 8250DDBC: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8250DDC0: 81690008  lwz r11, 8(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250DDC4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8250DDC8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8250DDCC: 4E800421  bctrl
	ctx.lr = 0x8250DDD0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8250DDD0: 817F0048  lwz r11, 0x48(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 8250DDD4: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 8250DDD8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8250DDDC: 40990048  ble cr6, 0x8250de24
	if !ctx.cr[6].gt {
	pc = 0x8250DE24; continue 'dispatch;
	}
	// 8250DDE0: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8250DDE4: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 8250DDE8: 3B6B485C  addi r27, r11, 0x485c
	ctx.r[27].s64 = ctx.r[11].s64 + 18524;
	// 8250DDEC: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250DDF0: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8250DDF4: 815F0044  lwz r10, 0x44(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(68 as u32) ) } as u64;
	// 8250DDF8: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8250DDFC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8250DE00: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8250DE04: 7CCAE82E  lwzx r6, r10, r29
	ctx.r[6].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 8250DE08: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8250DE0C: 4E800421  bctrl
	ctx.lr = 0x8250DE10;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8250DE10: 817F0048  lwz r11, 0x48(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 8250DE14: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 8250DE18: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 8250DE1C: 7F1C5800  cmpw cr6, r28, r11
	ctx.cr[6].compare_i32(ctx.r[28].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8250DE20: 4198FFCC  blt cr6, 0x8250ddec
	if ctx.cr[6].lt {
	pc = 0x8250DDEC; continue 'dispatch;
	}
	// 8250DE24: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250DE28: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8250DE2C: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 8250DE30: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8250DE34: 4E800421  bctrl
	ctx.lr = 0x8250DE38;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8250DE38: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8250DE3C: 480272C8  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250DE40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8250DE40 size=360
    let mut pc: u32 = 0x8250DE40;
    'dispatch: loop {
        match pc {
            0x8250DE40 => {
    //   block [0x8250DE40..0x8250DFA8)
	// 8250DE40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250DE44: 48027261  bl 0x825350a4
	ctx.lr = 0x8250DE48;
	sub_82535080(ctx, base);
	// 8250DE48: 9421FD50  stwu r1, -0x2b0(r1)
	ea = ctx.r[1].u32.wrapping_add(-688 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250DE4C: 830D0000  lwz r24, 0(r13)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250DE50: 3B200014  li r25, 0x14
	ctx.r[25].s64 = 20;
	// 8250DE54: 7C771B78  mr r23, r3
	ctx.r[23].u64 = ctx.r[3].u64;
	// 8250DE58: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 8250DE5C: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 8250DE60: 7D79C02E  lwzx r11, r25, r24
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[25].u32.wrapping_add(ctx.r[24].u32)) } as u64;
	// 8250DE64: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8250DE68: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8250DE6C: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 8250DE70: 40980020  bge cr6, 0x8250de90
	if !ctx.cr[6].lt {
	pc = 0x8250DE90; continue 'dispatch;
	}
	// 8250DE74: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 8250DE78: 39294A74  addi r9, r9, 0x4a74
	ctx.r[9].s64 = ctx.r[9].s64 + 19060;
	// 8250DE7C: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8250DE80: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 8250DE84: 390A000C  addi r8, r10, 0xc
	ctx.r[8].s64 = ctx.r[10].s64 + 12;
	// 8250DE88: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 8250DE8C: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 8250DE90: 817D0040  lwz r11, 0x40(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(64 as u32) ) } as u64;
	// 8250DE94: 3BC40014  addi r30, r4, 0x14
	ctx.r[30].s64 = ctx.r[4].s64 + 20;
	// 8250DE98: 3B40FFFF  li r26, -1
	ctx.r[26].s64 = -1;
	// 8250DE9C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8250DEA0: 7F5CD378  mr r28, r26
	ctx.r[28].u64 = ctx.r[26].u64;
	// 8250DEA4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8250DEA8: 917D0040  stw r11, 0x40(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(64 as u32), ctx.r[11].u32 ) };
	// 8250DEAC: 81640014  lwz r11, 0x14(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(20 as u32) ) } as u64;
	// 8250DEB0: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250DEB4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8250DEB8: 4E800421  bctrl
	ctx.lr = 0x8250DEBC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8250DEBC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8250DEC0: 2F1FFFFF  cmpwi cr6, r31, -1
	ctx.cr[6].compare_i32(ctx.r[31].s32, -1, &mut ctx.xer);
	// 8250DEC4: 419A0074  beq cr6, 0x8250df38
	if ctx.cr[6].eq {
	pc = 0x8250DF38; continue 'dispatch;
	}
	// 8250DEC8: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250DECC: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 8250DED0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8250DED4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8250DED8: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 8250DEDC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8250DEE0: 4E800421  bctrl
	ctx.lr = 0x8250DEE4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8250DEE4: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8250DEE8: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 8250DEEC: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 8250DEF0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8250DEF4: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250DEF8: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 8250DEFC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8250DF00: 4E800421  bctrl
	ctx.lr = 0x8250DF04;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8250DF04: 89610050  lbz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8250DF08: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8250DF0C: 419A0008  beq cr6, 0x8250df14
	if ctx.cr[6].eq {
	pc = 0x8250DF14; continue 'dispatch;
	}
	// 8250DF10: 7FFCFB78  mr r28, r31
	ctx.r[28].u64 = ctx.r[31].u64;
	// 8250DF14: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250DF18: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8250DF1C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8250DF20: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8250DF24: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8250DF28: 4E800421  bctrl
	ctx.lr = 0x8250DF2C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8250DF2C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8250DF30: 2F1FFFFF  cmpwi cr6, r31, -1
	ctx.cr[6].compare_i32(ctx.r[31].s32, -1, &mut ctx.xer);
	// 8250DF34: 409AFF94  bne cr6, 0x8250dec8
	if !ctx.cr[6].eq {
	pc = 0x8250DEC8; continue 'dispatch;
	}
	// 8250DF38: 817D0040  lwz r11, 0x40(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(64 as u32) ) } as u64;
	// 8250DF3C: 2F1CFFFF  cmpwi cr6, r28, -1
	ctx.cr[6].compare_i32(ctx.r[28].s32, -1, &mut ctx.xer);
	// 8250DF40: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8250DF44: 917D0040  stw r11, 0x40(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(64 as u32), ctx.r[11].u32 ) };
	// 8250DF48: 419A0010  beq cr6, 0x8250df58
	if ctx.cr[6].eq {
	pc = 0x8250DF58; continue 'dispatch;
	}
	// 8250DF4C: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 8250DF50: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8250DF54: 7F8BE92E  stwx r28, r11, r29
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[29].u32), ctx.r[28].u32) };
	// 8250DF58: 7D7CD050  subf r11, r28, r26
	ctx.r[11].s64 = ctx.r[26].s64 - ctx.r[28].s64;
	// 8250DF5C: 7D59C02E  lwzx r10, r25, r24
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[25].u32.wrapping_add(ctx.r[24].u32)) } as u64;
	// 8250DF60: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 8250DF64: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8250DF68: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 8250DF6C: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 8250DF70: 99770000  stb r11, 0(r23)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[23].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 8250DF74: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 8250DF78: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 8250DF7C: 40980020  bge cr6, 0x8250df9c
	if !ctx.cr[6].lt {
	pc = 0x8250DF9C; continue 'dispatch;
	}
	// 8250DF80: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 8250DF84: 392974DC  addi r9, r9, 0x74dc
	ctx.r[9].s64 = ctx.r[9].s64 + 29916;
	// 8250DF88: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8250DF8C: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 8250DF90: 390B000C  addi r8, r11, 0xc
	ctx.r[8].s64 = ctx.r[11].s64 + 12;
	// 8250DF94: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 8250DF98: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 8250DF9C: 7EE3BB78  mr r3, r23
	ctx.r[3].u64 = ctx.r[23].u64;
	// 8250DFA0: 382102B0  addi r1, r1, 0x2b0
	ctx.r[1].s64 = ctx.r[1].s64 + 688;
	// 8250DFA4: 48027150  b 0x825350f4
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250DFA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8250DFA8 size=284
    let mut pc: u32 = 0x8250DFA8;
    'dispatch: loop {
        match pc {
            0x8250DFA8 => {
    //   block [0x8250DFA8..0x8250E0C4)
	// 8250DFA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250DFAC: 48027101  bl 0x825350ac
	ctx.lr = 0x8250DFB0;
	sub_82535080(ctx, base);
	// 8250DFB0: 9421FD60  stwu r1, -0x2a0(r1)
	ea = ctx.r[1].u32.wrapping_add(-672 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250DFB4: 832D0000  lwz r25, 0(r13)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250DFB8: 3B400014  li r26, 0x14
	ctx.r[26].s64 = 20;
	// 8250DFBC: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 8250DFC0: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8250DFC4: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 8250DFC8: 7D7AC82E  lwzx r11, r26, r25
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[25].u32)) } as u64;
	// 8250DFCC: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8250DFD0: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8250DFD4: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 8250DFD8: 40980020  bge cr6, 0x8250dff8
	if !ctx.cr[6].lt {
	pc = 0x8250DFF8; continue 'dispatch;
	}
	// 8250DFDC: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 8250DFE0: 39294948  addi r9, r9, 0x4948
	ctx.r[9].s64 = ctx.r[9].s64 + 18760;
	// 8250DFE4: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8250DFE8: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 8250DFEC: 390A000C  addi r8, r10, 0xc
	ctx.r[8].s64 = ctx.r[10].s64 + 12;
	// 8250DFF0: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 8250DFF4: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 8250DFF8: 81630014  lwz r11, 0x14(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 8250DFFC: 3BC30014  addi r30, r3, 0x14
	ctx.r[30].s64 = ctx.r[3].s64 + 20;
	// 8250E000: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8250E004: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250E008: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8250E00C: 4E800421  bctrl
	ctx.lr = 0x8250E010;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8250E010: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8250E014: 2F1FFFFF  cmpwi cr6, r31, -1
	ctx.cr[6].compare_i32(ctx.r[31].s32, -1, &mut ctx.xer);
	// 8250E018: 419A0074  beq cr6, 0x8250e08c
	if ctx.cr[6].eq {
	pc = 0x8250E08C; continue 'dispatch;
	}
	// 8250E01C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250E020: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 8250E024: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8250E028: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8250E02C: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 8250E030: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8250E034: 4E800421  bctrl
	ctx.lr = 0x8250E038;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8250E038: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250E03C: 93A1005C  stw r29, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[29].u32 ) };
	// 8250E040: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 8250E044: 93E10054  stw r31, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[31].u32 ) };
	// 8250E048: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8250E04C: 90610050  stw r3, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 8250E050: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8250E054: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 8250E058: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250E05C: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 8250E060: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8250E064: 4E800421  bctrl
	ctx.lr = 0x8250E068;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8250E068: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250E06C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8250E070: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8250E074: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8250E078: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8250E07C: 4E800421  bctrl
	ctx.lr = 0x8250E080;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8250E080: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8250E084: 2F1FFFFF  cmpwi cr6, r31, -1
	ctx.cr[6].compare_i32(ctx.r[31].s32, -1, &mut ctx.xer);
	// 8250E088: 409AFF94  bne cr6, 0x8250e01c
	if !ctx.cr[6].eq {
	pc = 0x8250E01C; continue 'dispatch;
	}
	// 8250E08C: 7D5AC82E  lwzx r10, r26, r25
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[25].u32)) } as u64;
	// 8250E090: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 8250E094: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 8250E098: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 8250E09C: 40980020  bge cr6, 0x8250e0bc
	if !ctx.cr[6].lt {
	pc = 0x8250E0BC; continue 'dispatch;
	}
	// 8250E0A0: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 8250E0A4: 392974DC  addi r9, r9, 0x74dc
	ctx.r[9].s64 = ctx.r[9].s64 + 29916;
	// 8250E0A8: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8250E0AC: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 8250E0B0: 390B000C  addi r8, r11, 0xc
	ctx.r[8].s64 = ctx.r[11].s64 + 12;
	// 8250E0B4: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 8250E0B8: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 8250E0BC: 382102A0  addi r1, r1, 0x2a0
	ctx.r[1].s64 = ctx.r[1].s64 + 672;
	// 8250E0C0: 4802703C  b 0x825350fc
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250E0C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8250E0C8 size=160
    let mut pc: u32 = 0x8250E0C8;
    'dispatch: loop {
        match pc {
            0x8250E0C8 => {
    //   block [0x8250E0C8..0x8250E168)
	// 8250E0C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250E0CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8250E0D0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8250E0D4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250E0D8: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8250E0DC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8250E0E0: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 8250E0E4: 38C00015  li r6, 0x15
	ctx.r[6].s64 = 21;
	// 8250E0E8: C00B1FF8  lfs f0, 0x1ff8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8250E0EC: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8250E0F0: D01F0010  stfs f0, 0x10(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 8250E0F4: 394B2A44  addi r10, r11, 0x2a44
	ctx.r[10].s64 = ctx.r[11].s64 + 10820;
	// 8250E0F8: B0FF0006  sth r7, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[7].u16 ) };
	// 8250E0FC: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8250E100: 90DF000C  stw r6, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[6].u32 ) };
	// 8250E104: 3CE08000  lis r7, -0x8000
	ctx.r[7].s64 = -2147483648;
	// 8250E108: 392B38EC  addi r9, r11, 0x38ec
	ctx.r[9].s64 = ctx.r[11].s64 + 14572;
	// 8250E10C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8250E110: C00B1850  lfs f0, 0x1850(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(6224 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8250E114: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8250E118: 390B38C4  addi r8, r11, 0x38c4
	ctx.r[8].s64 = ctx.r[11].s64 + 14532;
	// 8250E11C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8250E120: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8250E124: 915F0014  stw r10, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 8250E128: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8250E12C: 911F0014  stw r8, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[8].u32 ) };
	// 8250E130: 917F0044  stw r11, 0x44(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(68 as u32), ctx.r[11].u32 ) };
	// 8250E134: 917F0048  stw r11, 0x48(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[11].u32 ) };
	// 8250E138: 90FF004C  stw r7, 0x4c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(76 as u32), ctx.r[7].u32 ) };
	// 8250E13C: D01F0018  stfs f0, 0x18(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 8250E140: 4BFFFA71  bl 0x8250dbb0
	ctx.lr = 0x8250E144;
	sub_8250DBB0(ctx, base);
	// 8250E144: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8250E148: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8250E14C: 4BFFF73D  bl 0x8250d888
	ctx.lr = 0x8250E150;
	sub_8250D888(ctx, base);
	// 8250E150: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8250E154: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8250E158: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8250E15C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8250E160: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8250E164: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250E168(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8250E168 size=8
    let mut pc: u32 = 0x8250E168;
    'dispatch: loop {
        match pc {
            0x8250E168 => {
    //   block [0x8250E168..0x8250E170)
	// 8250E168: 5463063E  clrlwi r3, r3, 0x18
	ctx.r[3].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 8250E16C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250E170(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8250E170 size=8
    let mut pc: u32 = 0x8250E170;
    'dispatch: loop {
        match pc {
            0x8250E170 => {
    //   block [0x8250E170..0x8250E178)
	// 8250E170: 5463C23E  srwi r3, r3, 8
	ctx.r[3].u32 = ctx.r[3].u32.wrapping_shr(8);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 8250E174: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250E178(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8250E178 size=28
    let mut pc: u32 = 0x8250E178;
    'dispatch: loop {
        match pc {
            0x8250E178 => {
    //   block [0x8250E178..0x8250E194)
	// 8250E178: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8250E17C: 548A063E  clrlwi r10, r4, 0x18
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x000000FFu64;
	// 8250E180: 99630003  stb r11, 3(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(3 as u32), ctx.r[11].u8 ) };
	// 8250E184: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250E188: 7D4B5B78  or r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 | ctx.r[11].u64;
	// 8250E18C: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8250E190: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250E198(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8250E198 size=20
    let mut pc: u32 = 0x8250E198;
    'dispatch: loop {
        match pc {
            0x8250E198 => {
    //   block [0x8250E198..0x8250E1AC)
	// 8250E198: 89630003  lbz r11, 3(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(3 as u32) ) } as u64;
	// 8250E19C: 548A402E  slwi r10, r4, 8
	ctx.r[10].u32 = ctx.r[4].u32.wrapping_shl(8);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8250E1A0: 7D6B5378  or r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 8250E1A4: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8250E1A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250E1B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8250E1B0 size=100
    let mut pc: u32 = 0x8250E1B0;
    'dispatch: loop {
        match pc {
            0x8250E1B0 => {
    //   block [0x8250E1B0..0x8250E214)
	// 8250E1B0: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 8250E1B4: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 8250E1B8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8250E1BC: 7CAA2B78  mr r10, r5
	ctx.r[10].u64 = ctx.r[5].u64;
	// 8250E1C0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8250E1C4: 40990048  ble cr6, 0x8250e20c
	if !ctx.cr[6].gt {
	pc = 0x8250E20C; continue 'dispatch;
	}
	// 8250E1C8: 7CA92B78  mr r9, r5
	ctx.r[9].u64 = ctx.r[5].u64;
	// 8250E1CC: 8163000C  lwz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 8250E1D0: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8250E1D4: 80E40020  lwz r7, 0x20(r4)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(32 as u32) ) } as u64;
	// 8250E1D8: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 8250E1DC: 39290004  addi r9, r9, 4
	ctx.r[9].s64 = ctx.r[9].s64 + 4;
	// 8250E1E0: 810B0000  lwz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250E1E4: 5508C23E  srwi r8, r8, 8
	ctx.r[8].u32 = ctx.r[8].u32.wrapping_shr(8);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 8250E1E8: 7CC838AE  lbzx r6, r8, r7
	ctx.r[6].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[7].u32)) } as u64;
	// 8250E1EC: 98AB0003  stb r5, 3(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(3 as u32), ctx.r[5].u8 ) };
	// 8250E1F0: 83EB0000  lwz r31, 0(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250E1F4: 7CC6FB78  or r6, r6, r31
	ctx.r[6].u64 = ctx.r[6].u64 | ctx.r[31].u64;
	// 8250E1F8: 90CB0000  stw r6, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[6].u32 ) };
	// 8250E1FC: 7CA839AE  stbx r5, r8, r7
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[7].u32), ctx.r[5].u8) };
	// 8250E200: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 8250E204: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8250E208: 4198FFC4  blt cr6, 0x8250e1cc
	if ctx.cr[6].lt {
	pc = 0x8250E1CC; continue 'dispatch;
	}
	// 8250E20C: EBE1FFF8  ld r31, -8(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) };
	// 8250E210: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250E218(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8250E218 size=16
    let mut pc: u32 = 0x8250E218;
    'dispatch: loop {
        match pc {
            0x8250E218 => {
    //   block [0x8250E218..0x8250E228)
	// 8250E218: 81430010  lwz r10, 0x10(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 8250E21C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8250E220: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8250E224: 4C990020  blelr cr6
	if !ctx.cr[6].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250E228(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8250E228 size=52
    let mut pc: u32 = 0x8250E228;
    'dispatch: loop {
        match pc {
            0x8250E228 => {
    //   block [0x8250E228..0x8250E25C)
	// 8250E228: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8250E22C: 8123000C  lwz r9, 0xc(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 8250E230: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8250E234: 81040020  lwz r8, 0x20(r4)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(32 as u32) ) } as u64;
	// 8250E238: 7D2A482E  lwzx r9, r10, r9
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 8250E23C: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 8250E240: 7D274B78  mr r7, r9
	ctx.r[7].u64 = ctx.r[9].u64;
	// 8250E244: 5529C23E  srwi r9, r9, 8
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shr(8);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 8250E248: 7CE941AE  stbx r7, r9, r8
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[8].u32), ctx.r[7].u8) };
	// 8250E24C: 81230010  lwz r9, 0x10(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 8250E250: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 8250E254: 4198FFD8  blt cr6, 0x8250e22c
	if ctx.cr[6].lt {
	pc = 0x8250E22C; continue 'dispatch;
	}
	// 8250E258: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250E260(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8250E260 size=80
    let mut pc: u32 = 0x8250E260;
    'dispatch: loop {
        match pc {
            0x8250E260 => {
    //   block [0x8250E260..0x8250E2B0)
	// 8250E260: 81440010  lwz r10, 0x10(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(16 as u32) ) } as u64;
	// 8250E264: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8250E268: 812A0004  lwz r9, 4(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 8250E26C: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8250E270: 40990024  ble cr6, 0x8250e294
	if !ctx.cr[6].gt {
	pc = 0x8250E294; continue 'dispatch;
	}
	// 8250E274: 814A0000  lwz r10, 0(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250E278: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250E27C: 7F082840  cmplw cr6, r8, r5
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[5].u32, &mut ctx.xer);
	// 8250E280: 419A0018  beq cr6, 0x8250e298
	if ctx.cr[6].eq {
	pc = 0x8250E298; continue 'dispatch;
	}
	// 8250E284: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8250E288: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 8250E28C: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 8250E290: 4198FFE8  blt cr6, 0x8250e278
	if ctx.cr[6].lt {
	pc = 0x8250E278; continue 'dispatch;
	}
	// 8250E294: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 8250E298: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 8250E29C: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 8250E2A0: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8250E2A4: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 8250E2A8: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 8250E2AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250E2B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8250E2B0 size=380
    let mut pc: u32 = 0x8250E2B0;
    'dispatch: loop {
        match pc {
            0x8250E2B0 => {
    //   block [0x8250E2B0..0x8250E42C)
	// 8250E2B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250E2B4: 48026DF1  bl 0x825350a4
	ctx.lr = 0x8250E2B8;
	sub_82535080(ctx, base);
	// 8250E2B8: DBE1FFA8  stfd f31, -0x58(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-88 as u32), ctx.f[31].u64 ) };
	// 8250E2BC: 9421FD10  stwu r1, -0x2f0(r1)
	ea = ctx.r[1].u32.wrapping_add(-752 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250E2C0: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8250E2C4: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 8250E2C8: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 8250E2CC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8250E2D0: 3B800001  li r28, 1
	ctx.r[28].s64 = 1;
	// 8250E2D4: 396B3960  addi r11, r11, 0x3960
	ctx.r[11].s64 = ctx.r[11].s64 + 14688;
	// 8250E2D8: 394A3980  addi r10, r10, 0x3980
	ctx.r[10].s64 = ctx.r[10].s64 + 14720;
	// 8250E2DC: 39293970  addi r9, r9, 0x3970
	ctx.r[9].s64 = ctx.r[9].s64 + 14704;
	// 8250E2E0: 3B200000  li r25, 0
	ctx.r[25].s64 = 0;
	// 8250E2E4: 3D008000  lis r8, -0x8000
	ctx.r[8].s64 = -2147483648;
	// 8250E2E8: B39F0006  sth r28, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[28].u16 ) };
	// 8250E2EC: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 8250E2F0: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8250E2F4: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8250E2F8: 7C982378  mr r24, r4
	ctx.r[24].u64 = ctx.r[4].u64;
	// 8250E2FC: 913F0008  stw r9, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 8250E300: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8250E304: 933F000C  stw r25, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[25].u32 ) };
	// 8250E308: 3AFF0008  addi r23, r31, 8
	ctx.r[23].s64 = ctx.r[31].s64 + 8;
	// 8250E30C: 933F0010  stw r25, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[25].u32 ) };
	// 8250E310: 911F0014  stw r8, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[8].u32 ) };
	// 8250E314: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8250E318: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8250E31C: 419A0100  beq cr6, 0x8250e41c
	if ctx.cr[6].eq {
	pc = 0x8250E41C; continue 'dispatch;
	}
	// 8250E320: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250E324: 38A10090  addi r5, r1, 0x90
	ctx.r[5].s64 = ctx.r[1].s64 + 144;
	// 8250E328: 815D0000  lwz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250E32C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8250E330: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250E334: 816A0014  lwz r11, 0x14(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 8250E338: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8250E33C: 4E800421  bctrl
	ctx.lr = 0x8250E340;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8250E340: 3D608283  lis r11, -0x7d7d
	ctx.r[11].s64 = -2105344000;
	// 8250E344: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 8250E348: 3B4B2690  addi r26, r11, 0x2690
	ctx.r[26].s64 = ctx.r[11].s64 + 9872;
	// 8250E34C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8250E350: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 8250E354: C3EB1FF8  lfs f31, 0x1ff8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 8250E358: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250E35C: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 8250E360: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 8250E364: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8250E368: 4E800421  bctrl
	ctx.lr = 0x8250E36C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8250E36C: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8250E370: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8250E374: 40990090  ble cr6, 0x8250e404
	if !ctx.cr[6].gt {
	pc = 0x8250E404; continue 'dispatch;
	}
	// 8250E378: 3B600004  li r27, 4
	ctx.r[27].s64 = 4;
	// 8250E37C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250E380: 38A10090  addi r5, r1, 0x90
	ctx.r[5].s64 = ctx.r[1].s64 + 144;
	// 8250E384: 815D0000  lwz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250E388: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8250E38C: 7C9B582E  lwzx r4, r27, r11
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[27].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8250E390: 816A0014  lwz r11, 0x14(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 8250E394: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8250E398: 4E800421  bctrl
	ctx.lr = 0x8250E39C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8250E39C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250E3A0: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 8250E3A4: 38C10070  addi r6, r1, 0x70
	ctx.r[6].s64 = ctx.r[1].s64 + 112;
	// 8250E3A8: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 8250E3AC: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 8250E3B0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8250E3B4: 4E800421  bctrl
	ctx.lr = 0x8250E3B8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8250E3B8: 39410060  addi r10, r1, 0x60
	ctx.r[10].s64 = ctx.r[1].s64 + 96;
	// 8250E3BC: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 8250E3C0: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 8250E3C4: 3B7B0004  addi r27, r27, 4
	ctx.r[27].s64 = ctx.r[27].s64 + 4;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250E430(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8250E430 size=140
    let mut pc: u32 = 0x8250E430;
    'dispatch: loop {
        match pc {
            0x8250E430 => {
    //   block [0x8250E430..0x8250E4BC)
	// 8250E430: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250E434: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8250E438: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8250E43C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8250E440: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250E444: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8250E448: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8250E44C: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8250E450: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 8250E454: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 8250E458: 396B3960  addi r11, r11, 0x3960
	ctx.r[11].s64 = ctx.r[11].s64 + 14688;
	// 8250E45C: B11F0006  sth r8, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[8].u16 ) };
	// 8250E460: 394A3980  addi r10, r10, 0x3980
	ctx.r[10].s64 = ctx.r[10].s64 + 14720;
	// 8250E464: 39293970  addi r9, r9, 0x3970
	ctx.r[9].s64 = ctx.r[9].s64 + 14704;
	// 8250E468: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8250E46C: 3D008000  lis r8, -0x8000
	ctx.r[8].s64 = -2147483648;
	// 8250E470: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 8250E474: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8250E478: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 8250E47C: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8250E480: 913F0008  stw r9, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 8250E484: 38BF0008  addi r5, r31, 8
	ctx.r[5].s64 = ctx.r[31].s64 + 8;
	// 8250E488: 93DF000C  stw r30, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[30].u32 ) };
	// 8250E48C: 93DF0010  stw r30, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[30].u32 ) };
	// 8250E490: 911F0014  stw r8, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[8].u32 ) };
	// 8250E494: 90DF0018  stw r6, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[6].u32 ) };
	// 8250E498: 480AAD39  bl 0x825b91d0
	ctx.lr = 0x8250E49C;
	sub_825B91D0(ctx, base);
	// 8250E49C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8250E4A0: 93DF0018  stw r30, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[30].u32 ) };
	// 8250E4A4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8250E4A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8250E4AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8250E4B0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8250E4B4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8250E4B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250E4C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8250E4C0 size=132
    let mut pc: u32 = 0x8250E4C0;
    'dispatch: loop {
        match pc {
            0x8250E4C0 => {
    //   block [0x8250E4C0..0x8250E544)
	// 8250E4C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250E4C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8250E4C8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8250E4CC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250E4D0: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8250E4D4: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 8250E4D8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8250E4DC: 396B3980  addi r11, r11, 0x3980
	ctx.r[11].s64 = ctx.r[11].s64 + 14720;
	// 8250E4E0: 394A3970  addi r10, r10, 0x3970
	ctx.r[10].s64 = ctx.r[10].s64 + 14704;
	// 8250E4E4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8250E4E8: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8250E4EC: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 8250E4F0: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 8250E4F4: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8250E4F8: 409A0020  bne cr6, 0x8250e518
	if !ctx.cr[6].eq {
	pc = 0x8250E518; continue 'dispatch;
	}
	// 8250E4FC: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250E500: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 8250E504: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 8250E508: 809F000C  lwz r4, 0xc(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8250E50C: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 8250E510: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 8250E514: 4BF55BA5  bl 0x824640b8
	ctx.lr = 0x8250E518;
	sub_824640B8(ctx, base);
	// 8250E518: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8250E51C: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8250E520: 396B3960  addi r11, r11, 0x3960
	ctx.r[11].s64 = ctx.r[11].s64 + 14688;
	// 8250E524: 394A6DD0  addi r10, r10, 0x6dd0
	ctx.r[10].s64 = ctx.r[10].s64 + 28112;
	// 8250E528: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8250E52C: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8250E530: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8250E534: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8250E538: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8250E53C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8250E540: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250E548(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8250E548 size=132
    let mut pc: u32 = 0x8250E548;
    'dispatch: loop {
        match pc {
            0x8250E548 => {
    //   block [0x8250E548..0x8250E5CC)
	// 8250E548: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250E54C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8250E550: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8250E554: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8250E558: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250E55C: 3BE30004  addi r31, r3, 4
	ctx.r[31].s64 = ctx.r[3].s64 + 4;
	// 8250E560: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8250E564: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250E568: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8250E56C: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 8250E570: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8250E574: 409A0010  bne cr6, 0x8250e584
	if !ctx.cr[6].eq {
	pc = 0x8250E584; continue 'dispatch;
	}
	// 8250E578: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 8250E57C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8250E580: 4BF5FDD1  bl 0x8246e350
	ctx.lr = 0x8250E584;
	sub_8246E350(ctx, base);
	// 8250E584: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8250E588: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8250E58C: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250E590: 57C7402E  slwi r7, r30, 8
	ctx.r[7].u32 = ctx.r[30].u32.wrapping_shl(8);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 8250E594: 556A103A  slwi r10, r11, 2
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8250E598: 38CB0001  addi r6, r11, 1
	ctx.r[6].s64 = ctx.r[11].s64 + 1;
	// 8250E59C: 7D6A4A14  add r11, r10, r9
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 8250E5A0: 550A063E  clrlwi r10, r8, 0x18
	ctx.r[10].u64 = ctx.r[8].u32 as u64 & 0x000000FFu64;
	// 8250E5A4: 7D4A3B78  or r10, r10, r7
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[7].u64;
	// 8250E5A8: 90DF0004  stw r6, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[6].u32 ) };
	// 8250E5AC: 990B0003  stb r8, 3(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(3 as u32), ctx.r[8].u8 ) };
	// 8250E5B0: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8250E5B4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8250E5B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8250E5BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8250E5C0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8250E5C4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8250E5C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250E5D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8250E5D0 size=16
    let mut pc: u32 = 0x8250E5D0;
    'dispatch: loop {
        match pc {
            0x8250E5D0 => {
    //   block [0x8250E5D0..0x8250E5E0)
	// 8250E5D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8250E5D4: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 8250E5D8: 91630018  stw r11, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 8250E5DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250E5E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8250E5E0 size=192
    let mut pc: u32 = 0x8250E5E0;
    'dispatch: loop {
        match pc {
            0x8250E5E0 => {
    //   block [0x8250E5E0..0x8250E6A0)
	// 8250E5E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250E5E4: 48026AD1  bl 0x825350b4
	ctx.lr = 0x8250E5E8;
	sub_82535080(ctx, base);
	// 8250E5E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250E5EC: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 8250E5F0: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8250E5F4: 3BFC0014  addi r31, r28, 0x14
	ctx.r[31].s64 = ctx.r[28].s64 + 20;
	// 8250E5F8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8250E5FC: 57BB063E  clrlwi r27, r29, 0x18
	ctx.r[27].u64 = ctx.r[29].u32 as u64 & 0x000000FFu64;
	// 8250E600: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250E604: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8250E608: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 8250E60C: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8250E610: 409A0010  bne cr6, 0x8250e620
	if !ctx.cr[6].eq {
	pc = 0x8250E620; continue 'dispatch;
	}
	// 8250E614: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8250E618: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8250E61C: 4BF5FD35  bl 0x8246e350
	ctx.lr = 0x8250E620;
	sub_8246E350(ctx, base);
	// 8250E620: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8250E624: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 8250E628: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250E62C: 7F6A59AE  stbx r27, r10, r11
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32), ctx.r[27].u8) };
	// 8250E630: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8250E634: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8250E638: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8250E63C: 4099005C  ble cr6, 0x8250e698
	if !ctx.cr[6].gt {
	pc = 0x8250E698; continue 'dispatch;
	}
	// 8250E640: 3BFC0008  addi r31, r28, 8
	ctx.r[31].s64 = ctx.r[28].s64 + 8;
	// 8250E644: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250E648: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250E64C: 813F0004  lwz r9, 4(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8250E650: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 8250E654: 555C043E  clrlwi r28, r10, 0x10
	ctx.r[28].u64 = ctx.r[10].u32 as u64 & 0x0000FFFFu64;
	// 8250E658: 7F095800  cmpw cr6, r9, r11
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8250E65C: 409A0010  bne cr6, 0x8250e66c
	if !ctx.cr[6].eq {
	pc = 0x8250E66C; continue 'dispatch;
	}
	// 8250E660: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 8250E664: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8250E668: 4BF5FCE9  bl 0x8246e350
	ctx.lr = 0x8250E66C;
	sub_8246E350(ctx, base);
	// 8250E66C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8250E670: 3BBDFFFF  addi r29, r29, -1
	ctx.r[29].s64 = ctx.r[29].s64 + -1;
	// 8250E674: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250E678: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 8250E67C: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8250E680: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 8250E684: 7F8B532E  sthx r28, r11, r10
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[28].u16) };
	// 8250E688: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8250E68C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8250E690: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8250E694: 409AFFB0  bne cr6, 0x8250e644
	if !ctx.cr[6].eq {
	pc = 0x8250E644; continue 'dispatch;
	}
	// 8250E698: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8250E69C: 48026A68  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250E6A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8250E6A0 size=500
    let mut pc: u32 = 0x8250E6A0;
    'dispatch: loop {
        match pc {
            0x8250E6A0 => {
    //   block [0x8250E6A0..0x8250E894)
	// 8250E6A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250E6A4: 480269F5  bl 0x82535098
	ctx.lr = 0x8250E6A8;
	sub_82535080(ctx, base);
	// 8250E6A8: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250E6AC: 7C741B78  mr r20, r3
	ctx.r[20].u64 = ctx.r[3].u64;
	// 8250E6B0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8250E6B4: 7C982378  mr r24, r4
	ctx.r[24].u64 = ctx.r[4].u64;
	// 8250E6B8: 4BF5EC89  bl 0x8246d340
	ctx.lr = 0x8250E6BC;
	sub_8246D340(ctx, base);
	// 8250E6BC: 82D80018  lwz r22, 0x18(r24)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(24 as u32) ) } as u64;
	// 8250E6C0: 3AA00000  li r21, 0
	ctx.r[21].s64 = 0;
	// 8250E6C4: 2F160000  cmpwi cr6, r22, 0
	ctx.cr[6].compare_i32(ctx.r[22].s32, 0, &mut ctx.xer);
	// 8250E6C8: 7EB7AB78  mr r23, r21
	ctx.r[23].u64 = ctx.r[21].u64;
	// 8250E6CC: 7EB9AB78  mr r25, r21
	ctx.r[25].u64 = ctx.r[21].u64;
	// 8250E6D0: 409900F4  ble cr6, 0x8250e7c4
	if !ctx.cr[6].gt {
	pc = 0x8250E7C4; continue 'dispatch;
	}
	// 8250E6D4: 81380014  lwz r9, 0x14(r24)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(20 as u32) ) } as u64;
	// 8250E6D8: 56EB083C  slwi r11, r23, 1
	ctx.r[11].u32 = ctx.r[23].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8250E6DC: 81580008  lwz r10, 8(r24)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250E6E0: 7EBBAB78  mr r27, r21
	ctx.r[27].u64 = ctx.r[21].u64;
	// 8250E6E4: 7D4B5214  add r10, r11, r10
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8250E6E8: 7F9948AE  lbzx r28, r25, r9
	ctx.r[28].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[25].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 8250E6EC: 578B083E  rotlwi r11, r28, 1
	ctx.r[11].u64 = ((ctx.r[28].u32).rotate_left(1)) as u64;
	// 8250E6F0: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 8250E6F4: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8250E6F8: A16BFFFE  lhz r11, -2(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(-2 as u32) ) } as u64;
	// 8250E6FC: 409900B8  ble cr6, 0x8250e7b4
	if !ctx.cr[6].gt {
	pc = 0x8250E7B4; continue 'dispatch;
	}
	// 8250E700: 7D5A5378  mr r26, r10
	ctx.r[26].u64 = ctx.r[10].u64;
	// 8250E704: A3BA0000  lhz r29, 0(r26)
	ctx.r[29].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250E708: 7F0BE800  cmpw cr6, r11, r29
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[29].s32, &mut ctx.xer);
	// 8250E70C: 40980010  bge cr6, 0x8250e71c
	if !ctx.cr[6].lt {
	pc = 0x8250E71C; continue 'dispatch;
	}
	// 8250E710: 556A801E  slwi r10, r11, 0x10
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(16);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8250E714: 7D4AEB78  or r10, r10, r29
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[29].u64;
	// 8250E718: 4800000C  b 0x8250e724
	pc = 0x8250E724; continue 'dispatch;
	// 8250E71C: 57AA801E  slwi r10, r29, 0x10
	ctx.r[10].u32 = ctx.r[29].u32.wrapping_shl(16);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8250E720: 7D4A5B78  or r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[11].u64;
	// 8250E724: 3BCA0001  addi r30, r10, 1
	ctx.r[30].s64 = ctx.r[10].s64 + 1;
	// 8250E728: 7F0BE800  cmpw cr6, r11, r29
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[29].s32, &mut ctx.xer);
	// 8250E72C: 3BE00002  li r31, 2
	ctx.r[31].s64 = 2;
	// 8250E730: 41980008  blt cr6, 0x8250e738
	if ctx.cr[6].lt {
	pc = 0x8250E738; continue 'dispatch;
	}
	// 8250E734: 3BE00001  li r31, 1
	ctx.r[31].s64 = 1;
	// 8250E738: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8250E73C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8250E740: 4BF5ED99  bl 0x8246d4d8
	ctx.lr = 0x8250E744;
	sub_8246D4D8(ctx, base);
	// 8250E744: 81410058  lwz r10, 0x58(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 8250E748: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8250E74C: 7F035000  cmpw cr6, r3, r10
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[10].s32, &mut ctx.xer);
	// 8250E750: 40990008  ble cr6, 0x8250e758
	if !ctx.cr[6].gt {
	pc = 0x8250E758; continue 'dispatch;
	}
	// 8250E754: 7EABAB78  mr r11, r21
	ctx.r[11].u64 = ctx.r[21].u64;
	// 8250E758: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 8250E75C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8250E760: 419A0030  beq cr6, 0x8250e790
	if ctx.cr[6].eq {
	pc = 0x8250E790; continue 'dispatch;
	}
	// 8250E764: 7D6A1A14  add r11, r10, r3
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[3].u64;
	// 8250E768: 81210050  lwz r9, 0x50(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8250E76C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8250E770: 556A103A  slwi r10, r11, 2
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8250E774: 7D6A482E  lwzx r11, r10, r9
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 8250E778: 7D68F838  and r8, r11, r31
	ctx.r[8].u64 = ctx.r[11].u64 & ctx.r[31].u64;
	// 8250E77C: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 8250E780: 409A00E0  bne cr6, 0x8250e860
	if !ctx.cr[6].eq {
	pc = 0x8250E860; continue 'dispatch;
	}
	// 8250E784: 7D6BFB78  or r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[31].u64;
	// 8250E788: 7D6A492E  stwx r11, r10, r9
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32), ctx.r[11].u32) };
	// 8250E78C: 48000014  b 0x8250e7a0
	pc = 0x8250E7A0; continue 'dispatch;
	// 8250E790: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8250E794: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8250E798: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8250E79C: 4BF5EC65  bl 0x8246d400
	ctx.lr = 0x8250E7A0;
	sub_8246D400(ctx, base);
	// 8250E7A0: 3B7B0001  addi r27, r27, 1
	ctx.r[27].s64 = ctx.r[27].s64 + 1;
	// 8250E7A4: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 8250E7A8: 3B5A0002  addi r26, r26, 2
	ctx.r[26].s64 = ctx.r[26].s64 + 2;
	// 8250E7AC: 7F1BE000  cmpw cr6, r27, r28
	ctx.cr[6].compare_i32(ctx.r[27].s32, ctx.r[28].s32, &mut ctx.xer);
	// 8250E7B0: 4198FF54  blt cr6, 0x8250e704
	if ctx.cr[6].lt {
	pc = 0x8250E704; continue 'dispatch;
	}
	// 8250E7B4: 3B390001  addi r25, r25, 1
	ctx.r[25].s64 = ctx.r[25].s64 + 1;
	// 8250E7B8: 7EFCBA14  add r23, r28, r23
	ctx.r[23].u64 = ctx.r[28].u64 + ctx.r[23].u64;
	// 8250E7BC: 7F19B000  cmpw cr6, r25, r22
	ctx.cr[6].compare_i32(ctx.r[25].s32, ctx.r[22].s32, &mut ctx.xer);
	// 8250E7C0: 4198FF14  blt cr6, 0x8250e6d4
	if ctx.cr[6].lt {
	pc = 0x8250E6D4; continue 'dispatch;
	}
	// 8250E7C4: 81210058  lwz r9, 0x58(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 8250E7C8: 7EABAB78  mr r11, r21
	ctx.r[11].u64 = ctx.r[21].u64;
	// 8250E7CC: 81010050  lwz r8, 0x50(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8250E7D0: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8250E7D4: 41980024  blt cr6, 0x8250e7f8
	if ctx.cr[6].lt {
	pc = 0x8250E7F8; continue 'dispatch;
	}
	// 8250E7D8: 7D0A4378  mr r10, r8
	ctx.r[10].u64 = ctx.r[8].u64;
	// 8250E7DC: 80EA0000  lwz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250E7E0: 2F07FFFF  cmpwi cr6, r7, -1
	ctx.cr[6].compare_i32(ctx.r[7].s32, -1, &mut ctx.xer);
	// 8250E7E4: 409A0014  bne cr6, 0x8250e7f8
	if !ctx.cr[6].eq {
	pc = 0x8250E7F8; continue 'dispatch;
	}
	// 8250E7E8: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8250E7EC: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 8250E7F0: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 8250E7F4: 4099FFE8  ble cr6, 0x8250e7dc
	if !ctx.cr[6].gt {
	pc = 0x8250E7DC; continue 'dispatch;
	}
	// 8250E7F8: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 8250E7FC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8250E800: 40990008  ble cr6, 0x8250e808
	if !ctx.cr[6].gt {
	pc = 0x8250E808; continue 'dispatch;
	}
	// 8250E804: 7EAAAB78  mr r10, r21
	ctx.r[10].u64 = ctx.r[21].u64;
	// 8250E808: 7D4A0774  extsb r10, r10
	ctx.r[10].s64 = ctx.r[10].s8 as i64;
	// 8250E80C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8250E810: 419A0068  beq cr6, 0x8250e878
	if ctx.cr[6].eq {
	pc = 0x8250E878; continue 'dispatch;
	}
	// 8250E814: 7D495A14  add r10, r9, r11
	ctx.r[10].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 8250E818: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8250E81C: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8250E820: 7D4A402E  lwzx r10, r10, r8
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 8250E824: 2B0A0003  cmplwi cr6, r10, 3
	ctx.cr[6].compare_u32(ctx.r[10].u32, 3 as u32, &mut ctx.xer);
	// 8250E828: 409A0038  bne cr6, 0x8250e860
	if !ctx.cr[6].eq {
	pc = 0x8250E860; continue 'dispatch;
	}
	// 8250E82C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8250E830: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 8250E834: 4199FFC4  bgt cr6, 0x8250e7f8
	if ctx.cr[6].gt {
	pc = 0x8250E7F8; continue 'dispatch;
	}
	// 8250E838: 556A103A  slwi r10, r11, 2
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8250E83C: 7D4A4214  add r10, r10, r8
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[8].u64;
	// 8250E840: 80EA0000  lwz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250E844: 2F07FFFF  cmpwi cr6, r7, -1
	ctx.cr[6].compare_i32(ctx.r[7].s32, -1, &mut ctx.xer);
	// 8250E848: 409AFFB0  bne cr6, 0x8250e7f8
	if !ctx.cr[6].eq {
	pc = 0x8250E7F8; continue 'dispatch;
	}
	// 8250E84C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8250E850: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 8250E854: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 8250E858: 4099FFE8  ble cr6, 0x8250e840
	if !ctx.cr[6].gt {
	pc = 0x8250E840; continue 'dispatch;
	}
	// 8250E85C: 4BFFFF9C  b 0x8250e7f8
	pc = 0x8250E7F8; continue 'dispatch;
	// 8250E860: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8250E864: 9AB40000  stb r21, 0(r20)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[20].u32.wrapping_add(0 as u32), ctx.r[21].u8 ) };
	// 8250E868: 4BF5EB61  bl 0x8246d3c8
	ctx.lr = 0x8250E86C;
	sub_8246D3C8(ctx, base);
	// 8250E86C: 7E83A378  mr r3, r20
	ctx.r[3].u64 = ctx.r[20].u64;
	// 8250E870: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 8250E874: 48026874  b 0x825350e8
	sub_825350D0(ctx, base);
	return;
	// 8250E878: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8250E87C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8250E880: 99740000  stb r11, 0(r20)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[20].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 8250E884: 4BF5EB45  bl 0x8246d3c8
	ctx.lr = 0x8250E888;
	sub_8246D3C8(ctx, base);
	// 8250E888: 7E83A378  mr r3, r20
	ctx.r[3].u64 = ctx.r[20].u64;
	// 8250E88C: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 8250E890: 48026858  b 0x825350e8
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250E898(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8250E898 size=4
    let mut pc: u32 = 0x8250E898;
    'dispatch: loop {
        match pc {
            0x8250E898 => {
    //   block [0x8250E898..0x8250E89C)
	// 8250E898: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250E8A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8250E8A0 size=196
    let mut pc: u32 = 0x8250E8A0;
    'dispatch: loop {
        match pc {
            0x8250E8A0 => {
    //   block [0x8250E8A0..0x8250E964)
	// 8250E8A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250E8A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8250E8A8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8250E8AC: 9421FD90  stwu r1, -0x270(r1)
	ea = ctx.r[1].u32.wrapping_add(-624 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250E8B0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8250E8B4: 817F0028  lwz r11, 0x28(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 8250E8B8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8250E8BC: 409A001C  bne cr6, 0x8250e8d8
	if !ctx.cr[6].eq {
	pc = 0x8250E8D8; continue 'dispatch;
	}
	// 8250E8C0: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 8250E8C4: 38210270  addi r1, r1, 0x270
	ctx.r[1].s64 = ctx.r[1].s64 + 624;
	// 8250E8C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8250E8CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8250E8D0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8250E8D4: 4E800020  blr
	return;
	// 8250E8D8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250E8DC: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 8250E8E0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8250E8E4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8250E8E8: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 8250E8EC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8250E8F0: 4E800421  bctrl
	ctx.lr = 0x8250E8F4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8250E8F4: 3D408282  lis r10, -0x7d7e
	ctx.r[10].s64 = -2105409536;
	// 8250E8F8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8250E8FC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8250E900: 38CB0040  addi r6, r11, 0x40
	ctx.r[6].s64 = ctx.r[11].s64 + 64;
	// 8250E904: 38AB0030  addi r5, r11, 0x30
	ctx.r[5].s64 = ctx.r[11].s64 + 48;
	// 8250E908: 388B0020  addi r4, r11, 0x20
	ctx.r[4].s64 = ctx.r[11].s64 + 32;
	// 8250E90C: C02AE098  lfs f1, -0x1f68(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-8040 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8250E910: 4801B831  bl 0x8252a140
	ctx.lr = 0x8250E914;
	sub_8252A140(ctx, base);
	// 8250E914: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250E918: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8250E91C: 409A001C  bne cr6, 0x8250e938
	if !ctx.cr[6].eq {
	pc = 0x8250E938; continue 'dispatch;
	}
	// 8250E920: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8250E924: 38210270  addi r1, r1, 0x270
	ctx.r[1].s64 = ctx.r[1].s64 + 624;
	// 8250E928: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8250E92C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8250E930: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8250E934: 4E800020  blr
	return;
	// 8250E938: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250E93C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8250E940: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8250E944: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8250E948: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8250E94C: 4E800421  bctrl
	ctx.lr = 0x8250E950;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8250E950: 38210270  addi r1, r1, 0x270
	ctx.r[1].s64 = ctx.r[1].s64 + 624;
	// 8250E954: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8250E958: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8250E95C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8250E960: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250E968(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8250E968 size=204
    let mut pc: u32 = 0x8250E968;
    'dispatch: loop {
        match pc {
            0x8250E968 => {
    //   block [0x8250E968..0x8250EA34)
	// 8250E968: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250E96C: 48026741  bl 0x825350ac
	ctx.lr = 0x8250E970;
	sub_82535080(ctx, base);
	// 8250E970: 9421FD60  stwu r1, -0x2a0(r1)
	ea = ctx.r[1].u32.wrapping_add(-672 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250E974: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8250E978: 3B20FFFF  li r25, -1
	ctx.r[25].s64 = -1;
	// 8250E97C: 3F408282  lis r26, -0x7d7e
	ctx.r[26].s64 = -2105409536;
	// 8250E980: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 8250E984: 214B0020  subfic r10, r11, 0x20
	ctx.xer.ca = ctx.r[11].u32 <= 32 as u32;
	ctx.r[10].s64 = (32 as i64) - ctx.r[11].s64;
	// 8250E988: 7F2B5C30  srw r11, r25, r11
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[11].u64 = 0;
	} else {
		ctx.r[11].u64 = ((ctx.r[25].u32) >> ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	// 8250E98C: 7C9D5430  srw r29, r4, r10
	if (ctx.r[10].u8 & 0x20) != 0 {
		ctx.r[29].u64 = 0;
	} else {
		ctx.r[29].u64 = ((ctx.r[4].u32) >> ((ctx.r[10].u8 & 0x1F) as u32)) as u64;
	}
	// 8250E990: 7D7E2038  and r30, r11, r4
	ctx.r[30].u64 = ctx.r[11].u64 & ctx.r[4].u64;
	// 8250E994: 1F7D0038  mulli r27, r29, 0x38
	ctx.r[27].s64 = ctx.r[29].s64 * 56;
	// 8250E998: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 8250E99C: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 8250E9A0: 7D7B5A14  add r11, r27, r11
	ctx.r[11].u64 = ctx.r[27].u64 + ctx.r[11].u64;
	// 8250E9A4: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 8250E9A8: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8250E9AC: 4198001C  blt cr6, 0x8250e9c8
	if ctx.cr[6].lt {
	pc = 0x8250E9C8; continue 'dispatch;
	}
	// 8250E9B0: 817F0028  lwz r11, 0x28(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 8250E9B4: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 8250E9B8: 3B7B0038  addi r27, r27, 0x38
	ctx.r[27].s64 = ctx.r[27].s64 + 56;
	// 8250E9BC: 7F1D5840  cmplw cr6, r29, r11
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8250E9C0: 40980068  bge cr6, 0x8250ea28
	if !ctx.cr[6].lt {
	pc = 0x8250EA28; continue 'dispatch;
	}
	// 8250E9C4: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8250E9C8: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 8250E9CC: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 8250E9D0: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250E9D4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8250E9D8: 216B0020  subfic r11, r11, 0x20
	ctx.xer.ca = ctx.r[11].u32 <= 32 as u32;
	ctx.r[11].s64 = (32 as i64) - ctx.r[11].s64;
	// 8250E9DC: 814A0014  lwz r10, 0x14(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 8250E9E0: 7FAB5830  slw r11, r29, r11
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[11].u64 = 0;
	} else {
		ctx.r[11].u64 = ((ctx.r[29].u32) << ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	// 8250E9E4: 7D7CF378  or r28, r11, r30
	ctx.r[28].u64 = ctx.r[11].u64 | ctx.r[30].u64;
	// 8250E9E8: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8250E9EC: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 8250E9F0: 4E800421  bctrl
	ctx.lr = 0x8250E9F4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8250E9F4: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8250E9F8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8250E9FC: C03AE098  lfs f1, -0x1f68(r26)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-8040 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8250EA00: 38CB0040  addi r6, r11, 0x40
	ctx.r[6].s64 = ctx.r[11].s64 + 64;
	// 8250EA04: 38AB0030  addi r5, r11, 0x30
	ctx.r[5].s64 = ctx.r[11].s64 + 48;
	// 8250EA08: 388B0020  addi r4, r11, 0x20
	ctx.r[4].s64 = ctx.r[11].s64 + 32;
	// 8250EA0C: 4801B735  bl 0x8252a140
	ctx.lr = 0x8250EA10;
	sub_8252A140(ctx, base);
	// 8250EA10: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250EA14: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8250EA18: 409AFF80  bne cr6, 0x8250e998
	if !ctx.cr[6].eq {
	pc = 0x8250E998; continue 'dispatch;
	}
	// 8250EA1C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8250EA20: 382102A0  addi r1, r1, 0x2a0
	ctx.r[1].s64 = ctx.r[1].s64 + 672;
	// 8250EA24: 480266D8  b 0x825350fc
	sub_825350D0(ctx, base);
	return;
	// 8250EA28: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 8250EA2C: 382102A0  addi r1, r1, 0x2a0
	ctx.r[1].s64 = ctx.r[1].s64 + 672;
	// 8250EA30: 480266CC  b 0x825350fc
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250EA38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8250EA38 size=68
    let mut pc: u32 = 0x8250EA38;
    'dispatch: loop {
        match pc {
            0x8250EA38 => {
    //   block [0x8250EA38..0x8250EA7C)
	// 8250EA38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250EA3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8250EA40: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250EA44: 3863FFF0  addi r3, r3, -0x10
	ctx.r[3].s64 = ctx.r[3].s64 + -16;
	// 8250EA48: 480009D9  bl 0x8250f420
	ctx.lr = 0x8250EA4C;
	sub_8250F420(ctx, base);
	// 8250EA4C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8250EA50: 419A0018  beq cr6, 0x8250ea68
	if ctx.cr[6].eq {
	pc = 0x8250EA68; continue 'dispatch;
	}
	// 8250EA54: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250EA58: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8250EA5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8250EA60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8250EA64: 4E800020  blr
	return;
	// 8250EA68: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8250EA6C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8250EA70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8250EA74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8250EA78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250EA80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8250EA80 size=168
    let mut pc: u32 = 0x8250EA80;
    'dispatch: loop {
        match pc {
            0x8250EA80 => {
    //   block [0x8250EA80..0x8250EB28)
	// 8250EA80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250EA84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8250EA88: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8250EA8C: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250EA90: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 8250EA94: 7CAB2B78  mr r11, r5
	ctx.r[11].u64 = ctx.r[5].u64;
	// 8250EA98: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8250EA9C: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 8250EAA0: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 8250EAA4: C00A1FF8  lfs f0, 0x1ff8(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8184 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8250EAA8: D001005C  stfs f0, 0x5c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), tmp.u32 ) };
	// 8250EAAC: C1AB0000  lfs f13, 0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8250EAB0: C0060000  lfs f0, 0(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8250EAB4: EC0D0032  fmuls f0, f13, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 8250EAB8: D0010050  stfs f0, 0x50(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 8250EABC: C18B0004  lfs f12, 4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 8250EAC0: C0060004  lfs f0, 4(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8250EAC4: EC0C0032  fmuls f0, f12, f0
	ctx.f[0].f64 = (((ctx.f[12].f64 * ctx.f[0].f64) as f32) as f64);
	// 8250EAC8: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 8250EACC: C1AB0008  lfs f13, 8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8250EAD0: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 8250EAD4: C0060008  lfs f0, 8(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8250EAD8: EC0D0032  fmuls f0, f13, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 8250EADC: D0010058  stfs f0, 0x58(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), tmp.u32 ) };
	// 8250EAE0: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250EB28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8250EB28 size=704
    let mut pc: u32 = 0x8250EB28;
    'dispatch: loop {
        match pc {
            0x8250EB28 => {
    //   block [0x8250EB28..0x8250EDE8)
	// 8250EB28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250EB2C: 48026571  bl 0x8253509c
	ctx.lr = 0x8250EB30;
	sub_82535080(ctx, base);
	// 8250EB30: DBC1FF90  stfd f30, -0x70(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-112 as u32), ctx.f[30].u64 ) };
	// 8250EB34: DBE1FF98  stfd f31, -0x68(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-104 as u32), ctx.f[31].u64 ) };
	// 8250EB38: 9421FEB0  stwu r1, -0x150(r1)
	ea = ctx.r[1].u32.wrapping_add(-336 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250EB3C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8250EB40: FFC00890  fmr f30, f1
	ctx.f[30].f64 = ctx.f[1].f64;
	// 8250EB44: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 8250EB48: 7C771B78  mr r23, r3
	ctx.r[23].u64 = ctx.r[3].u64;
	// 8250EB4C: 3B3A0010  addi r25, r26, 0x10
	ctx.r[25].s64 = ctx.r[26].s64 + 16;
	// 8250EB50: 7C982378  mr r24, r4
	ctx.r[24].u64 = ctx.r[4].u64;
	// 8250EB54: C00B8CB4  lfs f0, -0x734c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-29516 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8250EB58: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8250EB5C: D0010060  stfs f0, 0x60(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), tmp.u32 ) };
	// 8250EB60: 3AA00000  li r21, 0
	ctx.r[21].s64 = 0;
	// 8250EB64: D0010064  stfs f0, 0x64(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), tmp.u32 ) };
	// 8250EB68: D0010068  stfs f0, 0x68(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), tmp.u32 ) };
	// 8250EB6C: C3EB1FF8  lfs f31, 0x1ff8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 8250EB70: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8250EB74: D3E1006C  stfs f31, 0x6c(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), tmp.u32 ) };
	// 8250EB78: D3E1005C  stfs f31, 0x5c(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), tmp.u32 ) };
	// 8250EB7C: C00B0DA0  lfs f0, 0xda0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3488 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8250EB80: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 8250EB84: D0010050  stfs f0, 0x50(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 8250EB88: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 8250EB8C: D0010058  stfs f0, 0x58(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), tmp.u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250EDE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8250EDE8 size=16
    let mut pc: u32 = 0x8250EDE8;
    'dispatch: loop {
        match pc {
            0x8250EDE8 => {
    //   block [0x8250EDE8..0x8250EDF8)
	// 8250EDE8: 39600020  li r11, 0x20
	ctx.r[11].s64 = 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250EDF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8250EDF8 size=176
    let mut pc: u32 = 0x8250EDF8;
    'dispatch: loop {
        match pc {
            0x8250EDF8 => {
    //   block [0x8250EDF8..0x8250EEA8)
	// 8250EDF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250EDFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8250EE00: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8250EE04: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8250EE08: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 8250EE0C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250EE10: 38800016  li r4, 0x16
	ctx.r[4].s64 = 22;
	// 8250EE14: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 8250EE18: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8250EE1C: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 8250EE20: 4BFFC4A1  bl 0x8250b2c0
	ctx.lr = 0x8250EE24;
	sub_8250B2C0(ctx, base);
	// 8250EE24: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 8250EE28: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8250EE2C: 392A4AA8  addi r9, r10, 0x4aa8
	ctx.r[9].s64 = ctx.r[10].s64 + 19112;
	// 8250EE30: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 8250EE34: 3CE08000  lis r7, -0x8000
	ctx.r[7].s64 = -2147483648;
	// 8250EE38: 390A4A84  addi r8, r10, 0x4a84
	ctx.r[8].s64 = ctx.r[10].s64 + 19076;
	// 8250EE3C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8250EE40: C00B1850  lfs f0, 0x1850(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(6224 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8250EE44: 397F0020  addi r11, r31, 0x20
	ctx.r[11].s64 = ctx.r[31].s64 + 32;
	// 8250EE48: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8250EE4C: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 8250EE50: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8250EE54: 911F0010  stw r8, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[8].u32 ) };
	// 8250EE58: 915F0034  stw r10, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[10].u32 ) };
	// 8250EE5C: 915F0038  stw r10, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[10].u32 ) };
	// 8250EE60: 90FF003C  stw r7, 0x3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[7].u32 ) };
	// 8250EE64: 915F0040  stw r10, 0x40(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), ctx.r[10].u32 ) };
	// 8250EE68: 915F0044  stw r10, 0x44(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(68 as u32), ctx.r[10].u32 ) };
	// 8250EE6C: 90FF0048  stw r7, 0x48(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[7].u32 ) };
	// 8250EE70: D00B0000  stfs f0, 0(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), tmp.u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250EEA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8250EEA8 size=136
    let mut pc: u32 = 0x8250EEA8;
    'dispatch: loop {
        match pc {
            0x8250EEA8 => {
    //   block [0x8250EEA8..0x8250EF30)
	// 8250EEA8: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8250EEAC: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 8250EEB0: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 8250EEB4: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8250EEB8: 396B2A44  addi r11, r11, 0x2a44
	ctx.r[11].s64 = ctx.r[11].s64 + 10820;
	// 8250EEBC: 394A4AA8  addi r10, r10, 0x4aa8
	ctx.r[10].s64 = ctx.r[10].s64 + 19112;
	// 8250EEC0: 39294A84  addi r9, r9, 0x4a84
	ctx.r[9].s64 = ctx.r[9].s64 + 19076;
	// 8250EEC4: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 8250EEC8: B1030006  sth r8, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[8].u16 ) };
	// 8250EECC: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 8250EED0: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 8250EED4: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8250EED8: 91230010  stw r9, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[9].u32 ) };
	// 8250EEDC: 90E3000C  stw r7, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[7].u32 ) };
	// 8250EEE0: 419A0044  beq cr6, 0x8250ef24
	if ctx.cr[6].eq {
	pc = 0x8250EF24; continue 'dispatch;
	}
	// 8250EEE4: 81630038  lwz r11, 0x38(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(56 as u32) ) } as u64;
	// 8250EEE8: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8250EEEC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8250EEF0: 40990034  ble cr6, 0x8250ef24
	if !ctx.cr[6].gt {
	pc = 0x8250EF24; continue 'dispatch;
	}
	// 8250EEF4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8250EEF8: 81630034  lwz r11, 0x34(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(52 as u32) ) } as u64;
	// 8250EEFC: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8250EF00: 88EB0011  lbz r7, 0x11(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(17 as u32) ) } as u64;
	// 8250EF04: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 8250EF08: 409A0008  bne cr6, 0x8250ef10
	if !ctx.cr[6].eq {
	pc = 0x8250EF10; continue 'dispatch;
	}
	// 8250EF0C: 990B0011  stb r8, 0x11(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(17 as u32), ctx.r[8].u8 ) };
	// 8250EF10: 81630038  lwz r11, 0x38(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(56 as u32) ) } as u64;
	// 8250EF14: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 8250EF18: 394A0038  addi r10, r10, 0x38
	ctx.r[10].s64 = ctx.r[10].s64 + 56;
	// 8250EF1C: 7F095800  cmpw cr6, r9, r11
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8250EF20: 4198FFD8  blt cr6, 0x8250eef8
	if ctx.cr[6].lt {
	pc = 0x8250EEF8; continue 'dispatch;
	}
	// 8250EF24: 39600016  li r11, 0x16
	ctx.r[11].s64 = 22;
	// 8250EF28: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 8250EF2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250EF30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8250EF30 size=364
    let mut pc: u32 = 0x8250EF30;
    'dispatch: loop {
        match pc {
            0x8250EF30 => {
    //   block [0x8250EF30..0x8250F09C)
	// 8250EF30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250EF34: 48026169  bl 0x8253509c
	ctx.lr = 0x8250EF38;
	sub_82535080(ctx, base);
	// 8250EF38: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250EF3C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8250EF40: 7C962378  mr r22, r4
	ctx.r[22].u64 = ctx.r[4].u64;
	// 8250EF44: 7CB52B78  mr r21, r5
	ctx.r[21].u64 = ctx.r[5].u64;
	// 8250EF48: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8250EF4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8250EF50: 817D0038  lwz r11, 0x38(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(56 as u32) ) } as u64;
	// 8250EF54: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8250EF58: 40990038  ble cr6, 0x8250ef90
	if !ctx.cr[6].gt {
	pc = 0x8250EF90; continue 'dispatch;
	}
	// 8250EF5C: 815D0034  lwz r10, 0x34(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(52 as u32) ) } as u64;
	// 8250EF60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8250EF64: 7D4B5214  add r10, r11, r10
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8250EF68: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 8250EF6C: 93EA0034  stw r31, 0x34(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(52 as u32), ctx.r[31].u32 ) };
	// 8250EF70: 811D0038  lwz r8, 0x38(r29)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(56 as u32) ) } as u64;
	// 8250EF74: 815D0034  lwz r10, 0x34(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(52 as u32) ) } as u64;
	// 8250EF78: 7F094000  cmpw cr6, r9, r8
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[8].s32, &mut ctx.xer);
	// 8250EF7C: 7D0B5214  add r8, r11, r10
	ctx.r[8].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8250EF80: 396B0038  addi r11, r11, 0x38
	ctx.r[11].s64 = ctx.r[11].s64 + 56;
	// 8250EF84: 8108001C  lwz r8, 0x1c(r8)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(28 as u32) ) } as u64;
	// 8250EF88: 7FE8FA14  add r31, r8, r31
	ctx.r[31].u64 = ctx.r[8].u64 + ctx.r[31].u64;
	// 8250EF8C: 4198FFD8  blt cr6, 0x8250ef64
	if ctx.cr[6].lt {
	pc = 0x8250EF64; continue 'dispatch;
	}
	// 8250EF90: 3AFD0040  addi r23, r29, 0x40
	ctx.r[23].s64 = ctx.r[29].s64 + 64;
	// 8250EF94: 81770008  lwz r11, 8(r23)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250EF98: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 8250EF9C: 7F0BF800  cmpw cr6, r11, r31
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[31].s32, &mut ctx.xer);
	// 8250EFA0: 40980014  bge cr6, 0x8250efb4
	if !ctx.cr[6].lt {
	pc = 0x8250EFB4; continue 'dispatch;
	}
	// 8250EFA4: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 8250EFA8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8250EFAC: 7EE3BB78  mr r3, r23
	ctx.r[3].u64 = ctx.r[23].u64;
	// 8250EFB0: 4BF5F319  bl 0x8246e2c8
	ctx.lr = 0x8250EFB4;
	sub_8246E2C8(ctx, base);
	// 8250EFB4: 81770008  lwz r11, 8(r23)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250EFB8: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 8250EFBC: 7F0BF800  cmpw cr6, r11, r31
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[31].s32, &mut ctx.xer);
	// 8250EFC0: 40980024  bge cr6, 0x8250efe4
	if !ctx.cr[6].lt {
	pc = 0x8250EFE4; continue 'dispatch;
	}
	// 8250EFC4: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8250EFC8: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8250EFCC: 41980008  blt cr6, 0x8250efd4
	if ctx.cr[6].lt {
	pc = 0x8250EFD4; continue 'dispatch;
	}
	// 8250EFD0: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 8250EFD4: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 8250EFD8: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 8250EFDC: 7EE3BB78  mr r3, r23
	ctx.r[3].u64 = ctx.r[23].u64;
	// 8250EFE0: 4BF5F2E9  bl 0x8246e2c8
	ctx.lr = 0x8250EFE4;
	sub_8246E2C8(ctx, base);
	// 8250EFE4: 93F70004  stw r31, 4(r23)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[23].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 8250EFE8: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 8250EFEC: 817D0038  lwz r11, 0x38(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(56 as u32) ) } as u64;
	// 8250EFF0: 3B000000  li r24, 0
	ctx.r[24].s64 = 0;
	// 8250EFF4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8250EFF8: 40990090  ble cr6, 0x8250f088
	if !ctx.cr[6].gt {
	pc = 0x8250F088; continue 'dispatch;
	}
	// 8250EFFC: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 8250F000: 817D0034  lwz r11, 0x34(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(52 as u32) ) } as u64;
	// 8250F004: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8250F008: 7D7A5A14  add r11, r26, r11
	ctx.r[11].u64 = ctx.r[26].u64 + ctx.r[11].u64;
	// 8250F00C: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 8250F010: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8250F014: 40990060  ble cr6, 0x8250f074
	if !ctx.cr[6].gt {
	pc = 0x8250F074; continue 'dispatch;
	}
	// 8250F018: 3975FFFB  addi r11, r21, -5
	ctx.r[11].s64 = ctx.r[21].s64 + -5;
	// 8250F01C: 577E083C  slwi r30, r27, 1
	ctx.r[30].u32 = ctx.r[27].u32.wrapping_shl(1);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 8250F020: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 8250F024: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8250F028: 69790001  xori r25, r11, 1
	ctx.r[25].u64 = ctx.r[11].u64 ^ 1;
	// 8250F02C: 815D0030  lwz r10, 0x30(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(48 as u32) ) } as u64;
	// 8250F030: 7F25CB78  mr r5, r25
	ctx.r[5].u64 = ctx.r[25].u64;
	// 8250F034: 81770000  lwz r11, 0(r23)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250F038: 7EC4B378  mr r4, r22
	ctx.r[4].u64 = ctx.r[22].u64;
	// 8250F03C: 214A0020  subfic r10, r10, 0x20
	ctx.xer.ca = ctx.r[10].u32 <= 32 as u32;
	ctx.r[10].s64 = (32 as i64) - ctx.r[10].s64;
	// 8250F040: 7F8BF214  add r28, r11, r30
	ctx.r[28].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 8250F044: 7F0B5030  slw r11, r24, r10
	if (ctx.r[10].u8 & 0x20) != 0 {
		ctx.r[11].u64 = 0;
	} else {
		ctx.r[11].u64 = ((ctx.r[24].u32) << ((ctx.r[10].u8 & 0x1F) as u32)) as u64;
	}
	// 8250F048: 7D63FB78  or r3, r11, r31
	ctx.r[3].u64 = ctx.r[11].u64 | ctx.r[31].u64;
	// 8250F04C: 4801ADA5  bl 0x82529df0
	ctx.lr = 0x8250F050;
	sub_82529DF0(ctx, base);
	// 8250F050: B07C0000  sth r3, 0(r28)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[3].u16 ) };
	// 8250F054: 817D0034  lwz r11, 0x34(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(52 as u32) ) } as u64;
	// 8250F058: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 8250F05C: 3B7B0001  addi r27, r27, 1
	ctx.r[27].s64 = ctx.r[27].s64 + 1;
	// 8250F060: 7D7A5A14  add r11, r26, r11
	ctx.r[11].u64 = ctx.r[26].u64 + ctx.r[11].u64;
	// 8250F064: 3BDE0002  addi r30, r30, 2
	ctx.r[30].s64 = ctx.r[30].s64 + 2;
	// 8250F068: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 8250F06C: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8250F070: 4198FFBC  blt cr6, 0x8250f02c
	if ctx.cr[6].lt {
	pc = 0x8250F02C; continue 'dispatch;
	}
	// 8250F074: 817D0038  lwz r11, 0x38(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(56 as u32) ) } as u64;
	// 8250F078: 3B180001  addi r24, r24, 1
	ctx.r[24].s64 = ctx.r[24].s64 + 1;
	// 8250F07C: 3B5A0038  addi r26, r26, 0x38
	ctx.r[26].s64 = ctx.r[26].s64 + 56;
	// 8250F080: 7F185800  cmpw cr6, r24, r11
	ctx.cr[6].compare_i32(ctx.r[24].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8250F084: 4198FF7C  blt cr6, 0x8250f000
	if ctx.cr[6].lt {
	pc = 0x8250F000; continue 'dispatch;
	}
	// 8250F088: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8250F08C: 997D0014  stb r11, 0x14(r29)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[29].u32.wrapping_add(20 as u32), ctx.r[11].u8 ) };
	// 8250F090: 9ABD004C  stb r21, 0x4c(r29)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[29].u32.wrapping_add(76 as u32), ctx.r[21].u8 ) };
	// 8250F094: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 8250F098: 48026054  b 0x825350ec
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250F0A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8250F0A0 size=560
    let mut pc: u32 = 0x8250F0A0;
    'dispatch: loop {
        match pc {
            0x8250F0A0 => {
    //   block [0x8250F0A0..0x8250F2D0)
	// 8250F0A0: FBC1FFF0  std r30, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[30].u64 ) };
	// 8250F0A4: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 8250F0A8: 81630020  lwz r11, 0x20(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 8250F0AC: 3920FFFF  li r9, -1
	ctx.r[9].s64 = -1;
	// 8250F0B0: 81430024  lwz r10, 0x24(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(36 as u32) ) } as u64;
	// 8250F0B4: 210B0020  subfic r8, r11, 0x20
	ctx.xer.ca = ctx.r[11].u32 <= 32 as u32;
	ctx.r[8].s64 = (32 as i64) - ctx.r[11].s64;
	// 8250F0B8: 7D2B5C30  srw r11, r9, r11
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[11].u64 = 0;
	} else {
		ctx.r[11].u64 = ((ctx.r[9].u32) >> ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	// 8250F0BC: 7D662038  and r6, r11, r4
	ctx.r[6].u64 = ctx.r[11].u64 & ctx.r[4].u64;
	// 8250F0C0: 7C8B4430  srw r11, r4, r8
	if (ctx.r[8].u8 & 0x20) != 0 {
		ctx.r[11].u64 = 0;
	} else {
		ctx.r[11].u64 = ((ctx.r[4].u32) >> ((ctx.r[8].u8 & 0x1F) as u32)) as u64;
	}
	// 8250F0C4: 1D6B0038  mulli r11, r11, 0x38
	ctx.r[11].s64 = ctx.r[11].s64 * 56;
	// 8250F0C8: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8250F0CC: 892B0010  lbz r9, 0x10(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 8250F0D0: 814B0018  lwz r10, 0x18(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 8250F0D4: 2B090001  cmplwi cr6, r9, 1
	ctx.cr[6].compare_u32(ctx.r[9].u32, 1 as u32, &mut ctx.xer);
	// 8250F0D8: 812B0014  lwz r9, 0x14(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 8250F0DC: 7D4A3038  and r10, r10, r6
	ctx.r[10].u64 = ctx.r[10].u64 & ctx.r[6].u64;
	// 8250F0E0: 409A0054  bne cr6, 0x8250f134
	if !ctx.cr[6].eq {
	pc = 0x8250F134; continue 'dispatch;
	}
	// 8250F0E4: 69440001  xori r4, r10, 1
	ctx.r[4].u64 = ctx.r[10].u64 ^ 1;
	// 8250F0E8: 83EB000C  lwz r31, 0xc(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8250F0EC: 7CE931D6  mullw r7, r9, r6
	ctx.r[7].s64 = (ctx.r[9].s32 as i64) * (ctx.r[6].s32 as i64);
	// 8250F0F0: 812B0004  lwz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8250F0F4: 810B0000  lwz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250F0F8: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8250F0FC: 38840001  addi r4, r4, 1
	ctx.r[4].s64 = ctx.r[4].s64 + 1;
	// 8250F100: 7CE7FA14  add r7, r7, r31
	ctx.r[7].u64 = ctx.r[7].u64 + ctx.r[31].u64;
	// 8250F104: 554A083C  slwi r10, r10, 1
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8250F108: 5484083C  slwi r4, r4, 1
	ctx.r[4].u32 = ctx.r[4].u32.wrapping_shl(1);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 8250F10C: A3E70000  lhz r31, 0(r7)
	ctx.r[31].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250F110: 7FCA3A2E  lhzx r30, r10, r7
	ctx.r[30].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[7].u32)) } as u64;
	// 8250F114: 7C843A2E  lhzx r4, r4, r7
	ctx.r[4].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[4].u32.wrapping_add(ctx.r[7].u32)) } as u64;
	// 8250F118: 7D5F49D6  mullw r10, r31, r9
	ctx.r[10].s64 = (ctx.r[31].s32 as i64) * (ctx.r[9].s32 as i64);
	// 8250F11C: 7CFE49D6  mullw r7, r30, r9
	ctx.r[7].s64 = (ctx.r[30].s32 as i64) * (ctx.r[9].s32 as i64);
	// 8250F120: 7D2449D6  mullw r9, r4, r9
	ctx.r[9].s64 = (ctx.r[4].s32 as i64) * (ctx.r[9].s32 as i64);
	// 8250F124: 7D4A4214  add r10, r10, r8
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[8].u64;
	// 8250F128: 7CE74214  add r7, r7, r8
	ctx.r[7].u64 = ctx.r[7].u64 + ctx.r[8].u64;
	// 8250F12C: 7D294214  add r9, r9, r8
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[8].u64;
	// 8250F130: 48000050  b 0x8250f180
	pc = 0x8250F180; continue 'dispatch;
	// 8250F134: 69470001  xori r7, r10, 1
	ctx.r[7].u64 = ctx.r[10].u64 ^ 1;
	// 8250F138: 808B000C  lwz r4, 0xc(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8250F13C: 7D0931D6  mullw r8, r9, r6
	ctx.r[8].s64 = (ctx.r[9].s32 as i64) * (ctx.r[6].s32 as i64);
	// 8250F140: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250F144: 3BEA0001  addi r31, r10, 1
	ctx.r[31].s64 = ctx.r[10].s64 + 1;
	// 8250F148: 38E70001  addi r7, r7, 1
	ctx.r[7].s64 = ctx.r[7].s64 + 1;
	// 8250F14C: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8250F150: 7D082214  add r8, r8, r4
	ctx.r[8].u64 = ctx.r[8].u64 + ctx.r[4].u64;
	// 8250F154: 57E4103A  slwi r4, r31, 2
	ctx.r[4].u32 = ctx.r[31].u32.wrapping_shl(2);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 8250F158: 54E7103A  slwi r7, r7, 2
	ctx.r[7].u32 = ctx.r[7].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 8250F15C: 83E80000  lwz r31, 0(r8)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250F160: 7FC4402E  lwzx r30, r4, r8
	ctx.r[30].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[4].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 8250F164: 7D07402E  lwzx r8, r7, r8
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[7].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 8250F168: 7C9F51D6  mullw r4, r31, r10
	ctx.r[4].s64 = (ctx.r[31].s32 as i64) * (ctx.r[10].s32 as i64);
	// 8250F16C: 7CFE51D6  mullw r7, r30, r10
	ctx.r[7].s64 = (ctx.r[30].s32 as i64) * (ctx.r[10].s32 as i64);
	// 8250F170: 7D0851D6  mullw r8, r8, r10
	ctx.r[8].s64 = (ctx.r[8].s32 as i64) * (ctx.r[10].s32 as i64);
	// 8250F174: 7D444A14  add r10, r4, r9
	ctx.r[10].u64 = ctx.r[4].u64 + ctx.r[9].u64;
	// 8250F178: 7CE74A14  add r7, r7, r9
	ctx.r[7].u64 = ctx.r[7].u64 + ctx.r[9].u64;
	// 8250F17C: 7D284A14  add r9, r8, r9
	ctx.r[9].u64 = ctx.r[8].u64 + ctx.r[9].u64;
	// 8250F180: 3D00820D  lis r8, -0x7df3
	ctx.r[8].s64 = -2113077248;
	// 8250F184: C1A30014  lfs f13, 0x14(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8250F188: C1630018  lfs f11, 0x18(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 8250F18C: 80830034  lwz r4, 0x34(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(52 as u32) ) } as u64;
	// 8250F190: C12A0008  lfs f9, 8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) };
	ctx.f[9].f64 = (tmp.f32 as f64);
	// 8250F194: C0030010  lfs f0, 0x10(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8250F198: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 8250F19C: C1070000  lfs f8, 0(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) };
	ctx.f[8].f64 = (tmp.f32 as f64);
	// 8250F1A0: C1881FF8  lfs f12, 0x1ff8(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(8184 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 8250F1A4: D181FFCC  stfs f12, -0x34(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-52 as u32), tmp.u32 ) };
	// 8250F1A8: D181FFDC  stfs f12, -0x24(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-36 as u32), tmp.u32 ) };
	// 8250F1AC: D181FFEC  stfs f12, -0x14(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-20 as u32), tmp.u32 ) };
	// 8250F1B0: C18A0004  lfs f12, 4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 8250F1B4: ED8C0372  fmuls f12, f12, f13
	ctx.f[12].f64 = (((ctx.f[12].f64 * ctx.f[13].f64) as f32) as f64);
	// 8250F1B8: D181FFC4  stfs f12, -0x3c(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-60 as u32), tmp.u32 ) };
	// 8250F1BC: C14A0000  lfs f10, 0(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 8250F1C0: ED8902F2  fmuls f12, f9, f11
	ctx.f[12].f64 = (((ctx.f[9].f64 * ctx.f[11].f64) as f32) as f64);
	// 8250F1C4: C0A90000  lfs f5, 0(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) };
	ctx.f[5].f64 = (tmp.f32 as f64);
	// 8250F1C8: ED4A0032  fmuls f10, f10, f0
	ctx.f[10].f64 = (((ctx.f[10].f64 * ctx.f[0].f64) as f32) as f64);
	// 8250F1CC: D181FFC8  stfs f12, -0x38(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-56 as u32), tmp.u32 ) };
	// 8250F1D0: ED800232  fmuls f12, f0, f8
	ctx.f[12].f64 = (((ctx.f[0].f64 * ctx.f[8].f64) as f32) as f64);
	// 8250F1D4: C0E70004  lfs f7, 4(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(4 as u32) ) };
	ctx.f[7].f64 = (tmp.f32 as f64);
	// 8250F1D8: EC000172  fmuls f0, f0, f5
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[5].f64) as f32) as f64);
	// 8250F1DC: C0890004  lfs f4, 4(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) };
	ctx.f[4].f64 = (tmp.f32 as f64);
	// 8250F1E0: D181FFD0  stfs f12, -0x30(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), tmp.u32 ) };
	// 8250F1E4: ED870372  fmuls f12, f7, f13
	ctx.f[12].f64 = (((ctx.f[7].f64 * ctx.f[13].f64) as f32) as f64);
	// 8250F1E8: D001FFE0  stfs f0, -0x20(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), tmp.u32 ) };
	// 8250F1EC: EC040372  fmuls f0, f4, f13
	ctx.f[0].f64 = (((ctx.f[4].f64 * ctx.f[13].f64) as f32) as f64);
	// 8250F1F0: C0C70008  lfs f6, 8(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(8 as u32) ) };
	ctx.f[6].f64 = (tmp.f32 as f64);
	// 8250F1F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8250F1F8: C0690008  lfs f3, 8(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) };
	ctx.f[3].f64 = (tmp.f32 as f64);
	// 8250F1FC: D181FFD4  stfs f12, -0x2c(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-44 as u32), tmp.u32 ) };
	// 8250F200: ED8602F2  fmuls f12, f6, f11
	ctx.f[12].f64 = (((ctx.f[6].f64 * ctx.f[11].f64) as f32) as f64);
	// 8250F204: D001FFE4  stfs f0, -0x1c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-28 as u32), tmp.u32 ) };
	// 8250F208: EC0302F2  fmuls f0, f3, f11
	ctx.f[0].f64 = (((ctx.f[3].f64 * ctx.f[11].f64) as f32) as f64);
	// 8250F20C: D141FFC0  stfs f10, -0x40(r1)
	tmp.f32 = (ctx.f[10].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-64 as u32), tmp.u32 ) };
	// 8250F210: D181FFD8  stfs f12, -0x28(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), tmp.u32 ) };
	// 8250F214: D001FFE8  stfs f0, -0x18(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), tmp.u32 ) };
	// 8250F218: 409A000C  bne cr6, 0x8250f224
	if !ctx.cr[6].eq {
	pc = 0x8250F224; continue 'dispatch;
	}
	// 8250F21C: 7CE83B78  mr r8, r7
	ctx.r[8].u64 = ctx.r[7].u64;
	// 8250F220: 48000018  b 0x8250f238
	pc = 0x8250F238; continue 'dispatch;
	// 8250F224: 816B0034  lwz r11, 0x34(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 8250F228: 81430030  lwz r10, 0x30(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) } as u64;
	// 8250F22C: 7D6B3214  add r11, r11, r6
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[6].u64;
	// 8250F230: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8250F234: 7D0B522E  lhzx r8, r11, r10
	ctx.r[8].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 8250F238: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 8250F23C: 419A0054  beq cr6, 0x8250f290
	if ctx.cr[6].eq {
	pc = 0x8250F290; continue 'dispatch;
	}
	// 8250F240: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 8250F244: 8943003C  lbz r10, 0x3c(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(60 as u32) ) } as u64;
	// 8250F248: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 8250F24C: C0030040  lfs f0, 0x40(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(64 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8250F250: 39293134  addi r9, r9, 0x3134
	ctx.r[9].s64 = ctx.r[9].s64 + 12596;
	// 8250F254: D0050010  stfs f0, 0x10(r5)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 8250F258: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 8250F25C: 90E50008  stw r7, 8(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(8 as u32), ctx.r[7].u32 ) };
	// 8250F260: 39650050  addi r11, r5, 0x50
	ctx.r[11].s64 = ctx.r[5].s64 + 80;
	// 8250F264: B1050014  sth r8, 0x14(r5)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[5].u32.wrapping_add(20 as u32), ctx.r[8].u16 ) };
	// 8250F268: 99450016  stb r10, 0x16(r5)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[5].u32.wrapping_add(22 as u32), ctx.r[10].u8 ) };
	// 8250F26C: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 8250F270: B0C50006  sth r6, 6(r5)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[5].u32.wrapping_add(6 as u32), ctx.r[6].u16 ) };
	// 8250F274: 91250000  stw r9, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8250F278: 9085000C  stw r4, 0xc(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(12 as u32), ctx.r[4].u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250F2D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8250F2D0 size=160
    let mut pc: u32 = 0x8250F2D0;
    'dispatch: loop {
        match pc {
            0x8250F2D0 => {
    //   block [0x8250F2D0..0x8250F370)
	// 8250F2D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250F2D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8250F2D8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8250F2DC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8250F2E0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250F2E4: 3BE30034  addi r31, r3, 0x34
	ctx.r[31].s64 = ctx.r[3].s64 + 52;
	// 8250F2E8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8250F2EC: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250F2F0: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8250F2F4: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 8250F2F8: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8250F2FC: 409A0010  bne cr6, 0x8250f30c
	if !ctx.cr[6].eq {
	pc = 0x8250F30C; continue 'dispatch;
	}
	// 8250F300: 38800038  li r4, 0x38
	ctx.r[4].s64 = 56;
	// 8250F304: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8250F308: 4BF5F049  bl 0x8246e350
	ctx.lr = 0x8250F30C;
	sub_8246E350(ctx, base);
	// 8250F30C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8250F310: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8250F314: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250F318: 38A00038  li r5, 0x38
	ctx.r[5].s64 = 56;
	// 8250F31C: 1D4B0038  mulli r10, r11, 0x38
	ctx.r[10].s64 = ctx.r[11].s64 * 56;
	// 8250F320: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8250F324: 7FCA4A14  add r30, r10, r9
	ctx.r[30].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 8250F328: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8250F32C: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8250F330: 48025821  bl 0x82534b50
	ctx.lr = 0x8250F334;
	sub_82534B50(ctx, base);
	// 8250F334: 817E0020  lwz r11, 0x20(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 8250F338: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8250F33C: 409A001C  bne cr6, 0x8250f358
	if !ctx.cr[6].eq {
	pc = 0x8250F358; continue 'dispatch;
	}
	// 8250F340: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8250F344: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8250F348: 396B9F50  addi r11, r11, -0x60b0
	ctx.r[11].s64 = ctx.r[11].s64 + -24752;
	// 8250F34C: 915E0030  stw r10, 0x30(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(48 as u32), ctx.r[10].u32 ) };
	// 8250F350: 917E0028  stw r11, 0x28(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 8250F354: 917E0020  stw r11, 0x20(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 8250F358: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8250F35C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8250F360: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8250F364: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8250F368: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8250F36C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250F370(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8250F370 size=176
    let mut pc: u32 = 0x8250F370;
    'dispatch: loop {
        match pc {
            0x8250F370 => {
    //   block [0x8250F370..0x8250F420)
	// 8250F370: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250F374: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8250F378: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8250F37C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8250F380: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250F384: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8250F388: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8250F38C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8250F390: 388B46BC  addi r4, r11, 0x46bc
	ctx.r[4].s64 = ctx.r[11].s64 + 18108;
	// 8250F394: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 8250F398: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250F39C: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8250F3A0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8250F3A4: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 8250F3A8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8250F3AC: 4E800421  bctrl
	ctx.lr = 0x8250F3B0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8250F3B0: 817E003C  lwz r11, 0x3c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(60 as u32) ) } as u64;
	// 8250F3B4: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 8250F3B8: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8250F3BC: 409A0038  bne cr6, 0x8250f3f4
	if !ctx.cr[6].eq {
	pc = 0x8250F3F4; continue 'dispatch;
	}
	// 8250F3C0: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250F3C4: 556800BE  clrlwi r8, r11, 2
	ctx.r[8].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 8250F3C8: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8250F3CC: 813E0038  lwz r9, 0x38(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(56 as u32) ) } as u64;
	// 8250F3D0: 80DE0034  lwz r6, 0x34(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(52 as u32) ) } as u64;
	// 8250F3D4: 1D080038  mulli r8, r8, 0x38
	ctx.r[8].s64 = ctx.r[8].s64 * 56;
	// 8250F3D8: 388B4AD0  addi r4, r11, 0x4ad0
	ctx.r[4].s64 = ctx.r[11].s64 + 19152;
	// 8250F3DC: 816A0008  lwz r11, 8(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250F3E0: 1CE90038  mulli r7, r9, 0x38
	ctx.r[7].s64 = ctx.r[9].s64 * 56;
	// 8250F3E4: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8250F3E8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8250F3EC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8250F3F0: 4E800421  bctrl
	ctx.lr = 0x8250F3F4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8250F3F4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250F3F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8250F3FC: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 8250F400: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8250F404: 4E800421  bctrl
	ctx.lr = 0x8250F408;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8250F408: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8250F40C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8250F410: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8250F414: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8250F418: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8250F41C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250F420(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8250F420 size=76
    let mut pc: u32 = 0x8250F420;
    'dispatch: loop {
        match pc {
            0x8250F420 => {
    //   block [0x8250F420..0x8250F46C)
	// 8250F420: 81430030  lwz r10, 0x30(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) } as u64;
	// 8250F424: 81230034  lwz r9, 0x34(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(52 as u32) ) } as u64;
	// 8250F428: 216A0020  subfic r11, r10, 0x20
	ctx.xer.ca = ctx.r[10].u32 <= 32 as u32;
	ctx.r[11].s64 = (32 as i64) - ctx.r[10].s64;
	// 8250F42C: 7C8B5C30  srw r11, r4, r11
	if (ctx.r[11].u8 & 0x20) != 0 {
		ctx.r[11].u64 = 0;
	} else {
		ctx.r[11].u64 = ((ctx.r[4].u32) >> ((ctx.r[11].u8 & 0x1F) as u32)) as u64;
	}
	// 8250F430: 1D6B0038  mulli r11, r11, 0x38
	ctx.r[11].s64 = ctx.r[11].s64 * 56;
	// 8250F434: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 8250F438: 812B0020  lwz r9, 0x20(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 8250F43C: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 8250F440: 419A004C  beq cr6, 0x8250f48c
	if ctx.cr[6].eq {
		sub_8250F48C(ctx, base);
		return;
	}
	// 8250F444: 3900FFFF  li r8, -1
	ctx.r[8].s64 = -1;
	// 8250F448: 80EB0024  lwz r7, 0x24(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 8250F44C: 88CB0011  lbz r6, 0x11(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(17 as u32) ) } as u64;
	// 8250F450: 2B060001  cmplwi cr6, r6, 1
	ctx.cr[6].compare_u32(ctx.r[6].u32, 1 as u32, &mut ctx.xer);
	// 8250F454: 7D0A5430  srw r10, r8, r10
	if (ctx.r[10].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[8].u32) >> ((ctx.r[10].u8 & 0x1F) as u32)) as u64;
	}
	// 8250F458: 7D4A2038  and r10, r10, r4
	ctx.r[10].u64 = ctx.r[10].u64 & ctx.r[4].u64;
	// 8250F45C: 7D4A39D6  mullw r10, r10, r7
	ctx.r[10].s64 = (ctx.r[10].s32 as i64) * (ctx.r[7].s32 as i64);
	// 8250F460: 409A000C  bne cr6, 0x8250f46c
	if !ctx.cr[6].eq {
		sub_8250F46C(ctx, base);
		return;
	}
	// 8250F464: 7D4A48AE  lbzx r10, r10, r9
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 8250F468: 48000008  b 0x8250f470
	sub_8250F46C(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250F46C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8250F46C size=32
    let mut pc: u32 = 0x8250F46C;
    'dispatch: loop {
        match pc {
            0x8250F46C => {
    //   block [0x8250F46C..0x8250F48C)
	// 8250F46C: 7D4A4A2E  lhzx r10, r10, r9
	ctx.r[10].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 8250F470: 2F0AFFFF  cmpwi cr6, r10, -1
	ctx.cr[6].compare_i32(ctx.r[10].s32, -1, &mut ctx.xer);
	// 8250F474: 419A0018  beq cr6, 0x8250f48c
	if ctx.cr[6].eq {
		sub_8250F48C(ctx, base);
		return;
	}
	// 8250F478: 810B002C  lwz r8, 0x2c(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 8250F47C: 812B0028  lwz r9, 0x28(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 8250F480: 7D6851D6  mullw r11, r8, r10
	ctx.r[11].s64 = (ctx.r[8].s32 as i64) * (ctx.r[10].s32 as i64);
	// 8250F484: 7C6B4A14  add r3, r11, r9
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 8250F488: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250F48C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8250F48C size=8
    let mut pc: u32 = 0x8250F48C;
    'dispatch: loop {
        match pc {
            0x8250F48C => {
    //   block [0x8250F48C..0x8250F494)
	// 8250F48C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8250F490: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250F498(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8250F498 size=308
    let mut pc: u32 = 0x8250F498;
    'dispatch: loop {
        match pc {
            0x8250F498 => {
    //   block [0x8250F498..0x8250F5CC)
	// 8250F498: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250F49C: 48025C1D  bl 0x825350b8
	ctx.lr = 0x8250F4A0;
	sub_82535080(ctx, base);
	// 8250F4A0: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 8250F4A4: 816A0024  lwz r11, 0x24(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(36 as u32) ) } as u64;
	// 8250F4A8: 812A0034  lwz r9, 0x34(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(52 as u32) ) } as u64;
	// 8250F4AC: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8250F4B0: 810B0014  lwz r8, 0x14(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 8250F4B4: 812B0018  lwz r9, 0x18(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 8250F4B8: 806B000C  lwz r3, 0xc(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8250F4BC: 7CE821D6  mullw r7, r8, r4
	ctx.r[7].s64 = (ctx.r[8].s32 as i64) * (ctx.r[4].s32 as i64);
	// 8250F4C0: 810B0000  lwz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250F4C4: 7D262038  and r6, r9, r4
	ctx.r[6].u64 = ctx.r[9].u64 & ctx.r[4].u64;
	// 8250F4C8: 7CE71A14  add r7, r7, r3
	ctx.r[7].u64 = ctx.r[7].u64 + ctx.r[3].u64;
	// 8250F4CC: 812B0004  lwz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8250F4D0: 68C30001  xori r3, r6, 1
	ctx.r[3].u64 = ctx.r[6].u64 ^ 1;
	// 8250F4D4: 38C60001  addi r6, r6, 1
	ctx.r[6].s64 = ctx.r[6].s64 + 1;
	// 8250F4D8: 38630001  addi r3, r3, 1
	ctx.r[3].s64 = ctx.r[3].s64 + 1;
	// 8250F4DC: 54C6083C  slwi r6, r6, 1
	ctx.r[6].u32 = ctx.r[6].u32.wrapping_shl(1);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 8250F4E0: 5463083C  slwi r3, r3, 1
	ctx.r[3].u32 = ctx.r[3].u32.wrapping_shl(1);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 8250F4E4: A3E70000  lhz r31, 0(r7)
	ctx.r[31].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250F4E8: 7FC63A2E  lhzx r30, r6, r7
	ctx.r[30].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[6].u32.wrapping_add(ctx.r[7].u32)) } as u64;
	// 8250F4EC: 7CDF49D6  mullw r6, r31, r9
	ctx.r[6].s64 = (ctx.r[31].s32 as i64) * (ctx.r[9].s32 as i64);
	// 8250F4F0: 7C633A2E  lhzx r3, r3, r7
	ctx.r[3].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[3].u32.wrapping_add(ctx.r[7].u32)) } as u64;
	// 8250F4F4: 7CFE49D6  mullw r7, r30, r9
	ctx.r[7].s64 = (ctx.r[30].s32 as i64) * (ctx.r[9].s32 as i64);
	// 8250F4F8: 7D2349D6  mullw r9, r3, r9
	ctx.r[9].s64 = (ctx.r[3].s32 as i64) * (ctx.r[9].s32 as i64);
	// 8250F4FC: 7FE64214  add r31, r6, r8
	ctx.r[31].u64 = ctx.r[6].u64 + ctx.r[8].u64;
	// 8250F500: 7FC74214  add r30, r7, r8
	ctx.r[30].u64 = ctx.r[7].u64 + ctx.r[8].u64;
	// 8250F504: 7FA94214  add r29, r9, r8
	ctx.r[29].u64 = ctx.r[9].u64 + ctx.r[8].u64;
	// 8250F508: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8250F50C: 409A000C  bne cr6, 0x8250f518
	if !ctx.cr[6].eq {
	pc = 0x8250F518; continue 'dispatch;
	}
	// 8250F510: 7CC73378  mr r7, r6
	ctx.r[7].u64 = ctx.r[6].u64;
	// 8250F514: 48000018  b 0x8250f52c
	pc = 0x8250F52C; continue 'dispatch;
	// 8250F518: 816B0034  lwz r11, 0x34(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 8250F51C: 812A0030  lwz r9, 0x30(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(48 as u32) ) } as u64;
	// 8250F520: 7D6B2214  add r11, r11, r4
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 8250F524: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8250F528: 7CEB4A2E  lhzx r7, r11, r9
	ctx.r[7].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 8250F52C: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 8250F530: 419A0054  beq cr6, 0x8250f584
	if ctx.cr[6].eq {
	pc = 0x8250F584; continue 'dispatch;
	}
	// 8250F534: 3D008206  lis r8, -0x7dfa
	ctx.r[8].s64 = -2113536000;
	// 8250F538: 892A003C  lbz r9, 0x3c(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(60 as u32) ) } as u64;
	// 8250F53C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8250F540: C00A0040  lfs f0, 0x40(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(64 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8250F544: 39083134  addi r8, r8, 0x3134
	ctx.r[8].s64 = ctx.r[8].s64 + 12596;
	// 8250F548: D0050010  stfs f0, 0x10(r5)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 8250F54C: 3B800004  li r28, 4
	ctx.r[28].s64 = 4;
	// 8250F550: 90C50008  stw r6, 8(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(8 as u32), ctx.r[6].u32 ) };
	// 8250F554: 39650050  addi r11, r5, 0x50
	ctx.r[11].s64 = ctx.r[5].s64 + 80;
	// 8250F558: B0E50014  sth r7, 0x14(r5)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[5].u32.wrapping_add(20 as u32), ctx.r[7].u16 ) };
	// 8250F55C: 99250016  stb r9, 0x16(r5)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[5].u32.wrapping_add(22 as u32), ctx.r[9].u8 ) };
	// 8250F560: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 8250F564: B0850006  sth r4, 6(r5)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[5].u32.wrapping_add(6 as u32), ctx.r[4].u16 ) };
	// 8250F568: 91050000  stw r8, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 8250F56C: 9385000C  stw r28, 0xc(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(12 as u32), ctx.r[28].u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250F5D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8250F5D0 size=72
    let mut pc: u32 = 0x8250F5D0;
    'dispatch: loop {
        match pc {
            0x8250F5D0 => {
    //   block [0x8250F5D0..0x8250F618)
	// 8250F5D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250F5D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8250F5D8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8250F5DC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250F5E0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8250F5E4: 4BFFF815  bl 0x8250edf8
	ctx.lr = 0x8250F5E8;
	sub_8250EDF8(ctx, base);
	// 8250F5E8: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8250F5EC: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 8250F5F0: 396B3BA0  addi r11, r11, 0x3ba0
	ctx.r[11].s64 = ctx.r[11].s64 + 15264;
	// 8250F5F4: 394A3B7C  addi r10, r10, 0x3b7c
	ctx.r[10].s64 = ctx.r[10].s64 + 15228;
	// 8250F5F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8250F5FC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8250F600: 915F0010  stw r10, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 8250F604: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8250F608: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8250F60C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8250F610: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8250F614: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250F618(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8250F618 size=8
    let mut pc: u32 = 0x8250F618;
    'dispatch: loop {
        match pc {
            0x8250F618 => {
    //   block [0x8250F618..0x8250F620)
	// 8250F618: 80630010  lwz r3, 0x10(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 8250F61C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250F620(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8250F620 size=96
    let mut pc: u32 = 0x8250F620;
    'dispatch: loop {
        match pc {
            0x8250F620 => {
    //   block [0x8250F620..0x8250F680)
	// 8250F620: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250F624: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8250F628: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8250F62C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250F630: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8250F634: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8250F638: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 8250F63C: 388B4AF0  addi r4, r11, 0x4af0
	ctx.r[4].s64 = ctx.r[11].s64 + 19184;
	// 8250F640: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8250F644: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250F648: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8250F64C: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 8250F650: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8250F654: 4E800421  bctrl
	ctx.lr = 0x8250F658;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8250F658: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250F65C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8250F660: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 8250F664: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8250F668: 4E800421  bctrl
	ctx.lr = 0x8250F66C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8250F66C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8250F670: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8250F674: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8250F678: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8250F67C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250F680(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8250F680 size=388
    let mut pc: u32 = 0x8250F680;
    'dispatch: loop {
        match pc {
            0x8250F680 => {
    //   block [0x8250F680..0x8250F804)
	// 8250F680: FBC1FFF0  std r30, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[30].u64 ) };
	// 8250F684: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 8250F688: C0090010  lfs f0, 0x10(r9)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(16 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8250F68C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8250F690: D001FFE0  stfs f0, -0x20(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), tmp.u32 ) };
	// 8250F694: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 8250F698: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8250F69C: 2F070004  cmpwi cr6, r7, 4
	ctx.cr[6].compare_i32(ctx.r[7].s32, 4, &mut ctx.xer);
	// 8250F6A0: 41980070  blt cr6, 0x8250f710
	if ctx.cr[6].lt {
	pc = 0x8250F710; continue 'dispatch;
	}
	// 8250F6A4: 3887FFFD  addi r4, r7, -3
	ctx.r[4].s64 = ctx.r[7].s64 + -3;
	// 8250F6A8: 39450008  addi r10, r5, 8
	ctx.r[10].s64 = ctx.r[5].s64 + 8;
	// 8250F6AC: C1AAFFF8  lfs f13, -8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-8 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8250F6B0: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 8250F6B4: 4098000C  bge cr6, 0x8250f6c0
	if !ctx.cr[6].lt {
	pc = 0x8250F6C0; continue 'dispatch;
	}
	// 8250F6B8: FC006890  fmr f0, f13
	ctx.f[0].f64 = ctx.f[13].f64;
	// 8250F6BC: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 8250F6C0: C1AAFFFC  lfs f13, -4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8250F6C4: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 8250F6C8: 4098000C  bge cr6, 0x8250f6d4
	if !ctx.cr[6].lt {
	pc = 0x8250F6D4; continue 'dispatch;
	}
	// 8250F6CC: FC006890  fmr f0, f13
	ctx.f[0].f64 = ctx.f[13].f64;
	// 8250F6D0: 386B0001  addi r3, r11, 1
	ctx.r[3].s64 = ctx.r[11].s64 + 1;
	// 8250F6D4: C1AA0000  lfs f13, 0(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8250F6D8: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 8250F6DC: 4098000C  bge cr6, 0x8250f6e8
	if !ctx.cr[6].lt {
	pc = 0x8250F6E8; continue 'dispatch;
	}
	// 8250F6E0: FC006890  fmr f0, f13
	ctx.f[0].f64 = ctx.f[13].f64;
	// 8250F6E4: 386B0002  addi r3, r11, 2
	ctx.r[3].s64 = ctx.r[11].s64 + 2;
	// 8250F6E8: C1AA0004  lfs f13, 4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8250F6EC: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 8250F6F0: 4098000C  bge cr6, 0x8250f6fc
	if !ctx.cr[6].lt {
	pc = 0x8250F6FC; continue 'dispatch;
	}
	// 8250F6F4: FC006890  fmr f0, f13
	ctx.f[0].f64 = ctx.f[13].f64;
	// 8250F6F8: 386B0003  addi r3, r11, 3
	ctx.r[3].s64 = ctx.r[11].s64 + 3;
	// 8250F6FC: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8250F700: 394A0010  addi r10, r10, 0x10
	ctx.r[10].s64 = ctx.r[10].s64 + 16;
	// 8250F704: 7F0B2000  cmpw cr6, r11, r4
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[4].s32, &mut ctx.xer);
	// 8250F708: 4198FFA4  blt cr6, 0x8250f6ac
	if ctx.cr[6].lt {
	pc = 0x8250F6AC; continue 'dispatch;
	}
	// 8250F70C: D001FFE0  stfs f0, -0x20(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), tmp.u32 ) };
	// 8250F710: 7F0B3800  cmpw cr6, r11, r7
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[7].s32, &mut ctx.xer);
	// 8250F714: 40980034  bge cr6, 0x8250f748
	if !ctx.cr[6].lt {
	pc = 0x8250F748; continue 'dispatch;
	}
	// 8250F718: 556A103A  slwi r10, r11, 2
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8250F71C: 7D4A2A14  add r10, r10, r5
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[5].u64;
	// 8250F720: C1AA0000  lfs f13, 0(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8250F724: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 8250F728: 4098000C  bge cr6, 0x8250f734
	if !ctx.cr[6].lt {
	pc = 0x8250F734; continue 'dispatch;
	}
	// 8250F72C: FC006890  fmr f0, f13
	ctx.f[0].f64 = ctx.f[13].f64;
	// 8250F730: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 8250F734: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8250F738: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 8250F73C: 7F0B3800  cmpw cr6, r11, r7
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[7].s32, &mut ctx.xer);
	// 8250F740: 4198FFE0  blt cr6, 0x8250f720
	if ctx.cr[6].lt {
	pc = 0x8250F720; continue 'dispatch;
	}
	// 8250F744: D001FFE0  stfs f0, -0x20(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), tmp.u32 ) };
	// 8250F748: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 8250F74C: 419A00A8  beq cr6, 0x8250f7f4
	if ctx.cr[6].eq {
	pc = 0x8250F7F4; continue 'dispatch;
	}
	// 8250F750: 81690040  lwz r11, 0x40(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(64 as u32) ) } as u64;
	// 8250F754: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 8250F758: 38A1FFE0  addi r5, r1, -0x20
	ctx.r[5].s64 = ctx.r[1].s64 + -32;
	// 8250F75C: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 8250F760: 5467103A  slwi r7, r3, 2
	ctx.r[7].u32 = ctx.r[3].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 8250F764: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8250F768: 38800010  li r4, 0x10
	ctx.r[4].s64 = 16;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250F808(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8250F808 size=92
    let mut pc: u32 = 0x8250F808;
    'dispatch: loop {
        match pc {
            0x8250F808 => {
    //   block [0x8250F808..0x8250F864)
	// 8250F808: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8250F80C: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8250F810: 396B3BFC  addi r11, r11, 0x3bfc
	ctx.r[11].s64 = ctx.r[11].s64 + 15356;
	// 8250F814: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8250F818: 38E00014  li r7, 0x14
	ctx.r[7].s64 = 20;
	// 8250F81C: 7C8A2378  mr r10, r4
	ctx.r[10].u64 = ctx.r[4].u64;
	// 8250F820: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 8250F824: B1230006  sth r9, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 8250F828: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8250F82C: 91030008  stw r8, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[8].u32 ) };
	// 8250F830: 90E3000C  stw r7, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[7].u32 ) };
	// 8250F834: 40990028  ble cr6, 0x8250f85c
	if !ctx.cr[6].gt {
	pc = 0x8250F85C; continue 'dispatch;
	}
	// 8250F838: 39230020  addi r9, r3, 0x20
	ctx.r[9].s64 = ctx.r[3].s64 + 32;
	// 8250F83C: 7CAB2B78  mr r11, r5
	ctx.r[11].u64 = ctx.r[5].u64;
	// 8250F840: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250F868(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8250F868 size=372
    let mut pc: u32 = 0x8250F868;
    'dispatch: loop {
        match pc {
            0x8250F868 => {
    //   block [0x8250F868..0x8250F9DC)
	// 8250F868: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250F86C: 4802583D  bl 0x825350a8
	ctx.lr = 0x8250F870;
	sub_82535080(ctx, base);
	// 8250F870: 39040030  addi r8, r4, 0x30
	ctx.r[8].s64 = ctx.r[4].s64 + 48;
	// 8250F874: EB640000  ld r27, 0(r4)
	ctx.r[27].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
	// 8250F878: 39440010  addi r10, r4, 0x10
	ctx.r[10].s64 = ctx.r[4].s64 + 16;
	// 8250F87C: 83A30010  lwz r29, 0x10(r3)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 8250F880: 39240020  addi r9, r4, 0x20
	ctx.r[9].s64 = ctx.r[4].s64 + 32;
	// 8250F884: E8840008  ld r4, 8(r4)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) };
	// 8250F888: 38E1FF20  addi r7, r1, -0xe0
	ctx.r[7].s64 = ctx.r[1].s64 + -224;
	// 8250F88C: 3BC1FEF0  addi r30, r1, -0x110
	ctx.r[30].s64 = ctx.r[1].s64 + -272;
	// 8250F890: EB080000  ld r24, 0(r8)
	ctx.r[24].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) };
	// 8250F894: 38A1FF10  addi r5, r1, -0xf0
	ctx.r[5].s64 = ctx.r[1].s64 + -240;
	// 8250F898: E9080008  ld r8, 8(r8)
	ctx.r[8].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[8].u32.wrapping_add(8 as u32) ) };
	// 8250F89C: 3BE1FF00  addi r31, r1, -0x100
	ctx.r[31].s64 = ctx.r[1].s64 + -256;
	// 8250F8A0: EB4A0000  ld r26, 0(r10)
	ctx.r[26].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	// 8250F8A4: 397DFFFF  addi r11, r29, -1
	ctx.r[11].s64 = ctx.r[29].s64 + -1;
	// 8250F8A8: FB670000  std r27, 0(r7)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), ctx.r[27].u64 ) };
	// 8250F8AC: 3B81FF30  addi r28, r1, -0xd0
	ctx.r[28].s64 = ctx.r[1].s64 + -208;
	// 8250F8B0: F8870008  std r4, 8(r7)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[7].u32.wrapping_add(8 as u32), ctx.r[4].u64 ) };
	// 8250F8B4: FB1E0000  std r24, 0(r30)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[24].u64 ) };
	// 8250F8B8: F91E0008  std r8, 8(r30)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[8].u64 ) };
	// 8250F8BC: 3901FF20  addi r8, r1, -0xe0
	ctx.r[8].s64 = ctx.r[1].s64 + -224;
	// 8250F8C0: E94A0008  ld r10, 8(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) };
	// 8250F8C4: FB450000  std r26, 0(r5)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[26].u64 ) };
	// 8250F8C8: EB290000  ld r25, 0(r9)
	ctx.r[25].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) };
	// 8250F8CC: E9290008  ld r9, 8(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250F9E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8250F9E0 size=24
    let mut pc: u32 = 0x8250F9E0;
    'dispatch: loop {
        match pc {
            0x8250F9E0 => {
    //   block [0x8250F9E0..0x8250F9F8)
	// 8250F9E0: 7C681B78  mr r8, r3
	ctx.r[8].u64 = ctx.r[3].u64;
	// 8250F9E4: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 8250F9E8: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8250F9EC: 81680010  lwz r11, 0x10(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(16 as u32) ) } as u64;
	// 8250F9F0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8250F9F4: 4C990020  blelr cr6
	if !ctx.cr[6].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250F9F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8250F9F8 size=44
    let mut pc: u32 = 0x8250F9F8;
    'dispatch: loop {
        match pc {
            0x8250F9F8 => {
    //   block [0x8250F9F8..0x8250FA24)
	// 8250F9F8: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 8250F9FC: 39680020  addi r11, r8, 0x20
	ctx.r[11].s64 = ctx.r[8].s64 + 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250FA28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8250FA28 size=424
    let mut pc: u32 = 0x8250FA28;
    'dispatch: loop {
        match pc {
            0x8250FA28 => {
    //   block [0x8250FA28..0x8250FBD0)
	// 8250FA28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250FA2C: 4802567D  bl 0x825350a8
	ctx.lr = 0x8250FA30;
	sub_82535080(ctx, base);
	// 8250FA30: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250FA34: 39600014  li r11, 0x14
	ctx.r[11].s64 = 20;
	// 8250FA38: 7CE83B78  mr r8, r7
	ctx.r[8].u64 = ctx.r[7].u64;
	// 8250FA3C: 7F8B5214  add r28, r11, r10
	ctx.r[28].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8250FA40: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8250FA44: 7C892378  mr r9, r4
	ctx.r[9].u64 = ctx.r[4].u64;
	// 8250FA48: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250FA4C: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8250FA50: 80EB000C  lwz r7, 0xc(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8250FA54: 7F0A3840  cmplw cr6, r10, r7
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[7].u32, &mut ctx.xer);
	// 8250FA58: 40980020  bge cr6, 0x8250fa78
	if !ctx.cr[6].lt {
	pc = 0x8250FA78; continue 'dispatch;
	}
	// 8250FA5C: 3CE08206  lis r7, -0x7dfa
	ctx.r[7].s64 = -2113536000;
	// 8250FA60: 38E74AFC  addi r7, r7, 0x4afc
	ctx.r[7].s64 = ctx.r[7].s64 + 19196;
	// 8250FA64: 90EA0000  stw r7, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 8250FA68: 7CEC42E6  mftb r7, 0x10c
	ctx.r[7].u64 = crate::rt::rdtsc_u64();
	// 8250FA6C: 388A000C  addi r4, r10, 0xc
	ctx.r[4].s64 = ctx.r[10].s64 + 12;
	// 8250FA70: 90EA0004  stw r7, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 8250FA74: 908B0004  stw r4, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 8250FA78: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8250FA7C: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8250FA80: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 8250FA84: 40990118  ble cr6, 0x8250fb9c
	if !ctx.cr[6].gt {
	pc = 0x8250FB9C; continue 'dispatch;
	}
	// 8250FA88: 7FA83050  subf r29, r8, r6
	ctx.r[29].s64 = ctx.r[6].s64 - ctx.r[8].s64;
	// 8250FA8C: 3C80820D  lis r4, -0x7df3
	ctx.r[4].s64 = -2113077248;
	// 8250FA90: 3CC08200  lis r6, -0x7e00
	ctx.r[6].s64 = -2113929216;
	// 8250FA94: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 8250FA98: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8250FA9C: 38FF0010  addi r7, r31, 0x10
	ctx.r[7].s64 = ctx.r[31].s64 + 16;
	// 8250FAA0: C124BFFC  lfs f9, -0x4004(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(-16388 as u32) ) };
	ctx.f[9].f64 = (tmp.f32 as f64);
	// 8250FAA4: C146294C  lfs f10, 0x294c(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(10572 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 8250FAA8: C16A1FF8  lfs f11, 0x1ff8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8184 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 8250FAAC: C10B2068  lfs f8, 0x2068(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8296 as u32) ) };
	ctx.f[8].f64 = (tmp.f32 as f64);
	// 8250FAB0: EB7F0000  ld r27, 0(r31)
	ctx.r[27].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) };
	// 8250FAB4: 3961FF90  addi r11, r1, -0x70
	ctx.r[11].s64 = ctx.r[1].s64 + -112;
	// 8250FAB8: EB5F0008  ld r26, 8(r31)
	ctx.r[26].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) };
	// 8250FABC: 3941FFA0  addi r10, r1, -0x60
	ctx.r[10].s64 = ctx.r[1].s64 + -96;
	// 8250FAC0: EB270000  ld r25, 0(r7)
	ctx.r[25].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250FBD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8250FBD0 size=124
    let mut pc: u32 = 0x8250FBD0;
    'dispatch: loop {
        match pc {
            0x8250FBD0 => {
    //   block [0x8250FBD0..0x8250FC4C)
	// 8250FBD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250FBD4: 480254E1  bl 0x825350b4
	ctx.lr = 0x8250FBD8;
	sub_82535080(ctx, base);
	// 8250FBD8: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250FBDC: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8250FBE0: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8250FBE4: 3B9F0020  addi r28, r31, 0x20
	ctx.r[28].s64 = ctx.r[31].s64 + 32;
	// 8250FBE8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8250FBEC: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 8250FBF0: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 8250FBF4: 80BF0010  lwz r5, 0x10(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8250FBF8: 38C10070  addi r6, r1, 0x70
	ctx.r[6].s64 = ctx.r[1].s64 + 112;
	// 8250FBFC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8250FC00: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8250FC04: 4BFFFE25  bl 0x8250fa28
	ctx.lr = 0x8250FC08;
	sub_8250FA28(ctx, base);
	// 8250FC08: 7C671B78  mr r7, r3
	ctx.r[7].u64 = ctx.r[3].u64;
	// 8250FC0C: 7F69DB78  mr r9, r27
	ctx.r[9].u64 = ctx.r[27].u64;
	// 8250FC10: 809F0010  lwz r4, 0x10(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8250FC14: 7FA8EB78  mr r8, r29
	ctx.r[8].u64 = ctx.r[29].u64;
	// 8250FC18: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 8250FC1C: 38A10070  addi r5, r1, 0x70
	ctx.r[5].s64 = ctx.r[1].s64 + 112;
	// 8250FC20: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8250FC24: 4BFFFA5D  bl 0x8250f680
	ctx.lr = 0x8250FC28;
	sub_8250F680(ctx, base);
	// 8250FC28: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8250FC2C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8250FC30: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8250FC34: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 8250FC38: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8250FC3C: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 8250FC40: 997E0000  stb r11, 0(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 8250FC44: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 8250FC48: 480254BC  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250FC50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8250FC50 size=320
    let mut pc: u32 = 0x8250FC50;
    'dispatch: loop {
        match pc {
            0x8250FC50 => {
    //   block [0x8250FC50..0x8250FD90)
	// 8250FC50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250FC54: 48025441  bl 0x82535094
	ctx.lr = 0x8250FC58;
	sub_82535080(ctx, base);
	// 8250FC58: DBE1FF88  stfd f31, -0x78(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-120 as u32), ctx.f[31].u64 ) };
	// 8250FC5C: 9421FEA0  stwu r1, -0x160(r1)
	ea = ctx.r[1].u32.wrapping_add(-352 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250FC60: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 8250FC64: 7C982378  mr r24, r4
	ctx.r[24].u64 = ctx.r[4].u64;
	// 8250FC68: 3AF90020  addi r23, r25, 0x20
	ctx.r[23].s64 = ctx.r[25].s64 + 32;
	// 8250FC6C: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 8250FC70: 38E100A0  addi r7, r1, 0xa0
	ctx.r[7].s64 = ctx.r[1].s64 + 160;
	// 8250FC74: 38C100C0  addi r6, r1, 0xc0
	ctx.r[6].s64 = ctx.r[1].s64 + 192;
	// 8250FC78: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 8250FC7C: 7EE4BB78  mr r4, r23
	ctx.r[4].u64 = ctx.r[23].u64;
	// 8250FC80: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 8250FC84: 80B90010  lwz r5, 0x10(r25)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(16 as u32) ) } as u64;
	// 8250FC88: 4BFFFDA1  bl 0x8250fa28
	ctx.lr = 0x8250FC8C;
	sub_8250FA28(ctx, base);
	// 8250FC8C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8250FC90: 3AA0FFFF  li r21, -1
	ctx.r[21].s64 = -1;
	// 8250FC94: 3AC00000  li r22, 0
	ctx.r[22].s64 = 0;
	// 8250FC98: 3BA0FFFF  li r29, -1
	ctx.r[29].s64 = -1;
	// 8250FC9C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8250FCA0: C3EB1850  lfs f31, 0x1850(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(6224 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 8250FCA4: D3E10060  stfs f31, 0x60(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), tmp.u32 ) };
	// 8250FCA8: 92A10064  stw r21, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[21].u32 ) };
	// 8250FCAC: 92C10090  stw r22, 0x90(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(144 as u32), ctx.r[22].u32 ) };
	// 8250FCB0: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 8250FCB4: 93A10070  stw r29, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[29].u32 ) };
	// 8250FCB8: 419A00CC  beq cr6, 0x8250fd84
	if ctx.cr[6].eq {
	pc = 0x8250FD84; continue 'dispatch;
	}
	// 8250FCBC: 57FE103A  slwi r30, r31, 2
	ctx.r[30].u32 = ctx.r[31].u32.wrapping_shl(2);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 8250FCC0: 3A600010  li r19, 0x10
	ctx.r[19].s64 = 16;
	// 8250FCC4: 3A800020  li r20, 0x20
	ctx.r[20].s64 = 32;
	// 8250FCC8: 39210050  addi r9, r1, 0x50
	ctx.r[9].s64 = ctx.r[1].s64 + 80;
	// 8250FCCC: 80990010  lwz r4, 0x10(r25)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(16 as u32) ) } as u64;
	// 8250FCD0: 7F08C378  mr r8, r24
	ctx.r[8].u64 = ctx.r[24].u64;
	// 8250FCD4: D3E10060  stfs f31, 0x60(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), tmp.u32 ) };
	// 8250FCD8: 7FE7FB78  mr r7, r31
	ctx.r[7].u64 = ctx.r[31].u64;
	// 8250FCDC: 92A10064  stw r21, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[21].u32 ) };
	// 8250FCE0: 38C100A0  addi r6, r1, 0xa0
	ctx.r[6].s64 = ctx.r[1].s64 + 160;
	// 8250FCE4: 92C10090  stw r22, 0x90(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(144 as u32), ctx.r[22].u32 ) };
	// 8250FCE8: 38A100C0  addi r5, r1, 0xc0
	ctx.r[5].s64 = ctx.r[1].s64 + 192;
	// 8250FCEC: 93A10070  stw r29, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[29].u32 ) };
	// 8250FCF0: 7EE3BB78  mr r3, r23
	ctx.r[3].u64 = ctx.r[23].u64;
	// 8250FCF4: 4BFFF98D  bl 0x8250f680
	ctx.lr = 0x8250FCF8;
	sub_8250F680(ctx, base);
	// 8250FCF8: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 8250FCFC: 817C0008  lwz r11, 8(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250FD00: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 8250FD04: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8250FD08: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8250FD0C: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250FD90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8250FD90 size=20
    let mut pc: u32 = 0x8250FD90;
    'dispatch: loop {
        match pc {
            0x8250FD90 => {
    //   block [0x8250FD90..0x8250FDA4)
	// 8250FD90: 80630018  lwz r3, 0x18(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 8250FD94: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250FD98: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 8250FD9C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8250FDA0: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250FDA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8250FDA8 size=144
    let mut pc: u32 = 0x8250FDA8;
    'dispatch: loop {
        match pc {
            0x8250FDA8 => {
    //   block [0x8250FDA8..0x8250FE38)
	// 8250FDA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250FDAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8250FDB0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8250FDB4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8250FDB8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250FDBC: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8250FDC0: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8250FDC4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8250FDC8: 388B4B20  addi r4, r11, 0x4b20
	ctx.r[4].s64 = ctx.r[11].s64 + 19232;
	// 8250FDCC: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 8250FDD0: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250FDD4: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8250FDD8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8250FDDC: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 8250FDE0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8250FDE4: 4E800421  bctrl
	ctx.lr = 0x8250FDE8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8250FDE8: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250FDEC: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8250FDF0: 80DE0018  lwz r6, 0x18(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 8250FDF4: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8250FDF8: 388B485C  addi r4, r11, 0x485c
	ctx.r[4].s64 = ctx.r[11].s64 + 18524;
	// 8250FDFC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8250FE00: 816A000C  lwz r11, 0xc(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 8250FE04: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8250FE08: 4E800421  bctrl
	ctx.lr = 0x8250FE0C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8250FE0C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250FE10: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8250FE14: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 8250FE18: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8250FE1C: 4E800421  bctrl
	ctx.lr = 0x8250FE20;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8250FE20: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8250FE24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8250FE28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8250FE2C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8250FE30: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8250FE34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250FE38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8250FE38 size=8
    let mut pc: u32 = 0x8250FE38;
    'dispatch: loop {
        match pc {
            0x8250FE38 => {
    //   block [0x8250FE38..0x8250FE40)
	// 8250FE38: 38630014  addi r3, r3, 0x14
	ctx.r[3].s64 = ctx.r[3].s64 + 20;
	// 8250FE3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250FE40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8250FE40 size=52
    let mut pc: u32 = 0x8250FE40;
    'dispatch: loop {
        match pc {
            0x8250FE40 => {
    //   block [0x8250FE40..0x8250FE74)
	// 8250FE40: 39630020  addi r11, r3, 0x20
	ctx.r[11].s64 = ctx.r[3].s64 + 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250FE78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8250FE78 size=136
    let mut pc: u32 = 0x8250FE78;
    'dispatch: loop {
        match pc {
            0x8250FE78 => {
    //   block [0x8250FE78..0x8250FF00)
	// 8250FE78: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8250FE7C: C0040010  lfs f0, 0x10(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(16 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8250FE80: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 8250FE84: D0030010  stfs f0, 0x10(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 8250FE88: 396B3D1C  addi r11, r11, 0x3d1c
	ctx.r[11].s64 = ctx.r[11].s64 + 15644;
	// 8250FE8C: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8250FE90: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8250FE94: 38E0000D  li r7, 0xd
	ctx.r[7].s64 = 13;
	// 8250FE98: 394A2A68  addi r10, r10, 0x2a68
	ctx.r[10].s64 = ctx.r[10].s64 + 10856;
	// 8250FE9C: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8250FEA0: B1230006  sth r9, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 8250FEA4: 91030008  stw r8, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[8].u32 ) };
	// 8250FEA8: 90E3000C  stw r7, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[7].u32 ) };
	// 8250FEAC: 91430014  stw r10, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 8250FEB0: 90830018  stw r4, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[4].u32 ) };
	// 8250FEB4: A1640004  lhz r11, 4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 8250FEB8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8250FEBC: 419A0010  beq cr6, 0x8250fecc
	if ctx.cr[6].eq {
	pc = 0x8250FECC; continue 'dispatch;
	}
	// 8250FEC0: A1640006  lhz r11, 6(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(6 as u32) ) } as u64;
	// 8250FEC4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8250FEC8: B1640006  sth r11, 6(r4)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[4].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 8250FECC: 39630020  addi r11, r3, 0x20
	ctx.r[11].s64 = ctx.r[3].s64 + 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250FF00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8250FF00 size=108
    let mut pc: u32 = 0x8250FF00;
    'dispatch: loop {
        match pc {
            0x8250FF00 => {
    //   block [0x8250FF00..0x8250FF6C)
	// 8250FF00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250FF04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8250FF08: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8250FF0C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8250FF10: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 8250FF14: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250FF18: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8250FF1C: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 8250FF20: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8250FF24: 38BF0020  addi r5, r31, 0x20
	ctx.r[5].s64 = ctx.r[31].s64 + 32;
	// 8250FF28: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 8250FF2C: 4809916D  bl 0x825a9098
	ctx.lr = 0x8250FF30;
	sub_825A9098(ctx, base);
	// 8250FF30: 807F0018  lwz r3, 0x18(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 8250FF34: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 8250FF38: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 8250FF3C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8250FF40: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250FF44: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 8250FF48: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8250FF4C: 4E800421  bctrl
	ctx.lr = 0x8250FF50;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8250FF50: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 8250FF54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8250FF58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8250FF5C: CBE1FFE0  lfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 8250FF60: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8250FF64: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8250FF68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250FF70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8250FF70 size=216
    let mut pc: u32 = 0x8250FF70;
    'dispatch: loop {
        match pc {
            0x8250FF70 => {
    //   block [0x8250FF70..0x82510048)
	// 8250FF70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250FF74: 48025145  bl 0x825350b8
	ctx.lr = 0x8250FF78;
	sub_82535080(ctx, base);
	// 8250FF78: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250FF7C: 3BE30020  addi r31, r3, 0x20
	ctx.r[31].s64 = ctx.r[3].s64 + 32;
	// 8250FF80: 80630018  lwz r3, 0x18(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 8250FF84: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8250FF88: 3B800010  li r28, 0x10
	ctx.r[28].s64 = 16;
	// 8250FF8C: 3BA00020  li r29, 0x20
	ctx.r[29].s64 = 32;
	// 8250FF90: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82510048(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82510048 size=436
    let mut pc: u32 = 0x82510048;
    'dispatch: loop {
        match pc {
            0x82510048 => {
    //   block [0x82510048..0x825101FC)
	// 82510048: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251004C: 48025069  bl 0x825350b4
	ctx.lr = 0x82510050;
	sub_82535080(ctx, base);
	// 82510050: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82510054: 836D0000  lwz r27, 0(r13)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82510058: 3B800014  li r28, 0x14
	ctx.r[28].s64 = 20;
	// 8251005C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82510060: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82510064: 7D7CD82E  lwzx r11, r28, r27
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[28].u32.wrapping_add(ctx.r[27].u32)) } as u64;
	// 82510068: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8251006C: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82510070: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82510074: 40980020  bge cr6, 0x82510094
	if !ctx.cr[6].lt {
	pc = 0x82510094; continue 'dispatch;
	}
	// 82510078: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 8251007C: 39294A54  addi r9, r9, 0x4a54
	ctx.r[9].s64 = ctx.r[9].s64 + 19028;
	// 82510080: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82510084: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82510088: 390A000C  addi r8, r10, 0xc
	ctx.r[8].s64 = ctx.r[10].s64 + 12;
	// 8251008C: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82510090: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82510094: 39410060  addi r10, r1, 0x60
	ctx.r[10].s64 = ctx.r[1].s64 + 96;
	// 82510098: 7CAB2B78  mr r11, r5
	ctx.r[11].u64 = ctx.r[5].u64;
	// 8251009C: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 825100A0: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 825100A4: E92B0000  ld r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 825100A8: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 825100AC: F92A0000  std r9, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u64 ) };
	// 825100B0: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 825100B4: 4200FFF0  bdnz 0x825100a4
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x825100A4; continue 'dispatch;
	}
	// 825100B8: 3BE40020  addi r31, r4, 0x20
	ctx.r[31].s64 = ctx.r[4].s64 + 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82510200(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82510200 size=384
    let mut pc: u32 = 0x82510200;
    'dispatch: loop {
        match pc {
            0x82510200 => {
    //   block [0x82510200..0x82510380)
	// 82510200: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82510204: 48024EB1  bl 0x825350b4
	ctx.lr = 0x82510208;
	sub_82535080(ctx, base);
	// 82510208: 9421FF00  stwu r1, -0x100(r1)
	ea = ctx.r[1].u32.wrapping_add(-256 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251020C: 838D0000  lwz r28, 0(r13)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82510210: 3BA00014  li r29, 0x14
	ctx.r[29].s64 = 20;
	// 82510214: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82510218: 7C8A2378  mr r10, r4
	ctx.r[10].u64 = ctx.r[4].u64;
	// 8251021C: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82510220: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 82510224: 7D7DE02E  lwzx r11, r29, r28
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 82510228: 812B0004  lwz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8251022C: 810B000C  lwz r8, 0xc(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82510230: 7F094040  cmplw cr6, r9, r8
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82510234: 40980020  bge cr6, 0x82510254
	if !ctx.cr[6].lt {
	pc = 0x82510254; continue 'dispatch;
	}
	// 82510238: 3D008206  lis r8, -0x7dfa
	ctx.r[8].s64 = -2113536000;
	// 8251023C: 39084B30  addi r8, r8, 0x4b30
	ctx.r[8].s64 = ctx.r[8].s64 + 19248;
	// 82510240: 91090000  stw r8, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82510244: 7D0C42E6  mftb r8, 0x10c
	ctx.r[8].u64 = crate::rt::rdtsc_u64();
	// 82510248: 38E9000C  addi r7, r9, 0xc
	ctx.r[7].s64 = ctx.r[9].s64 + 12;
	// 8251024C: 91090004  stw r8, 4(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82510250: 90EB0004  stw r7, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82510254: 39210060  addi r9, r1, 0x60
	ctx.r[9].s64 = ctx.r[1].s64 + 96;
	// 82510258: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 8251025C: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 82510260: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	// 82510264: E90B0000  ld r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82510268: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 8251026C: F9090000  std r8, 0(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[8].u64 ) };
	// 82510270: 39290008  addi r9, r9, 8
	ctx.r[9].s64 = ctx.r[9].s64 + 8;
	// 82510274: 4200FFF0  bdnz 0x82510264
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82510264; continue 'dispatch;
	}
	// 82510278: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82510380(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82510380 size=188
    let mut pc: u32 = 0x82510380;
    'dispatch: loop {
        match pc {
            0x82510380 => {
    //   block [0x82510380..0x8251043C)
	// 82510380: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82510384: 48024D35  bl 0x825350b8
	ctx.lr = 0x82510388;
	sub_82535080(ctx, base);
	// 82510388: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251038C: 3BE30020  addi r31, r3, 0x20
	ctx.r[31].s64 = ctx.r[3].s64 + 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82510440(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82510440 size=252
    let mut pc: u32 = 0x82510440;
    'dispatch: loop {
        match pc {
            0x82510440 => {
    //   block [0x82510440..0x8251053C)
	// 82510440: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82510444: 48024C71  bl 0x825350b4
	ctx.lr = 0x82510448;
	sub_82535080(ctx, base);
	// 82510448: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251044C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82510450: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82510454: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82510458: 807F0018  lwz r3, 0x18(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 8251045C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82510460: 816B0034  lwz r11, 0x34(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 82510464: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82510468: 4E800421  bctrl
	ctx.lr = 0x8251046C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8251046C: 397F0020  addi r11, r31, 0x20
	ctx.r[11].s64 = ctx.r[31].s64 + 32;
	// 82510470: 387EFFFF  addi r3, r30, -1
	ctx.r[3].s64 = ctx.r[30].s64 + -1;
	// 82510474: 38CB0010  addi r6, r11, 0x10
	ctx.r[6].s64 = ctx.r[11].s64 + 16;
	// 82510478: 38AB0020  addi r5, r11, 0x20
	ctx.r[5].s64 = ctx.r[11].s64 + 32;
	// 8251047C: 388B0030  addi r4, r11, 0x30
	ctx.r[4].s64 = ctx.r[11].s64 + 48;
	// 82510480: EBCB0000  ld r30, 0(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82510484: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 82510488: E96B0008  ld r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	// 8251048C: 547F2036  slwi r31, r3, 4
	ctx.r[31].u32 = ctx.r[3].u32.wrapping_shl(4);
	ctx.r[31].u64 = ctx.r[31].u32 as u64;
	// 82510490: 39210060  addi r9, r1, 0x60
	ctx.r[9].s64 = ctx.r[1].s64 + 96;
	// 82510494: 7FFFEA14  add r31, r31, r29
	ctx.r[31].u64 = ctx.r[31].u64 + ctx.r[29].u64;
	// 82510498: EBA60000  ld r29, 0(r6)
	ctx.r[29].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) };
	// 8251049C: E8C60008  ld r6, 8(r6)
	ctx.r[6].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[6].u32.wrapping_add(8 as u32) ) };
	// 825104A0: 39010070  addi r8, r1, 0x70
	ctx.r[8].s64 = ctx.r[1].s64 + 112;
	// 825104A4: FBCA0000  std r30, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[30].u64 ) };
	// 825104A8: 38E10080  addi r7, r1, 0x80
	ctx.r[7].s64 = ctx.r[1].s64 + 128;
	// 825104AC: F96A0008  std r11, 8(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 825104B0: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 825104B4: EB850000  ld r28, 0(r5)
	ctx.r[28].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) };
	// 825104B8: FBA90000  std r29, 0(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[29].u64 ) };
	// 825104BC: F8C90008  std r6, 8(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), ctx.r[6].u64 ) };
	// 825104C0: E8A50008  ld r5, 8(r5)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[5].u32.wrapping_add(8 as u32) ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82510540(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82510540 size=140
    let mut pc: u32 = 0x82510540;
    'dispatch: loop {
        match pc {
            0x82510540 => {
    //   block [0x82510540..0x825105CC)
	// 82510540: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82510544: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82510548: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8251054C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82510550: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82510554: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82510558: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8251055C: 807E0018  lwz r3, 0x18(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 82510560: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82510564: 816B003C  lwz r11, 0x3c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 82510568: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8251056C: 4E800421  bctrl
	ctx.lr = 0x82510570;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82510570: 397E0020  addi r11, r30, 0x20
	ctx.r[11].s64 = ctx.r[30].s64 + 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825105D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825105D0 size=148
    let mut pc: u32 = 0x825105D0;
    'dispatch: loop {
        match pc {
            0x825105D0 => {
    //   block [0x825105D0..0x82510664)
	// 825105D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825105D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825105D8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825105DC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825105E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825105E4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825105E8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825105EC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 825105F0: 807F0018  lwz r3, 0x18(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 825105F4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825105F8: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 825105FC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82510600: 4E800421  bctrl
	ctx.lr = 0x82510604;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82510604: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 82510608: 397F0020  addi r11, r31, 0x20
	ctx.r[11].s64 = ctx.r[31].s64 + 32;
	// 8251060C: 39200020  li r9, 0x20
	ctx.r[9].s64 = 32;
	// 82510610: 39000030  li r8, 0x30
	ctx.r[8].s64 = 48;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82510668(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82510668 size=280
    let mut pc: u32 = 0x82510668;
    'dispatch: loop {
        match pc {
            0x82510668 => {
    //   block [0x82510668..0x82510780)
	// 82510668: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251066C: 48024A3D  bl 0x825350a8
	ctx.lr = 0x82510670;
	sub_82535080(ctx, base);
	// 82510670: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82510674: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82510678: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8251067C: 807F0018  lwz r3, 0x18(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82510680: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82510684: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 82510688: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8251068C: 4E800421  bctrl
	ctx.lr = 0x82510690;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82510690: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82510694: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82510698: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 8251069C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825106A0: 816A0024  lwz r11, 0x24(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(36 as u32) ) } as u64;
	// 825106A4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825106A8: 4E800421  bctrl
	ctx.lr = 0x825106AC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825106AC: 397F0020  addi r11, r31, 0x20
	ctx.r[11].s64 = ctx.r[31].s64 + 32;
	// 825106B0: 3943FFFF  addi r10, r3, -1
	ctx.r[10].s64 = ctx.r[3].s64 + -1;
	// 825106B4: 39210050  addi r9, r1, 0x50
	ctx.r[9].s64 = ctx.r[1].s64 + 80;
	// 825106B8: 38AB0010  addi r5, r11, 0x10
	ctx.r[5].s64 = ctx.r[11].s64 + 16;
	// 825106BC: 39010060  addi r8, r1, 0x60
	ctx.r[8].s64 = ctx.r[1].s64 + 96;
	// 825106C0: E86B0000  ld r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 825106C4: 388B0020  addi r4, r11, 0x20
	ctx.r[4].s64 = ctx.r[11].s64 + 32;
	// 825106C8: EB0B0008  ld r24, 8(r11)
	ctx.r[24].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	// 825106CC: 38E10070  addi r7, r1, 0x70
	ctx.r[7].s64 = ctx.r[1].s64 + 112;
	// 825106D0: 3BEB0030  addi r31, r11, 0x30
	ctx.r[31].s64 = ctx.r[11].s64 + 48;
	// 825106D4: EB650000  ld r27, 0(r5)
	ctx.r[27].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) };
	// 825106D8: 38C10080  addi r6, r1, 0x80
	ctx.r[6].s64 = ctx.r[1].s64 + 128;
	// 825106DC: E8A50008  ld r5, 8(r5)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[5].u32.wrapping_add(8 as u32) ) };
	// 825106E0: 554B2036  slwi r11, r10, 4
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 825106E4: F8690000  std r3, 0(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[3].u64 ) };
	// 825106E8: 7F9EE850  subf r28, r30, r29
	ctx.r[28].s64 = ctx.r[29].s64 - ctx.r[30].s64;
	// 825106EC: FB090008  std r24, 8(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), ctx.r[24].u64 ) };
	// 825106F0: 39210050  addi r9, r1, 0x50
	ctx.r[9].s64 = ctx.r[1].s64 + 80;
	// 825106F4: EB440000  ld r26, 0(r4)
	ctx.r[26].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
	// 825106F8: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 825106FC: FB680000  std r27, 0(r8)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[27].u64 ) };
	// 82510700: F8A80008  std r5, 8(r8)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[5].u64 ) };
	// 82510704: E8840008  ld r4, 8(r4)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82510780(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82510780 size=4
    let mut pc: u32 = 0x82510780;
    'dispatch: loop {
        match pc {
            0x82510780 => {
    //   block [0x82510780..0x82510784)
	// 82510780: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82510788(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82510788 size=8
    let mut pc: u32 = 0x82510788;
    'dispatch: loop {
        match pc {
            0x82510788 => {
    //   block [0x82510788..0x82510790)
	// 82510788: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 8251078C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82510790(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82510790 size=16
    let mut pc: u32 = 0x82510790;
    'dispatch: loop {
        match pc {
            0x82510790 => {
    //   block [0x82510790..0x825107A0)
	// 82510790: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82510794: 396B6DD0  addi r11, r11, 0x6dd0
	ctx.r[11].s64 = ctx.r[11].s64 + 28112;
	// 82510798: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8251079C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825107A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x825107A0 size=516
    let mut pc: u32 = 0x825107A0;
    'dispatch: loop {
        match pc {
            0x825107A0 => {
    //   block [0x825107A0..0x825109A4)
	// 825107A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825107A4: 48024905  bl 0x825350a8
	ctx.lr = 0x825107A8;
	sub_82535080(ctx, base);
	// 825107A8: 3981FFB8  addi r12, r1, -0x48
	ctx.r[12].s64 = ctx.r[1].s64 + -72;
	// 825107AC: 48025839  bl 0x82535fe4
	ctx.lr = 0x825107B0;
	sub_82535FB0(ctx, base);
	// 825107B0: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825107B4: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 825107B8: FFE00890  fmr f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].f64 = ctx.f[1].f64;
	// 825107BC: 7D1E4378  mr r30, r8
	ctx.r[30].u64 = ctx.r[8].u64;
	// 825107C0: FFC01090  fmr f30, f2
	ctx.f[30].f64 = ctx.f[2].f64;
	// 825107C4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825107C8: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 825107CC: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 825107D0: C36B1850  lfs f27, 0x1850(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(6224 as u32) ) };
	ctx.f[27].f64 = (tmp.f32 as f64);
	// 825107D4: 7D3B4B78  mr r27, r9
	ctx.r[27].u64 = ctx.r[9].u64;
	// 825107D8: D37E0000  stfs f27, 0(r30)
	tmp.f32 = (ctx.f[27].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 825107DC: 7D5A5378  mr r26, r10
	ctx.r[26].u64 = ctx.r[10].u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825109A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x825109A8 size=440
    let mut pc: u32 = 0x825109A8;
    'dispatch: loop {
        match pc {
            0x825109A8 => {
    //   block [0x825109A8..0x82510B60)
	// 825109A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825109AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825109B0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825109B4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825109B8: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 825109BC: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825109C0: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 825109C4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825109C8: 394B3F04  addi r10, r11, 0x3f04
	ctx.r[10].s64 = ctx.r[11].s64 + 16132;
	// 825109CC: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 825109D0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825109D4: 38E0000E  li r7, 0xe
	ctx.r[7].s64 = 14;
	// 825109D8: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 825109DC: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 825109E0: B13F0006  sth r9, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 825109E4: 911F0008  stw r8, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[8].u32 ) };
	// 825109E8: 90FF000C  stw r7, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[7].u32 ) };
	// 825109EC: 81440010  lwz r10, 0x10(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(16 as u32) ) } as u64;
	// 825109F0: C3EB1FF8  lfs f31, 0x1ff8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 825109F4: D3E1006C  stfs f31, 0x6c(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), tmp.u32 ) };
	// 825109F8: 397F0050  addi r11, r31, 0x50
	ctx.r[11].s64 = ctx.r[31].s64 + 80;
	// 825109FC: 915F0010  stw r10, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 82510A00: 81440014  lwz r10, 0x14(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(20 as u32) ) } as u64;
	// 82510A04: 915F0014  stw r10, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 82510A08: 89440020  lbz r10, 0x20(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(32 as u32) ) } as u64;
	// 82510A0C: 995F001C  stb r10, 0x1c(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[10].u8 ) };
	// 82510A10: C1A40018  lfs f13, 0x18(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(24 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82510A14: 81240014  lwz r9, 0x14(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(20 as u32) ) } as u64;
	// 82510A18: C004001C  lfs f0, 0x1c(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(28 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82510A1C: 81440010  lwz r10, 0x10(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(16 as u32) ) } as u64;
	// 82510A20: EC006828  fsubs f0, f0, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 - ctx.f[13].f64) as f32) as f64);
	// 82510A24: 7D2907B4  extsw r9, r9
	ctx.r[9].s64 = ctx.r[9].s32 as i64;
	// 82510A28: D0010064  stfs f0, 0x64(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), tmp.u32 ) };
	// 82510A2C: 7D4A07B4  extsw r10, r10
	ctx.r[10].s64 = ctx.r[10].s32 as i64;
	// 82510A30: F9210058  std r9, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[9].u64 ) };
	// 82510A34: F9410050  std r10, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u64 ) };
	// 82510A38: C8010050  lfd f0, 0x50(r1)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82510A3C: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 82510A40: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82510A44: FD800018  frsp f12, f0
	ctx.f[12].f64 = (ctx.f[0].f64 as f32) as f64;
	// 82510A48: C00A1850  lfs f0, 0x1850(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(6224 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82510A4C: ED8C0028  fsubs f12, f12, f0
	ctx.f[12].f64 = (((ctx.f[12].f64 - ctx.f[0].f64) as f32) as f64);
	// 82510A50: D1810060  stfs f12, 0x60(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), tmp.u32 ) };
	// 82510A54: C9A10058  lfd f13, 0x58(r1)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 82510A58: FDA06E9C  fcfid f13, f13
	ctx.f[13].f64 = (ctx.f[13].s64 as f64);
	// 82510A5C: FDA06818  frsp f13, f13
	ctx.f[13].f64 = (ctx.f[13].f64 as f32) as f64;
	// 82510A60: EDAD0028  fsubs f13, f13, f0
	ctx.f[13].f64 = (((ctx.f[13].f64 - ctx.f[0].f64) as f32) as f64);
	// 82510A64: D1A10068  stfs f13, 0x68(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), tmp.u32 ) };
	// 82510A68: 39410060  addi r10, r1, 0x60
	ctx.r[10].s64 = ctx.r[1].s64 + 96;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82510B60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82510B60 size=516
    let mut pc: u32 = 0x82510B60;
    'dispatch: loop {
        match pc {
            0x82510B60 => {
    //   block [0x82510B60..0x82510D64)
	// 82510B60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82510B64: 48024551  bl 0x825350b4
	ctx.lr = 0x82510B68;
	sub_82535080(ctx, base);
	// 82510B68: 3981FFD0  addi r12, r1, -0x30
	ctx.r[12].s64 = ctx.r[1].s64 + -48;
	// 82510B6C: 4802547D  bl 0x82535fe8
	ctx.lr = 0x82510B70;
	sub_82535FB0(ctx, base);
	// 82510B70: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82510B74: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82510B78: FF800890  fmr f28, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[28].f64 = ctx.f[1].f64;
	// 82510B7C: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82510B80: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 82510B84: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82510B88: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82510B8C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82510B90: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 82510B94: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82510B98: 4E800421  bctrl
	ctx.lr = 0x82510B9C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82510B9C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 82510BA0: FFE00890  fmr f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82510BA4: C1BF0054  lfs f13, 0x54(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82510BA8: C00B1FF8  lfs f0, 0x1ff8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82510BAC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 82510BB0: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 82510BB4: C3ABBFFC  lfs f29, -0x4004(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16388 as u32) ) };
	ctx.f[29].f64 = (tmp.f32 as f64);
	// 82510BB8: FFC0F890  fmr f30, f31
	ctx.f[30].f64 = ctx.f[31].f64;
	// 82510BBC: 409800A0  bge cr6, 0x82510c5c
	if !ctx.cr[6].lt {
	pc = 0x82510C5C; continue 'dispatch;
	}
	// 82510BC0: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82510BC4: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82510BC8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82510BCC: 40990060  ble cr6, 0x82510c2c
	if !ctx.cr[6].gt {
	pc = 0x82510C2C; continue 'dispatch;
	}
	// 82510BD0: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82510BD4: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82510BD8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82510BDC: 40990040  ble cr6, 0x82510c1c
	if !ctx.cr[6].gt {
	pc = 0x82510C1C; continue 'dispatch;
	}
	// 82510BE0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82510BE4: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82510BE8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82510BEC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82510BF0: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 82510BF4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82510BF8: 4E800421  bctrl
	ctx.lr = 0x82510BFC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82510BFC: EC1F0828  fsubs f0, f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = (((ctx.f[31].f64 - ctx.f[1].f64) as f32) as f64);
	// 82510C00: EDBE0828  fsubs f13, f30, f1
	ctx.f[13].f64 = (((ctx.f[30].f64 - ctx.f[1].f64) as f32) as f64);
	// 82510C04: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82510C08: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82510C0C: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82510C10: FFE0F86E  fsel f31, f0, f1, f31
	ctx.f[31].f64 = if ctx.f[0].f64 >= 0.0 { ctx.f[1].f64 } else { ctx.f[31].f64 };
	// 82510C14: FFCD0FAE  fsel f30, f13, f30, f1
	ctx.f[30].f64 = if ctx.f[13].f64 >= 0.0 { ctx.f[30].f64 } else { ctx.f[1].f64 };
	// 82510C18: 4198FFC8  blt cr6, 0x82510be0
	if ctx.cr[6].lt {
	pc = 0x82510BE0; continue 'dispatch;
	}
	// 82510C1C: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82510C20: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82510C24: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82510C28: 4198FFA8  blt cr6, 0x82510bd0
	if ctx.cr[6].lt {
	pc = 0x82510BD0; continue 'dispatch;
	}
	// 82510C2C: C1BF0024  lfs f13, 0x24(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82510C30: EC0D07F2  fmuls f0, f13, f31
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[31].f64) as f32) as f64);
	// 82510C34: EDAD07B2  fmuls f13, f13, f30
	ctx.f[13].f64 = (((ctx.f[13].f64 * ctx.f[30].f64) as f32) as f64);
	// 82510C38: FD600090  fmr f11, f0
	ctx.f[11].f64 = ctx.f[0].f64;
	// 82510C3C: ED806828  fsubs f12, f0, f13
	ctx.f[12].f64 = (((ctx.f[0].f64 - ctx.f[13].f64) as f32) as f64);
	// 82510C40: FC0C036E  fsel f0, f12, f13, f0
	ctx.f[0].f64 = if ctx.f[12].f64 >= 0.0 { ctx.f[13].f64 } else { ctx.f[0].f64 };
	// 82510C44: FDAC6AEE  fsel f13, f12, f11, f13
	ctx.f[13].f64 = if ctx.f[12].f64 >= 0.0 { ctx.f[11].f64 } else { ctx.f[13].f64 };
	// 82510C48: ED8D002A  fadds f12, f13, f0
	ctx.f[12].f64 = ((ctx.f[13].f64 + ctx.f[0].f64) as f32) as f64;
	// 82510C4C: EC0D0028  fsubs f0, f13, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 - ctx.f[0].f64) as f32) as f64);
	// 82510C50: EDAC0772  fmuls f13, f12, f29
	ctx.f[13].f64 = (((ctx.f[12].f64 * ctx.f[29].f64) as f32) as f64);
	// 82510C54: D1BF0018  stfs f13, 0x18(r31)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 82510C58: D01F0054  stfs f0, 0x54(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82510C5C: 39210050  addi r9, r1, 0x50
	ctx.r[9].s64 = ctx.r[1].s64 + 80;
	// 82510C60: D3A10050  stfs f29, 0x50(r1)
	tmp.f32 = (ctx.f[29].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 82510C64: 38C00050  li r6, 0x50
	ctx.r[6].s64 = 80;
	// 82510C68: C01F0018  lfs f0, 0x18(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82510C6C: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82510D68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82510D68 size=4216
    let mut pc: u32 = 0x82510D68;
    'dispatch: loop {
        match pc {
            0x82510D68 => {
    //   block [0x82510D68..0x82511DE0)
	// 82510D68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82510D6C: 48024315  bl 0x82535080
	ctx.lr = 0x82510D70;
	sub_82535080(ctx, base);
	// 82510D70: 3981FF68  addi r12, r1, -0x98
	ctx.r[12].s64 = ctx.r[1].s64 + -152;
	// 82510D74: 48025251  bl 0x82535fc4
	ctx.lr = 0x82510D78;
	sub_82535FB0(ctx, base);
	// 82510D78: 3980FEE0  li r12, -0x120
	ctx.r[12].s64 = -288;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82511DE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82511DE0 size=124
    let mut pc: u32 = 0x82511DE0;
    'dispatch: loop {
        match pc {
            0x82511DE0 => {
    //   block [0x82511DE0..0x82511E5C)
	// 82511DE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82511DE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82511DE8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82511DEC: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82511DF0: 3D20820D  lis r9, -0x7df3
	ctx.r[9].s64 = -2113077248;
	// 82511DF4: C0060010  lfs f0, 0x10(r6)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(16 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82511DF8: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 82511DFC: 90C1006C  stw r6, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[6].u32 ) };
	// 82511E00: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82511E04: D0010064  stfs f0, 0x64(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), tmp.u32 ) };
	// 82511E08: 394A4B60  addi r10, r10, 0x4b60
	ctx.r[10].s64 = ctx.r[10].s64 + 19296;
	// 82511E0C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82511E10: C0291FF8  lfs f1, 0x1ff8(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8184 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82511E14: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82511E18: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82511E1C: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82511E20: 39010060  addi r8, r1, 0x60
	ctx.r[8].s64 = ctx.r[1].s64 + 96;
	// 82511E24: 91410060  stw r10, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[10].u32 ) };
	// 82511E28: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82511E2C: 99210050  stb r9, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u8 ) };
	// 82511E30: 88CB0000  lbz r6, 0(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82511E34: 99210068  stb r9, 0x68(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[9].u8 ) };
	// 82511E38: 4BFFEF31  bl 0x82510d68
	ctx.lr = 0x82511E3C;
	sub_82510D68(ctx, base);
	// 82511E3C: 89610068  lbz r11, 0x68(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 82511E40: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82511E44: 997F0000  stb r11, 0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82511E48: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82511E4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82511E50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82511E54: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82511E58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82511E60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82511E60 size=84
    let mut pc: u32 = 0x82511E60;
    'dispatch: loop {
        match pc {
            0x82511E60 => {
    //   block [0x82511E60..0x82511EB4)
	// 82511E60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82511E64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82511E68: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82511E6C: 3D20820D  lis r9, -0x7df3
	ctx.r[9].s64 = -2113077248;
	// 82511E70: C0060004  lfs f0, 4(r6)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82511E74: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 82511E78: 90C10060  stw r6, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[6].u32 ) };
	// 82511E7C: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82511E80: D001005C  stfs f0, 0x5c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), tmp.u32 ) };
	// 82511E84: 394A4B6C  addi r10, r10, 0x4b6c
	ctx.r[10].s64 = ctx.r[10].s64 + 19308;
	// 82511E88: 39010058  addi r8, r1, 0x58
	ctx.r[8].s64 = ctx.r[1].s64 + 88;
	// 82511E8C: C0291FF8  lfs f1, 0x1ff8(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8184 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82511E90: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82511E94: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 82511E98: 99210050  stb r9, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u8 ) };
	// 82511E9C: 88CB0000  lbz r6, 0(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82511EA0: 4BFFEEC9  bl 0x82510d68
	ctx.lr = 0x82511EA4;
	sub_82510D68(ctx, base);
	// 82511EA4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82511EA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82511EAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82511EB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82511EB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82511EB8 size=148
    let mut pc: u32 = 0x82511EB8;
    'dispatch: loop {
        match pc {
            0x82511EB8 => {
    //   block [0x82511EB8..0x82511F4C)
	// 82511EB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82511EBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82511EC0: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82511EC4: 7CC83378  mr r8, r6
	ctx.r[8].u64 = ctx.r[6].u64;
	// 82511EC8: 39410060  addi r10, r1, 0x60
	ctx.r[10].s64 = ctx.r[1].s64 + 96;
	// 82511ECC: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82511ED0: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 82511ED4: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82511ED8: E92B0000  ld r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82511EDC: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82511EE0: F92A0000  std r9, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u64 ) };
	// 82511EE4: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 82511EE8: 4200FFF0  bdnz 0x82511ed8
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82511ED8; continue 'dispatch;
	}
	// 82511EEC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 82511EF0: C1830024  lfs f12, 0x24(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(36 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82511EF4: C0040030  lfs f0, 0x30(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(48 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82511EF8: C1AB1FF8  lfs f13, 0x1ff8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82511EFC: FF0C6800  fcmpu cr6, f12, f13
	ctx.cr[6].compare_f64(ctx.f[12].f64, ctx.f[13].f64);
	// 82511F00: 40990008  ble cr6, 0x82511f08
	if !ctx.cr[6].gt {
	pc = 0x82511F08; continue 'dispatch;
	}
	// 82511F04: FC000050  fneg f0, f0
	ctx.f[0].u64 = ctx.f[0].u64 ^ 0x8000_0000_0000_0000u64;
	// 82511F08: C1A10064  lfs f13, 0x64(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82511F0C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82511F10: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82511F14: EDAD002A  fadds f13, f13, f0
	ctx.f[13].f64 = ((ctx.f[13].f64 + ctx.f[0].f64) as f32) as f64;
	// 82511F18: D1A10064  stfs f13, 0x64(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), tmp.u32 ) };
	// 82511F1C: C1A10074  lfs f13, 0x74(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82511F20: C0240034  lfs f1, 0x34(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(52 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82511F24: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82511F28: EC0D002A  fadds f0, f13, f0
	ctx.f[0].f64 = ((ctx.f[13].f64 + ctx.f[0].f64) as f32) as f64;
	// 82511F2C: 99410050  stb r10, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u8 ) };
	// 82511F30: 88CB0000  lbz r6, 0(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82511F34: D0010074  stfs f0, 0x74(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), tmp.u32 ) };
	// 82511F38: 4BFFEE31  bl 0x82510d68
	ctx.lr = 0x82511F3C;
	sub_82510D68(ctx, base);
	// 82511F3C: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82511F40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82511F44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82511F48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82511F50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82511F50 size=16
    let mut pc: u32 = 0x82511F50;
    'dispatch: loop {
        match pc {
            0x82511F50 => {
    //   block [0x82511F50..0x82511F60)
	// 82511F50: C0050010  lfs f0, 0x10(r5)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(16 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82511F54: C1A30004  lfs f13, 4(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82511F58: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 82511F5C: 4C980020  bgelr cr6
	if !ctx.cr[6].lt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82511F60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82511F60 size=76
    let mut pc: u32 = 0x82511F60;
    'dispatch: loop {
        match pc {
            0x82511F60 => {
    //   block [0x82511F60..0x82511FAC)
	// 82511F60: 8163000C  lwz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82511F64: D0030004  stfs f0, 4(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), tmp.u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82511FB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82511FB0 size=168
    let mut pc: u32 = 0x82511FB0;
    'dispatch: loop {
        match pc {
            0x82511FB0 => {
    //   block [0x82511FB0..0x82512058)
	// 82511FB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82511FB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82511FB8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82511FBC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82511FC0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82511FC4: 7CAB2B78  mr r11, r5
	ctx.r[11].u64 = ctx.r[5].u64;
	// 82511FC8: 81240008  lwz r9, 8(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 82511FCC: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 82511FD0: 80630008  lwz r3, 8(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82511FD4: 39000010  li r8, 0x10
	ctx.r[8].s64 = 16;
	// 82511FD8: 38E00020  li r7, 0x20
	ctx.r[7].s64 = 32;
	// 82511FDC: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82511FE0: E8CB0000  ld r6, 0(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82512058(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82512058 size=72
    let mut pc: u32 = 0x82512058;
    'dispatch: loop {
        match pc {
            0x82512058 => {
    //   block [0x82512058..0x825120A0)
	// 82512058: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251205C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82512060: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82512064: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82512068: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8251206C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82512070: 396B4B54  addi r11, r11, 0x4b54
	ctx.r[11].s64 = ctx.r[11].s64 + 19284;
	// 82512074: 548A07FE  clrlwi r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	// 82512078: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8251207C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82512080: 419A000C  beq cr6, 0x8251208c
	if ctx.cr[6].eq {
	pc = 0x8251208C; continue 'dispatch;
	}
	// 82512084: 48020B35  bl 0x82532bb8
	ctx.lr = 0x82512088;
	sub_82532BB8(ctx, base);
	// 82512088: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8251208C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82512090: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82512094: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82512098: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8251209C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825120A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825120A0 size=76
    let mut pc: u32 = 0x825120A0;
    'dispatch: loop {
        match pc {
            0x825120A0 => {
    //   block [0x825120A0..0x825120EC)
	// 825120A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825120A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825120A8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825120AC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 825120B0: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 825120B4: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 825120B8: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 825120BC: 7CCA3378  mr r10, r6
	ctx.r[10].u64 = ctx.r[6].u64;
	// 825120C0: 39294BAC  addi r9, r9, 0x4bac
	ctx.r[9].s64 = ctx.r[9].s64 + 19372;
	// 825120C4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825120C8: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 825120CC: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 825120D0: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 825120D4: 99610054  stb r11, 0x54(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u8 ) };
	// 825120D8: 48004221  bl 0x825162f8
	ctx.lr = 0x825120DC;
	sub_825162F8(ctx, base);
	// 825120DC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825120E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825120E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825120E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825120F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825120F0 size=232
    let mut pc: u32 = 0x825120F0;
    'dispatch: loop {
        match pc {
            0x825120F0 => {
    //   block [0x825120F0..0x825121D8)
	// 825120F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825120F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825120F8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825120FC: 9421FED0  stwu r1, -0x130(r1)
	ea = ctx.r[1].u32.wrapping_add(-304 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82512100: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82512104: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82512108: 7CE83B78  mr r8, r7
	ctx.r[8].u64 = ctx.r[7].u64;
	// 8251210C: 394100B0  addi r10, r1, 0xb0
	ctx.r[10].s64 = ctx.r[1].s64 + 176;
	// 82512110: 7CAB2B78  mr r11, r5
	ctx.r[11].u64 = ctx.r[5].u64;
	// 82512114: 3920000E  li r9, 0xe
	ctx.r[9].s64 = 14;
	// 82512118: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 8251211C: E92B0000  ld r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82512120: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82512124: F92A0000  std r9, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u64 ) };
	// 82512128: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 8251212C: 4200FFF0  bdnz 0x8251211c
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x8251211C; continue 'dispatch;
	}
	// 82512130: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 82512134: 39650050  addi r11, r5, 0x50
	ctx.r[11].s64 = ctx.r[5].s64 + 80;
	// 82512138: 3929FA60  addi r9, r9, -0x5a0
	ctx.r[9].s64 = ctx.r[9].s64 + -1440;
	// 8251213C: 3CE08201  lis r7, -0x7dff
	ctx.r[7].s64 = -2113863680;
	// 82512140: 39410060  addi r10, r1, 0x60
	ctx.r[10].s64 = ctx.r[1].s64 + 96;
	// 82512144: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825121D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825121D8 size=140
    let mut pc: u32 = 0x825121D8;
    'dispatch: loop {
        match pc {
            0x825121D8 => {
    //   block [0x825121D8..0x82512264)
	// 825121D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825121DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825121E0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825121E4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825121E8: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 825121EC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825121F0: 396B4B88  addi r11, r11, 0x4b88
	ctx.r[11].s64 = ctx.r[11].s64 + 19336;
	// 825121F4: 548A07FE  clrlwi r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	// 825121F8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 825121FC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82512200: 419A004C  beq cr6, 0x8251224c
	if ctx.cr[6].eq {
	pc = 0x8251224C; continue 'dispatch;
	}
	// 82512204: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82512208: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 8251220C: 7D6A582E  lwzx r11, r10, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82512210: 814B0044  lwz r10, 0x44(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(68 as u32) ) } as u64;
	// 82512214: 812B0034  lwz r9, 0x34(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 82512218: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 8251221C: 41980018  blt cr6, 0x82512234
	if ctx.cr[6].lt {
	pc = 0x82512234; continue 'dispatch;
	}
	// 82512220: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82512224: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82512228: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 8251222C: 4BF51CED  bl 0x82463f18
	ctx.lr = 0x82512230;
	sub_82463F18(ctx, base);
	// 82512230: 4800001C  b 0x8251224c
	pc = 0x8251224C; continue 'dispatch;
	// 82512234: 814B0044  lwz r10, 0x44(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(68 as u32) ) } as u64;
	// 82512238: 812B0040  lwz r9, 0x40(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(64 as u32) ) } as u64;
	// 8251223C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82512240: 914B0044  stw r10, 0x44(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(68 as u32), ctx.r[10].u32 ) };
	// 82512244: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82512248: 93EB0040  stw r31, 0x40(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(64 as u32), ctx.r[31].u32 ) };
	// 8251224C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82512250: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82512254: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82512258: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8251225C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82512260: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82512268(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82512268 size=140
    let mut pc: u32 = 0x82512268;
    'dispatch: loop {
        match pc {
            0x82512268 => {
    //   block [0x82512268..0x825122F4)
	// 82512268: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251226C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82512270: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82512274: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82512278: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8251227C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82512280: 396B228C  addi r11, r11, 0x228c
	ctx.r[11].s64 = ctx.r[11].s64 + 8844;
	// 82512284: 548A07FE  clrlwi r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	// 82512288: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8251228C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82512290: 419A004C  beq cr6, 0x825122dc
	if ctx.cr[6].eq {
	pc = 0x825122DC; continue 'dispatch;
	}
	// 82512294: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82512298: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 8251229C: 7D6A582E  lwzx r11, r10, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 825122A0: 814B0044  lwz r10, 0x44(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(68 as u32) ) } as u64;
	// 825122A4: 812B0034  lwz r9, 0x34(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 825122A8: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 825122AC: 41980018  blt cr6, 0x825122c4
	if ctx.cr[6].lt {
	pc = 0x825122C4; continue 'dispatch;
	}
	// 825122B0: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 825122B4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 825122B8: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 825122BC: 4BF51C5D  bl 0x82463f18
	ctx.lr = 0x825122C0;
	sub_82463F18(ctx, base);
	// 825122C0: 4800001C  b 0x825122dc
	pc = 0x825122DC; continue 'dispatch;
	// 825122C4: 814B0044  lwz r10, 0x44(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(68 as u32) ) } as u64;
	// 825122C8: 812B0040  lwz r9, 0x40(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(64 as u32) ) } as u64;
	// 825122CC: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 825122D0: 914B0044  stw r10, 0x44(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(68 as u32), ctx.r[10].u32 ) };
	// 825122D4: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 825122D8: 93EB0040  stw r31, 0x40(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(64 as u32), ctx.r[31].u32 ) };
	// 825122DC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825122E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825122E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825122E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825122EC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825122F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825122F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825122F8 size=140
    let mut pc: u32 = 0x825122F8;
    'dispatch: loop {
        match pc {
            0x825122F8 => {
    //   block [0x825122F8..0x82512384)
	// 825122F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825122FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82512300: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82512304: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82512308: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8251230C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82512310: 396B4B88  addi r11, r11, 0x4b88
	ctx.r[11].s64 = ctx.r[11].s64 + 19336;
	// 82512314: 548A07FE  clrlwi r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	// 82512318: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8251231C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82512320: 419A004C  beq cr6, 0x8251236c
	if ctx.cr[6].eq {
	pc = 0x8251236C; continue 'dispatch;
	}
	// 82512324: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82512328: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 8251232C: 7D6A582E  lwzx r11, r10, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82512330: 814B004C  lwz r10, 0x4c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(76 as u32) ) } as u64;
	// 82512334: 812B0034  lwz r9, 0x34(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 82512338: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 8251233C: 41980018  blt cr6, 0x82512354
	if ctx.cr[6].lt {
	pc = 0x82512354; continue 'dispatch;
	}
	// 82512340: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82512344: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 82512348: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 8251234C: 4BF51BCD  bl 0x82463f18
	ctx.lr = 0x82512350;
	sub_82463F18(ctx, base);
	// 82512350: 4800001C  b 0x8251236c
	pc = 0x8251236C; continue 'dispatch;
	// 82512354: 814B004C  lwz r10, 0x4c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(76 as u32) ) } as u64;
	// 82512358: 812B0048  lwz r9, 0x48(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(72 as u32) ) } as u64;
	// 8251235C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82512360: 914B004C  stw r10, 0x4c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(76 as u32), ctx.r[10].u32 ) };
	// 82512364: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82512368: 93EB0048  stw r31, 0x48(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(72 as u32), ctx.r[31].u32 ) };
	// 8251236C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82512370: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82512374: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82512378: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8251237C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82512380: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82512388(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82512388 size=256
    let mut pc: u32 = 0x82512388;
    'dispatch: loop {
        match pc {
            0x82512388 => {
    //   block [0x82512388..0x82512488)
	// 82512388: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251238C: 48022D31  bl 0x825350bc
	ctx.lr = 0x82512390;
	sub_82535080(ctx, base);
	// 82512390: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82512394: 3D008251  lis r8, -0x7daf
	ctx.r[8].s64 = -2108620800;
	// 82512398: 3D208251  lis r9, -0x7daf
	ctx.r[9].s64 = -2108620800;
	// 8251239C: 3D408251  lis r10, -0x7daf
	ctx.r[10].s64 = -2108620800;
	// 825123A0: 3D608251  lis r11, -0x7daf
	ctx.r[11].s64 = -2108620800;
	// 825123A4: 39082E60  addi r8, r8, 0x2e60
	ctx.r[8].s64 = ctx.r[8].s64 + 11872;
	// 825123A8: 392920A0  addi r9, r9, 0x20a0
	ctx.r[9].s64 = ctx.r[9].s64 + 8352;
	// 825123AC: 394A3260  addi r10, r10, 0x3260
	ctx.r[10].s64 = ctx.r[10].s64 + 12896;
	// 825123B0: 396B20F0  addi r11, r11, 0x20f0
	ctx.r[11].s64 = ctx.r[11].s64 + 8432;
	// 825123B4: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 825123B8: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 825123BC: 91010050  stw r8, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[8].u32 ) };
	// 825123C0: 38A0000B  li r5, 0xb
	ctx.r[5].s64 = 11;
	// 825123C4: 91210054  stw r9, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[9].u32 ) };
	// 825123C8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 825123CC: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 825123D0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825123D4: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 825123D8: 9BC10060  stb r30, 0x60(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[30].u8 ) };
	// 825123DC: 9BC10061  stb r30, 0x61(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(97 as u32), ctx.r[30].u8 ) };
	// 825123E0: 4BFED9A9  bl 0x824ffd88
	ctx.lr = 0x825123E4;
	sub_824FFD88(ctx, base);
	// 825123E4: 3D008251  lis r8, -0x7daf
	ctx.r[8].s64 = -2108620800;
	// 825123E8: 9BC10081  stb r30, 0x81(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(129 as u32), ctx.r[30].u8 ) };
	// 825123EC: 3D208251  lis r9, -0x7daf
	ctx.r[9].s64 = -2108620800;
	// 825123F0: 3D408251  lis r10, -0x7daf
	ctx.r[10].s64 = -2108620800;
	// 825123F4: 3D608251  lis r11, -0x7daf
	ctx.r[11].s64 = -2108620800;
	// 825123F8: 390826F8  addi r8, r8, 0x26f8
	ctx.r[8].s64 = ctx.r[8].s64 + 9976;
	// 825123FC: 392962F8  addi r9, r9, 0x62f8
	ctx.r[9].s64 = ctx.r[9].s64 + 25336;
	// 82512400: 394A60A0  addi r10, r10, 0x60a0
	ctx.r[10].s64 = ctx.r[10].s64 + 24736;
	// 82512404: 396B39C8  addi r11, r11, 0x39c8
	ctx.r[11].s64 = ctx.r[11].s64 + 14792;
	// 82512408: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 8251240C: 38C0000B  li r6, 0xb
	ctx.r[6].s64 = 11;
	// 82512410: 91010070  stw r8, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[8].u32 ) };
	// 82512414: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82512418: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8251241C: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 82512420: 91410078  stw r10, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[10].u32 ) };
	// 82512424: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82512428: 9161007C  stw r11, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[11].u32 ) };
	// 8251242C: 9BA10080  stb r29, 0x80(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[29].u8 ) };
	// 82512430: 4BFED959  bl 0x824ffd88
	ctx.lr = 0x82512434;
	sub_824FFD88(ctx, base);
	// 82512434: 3D008251  lis r8, -0x7daf
	ctx.r[8].s64 = -2108620800;
	// 82512438: 3D208251  lis r9, -0x7daf
	ctx.r[9].s64 = -2108620800;
	// 8251243C: 9BA100A0  stb r29, 0xa0(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(160 as u32), ctx.r[29].u8 ) };
	// 82512440: 3D408251  lis r10, -0x7daf
	ctx.r[10].s64 = -2108620800;
	// 82512444: 9BC100A1  stb r30, 0xa1(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(161 as u32), ctx.r[30].u8 ) };
	// 82512448: 3D608251  lis r11, -0x7daf
	ctx.r[11].s64 = -2108620800;
	// 8251244C: 39083B78  addi r8, r8, 0x3b78
	ctx.r[8].s64 = ctx.r[8].s64 + 15224;
	// 82512450: 392962F8  addi r9, r9, 0x62f8
	ctx.r[9].s64 = ctx.r[9].s64 + 25336;
	// 82512454: 394A60A0  addi r10, r10, 0x60a0
	ctx.r[10].s64 = ctx.r[10].s64 + 24736;
	// 82512458: 396B39C8  addi r11, r11, 0x39c8
	ctx.r[11].s64 = ctx.r[11].s64 + 14792;
	// 8251245C: 38C0000B  li r6, 0xb
	ctx.r[6].s64 = 11;
	// 82512460: 38A0000B  li r5, 0xb
	ctx.r[5].s64 = 11;
	// 82512464: 91010090  stw r8, 0x90(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(144 as u32), ctx.r[8].u32 ) };
	// 82512468: 38810090  addi r4, r1, 0x90
	ctx.r[4].s64 = ctx.r[1].s64 + 144;
	// 8251246C: 91210094  stw r9, 0x94(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(148 as u32), ctx.r[9].u32 ) };
	// 82512470: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82512474: 91410098  stw r10, 0x98(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(152 as u32), ctx.r[10].u32 ) };
	// 82512478: 9161009C  stw r11, 0x9c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(156 as u32), ctx.r[11].u32 ) };
	// 8251247C: 4BFED90D  bl 0x824ffd88
	ctx.lr = 0x82512480;
	sub_824FFD88(ctx, base);
	// 82512480: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 82512484: 48022C88  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82512488(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82512488 size=20
    let mut pc: u32 = 0x82512488;
    'dispatch: loop {
        match pc {
            0x82512488 => {
    //   block [0x82512488..0x8251249C)
	// 82512488: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 8251248C: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82512490: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 82512494: 7CE63B78  mr r6, r7
	ctx.r[6].u64 = ctx.r[7].u64;
	// 82512498: 48003E60  b 0x825162f8
	sub_825162F8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825124A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825124A0 size=20
    let mut pc: u32 = 0x825124A0;
    'dispatch: loop {
        match pc {
            0x825124A0 => {
    //   block [0x825124A0..0x825124B4)
	// 825124A0: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 825124A4: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 825124A8: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 825124AC: 7CE63B78  mr r6, r7
	ctx.r[6].u64 = ctx.r[7].u64;
	// 825124B0: 48003BF0  b 0x825160a0
	sub_825160A0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825124B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825124B8 size=24
    let mut pc: u32 = 0x825124B8;
    'dispatch: loop {
        match pc {
            0x825124B8 => {
    //   block [0x825124B8..0x825124D0)
	// 825124B8: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 825124BC: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 825124C0: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 825124C4: 7CE63B78  mr r6, r7
	ctx.r[6].u64 = ctx.r[7].u64;
	// 825124C8: 7D074378  mr r7, r8
	ctx.r[7].u64 = ctx.r[8].u64;
	// 825124CC: 48003994  b 0x82515e60
	sub_82515E60(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825124D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825124D0 size=84
    let mut pc: u32 = 0x825124D0;
    'dispatch: loop {
        match pc {
            0x825124D0 => {
    //   block [0x825124D0..0x82512524)
	// 825124D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825124D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825124D8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825124DC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825124E0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825124E4: 7C862378  mr r6, r4
	ctx.r[6].u64 = ctx.r[4].u64;
	// 825124E8: 387F0030  addi r3, r31, 0x30
	ctx.r[3].s64 = ctx.r[31].s64 + 48;
	// 825124EC: 80BF0008  lwz r5, 8(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 825124F0: 809F000C  lwz r4, 0xc(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 825124F4: 480AB5CD  bl 0x825bdac0
	ctx.lr = 0x825124F8;
	sub_825BDAC0(ctx, base);
	// 825124F8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825124FC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82512500: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82512504: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82512508: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8251250C: 4E800421  bctrl
	ctx.lr = 0x82512510;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82512510: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82512514: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82512518: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8251251C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82512520: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82512528(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82512528 size=8
    let mut pc: u32 = 0x82512528;
    'dispatch: loop {
        match pc {
            0x82512528 => {
    //   block [0x82512528..0x82512530)
	// 82512528: 38630030  addi r3, r3, 0x30
	ctx.r[3].s64 = ctx.r[3].s64 + 48;
	// 8251252C: 480AB1F4  b 0x825bd720
	sub_825BD720(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82512530(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82512530 size=8
    let mut pc: u32 = 0x82512530;
    'dispatch: loop {
        match pc {
            0x82512530 => {
    //   block [0x82512530..0x82512538)
	// 82512530: 38630030  addi r3, r3, 0x30
	ctx.r[3].s64 = ctx.r[3].s64 + 48;
	// 82512534: 480AB33C  b 0x825bd870
	sub_825BD870(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82512538(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82512538 size=136
    let mut pc: u32 = 0x82512538;
    'dispatch: loop {
        match pc {
            0x82512538 => {
    //   block [0x82512538..0x825125C0)
	// 82512538: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251253C: 48022B79  bl 0x825350b4
	ctx.lr = 0x82512540;
	sub_82535080(ctx, base);
	// 82512540: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82512544: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82512548: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 8251254C: 397F0010  addi r11, r31, 0x10
	ctx.r[11].s64 = ctx.r[31].s64 + 16;
	// 82512550: 395F0020  addi r10, r31, 0x20
	ctx.r[10].s64 = ctx.r[31].s64 + 32;
	// 82512554: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82512558: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 8251255C: 7CFB3B78  mr r27, r7
	ctx.r[27].u64 = ctx.r[7].u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825125C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825125C0 size=172
    let mut pc: u32 = 0x825125C0;
    'dispatch: loop {
        match pc {
            0x825125C0 => {
    //   block [0x825125C0..0x8251266C)
	// 825125C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825125C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825125C8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825125CC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825125D0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825125D4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 825125D8: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 825125DC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 825125E0: 388B4BB4  addi r4, r11, 0x4bb4
	ctx.r[4].s64 = ctx.r[11].s64 + 19380;
	// 825125E4: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 825125E8: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825125EC: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 825125F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825125F4: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 825125F8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825125FC: 4E800421  bctrl
	ctx.lr = 0x82512600;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82512600: 3BDE0030  addi r30, r30, 0x30
	ctx.r[30].s64 = ctx.r[30].s64 + 48;
	// 82512604: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82512608: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 8251260C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82512610: 409A0034  bne cr6, 0x82512644
	if !ctx.cr[6].eq {
	pc = 0x82512644; continue 'dispatch;
	}
	// 82512614: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82512618: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 8251261C: 80FE0004  lwz r7, 4(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82512620: 5568103A  slwi r8, r11, 2
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82512624: 80DE0000  lwz r6, 0(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82512628: 388A1524  addi r4, r10, 0x1524
	ctx.r[4].s64 = ctx.r[10].s64 + 5412;
	// 8251262C: 54E7103A  slwi r7, r7, 2
	ctx.r[7].u32 = ctx.r[7].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82512630: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 82512634: 81690008  lwz r11, 8(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) } as u64;
	// 82512638: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8251263C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82512640: 4E800421  bctrl
	ctx.lr = 0x82512644;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82512644: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82512648: 809F0004  lwz r4, 4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8251264C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82512650: 4BFC3FC1  bl 0x824d6610
	ctx.lr = 0x82512654;
	sub_824D6610(ctx, base);
	// 82512654: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82512658: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8251265C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82512660: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82512664: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82512668: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82512670(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82512670 size=136
    let mut pc: u32 = 0x82512670;
    'dispatch: loop {
        match pc {
            0x82512670 => {
    //   block [0x82512670..0x825126F8)
	// 82512670: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82512674: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82512678: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8251267C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82512680: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 82512684: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82512688: 392A4BC4  addi r9, r10, 0x4bc4
	ctx.r[9].s64 = ctx.r[10].s64 + 19396;
	// 8251268C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82512690: 387F0030  addi r3, r31, 0x30
	ctx.r[3].s64 = ctx.r[31].s64 + 48;
	// 82512694: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82512698: 3CA08000  lis r5, -0x8000
	ctx.r[5].s64 = -2147483648;
	// 8251269C: 90FF0008  stw r7, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[7].u32 ) };
	// 825126A0: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 825126A4: 397F0010  addi r11, r31, 0x10
	ctx.r[11].s64 = ctx.r[31].s64 + 16;
	// 825126A8: B15F0006  sth r10, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 825126AC: 395F0020  addi r10, r31, 0x20
	ctx.r[10].s64 = ctx.r[31].s64 + 32;
	// 825126B0: 91030000  stw r8, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 825126B4: 91030004  stw r8, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 825126B8: 90A30008  stw r5, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[5].u32 ) };
	// 825126BC: 81260000  lwz r9, 0(r6)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 825126C0: 913F000C  stw r9, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825126F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825126F8 size=88
    let mut pc: u32 = 0x825126F8;
    'dispatch: loop {
        match pc {
            0x825126F8 => {
    //   block [0x825126F8..0x82512750)
	// 825126F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825126FC: 480229BD  bl 0x825350b8
	ctx.lr = 0x82512700;
	sub_82535080(ctx, base);
	// 82512700: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82512704: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82512708: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 8251270C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82512710: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82512714: 38A0001F  li r5, 0x1f
	ctx.r[5].s64 = 31;
	// 82512718: 38800040  li r4, 0x40
	ctx.r[4].s64 = 64;
	// 8251271C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82512720: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82512724: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82512728: 4BF51911  bl 0x82464038
	ctx.lr = 0x8251272C;
	sub_82464038(ctx, base);
	// 8251272C: 39600040  li r11, 0x40
	ctx.r[11].s64 = 64;
	// 82512730: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 82512734: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82512738: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8251273C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82512740: B1630004  sth r11, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 82512744: 4BFFFF2D  bl 0x82512670
	ctx.lr = 0x82512748;
	sub_82512670(ctx, base);
	// 82512748: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8251274C: 480229BC  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82512750(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82512750 size=1504
    let mut pc: u32 = 0x82512750;
    'dispatch: loop {
        match pc {
            0x82512750 => {
    //   block [0x82512750..0x82512D30)
	// 82512750: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82512754: 4802293D  bl 0x82535090
	ctx.lr = 0x82512758;
	sub_82535080(ctx, base);
	// 82512758: DBC1FF78  stfd f30, -0x88(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-136 as u32), ctx.f[30].u64 ) };
	// 8251275C: DBE1FF80  stfd f31, -0x80(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-128 as u32), ctx.f[31].u64 ) };
	// 82512760: 3980FF60  li r12, -0xa0
	ctx.r[12].s64 = -160;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82512D30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82512D30 size=300
    let mut pc: u32 = 0x82512D30;
    'dispatch: loop {
        match pc {
            0x82512D30 => {
    //   block [0x82512D30..0x82512E5C)
	// 82512D30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82512D34: 48022385  bl 0x825350b8
	ctx.lr = 0x82512D38;
	sub_82535080(ctx, base);
	// 82512D38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82512D3C: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 82512D40: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82512D44: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82512D48: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82512D4C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82512D50: 419A00BC  beq cr6, 0x82512e0c
	if ctx.cr[6].eq {
	pc = 0x82512E0C; continue 'dispatch;
	}
	// 82512D54: 807E000C  lwz r3, 0xc(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82512D58: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82512D5C: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82512D60: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82512D64: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82512D68: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82512D6C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82512D70: 4E800421  bctrl
	ctx.lr = 0x82512D74;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82512D74: 2B030001  cmplwi cr6, r3, 1
	ctx.cr[6].compare_u32(ctx.r[3].u32, 1 as u32, &mut ctx.xer);
	// 82512D78: 41980058  blt cr6, 0x82512dd0
	if ctx.cr[6].lt {
	pc = 0x82512DD0; continue 'dispatch;
	}
	// 82512D7C: 419A0018  beq cr6, 0x82512d94
	if ctx.cr[6].eq {
	pc = 0x82512D94; continue 'dispatch;
	}
	// 82512D80: 2B030003  cmplwi cr6, r3, 3
	ctx.cr[6].compare_u32(ctx.r[3].u32, 3 as u32, &mut ctx.xer);
	// 82512D84: 4198004C  blt cr6, 0x82512dd0
	if ctx.cr[6].lt {
	pc = 0x82512DD0; continue 'dispatch;
	}
	// 82512D88: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82512D8C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82512D90: 48022378  b 0x82535108
	sub_825350D0(ctx, base);
	return;
	// 82512D94: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82512D98: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 82512D9C: 38A0001F  li r5, 0x1f
	ctx.r[5].s64 = 31;
	// 82512DA0: 38800038  li r4, 0x38
	ctx.r[4].s64 = 56;
	// 82512DA4: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82512DA8: 4BF51291  bl 0x82464038
	ctx.lr = 0x82512DAC;
	sub_82464038(ctx, base);
	// 82512DAC: 39600038  li r11, 0x38
	ctx.r[11].s64 = 56;
	// 82512DB0: 7FE7FB78  mr r7, r31
	ctx.r[7].u64 = ctx.r[31].u64;
	// 82512DB4: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82512DB8: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82512DBC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82512DC0: B1630004  sth r11, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 82512DC4: 48012FA5  bl 0x82525d68
	ctx.lr = 0x82512DC8;
	sub_82525D68(ctx, base);
	// 82512DC8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82512DCC: 4802233C  b 0x82535108
	sub_825350D0(ctx, base);
	return;
	// 82512DD0: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82512DD4: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 82512DD8: 38A0001F  li r5, 0x1f
	ctx.r[5].s64 = 31;
	// 82512DDC: 38800040  li r4, 0x40
	ctx.r[4].s64 = 64;
	// 82512DE0: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82512DE4: 4BF51255  bl 0x82464038
	ctx.lr = 0x82512DE8;
	sub_82464038(ctx, base);
	// 82512DE8: 39600040  li r11, 0x40
	ctx.r[11].s64 = 64;
	// 82512DEC: 7FE7FB78  mr r7, r31
	ctx.r[7].u64 = ctx.r[31].u64;
	// 82512DF0: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82512DF4: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82512DF8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82512DFC: B1630004  sth r11, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 82512E00: 4BFFF871  bl 0x82512670
	ctx.lr = 0x82512E04;
	sub_82512670(ctx, base);
	// 82512E04: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82512E08: 48022300  b 0x82535108
	sub_825350D0(ctx, base);
	return;
	// 82512E0C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82512E10: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 82512E14: 38A0001F  li r5, 0x1f
	ctx.r[5].s64 = 31;
	// 82512E18: 38800040  li r4, 0x40
	ctx.r[4].s64 = 64;
	// 82512E1C: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82512E20: 4BF51219  bl 0x82464038
	ctx.lr = 0x82512E24;
	sub_82464038(ctx, base);
	// 82512E24: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82512E28: 39600040  li r11, 0x40
	ctx.r[11].s64 = 64;
	// 82512E2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82512E30: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82512E34: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82512E38: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82512E3C: B17F0004  sth r11, 4(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 82512E40: 4BFFF831  bl 0x82512670
	ctx.lr = 0x82512E44;
	sub_82512670(ctx, base);
	// 82512E44: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82512E48: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82512E4C: 396B4C24  addi r11, r11, 0x4c24
	ctx.r[11].s64 = ctx.r[11].s64 + 19492;
	// 82512E50: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82512E54: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82512E58: 480222B0  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82512E60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82512E60 size=108
    let mut pc: u32 = 0x82512E60;
    'dispatch: loop {
        match pc {
            0x82512E60 => {
    //   block [0x82512E60..0x82512ECC)
	// 82512E60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82512E64: 48022251  bl 0x825350b4
	ctx.lr = 0x82512E68;
	sub_82535080(ctx, base);
	// 82512E68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82512E6C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82512E70: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 82512E74: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82512E78: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82512E7C: 38A0001F  li r5, 0x1f
	ctx.r[5].s64 = 31;
	// 82512E80: 38800040  li r4, 0x40
	ctx.r[4].s64 = 64;
	// 82512E84: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82512E88: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82512E8C: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 82512E90: 4BF511A9  bl 0x82464038
	ctx.lr = 0x82512E94;
	sub_82464038(ctx, base);
	// 82512E94: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82512E98: 39600040  li r11, 0x40
	ctx.r[11].s64 = 64;
	// 82512E9C: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 82512EA0: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82512EA4: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82512EA8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82512EAC: B17F0004  sth r11, 4(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 82512EB0: 4BFFF7C1  bl 0x82512670
	ctx.lr = 0x82512EB4;
	sub_82512670(ctx, base);
	// 82512EB4: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82512EB8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82512EBC: 396B4C24  addi r11, r11, 0x4c24
	ctx.r[11].s64 = ctx.r[11].s64 + 19492;
	// 82512EC0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82512EC4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82512EC8: 4802223C  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82512ED0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82512ED0 size=204
    let mut pc: u32 = 0x82512ED0;
    'dispatch: loop {
        match pc {
            0x82512ED0 => {
    //   block [0x82512ED0..0x82512F9C)
	// 82512ED0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82512ED4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82512ED8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82512EDC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82512EE0: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82512EE4: 3D008251  lis r8, -0x7daf
	ctx.r[8].s64 = -2108620800;
	// 82512EE8: 3D208251  lis r9, -0x7daf
	ctx.r[9].s64 = -2108620800;
	// 82512EEC: 3D408251  lis r10, -0x7daf
	ctx.r[10].s64 = -2108620800;
	// 82512EF0: 3D608251  lis r11, -0x7daf
	ctx.r[11].s64 = -2108620800;
	// 82512EF4: 39082E60  addi r8, r8, 0x2e60
	ctx.r[8].s64 = ctx.r[8].s64 + 11872;
	// 82512EF8: 392920A0  addi r9, r9, 0x20a0
	ctx.r[9].s64 = ctx.r[9].s64 + 8352;
	// 82512EFC: 394A3260  addi r10, r10, 0x3260
	ctx.r[10].s64 = ctx.r[10].s64 + 12896;
	// 82512F00: 396B32B0  addi r11, r11, 0x32b0
	ctx.r[11].s64 = ctx.r[11].s64 + 12976;
	// 82512F04: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 82512F08: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82512F0C: 91010050  stw r8, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[8].u32 ) };
	// 82512F10: 38A00009  li r5, 9
	ctx.r[5].s64 = 9;
	// 82512F14: 91210054  stw r9, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[9].u32 ) };
	// 82512F18: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82512F1C: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 82512F20: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82512F24: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82512F28: 9BC10060  stb r30, 0x60(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[30].u8 ) };
	// 82512F2C: 9BC10061  stb r30, 0x61(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(97 as u32), ctx.r[30].u8 ) };
	// 82512F30: 4BFECE59  bl 0x824ffd88
	ctx.lr = 0x82512F34;
	sub_824FFD88(ctx, base);
	// 82512F34: 3D608251  lis r11, -0x7daf
	ctx.r[11].s64 = -2108620800;
	// 82512F38: 9BC10081  stb r30, 0x81(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(129 as u32), ctx.r[30].u8 ) };
	// 82512F3C: 3D008251  lis r8, -0x7daf
	ctx.r[8].s64 = -2108620800;
	// 82512F40: 396B5E60  addi r11, r11, 0x5e60
	ctx.r[11].s64 = ctx.r[11].s64 + 24160;
	// 82512F44: 3D208251  lis r9, -0x7daf
	ctx.r[9].s64 = -2108620800;
	// 82512F48: 3D408251  lis r10, -0x7daf
	ctx.r[10].s64 = -2108620800;
	// 82512F4C: 390826F8  addi r8, r8, 0x26f8
	ctx.r[8].s64 = ctx.r[8].s64 + 9976;
	// 82512F50: 392962F8  addi r9, r9, 0x62f8
	ctx.r[9].s64 = ctx.r[9].s64 + 25336;
	// 82512F54: 9161007C  stw r11, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[11].u32 ) };
	// 82512F58: 394A60A0  addi r10, r10, 0x60a0
	ctx.r[10].s64 = ctx.r[10].s64 + 24736;
	// 82512F5C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82512F60: 38C00009  li r6, 9
	ctx.r[6].s64 = 9;
	// 82512F64: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82512F68: 91010070  stw r8, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[8].u32 ) };
	// 82512F6C: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 82512F70: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82512F74: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82512F78: 91410078  stw r10, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[10].u32 ) };
	// 82512F7C: 99610080  stb r11, 0x80(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[11].u8 ) };
	// 82512F80: 4BFECE09  bl 0x824ffd88
	ctx.lr = 0x82512F84;
	sub_824FFD88(ctx, base);
	// 82512F84: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82512F88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82512F8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82512F90: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82512F94: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82512F98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82512FA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82512FA0 size=204
    let mut pc: u32 = 0x82512FA0;
    'dispatch: loop {
        match pc {
            0x82512FA0 => {
    //   block [0x82512FA0..0x8251306C)
	// 82512FA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82512FA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82512FA8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82512FAC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82512FB0: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82512FB4: 3D008251  lis r8, -0x7daf
	ctx.r[8].s64 = -2108620800;
	// 82512FB8: 3D208251  lis r9, -0x7daf
	ctx.r[9].s64 = -2108620800;
	// 82512FBC: 3D408251  lis r10, -0x7daf
	ctx.r[10].s64 = -2108620800;
	// 82512FC0: 3D608251  lis r11, -0x7daf
	ctx.r[11].s64 = -2108620800;
	// 82512FC4: 39082E60  addi r8, r8, 0x2e60
	ctx.r[8].s64 = ctx.r[8].s64 + 11872;
	// 82512FC8: 392920A0  addi r9, r9, 0x20a0
	ctx.r[9].s64 = ctx.r[9].s64 + 8352;
	// 82512FCC: 394A3260  addi r10, r10, 0x3260
	ctx.r[10].s64 = ctx.r[10].s64 + 12896;
	// 82512FD0: 396B32B0  addi r11, r11, 0x32b0
	ctx.r[11].s64 = ctx.r[11].s64 + 12976;
	// 82512FD4: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 82512FD8: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82512FDC: 91010050  stw r8, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[8].u32 ) };
	// 82512FE0: 38A00009  li r5, 9
	ctx.r[5].s64 = 9;
	// 82512FE4: 91210054  stw r9, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[9].u32 ) };
	// 82512FE8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82512FEC: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 82512FF0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82512FF4: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82512FF8: 9BC10060  stb r30, 0x60(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[30].u8 ) };
	// 82512FFC: 9BC10061  stb r30, 0x61(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(97 as u32), ctx.r[30].u8 ) };
	// 82513000: 4BFECD89  bl 0x824ffd88
	ctx.lr = 0x82513004;
	sub_824FFD88(ctx, base);
	// 82513004: 3D608251  lis r11, -0x7daf
	ctx.r[11].s64 = -2108620800;
	// 82513008: 9BC10081  stb r30, 0x81(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(129 as u32), ctx.r[30].u8 ) };
	// 8251300C: 3D008251  lis r8, -0x7daf
	ctx.r[8].s64 = -2108620800;
	// 82513010: 396B5E60  addi r11, r11, 0x5e60
	ctx.r[11].s64 = ctx.r[11].s64 + 24160;
	// 82513014: 3D208251  lis r9, -0x7daf
	ctx.r[9].s64 = -2108620800;
	// 82513018: 3D408251  lis r10, -0x7daf
	ctx.r[10].s64 = -2108620800;
	// 8251301C: 390826F8  addi r8, r8, 0x26f8
	ctx.r[8].s64 = ctx.r[8].s64 + 9976;
	// 82513020: 392962F8  addi r9, r9, 0x62f8
	ctx.r[9].s64 = ctx.r[9].s64 + 25336;
	// 82513024: 9161007C  stw r11, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[11].u32 ) };
	// 82513028: 394A60A0  addi r10, r10, 0x60a0
	ctx.r[10].s64 = ctx.r[10].s64 + 24736;
	// 8251302C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82513030: 38C00009  li r6, 9
	ctx.r[6].s64 = 9;
	// 82513034: 38A00017  li r5, 0x17
	ctx.r[5].s64 = 23;
	// 82513038: 91010070  stw r8, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[8].u32 ) };
	// 8251303C: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 82513040: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82513044: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82513048: 91410078  stw r10, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[10].u32 ) };
	// 8251304C: 99610080  stb r11, 0x80(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[11].u8 ) };
	// 82513050: 4BFECD39  bl 0x824ffd88
	ctx.lr = 0x82513054;
	sub_824FFD88(ctx, base);
	// 82513054: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82513058: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8251305C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82513060: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82513064: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82513068: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82513070(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82513070 size=288
    let mut pc: u32 = 0x82513070;
    'dispatch: loop {
        match pc {
            0x82513070 => {
    //   block [0x82513070..0x82513190)
	// 82513070: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82513074: 48022041  bl 0x825350b4
	ctx.lr = 0x82513078;
	sub_82535080(ctx, base);
	// 82513078: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251307C: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 82513080: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82513084: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82513088: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 8251308C: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 82513090: 419A00B0  beq cr6, 0x82513140
	if ctx.cr[6].eq {
	pc = 0x82513140; continue 'dispatch;
	}
	// 82513094: 807E000C  lwz r3, 0xc(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82513098: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 8251309C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 825130A0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825130A4: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 825130A8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825130AC: 4E800421  bctrl
	ctx.lr = 0x825130B0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825130B0: 2B030001  cmplwi cr6, r3, 1
	ctx.cr[6].compare_u32(ctx.r[3].u32, 1 as u32, &mut ctx.xer);
	// 825130B4: 4198006C  blt cr6, 0x82513120
	if ctx.cr[6].lt {
	pc = 0x82513120; continue 'dispatch;
	}
	// 825130B8: 419A0018  beq cr6, 0x825130d0
	if ctx.cr[6].eq {
	pc = 0x825130D0; continue 'dispatch;
	}
	// 825130BC: 2B030003  cmplwi cr6, r3, 3
	ctx.cr[6].compare_u32(ctx.r[3].u32, 3 as u32, &mut ctx.xer);
	// 825130C0: 41980060  blt cr6, 0x82513120
	if ctx.cr[6].lt {
	pc = 0x82513120; continue 'dispatch;
	}
	// 825130C4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 825130C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825130CC: 48022038  b 0x82535104
	sub_825350D0(ctx, base);
	return;
	// 825130D0: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 825130D4: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 825130D8: 38A0001F  li r5, 0x1f
	ctx.r[5].s64 = 31;
	// 825130DC: 38800038  li r4, 0x38
	ctx.r[4].s64 = 56;
	// 825130E0: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 825130E4: 4BF50F55  bl 0x82464038
	ctx.lr = 0x825130E8;
	sub_82464038(ctx, base);
	// 825130E8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825130EC: 39600038  li r11, 0x38
	ctx.r[11].s64 = 56;
	// 825130F0: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 825130F4: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 825130F8: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 825130FC: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82513100: B17F0004  sth r11, 4(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 82513104: 48012C65  bl 0x82525d68
	ctx.lr = 0x82513108;
	sub_82525D68(ctx, base);
	// 82513108: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8251310C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82513110: 396B4C60  addi r11, r11, 0x4c60
	ctx.r[11].s64 = ctx.r[11].s64 + 19552;
	// 82513114: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82513118: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8251311C: 48021FE8  b 0x82535104
	sub_825350D0(ctx, base);
	return;
	// 82513120: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82513124: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 82513128: 38A0001F  li r5, 0x1f
	ctx.r[5].s64 = 31;
	// 8251312C: 38800040  li r4, 0x40
	ctx.r[4].s64 = 64;
	// 82513130: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82513134: 4BF50F05  bl 0x82464038
	ctx.lr = 0x82513138;
	sub_82464038(ctx, base);
	// 82513138: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 8251313C: 48000020  b 0x8251315c
	pc = 0x8251315C; continue 'dispatch;
	// 82513140: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82513144: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 82513148: 38A0001F  li r5, 0x1f
	ctx.r[5].s64 = 31;
	// 8251314C: 38800040  li r4, 0x40
	ctx.r[4].s64 = 64;
	// 82513150: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82513154: 4BF50EE5  bl 0x82464038
	ctx.lr = 0x82513158;
	sub_82464038(ctx, base);
	// 82513158: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8251315C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82513160: 39600040  li r11, 0x40
	ctx.r[11].s64 = 64;
	// 82513164: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82513168: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8251316C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82513170: B17F0004  sth r11, 4(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 82513174: 4BFFF4FD  bl 0x82512670
	ctx.lr = 0x82513178;
	sub_82512670(ctx, base);
	// 82513178: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8251317C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82513180: 396B4C24  addi r11, r11, 0x4c24
	ctx.r[11].s64 = ctx.r[11].s64 + 19492;
	// 82513184: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82513188: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8251318C: 48021F78  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82513190(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82513190 size=204
    let mut pc: u32 = 0x82513190;
    'dispatch: loop {
        match pc {
            0x82513190 => {
    //   block [0x82513190..0x8251325C)
	// 82513190: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82513194: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82513198: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8251319C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825131A0: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825131A4: 3D008251  lis r8, -0x7daf
	ctx.r[8].s64 = -2108620800;
	// 825131A8: 3D208251  lis r9, -0x7daf
	ctx.r[9].s64 = -2108620800;
	// 825131AC: 3D408251  lis r10, -0x7daf
	ctx.r[10].s64 = -2108620800;
	// 825131B0: 3D608251  lis r11, -0x7daf
	ctx.r[11].s64 = -2108620800;
	// 825131B4: 39083070  addi r8, r8, 0x3070
	ctx.r[8].s64 = ctx.r[8].s64 + 12400;
	// 825131B8: 392920A0  addi r9, r9, 0x20a0
	ctx.r[9].s64 = ctx.r[9].s64 + 8352;
	// 825131BC: 394A3260  addi r10, r10, 0x3260
	ctx.r[10].s64 = ctx.r[10].s64 + 12896;
	// 825131C0: 396B32B0  addi r11, r11, 0x32b0
	ctx.r[11].s64 = ctx.r[11].s64 + 12976;
	// 825131C4: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 825131C8: 38C00015  li r6, 0x15
	ctx.r[6].s64 = 21;
	// 825131CC: 91010050  stw r8, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[8].u32 ) };
	// 825131D0: 38A00009  li r5, 9
	ctx.r[5].s64 = 9;
	// 825131D4: 91210054  stw r9, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[9].u32 ) };
	// 825131D8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 825131DC: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 825131E0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825131E4: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 825131E8: 9BC10060  stb r30, 0x60(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[30].u8 ) };
	// 825131EC: 9BC10061  stb r30, 0x61(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(97 as u32), ctx.r[30].u8 ) };
	// 825131F0: 4BFECB99  bl 0x824ffd88
	ctx.lr = 0x825131F4;
	sub_824FFD88(ctx, base);
	// 825131F4: 3D608251  lis r11, -0x7daf
	ctx.r[11].s64 = -2108620800;
	// 825131F8: 9BC10081  stb r30, 0x81(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(129 as u32), ctx.r[30].u8 ) };
	// 825131FC: 3D008251  lis r8, -0x7daf
	ctx.r[8].s64 = -2108620800;
	// 82513200: 396B5E60  addi r11, r11, 0x5e60
	ctx.r[11].s64 = ctx.r[11].s64 + 24160;
	// 82513204: 3D208251  lis r9, -0x7daf
	ctx.r[9].s64 = -2108620800;
	// 82513208: 3D408251  lis r10, -0x7daf
	ctx.r[10].s64 = -2108620800;
	// 8251320C: 39082D30  addi r8, r8, 0x2d30
	ctx.r[8].s64 = ctx.r[8].s64 + 11568;
	// 82513210: 392962F8  addi r9, r9, 0x62f8
	ctx.r[9].s64 = ctx.r[9].s64 + 25336;
	// 82513214: 9161007C  stw r11, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[11].u32 ) };
	// 82513218: 394A60A0  addi r10, r10, 0x60a0
	ctx.r[10].s64 = ctx.r[10].s64 + 24736;
	// 8251321C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82513220: 38C00009  li r6, 9
	ctx.r[6].s64 = 9;
	// 82513224: 38A00015  li r5, 0x15
	ctx.r[5].s64 = 21;
	// 82513228: 91010070  stw r8, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[8].u32 ) };
	// 8251322C: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 82513230: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82513234: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82513238: 91410078  stw r10, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[10].u32 ) };
	// 8251323C: 99610080  stb r11, 0x80(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[11].u8 ) };
	// 82513240: 4BFECB49  bl 0x824ffd88
	ctx.lr = 0x82513244;
	sub_824FFD88(ctx, base);
	// 82513244: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82513248: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8251324C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82513250: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82513254: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82513258: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82513260(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82513260 size=80
    let mut pc: u32 = 0x82513260;
    'dispatch: loop {
        match pc {
            0x82513260 => {
    //   block [0x82513260..0x825132B0)
	// 82513260: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82513264: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82513268: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251326C: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 82513270: 3D008201  lis r8, -0x7dff
	ctx.r[8].s64 = -2113863680;
	// 82513274: 7CCA3378  mr r10, r6
	ctx.r[10].u64 = ctx.r[6].u64;
	// 82513278: 39294B94  addi r9, r9, 0x4b94
	ctx.r[9].s64 = ctx.r[9].s64 + 19348;
	// 8251327C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82513280: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82513284: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82513288: C0088CB4  lfs f0, -0x734c(r8)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(-29516 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8251328C: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82513290: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82513294: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 82513298: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 8251329C: 48002E05  bl 0x825160a0
	ctx.lr = 0x825132A0;
	sub_825160A0(ctx, base);
	// 825132A0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825132A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825132A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825132AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825132B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825132B0 size=232
    let mut pc: u32 = 0x825132B0;
    'dispatch: loop {
        match pc {
            0x825132B0 => {
    //   block [0x825132B0..0x82513398)
	// 825132B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825132B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825132B8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825132BC: 9421FED0  stwu r1, -0x130(r1)
	ea = ctx.r[1].u32.wrapping_add(-304 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825132C0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825132C4: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 825132C8: 7CE83B78  mr r8, r7
	ctx.r[8].u64 = ctx.r[7].u64;
	// 825132CC: 394100B0  addi r10, r1, 0xb0
	ctx.r[10].s64 = ctx.r[1].s64 + 176;
	// 825132D0: 7CAB2B78  mr r11, r5
	ctx.r[11].u64 = ctx.r[5].u64;
	// 825132D4: 3920000E  li r9, 0xe
	ctx.r[9].s64 = 14;
	// 825132D8: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 825132DC: E92B0000  ld r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 825132E0: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 825132E4: F92A0000  std r9, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u64 ) };
	// 825132E8: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 825132EC: 4200FFF0  bdnz 0x825132dc
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x825132DC; continue 'dispatch;
	}
	// 825132F0: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 825132F4: 39650050  addi r11, r5, 0x50
	ctx.r[11].s64 = ctx.r[5].s64 + 80;
	// 825132F8: 3929FA60  addi r9, r9, -0x5a0
	ctx.r[9].s64 = ctx.r[9].s64 + -1440;
	// 825132FC: 3CE08201  lis r7, -0x7dff
	ctx.r[7].s64 = -2113863680;
	// 82513300: 39410060  addi r10, r1, 0x60
	ctx.r[10].s64 = ctx.r[1].s64 + 96;
	// 82513304: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82513398(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82513398 size=164
    let mut pc: u32 = 0x82513398;
    'dispatch: loop {
        match pc {
            0x82513398 => {
    //   block [0x82513398..0x8251343C)
	// 82513398: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251339C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825133A0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825133A4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825133A8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825133AC: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 825133B0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825133B4: 396B4BC4  addi r11, r11, 0x4bc4
	ctx.r[11].s64 = ctx.r[11].s64 + 19396;
	// 825133B8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825133BC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825133C0: 817F0038  lwz r11, 0x38(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 825133C4: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 825133C8: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 825133CC: 409A0020  bne cr6, 0x825133ec
	if !ctx.cr[6].eq {
	pc = 0x825133EC; continue 'dispatch;
	}
	// 825133D0: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 825133D4: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 825133D8: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 825133DC: 809F0030  lwz r4, 0x30(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 825133E0: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 825133E4: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 825133E8: 4BF50CD1  bl 0x824640b8
	ctx.lr = 0x825133EC;
	sub_824640B8(ctx, base);
	// 825133EC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 825133F0: 57CA07FE  clrlwi r10, r30, 0x1f
	ctx.r[10].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 825133F4: 396B6DD0  addi r11, r11, 0x6dd0
	ctx.r[11].s64 = ctx.r[11].s64 + 28112;
	// 825133F8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 825133FC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82513400: 419A0020  beq cr6, 0x82513420
	if ctx.cr[6].eq {
	pc = 0x82513420; continue 'dispatch;
	}
	// 82513404: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82513408: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 8251340C: 38C0001F  li r6, 0x1f
	ctx.r[6].s64 = 31;
	// 82513410: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82513414: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82513418: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8251341C: 4BF50C9D  bl 0x824640b8
	ctx.lr = 0x82513420;
	sub_824640B8(ctx, base);
	// 82513420: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82513424: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82513428: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8251342C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82513430: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82513434: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82513438: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82513440(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82513440 size=120
    let mut pc: u32 = 0x82513440;
    'dispatch: loop {
        match pc {
            0x82513440 => {
    //   block [0x82513440..0x825134B8)
	// 82513440: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82513444: 39430010  addi r10, r3, 0x10
	ctx.r[10].s64 = ctx.r[3].s64 + 16;
	// 82513448: 396B08F0  addi r11, r11, 0x8f0
	ctx.r[11].s64 = ctx.r[11].s64 + 2288;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825134B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825134B8 size=20
    let mut pc: u32 = 0x825134B8;
    'dispatch: loop {
        match pc {
            0x825134B8 => {
    //   block [0x825134B8..0x825134CC)
	// 825134B8: A14B0002  lhz r10, 2(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(2 as u32) ) } as u64;
	// 825134BC: A12B0004  lhz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 825134C0: B14B0004  sth r10, 4(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u16 ) };
	// 825134C4: B12B0002  sth r9, 2(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(2 as u32), ctx.r[9].u16 ) };
	// 825134C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825134D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x825134D0 size=72
    let mut pc: u32 = 0x825134D0;
    'dispatch: loop {
        match pc {
            0x825134D0 => {
    //   block [0x825134D0..0x82513518)
	// 825134D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825134D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825134D8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825134DC: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 825134E0: 90E10058  stw r7, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[7].u32 ) };
	// 825134E4: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 825134E8: 396B4B94  addi r11, r11, 0x4b94
	ctx.r[11].s64 = ctx.r[11].s64 + 19348;
	// 825134EC: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 825134F0: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 825134F4: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 825134F8: C00A8CB4  lfs f0, -0x734c(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-29516 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825134FC: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82513500: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82513504: 48002B9D  bl 0x825160a0
	ctx.lr = 0x82513508;
	sub_825160A0(ctx, base);
	// 82513508: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8251350C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82513510: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82513514: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82513518(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82513518 size=16
    let mut pc: u32 = 0x82513518;
    'dispatch: loop {
        match pc {
            0x82513518 => {
    //   block [0x82513518..0x82513528)
	// 82513518: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 8251351C: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82513520: 7D655B78  mr r5, r11
	ctx.r[5].u64 = ctx.r[11].u64;
	// 82513524: 4BFFF014  b 0x82512538
	sub_82512538(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82513528(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82513528 size=176
    let mut pc: u32 = 0x82513528;
    'dispatch: loop {
        match pc {
            0x82513528 => {
    //   block [0x82513528..0x825135D8)
	// 82513528: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251352C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82513530: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82513534: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82513538: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 8251353C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82513540: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 82513544: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82513548: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 8251354C: 7D655B78  mr r5, r11
	ctx.r[5].u64 = ctx.r[11].u64;
	// 82513550: 83FE0000  lwz r31, 0(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82513554: C3FE3030  lfs f31, 0x3030(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12336 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82513558: 4BFFF1F9  bl 0x82512750
	ctx.lr = 0x8251355C;
	sub_82512750(ctx, base);
	// 8251355C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82513560: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82513564: 40980044  bge cr6, 0x825135a8
	if !ctx.cr[6].lt {
	pc = 0x825135A8; continue 'dispatch;
	}
	// 82513568: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8251356C: 394B08F0  addi r10, r11, 0x8f0
	ctx.r[10].s64 = ctx.r[11].s64 + 2288;
	// 82513570: 397F0010  addi r11, r31, 0x10
	ctx.r[11].s64 = ctx.r[31].s64 + 16;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


