pub fn sub_8250F5C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8250F5C0 size=92
    let mut pc: u32 = 0x8250F5C0;
    'dispatch: loop {
        match pc {
            0x8250F5C0 => {
    //   block [0x8250F5C0..0x8250F61C)
	// 8250F5C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250F5C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8250F5C8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8250F5CC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250F5D0: 816300BC  lwz r11, 0xbc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(188 as u32) ) } as u64;
	// 8250F5D4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8250F5D8: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8250F5DC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8250F5E0: 388B0094  addi r4, r11, 0x94
	ctx.r[4].s64 = ctx.r[11].s64 + 148;
	// 8250F5E4: 409A0008  bne cr6, 0x8250f5ec
	if !ctx.cr[6].eq {
	pc = 0x8250F5EC; continue 'dispatch;
	}
	// 8250F5E8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8250F5EC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8250F5F0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8250F5F4: 488E2605  bl 0x82df1bf8
	ctx.lr = 0x8250F5F8;
	sub_82DF1BF8(ctx, base);
	// 8250F5F8: E9610050  ld r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 8250F5FC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8250F600: F97F0018  std r11, 0x18(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u64 ) };
	// 8250F604: 488E268D  bl 0x82df1c90
	ctx.lr = 0x8250F608;
	sub_82DF1C90(ctx, base);
	// 8250F608: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8250F60C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8250F610: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8250F614: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8250F618: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250F620(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8250F620 size=116
    let mut pc: u32 = 0x8250F620;
    'dispatch: loop {
        match pc {
            0x8250F620 => {
    //   block [0x8250F620..0x8250F694)
	// 8250F620: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250F624: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8250F628: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8250F62C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250F630: 3D608328  lis r11, -0x7cd8
	ctx.r[11].s64 = -2094530560;
	// 8250F634: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8250F638: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8250F63C: 808BE254  lwz r4, -0x1dac(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-7596 as u32) ) } as u64;
	// 8250F640: 488E43C9  bl 0x82df3a08
	ctx.lr = 0x8250F644;
	sub_82DF3A08(ctx, base);
	// 8250F644: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8250F648: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8250F64C: 4BFFFE7D  bl 0x8250f4c8
	ctx.lr = 0x8250F650;
	sub_8250F4C8(ctx, base);
	// 8250F650: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250F654: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8250F658: 386BFFFC  addi r3, r11, -4
	ctx.r[3].s64 = ctx.r[11].s64 + -4;
	// 8250F65C: 409A0008  bne cr6, 0x8250f664
	if !ctx.cr[6].eq {
	pc = 0x8250F664; continue 'dispatch;
	}
	// 8250F660: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8250F664: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8250F668: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8250F66C: 4BFF9115  bl 0x82508780
	ctx.lr = 0x8250F670;
	sub_82508780(ctx, base);
	// 8250F670: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8250F674: 488E261D  bl 0x82df1c90
	ctx.lr = 0x8250F678;
	sub_82DF1C90(ctx, base);
	// 8250F678: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8250F67C: 488E3DAD  bl 0x82df3428
	ctx.lr = 0x8250F680;
	sub_82DF3428(ctx, base);
	// 8250F680: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8250F684: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8250F688: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8250F68C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8250F690: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250F698(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8250F698 size=116
    let mut pc: u32 = 0x8250F698;
    'dispatch: loop {
        match pc {
            0x8250F698 => {
    //   block [0x8250F698..0x8250F70C)
	// 8250F698: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250F69C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8250F6A0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8250F6A4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250F6A8: 3D608328  lis r11, -0x7cd8
	ctx.r[11].s64 = -2094530560;
	// 8250F6AC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8250F6B0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8250F6B4: 808BE254  lwz r4, -0x1dac(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-7596 as u32) ) } as u64;
	// 8250F6B8: 488E4351  bl 0x82df3a08
	ctx.lr = 0x8250F6BC;
	sub_82DF3A08(ctx, base);
	// 8250F6BC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8250F6C0: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8250F6C4: 4BFFFE05  bl 0x8250f4c8
	ctx.lr = 0x8250F6C8;
	sub_8250F4C8(ctx, base);
	// 8250F6C8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250F6CC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8250F6D0: 386BFFFC  addi r3, r11, -4
	ctx.r[3].s64 = ctx.r[11].s64 + -4;
	// 8250F6D4: 409A0008  bne cr6, 0x8250f6dc
	if !ctx.cr[6].eq {
	pc = 0x8250F6DC; continue 'dispatch;
	}
	// 8250F6D8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8250F6DC: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8250F6E0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8250F6E4: 4BFF97A5  bl 0x82508e88
	ctx.lr = 0x8250F6E8;
	sub_82508E88(ctx, base);
	// 8250F6E8: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8250F6EC: 488E25A5  bl 0x82df1c90
	ctx.lr = 0x8250F6F0;
	sub_82DF1C90(ctx, base);
	// 8250F6F0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8250F6F4: 488E3D35  bl 0x82df3428
	ctx.lr = 0x8250F6F8;
	sub_82DF3428(ctx, base);
	// 8250F6F8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8250F6FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8250F700: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8250F704: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8250F708: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250F710(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8250F710 size=80
    let mut pc: u32 = 0x8250F710;
    'dispatch: loop {
        match pc {
            0x8250F710 => {
    //   block [0x8250F710..0x8250F760)
	// 8250F710: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250F714: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8250F718: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250F71C: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 8250F720: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 8250F724: 38C00030  li r6, 0x30
	ctx.r[6].s64 = 48;
	// 8250F728: 388A08B0  addi r4, r10, 0x8b0
	ctx.r[4].s64 = ctx.r[10].s64 + 2224;
	// 8250F72C: 38A0002D  li r5, 0x2d
	ctx.r[5].s64 = 45;
	// 8250F730: 806B110C  lwz r3, 0x110c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4364 as u32) ) } as u64;
	// 8250F734: 488E2995  bl 0x82df20c8
	ctx.lr = 0x8250F738;
	sub_82DF20C8(ctx, base);
	// 8250F738: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8250F73C: 41820008  beq 0x8250f744
	if ctx.cr[0].eq {
	pc = 0x8250F744; continue 'dispatch;
	}
	// 8250F740: 90630000  stw r3, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 8250F744: 35630004  addic. r11, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8250F748: 41820008  beq 0x8250f750
	if ctx.cr[0].eq {
	pc = 0x8250F750; continue 'dispatch;
	}
	// 8250F74C: 906B0000  stw r3, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 8250F750: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8250F754: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8250F758: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8250F75C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250F760(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8250F760 size=88
    let mut pc: u32 = 0x8250F760;
    'dispatch: loop {
        match pc {
            0x8250F760 => {
    //   block [0x8250F760..0x8250F7B8)
	// 8250F760: 81440000  lwz r10, 0(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250F764: 39630004  addi r11, r3, 4
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	// 8250F768: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8250F76C: 81640004  lwz r11, 4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 8250F770: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8250F774: 81640008  lwz r11, 8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250F778: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8250F77C: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8250F780: 419A0024  beq cr6, 0x8250f7a4
	if ctx.cr[6].eq {
	pc = 0x8250F7A4; continue 'dispatch;
	}
	// 8250F784: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8250F788: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8250F78C: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8250F790: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8250F794: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8250F798: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8250F79C: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8250F7A0: 4082FFE8  bne 0x8250f788
	if !ctx.cr[0].eq {
	pc = 0x8250F788; continue 'dispatch;
	}
	// 8250F7A4: 8964000C  lbz r11, 0xc(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(12 as u32) ) } as u64;
	// 8250F7A8: 9963000C  stb r11, 0xc(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u8 ) };
	// 8250F7AC: 8964000D  lbz r11, 0xd(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(13 as u32) ) } as u64;
	// 8250F7B0: 9963000D  stb r11, 0xd(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(13 as u32), ctx.r[11].u8 ) };
	// 8250F7B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250F7B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8250F7B8 size=112
    let mut pc: u32 = 0x8250F7B8;
    'dispatch: loop {
        match pc {
            0x8250F7B8 => {
    //   block [0x8250F7B8..0x8250F828)
	// 8250F7B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250F7BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8250F7C0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8250F7C4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250F7C8: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250F7CC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8250F7D0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8250F7D4: 81640004  lwz r11, 4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 8250F7D8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8250F7DC: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8250F7E0: 419A0024  beq cr6, 0x8250f804
	if ctx.cr[6].eq {
	pc = 0x8250F804; continue 'dispatch;
	}
	// 8250F7E4: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8250F7E8: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8250F7EC: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8250F7F0: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8250F7F4: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8250F7F8: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8250F7FC: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8250F800: 4082FFE8  bne 0x8250f7e8
	if !ctx.cr[0].eq {
	pc = 0x8250F7E8; continue 'dispatch;
	}
	// 8250F804: 38840008  addi r4, r4, 8
	ctx.r[4].s64 = ctx.r[4].s64 + 8;
	// 8250F808: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 8250F80C: 4807C705  bl 0x8258bf10
	ctx.lr = 0x8250F810;
	sub_8258BF10(ctx, base);
	// 8250F810: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8250F814: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8250F818: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8250F81C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8250F820: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8250F824: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250F828(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8250F828 size=140
    let mut pc: u32 = 0x8250F828;
    'dispatch: loop {
        match pc {
            0x8250F828 => {
    //   block [0x8250F828..0x8250F8B4)
	// 8250F828: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250F82C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8250F830: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8250F834: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250F838: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8250F83C: 39650004  addi r11, r5, 4
	ctx.r[11].s64 = ctx.r[5].s64 + 4;
	// 8250F840: 395F0004  addi r10, r31, 4
	ctx.r[10].s64 = ctx.r[31].s64 + 4;
	// 8250F844: 909F0000  stw r4, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 8250F848: 81250000  lwz r9, 0(r5)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250F84C: 913F0004  stw r9, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 8250F850: 81450004  lwz r10, 4(r5)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) } as u64;
	// 8250F854: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8250F858: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8250F85C: 419A0024  beq cr6, 0x8250f880
	if ctx.cr[6].eq {
	pc = 0x8250F880; continue 'dispatch;
	}
	// 8250F860: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 8250F864: 7D0000A6  mfmsr r8
	ctx.r[8].u64 = ctx.msr;
	// 8250F868: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8250F86C: 7D205028  lwarx r9, 0, r10
	// lwarx
	let ea = ctx.r[10].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[9].u64 = ctx.reserved.u32 as u64;
	// 8250F870: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 8250F874: 7D20512D  stwcx. r9, 0, r10
	// stwcx.
	let addr = ctx.r[10].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[9].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8250F878: 7D010164  mtmsrd r8, 1
	ctx.msr = (ctx.r[8].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8250F87C: 4082FFE8  bne 0x8250f864
	if !ctx.cr[0].eq {
	pc = 0x8250F864; continue 'dispatch;
	}
	// 8250F880: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8250F884: 98DF000C  stb r6, 0xc(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[6].u8 ) };
	// 8250F888: 995F000D  stb r10, 0xd(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(13 as u32), ctx.r[10].u8 ) };
	// 8250F88C: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250F890: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8250F894: 419A0008  beq cr6, 0x8250f89c
	if ctx.cr[6].eq {
	pc = 0x8250F89C; continue 'dispatch;
	}
	// 8250F898: 4BDB0FF9  bl 0x822c0890
	ctx.lr = 0x8250F89C;
	sub_822C0890(ctx, base);
	// 8250F89C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8250F8A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8250F8A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8250F8A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8250F8AC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8250F8B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250F8B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8250F8B8 size=112
    let mut pc: u32 = 0x8250F8B8;
    'dispatch: loop {
        match pc {
            0x8250F8B8 => {
    //   block [0x8250F8B8..0x8250F928)
	// 8250F8B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250F8BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8250F8C0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8250F8C4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250F8C8: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250F8CC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8250F8D0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8250F8D4: 81640004  lwz r11, 4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 8250F8D8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8250F8DC: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8250F8E0: 419A0024  beq cr6, 0x8250f904
	if ctx.cr[6].eq {
	pc = 0x8250F904; continue 'dispatch;
	}
	// 8250F8E4: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8250F8E8: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8250F8EC: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8250F8F0: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8250F8F4: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8250F8F8: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8250F8FC: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8250F900: 4082FFE8  bne 0x8250f8e8
	if !ctx.cr[0].eq {
	pc = 0x8250F8E8; continue 'dispatch;
	}
	// 8250F904: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 8250F908: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 8250F90C: 4807C605  bl 0x8258bf10
	ctx.lr = 0x8250F910;
	sub_8258BF10(ctx, base);
	// 8250F910: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8250F914: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8250F918: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8250F91C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8250F920: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8250F924: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250F928(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8250F928 size=176
    let mut pc: u32 = 0x8250F928;
    'dispatch: loop {
        match pc {
            0x8250F928 => {
    //   block [0x8250F928..0x8250F9D8)
	// 8250F928: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250F92C: 48C98839  bl 0x831a8164
	ctx.lr = 0x8250F930;
	sub_831A8130(ctx, base);
	// 8250F930: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250F934: 8964001C  lbz r11, 0x1c(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(28 as u32) ) } as u64;
	// 8250F938: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8250F93C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8250F940: 41820090  beq 0x8250f9d0
	if ctx.cr[0].eq {
	pc = 0x8250F9D0; continue 'dispatch;
	}
	// 8250F944: 81640018  lwz r11, 0x18(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(24 as u32) ) } as u64;
	// 8250F948: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 8250F94C: 409A0084  bne cr6, 0x8250f9d0
	if !ctx.cr[6].eq {
	pc = 0x8250F9D0; continue 'dispatch;
	}
	// 8250F950: 817E00BC  lwz r11, 0xbc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(188 as u32) ) } as u64;
	// 8250F954: 838B0034  lwz r28, 0x34(r11)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 8250F958: 83FC0000  lwz r31, 0(r28)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250F95C: 4800006C  b 0x8250f9c8
	pc = 0x8250F9C8; continue 'dispatch;
	// 8250F960: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8250F964: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8250F968: 3B7F0008  addi r27, r31, 8
	ctx.r[27].s64 = ctx.r[31].s64 + 8;
	// 8250F96C: 4BFFFB5D  bl 0x8250f4c8
	ctx.lr = 0x8250F970;
	sub_8250F4C8(ctx, base);
	// 8250F970: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250F974: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8250F978: 3BABFFFC  addi r29, r11, -4
	ctx.r[29].s64 = ctx.r[11].s64 + -4;
	// 8250F97C: 409A0008  bne cr6, 0x8250f984
	if !ctx.cr[6].eq {
	pc = 0x8250F984; continue 'dispatch;
	}
	// 8250F980: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 8250F984: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8250F988: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8250F98C: 4BDDE835  bl 0x822ee1c0
	ctx.lr = 0x8250F990;
	sub_822EE1C0(ctx, base);
	// 8250F990: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 8250F994: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8250F998: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8250F99C: 4BFFFB7D  bl 0x8250f518
	ctx.lr = 0x8250F9A0;
	sub_8250F518(ctx, base);
	// 8250F9A0: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8250F9A4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8250F9A8: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 8250F9AC: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 8250F9B0: 4BFFDB59  bl 0x8250d508
	ctx.lr = 0x8250F9B4;
	sub_8250D508(ctx, base);
	// 8250F9B4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8250F9B8: 488E22D9  bl 0x82df1c90
	ctx.lr = 0x8250F9BC;
	sub_82DF1C90(ctx, base);
	// 8250F9BC: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8250F9C0: 488E22D1  bl 0x82df1c90
	ctx.lr = 0x8250F9C4;
	sub_82DF1C90(ctx, base);
	// 8250F9C4: 83FF0000  lwz r31, 0(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250F9C8: 7F1FE040  cmplw cr6, r31, r28
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[28].u32, &mut ctx.xer);
	// 8250F9CC: 409AFF94  bne cr6, 0x8250f960
	if !ctx.cr[6].eq {
	pc = 0x8250F960; continue 'dispatch;
	}
	// 8250F9D0: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 8250F9D4: 48C987E0  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250F9D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8250F9D8 size=120
    let mut pc: u32 = 0x8250F9D8;
    'dispatch: loop {
        match pc {
            0x8250F9D8 => {
    //   block [0x8250F9D8..0x8250FA50)
	// 8250F9D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250F9DC: 48C98791  bl 0x831a816c
	ctx.lr = 0x8250F9E0;
	sub_831A8130(ctx, base);
	// 8250F9E0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250F9E4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8250F9E8: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8250F9EC: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8250F9F0: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8250F9F4: 388BA66C  addi r4, r11, -0x5994
	ctx.r[4].s64 = ctx.r[11].s64 + -22932;
	// 8250F9F8: 38A00089  li r5, 0x89
	ctx.r[5].s64 = 137;
	// 8250F9FC: 3860001C  li r3, 0x1c
	ctx.r[3].s64 = 28;
	// 8250FA00: 488E29E9  bl 0x82df23e8
	ctx.lr = 0x8250FA04;
	sub_82DF23E8(ctx, base);
	// 8250FA04: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8250FA08: 41820014  beq 0x8250fa1c
	if ctx.cr[0].eq {
	pc = 0x8250FA1C; continue 'dispatch;
	}
	// 8250FA0C: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250FA10: 48645D79  bl 0x82b55788
	ctx.lr = 0x8250FA14;
	sub_82B55788(ctx, base);
	// 8250FA14: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8250FA18: 48000008  b 0x8250fa20
	pc = 0x8250FA20; continue 'dispatch;
	// 8250FA1C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8250FA20: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 8250FA24: 3BDD0004  addi r30, r29, 4
	ctx.r[30].s64 = ctx.r[29].s64 + 4;
	// 8250FA28: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8250FA2C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8250FA30: 4BFFF9D1  bl 0x8250f400
	ctx.lr = 0x8250FA34;
	sub_8250F400(ctx, base);
	// 8250FA34: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8250FA38: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8250FA3C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8250FA40: 4BDB05C1  bl 0x822c0000
	ctx.lr = 0x8250FA44;
	sub_822C0000(ctx, base);
	// 8250FA44: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8250FA48: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8250FA4C: 48C98770  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250FA50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8250FA50 size=236
    let mut pc: u32 = 0x8250FA50;
    'dispatch: loop {
        match pc {
            0x8250FA50 => {
    //   block [0x8250FA50..0x8250FB3C)
	// 8250FA50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250FA54: 48C98715  bl 0x831a8168
	ctx.lr = 0x8250FA58;
	sub_831A8130(ctx, base);
	// 8250FA58: DBE1FFD0  stfd f31, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[31].u64 ) };
	// 8250FA5C: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250FA60: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 8250FA64: 908100BC  stw r4, 0xbc(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(188 as u32), ctx.r[4].u32 ) };
	// 8250FA68: 817C00BC  lwz r11, 0xbc(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(188 as u32) ) } as u64;
	// 8250FA6C: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 8250FA70: 83EB0000  lwz r31, 0(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250FA74: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8250FA78: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 8250FA7C: 419A00B4  beq cr6, 0x8250fb30
	if ctx.cr[6].eq {
	pc = 0x8250FB30; continue 'dispatch;
	}
	// 8250FA80: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 8250FA84: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8250FA88: 3BBC0028  addi r29, r28, 0x28
	ctx.r[29].s64 = ctx.r[28].s64 + 40;
	// 8250FA8C: 3BCB1B3C  addi r30, r11, 0x1b3c
	ctx.r[30].s64 = ctx.r[11].s64 + 6972;
	// 8250FA90: C3EA08A4  lfs f31, 0x8a4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2212 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 8250FA94: 388100BC  addi r4, r1, 0xbc
	ctx.r[4].s64 = ctx.r[1].s64 + 188;
	// 8250FA98: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8250FA9C: 4BFFFF3D  bl 0x8250f9d8
	ctx.lr = 0x8250FAA0;
	sub_8250F9D8(ctx, base);
	// 8250FAA0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250FAA4: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 8250FAA8: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8250FAAC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8250FAB0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8250FAB4: 419A0024  beq cr6, 0x8250fad8
	if ctx.cr[6].eq {
	pc = 0x8250FAD8; continue 'dispatch;
	}
	// 8250FAB8: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8250FABC: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8250FAC0: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8250FAC4: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8250FAC8: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8250FACC: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8250FAD0: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8250FAD4: 4082FFE8  bne 0x8250fabc
	if !ctx.cr[0].eq {
	pc = 0x8250FABC; continue 'dispatch;
	}
	// 8250FAD8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8250FADC: 80DF000C  lwz r6, 0xc(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8250FAE0: 38E10058  addi r7, r1, 0x58
	ctx.r[7].s64 = ctx.r[1].s64 + 88;
	// 8250FAE4: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 8250FAE8: 38A0032E  li r5, 0x32e
	ctx.r[5].s64 = 814;
	// 8250FAEC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8250FAF0: 48948F51  bl 0x82e58a40
	ctx.lr = 0x8250FAF4;
	sub_82E58A40(ctx, base);
	// 8250FAF4: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 8250FAF8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8250FAFC: 419A0008  beq cr6, 0x8250fb04
	if ctx.cr[6].eq {
	pc = 0x8250FB04; continue 'dispatch;
	}
	// 8250FB00: 4BDB0D91  bl 0x822c0890
	ctx.lr = 0x8250FB04;
	sub_822C0890(ctx, base);
	// 8250FB04: 80610064  lwz r3, 0x64(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 8250FB08: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8250FB0C: 419A0008  beq cr6, 0x8250fb14
	if ctx.cr[6].eq {
	pc = 0x8250FB14; continue 'dispatch;
	}
	// 8250FB10: 4BDB0D81  bl 0x822c0890
	ctx.lr = 0x8250FB14;
	sub_822C0890(ctx, base);
	// 8250FB14: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8250FB18: 4803EDD1  bl 0x8254e8e8
	ctx.lr = 0x8250FB1C;
	sub_8254E8E8(ctx, base);
	// 8250FB1C: 817C00BC  lwz r11, 0xbc(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(188 as u32) ) } as u64;
	// 8250FB20: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 8250FB24: 83E10050  lwz r31, 0x50(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8250FB28: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8250FB2C: 409AFF68  bne cr6, 0x8250fa94
	if !ctx.cr[6].eq {
	pc = 0x8250FA94; continue 'dispatch;
	}
	// 8250FB30: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 8250FB34: CBE1FFD0  lfd f31, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-48 as u32) ) };
	// 8250FB38: 48C98680  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250FB40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8250FB40 size=372
    let mut pc: u32 = 0x8250FB40;
    'dispatch: loop {
        match pc {
            0x8250FB40 => {
    //   block [0x8250FB40..0x8250FCB4)
	// 8250FB40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250FB44: 48C98629  bl 0x831a816c
	ctx.lr = 0x8250FB48;
	sub_831A8130(ctx, base);
	// 8250FB48: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250FB4C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8250FB50: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8250FB54: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8250FB58: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8250FB5C: 817F00BC  lwz r11, 0xbc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(188 as u32) ) } as u64;
	// 8250FB60: 9BAB0050  stb r29, 0x50(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(80 as u32), ctx.r[29].u8 ) };
	// 8250FB64: 4BFFF9B5  bl 0x8250f518
	ctx.lr = 0x8250FB68;
	sub_8250F518(ctx, base);
	// 8250FB68: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8250FB6C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8250FB70: 396AFF6C  addi r11, r10, -0x94
	ctx.r[11].s64 = ctx.r[10].s64 + -148;
	// 8250FB74: 409A0008  bne cr6, 0x8250fb7c
	if !ctx.cr[6].eq {
	pc = 0x8250FB7C; continue 'dispatch;
	}
	// 8250FB78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8250FB7C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8250FB80: 419A0124  beq cr6, 0x8250fca4
	if ctx.cr[6].eq {
	pc = 0x8250FCA4; continue 'dispatch;
	}
	// 8250FB84: 817F00BC  lwz r11, 0xbc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(188 as u32) ) } as u64;
	// 8250FB88: 83CB0010  lwz r30, 0x10(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 8250FB8C: 83FE0000  lwz r31, 0(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250FB90: 7F1FF040  cmplw cr6, r31, r30
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[30].u32, &mut ctx.xer);
	// 8250FB94: 419A0110  beq cr6, 0x8250fca4
	if ctx.cr[6].eq {
	pc = 0x8250FCA4; continue 'dispatch;
	}
	// 8250FB98: 48000008  b 0x8250fba0
	pc = 0x8250FBA0; continue 'dispatch;
	// 8250FB9C: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8250FBA0: 897F0014  lbz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 8250FBA4: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8250FBA8: 418200E0  beq 0x8250fc88
	if ctx.cr[0].eq {
	pc = 0x8250FC88; continue 'dispatch;
	}
	// 8250FBAC: 893F0015  lbz r9, 0x15(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(21 as u32) ) } as u64;
	// 8250FBB0: 57AB063E  clrlwi r11, r29, 0x18
	ctx.r[11].u64 = ctx.r[29].u32 as u64 & 0x000000FFu64;
	// 8250FBB4: 7F095840  cmplw cr6, r9, r11
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8250FBB8: 419A00D0  beq cr6, 0x8250fc88
	if ctx.cr[6].eq {
	pc = 0x8250FC88; continue 'dispatch;
	}
	// 8250FBBC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8250FBC0: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8250FBC4: 419A0060  beq cr6, 0x8250fc24
	if ctx.cr[6].eq {
	pc = 0x8250FC24; continue 'dispatch;
	}
	// 8250FBC8: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 8250FBCC: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8250FBD0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8250FBD4: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8250FBD8: 419A0028  beq cr6, 0x8250fc00
	if ctx.cr[6].eq {
	pc = 0x8250FC00; continue 'dispatch;
	}
	// 8250FBDC: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8250FBE0: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8250FBE4: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8250FBE8: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8250FBEC: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8250FBF0: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8250FBF4: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8250FBF8: 4082FFE8  bne 0x8250fbe0
	if !ctx.cr[0].eq {
	pc = 0x8250FBE0; continue 'dispatch;
	}
	// 8250FBFC: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8250FC00: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8250FC04: 386AFF6C  addi r3, r10, -0x94
	ctx.r[3].s64 = ctx.r[10].s64 + -148;
	// 8250FC08: 409A0008  bne cr6, 0x8250fc10
	if !ctx.cr[6].eq {
	pc = 0x8250FC10; continue 'dispatch;
	}
	// 8250FC0C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8250FC10: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 8250FC14: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250FC18: 4801A251  bl 0x82529e68
	ctx.lr = 0x8250FC1C;
	sub_82529E68(ctx, base);
	// 8250FC1C: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 8250FC20: 4800005C  b 0x8250fc7c
	pc = 0x8250FC7C; continue 'dispatch;
	// 8250FC24: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 8250FC28: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8250FC2C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8250FC30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8250FC34: 419A0028  beq cr6, 0x8250fc5c
	if ctx.cr[6].eq {
	pc = 0x8250FC5C; continue 'dispatch;
	}
	// 8250FC38: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8250FC3C: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8250FC40: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8250FC44: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8250FC48: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8250FC4C: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8250FC50: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8250FC54: 4082FFE8  bne 0x8250fc3c
	if !ctx.cr[0].eq {
	pc = 0x8250FC3C; continue 'dispatch;
	}
	// 8250FC58: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8250FC5C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8250FC60: 386AFF6C  addi r3, r10, -0x94
	ctx.r[3].s64 = ctx.r[10].s64 + -148;
	// 8250FC64: 409A0008  bne cr6, 0x8250fc6c
	if !ctx.cr[6].eq {
	pc = 0x8250FC6C; continue 'dispatch;
	}
	// 8250FC68: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8250FC6C: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 8250FC70: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250FC74: 48019005  bl 0x82528c78
	ctx.lr = 0x8250FC78;
	sub_82528C78(ctx, base);
	// 8250FC78: 80610064  lwz r3, 0x64(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 8250FC7C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8250FC80: 419A0008  beq cr6, 0x8250fc88
	if ctx.cr[6].eq {
	pc = 0x8250FC88; continue 'dispatch;
	}
	// 8250FC84: 4BDB0C0D  bl 0x822c0890
	ctx.lr = 0x8250FC88;
	sub_822C0890(ctx, base);
	// 8250FC88: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8250FC8C: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8250FC90: 48902291  bl 0x82e11f20
	ctx.lr = 0x8250FC94;
	sub_82E11F20(ctx, base);
	// 8250FC94: 9BBF0015  stb r29, 0x15(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(21 as u32), ctx.r[29].u8 ) };
	// 8250FC98: 83FF0000  lwz r31, 0(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250FC9C: 7F1FF040  cmplw cr6, r31, r30
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[30].u32, &mut ctx.xer);
	// 8250FCA0: 409AFEFC  bne cr6, 0x8250fb9c
	if !ctx.cr[6].eq {
	pc = 0x8250FB9C; continue 'dispatch;
	}
	// 8250FCA4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8250FCA8: 488E1FE9  bl 0x82df1c90
	ctx.lr = 0x8250FCAC;
	sub_82DF1C90(ctx, base);
	// 8250FCAC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8250FCB0: 48C9850C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250FCB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8250FCB8 size=156
    let mut pc: u32 = 0x8250FCB8;
    'dispatch: loop {
        match pc {
            0x8250FCB8 => {
    //   block [0x8250FCB8..0x8250FD54)
	// 8250FCB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250FCBC: 48C984A9  bl 0x831a8164
	ctx.lr = 0x8250FCC0;
	sub_831A8130(ctx, base);
	// 8250FCC0: DBE1FFC8  stfd f31, -0x38(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-56 as u32), ctx.f[31].u64 ) };
	// 8250FCC4: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250FCC8: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8250FCCC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8250FCD0: 897D0018  lbz r11, 0x18(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(24 as u32) ) } as u64;
	// 8250FCD4: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 8250FCD8: 5564DFFE  rlwinm r4, r11, 0x1b, 0x1f, 0x1f
	ctx.r[4].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8250FCDC: 4BFFFE65  bl 0x8250fb40
	ctx.lr = 0x8250FCE0;
	sub_8250FB40(ctx, base);
	// 8250FCE0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8250FCE4: 889D0018  lbz r4, 0x18(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(24 as u32) ) } as u64;
	// 8250FCE8: 489466E1  bl 0x82e563c8
	ctx.lr = 0x8250FCEC;
	sub_82E563C8(ctx, base);
	// 8250FCEC: 817E00BC  lwz r11, 0xbc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(188 as u32) ) } as u64;
	// 8250FCF0: 838B001C  lwz r28, 0x1c(r11)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 8250FCF4: 83FC0000  lwz r31, 0(r28)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250FCF8: 7F1FE040  cmplw cr6, r31, r28
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[28].u32, &mut ctx.xer);
	// 8250FCFC: 419A004C  beq cr6, 0x8250fd48
	if ctx.cr[6].eq {
	pc = 0x8250FD48; continue 'dispatch;
	}
	// 8250FD00: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 8250FD04: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8250FD08: 3B7E0028  addi r27, r30, 0x28
	ctx.r[27].s64 = ctx.r[30].s64 + 40;
	// 8250FD0C: 3BCB1B3C  addi r30, r11, 0x1b3c
	ctx.r[30].s64 = ctx.r[11].s64 + 6972;
	// 8250FD10: C3EA08A4  lfs f31, 0x8a4(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2212 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 8250FD14: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250FD18: 386B0028  addi r3, r11, 0x28
	ctx.r[3].s64 = ctx.r[11].s64 + 40;
	// 8250FD1C: 48AF929D  bl 0x83008fb8
	ctx.lr = 0x8250FD20;
	sub_83008FB8(ctx, base);
	// 8250FD20: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 8250FD24: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8250FD28: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 8250FD2C: 38A003E3  li r5, 0x3e3
	ctx.r[5].s64 = 995;
	// 8250FD30: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 8250FD34: 7FA8EB78  mr r8, r29
	ctx.r[8].u64 = ctx.r[29].u64;
	// 8250FD38: 48948F69  bl 0x82e58ca0
	ctx.lr = 0x8250FD3C;
	sub_82E58CA0(ctx, base);
	// 8250FD3C: 83FF0000  lwz r31, 0(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250FD40: 7F1FE040  cmplw cr6, r31, r28
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[28].u32, &mut ctx.xer);
	// 8250FD44: 409AFFD0  bne cr6, 0x8250fd14
	if !ctx.cr[6].eq {
	pc = 0x8250FD14; continue 'dispatch;
	}
	// 8250FD48: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8250FD4C: CBE1FFC8  lfd f31, -0x38(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-56 as u32) ) };
	// 8250FD50: 48C98464  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250FD58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8250FD58 size=284
    let mut pc: u32 = 0x8250FD58;
    'dispatch: loop {
        match pc {
            0x8250FD58 => {
    //   block [0x8250FD58..0x8250FE74)
	// 8250FD58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250FD5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8250FD60: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8250FD64: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8250FD68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250FD6C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8250FD70: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8250FD74: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8250FD78: 4BFFF7A1  bl 0x8250f518
	ctx.lr = 0x8250FD7C;
	sub_8250F518(ctx, base);
	// 8250FD7C: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8250FD80: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8250FD84: 396AFF6C  addi r11, r10, -0x94
	ctx.r[11].s64 = ctx.r[10].s64 + -148;
	// 8250FD88: 409A0008  bne cr6, 0x8250fd90
	if !ctx.cr[6].eq {
	pc = 0x8250FD90; continue 'dispatch;
	}
	// 8250FD8C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8250FD90: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8250FD94: 419A00C0  beq cr6, 0x8250fe54
	if ctx.cr[6].eq {
	pc = 0x8250FE54; continue 'dispatch;
	}
	// 8250FD98: 817F00BC  lwz r11, 0xbc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(188 as u32) ) } as u64;
	// 8250FD9C: 83CB0010  lwz r30, 0x10(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 8250FDA0: 83FE0000  lwz r31, 0(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250FDA4: 7F1FF040  cmplw cr6, r31, r30
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[30].u32, &mut ctx.xer);
	// 8250FDA8: 419A00AC  beq cr6, 0x8250fe54
	if ctx.cr[6].eq {
	pc = 0x8250FE54; continue 'dispatch;
	}
	// 8250FDAC: 48000008  b 0x8250fdb4
	pc = 0x8250FDB4; continue 'dispatch;
	// 8250FDB0: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8250FDB4: 897F0014  lbz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 8250FDB8: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8250FDBC: 41820070  beq 0x8250fe2c
	if ctx.cr[0].eq {
	pc = 0x8250FE2C; continue 'dispatch;
	}
	// 8250FDC0: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8250FDC4: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 8250FDC8: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8250FDCC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8250FDD0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8250FDD4: 419A0028  beq cr6, 0x8250fdfc
	if ctx.cr[6].eq {
	pc = 0x8250FDFC; continue 'dispatch;
	}
	// 8250FDD8: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8250FDDC: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8250FDE0: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8250FDE4: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8250FDE8: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8250FDEC: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8250FDF0: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8250FDF4: 4082FFE8  bne 0x8250fddc
	if !ctx.cr[0].eq {
	pc = 0x8250FDDC; continue 'dispatch;
	}
	// 8250FDF8: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8250FDFC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8250FE00: 386AFF6C  addi r3, r10, -0x94
	ctx.r[3].s64 = ctx.r[10].s64 + -148;
	// 8250FE04: 409A0008  bne cr6, 0x8250fe0c
	if !ctx.cr[6].eq {
	pc = 0x8250FE0C; continue 'dispatch;
	}
	// 8250FE08: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8250FE0C: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 8250FE10: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250FE14: 4801A055  bl 0x82529e68
	ctx.lr = 0x8250FE18;
	sub_82529E68(ctx, base);
	// 8250FE18: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 8250FE1C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8250FE20: 419A0028  beq cr6, 0x8250fe48
	if ctx.cr[6].eq {
	pc = 0x8250FE48; continue 'dispatch;
	}
	// 8250FE24: 4BDB0A6D  bl 0x822c0890
	ctx.lr = 0x8250FE28;
	sub_822C0890(ctx, base);
	// 8250FE28: 48000020  b 0x8250fe48
	pc = 0x8250FE48; continue 'dispatch;
	// 8250FE2C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8250FE30: 386AFF6C  addi r3, r10, -0x94
	ctx.r[3].s64 = ctx.r[10].s64 + -148;
	// 8250FE34: 409A0008  bne cr6, 0x8250fe3c
	if !ctx.cr[6].eq {
	pc = 0x8250FE3C; continue 'dispatch;
	}
	// 8250FE38: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8250FE3C: 38BF000C  addi r5, r31, 0xc
	ctx.r[5].s64 = ctx.r[31].s64 + 12;
	// 8250FE40: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250FE44: 48019D45  bl 0x82529b88
	ctx.lr = 0x8250FE48;
	sub_82529B88(ctx, base);
	// 8250FE48: 83FF0000  lwz r31, 0(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250FE4C: 7F1FF040  cmplw cr6, r31, r30
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[30].u32, &mut ctx.xer);
	// 8250FE50: 409AFF60  bne cr6, 0x8250fdb0
	if !ctx.cr[6].eq {
	pc = 0x8250FDB0; continue 'dispatch;
	}
	// 8250FE54: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8250FE58: 488E1E39  bl 0x82df1c90
	ctx.lr = 0x8250FE5C;
	sub_82DF1C90(ctx, base);
	// 8250FE5C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8250FE60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8250FE64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8250FE68: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8250FE6C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8250FE70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250FE78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8250FE78 size=8
    let mut pc: u32 = 0x8250FE78;
    'dispatch: loop {
        match pc {
            0x8250FE78 => {
    //   block [0x8250FE78..0x8250FE80)
	// 8250FE78: 88840018  lbz r4, 0x18(r4)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(24 as u32) ) } as u64;
	// 8250FE7C: 4BFFFCC4  b 0x8250fb40
	sub_8250FB40(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250FE80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8250FE80 size=72
    let mut pc: u32 = 0x8250FE80;
    'dispatch: loop {
        match pc {
            0x8250FE80 => {
    //   block [0x8250FE80..0x8250FEC8)
	// 8250FE80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250FE84: 48C982E9  bl 0x831a816c
	ctx.lr = 0x8250FE88;
	sub_831A8130(ctx, base);
	// 8250FE88: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250FE8C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8250FE90: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8250FE94: 4BFFFCAD  bl 0x8250fb40
	ctx.lr = 0x8250FE98;
	sub_8250FB40(ctx, base);
	// 8250FE98: 817F00BC  lwz r11, 0xbc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(188 as u32) ) } as u64;
	// 8250FE9C: 83CB001C  lwz r30, 0x1c(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 8250FEA0: 83FE0000  lwz r31, 0(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250FEA4: 48000014  b 0x8250feb8
	pc = 0x8250FEB8; continue 'dispatch;
	// 8250FEA8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8250FEAC: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8250FEB0: 4BFFFFD1  bl 0x8250fe80
	ctx.lr = 0x8250FEB4;
	sub_8250FE80(ctx, base);
	// 8250FEB4: 83FF0000  lwz r31, 0(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250FEB8: 7F1FF040  cmplw cr6, r31, r30
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[30].u32, &mut ctx.xer);
	// 8250FEBC: 409AFFEC  bne cr6, 0x8250fea8
	if !ctx.cr[6].eq {
	pc = 0x8250FEA8; continue 'dispatch;
	}
	// 8250FEC0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8250FEC4: 48C982F8  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250FEC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8250FEC8 size=100
    let mut pc: u32 = 0x8250FEC8;
    'dispatch: loop {
        match pc {
            0x8250FEC8 => {
    //   block [0x8250FEC8..0x8250FF2C)
	// 8250FEC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250FECC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8250FED0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8250FED4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8250FED8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250FEDC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8250FEE0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8250FEE4: 397F0008  addi r11, r31, 8
	ctx.r[11].s64 = ctx.r[31].s64 + 8;
	// 8250FEE8: 386B0008  addi r3, r11, 8
	ctx.r[3].s64 = ctx.r[11].s64 + 8;
	// 8250FEEC: 4BDB8DCD  bl 0x822c8cb8
	ctx.lr = 0x8250FEF0;
	sub_822C8CB8(ctx, base);
	// 8250FEF0: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8250FEF4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8250FEF8: 419A0008  beq cr6, 0x8250ff00
	if ctx.cr[6].eq {
	pc = 0x8250FF00; continue 'dispatch;
	}
	// 8250FEFC: 4BDB0995  bl 0x822c0890
	ctx.lr = 0x8250FF00;
	sub_822C0890(ctx, base);
	// 8250FF00: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8250FF04: 4182000C  beq 0x8250ff10
	if ctx.cr[0].eq {
	pc = 0x8250FF10; continue 'dispatch;
	}
	// 8250FF08: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8250FF0C: 4BDB035D  bl 0x822c0268
	ctx.lr = 0x8250FF10;
	sub_822C0268(ctx, base);
	// 8250FF10: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8250FF14: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8250FF18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8250FF1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8250FF20: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8250FF24: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8250FF28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250FF30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8250FF30 size=76
    let mut pc: u32 = 0x8250FF30;
    'dispatch: loop {
        match pc {
            0x8250FF30 => {
    //   block [0x8250FF30..0x8250FF7C)
	// 8250FF30: 816300BC  lwz r11, 0xbc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(188 as u32) ) } as u64;
	// 8250FF34: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8250FF38: 3920FFFF  li r9, -1
	ctx.r[9].s64 = -1;
	// 8250FF3C: 994B0008  stb r10, 8(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u8 ) };
	// 8250FF40: 816300BC  lwz r11, 0xbc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(188 as u32) ) } as u64;
	// 8250FF44: 912B0074  stw r9, 0x74(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8250FF48: 816300BC  lwz r11, 0xbc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(188 as u32) ) } as u64;
	// 8250FF4C: 90AB0000  stw r5, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[5].u32 ) };
	// 8250FF50: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250FF54: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8250FF58: 814300BC  lwz r10, 0xbc(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(188 as u32) ) } as u64;
	// 8250FF5C: 396BFF6C  addi r11, r11, -0x94
	ctx.r[11].s64 = ctx.r[11].s64 + -148;
	// 8250FF60: 409A0008  bne cr6, 0x8250ff68
	if !ctx.cr[6].eq {
	pc = 0x8250FF68; continue 'dispatch;
	}
	// 8250FF64: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8250FF68: 916A0004  stw r11, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8250FF6C: 81660000  lwz r11, 0(r6)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 8250FF70: 814300BC  lwz r10, 0xbc(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(188 as u32) ) } as u64;
	// 8250FF74: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 8250FF78: 4BFFFDE0  b 0x8250fd58
	sub_8250FD58(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250FF80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8250FF80 size=104
    let mut pc: u32 = 0x8250FF80;
    'dispatch: loop {
        match pc {
            0x8250FF80 => {
    //   block [0x8250FF80..0x8250FFE8)
	// 8250FF80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250FF84: 48C981E5  bl 0x831a8168
	ctx.lr = 0x8250FF88;
	sub_831A8130(ctx, base);
	// 8250FF88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250FF8C: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 8250FF90: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 8250FF94: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8250FF98: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8250FF9C: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 8250FFA0: 38C00018  li r6, 0x18
	ctx.r[6].s64 = 24;
	// 8250FFA4: 806B110C  lwz r3, 0x110c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4364 as u32) ) } as u64;
	// 8250FFA8: 388A08B0  addi r4, r10, 0x8b0
	ctx.r[4].s64 = ctx.r[10].s64 + 2224;
	// 8250FFAC: 38A0002D  li r5, 0x2d
	ctx.r[5].s64 = 45;
	// 8250FFB0: 488E2119  bl 0x82df20c8
	ctx.lr = 0x8250FFB4;
	sub_82DF20C8(ctx, base);
	// 8250FFB4: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 8250FFB8: 41820008  beq 0x8250ffc0
	if ctx.cr[0].eq {
	pc = 0x8250FFC0; continue 'dispatch;
	}
	// 8250FFBC: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 8250FFC0: 357F0004  addic. r11, r31, 4
	ctx.xer.ca = (ctx.r[31].u32 > (!(4 as u32)));
	ctx.r[11].s64 = ctx.r[31].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8250FFC4: 41820008  beq 0x8250ffcc
	if ctx.cr[0].eq {
	pc = 0x8250FFCC; continue 'dispatch;
	}
	// 8250FFC8: 93AB0000  stw r29, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 8250FFCC: 347F0008  addic. r3, r31, 8
	ctx.xer.ca = (ctx.r[31].u32 > (!(8 as u32)));
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	ctx.cr[0].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8250FFD0: 4182000C  beq 0x8250ffdc
	if ctx.cr[0].eq {
	pc = 0x8250FFDC; continue 'dispatch;
	}
	// 8250FFD4: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8250FFD8: 4BFFF789  bl 0x8250f760
	ctx.lr = 0x8250FFDC;
	sub_8250F760(ctx, base);
	// 8250FFDC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8250FFE0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8250FFE4: 48C981D4  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8250FFE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8250FFE8 size=104
    let mut pc: u32 = 0x8250FFE8;
    'dispatch: loop {
        match pc {
            0x8250FFE8 => {
    //   block [0x8250FFE8..0x82510050)
	// 8250FFE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8250FFEC: 48C9817D  bl 0x831a8168
	ctx.lr = 0x8250FFF0;
	sub_831A8130(ctx, base);
	// 8250FFF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8250FFF4: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 8250FFF8: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 8250FFFC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82510000: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82510004: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82510008: 38C00030  li r6, 0x30
	ctx.r[6].s64 = 48;
	// 8251000C: 806B110C  lwz r3, 0x110c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4364 as u32) ) } as u64;
	// 82510010: 388A08B0  addi r4, r10, 0x8b0
	ctx.r[4].s64 = ctx.r[10].s64 + 2224;
	// 82510014: 38A0002D  li r5, 0x2d
	ctx.r[5].s64 = 45;
	// 82510018: 488E20B1  bl 0x82df20c8
	ctx.lr = 0x8251001C;
	sub_82DF20C8(ctx, base);
	// 8251001C: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82510020: 41820008  beq 0x82510028
	if ctx.cr[0].eq {
	pc = 0x82510028; continue 'dispatch;
	}
	// 82510024: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82510028: 357F0004  addic. r11, r31, 4
	ctx.xer.ca = (ctx.r[31].u32 > (!(4 as u32)));
	ctx.r[11].s64 = ctx.r[31].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8251002C: 41820008  beq 0x82510034
	if ctx.cr[0].eq {
	pc = 0x82510034; continue 'dispatch;
	}
	// 82510030: 93AB0000  stw r29, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82510034: 347F0008  addic. r3, r31, 8
	ctx.xer.ca = (ctx.r[31].u32 > (!(8 as u32)));
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	ctx.cr[0].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82510038: 4182000C  beq 0x82510044
	if ctx.cr[0].eq {
	pc = 0x82510044; continue 'dispatch;
	}
	// 8251003C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82510040: 4BFFF779  bl 0x8250f7b8
	ctx.lr = 0x82510044;
	sub_8250F7B8(ctx, base);
	// 82510044: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82510048: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8251004C: 48C9816C  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82510050(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82510050 size=112
    let mut pc: u32 = 0x82510050;
    'dispatch: loop {
        match pc {
            0x82510050 => {
    //   block [0x82510050..0x825100C0)
	// 82510050: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82510054: 48C98115  bl 0x831a8168
	ctx.lr = 0x82510058;
	sub_831A8130(ctx, base);
	// 82510058: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251005C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82510060: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82510064: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82510068: 83EB0000  lwz r31, 0(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251006C: 916B0000  stw r11, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82510070: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82510074: 916B0004  stw r11, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82510078: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 8251007C: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82510080: 915D0008  stw r10, 8(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82510084: 419A0034  beq cr6, 0x825100b8
	if ctx.cr[6].eq {
	pc = 0x825100B8; continue 'dispatch;
	}
	// 82510088: 3F808335  lis r28, -0x7ccb
	ctx.r[28].s64 = -2093678592;
	// 8251008C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82510090: 83DF0000  lwz r30, 0(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82510094: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82510098: 4BFFFE31  bl 0x8250fec8
	ctx.lr = 0x8251009C;
	sub_8250FEC8(ctx, base);
	// 8251009C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825100A0: 807C110C  lwz r3, 0x110c(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4364 as u32) ) } as u64;
	// 825100A4: 488E20E5  bl 0x82df2188
	ctx.lr = 0x825100A8;
	sub_82DF2188(ctx, base);
	// 825100A8: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 825100AC: 7FDFF378  mr r31, r30
	ctx.r[31].u64 = ctx.r[30].u64;
	// 825100B0: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 825100B4: 409AFFD8  bne cr6, 0x8251008c
	if !ctx.cr[6].eq {
	pc = 0x8251008C; continue 'dispatch;
	}
	// 825100B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825100BC: 48C980FC  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825100C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825100C0 size=300
    let mut pc: u32 = 0x825100C0;
    'dispatch: loop {
        match pc {
            0x825100C0 => {
    //   block [0x825100C0..0x825101EC)
	// 825100C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825100C4: 48C9809D  bl 0x831a8160
	ctx.lr = 0x825100C8;
	sub_831A8130(ctx, base);
	// 825100C8: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825100CC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 825100D0: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 825100D4: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 825100D8: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 825100DC: 817E00BC  lwz r11, 0xbc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(188 as u32) ) } as u64;
	// 825100E0: 838B0010  lwz r28, 0x10(r11)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 825100E4: 83FC0000  lwz r31, 0(r28)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 825100E8: 48000098  b 0x82510180
	pc = 0x82510180; continue 'dispatch;
	// 825100EC: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 825100F0: 7F0BE840  cmplw cr6, r11, r29
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[29].u32, &mut ctx.xer);
	// 825100F4: 409A0088  bne cr6, 0x8251017c
	if !ctx.cr[6].eq {
	pc = 0x8251017C; continue 'dispatch;
	}
	// 825100F8: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 825100FC: 815B0000  lwz r10, 0(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82510100: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82510104: 409A0078  bne cr6, 0x8251017c
	if !ctx.cr[6].eq {
	pc = 0x8251017C; continue 'dispatch;
	}
	// 82510108: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8251010C: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82510110: 4BFFF409  bl 0x8250f518
	ctx.lr = 0x82510114;
	sub_8250F518(ctx, base);
	// 82510114: 81410058  lwz r10, 0x58(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82510118: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8251011C: 396AFF6C  addi r11, r10, -0x94
	ctx.r[11].s64 = ctx.r[10].s64 + -148;
	// 82510120: 409A0008  bne cr6, 0x82510128
	if !ctx.cr[6].eq {
	pc = 0x82510128; continue 'dispatch;
	}
	// 82510124: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82510128: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8251012C: 419A0040  beq cr6, 0x8251016c
	if ctx.cr[6].eq {
	pc = 0x8251016C; continue 'dispatch;
	}
	// 82510130: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82510134: 386AFF6C  addi r3, r10, -0x94
	ctx.r[3].s64 = ctx.r[10].s64 + -148;
	// 82510138: 409A0008  bne cr6, 0x82510140
	if !ctx.cr[6].eq {
	pc = 0x82510140; continue 'dispatch;
	}
	// 8251013C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82510140: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82510144: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82510148: 48018AC9  bl 0x82528c10
	ctx.lr = 0x8251014C;
	sub_82528C10(ctx, base);
	// 8251014C: 817E00BC  lwz r11, 0xbc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(188 as u32) ) } as u64;
	// 82510150: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82510154: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82510158: 388B000C  addi r4, r11, 0xc
	ctx.r[4].s64 = ctx.r[11].s64 + 12;
	// 8251015C: 4829C4AD  bl 0x827ac608
	ctx.lr = 0x82510160;
	sub_827AC608(ctx, base);
	// 82510160: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82510164: 83EB0000  lwz r31, 0(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82510168: 48000008  b 0x82510170
	pc = 0x82510170; continue 'dispatch;
	// 8251016C: 83FF0000  lwz r31, 0(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82510170: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82510174: 488E1B1D  bl 0x82df1c90
	ctx.lr = 0x82510178;
	sub_82DF1C90(ctx, base);
	// 82510178: 48000008  b 0x82510180
	pc = 0x82510180; continue 'dispatch;
	// 8251017C: 83FF0000  lwz r31, 0(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82510180: 7F1FE040  cmplw cr6, r31, r28
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[28].u32, &mut ctx.xer);
	// 82510184: 409AFF68  bne cr6, 0x825100ec
	if !ctx.cr[6].eq {
	pc = 0x825100EC; continue 'dispatch;
	}
	// 82510188: 574B063F  clrlwi. r11, r26, 0x18
	ctx.r[11].u64 = ctx.r[26].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8251018C: 41820058  beq 0x825101e4
	if ctx.cr[0].eq {
	pc = 0x825101E4; continue 'dispatch;
	}
	// 82510190: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 82510194: 816B7078  lwz r11, 0x7078(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28792 as u32) ) } as u64;
	// 82510198: 7F1D5840  cmplw cr6, r29, r11
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8251019C: 409A0010  bne cr6, 0x825101ac
	if !ctx.cr[6].eq {
	pc = 0x825101AC; continue 'dispatch;
	}
	// 825101A0: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 825101A4: 808B70B8  lwz r4, 0x70b8(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28856 as u32) ) } as u64;
	// 825101A8: 4800002C  b 0x825101d4
	pc = 0x825101D4; continue 'dispatch;
	// 825101AC: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 825101B0: 816B7058  lwz r11, 0x7058(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28760 as u32) ) } as u64;
	// 825101B4: 7F1D5840  cmplw cr6, r29, r11
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[11].u32, &mut ctx.xer);
	// 825101B8: 419A0014  beq cr6, 0x825101cc
	if ctx.cr[6].eq {
	pc = 0x825101CC; continue 'dispatch;
	}
	// 825101BC: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 825101C0: 816B7074  lwz r11, 0x7074(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28788 as u32) ) } as u64;
	// 825101C4: 7F1D5840  cmplw cr6, r29, r11
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[11].u32, &mut ctx.xer);
	// 825101C8: 409A001C  bne cr6, 0x825101e4
	if !ctx.cr[6].eq {
	pc = 0x825101E4; continue 'dispatch;
	}
	// 825101CC: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 825101D0: 808B70B4  lwz r4, 0x70b4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28852 as u32) ) } as u64;
	// 825101D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825101D8: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 825101DC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825101E0: 4BFFFEE1  bl 0x825100c0
	ctx.lr = 0x825101E4;
	sub_825100C0(ctx, base);
	// 825101E4: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 825101E8: 48C97FC8  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825101F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825101F0 size=300
    let mut pc: u32 = 0x825101F0;
    'dispatch: loop {
        match pc {
            0x825101F0 => {
    //   block [0x825101F0..0x8251031C)
	// 825101F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825101F4: 48C97F6D  bl 0x831a8160
	ctx.lr = 0x825101F8;
	sub_831A8130(ctx, base);
	// 825101F8: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825101FC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82510200: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82510204: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82510208: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 8251020C: 817E00BC  lwz r11, 0xbc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(188 as u32) ) } as u64;
	// 82510210: 838B0010  lwz r28, 0x10(r11)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82510214: 83FC0000  lwz r31, 0(r28)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82510218: 48000098  b 0x825102b0
	pc = 0x825102B0; continue 'dispatch;
	// 8251021C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82510220: 7F0BE840  cmplw cr6, r11, r29
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[29].u32, &mut ctx.xer);
	// 82510224: 409A0088  bne cr6, 0x825102ac
	if !ctx.cr[6].eq {
	pc = 0x825102AC; continue 'dispatch;
	}
	// 82510228: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8251022C: 815B0000  lwz r10, 0(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82510230: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82510234: 409A0078  bne cr6, 0x825102ac
	if !ctx.cr[6].eq {
	pc = 0x825102AC; continue 'dispatch;
	}
	// 82510238: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8251023C: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82510240: 4BFFF2D9  bl 0x8250f518
	ctx.lr = 0x82510244;
	sub_8250F518(ctx, base);
	// 82510244: 81410058  lwz r10, 0x58(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82510248: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8251024C: 396AFF6C  addi r11, r10, -0x94
	ctx.r[11].s64 = ctx.r[10].s64 + -148;
	// 82510250: 409A0008  bne cr6, 0x82510258
	if !ctx.cr[6].eq {
	pc = 0x82510258; continue 'dispatch;
	}
	// 82510254: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82510258: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8251025C: 419A0040  beq cr6, 0x8251029c
	if ctx.cr[6].eq {
	pc = 0x8251029C; continue 'dispatch;
	}
	// 82510260: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82510264: 386AFF6C  addi r3, r10, -0x94
	ctx.r[3].s64 = ctx.r[10].s64 + -148;
	// 82510268: 409A0008  bne cr6, 0x82510270
	if !ctx.cr[6].eq {
	pc = 0x82510270; continue 'dispatch;
	}
	// 8251026C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82510270: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82510274: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82510278: 48018A01  bl 0x82528c78
	ctx.lr = 0x8251027C;
	sub_82528C78(ctx, base);
	// 8251027C: 817E00BC  lwz r11, 0xbc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(188 as u32) ) } as u64;
	// 82510280: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82510284: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82510288: 388B000C  addi r4, r11, 0xc
	ctx.r[4].s64 = ctx.r[11].s64 + 12;
	// 8251028C: 4829C37D  bl 0x827ac608
	ctx.lr = 0x82510290;
	sub_827AC608(ctx, base);
	// 82510290: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82510294: 83EB0000  lwz r31, 0(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82510298: 48000008  b 0x825102a0
	pc = 0x825102A0; continue 'dispatch;
	// 8251029C: 83FF0000  lwz r31, 0(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825102A0: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 825102A4: 488E19ED  bl 0x82df1c90
	ctx.lr = 0x825102A8;
	sub_82DF1C90(ctx, base);
	// 825102A8: 48000008  b 0x825102b0
	pc = 0x825102B0; continue 'dispatch;
	// 825102AC: 83FF0000  lwz r31, 0(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825102B0: 7F1FE040  cmplw cr6, r31, r28
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[28].u32, &mut ctx.xer);
	// 825102B4: 409AFF68  bne cr6, 0x8251021c
	if !ctx.cr[6].eq {
	pc = 0x8251021C; continue 'dispatch;
	}
	// 825102B8: 574B063F  clrlwi. r11, r26, 0x18
	ctx.r[11].u64 = ctx.r[26].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825102BC: 41820058  beq 0x82510314
	if ctx.cr[0].eq {
	pc = 0x82510314; continue 'dispatch;
	}
	// 825102C0: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 825102C4: 816B7078  lwz r11, 0x7078(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28792 as u32) ) } as u64;
	// 825102C8: 7F1D5840  cmplw cr6, r29, r11
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[11].u32, &mut ctx.xer);
	// 825102CC: 409A0010  bne cr6, 0x825102dc
	if !ctx.cr[6].eq {
	pc = 0x825102DC; continue 'dispatch;
	}
	// 825102D0: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 825102D4: 808B70B8  lwz r4, 0x70b8(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28856 as u32) ) } as u64;
	// 825102D8: 4800002C  b 0x82510304
	pc = 0x82510304; continue 'dispatch;
	// 825102DC: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 825102E0: 816B7058  lwz r11, 0x7058(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28760 as u32) ) } as u64;
	// 825102E4: 7F1D5840  cmplw cr6, r29, r11
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[11].u32, &mut ctx.xer);
	// 825102E8: 419A0014  beq cr6, 0x825102fc
	if ctx.cr[6].eq {
	pc = 0x825102FC; continue 'dispatch;
	}
	// 825102EC: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 825102F0: 816B7074  lwz r11, 0x7074(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28788 as u32) ) } as u64;
	// 825102F4: 7F1D5840  cmplw cr6, r29, r11
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[11].u32, &mut ctx.xer);
	// 825102F8: 409A001C  bne cr6, 0x82510314
	if !ctx.cr[6].eq {
	pc = 0x82510314; continue 'dispatch;
	}
	// 825102FC: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 82510300: 808B70B4  lwz r4, 0x70b4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28852 as u32) ) } as u64;
	// 82510304: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82510308: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 8251030C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82510310: 4BFFFEE1  bl 0x825101f0
	ctx.lr = 0x82510314;
	sub_825101F0(ctx, base);
	// 82510314: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82510318: 48C97E98  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82510320(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82510320 size=272
    let mut pc: u32 = 0x82510320;
    'dispatch: loop {
        match pc {
            0x82510320 => {
    //   block [0x82510320..0x82510430)
	// 82510320: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82510324: 48C97E49  bl 0x831a816c
	ctx.lr = 0x82510328;
	sub_831A8130(ctx, base);
	// 82510328: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251032C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82510330: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82510334: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82510338: 4BFFF1E1  bl 0x8250f518
	ctx.lr = 0x8251033C;
	sub_8250F518(ctx, base);
	// 8251033C: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82510340: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82510344: 396AFF6C  addi r11, r10, -0x94
	ctx.r[11].s64 = ctx.r[10].s64 + -148;
	// 82510348: 409A0008  bne cr6, 0x82510350
	if !ctx.cr[6].eq {
	pc = 0x82510350; continue 'dispatch;
	}
	// 8251034C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82510350: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82510354: 419A00CC  beq cr6, 0x82510420
	if ctx.cr[6].eq {
	pc = 0x82510420; continue 'dispatch;
	}
	// 82510358: 817D00BC  lwz r11, 0xbc(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(188 as u32) ) } as u64;
	// 8251035C: 83CB0010  lwz r30, 0x10(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82510360: 83FE0000  lwz r31, 0(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82510364: 7F1FF040  cmplw cr6, r31, r30
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82510368: 419A00AC  beq cr6, 0x82510414
	if ctx.cr[6].eq {
	pc = 0x82510414; continue 'dispatch;
	}
	// 8251036C: 48000008  b 0x82510374
	pc = 0x82510374; continue 'dispatch;
	// 82510370: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82510374: 897F0014  lbz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82510378: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8251037C: 41820070  beq 0x825103ec
	if ctx.cr[0].eq {
	pc = 0x825103EC; continue 'dispatch;
	}
	// 82510380: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82510384: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 82510388: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8251038C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82510390: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82510394: 419A0028  beq cr6, 0x825103bc
	if ctx.cr[6].eq {
	pc = 0x825103BC; continue 'dispatch;
	}
	// 82510398: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8251039C: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 825103A0: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 825103A4: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 825103A8: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 825103AC: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 825103B0: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 825103B4: 4082FFE8  bne 0x8251039c
	if !ctx.cr[0].eq {
	pc = 0x8251039C; continue 'dispatch;
	}
	// 825103B8: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 825103BC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 825103C0: 386AFF6C  addi r3, r10, -0x94
	ctx.r[3].s64 = ctx.r[10].s64 + -148;
	// 825103C4: 409A0008  bne cr6, 0x825103cc
	if !ctx.cr[6].eq {
	pc = 0x825103CC; continue 'dispatch;
	}
	// 825103C8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 825103CC: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 825103D0: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 825103D4: 480188A5  bl 0x82528c78
	ctx.lr = 0x825103D8;
	sub_82528C78(ctx, base);
	// 825103D8: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 825103DC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825103E0: 419A0028  beq cr6, 0x82510408
	if ctx.cr[6].eq {
	pc = 0x82510408; continue 'dispatch;
	}
	// 825103E4: 4BDB04AD  bl 0x822c0890
	ctx.lr = 0x825103E8;
	sub_822C0890(ctx, base);
	// 825103E8: 48000020  b 0x82510408
	pc = 0x82510408; continue 'dispatch;
	// 825103EC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 825103F0: 386AFF6C  addi r3, r10, -0x94
	ctx.r[3].s64 = ctx.r[10].s64 + -148;
	// 825103F4: 409A0008  bne cr6, 0x825103fc
	if !ctx.cr[6].eq {
	pc = 0x825103FC; continue 'dispatch;
	}
	// 825103F8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 825103FC: 38BF000C  addi r5, r31, 0xc
	ctx.r[5].s64 = ctx.r[31].s64 + 12;
	// 82510400: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82510404: 4801880D  bl 0x82528c10
	ctx.lr = 0x82510408;
	sub_82528C10(ctx, base);
	// 82510408: 83FF0000  lwz r31, 0(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251040C: 7F1FF040  cmplw cr6, r31, r30
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82510410: 409AFF60  bne cr6, 0x82510370
	if !ctx.cr[6].eq {
	pc = 0x82510370; continue 'dispatch;
	}
	// 82510414: 817D00BC  lwz r11, 0xbc(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(188 as u32) ) } as u64;
	// 82510418: 386B000C  addi r3, r11, 0xc
	ctx.r[3].s64 = ctx.r[11].s64 + 12;
	// 8251041C: 4829C265  bl 0x827ac680
	ctx.lr = 0x82510420;
	sub_827AC680(ctx, base);
	// 82510420: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82510424: 488E186D  bl 0x82df1c90
	ctx.lr = 0x82510428;
	sub_82DF1C90(ctx, base);
	// 82510428: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8251042C: 48C97D90  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82510430(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82510430 size=148
    let mut pc: u32 = 0x82510430;
    'dispatch: loop {
        match pc {
            0x82510430 => {
    //   block [0x82510430..0x825104C4)
	// 82510430: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82510434: 48C97D39  bl 0x831a816c
	ctx.lr = 0x82510438;
	sub_831A8130(ctx, base);
	// 82510438: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251043C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82510440: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82510444: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82510448: 4BFFF0D1  bl 0x8250f518
	ctx.lr = 0x8251044C;
	sub_8250F518(ctx, base);
	// 8251044C: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82510450: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82510454: 396AFF6C  addi r11, r10, -0x94
	ctx.r[11].s64 = ctx.r[10].s64 + -148;
	// 82510458: 409A0008  bne cr6, 0x82510460
	if !ctx.cr[6].eq {
	pc = 0x82510460; continue 'dispatch;
	}
	// 8251045C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82510460: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82510464: 419A0050  beq cr6, 0x825104b4
	if ctx.cr[6].eq {
	pc = 0x825104B4; continue 'dispatch;
	}
	// 82510468: 817D00BC  lwz r11, 0xbc(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(188 as u32) ) } as u64;
	// 8251046C: 83CB0034  lwz r30, 0x34(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 82510470: 83FE0000  lwz r31, 0(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82510474: 7F1FF040  cmplw cr6, r31, r30
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82510478: 419A0030  beq cr6, 0x825104a8
	if ctx.cr[6].eq {
	pc = 0x825104A8; continue 'dispatch;
	}
	// 8251047C: 48000008  b 0x82510484
	pc = 0x82510484; continue 'dispatch;
	// 82510480: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82510484: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82510488: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8251048C: 386AFF6C  addi r3, r10, -0x94
	ctx.r[3].s64 = ctx.r[10].s64 + -148;
	// 82510490: 409A0008  bne cr6, 0x82510498
	if !ctx.cr[6].eq {
	pc = 0x82510498; continue 'dispatch;
	}
	// 82510494: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82510498: 48017C19  bl 0x825280b0
	ctx.lr = 0x8251049C;
	sub_825280B0(ctx, base);
	// 8251049C: 83FF0000  lwz r31, 0(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825104A0: 7F1FF040  cmplw cr6, r31, r30
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[30].u32, &mut ctx.xer);
	// 825104A4: 409AFFDC  bne cr6, 0x82510480
	if !ctx.cr[6].eq {
	pc = 0x82510480; continue 'dispatch;
	}
	// 825104A8: 817D00BC  lwz r11, 0xbc(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(188 as u32) ) } as u64;
	// 825104AC: 386B0030  addi r3, r11, 0x30
	ctx.r[3].s64 = ctx.r[11].s64 + 48;
	// 825104B0: 482F6009  bl 0x828064b8
	ctx.lr = 0x825104B4;
	sub_828064B8(ctx, base);
	// 825104B4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825104B8: 488E17D9  bl 0x82df1c90
	ctx.lr = 0x825104BC;
	sub_82DF1C90(ctx, base);
	// 825104BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825104C0: 48C97CFC  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825104C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825104C8 size=144
    let mut pc: u32 = 0x825104C8;
    'dispatch: loop {
        match pc {
            0x825104C8 => {
    //   block [0x825104C8..0x82510558)
	// 825104C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825104CC: 48C97C9D  bl 0x831a8168
	ctx.lr = 0x825104D0;
	sub_831A8130(ctx, base);
	// 825104D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825104D4: 83A40004  lwz r29, 4(r4)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 825104D8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 825104DC: 83840000  lwz r28, 0(r4)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 825104E0: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 825104E4: 419A0024  beq cr6, 0x82510508
	if ctx.cr[6].eq {
	pc = 0x82510508; continue 'dispatch;
	}
	// 825104E8: 397D0004  addi r11, r29, 4
	ctx.r[11].s64 = ctx.r[29].s64 + 4;
	// 825104EC: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 825104F0: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 825104F4: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 825104F8: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 825104FC: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82510500: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82510504: 4082FFE8  bne 0x825104ec
	if !ctx.cr[0].eq {
	pc = 0x825104EC; continue 'dispatch;
	}
	// 82510508: 83FE0004  lwz r31, 4(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8251050C: 80BF0000  lwz r5, 0(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82510510: 48000028  b 0x82510538
	pc = 0x82510538; continue 'dispatch;
	// 82510514: 81650008  lwz r11, 8(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(8 as u32) ) } as u64;
	// 82510518: 7F0BE040  cmplw cr6, r11, r28
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[28].u32, &mut ctx.xer);
	// 8251051C: 409A0018  bne cr6, 0x82510534
	if !ctx.cr[6].eq {
	pc = 0x82510534; continue 'dispatch;
	}
	// 82510520: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82510524: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82510528: 481B4C41  bl 0x826c5168
	ctx.lr = 0x8251052C;
	sub_826C5168(ctx, base);
	// 8251052C: 80A30000  lwz r5, 0(r3)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82510530: 48000008  b 0x82510538
	pc = 0x82510538; continue 'dispatch;
	// 82510534: 80A50000  lwz r5, 0(r5)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 82510538: 7F05F840  cmplw cr6, r5, r31
	ctx.cr[6].compare_u32(ctx.r[5].u32, ctx.r[31].u32, &mut ctx.xer);
	// 8251053C: 409AFFD8  bne cr6, 0x82510514
	if !ctx.cr[6].eq {
	pc = 0x82510514; continue 'dispatch;
	}
	// 82510540: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82510544: 419A000C  beq cr6, 0x82510550
	if ctx.cr[6].eq {
	pc = 0x82510550; continue 'dispatch;
	}
	// 82510548: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8251054C: 4BDB0345  bl 0x822c0890
	ctx.lr = 0x82510550;
	sub_822C0890(ctx, base);
	// 82510550: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82510554: 48C97C64  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82510558(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82510558 size=120
    let mut pc: u32 = 0x82510558;
    'dispatch: loop {
        match pc {
            0x82510558 => {
    //   block [0x82510558..0x825105D0)
	// 82510558: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251055C: 48C97C0D  bl 0x831a8168
	ctx.lr = 0x82510560;
	sub_831A8130(ctx, base);
	// 82510560: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82510564: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82510568: 83850000  lwz r28, 0(r5)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251056C: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82510570: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82510574: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82510578: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8251057C: 419A0044  beq cr6, 0x825105c0
	if ctx.cr[6].eq {
	pc = 0x825105C0; continue 'dispatch;
	}
	// 82510580: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82510584: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82510588: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251058C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82510590: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82510594: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82510598: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8251059C: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 825105A0: 4BFFF929  bl 0x8250fec8
	ctx.lr = 0x825105A4;
	sub_8250FEC8(ctx, base);
	// 825105A4: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 825105A8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825105AC: 806B110C  lwz r3, 0x110c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4364 as u32) ) } as u64;
	// 825105B0: 488E1BD9  bl 0x82df2188
	ctx.lr = 0x825105B4;
	sub_82DF2188(ctx, base);
	// 825105B4: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 825105B8: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 825105BC: 917E0008  stw r11, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 825105C0: 939D0000  stw r28, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 825105C4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 825105C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825105CC: 48C97BEC  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825105D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825105D0 size=200
    let mut pc: u32 = 0x825105D0;
    'dispatch: loop {
        match pc {
            0x825105D0 => {
    //   block [0x825105D0..0x82510698)
	// 825105D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825105D4: 48C97B99  bl 0x831a816c
	ctx.lr = 0x825105D8;
	sub_831A8130(ctx, base);
	// 825105D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825105DC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825105E0: 817F00BC  lwz r11, 0xbc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(188 as u32) ) } as u64;
	// 825105E4: 894B0008  lbz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 825105E8: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 825105EC: 408200A4  bne 0x82510690
	if !ctx.cr[0].eq {
	pc = 0x82510690; continue 'dispatch;
	}
	// 825105F0: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 825105F4: 9BCB0008  stb r30, 8(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[30].u8 ) };
	// 825105F8: 48945DB9  bl 0x82e563b0
	ctx.lr = 0x825105FC;
	sub_82E563B0(ctx, base);
	// 825105FC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82510600: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82510604: 9BDF0058  stb r30, 0x58(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[30].u8 ) };
	// 82510608: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 8251060C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82510610: 4E800421  bctrl
	ctx.lr = 0x82510614;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82510614: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82510618: 4BFFFD09  bl 0x82510320
	ctx.lr = 0x8251061C;
	sub_82510320(ctx, base);
	// 8251061C: 817F00BC  lwz r11, 0xbc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(188 as u32) ) } as u64;
	// 82510620: 83AB0034  lwz r29, 0x34(r11)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 82510624: 83DD0000  lwz r30, 0(r29)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82510628: 48000014  b 0x8251063c
	pc = 0x8251063C; continue 'dispatch;
	// 8251062C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82510630: 807E0008  lwz r3, 8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82510634: 4BF7ED15  bl 0x8248f348
	ctx.lr = 0x82510638;
	sub_8248F348(ctx, base);
	// 82510638: 83DE0000  lwz r30, 0(r30)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251063C: 7F1EE840  cmplw cr6, r30, r29
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[29].u32, &mut ctx.xer);
	// 82510640: 409AFFEC  bne cr6, 0x8251062c
	if !ctx.cr[6].eq {
	pc = 0x8251062C; continue 'dispatch;
	}
	// 82510644: 817F00BC  lwz r11, 0xbc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(188 as u32) ) } as u64;
	// 82510648: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251064C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82510650: 419A0040  beq cr6, 0x82510690
	if ctx.cr[6].eq {
	pc = 0x82510690; continue 'dispatch;
	}
	// 82510654: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82510658: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8251065C: 4BFFEE6D  bl 0x8250f4c8
	ctx.lr = 0x82510660;
	sub_8250F4C8(ctx, base);
	// 82510660: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82510664: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82510668: 386BFFFC  addi r3, r11, -4
	ctx.r[3].s64 = ctx.r[11].s64 + -4;
	// 8251066C: 409A0008  bne cr6, 0x82510674
	if !ctx.cr[6].eq {
	pc = 0x82510674; continue 'dispatch;
	}
	// 82510670: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82510674: 817F00BC  lwz r11, 0xbc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(188 as u32) ) } as u64;
	// 82510678: 388B0068  addi r4, r11, 0x68
	ctx.r[4].s64 = ctx.r[11].s64 + 104;
	// 8251067C: 4BFFD4D5  bl 0x8250db50
	ctx.lr = 0x82510680;
	sub_8250DB50(ctx, base);
	// 82510680: 817F00BC  lwz r11, 0xbc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(188 as u32) ) } as u64;
	// 82510684: 906B0074  stw r3, 0x74(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(116 as u32), ctx.r[3].u32 ) };
	// 82510688: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8251068C: 488E1605  bl 0x82df1c90
	ctx.lr = 0x82510690;
	sub_82DF1C90(ctx, base);
	// 82510690: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82510694: 48C97B28  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82510698(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82510698 size=12
    let mut pc: u32 = 0x82510698;
    'dispatch: loop {
        match pc {
            0x82510698 => {
    //   block [0x82510698..0x825106A4)
	// 82510698: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8251069C: 99640018  stb r11, 0x18(r4)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[4].u32.wrapping_add(24 as u32), ctx.r[11].u8 ) };
	// 825106A0: 4BFFFF30  b 0x825105d0
	sub_825105D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825106A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825106A8 size=96
    let mut pc: u32 = 0x825106A8;
    'dispatch: loop {
        match pc {
            0x825106A8 => {
    //   block [0x825106A8..0x82510708)
	// 825106A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825106AC: 48C97AC1  bl 0x831a816c
	ctx.lr = 0x825106B0;
	sub_831A8130(ctx, base);
	// 825106B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825106B4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825106B8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825106BC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825106C0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825106C4: 83BE0000  lwz r29, 0(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 825106C8: 4BFFEE51  bl 0x8250f518
	ctx.lr = 0x825106CC;
	sub_8250F518(ctx, base);
	// 825106CC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825106D0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825106D4: 386BFF6C  addi r3, r11, -0x94
	ctx.r[3].s64 = ctx.r[11].s64 + -148;
	// 825106D8: 409A0008  bne cr6, 0x825106e0
	if !ctx.cr[6].eq {
	pc = 0x825106E0; continue 'dispatch;
	}
	// 825106DC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 825106E0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 825106E4: 480179CD  bl 0x825280b0
	ctx.lr = 0x825106E8;
	sub_825280B0(ctx, base);
	// 825106E8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825106EC: 488E15A5  bl 0x82df1c90
	ctx.lr = 0x825106F0;
	sub_82DF1C90(ctx, base);
	// 825106F0: 817F00BC  lwz r11, 0xbc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(188 as u32) ) } as u64;
	// 825106F4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 825106F8: 386B0030  addi r3, r11, 0x30
	ctx.r[3].s64 = ctx.r[11].s64 + 48;
	// 825106FC: 4BFFFDCD  bl 0x825104c8
	ctx.lr = 0x82510700;
	sub_825104C8(ctx, base);
	// 82510700: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82510704: 48C97AB8  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82510708(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82510708 size=132
    let mut pc: u32 = 0x82510708;
    'dispatch: loop {
        match pc {
            0x82510708 => {
    //   block [0x82510708..0x8251078C)
	// 82510708: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251070C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82510710: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82510714: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82510718: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251071C: 816300BC  lwz r11, 0xbc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(188 as u32) ) } as u64;
	// 82510720: 3BCB005C  addi r30, r11, 0x5c
	ctx.r[30].s64 = ctx.r[11].s64 + 92;
	// 82510724: 816B0060  lwz r11, 0x60(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(96 as u32) ) } as u64;
	// 82510728: 83EB0000  lwz r31, 0(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251072C: 48000040  b 0x8251076c
	pc = 0x8251076C; continue 'dispatch;
	// 82510730: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82510734: 488EE0AD  bl 0x82dfe7e0
	ctx.lr = 0x82510738;
	sub_82DFE7E0(ctx, base);
	// 82510738: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8251073C: 41820028  beq 0x82510764
	if ctx.cr[0].eq {
	pc = 0x82510764; continue 'dispatch;
	}
	// 82510740: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 82510744: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82510748: 480A1181  bl 0x825b18c8
	ctx.lr = 0x8251074C;
	sub_825B18C8(ctx, base);
	// 8251074C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82510750: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82510754: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82510758: 4BFFFE01  bl 0x82510558
	ctx.lr = 0x8251075C;
	sub_82510558(ctx, base);
	// 8251075C: 83E30000  lwz r31, 0(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82510760: 48000008  b 0x82510768
	pc = 0x82510768; continue 'dispatch;
	// 82510764: 83FF0000  lwz r31, 0(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82510768: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8251076C: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82510770: 409AFFC0  bne cr6, 0x82510730
	if !ctx.cr[6].eq {
	pc = 0x82510730; continue 'dispatch;
	}
	// 82510774: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82510778: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8251077C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82510780: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82510784: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82510788: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82510790(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82510790 size=72
    let mut pc: u32 = 0x82510790;
    'dispatch: loop {
        match pc {
            0x82510790 => {
    //   block [0x82510790..0x825107D8)
	// 82510790: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82510794: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82510798: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251079C: 816300BC  lwz r11, 0xbc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(188 as u32) ) } as u64;
	// 825107A0: 816B006C  lwz r11, 0x6c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(108 as u32) ) } as u64;
	// 825107A4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825107A8: 419A0020  beq cr6, 0x825107c8
	if ctx.cr[6].eq {
	pc = 0x825107C8; continue 'dispatch;
	}
	// 825107AC: 816300BC  lwz r11, 0xbc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(188 as u32) ) } as u64;
	// 825107B0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825107B4: 814B006C  lwz r10, 0x6c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(108 as u32) ) } as u64;
	// 825107B8: 80AB0070  lwz r5, 0x70(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(112 as u32) ) } as u64;
	// 825107BC: 816A00BC  lwz r11, 0xbc(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(188 as u32) ) } as u64;
	// 825107C0: 388B0018  addi r4, r11, 0x18
	ctx.r[4].s64 = ctx.r[11].s64 + 24;
	// 825107C4: 481B49A5  bl 0x826c5168
	ctx.lr = 0x825107C8;
	sub_826C5168(ctx, base);
	// 825107C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825107CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825107D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825107D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825107D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825107D8 size=240
    let mut pc: u32 = 0x825107D8;
    'dispatch: loop {
        match pc {
            0x825107D8 => {
    //   block [0x825107D8..0x825108C8)
	// 825107D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825107DC: 48C9798D  bl 0x831a8168
	ctx.lr = 0x825107E0;
	sub_831A8130(ctx, base);
	// 825107E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825107E4: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 825107E8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825107EC: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 825107F0: 4BFFECD9  bl 0x8250f4c8
	ctx.lr = 0x825107F4;
	sub_8250F4C8(ctx, base);
	// 825107F4: 817C00BC  lwz r11, 0xbc(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(188 as u32) ) } as u64;
	// 825107F8: 83AB001C  lwz r29, 0x1c(r11)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 825107FC: 83DD0000  lwz r30, 0(r29)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82510800: 480000A4  b 0x825108a4
	pc = 0x825108A4; continue 'dispatch;
	// 82510804: 83FE0008  lwz r31, 8(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82510808: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8251080C: 815F00BC  lwz r10, 0xbc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(188 as u32) ) } as u64;
	// 82510810: 996A0008  stb r11, 8(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 82510814: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82510818: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8251081C: 388BFFFC  addi r4, r11, -4
	ctx.r[4].s64 = ctx.r[11].s64 + -4;
	// 82510820: 409A0008  bne cr6, 0x82510828
	if !ctx.cr[6].eq {
	pc = 0x82510828; continue 'dispatch;
	}
	// 82510824: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82510828: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251082C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82510830: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82510834: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82510838: 4E800421  bctrl
	ctx.lr = 0x8251083C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8251083C: 387F0028  addi r3, r31, 0x28
	ctx.r[3].s64 = ctx.r[31].s64 + 40;
	// 82510840: 48948CF1  bl 0x82e59530
	ctx.lr = 0x82510844;
	sub_82E59530(ctx, base);
	// 82510844: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82510848: 48000081  bl 0x825108c8
	ctx.lr = 0x8251084C;
	sub_825108C8(ctx, base);
	// 8251084C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82510850: 4BFFFF41  bl 0x82510790
	ctx.lr = 0x82510854;
	sub_82510790(ctx, base);
	// 82510854: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82510858: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8251085C: 386BFFFC  addi r3, r11, -4
	ctx.r[3].s64 = ctx.r[11].s64 + -4;
	// 82510860: 409A0008  bne cr6, 0x82510868
	if !ctx.cr[6].eq {
	pc = 0x82510868; continue 'dispatch;
	}
	// 82510864: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82510868: 817F00BC  lwz r11, 0xbc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(188 as u32) ) } as u64;
	// 8251086C: 388B0068  addi r4, r11, 0x68
	ctx.r[4].s64 = ctx.r[11].s64 + 104;
	// 82510870: 4BFFBC69  bl 0x8250c4d8
	ctx.lr = 0x82510874;
	sub_8250C4D8(ctx, base);
	// 82510874: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82510878: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8251087C: 386BFFFC  addi r3, r11, -4
	ctx.r[3].s64 = ctx.r[11].s64 + -4;
	// 82510880: 409A0008  bne cr6, 0x82510888
	if !ctx.cr[6].eq {
	pc = 0x82510888; continue 'dispatch;
	}
	// 82510884: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82510888: 817F00BC  lwz r11, 0xbc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(188 as u32) ) } as u64;
	// 8251088C: 808B0074  lwz r4, 0x74(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(116 as u32) ) } as u64;
	// 82510890: 4BFFA449  bl 0x8250acd8
	ctx.lr = 0x82510894;
	sub_8250ACD8(ctx, base);
	// 82510894: 815F00BC  lwz r10, 0xbc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(188 as u32) ) } as u64;
	// 82510898: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8251089C: 916A006C  stw r11, 0x6c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825108A0: 83DE0000  lwz r30, 0(r30)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 825108A4: 7F1EE840  cmplw cr6, r30, r29
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[29].u32, &mut ctx.xer);
	// 825108A8: 409AFF5C  bne cr6, 0x82510804
	if !ctx.cr[6].eq {
	pc = 0x82510804; continue 'dispatch;
	}
	// 825108AC: 817C00BC  lwz r11, 0xbc(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(188 as u32) ) } as u64;
	// 825108B0: 386B0018  addi r3, r11, 0x18
	ctx.r[3].s64 = ctx.r[11].s64 + 24;
	// 825108B4: 482F5C05  bl 0x828064b8
	ctx.lr = 0x825108B8;
	sub_828064B8(ctx, base);
	// 825108B8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825108BC: 488E13D5  bl 0x82df1c90
	ctx.lr = 0x825108C0;
	sub_82DF1C90(ctx, base);
	// 825108C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825108C4: 48C978F4  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825108C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825108C8 size=168
    let mut pc: u32 = 0x825108C8;
    'dispatch: loop {
        match pc {
            0x825108C8 => {
    //   block [0x825108C8..0x82510970)
	// 825108C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825108CC: 48C978A1  bl 0x831a816c
	ctx.lr = 0x825108D0;
	sub_831A8130(ctx, base);
	// 825108D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825108D4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825108D8: 4BFFFA49  bl 0x82510320
	ctx.lr = 0x825108DC;
	sub_82510320(ctx, base);
	// 825108DC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825108E0: 4BFFFB51  bl 0x82510430
	ctx.lr = 0x825108E4;
	sub_82510430(ctx, base);
	// 825108E4: 3BDF0028  addi r30, r31, 0x28
	ctx.r[30].s64 = ctx.r[31].s64 + 40;
	// 825108E8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825108EC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 825108F0: 48946171  bl 0x82e56a60
	ctx.lr = 0x825108F4;
	sub_82E56A60(ctx, base);
	// 825108F4: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 825108F8: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 825108FC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82510900: 419A0018  beq cr6, 0x82510918
	if ctx.cr[6].eq {
	pc = 0x82510918; continue 'dispatch;
	}
	// 82510904: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82510908: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8251090C: 409A0008  bne cr6, 0x82510914
	if !ctx.cr[6].eq {
	pc = 0x82510914; continue 'dispatch;
	}
	// 82510910: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82510914: 48951DF5  bl 0x82e62708
	ctx.lr = 0x82510918;
	sub_82E62708(ctx, base);
	// 82510918: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8251091C: 488E1375  bl 0x82df1c90
	ctx.lr = 0x82510920;
	sub_82DF1C90(ctx, base);
	// 82510920: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82510924: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82510928: 48945AB1  bl 0x82e563d8
	ctx.lr = 0x8251092C;
	sub_82E563D8(ctx, base);
	// 8251092C: 80610058  lwz r3, 0x58(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82510930: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82510934: 419A000C  beq cr6, 0x82510940
	if ctx.cr[6].eq {
	pc = 0x82510940; continue 'dispatch;
	}
	// 82510938: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8251093C: 48952DED  bl 0x82e63728
	ctx.lr = 0x82510940;
	sub_82E63728(ctx, base);
	// 82510940: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82510944: 488E134D  bl 0x82df1c90
	ctx.lr = 0x82510948;
	sub_82DF1C90(ctx, base);
	// 82510948: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8251094C: 4BFFFE8D  bl 0x825107d8
	ctx.lr = 0x82510950;
	sub_825107D8(ctx, base);
	// 82510950: 817F00BC  lwz r11, 0xbc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(188 as u32) ) } as u64;
	// 82510954: 93AB0000  stw r29, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82510958: 817F00BC  lwz r11, 0xbc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(188 as u32) ) } as u64;
	// 8251095C: 93AB0004  stw r29, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82510960: 817F00BC  lwz r11, 0xbc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(188 as u32) ) } as u64;
	// 82510964: 93AB006C  stw r29, 0x6c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(108 as u32), ctx.r[29].u32 ) };
	// 82510968: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8251096C: 48C97850  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82510970(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82510970 size=136
    let mut pc: u32 = 0x82510970;
    'dispatch: loop {
        match pc {
            0x82510970 => {
    //   block [0x82510970..0x825109F8)
	// 82510970: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82510974: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82510978: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8251097C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82510980: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82510984: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82510988: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8251098C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82510990: 809F0018  lwz r4, 0x18(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82510994: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82510998: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8251099C: 4E800421  bctrl
	ctx.lr = 0x825109A0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825109A0: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 825109A4: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 825109A8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 825109AC: 419A0034  beq cr6, 0x825109e0
	if ctx.cr[6].eq {
	pc = 0x825109E0; continue 'dispatch;
	}
	// 825109B0: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 825109B4: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 825109B8: 7D6B1671  srawi. r11, r11, 2
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 2) as i64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825109BC: 41820024  beq 0x825109e0
	if ctx.cr[0].eq {
	pc = 0x825109E0; continue 'dispatch;
	}
	// 825109C0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825109C4: 48AF7765  bl 0x83008128
	ctx.lr = 0x825109C8;
	sub_83008128(ctx, base);
	// 825109C8: 90610050  stw r3, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 825109CC: 817E00BC  lwz r11, 0xbc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(188 as u32) ) } as u64;
	// 825109D0: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 825109D4: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 825109D8: 388B0024  addi r4, r11, 0x24
	ctx.r[4].s64 = ctx.r[11].s64 + 36;
	// 825109DC: 4803FF7D  bl 0x82550958
	ctx.lr = 0x825109E0;
	sub_82550958(ctx, base);
	// 825109E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825109E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825109E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825109EC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825109F0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825109F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825109F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825109F8 size=352
    let mut pc: u32 = 0x825109F8;
    'dispatch: loop {
        match pc {
            0x825109F8 => {
    //   block [0x825109F8..0x82510B58)
	// 825109F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825109FC: 48C97771  bl 0x831a816c
	ctx.lr = 0x82510A00;
	sub_831A8130(ctx, base);
	// 82510A00: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82510A04: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82510A08: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82510A0C: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82510A10: 54CB063F  clrlwi. r11, r6, 0x18
	ctx.r[11].u64 = ctx.r[6].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82510A14: 41820058  beq 0x82510a6c
	if ctx.cr[0].eq {
	pc = 0x82510A6C; continue 'dispatch;
	}
	// 82510A18: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 82510A1C: 816B7078  lwz r11, 0x7078(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28792 as u32) ) } as u64;
	// 82510A20: 7F1D5840  cmplw cr6, r29, r11
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82510A24: 409A0010  bne cr6, 0x82510a34
	if !ctx.cr[6].eq {
	pc = 0x82510A34; continue 'dispatch;
	}
	// 82510A28: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 82510A2C: 808B70B8  lwz r4, 0x70b8(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28856 as u32) ) } as u64;
	// 82510A30: 4800002C  b 0x82510a5c
	pc = 0x82510A5C; continue 'dispatch;
	// 82510A34: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 82510A38: 816B7058  lwz r11, 0x7058(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28760 as u32) ) } as u64;
	// 82510A3C: 7F1D5840  cmplw cr6, r29, r11
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82510A40: 419A0014  beq cr6, 0x82510a54
	if ctx.cr[6].eq {
	pc = 0x82510A54; continue 'dispatch;
	}
	// 82510A44: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 82510A48: 816B7074  lwz r11, 0x7074(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28788 as u32) ) } as u64;
	// 82510A4C: 7F1D5840  cmplw cr6, r29, r11
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82510A50: 409A001C  bne cr6, 0x82510a6c
	if !ctx.cr[6].eq {
	pc = 0x82510A6C; continue 'dispatch;
	}
	// 82510A54: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 82510A58: 808B70B4  lwz r4, 0x70b4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28852 as u32) ) } as u64;
	// 82510A5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82510A60: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82510A64: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82510A68: 4BFFFF91  bl 0x825109f8
	ctx.lr = 0x82510A6C;
	sub_825109F8(ctx, base);
	// 82510A6C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82510A70: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82510A74: 4BFFEAA5  bl 0x8250f518
	ctx.lr = 0x82510A78;
	sub_8250F518(ctx, base);
	// 82510A78: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82510A7C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82510A80: 396AFF6C  addi r11, r10, -0x94
	ctx.r[11].s64 = ctx.r[10].s64 + -148;
	// 82510A84: 409A0008  bne cr6, 0x82510a8c
	if !ctx.cr[6].eq {
	pc = 0x82510A8C; continue 'dispatch;
	}
	// 82510A88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82510A8C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82510A90: 419A0020  beq cr6, 0x82510ab0
	if ctx.cr[6].eq {
	pc = 0x82510AB0; continue 'dispatch;
	}
	// 82510A94: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82510A98: 386AFF6C  addi r3, r10, -0x94
	ctx.r[3].s64 = ctx.r[10].s64 + -148;
	// 82510A9C: 409A0008  bne cr6, 0x82510aa4
	if !ctx.cr[6].eq {
	pc = 0x82510AA4; continue 'dispatch;
	}
	// 82510AA0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82510AA4: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82510AA8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82510AAC: 480190DD  bl 0x82529b88
	ctx.lr = 0x82510AB0;
	sub_82529B88(ctx, base);
	// 82510AB0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82510AB4: 488E11DD  bl 0x82df1c90
	ctx.lr = 0x82510AB8;
	sub_82DF1C90(ctx, base);
	// 82510AB8: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82510ABC: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82510AC0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82510AC4: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82510AC8: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 82510ACC: 419A0024  beq cr6, 0x82510af0
	if ctx.cr[6].eq {
	pc = 0x82510AF0; continue 'dispatch;
	}
	// 82510AD0: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82510AD4: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82510AD8: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82510ADC: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82510AE0: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82510AE4: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82510AE8: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82510AEC: 4082FFE8  bne 0x82510ad4
	if !ctx.cr[0].eq {
	pc = 0x82510AD4; continue 'dispatch;
	}
	// 82510AF0: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82510AF4: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 82510AF8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82510AFC: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82510B00: 4BFFED29  bl 0x8250f828
	ctx.lr = 0x82510B04;
	sub_8250F828(ctx, base);
	// 82510B04: 817E00BC  lwz r11, 0xbc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(188 as u32) ) } as u64;
	// 82510B08: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 82510B0C: 3BEB000C  addi r31, r11, 0xc
	ctx.r[31].s64 = ctx.r[11].s64 + 12;
	// 82510B10: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82510B14: 83CB0010  lwz r30, 0x10(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82510B18: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82510B1C: 80BE0004  lwz r5, 4(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82510B20: 4BFFF461  bl 0x8250ff80
	ctx.lr = 0x82510B24;
	sub_8250FF80(ctx, base);
	// 82510B24: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82510B28: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82510B2C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82510B30: 4BF18961  bl 0x82429490
	ctx.lr = 0x82510B34;
	sub_82429490(ctx, base);
	// 82510B34: 93BE0004  stw r29, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82510B38: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82510B3C: 93AB0000  stw r29, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82510B40: 80610068  lwz r3, 0x68(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 82510B44: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82510B48: 419A0008  beq cr6, 0x82510b50
	if ctx.cr[6].eq {
	pc = 0x82510B50; continue 'dispatch;
	}
	// 82510B4C: 4BDAFD45  bl 0x822c0890
	ctx.lr = 0x82510B50;
	sub_822C0890(ctx, base);
	// 82510B50: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82510B54: 48C97668  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82510B58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82510B58 size=328
    let mut pc: u32 = 0x82510B58;
    'dispatch: loop {
        match pc {
            0x82510B58 => {
    //   block [0x82510B58..0x82510CA0)
	// 82510B58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82510B5C: 48C97611  bl 0x831a816c
	ctx.lr = 0x82510B60;
	sub_831A8130(ctx, base);
	// 82510B60: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82510B64: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82510B68: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82510B6C: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82510B70: 54CB063F  clrlwi. r11, r6, 0x18
	ctx.r[11].u64 = ctx.r[6].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82510B74: 41820058  beq 0x82510bcc
	if ctx.cr[0].eq {
	pc = 0x82510BCC; continue 'dispatch;
	}
	// 82510B78: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 82510B7C: 816B7078  lwz r11, 0x7078(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28792 as u32) ) } as u64;
	// 82510B80: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82510B84: 409A0010  bne cr6, 0x82510b94
	if !ctx.cr[6].eq {
	pc = 0x82510B94; continue 'dispatch;
	}
	// 82510B88: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 82510B8C: 808B70B8  lwz r4, 0x70b8(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28856 as u32) ) } as u64;
	// 82510B90: 4800002C  b 0x82510bbc
	pc = 0x82510BBC; continue 'dispatch;
	// 82510B94: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 82510B98: 816B7058  lwz r11, 0x7058(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28760 as u32) ) } as u64;
	// 82510B9C: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82510BA0: 419A0014  beq cr6, 0x82510bb4
	if ctx.cr[6].eq {
	pc = 0x82510BB4; continue 'dispatch;
	}
	// 82510BA4: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 82510BA8: 816B7074  lwz r11, 0x7074(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28788 as u32) ) } as u64;
	// 82510BAC: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82510BB0: 409A001C  bne cr6, 0x82510bcc
	if !ctx.cr[6].eq {
	pc = 0x82510BCC; continue 'dispatch;
	}
	// 82510BB4: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 82510BB8: 808B70B4  lwz r4, 0x70b4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28852 as u32) ) } as u64;
	// 82510BBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82510BC0: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82510BC4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82510BC8: 4BFFFF91  bl 0x82510b58
	ctx.lr = 0x82510BCC;
	sub_82510B58(ctx, base);
	// 82510BCC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82510BD0: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82510BD4: 4BFFE945  bl 0x8250f518
	ctx.lr = 0x82510BD8;
	sub_8250F518(ctx, base);
	// 82510BD8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82510BDC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82510BE0: 386BFF6C  addi r3, r11, -0x94
	ctx.r[3].s64 = ctx.r[11].s64 + -148;
	// 82510BE4: 409A0008  bne cr6, 0x82510bec
	if !ctx.cr[6].eq {
	pc = 0x82510BEC; continue 'dispatch;
	}
	// 82510BE8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82510BEC: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82510BF0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82510BF4: 48019275  bl 0x82529e68
	ctx.lr = 0x82510BF8;
	sub_82529E68(ctx, base);
	// 82510BF8: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82510BFC: 488E1095  bl 0x82df1c90
	ctx.lr = 0x82510C00;
	sub_82DF1C90(ctx, base);
	// 82510C00: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82510C04: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82510C08: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82510C0C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82510C10: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82510C14: 419A0024  beq cr6, 0x82510c38
	if ctx.cr[6].eq {
	pc = 0x82510C38; continue 'dispatch;
	}
	// 82510C18: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82510C1C: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82510C20: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82510C24: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82510C28: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82510C2C: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82510C30: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82510C34: 4082FFE8  bne 0x82510c1c
	if !ctx.cr[0].eq {
	pc = 0x82510C1C; continue 'dispatch;
	}
	// 82510C38: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82510C3C: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82510C40: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82510C44: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82510C48: 4BFFEBE1  bl 0x8250f828
	ctx.lr = 0x82510C4C;
	sub_8250F828(ctx, base);
	// 82510C4C: 817D00BC  lwz r11, 0xbc(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(188 as u32) ) } as u64;
	// 82510C50: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 82510C54: 3BEB000C  addi r31, r11, 0xc
	ctx.r[31].s64 = ctx.r[11].s64 + 12;
	// 82510C58: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82510C5C: 83CB0010  lwz r30, 0x10(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82510C60: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82510C64: 80BE0004  lwz r5, 4(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82510C68: 4BFFF319  bl 0x8250ff80
	ctx.lr = 0x82510C6C;
	sub_8250FF80(ctx, base);
	// 82510C6C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82510C70: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82510C74: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82510C78: 4BF18819  bl 0x82429490
	ctx.lr = 0x82510C7C;
	sub_82429490(ctx, base);
	// 82510C7C: 93BE0004  stw r29, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82510C80: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82510C84: 93AB0000  stw r29, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82510C88: 80610068  lwz r3, 0x68(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 82510C8C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82510C90: 419A0008  beq cr6, 0x82510c98
	if ctx.cr[6].eq {
	pc = 0x82510C98; continue 'dispatch;
	}
	// 82510C94: 4BDAFBFD  bl 0x822c0890
	ctx.lr = 0x82510C98;
	sub_822C0890(ctx, base);
	// 82510C98: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82510C9C: 48C97520  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82510CA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82510CA0 size=156
    let mut pc: u32 = 0x82510CA0;
    'dispatch: loop {
        match pc {
            0x82510CA0 => {
    //   block [0x82510CA0..0x82510D3C)
	// 82510CA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82510CA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82510CA8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82510CAC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82510CB0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82510CB4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82510CB8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82510CBC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82510CC0: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 82510CC4: 817E00BC  lwz r11, 0xbc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(188 as u32) ) } as u64;
	// 82510CC8: 388B0018  addi r4, r11, 0x18
	ctx.r[4].s64 = ctx.r[11].s64 + 24;
	// 82510CCC: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82510CD0: 80AB0000  lwz r5, 0(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82510CD4: 484BCD85  bl 0x829cda58
	ctx.lr = 0x82510CD8;
	sub_829CDA58(ctx, base);
	// 82510CD8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82510CDC: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82510CE0: 816B00BC  lwz r11, 0xbc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(188 as u32) ) } as u64;
	// 82510CE4: 93CB006C  stw r30, 0x6c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(108 as u32), ctx.r[30].u32 ) };
	// 82510CE8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82510CEC: 816B00BC  lwz r11, 0xbc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(188 as u32) ) } as u64;
	// 82510CF0: 914B0070  stw r10, 0x70(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(112 as u32), ctx.r[10].u32 ) };
	// 82510CF4: 817E00BC  lwz r11, 0xbc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(188 as u32) ) } as u64;
	// 82510CF8: E86B0040  ld r3, 0x40(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(64 as u32) ) };
	// 82510CFC: 2B230000  cmpldi cr6, r3, 0
	ctx.cr[6].compare_u64(ctx.r[3].u64, 0, &mut ctx.xer);
	// 82510D00: 419A0024  beq cr6, 0x82510d24
	if ctx.cr[6].eq {
	pc = 0x82510D24; continue 'dispatch;
	}
	// 82510D04: 556B003E  slwi r11, r11, 0
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82510D08: E88B0048  ld r4, 0x48(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(72 as u32) ) };
	// 82510D0C: 39440001  addi r10, r4, 1
	ctx.r[10].s64 = ctx.r[4].s64 + 1;
	// 82510D10: F94B0048  std r10, 0x48(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(72 as u32), ctx.r[10].u64 ) };
	// 82510D14: 83FF0000  lwz r31, 0(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82510D18: 4807A349  bl 0x8258b060
	ctx.lr = 0x82510D1C;
	sub_8258B060(ctx, base);
	// 82510D1C: 817F00BC  lwz r11, 0xbc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(188 as u32) ) } as u64;
	// 82510D20: F86B0040  std r3, 0x40(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(64 as u32), ctx.r[3].u64 ) };
	// 82510D24: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82510D28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82510D2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82510D30: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82510D34: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82510D38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82510D40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82510D40 size=252
    let mut pc: u32 = 0x82510D40;
    'dispatch: loop {
        match pc {
            0x82510D40 => {
    //   block [0x82510D40..0x82510E3C)
	// 82510D40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82510D44: 48C97421  bl 0x831a8164
	ctx.lr = 0x82510D48;
	sub_831A8130(ctx, base);
	// 82510D48: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82510D4C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82510D50: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82510D54: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82510D58: 817E00BC  lwz r11, 0xbc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(188 as u32) ) } as u64;
	// 82510D5C: 3BEB0030  addi r31, r11, 0x30
	ctx.r[31].s64 = ctx.r[11].s64 + 48;
	// 82510D60: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82510D64: 838B0034  lwz r28, 0x34(r11)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 82510D68: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82510D6C: 80BC0004  lwz r5, 4(r28)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82510D70: 481B8B61  bl 0x826c98d0
	ctx.lr = 0x82510D74;
	sub_826C98D0(ctx, base);
	// 82510D74: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82510D78: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82510D7C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82510D80: 485F75A1  bl 0x82b08320
	ctx.lr = 0x82510D84;
	sub_82B08320(ctx, base);
	// 82510D84: 937C0004  stw r27, 4(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(4 as u32), ctx.r[27].u32 ) };
	// 82510D88: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82510D8C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82510D90: 936B0000  stw r27, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 82510D94: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82510D98: 83FD0000  lwz r31, 0(r29)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82510D9C: 4BFFE77D  bl 0x8250f518
	ctx.lr = 0x82510DA0;
	sub_8250F518(ctx, base);
	// 82510DA0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82510DA4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82510DA8: 386BFF6C  addi r3, r11, -0x94
	ctx.r[3].s64 = ctx.r[11].s64 + -148;
	// 82510DAC: 409A0008  bne cr6, 0x82510db4
	if !ctx.cr[6].eq {
	pc = 0x82510DB4; continue 'dispatch;
	}
	// 82510DB0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82510DB4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82510DB8: 48017239  bl 0x82527ff0
	ctx.lr = 0x82510DBC;
	sub_82527FF0(ctx, base);
	// 82510DBC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82510DC0: 488E0ED1  bl 0x82df1c90
	ctx.lr = 0x82510DC4;
	sub_82DF1C90(ctx, base);
	// 82510DC4: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 82510DC8: 896B701D  lbz r11, 0x701d(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(28701 as u32) ) } as u64;
	// 82510DCC: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82510DD0: 41820064  beq 0x82510e34
	if ctx.cr[0].eq {
	pc = 0x82510E34; continue 'dispatch;
	}
	// 82510DD4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82510DD8: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82510DDC: 4BFFE6ED  bl 0x8250f4c8
	ctx.lr = 0x82510DE0;
	sub_8250F4C8(ctx, base);
	// 82510DE0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82510DE4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82510DE8: 3BEBFFFC  addi r31, r11, -4
	ctx.r[31].s64 = ctx.r[11].s64 + -4;
	// 82510DEC: 409A0008  bne cr6, 0x82510df4
	if !ctx.cr[6].eq {
	pc = 0x82510DF4; continue 'dispatch;
	}
	// 82510DF0: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82510DF4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82510DF8: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 82510DFC: 4BDDD3C5  bl 0x822ee1c0
	ctx.lr = 0x82510E00;
	sub_822EE1C0(ctx, base);
	// 82510E00: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82510E04: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82510E08: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82510E0C: 4BFFE70D  bl 0x8250f518
	ctx.lr = 0x82510E10;
	sub_8250F518(ctx, base);
	// 82510E10: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82510E14: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82510E18: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82510E1C: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82510E20: 4BFFC6E9  bl 0x8250d508
	ctx.lr = 0x82510E24;
	sub_8250D508(ctx, base);
	// 82510E24: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82510E28: 488E0E69  bl 0x82df1c90
	ctx.lr = 0x82510E2C;
	sub_82DF1C90(ctx, base);
	// 82510E2C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82510E30: 488E0E61  bl 0x82df1c90
	ctx.lr = 0x82510E34;
	sub_82DF1C90(ctx, base);
	// 82510E34: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82510E38: 48C9737C  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82510E40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82510E40 size=128
    let mut pc: u32 = 0x82510E40;
    'dispatch: loop {
        match pc {
            0x82510E40 => {
    //   block [0x82510E40..0x82510EC0)
	// 82510E40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82510E44: 48C97325  bl 0x831a8168
	ctx.lr = 0x82510E48;
	sub_831A8130(ctx, base);
	// 82510E48: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82510E4C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82510E50: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82510E54: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82510E58: 4BFFEA61  bl 0x8250f8b8
	ctx.lr = 0x82510E5C;
	sub_8250F8B8(ctx, base);
	// 82510E5C: 817F00BC  lwz r11, 0xbc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(188 as u32) ) } as u64;
	// 82510E60: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 82510E64: 3BEB005C  addi r31, r11, 0x5c
	ctx.r[31].s64 = ctx.r[11].s64 + 92;
	// 82510E68: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82510E6C: 83AB0060  lwz r29, 0x60(r11)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(96 as u32) ) } as u64;
	// 82510E70: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82510E74: 80BD0004  lwz r5, 4(r29)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82510E78: 4BFFF171  bl 0x8250ffe8
	ctx.lr = 0x82510E7C;
	sub_8250FFE8(ctx, base);
	// 82510E7C: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82510E80: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82510E84: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82510E88: 482C8A89  bl 0x827d9910
	ctx.lr = 0x82510E8C;
	sub_827D9910(ctx, base);
	// 82510E8C: 939D0004  stw r28, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 82510E90: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82510E94: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82510E98: 938B0000  stw r28, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82510E9C: 4BDB7E1D  bl 0x822c8cb8
	ctx.lr = 0x82510EA0;
	sub_822C8CB8(ctx, base);
	// 82510EA0: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82510EA4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82510EA8: 419A0008  beq cr6, 0x82510eb0
	if ctx.cr[6].eq {
	pc = 0x82510EB0; continue 'dispatch;
	}
	// 82510EAC: 4BDAF9E5  bl 0x822c0890
	ctx.lr = 0x82510EB0;
	sub_822C0890(ctx, base);
	// 82510EB0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82510EB4: 4BDB7E05  bl 0x822c8cb8
	ctx.lr = 0x82510EB8;
	sub_822C8CB8(ctx, base);
	// 82510EB8: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82510EBC: 48C972FC  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82510EC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82510EC0 size=64
    let mut pc: u32 = 0x82510EC0;
    'dispatch: loop {
        match pc {
            0x82510EC0 => {
    //   block [0x82510EC0..0x82510F00)
	// 82510EC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82510EC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82510EC8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82510ECC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82510ED0: 38630028  addi r3, r3, 0x28
	ctx.r[3].s64 = ctx.r[3].s64 + 40;
	// 82510ED4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82510ED8: 48AF80E1  bl 0x83008fb8
	ctx.lr = 0x82510EDC;
	sub_83008FB8(ctx, base);
	// 82510EDC: 90610050  stw r3, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82510EE0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82510EE4: 807F0018  lwz r3, 0x18(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82510EE8: 4BFA8229  bl 0x824b9110
	ctx.lr = 0x82510EEC;
	sub_824B9110(ctx, base);
	// 82510EEC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82510EF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82510EF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82510EF8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82510EFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82510F00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82510F00 size=136
    let mut pc: u32 = 0x82510F00;
    'dispatch: loop {
        match pc {
            0x82510F00 => {
    //   block [0x82510F00..0x82510F88)
	// 82510F00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82510F04: 48C97269  bl 0x831a816c
	ctx.lr = 0x82510F08;
	sub_831A8130(ctx, base);
	// 82510F08: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82510F0C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82510F10: 387F005C  addi r3, r31, 0x5c
	ctx.r[3].s64 = ctx.r[31].s64 + 92;
	// 82510F14: 4BFFF13D  bl 0x82510050
	ctx.lr = 0x82510F18;
	sub_82510050(ctx, base);
	// 82510F18: 3FA08335  lis r29, -0x7ccb
	ctx.r[29].s64 = -2093678592;
	// 82510F1C: 809F0060  lwz r4, 0x60(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 82510F20: 807D110C  lwz r3, 0x110c(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4364 as u32) ) } as u64;
	// 82510F24: 488E1265  bl 0x82df2188
	ctx.lr = 0x82510F28;
	sub_82DF2188(ctx, base);
	// 82510F28: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82510F2C: 387F0030  addi r3, r31, 0x30
	ctx.r[3].s64 = ctx.r[31].s64 + 48;
	// 82510F30: 93DF0060  stw r30, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[30].u32 ) };
	// 82510F34: 482F5585  bl 0x828064b8
	ctx.lr = 0x82510F38;
	sub_828064B8(ctx, base);
	// 82510F38: 807D110C  lwz r3, 0x110c(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4364 as u32) ) } as u64;
	// 82510F3C: 809F0034  lwz r4, 0x34(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 82510F40: 488E1249  bl 0x82df2188
	ctx.lr = 0x82510F44;
	sub_82DF2188(ctx, base);
	// 82510F44: 93DF0034  stw r30, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[30].u32 ) };
	// 82510F48: 387F0024  addi r3, r31, 0x24
	ctx.r[3].s64 = ctx.r[31].s64 + 36;
	// 82510F4C: 4BE0FFA5  bl 0x82320ef0
	ctx.lr = 0x82510F50;
	sub_82320EF0(ctx, base);
	// 82510F50: 387F0018  addi r3, r31, 0x18
	ctx.r[3].s64 = ctx.r[31].s64 + 24;
	// 82510F54: 482F5565  bl 0x828064b8
	ctx.lr = 0x82510F58;
	sub_828064B8(ctx, base);
	// 82510F58: 807D110C  lwz r3, 0x110c(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4364 as u32) ) } as u64;
	// 82510F5C: 809F001C  lwz r4, 0x1c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82510F60: 488E1229  bl 0x82df2188
	ctx.lr = 0x82510F64;
	sub_82DF2188(ctx, base);
	// 82510F64: 93DF001C  stw r30, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[30].u32 ) };
	// 82510F68: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 82510F6C: 4829B715  bl 0x827ac680
	ctx.lr = 0x82510F70;
	sub_827AC680(ctx, base);
	// 82510F70: 807D110C  lwz r3, 0x110c(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4364 as u32) ) } as u64;
	// 82510F74: 809F0010  lwz r4, 0x10(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82510F78: 488E1211  bl 0x82df2188
	ctx.lr = 0x82510F7C;
	sub_82DF2188(ctx, base);
	// 82510F7C: 93DF0010  stw r30, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[30].u32 ) };
	// 82510F80: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82510F84: 48C97238  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82510F88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82510F88 size=84
    let mut pc: u32 = 0x82510F88;
    'dispatch: loop {
        match pc {
            0x82510F88 => {
    //   block [0x82510F88..0x82510FDC)
	// 82510F88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82510F8C: 48C971DD  bl 0x831a8168
	ctx.lr = 0x82510F90;
	sub_831A8130(ctx, base);
	// 82510F90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82510F94: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82510F98: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82510F9C: 3BDF0004  addi r30, r31, 4
	ctx.r[30].s64 = ctx.r[31].s64 + 4;
	// 82510FA0: 396B1B78  addi r11, r11, 0x1b78
	ctx.r[11].s64 = ctx.r[11].s64 + 7032;
	// 82510FA4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82510FA8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82510FAC: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82510FB0: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82510FB4: 4BF2405D  bl 0x82435010
	ctx.lr = 0x82510FB8;
	sub_82435010(ctx, base);
	// 82510FB8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82510FBC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82510FC0: 4803EB59  bl 0x8254fb18
	ctx.lr = 0x82510FC4;
	sub_8254FB18(ctx, base);
	// 82510FC4: 939F0010  stw r28, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[28].u32 ) };
	// 82510FC8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82510FCC: 4BE0FF25  bl 0x82320ef0
	ctx.lr = 0x82510FD0;
	sub_82320EF0(ctx, base);
	// 82510FD0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82510FD4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82510FD8: 48C971E0  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82510FE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82510FE0 size=8
    let mut pc: u32 = 0x82510FE0;
    'dispatch: loop {
        match pc {
            0x82510FE0 => {
    //   block [0x82510FE0..0x82510FE8)
	// 82510FE0: 80630010  lwz r3, 0x10(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82510FE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82510FE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82510FE8 size=92
    let mut pc: u32 = 0x82510FE8;
    'dispatch: loop {
        match pc {
            0x82510FE8 => {
    //   block [0x82510FE8..0x82511044)
	// 82510FE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82510FEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82510FF0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82510FF4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82510FF8: 3D608328  lis r11, -0x7cd8
	ctx.r[11].s64 = -2094530560;
	// 82510FFC: 3D408328  lis r10, -0x7cd8
	ctx.r[10].s64 = -2094530560;
	// 82511000: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82511004: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82511008: 38CBC7F8  addi r6, r11, -0x3808
	ctx.r[6].s64 = ctx.r[11].s64 + -14344;
	// 8251100C: 38AABE54  addi r5, r10, -0x41ac
	ctx.r[5].s64 = ctx.r[10].s64 + -16812;
	// 82511010: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82511014: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82511018: 48C98F31  bl 0x831a9f48
	ctx.lr = 0x8251101C;
	sub_831A9F48(ctx, base);
	// 8251101C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82511020: 41820010  beq 0x82511030
	if ctx.cr[0].eq {
	pc = 0x82511030; continue 'dispatch;
	}
	// 82511024: 38830004  addi r4, r3, 4
	ctx.r[4].s64 = ctx.r[3].s64 + 4;
	// 82511028: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 8251102C: 4804015D  bl 0x82551188
	ctx.lr = 0x82511030;
	sub_82551188(ctx, base);
	// 82511030: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82511034: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82511038: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8251103C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82511040: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82511048(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82511048 size=168
    let mut pc: u32 = 0x82511048;
    'dispatch: loop {
        match pc {
            0x82511048 => {
    //   block [0x82511048..0x825110F0)
	// 82511048: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251104C: 48C97121  bl 0x831a816c
	ctx.lr = 0x82511050;
	sub_831A8130(ctx, base);
	// 82511050: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82511054: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82511058: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8251105C: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 82511060: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82511064: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82511068: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 8251106C: 9BDF0008  stb r30, 8(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u8 ) };
	// 82511070: 4BF17671  bl 0x824286e0
	ctx.lr = 0x82511074;
	sub_824286E0(ctx, base);
	// 82511074: 907F0010  stw r3, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[3].u32 ) };
	// 82511078: 93DF0014  stw r30, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[30].u32 ) };
	// 8251107C: 387F0018  addi r3, r31, 0x18
	ctx.r[3].s64 = ctx.r[31].s64 + 24;
	// 82511080: 485F4C11  bl 0x82b05c90
	ctx.lr = 0x82511084;
	sub_82B05C90(ctx, base);
	// 82511084: 907F001C  stw r3, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[3].u32 ) };
	// 82511088: 93DF0020  stw r30, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[30].u32 ) };
	// 8251108C: 387F0024  addi r3, r31, 0x24
	ctx.r[3].s64 = ctx.r[31].s64 + 36;
	// 82511090: 4BF23F81  bl 0x82435010
	ctx.lr = 0x82511094;
	sub_82435010(ctx, base);
	// 82511094: 387F0030  addi r3, r31, 0x30
	ctx.r[3].s64 = ctx.r[31].s64 + 48;
	// 82511098: 485F4BF9  bl 0x82b05c90
	ctx.lr = 0x8251109C;
	sub_82B05C90(ctx, base);
	// 8251109C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 825110A0: 39400006  li r10, 6
	ctx.r[10].s64 = 6;
	// 825110A4: 907F0034  stw r3, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[3].u32 ) };
	// 825110A8: 93DF0038  stw r30, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[30].u32 ) };
	// 825110AC: 387F005C  addi r3, r31, 0x5c
	ctx.r[3].s64 = ctx.r[31].s64 + 92;
	// 825110B0: FBDF0040  std r30, 0x40(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), ctx.r[30].u64 ) };
	// 825110B4: F97F0048  std r11, 0x48(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[11].u64 ) };
	// 825110B8: 997F0050  stb r11, 0x50(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[11].u8 ) };
	// 825110BC: 915F0054  stw r10, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 825110C0: 93BF0058  stw r29, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[29].u32 ) };
	// 825110C4: 4BFFE64D  bl 0x8250f710
	ctx.lr = 0x825110C8;
	sub_8250F710(ctx, base);
	// 825110C8: 907F0060  stw r3, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[3].u32 ) };
	// 825110CC: 93DF0064  stw r30, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[30].u32 ) };
	// 825110D0: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 825110D4: 93DF0068  stw r30, 0x68(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(104 as u32), ctx.r[30].u32 ) };
	// 825110D8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825110DC: 93DF006C  stw r30, 0x6c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(108 as u32), ctx.r[30].u32 ) };
	// 825110E0: 93DF0070  stw r30, 0x70(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(112 as u32), ctx.r[30].u32 ) };
	// 825110E4: 917F0074  stw r11, 0x74(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825110E8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825110EC: 48C970D0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825110F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825110F0 size=168
    let mut pc: u32 = 0x825110F0;
    'dispatch: loop {
        match pc {
            0x825110F0 => {
    //   block [0x825110F0..0x82511198)
	// 825110F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825110F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825110F8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825110FC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82511100: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82511104: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82511108: 48945449  bl 0x82e56550
	ctx.lr = 0x8251110C;
	sub_82E56550(ctx, base);
	// 8251110C: 3BDF0028  addi r30, r31, 0x28
	ctx.r[30].s64 = ctx.r[31].s64 + 40;
	// 82511110: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82511114: 48947DDD  bl 0x82e58ef0
	ctx.lr = 0x82511118;
	sub_82E58EF0(ctx, base);
	// 82511118: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8251111C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82511120: 396B1B9C  addi r11, r11, 0x1b9c
	ctx.r[11].s64 = ctx.r[11].s64 + 7068;
	// 82511124: 394A1B88  addi r10, r10, 0x1b88
	ctx.r[10].s64 = ctx.r[10].s64 + 7048;
	// 82511128: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8251112C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82511130: 915F0028  stw r10, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[10].u32 ) };
	// 82511134: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82511138: 388B1B3C  addi r4, r11, 0x1b3c
	ctx.r[4].s64 = ctx.r[11].s64 + 6972;
	// 8251113C: 38A000C9  li r5, 0xc9
	ctx.r[5].s64 = 201;
	// 82511140: 38600078  li r3, 0x78
	ctx.r[3].s64 = 120;
	// 82511144: 488E12A5  bl 0x82df23e8
	ctx.lr = 0x82511148;
	sub_82DF23E8(ctx, base);
	// 82511148: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8251114C: 41820010  beq 0x8251115c
	if ctx.cr[0].eq {
	pc = 0x8251115C; continue 'dispatch;
	}
	// 82511150: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82511154: 4BFFFEF5  bl 0x82511048
	ctx.lr = 0x82511158;
	sub_82511048(ctx, base);
	// 82511158: 48000008  b 0x82511160
	pc = 0x82511160; continue 'dispatch;
	// 8251115C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82511160: 907F00BC  stw r3, 0xbc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(188 as u32), ctx.r[3].u32 ) };
	// 82511164: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82511168: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8251116C: 489452A5  bl 0x82e56410
	ctx.lr = 0x82511170;
	sub_82E56410(ctx, base);
	// 82511170: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82511174: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82511178: 489454F9  bl 0x82e56670
	ctx.lr = 0x8251117C;
	sub_82E56670(ctx, base);
	// 8251117C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82511180: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82511184: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82511188: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8251118C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82511190: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82511194: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82511198(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82511198 size=100
    let mut pc: u32 = 0x82511198;
    'dispatch: loop {
        match pc {
            0x82511198 => {
    //   block [0x82511198..0x825111FC)
	// 82511198: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251119C: 48C96FD1  bl 0x831a816c
	ctx.lr = 0x825111A0;
	sub_831A8130(ctx, base);
	// 825111A0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825111A4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825111A8: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 825111AC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825111B0: 396B1B9C  addi r11, r11, 0x1b9c
	ctx.r[11].s64 = ctx.r[11].s64 + 7068;
	// 825111B4: 394A1B88  addi r10, r10, 0x1b88
	ctx.r[10].s64 = ctx.r[10].s64 + 7048;
	// 825111B8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825111BC: 3BBF0028  addi r29, r31, 0x28
	ctx.r[29].s64 = ctx.r[31].s64 + 40;
	// 825111C0: 915F0028  stw r10, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[10].u32 ) };
	// 825111C4: 4BFFF615  bl 0x825107d8
	ctx.lr = 0x825111C8;
	sub_825107D8(ctx, base);
	// 825111C8: 83DF00BC  lwz r30, 0xbc(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(188 as u32) ) } as u64;
	// 825111CC: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 825111D0: 419A0014  beq cr6, 0x825111e4
	if ctx.cr[6].eq {
	pc = 0x825111E4; continue 'dispatch;
	}
	// 825111D4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825111D8: 4BFFFD29  bl 0x82510f00
	ctx.lr = 0x825111DC;
	sub_82510F00(ctx, base);
	// 825111DC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825111E0: 488E11F9  bl 0x82df23d8
	ctx.lr = 0x825111E4;
	sub_82DF23D8(ctx, base);
	// 825111E4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 825111E8: 48947B81  bl 0x82e58d68
	ctx.lr = 0x825111EC;
	sub_82E58D68(ctx, base);
	// 825111EC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825111F0: 48945291  bl 0x82e56480
	ctx.lr = 0x825111F4;
	sub_82E56480(ctx, base);
	// 825111F4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825111F8: 48C96FC4  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82511200(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82511200 size=188
    let mut pc: u32 = 0x82511200;
    'dispatch: loop {
        match pc {
            0x82511200 => {
    //   block [0x82511200..0x825112BC)
	// 82511200: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82511204: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82511208: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8251120C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82511210: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82511214: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82511218: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8251121C: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82511220: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82511224: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82511228: 4BDAF711  bl 0x822c0938
	ctx.lr = 0x8251122C;
	sub_822C0938(ctx, base);
	// 8251122C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82511230: 41820028  beq 0x82511258
	if ctx.cr[0].eq {
	pc = 0x82511258; continue 'dispatch;
	}
	// 82511234: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82511238: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 8251123C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82511240: 392B1B18  addi r9, r11, 0x1b18
	ctx.r[9].s64 = ctx.r[11].s64 + 6936;
	// 82511244: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82511248: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8251124C: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82511250: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82511254: 48000008  b 0x8251125c
	pc = 0x8251125C; continue 'dispatch;
	// 82511258: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8251125C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82511260: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82511264: 409A003C  bne cr6, 0x825112a0
	if !ctx.cr[6].eq {
	pc = 0x825112A0; continue 'dispatch;
	}
	// 82511268: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8251126C: 419A0014  beq cr6, 0x82511280
	if ctx.cr[6].eq {
	pc = 0x82511280; continue 'dispatch;
	}
	// 82511270: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 82511274: 4BE0FC7D  bl 0x82320ef0
	ctx.lr = 0x82511278;
	sub_82320EF0(ctx, base);
	// 82511278: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8251127C: 4BDAEFED  bl 0x822c0268
	ctx.lr = 0x82511280;
	sub_822C0268(ctx, base);
	// 82511280: 3D608328  lis r11, -0x7cd8
	ctx.r[11].s64 = -2094530560;
	// 82511284: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82511288: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8251128C: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 82511290: 816BC71C  lwz r11, -0x38e4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-14564 as u32) ) } as u64;
	// 82511294: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82511298: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8251129C: 4BDAED65  bl 0x822c0000
	ctx.lr = 0x825112A0;
	sub_822C0000(ctx, base);
	// 825112A0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825112A4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825112A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825112AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825112B0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825112B4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825112B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825112C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825112C0 size=64
    let mut pc: u32 = 0x825112C0;
    'dispatch: loop {
        match pc {
            0x825112C0 => {
    //   block [0x825112C0..0x82511300)
	// 825112C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825112C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825112C8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825112CC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825112D0: 83E3000C  lwz r31, 0xc(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 825112D4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 825112D8: 419A0014  beq cr6, 0x825112ec
	if ctx.cr[6].eq {
	pc = 0x825112EC; continue 'dispatch;
	}
	// 825112DC: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 825112E0: 4BE0FC11  bl 0x82320ef0
	ctx.lr = 0x825112E4;
	sub_82320EF0(ctx, base);
	// 825112E4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825112E8: 4BDAEF81  bl 0x822c0268
	ctx.lr = 0x825112EC;
	sub_822C0268(ctx, base);
	// 825112EC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825112F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825112F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825112F8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825112FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82511300(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82511300 size=380
    let mut pc: u32 = 0x82511300;
    'dispatch: loop {
        match pc {
            0x82511300 => {
    //   block [0x82511300..0x8251147C)
	// 82511300: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82511304: 48C96E61  bl 0x831a8164
	ctx.lr = 0x82511308;
	sub_831A8130(ctx, base);
	// 82511308: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251130C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82511310: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82511314: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82511318: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 8251131C: 897D0000  lbz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82511320: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82511324: 418200FC  beq 0x82511420
	if ctx.cr[0].eq {
	pc = 0x82511420; continue 'dispatch;
	}
	// 82511328: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8251132C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82511330: 388B19D8  addi r4, r11, 0x19d8
	ctx.r[4].s64 = ctx.r[11].s64 + 6616;
	// 82511334: 38A00018  li r5, 0x18
	ctx.r[5].s64 = 24;
	// 82511338: 38600014  li r3, 0x14
	ctx.r[3].s64 = 20;
	// 8251133C: 4BDAF09D  bl 0x822c03d8
	ctx.lr = 0x82511340;
	sub_822C03D8(ctx, base);
	// 82511340: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82511344: 41820030  beq 0x82511374
	if ctx.cr[0].eq {
	pc = 0x82511374; continue 'dispatch;
	}
	// 82511348: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 8251134C: 4BF23CC5  bl 0x82435010
	ctx.lr = 0x82511350;
	sub_82435010(ctx, base);
	// 82511350: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82511354: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 82511358: 4803E7C1  bl 0x8254fb18
	ctx.lr = 0x8251135C;
	sub_8254FB18(ctx, base);
	// 8251135C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82511360: 38810080  addi r4, r1, 0x80
	ctx.r[4].s64 = ctx.r[1].s64 + 128;
	// 82511364: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82511368: 4BFFFC21  bl 0x82510f88
	ctx.lr = 0x8251136C;
	sub_82510F88(ctx, base);
	// 8251136C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82511370: 48000008  b 0x82511378
	pc = 0x82511378; continue 'dispatch;
	// 82511374: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82511378: 93E10058  stw r31, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[31].u32 ) };
	// 8251137C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82511380: 3861005C  addi r3, r1, 0x5c
	ctx.r[3].s64 = ctx.r[1].s64 + 92;
	// 82511384: 4BFFFE7D  bl 0x82511200
	ctx.lr = 0x82511388;
	sub_82511200(ctx, base);
	// 82511388: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8251138C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82511390: 3861005C  addi r3, r1, 0x5c
	ctx.r[3].s64 = ctx.r[1].s64 + 92;
	// 82511394: 4BDAEC6D  bl 0x822c0000
	ctx.lr = 0x82511398;
	sub_822C0000(ctx, base);
	// 82511398: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8251139C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825113A0: 488E2861  bl 0x82df3c00
	ctx.lr = 0x825113A4;
	sub_82DF3C00(ctx, base);
	// 825113A4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825113A8: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 825113AC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825113B0: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 825113B4: 4857A45D  bl 0x82a8b810
	ctx.lr = 0x825113B8;
	sub_82A8B810(ctx, base);
	// 825113B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825113BC: 488E206D  bl 0x82df3428
	ctx.lr = 0x825113C0;
	sub_82DF3428(ctx, base);
	// 825113C0: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 825113C4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825113C8: 419A0008  beq cr6, 0x825113d0
	if ctx.cr[6].eq {
	pc = 0x825113D0; continue 'dispatch;
	}
	// 825113CC: 4BDAF4C5  bl 0x822c0890
	ctx.lr = 0x825113D0;
	sub_822C0890(ctx, base);
	// 825113D0: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 825113D4: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 825113D8: 486743D9  bl 0x82b857b0
	ctx.lr = 0x825113DC;
	sub_82B857B0(ctx, base);
	// 825113DC: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 825113E0: 389D0004  addi r4, r29, 4
	ctx.r[4].s64 = ctx.r[29].s64 + 4;
	// 825113E4: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 825113E8: 486755D9  bl 0x82b869c0
	ctx.lr = 0x825113EC;
	sub_82B869C0(ctx, base);
	// 825113EC: 80610068  lwz r3, 0x68(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 825113F0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825113F4: 419A0008  beq cr6, 0x825113fc
	if ctx.cr[6].eq {
	pc = 0x825113FC; continue 'dispatch;
	}
	// 825113F8: 4BDAF499  bl 0x822c0890
	ctx.lr = 0x825113FC;
	sub_822C0890(ctx, base);
	// 825113FC: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82511400: 488E2029  bl 0x82df3428
	ctx.lr = 0x82511404;
	sub_82DF3428(ctx, base);
	// 82511404: 80610078  lwz r3, 0x78(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(120 as u32) ) } as u64;
	// 82511408: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8251140C: 419A0008  beq cr6, 0x82511414
	if ctx.cr[6].eq {
	pc = 0x82511414; continue 'dispatch;
	}
	// 82511410: 4BDAF481  bl 0x822c0890
	ctx.lr = 0x82511414;
	sub_822C0890(ctx, base);
	// 82511414: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82511418: 488E2011  bl 0x82df3428
	ctx.lr = 0x8251141C;
	sub_82DF3428(ctx, base);
	// 8251141C: 48000058  b 0x82511474
	pc = 0x82511474; continue 'dispatch;
	// 82511420: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82511424: 389D0004  addi r4, r29, 4
	ctx.r[4].s64 = ctx.r[29].s64 + 4;
	// 82511428: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8251142C: 482E539D  bl 0x827f67c8
	ctx.lr = 0x82511430;
	sub_827F67C8(ctx, base);
	// 82511430: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82511434: 815D0008  lwz r10, 8(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82511438: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 8251143C: 419A0038  beq cr6, 0x82511474
	if ctx.cr[6].eq {
	pc = 0x82511474; continue 'dispatch;
	}
	// 82511440: 806B0010  lwz r3, 0x10(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82511444: 3D408328  lis r10, -0x7cd8
	ctx.r[10].s64 = -2094530560;
	// 82511448: 3D608328  lis r11, -0x7cd8
	ctx.r[11].s64 = -2094530560;
	// 8251144C: 38CAC7F8  addi r6, r10, -0x3808
	ctx.r[6].s64 = ctx.r[10].s64 + -14344;
	// 82511450: 38ABBE54  addi r5, r11, -0x41ac
	ctx.r[5].s64 = ctx.r[11].s64 + -16812;
	// 82511454: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82511458: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8251145C: 48C98AED  bl 0x831a9f48
	ctx.lr = 0x82511460;
	sub_831A9F48(ctx, base);
	// 82511460: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82511464: 41820010  beq 0x82511474
	if ctx.cr[0].eq {
	pc = 0x82511474; continue 'dispatch;
	}
	// 82511468: 38830004  addi r4, r3, 4
	ctx.r[4].s64 = ctx.r[3].s64 + 4;
	// 8251146C: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82511470: 4803FD19  bl 0x82551188
	ctx.lr = 0x82511474;
	sub_82551188(ctx, base);
	// 82511474: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 82511478: 48C96D3C  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82511480(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82511480 size=148
    let mut pc: u32 = 0x82511480;
    'dispatch: loop {
        match pc {
            0x82511480 => {
    //   block [0x82511480..0x82511514)
	// 82511480: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82511484: 48C96CE9  bl 0x831a816c
	ctx.lr = 0x82511488;
	sub_831A8130(ctx, base);
	// 82511488: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251148C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82511490: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82511494: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82511498: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8251149C: 388B19D8  addi r4, r11, 0x19d8
	ctx.r[4].s64 = ctx.r[11].s64 + 6616;
	// 825114A0: 38A00060  li r5, 0x60
	ctx.r[5].s64 = 96;
	// 825114A4: 38600014  li r3, 0x14
	ctx.r[3].s64 = 20;
	// 825114A8: 4BDAEF31  bl 0x822c03d8
	ctx.lr = 0x825114AC;
	sub_822C03D8(ctx, base);
	// 825114AC: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 825114B0: 41820030  beq 0x825114e0
	if ctx.cr[0].eq {
	pc = 0x825114E0; continue 'dispatch;
	}
	// 825114B4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825114B8: 4BF23B59  bl 0x82435010
	ctx.lr = 0x825114BC;
	sub_82435010(ctx, base);
	// 825114BC: 389F0004  addi r4, r31, 4
	ctx.r[4].s64 = ctx.r[31].s64 + 4;
	// 825114C0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825114C4: 4803E655  bl 0x8254fb18
	ctx.lr = 0x825114C8;
	sub_8254FB18(ctx, base);
	// 825114C8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 825114CC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825114D0: 80BF0010  lwz r5, 0x10(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 825114D4: 4BFFFAB5  bl 0x82510f88
	ctx.lr = 0x825114D8;
	sub_82510F88(ctx, base);
	// 825114D8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825114DC: 48000008  b 0x825114e4
	pc = 0x825114E4; continue 'dispatch;
	// 825114E0: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 825114E4: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 825114E8: 3BDD0004  addi r30, r29, 4
	ctx.r[30].s64 = ctx.r[29].s64 + 4;
	// 825114EC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825114F0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825114F4: 4BFFFD0D  bl 0x82511200
	ctx.lr = 0x825114F8;
	sub_82511200(ctx, base);
	// 825114F8: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 825114FC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82511500: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82511504: 4BDAEAFD  bl 0x822c0000
	ctx.lr = 0x82511508;
	sub_822C0000(ctx, base);
	// 82511508: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8251150C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82511510: 48C96CAC  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82511518(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82511518 size=116
    let mut pc: u32 = 0x82511518;
    'dispatch: loop {
        match pc {
            0x82511518 => {
    //   block [0x82511518..0x8251158C)
	// 82511518: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251151C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82511520: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82511524: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82511528: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251152C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82511530: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82511534: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82511538: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 8251153C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82511540: 4E800421  bctrl
	ctx.lr = 0x82511544;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82511544: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82511548: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8251154C: 388B1BC4  addi r4, r11, 0x1bc4
	ctx.r[4].s64 = ctx.r[11].s64 + 7108;
	// 82511550: 488E24B9  bl 0x82df3a08
	ctx.lr = 0x82511554;
	sub_82DF3A08(ctx, base);
	// 82511554: 817F00BC  lwz r11, 0xbc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(188 as u32) ) } as u64;
	// 82511558: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 8251155C: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82511560: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82511564: 38AB0024  addi r5, r11, 0x24
	ctx.r[5].s64 = ctx.r[11].s64 + 36;
	// 82511568: 4BFFFD99  bl 0x82511300
	ctx.lr = 0x8251156C;
	sub_82511300(ctx, base);
	// 8251156C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82511570: 488E1EB9  bl 0x82df3428
	ctx.lr = 0x82511574;
	sub_82DF3428(ctx, base);
	// 82511574: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82511578: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8251157C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82511580: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82511584: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82511588: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82511590(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82511590 size=8
    let mut pc: u32 = 0x82511590;
    'dispatch: loop {
        match pc {
            0x82511590 => {
    //   block [0x82511590..0x82511598)
	// 82511590: 38840018  addi r4, r4, 0x18
	ctx.r[4].s64 = ctx.r[4].s64 + 24;
	// 82511594: 4BFFFF84  b 0x82511518
	sub_82511518(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82511598(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82511598 size=724
    let mut pc: u32 = 0x82511598;
    'dispatch: loop {
        match pc {
            0x82511598 => {
    //   block [0x82511598..0x8251186C)
	// 82511598: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251159C: 48C96BCD  bl 0x831a8168
	ctx.lr = 0x825115A0;
	sub_831A8130(ctx, base);
	// 825115A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825115A4: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 825115A8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 825115AC: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 825115B0: 579D063F  clrlwi. r29, r28, 0x18
	ctx.r[29].u64 = ctx.r[28].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 825115B4: 41820038  beq 0x825115ec
	if ctx.cr[0].eq {
	pc = 0x825115EC; continue 'dispatch;
	}
	// 825115B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825115BC: 48C983CD  bl 0x831a9988
	ctx.lr = 0x825115C0;
	sub_831A9988(ctx, base);
	// 825115C0: 3D608327  lis r11, -0x7cd9
	ctx.r[11].s64 = -2094596096;
	// 825115C4: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 825115C8: 386BF464  addi r3, r11, -0xb9c
	ctx.r[3].s64 = ctx.r[11].s64 + -2972;
	// 825115CC: 48C96B2D  bl 0x831a80f8
	ctx.lr = 0x825115D0;
	sub_831A80F8(ctx, base);
	// 825115D0: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825115D4: 41820018  beq 0x825115ec
	if ctx.cr[0].eq {
	pc = 0x825115EC; continue 'dispatch;
	}
	// 825115D8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825115DC: 387EFFD8  addi r3, r30, -0x28
	ctx.r[3].s64 = ctx.r[30].s64 + -40;
	// 825115E0: 485FB1C1  bl 0x82b0c7a0
	ctx.lr = 0x825115E4;
	sub_82B0C7A0(ctx, base);
	// 825115E4: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 825115E8: 4800027C  b 0x82511864
	pc = 0x82511864; continue 'dispatch;
	// 825115EC: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 825115F0: 419A0264  beq cr6, 0x82511854
	if ctx.cr[6].eq {
	pc = 0x82511854; continue 'dispatch;
	}
	// 825115F4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825115F8: 48C98391  bl 0x831a9988
	ctx.lr = 0x825115FC;
	sub_831A9988(ctx, base);
	// 825115FC: 3D608325  lis r11, -0x7cdb
	ctx.r[11].s64 = -2094727168;
	// 82511600: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82511604: 386B75A4  addi r3, r11, 0x75a4
	ctx.r[3].s64 = ctx.r[11].s64 + 30116;
	// 82511608: 48C96AF1  bl 0x831a80f8
	ctx.lr = 0x8251160C;
	sub_831A80F8(ctx, base);
	// 8251160C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82511610: 41820014  beq 0x82511624
	if ctx.cr[0].eq {
	pc = 0x82511624; continue 'dispatch;
	}
	// 82511614: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82511618: 387EFFD8  addi r3, r30, -0x28
	ctx.r[3].s64 = ctx.r[30].s64 + -40;
	// 8251161C: 485FB185  bl 0x82b0c7a0
	ctx.lr = 0x82511620;
	sub_82B0C7A0(ctx, base);
	// 82511620: 4BFFFFC4  b 0x825115e4
	pc = 0x825115E4; continue 'dispatch;
	// 82511624: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82511628: 419A022C  beq cr6, 0x82511854
	if ctx.cr[6].eq {
	pc = 0x82511854; continue 'dispatch;
	}
	// 8251162C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82511630: 48C98359  bl 0x831a9988
	ctx.lr = 0x82511634;
	sub_831A9988(ctx, base);
	// 82511634: 3D608328  lis r11, -0x7cd8
	ctx.r[11].s64 = -2094530560;
	// 82511638: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8251163C: 386BC90C  addi r3, r11, -0x36f4
	ctx.r[3].s64 = ctx.r[11].s64 + -14068;
	// 82511640: 48C96AB9  bl 0x831a80f8
	ctx.lr = 0x82511644;
	sub_831A80F8(ctx, base);
	// 82511644: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82511648: 41820014  beq 0x8251165c
	if ctx.cr[0].eq {
	pc = 0x8251165C; continue 'dispatch;
	}
	// 8251164C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82511650: 387EFFD8  addi r3, r30, -0x28
	ctx.r[3].s64 = ctx.r[30].s64 + -40;
	// 82511654: 4BFFDF6D  bl 0x8250f5c0
	ctx.lr = 0x82511658;
	sub_8250F5C0(ctx, base);
	// 82511658: 4BFFFF8C  b 0x825115e4
	pc = 0x825115E4; continue 'dispatch;
	// 8251165C: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82511660: 419A01F4  beq cr6, 0x82511854
	if ctx.cr[6].eq {
	pc = 0x82511854; continue 'dispatch;
	}
	// 82511664: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82511668: 48C98321  bl 0x831a9988
	ctx.lr = 0x8251166C;
	sub_831A9988(ctx, base);
	// 8251166C: 3D608328  lis r11, -0x7cd8
	ctx.r[11].s64 = -2094530560;
	// 82511670: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82511674: 386BC8E4  addi r3, r11, -0x371c
	ctx.r[3].s64 = ctx.r[11].s64 + -14108;
	// 82511678: 48C96A81  bl 0x831a80f8
	ctx.lr = 0x8251167C;
	sub_831A80F8(ctx, base);
	// 8251167C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82511680: 41820014  beq 0x82511694
	if ctx.cr[0].eq {
	pc = 0x82511694; continue 'dispatch;
	}
	// 82511684: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82511688: 387EFFD8  addi r3, r30, -0x28
	ctx.r[3].s64 = ctx.r[30].s64 + -40;
	// 8251168C: 4BFFF00D  bl 0x82510698
	ctx.lr = 0x82511690;
	sub_82510698(ctx, base);
	// 82511690: 4BFFFF54  b 0x825115e4
	pc = 0x825115E4; continue 'dispatch;
	// 82511694: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82511698: 419A01BC  beq cr6, 0x82511854
	if ctx.cr[6].eq {
	pc = 0x82511854; continue 'dispatch;
	}
	// 8251169C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825116A0: 48C982E9  bl 0x831a9988
	ctx.lr = 0x825116A4;
	sub_831A9988(ctx, base);
	// 825116A4: 3D608328  lis r11, -0x7cd8
	ctx.r[11].s64 = -2094530560;
	// 825116A8: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 825116AC: 386BC8B0  addi r3, r11, -0x3750
	ctx.r[3].s64 = ctx.r[11].s64 + -14160;
	// 825116B0: 48C96A49  bl 0x831a80f8
	ctx.lr = 0x825116B4;
	sub_831A80F8(ctx, base);
	// 825116B4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825116B8: 41820014  beq 0x825116cc
	if ctx.cr[0].eq {
	pc = 0x825116CC; continue 'dispatch;
	}
	// 825116BC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825116C0: 387EFFD8  addi r3, r30, -0x28
	ctx.r[3].s64 = ctx.r[30].s64 + -40;
	// 825116C4: 4BFFF2AD  bl 0x82510970
	ctx.lr = 0x825116C8;
	sub_82510970(ctx, base);
	// 825116C8: 4BFFFF1C  b 0x825115e4
	pc = 0x825115E4; continue 'dispatch;
	// 825116CC: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 825116D0: 419A0184  beq cr6, 0x82511854
	if ctx.cr[6].eq {
	pc = 0x82511854; continue 'dispatch;
	}
	// 825116D4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825116D8: 48C982B1  bl 0x831a9988
	ctx.lr = 0x825116DC;
	sub_831A9988(ctx, base);
	// 825116DC: 3D608328  lis r11, -0x7cd8
	ctx.r[11].s64 = -2094530560;
	// 825116E0: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 825116E4: 386BC880  addi r3, r11, -0x3780
	ctx.r[3].s64 = ctx.r[11].s64 + -14208;
	// 825116E8: 48C96A11  bl 0x831a80f8
	ctx.lr = 0x825116EC;
	sub_831A80F8(ctx, base);
	// 825116EC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825116F0: 41820014  beq 0x82511704
	if ctx.cr[0].eq {
	pc = 0x82511704; continue 'dispatch;
	}
	// 825116F4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825116F8: 387EFFD8  addi r3, r30, -0x28
	ctx.r[3].s64 = ctx.r[30].s64 + -40;
	// 825116FC: 4BFFFE95  bl 0x82511590
	ctx.lr = 0x82511700;
	sub_82511590(ctx, base);
	// 82511700: 4BFFFEE4  b 0x825115e4
	pc = 0x825115E4; continue 'dispatch;
	// 82511704: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82511708: 419A014C  beq cr6, 0x82511854
	if ctx.cr[6].eq {
	pc = 0x82511854; continue 'dispatch;
	}
	// 8251170C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82511710: 48C98279  bl 0x831a9988
	ctx.lr = 0x82511714;
	sub_831A9988(ctx, base);
	// 82511714: 3D608325  lis r11, -0x7cdb
	ctx.r[11].s64 = -2094727168;
	// 82511718: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8251171C: 386B5A60  addi r3, r11, 0x5a60
	ctx.r[3].s64 = ctx.r[11].s64 + 23136;
	// 82511720: 48C969D9  bl 0x831a80f8
	ctx.lr = 0x82511724;
	sub_831A80F8(ctx, base);
	// 82511724: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82511728: 41820014  beq 0x8251173c
	if ctx.cr[0].eq {
	pc = 0x8251173C; continue 'dispatch;
	}
	// 8251172C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82511730: 387EFFD8  addi r3, r30, -0x28
	ctx.r[3].s64 = ctx.r[30].s64 + -40;
	// 82511734: 4BFFE1F5  bl 0x8250f928
	ctx.lr = 0x82511738;
	sub_8250F928(ctx, base);
	// 82511738: 4BFFFEAC  b 0x825115e4
	pc = 0x825115E4; continue 'dispatch;
	// 8251173C: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82511740: 419A0114  beq cr6, 0x82511854
	if ctx.cr[6].eq {
	pc = 0x82511854; continue 'dispatch;
	}
	// 82511744: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82511748: 48C98241  bl 0x831a9988
	ctx.lr = 0x8251174C;
	sub_831A9988(ctx, base);
	// 8251174C: 3D608328  lis r11, -0x7cd8
	ctx.r[11].s64 = -2094530560;
	// 82511750: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82511754: 386BC854  addi r3, r11, -0x37ac
	ctx.r[3].s64 = ctx.r[11].s64 + -14252;
	// 82511758: 48C969A1  bl 0x831a80f8
	ctx.lr = 0x8251175C;
	sub_831A80F8(ctx, base);
	// 8251175C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82511760: 41820014  beq 0x82511774
	if ctx.cr[0].eq {
	pc = 0x82511774; continue 'dispatch;
	}
	// 82511764: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82511768: 387EFFD8  addi r3, r30, -0x28
	ctx.r[3].s64 = ctx.r[30].s64 + -40;
	// 8251176C: 4BFFF755  bl 0x82510ec0
	ctx.lr = 0x82511770;
	sub_82510EC0(ctx, base);
	// 82511770: 4BFFFE74  b 0x825115e4
	pc = 0x825115E4; continue 'dispatch;
	// 82511774: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82511778: 419A00DC  beq cr6, 0x82511854
	if ctx.cr[6].eq {
	pc = 0x82511854; continue 'dispatch;
	}
	// 8251177C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82511780: 48C98209  bl 0x831a9988
	ctx.lr = 0x82511784;
	sub_831A9988(ctx, base);
	// 82511784: 3D608325  lis r11, -0x7cdb
	ctx.r[11].s64 = -2094727168;
	// 82511788: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8251178C: 386B595C  addi r3, r11, 0x595c
	ctx.r[3].s64 = ctx.r[11].s64 + 22876;
	// 82511790: 48C96969  bl 0x831a80f8
	ctx.lr = 0x82511794;
	sub_831A80F8(ctx, base);
	// 82511794: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82511798: 41820014  beq 0x825117ac
	if ctx.cr[0].eq {
	pc = 0x825117AC; continue 'dispatch;
	}
	// 8251179C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825117A0: 387EFFD8  addi r3, r30, -0x28
	ctx.r[3].s64 = ctx.r[30].s64 + -40;
	// 825117A4: 4BFFE515  bl 0x8250fcb8
	ctx.lr = 0x825117A8;
	sub_8250FCB8(ctx, base);
	// 825117A8: 4BFFFE3C  b 0x825115e4
	pc = 0x825115E4; continue 'dispatch;
	// 825117AC: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 825117B0: 419A00A4  beq cr6, 0x82511854
	if ctx.cr[6].eq {
	pc = 0x82511854; continue 'dispatch;
	}
	// 825117B4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825117B8: 48C981D1  bl 0x831a9988
	ctx.lr = 0x825117BC;
	sub_831A9988(ctx, base);
	// 825117BC: 3D608325  lis r11, -0x7cdb
	ctx.r[11].s64 = -2094727168;
	// 825117C0: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 825117C4: 386B59D8  addi r3, r11, 0x59d8
	ctx.r[3].s64 = ctx.r[11].s64 + 23000;
	// 825117C8: 48C96931  bl 0x831a80f8
	ctx.lr = 0x825117CC;
	sub_831A80F8(ctx, base);
	// 825117CC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825117D0: 41820014  beq 0x825117e4
	if ctx.cr[0].eq {
	pc = 0x825117E4; continue 'dispatch;
	}
	// 825117D4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825117D8: 387EFFD8  addi r3, r30, -0x28
	ctx.r[3].s64 = ctx.r[30].s64 + -40;
	// 825117DC: 4BFFE69D  bl 0x8250fe78
	ctx.lr = 0x825117E0;
	sub_8250FE78(ctx, base);
	// 825117E0: 4BFFFE04  b 0x825115e4
	pc = 0x825115E4; continue 'dispatch;
	// 825117E4: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 825117E8: 419A006C  beq cr6, 0x82511854
	if ctx.cr[6].eq {
	pc = 0x82511854; continue 'dispatch;
	}
	// 825117EC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825117F0: 48C98199  bl 0x831a9988
	ctx.lr = 0x825117F4;
	sub_831A9988(ctx, base);
	// 825117F4: 3D608327  lis r11, -0x7cd9
	ctx.r[11].s64 = -2094596096;
	// 825117F8: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 825117FC: 386B5514  addi r3, r11, 0x5514
	ctx.r[3].s64 = ctx.r[11].s64 + 21780;
	// 82511800: 48C968F9  bl 0x831a80f8
	ctx.lr = 0x82511804;
	sub_831A80F8(ctx, base);
	// 82511804: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82511808: 41820014  beq 0x8251181c
	if ctx.cr[0].eq {
	pc = 0x8251181C; continue 'dispatch;
	}
	// 8251180C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82511810: 387EFFD8  addi r3, r30, -0x28
	ctx.r[3].s64 = ctx.r[30].s64 + -40;
	// 82511814: 4BFFDE0D  bl 0x8250f620
	ctx.lr = 0x82511818;
	sub_8250F620(ctx, base);
	// 82511818: 4BFFFDCC  b 0x825115e4
	pc = 0x825115E4; continue 'dispatch;
	// 8251181C: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82511820: 419A0034  beq cr6, 0x82511854
	if ctx.cr[6].eq {
	pc = 0x82511854; continue 'dispatch;
	}
	// 82511824: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82511828: 48C98161  bl 0x831a9988
	ctx.lr = 0x8251182C;
	sub_831A9988(ctx, base);
	// 8251182C: 3D608327  lis r11, -0x7cd9
	ctx.r[11].s64 = -2094596096;
	// 82511830: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82511834: 386B54E4  addi r3, r11, 0x54e4
	ctx.r[3].s64 = ctx.r[11].s64 + 21732;
	// 82511838: 48C968C1  bl 0x831a80f8
	ctx.lr = 0x8251183C;
	sub_831A80F8(ctx, base);
	// 8251183C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82511840: 41820014  beq 0x82511854
	if ctx.cr[0].eq {
	pc = 0x82511854; continue 'dispatch;
	}
	// 82511844: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82511848: 387EFFD8  addi r3, r30, -0x28
	ctx.r[3].s64 = ctx.r[30].s64 + -40;
	// 8251184C: 4BFFDE4D  bl 0x8250f698
	ctx.lr = 0x82511850;
	sub_8250F698(ctx, base);
	// 82511850: 4BFFFD94  b 0x825115e4
	pc = 0x825115E4; continue 'dispatch;
	// 82511854: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82511858: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8251185C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82511860: 48C6CC29  bl 0x8317e488
	ctx.lr = 0x82511864;
	sub_8317E488(ctx, base);
	// 82511864: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82511868: 48C96950  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82511870(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82511870 size=28
    let mut pc: u32 = 0x82511870;
    'dispatch: loop {
        match pc {
            0x82511870 => {
    //   block [0x82511870..0x8251188C)
	// 82511870: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82511874: 81640018  lwz r11, 0x18(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(24 as u32) ) } as u64;
	// 82511878: C00300D4  lfs f0, 0xd4(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8251187C: C1AA08A4  lfs f13, 0x8a4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2212 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82511880: 916300C0  stw r11, 0xc0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(192 as u32), ctx.r[11].u32 ) };
	// 82511884: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 82511888: 4C990020  blelr cr6
	if !ctx.cr[6].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251188C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8251188C size=8
    let mut pc: u32 = 0x8251188C;
    'dispatch: loop {
        match pc {
            0x8251188C => {
    //   block [0x8251188C..0x82511894)
	// 8251188C: D00B0068  stfs f0, 0x68(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(104 as u32), tmp.u32 ) };
	// 82511890: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82511898(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82511898 size=16
    let mut pc: u32 = 0x82511898;
    'dispatch: loop {
        match pc {
            0x82511898 => {
    //   block [0x82511898..0x825118A8)
	// 82511898: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8251189C: 806400C0  lwz r3, 0xc0(r4)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(192 as u32) ) } as u64;
	// 825118A0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825118A4: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825118A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825118A8 size=8
    let mut pc: u32 = 0x825118A8;
    'dispatch: loop {
        match pc {
            0x825118A8 => {
    //   block [0x825118A8..0x825118B0)
	// 825118A8: 482A2CF8  b 0x827b45a0
	sub_827B45A0(ctx, base);
	return;
	// 825118AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825118B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x825118B0 size=16
    let mut pc: u32 = 0x825118B0;
    'dispatch: loop {
        match pc {
            0x825118B0 => {
    //   block [0x825118B0..0x825118C0)
	// 825118B0: 816300C0  lwz r11, 0xc0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(192 as u32) ) } as u64;
	// 825118B4: D02300D4  stfs f1, 0xd4(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(212 as u32), tmp.u32 ) };
	// 825118B8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825118BC: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825118C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x825118C0 size=8
    let mut pc: u32 = 0x825118C0;
    'dispatch: loop {
        match pc {
            0x825118C0 => {
    //   block [0x825118C0..0x825118C8)
	// 825118C0: D02B0068  stfs f1, 0x68(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(104 as u32), tmp.u32 ) };
	// 825118C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825118C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825118C8 size=136
    let mut pc: u32 = 0x825118C8;
    'dispatch: loop {
        match pc {
            0x825118C8 => {
    //   block [0x825118C8..0x82511950)
	// 825118C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825118CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825118D0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825118D4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825118D8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825118DC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 825118E0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 825118E4: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 825118E8: 409A0020  bne cr6, 0x82511908
	if !ctx.cr[6].eq {
	pc = 0x82511908; continue 'dispatch;
	}
	// 825118EC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 825118F0: 419A0048  beq cr6, 0x82511938
	if ctx.cr[6].eq {
	pc = 0x82511938; continue 'dispatch;
	}
	// 825118F4: E97E0000  ld r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) };
	// 825118F8: F97F0000  std r11, 0(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u64 ) };
	// 825118FC: E97E0008  ld r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) };
	// 82511900: F97F0008  std r11, 8(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 82511904: 48000034  b 0x82511938
	pc = 0x82511938; continue 'dispatch;
	// 82511908: 2F050001  cmpwi cr6, r5, 1
	ctx.cr[6].compare_i32(ctx.r[5].s32, 1, &mut ctx.xer);
	// 8251190C: 419A002C  beq cr6, 0x82511938
	if ctx.cr[6].eq {
	pc = 0x82511938; continue 'dispatch;
	}
	// 82511910: 3D608328  lis r11, -0x7cd8
	ctx.r[11].s64 = -2094530560;
	// 82511914: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82511918: 388BC938  addi r4, r11, -0x36c8
	ctx.r[4].s64 = ctx.r[11].s64 + -14024;
	// 8251191C: 48C967DD  bl 0x831a80f8
	ctx.lr = 0x82511920;
	sub_831A80F8(ctx, base);
	// 82511920: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82511924: 4182000C  beq 0x82511930
	if ctx.cr[0].eq {
	pc = 0x82511930; continue 'dispatch;
	}
	// 82511928: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 8251192C: 4800000C  b 0x82511938
	pc = 0x82511938; continue 'dispatch;
	// 82511930: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82511934: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82511938: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8251193C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82511940: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82511944: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82511948: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8251194C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82511950(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82511950 size=136
    let mut pc: u32 = 0x82511950;
    'dispatch: loop {
        match pc {
            0x82511950 => {
    //   block [0x82511950..0x825119D8)
	// 82511950: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82511954: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82511958: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8251195C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82511960: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82511964: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82511968: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8251196C: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 82511970: 409A0020  bne cr6, 0x82511990
	if !ctx.cr[6].eq {
	pc = 0x82511990; continue 'dispatch;
	}
	// 82511974: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82511978: 419A0048  beq cr6, 0x825119c0
	if ctx.cr[6].eq {
	pc = 0x825119C0; continue 'dispatch;
	}
	// 8251197C: E97E0000  ld r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) };
	// 82511980: F97F0000  std r11, 0(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u64 ) };
	// 82511984: E97E0008  ld r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) };
	// 82511988: F97F0008  std r11, 8(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 8251198C: 48000034  b 0x825119c0
	pc = 0x825119C0; continue 'dispatch;
	// 82511990: 2F050001  cmpwi cr6, r5, 1
	ctx.cr[6].compare_i32(ctx.r[5].s32, 1, &mut ctx.xer);
	// 82511994: 419A002C  beq cr6, 0x825119c0
	if ctx.cr[6].eq {
	pc = 0x825119C0; continue 'dispatch;
	}
	// 82511998: 3D608328  lis r11, -0x7cd8
	ctx.r[11].s64 = -2094530560;
	// 8251199C: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825119A0: 388BCA00  addi r4, r11, -0x3600
	ctx.r[4].s64 = ctx.r[11].s64 + -13824;
	// 825119A4: 48C96755  bl 0x831a80f8
	ctx.lr = 0x825119A8;
	sub_831A80F8(ctx, base);
	// 825119A8: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825119AC: 4182000C  beq 0x825119b8
	if ctx.cr[0].eq {
	pc = 0x825119B8; continue 'dispatch;
	}
	// 825119B0: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 825119B4: 4800000C  b 0x825119c0
	pc = 0x825119C0; continue 'dispatch;
	// 825119B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825119BC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825119C0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825119C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825119C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825119CC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825119D0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825119D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825119D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825119D8 size=76
    let mut pc: u32 = 0x825119D8;
    'dispatch: loop {
        match pc {
            0x825119D8 => {
    //   block [0x825119D8..0x82511A24)
	// 825119D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825119DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825119E0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825119E4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825119E8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825119EC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 825119F0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825119F4: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 825119F8: 4BFFD979  bl 0x8250f370
	ctx.lr = 0x825119FC;
	sub_8250F370(ctx, base);
	// 825119FC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82511A00: 80BF00C4  lwz r5, 0xc4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(196 as u32) ) } as u64;
	// 82511A04: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82511A08: 4BFF7199  bl 0x82508ba0
	ctx.lr = 0x82511A0C;
	sub_82508BA0(ctx, base);
	// 82511A0C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82511A10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82511A14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82511A18: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82511A1C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82511A20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82511A28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82511A28 size=84
    let mut pc: u32 = 0x82511A28;
    'dispatch: loop {
        match pc {
            0x82511A28 => {
    //   block [0x82511A28..0x82511A7C)
	// 82511A28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82511A2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82511A30: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82511A34: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82511A38: 806300C4  lwz r3, 0xc4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(196 as u32) ) } as u64;
	// 82511A3C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82511A40: 4BF75079  bl 0x82486ab8
	ctx.lr = 0x82511A44;
	sub_82486AB8(ctx, base);
	// 82511A44: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82511A48: C0030000  lfs f0, 0(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82511A4C: D00B0000  stfs f0, 0(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82511A50: C0030004  lfs f0, 4(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82511A54: D00B0004  stfs f0, 4(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82511A58: C0030008  lfs f0, 8(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82511A5C: D00B0008  stfs f0, 8(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82511A60: C003000C  lfs f0, 0xc(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82511A64: D00B000C  stfs f0, 0xc(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 82511A68: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82511A6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82511A70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82511A74: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82511A78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82511A80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82511A80 size=100
    let mut pc: u32 = 0x82511A80;
    'dispatch: loop {
        match pc {
            0x82511A80 => {
    //   block [0x82511A80..0x82511AE4)
	// 82511A80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82511A84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82511A88: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82511A8C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82511A90: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82511A94: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82511A98: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82511A9C: 807F00C4  lwz r3, 0xc4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(196 as u32) ) } as u64;
	// 82511AA0: 480793C9  bl 0x8258ae68
	ctx.lr = 0x82511AA4;
	sub_8258AE68(ctx, base);
	// 82511AA4: 807F00C0  lwz r3, 0xc0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(192 as u32) ) } as u64;
	// 82511AA8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82511AAC: 419A000C  beq cr6, 0x82511ab8
	if ctx.cr[6].eq {
	pc = 0x82511AB8; continue 'dispatch;
	}
	// 82511AB0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82511AB4: 482A29F5  bl 0x827b44a8
	ctx.lr = 0x82511AB8;
	sub_827B44A8(ctx, base);
	// 82511AB8: 807F00D8  lwz r3, 0xd8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(216 as u32) ) } as u64;
	// 82511ABC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82511AC0: 419A000C  beq cr6, 0x82511acc
	if ctx.cr[6].eq {
	pc = 0x82511ACC; continue 'dispatch;
	}
	// 82511AC4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82511AC8: 4807A2C1  bl 0x8258bd88
	ctx.lr = 0x82511ACC;
	sub_8258BD88(ctx, base);
	// 82511ACC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82511AD0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82511AD4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82511AD8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82511ADC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82511AE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82511AE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82511AE8 size=8
    let mut pc: u32 = 0x82511AE8;
    'dispatch: loop {
        match pc {
            0x82511AE8 => {
    //   block [0x82511AE8..0x82511AF0)
	// 82511AE8: 806300C4  lwz r3, 0xc4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(196 as u32) ) } as u64;
	// 82511AEC: 482E38E4  b 0x827f53d0
	sub_827F53D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82511AF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82511AF0 size=104
    let mut pc: u32 = 0x82511AF0;
    'dispatch: loop {
        match pc {
            0x82511AF0 => {
    //   block [0x82511AF0..0x82511B58)
	// 82511AF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82511AF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82511AF8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82511AFC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82511B00: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82511B04: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82511B08: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82511B0C: 807E00C4  lwz r3, 0xc4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(196 as u32) ) } as u64;
	// 82511B10: 480793A1  bl 0x8258aeb0
	ctx.lr = 0x82511B14;
	sub_8258AEB0(ctx, base);
	// 82511B14: 817E00C0  lwz r11, 0xc0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(192 as u32) ) } as u64;
	// 82511B18: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82511B1C: 419A0024  beq cr6, 0x82511b40
	if ctx.cr[6].eq {
	pc = 0x82511B40; continue 'dispatch;
	}
	// 82511B20: C01F0000  lfs f0, 0(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82511B24: D00B0040  stfs f0, 0x40(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(64 as u32), tmp.u32 ) };
	// 82511B28: C01F0004  lfs f0, 4(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82511B2C: D00B0044  stfs f0, 0x44(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(68 as u32), tmp.u32 ) };
	// 82511B30: C01F0008  lfs f0, 8(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82511B34: D00B0048  stfs f0, 0x48(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(72 as u32), tmp.u32 ) };
	// 82511B38: C01F000C  lfs f0, 0xc(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82511B3C: D00B004C  stfs f0, 0x4c(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(76 as u32), tmp.u32 ) };
	// 82511B40: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82511B44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82511B48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82511B4C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82511B50: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82511B54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82511B58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82511B58 size=8
    let mut pc: u32 = 0x82511B58;
    'dispatch: loop {
        match pc {
            0x82511B58 => {
    //   block [0x82511B58..0x82511B60)
	// 82511B58: 806300C4  lwz r3, 0xc4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(196 as u32) ) } as u64;
	// 82511B5C: 4BF74F5C  b 0x82486ab8
	sub_82486AB8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82511B60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82511B60 size=112
    let mut pc: u32 = 0x82511B60;
    'dispatch: loop {
        match pc {
            0x82511B60 => {
    //   block [0x82511B60..0x82511BD0)
	// 82511B60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82511B64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82511B68: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82511B6C: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82511B70: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82511B74: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82511B78: 388B0020  addi r4, r11, 0x20
	ctx.r[4].s64 = ctx.r[11].s64 + 32;
	// 82511B7C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82511B80: C00B0050  lfs f0, 0x50(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(80 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82511B84: C1AB0054  lfs f13, 0x54(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(84 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82511B88: C18B0058  lfs f12, 0x58(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(88 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82511B8C: C16B005C  lfs f11, 0x5c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(92 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82511B90: D0010050  stfs f0, 0x50(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 82511B94: D1A10054  stfs f13, 0x54(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82511B98: D1810058  stfs f12, 0x58(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), tmp.u32 ) };
	// 82511B9C: D161005C  stfs f11, 0x5c(r1)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), tmp.u32 ) };
	// 82511BA0: 4896B269  bl 0x82e7ce08
	ctx.lr = 0x82511BA4;
	sub_82E7CE08(ctx, base);
	// 82511BA4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82511BA8: 807F00C4  lwz r3, 0xc4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(196 as u32) ) } as u64;
	// 82511BAC: 480792BD  bl 0x8258ae68
	ctx.lr = 0x82511BB0;
	sub_8258AE68(ctx, base);
	// 82511BB0: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82511BB4: 807F00C4  lwz r3, 0xc4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(196 as u32) ) } as u64;
	// 82511BB8: 480792F9  bl 0x8258aeb0
	ctx.lr = 0x82511BBC;
	sub_8258AEB0(ctx, base);
	// 82511BBC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82511BC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82511BC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82511BC8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82511BCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82511BD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82511BD0 size=148
    let mut pc: u32 = 0x82511BD0;
    'dispatch: loop {
        match pc {
            0x82511BD0 => {
    //   block [0x82511BD0..0x82511C64)
	// 82511BD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82511BD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82511BD8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82511BDC: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82511BE0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82511BE4: 807F00C0  lwz r3, 0xc0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(192 as u32) ) } as u64;
	// 82511BE8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82511BEC: 419A0060  beq cr6, 0x82511c4c
	if ctx.cr[6].eq {
	pc = 0x82511C4C; continue 'dispatch;
	}
	// 82511BF0: 482A2939  bl 0x827b4528
	ctx.lr = 0x82511BF4;
	sub_827B4528(ctx, base);
	// 82511BF4: 813F00C0  lwz r9, 0xc0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(192 as u32) ) } as u64;
	// 82511BF8: 39600020  li r11, 0x20
	ctx.r[11].s64 = 32;
	// 82511BFC: 39410060  addi r10, r1, 0x60
	ctx.r[10].s64 = ctx.r[1].s64 + 96;
	// 82511C00: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82511C04: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82511C08: 13E958C7  vcmpequd (lvx128) v31, v9, v11
	tmp.u32 = ctx.r[9].u32 + ctx.r[11].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82511C68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82511C68 size=12
    let mut pc: u32 = 0x82511C68;
    'dispatch: loop {
        match pc {
            0x82511C68 => {
    //   block [0x82511C68..0x82511C74)
	// 82511C68: 806300CC  lwz r3, 0xcc(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(204 as u32) ) } as u64;
	// 82511C6C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82511C70: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82511C74(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82511C74 size=8
    let mut pc: u32 = 0x82511C74;
    'dispatch: loop {
        match pc {
            0x82511C74 => {
    //   block [0x82511C74..0x82511C7C)
	// 82511C74: 4BF7B2DC  b 0x8248cf50
	sub_8248CF50(ctx, base);
	return;
	// 82511C78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82511C80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82511C80 size=8
    let mut pc: u32 = 0x82511C80;
    'dispatch: loop {
        match pc {
            0x82511C80 => {
    //   block [0x82511C80..0x82511C88)
	// 82511C80: 806300CC  lwz r3, 0xcc(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(204 as u32) ) } as u64;
	// 82511C84: 4BF7B264  b 0x8248cee8
	sub_8248CEE8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82511C88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82511C88 size=12
    let mut pc: u32 = 0x82511C88;
    'dispatch: loop {
        match pc {
            0x82511C88 => {
    //   block [0x82511C88..0x82511C94)
	// 82511C88: 806300CC  lwz r3, 0xcc(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(204 as u32) ) } as u64;
	// 82511C8C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82511C90: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82511C94(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82511C94 size=8
    let mut pc: u32 = 0x82511C94;
    'dispatch: loop {
        match pc {
            0x82511C94 => {
    //   block [0x82511C94..0x82511C9C)
	// 82511C94: 4BF7B374  b 0x8248d008
	sub_8248D008(ctx, base);
	return;
	// 82511C98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82511CA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82511CA0 size=8
    let mut pc: u32 = 0x82511CA0;
    'dispatch: loop {
        match pc {
            0x82511CA0 => {
    //   block [0x82511CA0..0x82511CA8)
	// 82511CA0: 806300CC  lwz r3, 0xcc(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(204 as u32) ) } as u64;
	// 82511CA4: 4BF7B30C  b 0x8248cfb0
	sub_8248CFB0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82511CA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82511CA8 size=12
    let mut pc: u32 = 0x82511CA8;
    'dispatch: loop {
        match pc {
            0x82511CA8 => {
    //   block [0x82511CA8..0x82511CB4)
	// 82511CA8: 806300CC  lwz r3, 0xcc(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(204 as u32) ) } as u64;
	// 82511CAC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82511CB0: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82511CB4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82511CB4 size=8
    let mut pc: u32 = 0x82511CB4;
    'dispatch: loop {
        match pc {
            0x82511CB4 => {
    //   block [0x82511CB4..0x82511CBC)
	// 82511CB4: 4BF7B3FC  b 0x8248d0b0
	sub_8248D0B0(ctx, base);
	return;
	// 82511CB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82511CC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82511CC0 size=8
    let mut pc: u32 = 0x82511CC0;
    'dispatch: loop {
        match pc {
            0x82511CC0 => {
    //   block [0x82511CC0..0x82511CC8)
	// 82511CC0: 806300CC  lwz r3, 0xcc(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(204 as u32) ) } as u64;
	// 82511CC4: 4BF7B394  b 0x8248d058
	sub_8248D058(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82511CC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82511CC8 size=8
    let mut pc: u32 = 0x82511CC8;
    'dispatch: loop {
        match pc {
            0x82511CC8 => {
    //   block [0x82511CC8..0x82511CD0)
	// 82511CC8: 806300CC  lwz r3, 0xcc(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(204 as u32) ) } as u64;
	// 82511CCC: 4BF7AFC4  b 0x8248cc90
	sub_8248CC90(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82511CD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82511CD0 size=8
    let mut pc: u32 = 0x82511CD0;
    'dispatch: loop {
        match pc {
            0x82511CD0 => {
    //   block [0x82511CD0..0x82511CD8)
	// 82511CD0: 806300CC  lwz r3, 0xcc(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(204 as u32) ) } as u64;
	// 82511CD4: 4BF7B024  b 0x8248ccf8
	sub_8248CCF8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82511CD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82511CD8 size=8
    let mut pc: u32 = 0x82511CD8;
    'dispatch: loop {
        match pc {
            0x82511CD8 => {
    //   block [0x82511CD8..0x82511CE0)
	// 82511CD8: 806300CC  lwz r3, 0xcc(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(204 as u32) ) } as u64;
	// 82511CDC: 4BF7B424  b 0x8248d100
	sub_8248D100(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82511CE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82511CE0 size=60
    let mut pc: u32 = 0x82511CE0;
    'dispatch: loop {
        match pc {
            0x82511CE0 => {
    //   block [0x82511CE0..0x82511D1C)
	// 82511CE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82511CE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82511CE8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82511CEC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82511CF0: 806300C4  lwz r3, 0xc4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(196 as u32) ) } as u64;
	// 82511CF4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82511CF8: 482E36D9  bl 0x827f53d0
	ctx.lr = 0x82511CFC;
	sub_827F53D0(ctx, base);
	// 82511CFC: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82511D00: 13E018C7  vcmpequd (lvx128) v31, v0, v3
	tmp.u32 = ctx.r[3].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82511D20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82511D20 size=104
    let mut pc: u32 = 0x82511D20;
    'dispatch: loop {
        match pc {
            0x82511D20 => {
    //   block [0x82511D20..0x82511D88)
	// 82511D20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82511D24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82511D28: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82511D2C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82511D30: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82511D34: 807F00D8  lwz r3, 0xd8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(216 as u32) ) } as u64;
	// 82511D38: 4807BFD1  bl 0x8258dd08
	ctx.lr = 0x82511D3C;
	sub_8258DD08(ctx, base);
	// 82511D3C: 897F00E0  lbz r11, 0xe0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(224 as u32) ) } as u64;
	// 82511D40: 546A063E  clrlwi r10, r3, 0x18
	ctx.r[10].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82511D44: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82511D48: 419A002C  beq cr6, 0x82511d74
	if ctx.cr[6].eq {
	pc = 0x82511D74; continue 'dispatch;
	}
	// 82511D4C: 807F00D8  lwz r3, 0xd8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(216 as u32) ) } as u64;
	// 82511D50: 4807BFB9  bl 0x8258dd08
	ctx.lr = 0x82511D54;
	sub_8258DD08(ctx, base);
	// 82511D54: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82511D58: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 82511D5C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82511D60: 995F00E0  stb r10, 0xe0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(224 as u32), ctx.r[10].u8 ) };
	// 82511D64: 7D445378  mr r4, r10
	ctx.r[4].u64 = ctx.r[10].u64;
	// 82511D68: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 82511D6C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82511D70: 4E800421  bctrl
	ctx.lr = 0x82511D74;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82511D74: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82511D78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82511D7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82511D80: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82511D84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82511D88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82511D88 size=8
    let mut pc: u32 = 0x82511D88;
    'dispatch: loop {
        match pc {
            0x82511D88 => {
    //   block [0x82511D88..0x82511D90)
	// 82511D88: 38840020  addi r4, r4, 0x20
	ctx.r[4].s64 = ctx.r[4].s64 + 32;
	// 82511D8C: 4BFFFCF4  b 0x82511a80
	sub_82511A80(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82511D90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82511D90 size=8
    let mut pc: u32 = 0x82511D90;
    'dispatch: loop {
        match pc {
            0x82511D90 => {
    //   block [0x82511D90..0x82511D98)
	// 82511D90: 38840020  addi r4, r4, 0x20
	ctx.r[4].s64 = ctx.r[4].s64 + 32;
	// 82511D94: 4BFFFD5C  b 0x82511af0
	sub_82511AF0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82511D98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82511D98 size=104
    let mut pc: u32 = 0x82511D98;
    'dispatch: loop {
        match pc {
            0x82511D98 => {
    //   block [0x82511D98..0x82511E00)
	// 82511D98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82511D9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82511DA0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82511DA4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82511DA8: 806300C4  lwz r3, 0xc4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(196 as u32) ) } as u64;
	// 82511DAC: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82511DB0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82511DB4: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82511DB8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82511DBC: 4E800421  bctrl
	ctx.lr = 0x82511DC0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82511DC0: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 82511DC4: 39400020  li r10, 0x20
	ctx.r[10].s64 = 32;
	// 82511DC8: 13E01C07  vcmpneb. (lvlx128) v31, v0, v3
	tmp.u32 = ctx.r[3].u32;
	// load shuffled into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82511DCC: 39600030  li r11, 0x30
	ctx.r[11].s64 = 48;
	// 82511DD0: 13C91C07  vcmpneb. (lvlx128) v30, v9, v3
	tmp.u32 = ctx.r[9].u32 + ctx.r[3].u32;
	// load shuffled into ctx.v[62] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82511DD4: 13AA1C07  vcmpneb. (lvlx128) v29, v10, v3
	tmp.u32 = ctx.r[10].u32 + ctx.r[3].u32;
	// load shuffled into ctx.v[61] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82511DD8: 138B1C07  vcmpneb. (lvlx128) v28, v11, v3
	tmp.u32 = ctx.r[11].u32 + ctx.r[3].u32;
	// load shuffled into ctx.v[60] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82511E00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82511E00 size=8
    let mut pc: u32 = 0x82511E00;
    'dispatch: loop {
        match pc {
            0x82511E00 => {
    //   block [0x82511E00..0x82511E08)
	// 82511E00: 80840018  lwz r4, 0x18(r4)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(24 as u32) ) } as u64;
	// 82511E04: 4BFFFF94  b 0x82511d98
	sub_82511D98(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82511E08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82511E08 size=284
    let mut pc: u32 = 0x82511E08;
    'dispatch: loop {
        match pc {
            0x82511E08 => {
    //   block [0x82511E08..0x82511F24)
	// 82511E08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82511E0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82511E10: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82511E14: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82511E18: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82511E1C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82511E20: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82511E24: 2B050001  cmplwi cr6, r5, 1
	ctx.cr[6].compare_u32(ctx.r[5].u32, 1 as u32, &mut ctx.xer);
	// 82511E28: 4198006C  blt cr6, 0x82511e94
	if ctx.cr[6].lt {
	pc = 0x82511E94; continue 'dispatch;
	}
	// 82511E2C: 409A00E0  bne cr6, 0x82511f0c
	if !ctx.cr[6].eq {
	pc = 0x82511F0C; continue 'dispatch;
	}
	// 82511E30: 807F00C4  lwz r3, 0xc4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(196 as u32) ) } as u64;
	// 82511E34: 4BF74C85  bl 0x82486ab8
	ctx.lr = 0x82511E38;
	sub_82486AB8(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82511F28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82511F28 size=16
    let mut pc: u32 = 0x82511F28;
    'dispatch: loop {
        match pc {
            0x82511F28 => {
    //   block [0x82511F28..0x82511F38)
	// 82511F28: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82511F2C: 388B0020  addi r4, r11, 0x20
	ctx.r[4].s64 = ctx.r[11].s64 + 32;
	// 82511F30: 80AB0030  lwz r5, 0x30(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 82511F34: 4BFFFED4  b 0x82511e08
	sub_82511E08(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82511F38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82511F38 size=76
    let mut pc: u32 = 0x82511F38;
    'dispatch: loop {
        match pc {
            0x82511F38 => {
    //   block [0x82511F38..0x82511F84)
	// 82511F38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82511F3C: 48C96231  bl 0x831a816c
	ctx.lr = 0x82511F40;
	sub_831A8130(ctx, base);
	// 82511F40: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82511F44: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82511F48: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82511F4C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82511F50: 807E00C4  lwz r3, 0xc4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(196 as u32) ) } as u64;
	// 82511F54: 482E347D  bl 0x827f53d0
	ctx.lr = 0x82511F58;
	sub_827F53D0(ctx, base);
	// 82511F58: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82511F5C: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 82511F60: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82511F64: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82511F68: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82511F6C: 816B0034  lwz r11, 0x34(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 82511F70: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82511F74: 4E800421  bctrl
	ctx.lr = 0x82511F78;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82511F78: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82511F7C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82511F80: 48C9623C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82511F88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82511F88 size=80
    let mut pc: u32 = 0x82511F88;
    'dispatch: loop {
        match pc {
            0x82511F88 => {
    //   block [0x82511F88..0x82511FD8)
	// 82511F88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82511F8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82511F90: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82511F94: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82511F98: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82511F9C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82511FA0: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 82511FA4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82511FA8: 4E800421  bctrl
	ctx.lr = 0x82511FAC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82511FAC: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82511FB0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82511FB4: 419A000C  beq cr6, 0x82511fc0
	if ctx.cr[6].eq {
	pc = 0x82511FC0; continue 'dispatch;
	}
	// 82511FB8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82511FBC: 48634BBD  bl 0x82b46b78
	ctx.lr = 0x82511FC0;
	sub_82B46B78(ctx, base);
	// 82511FC0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82511FC4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82511FC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82511FCC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82511FD0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82511FD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82511FD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82511FD8 size=72
    let mut pc: u32 = 0x82511FD8;
    'dispatch: loop {
        match pc {
            0x82511FD8 => {
    //   block [0x82511FD8..0x82512020)
	// 82511FD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82511FDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82511FE0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82511FE4: 2F050003  cmpwi cr6, r5, 3
	ctx.cr[6].compare_i32(ctx.r[5].s32, 3, &mut ctx.xer);
	// 82511FE8: 419A001C  beq cr6, 0x82512004
	if ctx.cr[6].eq {
	pc = 0x82512004; continue 'dispatch;
	}
	// 82511FEC: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82511FF0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82511FF4: 994B0000  stb r10, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 82511FF8: 88C10050  lbz r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82511FFC: 4BFFF8CD  bl 0x825118c8
	ctx.lr = 0x82512000;
	sub_825118C8(ctx, base);
	// 82512000: 48000010  b 0x82512010
	pc = 0x82512010; continue 'dispatch;
	// 82512004: 3D608328  lis r11, -0x7cd8
	ctx.r[11].s64 = -2094530560;
	// 82512008: 396BC938  addi r11, r11, -0x36c8
	ctx.r[11].s64 = ctx.r[11].s64 + -14024;
	// 8251200C: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82512010: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82512014: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82512018: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8251201C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82512020(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82512020 size=72
    let mut pc: u32 = 0x82512020;
    'dispatch: loop {
        match pc {
            0x82512020 => {
    //   block [0x82512020..0x82512068)
	// 82512020: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82512024: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82512028: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251202C: 2F050003  cmpwi cr6, r5, 3
	ctx.cr[6].compare_i32(ctx.r[5].s32, 3, &mut ctx.xer);
	// 82512030: 419A001C  beq cr6, 0x8251204c
	if ctx.cr[6].eq {
	pc = 0x8251204C; continue 'dispatch;
	}
	// 82512034: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82512038: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8251203C: 994B0000  stb r10, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 82512040: 88C10050  lbz r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82512044: 4BFFF90D  bl 0x82511950
	ctx.lr = 0x82512048;
	sub_82511950(ctx, base);
	// 82512048: 48000010  b 0x82512058
	pc = 0x82512058; continue 'dispatch;
	// 8251204C: 3D608328  lis r11, -0x7cd8
	ctx.r[11].s64 = -2094530560;
	// 82512050: 396BCA00  addi r11, r11, -0x3600
	ctx.r[11].s64 = ctx.r[11].s64 + -13824;
	// 82512054: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82512058: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8251205C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82512060: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82512064: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82512068(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82512068 size=88
    let mut pc: u32 = 0x82512068;
    'dispatch: loop {
        match pc {
            0x82512068 => {
    //   block [0x82512068..0x825120C0)
	// 82512068: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251206C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82512070: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82512074: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82512078: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8251207C: 4BDADF85  bl 0x822c0000
	ctx.lr = 0x82512080;
	sub_822C0000(ctx, base);
	// 82512080: 807F00D8  lwz r3, 0xd8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(216 as u32) ) } as u64;
	// 82512084: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82512088: 419A0024  beq cr6, 0x825120ac
	if ctx.cr[6].eq {
	pc = 0x825120AC; continue 'dispatch;
	}
	// 8251208C: 48079D65  bl 0x8258bdf0
	ctx.lr = 0x82512090;
	sub_8258BDF0(ctx, base);
	// 82512090: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82512094: 917F00D8  stw r11, 0xd8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(216 as u32), ctx.r[11].u32 ) };
	// 82512098: 807F00DC  lwz r3, 0xdc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(220 as u32) ) } as u64;
	// 8251209C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825120A0: 917F00DC  stw r11, 0xdc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(220 as u32), ctx.r[11].u32 ) };
	// 825120A4: 419A0008  beq cr6, 0x825120ac
	if ctx.cr[6].eq {
	pc = 0x825120AC; continue 'dispatch;
	}
	// 825120A8: 4BDAE7E9  bl 0x822c0890
	ctx.lr = 0x825120AC;
	sub_822C0890(ctx, base);
	// 825120AC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825120B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825120B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825120B8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825120BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825120C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825120C0 size=84
    let mut pc: u32 = 0x825120C0;
    'dispatch: loop {
        match pc {
            0x825120C0 => {
    //   block [0x825120C0..0x82512114)
	// 825120C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825120C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825120C8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825120CC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825120D0: 816300CC  lwz r11, 0xcc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(204 as u32) ) } as u64;
	// 825120D4: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 825120D8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825120DC: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 825120E0: 4BF7AD61  bl 0x8248ce40
	ctx.lr = 0x825120E4;
	sub_8248CE40(ctx, base);
	// 825120E4: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 825120E8: 83E30000  lwz r31, 0(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825120EC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825120F0: 419A000C  beq cr6, 0x825120fc
	if ctx.cr[6].eq {
	pc = 0x825120FC; continue 'dispatch;
	}
	// 825120F4: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 825120F8: 4BDAE799  bl 0x822c0890
	ctx.lr = 0x825120FC;
	sub_822C0890(ctx, base);
	// 825120FC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82512100: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82512104: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82512108: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8251210C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82512110: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82512118(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82512118 size=24
    let mut pc: u32 = 0x82512118;
    'dispatch: loop {
        match pc {
            0x82512118 => {
    //   block [0x82512118..0x82512130)
	// 82512118: 816400C4  lwz r11, 0xc4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(196 as u32) ) } as u64;
	// 8251211C: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82512120: 816400C8  lwz r11, 0xc8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(200 as u32) ) } as u64;
	// 82512124: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82512128: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8251212C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82512130(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82512130 size=36
    let mut pc: u32 = 0x82512130;
    'dispatch: loop {
        match pc {
            0x82512130 => {
    //   block [0x82512130..0x82512154)
	// 82512130: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82512134: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82512138: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8251213C: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82512140: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82512144: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82512148: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8251214C: 4082FFE8  bne 0x82512134
	if !ctx.cr[0].eq {
	pc = 0x82512134; continue 'dispatch;
	}
	// 82512150: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82512158(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82512158 size=32
    let mut pc: u32 = 0x82512158;
    'dispatch: loop {
        match pc {
            0x82512158 => {
    //   block [0x82512158..0x82512178)
	// 82512158: 810300C4  lwz r8, 0xc4(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(196 as u32) ) } as u64;
	// 8251215C: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82512160: 394300C4  addi r10, r3, 0xc4
	ctx.r[10].s64 = ctx.r[3].s64 + 196;
	// 82512164: 392B0018  addi r9, r11, 0x18
	ctx.r[9].s64 = ctx.r[11].s64 + 24;
	// 82512168: 388A0004  addi r4, r10, 4
	ctx.r[4].s64 = ctx.r[10].s64 + 4;
	// 8251216C: 38690004  addi r3, r9, 4
	ctx.r[3].s64 = ctx.r[9].s64 + 4;
	// 82512170: 910B0018  stw r8, 0x18(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[8].u32 ) };
	// 82512174: 4BDB22EC  b 0x822c4460
	sub_822C4460(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82512178(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82512178 size=76
    let mut pc: u32 = 0x82512178;
    'dispatch: loop {
        match pc {
            0x82512178 => {
    //   block [0x82512178..0x825121C4)
	// 82512178: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251217C: 48C95FF1  bl 0x831a816c
	ctx.lr = 0x82512180;
	sub_831A8130(ctx, base);
	// 82512180: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82512184: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82512188: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8251218C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82512190: 807E00C4  lwz r3, 0xc4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(196 as u32) ) } as u64;
	// 82512194: 482E323D  bl 0x827f53d0
	ctx.lr = 0x82512198;
	sub_827F53D0(ctx, base);
	// 82512198: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251219C: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 825121A0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825121A4: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 825121A8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 825121AC: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 825121B0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825121B4: 4E800421  bctrl
	ctx.lr = 0x825121B8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825121B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825121BC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825121C0: 48C95FFC  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825121C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825121C8 size=148
    let mut pc: u32 = 0x825121C8;
    'dispatch: loop {
        match pc {
            0x825121C8 => {
    //   block [0x825121C8..0x8251225C)
	// 825121C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825121CC: 48C95FA1  bl 0x831a816c
	ctx.lr = 0x825121D0;
	sub_831A8130(ctx, base);
	// 825121D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825121D4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825121D8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825121DC: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 825121E0: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 825121E4: 4BFFD2E5  bl 0x8250f4c8
	ctx.lr = 0x825121E8;
	sub_8250F4C8(ctx, base);
	// 825121E8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825121EC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825121F0: 388BFFFC  addi r4, r11, -4
	ctx.r[4].s64 = ctx.r[11].s64 + -4;
	// 825121F4: 409A0008  bne cr6, 0x825121fc
	if !ctx.cr[6].eq {
	pc = 0x825121FC; continue 'dispatch;
	}
	// 825121F8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 825121FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82512200: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82512204: 38A00003  li r5, 3
	ctx.r[5].s64 = 3;
	// 82512208: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8251220C: 48634C95  bl 0x82b46ea0
	ctx.lr = 0x82512210;
	sub_82B46EA0(ctx, base);
	// 82512210: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82512214: 488DFA7D  bl 0x82df1c90
	ctx.lr = 0x82512218;
	sub_82DF1C90(ctx, base);
	// 82512218: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251221C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82512220: 419A0030  beq cr6, 0x82512250
	if ctx.cr[6].eq {
	pc = 0x82512250; continue 'dispatch;
	}
	// 82512224: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82512228: 48634981  bl 0x82b46ba8
	ctx.lr = 0x8251222C;
	sub_82B46BA8(ctx, base);
	// 8251222C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82512230: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82512234: 4863468D  bl 0x82b468c0
	ctx.lr = 0x82512238;
	sub_82B468C0(ctx, base);
	// 82512238: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251223C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82512240: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82512244: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82512248: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8251224C: 4E800421  bctrl
	ctx.lr = 0x82512250;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82512250: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82512254: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82512258: 48C95F64  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82512260(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82512260 size=204
    let mut pc: u32 = 0x82512260;
    'dispatch: loop {
        match pc {
            0x82512260 => {
    //   block [0x82512260..0x8251232C)
	// 82512260: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82512264: 48C95F05  bl 0x831a8168
	ctx.lr = 0x82512268;
	sub_831A8130(ctx, base);
	// 82512268: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251226C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82512270: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82512274: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82512278: 4BFFEE79  bl 0x825110f0
	ctx.lr = 0x8251227C;
	sub_825110F0(ctx, base);
	// 8251227C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82512280: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82512284: 396B315C  addi r11, r11, 0x315c
	ctx.r[11].s64 = ctx.r[11].s64 + 12636;
	// 82512288: 394A3148  addi r10, r10, 0x3148
	ctx.r[10].s64 = ctx.r[10].s64 + 12616;
	// 8251228C: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82512290: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82512294: 915F0028  stw r10, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[10].u32 ) };
	// 82512298: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8251229C: 93BF00C0  stw r29, 0xc0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(192 as u32), ctx.r[29].u32 ) };
	// 825122A0: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825122A4: 388B1BD0  addi r4, r11, 0x1bd0
	ctx.r[4].s64 = ctx.r[11].s64 + 7120;
	// 825122A8: 38A00039  li r5, 0x39
	ctx.r[5].s64 = 57;
	// 825122AC: 38600100  li r3, 0x100
	ctx.r[3].s64 = 256;
	// 825122B0: 488E0139  bl 0x82df23e8
	ctx.lr = 0x825122B4;
	sub_82DF23E8(ctx, base);
	// 825122B4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 825122B8: 41820018  beq 0x825122d0
	if ctx.cr[0].eq {
	pc = 0x825122D0; continue 'dispatch;
	}
	// 825122BC: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 825122C0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 825122C4: 48078A5D  bl 0x8258ad20
	ctx.lr = 0x825122C8;
	sub_8258AD20(ctx, base);
	// 825122C8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 825122CC: 48000008  b 0x825122d4
	pc = 0x825122D4; continue 'dispatch;
	// 825122D0: 7FBEEB78  mr r30, r29
	ctx.r[30].u64 = ctx.r[29].u64;
	// 825122D4: 93DF00C4  stw r30, 0xc4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(196 as u32), ctx.r[30].u32 ) };
	// 825122D8: 397F00C4  addi r11, r31, 0xc4
	ctx.r[11].s64 = ctx.r[31].s64 + 196;
	// 825122DC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 825122E0: 3B8B0004  addi r28, r11, 4
	ctx.r[28].s64 = ctx.r[11].s64 + 4;
	// 825122E4: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 825122E8: 4BDF46A9  bl 0x82306990
	ctx.lr = 0x825122EC;
	sub_82306990(ctx, base);
	// 825122EC: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 825122F0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 825122F4: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 825122F8: 4BDADD09  bl 0x822c0000
	ctx.lr = 0x825122FC;
	sub_822C0000(ctx, base);
	// 825122FC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82512300: 93BF00CC  stw r29, 0xcc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(204 as u32), ctx.r[29].u32 ) };
	// 82512304: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82512308: 93BF00D0  stw r29, 0xd0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(208 as u32), ctx.r[29].u32 ) };
	// 8251230C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82512310: C00B9534  lfs f0, -0x6acc(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27340 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82512314: D01F00D4  stfs f0, 0xd4(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(212 as u32), tmp.u32 ) };
	// 82512318: 93BF00D8  stw r29, 0xd8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(216 as u32), ctx.r[29].u32 ) };
	// 8251231C: 93BF00DC  stw r29, 0xdc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(220 as u32), ctx.r[29].u32 ) };
	// 82512320: 995F00E0  stb r10, 0xe0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(224 as u32), ctx.r[10].u8 ) };
	// 82512324: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82512328: 48C95E90  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82512330(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82512330 size=188
    let mut pc: u32 = 0x82512330;
    'dispatch: loop {
        match pc {
            0x82512330 => {
    //   block [0x82512330..0x825123EC)
	// 82512330: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82512334: 48C95E35  bl 0x831a8168
	ctx.lr = 0x82512338;
	sub_831A8130(ctx, base);
	// 82512338: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251233C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82512340: 4BFFEDB1  bl 0x825110f0
	ctx.lr = 0x82512344;
	sub_825110F0(ctx, base);
	// 82512344: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82512348: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8251234C: 396B315C  addi r11, r11, 0x315c
	ctx.r[11].s64 = ctx.r[11].s64 + 12636;
	// 82512350: 394A3148  addi r10, r10, 0x3148
	ctx.r[10].s64 = ctx.r[10].s64 + 12616;
	// 82512354: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82512358: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8251235C: 915F0028  stw r10, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[10].u32 ) };
	// 82512360: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82512364: 93BF00C0  stw r29, 0xc0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(192 as u32), ctx.r[29].u32 ) };
	// 82512368: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8251236C: 388B1BD0  addi r4, r11, 0x1bd0
	ctx.r[4].s64 = ctx.r[11].s64 + 7120;
	// 82512370: 38A00042  li r5, 0x42
	ctx.r[5].s64 = 66;
	// 82512374: 38600100  li r3, 0x100
	ctx.r[3].s64 = 256;
	// 82512378: 488E0071  bl 0x82df23e8
	ctx.lr = 0x8251237C;
	sub_82DF23E8(ctx, base);
	// 8251237C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82512380: 41820010  beq 0x82512390
	if ctx.cr[0].eq {
	pc = 0x82512390; continue 'dispatch;
	}
	// 82512384: 48078915  bl 0x8258ac98
	ctx.lr = 0x82512388;
	sub_8258AC98(ctx, base);
	// 82512388: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8251238C: 48000008  b 0x82512394
	pc = 0x82512394; continue 'dispatch;
	// 82512390: 7FBEEB78  mr r30, r29
	ctx.r[30].u64 = ctx.r[29].u64;
	// 82512394: 93DF00C4  stw r30, 0xc4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(196 as u32), ctx.r[30].u32 ) };
	// 82512398: 397F00C4  addi r11, r31, 0xc4
	ctx.r[11].s64 = ctx.r[31].s64 + 196;
	// 8251239C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 825123A0: 3B8B0004  addi r28, r11, 4
	ctx.r[28].s64 = ctx.r[11].s64 + 4;
	// 825123A4: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 825123A8: 4BDF45E9  bl 0x82306990
	ctx.lr = 0x825123AC;
	sub_82306990(ctx, base);
	// 825123AC: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 825123B0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 825123B4: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 825123B8: 4BDADC49  bl 0x822c0000
	ctx.lr = 0x825123BC;
	sub_822C0000(ctx, base);
	// 825123BC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 825123C0: 93BF00CC  stw r29, 0xcc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(204 as u32), ctx.r[29].u32 ) };
	// 825123C4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825123C8: 93BF00D0  stw r29, 0xd0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(208 as u32), ctx.r[29].u32 ) };
	// 825123CC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825123D0: C00B9534  lfs f0, -0x6acc(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27340 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825123D4: D01F00D4  stfs f0, 0xd4(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(212 as u32), tmp.u32 ) };
	// 825123D8: 93BF00D8  stw r29, 0xd8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(216 as u32), ctx.r[29].u32 ) };
	// 825123DC: 93BF00DC  stw r29, 0xdc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(220 as u32), ctx.r[29].u32 ) };
	// 825123E0: 995F00E0  stb r10, 0xe0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(224 as u32), ctx.r[10].u8 ) };
	// 825123E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825123E8: 48C95DD0  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825123F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825123F0 size=128
    let mut pc: u32 = 0x825123F0;
    'dispatch: loop {
        match pc {
            0x825123F0 => {
    //   block [0x825123F0..0x82512470)
	// 825123F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825123F4: 48C95D79  bl 0x831a816c
	ctx.lr = 0x825123F8;
	sub_831A8130(ctx, base);
	// 825123F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825123FC: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 82512400: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 82512404: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82512408: 3BEB7020  addi r31, r11, 0x7020
	ctx.r[31].s64 = ctx.r[11].s64 + 28704;
	// 8251240C: 816A7028  lwz r11, 0x7028(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(28712 as u32) ) } as u64;
	// 82512410: 556907FF  clrlwi. r9, r11, 0x1f
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82512414: 40820024  bne 0x82512438
	if !ctx.cr[0].eq {
	pc = 0x82512438; continue 'dispatch;
	}
	// 82512418: 3D208290  lis r9, -0x7d70
	ctx.r[9].s64 = -2104492032;
	// 8251241C: 3D008251  lis r8, -0x7daf
	ctx.r[8].s64 = -2108620800;
	// 82512420: 616B0001  ori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 | 1;
	// 82512424: 3929DD90  addi r9, r9, -0x2270
	ctx.r[9].s64 = ctx.r[9].s64 + -8816;
	// 82512428: 39081FD8  addi r8, r8, 0x1fd8
	ctx.r[8].s64 = ctx.r[8].s64 + 8152;
	// 8251242C: 916A7028  stw r11, 0x7028(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(28712 as u32), ctx.r[11].u32 ) };
	// 82512430: 913F0004  stw r9, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82512434: 911F0000  stw r8, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82512438: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 8251243C: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82512440: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82512444: 38DE0008  addi r6, r30, 8
	ctx.r[6].s64 = ctx.r[30].s64 + 8;
	// 82512448: 9BAB0000  stb r29, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u8 ) };
	// 8251244C: 88E10050  lbz r7, 0x50(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82512450: 48142171  bl 0x826545c0
	ctx.lr = 0x82512454;
	sub_826545C0(ctx, base);
	// 82512454: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82512458: 4182000C  beq 0x82512464
	if ctx.cr[0].eq {
	pc = 0x82512464; continue 'dispatch;
	}
	// 8251245C: 93FE0000  stw r31, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82512460: 48000008  b 0x82512468
	pc = 0x82512468; continue 'dispatch;
	// 82512464: 93BE0000  stw r29, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82512468: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8251246C: 48C95D50  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82512470(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82512470 size=128
    let mut pc: u32 = 0x82512470;
    'dispatch: loop {
        match pc {
            0x82512470 => {
    //   block [0x82512470..0x825124F0)
	// 82512470: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82512474: 48C95CF9  bl 0x831a816c
	ctx.lr = 0x82512478;
	sub_831A8130(ctx, base);
	// 82512478: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251247C: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 82512480: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 82512484: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82512488: 3BEB702C  addi r31, r11, 0x702c
	ctx.r[31].s64 = ctx.r[11].s64 + 28716;
	// 8251248C: 816A7034  lwz r11, 0x7034(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(28724 as u32) ) } as u64;
	// 82512490: 556907FF  clrlwi. r9, r11, 0x1f
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82512494: 40820024  bne 0x825124b8
	if !ctx.cr[0].eq {
	pc = 0x825124B8; continue 'dispatch;
	}
	// 82512498: 3D208290  lis r9, -0x7d70
	ctx.r[9].s64 = -2104492032;
	// 8251249C: 3D008251  lis r8, -0x7daf
	ctx.r[8].s64 = -2108620800;
	// 825124A0: 616B0001  ori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 | 1;
	// 825124A4: 3929DD90  addi r9, r9, -0x2270
	ctx.r[9].s64 = ctx.r[9].s64 + -8816;
	// 825124A8: 39082020  addi r8, r8, 0x2020
	ctx.r[8].s64 = ctx.r[8].s64 + 8224;
	// 825124AC: 916A7034  stw r11, 0x7034(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(28724 as u32), ctx.r[11].u32 ) };
	// 825124B0: 913F0004  stw r9, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 825124B4: 911F0000  stw r8, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 825124B8: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 825124BC: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 825124C0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825124C4: 38DE0008  addi r6, r30, 8
	ctx.r[6].s64 = ctx.r[30].s64 + 8;
	// 825124C8: 9BAB0000  stb r29, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u8 ) };
	// 825124CC: 88E10050  lbz r7, 0x50(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 825124D0: 481420F1  bl 0x826545c0
	ctx.lr = 0x825124D4;
	sub_826545C0(ctx, base);
	// 825124D4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825124D8: 4182000C  beq 0x825124e4
	if ctx.cr[0].eq {
	pc = 0x825124E4; continue 'dispatch;
	}
	// 825124DC: 93FE0000  stw r31, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 825124E0: 48000008  b 0x825124e8
	pc = 0x825124E8; continue 'dispatch;
	// 825124E4: 93BE0000  stw r29, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 825124E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825124EC: 48C95CD0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825124F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x825124F0 size=136
    let mut pc: u32 = 0x825124F0;
    'dispatch: loop {
        match pc {
            0x825124F0 => {
    //   block [0x825124F0..0x82512578)
	// 825124F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825124F4: 48C95C79  bl 0x831a816c
	ctx.lr = 0x825124F8;
	sub_831A8130(ctx, base);
	// 825124F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825124FC: 806300C4  lwz r3, 0xc4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(196 as u32) ) } as u64;
	// 82512500: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82512504: 482E2ECD  bl 0x827f53d0
	ctx.lr = 0x82512508;
	sub_827F53D0(ctx, base);
	// 82512508: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 8251250C: 13E018C7  vcmpequd (lvx128) v31, v0, v3
	tmp.u32 = ctx.r[3].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82512510: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82512514: 83FF0018  lwz r31, 0x18(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82512518: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 8251251C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82512578(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82512578 size=160
    let mut pc: u32 = 0x82512578;
    'dispatch: loop {
        match pc {
            0x82512578 => {
    //   block [0x82512578..0x82512618)
	// 82512578: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251257C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82512580: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82512584: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82512588: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251258C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82512590: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82512594: 897E001C  lbz r11, 0x1c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 82512598: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8251259C: 41820054  beq 0x825125f0
	if ctx.cr[0].eq {
	pc = 0x825125F0; continue 'dispatch;
	}
	// 825125A0: 817E0018  lwz r11, 0x18(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 825125A4: 556B07FF  clrlwi. r11, r11, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825125A8: 41820048  beq 0x825125f0
	if ctx.cr[0].eq {
	pc = 0x825125F0; continue 'dispatch;
	}
	// 825125AC: 817F00CC  lwz r11, 0xcc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(204 as u32) ) } as u64;
	// 825125B0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825125B4: 419A003C  beq cr6, 0x825125f0
	if ctx.cr[6].eq {
	pc = 0x825125F0; continue 'dispatch;
	}
	// 825125B8: 3D408230  lis r10, -0x7dd0
	ctx.r[10].s64 = -2110783488;
	// 825125BC: 93E10058  stw r31, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[31].u32 ) };
	// 825125C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825125C4: E8A10058  ld r5, 0x58(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 825125C8: 394A25F0  addi r10, r10, 0x25f0
	ctx.r[10].s64 = ctx.r[10].s64 + 9712;
	// 825125CC: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825125D0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825125D4: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 825125D8: E8810050  ld r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 825125DC: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 825125E0: 4BFFFE11  bl 0x825123f0
	ctx.lr = 0x825125E4;
	sub_825123F0(ctx, base);
	// 825125E4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 825125E8: 807F00CC  lwz r3, 0xcc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(204 as u32) ) } as u64;
	// 825125EC: 4BF7AD55  bl 0x8248d340
	ctx.lr = 0x825125F0;
	sub_8248D340(ctx, base);
	// 825125F0: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 825125F4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 825125F8: 387F0028  addi r3, r31, 0x28
	ctx.r[3].s64 = ctx.r[31].s64 + 40;
	// 825125FC: 4BFFEF9D  bl 0x82511598
	ctx.lr = 0x82512600;
	sub_82511598(ctx, base);
	// 82512600: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82512604: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82512608: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8251260C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82512610: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82512614: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82512618(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82512618 size=892
    let mut pc: u32 = 0x82512618;
    'dispatch: loop {
        match pc {
            0x82512618 => {
    //   block [0x82512618..0x82512994)
	// 82512618: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251261C: 48C95B4D  bl 0x831a8168
	ctx.lr = 0x82512620;
	sub_831A8130(ctx, base);
	// 82512620: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82512624: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82512628: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8251262C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82512630: 579D063F  clrlwi. r29, r28, 0x18
	ctx.r[29].u64 = ctx.r[28].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82512634: 41820038  beq 0x8251266c
	if ctx.cr[0].eq {
	pc = 0x8251266C; continue 'dispatch;
	}
	// 82512638: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8251263C: 48C9734D  bl 0x831a9988
	ctx.lr = 0x82512640;
	sub_831A9988(ctx, base);
	// 82512640: 3D608325  lis r11, -0x7cdb
	ctx.r[11].s64 = -2094727168;
	// 82512644: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82512648: 386B5BA8  addi r3, r11, 0x5ba8
	ctx.r[3].s64 = ctx.r[11].s64 + 23464;
	// 8251264C: 48C95AAD  bl 0x831a80f8
	ctx.lr = 0x82512650;
	sub_831A80F8(ctx, base);
	// 82512650: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82512654: 41820018  beq 0x8251266c
	if ctx.cr[0].eq {
	pc = 0x8251266C; continue 'dispatch;
	}
	// 82512658: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8251265C: 387EFFD8  addi r3, r30, -0x28
	ctx.r[3].s64 = ctx.r[30].s64 + -40;
	// 82512660: 4BFFF729  bl 0x82511d88
	ctx.lr = 0x82512664;
	sub_82511D88(ctx, base);
	// 82512664: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82512668: 48000324  b 0x8251298c
	pc = 0x8251298C; continue 'dispatch;
	// 8251266C: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82512670: 419A030C  beq cr6, 0x8251297c
	if ctx.cr[6].eq {
	pc = 0x8251297C; continue 'dispatch;
	}
	// 82512674: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82512678: 48C97311  bl 0x831a9988
	ctx.lr = 0x8251267C;
	sub_831A9988(ctx, base);
	// 8251267C: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 82512680: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82512684: 386B2E70  addi r3, r11, 0x2e70
	ctx.r[3].s64 = ctx.r[11].s64 + 11888;
	// 82512688: 48C95A71  bl 0x831a80f8
	ctx.lr = 0x8251268C;
	sub_831A80F8(ctx, base);
	// 8251268C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82512690: 41820014  beq 0x825126a4
	if ctx.cr[0].eq {
	pc = 0x825126A4; continue 'dispatch;
	}
	// 82512694: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82512698: 387EFFD8  addi r3, r30, -0x28
	ctx.r[3].s64 = ctx.r[30].s64 + -40;
	// 8251269C: 4BFFF6F5  bl 0x82511d90
	ctx.lr = 0x825126A0;
	sub_82511D90(ctx, base);
	// 825126A0: 4BFFFFC4  b 0x82512664
	pc = 0x82512664; continue 'dispatch;
	// 825126A4: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 825126A8: 419A02D4  beq cr6, 0x8251297c
	if ctx.cr[6].eq {
	pc = 0x8251297C; continue 'dispatch;
	}
	// 825126AC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825126B0: 48C972D9  bl 0x831a9988
	ctx.lr = 0x825126B4;
	sub_831A9988(ctx, base);
	// 825126B4: 3D608328  lis r11, -0x7cd8
	ctx.r[11].s64 = -2094530560;
	// 825126B8: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 825126BC: 386BCB78  addi r3, r11, -0x3488
	ctx.r[3].s64 = ctx.r[11].s64 + -13448;
	// 825126C0: 48C95A39  bl 0x831a80f8
	ctx.lr = 0x825126C4;
	sub_831A80F8(ctx, base);
	// 825126C4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825126C8: 41820014  beq 0x825126dc
	if ctx.cr[0].eq {
	pc = 0x825126DC; continue 'dispatch;
	}
	// 825126CC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825126D0: 387EFFD8  addi r3, r30, -0x28
	ctx.r[3].s64 = ctx.r[30].s64 + -40;
	// 825126D4: 4BFFF855  bl 0x82511f28
	ctx.lr = 0x825126D8;
	sub_82511F28(ctx, base);
	// 825126D8: 4BFFFF8C  b 0x82512664
	pc = 0x82512664; continue 'dispatch;
	// 825126DC: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 825126E0: 419A029C  beq cr6, 0x8251297c
	if ctx.cr[6].eq {
	pc = 0x8251297C; continue 'dispatch;
	}
	// 825126E4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825126E8: 48C972A1  bl 0x831a9988
	ctx.lr = 0x825126EC;
	sub_831A9988(ctx, base);
	// 825126EC: 3D608328  lis r11, -0x7cd8
	ctx.r[11].s64 = -2094530560;
	// 825126F0: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 825126F4: 386BCB50  addi r3, r11, -0x34b0
	ctx.r[3].s64 = ctx.r[11].s64 + -13488;
	// 825126F8: 48C95A01  bl 0x831a80f8
	ctx.lr = 0x825126FC;
	sub_831A80F8(ctx, base);
	// 825126FC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82512700: 41820014  beq 0x82512714
	if ctx.cr[0].eq {
	pc = 0x82512714; continue 'dispatch;
	}
	// 82512704: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82512708: 387EFFD8  addi r3, r30, -0x28
	ctx.r[3].s64 = ctx.r[30].s64 + -40;
	// 8251270C: 4BFFF455  bl 0x82511b60
	ctx.lr = 0x82512710;
	sub_82511B60(ctx, base);
	// 82512710: 4BFFFF54  b 0x82512664
	pc = 0x82512664; continue 'dispatch;
	// 82512714: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82512718: 419A0264  beq cr6, 0x8251297c
	if ctx.cr[6].eq {
	pc = 0x8251297C; continue 'dispatch;
	}
	// 8251271C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82512720: 48C97269  bl 0x831a9988
	ctx.lr = 0x82512724;
	sub_831A9988(ctx, base);
	// 82512724: 3D608325  lis r11, -0x7cdb
	ctx.r[11].s64 = -2094727168;
	// 82512728: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8251272C: 386B5AB4  addi r3, r11, 0x5ab4
	ctx.r[3].s64 = ctx.r[11].s64 + 23220;
	// 82512730: 48C959C9  bl 0x831a80f8
	ctx.lr = 0x82512734;
	sub_831A80F8(ctx, base);
	// 82512734: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82512738: 41820014  beq 0x8251274c
	if ctx.cr[0].eq {
	pc = 0x8251274C; continue 'dispatch;
	}
	// 8251273C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82512740: 387EFFD8  addi r3, r30, -0x28
	ctx.r[3].s64 = ctx.r[30].s64 + -40;
	// 82512744: 4BFFF59D  bl 0x82511ce0
	ctx.lr = 0x82512748;
	sub_82511CE0(ctx, base);
	// 82512748: 4BFFFF1C  b 0x82512664
	pc = 0x82512664; continue 'dispatch;
	// 8251274C: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82512750: 419A022C  beq cr6, 0x8251297c
	if ctx.cr[6].eq {
	pc = 0x8251297C; continue 'dispatch;
	}
	// 82512754: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82512758: 48C97231  bl 0x831a9988
	ctx.lr = 0x8251275C;
	sub_831A9988(ctx, base);
	// 8251275C: 3D608325  lis r11, -0x7cdb
	ctx.r[11].s64 = -2094727168;
	// 82512760: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82512764: 386B5C00  addi r3, r11, 0x5c00
	ctx.r[3].s64 = ctx.r[11].s64 + 23552;
	// 82512768: 48C95991  bl 0x831a80f8
	ctx.lr = 0x8251276C;
	sub_831A80F8(ctx, base);
	// 8251276C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82512770: 41820014  beq 0x82512784
	if ctx.cr[0].eq {
	pc = 0x82512784; continue 'dispatch;
	}
	// 82512774: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82512778: 387EFFD8  addi r3, r30, -0x28
	ctx.r[3].s64 = ctx.r[30].s64 + -40;
	// 8251277C: 4BFFF565  bl 0x82511ce0
	ctx.lr = 0x82512780;
	sub_82511CE0(ctx, base);
	// 82512780: 4BFFFEE4  b 0x82512664
	pc = 0x82512664; continue 'dispatch;
	// 82512784: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82512788: 419A01F4  beq cr6, 0x8251297c
	if ctx.cr[6].eq {
	pc = 0x8251297C; continue 'dispatch;
	}
	// 8251278C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82512790: 48C971F9  bl 0x831a9988
	ctx.lr = 0x82512794;
	sub_831A9988(ctx, base);
	// 82512794: 3D608328  lis r11, -0x7cd8
	ctx.r[11].s64 = -2094530560;
	// 82512798: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8251279C: 386BCB20  addi r3, r11, -0x34e0
	ctx.r[3].s64 = ctx.r[11].s64 + -13536;
	// 825127A0: 48C95959  bl 0x831a80f8
	ctx.lr = 0x825127A4;
	sub_831A80F8(ctx, base);
	// 825127A4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825127A8: 41820014  beq 0x825127bc
	if ctx.cr[0].eq {
	pc = 0x825127BC; continue 'dispatch;
	}
	// 825127AC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825127B0: 387EFFD8  addi r3, r30, -0x28
	ctx.r[3].s64 = ctx.r[30].s64 + -40;
	// 825127B4: 4BFFF52D  bl 0x82511ce0
	ctx.lr = 0x825127B8;
	sub_82511CE0(ctx, base);
	// 825127B8: 4BFFFEAC  b 0x82512664
	pc = 0x82512664; continue 'dispatch;
	// 825127BC: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 825127C0: 419A01BC  beq cr6, 0x8251297c
	if ctx.cr[6].eq {
	pc = 0x8251297C; continue 'dispatch;
	}
	// 825127C4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825127C8: 48C971C1  bl 0x831a9988
	ctx.lr = 0x825127CC;
	sub_831A9988(ctx, base);
	// 825127CC: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 825127D0: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 825127D4: 386B31EC  addi r3, r11, 0x31ec
	ctx.r[3].s64 = ctx.r[11].s64 + 12780;
	// 825127D8: 48C95921  bl 0x831a80f8
	ctx.lr = 0x825127DC;
	sub_831A80F8(ctx, base);
	// 825127DC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825127E0: 41820014  beq 0x825127f4
	if ctx.cr[0].eq {
	pc = 0x825127F4; continue 'dispatch;
	}
	// 825127E4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825127E8: 387EFFD8  addi r3, r30, -0x28
	ctx.r[3].s64 = ctx.r[30].s64 + -40;
	// 825127EC: 4BFFF4F5  bl 0x82511ce0
	ctx.lr = 0x825127F0;
	sub_82511CE0(ctx, base);
	// 825127F0: 4BFFFE74  b 0x82512664
	pc = 0x82512664; continue 'dispatch;
	// 825127F4: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 825127F8: 419A0184  beq cr6, 0x8251297c
	if ctx.cr[6].eq {
	pc = 0x8251297C; continue 'dispatch;
	}
	// 825127FC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82512800: 48C97189  bl 0x831a9988
	ctx.lr = 0x82512804;
	sub_831A9988(ctx, base);
	// 82512804: 3D608328  lis r11, -0x7cd8
	ctx.r[11].s64 = -2094530560;
	// 82512808: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8251280C: 386BCAE8  addi r3, r11, -0x3518
	ctx.r[3].s64 = ctx.r[11].s64 + -13592;
	// 82512810: 48C958E9  bl 0x831a80f8
	ctx.lr = 0x82512814;
	sub_831A80F8(ctx, base);
	// 82512814: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82512818: 41820014  beq 0x8251282c
	if ctx.cr[0].eq {
	pc = 0x8251282C; continue 'dispatch;
	}
	// 8251281C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82512820: 387EFFD8  addi r3, r30, -0x28
	ctx.r[3].s64 = ctx.r[30].s64 + -40;
	// 82512824: 4BFFF4BD  bl 0x82511ce0
	ctx.lr = 0x82512828;
	sub_82511CE0(ctx, base);
	// 82512828: 4BFFFE3C  b 0x82512664
	pc = 0x82512664; continue 'dispatch;
	// 8251282C: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82512830: 419A014C  beq cr6, 0x8251297c
	if ctx.cr[6].eq {
	pc = 0x8251297C; continue 'dispatch;
	}
	// 82512834: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82512838: 48C97151  bl 0x831a9988
	ctx.lr = 0x8251283C;
	sub_831A9988(ctx, base);
	// 8251283C: 3D608325  lis r11, -0x7cdb
	ctx.r[11].s64 = -2094727168;
	// 82512840: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82512844: 386B5BD4  addi r3, r11, 0x5bd4
	ctx.r[3].s64 = ctx.r[11].s64 + 23508;
	// 82512848: 48C958B1  bl 0x831a80f8
	ctx.lr = 0x8251284C;
	sub_831A80F8(ctx, base);
	// 8251284C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82512850: 41820014  beq 0x82512864
	if ctx.cr[0].eq {
	pc = 0x82512864; continue 'dispatch;
	}
	// 82512854: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82512858: 387EFFD8  addi r3, r30, -0x28
	ctx.r[3].s64 = ctx.r[30].s64 + -40;
	// 8251285C: 4BFFF1CD  bl 0x82511a28
	ctx.lr = 0x82512860;
	sub_82511A28(ctx, base);
	// 82512860: 4BFFFE04  b 0x82512664
	pc = 0x82512664; continue 'dispatch;
	// 82512864: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82512868: 419A0114  beq cr6, 0x8251297c
	if ctx.cr[6].eq {
	pc = 0x8251297C; continue 'dispatch;
	}
	// 8251286C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82512870: 48C97119  bl 0x831a9988
	ctx.lr = 0x82512874;
	sub_831A9988(ctx, base);
	// 82512874: 3D608327  lis r11, -0x7cd9
	ctx.r[11].s64 = -2094596096;
	// 82512878: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8251287C: 386B3324  addi r3, r11, 0x3324
	ctx.r[3].s64 = ctx.r[11].s64 + 13092;
	// 82512880: 48C95879  bl 0x831a80f8
	ctx.lr = 0x82512884;
	sub_831A80F8(ctx, base);
	// 82512884: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82512888: 41820014  beq 0x8251289c
	if ctx.cr[0].eq {
	pc = 0x8251289C; continue 'dispatch;
	}
	// 8251288C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82512890: 387EFFD8  addi r3, r30, -0x28
	ctx.r[3].s64 = ctx.r[30].s64 + -40;
	// 82512894: 4BFFF56D  bl 0x82511e00
	ctx.lr = 0x82512898;
	sub_82511E00(ctx, base);
	// 82512898: 4BFFFDCC  b 0x82512664
	pc = 0x82512664; continue 'dispatch;
	// 8251289C: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 825128A0: 419A00DC  beq cr6, 0x8251297c
	if ctx.cr[6].eq {
	pc = 0x8251297C; continue 'dispatch;
	}
	// 825128A4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825128A8: 48C970E1  bl 0x831a9988
	ctx.lr = 0x825128AC;
	sub_831A9988(ctx, base);
	// 825128AC: 3D608325  lis r11, -0x7cdb
	ctx.r[11].s64 = -2094727168;
	// 825128B0: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 825128B4: 386B6B84  addi r3, r11, 0x6b84
	ctx.r[3].s64 = ctx.r[11].s64 + 27524;
	// 825128B8: 48C95841  bl 0x831a80f8
	ctx.lr = 0x825128BC;
	sub_831A80F8(ctx, base);
	// 825128BC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825128C0: 41820014  beq 0x825128d4
	if ctx.cr[0].eq {
	pc = 0x825128D4; continue 'dispatch;
	}
	// 825128C4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825128C8: 387EFFD8  addi r3, r30, -0x28
	ctx.r[3].s64 = ctx.r[30].s64 + -40;
	// 825128CC: 4BFFF88D  bl 0x82512158
	ctx.lr = 0x825128D0;
	sub_82512158(ctx, base);
	// 825128D0: 4BFFFD94  b 0x82512664
	pc = 0x82512664; continue 'dispatch;
	// 825128D4: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 825128D8: 419A00A4  beq cr6, 0x8251297c
	if ctx.cr[6].eq {
	pc = 0x8251297C; continue 'dispatch;
	}
	// 825128DC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825128E0: 48C970A9  bl 0x831a9988
	ctx.lr = 0x825128E4;
	sub_831A9988(ctx, base);
	// 825128E4: 3D608325  lis r11, -0x7cdb
	ctx.r[11].s64 = -2094727168;
	// 825128E8: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 825128EC: 386B5AEC  addi r3, r11, 0x5aec
	ctx.r[3].s64 = ctx.r[11].s64 + 23276;
	// 825128F0: 48C95809  bl 0x831a80f8
	ctx.lr = 0x825128F4;
	sub_831A80F8(ctx, base);
	// 825128F4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825128F8: 41820014  beq 0x8251290c
	if ctx.cr[0].eq {
	pc = 0x8251290C; continue 'dispatch;
	}
	// 825128FC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82512900: 387EFFD8  addi r3, r30, -0x28
	ctx.r[3].s64 = ctx.r[30].s64 + -40;
	// 82512904: 4BFFFBED  bl 0x825124f0
	ctx.lr = 0x82512908;
	sub_825124F0(ctx, base);
	// 82512908: 4BFFFD5C  b 0x82512664
	pc = 0x82512664; continue 'dispatch;
	// 8251290C: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82512910: 419A006C  beq cr6, 0x8251297c
	if ctx.cr[6].eq {
	pc = 0x8251297C; continue 'dispatch;
	}
	// 82512914: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82512918: 48C97071  bl 0x831a9988
	ctx.lr = 0x8251291C;
	sub_831A9988(ctx, base);
	// 8251291C: 3D608325  lis r11, -0x7cdb
	ctx.r[11].s64 = -2094727168;
	// 82512920: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82512924: 386B5A60  addi r3, r11, 0x5a60
	ctx.r[3].s64 = ctx.r[11].s64 + 23136;
	// 82512928: 48C957D1  bl 0x831a80f8
	ctx.lr = 0x8251292C;
	sub_831A80F8(ctx, base);
	// 8251292C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82512930: 41820014  beq 0x82512944
	if ctx.cr[0].eq {
	pc = 0x82512944; continue 'dispatch;
	}
	// 82512934: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82512938: 387EFFD8  addi r3, r30, -0x28
	ctx.r[3].s64 = ctx.r[30].s64 + -40;
	// 8251293C: 4BFFFC3D  bl 0x82512578
	ctx.lr = 0x82512940;
	sub_82512578(ctx, base);
	// 82512940: 4BFFFD24  b 0x82512664
	pc = 0x82512664; continue 'dispatch;
	// 82512944: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82512948: 419A0034  beq cr6, 0x8251297c
	if ctx.cr[6].eq {
	pc = 0x8251297C; continue 'dispatch;
	}
	// 8251294C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82512950: 48C97039  bl 0x831a9988
	ctx.lr = 0x82512954;
	sub_831A9988(ctx, base);
	// 82512954: 3D608328  lis r11, -0x7cd8
	ctx.r[11].s64 = -2094530560;
	// 82512958: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8251295C: 386BCABC  addi r3, r11, -0x3544
	ctx.r[3].s64 = ctx.r[11].s64 + -13636;
	// 82512960: 48C95799  bl 0x831a80f8
	ctx.lr = 0x82512964;
	sub_831A80F8(ctx, base);
	// 82512964: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82512968: 41820014  beq 0x8251297c
	if ctx.cr[0].eq {
	pc = 0x8251297C; continue 'dispatch;
	}
	// 8251296C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82512970: 387EFFD8  addi r3, r30, -0x28
	ctx.r[3].s64 = ctx.r[30].s64 + -40;
	// 82512974: 4BFFEEFD  bl 0x82511870
	ctx.lr = 0x82512978;
	sub_82511870(ctx, base);
	// 82512978: 4BFFFCEC  b 0x82512664
	pc = 0x82512664; continue 'dispatch;
	// 8251297C: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82512980: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82512984: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82512988: 4BFFEC11  bl 0x82511598
	ctx.lr = 0x8251298C;
	sub_82511598(ctx, base);
	// 8251298C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82512990: 48C95828  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82512998(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82512998 size=256
    let mut pc: u32 = 0x82512998;
    'dispatch: loop {
        match pc {
            0x82512998 => {
    //   block [0x82512998..0x82512A98)
	// 82512998: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251299C: 48C957CD  bl 0x831a8168
	ctx.lr = 0x825129A0;
	sub_831A8130(ctx, base);
	// 825129A0: DBE1FFD0  stfd f31, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[31].u64 ) };
	// 825129A4: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825129A8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825129AC: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 825129B0: 3BBF00D8  addi r29, r31, 0xd8
	ctx.r[29].s64 = ctx.r[31].s64 + 216;
	// 825129B4: 807F00D8  lwz r3, 0xd8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(216 as u32) ) } as u64;
	// 825129B8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825129BC: 419A0008  beq cr6, 0x825129c4
	if ctx.cr[6].eq {
	pc = 0x825129C4; continue 'dispatch;
	}
	// 825129C0: 48079431  bl 0x8258bdf0
	ctx.lr = 0x825129C4;
	sub_8258BDF0(ctx, base);
	// 825129C4: 3D608251  lis r11, -0x7daf
	ctx.r[11].s64 = -2108620800;
	// 825129C8: 93E10058  stw r31, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[31].u32 ) };
	// 825129CC: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 825129D0: E8A10058  ld r5, 0x58(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 825129D4: 396B1D20  addi r11, r11, 0x1d20
	ctx.r[11].s64 = ctx.r[11].s64 + 7456;
	// 825129D8: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 825129DC: 93C10054  stw r30, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[30].u32 ) };
	// 825129E0: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 825129E4: E8810050  ld r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 825129E8: 93C10080  stw r30, 0x80(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[30].u32 ) };
	// 825129EC: 4BFFFA85  bl 0x82512470
	ctx.lr = 0x825129F0;
	sub_82512470(ctx, base);
	// 825129F0: 807F00C4  lwz r3, 0xc4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(196 as u32) ) } as u64;
	// 825129F4: 482E29DD  bl 0x827f53d0
	ctx.lr = 0x825129F8;
	sub_827F53D0(ctx, base);
	// 825129F8: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 825129FC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82512A00: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82512A04: 4BFFCAC5  bl 0x8250f4c8
	ctx.lr = 0x82512A08;
	sub_8250F4C8(ctx, base);
	// 82512A08: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82512A0C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82512A10: 388BFFFC  addi r4, r11, -4
	ctx.r[4].s64 = ctx.r[11].s64 + -4;
	// 82512A14: 409A0008  bne cr6, 0x82512a1c
	if !ctx.cr[6].eq {
	pc = 0x82512A1C; continue 'dispatch;
	}
	// 82512A18: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82512A1C: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 82512A20: 4BFF7E91  bl 0x8250a8b0
	ctx.lr = 0x82512A24;
	sub_8250A8B0(ctx, base);
	// 82512A24: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82512A28: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82512A2C: 388BFF40  addi r4, r11, -0xc0
	ctx.r[4].s64 = ctx.r[11].s64 + -192;
	// 82512A30: 409A0008  bne cr6, 0x82512a38
	if !ctx.cr[6].eq {
	pc = 0x82512A38; continue 'dispatch;
	}
	// 82512A34: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82512A38: 38E10080  addi r7, r1, 0x80
	ctx.r[7].s64 = ctx.r[1].s64 + 128;
	// 82512A3C: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82512A40: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82512A44: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82512A48: 4807CDF9  bl 0x8258f840
	ctx.lr = 0x82512A4C;
	sub_8258F840(ctx, base);
	// 82512A4C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82512A50: 387D0004  addi r3, r29, 4
	ctx.r[3].s64 = ctx.r[29].s64 + 4;
	// 82512A54: 388B0004  addi r4, r11, 4
	ctx.r[4].s64 = ctx.r[11].s64 + 4;
	// 82512A58: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82512A5C: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82512A60: 4BDB1A01  bl 0x822c4460
	ctx.lr = 0x82512A64;
	sub_822C4460(ctx, base);
	// 82512A64: 80610064  lwz r3, 0x64(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82512A68: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82512A6C: 419A0008  beq cr6, 0x82512a74
	if ctx.cr[6].eq {
	pc = 0x82512A74; continue 'dispatch;
	}
	// 82512A70: 4BDADE21  bl 0x822c0890
	ctx.lr = 0x82512A74;
	sub_822C0890(ctx, base);
	// 82512A74: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 82512A78: 488DF219  bl 0x82df1c90
	ctx.lr = 0x82512A7C;
	sub_82DF1C90(ctx, base);
	// 82512A7C: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82512A80: 488DF211  bl 0x82df1c90
	ctx.lr = 0x82512A84;
	sub_82DF1C90(ctx, base);
	// 82512A84: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 82512A88: 4BDB6231  bl 0x822c8cb8
	ctx.lr = 0x82512A8C;
	sub_822C8CB8(ctx, base);
	// 82512A8C: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 82512A90: CBE1FFD0  lfd f31, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-48 as u32) ) };
	// 82512A94: 48C95724  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82512A98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82512A98 size=112
    let mut pc: u32 = 0x82512A98;
    'dispatch: loop {
        match pc {
            0x82512A98 => {
    //   block [0x82512A98..0x82512B08)
	// 82512A98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82512A9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82512AA0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82512AA4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82512AA8: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82512AAC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82512AB0: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82512AB4: 388B1BD0  addi r4, r11, 0x1bd0
	ctx.r[4].s64 = ctx.r[11].s64 + 7120;
	// 82512AB8: 38A00103  li r5, 0x103
	ctx.r[5].s64 = 259;
	// 82512ABC: 38600014  li r3, 0x14
	ctx.r[3].s64 = 20;
	// 82512AC0: 488DF929  bl 0x82df23e8
	ctx.lr = 0x82512AC4;
	sub_82DF23E8(ctx, base);
	// 82512AC4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82512AC8: 41820020  beq 0x82512ae8
	if ctx.cr[0].eq {
	pc = 0x82512AE8; continue 'dispatch;
	}
	// 82512ACC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82512AD0: 389F0028  addi r4, r31, 0x28
	ctx.r[4].s64 = ctx.r[31].s64 + 40;
	// 82512AD4: 409A0008  bne cr6, 0x82512adc
	if !ctx.cr[6].eq {
	pc = 0x82512ADC; continue 'dispatch;
	}
	// 82512AD8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82512ADC: 4BF7AA55  bl 0x8248d530
	ctx.lr = 0x82512AE0;
	sub_8248D530(ctx, base);
	// 82512AE0: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82512AE4: 48000008  b 0x82512aec
	pc = 0x82512AEC; continue 'dispatch;
	// 82512AE8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82512AEC: 387F00CC  addi r3, r31, 0xcc
	ctx.r[3].s64 = ctx.r[31].s64 + 204;
	// 82512AF0: 4BDF2489  bl 0x82304f78
	ctx.lr = 0x82512AF4;
	sub_82304F78(ctx, base);
	// 82512AF4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82512AF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82512AFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82512B00: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82512B04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82512B08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82512B08 size=112
    let mut pc: u32 = 0x82512B08;
    'dispatch: loop {
        match pc {
            0x82512B08 => {
    //   block [0x82512B08..0x82512B78)
	// 82512B08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82512B0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82512B10: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82512B14: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82512B18: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82512B1C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82512B20: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82512B24: 817F00CC  lwz r11, 0xcc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(204 as u32) ) } as u64;
	// 82512B28: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82512B2C: 409A0008  bne cr6, 0x82512b34
	if !ctx.cr[6].eq {
	pc = 0x82512B34; continue 'dispatch;
	}
	// 82512B30: 4BFFFF69  bl 0x82512a98
	ctx.lr = 0x82512B34;
	sub_82512A98(ctx, base);
	// 82512B34: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82512B38: 83FF00CC  lwz r31, 0xcc(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(204 as u32) ) } as u64;
	// 82512B3C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82512B40: 4BFFC9D9  bl 0x8250f518
	ctx.lr = 0x82512B44;
	sub_8250F518(ctx, base);
	// 82512B44: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82512B48: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82512B4C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82512B50: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82512B54: 4BF7A8C5  bl 0x8248d418
	ctx.lr = 0x82512B58;
	sub_8248D418(ctx, base);
	// 82512B58: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82512B5C: 488DF135  bl 0x82df1c90
	ctx.lr = 0x82512B60;
	sub_82DF1C90(ctx, base);
	// 82512B60: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82512B64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82512B68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82512B6C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82512B70: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82512B74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82512B78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82512B78 size=92
    let mut pc: u32 = 0x82512B78;
    'dispatch: loop {
        match pc {
            0x82512B78 => {
    //   block [0x82512B78..0x82512BD4)
	// 82512B78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82512B7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82512B80: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82512B84: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82512B88: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82512B8C: 817F00CC  lwz r11, 0xcc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(204 as u32) ) } as u64;
	// 82512B90: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82512B94: 409A0008  bne cr6, 0x82512b9c
	if !ctx.cr[6].eq {
	pc = 0x82512B9C; continue 'dispatch;
	}
	// 82512B98: 4BFFFF01  bl 0x82512a98
	ctx.lr = 0x82512B9C;
	sub_82512A98(ctx, base);
	// 82512B9C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82512BA0: 83FF00CC  lwz r31, 0xcc(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(204 as u32) ) } as u64;
	// 82512BA4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82512BA8: 4BFFC971  bl 0x8250f518
	ctx.lr = 0x82512BAC;
	sub_8250F518(ctx, base);
	// 82512BAC: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82512BB0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82512BB4: 4BF7A60D  bl 0x8248d1c0
	ctx.lr = 0x82512BB8;
	sub_8248D1C0(ctx, base);
	// 82512BB8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82512BBC: 488DF0D5  bl 0x82df1c90
	ctx.lr = 0x82512BC0;
	sub_82DF1C90(ctx, base);
	// 82512BC0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82512BC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82512BC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82512BCC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82512BD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82512BD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82512BD8 size=64
    let mut pc: u32 = 0x82512BD8;
    'dispatch: loop {
        match pc {
            0x82512BD8 => {
    //   block [0x82512BD8..0x82512C18)
	// 82512BD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82512BDC: 48C95591  bl 0x831a816c
	ctx.lr = 0x82512BE0;
	sub_831A8130(ctx, base);
	// 82512BE0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82512BE4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82512BE8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82512BEC: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82512BF0: 817F00CC  lwz r11, 0xcc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(204 as u32) ) } as u64;
	// 82512BF4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82512BF8: 409A0008  bne cr6, 0x82512c00
	if !ctx.cr[6].eq {
	pc = 0x82512C00; continue 'dispatch;
	}
	// 82512BFC: 4BFFFE9D  bl 0x82512a98
	ctx.lr = 0x82512C00;
	sub_82512A98(ctx, base);
	// 82512C00: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82512C04: 807F00CC  lwz r3, 0xcc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(204 as u32) ) } as u64;
	// 82512C08: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82512C0C: 4BF7A1CD  bl 0x8248cdd8
	ctx.lr = 0x82512C10;
	sub_8248CDD8(ctx, base);
	// 82512C10: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82512C14: 48C955A8  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82512C18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82512C18 size=360
    let mut pc: u32 = 0x82512C18;
    'dispatch: loop {
        match pc {
            0x82512C18 => {
    //   block [0x82512C18..0x82512D80)
	// 82512C18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82512C1C: 48C9553D  bl 0x831a8158
	ctx.lr = 0x82512C20;
	sub_831A8130(ctx, base);
	// 82512C20: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82512C24: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82512C28: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82512C2C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82512C30: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82512C34: 7CFB3B78  mr r27, r7
	ctx.r[27].u64 = ctx.r[7].u64;
	// 82512C38: 817F00CC  lwz r11, 0xcc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(204 as u32) ) } as u64;
	// 82512C3C: 7D1A4378  mr r26, r8
	ctx.r[26].u64 = ctx.r[8].u64;
	// 82512C40: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82512C44: 409A0008  bne cr6, 0x82512c4c
	if !ctx.cr[6].eq {
	pc = 0x82512C4C; continue 'dispatch;
	}
	// 82512C48: 4BFFFE51  bl 0x82512a98
	ctx.lr = 0x82512C4C;
	sub_82512A98(ctx, base);
	// 82512C4C: 807F00C4  lwz r3, 0xc4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(196 as u32) ) } as u64;
	// 82512C50: 833F00CC  lwz r25, 0xcc(r31)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(204 as u32) ) } as u64;
	// 82512C54: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82512C58: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82512C5C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82512C60: 4E800421  bctrl
	ctx.lr = 0x82512C64;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82512C64: 7C781B78  mr r24, r3
	ctx.r[24].u64 = ctx.r[3].u64;
	// 82512C68: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82512C6C: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82512C70: 4BFFC8A9  bl 0x8250f518
	ctx.lr = 0x82512C74;
	sub_8250F518(ctx, base);
	// 82512C74: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82512C78: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82512C7C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82512C80: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82512C84: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 82512C88: 7F48D378  mr r8, r26
	ctx.r[8].u64 = ctx.r[26].u64;
	// 82512C8C: 7F09C378  mr r9, r24
	ctx.r[9].u64 = ctx.r[24].u64;
	// 82512C90: 4BF7A719  bl 0x8248d3a8
	ctx.lr = 0x82512C94;
	sub_8248D3A8(ctx, base);
	// 82512C94: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82512C98: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82512C9C: 488DEFF5  bl 0x82df1c90
	ctx.lr = 0x82512CA0;
	sub_82DF1C90(ctx, base);
	// 82512CA0: 576B063F  clrlwi. r11, r27, 0x18
	ctx.r[11].u64 = ctx.r[27].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82512CA4: 4082005C  bne 0x82512d00
	if !ctx.cr[0].eq {
	pc = 0x82512D00; continue 'dispatch;
	}
	// 82512CA8: 817F00C8  lwz r11, 0xc8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(200 as u32) ) } as u64;
	// 82512CAC: 815F00C4  lwz r10, 0xc4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(196 as u32) ) } as u64;
	// 82512CB0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82512CB4: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82512CB8: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82512CBC: 419A0024  beq cr6, 0x82512ce0
	if ctx.cr[6].eq {
	pc = 0x82512CE0; continue 'dispatch;
	}
	// 82512CC0: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82512CC4: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82512CC8: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82512CCC: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82512CD0: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82512CD4: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82512CD8: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82512CDC: 4082FFE8  bne 0x82512cc4
	if !ctx.cr[0].eq {
	pc = 0x82512CC4; continue 'dispatch;
	}
	// 82512CE0: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82512CE4: 807F00CC  lwz r3, 0xcc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(204 as u32) ) } as u64;
	// 82512CE8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82512CEC: 4BF79FA5  bl 0x8248cc90
	ctx.lr = 0x82512CF0;
	sub_8248CC90(ctx, base);
	// 82512CF0: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82512CF4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82512CF8: 419A0008  beq cr6, 0x82512d00
	if ctx.cr[6].eq {
	pc = 0x82512D00; continue 'dispatch;
	}
	// 82512CFC: 4BDADB95  bl 0x822c0890
	ctx.lr = 0x82512D00;
	sub_822C0890(ctx, base);
	// 82512D00: 4BFFC611  bl 0x8250f310
	ctx.lr = 0x82512D04;
	sub_8250F310(ctx, base);
	// 82512D04: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82512D08: 4182006C  beq 0x82512d74
	if ctx.cr[0].eq {
	pc = 0x82512D74; continue 'dispatch;
	}
	// 82512D0C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82512D10: 839F00CC  lwz r28, 0xcc(r31)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(204 as u32) ) } as u64;
	// 82512D14: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82512D18: 4BFFC7B1  bl 0x8250f4c8
	ctx.lr = 0x82512D1C;
	sub_8250F4C8(ctx, base);
	// 82512D1C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82512D20: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82512D24: 3BCBFFFC  addi r30, r11, -4
	ctx.r[30].s64 = ctx.r[11].s64 + -4;
	// 82512D28: 409A0008  bne cr6, 0x82512d30
	if !ctx.cr[6].eq {
	pc = 0x82512D30; continue 'dispatch;
	}
	// 82512D2C: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82512D30: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82512D34: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82512D38: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82512D3C: 4BF7A025  bl 0x8248cd60
	ctx.lr = 0x82512D40;
	sub_8248CD60(ctx, base);
	// 82512D40: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82512D44: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82512D48: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 82512D4C: 4BFFC7CD  bl 0x8250f518
	ctx.lr = 0x82512D50;
	sub_8250F518(ctx, base);
	// 82512D50: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82512D54: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82512D58: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82512D5C: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 82512D60: 4BFFA7A9  bl 0x8250d508
	ctx.lr = 0x82512D64;
	sub_8250D508(ctx, base);
	// 82512D64: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82512D68: 488DEF29  bl 0x82df1c90
	ctx.lr = 0x82512D6C;
	sub_82DF1C90(ctx, base);
	// 82512D6C: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 82512D70: 488DEF21  bl 0x82df1c90
	ctx.lr = 0x82512D74;
	sub_82DF1C90(ctx, base);
	// 82512D74: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82512D78: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 82512D7C: 48C9542C  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82512D80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82512D80 size=420
    let mut pc: u32 = 0x82512D80;
    'dispatch: loop {
        match pc {
            0x82512D80 => {
    //   block [0x82512D80..0x82512F24)
	// 82512D80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82512D84: 48C953D1  bl 0x831a8154
	ctx.lr = 0x82512D88;
	sub_831A8130(ctx, base);
	// 82512D88: 9421FEF0  stwu r1, -0x110(r1)
	ea = ctx.r[1].u32.wrapping_add(-272 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82512D8C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82512D90: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82512D94: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82512D98: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 82512D9C: 7CFA3B78  mr r26, r7
	ctx.r[26].u64 = ctx.r[7].u64;
	// 82512DA0: 817F00CC  lwz r11, 0xcc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(204 as u32) ) } as u64;
	// 82512DA4: 7D1E4378  mr r30, r8
	ctx.r[30].u64 = ctx.r[8].u64;
	// 82512DA8: 7D394B78  mr r25, r9
	ctx.r[25].u64 = ctx.r[9].u64;
	// 82512DAC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82512DB0: 409A0008  bne cr6, 0x82512db8
	if !ctx.cr[6].eq {
	pc = 0x82512DB8; continue 'dispatch;
	}
	// 82512DB4: 4BFFFCE5  bl 0x82512a98
	ctx.lr = 0x82512DB8;
	sub_82512A98(ctx, base);
	// 82512DB8: 807F00C4  lwz r3, 0xc4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(196 as u32) ) } as u64;
	// 82512DBC: 831F00CC  lwz r24, 0xcc(r31)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(204 as u32) ) } as u64;
	// 82512DC0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82512DC4: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82512DC8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82512DCC: 4E800421  bctrl
	ctx.lr = 0x82512DD0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82512DD0: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82512DD4: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 82512DD8: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82512DDC: 4BDB1B25  bl 0x822c4900
	ctx.lr = 0x82512DE0;
	sub_822C4900(ctx, base);
	// 82512DE0: 7C771B78  mr r23, r3
	ctx.r[23].u64 = ctx.r[3].u64;
	// 82512DE4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82512DE8: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82512DEC: 4BFFC72D  bl 0x8250f518
	ctx.lr = 0x82512DF0;
	sub_8250F518(ctx, base);
	// 82512DF0: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82512DF4: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 82512DF8: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82512DFC: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82512E00: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 82512E04: 7F28CB78  mr r8, r25
	ctx.r[8].u64 = ctx.r[25].u64;
	// 82512E08: 7EE9BB78  mr r9, r23
	ctx.r[9].u64 = ctx.r[23].u64;
	// 82512E0C: 4BF7A59D  bl 0x8248d3a8
	ctx.lr = 0x82512E10;
	sub_8248D3A8(ctx, base);
	// 82512E10: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82512E14: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82512E18: 488DEE79  bl 0x82df1c90
	ctx.lr = 0x82512E1C;
	sub_82DF1C90(ctx, base);
	// 82512E1C: 574B063F  clrlwi. r11, r26, 0x18
	ctx.r[11].u64 = ctx.r[26].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82512E20: 40820084  bne 0x82512ea4
	if !ctx.cr[0].eq {
	pc = 0x82512EA4; continue 'dispatch;
	}
	// 82512E24: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82512E28: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82512E2C: 388B1BD0  addi r4, r11, 0x1bd0
	ctx.r[4].s64 = ctx.r[11].s64 + 7120;
	// 82512E30: 38A0030E  li r5, 0x30e
	ctx.r[5].s64 = 782;
	// 82512E34: 386000E0  li r3, 0xe0
	ctx.r[3].s64 = 224;
	// 82512E38: 488DF5B1  bl 0x82df23e8
	ctx.lr = 0x82512E3C;
	sub_82DF23E8(ctx, base);
	// 82512E3C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82512E40: 41820014  beq 0x82512e54
	if ctx.cr[0].eq {
	pc = 0x82512E54; continue 'dispatch;
	}
	// 82512E44: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82512E48: 489002A9  bl 0x82e130f0
	ctx.lr = 0x82512E4C;
	sub_82E130F0(ctx, base);
	// 82512E4C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82512E50: 48000008  b 0x82512e58
	pc = 0x82512E58; continue 'dispatch;
	// 82512E54: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82512E58: 93C10050  stw r30, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 82512E5C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82512E60: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82512E64: 4BDCE1B5  bl 0x822e1018
	ctx.lr = 0x82512E68;
	sub_822E1018(ctx, base);
	// 82512E68: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82512E6C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82512E70: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82512E74: 4BDAD18D  bl 0x822c0000
	ctx.lr = 0x82512E78;
	sub_822C0000(ctx, base);
	// 82512E78: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82512E7C: 807F00C4  lwz r3, 0xc4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(196 as u32) ) } as u64;
	// 82512E80: 488FFFB1  bl 0x82e12e30
	ctx.lr = 0x82512E84;
	sub_82E12E30(ctx, base);
	// 82512E84: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82512E88: 807F00CC  lwz r3, 0xcc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(204 as u32) ) } as u64;
	// 82512E8C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82512E90: 4BF79E01  bl 0x8248cc90
	ctx.lr = 0x82512E94;
	sub_8248CC90(ctx, base);
	// 82512E94: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82512E98: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82512E9C: 419A0008  beq cr6, 0x82512ea4
	if ctx.cr[6].eq {
	pc = 0x82512EA4; continue 'dispatch;
	}
	// 82512EA0: 4BDAD9F1  bl 0x822c0890
	ctx.lr = 0x82512EA4;
	sub_822C0890(ctx, base);
	// 82512EA4: 4BFFC46D  bl 0x8250f310
	ctx.lr = 0x82512EA8;
	sub_8250F310(ctx, base);
	// 82512EA8: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82512EAC: 4182006C  beq 0x82512f18
	if ctx.cr[0].eq {
	pc = 0x82512F18; continue 'dispatch;
	}
	// 82512EB0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82512EB4: 839F00CC  lwz r28, 0xcc(r31)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(204 as u32) ) } as u64;
	// 82512EB8: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82512EBC: 4BFFC60D  bl 0x8250f4c8
	ctx.lr = 0x82512EC0;
	sub_8250F4C8(ctx, base);
	// 82512EC0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82512EC4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82512EC8: 3BCBFFFC  addi r30, r11, -4
	ctx.r[30].s64 = ctx.r[11].s64 + -4;
	// 82512ECC: 409A0008  bne cr6, 0x82512ed4
	if !ctx.cr[6].eq {
	pc = 0x82512ED4; continue 'dispatch;
	}
	// 82512ED0: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82512ED4: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82512ED8: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82512EDC: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82512EE0: 4BF79E81  bl 0x8248cd60
	ctx.lr = 0x82512EE4;
	sub_8248CD60(ctx, base);
	// 82512EE4: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82512EE8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82512EEC: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 82512EF0: 4BFFC629  bl 0x8250f518
	ctx.lr = 0x82512EF4;
	sub_8250F518(ctx, base);
	// 82512EF4: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82512EF8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82512EFC: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82512F00: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 82512F04: 4BFFA605  bl 0x8250d508
	ctx.lr = 0x82512F08;
	sub_8250D508(ctx, base);
	// 82512F08: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82512F0C: 488DED85  bl 0x82df1c90
	ctx.lr = 0x82512F10;
	sub_82DF1C90(ctx, base);
	// 82512F10: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 82512F14: 488DED7D  bl 0x82df1c90
	ctx.lr = 0x82512F18;
	sub_82DF1C90(ctx, base);
	// 82512F18: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82512F1C: 38210110  addi r1, r1, 0x110
	ctx.r[1].s64 = ctx.r[1].s64 + 272;
	// 82512F20: 48C95284  b 0x831a81a4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82512F28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82512F28 size=296
    let mut pc: u32 = 0x82512F28;
    'dispatch: loop {
        match pc {
            0x82512F28 => {
    //   block [0x82512F28..0x82513050)
	// 82512F28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82512F2C: 48C9522D  bl 0x831a8158
	ctx.lr = 0x82512F30;
	sub_831A8130(ctx, base);
	// 82512F30: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82512F34: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82512F38: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82512F3C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82512F40: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82512F44: 7CFA3B78  mr r26, r7
	ctx.r[26].u64 = ctx.r[7].u64;
	// 82512F48: 817F00CC  lwz r11, 0xcc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(204 as u32) ) } as u64;
	// 82512F4C: 7D1B4378  mr r27, r8
	ctx.r[27].u64 = ctx.r[8].u64;
	// 82512F50: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82512F54: 409A0008  bne cr6, 0x82512f5c
	if !ctx.cr[6].eq {
	pc = 0x82512F5C; continue 'dispatch;
	}
	// 82512F58: 4BFFFB41  bl 0x82512a98
	ctx.lr = 0x82512F5C;
	sub_82512A98(ctx, base);
	// 82512F5C: 807A0000  lwz r3, 0(r26)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82512F60: 833F00CC  lwz r25, 0xcc(r31)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(204 as u32) ) } as u64;
	// 82512F64: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82512F68: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82512F6C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82512F70: 4E800421  bctrl
	ctx.lr = 0x82512F74;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82512F74: 7C781B78  mr r24, r3
	ctx.r[24].u64 = ctx.r[3].u64;
	// 82512F78: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82512F7C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82512F80: 4BFFC599  bl 0x8250f518
	ctx.lr = 0x82512F84;
	sub_8250F518(ctx, base);
	// 82512F84: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82512F88: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82512F8C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82512F90: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82512F94: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 82512F98: 7F68DB78  mr r8, r27
	ctx.r[8].u64 = ctx.r[27].u64;
	// 82512F9C: 7F09C378  mr r9, r24
	ctx.r[9].u64 = ctx.r[24].u64;
	// 82512FA0: 4BF7A409  bl 0x8248d3a8
	ctx.lr = 0x82512FA4;
	sub_8248D3A8(ctx, base);
	// 82512FA4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82512FA8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82512FAC: 488DECE5  bl 0x82df1c90
	ctx.lr = 0x82512FB0;
	sub_82DF1C90(ctx, base);
	// 82512FB0: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 82512FB4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82512FB8: 807F00CC  lwz r3, 0xcc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(204 as u32) ) } as u64;
	// 82512FBC: 4BF79CD5  bl 0x8248cc90
	ctx.lr = 0x82512FC0;
	sub_8248CC90(ctx, base);
	// 82512FC0: 4BFFC351  bl 0x8250f310
	ctx.lr = 0x82512FC4;
	sub_8250F310(ctx, base);
	// 82512FC4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82512FC8: 4182006C  beq 0x82513034
	if ctx.cr[0].eq {
	pc = 0x82513034; continue 'dispatch;
	}
	// 82512FCC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82512FD0: 839F00CC  lwz r28, 0xcc(r31)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(204 as u32) ) } as u64;
	// 82512FD4: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82512FD8: 4BFFC4F1  bl 0x8250f4c8
	ctx.lr = 0x82512FDC;
	sub_8250F4C8(ctx, base);
	// 82512FDC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82512FE0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82512FE4: 3BABFFFC  addi r29, r11, -4
	ctx.r[29].s64 = ctx.r[11].s64 + -4;
	// 82512FE8: 409A0008  bne cr6, 0x82512ff0
	if !ctx.cr[6].eq {
	pc = 0x82512FF0; continue 'dispatch;
	}
	// 82512FEC: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82512FF0: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82512FF4: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82512FF8: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 82512FFC: 4BF79D65  bl 0x8248cd60
	ctx.lr = 0x82513000;
	sub_8248CD60(ctx, base);
	// 82513000: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82513004: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82513008: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8251300C: 4BFFC50D  bl 0x8250f518
	ctx.lr = 0x82513010;
	sub_8250F518(ctx, base);
	// 82513010: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82513014: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82513018: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 8251301C: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 82513020: 4BFFA4E9  bl 0x8250d508
	ctx.lr = 0x82513024;
	sub_8250D508(ctx, base);
	// 82513024: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82513028: 488DEC69  bl 0x82df1c90
	ctx.lr = 0x8251302C;
	sub_82DF1C90(ctx, base);
	// 8251302C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82513030: 488DEC61  bl 0x82df1c90
	ctx.lr = 0x82513034;
	sub_82DF1C90(ctx, base);
	// 82513034: 807A0004  lwz r3, 4(r26)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 82513038: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8251303C: 419A0008  beq cr6, 0x82513044
	if ctx.cr[6].eq {
	pc = 0x82513044; continue 'dispatch;
	}
	// 82513040: 4BDAD851  bl 0x822c0890
	ctx.lr = 0x82513044;
	sub_822C0890(ctx, base);
	// 82513044: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82513048: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 8251304C: 48C9515C  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82513050(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82513050 size=8
    let mut pc: u32 = 0x82513050;
    'dispatch: loop {
        match pc {
            0x82513050 => {
    //   block [0x82513050..0x82513058)
	// 82513050: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82513054: 4BFFFBC4  b 0x82512c18
	sub_82512C18(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82513058(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82513058 size=8
    let mut pc: u32 = 0x82513058;
    'dispatch: loop {
        match pc {
            0x82513058 => {
    //   block [0x82513058..0x82513060)
	// 82513058: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8251305C: 4BFFFD24  b 0x82512d80
	sub_82512D80(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82513060(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82513060 size=136
    let mut pc: u32 = 0x82513060;
    'dispatch: loop {
        match pc {
            0x82513060 => {
    //   block [0x82513060..0x825130E8)
	// 82513060: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82513064: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82513068: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8251306C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82513070: 7CFF3B78  mr r31, r7
	ctx.r[31].u64 = ctx.r[7].u64;
	// 82513074: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82513078: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251307C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82513080: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82513084: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82513088: 419A0024  beq cr6, 0x825130ac
	if ctx.cr[6].eq {
	pc = 0x825130AC; continue 'dispatch;
	}
	// 8251308C: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82513090: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82513094: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82513098: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8251309C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 825130A0: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 825130A4: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 825130A8: 4082FFE8  bne 0x82513090
	if !ctx.cr[0].eq {
	pc = 0x82513090; continue 'dispatch;
	}
	// 825130AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825130B0: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 825130B4: 4BFFFE75  bl 0x82512f28
	ctx.lr = 0x825130B8;
	sub_82512F28(ctx, base);
	// 825130B8: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 825130BC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825130C0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825130C4: 419A000C  beq cr6, 0x825130d0
	if ctx.cr[6].eq {
	pc = 0x825130D0; continue 'dispatch;
	}
	// 825130C8: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 825130CC: 4BDAD7C5  bl 0x822c0890
	ctx.lr = 0x825130D0;
	sub_822C0890(ctx, base);
	// 825130D0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825130D4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825130D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825130DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825130E0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825130E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825130E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825130E8 size=8
    let mut pc: u32 = 0x825130E8;
    'dispatch: loop {
        match pc {
            0x825130E8 => {
    //   block [0x825130E8..0x825130F0)
	// 825130E8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 825130EC: 4BFFFB2C  b 0x82512c18
	sub_82512C18(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825130F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825130F0 size=8
    let mut pc: u32 = 0x825130F0;
    'dispatch: loop {
        match pc {
            0x825130F0 => {
    //   block [0x825130F0..0x825130F8)
	// 825130F0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 825130F4: 4BFFFC8C  b 0x82512d80
	sub_82512D80(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825130F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825130F8 size=136
    let mut pc: u32 = 0x825130F8;
    'dispatch: loop {
        match pc {
            0x825130F8 => {
    //   block [0x825130F8..0x82513180)
	// 825130F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825130FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82513100: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82513104: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82513108: 7CFF3B78  mr r31, r7
	ctx.r[31].u64 = ctx.r[7].u64;
	// 8251310C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82513110: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82513114: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82513118: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8251311C: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82513120: 419A0024  beq cr6, 0x82513144
	if ctx.cr[6].eq {
	pc = 0x82513144; continue 'dispatch;
	}
	// 82513124: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82513128: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8251312C: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82513130: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82513134: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82513138: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8251313C: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82513140: 4082FFE8  bne 0x82513128
	if !ctx.cr[0].eq {
	pc = 0x82513128; continue 'dispatch;
	}
	// 82513144: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82513148: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 8251314C: 4BFFFDDD  bl 0x82512f28
	ctx.lr = 0x82513150;
	sub_82512F28(ctx, base);
	// 82513150: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82513154: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82513158: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8251315C: 419A000C  beq cr6, 0x82513168
	if ctx.cr[6].eq {
	pc = 0x82513168; continue 'dispatch;
	}
	// 82513160: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82513164: 4BDAD72D  bl 0x822c0890
	ctx.lr = 0x82513168;
	sub_822C0890(ctx, base);
	// 82513168: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8251316C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82513170: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82513174: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82513178: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8251317C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82513180(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82513180 size=8
    let mut pc: u32 = 0x82513180;
    'dispatch: loop {
        match pc {
            0x82513180 => {
    //   block [0x82513180..0x82513188)
	// 82513180: 3863FFD8  addi r3, r3, -0x28
	ctx.r[3].s64 = ctx.r[3].s64 + -40;
	// 82513184: 48000064  b 0x825131e8
	sub_825131E8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82513188(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82513188 size=96
    let mut pc: u32 = 0x82513188;
    'dispatch: loop {
        match pc {
            0x82513188 => {
    //   block [0x82513188..0x825131E8)
	// 82513188: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251318C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82513190: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82513194: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82513198: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8251319C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 825131A0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825131A4: 396B1C3C  addi r11, r11, 0x1c3c
	ctx.r[11].s64 = ctx.r[11].s64 + 7228;
	// 825131A8: 394A1C28  addi r10, r10, 0x1c28
	ctx.r[10].s64 = ctx.r[10].s64 + 7208;
	// 825131AC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825131B0: 387F00F4  addi r3, r31, 0xf4
	ctx.r[3].s64 = ctx.r[31].s64 + 244;
	// 825131B4: 915F0028  stw r10, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[10].u32 ) };
	// 825131B8: 488E0271  bl 0x82df3428
	ctx.lr = 0x825131BC;
	sub_82DF3428(ctx, base);
	// 825131BC: 807F00E8  lwz r3, 0xe8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(232 as u32) ) } as u64;
	// 825131C0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825131C4: 419A0008  beq cr6, 0x825131cc
	if ctx.cr[6].eq {
	pc = 0x825131CC; continue 'dispatch;
	}
	// 825131C8: 4BDAD6C9  bl 0x822c0890
	ctx.lr = 0x825131CC;
	sub_822C0890(ctx, base);
	// 825131CC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825131D0: 4BE3C181  bl 0x8234f350
	ctx.lr = 0x825131D4;
	sub_8234F350(ctx, base);
	// 825131D4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825131D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825131DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825131E0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825131E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825131E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825131E8 size=76
    let mut pc: u32 = 0x825131E8;
    'dispatch: loop {
        match pc {
            0x825131E8 => {
    //   block [0x825131E8..0x82513234)
	// 825131E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825131EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825131F0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825131F4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825131F8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825131FC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82513200: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82513204: 4BFFFF85  bl 0x82513188
	ctx.lr = 0x82513208;
	sub_82513188(ctx, base);
	// 82513208: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8251320C: 4182000C  beq 0x82513218
	if ctx.cr[0].eq {
	pc = 0x82513218; continue 'dispatch;
	}
	// 82513210: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82513214: 488DF1C5  bl 0x82df23d8
	ctx.lr = 0x82513218;
	sub_82DF23D8(ctx, base);
	// 82513218: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8251321C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82513220: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82513224: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82513228: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8251322C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82513230: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82513238(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82513238 size=256
    let mut pc: u32 = 0x82513238;
    'dispatch: loop {
        match pc {
            0x82513238 => {
    //   block [0x82513238..0x82513338)
	// 82513238: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251323C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82513240: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82513244: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82513248: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251324C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82513250: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82513254: 897F00F0  lbz r11, 0xf0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(240 as u32) ) } as u64;
	// 82513258: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8251325C: 408200C4  bne 0x82513320
	if !ctx.cr[0].eq {
	pc = 0x82513320; continue 'dispatch;
	}
	// 82513260: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82513264: 80840000  lwz r4, 0(r4)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82513268: 48917881  bl 0x82e2aae8
	ctx.lr = 0x8251326C;
	sub_82E2AAE8(ctx, base);
	// 8251326C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82513270: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82513274: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 82513278: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8251327C: 4891BBF5  bl 0x82e2ee70
	ctx.lr = 0x82513280;
	sub_82E2EE70(ctx, base);
	// 82513280: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82513284: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82513288: 388B1C78  addi r4, r11, 0x1c78
	ctx.r[4].s64 = ctx.r[11].s64 + 7288;
	// 8251328C: 38A00064  li r5, 0x64
	ctx.r[5].s64 = 100;
	// 82513290: 38600098  li r3, 0x98
	ctx.r[3].s64 = 152;
	// 82513294: 488DF155  bl 0x82df23e8
	ctx.lr = 0x82513298;
	sub_82DF23E8(ctx, base);
	// 82513298: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8251329C: 41820014  beq 0x825132b0
	if ctx.cr[0].eq {
	pc = 0x825132B0; continue 'dispatch;
	}
	// 825132A0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 825132A4: 489035ED  bl 0x82e16890
	ctx.lr = 0x825132A8;
	sub_82E16890(ctx, base);
	// 825132A8: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 825132AC: 48000008  b 0x825132b4
	pc = 0x825132B4; continue 'dispatch;
	// 825132B0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 825132B4: 387F00E4  addi r3, r31, 0xe4
	ctx.r[3].s64 = ctx.r[31].s64 + 228;
	// 825132B8: 4BE4A0B9  bl 0x8235d370
	ctx.lr = 0x825132BC;
	sub_8235D370(ctx, base);
	// 825132BC: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 825132C0: 80FF00E4  lwz r7, 0xe4(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(228 as u32) ) } as u64;
	// 825132C4: 39000010  li r8, 0x10
	ctx.r[8].s64 = 16;
	// 825132C8: 396B6880  addi r11, r11, 0x6880
	ctx.r[11].s64 = ctx.r[11].s64 + 26752;
	// 825132CC: 39200020  li r9, 0x20
	ctx.r[9].s64 = 32;
	// 825132D0: 39400030  li r10, 0x30
	ctx.r[10].s64 = 48;
	// 825132D4: 80670084  lwz r3, 0x84(r7)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(132 as u32) ) } as u64;
	// 825132D8: 13E85C07  vcmpneb. (lvlx128) v31, v8, v11
	tmp.u32 = ctx.r[8].u32 + ctx.r[11].u32;
	// load shuffled into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 825132DC: 13C95C07  vcmpneb. (lvlx128) v30, v9, v11
	tmp.u32 = ctx.r[9].u32 + ctx.r[11].u32;
	// load shuffled into ctx.v[62] using VectorMaskL[(tmp.u32 & 0xF)]
	// 825132E0: 13AA5C07  vcmpneb. (lvlx128) v29, v10, v11
	tmp.u32 = ctx.r[10].u32 + ctx.r[11].u32;
	// load shuffled into ctx.v[61] using VectorMaskL[(tmp.u32 & 0xF)]
	// 825132E4: 13805C07  vcmpneb. (lvlx128) v28, v0, v11
	tmp.u32 = ctx.r[11].u32;
	// load shuffled into ctx.v[60] using VectorMaskL[(tmp.u32 & 0xF)]
	// 825132E8: 39630010  addi r11, r3, 0x10
	ctx.r[11].s64 = ctx.r[3].s64 + 16;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82513338(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82513338 size=112
    let mut pc: u32 = 0x82513338;
    'dispatch: loop {
        match pc {
            0x82513338 => {
    //   block [0x82513338..0x825133A8)
	// 82513338: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251333C: 48C94E2D  bl 0x831a8168
	ctx.lr = 0x82513340;
	sub_831A8130(ctx, base);
	// 82513340: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82513344: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82513348: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8251334C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82513350: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82513354: 4BFFEFDD  bl 0x82512330
	ctx.lr = 0x82513358;
	sub_82512330(ctx, base);
	// 82513358: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8251335C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82513360: 392B1C3C  addi r9, r11, 0x1c3c
	ctx.r[9].s64 = ctx.r[11].s64 + 7228;
	// 82513364: 394A1C28  addi r10, r10, 0x1c28
	ctx.r[10].s64 = ctx.r[10].s64 + 7208;
	// 82513368: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8251336C: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82513370: 915F0028  stw r10, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[10].u32 ) };
	// 82513374: 387F00F4  addi r3, r31, 0xf4
	ctx.r[3].s64 = ctx.r[31].s64 + 244;
	// 82513378: 917F00E4  stw r11, 0xe4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(228 as u32), ctx.r[11].u32 ) };
	// 8251337C: 917F00E8  stw r11, 0xe8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(232 as u32), ctx.r[11].u32 ) };
	// 82513380: 93BF00EC  stw r29, 0xec(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(236 as u32), ctx.r[29].u32 ) };
	// 82513384: 997F00F0  stb r11, 0xf0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(240 as u32), ctx.r[11].u8 ) };
	// 82513388: 488DFD69  bl 0x82df30f0
	ctx.lr = 0x8251338C;
	sub_82DF30F0(ctx, base);
	// 8251338C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82513390: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82513394: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82513398: 4BFFFEA1  bl 0x82513238
	ctx.lr = 0x8251339C;
	sub_82513238(ctx, base);
	// 8251339C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825133A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825133A4: 48C94E14  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825133A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825133A8 size=108
    let mut pc: u32 = 0x825133A8;
    'dispatch: loop {
        match pc {
            0x825133A8 => {
    //   block [0x825133A8..0x82513414)
	// 825133A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825133AC: 48C94DBD  bl 0x831a8168
	ctx.lr = 0x825133B0;
	sub_831A8130(ctx, base);
	// 825133B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825133B4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825133B8: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 825133BC: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 825133C0: 4BFFE619  bl 0x825119d8
	ctx.lr = 0x825133C4;
	sub_825119D8(ctx, base);
	// 825133C4: 897F00F0  lbz r11, 0xf0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(240 as u32) ) } as u64;
	// 825133C8: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 825133CC: 40820020  bne 0x825133ec
	if !ctx.cr[0].eq {
	pc = 0x825133EC; continue 'dispatch;
	}
	// 825133D0: 3BDF00F4  addi r30, r31, 0xf4
	ctx.r[30].s64 = ctx.r[31].s64 + 244;
	// 825133D4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 825133D8: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 825133DC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825133E0: 4BFFFE59  bl 0x82513238
	ctx.lr = 0x825133E4;
	sub_82513238(ctx, base);
	// 825133E4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825133E8: 488E0041  bl 0x82df3428
	ctx.lr = 0x825133EC;
	sub_82DF3428(ctx, base);
	// 825133EC: 389F0028  addi r4, r31, 0x28
	ctx.r[4].s64 = ctx.r[31].s64 + 40;
	// 825133F0: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 825133F4: 4BFF53AD  bl 0x825087a0
	ctx.lr = 0x825133F8;
	sub_825087A0(ctx, base);
	// 825133F8: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 825133FC: 38BF00E4  addi r5, r31, 0xe4
	ctx.r[5].s64 = ctx.r[31].s64 + 228;
	// 82513400: 809F00EC  lwz r4, 0xec(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(236 as u32) ) } as u64;
	// 82513404: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82513408: 4BFFD751  bl 0x82510b58
	ctx.lr = 0x8251340C;
	sub_82510B58(ctx, base);
	// 8251340C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82513410: 48C94DA8  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82513418(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82513418 size=64
    let mut pc: u32 = 0x82513418;
    'dispatch: loop {
        match pc {
            0x82513418 => {
    //   block [0x82513418..0x82513458)
	// 82513418: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251341C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82513420: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82513424: 896300D4  lbz r11, 0xd4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(212 as u32) ) } as u64;
	// 82513428: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8251342C: 41820018  beq 0x82513444
	if ctx.cr[0].eq {
	pc = 0x82513444; continue 'dispatch;
	}
	// 82513430: 38840020  addi r4, r4, 0x20
	ctx.r[4].s64 = ctx.r[4].s64 + 32;
	// 82513434: 806300C0  lwz r3, 0xc0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(192 as u32) ) } as u64;
	// 82513438: 4BDD9BA9  bl 0x822ecfe0
	ctx.lr = 0x8251343C;
	sub_822ECFE0(ctx, base);
	// 8251343C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82513440: 48000008  b 0x82513448
	pc = 0x82513448; continue 'dispatch;
	// 82513444: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82513448: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8251344C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82513450: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82513454: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82513458(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82513458 size=68
    let mut pc: u32 = 0x82513458;
    'dispatch: loop {
        match pc {
            0x82513458 => {
    //   block [0x82513458..0x8251349C)
	// 82513458: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251345C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82513460: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82513464: 896300D4  lbz r11, 0xd4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(212 as u32) ) } as u64;
	// 82513468: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8251346C: 4182001C  beq 0x82513488
	if ctx.cr[0].eq {
	pc = 0x82513488; continue 'dispatch;
	}
	// 82513470: 38A40030  addi r5, r4, 0x30
	ctx.r[5].s64 = ctx.r[4].s64 + 48;
	// 82513474: 806300C0  lwz r3, 0xc0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(192 as u32) ) } as u64;
	// 82513478: 38840020  addi r4, r4, 0x20
	ctx.r[4].s64 = ctx.r[4].s64 + 32;
	// 8251347C: 4BDD9BB5  bl 0x822ed030
	ctx.lr = 0x82513480;
	sub_822ED030(ctx, base);
	// 82513480: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82513484: 48000008  b 0x8251348c
	pc = 0x8251348C; continue 'dispatch;
	// 82513488: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8251348C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82513490: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82513494: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82513498: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825134A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825134A0 size=20
    let mut pc: u32 = 0x825134A0;
    'dispatch: loop {
        match pc {
            0x825134A0 => {
    //   block [0x825134A0..0x825134B4)
	// 825134A0: 896300D4  lbz r11, 0xd4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(212 as u32) ) } as u64;
	// 825134A4: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 825134A8: 4182000C  beq 0x825134b4
	if ctx.cr[0].eq {
		sub_825134B4(ctx, base);
		return;
	}
	// 825134AC: 806300C0  lwz r3, 0xc0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(192 as u32) ) } as u64;
	// 825134B0: 4BF7BEF0  b 0x8248f3a0
	sub_8248F3A0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825134B4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825134B4 size=8
    let mut pc: u32 = 0x825134B4;
    'dispatch: loop {
        match pc {
            0x825134B4 => {
    //   block [0x825134B4..0x825134BC)
	// 825134B4: 908300E4  stw r4, 0xe4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(228 as u32), ctx.r[4].u32 ) };
	// 825134B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825134C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825134C0 size=96
    let mut pc: u32 = 0x825134C0;
    'dispatch: loop {
        match pc {
            0x825134C0 => {
    //   block [0x825134C0..0x82513520)
	// 825134C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825134C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825134C8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825134CC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825134D0: 896300D4  lbz r11, 0xd4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(212 as u32) ) } as u64;
	// 825134D4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 825134D8: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 825134DC: 4182002C  beq 0x82513508
	if ctx.cr[0].eq {
	pc = 0x82513508; continue 'dispatch;
	}
	// 825134E0: 806300C0  lwz r3, 0xc0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(192 as u32) ) } as u64;
	// 825134E4: 4BDDA405  bl 0x822ed8e8
	ctx.lr = 0x825134E8;
	sub_822ED8E8(ctx, base);
	// 825134E8: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825134EC: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 825134F0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825134F4: 40820008  bne 0x825134fc
	if !ctx.cr[0].eq {
	pc = 0x825134FC; continue 'dispatch;
	}
	// 825134F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825134FC: 994B0000  stb r10, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 82513500: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82513504: 48000008  b 0x8251350c
	pc = 0x8251350C; continue 'dispatch;
	// 82513508: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8251350C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82513510: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82513514: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82513518: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8251351C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82513520(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82513520 size=12
    let mut pc: u32 = 0x82513520;
    'dispatch: loop {
        match pc {
            0x82513520 => {
    //   block [0x82513520..0x8251352C)
	// 82513520: 896300D4  lbz r11, 0xd4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(212 as u32) ) } as u64;
	// 82513524: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82513528: 4D820020  beqlr
	if ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251352C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8251352C size=8
    let mut pc: u32 = 0x8251352C;
    'dispatch: loop {
        match pc {
            0x8251352C => {
    //   block [0x8251352C..0x82513534)
	// 8251352C: 806300C0  lwz r3, 0xc0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(192 as u32) ) } as u64;
	// 82513530: 4BDDA3F0  b 0x822ed920
	sub_822ED920(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82513534(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82513534 size=4
    let mut pc: u32 = 0x82513534;
    'dispatch: loop {
        match pc {
            0x82513534 => {
    //   block [0x82513534..0x82513538)
	// 82513534: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82513538(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82513538 size=60
    let mut pc: u32 = 0x82513538;
    'dispatch: loop {
        match pc {
            0x82513538 => {
    //   block [0x82513538..0x82513574)
	// 82513538: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251353C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82513540: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82513544: 896300D4  lbz r11, 0xd4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(212 as u32) ) } as u64;
	// 82513548: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8251354C: 41820014  beq 0x82513560
	if ctx.cr[0].eq {
	pc = 0x82513560; continue 'dispatch;
	}
	// 82513550: 806300C0  lwz r3, 0xc0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(192 as u32) ) } as u64;
	// 82513554: 4BDD99F5  bl 0x822ecf48
	ctx.lr = 0x82513558;
	sub_822ECF48(ctx, base);
	// 82513558: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8251355C: 48000008  b 0x82513564
	pc = 0x82513564; continue 'dispatch;
	// 82513560: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82513564: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82513568: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8251356C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82513570: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82513578(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82513578 size=108
    let mut pc: u32 = 0x82513578;
    'dispatch: loop {
        match pc {
            0x82513578 => {
    //   block [0x82513578..0x825135E4)
	// 82513578: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251357C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82513580: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82513584: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82513588: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8251358C: 808300C0  lwz r4, 0xc0(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(192 as u32) ) } as u64;
	// 82513590: 896300D4  lbz r11, 0xd4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(212 as u32) ) } as u64;
	// 82513594: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82513598: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8251359C: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 825135A0: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 825135A4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825135A8: 41820014  beq 0x825135bc
	if ctx.cr[0].eq {
	pc = 0x825135BC; continue 'dispatch;
	}
	// 825135AC: 4E800421  bctrl
	ctx.lr = 0x825135B0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825135B0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 825135B4: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 825135B8: 48000010  b 0x825135c8
	pc = 0x825135C8; continue 'dispatch;
	// 825135BC: 4E800421  bctrl
	ctx.lr = 0x825135C0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825135C0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 825135C4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 825135C8: 13E058C7  vcmpequd (lvx128) v31, v0, v11
	tmp.u32 = ctx.r[11].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825135E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x825135E8 size=116
    let mut pc: u32 = 0x825135E8;
    'dispatch: loop {
        match pc {
            0x825135E8 => {
    //   block [0x825135E8..0x8251365C)
	// 825135E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825135EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825135F0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825135F4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825135F8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 825135FC: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82513600: 894B00D4  lbz r10, 0xd4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(212 as u32) ) } as u64;
	// 82513604: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82513608: 4182003C  beq 0x82513644
	if ctx.cr[0].eq {
	pc = 0x82513644; continue 'dispatch;
	}
	// 8251360C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82513610: 808B00C0  lwz r4, 0xc0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(192 as u32) ) } as u64;
	// 82513614: 4BDD995D  bl 0x822ecf70
	ctx.lr = 0x82513618;
	sub_822ECF70(ctx, base);
	// 82513618: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8251361C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82513620: C00B0000  lfs f0, 0(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82513624: D01F0000  stfs f0, 0(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82513628: C00B0004  lfs f0, 4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8251362C: D01F0004  stfs f0, 4(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82513630: C00B0008  lfs f0, 8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82513634: D01F0008  stfs f0, 8(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82513638: C00B000C  lfs f0, 0xc(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8251363C: D01F000C  stfs f0, 0xc(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 82513640: 48000008  b 0x82513648
	pc = 0x82513648; continue 'dispatch;
	// 82513644: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82513648: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8251364C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82513650: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82513654: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82513658: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82513660(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82513660 size=8
    let mut pc: u32 = 0x82513660;
    'dispatch: loop {
        match pc {
            0x82513660 => {
    //   block [0x82513660..0x82513668)
	// 82513660: 806300C0  lwz r3, 0xc0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(192 as u32) ) } as u64;
	// 82513664: 4BDD9AEC  b 0x822ed150
	sub_822ED150(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82513668(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82513668 size=92
    let mut pc: u32 = 0x82513668;
    'dispatch: loop {
        match pc {
            0x82513668 => {
    //   block [0x82513668..0x825136C4)
	// 82513668: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251366C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82513670: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82513674: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82513678: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8251367C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82513680: 894B00D4  lbz r10, 0xd4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(212 as u32) ) } as u64;
	// 82513684: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82513688: 41820024  beq 0x825136ac
	if ctx.cr[0].eq {
	pc = 0x825136AC; continue 'dispatch;
	}
	// 8251368C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82513690: 808B00C0  lwz r4, 0xc0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(192 as u32) ) } as u64;
	// 82513694: 4BDD9905  bl 0x822ecf98
	ctx.lr = 0x82513698;
	sub_822ECF98(ctx, base);
	// 82513698: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8251369C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 825136A0: 13E058C7  vcmpequd (lvx128) v31, v0, v11
	tmp.u32 = ctx.r[11].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825136C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825136C8 size=128
    let mut pc: u32 = 0x825136C8;
    'dispatch: loop {
        match pc {
            0x825136C8 => {
    //   block [0x825136C8..0x82513748)
	// 825136C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825136CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825136D0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825136D4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825136D8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825136DC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 825136E0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825136E4: 396B1CD0  addi r11, r11, 0x1cd0
	ctx.r[11].s64 = ctx.r[11].s64 + 7376;
	// 825136E8: 394A1CBC  addi r10, r10, 0x1cbc
	ctx.r[10].s64 = ctx.r[10].s64 + 7356;
	// 825136EC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825136F0: 387F00E0  addi r3, r31, 0xe0
	ctx.r[3].s64 = ctx.r[31].s64 + 224;
	// 825136F4: 915F0028  stw r10, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[10].u32 ) };
	// 825136F8: 488DFD31  bl 0x82df3428
	ctx.lr = 0x825136FC;
	sub_82DF3428(ctx, base);
	// 825136FC: 387F00DC  addi r3, r31, 0xdc
	ctx.r[3].s64 = ctx.r[31].s64 + 220;
	// 82513700: 488DFD29  bl 0x82df3428
	ctx.lr = 0x82513704;
	sub_82DF3428(ctx, base);
	// 82513704: 387F00D8  addi r3, r31, 0xd8
	ctx.r[3].s64 = ctx.r[31].s64 + 216;
	// 82513708: 488DFD21  bl 0x82df3428
	ctx.lr = 0x8251370C;
	sub_82DF3428(ctx, base);
	// 8251370C: 807F00CC  lwz r3, 0xcc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(204 as u32) ) } as u64;
	// 82513710: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82513714: 419A0008  beq cr6, 0x8251371c
	if ctx.cr[6].eq {
	pc = 0x8251371C; continue 'dispatch;
	}
	// 82513718: 4BDAD179  bl 0x822c0890
	ctx.lr = 0x8251371C;
	sub_822C0890(ctx, base);
	// 8251371C: 807F00C4  lwz r3, 0xc4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(196 as u32) ) } as u64;
	// 82513720: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82513724: 419A0008  beq cr6, 0x8251372c
	if ctx.cr[6].eq {
	pc = 0x8251372C; continue 'dispatch;
	}
	// 82513728: 4BDAD169  bl 0x822c0890
	ctx.lr = 0x8251372C;
	sub_822C0890(ctx, base);
	// 8251372C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82513730: 4BFFDA69  bl 0x82511198
	ctx.lr = 0x82513734;
	sub_82511198(ctx, base);
	// 82513734: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82513738: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8251373C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82513740: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82513744: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82513748(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82513748 size=8
    let mut pc: u32 = 0x82513748;
    'dispatch: loop {
        match pc {
            0x82513748 => {
    //   block [0x82513748..0x82513750)
	// 82513748: 3863FFD8  addi r3, r3, -0x28
	ctx.r[3].s64 = ctx.r[3].s64 + -40;
	// 8251374C: 48000344  b 0x82513a90
	sub_82513A90(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82513750(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82513750 size=8
    let mut pc: u32 = 0x82513750;
    'dispatch: loop {
        match pc {
            0x82513750 => {
    //   block [0x82513750..0x82513758)
	// 82513750: 80840018  lwz r4, 0x18(r4)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(24 as u32) ) } as u64;
	// 82513754: 4BFFFE24  b 0x82513578
	sub_82513578(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82513758(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82513758 size=8
    let mut pc: u32 = 0x82513758;
    'dispatch: loop {
        match pc {
            0x82513758 => {
    //   block [0x82513758..0x82513760)
	// 82513758: 80840018  lwz r4, 0x18(r4)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(24 as u32) ) } as u64;
	// 8251375C: 4BFFFE8C  b 0x825135e8
	sub_825135E8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82513760(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82513760 size=8
    let mut pc: u32 = 0x82513760;
    'dispatch: loop {
        match pc {
            0x82513760 => {
    //   block [0x82513760..0x82513768)
	// 82513760: 80840018  lwz r4, 0x18(r4)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(24 as u32) ) } as u64;
	// 82513764: 4BFFFF04  b 0x82513668
	sub_82513668(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82513768(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82513768 size=64
    let mut pc: u32 = 0x82513768;
    'dispatch: loop {
        match pc {
            0x82513768 => {
    //   block [0x82513768..0x825137A8)
	// 82513768: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251376C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82513770: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82513774: 896300D4  lbz r11, 0xd4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(212 as u32) ) } as u64;
	// 82513778: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8251377C: 41820018  beq 0x82513794
	if ctx.cr[0].eq {
	pc = 0x82513794; continue 'dispatch;
	}
	// 82513780: 80840018  lwz r4, 0x18(r4)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(24 as u32) ) } as u64;
	// 82513784: 806300C0  lwz r3, 0xc0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(192 as u32) ) } as u64;
	// 82513788: 4BDD97C1  bl 0x822ecf48
	ctx.lr = 0x8251378C;
	sub_822ECF48(ctx, base);
	// 8251378C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82513790: 48000008  b 0x82513798
	pc = 0x82513798; continue 'dispatch;
	// 82513794: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82513798: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8251379C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825137A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825137A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825137A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825137A8 size=12
    let mut pc: u32 = 0x825137A8;
    'dispatch: loop {
        match pc {
            0x825137A8 => {
    //   block [0x825137A8..0x825137B4)
	// 825137A8: 896300D4  lbz r11, 0xd4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(212 as u32) ) } as u64;
	// 825137AC: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 825137B0: 4D820020  beqlr
	if ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825137B4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825137B4 size=12
    let mut pc: u32 = 0x825137B4;
    'dispatch: loop {
        match pc {
            0x825137B4 => {
    //   block [0x825137B4..0x825137C0)
	// 825137B4: 806300C0  lwz r3, 0xc0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(192 as u32) ) } as u64;
	// 825137B8: 38840020  addi r4, r4, 0x20
	ctx.r[4].s64 = ctx.r[4].s64 + 32;
	// 825137BC: 4BDDA164  b 0x822ed920
	sub_822ED920(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825137C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825137C0 size=4
    let mut pc: u32 = 0x825137C0;
    'dispatch: loop {
        match pc {
            0x825137C0 => {
    //   block [0x825137C0..0x825137C4)
	// 825137C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825137C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x825137C8 size=120
    let mut pc: u32 = 0x825137C8;
    'dispatch: loop {
        match pc {
            0x825137C8 => {
    //   block [0x825137C8..0x82513840)
	// 825137C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825137CC: 48C949A1  bl 0x831a816c
	ctx.lr = 0x825137D0;
	sub_831A8130(ctx, base);
	// 825137D0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825137D4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825137D8: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 825137DC: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 825137E0: 394BBA80  addi r10, r11, -0x4580
	ctx.r[10].s64 = ctx.r[11].s64 + -17792;
	// 825137E4: 3BDF00F0  addi r30, r31, 0xf0
	ctx.r[30].s64 = ctx.r[31].s64 + 240;
	// 825137E8: 9BBF00D4  stb r29, 0xd4(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(212 as u32), ctx.r[29].u8 ) };
	// 825137EC: C00BBA80  lfs f0, -0x4580(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-17792 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825137F0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825137F4: D01F00F0  stfs f0, 0xf0(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(240 as u32), tmp.u32 ) };
	// 825137F8: C00A0004  lfs f0, 4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825137FC: D01F00F4  stfs f0, 0xf4(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(244 as u32), tmp.u32 ) };
	// 82513800: C00A0008  lfs f0, 8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82513804: D01F00F8  stfs f0, 0xf8(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(248 as u32), tmp.u32 ) };
	// 82513808: C00A000C  lfs f0, 0xc(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8251380C: D01F00FC  stfs f0, 0xfc(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(252 as u32), tmp.u32 ) };
	// 82513810: 4BDC0B61  bl 0x822d4370
	ctx.lr = 0x82513814;
	sub_822D4370(ctx, base);
	// 82513814: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 82513818: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 8251381C: 396B6910  addi r11, r11, 0x6910
	ctx.r[11].s64 = ctx.r[11].s64 + 26896;
	// 82513820: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82513824: 13E058C7  vcmpequd (lvx128) v31, v0, v11
	tmp.u32 = ctx.r[11].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82513840(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82513840 size=92
    let mut pc: u32 = 0x82513840;
    'dispatch: loop {
        match pc {
            0x82513840 => {
    //   block [0x82513840..0x8251389C)
	// 82513840: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82513844: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82513848: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8251384C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82513850: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82513854: 897F00D4  lbz r11, 0xd4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(212 as u32) ) } as u64;
	// 82513858: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8251385C: 41820010  beq 0x8251386c
	if ctx.cr[0].eq {
	pc = 0x8251386C; continue 'dispatch;
	}
	// 82513860: 807F00C0  lwz r3, 0xc0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(192 as u32) ) } as u64;
	// 82513864: 4BDD9EBD  bl 0x822ed720
	ctx.lr = 0x82513868;
	sub_822ED720(ctx, base);
	// 82513868: 48000020  b 0x82513888
	pc = 0x82513888; continue 'dispatch;
	// 8251386C: 387F00F0  addi r3, r31, 0xf0
	ctx.r[3].s64 = ctx.r[31].s64 + 240;
	// 82513870: 13E020C7  vcmpequd (lvx128) v31, v0, v4
	tmp.u32 = ctx.r[4].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82513874: 39600010  li r11, 0x10
	ctx.r[11].s64 = 16;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825138A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x825138A0 size=112
    let mut pc: u32 = 0x825138A0;
    'dispatch: loop {
        match pc {
            0x825138A0 => {
    //   block [0x825138A0..0x82513910)
	// 825138A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825138A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825138A8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825138AC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825138B0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825138B4: 897F00D4  lbz r11, 0xd4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(212 as u32) ) } as u64;
	// 825138B8: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 825138BC: 41820010  beq 0x825138cc
	if ctx.cr[0].eq {
	pc = 0x825138CC; continue 'dispatch;
	}
	// 825138C0: 807F00C0  lwz r3, 0xc0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(192 as u32) ) } as u64;
	// 825138C4: 4BDD9ECD  bl 0x822ed790
	ctx.lr = 0x825138C8;
	sub_822ED790(ctx, base);
	// 825138C8: 48000034  b 0x825138fc
	pc = 0x825138FC; continue 'dispatch;
	// 825138CC: C0040000  lfs f0, 0(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825138D0: 387F00F0  addi r3, r31, 0xf0
	ctx.r[3].s64 = ctx.r[31].s64 + 240;
	// 825138D4: D01F00F0  stfs f0, 0xf0(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(240 as u32), tmp.u32 ) };
	// 825138D8: C0040004  lfs f0, 4(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825138DC: D01F00F4  stfs f0, 0xf4(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(244 as u32), tmp.u32 ) };
	// 825138E0: C0040008  lfs f0, 8(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825138E4: D01F00F8  stfs f0, 0xf8(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(248 as u32), tmp.u32 ) };
	// 825138E8: C004000C  lfs f0, 0xc(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825138EC: D01F00FC  stfs f0, 0xfc(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(252 as u32), tmp.u32 ) };
	// 825138F0: 4BDC0A81  bl 0x822d4370
	ctx.lr = 0x825138F4;
	sub_822D4370(ctx, base);
	// 825138F4: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 825138F8: 997F0150  stb r11, 0x150(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(336 as u32), ctx.r[11].u8 ) };
	// 825138FC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82513900: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82513904: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82513908: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8251390C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82513910(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82513910 size=316
    let mut pc: u32 = 0x82513910;
    'dispatch: loop {
        match pc {
            0x82513910 => {
    //   block [0x82513910..0x82513A4C)
	// 82513910: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82513914: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82513918: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8251391C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82513920: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82513924: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82513928: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8251392C: 897F00D4  lbz r11, 0xd4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(212 as u32) ) } as u64;
	// 82513930: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82513934: 418200FC  beq 0x82513a30
	if ctx.cr[0].eq {
	pc = 0x82513A30; continue 'dispatch;
	}
	// 82513938: 2B050001  cmplwi cr6, r5, 1
	ctx.cr[6].compare_u32(ctx.r[5].u32, 1 as u32, &mut ctx.xer);
	// 8251393C: 41980070  blt cr6, 0x825139ac
	if ctx.cr[6].lt {
	pc = 0x825139AC; continue 'dispatch;
	}
	// 82513940: 409A00E8  bne cr6, 0x82513a28
	if !ctx.cr[6].eq {
	pc = 0x82513A28; continue 'dispatch;
	}
	// 82513944: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82513948: 809F00C0  lwz r4, 0xc0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(192 as u32) ) } as u64;
	// 8251394C: 4BDD9625  bl 0x822ecf70
	ctx.lr = 0x82513950;
	sub_822ECF70(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82513A50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82513A50 size=24
    let mut pc: u32 = 0x82513A50;
    'dispatch: loop {
        match pc {
            0x82513A50 => {
    //   block [0x82513A50..0x82513A68)
	// 82513A50: 816400C8  lwz r11, 0xc8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(200 as u32) ) } as u64;
	// 82513A54: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82513A58: 816400CC  lwz r11, 0xcc(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(204 as u32) ) } as u64;
	// 82513A5C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82513A60: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82513A64: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82513A68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82513A68 size=36
    let mut pc: u32 = 0x82513A68;
    'dispatch: loop {
        match pc {
            0x82513A68 => {
    //   block [0x82513A68..0x82513A8C)
	// 82513A68: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82513A6C: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82513A70: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82513A74: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82513A78: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82513A7C: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82513A80: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82513A84: 4082FFE8  bne 0x82513a6c
	if !ctx.cr[0].eq {
	pc = 0x82513A6C; continue 'dispatch;
	}
	// 82513A88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82513A90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82513A90 size=76
    let mut pc: u32 = 0x82513A90;
    'dispatch: loop {
        match pc {
            0x82513A90 => {
    //   block [0x82513A90..0x82513ADC)
	// 82513A90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82513A94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82513A98: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82513A9C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82513AA0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82513AA4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82513AA8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82513AAC: 4BFFFC1D  bl 0x825136c8
	ctx.lr = 0x82513AB0;
	sub_825136C8(ctx, base);
	// 82513AB0: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82513AB4: 4182000C  beq 0x82513ac0
	if ctx.cr[0].eq {
	pc = 0x82513AC0; continue 'dispatch;
	}
	// 82513AB8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82513ABC: 488DE91D  bl 0x82df23d8
	ctx.lr = 0x82513AC0;
	sub_82DF23D8(ctx, base);
	// 82513AC0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82513AC4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82513AC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82513ACC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82513AD0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82513AD4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82513AD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82513AE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82513AE0 size=148
    let mut pc: u32 = 0x82513AE0;
    'dispatch: loop {
        match pc {
            0x82513AE0 => {
    //   block [0x82513AE0..0x82513B74)
	// 82513AE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82513AE4: 48C9467D  bl 0x831a8160
	ctx.lr = 0x82513AE8;
	sub_831A8130(ctx, base);
	// 82513AE8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82513AEC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82513AF0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82513AF4: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82513AF8: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82513AFC: 7CFB3B78  mr r27, r7
	ctx.r[27].u64 = ctx.r[7].u64;
	// 82513B00: 7D1A4378  mr r26, r8
	ctx.r[26].u64 = ctx.r[8].u64;
	// 82513B04: 4BFFD5ED  bl 0x825110f0
	ctx.lr = 0x82513B08;
	sub_825110F0(ctx, base);
	// 82513B08: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82513B0C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82513B10: 392B1CD0  addi r9, r11, 0x1cd0
	ctx.r[9].s64 = ctx.r[11].s64 + 7376;
	// 82513B14: 394A1CBC  addi r10, r10, 0x1cbc
	ctx.r[10].s64 = ctx.r[10].s64 + 7356;
	// 82513B18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82513B1C: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82513B20: 915F0028  stw r10, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[10].u32 ) };
	// 82513B24: 387F00D8  addi r3, r31, 0xd8
	ctx.r[3].s64 = ctx.r[31].s64 + 216;
	// 82513B28: 917F00C0  stw r11, 0xc0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(192 as u32), ctx.r[11].u32 ) };
	// 82513B2C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82513B30: 917F00C4  stw r11, 0xc4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(196 as u32), ctx.r[11].u32 ) };
	// 82513B34: 917F00C8  stw r11, 0xc8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(200 as u32), ctx.r[11].u32 ) };
	// 82513B38: 917F00CC  stw r11, 0xcc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(204 as u32), ctx.r[11].u32 ) };
	// 82513B3C: 93DF00D0  stw r30, 0xd0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(208 as u32), ctx.r[30].u32 ) };
	// 82513B40: 488E00C1  bl 0x82df3c00
	ctx.lr = 0x82513B44;
	sub_82DF3C00(ctx, base);
	// 82513B44: 387F00DC  addi r3, r31, 0xdc
	ctx.r[3].s64 = ctx.r[31].s64 + 220;
	// 82513B48: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82513B4C: 488E00B5  bl 0x82df3c00
	ctx.lr = 0x82513B50;
	sub_82DF3C00(ctx, base);
	// 82513B50: 387F00E0  addi r3, r31, 0xe0
	ctx.r[3].s64 = ctx.r[31].s64 + 224;
	// 82513B54: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82513B58: 488E00A9  bl 0x82df3c00
	ctx.lr = 0x82513B5C;
	sub_82DF3C00(ctx, base);
	// 82513B5C: 9B5F0151  stb r26, 0x151(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(337 as u32), ctx.r[26].u8 ) };
	// 82513B60: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82513B64: 4BFFFC65  bl 0x825137c8
	ctx.lr = 0x82513B68;
	sub_825137C8(ctx, base);
	// 82513B68: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82513B6C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82513B70: 48C94640  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82513B78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82513B78 size=8
    let mut pc: u32 = 0x82513B78;
    'dispatch: loop {
        match pc {
            0x82513B78 => {
    //   block [0x82513B78..0x82513B80)
	// 82513B78: 38840020  addi r4, r4, 0x20
	ctx.r[4].s64 = ctx.r[4].s64 + 32;
	// 82513B7C: 4BFFFCC4  b 0x82513840
	sub_82513840(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82513B80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82513B80 size=8
    let mut pc: u32 = 0x82513B80;
    'dispatch: loop {
        match pc {
            0x82513B80 => {
    //   block [0x82513B80..0x82513B88)
	// 82513B80: 38840020  addi r4, r4, 0x20
	ctx.r[4].s64 = ctx.r[4].s64 + 32;
	// 82513B84: 4BFFFD1C  b 0x825138a0
	sub_825138A0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82513B88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82513B88 size=16
    let mut pc: u32 = 0x82513B88;
    'dispatch: loop {
        match pc {
            0x82513B88 => {
    //   block [0x82513B88..0x82513B98)
	// 82513B88: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82513B8C: 388B0020  addi r4, r11, 0x20
	ctx.r[4].s64 = ctx.r[11].s64 + 32;
	// 82513B90: 80AB0030  lwz r5, 0x30(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 82513B94: 4BFFFD7C  b 0x82513910
	sub_82513910(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82513B98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82513B98 size=152
    let mut pc: u32 = 0x82513B98;
    'dispatch: loop {
        match pc {
            0x82513B98 => {
    //   block [0x82513B98..0x82513C30)
	// 82513B98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82513B9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82513BA0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82513BA4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82513BA8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82513BAC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82513BB0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82513BB4: 806B00C4  lwz r3, 0xc4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(196 as u32) ) } as u64;
	// 82513BB8: 816B00C0  lwz r11, 0xc0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(192 as u32) ) } as u64;
	// 82513BBC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82513BC0: 419A0024  beq cr6, 0x82513be4
	if ctx.cr[6].eq {
	pc = 0x82513BE4; continue 'dispatch;
	}
	// 82513BC4: 39430004  addi r10, r3, 4
	ctx.r[10].s64 = ctx.r[3].s64 + 4;
	// 82513BC8: 7D0000A6  mfmsr r8
	ctx.r[8].u64 = ctx.msr;
	// 82513BCC: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82513BD0: 7D205028  lwarx r9, 0, r10
	// lwarx
	let ea = ctx.r[10].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[9].u64 = ctx.reserved.u32 as u64;
	// 82513BD4: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82513BD8: 7D20512D  stwcx. r9, 0, r10
	// stwcx.
	let addr = ctx.r[10].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[9].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82513BDC: 7D010164  mtmsrd r8, 1
	ctx.msr = (ctx.r[8].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82513BE0: 4082FFE8  bne 0x82513bc8
	if !ctx.cr[0].eq {
	pc = 0x82513BC8; continue 'dispatch;
	}
	// 82513BE4: 815F0018  lwz r10, 0x18(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82513BE8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82513BEC: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82513BF0: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82513BF4: 557EDFFE  rlwinm r30, r11, 0x1b, 0x1f, 0x1f
	ctx.r[30].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82513BF8: 419A0008  beq cr6, 0x82513c00
	if ctx.cr[6].eq {
	pc = 0x82513C00; continue 'dispatch;
	}
	// 82513BFC: 4BDACC95  bl 0x822c0890
	ctx.lr = 0x82513C00;
	sub_822C0890(ctx, base);
	// 82513C00: 57CB063F  clrlwi. r11, r30, 0x18
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82513C04: 41820014  beq 0x82513c18
	if ctx.cr[0].eq {
	pc = 0x82513C18; continue 'dispatch;
	}
	// 82513C08: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82513C0C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82513C10: 997F0020  stb r11, 0x20(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u8 ) };
	// 82513C14: 915F001C  stw r10, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[10].u32 ) };
	// 82513C18: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82513C1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82513C20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82513C24: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82513C28: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82513C2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82513C30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82513C30 size=128
    let mut pc: u32 = 0x82513C30;
    'dispatch: loop {
        match pc {
            0x82513C30 => {
    //   block [0x82513C30..0x82513CB0)
	// 82513C30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82513C34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82513C38: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82513C3C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82513C40: 83E300C4  lwz r31, 0xc4(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(196 as u32) ) } as u64;
	// 82513C44: 816300C0  lwz r11, 0xc0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(192 as u32) ) } as u64;
	// 82513C48: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82513C4C: 93E10054  stw r31, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[31].u32 ) };
	// 82513C50: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82513C54: 419A0024  beq cr6, 0x82513c78
	if ctx.cr[6].eq {
	pc = 0x82513C78; continue 'dispatch;
	}
	// 82513C58: 395F0004  addi r10, r31, 4
	ctx.r[10].s64 = ctx.r[31].s64 + 4;
	// 82513C5C: 7D0000A6  mfmsr r8
	ctx.r[8].u64 = ctx.msr;
	// 82513C60: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82513C64: 7D205028  lwarx r9, 0, r10
	// lwarx
	let ea = ctx.r[10].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[9].u64 = ctx.reserved.u32 as u64;
	// 82513C68: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82513C6C: 7D20512D  stwcx. r9, 0, r10
	// stwcx.
	let addr = ctx.r[10].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[9].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82513C70: 7D010164  mtmsrd r8, 1
	ctx.msr = (ctx.r[8].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82513C74: 4082FFE8  bne 0x82513c5c
	if !ctx.cr[0].eq {
	pc = 0x82513C5C; continue 'dispatch;
	}
	// 82513C78: 9164001C  stw r11, 0x1c(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82513C7C: 3964001C  addi r11, r4, 0x1c
	ctx.r[11].s64 = ctx.r[4].s64 + 28;
	// 82513C80: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 82513C84: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 82513C88: 4BDB07D9  bl 0x822c4460
	ctx.lr = 0x82513C8C;
	sub_822C4460(ctx, base);
	// 82513C8C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82513C90: 419A000C  beq cr6, 0x82513c9c
	if ctx.cr[6].eq {
	pc = 0x82513C9C; continue 'dispatch;
	}
	// 82513C94: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82513C98: 4BDACBF9  bl 0x822c0890
	ctx.lr = 0x82513C9C;
	sub_822C0890(ctx, base);
	// 82513C9C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82513CA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82513CA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82513CA8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82513CAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82513CB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82513CB0 size=820
    let mut pc: u32 = 0x82513CB0;
    'dispatch: loop {
        match pc {
            0x82513CB0 => {
    //   block [0x82513CB0..0x82513FE4)
	// 82513CB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82513CB4: 48C944B1  bl 0x831a8164
	ctx.lr = 0x82513CB8;
	sub_831A8130(ctx, base);
	// 82513CB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82513CBC: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82513CC0: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82513CC4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82513CC8: 577E063F  clrlwi. r30, r27, 0x18
	ctx.r[30].u64 = ctx.r[27].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82513CCC: 41820038  beq 0x82513d04
	if ctx.cr[0].eq {
	pc = 0x82513D04; continue 'dispatch;
	}
	// 82513CD0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82513CD4: 48C95CB5  bl 0x831a9988
	ctx.lr = 0x82513CD8;
	sub_831A9988(ctx, base);
	// 82513CD8: 3D608325  lis r11, -0x7cdb
	ctx.r[11].s64 = -2094727168;
	// 82513CDC: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82513CE0: 386B5BA8  addi r3, r11, 0x5ba8
	ctx.r[3].s64 = ctx.r[11].s64 + 23464;
	// 82513CE4: 48C94415  bl 0x831a80f8
	ctx.lr = 0x82513CE8;
	sub_831A80F8(ctx, base);
	// 82513CE8: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82513CEC: 41820018  beq 0x82513d04
	if ctx.cr[0].eq {
	pc = 0x82513D04; continue 'dispatch;
	}
	// 82513CF0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82513CF4: 387CFFD8  addi r3, r28, -0x28
	ctx.r[3].s64 = ctx.r[28].s64 + -40;
	// 82513CF8: 4BFFFE81  bl 0x82513b78
	ctx.lr = 0x82513CFC;
	sub_82513B78(ctx, base);
	// 82513CFC: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82513D00: 480002DC  b 0x82513fdc
	pc = 0x82513FDC; continue 'dispatch;
	// 82513D04: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82513D08: 419A006C  beq cr6, 0x82513d74
	if ctx.cr[6].eq {
	pc = 0x82513D74; continue 'dispatch;
	}
	// 82513D0C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82513D10: 48C95C79  bl 0x831a9988
	ctx.lr = 0x82513D14;
	sub_831A9988(ctx, base);
	// 82513D14: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 82513D18: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82513D1C: 386B2E70  addi r3, r11, 0x2e70
	ctx.r[3].s64 = ctx.r[11].s64 + 11888;
	// 82513D20: 48C943D9  bl 0x831a80f8
	ctx.lr = 0x82513D24;
	sub_831A80F8(ctx, base);
	// 82513D24: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82513D28: 41820014  beq 0x82513d3c
	if ctx.cr[0].eq {
	pc = 0x82513D3C; continue 'dispatch;
	}
	// 82513D2C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82513D30: 387CFFD8  addi r3, r28, -0x28
	ctx.r[3].s64 = ctx.r[28].s64 + -40;
	// 82513D34: 4BFFFE4D  bl 0x82513b80
	ctx.lr = 0x82513D38;
	sub_82513B80(ctx, base);
	// 82513D38: 4BFFFFC4  b 0x82513cfc
	pc = 0x82513CFC; continue 'dispatch;
	// 82513D3C: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82513D40: 419A0034  beq cr6, 0x82513d74
	if ctx.cr[6].eq {
	pc = 0x82513D74; continue 'dispatch;
	}
	// 82513D44: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82513D48: 48C95C41  bl 0x831a9988
	ctx.lr = 0x82513D4C;
	sub_831A9988(ctx, base);
	// 82513D4C: 3D608328  lis r11, -0x7cd8
	ctx.r[11].s64 = -2094530560;
	// 82513D50: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82513D54: 386BCB78  addi r3, r11, -0x3488
	ctx.r[3].s64 = ctx.r[11].s64 + -13448;
	// 82513D58: 48C943A1  bl 0x831a80f8
	ctx.lr = 0x82513D5C;
	sub_831A80F8(ctx, base);
	// 82513D5C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82513D60: 41820014  beq 0x82513d74
	if ctx.cr[0].eq {
	pc = 0x82513D74; continue 'dispatch;
	}
	// 82513D64: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82513D68: 387CFFD8  addi r3, r28, -0x28
	ctx.r[3].s64 = ctx.r[28].s64 + -40;
	// 82513D6C: 4BFFFE1D  bl 0x82513b88
	ctx.lr = 0x82513D70;
	sub_82513B88(ctx, base);
	// 82513D70: 4800026C  b 0x82513fdc
	pc = 0x82513FDC; continue 'dispatch;
	// 82513D74: 3D608328  lis r11, -0x7cd8
	ctx.r[11].s64 = -2094530560;
	// 82513D78: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82513D7C: 3BABCB50  addi r29, r11, -0x34b0
	ctx.r[29].s64 = ctx.r[11].s64 + -13488;
	// 82513D80: 419A024C  beq cr6, 0x82513fcc
	if ctx.cr[6].eq {
	pc = 0x82513FCC; continue 'dispatch;
	}
	// 82513D84: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82513D88: 48C95C01  bl 0x831a9988
	ctx.lr = 0x82513D8C;
	sub_831A9988(ctx, base);
	// 82513D8C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82513D90: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82513D94: 48C94365  bl 0x831a80f8
	ctx.lr = 0x82513D98;
	sub_831A80F8(ctx, base);
	// 82513D98: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82513D9C: 41820014  beq 0x82513db0
	if ctx.cr[0].eq {
	pc = 0x82513DB0; continue 'dispatch;
	}
	// 82513DA0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82513DA4: 387CFFD8  addi r3, r28, -0x28
	ctx.r[3].s64 = ctx.r[28].s64 + -40;
	// 82513DA8: 4BFFFA01  bl 0x825137a8
	ctx.lr = 0x82513DAC;
	sub_825137A8(ctx, base);
	// 82513DAC: 4BFFFF50  b 0x82513cfc
	pc = 0x82513CFC; continue 'dispatch;
	// 82513DB0: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82513DB4: 419A0218  beq cr6, 0x82513fcc
	if ctx.cr[6].eq {
	pc = 0x82513FCC; continue 'dispatch;
	}
	// 82513DB8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82513DBC: 48C95BCD  bl 0x831a9988
	ctx.lr = 0x82513DC0;
	sub_831A9988(ctx, base);
	// 82513DC0: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82513DC4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82513DC8: 48C94331  bl 0x831a80f8
	ctx.lr = 0x82513DCC;
	sub_831A80F8(ctx, base);
	// 82513DCC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82513DD0: 4082FFD0  bne 0x82513da0
	if !ctx.cr[0].eq {
	pc = 0x82513DA0; continue 'dispatch;
	}
	// 82513DD4: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82513DD8: 419A01F4  beq cr6, 0x82513fcc
	if ctx.cr[6].eq {
	pc = 0x82513FCC; continue 'dispatch;
	}
	// 82513DDC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82513DE0: 48C95BA9  bl 0x831a9988
	ctx.lr = 0x82513DE4;
	sub_831A9988(ctx, base);
	// 82513DE4: 3D608328  lis r11, -0x7cd8
	ctx.r[11].s64 = -2094530560;
	// 82513DE8: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82513DEC: 386BCCC4  addi r3, r11, -0x333c
	ctx.r[3].s64 = ctx.r[11].s64 + -13116;
	// 82513DF0: 48C94309  bl 0x831a80f8
	ctx.lr = 0x82513DF4;
	sub_831A80F8(ctx, base);
	// 82513DF4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82513DF8: 41820014  beq 0x82513e0c
	if ctx.cr[0].eq {
	pc = 0x82513E0C; continue 'dispatch;
	}
	// 82513DFC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82513E00: 387CFFD8  addi r3, r28, -0x28
	ctx.r[3].s64 = ctx.r[28].s64 + -40;
	// 82513E04: 4BFFF615  bl 0x82513418
	ctx.lr = 0x82513E08;
	sub_82513418(ctx, base);
	// 82513E08: 480001D4  b 0x82513fdc
	pc = 0x82513FDC; continue 'dispatch;
	// 82513E0C: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82513E10: 419A01BC  beq cr6, 0x82513fcc
	if ctx.cr[6].eq {
	pc = 0x82513FCC; continue 'dispatch;
	}
	// 82513E14: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82513E18: 48C95B71  bl 0x831a9988
	ctx.lr = 0x82513E1C;
	sub_831A9988(ctx, base);
	// 82513E1C: 3D608328  lis r11, -0x7cd8
	ctx.r[11].s64 = -2094530560;
	// 82513E20: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82513E24: 386BCC94  addi r3, r11, -0x336c
	ctx.r[3].s64 = ctx.r[11].s64 + -13164;
	// 82513E28: 48C942D1  bl 0x831a80f8
	ctx.lr = 0x82513E2C;
	sub_831A80F8(ctx, base);
	// 82513E2C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82513E30: 41820014  beq 0x82513e44
	if ctx.cr[0].eq {
	pc = 0x82513E44; continue 'dispatch;
	}
	// 82513E34: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82513E38: 387CFFD8  addi r3, r28, -0x28
	ctx.r[3].s64 = ctx.r[28].s64 + -40;
	// 82513E3C: 4BFFF61D  bl 0x82513458
	ctx.lr = 0x82513E40;
	sub_82513458(ctx, base);
	// 82513E40: 4800019C  b 0x82513fdc
	pc = 0x82513FDC; continue 'dispatch;
	// 82513E44: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82513E48: 419A0184  beq cr6, 0x82513fcc
	if ctx.cr[6].eq {
	pc = 0x82513FCC; continue 'dispatch;
	}
	// 82513E4C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82513E50: 48C95B39  bl 0x831a9988
	ctx.lr = 0x82513E54;
	sub_831A9988(ctx, base);
	// 82513E54: 3D608325  lis r11, -0x7cdb
	ctx.r[11].s64 = -2094727168;
	// 82513E58: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82513E5C: 386B5C00  addi r3, r11, 0x5c00
	ctx.r[3].s64 = ctx.r[11].s64 + 23552;
	// 82513E60: 48C94299  bl 0x831a80f8
	ctx.lr = 0x82513E64;
	sub_831A80F8(ctx, base);
	// 82513E64: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82513E68: 41820014  beq 0x82513e7c
	if ctx.cr[0].eq {
	pc = 0x82513E7C; continue 'dispatch;
	}
	// 82513E6C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82513E70: 387CFFD8  addi r3, r28, -0x28
	ctx.r[3].s64 = ctx.r[28].s64 + -40;
	// 82513E74: 4BFFF8DD  bl 0x82513750
	ctx.lr = 0x82513E78;
	sub_82513750(ctx, base);
	// 82513E78: 48000164  b 0x82513fdc
	pc = 0x82513FDC; continue 'dispatch;
	// 82513E7C: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82513E80: 419A014C  beq cr6, 0x82513fcc
	if ctx.cr[6].eq {
	pc = 0x82513FCC; continue 'dispatch;
	}
	// 82513E84: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82513E88: 48C95B01  bl 0x831a9988
	ctx.lr = 0x82513E8C;
	sub_831A9988(ctx, base);
	// 82513E8C: 3D608325  lis r11, -0x7cdb
	ctx.r[11].s64 = -2094727168;
	// 82513E90: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82513E94: 386B5BD4  addi r3, r11, 0x5bd4
	ctx.r[3].s64 = ctx.r[11].s64 + 23508;
	// 82513E98: 48C94261  bl 0x831a80f8
	ctx.lr = 0x82513E9C;
	sub_831A80F8(ctx, base);
	// 82513E9C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82513EA0: 41820014  beq 0x82513eb4
	if ctx.cr[0].eq {
	pc = 0x82513EB4; continue 'dispatch;
	}
	// 82513EA4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82513EA8: 387CFFD8  addi r3, r28, -0x28
	ctx.r[3].s64 = ctx.r[28].s64 + -40;
	// 82513EAC: 4BFFF8AD  bl 0x82513758
	ctx.lr = 0x82513EB0;
	sub_82513758(ctx, base);
	// 82513EB0: 4800012C  b 0x82513fdc
	pc = 0x82513FDC; continue 'dispatch;
	// 82513EB4: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82513EB8: 419A0114  beq cr6, 0x82513fcc
	if ctx.cr[6].eq {
	pc = 0x82513FCC; continue 'dispatch;
	}
	// 82513EBC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82513EC0: 48C95AC9  bl 0x831a9988
	ctx.lr = 0x82513EC4;
	sub_831A9988(ctx, base);
	// 82513EC4: 3D608325  lis r11, -0x7cdb
	ctx.r[11].s64 = -2094727168;
	// 82513EC8: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82513ECC: 386B7818  addi r3, r11, 0x7818
	ctx.r[3].s64 = ctx.r[11].s64 + 30744;
	// 82513ED0: 48C94229  bl 0x831a80f8
	ctx.lr = 0x82513ED4;
	sub_831A80F8(ctx, base);
	// 82513ED4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82513ED8: 41820014  beq 0x82513eec
	if ctx.cr[0].eq {
	pc = 0x82513EEC; continue 'dispatch;
	}
	// 82513EDC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82513EE0: 387CFFD8  addi r3, r28, -0x28
	ctx.r[3].s64 = ctx.r[28].s64 + -40;
	// 82513EE4: 4BFFF87D  bl 0x82513760
	ctx.lr = 0x82513EE8;
	sub_82513760(ctx, base);
	// 82513EE8: 480000F4  b 0x82513fdc
	pc = 0x82513FDC; continue 'dispatch;
	// 82513EEC: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82513EF0: 419A00DC  beq cr6, 0x82513fcc
	if ctx.cr[6].eq {
	pc = 0x82513FCC; continue 'dispatch;
	}
	// 82513EF4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82513EF8: 48C95A91  bl 0x831a9988
	ctx.lr = 0x82513EFC;
	sub_831A9988(ctx, base);
	// 82513EFC: 3D608327  lis r11, -0x7cd9
	ctx.r[11].s64 = -2094596096;
	// 82513F00: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82513F04: 386B3324  addi r3, r11, 0x3324
	ctx.r[3].s64 = ctx.r[11].s64 + 13092;
	// 82513F08: 48C941F1  bl 0x831a80f8
	ctx.lr = 0x82513F0C;
	sub_831A80F8(ctx, base);
	// 82513F0C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82513F10: 41820014  beq 0x82513f24
	if ctx.cr[0].eq {
	pc = 0x82513F24; continue 'dispatch;
	}
	// 82513F14: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82513F18: 387CFFD8  addi r3, r28, -0x28
	ctx.r[3].s64 = ctx.r[28].s64 + -40;
	// 82513F1C: 4BFFF84D  bl 0x82513768
	ctx.lr = 0x82513F20;
	sub_82513768(ctx, base);
	// 82513F20: 480000BC  b 0x82513fdc
	pc = 0x82513FDC; continue 'dispatch;
	// 82513F24: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82513F28: 419A00A4  beq cr6, 0x82513fcc
	if ctx.cr[6].eq {
	pc = 0x82513FCC; continue 'dispatch;
	}
	// 82513F2C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82513F30: 48C95A59  bl 0x831a9988
	ctx.lr = 0x82513F34;
	sub_831A9988(ctx, base);
	// 82513F34: 3D608328  lis r11, -0x7cd8
	ctx.r[11].s64 = -2094530560;
	// 82513F38: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82513F3C: 386BCC70  addi r3, r11, -0x3390
	ctx.r[3].s64 = ctx.r[11].s64 + -13200;
	// 82513F40: 48C941B9  bl 0x831a80f8
	ctx.lr = 0x82513F44;
	sub_831A80F8(ctx, base);
	// 82513F44: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82513F48: 41820014  beq 0x82513f5c
	if ctx.cr[0].eq {
	pc = 0x82513F5C; continue 'dispatch;
	}
	// 82513F4C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82513F50: 387CFFD8  addi r3, r28, -0x28
	ctx.r[3].s64 = ctx.r[28].s64 + -40;
	// 82513F54: 4BFFF56D  bl 0x825134c0
	ctx.lr = 0x82513F58;
	sub_825134C0(ctx, base);
	// 82513F58: 48000084  b 0x82513fdc
	pc = 0x82513FDC; continue 'dispatch;
	// 82513F5C: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82513F60: 419A006C  beq cr6, 0x82513fcc
	if ctx.cr[6].eq {
	pc = 0x82513FCC; continue 'dispatch;
	}
	// 82513F64: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82513F68: 48C95A21  bl 0x831a9988
	ctx.lr = 0x82513F6C;
	sub_831A9988(ctx, base);
	// 82513F6C: 3D608328  lis r11, -0x7cd8
	ctx.r[11].s64 = -2094530560;
	// 82513F70: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82513F74: 386BCC44  addi r3, r11, -0x33bc
	ctx.r[3].s64 = ctx.r[11].s64 + -13244;
	// 82513F78: 48C94181  bl 0x831a80f8
	ctx.lr = 0x82513F7C;
	sub_831A80F8(ctx, base);
	// 82513F7C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82513F80: 41820014  beq 0x82513f94
	if ctx.cr[0].eq {
	pc = 0x82513F94; continue 'dispatch;
	}
	// 82513F84: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82513F88: 387CFFD8  addi r3, r28, -0x28
	ctx.r[3].s64 = ctx.r[28].s64 + -40;
	// 82513F8C: 4BFFFC0D  bl 0x82513b98
	ctx.lr = 0x82513F90;
	sub_82513B98(ctx, base);
	// 82513F90: 4BFFFD6C  b 0x82513cfc
	pc = 0x82513CFC; continue 'dispatch;
	// 82513F94: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82513F98: 419A0034  beq cr6, 0x82513fcc
	if ctx.cr[6].eq {
	pc = 0x82513FCC; continue 'dispatch;
	}
	// 82513F9C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82513FA0: 48C959E9  bl 0x831a9988
	ctx.lr = 0x82513FA4;
	sub_831A9988(ctx, base);
	// 82513FA4: 3D608328  lis r11, -0x7cd8
	ctx.r[11].s64 = -2094530560;
	// 82513FA8: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82513FAC: 386BCC18  addi r3, r11, -0x33e8
	ctx.r[3].s64 = ctx.r[11].s64 + -13288;
	// 82513FB0: 48C94149  bl 0x831a80f8
	ctx.lr = 0x82513FB4;
	sub_831A80F8(ctx, base);
	// 82513FB4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82513FB8: 41820014  beq 0x82513fcc
	if ctx.cr[0].eq {
	pc = 0x82513FCC; continue 'dispatch;
	}
	// 82513FBC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82513FC0: 387CFFD8  addi r3, r28, -0x28
	ctx.r[3].s64 = ctx.r[28].s64 + -40;
	// 82513FC4: 4BFFFC6D  bl 0x82513c30
	ctx.lr = 0x82513FC8;
	sub_82513C30(ctx, base);
	// 82513FC8: 4BFFFD34  b 0x82513cfc
	pc = 0x82513CFC; continue 'dispatch;
	// 82513FCC: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82513FD0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82513FD4: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82513FD8: 4BFFD5C1  bl 0x82511598
	ctx.lr = 0x82513FDC;
	sub_82511598(ctx, base);
	// 82513FDC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82513FE0: 48C941D4  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82513FE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82513FE8 size=228
    let mut pc: u32 = 0x82513FE8;
    'dispatch: loop {
        match pc {
            0x82513FE8 => {
    //   block [0x82513FE8..0x825140CC)
	// 82513FE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82513FEC: 48C94181  bl 0x831a816c
	ctx.lr = 0x82513FF0;
	sub_831A8130(ctx, base);
	// 82513FF0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82513FF4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82513FF8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82513FFC: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82514000: 897E00D4  lbz r11, 0xd4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(212 as u32) ) } as u64;
	// 82514004: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82514008: 408200BC  bne 0x825140c4
	if !ctx.cr[0].eq {
	pc = 0x825140C4; continue 'dispatch;
	}
	// 8251400C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82514010: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82514014: 388B1CF8  addi r4, r11, 0x1cf8
	ctx.r[4].s64 = ctx.r[11].s64 + 7416;
	// 82514018: 38A00062  li r5, 0x62
	ctx.r[5].s64 = 98;
	// 8251401C: 38600098  li r3, 0x98
	ctx.r[3].s64 = 152;
	// 82514020: 488DE3C9  bl 0x82df23e8
	ctx.lr = 0x82514024;
	sub_82DF23E8(ctx, base);
	// 82514024: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82514028: 41820014  beq 0x8251403c
	if ctx.cr[0].eq {
	pc = 0x8251403C; continue 'dispatch;
	}
	// 8251402C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82514030: 48902861  bl 0x82e16890
	ctx.lr = 0x82514034;
	sub_82E16890(ctx, base);
	// 82514034: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82514038: 48000008  b 0x82514040
	pc = 0x82514040; continue 'dispatch;
	// 8251403C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82514040: 387E00C8  addi r3, r30, 0xc8
	ctx.r[3].s64 = ctx.r[30].s64 + 200;
	// 82514044: 4BE4932D  bl 0x8235d370
	ctx.lr = 0x82514048;
	sub_8235D370(ctx, base);
	// 82514048: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251404C: 3BFE00C0  addi r31, r30, 0xc0
	ctx.r[31].s64 = ctx.r[30].s64 + 192;
	// 82514050: 389D0004  addi r4, r29, 4
	ctx.r[4].s64 = ctx.r[29].s64 + 4;
	// 82514054: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 82514058: 917E00C0  stw r11, 0xc0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(192 as u32), ctx.r[11].u32 ) };
	// 8251405C: 4BDB0405  bl 0x822c4460
	ctx.lr = 0x82514060;
	sub_822C4460(ctx, base);
	// 82514060: 809E00E4  lwz r4, 0xe4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(228 as u32) ) } as u64;
	// 82514064: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82514068: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 8251406C: 409A0028  bne cr6, 0x82514094
	if !ctx.cr[6].eq {
	pc = 0x82514094; continue 'dispatch;
	}
	// 82514070: 4BDD9879  bl 0x822ed8e8
	ctx.lr = 0x82514074;
	sub_822ED8E8(ctx, base);
	// 82514074: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82514078: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251407C: 41820010  beq 0x8251408c
	if ctx.cr[0].eq {
	pc = 0x8251408C; continue 'dispatch;
	}
	// 82514080: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 82514084: 808B674C  lwz r4, 0x674c(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(26444 as u32) ) } as u64;
	// 82514088: 4800000C  b 0x82514094
	pc = 0x82514094; continue 'dispatch;
	// 8251408C: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 82514090: 808B6744  lwz r4, 0x6744(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(26436 as u32) ) } as u64;
	// 82514094: 4BF7B30D  bl 0x8248f3a0
	ctx.lr = 0x82514098;
	sub_8248F3A0(ctx, base);
	// 82514098: 897E0150  lbz r11, 0x150(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(336 as u32) ) } as u64;
	// 8251409C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 825140A0: 4182001C  beq 0x825140bc
	if ctx.cr[0].eq {
	pc = 0x825140BC; continue 'dispatch;
	}
	// 825140A4: 38BE00F0  addi r5, r30, 0xf0
	ctx.r[5].s64 = ctx.r[30].s64 + 240;
	// 825140A8: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825140AC: 389E0100  addi r4, r30, 0x100
	ctx.r[4].s64 = ctx.r[30].s64 + 256;
	// 825140B0: 4BDD9789  bl 0x822ed838
	ctx.lr = 0x825140B4;
	sub_822ED838(ctx, base);
	// 825140B4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825140B8: 997E0150  stb r11, 0x150(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(336 as u32), ctx.r[11].u8 ) };
	// 825140BC: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 825140C0: 997E00D4  stb r11, 0xd4(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(212 as u32), ctx.r[11].u8 ) };
	// 825140C4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825140C8: 48C940F4  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825140D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825140D0 size=240
    let mut pc: u32 = 0x825140D0;
    'dispatch: loop {
        match pc {
            0x825140D0 => {
    //   block [0x825140D0..0x825141C0)
	// 825140D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825140D4: 48C94091  bl 0x831a8164
	ctx.lr = 0x825140D8;
	sub_831A8130(ctx, base);
	// 825140D8: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825140DC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825140E0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825140E4: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 825140E8: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 825140EC: 7CFB3B78  mr r27, r7
	ctx.r[27].u64 = ctx.r[7].u64;
	// 825140F0: 897F00D4  lbz r11, 0xd4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(212 as u32) ) } as u64;
	// 825140F4: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 825140F8: 408200C0  bne 0x825141b8
	if !ctx.cr[0].eq {
	pc = 0x825141B8; continue 'dispatch;
	}
	// 825140FC: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82514100: 809E0000  lwz r4, 0(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82514104: 489169E5  bl 0x82e2aae8
	ctx.lr = 0x82514108;
	sub_82E2AAE8(ctx, base);
	// 82514108: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8251410C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82514110: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 82514114: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 82514118: 4891AD59  bl 0x82e2ee70
	ctx.lr = 0x8251411C;
	sub_82E2EE70(ctx, base);
	// 8251411C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82514120: 809E0000  lwz r4, 0(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82514124: 4BDD30E5  bl 0x822e7208
	ctx.lr = 0x82514128;
	sub_822E7208(ctx, base);
	// 82514128: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8251412C: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82514130: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82514134: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82514138: 4BDD3259  bl 0x822e7390
	ctx.lr = 0x8251413C;
	sub_822E7390(ctx, base);
	// 8251413C: 38DF0028  addi r6, r31, 0x28
	ctx.r[6].s64 = ctx.r[31].s64 + 40;
	// 82514140: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82514144: 80810058  lwz r4, 0x58(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82514148: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8251414C: 4BDDA58D  bl 0x822ee6d8
	ctx.lr = 0x82514150;
	sub_822EE6D8(ctx, base);
	// 82514150: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82514154: 38810068  addi r4, r1, 0x68
	ctx.r[4].s64 = ctx.r[1].s64 + 104;
	// 82514158: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8251415C: 4BFFFE8D  bl 0x82513fe8
	ctx.lr = 0x82514160;
	sub_82513FE8(ctx, base);
	// 82514160: 387F00D8  addi r3, r31, 0xd8
	ctx.r[3].s64 = ctx.r[31].s64 + 216;
	// 82514164: 488DF2C5  bl 0x82df3428
	ctx.lr = 0x82514168;
	sub_82DF3428(ctx, base);
	// 82514168: 387F00DC  addi r3, r31, 0xdc
	ctx.r[3].s64 = ctx.r[31].s64 + 220;
	// 8251416C: 488DF2BD  bl 0x82df3428
	ctx.lr = 0x82514170;
	sub_82DF3428(ctx, base);
	// 82514170: 387F00E0  addi r3, r31, 0xe0
	ctx.r[3].s64 = ctx.r[31].s64 + 224;
	// 82514174: 488DF2B5  bl 0x82df3428
	ctx.lr = 0x82514178;
	sub_82DF3428(ctx, base);
	// 82514178: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8251417C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82514180: 419A0008  beq cr6, 0x82514188
	if ctx.cr[6].eq {
	pc = 0x82514188; continue 'dispatch;
	}
	// 82514184: 4BDAC70D  bl 0x822c0890
	ctx.lr = 0x82514188;
	sub_822C0890(ctx, base);
	// 82514188: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 8251418C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82514190: 419A0008  beq cr6, 0x82514198
	if ctx.cr[6].eq {
	pc = 0x82514198; continue 'dispatch;
	}
	// 82514194: 4BDAC6FD  bl 0x822c0890
	ctx.lr = 0x82514198;
	sub_822C0890(ctx, base);
	// 82514198: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8251419C: 4BDD3085  bl 0x822e7220
	ctx.lr = 0x825141A0;
	sub_822E7220(ctx, base);
	// 825141A0: 8061006C  lwz r3, 0x6c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(108 as u32) ) } as u64;
	// 825141A4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825141A8: 419A0008  beq cr6, 0x825141b0
	if ctx.cr[6].eq {
	pc = 0x825141B0; continue 'dispatch;
	}
	// 825141AC: 4BDAC6E5  bl 0x822c0890
	ctx.lr = 0x825141B0;
	sub_822C0890(ctx, base);
	// 825141B0: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 825141B4: 4891694D  bl 0x82e2ab00
	ctx.lr = 0x825141B8;
	sub_82E2AB00(ctx, base);
	// 825141B8: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 825141BC: 48C93FF8  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825141C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825141C0 size=172
    let mut pc: u32 = 0x825141C0;
    'dispatch: loop {
        match pc {
            0x825141C0 => {
    //   block [0x825141C0..0x8251426C)
	// 825141C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825141C4: 48C93F99  bl 0x831a815c
	ctx.lr = 0x825141C8;
	sub_831A8130(ctx, base);
	// 825141C8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825141CC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825141D0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825141D4: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 825141D8: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 825141DC: 7CFB3B78  mr r27, r7
	ctx.r[27].u64 = ctx.r[7].u64;
	// 825141E0: 7D1A4378  mr r26, r8
	ctx.r[26].u64 = ctx.r[8].u64;
	// 825141E4: 7D394B78  mr r25, r9
	ctx.r[25].u64 = ctx.r[9].u64;
	// 825141E8: 4BFFCF09  bl 0x825110f0
	ctx.lr = 0x825141EC;
	sub_825110F0(ctx, base);
	// 825141EC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 825141F0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825141F4: 392B1CD0  addi r9, r11, 0x1cd0
	ctx.r[9].s64 = ctx.r[11].s64 + 7376;
	// 825141F8: 394A1CBC  addi r10, r10, 0x1cbc
	ctx.r[10].s64 = ctx.r[10].s64 + 7356;
	// 825141FC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82514200: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82514204: 915F0028  stw r10, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[10].u32 ) };
	// 82514208: 387F00D0  addi r3, r31, 0xd0
	ctx.r[3].s64 = ctx.r[31].s64 + 208;
	// 8251420C: 917F00C0  stw r11, 0xc0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(192 as u32), ctx.r[11].u32 ) };
	// 82514210: 917F00C4  stw r11, 0xc4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(196 as u32), ctx.r[11].u32 ) };
	// 82514214: 917F00C8  stw r11, 0xc8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(200 as u32), ctx.r[11].u32 ) };
	// 82514218: 917F00CC  stw r11, 0xcc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(204 as u32), ctx.r[11].u32 ) };
	// 8251421C: 48C87665  bl 0x8319b880
	ctx.lr = 0x82514220;
	sub_8319B880(ctx, base);
	// 82514220: 387F00D8  addi r3, r31, 0xd8
	ctx.r[3].s64 = ctx.r[31].s64 + 216;
	// 82514224: 488DEECD  bl 0x82df30f0
	ctx.lr = 0x82514228;
	sub_82DF30F0(ctx, base);
	// 82514228: 387F00DC  addi r3, r31, 0xdc
	ctx.r[3].s64 = ctx.r[31].s64 + 220;
	// 8251422C: 488DEEC5  bl 0x82df30f0
	ctx.lr = 0x82514230;
	sub_82DF30F0(ctx, base);
	// 82514230: 387F00E0  addi r3, r31, 0xe0
	ctx.r[3].s64 = ctx.r[31].s64 + 224;
	// 82514234: 488DEEBD  bl 0x82df30f0
	ctx.lr = 0x82514238;
	sub_82DF30F0(ctx, base);
	// 82514238: 9B3F0151  stb r25, 0x151(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(337 as u32), ctx.r[25].u8 ) };
	// 8251423C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82514240: 4BFFF589  bl 0x825137c8
	ctx.lr = 0x82514244;
	sub_825137C8(ctx, base);
	// 82514244: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82514248: 7F47D378  mr r7, r26
	ctx.r[7].u64 = ctx.r[26].u64;
	// 8251424C: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 82514250: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82514254: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82514258: 4BFFFE79  bl 0x825140d0
	ctx.lr = 0x8251425C;
	sub_825140D0(ctx, base);
	// 8251425C: 93BF00D0  stw r29, 0xd0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(208 as u32), ctx.r[29].u32 ) };
	// 82514260: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82514264: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82514268: 48C93F44  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82514270(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82514270 size=324
    let mut pc: u32 = 0x82514270;
    'dispatch: loop {
        match pc {
            0x82514270 => {
    //   block [0x82514270..0x825143B4)
	// 82514270: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82514274: 48C93EF5  bl 0x831a8168
	ctx.lr = 0x82514278;
	sub_831A8130(ctx, base);
	// 82514278: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251427C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82514280: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82514284: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82514288: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 8251428C: 4BFFB0E5  bl 0x8250f370
	ctx.lr = 0x82514290;
	sub_8250F370(ctx, base);
	// 82514290: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82514294: 389F0028  addi r4, r31, 0x28
	ctx.r[4].s64 = ctx.r[31].s64 + 40;
	// 82514298: 409A0008  bne cr6, 0x825142a0
	if !ctx.cr[6].eq {
	pc = 0x825142A0; continue 'dispatch;
	}
	// 8251429C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 825142A0: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 825142A4: 4BFF44FD  bl 0x825087a0
	ctx.lr = 0x825142A8;
	sub_825087A0(ctx, base);
	// 825142A8: 897F00D4  lbz r11, 0xd4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(212 as u32) ) } as u64;
	// 825142AC: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 825142B0: 4082001C  bne 0x825142cc
	if !ctx.cr[0].eq {
	pc = 0x825142CC; continue 'dispatch;
	}
	// 825142B4: 38FF00E0  addi r7, r31, 0xe0
	ctx.r[7].s64 = ctx.r[31].s64 + 224;
	// 825142B8: 38DF00DC  addi r6, r31, 0xdc
	ctx.r[6].s64 = ctx.r[31].s64 + 220;
	// 825142BC: 38BF00D8  addi r5, r31, 0xd8
	ctx.r[5].s64 = ctx.r[31].s64 + 216;
	// 825142C0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 825142C4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825142C8: 4BFFFE09  bl 0x825140d0
	ctx.lr = 0x825142CC;
	sub_825140D0(ctx, base);
	// 825142CC: 897F0151  lbz r11, 0x151(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(337 as u32) ) } as u64;
	// 825142D0: 3BBF00C8  addi r29, r31, 0xc8
	ctx.r[29].s64 = ctx.r[31].s64 + 200;
	// 825142D4: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 825142D8: 41820064  beq 0x8251433c
	if ctx.cr[0].eq {
	pc = 0x8251433C; continue 'dispatch;
	}
	// 825142DC: 817F00CC  lwz r11, 0xcc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(204 as u32) ) } as u64;
	// 825142E0: 815F00C8  lwz r10, 0xc8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(200 as u32) ) } as u64;
	// 825142E4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825142E8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825142EC: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 825142F0: 419A0024  beq cr6, 0x82514314
	if ctx.cr[6].eq {
	pc = 0x82514314; continue 'dispatch;
	}
	// 825142F4: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 825142F8: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 825142FC: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82514300: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82514304: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82514308: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8251430C: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82514310: 4082FFE8  bne 0x825142f8
	if !ctx.cr[0].eq {
	pc = 0x825142F8; continue 'dispatch;
	}
	// 82514314: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82514318: 809F00D0  lwz r4, 0xd0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(208 as u32) ) } as u64;
	// 8251431C: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82514320: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82514324: 4BFFC6D5  bl 0x825109f8
	ctx.lr = 0x82514328;
	sub_825109F8(ctx, base);
	// 82514328: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8251432C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82514330: 419A0020  beq cr6, 0x82514350
	if ctx.cr[6].eq {
	pc = 0x82514350; continue 'dispatch;
	}
	// 82514334: 4BDAC55D  bl 0x822c0890
	ctx.lr = 0x82514338;
	sub_822C0890(ctx, base);
	// 82514338: 48000018  b 0x82514350
	pc = 0x82514350; continue 'dispatch;
	// 8251433C: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82514340: 809F00D0  lwz r4, 0xd0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(208 as u32) ) } as u64;
	// 82514344: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82514348: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8251434C: 4BFFC80D  bl 0x82510b58
	ctx.lr = 0x82514350;
	sub_82510B58(ctx, base);
	// 82514350: 3BDF00C0  addi r30, r31, 0xc0
	ctx.r[30].s64 = ctx.r[31].s64 + 192;
	// 82514354: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82514358: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8251435C: 4BFFC9E5  bl 0x82510d40
	ctx.lr = 0x82514360;
	sub_82510D40(ctx, base);
	// 82514360: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82514364: 809F00C0  lwz r4, 0xc0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(192 as u32) ) } as u64;
	// 82514368: 4BDD9F81  bl 0x822ee2e8
	ctx.lr = 0x8251436C;
	sub_822EE2E8(ctx, base);
	// 8251436C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82514370: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 82514374: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82514378: 80AB0000  lwz r5, 0(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251437C: 4BFF4825  bl 0x82508ba0
	ctx.lr = 0x82514380;
	sub_82508BA0(ctx, base);
	// 82514380: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82514384: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82514388: 419A0008  beq cr6, 0x82514390
	if ctx.cr[6].eq {
	pc = 0x82514390; continue 'dispatch;
	}
	// 8251438C: 4BDAC505  bl 0x822c0890
	ctx.lr = 0x82514390;
	sub_822C0890(ctx, base);
	// 82514390: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82514394: 809E0000  lwz r4, 0(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82514398: 83FD0000  lwz r31, 0(r29)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251439C: 4BDD9FF5  bl 0x822ee390
	ctx.lr = 0x825143A0;
	sub_822EE390(ctx, base);
	// 825143A0: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 825143A4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825143A8: 48900D49  bl 0x82e150f0
	ctx.lr = 0x825143AC;
	sub_82E150F0(ctx, base);
	// 825143AC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 825143B0: 48C93E08  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825143B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825143B8 size=16
    let mut pc: u32 = 0x825143B8;
    'dispatch: loop {
        match pc {
            0x825143B8 => {
    //   block [0x825143B8..0x825143C8)
	// 825143B8: 808300C8  lwz r4, 0xc8(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(200 as u32) ) } as u64;
	// 825143BC: 38A300C0  addi r5, r3, 0xc0
	ctx.r[5].s64 = ctx.r[3].s64 + 192;
	// 825143C0: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 825143C4: 4BFFC794  b 0x82510b58
	sub_82510B58(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825143C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825143C8 size=88
    let mut pc: u32 = 0x825143C8;
    'dispatch: loop {
        match pc {
            0x825143C8 => {
    //   block [0x825143C8..0x82514420)
	// 825143C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825143CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825143D0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825143D4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825143D8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825143DC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 825143E0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825143E4: 396B1D54  addi r11, r11, 0x1d54
	ctx.r[11].s64 = ctx.r[11].s64 + 7508;
	// 825143E8: 394A1D40  addi r10, r10, 0x1d40
	ctx.r[10].s64 = ctx.r[10].s64 + 7488;
	// 825143EC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825143F0: 915F0028  stw r10, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[10].u32 ) };
	// 825143F4: 807F00C4  lwz r3, 0xc4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(196 as u32) ) } as u64;
	// 825143F8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825143FC: 419A0008  beq cr6, 0x82514404
	if ctx.cr[6].eq {
	pc = 0x82514404; continue 'dispatch;
	}
	// 82514400: 4BDAC491  bl 0x822c0890
	ctx.lr = 0x82514404;
	sub_822C0890(ctx, base);
	// 82514404: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82514408: 4BFFCD91  bl 0x82511198
	ctx.lr = 0x8251440C;
	sub_82511198(ctx, base);
	// 8251440C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82514410: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82514414: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82514418: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8251441C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82514420(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82514420 size=8
    let mut pc: u32 = 0x82514420;
    'dispatch: loop {
        match pc {
            0x82514420 => {
    //   block [0x82514420..0x82514428)
	// 82514420: 3863FFD8  addi r3, r3, -0x28
	ctx.r[3].s64 = ctx.r[3].s64 + -40;
	// 82514424: 48000004  b 0x82514428
	sub_82514428(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82514428(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82514428 size=76
    let mut pc: u32 = 0x82514428;
    'dispatch: loop {
        match pc {
            0x82514428 => {
    //   block [0x82514428..0x82514474)
	// 82514428: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251442C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82514430: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82514434: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82514438: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251443C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82514440: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82514444: 4BFFFF85  bl 0x825143c8
	ctx.lr = 0x82514448;
	sub_825143C8(ctx, base);
	// 82514448: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8251444C: 4182000C  beq 0x82514458
	if ctx.cr[0].eq {
	pc = 0x82514458; continue 'dispatch;
	}
	// 82514450: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82514454: 488DDF85  bl 0x82df23d8
	ctx.lr = 0x82514458;
	sub_82DF23D8(ctx, base);
	// 82514458: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8251445C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82514460: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82514464: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82514468: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8251446C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82514470: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82514478(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82514478 size=220
    let mut pc: u32 = 0x82514478;
    'dispatch: loop {
        match pc {
            0x82514478 => {
    //   block [0x82514478..0x82514554)
	// 82514478: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251447C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82514480: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82514484: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82514488: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251448C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82514490: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82514494: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82514498: 90BF00C8  stw r5, 0xc8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(200 as u32), ctx.r[5].u32 ) };
	// 8251449C: 80840000  lwz r4, 0(r4)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 825144A0: 48916649  bl 0x82e2aae8
	ctx.lr = 0x825144A4;
	sub_82E2AAE8(ctx, base);
	// 825144A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825144A8: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 825144AC: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 825144B0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825144B4: 4891A9BD  bl 0x82e2ee70
	ctx.lr = 0x825144B8;
	sub_82E2EE70(ctx, base);
	// 825144B8: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 825144BC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825144C0: 419A0060  beq cr6, 0x82514520
	if ctx.cr[6].eq {
	pc = 0x82514520; continue 'dispatch;
	}
	// 825144C4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 825144C8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825144CC: 388B1D7C  addi r4, r11, 0x1d7c
	ctx.r[4].s64 = ctx.r[11].s64 + 7548;
	// 825144D0: 38A00021  li r5, 0x21
	ctx.r[5].s64 = 33;
	// 825144D4: 38600098  li r3, 0x98
	ctx.r[3].s64 = 152;
	// 825144D8: 488DDF11  bl 0x82df23e8
	ctx.lr = 0x825144DC;
	sub_82DF23E8(ctx, base);
	// 825144DC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 825144E0: 41820014  beq 0x825144f4
	if ctx.cr[0].eq {
	pc = 0x825144F4; continue 'dispatch;
	}
	// 825144E4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 825144E8: 489023A9  bl 0x82e16890
	ctx.lr = 0x825144EC;
	sub_82E16890(ctx, base);
	// 825144EC: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 825144F0: 48000008  b 0x825144f8
	pc = 0x825144F8; continue 'dispatch;
	// 825144F4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 825144F8: 387F00C0  addi r3, r31, 0xc0
	ctx.r[3].s64 = ctx.r[31].s64 + 192;
	// 825144FC: 4BE48E75  bl 0x8235d370
	ctx.lr = 0x82514500;
	sub_8235D370(ctx, base);
	// 82514500: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82514504: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82514508: 419A0008  beq cr6, 0x82514510
	if ctx.cr[6].eq {
	pc = 0x82514510; continue 'dispatch;
	}
	// 8251450C: 4BDAC385  bl 0x822c0890
	ctx.lr = 0x82514510;
	sub_822C0890(ctx, base);
	// 82514510: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82514514: 489165ED  bl 0x82e2ab00
	ctx.lr = 0x82514518;
	sub_82E2AB00(ctx, base);
	// 82514518: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8251451C: 48000020  b 0x8251453c
	pc = 0x8251453C; continue 'dispatch;
	// 82514520: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82514524: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82514528: 419A0008  beq cr6, 0x82514530
	if ctx.cr[6].eq {
	pc = 0x82514530; continue 'dispatch;
	}
	// 8251452C: 4BDAC365  bl 0x822c0890
	ctx.lr = 0x82514530;
	sub_822C0890(ctx, base);
	// 82514530: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82514534: 489165CD  bl 0x82e2ab00
	ctx.lr = 0x82514538;
	sub_82E2AB00(ctx, base);
	// 82514538: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8251453C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82514540: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82514544: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82514548: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8251454C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82514550: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82514558(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82514558 size=16
    let mut pc: u32 = 0x82514558;
    'dispatch: loop {
        match pc {
            0x82514558 => {
    //   block [0x82514558..0x82514568)
	// 82514558: 81640018  lwz r11, 0x18(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(24 as u32) ) } as u64;
	// 8251455C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82514560: 994B0000  stb r10, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 82514564: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82514568(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82514568 size=80
    let mut pc: u32 = 0x82514568;
    'dispatch: loop {
        match pc {
            0x82514568 => {
    //   block [0x82514568..0x825145B8)
	// 82514568: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251456C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82514570: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82514574: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82514578: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8251457C: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82514580: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82514584: 394A9B84  addi r10, r10, -0x647c
	ctx.r[10].s64 = ctx.r[10].s64 + -25724;
	// 82514588: 388B0008  addi r4, r11, 8
	ctx.r[4].s64 = ctx.r[11].s64 + 8;
	// 8251458C: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82514590: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 82514594: 896B0004  lbz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82514598: 997F0004  stb r11, 4(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u8 ) };
	// 8251459C: 488DF665  bl 0x82df3c00
	ctx.lr = 0x825145A0;
	sub_82DF3C00(ctx, base);
	// 825145A0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825145A4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825145A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825145AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825145B0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825145B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825145B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825145B8 size=132
    let mut pc: u32 = 0x825145B8;
    'dispatch: loop {
        match pc {
            0x825145B8 => {
    //   block [0x825145B8..0x8251463C)
	// 825145B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825145BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825145C0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825145C4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825145C8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 825145CC: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 825145D0: 388BC278  addi r4, r11, -0x3d88
	ctx.r[4].s64 = ctx.r[11].s64 + -15752;
	// 825145D4: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 825145D8: 488DF9F1  bl 0x82df3fc8
	ctx.lr = 0x825145DC;
	sub_82DF3FC8(ctx, base);
	// 825145DC: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 825145E0: 816BB230  lwz r11, -0x4dd0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-19920 as u32) ) } as u64;
	// 825145E4: 7F035800  cmpw cr6, r3, r11
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[11].s32, &mut ctx.xer);
	// 825145E8: 419A000C  beq cr6, 0x825145f4
	if ctx.cr[6].eq {
	pc = 0x825145F4; continue 'dispatch;
	}
	// 825145EC: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 825145F0: 48000038  b 0x82514628
	pc = 0x82514628; continue 'dispatch;
	// 825145F4: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 825145F8: 38802004  li r4, 0x2004
	ctx.r[4].s64 = 8196;
	// 825145FC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82514600: 4BF7AC61  bl 0x8248f260
	ctx.lr = 0x82514604;
	sub_8248F260(ctx, base);
	// 82514604: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82514608: 4182001C  beq 0x82514624
	if ctx.cr[0].eq {
	pc = 0x82514624; continue 'dispatch;
	}
	// 8251460C: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82514610: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 82514614: 419AFFD8  beq cr6, 0x825145ec
	if ctx.cr[6].eq {
	pc = 0x825145EC; continue 'dispatch;
	}
	// 82514618: 2F0B000E  cmpwi cr6, r11, 0xe
	ctx.cr[6].compare_i32(ctx.r[11].s32, 14, &mut ctx.xer);
	// 8251461C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82514620: 419A0008  beq cr6, 0x82514628
	if ctx.cr[6].eq {
	pc = 0x82514628; continue 'dispatch;
	}
	// 82514624: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82514628: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8251462C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82514630: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82514634: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82514638: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82514640(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82514640 size=196
    let mut pc: u32 = 0x82514640;
    'dispatch: loop {
        match pc {
            0x82514640 => {
    //   block [0x82514640..0x82514704)
	// 82514640: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82514644: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82514648: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8251464C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82514650: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82514654: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82514658: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8251465C: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82514660: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82514664: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82514668: 4BDAC2D1  bl 0x822c0938
	ctx.lr = 0x8251466C;
	sub_822C0938(ctx, base);
	// 8251466C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82514670: 41820028  beq 0x82514698
	if ctx.cr[0].eq {
	pc = 0x82514698; continue 'dispatch;
	}
	// 82514674: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82514678: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 8251467C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82514680: 392B1DBC  addi r9, r11, 0x1dbc
	ctx.r[9].s64 = ctx.r[11].s64 + 7612;
	// 82514684: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82514688: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8251468C: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82514690: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82514694: 48000008  b 0x8251469c
	pc = 0x8251469C; continue 'dispatch;
	// 82514698: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8251469C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825146A0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825146A4: 409A0044  bne cr6, 0x825146e8
	if !ctx.cr[6].eq {
	pc = 0x825146E8; continue 'dispatch;
	}
	// 825146A8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 825146AC: 419A001C  beq cr6, 0x825146c8
	if ctx.cr[6].eq {
	pc = 0x825146C8; continue 'dispatch;
	}
	// 825146B0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825146B4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 825146B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825146BC: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825146C0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825146C4: 4E800421  bctrl
	ctx.lr = 0x825146C8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825146C8: 3D608328  lis r11, -0x7cd8
	ctx.r[11].s64 = -2094530560;
	// 825146CC: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 825146D0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825146D4: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 825146D8: 816BCD18  lwz r11, -0x32e8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-13032 as u32) ) } as u64;
	// 825146DC: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 825146E0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825146E4: 4BDAB91D  bl 0x822c0000
	ctx.lr = 0x825146E8;
	sub_822C0000(ctx, base);
	// 825146E8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825146EC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825146F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825146F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825146F8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825146FC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82514700: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82514708(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82514708 size=100
    let mut pc: u32 = 0x82514708;
    'dispatch: loop {
        match pc {
            0x82514708 => {
    //   block [0x82514708..0x8251476C)
	// 82514708: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251470C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82514710: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82514714: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82514718: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8251471C: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82514720: 39640010  addi r11, r4, 0x10
	ctx.r[11].s64 = ctx.r[4].s64 + 16;
	// 82514724: 409A0008  bne cr6, 0x8251472c
	if !ctx.cr[6].eq {
	pc = 0x8251472C; continue 'dispatch;
	}
	// 82514728: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8251472C: 395F0010  addi r10, r31, 0x10
	ctx.r[10].s64 = ctx.r[31].s64 + 16;
	// 82514730: 13E058C7  vcmpequd (lvx128) v31, v0, v11
	tmp.u32 = ctx.r[11].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82514734: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82514770(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82514770 size=200
    let mut pc: u32 = 0x82514770;
    'dispatch: loop {
        match pc {
            0x82514770 => {
    //   block [0x82514770..0x82514838)
	// 82514770: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82514774: 48C939F5  bl 0x831a8168
	ctx.lr = 0x82514778;
	sub_831A8130(ctx, base);
	// 82514778: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251477C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82514780: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82514784: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82514788: 7CFD3B78  mr r29, r7
	ctx.r[29].u64 = ctx.r[7].u64;
	// 8251478C: 90BF00DC  stw r5, 0xdc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(220 as u32), ctx.r[5].u32 ) };
	// 82514790: 80840000  lwz r4, 0(r4)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82514794: 4BDD2A75  bl 0x822e7208
	ctx.lr = 0x82514798;
	sub_822E7208(ctx, base);
	// 82514798: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8251479C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825147A0: 388B9BC9  addi r4, r11, -0x6437
	ctx.r[4].s64 = ctx.r[11].s64 + -25655;
	// 825147A4: 488DF265  bl 0x82df3a08
	ctx.lr = 0x825147A8;
	sub_82DF3A08(ctx, base);
	// 825147A8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 825147AC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825147B0: 488DEAF1  bl 0x82df32a0
	ctx.lr = 0x825147B4;
	sub_82DF32A0(ctx, base);
	// 825147B4: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 825147B8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825147BC: 488DEC6D  bl 0x82df3428
	ctx.lr = 0x825147C0;
	sub_82DF3428(ctx, base);
	// 825147C0: 578B063F  clrlwi. r11, r28, 0x18
	ctx.r[11].u64 = ctx.r[28].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825147C4: 41820044  beq 0x82514808
	if ctx.cr[0].eq {
	pc = 0x82514808; continue 'dispatch;
	}
	// 825147C8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825147CC: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 825147D0: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 825147D4: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 825147D8: 4BDD2BB9  bl 0x822e7390
	ctx.lr = 0x825147DC;
	sub_822E7390(ctx, base);
	// 825147DC: 80610058  lwz r3, 0x58(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 825147E0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825147E4: 419A002C  beq cr6, 0x82514810
	if ctx.cr[6].eq {
	pc = 0x82514810; continue 'dispatch;
	}
	// 825147E8: 38BF0028  addi r5, r31, 0x28
	ctx.r[5].s64 = ctx.r[31].s64 + 40;
	// 825147EC: 389F00C4  addi r4, r31, 0xc4
	ctx.r[4].s64 = ctx.r[31].s64 + 196;
	// 825147F0: 4BDDA729  bl 0x822eef18
	ctx.lr = 0x825147F4;
	sub_822EEF18(ctx, base);
	// 825147F4: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 825147F8: 9BBF00E1  stb r29, 0xe1(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(225 as u32), ctx.r[29].u8 ) };
	// 825147FC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82514800: 419A0008  beq cr6, 0x82514808
	if ctx.cr[6].eq {
	pc = 0x82514808; continue 'dispatch;
	}
	// 82514804: 4BDAC08D  bl 0x822c0890
	ctx.lr = 0x82514808;
	sub_822C0890(ctx, base);
	// 82514808: 3BE00001  li r31, 1
	ctx.r[31].s64 = 1;
	// 8251480C: 48000018  b 0x82514824
	pc = 0x82514824; continue 'dispatch;
	// 82514810: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82514814: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82514818: 419A0008  beq cr6, 0x82514820
	if ctx.cr[6].eq {
	pc = 0x82514820; continue 'dispatch;
	}
	// 8251481C: 4BDAC075  bl 0x822c0890
	ctx.lr = 0x82514820;
	sub_822C0890(ctx, base);
	// 82514820: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82514824: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82514828: 4BDD29F9  bl 0x822e7220
	ctx.lr = 0x8251482C;
	sub_822E7220(ctx, base);
	// 8251482C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82514830: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82514834: 48C93984  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82514838(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82514838 size=196
    let mut pc: u32 = 0x82514838;
    'dispatch: loop {
        match pc {
            0x82514838 => {
    //   block [0x82514838..0x825148FC)
	// 82514838: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251483C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82514840: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82514844: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82514848: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8251484C: 38BF00D4  addi r5, r31, 0xd4
	ctx.r[5].s64 = ctx.r[31].s64 + 212;
	// 82514850: 817F00D4  lwz r11, 0xd4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(212 as u32) ) } as u64;
	// 82514854: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82514858: 419A0090  beq cr6, 0x825148e8
	if ctx.cr[6].eq {
	pc = 0x825148E8; continue 'dispatch;
	}
	// 8251485C: 897F00E0  lbz r11, 0xe0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(224 as u32) ) } as u64;
	// 82514860: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82514864: 40820084  bne 0x825148e8
	if !ctx.cr[0].eq {
	pc = 0x825148E8; continue 'dispatch;
	}
	// 82514868: 897F00C0  lbz r11, 0xc0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(192 as u32) ) } as u64;
	// 8251486C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82514870: 41820014  beq 0x82514884
	if ctx.cr[0].eq {
	pc = 0x82514884; continue 'dispatch;
	}
	// 82514874: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82514878: 809F00DC  lwz r4, 0xdc(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(220 as u32) ) } as u64;
	// 8251487C: 4BFFC17D  bl 0x825109f8
	ctx.lr = 0x82514880;
	sub_825109F8(ctx, base);
	// 82514880: 48000060  b 0x825148e0
	pc = 0x825148E0; continue 'dispatch;
	// 82514884: 81650004  lwz r11, 4(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) } as u64;
	// 82514888: 81450000  lwz r10, 0(r5)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251488C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82514890: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82514894: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82514898: 419A0024  beq cr6, 0x825148bc
	if ctx.cr[6].eq {
	pc = 0x825148BC; continue 'dispatch;
	}
	// 8251489C: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 825148A0: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 825148A4: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 825148A8: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 825148AC: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 825148B0: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 825148B4: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 825148B8: 4082FFE8  bne 0x825148a0
	if !ctx.cr[0].eq {
	pc = 0x825148A0; continue 'dispatch;
	}
	// 825148BC: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 825148C0: 809F00DC  lwz r4, 0xdc(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(220 as u32) ) } as u64;
	// 825148C4: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 825148C8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825148CC: 4BFFC28D  bl 0x82510b58
	ctx.lr = 0x825148D0;
	sub_82510B58(ctx, base);
	// 825148D0: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 825148D4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825148D8: 419A0008  beq cr6, 0x825148e0
	if ctx.cr[6].eq {
	pc = 0x825148E0; continue 'dispatch;
	}
	// 825148DC: 4BDABFB5  bl 0x822c0890
	ctx.lr = 0x825148E0;
	sub_822C0890(ctx, base);
	// 825148E0: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 825148E4: 997F00E0  stb r11, 0xe0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(224 as u32), ctx.r[11].u8 ) };
	// 825148E8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825148EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825148F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825148F4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825148F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82514900(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82514900 size=196
    let mut pc: u32 = 0x82514900;
    'dispatch: loop {
        match pc {
            0x82514900 => {
    //   block [0x82514900..0x825149C4)
	// 82514900: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82514904: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82514908: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8251490C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82514910: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82514914: 38BF00D4  addi r5, r31, 0xd4
	ctx.r[5].s64 = ctx.r[31].s64 + 212;
	// 82514918: 817F00D4  lwz r11, 0xd4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(212 as u32) ) } as u64;
	// 8251491C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82514920: 419A0090  beq cr6, 0x825149b0
	if ctx.cr[6].eq {
	pc = 0x825149B0; continue 'dispatch;
	}
	// 82514924: 897F00E0  lbz r11, 0xe0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(224 as u32) ) } as u64;
	// 82514928: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8251492C: 41820084  beq 0x825149b0
	if ctx.cr[0].eq {
	pc = 0x825149B0; continue 'dispatch;
	}
	// 82514930: 897F00C0  lbz r11, 0xc0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(192 as u32) ) } as u64;
	// 82514934: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82514938: 41820014  beq 0x8251494c
	if ctx.cr[0].eq {
	pc = 0x8251494C; continue 'dispatch;
	}
	// 8251493C: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82514940: 809F00DC  lwz r4, 0xdc(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(220 as u32) ) } as u64;
	// 82514944: 4BFFB77D  bl 0x825100c0
	ctx.lr = 0x82514948;
	sub_825100C0(ctx, base);
	// 82514948: 48000060  b 0x825149a8
	pc = 0x825149A8; continue 'dispatch;
	// 8251494C: 81650004  lwz r11, 4(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) } as u64;
	// 82514950: 81450000  lwz r10, 0(r5)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 82514954: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82514958: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8251495C: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82514960: 419A0024  beq cr6, 0x82514984
	if ctx.cr[6].eq {
	pc = 0x82514984; continue 'dispatch;
	}
	// 82514964: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82514968: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8251496C: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82514970: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82514974: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82514978: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8251497C: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82514980: 4082FFE8  bne 0x82514968
	if !ctx.cr[0].eq {
	pc = 0x82514968; continue 'dispatch;
	}
	// 82514984: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82514988: 809F00DC  lwz r4, 0xdc(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(220 as u32) ) } as u64;
	// 8251498C: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82514990: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82514994: 4BFFB85D  bl 0x825101f0
	ctx.lr = 0x82514998;
	sub_825101F0(ctx, base);
	// 82514998: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8251499C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825149A0: 419A0008  beq cr6, 0x825149a8
	if ctx.cr[6].eq {
	pc = 0x825149A8; continue 'dispatch;
	}
	// 825149A4: 4BDABEED  bl 0x822c0890
	ctx.lr = 0x825149A8;
	sub_822C0890(ctx, base);
	// 825149A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825149AC: 997F00E0  stb r11, 0xe0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(224 as u32), ctx.r[11].u8 ) };
	// 825149B0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825149B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825149B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825149BC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825149C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825149C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825149C8 size=112
    let mut pc: u32 = 0x825149C8;
    'dispatch: loop {
        match pc {
            0x825149C8 => {
    //   block [0x825149C8..0x82514A38)
	// 825149C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825149CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825149D0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825149D4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825149D8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825149DC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825149E0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825149E4: 93C10050  stw r30, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 825149E8: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 825149EC: 4BFFFC55  bl 0x82514640
	ctx.lr = 0x825149F0;
	sub_82514640(ctx, base);
	// 825149F0: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 825149F4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 825149F8: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 825149FC: 4BDAB605  bl 0x822c0000
	ctx.lr = 0x82514A00;
	sub_822C0000(ctx, base);
	// 82514A00: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82514A04: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82514A08: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82514A0C: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82514A10: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82514A14: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82514A18: 419A0008  beq cr6, 0x82514a20
	if ctx.cr[6].eq {
	pc = 0x82514A20; continue 'dispatch;
	}
	// 82514A1C: 4BDABE75  bl 0x822c0890
	ctx.lr = 0x82514A20;
	sub_822C0890(ctx, base);
	// 82514A20: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82514A24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82514A28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82514A2C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82514A30: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82514A34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82514A38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82514A38 size=344
    let mut pc: u32 = 0x82514A38;
    'dispatch: loop {
        match pc {
            0x82514A38 => {
    //   block [0x82514A38..0x82514B90)
	// 82514A38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82514A3C: 48C9372D  bl 0x831a8168
	ctx.lr = 0x82514A40;
	sub_831A8130(ctx, base);
	// 82514A40: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82514A44: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 82514A48: 7CE63B78  mr r6, r7
	ctx.r[6].u64 = ctx.r[7].u64;
	// 82514A4C: 7D074378  mr r7, r8
	ctx.r[7].u64 = ctx.r[8].u64;
	// 82514A50: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82514A54: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82514A58: 4BFFFD19  bl 0x82514770
	ctx.lr = 0x82514A5C;
	sub_82514770(ctx, base);
	// 82514A5C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82514A60: 4082000C  bne 0x82514a6c
	if !ctx.cr[0].eq {
	pc = 0x82514A6C; continue 'dispatch;
	}
	// 82514A64: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82514A68: 48000104  b 0x82514b6c
	pc = 0x82514B6C; continue 'dispatch;
	// 82514A6C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82514A70: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82514A74: 388B9BC9  addi r4, r11, -0x6437
	ctx.r[4].s64 = ctx.r[11].s64 + -25655;
	// 82514A78: 488DEF91  bl 0x82df3a08
	ctx.lr = 0x82514A7C;
	sub_82DF3A08(ctx, base);
	// 82514A7C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82514A80: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82514A84: 488DE81D  bl 0x82df32a0
	ctx.lr = 0x82514A88;
	sub_82DF32A0(ctx, base);
	// 82514A88: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82514A8C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82514A90: 488DE999  bl 0x82df3428
	ctx.lr = 0x82514A94;
	sub_82DF3428(ctx, base);
	// 82514A94: 578B063F  clrlwi. r11, r28, 0x18
	ctx.r[11].u64 = ctx.r[28].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82514A98: 418200C8  beq 0x82514b60
	if ctx.cr[0].eq {
	pc = 0x82514B60; continue 'dispatch;
	}
	// 82514A9C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82514AA0: 809D0000  lwz r4, 0(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82514AA4: 48916045  bl 0x82e2aae8
	ctx.lr = 0x82514AA8;
	sub_82E2AAE8(ctx, base);
	// 82514AA8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82514AAC: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82514AB0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82514AB4: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82514AB8: 4891A3B9  bl 0x82e2ee70
	ctx.lr = 0x82514ABC;
	sub_82E2EE70(ctx, base);
	// 82514ABC: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82514AC0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82514AC4: 419A00B0  beq cr6, 0x82514b74
	if ctx.cr[6].eq {
	pc = 0x82514B74; continue 'dispatch;
	}
	// 82514AC8: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82514ACC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82514AD0: 388B1DD8  addi r4, r11, 0x1dd8
	ctx.r[4].s64 = ctx.r[11].s64 + 7640;
	// 82514AD4: 38A00052  li r5, 0x52
	ctx.r[5].s64 = 82;
	// 82514AD8: 38600098  li r3, 0x98
	ctx.r[3].s64 = 152;
	// 82514ADC: 488DD90D  bl 0x82df23e8
	ctx.lr = 0x82514AE0;
	sub_82DF23E8(ctx, base);
	// 82514AE0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82514AE4: 41820014  beq 0x82514af8
	if ctx.cr[0].eq {
	pc = 0x82514AF8; continue 'dispatch;
	}
	// 82514AE8: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 82514AEC: 48901DA5  bl 0x82e16890
	ctx.lr = 0x82514AF0;
	sub_82E16890(ctx, base);
	// 82514AF0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82514AF4: 48000008  b 0x82514afc
	pc = 0x82514AFC; continue 'dispatch;
	// 82514AF8: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82514AFC: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 82514B00: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 82514B04: 39000010  li r8, 0x10
	ctx.r[8].s64 = 16;
	// 82514B08: 38EB6880  addi r7, r11, 0x6880
	ctx.r[7].s64 = ctx.r[11].s64 + 26752;
	// 82514B0C: 39200020  li r9, 0x20
	ctx.r[9].s64 = 32;
	// 82514B10: 39400030  li r10, 0x30
	ctx.r[10].s64 = 48;
	// 82514B14: 39630010  addi r11, r3, 0x10
	ctx.r[11].s64 = ctx.r[3].s64 + 16;
	// 82514B18: 13E83C07  vcmpneb. (lvlx128) v31, v8, v7
	tmp.u32 = ctx.r[8].u32 + ctx.r[7].u32;
	// load shuffled into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82514B1C: 13C93C07  vcmpneb. (lvlx128) v30, v9, v7
	tmp.u32 = ctx.r[9].u32 + ctx.r[7].u32;
	// load shuffled into ctx.v[62] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82514B20: 13AA3C07  vcmpneb. (lvlx128) v29, v10, v7
	tmp.u32 = ctx.r[10].u32 + ctx.r[7].u32;
	// load shuffled into ctx.v[61] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82514B24: 13803C07  vcmpneb. (lvlx128) v28, v0, v7
	tmp.u32 = ctx.r[7].u32;
	// load shuffled into ctx.v[60] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82514B90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82514B90 size=404
    let mut pc: u32 = 0x82514B90;
    'dispatch: loop {
        match pc {
            0x82514B90 => {
    //   block [0x82514B90..0x82514D24)
	// 82514B90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82514B94: 48C935D1  bl 0x831a8164
	ctx.lr = 0x82514B98;
	sub_831A8130(ctx, base);
	// 82514B98: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82514B9C: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82514BA0: 7CE63B78  mr r6, r7
	ctx.r[6].u64 = ctx.r[7].u64;
	// 82514BA4: 7D074378  mr r7, r8
	ctx.r[7].u64 = ctx.r[8].u64;
	// 82514BA8: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82514BAC: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82514BB0: 3B800001  li r28, 1
	ctx.r[28].s64 = 1;
	// 82514BB4: 4BFFFBBD  bl 0x82514770
	ctx.lr = 0x82514BB8;
	sub_82514770(ctx, base);
	// 82514BB8: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82514BBC: 40820008  bne 0x82514bc4
	if !ctx.cr[0].eq {
	pc = 0x82514BC4; continue 'dispatch;
	}
	// 82514BC0: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82514BC4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82514BC8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82514BCC: 388B9BC9  addi r4, r11, -0x6437
	ctx.r[4].s64 = ctx.r[11].s64 + -25655;
	// 82514BD0: 488DEE39  bl 0x82df3a08
	ctx.lr = 0x82514BD4;
	sub_82DF3A08(ctx, base);
	// 82514BD4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82514BD8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82514BDC: 488DE6C5  bl 0x82df32a0
	ctx.lr = 0x82514BE0;
	sub_82DF32A0(ctx, base);
	// 82514BE0: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82514BE4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82514BE8: 488DE841  bl 0x82df3428
	ctx.lr = 0x82514BEC;
	sub_82DF3428(ctx, base);
	// 82514BEC: 576B063F  clrlwi. r11, r27, 0x18
	ctx.r[11].u64 = ctx.r[27].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82514BF0: 41820120  beq 0x82514d10
	if ctx.cr[0].eq {
	pc = 0x82514D10; continue 'dispatch;
	}
	// 82514BF4: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82514BF8: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82514BFC: 48915EED  bl 0x82e2aae8
	ctx.lr = 0x82514C00;
	sub_82E2AAE8(ctx, base);
	// 82514C00: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82514C04: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82514C08: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 82514C0C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82514C10: 489170B1  bl 0x82e2bcc0
	ctx.lr = 0x82514C14;
	sub_82E2BCC0(ctx, base);
	// 82514C14: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82514C18: 3BFD00E4  addi r31, r29, 0xe4
	ctx.r[31].s64 = ctx.r[29].s64 + 228;
	// 82514C1C: 388B0004  addi r4, r11, 4
	ctx.r[4].s64 = ctx.r[11].s64 + 4;
	// 82514C20: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 82514C24: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82514C28: 917D00E4  stw r11, 0xe4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(228 as u32), ctx.r[11].u32 ) };
	// 82514C2C: 4BDAF835  bl 0x822c4460
	ctx.lr = 0x82514C30;
	sub_822C4460(ctx, base);
	// 82514C30: 80610064  lwz r3, 0x64(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82514C34: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82514C38: 419A0008  beq cr6, 0x82514c40
	if ctx.cr[6].eq {
	pc = 0x82514C40; continue 'dispatch;
	}
	// 82514C3C: 4BDABC55  bl 0x822c0890
	ctx.lr = 0x82514C40;
	sub_822C0890(ctx, base);
	// 82514C40: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82514C44: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82514C48: 419A00BC  beq cr6, 0x82514d04
	if ctx.cr[6].eq {
	pc = 0x82514D04; continue 'dispatch;
	}
	// 82514C4C: 488E9B95  bl 0x82dfe7e0
	ctx.lr = 0x82514C50;
	sub_82DFE7E0(ctx, base);
	// 82514C50: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82514C54: 546A063F  clrlwi. r10, r3, 0x18
	ctx.r[10].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82514C58: 3BCB1DD8  addi r30, r11, 0x1dd8
	ctx.r[30].s64 = ctx.r[11].s64 + 7640;
	// 82514C5C: 40820060  bne 0x82514cbc
	if !ctx.cr[0].eq {
	pc = 0x82514CBC; continue 'dispatch;
	}
	// 82514C60: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82514C64: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82514C68: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82514C6C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82514C70: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82514C74: 419A0024  beq cr6, 0x82514c98
	if ctx.cr[6].eq {
	pc = 0x82514C98; continue 'dispatch;
	}
	// 82514C78: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82514C7C: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82514C80: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82514C84: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82514C88: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82514C8C: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82514C90: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82514C94: 4082FFE8  bne 0x82514c7c
	if !ctx.cr[0].eq {
	pc = 0x82514C7C; continue 'dispatch;
	}
	// 82514C98: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 82514C9C: 4BFD5DED  bl 0x824eaa88
	ctx.lr = 0x82514CA0;
	sub_824EAA88(ctx, base);
	// 82514CA0: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82514CA4: 38C00077  li r6, 0x77
	ctx.r[6].s64 = 119;
	// 82514CA8: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82514CAC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82514CB0: 486A54D1  bl 0x82bba180
	ctx.lr = 0x82514CB4;
	sub_82BBA180(ctx, base);
	// 82514CB4: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 82514CB8: 488DCFD9  bl 0x82df1c90
	ctx.lr = 0x82514CBC;
	sub_82DF1C90(ctx, base);
	// 82514CBC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82514CC0: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82514CC4: 38A00078  li r5, 0x78
	ctx.r[5].s64 = 120;
	// 82514CC8: 38600084  li r3, 0x84
	ctx.r[3].s64 = 132;
	// 82514CCC: 488DD71D  bl 0x82df23e8
	ctx.lr = 0x82514CD0;
	sub_82DF23E8(ctx, base);
	// 82514CD0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82514CD4: 41820010  beq 0x82514ce4
	if ctx.cr[0].eq {
	pc = 0x82514CE4; continue 'dispatch;
	}
	// 82514CD8: 4890C749  bl 0x82e21420
	ctx.lr = 0x82514CDC;
	sub_82E21420(ctx, base);
	// 82514CDC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82514CE0: 48000008  b 0x82514ce8
	pc = 0x82514CE8; continue 'dispatch;
	// 82514CE4: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82514CE8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82514CEC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82514CF0: 4890CD99  bl 0x82e21a88
	ctx.lr = 0x82514CF4;
	sub_82E21A88(ctx, base);
	// 82514CF4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82514CF8: 387D00D4  addi r3, r29, 0xd4
	ctx.r[3].s64 = ctx.r[29].s64 + 212;
	// 82514CFC: 4BFFFCCD  bl 0x825149c8
	ctx.lr = 0x82514D00;
	sub_825149C8(ctx, base);
	// 82514D00: 48000008  b 0x82514d08
	pc = 0x82514D08; continue 'dispatch;
	// 82514D04: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82514D08: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82514D0C: 48915DF5  bl 0x82e2ab00
	ctx.lr = 0x82514D10;
	sub_82E2AB00(ctx, base);
	// 82514D10: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82514D14: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82514D18: 997D00C0  stb r11, 0xc0(r29)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[29].u32.wrapping_add(192 as u32), ctx.r[11].u8 ) };
	// 82514D1C: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82514D20: 48C93494  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82514D28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82514D28 size=828
    let mut pc: u32 = 0x82514D28;
    'dispatch: loop {
        match pc {
            0x82514D28 => {
    //   block [0x82514D28..0x82515064)
	// 82514D28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82514D2C: 48C93431  bl 0x831a815c
	ctx.lr = 0x82514D30;
	sub_831A8130(ctx, base);
	// 82514D30: 9421FF00  stwu r1, -0x100(r1)
	ea = ctx.r[1].u32.wrapping_add(-256 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82514D34: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82514D38: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82514D3C: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 82514D40: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82514D44: 389F0028  addi r4, r31, 0x28
	ctx.r[4].s64 = ctx.r[31].s64 + 40;
	// 82514D48: 409A0008  bne cr6, 0x82514d50
	if !ctx.cr[6].eq {
	pc = 0x82514D50; continue 'dispatch;
	}
	// 82514D4C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82514D50: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82514D54: 4BFF3A4D  bl 0x825087a0
	ctx.lr = 0x82514D58;
	sub_825087A0(ctx, base);
	// 82514D58: 817F00D4  lwz r11, 0xd4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(212 as u32) ) } as u64;
	// 82514D5C: 3B7F00D4  addi r27, r31, 0xd4
	ctx.r[27].s64 = ctx.r[31].s64 + 212;
	// 82514D60: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82514D64: 419A008C  beq cr6, 0x82514df0
	if ctx.cr[6].eq {
	pc = 0x82514DF0; continue 'dispatch;
	}
	// 82514D68: 897F00C0  lbz r11, 0xc0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(192 as u32) ) } as u64;
	// 82514D6C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82514D70: 4182001C  beq 0x82514d8c
	if ctx.cr[0].eq {
	pc = 0x82514D8C; continue 'dispatch;
	}
	// 82514D74: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82514D78: 809F00DC  lwz r4, 0xdc(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(220 as u32) ) } as u64;
	// 82514D7C: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82514D80: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82514D84: 4BFFBC75  bl 0x825109f8
	ctx.lr = 0x82514D88;
	sub_825109F8(ctx, base);
	// 82514D88: 48000060  b 0x82514de8
	pc = 0x82514DE8; continue 'dispatch;
	// 82514D8C: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82514D90: 815B0000  lwz r10, 0(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82514D94: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82514D98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82514D9C: 91410068  stw r10, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[10].u32 ) };
	// 82514DA0: 419A0024  beq cr6, 0x82514dc4
	if ctx.cr[6].eq {
	pc = 0x82514DC4; continue 'dispatch;
	}
	// 82514DA4: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82514DA8: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82514DAC: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82514DB0: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82514DB4: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82514DB8: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82514DBC: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82514DC0: 4082FFE8  bne 0x82514da8
	if !ctx.cr[0].eq {
	pc = 0x82514DA8; continue 'dispatch;
	}
	// 82514DC4: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82514DC8: 809F00DC  lwz r4, 0xdc(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(220 as u32) ) } as u64;
	// 82514DCC: 38A10068  addi r5, r1, 0x68
	ctx.r[5].s64 = ctx.r[1].s64 + 104;
	// 82514DD0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82514DD4: 4BFFBD85  bl 0x82510b58
	ctx.lr = 0x82514DD8;
	sub_82510B58(ctx, base);
	// 82514DD8: 8061006C  lwz r3, 0x6c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(108 as u32) ) } as u64;
	// 82514DDC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82514DE0: 419A0008  beq cr6, 0x82514de8
	if ctx.cr[6].eq {
	pc = 0x82514DE8; continue 'dispatch;
	}
	// 82514DE4: 4BDABAAD  bl 0x822c0890
	ctx.lr = 0x82514DE8;
	sub_822C0890(ctx, base);
	// 82514DE8: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82514DEC: 997F00E0  stb r11, 0xe0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(224 as u32), ctx.r[11].u8 ) };
	// 82514DF0: 839F00CC  lwz r28, 0xcc(r31)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(204 as u32) ) } as u64;
	// 82514DF4: 83DF00C8  lwz r30, 0xc8(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(200 as u32) ) } as u64;
	// 82514DF8: 480000C4  b 0x82514ebc
	pc = 0x82514EBC; continue 'dispatch;
	// 82514DFC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82514E00: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82514E04: 485FA9E5  bl 0x82b0f7e8
	ctx.lr = 0x82514E08;
	sub_82B0F7E8(ctx, base);
	// 82514E08: 81610060  lwz r11, 0x60(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82514E0C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82514E10: 419A0078  beq cr6, 0x82514e88
	if ctx.cr[6].eq {
	pc = 0x82514E88; continue 'dispatch;
	}
	// 82514E14: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82514E18: 4BDAB1E9  bl 0x822c0000
	ctx.lr = 0x82514E1C;
	sub_822C0000(ctx, base);
	// 82514E1C: 81610064  lwz r11, 0x64(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82514E20: 81410060  lwz r10, 0x60(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82514E24: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82514E28: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82514E2C: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 82514E30: 419A0024  beq cr6, 0x82514e54
	if ctx.cr[6].eq {
	pc = 0x82514E54; continue 'dispatch;
	}
	// 82514E34: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82514E38: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82514E3C: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82514E40: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82514E44: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82514E48: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82514E4C: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82514E50: 4082FFE8  bne 0x82514e38
	if !ctx.cr[0].eq {
	pc = 0x82514E38; continue 'dispatch;
	}
	// 82514E54: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82514E58: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82514E5C: 3B210058  addi r25, r1, 0x58
	ctx.r[25].s64 = ctx.r[1].s64 + 88;
	// 82514E60: 4BFFA6B9  bl 0x8250f518
	ctx.lr = 0x82514E64;
	sub_8250F518(ctx, base);
	// 82514E64: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82514E68: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82514E6C: 7F25CB78  mr r5, r25
	ctx.r[5].u64 = ctx.r[25].u64;
	// 82514E70: 7F46D378  mr r6, r26
	ctx.r[6].u64 = ctx.r[26].u64;
	// 82514E74: 7FE7FB78  mr r7, r31
	ctx.r[7].u64 = ctx.r[31].u64;
	// 82514E78: 4BFF8199  bl 0x8250d010
	ctx.lr = 0x82514E7C;
	sub_8250D010(ctx, base);
	// 82514E7C: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82514E80: 488DCE11  bl 0x82df1c90
	ctx.lr = 0x82514E84;
	sub_82DF1C90(ctx, base);
	// 82514E84: 48000024  b 0x82514ea8
	pc = 0x82514EA8; continue 'dispatch;
	// 82514E88: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82514E8C: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82514E90: 4BDD8BA9  bl 0x822eda38
	ctx.lr = 0x82514E94;
	sub_822EDA38(ctx, base);
	// 82514E94: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82514E98: 4BDAB169  bl 0x822c0000
	ctx.lr = 0x82514E9C;
	sub_822C0000(ctx, base);
	// 82514E9C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82514EA0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82514EA4: 4BFFBE9D  bl 0x82510d40
	ctx.lr = 0x82514EA8;
	sub_82510D40(ctx, base);
	// 82514EA8: 80610064  lwz r3, 0x64(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82514EAC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82514EB0: 419A0008  beq cr6, 0x82514eb8
	if ctx.cr[6].eq {
	pc = 0x82514EB8; continue 'dispatch;
	}
	// 82514EB4: 4BDAB9DD  bl 0x822c0890
	ctx.lr = 0x82514EB8;
	sub_822C0890(ctx, base);
	// 82514EB8: 3BDE0008  addi r30, r30, 8
	ctx.r[30].s64 = ctx.r[30].s64 + 8;
	// 82514EBC: 7F1EE040  cmplw cr6, r30, r28
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[28].u32, &mut ctx.xer);
	// 82514EC0: 409AFF3C  bne cr6, 0x82514dfc
	if !ctx.cr[6].eq {
	pc = 0x82514DFC; continue 'dispatch;
	}
	// 82514EC4: 897F00E1  lbz r11, 0xe1(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(225 as u32) ) } as u64;
	// 82514EC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82514ECC: 915F00EC  stw r10, 0xec(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(236 as u32), ctx.r[10].u32 ) };
	// 82514ED0: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82514ED4: 418200B0  beq 0x82514f84
	if ctx.cr[0].eq {
	pc = 0x82514F84; continue 'dispatch;
	}
	// 82514ED8: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82514EDC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82514EE0: 409A00A4  bne cr6, 0x82514f84
	if !ctx.cr[6].eq {
	pc = 0x82514F84; continue 'dispatch;
	}
	// 82514EE4: 3D608328  lis r11, -0x7cd8
	ctx.r[11].s64 = -2094530560;
	// 82514EE8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82514EEC: 808BE250  lwz r4, -0x1db0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-7600 as u32) ) } as u64;
	// 82514EF0: 488DEB19  bl 0x82df3a08
	ctx.lr = 0x82514EF4;
	sub_82DF3A08(ctx, base);
	// 82514EF4: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82514EF8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82514EFC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82514F00: 4BFF3881  bl 0x82508780
	ctx.lr = 0x82514F04;
	sub_82508780(ctx, base);
	// 82514F04: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82514F08: 488DE521  bl 0x82df3428
	ctx.lr = 0x82514F0C;
	sub_82DF3428(ctx, base);
	// 82514F0C: 3D608328  lis r11, -0x7cd8
	ctx.r[11].s64 = -2094530560;
	// 82514F10: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82514F14: 808BE268  lwz r4, -0x1d98(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-7576 as u32) ) } as u64;
	// 82514F18: 488DEAF1  bl 0x82df3a08
	ctx.lr = 0x82514F1C;
	sub_82DF3A08(ctx, base);
	// 82514F1C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82514F20: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82514F24: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82514F28: 4BFF3859  bl 0x82508780
	ctx.lr = 0x82514F2C;
	sub_82508780(ctx, base);
	// 82514F2C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82514F30: 488DE4F9  bl 0x82df3428
	ctx.lr = 0x82514F34;
	sub_82DF3428(ctx, base);
	// 82514F34: 3D608328  lis r11, -0x7cd8
	ctx.r[11].s64 = -2094530560;
	// 82514F38: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82514F3C: 808BE25C  lwz r4, -0x1da4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-7588 as u32) ) } as u64;
	// 82514F40: 488DEAC9  bl 0x82df3a08
	ctx.lr = 0x82514F44;
	sub_82DF3A08(ctx, base);
	// 82514F44: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82514F48: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82514F4C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82514F50: 4BFF3831  bl 0x82508780
	ctx.lr = 0x82514F54;
	sub_82508780(ctx, base);
	// 82514F54: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82514F58: 488DE4D1  bl 0x82df3428
	ctx.lr = 0x82514F5C;
	sub_82DF3428(ctx, base);
	// 82514F5C: 3D608328  lis r11, -0x7cd8
	ctx.r[11].s64 = -2094530560;
	// 82514F60: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82514F64: 808BE254  lwz r4, -0x1dac(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-7596 as u32) ) } as u64;
	// 82514F68: 488DEAA1  bl 0x82df3a08
	ctx.lr = 0x82514F6C;
	sub_82DF3A08(ctx, base);
	// 82514F6C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82514F70: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82514F74: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82514F78: 4BFF3809  bl 0x82508780
	ctx.lr = 0x82514F7C;
	sub_82508780(ctx, base);
	// 82514F7C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82514F80: 488DE4A9  bl 0x82df3428
	ctx.lr = 0x82514F84;
	sub_82DF3428(ctx, base);
	// 82514F84: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82514F88: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82514F8C: 388B1E6C  addi r4, r11, 0x1e6c
	ctx.r[4].s64 = ctx.r[11].s64 + 7788;
	// 82514F90: 488DEA79  bl 0x82df3a08
	ctx.lr = 0x82514F94;
	sub_82DF3A08(ctx, base);
	// 82514F94: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82514F98: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82514F9C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82514FA0: 38610078  addi r3, r1, 0x78
	ctx.r[3].s64 = ctx.r[1].s64 + 120;
	// 82514FA4: 4BFF553D  bl 0x8250a4e0
	ctx.lr = 0x82514FA8;
	sub_8250A4E0(ctx, base);
	// 82514FA8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82514FAC: 3BFF00F0  addi r31, r31, 0xf0
	ctx.r[31].s64 = ctx.r[31].s64 + 240;
	// 82514FB0: 388B0004  addi r4, r11, 4
	ctx.r[4].s64 = ctx.r[11].s64 + 4;
	// 82514FB4: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 82514FB8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82514FBC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82514FC0: 4BDAF4A1  bl 0x822c4460
	ctx.lr = 0x82514FC4;
	sub_822C4460(ctx, base);
	// 82514FC4: 8061007C  lwz r3, 0x7c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(124 as u32) ) } as u64;
	// 82514FC8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82514FCC: 419A0008  beq cr6, 0x82514fd4
	if ctx.cr[6].eq {
	pc = 0x82514FD4; continue 'dispatch;
	}
	// 82514FD0: 4BDAB8C1  bl 0x822c0890
	ctx.lr = 0x82514FD4;
	sub_822C0890(ctx, base);
	// 82514FD4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82514FD8: 488DE451  bl 0x82df3428
	ctx.lr = 0x82514FDC;
	sub_82DF3428(ctx, base);
	// 82514FDC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82514FE0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82514FE4: 419A0078  beq cr6, 0x8251505c
	if ctx.cr[6].eq {
	pc = 0x8251505C; continue 'dispatch;
	}
	// 82514FE8: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82514FEC: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82514FF0: 388B1E2C  addi r4, r11, 0x1e2c
	ctx.r[4].s64 = ctx.r[11].s64 + 7724;
	// 82514FF4: 488DEA15  bl 0x82df3a08
	ctx.lr = 0x82514FF8;
	sub_82DF3A08(ctx, base);
	// 82514FF8: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82514FFC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82515000: 388B1E18  addi r4, r11, 0x1e18
	ctx.r[4].s64 = ctx.r[11].s64 + 7704;
	// 82515004: 488DEA05  bl 0x82df3a08
	ctx.lr = 0x82515008;
	sub_82DF3A08(ctx, base);
	// 82515008: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 8251500C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82515010: 38AB7038  addi r5, r11, 0x7038
	ctx.r[5].s64 = ctx.r[11].s64 + 28728;
	// 82515014: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 82515018: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8251501C: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 82515020: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 82515024: 83FF0000  lwz r31, 0(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82515028: 4808E3F1  bl 0x825a3418
	ctx.lr = 0x8251502C;
	sub_825A3418(ctx, base);
	// 8251502C: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82515030: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82515034: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82515038: 4808DE09  bl 0x825a2e40
	ctx.lr = 0x8251503C;
	sub_825A2E40(ctx, base);
	// 8251503C: 386100B8  addi r3, r1, 0xb8
	ctx.r[3].s64 = ctx.r[1].s64 + 184;
	// 82515040: 488DE3E9  bl 0x82df3428
	ctx.lr = 0x82515044;
	sub_82DF3428(ctx, base);
	// 82515044: 38610098  addi r3, r1, 0x98
	ctx.r[3].s64 = ctx.r[1].s64 + 152;
	// 82515048: 4BDB3C71  bl 0x822c8cb8
	ctx.lr = 0x8251504C;
	sub_822C8CB8(ctx, base);
	// 8251504C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82515050: 488DE3D9  bl 0x82df3428
	ctx.lr = 0x82515054;
	sub_82DF3428(ctx, base);
	// 82515054: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82515058: 488DE3D1  bl 0x82df3428
	ctx.lr = 0x8251505C;
	sub_82DF3428(ctx, base);
	// 8251505C: 38210100  addi r1, r1, 0x100
	ctx.r[1].s64 = ctx.r[1].s64 + 256;
	// 82515060: 48C9314C  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82515068(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82515068 size=128
    let mut pc: u32 = 0x82515068;
    'dispatch: loop {
        match pc {
            0x82515068 => {
    //   block [0x82515068..0x825150E8)
	// 82515068: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251506C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82515070: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82515074: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82515078: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8251507C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82515080: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82515084: 396B1E94  addi r11, r11, 0x1e94
	ctx.r[11].s64 = ctx.r[11].s64 + 7828;
	// 82515088: 394A1E80  addi r10, r10, 0x1e80
	ctx.r[10].s64 = ctx.r[10].s64 + 7808;
	// 8251508C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82515090: 915F0028  stw r10, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[10].u32 ) };
	// 82515094: 807F00F4  lwz r3, 0xf4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(244 as u32) ) } as u64;
	// 82515098: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8251509C: 419A0008  beq cr6, 0x825150a4
	if ctx.cr[6].eq {
	pc = 0x825150A4; continue 'dispatch;
	}
	// 825150A0: 4BDAB7F1  bl 0x822c0890
	ctx.lr = 0x825150A4;
	sub_822C0890(ctx, base);
	// 825150A4: 807F00E8  lwz r3, 0xe8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(232 as u32) ) } as u64;
	// 825150A8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825150AC: 419A0008  beq cr6, 0x825150b4
	if ctx.cr[6].eq {
	pc = 0x825150B4; continue 'dispatch;
	}
	// 825150B0: 4BDAB7E1  bl 0x822c0890
	ctx.lr = 0x825150B4;
	sub_822C0890(ctx, base);
	// 825150B4: 807F00D8  lwz r3, 0xd8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(216 as u32) ) } as u64;
	// 825150B8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825150BC: 419A0008  beq cr6, 0x825150c4
	if ctx.cr[6].eq {
	pc = 0x825150C4; continue 'dispatch;
	}
	// 825150C0: 4BDAB7D1  bl 0x822c0890
	ctx.lr = 0x825150C4;
	sub_822C0890(ctx, base);
	// 825150C4: 387F00C4  addi r3, r31, 0xc4
	ctx.r[3].s64 = ctx.r[31].s64 + 196;
	// 825150C8: 4BF9C1D9  bl 0x824b12a0
	ctx.lr = 0x825150CC;
	sub_824B12A0(ctx, base);
	// 825150CC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825150D0: 4BFFC0C9  bl 0x82511198
	ctx.lr = 0x825150D4;
	sub_82511198(ctx, base);
	// 825150D4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825150D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825150DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825150E0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825150E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825150E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825150E8 size=8
    let mut pc: u32 = 0x825150E8;
    'dispatch: loop {
        match pc {
            0x825150E8 => {
    //   block [0x825150E8..0x825150F0)
	// 825150E8: 3863FFD8  addi r3, r3, -0x28
	ctx.r[3].s64 = ctx.r[3].s64 + -40;
	// 825150EC: 4800008C  b 0x82515178
	sub_82515178(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825150F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825150F0 size=132
    let mut pc: u32 = 0x825150F0;
    'dispatch: loop {
        match pc {
            0x825150F0 => {
    //   block [0x825150F0..0x82515174)
	// 825150F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825150F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825150F8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825150FC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82515100: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82515104: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82515108: 4BFFBFE9  bl 0x825110f0
	ctx.lr = 0x8251510C;
	sub_825110F0(ctx, base);
	// 8251510C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82515110: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82515114: 396B1E94  addi r11, r11, 0x1e94
	ctx.r[11].s64 = ctx.r[11].s64 + 7828;
	// 82515118: 394A1E80  addi r10, r10, 0x1e80
	ctx.r[10].s64 = ctx.r[10].s64 + 7808;
	// 8251511C: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82515120: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82515124: 915F0028  stw r10, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[10].u32 ) };
	// 82515128: 387F00DC  addi r3, r31, 0xdc
	ctx.r[3].s64 = ctx.r[31].s64 + 220;
	// 8251512C: 93DF00C8  stw r30, 0xc8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(200 as u32), ctx.r[30].u32 ) };
	// 82515130: 93DF00CC  stw r30, 0xcc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(204 as u32), ctx.r[30].u32 ) };
	// 82515134: 93DF00D0  stw r30, 0xd0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(208 as u32), ctx.r[30].u32 ) };
	// 82515138: 93DF00D4  stw r30, 0xd4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(212 as u32), ctx.r[30].u32 ) };
	// 8251513C: 93DF00D8  stw r30, 0xd8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(216 as u32), ctx.r[30].u32 ) };
	// 82515140: 48C86741  bl 0x8319b880
	ctx.lr = 0x82515144;
	sub_8319B880(ctx, base);
	// 82515144: 9BDF00E1  stb r30, 0xe1(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(225 as u32), ctx.r[30].u8 ) };
	// 82515148: 93DF00E4  stw r30, 0xe4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(228 as u32), ctx.r[30].u32 ) };
	// 8251514C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82515150: 93DF00E8  stw r30, 0xe8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(232 as u32), ctx.r[30].u32 ) };
	// 82515154: 93DF00F0  stw r30, 0xf0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(240 as u32), ctx.r[30].u32 ) };
	// 82515158: 93DF00F4  stw r30, 0xf4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(244 as u32), ctx.r[30].u32 ) };
	// 8251515C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82515160: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82515164: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82515168: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8251516C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82515170: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82515178(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82515178 size=76
    let mut pc: u32 = 0x82515178;
    'dispatch: loop {
        match pc {
            0x82515178 => {
    //   block [0x82515178..0x825151C4)
	// 82515178: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251517C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82515180: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82515184: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82515188: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251518C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82515190: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82515194: 4BFFFED5  bl 0x82515068
	ctx.lr = 0x82515198;
	sub_82515068(ctx, base);
	// 82515198: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8251519C: 4182000C  beq 0x825151a8
	if ctx.cr[0].eq {
	pc = 0x825151A8; continue 'dispatch;
	}
	// 825151A0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825151A4: 488DD235  bl 0x82df23d8
	ctx.lr = 0x825151A8;
	sub_82DF23D8(ctx, base);
	// 825151A8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825151AC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825151B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825151B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825151B8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825151BC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825151C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825151C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825151C8 size=396
    let mut pc: u32 = 0x825151C8;
    'dispatch: loop {
        match pc {
            0x825151C8 => {
    //   block [0x825151C8..0x82515354)
	// 825151C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825151CC: 48C92F9D  bl 0x831a8168
	ctx.lr = 0x825151D0;
	sub_831A8130(ctx, base);
	// 825151D0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825151D4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825151D8: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 825151DC: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 825151E0: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 825151E4: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 825151E8: 4BFFF3D1  bl 0x825145b8
	ctx.lr = 0x825151EC;
	sub_825145B8(ctx, base);
	// 825151EC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825151F0: 41820028  beq 0x82515218
	if ctx.cr[0].eq {
	pc = 0x82515218; continue 'dispatch;
	}
	// 825151F4: 396000C4  li r11, 0xc4
	ctx.r[11].s64 = 196;
	// 825151F8: 39200080  li r9, 0x80
	ctx.r[9].s64 = 128;
	// 825151FC: 997F0000  stb r11, 0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82515200: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82515204: 396000FF  li r11, 0xff
	ctx.r[11].s64 = 255;
	// 82515208: 993F0002  stb r9, 2(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(2 as u32), ctx.r[9].u8 ) };
	// 8251520C: 995F0001  stb r10, 1(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(1 as u32), ctx.r[10].u8 ) };
	// 82515210: 997F0003  stb r11, 3(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(3 as u32), ctx.r[11].u8 ) };
	// 82515214: 48000134  b 0x82515348
	pc = 0x82515348; continue 'dispatch;
	// 82515218: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 8251521C: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82515220: 9BA10050  stb r29, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[29].u8 ) };
	// 82515224: 38802007  li r4, 0x2007
	ctx.r[4].s64 = 8199;
	// 82515228: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8251522C: 4BF79E4D  bl 0x8248f078
	ctx.lr = 0x82515230;
	sub_8248F078(ctx, base);
	// 82515230: 89610050  lbz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82515234: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82515238: 41820014  beq 0x8251524c
	if ctx.cr[0].eq {
	pc = 0x8251524C; continue 'dispatch;
	}
	// 8251523C: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82515240: 816B9FC4  lwz r11, -0x603c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-24636 as u32) ) } as u64;
	// 82515244: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82515248: 48000100  b 0x82515348
	pc = 0x82515348; continue 'dispatch;
	// 8251524C: 9BA10050  stb r29, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[29].u8 ) };
	// 82515250: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82515254: 38802010  li r4, 0x2010
	ctx.r[4].s64 = 8208;
	// 82515258: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8251525C: 4BF79E1D  bl 0x8248f078
	ctx.lr = 0x82515260;
	sub_8248F078(ctx, base);
	// 82515260: 89610050  lbz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82515264: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82515268: 41820018  beq 0x82515280
	if ctx.cr[0].eq {
	pc = 0x82515280; continue 'dispatch;
	}
	// 8251526C: 396000FF  li r11, 0xff
	ctx.r[11].s64 = 255;
	// 82515270: 39400080  li r10, 0x80
	ctx.r[10].s64 = 128;
	// 82515274: 997F0000  stb r11, 0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82515278: 997F0002  stb r11, 2(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(2 as u32), ctx.r[11].u8 ) };
	// 8251527C: 4BFFFF90  b 0x8251520c
	pc = 0x8251520C; continue 'dispatch;
	// 82515280: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82515284: 4BFFBD5D  bl 0x82510fe0
	ctx.lr = 0x82515288;
	sub_82510FE0(ctx, base);
	// 82515288: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 8251528C: 3FC08335  lis r30, -0x7ccb
	ctx.r[30].s64 = -2093678592;
	// 82515290: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82515294: 816A7040  lwz r11, 0x7040(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(28736 as u32) ) } as u64;
	// 82515298: 556907FF  clrlwi. r9, r11, 0x1f
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8251529C: 40820048  bne 0x825152e4
	if !ctx.cr[0].eq {
	pc = 0x825152E4; continue 'dispatch;
	}
	// 825152A0: 616B0001  ori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 | 1;
	// 825152A4: 3D208335  lis r9, -0x7ccb
	ctx.r[9].s64 = -2093678592;
	// 825152A8: 916A7040  stw r11, 0x7040(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(28736 as u32), ctx.r[11].u32 ) };
	// 825152AC: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 825152B0: 8089675C  lwz r4, 0x675c(r9)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(26460 as u32) ) } as u64;
	// 825152B4: 4BDCFC7D  bl 0x822e4f30
	ctx.lr = 0x825152B8;
	sub_822E4F30(ctx, base);
	// 825152B8: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 825152BC: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 825152C0: 4BDCFC01  bl 0x822e4ec0
	ctx.lr = 0x825152C4;
	sub_822E4EC0(ctx, base);
	// 825152C4: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 825152C8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825152CC: E89C0000  ld r4, 0(r28)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) };
	// 825152D0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825152D4: E8630000  ld r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	// 825152D8: 4BF76FB1  bl 0x8248c288
	ctx.lr = 0x825152DC;
	sub_8248C288(ctx, base);
	// 825152DC: 907E703C  stw r3, 0x703c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(28732 as u32), ctx.r[3].u32 ) };
	// 825152E0: 48000008  b 0x825152e8
	pc = 0x825152E8; continue 'dispatch;
	// 825152E4: 807E703C  lwz r3, 0x703c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28732 as u32) ) } as u64;
	// 825152E8: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 825152EC: 816B66D8  lwz r11, 0x66d8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(26328 as u32) ) } as u64;
	// 825152F0: 7F1D5840  cmplw cr6, r29, r11
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[11].u32, &mut ctx.xer);
	// 825152F4: 409A0010  bne cr6, 0x82515304
	if !ctx.cr[6].eq {
	pc = 0x82515304; continue 'dispatch;
	}
	// 825152F8: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 825152FC: 816B9FBC  lwz r11, -0x6044(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-24644 as u32) ) } as u64;
	// 82515300: 4BFFFF44  b 0x82515244
	pc = 0x82515244; continue 'dispatch;
	// 82515304: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82515308: 4BF76811  bl 0x8248bb18
	ctx.lr = 0x8251530C;
	sub_8248BB18(ctx, base);
	// 8251530C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82515310: 41820010  beq 0x82515320
	if ctx.cr[0].eq {
	pc = 0x82515320; continue 'dispatch;
	}
	// 82515314: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82515318: 816B9FC0  lwz r11, -0x6040(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-24640 as u32) ) } as u64;
	// 8251531C: 4BFFFF28  b 0x82515244
	pc = 0x82515244; continue 'dispatch;
	// 82515320: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82515324: 39400080  li r10, 0x80
	ctx.r[10].s64 = 128;
	// 82515328: 392B9FB0  addi r9, r11, -0x6050
	ctx.r[9].s64 = ctx.r[11].s64 + -24656;
	// 8251532C: 995F0000  stb r10, 0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 82515330: 89690001  lbz r11, 1(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[9].u32.wrapping_add(1 as u32) ) } as u64;
	// 82515334: 89490002  lbz r10, 2(r9)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[9].u32.wrapping_add(2 as u32) ) } as u64;
	// 82515338: 89290003  lbz r9, 3(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[9].u32.wrapping_add(3 as u32) ) } as u64;
	// 8251533C: 997F0001  stb r11, 1(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(1 as u32), ctx.r[11].u8 ) };
	// 82515340: 995F0002  stb r10, 2(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(2 as u32), ctx.r[10].u8 ) };
	// 82515344: 993F0003  stb r9, 3(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(3 as u32), ctx.r[9].u8 ) };
	// 82515348: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8251534C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82515350: 48C92E68  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82515358(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82515358 size=1040
    let mut pc: u32 = 0x82515358;
    'dispatch: loop {
        match pc {
            0x82515358 => {
    //   block [0x82515358..0x82515768)
	// 82515358: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251535C: 48C92DE5  bl 0x831a8140
	ctx.lr = 0x82515360;
	sub_831A8130(ctx, base);
	// 82515360: DBC1FF78  stfd f30, -0x88(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-136 as u32), ctx.f[30].u64 ) };
	// 82515364: DBE1FF80  stfd f31, -0x80(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-128 as u32), ctx.f[31].u64 ) };
	// 82515368: 9421FD80  stwu r1, -0x280(r1)
	ea = ctx.r[1].u32.wrapping_add(-640 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251536C: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82515370: 897C00E1  lbz r11, 0xe1(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(225 as u32) ) } as u64;
	// 82515374: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82515378: 418203E0  beq 0x82515758
	if ctx.cr[0].eq {
	pc = 0x82515758; continue 'dispatch;
	}
	// 8251537C: 817C00D4  lwz r11, 0xd4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(212 as u32) ) } as u64;
	// 82515380: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82515384: 409A03D4  bne cr6, 0x82515758
	if !ctx.cr[6].eq {
	pc = 0x82515758; continue 'dispatch;
	}
	// 82515388: 817C00C8  lwz r11, 0xc8(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(200 as u32) ) } as u64;
	// 8251538C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82515390: 419A0010  beq cr6, 0x825153a0
	if ctx.cr[6].eq {
	pc = 0x825153A0; continue 'dispatch;
	}
	// 82515394: 815C00CC  lwz r10, 0xcc(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(204 as u32) ) } as u64;
	// 82515398: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 8251539C: 7D6B1E70  srawi r11, r11, 3
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 3) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 3) as i64;
	// 825153A0: 813C00EC  lwz r9, 0xec(r28)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(236 as u32) ) } as u64;
	// 825153A4: 7F095840  cmplw cr6, r9, r11
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[11].u32, &mut ctx.xer);
	// 825153A8: 409803B0  bge cr6, 0x82515758
	if !ctx.cr[6].lt {
	pc = 0x82515758; continue 'dispatch;
	}
	// 825153AC: 817C00C8  lwz r11, 0xc8(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(200 as u32) ) } as u64;
	// 825153B0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825153B4: 419A0010  beq cr6, 0x825153c4
	if ctx.cr[6].eq {
	pc = 0x825153C4; continue 'dispatch;
	}
	// 825153B8: 815C00CC  lwz r10, 0xcc(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(204 as u32) ) } as u64;
	// 825153BC: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 825153C0: 7D6B1E70  srawi r11, r11, 3
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 3) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 3) as i64;
	// 825153C4: 3949000A  addi r10, r9, 0xa
	ctx.r[10].s64 = ctx.r[9].s64 + 10;
	// 825153C8: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 825153CC: 7D565378  mr r22, r10
	ctx.r[22].u64 = ctx.r[10].u64;
	// 825153D0: 41980008  blt cr6, 0x825153d8
	if ctx.cr[6].lt {
	pc = 0x825153D8; continue 'dispatch;
	}
	// 825153D4: 7D765B78  mr r22, r11
	ctx.r[22].u64 = ctx.r[11].u64;
	// 825153D8: 7F09B040  cmplw cr6, r9, r22
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[22].u32, &mut ctx.xer);
	// 825153DC: 40980378  bge cr6, 0x82515754
	if !ctx.cr[6].lt {
	pc = 0x82515754; continue 'dispatch;
	}
	// 825153E0: 3D008200  lis r8, -0x7e00
	ctx.r[8].s64 = -2113929216;
	// 825153E4: 3CE08201  lis r7, -0x7dff
	ctx.r[7].s64 = -2113863680;
	// 825153E8: 553A1838  slwi r26, r9, 3
	ctx.r[26].u32 = ctx.r[9].u32.wrapping_shl(3);
	ctx.r[26].u64 = ctx.r[26].u32 as u64;
	// 825153EC: 7EE9B050  subf r23, r9, r22
	ctx.r[23].s64 = ctx.r[22].s64 - ctx.r[9].s64;
	// 825153F0: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 825153F4: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 825153F8: C3C808A4  lfs f30, 0x8a4(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(2212 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 825153FC: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82515400: C3E79524  lfs f31, -0x6adc(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(-27356 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82515404: 3B2BBE0C  addi r25, r11, -0x41f4
	ctx.r[25].s64 = ctx.r[11].s64 + -16884;
	// 82515408: 3B0AA658  addi r24, r10, -0x59a8
	ctx.r[24].s64 = ctx.r[10].s64 + -22952;
	// 8251540C: 3B691DD8  addi r27, r9, 0x1dd8
	ctx.r[27].s64 = ctx.r[9].s64 + 7640;
	// 82515410: 38610130  addi r3, r1, 0x130
	ctx.r[3].s64 = ctx.r[1].s64 + 304;
	// 82515414: 489B75D5  bl 0x82ecc9e8
	ctx.lr = 0x82515418;
	sub_82ECC9E8(ctx, base);
	// 82515418: 817C00C8  lwz r11, 0xc8(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(200 as u32) ) } as u64;
	// 8251541C: 38810130  addi r4, r1, 0x130
	ctx.r[4].s64 = ctx.r[1].s64 + 304;
	// 82515420: 7FCBD02E  lwzx r30, r11, r26
	ctx.r[30].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[26].u32)) } as u64;
	// 82515424: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82515428: 489B7161  bl 0x82ecc588
	ctx.lr = 0x8251542C;
	sub_82ECC588(ctx, base);
	// 8251542C: 388100F0  addi r4, r1, 0xf0
	ctx.r[4].s64 = ctx.r[1].s64 + 240;
	// 82515430: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82515434: 4BDD7B15  bl 0x822ecf48
	ctx.lr = 0x82515438;
	sub_822ECF48(ctx, base);
	// 82515438: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8251543C: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82515440: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82515444: 38A0023D  li r5, 0x23d
	ctx.r[5].s64 = 573;
	// 82515448: 386000E0  li r3, 0xe0
	ctx.r[3].s64 = 224;
	// 8251544C: 83AB0070  lwz r29, 0x70(r11)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(112 as u32) ) } as u64;
	// 82515450: 488DCF99  bl 0x82df23e8
	ctx.lr = 0x82515454;
	sub_82DF23E8(ctx, base);
	// 82515454: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82515458: 41820014  beq 0x8251546c
	if ctx.cr[0].eq {
	pc = 0x8251546C; continue 'dispatch;
	}
	// 8251545C: 388100F0  addi r4, r1, 0xf0
	ctx.r[4].s64 = ctx.r[1].s64 + 240;
	// 82515460: 488FDC91  bl 0x82e130f0
	ctx.lr = 0x82515464;
	sub_82E130F0(ctx, base);
	// 82515464: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82515468: 48000008  b 0x82515470
	pc = 0x82515470; continue 'dispatch;
	// 8251546C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82515470: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82515474: 3861006C  addi r3, r1, 0x6c
	ctx.r[3].s64 = ctx.r[1].s64 + 108;
	// 82515478: 488DE591  bl 0x82df3a08
	ctx.lr = 0x8251547C;
	sub_82DF3A08(ctx, base);
	// 8251547C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82515480: 386100A0  addi r3, r1, 0xa0
	ctx.r[3].s64 = ctx.r[1].s64 + 160;
	// 82515484: 488DE585  bl 0x82df3a08
	ctx.lr = 0x82515488;
	sub_82DF3A08(ctx, base);
	// 82515488: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 8251548C: 38610064  addi r3, r1, 0x64
	ctx.r[3].s64 = ctx.r[1].s64 + 100;
	// 82515490: 488DE579  bl 0x82df3a08
	ctx.lr = 0x82515494;
	sub_82DF3A08(ctx, base);
	// 82515494: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82515498: 38610098  addi r3, r1, 0x98
	ctx.r[3].s64 = ctx.r[1].s64 + 152;
	// 8251549C: 488DE56D  bl 0x82df3a08
	ctx.lr = 0x825154A0;
	sub_82DF3A08(ctx, base);
	// 825154A0: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 825154A4: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 825154A8: 488DE561  bl 0x82df3a08
	ctx.lr = 0x825154AC;
	sub_82DF3A08(ctx, base);
	// 825154AC: 93E10080  stw r31, 0x80(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[31].u32 ) };
	// 825154B0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825154B4: 38610084  addi r3, r1, 0x84
	ctx.r[3].s64 = ctx.r[1].s64 + 132;
	// 825154B8: 4BDCBB61  bl 0x822e1018
	ctx.lr = 0x825154BC;
	sub_822E1018(ctx, base);
	// 825154BC: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 825154C0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825154C4: 38610084  addi r3, r1, 0x84
	ctx.r[3].s64 = ctx.r[1].s64 + 132;
	// 825154C8: 4BDAAB39  bl 0x822c0000
	ctx.lr = 0x825154CC;
	sub_822C0000(ctx, base);
	// 825154CC: 80610134  lwz r3, 0x134(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(308 as u32) ) } as u64;
	// 825154D0: D3E100B0  stfs f31, 0xb0(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(176 as u32), tmp.u32 ) };
	// 825154D4: D3E100B4  stfs f31, 0xb4(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(180 as u32), tmp.u32 ) };
	// 825154D8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825154DC: D3C100B8  stfs f30, 0xb8(r1)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(184 as u32), tmp.u32 ) };
	// 825154E0: D3C100BC  stfs f30, 0xbc(r1)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(188 as u32), tmp.u32 ) };
	// 825154E4: 90610068  stw r3, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[3].u32 ) };
	// 825154E8: 419A0008  beq cr6, 0x825154f0
	if ctx.cr[6].eq {
	pc = 0x825154F0; continue 'dispatch;
	}
	// 825154EC: 4BDD2D5D  bl 0x822e8248
	ctx.lr = 0x825154F0;
	sub_822E8248(ctx, base);
	// 825154F0: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 825154F4: 386100D8  addi r3, r1, 0xd8
	ctx.r[3].s64 = ctx.r[1].s64 + 216;
	// 825154F8: 4BFF9E39  bl 0x8250f330
	ctx.lr = 0x825154FC;
	sub_8250F330(ctx, base);
	// 825154FC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82515500: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82515504: 386100C0  addi r3, r1, 0xc0
	ctx.r[3].s64 = ctx.r[1].s64 + 192;
	// 82515508: 83EB0000  lwz r31, 0(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251550C: 4BFF9FBD  bl 0x8250f4c8
	ctx.lr = 0x82515510;
	sub_8250F4C8(ctx, base);
	// 82515510: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82515514: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82515518: 386BFFFC  addi r3, r11, -4
	ctx.r[3].s64 = ctx.r[11].s64 + -4;
	// 8251551C: 409A0008  bne cr6, 0x82515524
	if !ctx.cr[6].eq {
	pc = 0x82515524; continue 'dispatch;
	}
	// 82515520: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82515524: 4BFF3005  bl 0x82508528
	ctx.lr = 0x82515528;
	sub_82508528(ctx, base);
	// 82515528: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8251552C: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82515530: 38A1006C  addi r5, r1, 0x6c
	ctx.r[5].s64 = ctx.r[1].s64 + 108;
	// 82515534: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82515538: 386100C8  addi r3, r1, 0xc8
	ctx.r[3].s64 = ctx.r[1].s64 + 200;
	// 8251553C: 83CB0000  lwz r30, 0(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82515540: 3BA10080  addi r29, r1, 0x80
	ctx.r[29].s64 = ctx.r[1].s64 + 128;
	// 82515544: 4BFFFC85  bl 0x825151c8
	ctx.lr = 0x82515548;
	sub_825151C8(ctx, base);
	// 82515548: 7C751B78  mr r21, r3
	ctx.r[21].u64 = ctx.r[3].u64;
	// 8251554C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82515550: 3BE100B0  addi r31, r1, 0xb0
	ctx.r[31].s64 = ctx.r[1].s64 + 176;
	// 82515554: 3A810064  addi r20, r1, 0x64
	ctx.r[20].s64 = ctx.r[1].s64 + 100;
	// 82515558: 3A610060  addi r19, r1, 0x60
	ctx.r[19].s64 = ctx.r[1].s64 + 96;
	// 8251555C: 3A410068  addi r18, r1, 0x68
	ctx.r[18].s64 = ctx.r[1].s64 + 104;
	// 82515560: 4BFD43B9  bl 0x824e9918
	ctx.lr = 0x82515564;
	sub_824E9918(ctx, base);
	// 82515564: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82515568: 93A10054  stw r29, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[29].u32 ) };
	// 8251556C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82515570: 38610078  addi r3, r1, 0x78
	ctx.r[3].s64 = ctx.r[1].s64 + 120;
	// 82515574: 7E469378  mr r6, r18
	ctx.r[6].u64 = ctx.r[18].u64;
	// 82515578: 7E679B78  mr r7, r19
	ctx.r[7].u64 = ctx.r[19].u64;
	// 8251557C: 7E88A378  mr r8, r20
	ctx.r[8].u64 = ctx.r[20].u64;
	// 82515580: 7FE9FB78  mr r9, r31
	ctx.r[9].u64 = ctx.r[31].u64;
	// 82515584: 7EAAAB78  mr r10, r21
	ctx.r[10].u64 = ctx.r[21].u64;
	// 82515588: 4BDDBC11  bl 0x822f1198
	ctx.lr = 0x8251558C;
	sub_822F1198(ctx, base);
	// 8251558C: 386100C0  addi r3, r1, 0xc0
	ctx.r[3].s64 = ctx.r[1].s64 + 192;
	// 82515590: 488DC701  bl 0x82df1c90
	ctx.lr = 0x82515594;
	sub_82DF1C90(ctx, base);
	// 82515594: 386100D8  addi r3, r1, 0xd8
	ctx.r[3].s64 = ctx.r[1].s64 + 216;
	// 82515598: 488DC6F9  bl 0x82df1c90
	ctx.lr = 0x8251559C;
	sub_82DF1C90(ctx, base);
	// 8251559C: 80610068  lwz r3, 0x68(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 825155A0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825155A4: 419A0008  beq cr6, 0x825155ac
	if ctx.cr[6].eq {
	pc = 0x825155AC; continue 'dispatch;
	}
	// 825155A8: 4BDD2CC1  bl 0x822e8268
	ctx.lr = 0x825155AC;
	sub_822E8268(ctx, base);
	// 825155AC: 80610084  lwz r3, 0x84(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 825155B0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825155B4: 419A0008  beq cr6, 0x825155bc
	if ctx.cr[6].eq {
	pc = 0x825155BC; continue 'dispatch;
	}
	// 825155B8: 4BDAB2D9  bl 0x822c0890
	ctx.lr = 0x825155BC;
	sub_822C0890(ctx, base);
	// 825155BC: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 825155C0: 488DDE69  bl 0x82df3428
	ctx.lr = 0x825155C4;
	sub_82DF3428(ctx, base);
	// 825155C4: 38610098  addi r3, r1, 0x98
	ctx.r[3].s64 = ctx.r[1].s64 + 152;
	// 825155C8: 488DDE61  bl 0x82df3428
	ctx.lr = 0x825155CC;
	sub_82DF3428(ctx, base);
	// 825155CC: 38610064  addi r3, r1, 0x64
	ctx.r[3].s64 = ctx.r[1].s64 + 100;
	// 825155D0: 488DDE59  bl 0x82df3428
	ctx.lr = 0x825155D4;
	sub_82DF3428(ctx, base);
	// 825155D4: 386100A0  addi r3, r1, 0xa0
	ctx.r[3].s64 = ctx.r[1].s64 + 160;
	// 825155D8: 488DDE51  bl 0x82df3428
	ctx.lr = 0x825155DC;
	sub_82DF3428(ctx, base);
	// 825155DC: 3861006C  addi r3, r1, 0x6c
	ctx.r[3].s64 = ctx.r[1].s64 + 108;
	// 825155E0: 488DDE49  bl 0x82df3428
	ctx.lr = 0x825155E4;
	sub_82DF3428(ctx, base);
	// 825155E4: 80610078  lwz r3, 0x78(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(120 as u32) ) } as u64;
	// 825155E8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825155EC: 419A014C  beq cr6, 0x82515738
	if ctx.cr[6].eq {
	pc = 0x82515738; continue 'dispatch;
	}
	// 825155F0: 38A1009C  addi r5, r1, 0x9c
	ctx.r[5].s64 = ctx.r[1].s64 + 156;
	// 825155F4: 388100E0  addi r4, r1, 0xe0
	ctx.r[4].s64 = ctx.r[1].s64 + 224;
	// 825155F8: 4BDDB889  bl 0x822f0e80
	ctx.lr = 0x825155FC;
	sub_822F0E80(ctx, base);
	// 825155FC: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82515600: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82515604: 38A00245  li r5, 0x245
	ctx.r[5].s64 = 581;
	// 82515608: 386000F0  li r3, 0xf0
	ctx.r[3].s64 = 240;
	// 8251560C: 488DCDDD  bl 0x82df23e8
	ctx.lr = 0x82515610;
	sub_82DF23E8(ctx, base);
	// 82515610: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82515614: 41820018  beq 0x8251562c
	if ctx.cr[0].eq {
	pc = 0x8251562C; continue 'dispatch;
	}
	// 82515618: 388100E0  addi r4, r1, 0xe0
	ctx.r[4].s64 = ctx.r[1].s64 + 224;
	// 8251561C: C021009C  lfs f1, 0x9c(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(156 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82515620: 482A0DF1  bl 0x827b6410
	ctx.lr = 0x82515624;
	sub_827B6410(ctx, base);
	// 82515624: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82515628: 48000008  b 0x82515630
	pc = 0x82515630; continue 'dispatch;
	// 8251562C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82515630: 93E10070  stw r31, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[31].u32 ) };
	// 82515634: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82515638: 38610074  addi r3, r1, 0x74
	ctx.r[3].s64 = ctx.r[1].s64 + 116;
	// 8251563C: 4BDC9355  bl 0x822de990
	ctx.lr = 0x82515640;
	sub_822DE990(ctx, base);
	// 82515640: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82515644: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82515648: 38610074  addi r3, r1, 0x74
	ctx.r[3].s64 = ctx.r[1].s64 + 116;
	// 8251564C: 4BDAA9B5  bl 0x822c0000
	ctx.lr = 0x82515650;
	sub_822C0000(ctx, base);
	// 82515650: 8161007C  lwz r11, 0x7c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(124 as u32) ) } as u64;
	// 82515654: 81410078  lwz r10, 0x78(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(120 as u32) ) } as u64;
	// 82515658: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8251565C: 91610094  stw r11, 0x94(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(148 as u32), ctx.r[11].u32 ) };
	// 82515660: 91410090  stw r10, 0x90(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(144 as u32), ctx.r[10].u32 ) };
	// 82515664: 419A0024  beq cr6, 0x82515688
	if ctx.cr[6].eq {
	pc = 0x82515688; continue 'dispatch;
	}
	// 82515668: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8251566C: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82515670: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82515674: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82515678: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8251567C: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82515680: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82515684: 4082FFE8  bne 0x8251566c
	if !ctx.cr[0].eq {
	pc = 0x8251566C; continue 'dispatch;
	}
	// 82515688: 83E10070  lwz r31, 0x70(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	// 8251568C: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82515690: 38A10090  addi r5, r1, 0x90
	ctx.r[5].s64 = ctx.r[1].s64 + 144;
	// 82515694: 809C00DC  lwz r4, 0xdc(r28)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(220 as u32) ) } as u64;
	// 82515698: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8251569C: 4BFFB35D  bl 0x825109f8
	ctx.lr = 0x825156A0;
	sub_825109F8(ctx, base);
	// 825156A0: 80610094  lwz r3, 0x94(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(148 as u32) ) } as u64;
	// 825156A4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825156A8: 419A0008  beq cr6, 0x825156b0
	if ctx.cr[6].eq {
	pc = 0x825156B0; continue 'dispatch;
	}
	// 825156AC: 4BDAB1E5  bl 0x822c0890
	ctx.lr = 0x825156B0;
	sub_822C0890(ctx, base);
	// 825156B0: 83C10074  lwz r30, 0x74(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 825156B4: 93E10088  stw r31, 0x88(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(136 as u32), ctx.r[31].u32 ) };
	// 825156B8: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 825156BC: 93C1008C  stw r30, 0x8c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(140 as u32), ctx.r[30].u32 ) };
	// 825156C0: 419A0024  beq cr6, 0x825156e4
	if ctx.cr[6].eq {
	pc = 0x825156E4; continue 'dispatch;
	}
	// 825156C4: 397E0004  addi r11, r30, 4
	ctx.r[11].s64 = ctx.r[30].s64 + 4;
	// 825156C8: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 825156CC: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 825156D0: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 825156D4: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 825156D8: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 825156DC: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 825156E0: 4082FFE8  bne 0x825156c8
	if !ctx.cr[0].eq {
	pc = 0x825156C8; continue 'dispatch;
	}
	// 825156E4: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 825156E8: 386100D0  addi r3, r1, 0xd0
	ctx.r[3].s64 = ctx.r[1].s64 + 208;
	// 825156EC: 4BFF9DDD  bl 0x8250f4c8
	ctx.lr = 0x825156F0;
	sub_8250F4C8(ctx, base);
	// 825156F0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825156F4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825156F8: 386BFFFC  addi r3, r11, -4
	ctx.r[3].s64 = ctx.r[11].s64 + -4;
	// 825156FC: 409A0008  bne cr6, 0x82515704
	if !ctx.cr[6].eq {
	pc = 0x82515704; continue 'dispatch;
	}
	// 82515700: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82515704: 38A10088  addi r5, r1, 0x88
	ctx.r[5].s64 = ctx.r[1].s64 + 136;
	// 82515708: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8251570C: 4BFF533D  bl 0x8250aa48
	ctx.lr = 0x82515710;
	sub_8250AA48(ctx, base);
	// 82515710: 386100D0  addi r3, r1, 0xd0
	ctx.r[3].s64 = ctx.r[1].s64 + 208;
	// 82515714: 488DC57D  bl 0x82df1c90
	ctx.lr = 0x82515718;
	sub_82DF1C90(ctx, base);
	// 82515718: 8061008C  lwz r3, 0x8c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(140 as u32) ) } as u64;
	// 8251571C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82515720: 419A0008  beq cr6, 0x82515728
	if ctx.cr[6].eq {
	pc = 0x82515728; continue 'dispatch;
	}
	// 82515724: 4BDAB16D  bl 0x822c0890
	ctx.lr = 0x82515728;
	sub_822C0890(ctx, base);
	// 82515728: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8251572C: 419A000C  beq cr6, 0x82515738
	if ctx.cr[6].eq {
	pc = 0x82515738; continue 'dispatch;
	}
	// 82515730: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82515734: 4BDAB15D  bl 0x822c0890
	ctx.lr = 0x82515738;
	sub_822C0890(ctx, base);
	// 82515738: 8061007C  lwz r3, 0x7c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(124 as u32) ) } as u64;
	// 8251573C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82515740: 419A0008  beq cr6, 0x82515748
	if ctx.cr[6].eq {
	pc = 0x82515748; continue 'dispatch;
	}
	// 82515744: 4BDAB14D  bl 0x822c0890
	ctx.lr = 0x82515748;
	sub_822C0890(ctx, base);
	// 82515748: 36F7FFFF  addic. r23, r23, -1
	ctx.xer.ca = (ctx.r[23].u32 > (!(-1 as u32)));
	ctx.r[23].s64 = ctx.r[23].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[23].s32, 0, &mut ctx.xer);
	// 8251574C: 3B5A0008  addi r26, r26, 8
	ctx.r[26].s64 = ctx.r[26].s64 + 8;
	// 82515750: 4082FCC0  bne 0x82515410
	if !ctx.cr[0].eq {
	pc = 0x82515410; continue 'dispatch;
	}
	// 82515754: 92DC00EC  stw r22, 0xec(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(236 as u32), ctx.r[22].u32 ) };
	// 82515758: 38210280  addi r1, r1, 0x280
	ctx.r[1].s64 = ctx.r[1].s64 + 640;
	// 8251575C: CBC1FF78  lfd f30, -0x88(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-136 as u32) ) };
	// 82515760: CBE1FF80  lfd f31, -0x80(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-128 as u32) ) };
	// 82515764: 48C92A2C  b 0x831a8190
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82515768(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82515768 size=1324
    let mut pc: u32 = 0x82515768;
    'dispatch: loop {
        match pc {
            0x82515768 => {
    //   block [0x82515768..0x82515C94)
	// 82515768: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251576C: 48C929E1  bl 0x831a814c
	ctx.lr = 0x82515770;
	sub_831A8130(ctx, base);
	// 82515770: DBE1FF98  stfd f31, -0x68(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-104 as u32), ctx.f[31].u64 ) };
	// 82515774: 3980FF80  li r12, -0x80
	ctx.r[12].s64 = -128;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82515C98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82515C98 size=332
    let mut pc: u32 = 0x82515C98;
    'dispatch: loop {
        match pc {
            0x82515C98 => {
    //   block [0x82515C98..0x82515DE4)
	// 82515C98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82515C9C: 48C924CD  bl 0x831a8168
	ctx.lr = 0x82515CA0;
	sub_831A8130(ctx, base);
	// 82515CA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82515CA4: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82515CA8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82515CAC: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82515CB0: 579D063F  clrlwi. r29, r28, 0x18
	ctx.r[29].u64 = ctx.r[28].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82515CB4: 41820038  beq 0x82515cec
	if ctx.cr[0].eq {
	pc = 0x82515CEC; continue 'dispatch;
	}
	// 82515CB8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82515CBC: 48C93CCD  bl 0x831a9988
	ctx.lr = 0x82515CC0;
	sub_831A9988(ctx, base);
	// 82515CC0: 3D608328  lis r11, -0x7cd8
	ctx.r[11].s64 = -2094530560;
	// 82515CC4: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82515CC8: 386BCC70  addi r3, r11, -0x3390
	ctx.r[3].s64 = ctx.r[11].s64 + -13200;
	// 82515CCC: 48C9242D  bl 0x831a80f8
	ctx.lr = 0x82515CD0;
	sub_831A80F8(ctx, base);
	// 82515CD0: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82515CD4: 41820018  beq 0x82515cec
	if ctx.cr[0].eq {
	pc = 0x82515CEC; continue 'dispatch;
	}
	// 82515CD8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82515CDC: 387EFFD8  addi r3, r30, -0x28
	ctx.r[3].s64 = ctx.r[30].s64 + -40;
	// 82515CE0: 4BFFE879  bl 0x82514558
	ctx.lr = 0x82515CE4;
	sub_82514558(ctx, base);
	// 82515CE4: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82515CE8: 480000F4  b 0x82515ddc
	pc = 0x82515DDC; continue 'dispatch;
	// 82515CEC: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82515CF0: 419A00DC  beq cr6, 0x82515dcc
	if ctx.cr[6].eq {
	pc = 0x82515DCC; continue 'dispatch;
	}
	// 82515CF4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82515CF8: 48C93C91  bl 0x831a9988
	ctx.lr = 0x82515CFC;
	sub_831A9988(ctx, base);
	// 82515CFC: 3D608328  lis r11, -0x7cd8
	ctx.r[11].s64 = -2094530560;
	// 82515D00: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82515D04: 386BCE78  addi r3, r11, -0x3188
	ctx.r[3].s64 = ctx.r[11].s64 + -12680;
	// 82515D08: 48C923F1  bl 0x831a80f8
	ctx.lr = 0x82515D0C;
	sub_831A80F8(ctx, base);
	// 82515D0C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82515D10: 41820014  beq 0x82515d24
	if ctx.cr[0].eq {
	pc = 0x82515D24; continue 'dispatch;
	}
	// 82515D14: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82515D18: 387EFFD8  addi r3, r30, -0x28
	ctx.r[3].s64 = ctx.r[30].s64 + -40;
	// 82515D1C: 4BFFEB1D  bl 0x82514838
	ctx.lr = 0x82515D20;
	sub_82514838(ctx, base);
	// 82515D20: 4BFFFFC4  b 0x82515ce4
	pc = 0x82515CE4; continue 'dispatch;
	// 82515D24: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82515D28: 419A00A4  beq cr6, 0x82515dcc
	if ctx.cr[6].eq {
	pc = 0x82515DCC; continue 'dispatch;
	}
	// 82515D2C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82515D30: 48C93C59  bl 0x831a9988
	ctx.lr = 0x82515D34;
	sub_831A9988(ctx, base);
	// 82515D34: 3D608328  lis r11, -0x7cd8
	ctx.r[11].s64 = -2094530560;
	// 82515D38: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82515D3C: 386BCE48  addi r3, r11, -0x31b8
	ctx.r[3].s64 = ctx.r[11].s64 + -12728;
	// 82515D40: 48C923B9  bl 0x831a80f8
	ctx.lr = 0x82515D44;
	sub_831A80F8(ctx, base);
	// 82515D44: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82515D48: 41820014  beq 0x82515d5c
	if ctx.cr[0].eq {
	pc = 0x82515D5C; continue 'dispatch;
	}
	// 82515D4C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82515D50: 387EFFD8  addi r3, r30, -0x28
	ctx.r[3].s64 = ctx.r[30].s64 + -40;
	// 82515D54: 4BFFEBAD  bl 0x82514900
	ctx.lr = 0x82515D58;
	sub_82514900(ctx, base);
	// 82515D58: 4BFFFF8C  b 0x82515ce4
	pc = 0x82515CE4; continue 'dispatch;
	// 82515D5C: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82515D60: 419A006C  beq cr6, 0x82515dcc
	if ctx.cr[6].eq {
	pc = 0x82515DCC; continue 'dispatch;
	}
	// 82515D64: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82515D68: 48C93C21  bl 0x831a9988
	ctx.lr = 0x82515D6C;
	sub_831A9988(ctx, base);
	// 82515D6C: 3D608328  lis r11, -0x7cd8
	ctx.r[11].s64 = -2094530560;
	// 82515D70: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82515D74: 386BCE1C  addi r3, r11, -0x31e4
	ctx.r[3].s64 = ctx.r[11].s64 + -12772;
	// 82515D78: 48C92381  bl 0x831a80f8
	ctx.lr = 0x82515D7C;
	sub_831A80F8(ctx, base);
	// 82515D7C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82515D80: 41820014  beq 0x82515d94
	if ctx.cr[0].eq {
	pc = 0x82515D94; continue 'dispatch;
	}
	// 82515D84: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82515D88: 387EFFD8  addi r3, r30, -0x28
	ctx.r[3].s64 = ctx.r[30].s64 + -40;
	// 82515D8C: 4BFFF9DD  bl 0x82515768
	ctx.lr = 0x82515D90;
	sub_82515768(ctx, base);
	// 82515D90: 4BFFFF54  b 0x82515ce4
	pc = 0x82515CE4; continue 'dispatch;
	// 82515D94: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82515D98: 419A0034  beq cr6, 0x82515dcc
	if ctx.cr[6].eq {
	pc = 0x82515DCC; continue 'dispatch;
	}
	// 82515D9C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82515DA0: 48C93BE9  bl 0x831a9988
	ctx.lr = 0x82515DA4;
	sub_831A9988(ctx, base);
	// 82515DA4: 3D608328  lis r11, -0x7cd8
	ctx.r[11].s64 = -2094530560;
	// 82515DA8: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82515DAC: 386BCDF0  addi r3, r11, -0x3210
	ctx.r[3].s64 = ctx.r[11].s64 + -12816;
	// 82515DB0: 48C92349  bl 0x831a80f8
	ctx.lr = 0x82515DB4;
	sub_831A80F8(ctx, base);
	// 82515DB4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82515DB8: 41820014  beq 0x82515dcc
	if ctx.cr[0].eq {
	pc = 0x82515DCC; continue 'dispatch;
	}
	// 82515DBC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82515DC0: 387EFFD8  addi r3, r30, -0x28
	ctx.r[3].s64 = ctx.r[30].s64 + -40;
	// 82515DC4: 4BDAA23D  bl 0x822c0000
	ctx.lr = 0x82515DC8;
	sub_822C0000(ctx, base);
	// 82515DC8: 4BFFFF1C  b 0x82515ce4
	pc = 0x82515CE4; continue 'dispatch;
	// 82515DCC: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82515DD0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82515DD4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82515DD8: 4BFFB7C1  bl 0x82511598
	ctx.lr = 0x82515DDC;
	sub_82511598(ctx, base);
	// 82515DDC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82515DE0: 48C923D8  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82515DE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82515DE8 size=84
    let mut pc: u32 = 0x82515DE8;
    'dispatch: loop {
        match pc {
            0x82515DE8 => {
    //   block [0x82515DE8..0x82515E3C)
	// 82515DE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82515DEC: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82515DF0: 81430004  lwz r10, 4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82515DF4: 91440004  stw r10, 4(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82515DF8: 89430014  lbz r10, 0x14(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 82515DFC: 99440014  stb r10, 0x14(r4)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[4].u32.wrapping_add(20 as u32), ctx.r[10].u8 ) };
	// 82515E00: 89430015  lbz r10, 0x15(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(21 as u32) ) } as u64;
	// 82515E04: 99440015  stb r10, 0x15(r4)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[4].u32.wrapping_add(21 as u32), ctx.r[10].u8 ) };
	// 82515E08: 81430008  lwz r10, 8(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82515E0C: 91440008  stw r10, 8(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82515E10: 8143000C  lwz r10, 0xc(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82515E14: 9144000C  stw r10, 0xc(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 82515E18: 91640010  stw r11, 0x10(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82515E1C: 81430018  lwz r10, 0x18(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 82515E20: 91440018  stw r10, 0x18(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(24 as u32), ctx.r[10].u32 ) };
	// 82515E24: 8143001C  lwz r10, 0x1c(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 82515E28: 9144001C  stw r10, 0x1c(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(28 as u32), ctx.r[10].u32 ) };
	// 82515E2C: 91640020  stw r11, 0x20(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82515E30: 81630024  lwz r11, 0x24(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(36 as u32) ) } as u64;
	// 82515E34: 91640024  stw r11, 0x24(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 82515E38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82515E40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82515E40 size=8
    let mut pc: u32 = 0x82515E40;
    'dispatch: loop {
        match pc {
            0x82515E40 => {
    //   block [0x82515E40..0x82515E48)
	// 82515E40: 38600040  li r3, 0x40
	ctx.r[3].s64 = 64;
	// 82515E44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82515E48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82515E48 size=124
    let mut pc: u32 = 0x82515E48;
    'dispatch: loop {
        match pc {
            0x82515E48 => {
    //   block [0x82515E48..0x82515EC4)
	// 82515E48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82515E4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82515E50: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82515E54: 486B72CD  bl 0x82bcd120
	ctx.lr = 0x82515E58;
	sub_82BCD120(ctx, base);
	// 82515E58: 2B030001  cmplwi cr6, r3, 1
	ctx.cr[6].compare_u32(ctx.r[3].u32, 1 as u32, &mut ctx.xer);
	// 82515E5C: 419A0054  beq cr6, 0x82515eb0
	if ctx.cr[6].eq {
	pc = 0x82515EB0; continue 'dispatch;
	}
	// 82515E60: 2B030002  cmplwi cr6, r3, 2
	ctx.cr[6].compare_u32(ctx.r[3].u32, 2 as u32, &mut ctx.xer);
	// 82515E64: 419A0044  beq cr6, 0x82515ea8
	if ctx.cr[6].eq {
	pc = 0x82515EA8; continue 'dispatch;
	}
	// 82515E68: 2B030003  cmplwi cr6, r3, 3
	ctx.cr[6].compare_u32(ctx.r[3].u32, 3 as u32, &mut ctx.xer);
	// 82515E6C: 419A0034  beq cr6, 0x82515ea0
	if ctx.cr[6].eq {
	pc = 0x82515EA0; continue 'dispatch;
	}
	// 82515E70: 2B030004  cmplwi cr6, r3, 4
	ctx.cr[6].compare_u32(ctx.r[3].u32, 4 as u32, &mut ctx.xer);
	// 82515E74: 419A0024  beq cr6, 0x82515e98
	if ctx.cr[6].eq {
	pc = 0x82515E98; continue 'dispatch;
	}
	// 82515E78: 2B030005  cmplwi cr6, r3, 5
	ctx.cr[6].compare_u32(ctx.r[3].u32, 5 as u32, &mut ctx.xer);
	// 82515E7C: 419A0014  beq cr6, 0x82515e90
	if ctx.cr[6].eq {
	pc = 0x82515E90; continue 'dispatch;
	}
	// 82515E80: 3963FFFA  addi r11, r3, -6
	ctx.r[11].s64 = ctx.r[3].s64 + -6;
	// 82515E84: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82515E88: 5563EF7A  rlwinm r3, r11, 0x1d, 0x1d, 0x1d
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x00000007u64;
	// 82515E8C: 48000028  b 0x82515eb4
	pc = 0x82515EB4; continue 'dispatch;
	// 82515E90: 38600005  li r3, 5
	ctx.r[3].s64 = 5;
	// 82515E94: 48000020  b 0x82515eb4
	pc = 0x82515EB4; continue 'dispatch;
	// 82515E98: 38600003  li r3, 3
	ctx.r[3].s64 = 3;
	// 82515E9C: 48000018  b 0x82515eb4
	pc = 0x82515EB4; continue 'dispatch;
	// 82515EA0: 38600002  li r3, 2
	ctx.r[3].s64 = 2;
	// 82515EA4: 48000010  b 0x82515eb4
	pc = 0x82515EB4; continue 'dispatch;
	// 82515EA8: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82515EAC: 48000008  b 0x82515eb4
	pc = 0x82515EB4; continue 'dispatch;
	// 82515EB0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82515EB4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82515EB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82515EBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82515EC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82515EC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82515EC8 size=128
    let mut pc: u32 = 0x82515EC8;
    'dispatch: loop {
        match pc {
            0x82515EC8 => {
    //   block [0x82515EC8..0x82515F48)
	// 82515EC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82515ECC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82515ED0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82515ED4: 48D2C6D9  bl 0x832425ac
	ctx.lr = 0x82515ED8;
	// extern call 0x832425AC → crate::xboxkrnl::XGetGameRegion
	crate::xboxkrnl::XGetGameRegion(ctx, base);
	// 82515ED8: 2B0301FF  cmplwi cr6, r3, 0x1ff
	ctx.cr[6].compare_u32(ctx.r[3].u32, 511 as u32, &mut ctx.xer);
	// 82515EDC: 41990030  bgt cr6, 0x82515f0c
	if ctx.cr[6].gt {
	pc = 0x82515F0C; continue 'dispatch;
	}
	// 82515EE0: 419A0024  beq cr6, 0x82515f04
	if ctx.cr[6].eq {
	pc = 0x82515F04; continue 'dispatch;
	}
	// 82515EE4: 2B0300FF  cmplwi cr6, r3, 0xff
	ctx.cr[6].compare_u32(ctx.r[3].u32, 255 as u32, &mut ctx.xer);
	// 82515EE8: 419A0044  beq cr6, 0x82515f2c
	if ctx.cr[6].eq {
	pc = 0x82515F2C; continue 'dispatch;
	}
	// 82515EEC: 2B030100  cmplwi cr6, r3, 0x100
	ctx.cr[6].compare_u32(ctx.r[3].u32, 256 as u32, &mut ctx.xer);
	// 82515EF0: 40990044  ble cr6, 0x82515f34
	if !ctx.cr[6].gt {
	pc = 0x82515F34; continue 'dispatch;
	}
	// 82515EF4: 2B030102  cmplwi cr6, r3, 0x102
	ctx.cr[6].compare_u32(ctx.r[3].u32, 258 as u32, &mut ctx.xer);
	// 82515EF8: 4099000C  ble cr6, 0x82515f04
	if !ctx.cr[6].gt {
	pc = 0x82515F04; continue 'dispatch;
	}
	// 82515EFC: 2B0301FC  cmplwi cr6, r3, 0x1fc
	ctx.cr[6].compare_u32(ctx.r[3].u32, 508 as u32, &mut ctx.xer);
	// 82515F00: 409A0034  bne cr6, 0x82515f34
	if !ctx.cr[6].eq {
	pc = 0x82515F34; continue 'dispatch;
	}
	// 82515F04: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82515F08: 48000030  b 0x82515f38
	pc = 0x82515F38; continue 'dispatch;
	// 82515F0C: 2B030201  cmplwi cr6, r3, 0x201
	ctx.cr[6].compare_u32(ctx.r[3].u32, 513 as u32, &mut ctx.xer);
	// 82515F10: 419A0024  beq cr6, 0x82515f34
	if ctx.cr[6].eq {
	pc = 0x82515F34; continue 'dispatch;
	}
	// 82515F14: 2B0302FD  cmplwi cr6, r3, 0x2fd
	ctx.cr[6].compare_u32(ctx.r[3].u32, 765 as u32, &mut ctx.xer);
	// 82515F18: 4099001C  ble cr6, 0x82515f34
	if !ctx.cr[6].gt {
	pc = 0x82515F34; continue 'dispatch;
	}
	// 82515F1C: 2B0302FF  cmplwi cr6, r3, 0x2ff
	ctx.cr[6].compare_u32(ctx.r[3].u32, 767 as u32, &mut ctx.xer);
	// 82515F20: 40990014  ble cr6, 0x82515f34
	if !ctx.cr[6].gt {
	pc = 0x82515F34; continue 'dispatch;
	}
	// 82515F24: 2B0303FF  cmplwi cr6, r3, 0x3ff
	ctx.cr[6].compare_u32(ctx.r[3].u32, 1023 as u32, &mut ctx.xer);
	// 82515F28: 409A000C  bne cr6, 0x82515f34
	if !ctx.cr[6].eq {
	pc = 0x82515F34; continue 'dispatch;
	}
	// 82515F2C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82515F30: 48000008  b 0x82515f38
	pc = 0x82515F38; continue 'dispatch;
	// 82515F34: 38600002  li r3, 2
	ctx.r[3].s64 = 2;
	// 82515F38: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82515F3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82515F40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82515F44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82515F48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82515F48 size=140
    let mut pc: u32 = 0x82515F48;
    'dispatch: loop {
        match pc {
            0x82515F48 => {
    //   block [0x82515F48..0x82515FD4)
	// 82515F48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82515F4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82515F50: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82515F54: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82515F58: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82515F5C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82515F60: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82515F64: 4BFFFEE5  bl 0x82515e48
	ctx.lr = 0x82515F68;
	sub_82515E48(ctx, base);
	// 82515F68: 907F0000  stw r3, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82515F6C: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82515F70: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82515F74: 897E0014  lbz r11, 0x14(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82515F78: 997F0014  stb r11, 0x14(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u8 ) };
	// 82515F7C: 897E0015  lbz r11, 0x15(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(21 as u32) ) } as u64;
	// 82515F80: 997F0015  stb r11, 0x15(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(21 as u32), ctx.r[11].u8 ) };
	// 82515F84: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82515F88: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82515F8C: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82515F90: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82515F94: 4BFFFF35  bl 0x82515ec8
	ctx.lr = 0x82515F98;
	sub_82515EC8(ctx, base);
	// 82515F98: 907F0010  stw r3, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[3].u32 ) };
	// 82515F9C: 817E0018  lwz r11, 0x18(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 82515FA0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82515FA4: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82515FA8: 817E001C  lwz r11, 0x1c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 82515FAC: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82515FB0: 915F0020  stw r10, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 82515FB4: 817E0024  lwz r11, 0x24(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 82515FB8: 917F0024  stw r11, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 82515FBC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82515FC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82515FC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82515FC8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82515FCC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82515FD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82515FD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82515FD8 size=80
    let mut pc: u32 = 0x82515FD8;
    'dispatch: loop {
        match pc {
            0x82515FD8 => {
    //   block [0x82515FD8..0x82516028)
	// 82515FD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82515FDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82515FE0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82515FE4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82515FE8: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 82515FEC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82515FF0: 816B1654  lwz r11, 0x1654(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(5716 as u32) ) } as u64;
	// 82515FF4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82515FF8: 388B0004  addi r4, r11, 4
	ctx.r[4].s64 = ctx.r[11].s64 + 4;
	// 82515FFC: 409A0008  bne cr6, 0x82516004
	if !ctx.cr[6].eq {
	pc = 0x82516004; continue 'dispatch;
	}
	// 82516000: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82516004: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82516008: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8251600C: 488DBBED  bl 0x82df1bf8
	ctx.lr = 0x82516010;
	sub_82DF1BF8(ctx, base);
	// 82516010: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82516014: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82516018: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8251601C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82516020: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82516024: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82516028(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82516028 size=180
    let mut pc: u32 = 0x82516028;
    'dispatch: loop {
        match pc {
            0x82516028 => {
    //   block [0x82516028..0x825160DC)
	// 82516028: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251602C: 48C92141  bl 0x831a816c
	ctx.lr = 0x82516030;
	sub_831A8130(ctx, base);
	// 82516030: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82516034: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82516038: 4BFFFE11  bl 0x82515e48
	ctx.lr = 0x8251603C;
	sub_82515E48(ctx, base);
	// 8251603C: 3943FFFF  addi r10, r3, -1
	ctx.r[10].s64 = ctx.r[3].s64 + -1;
	// 82516040: 907F0000  stw r3, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82516044: 3960000A  li r11, 0xa
	ctx.r[11].s64 = 10;
	// 82516048: 7D4A0034  cntlzw r10, r10
	ctx.r[10].u64 = if ctx.r[10].u32 == 0 { 32 } else { ctx.r[10].u32.leading_zeros() as u64 };
	// 8251604C: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82516050: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82516054: 554BDFFE  rlwinm r11, r10, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0x0000001Fu64;
	// 82516058: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8251605C: 4BFFFE6D  bl 0x82515ec8
	ctx.lr = 0x82516060;
	sub_82515EC8(ctx, base);
	// 82516060: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82516064: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82516068: 907F0010  stw r3, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[3].u32 ) };
	// 8251606C: 997F0014  stb r11, 0x14(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u8 ) };
	// 82516070: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82516074: 997F0015  stb r11, 0x15(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(21 as u32), ctx.r[11].u8 ) };
	// 82516078: 93DF0018  stw r30, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[30].u32 ) };
	// 8251607C: 4BFFFF5D  bl 0x82515fd8
	ctx.lr = 0x82516080;
	sub_82515FD8(ctx, base);
	// 82516080: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82516084: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82516088: 386BFFFC  addi r3, r11, -4
	ctx.r[3].s64 = ctx.r[11].s64 + -4;
	// 8251608C: 409A0008  bne cr6, 0x82516094
	if !ctx.cr[6].eq {
	pc = 0x82516094; continue 'dispatch;
	}
	// 82516090: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82516094: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82516098: 38800005  li r4, 5
	ctx.r[4].s64 = 5;
	// 8251609C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 825160A0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825160A4: 4E800421  bctrl
	ctx.lr = 0x825160A8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825160A8: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 825160AC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825160B0: 488DBBE1  bl 0x82df1c90
	ctx.lr = 0x825160B4;
	sub_82DF1C90(ctx, base);
	// 825160B4: 7FAB0034  cntlzw r11, r29
	ctx.r[11].u64 = if ctx.r[29].u32 == 0 { 32 } else { ctx.r[29].u32.leading_zeros() as u64 };
	// 825160B8: 39400005  li r10, 5
	ctx.r[10].s64 = 5;
	// 825160BC: 93DF0020  stw r30, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[30].u32 ) };
	// 825160C0: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 825160C4: 915F0024  stw r10, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[10].u32 ) };
	// 825160C8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825160CC: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 825160D0: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 825160D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825160D8: 48C920E4  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825160E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825160E0 size=176
    let mut pc: u32 = 0x825160E0;
    'dispatch: loop {
        match pc {
            0x825160E0 => {
    //   block [0x825160E0..0x82516190)
	// 825160E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825160E4: 48C92089  bl 0x831a816c
	ctx.lr = 0x825160E8;
	sub_831A8130(ctx, base);
	// 825160E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825160EC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825160F0: 4BFFFD59  bl 0x82515e48
	ctx.lr = 0x825160F4;
	sub_82515E48(ctx, base);
	// 825160F4: 3923FFFF  addi r9, r3, -1
	ctx.r[9].s64 = ctx.r[3].s64 + -1;
	// 825160F8: 907F0000  stw r3, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 825160FC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82516100: 7D290034  cntlzw r9, r9
	ctx.r[9].u64 = if ctx.r[9].u32 == 0 { 32 } else { ctx.r[9].u32.leading_zeros() as u64 };
	// 82516104: 995F0014  stb r10, 0x14(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[10].u8 ) };
	// 82516108: 3960000A  li r11, 0xa
	ctx.r[11].s64 = 10;
	// 8251610C: 995F0015  stb r10, 0x15(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(21 as u32), ctx.r[10].u8 ) };
	// 82516110: 39400005  li r10, 5
	ctx.r[10].s64 = 5;
	// 82516114: 5529DFFE  rlwinm r9, r9, 0x1b, 0x1f, 0x1f
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0x0000001Fu64;
	// 82516118: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8251611C: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82516120: 915F0024  stw r10, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[10].u32 ) };
	// 82516124: 913F0004  stw r9, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82516128: 4BFFFDA1  bl 0x82515ec8
	ctx.lr = 0x8251612C;
	sub_82515EC8(ctx, base);
	// 8251612C: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82516130: 907F0010  stw r3, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[3].u32 ) };
	// 82516134: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82516138: 93DF0018  stw r30, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[30].u32 ) };
	// 8251613C: 4BFFFE9D  bl 0x82515fd8
	ctx.lr = 0x82516140;
	sub_82515FD8(ctx, base);
	// 82516140: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82516144: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82516148: 386BFFFC  addi r3, r11, -4
	ctx.r[3].s64 = ctx.r[11].s64 + -4;
	// 8251614C: 409A0008  bne cr6, 0x82516154
	if !ctx.cr[6].eq {
	pc = 0x82516154; continue 'dispatch;
	}
	// 82516150: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82516154: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82516158: 38800005  li r4, 5
	ctx.r[4].s64 = 5;
	// 8251615C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82516160: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82516164: 4E800421  bctrl
	ctx.lr = 0x82516168;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82516168: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8251616C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82516170: 488DBB21  bl 0x82df1c90
	ctx.lr = 0x82516174;
	sub_82DF1C90(ctx, base);
	// 82516174: 7FAB0034  cntlzw r11, r29
	ctx.r[11].u64 = if ctx.r[29].u32 == 0 { 32 } else { ctx.r[29].u32.leading_zeros() as u64 };
	// 82516178: 93DF0020  stw r30, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[30].u32 ) };
	// 8251617C: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82516180: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 82516184: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82516188: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8251618C: 48C92030  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82516190(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82516190 size=12
    let mut pc: u32 = 0x82516190;
    'dispatch: loop {
        match pc {
            0x82516190 => {
    //   block [0x82516190..0x8251619C)
	// 82516190: 3C600000  lis r3, 0
	ctx.r[3].s64 = 0;
	// 82516194: 6063877C  ori r3, r3, 0x877c
	ctx.r[3].u64 = ctx.r[3].u64 | 34684;
	// 82516198: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825161A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825161A0 size=8
    let mut pc: u32 = 0x825161A0;
    'dispatch: loop {
        match pc {
            0x825161A0 => {
    //   block [0x825161A0..0x825161A8)
	// 825161A0: 80630094  lwz r3, 0x94(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(148 as u32) ) } as u64;
	// 825161A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825161A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825161A8 size=24
    let mut pc: u32 = 0x825161A8;
    'dispatch: loop {
        match pc {
            0x825161A8 => {
    //   block [0x825161A8..0x825161C0)
	// 825161A8: 3D400000  lis r10, 0
	ctx.r[10].s64 = 0;
	// 825161AC: 81630094  lwz r11, 0x94(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(148 as u32) ) } as u64;
	// 825161B0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 825161B4: 614A85EC  ori r10, r10, 0x85ec
	ctx.r[10].u64 = ctx.r[10].u64 | 34284;
	// 825161B8: 7D2B512E  stwx r9, r11, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[9].u32) };
	// 825161BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825161C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825161C0 size=52
    let mut pc: u32 = 0x825161C0;
    'dispatch: loop {
        match pc {
            0x825161C0 => {
    //   block [0x825161C0..0x825161F4)
	// 825161C0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 825161C4: 2B050002  cmplwi cr6, r5, 2
	ctx.cr[6].compare_u32(ctx.r[5].u32, 2 as u32, &mut ctx.xer);
	// 825161C8: 419A0054  beq cr6, 0x8251621c
	if ctx.cr[6].eq {
		sub_8251621C(ctx, base);
		return;
	}
	// 825161CC: 2B050003  cmplwi cr6, r5, 3
	ctx.cr[6].compare_u32(ctx.r[5].u32, 3 as u32, &mut ctx.xer);
	// 825161D0: 419A0044  beq cr6, 0x82516214
	if ctx.cr[6].eq {
		sub_82516214(ctx, base);
		return;
	}
	// 825161D4: 2B050004  cmplwi cr6, r5, 4
	ctx.cr[6].compare_u32(ctx.r[5].u32, 4 as u32, &mut ctx.xer);
	// 825161D8: 419A0034  beq cr6, 0x8251620c
	if ctx.cr[6].eq {
		sub_8251620C(ctx, base);
		return;
	}
	// 825161DC: 2B050005  cmplwi cr6, r5, 5
	ctx.cr[6].compare_u32(ctx.r[5].u32, 5 as u32, &mut ctx.xer);
	// 825161E0: 419A0024  beq cr6, 0x82516204
	if ctx.cr[6].eq {
		sub_82516204(ctx, base);
		return;
	}
	// 825161E4: 2B050006  cmplwi cr6, r5, 6
	ctx.cr[6].compare_u32(ctx.r[5].u32, 6 as u32, &mut ctx.xer);
	// 825161E8: 419A0014  beq cr6, 0x825161fc
	if ctx.cr[6].eq {
		sub_825161FC(ctx, base);
		return;
	}
	// 825161EC: 2B050007  cmplwi cr6, r5, 7
	ctx.cr[6].compare_u32(ctx.r[5].u32, 7 as u32, &mut ctx.xer);
	// 825161F0: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825161F4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825161F4 size=8
    let mut pc: u32 = 0x825161F4;
    'dispatch: loop {
        match pc {
            0x825161F4 => {
    //   block [0x825161F4..0x825161FC)
	// 825161F4: 38600078  li r3, 0x78
	ctx.r[3].s64 = 120;
	// 825161F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825161FC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825161FC size=8
    let mut pc: u32 = 0x825161FC;
    'dispatch: loop {
        match pc {
            0x825161FC => {
    //   block [0x825161FC..0x82516204)
	// 825161FC: 38600050  li r3, 0x50
	ctx.r[3].s64 = 80;
	// 82516200: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82516204(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82516204 size=8
    let mut pc: u32 = 0x82516204;
    'dispatch: loop {
        match pc {
            0x82516204 => {
    //   block [0x82516204..0x8251620C)
	// 82516204: 3860003C  li r3, 0x3c
	ctx.r[3].s64 = 60;
	// 82516208: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251620C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8251620C size=8
    let mut pc: u32 = 0x8251620C;
    'dispatch: loop {
        match pc {
            0x8251620C => {
    //   block [0x8251620C..0x82516214)
	// 8251620C: 3860002D  li r3, 0x2d
	ctx.r[3].s64 = 45;
	// 82516210: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82516214(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82516214 size=8
    let mut pc: u32 = 0x82516214;
    'dispatch: loop {
        match pc {
            0x82516214 => {
    //   block [0x82516214..0x8251621C)
	// 82516214: 3860001E  li r3, 0x1e
	ctx.r[3].s64 = 30;
	// 82516218: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251621C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8251621C size=8
    let mut pc: u32 = 0x8251621C;
    'dispatch: loop {
        match pc {
            0x8251621C => {
    //   block [0x8251621C..0x82516224)
	// 8251621C: 3860000F  li r3, 0xf
	ctx.r[3].s64 = 15;
	// 82516220: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82516228(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82516228 size=20
    let mut pc: u32 = 0x82516228;
    'dispatch: loop {
        match pc {
            0x82516228 => {
    //   block [0x82516228..0x8251623C)
	// 82516228: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8251622C: 814B0098  lwz r10, 0x98(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(152 as u32) ) } as u64;
	// 82516230: 886A0020  lbz r3, 0x20(r10)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(32 as u32) ) } as u64;
	// 82516234: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82516238: 4C820020  bnelr
	if !ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251623C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8251623C size=20
    let mut pc: u32 = 0x8251623C;
    'dispatch: loop {
        match pc {
            0x8251623C => {
    //   block [0x8251623C..0x82516250)
	// 8251623C: 3D400000  lis r10, 0
	ctx.r[10].s64 = 0;
	// 82516240: 816B0094  lwz r11, 0x94(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(148 as u32) ) } as u64;
	// 82516244: 614A8000  ori r10, r10, 0x8000
	ctx.r[10].u64 = ctx.r[10].u64 | 32768;
	// 82516248: 7C6B502E  lwzx r3, r11, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 8251624C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82516250(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82516250 size=24
    let mut pc: u32 = 0x82516250;
    'dispatch: loop {
        match pc {
            0x82516250 => {
    //   block [0x82516250..0x82516268)
	// 82516250: 81630098  lwz r11, 0x98(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(152 as u32) ) } as u64;
	// 82516254: 894B0020  lbz r10, 0x20(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82516258: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8251625C: 4182000C  beq 0x82516268
	if ctx.cr[0].eq {
		sub_82516268(ctx, base);
		return;
	}
	// 82516260: 908B001C  stw r4, 0x1c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(28 as u32), ctx.r[4].u32 ) };
	// 82516264: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82516268(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82516268 size=20
    let mut pc: u32 = 0x82516268;
    'dispatch: loop {
        match pc {
            0x82516268 => {
    //   block [0x82516268..0x8251627C)
	// 82516268: 3D600000  lis r11, 0
	ctx.r[11].s64 = 0;
	// 8251626C: 81430094  lwz r10, 0x94(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(148 as u32) ) } as u64;
	// 82516270: 616B8000  ori r11, r11, 0x8000
	ctx.r[11].u64 = ctx.r[11].u64 | 32768;
	// 82516274: 7C8A592E  stwx r4, r10, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32), ctx.r[4].u32) };
	// 82516278: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82516280(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82516280 size=32
    let mut pc: u32 = 0x82516280;
    'dispatch: loop {
        match pc {
            0x82516280 => {
    //   block [0x82516280..0x825162A0)
	// 82516280: 81630098  lwz r11, 0x98(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(152 as u32) ) } as u64;
	// 82516284: 896B0020  lbz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82516288: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8251628C: 41820014  beq 0x825162a0
	if ctx.cr[0].eq {
		sub_825162A0(ctx, base);
		return;
	}
	// 82516290: 81630098  lwz r11, 0x98(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(152 as u32) ) } as u64;
	// 82516294: 814B0024  lwz r10, 0x24(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 82516298: 914B001C  stw r10, 0x1c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(28 as u32), ctx.r[10].u32 ) };
	// 8251629C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825162A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825162A0 size=28
    let mut pc: u32 = 0x825162A0;
    'dispatch: loop {
        match pc {
            0x825162A0 => {
    //   block [0x825162A0..0x825162BC)
	// 825162A0: 3D608328  lis r11, -0x7cd8
	ctx.r[11].s64 = -2094530560;
	// 825162A4: 81430094  lwz r10, 0x94(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(148 as u32) ) } as u64;
	// 825162A8: 3D200000  lis r9, 0
	ctx.r[9].s64 = 0;
	// 825162AC: 61298000  ori r9, r9, 0x8000
	ctx.r[9].u64 = ctx.r[9].u64 | 32768;
	// 825162B0: 816BCEA8  lwz r11, -0x3158(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-12632 as u32) ) } as u64;
	// 825162B4: 7D6A492E  stwx r11, r10, r9
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32), ctx.r[11].u32) };
	// 825162B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825162C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825162C0 size=32
    let mut pc: u32 = 0x825162C0;
    'dispatch: loop {
        match pc {
            0x825162C0 => {
    //   block [0x825162C0..0x825162E0)
	// 825162C0: 81630094  lwz r11, 0x94(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(148 as u32) ) } as u64;
	// 825162C4: 3D408328  lis r10, -0x7cd8
	ctx.r[10].s64 = -2094530560;
	// 825162C8: 3D6B0001  addis r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 65536;
	// 825162CC: 396B8000  addi r11, r11, -0x8000
	ctx.r[11].s64 = ctx.r[11].s64 + -32768;
	// 825162D0: 814ACEA8  lwz r10, -0x3158(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-12632 as u32) ) } as u64;
	// 825162D4: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825162D8: 7F095000  cmpw cr6, r9, r10
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[10].s32, &mut ctx.xer);
	// 825162DC: 4C980020  bgelr cr6
	if !ctx.cr[6].lt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825162E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825162E0 size=8
    let mut pc: u32 = 0x825162E0;
    'dispatch: loop {
        match pc {
            0x825162E0 => {
    //   block [0x825162E0..0x825162E8)
	// 825162E0: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 825162E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825162E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825162E8 size=32
    let mut pc: u32 = 0x825162E8;
    'dispatch: loop {
        match pc {
            0x825162E8 => {
    //   block [0x825162E8..0x82516308)
	// 825162E8: 81630098  lwz r11, 0x98(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(152 as u32) ) } as u64;
	// 825162EC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825162F0: 994B0020  stb r10, 0x20(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[10].u8 ) };
	// 825162F4: 81630098  lwz r11, 0x98(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(152 as u32) ) } as u64;
	// 825162F8: 908B0024  stw r4, 0x24(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(36 as u32), ctx.r[4].u32 ) };
	// 825162FC: 81630098  lwz r11, 0x98(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(152 as u32) ) } as u64;
	// 82516300: 908B001C  stw r4, 0x1c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(28 as u32), ctx.r[4].u32 ) };
	// 82516304: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82516308(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82516308 size=16
    let mut pc: u32 = 0x82516308;
    'dispatch: loop {
        match pc {
            0x82516308 => {
    //   block [0x82516308..0x82516318)
	// 82516308: 81630098  lwz r11, 0x98(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(152 as u32) ) } as u64;
	// 8251630C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82516310: 994B0020  stb r10, 0x20(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[10].u8 ) };
	// 82516314: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82516318(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82516318 size=24
    let mut pc: u32 = 0x82516318;
    'dispatch: loop {
        match pc {
            0x82516318 => {
    //   block [0x82516318..0x82516330)
	// 82516318: 81630094  lwz r11, 0x94(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(148 as u32) ) } as u64;
	// 8251631C: 3D400000  lis r10, 0
	ctx.r[10].s64 = 0;
	// 82516320: 614A8004  ori r10, r10, 0x8004
	ctx.r[10].u64 = ctx.r[10].u64 | 32772;
	// 82516324: 7D6B2214  add r11, r11, r4
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 82516328: 7C6B50AE  lbzx r3, r11, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 8251632C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82516330(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82516330 size=24
    let mut pc: u32 = 0x82516330;
    'dispatch: loop {
        match pc {
            0x82516330 => {
    //   block [0x82516330..0x82516348)
	// 82516330: 81630094  lwz r11, 0x94(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(148 as u32) ) } as u64;
	// 82516334: 3D400000  lis r10, 0
	ctx.r[10].s64 = 0;
	// 82516338: 614A8004  ori r10, r10, 0x8004
	ctx.r[10].u64 = ctx.r[10].u64 | 32772;
	// 8251633C: 7D6B2214  add r11, r11, r4
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 82516340: 7CAB51AE  stbx r5, r11, r10
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[5].u8) };
	// 82516344: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82516348(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82516348 size=16
    let mut pc: u32 = 0x82516348;
    'dispatch: loop {
        match pc {
            0x82516348 => {
    //   block [0x82516348..0x82516358)
	// 82516348: 2B040190  cmplwi cr6, r4, 0x190
	ctx.cr[6].compare_u32(ctx.r[4].u32, 400 as u32, &mut ctx.xer);
	// 8251634C: 4198000C  blt cr6, 0x82516358
	if ctx.cr[6].lt {
		sub_82516358(ctx, base);
		return;
	}
	// 82516350: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82516354: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82516358(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82516358 size=20
    let mut pc: u32 = 0x82516358;
    'dispatch: loop {
        match pc {
            0x82516358 => {
    //   block [0x82516358..0x8251636C)
	// 82516358: 39644004  addi r11, r4, 0x4004
	ctx.r[11].s64 = ctx.r[4].s64 + 16388;
	// 8251635C: 81430094  lwz r10, 0x94(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(148 as u32) ) } as u64;
	// 82516360: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82516364: 7C6B50AE  lbzx r3, r11, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82516368: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82516370(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82516370 size=16
    let mut pc: u32 = 0x82516370;
    'dispatch: loop {
        match pc {
            0x82516370 => {
    //   block [0x82516370..0x82516380)
	// 82516370: 2B040190  cmplwi cr6, r4, 0x190
	ctx.cr[6].compare_u32(ctx.r[4].u32, 400 as u32, &mut ctx.xer);
	// 82516374: 4198000C  blt cr6, 0x82516380
	if ctx.cr[6].lt {
		sub_82516380(ctx, base);
		return;
	}
	// 82516378: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8251637C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82516380(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82516380 size=28
    let mut pc: u32 = 0x82516380;
    'dispatch: loop {
        match pc {
            0x82516380 => {
    //   block [0x82516380..0x8251639C)
	// 82516380: 81630094  lwz r11, 0x94(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(148 as u32) ) } as u64;
	// 82516384: 548A083C  slwi r10, r4, 1
	ctx.r[10].u32 = ctx.r[4].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82516388: 3D200000  lis r9, 0
	ctx.r[9].s64 = 0;
	// 8251638C: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82516390: 612A8009  ori r10, r9, 0x8009
	ctx.r[10].u64 = ctx.r[9].u64 | 32777;
	// 82516394: 7C6B50AE  lbzx r3, r11, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82516398: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825163A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825163A0 size=8
    let mut pc: u32 = 0x825163A0;
    'dispatch: loop {
        match pc {
            0x825163A0 => {
    //   block [0x825163A0..0x825163A8)
	// 825163A0: 2B040190  cmplwi cr6, r4, 0x190
	ctx.cr[6].compare_u32(ctx.r[4].u32, 400 as u32, &mut ctx.xer);
	// 825163A4: 4C980020  bgelr cr6
	if !ctx.cr[6].lt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825163A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825163A8 size=28
    let mut pc: u32 = 0x825163A8;
    'dispatch: loop {
        match pc {
            0x825163A8 => {
    //   block [0x825163A8..0x825163C4)
	// 825163A8: 81630094  lwz r11, 0x94(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(148 as u32) ) } as u64;
	// 825163AC: 548A083C  slwi r10, r4, 1
	ctx.r[10].u32 = ctx.r[4].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 825163B0: 3D200000  lis r9, 0
	ctx.r[9].s64 = 0;
	// 825163B4: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 825163B8: 612A8009  ori r10, r9, 0x8009
	ctx.r[10].u64 = ctx.r[9].u64 | 32777;
	// 825163BC: 7CAB51AE  stbx r5, r11, r10
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[5].u8) };
	// 825163C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825163C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825163C8 size=16
    let mut pc: u32 = 0x825163C8;
    'dispatch: loop {
        match pc {
            0x825163C8 => {
    //   block [0x825163C8..0x825163D8)
	// 825163C8: 2B040190  cmplwi cr6, r4, 0x190
	ctx.cr[6].compare_u32(ctx.r[4].u32, 400 as u32, &mut ctx.xer);
	// 825163CC: 4198000C  blt cr6, 0x825163d8
	if ctx.cr[6].lt {
		sub_825163D8(ctx, base);
		return;
	}
	// 825163D0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 825163D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825163D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825163D8 size=48
    let mut pc: u32 = 0x825163D8;
    'dispatch: loop {
        match pc {
            0x825163D8 => {
    //   block [0x825163D8..0x82516408)
	// 825163D8: 81630094  lwz r11, 0x94(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(148 as u32) ) } as u64;
	// 825163DC: 548A083C  slwi r10, r4, 1
	ctx.r[10].u32 = ctx.r[4].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 825163E0: 3D200000  lis r9, 0
	ctx.r[9].s64 = 0;
	// 825163E4: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 825163E8: 612A8009  ori r10, r9, 0x8009
	ctx.r[10].u64 = ctx.r[9].u64 | 32777;
	// 825163EC: 54A9063E  clrlwi r9, r5, 0x18
	ctx.r[9].u64 = ctx.r[5].u32 as u64 & 0x000000FFu64;
	// 825163F0: 7D6B50AE  lbzx r11, r11, r10
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 825163F4: 7D6B4838  and r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 & ctx.r[9].u64;
	// 825163F8: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 825163FC: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82516400: 69630001  xori r3, r11, 1
	ctx.r[3].u64 = ctx.r[11].u64 ^ 1;
	// 82516404: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82516408(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82516408 size=8
    let mut pc: u32 = 0x82516408;
    'dispatch: loop {
        match pc {
            0x82516408 => {
    //   block [0x82516408..0x82516410)
	// 82516408: 2B040190  cmplwi cr6, r4, 0x190
	ctx.cr[6].compare_u32(ctx.r[4].u32, 400 as u32, &mut ctx.xer);
	// 8251640C: 4C980020  bgelr cr6
	if !ctx.cr[6].lt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82516410(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82516410 size=40
    let mut pc: u32 = 0x82516410;
    'dispatch: loop {
        match pc {
            0x82516410 => {
    //   block [0x82516410..0x82516438)
	// 82516410: 81630094  lwz r11, 0x94(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(148 as u32) ) } as u64;
	// 82516414: 548A083C  slwi r10, r4, 1
	ctx.r[10].u32 = ctx.r[4].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82516418: 54A9063E  clrlwi r9, r5, 0x18
	ctx.r[9].u64 = ctx.r[5].u32 as u64 & 0x000000FFu64;
	// 8251641C: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82516420: 3D6B0001  addis r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 65536;
	// 82516424: 396B8009  addi r11, r11, -0x7ff7
	ctx.r[11].s64 = ctx.r[11].s64 + -32759;
	// 82516428: 894B0000  lbz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251642C: 7D4A4B78  or r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[9].u64;
	// 82516430: 994B0000  stb r10, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 82516434: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82516438(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82516438 size=8
    let mut pc: u32 = 0x82516438;
    'dispatch: loop {
        match pc {
            0x82516438 => {
    //   block [0x82516438..0x82516440)
	// 82516438: 2B040190  cmplwi cr6, r4, 0x190
	ctx.cr[6].compare_u32(ctx.r[4].u32, 400 as u32, &mut ctx.xer);
	// 8251643C: 4C980020  bgelr cr6
	if !ctx.cr[6].lt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82516440(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82516440 size=40
    let mut pc: u32 = 0x82516440;
    'dispatch: loop {
        match pc {
            0x82516440 => {
    //   block [0x82516440..0x82516468)
	// 82516440: 81630094  lwz r11, 0x94(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(148 as u32) ) } as u64;
	// 82516444: 548A083C  slwi r10, r4, 1
	ctx.r[10].u32 = ctx.r[4].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82516448: 54A9063E  clrlwi r9, r5, 0x18
	ctx.r[9].u64 = ctx.r[5].u32 as u64 & 0x000000FFu64;
	// 8251644C: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82516450: 3D6B0001  addis r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 65536;
	// 82516454: 396B8009  addi r11, r11, -0x7ff7
	ctx.r[11].s64 = ctx.r[11].s64 + -32759;
	// 82516458: 894B0000  lbz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251645C: 7D4A4878  andc r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 & !ctx.r[9].u64;
	// 82516460: 994B0000  stb r10, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 82516464: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82516468(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82516468 size=16
    let mut pc: u32 = 0x82516468;
    'dispatch: loop {
        match pc {
            0x82516468 => {
    //   block [0x82516468..0x82516478)
	// 82516468: 2B040064  cmplwi cr6, r4, 0x64
	ctx.cr[6].compare_u32(ctx.r[4].u32, 100 as u32, &mut ctx.xer);
	// 8251646C: 4198000C  blt cr6, 0x82516478
	if ctx.cr[6].lt {
		sub_82516478(ctx, base);
		return;
	}
	// 82516470: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82516474: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82516478(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82516478 size=44
    let mut pc: u32 = 0x82516478;
    'dispatch: loop {
        match pc {
            0x82516478 => {
    //   block [0x82516478..0x825164A4)
	// 82516478: 81630094  lwz r11, 0x94(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(148 as u32) ) } as u64;
	// 8251647C: 3D400000  lis r10, 0
	ctx.r[10].s64 = 0;
	// 82516480: 54A9063E  clrlwi r9, r5, 0x18
	ctx.r[9].u64 = ctx.r[5].u32 as u64 & 0x000000FFu64;
	// 82516484: 7D6B2214  add r11, r11, r4
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 82516488: 614A8328  ori r10, r10, 0x8328
	ctx.r[10].u64 = ctx.r[10].u64 | 33576;
	// 8251648C: 7D6B50AE  lbzx r11, r11, r10
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82516490: 7D6B4838  and r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 & ctx.r[9].u64;
	// 82516494: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82516498: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8251649C: 69630001  xori r3, r11, 1
	ctx.r[3].u64 = ctx.r[11].u64 ^ 1;
	// 825164A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825164A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825164A8 size=8
    let mut pc: u32 = 0x825164A8;
    'dispatch: loop {
        match pc {
            0x825164A8 => {
    //   block [0x825164A8..0x825164B0)
	// 825164A8: 2B040064  cmplwi cr6, r4, 0x64
	ctx.cr[6].compare_u32(ctx.r[4].u32, 100 as u32, &mut ctx.xer);
	// 825164AC: 4C980020  bgelr cr6
	if !ctx.cr[6].lt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825164B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825164B0 size=36
    let mut pc: u32 = 0x825164B0;
    'dispatch: loop {
        match pc {
            0x825164B0 => {
    //   block [0x825164B0..0x825164D4)
	// 825164B0: 81630094  lwz r11, 0x94(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(148 as u32) ) } as u64;
	// 825164B4: 54AA063E  clrlwi r10, r5, 0x18
	ctx.r[10].u64 = ctx.r[5].u32 as u64 & 0x000000FFu64;
	// 825164B8: 7D6B2214  add r11, r11, r4
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 825164BC: 3D6B0001  addis r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 65536;
	// 825164C0: 396B8328  addi r11, r11, -0x7cd8
	ctx.r[11].s64 = ctx.r[11].s64 + -31960;
	// 825164C4: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825164C8: 7D2A5378  or r10, r9, r10
	ctx.r[10].u64 = ctx.r[9].u64 | ctx.r[10].u64;
	// 825164CC: 994B0000  stb r10, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 825164D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825164D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825164D8 size=8
    let mut pc: u32 = 0x825164D8;
    'dispatch: loop {
        match pc {
            0x825164D8 => {
    //   block [0x825164D8..0x825164E0)
	// 825164D8: 2B040064  cmplwi cr6, r4, 0x64
	ctx.cr[6].compare_u32(ctx.r[4].u32, 100 as u32, &mut ctx.xer);
	// 825164DC: 4C980020  bgelr cr6
	if !ctx.cr[6].lt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825164E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825164E0 size=36
    let mut pc: u32 = 0x825164E0;
    'dispatch: loop {
        match pc {
            0x825164E0 => {
    //   block [0x825164E0..0x82516504)
	// 825164E0: 81630094  lwz r11, 0x94(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(148 as u32) ) } as u64;
	// 825164E4: 54AA063E  clrlwi r10, r5, 0x18
	ctx.r[10].u64 = ctx.r[5].u32 as u64 & 0x000000FFu64;
	// 825164E8: 7D6B2214  add r11, r11, r4
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 825164EC: 3D6B0001  addis r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 65536;
	// 825164F0: 396B8328  addi r11, r11, -0x7cd8
	ctx.r[11].s64 = ctx.r[11].s64 + -31960;
	// 825164F4: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825164F8: 7D2A5078  andc r10, r9, r10
	ctx.r[10].u64 = ctx.r[9].u64 & !ctx.r[10].u64;
	// 825164FC: 994B0000  stb r10, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 82516500: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82516508(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82516508 size=16
    let mut pc: u32 = 0x82516508;
    'dispatch: loop {
        match pc {
            0x82516508 => {
    //   block [0x82516508..0x82516518)
	// 82516508: 2B0401F4  cmplwi cr6, r4, 0x1f4
	ctx.cr[6].compare_u32(ctx.r[4].u32, 500 as u32, &mut ctx.xer);
	// 8251650C: 4198000C  blt cr6, 0x82516518
	if ctx.cr[6].lt {
		sub_82516518(ctx, base);
		return;
	}
	// 82516510: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82516514: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82516518(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82516518 size=24
    let mut pc: u32 = 0x82516518;
    'dispatch: loop {
        match pc {
            0x82516518 => {
    //   block [0x82516518..0x82516530)
	// 82516518: 81630094  lwz r11, 0x94(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(148 as u32) ) } as u64;
	// 8251651C: 3D400000  lis r10, 0
	ctx.r[10].s64 = 0;
	// 82516520: 614A838C  ori r10, r10, 0x838c
	ctx.r[10].u64 = ctx.r[10].u64 | 33676;
	// 82516524: 7D6B2214  add r11, r11, r4
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 82516528: 7C6B50AE  lbzx r3, r11, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 8251652C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82516530(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82516530 size=8
    let mut pc: u32 = 0x82516530;
    'dispatch: loop {
        match pc {
            0x82516530 => {
    //   block [0x82516530..0x82516538)
	// 82516530: 2B0401F4  cmplwi cr6, r4, 0x1f4
	ctx.cr[6].compare_u32(ctx.r[4].u32, 500 as u32, &mut ctx.xer);
	// 82516534: 4C980020  bgelr cr6
	if !ctx.cr[6].lt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82516538(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82516538 size=24
    let mut pc: u32 = 0x82516538;
    'dispatch: loop {
        match pc {
            0x82516538 => {
    //   block [0x82516538..0x82516550)
	// 82516538: 81630094  lwz r11, 0x94(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(148 as u32) ) } as u64;
	// 8251653C: 3D400000  lis r10, 0
	ctx.r[10].s64 = 0;
	// 82516540: 614A838C  ori r10, r10, 0x838c
	ctx.r[10].u64 = ctx.r[10].u64 | 33676;
	// 82516544: 7D6B2214  add r11, r11, r4
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 82516548: 7CAB51AE  stbx r5, r11, r10
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[5].u8) };
	// 8251654C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82516550(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82516550 size=16
    let mut pc: u32 = 0x82516550;
    'dispatch: loop {
        match pc {
            0x82516550 => {
    //   block [0x82516550..0x82516560)
	// 82516550: 2B0401F4  cmplwi cr6, r4, 0x1f4
	ctx.cr[6].compare_u32(ctx.r[4].u32, 500 as u32, &mut ctx.xer);
	// 82516554: 4198000C  blt cr6, 0x82516560
	if ctx.cr[6].lt {
		sub_82516560(ctx, base);
		return;
	}
	// 82516558: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8251655C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82516560(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82516560 size=44
    let mut pc: u32 = 0x82516560;
    'dispatch: loop {
        match pc {
            0x82516560 => {
    //   block [0x82516560..0x8251658C)
	// 82516560: 81630094  lwz r11, 0x94(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(148 as u32) ) } as u64;
	// 82516564: 3D400000  lis r10, 0
	ctx.r[10].s64 = 0;
	// 82516568: 54A9063E  clrlwi r9, r5, 0x18
	ctx.r[9].u64 = ctx.r[5].u32 as u64 & 0x000000FFu64;
	// 8251656C: 7D6B2214  add r11, r11, r4
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 82516570: 614A838C  ori r10, r10, 0x838c
	ctx.r[10].u64 = ctx.r[10].u64 | 33676;
	// 82516574: 7D6B50AE  lbzx r11, r11, r10
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82516578: 7D6B4838  and r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 & ctx.r[9].u64;
	// 8251657C: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82516580: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82516584: 69630001  xori r3, r11, 1
	ctx.r[3].u64 = ctx.r[11].u64 ^ 1;
	// 82516588: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82516590(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82516590 size=8
    let mut pc: u32 = 0x82516590;
    'dispatch: loop {
        match pc {
            0x82516590 => {
    //   block [0x82516590..0x82516598)
	// 82516590: 2B0401F4  cmplwi cr6, r4, 0x1f4
	ctx.cr[6].compare_u32(ctx.r[4].u32, 500 as u32, &mut ctx.xer);
	// 82516594: 4C980020  bgelr cr6
	if !ctx.cr[6].lt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82516598(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82516598 size=36
    let mut pc: u32 = 0x82516598;
    'dispatch: loop {
        match pc {
            0x82516598 => {
    //   block [0x82516598..0x825165BC)
	// 82516598: 81630094  lwz r11, 0x94(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(148 as u32) ) } as u64;
	// 8251659C: 54AA063E  clrlwi r10, r5, 0x18
	ctx.r[10].u64 = ctx.r[5].u32 as u64 & 0x000000FFu64;
	// 825165A0: 7D6B2214  add r11, r11, r4
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 825165A4: 3D6B0001  addis r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 65536;
	// 825165A8: 396B838C  addi r11, r11, -0x7c74
	ctx.r[11].s64 = ctx.r[11].s64 + -31860;
	// 825165AC: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825165B0: 7D2A5378  or r10, r9, r10
	ctx.r[10].u64 = ctx.r[9].u64 | ctx.r[10].u64;
	// 825165B4: 994B0000  stb r10, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 825165B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


