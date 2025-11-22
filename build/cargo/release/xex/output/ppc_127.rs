pub fn sub_82C0EB20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C0EB20 size=224
    let mut pc: u32 = 0x82C0EB20;
    'dispatch: loop {
        match pc {
            0x82C0EB20 => {
    //   block [0x82C0EB20..0x82C0EC00)
	// 82C0EB20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C0EB24: 4809A8E5  bl 0x82ca9408
	ctx.lr = 0x82C0EB28;
	sub_82CA93D0(ctx, base);
	// 82C0EB28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C0EB2C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C0EB30: 9081009C  stw r4, 0x9c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(156 as u32), ctx.r[4].u32 ) };
	// 82C0EB34: 897F0038  lbz r11, 0x38(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 82C0EB38: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C0EB3C: 419A0028  beq cr6, 0x82c0eb64
	if ctx.cr[6].eq {
	pc = 0x82C0EB64; continue 'dispatch;
	}
	// 82C0EB40: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82C0EB44: 389F001C  addi r4, r31, 0x1c
	ctx.r[4].s64 = ctx.r[31].s64 + 28;
	// 82C0EB48: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C0EB4C: 4B5F1C25  bl 0x82200770
	ctx.lr = 0x82C0EB50;
	sub_82200770(ctx, base);
	// 82C0EB50: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C0EB54: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C0EB58: 409A0018  bne cr6, 0x82c0eb70
	if !ctx.cr[6].eq {
	pc = 0x82C0EB70; continue 'dispatch;
	}
	// 82C0EB5C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C0EB60: 4B5B4A59  bl 0x821c35b8
	ctx.lr = 0x82C0EB64;
	sub_821C35B8(ctx, base);
	// 82C0EB64: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82C0EB68: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82C0EB6C: 4809A8EC  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
	// 82C0EB70: 3B9F0010  addi r28, r31, 0x10
	ctx.r[28].s64 = ctx.r[31].s64 + 16;
	// 82C0EB74: 3881009C  addi r4, r1, 0x9c
	ctx.r[4].s64 = ctx.r[1].s64 + 156;
	// 82C0EB78: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82C0EB7C: 4B75BE35  bl 0x8236a9b0
	ctx.lr = 0x82C0EB80;
	sub_8236A9B0(ctx, base);
	// 82C0EB80: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C0EB84: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C0EB88: 813F0018  lwz r9, 0x18(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82C0EB8C: 7D0B5050  subf r8, r11, r10
	ctx.r[8].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 82C0EB90: 83C30000  lwz r30, 0(r3)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C0EB94: 7CE84B96  divwu r7, r8, r9
	ctx.r[7].u32 = ctx.r[8].u32 / ctx.r[9].u32;
	// 82C0EB98: 7F1E3840  cmplw cr6, r30, r7
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[7].u32, &mut ctx.xer);
	// 82C0EB9C: 4098FFC0  bge cr6, 0x82c0eb5c
	if !ctx.cr[6].lt {
	pc = 0x82C0EB5C; continue 'dispatch;
	}
	// 82C0EBA0: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82C0EBA4: 419A0010  beq cr6, 0x82c0ebb4
	if ctx.cr[6].eq {
	pc = 0x82C0EBB4; continue 'dispatch;
	}
	// 82C0EBA8: 815F000C  lwz r10, 0xc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C0EBAC: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82C0EBB0: 4099FFAC  ble cr6, 0x82c0eb5c
	if !ctx.cr[6].gt {
	pc = 0x82C0EB5C; continue 'dispatch;
	}
	// 82C0EBB4: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C0EBB8: 4BFFFBE1  bl 0x82c0e798
	ctx.lr = 0x82C0EBBC;
	sub_82C0E798(ctx, base);
	// 82C0EBBC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82C0EBC0: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82C0EBC4: 419AFF98  beq cr6, 0x82c0eb5c
	if ctx.cr[6].eq {
	pc = 0x82C0EB5C; continue 'dispatch;
	}
	// 82C0EBC8: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C0EBCC: 3881009C  addi r4, r1, 0x9c
	ctx.r[4].s64 = ctx.r[1].s64 + 156;
	// 82C0EBD0: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82C0EBD4: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82C0EBD8: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82C0EBDC: 4B75BDD5  bl 0x8236a9b0
	ctx.lr = 0x82C0EBE0;
	sub_8236A9B0(ctx, base);
	// 82C0EBE0: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 82C0EBE4: 393E0001  addi r9, r30, 1
	ctx.r[9].s64 = ctx.r[30].s64 + 1;
	// 82C0EBE8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C0EBEC: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82C0EBF0: 4B5B49C9  bl 0x821c35b8
	ctx.lr = 0x82C0EBF4;
	sub_821C35B8(ctx, base);
	// 82C0EBF4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82C0EBF8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82C0EBFC: 4809A85C  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C0EC00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C0EC00 size=20
    let mut pc: u32 = 0x82C0EC00;
    'dispatch: loop {
        match pc {
            0x82C0EC00 => {
    //   block [0x82C0EC00..0x82C0EC14)
	// 82C0EC00: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82C0EC04: 409A0010  bne cr6, 0x82c0ec14
	if !ctx.cr[6].eq {
		sub_82C0EC14(ctx, base);
		return;
	}
	// 82C0EC08: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 82C0EC0C: 386B4E74  addi r3, r11, 0x4e74
	ctx.r[3].s64 = ctx.r[11].s64 + 20084;
	// 82C0EC10: 4BFFFF10  b 0x82c0eb20
	sub_82C0EB20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C0EC14(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C0EC14 size=12
    let mut pc: u32 = 0x82C0EC14;
    'dispatch: loop {
        match pc {
            0x82C0EC14 => {
    //   block [0x82C0EC14..0x82C0EC20)
	// 82C0EC14: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 82C0EC18: 386B4EB0  addi r3, r11, 0x4eb0
	ctx.r[3].s64 = ctx.r[11].s64 + 20144;
	// 82C0EC1C: 4BFFFF04  b 0x82c0eb20
	sub_82C0EB20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C0EC20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C0EC20 size=16
    let mut pc: u32 = 0x82C0EC20;
    'dispatch: loop {
        match pc {
            0x82C0EC20 => {
    //   block [0x82C0EC20..0x82C0EC30)
	// 82C0EC20: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C0EC24: 394BA404  addi r10, r11, -0x5bfc
	ctx.r[10].s64 = ctx.r[11].s64 + -23548;
	// 82C0EC28: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82C0EC2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C0EC30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C0EC30 size=40
    let mut pc: u32 = 0x82C0EC30;
    'dispatch: loop {
        match pc {
            0x82C0EC30 => {
    //   block [0x82C0EC30..0x82C0EC58)
	// 82C0EC30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82C0EC34: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 82C0EC38: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82C0EC3C: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82C0EC40: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82C0EC44: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82C0EC48: 9143000C  stw r10, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 82C0EC4C: 91230010  stw r9, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[9].u32 ) };
	// 82C0EC50: 91630014  stw r11, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82C0EC54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C0EC58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C0EC58 size=12
    let mut pc: u32 = 0x82C0EC58;
    'dispatch: loop {
        match pc {
            0x82C0EC58 => {
    //   block [0x82C0EC58..0x82C0EC64)
	// 82C0EC58: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82C0EC5C: 9963009F  stb r11, 0x9f(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(159 as u32), ctx.r[11].u8 ) };
	// 82C0EC60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C0EC68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C0EC68 size=12
    let mut pc: u32 = 0x82C0EC68;
    'dispatch: loop {
        match pc {
            0x82C0EC68 => {
    //   block [0x82C0EC68..0x82C0EC74)
	// 82C0EC68: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82C0EC6C: 9963009D  stb r11, 0x9d(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(157 as u32), ctx.r[11].u8 ) };
	// 82C0EC70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C0EC78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C0EC78 size=84
    let mut pc: u32 = 0x82C0EC78;
    'dispatch: loop {
        match pc {
            0x82C0EC78 => {
    //   block [0x82C0EC78..0x82C0ECCC)
	// 82C0EC78: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 82C0EC7C: 7C691B78  mr r9, r3
	ctx.r[9].u64 = ctx.r[3].u64;
	// 82C0EC80: 816B4EEC  lwz r11, 0x4eec(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20204 as u32) ) } as u64;
	// 82C0EC84: 814B0044  lwz r10, 0x44(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(68 as u32) ) } as u64;
	// 82C0EC88: 7F035040  cmplw cr6, r3, r10
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82C0EC8C: 41980008  blt cr6, 0x82c0ec94
	if ctx.cr[6].lt {
	pc = 0x82C0EC94; continue 'dispatch;
	}
	// 82C0EC90: 7D495378  mr r9, r10
	ctx.r[9].u64 = ctx.r[10].u64;
	// 82C0EC94: 7CE95050  subf r7, r9, r10
	ctx.r[7].s64 = ctx.r[10].s64 - ctx.r[9].s64;
	// 82C0EC98: 814B0048  lwz r10, 0x48(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(72 as u32) ) } as u64;
	// 82C0EC9C: 7D091850  subf r8, r9, r3
	ctx.r[8].s64 = ctx.r[3].s64 - ctx.r[9].s64;
	// 82C0ECA0: 90EB0044  stw r7, 0x44(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(68 as u32), ctx.r[7].u32 ) };
	// 82C0ECA4: 7F085040  cmplw cr6, r8, r10
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82C0ECA8: 41980008  blt cr6, 0x82c0ecb0
	if ctx.cr[6].lt {
	pc = 0x82C0ECB0; continue 'dispatch;
	}
	// 82C0ECAC: 7D485378  mr r8, r10
	ctx.r[8].u64 = ctx.r[10].u64;
	// 82C0ECB0: 80EB004C  lwz r7, 0x4c(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(76 as u32) ) } as u64;
	// 82C0ECB4: 7C684A14  add r3, r8, r9
	ctx.r[3].u64 = ctx.r[8].u64 + ctx.r[9].u64;
	// 82C0ECB8: 7D485050  subf r10, r8, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[8].s64;
	// 82C0ECBC: 7D271A14  add r9, r7, r3
	ctx.r[9].u64 = ctx.r[7].u64 + ctx.r[3].u64;
	// 82C0ECC0: 914B0048  stw r10, 0x48(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(72 as u32), ctx.r[10].u32 ) };
	// 82C0ECC4: 912B004C  stw r9, 0x4c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(76 as u32), ctx.r[9].u32 ) };
	// 82C0ECC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C0ECD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C0ECD0 size=60
    let mut pc: u32 = 0x82C0ECD0;
    'dispatch: loop {
        match pc {
            0x82C0ECD0 => {
    //   block [0x82C0ECD0..0x82C0ED0C)
	// 82C0ECD0: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 82C0ECD4: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 82C0ECD8: 816B4EEC  lwz r11, 0x4eec(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20204 as u32) ) } as u64;
	// 82C0ECDC: 812B004C  lwz r9, 0x4c(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(76 as u32) ) } as u64;
	// 82C0ECE0: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82C0ECE4: 40990008  ble cr6, 0x82c0ecec
	if !ctx.cr[6].gt {
	pc = 0x82C0ECEC; continue 'dispatch;
	}
	// 82C0ECE8: 7D2A4B78  mr r10, r9
	ctx.r[10].u64 = ctx.r[9].u64;
	// 82C0ECEC: 812B0040  lwz r9, 0x40(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(64 as u32) ) } as u64;
	// 82C0ECF0: 810B0048  lwz r8, 0x48(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(72 as u32) ) } as u64;
	// 82C0ECF4: 7D284850  subf r9, r8, r9
	ctx.r[9].s64 = ctx.r[9].s64 - ctx.r[8].s64;
	// 82C0ECF8: 7F095040  cmplw cr6, r9, r10
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82C0ECFC: 41980010  blt cr6, 0x82c0ed0c
	if ctx.cr[6].lt {
		sub_82C0ED0C(ctx, base);
		return;
	}
	// 82C0ED00: 7D085214  add r8, r8, r10
	ctx.r[8].u64 = ctx.r[8].u64 + ctx.r[10].u64;
	// 82C0ED04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82C0ED08: 4800000C  b 0x82c0ed14
	sub_82C0ED0C(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C0ED0C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C0ED0C size=40
    let mut pc: u32 = 0x82C0ED0C;
    'dispatch: loop {
        match pc {
            0x82C0ED0C => {
    //   block [0x82C0ED0C..0x82C0ED34)
	// 82C0ED0C: 7D084A14  add r8, r8, r9
	ctx.r[8].u64 = ctx.r[8].u64 + ctx.r[9].u64;
	// 82C0ED10: 7D295050  subf r9, r9, r10
	ctx.r[9].s64 = ctx.r[10].s64 - ctx.r[9].s64;
	// 82C0ED14: 910B0048  stw r8, 0x48(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(72 as u32), ctx.r[8].u32 ) };
	// 82C0ED18: 810B0044  lwz r8, 0x44(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(68 as u32) ) } as u64;
	// 82C0ED1C: 80EB004C  lwz r7, 0x4c(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(76 as u32) ) } as u64;
	// 82C0ED20: 7CC84A14  add r6, r8, r9
	ctx.r[6].u64 = ctx.r[8].u64 + ctx.r[9].u64;
	// 82C0ED24: 7CAA3850  subf r5, r10, r7
	ctx.r[5].s64 = ctx.r[7].s64 - ctx.r[10].s64;
	// 82C0ED28: 90CB0044  stw r6, 0x44(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(68 as u32), ctx.r[6].u32 ) };
	// 82C0ED2C: 90AB004C  stw r5, 0x4c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(76 as u32), ctx.r[5].u32 ) };
	// 82C0ED30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C0ED38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C0ED38 size=4
    let mut pc: u32 = 0x82C0ED38;
    'dispatch: loop {
        match pc {
            0x82C0ED38 => {
    //   block [0x82C0ED38..0x82C0ED3C)
	// 82C0ED38: 4BFF6E90  b 0x82c05bc8
	sub_82C05BC8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C0ED40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C0ED40 size=12
    let mut pc: u32 = 0x82C0ED40;
    'dispatch: loop {
        match pc {
            0x82C0ED40 => {
    //   block [0x82C0ED40..0x82C0ED4C)
	// 82C0ED40: 81640008  lwz r11, 8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C0ED44: 7F0B1840  cmplw cr6, r11, r3
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[3].u32, &mut ctx.xer);
	// 82C0ED48: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C0ED4C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C0ED4C size=52
    let mut pc: u32 = 0x82C0ED4C;
    'dispatch: loop {
        match pc {
            0x82C0ED4C => {
    //   block [0x82C0ED4C..0x82C0ED80)
	// 82C0ED4C: 81640004  lwz r11, 4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C0ED50: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82C0ED54: 81440000  lwz r10, 0(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C0ED58: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C0ED5C: 409A003C  bne cr6, 0x82c0ed98
	if !ctx.cr[6].eq {
		sub_82C0ED98(ctx, base);
		return;
	}
	// 82C0ED60: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C0ED64: 409A001C  bne cr6, 0x82c0ed80
	if !ctx.cr[6].eq {
		sub_82C0ED80(ctx, base);
		return;
	}
	// 82C0ED68: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82C0ED6C: 91230004  stw r9, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82C0ED70: 91240004  stw r9, 4(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82C0ED74: 91240000  stw r9, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82C0ED78: 91240008  stw r9, 8(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82C0ED7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C0ED80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C0ED80 size=24
    let mut pc: u32 = 0x82C0ED80;
    'dispatch: loop {
        match pc {
            0x82C0ED80 => {
    //   block [0x82C0ED80..0x82C0ED98)
	// 82C0ED80: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82C0ED84: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82C0ED88: 91240004  stw r9, 4(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82C0ED8C: 91240000  stw r9, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82C0ED90: 91240008  stw r9, 8(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82C0ED94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C0ED98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C0ED98 size=32
    let mut pc: u32 = 0x82C0ED98;
    'dispatch: loop {
        match pc {
            0x82C0ED98 => {
    //   block [0x82C0ED98..0x82C0EDB8)
	// 82C0ED98: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C0ED9C: 409A001C  bne cr6, 0x82c0edb8
	if !ctx.cr[6].eq {
		sub_82C0EDB8(ctx, base);
		return;
	}
	// 82C0EDA0: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82C0EDA4: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82C0EDA8: 91240004  stw r9, 4(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82C0EDAC: 91240000  stw r9, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82C0EDB0: 91240008  stw r9, 8(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82C0EDB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C0EDB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C0EDB8 size=24
    let mut pc: u32 = 0x82C0EDB8;
    'dispatch: loop {
        match pc {
            0x82C0EDB8 => {
    //   block [0x82C0EDB8..0x82C0EDD0)
	// 82C0EDB8: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82C0EDBC: 916A0004  stw r11, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82C0EDC0: 91240004  stw r9, 4(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82C0EDC4: 91240000  stw r9, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82C0EDC8: 91240008  stw r9, 8(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82C0EDCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C0EDD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C0EDD0 size=44
    let mut pc: u32 = 0x82C0EDD0;
    'dispatch: loop {
        match pc {
            0x82C0EDD0 => {
    //   block [0x82C0EDD0..0x82C0EDFC)
	// 82C0EDD0: 8163000C  lwz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C0EDD4: 7F045840  cmplw cr6, r4, r11
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82C0EDD8: 41980018  blt cr6, 0x82c0edf0
	if ctx.cr[6].lt {
	pc = 0x82C0EDF0; continue 'dispatch;
	}
	// 82C0EDDC: 81430004  lwz r10, 4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C0EDE0: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82C0EDE4: 7F0B2040  cmplw cr6, r11, r4
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[4].u32, &mut ctx.xer);
	// 82C0EDE8: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82C0EDEC: 40980008  bge cr6, 0x82c0edf4
	if !ctx.cr[6].lt {
	pc = 0x82C0EDF4; continue 'dispatch;
	}
	// 82C0EDF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82C0EDF4: 5563063E  clrlwi r3, r11, 0x18
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82C0EDF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C0EE00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C0EE00 size=8
    let mut pc: u32 = 0x82C0EE00;
    'dispatch: loop {
        match pc {
            0x82C0EE00 => {
    //   block [0x82C0EE00..0x82C0EE08)
	// 82C0EE00: 80630010  lwz r3, 0x10(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C0EE04: 4BFF6DEC  b 0x82c05bf0
	sub_82C05BF0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C0EE08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C0EE08 size=8
    let mut pc: u32 = 0x82C0EE08;
    'dispatch: loop {
        match pc {
            0x82C0EE08 => {
    //   block [0x82C0EE08..0x82C0EE10)
	// 82C0EE08: 80630010  lwz r3, 0x10(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C0EE0C: 4BFF6E2C  b 0x82c05c38
	sub_82C05C38(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C0EE10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C0EE10 size=20
    let mut pc: u32 = 0x82C0EE10;
    'dispatch: loop {
        match pc {
            0x82C0EE10 => {
    //   block [0x82C0EE10..0x82C0EE24)
	// 82C0EE10: 80630010  lwz r3, 0x10(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C0EE14: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C0EE18: 814B0018  lwz r10, 0x18(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82C0EE1C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C0EE20: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C0EE28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C0EE28 size=128
    let mut pc: u32 = 0x82C0EE28;
    'dispatch: loop {
        match pc {
            0x82C0EE28 => {
    //   block [0x82C0EE28..0x82C0EEA8)
	// 82C0EE28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C0EE2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C0EE30: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C0EE34: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C0EE38: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C0EE3C: 817F002C  lwz r11, 0x2c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 82C0EE40: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C0EE44: 419A0014  beq cr6, 0x82c0ee58
	if ctx.cr[6].eq {
	pc = 0x82C0EE58; continue 'dispatch;
	}
	// 82C0EE48: 815F0030  lwz r10, 0x30(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 82C0EE4C: 7D2B5050  subf r9, r11, r10
	ctx.r[9].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 82C0EE50: 7D2B1671  srawi. r11, r9, 2
	ctx.xer.ca = (ctx.r[9].s32 < 0) && ((ctx.r[9].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[9].s32 >> 2) as i64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82C0EE54: 4082001C  bne 0x82c0ee70
	if !ctx.cr[0].eq {
	pc = 0x82C0EE70; continue 'dispatch;
	}
	// 82C0EE58: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82C0EE5C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82C0EE60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C0EE64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C0EE68: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C0EE6C: 4E800020  blr
	return;
	// 82C0EE70: 4BFF3DC1  bl 0x82c02c30
	ctx.lr = 0x82C0EE74;
	sub_82C02C30(ctx, base);
	// 82C0EE74: 817F002C  lwz r11, 0x2c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 82C0EE78: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C0EE7C: 419A0010  beq cr6, 0x82c0ee8c
	if ctx.cr[6].eq {
	pc = 0x82C0EE8C; continue 'dispatch;
	}
	// 82C0EE80: 815F0030  lwz r10, 0x30(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 82C0EE84: 7D2B5050  subf r9, r11, r10
	ctx.r[9].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 82C0EE88: 7D2B1670  srawi r11, r9, 2
	ctx.xer.ca = (ctx.r[9].s32 < 0) && ((ctx.r[9].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[9].s32 >> 2) as i64;
	// 82C0EE8C: 81430010  lwz r10, 0x10(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C0EE90: 7C6A59D6  mullw r3, r10, r11
	ctx.r[3].s64 = (ctx.r[10].s32 as i64) * (ctx.r[11].s32 as i64);
	// 82C0EE94: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82C0EE98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C0EE9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C0EEA0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C0EEA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C0EEA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C0EEA8 size=200
    let mut pc: u32 = 0x82C0EEA8;
    'dispatch: loop {
        match pc {
            0x82C0EEA8 => {
    //   block [0x82C0EEA8..0x82C0EF70)
	// 82C0EEA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C0EEAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C0EEB0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C0EEB4: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C0EEB8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C0EEBC: 897F00AC  lbz r11, 0xac(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(172 as u32) ) } as u64;
	// 82C0EEC0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C0EEC4: 419A001C  beq cr6, 0x82c0eee0
	if ctx.cr[6].eq {
	pc = 0x82C0EEE0; continue 'dispatch;
	}
	// 82C0EEC8: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82C0EECC: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82C0EED0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C0EED4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C0EED8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C0EEDC: 4E800020  blr
	return;
	// 82C0EEE0: 807F0018  lwz r3, 0x18(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82C0EEE4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C0EEE8: 419A0024  beq cr6, 0x82c0ef0c
	if ctx.cr[6].eq {
	pc = 0x82C0EF0C; continue 'dispatch;
	}
	// 82C0EEEC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C0EEF0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82C0EEF4: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C0EEF8: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C0EEFC: 4E800421  bctrl
	ctx.lr = 0x82C0EF00;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C0EF00: 81210058  lwz r9, 0x58(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82C0EF04: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82C0EF08: 419AFFC0  beq cr6, 0x82c0eec8
	if ctx.cr[6].eq {
	pc = 0x82C0EEC8; continue 'dispatch;
	}
	// 82C0EF0C: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C0EF10: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82C0EF14: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C0EF18: 814B0038  lwz r10, 0x38(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) } as u64;
	// 82C0EF1C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C0EF20: 4E800421  bctrl
	ctx.lr = 0x82C0EF24;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C0EF24: 893F009D  lbz r9, 0x9d(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(157 as u32) ) } as u64;
	// 82C0EF28: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82C0EF2C: 419A0028  beq cr6, 0x82c0ef54
	if ctx.cr[6].eq {
	pc = 0x82C0EF54; continue 'dispatch;
	}
	// 82C0EF30: E97F0060  ld r11, 0x60(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) };
	// 82C0EF34: 2B2B0000  cmpldi cr6, r11, 0
	ctx.cr[6].compare_u64(ctx.r[11].u64, 0, &mut ctx.xer);
	// 82C0EF38: 409A001C  bne cr6, 0x82c0ef54
	if !ctx.cr[6].eq {
	pc = 0x82C0EF54; continue 'dispatch;
	}
	// 82C0EF3C: E97F0068  ld r11, 0x68(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) };
	// 82C0EF40: E9410060  ld r10, 0x60(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) };
	// 82C0EF44: 7969F842  rldicl r9, r11, 0x3f, 1
	ctx.r[9].u64 = ctx.r[11].u64 & 0x0000000000000001u64;
	// 82C0EF48: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82C0EF4C: 7F2A4840  cmpld cr6, r10, r9
	ctx.cr[6].compare_u64(ctx.r[10].u64, ctx.r[9].u64, &mut ctx.xer);
	// 82C0EF50: 419A0008  beq cr6, 0x82c0ef58
	if ctx.cr[6].eq {
	pc = 0x82C0EF58; continue 'dispatch;
	}
	// 82C0EF54: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82C0EF58: 5563063E  clrlwi r3, r11, 0x18
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82C0EF5C: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82C0EF60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C0EF64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C0EF68: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C0EF6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C0EF70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C0EF70 size=272
    let mut pc: u32 = 0x82C0EF70;
    'dispatch: loop {
        match pc {
            0x82C0EF70 => {
    //   block [0x82C0EF70..0x82C0F080)
	// 82C0EF70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C0EF74: 4809A495  bl 0x82ca9408
	ctx.lr = 0x82C0EF78;
	sub_82CA93D0(ctx, base);
	// 82C0EF78: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C0EF7C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C0EF80: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82C0EF84: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C0EF88: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C0EF8C: 814B0038  lwz r10, 0x38(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) } as u64;
	// 82C0EF90: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C0EF94: 4E800421  bctrl
	ctx.lr = 0x82C0EF98;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C0EF98: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C0EF9C: 4BFFFE8D  bl 0x82c0ee28
	ctx.lr = 0x82C0EFA0;
	sub_82C0EE28(ctx, base);
	// 82C0EFA0: 8121006C  lwz r9, 0x6c(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(108 as u32) ) } as u64;
	// 82C0EFA4: 81010068  lwz r8, 0x68(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 82C0EFA8: 7D684850  subf r11, r8, r9
	ctx.r[11].s64 = ctx.r[9].s64 - ctx.r[8].s64;
	// 82C0EFAC: 7CEB1A15  add. r7, r11, r3
	ctx.r[7].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[7].s32, 0, &mut ctx.xer);
	// 82C0EFB0: 418200C8  beq 0x82c0f078
	if ctx.cr[0].eq {
	pc = 0x82C0F078; continue 'dispatch;
	}
	// 82C0EFB4: 817F002C  lwz r11, 0x2c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 82C0EFB8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C0EFBC: 419A00BC  beq cr6, 0x82c0f078
	if ctx.cr[6].eq {
	pc = 0x82C0F078; continue 'dispatch;
	}
	// 82C0EFC0: 815F0030  lwz r10, 0x30(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 82C0EFC4: 7D2B5050  subf r9, r11, r10
	ctx.r[9].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 82C0EFC8: 7D2B1671  srawi. r11, r9, 2
	ctx.xer.ca = (ctx.r[9].s32 < 0) && ((ctx.r[9].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[9].s32 >> 2) as i64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82C0EFCC: 418200AC  beq 0x82c0f078
	if ctx.cr[0].eq {
	pc = 0x82C0F078; continue 'dispatch;
	}
	// 82C0EFD0: 839F0020  lwz r28, 0x20(r31)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82C0EFD4: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 82C0EFD8: 419A00A0  beq cr6, 0x82c0f078
	if ctx.cr[6].eq {
	pc = 0x82C0F078; continue 'dispatch;
	}
	// 82C0EFDC: 4BFF3C55  bl 0x82c02c30
	ctx.lr = 0x82C0EFE0;
	sub_82C02C30(ctx, base);
	// 82C0EFE0: 813F0074  lwz r9, 0x74(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(116 as u32) ) } as u64;
	// 82C0EFE4: 81030010  lwz r8, 0x10(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C0EFE8: 815F002C  lwz r10, 0x2c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 82C0EFEC: 7CE94396  divwu r7, r9, r8
	ctx.r[7].u32 = ctx.r[9].u32 / ctx.r[8].u32;
	// 82C0EFF0: 7D694396  divwu r11, r9, r8
	ctx.r[11].u32 = ctx.r[9].u32 / ctx.r[8].u32;
	// 82C0EFF4: 7CC741D6  mullw r6, r7, r8
	ctx.r[6].s64 = (ctx.r[7].s32 as i64) * (ctx.r[8].s32 as i64);
	// 82C0EFF8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C0EFFC: 7D264850  subf r9, r6, r9
	ctx.r[9].s64 = ctx.r[9].s64 - ctx.r[6].s64;
	// 82C0F000: 419A0018  beq cr6, 0x82c0f018
	if ctx.cr[6].eq {
	pc = 0x82C0F018; continue 'dispatch;
	}
	// 82C0F004: 811F0030  lwz r8, 0x30(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 82C0F008: 7CEA4050  subf r7, r10, r8
	ctx.r[7].s64 = ctx.r[8].s64 - ctx.r[10].s64;
	// 82C0F00C: 7CE81670  srawi r8, r7, 2
	ctx.xer.ca = (ctx.r[7].s32 < 0) && ((ctx.r[7].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[8].s64 = (ctx.r[7].s32 >> 2) as i64;
	// 82C0F010: 7F0B4040  cmplw cr6, r11, r8
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82C0F014: 41980008  blt cr6, 0x82c0f01c
	if ctx.cr[6].lt {
	pc = 0x82C0F01C; continue 'dispatch;
	}
	// 82C0F018: 0FE00016  twui r0, 0x16
	// twui: trap word unsigned immediate — TODO: implement trap semantics
	// 82C0F01C: 5568103A  slwi r8, r11, 2
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82C0F020: 80FF007C  lwz r7, 0x7c(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(124 as u32) ) } as u64;
	// 82C0F024: 80DF0078  lwz r6, 0x78(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(120 as u32) ) } as u64;
	// 82C0F028: 817C0018  lwz r11, 0x18(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(24 as u32) ) } as u64;
	// 82C0F02C: 7D2749D6  mullw r9, r7, r9
	ctx.r[9].s64 = (ctx.r[7].s32 as i64) * (ctx.r[9].s32 as i64);
	// 82C0F030: 7D48502E  lwzx r10, r8, r10
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82C0F034: 7FC63850  subf r30, r6, r7
	ctx.r[30].s64 = ctx.r[7].s64 - ctx.r[6].s64;
	// 82C0F038: 7FAA4A14  add r29, r10, r9
	ctx.r[29].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82C0F03C: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82C0F040: 40990008  ble cr6, 0x82c0f048
	if !ctx.cr[6].gt {
	pc = 0x82C0F048; continue 'dispatch;
	}
	// 82C0F044: 7D7E5B78  mr r30, r11
	ctx.r[30].u64 = ctx.r[11].u64;
	// 82C0F048: 389C000C  addi r4, r28, 0xc
	ctx.r[4].s64 = ctx.r[28].s64 + 12;
	// 82C0F04C: 387F003C  addi r3, r31, 0x3c
	ctx.r[3].s64 = ctx.r[31].s64 + 60;
	// 82C0F050: 48002C79  bl 0x82c11cc8
	ctx.lr = 0x82C0F054;
	sub_82C11CC8(ctx, base);
	// 82C0F054: 817C0014  lwz r11, 0x14(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(20 as u32) ) } as u64;
	// 82C0F058: 815F0078  lwz r10, 0x78(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(120 as u32) ) } as u64;
	// 82C0F05C: 387F0038  addi r3, r31, 0x38
	ctx.r[3].s64 = ctx.r[31].s64 + 56;
	// 82C0F060: 93BF0048  stw r29, 0x48(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[29].u32 ) };
	// 82C0F064: 93DF0050  stw r30, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 82C0F068: 917F0044  stw r11, 0x44(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(68 as u32), ctx.r[11].u32 ) };
	// 82C0F06C: 915F004C  stw r10, 0x4c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(76 as u32), ctx.r[10].u32 ) };
	// 82C0F070: 48040781  bl 0x82c4f7f0
	ctx.lr = 0x82C0F074;
	sub_82C4F7F0(ctx, base);
	// 82C0F074: 987F009E  stb r3, 0x9e(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(158 as u32), ctx.r[3].u8 ) };
	// 82C0F078: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82C0F07C: 4809A3DC  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C0F080(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82C0F080 size=172
    let mut pc: u32 = 0x82C0F080;
    'dispatch: loop {
        match pc {
            0x82C0F080 => {
    //   block [0x82C0F080..0x82C0F12C)
	// 82C0F080: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C0F084: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C0F088: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82C0F08C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C0F090: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C0F094: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C0F098: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82C0F09C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82C0F0A0: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C0F0A4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C0F0A8: 814B0034  lwz r10, 0x34(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 82C0F0AC: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C0F0B0: 4E800421  bctrl
	ctx.lr = 0x82C0F0B4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C0F0B4: 3D20820A  lis r9, -0x7df6
	ctx.r[9].s64 = -2113273856;
	// 82C0F0B8: C1A10060  lfs f13, 0x60(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82C0F0BC: C0099054  lfs f0, -0x6fac(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-28588 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C0F0C0: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 82C0F0C4: 4098002C  bge cr6, 0x82c0f0f0
	if !ctx.cr[6].lt {
	pc = 0x82C0F0F0; continue 'dispatch;
	}
	// 82C0F0C8: 817F0090  lwz r11, 0x90(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(144 as u32) ) } as u64;
	// 82C0F0CC: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82C0F0D0: 917F0090  stw r11, 0x90(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(144 as u32), ctx.r[11].u32 ) };
	// 82C0F0D4: 2B0B0BB8  cmplwi cr6, r11, 0xbb8
	ctx.cr[6].compare_u32(ctx.r[11].u32, 3000 as u32, &mut ctx.xer);
	// 82C0F0D8: 4099000C  ble cr6, 0x82c0f0e4
	if !ctx.cr[6].gt {
	pc = 0x82C0F0E4; continue 'dispatch;
	}
	// 82C0F0DC: 39600BB9  li r11, 0xbb9
	ctx.r[11].s64 = 3001;
	// 82C0F0E0: 917F0090  stw r11, 0x90(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(144 as u32), ctx.r[11].u32 ) };
	// 82C0F0E4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82C0F0E8: 917F0094  stw r11, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[11].u32 ) };
	// 82C0F0EC: 48000028  b 0x82c0f114
	pc = 0x82C0F114; continue 'dispatch;
	// 82C0F0F0: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 82C0F0F4: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82C0F0F8: 917F0094  stw r11, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[11].u32 ) };
	// 82C0F0FC: 2B0B0064  cmplwi cr6, r11, 0x64
	ctx.cr[6].compare_u32(ctx.r[11].u32, 100 as u32, &mut ctx.xer);
	// 82C0F100: 4099000C  ble cr6, 0x82c0f10c
	if !ctx.cr[6].gt {
	pc = 0x82C0F10C; continue 'dispatch;
	}
	// 82C0F104: 39600065  li r11, 0x65
	ctx.r[11].s64 = 101;
	// 82C0F108: 917F0094  stw r11, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[11].u32 ) };
	// 82C0F10C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82C0F110: 917F0090  stw r11, 0x90(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(144 as u32), ctx.r[11].u32 ) };
	// 82C0F114: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82C0F118: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C0F11C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C0F120: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82C0F124: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C0F128: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C0F130(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C0F130 size=20
    let mut pc: u32 = 0x82C0F130;
    'dispatch: loop {
        match pc {
            0x82C0F130 => {
    //   block [0x82C0F130..0x82C0F144)
	// 82C0F130: 80630010  lwz r3, 0x10(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C0F134: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C0F138: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C0F13C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C0F140: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C0F148(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C0F148 size=104
    let mut pc: u32 = 0x82C0F148;
    'dispatch: loop {
        match pc {
            0x82C0F148 => {
    //   block [0x82C0F148..0x82C0F1B0)
	// 82C0F148: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C0F14C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C0F150: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C0F154: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C0F158: 81630070  lwz r11, 0x70(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(112 as u32) ) } as u64;
	// 82C0F15C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82C0F160: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82C0F164: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82C0F168: 8943009D  lbz r10, 0x9d(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(157 as u32) ) } as u64;
	// 82C0F16C: 995F0004  stb r10, 4(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u8 ) };
	// 82C0F170: E9230060  ld r9, 0x60(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(96 as u32) ) };
	// 82C0F174: F93F0008  std r9, 8(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[9].u64 ) };
	// 82C0F178: E9030068  ld r8, 0x68(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(104 as u32) ) };
	// 82C0F17C: F91F0010  std r8, 0x10(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[8].u64 ) };
	// 82C0F180: 80630010  lwz r3, 0x10(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C0F184: 80E30000  lwz r7, 0(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C0F188: 80C70038  lwz r6, 0x38(r7)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(56 as u32) ) } as u64;
	// 82C0F18C: 7CC903A6  mtctr r6
	ctx.ctr.u64 = ctx.r[6].u64;
	// 82C0F190: 4E800421  bctrl
	ctx.lr = 0x82C0F194;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C0F194: E8A10058  ld r5, 0x58(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 82C0F198: F8BF0018  std r5, 0x18(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[5].u64 ) };
	// 82C0F19C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82C0F1A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C0F1A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C0F1A8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C0F1AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C0F1B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C0F1B0 size=12
    let mut pc: u32 = 0x82C0F1B0;
    'dispatch: loop {
        match pc {
            0x82C0F1B0 => {
    //   block [0x82C0F1B0..0x82C0F1BC)
	// 82C0F1B0: 80630010  lwz r3, 0x10(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C0F1B4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C0F1B8: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C0F1BC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C0F1BC size=16
    let mut pc: u32 = 0x82C0F1BC;
    'dispatch: loop {
        match pc {
            0x82C0F1BC => {
    //   block [0x82C0F1BC..0x82C0F1CC)
	// 82C0F1BC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C0F1C0: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C0F1C4: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C0F1C8: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C0F1CC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C0F1CC size=4
    let mut pc: u32 = 0x82C0F1CC;
    'dispatch: loop {
        match pc {
            0x82C0F1CC => {
    //   block [0x82C0F1CC..0x82C0F1D0)
	// 82C0F1CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C0F1D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C0F1D0 size=16
    let mut pc: u32 = 0x82C0F1D0;
    'dispatch: loop {
        match pc {
            0x82C0F1D0 => {
    //   block [0x82C0F1D0..0x82C0F1E0)
	// 82C0F1D0: 80630010  lwz r3, 0x10(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C0F1D4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C0F1D8: 409A0008  bne cr6, 0x82c0f1e0
	if !ctx.cr[6].eq {
		sub_82C0F1E0(ctx, base);
		return;
	}
	// 82C0F1DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C0F1E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C0F1E0 size=16
    let mut pc: u32 = 0x82C0F1E0;
    'dispatch: loop {
        match pc {
            0x82C0F1E0 => {
    //   block [0x82C0F1E0..0x82C0F1F0)
	// 82C0F1E0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C0F1E4: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C0F1E8: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C0F1EC: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C0F1F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C0F1F0 size=16
    let mut pc: u32 = 0x82C0F1F0;
    'dispatch: loop {
        match pc {
            0x82C0F1F0 => {
    //   block [0x82C0F1F0..0x82C0F200)
	// 82C0F1F0: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C0F1F4: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82C0F1F8: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82C0F1FC: 4C820020  bnelr
	if !ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C0F200(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C0F200 size=20
    let mut pc: u32 = 0x82C0F200;
    'dispatch: loop {
        match pc {
            0x82C0F200 => {
    //   block [0x82C0F200..0x82C0F214)
	// 82C0F200: 80630004  lwz r3, 4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C0F204: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C0F208: 814B0044  lwz r10, 0x44(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(68 as u32) ) } as u64;
	// 82C0F20C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C0F210: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C0F214(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C0F214 size=4
    let mut pc: u32 = 0x82C0F214;
    'dispatch: loop {
        match pc {
            0x82C0F214 => {
    //   block [0x82C0F214..0x82C0F218)
	// 82C0F214: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C0F218(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C0F218 size=304
    let mut pc: u32 = 0x82C0F218;
    'dispatch: loop {
        match pc {
            0x82C0F218 => {
    //   block [0x82C0F218..0x82C0F348)
	// 82C0F218: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C0F21C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C0F220: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82C0F224: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C0F228: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C0F22C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C0F230: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82C0F234: 7CAA2B78  mr r10, r5
	ctx.r[10].u64 = ctx.r[5].u64;
	// 82C0F238: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82C0F23C: 7D2B5396  divwu r9, r11, r10
	ctx.r[9].u32 = ctx.r[11].u32 / ctx.r[10].u32;
	// 82C0F240: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82C0F244: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82C0F248: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82C0F24C: 3CC08201  lis r6, -0x7dff
	ctx.r[6].s64 = -2113863680;
	// 82C0F250: 9BDF0019  stb r30, 0x19(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(25 as u32), ctx.r[30].u8 ) };
	// 82C0F254: 3CA00001  lis r5, 1
	ctx.r[5].s64 = 65536;
	// 82C0F258: 913F0008  stw r9, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82C0F25C: 38C6AA20  addi r6, r6, -0x55e0
	ctx.r[6].s64 = ctx.r[6].s64 + -21984;
	// 82C0F260: 911F0010  stw r8, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[8].u32 ) };
	// 82C0F264: 7D445378  mr r4, r10
	ctx.r[4].u64 = ctx.r[10].u64;
	// 82C0F268: 913F0014  stw r9, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[9].u32 ) };
	// 82C0F26C: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82C0F270: 98FF0018  stb r7, 0x18(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[7].u8 ) };
	// 82C0F274: 4BF441C5  bl 0x82b53438
	ctx.lr = 0x82C0F278;
	sub_82B53438(ctx, base);
	// 82C0F278: 57C5063E  clrlwi r5, r30, 0x18
	ctx.r[5].u64 = ctx.r[30].u32 as u64 & 0x000000FFu64;
	// 82C0F27C: 907F000C  stw r3, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[3].u32 ) };
	// 82C0F280: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 82C0F284: 419A000C  beq cr6, 0x82c0f290
	if ctx.cr[6].eq {
	pc = 0x82C0F290; continue 'dispatch;
	}
	// 82C0F288: 809F0004  lwz r4, 4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C0F28C: 4BF2E10D  bl 0x82b3d398
	ctx.lr = 0x82C0F290;
	sub_82B3D398(ctx, base);
	// 82C0F290: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C0F294: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C0F298: 409A0008  bne cr6, 0x82c0f2a0
	if !ctx.cr[6].eq {
	pc = 0x82C0F2A0; continue 'dispatch;
	}
	// 82C0F29C: 0FE00016  twui r0, 0x16
	// twui: trap word unsigned immediate — TODO: implement trap semantics
	// 82C0F2A0: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C0F2A4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82C0F2A8: 352AFFFF  addic. r9, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[9].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82C0F2AC: 41820048  beq 0x82c0f2f4
	if ctx.cr[0].eq {
	pc = 0x82C0F2F4; continue 'dispatch;
	}
	// 82C0F2B0: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C0F2B4: 3BCB0001  addi r30, r11, 1
	ctx.r[30].s64 = ctx.r[11].s64 + 1;
	// 82C0F2B8: 815F000C  lwz r10, 0xc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C0F2BC: 7D6959D6  mullw r11, r9, r11
	ctx.r[11].s64 = (ctx.r[9].s32 as i64) * (ctx.r[11].s32 as i64);
	// 82C0F2C0: 7FCB512E  stwx r30, r11, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[30].u32) };
	// 82C0F2C4: 891F0018  lbz r8, 0x18(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82C0F2C8: 7C6B5214  add r3, r11, r10
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82C0F2CC: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 82C0F2D0: 419A0010  beq cr6, 0x82c0f2e0
	if ctx.cr[6].eq {
	pc = 0x82C0F2E0; continue 'dispatch;
	}
	// 82C0F2D4: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 82C0F2D8: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C0F2DC: 480B2B85  bl 0x82cc1e60
	ctx.lr = 0x82C0F2E0;
	sub_82CC1E60(ctx, base);
	// 82C0F2E0: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C0F2E4: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 82C0F2E8: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82C0F2EC: 7F1E5040  cmplw cr6, r30, r10
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82C0F2F0: 4198FFC0  blt cr6, 0x82c0f2b0
	if ctx.cr[6].lt {
	pc = 0x82C0F2B0; continue 'dispatch;
	}
	// 82C0F2F4: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C0F2F8: 3920FFFF  li r9, -1
	ctx.r[9].s64 = -1;
	// 82C0F2FC: 811F0000  lwz r8, 0(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C0F300: 38EBFFFF  addi r7, r11, -1
	ctx.r[7].s64 = ctx.r[11].s64 + -1;
	// 82C0F304: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C0F308: 7D4741D6  mullw r10, r7, r8
	ctx.r[10].s64 = (ctx.r[7].s32 as i64) * (ctx.r[8].s32 as i64);
	// 82C0F30C: 7D2A592E  stwx r9, r10, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32), ctx.r[9].u32) };
	// 82C0F310: 88DF0018  lbz r6, 0x18(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82C0F314: 7C6A5A14  add r3, r10, r11
	ctx.r[3].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82C0F318: 2B060000  cmplwi cr6, r6, 0
	ctx.cr[6].compare_u32(ctx.r[6].u32, 0 as u32, &mut ctx.xer);
	// 82C0F31C: 419A0010  beq cr6, 0x82c0f32c
	if ctx.cr[6].eq {
	pc = 0x82C0F32C; continue 'dispatch;
	}
	// 82C0F320: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 82C0F324: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C0F328: 480B2B39  bl 0x82cc1e60
	ctx.lr = 0x82C0F32C;
	sub_82CC1E60(ctx, base);
	// 82C0F32C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C0F330: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C0F334: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C0F338: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C0F33C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82C0F340: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C0F344: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C0F348(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C0F348 size=140
    let mut pc: u32 = 0x82C0F348;
    'dispatch: loop {
        match pc {
            0x82C0F348 => {
    //   block [0x82C0F348..0x82C0F3D4)
	// 82C0F348: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C0F34C: 4809A0C1  bl 0x82ca940c
	ctx.lr = 0x82C0F350;
	sub_82CA93D0(ctx, base);
	// 82C0F350: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C0F354: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C0F358: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C0F35C: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C0F360: 7D2B2050  subf r9, r11, r4
	ctx.r[9].s64 = ctx.r[4].s64 - ctx.r[11].s64;
	// 82C0F364: 811F0010  lwz r8, 0x10(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C0F368: 7FC95396  divwu r30, r9, r10
	ctx.r[30].u32 = ctx.r[9].u32 / ctx.r[10].u32;
	// 82C0F36C: 7D4AF1D6  mullw r10, r10, r30
	ctx.r[10].s64 = (ctx.r[10].s32 as i64) * (ctx.r[30].s32 as i64);
	// 82C0F370: 7D0A592E  stwx r8, r10, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32), ctx.r[8].u32) };
	// 82C0F374: 7FAA5A14  add r29, r10, r11
	ctx.r[29].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82C0F378: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82C0F37C: 4BFFFA55  bl 0x82c0edd0
	ctx.lr = 0x82C0F380;
	sub_82C0EDD0(ctx, base);
	// 82C0F380: 5467063E  clrlwi r7, r3, 0x18
	ctx.r[7].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82C0F384: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 82C0F388: 409A0008  bne cr6, 0x82c0f390
	if !ctx.cr[6].eq {
	pc = 0x82C0F390; continue 'dispatch;
	}
	// 82C0F38C: 0FE00016  twui r0, 0x16
	// twui: trap word unsigned immediate — TODO: implement trap semantics
	// 82C0F390: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C0F394: 93DF0010  stw r30, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[30].u32 ) };
	// 82C0F398: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82C0F39C: 41980008  blt cr6, 0x82c0f3a4
	if ctx.cr[6].lt {
	pc = 0x82C0F3A4; continue 'dispatch;
	}
	// 82C0F3A0: 0FE00016  twui r0, 0x16
	// twui: trap word unsigned immediate — TODO: implement trap semantics
	// 82C0F3A4: 897F0018  lbz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82C0F3A8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C0F3AC: 419A0014  beq cr6, 0x82c0f3c0
	if ctx.cr[6].eq {
	pc = 0x82C0F3C0; continue 'dispatch;
	}
	// 82C0F3B0: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 82C0F3B4: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C0F3B8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82C0F3BC: 480B2AA5  bl 0x82cc1e60
	ctx.lr = 0x82C0F3C0;
	sub_82CC1E60(ctx, base);
	// 82C0F3C0: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82C0F3C4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82C0F3C8: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82C0F3CC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C0F3D0: 4809A08C  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C0F3D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C0F3D8 size=20
    let mut pc: u32 = 0x82C0F3D8;
    'dispatch: loop {
        match pc {
            0x82C0F3D8 => {
    //   block [0x82C0F3D8..0x82C0F3EC)
	// 82C0F3D8: 80630004  lwz r3, 4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C0F3DC: 81030004  lwz r8, 4(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C0F3E0: 89680011  lbz r11, 0x11(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[8].u32.wrapping_add(17 as u32) ) } as u64;
	// 82C0F3E4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C0F3E8: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C0F3EC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C0F3EC size=44
    let mut pc: u32 = 0x82C0F3EC;
    'dispatch: loop {
        match pc {
            0x82C0F3EC => {
    //   block [0x82C0F3EC..0x82C0F418)
	// 82C0F3EC: 80C40000  lwz r6, 0(r4)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C0F3F0: 80E6008C  lwz r7, 0x8c(r6)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(140 as u32) ) } as u64;
	// 82C0F3F4: 8168000C  lwz r11, 0xc(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C0F3F8: 814B008C  lwz r10, 0x8c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(140 as u32) ) } as u64;
	// 82C0F3FC: 7F0A3840  cmplw cr6, r10, r7
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[7].u32, &mut ctx.xer);
	// 82C0F400: 419A0028  beq cr6, 0x82c0f428
	if ctx.cr[6].eq {
		sub_82C0F428(ctx, base);
		return;
	}
	// 82C0F404: 812B0098  lwz r9, 0x98(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(152 as u32) ) } as u64;
	// 82C0F408: 2F090002  cmpwi cr6, r9, 2
	ctx.cr[6].compare_i32(ctx.r[9].s32, 2, &mut ctx.xer);
	// 82C0F40C: 409A000C  bne cr6, 0x82c0f418
	if !ctx.cr[6].eq {
		sub_82C0F418(ctx, base);
		return;
	}
	// 82C0F410: 7D675010  subfc r11, r7, r10
	ctx.xer.ca = ctx.r[10].u32 >= ctx.r[7].u32;
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[7].s64;
	// 82C0F414: 48000018  b 0x82c0f42c
	sub_82C0F428(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C0F418(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C0F418 size=16
    let mut pc: u32 = 0x82C0F418;
    'dispatch: loop {
        match pc {
            0x82C0F418 => {
    //   block [0x82C0F418..0x82C0F428)
	// 82C0F418: 2F090001  cmpwi cr6, r9, 1
	ctx.cr[6].compare_i32(ctx.r[9].s32, 1, &mut ctx.xer);
	// 82C0F41C: 409A000C  bne cr6, 0x82c0f428
	if !ctx.cr[6].eq {
		sub_82C0F428(ctx, base);
		return;
	}
	// 82C0F420: 7D6A3810  subfc r11, r10, r7
	ctx.xer.ca = ctx.r[7].u32 >= ctx.r[10].u32;
	ctx.r[11].s64 = ctx.r[7].s64 - ctx.r[10].s64;
	// 82C0F424: 48000008  b 0x82c0f42c
	sub_82C0F428(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C0F428(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82C0F428 size=32
    let mut pc: u32 = 0x82C0F428;
    'dispatch: loop {
        match pc {
            0x82C0F428 => {
    //   block [0x82C0F428..0x82C0F448)
	// 82C0F428: 7D665810  subfc r11, r6, r11
	ctx.xer.ca = ctx.r[11].u32 >= ctx.r[6].u32;
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[6].s64;
	// 82C0F42C: 7D4B5910  subfe r10, r11, r11
	let x = (!ctx.r[11].u32);
	let y = ctx.r[11].u32;
	let s = x.wrapping_add(y);
	let res = s.wrapping_add(ctx.xer.ca as u32);
	tmp.u8 = (s < x) as u8 | (res < s) as u8;
	ctx.r[10].u32 = res;
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	ctx.xer.ca = (tmp.u8 != 0);
	// 82C0F430: 554B07FE  clrlwi r11, r10, 0x1f
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0x00000001u64;
	// 82C0F434: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82C0F438: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C0F43C: 419A000C  beq cr6, 0x82c0f448
	if ctx.cr[6].eq {
		sub_82C0F448(ctx, base);
		return;
	}
	// 82C0F440: 81080008  lwz r8, 8(r8)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C0F444: 4800000C  b 0x82c0f450
	sub_82C0F448(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C0F448(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C0F448 size=24
    let mut pc: u32 = 0x82C0F448;
    'dispatch: loop {
        match pc {
            0x82C0F448 => {
    //   block [0x82C0F448..0x82C0F460)
	// 82C0F448: 7D034378  mr r3, r8
	ctx.r[3].u64 = ctx.r[8].u64;
	// 82C0F44C: 81080000  lwz r8, 0(r8)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C0F450: 89680011  lbz r11, 0x11(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[8].u32.wrapping_add(17 as u32) ) } as u64;
	// 82C0F454: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C0F458: 419AFF9C  beq cr6, 0x82c0f3f4
	if ctx.cr[6].eq {
		sub_82C0F3EC(ctx, base);
		return;
	}
	// 82C0F45C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C0F460(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C0F460 size=20
    let mut pc: u32 = 0x82C0F460;
    'dispatch: loop {
        match pc {
            0x82C0F460 => {
    //   block [0x82C0F460..0x82C0F474)
	// 82C0F460: 80630004  lwz r3, 4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C0F464: 81030004  lwz r8, 4(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C0F468: 89680011  lbz r11, 0x11(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[8].u32.wrapping_add(17 as u32) ) } as u64;
	// 82C0F46C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C0F470: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C0F474(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C0F474 size=44
    let mut pc: u32 = 0x82C0F474;
    'dispatch: loop {
        match pc {
            0x82C0F474 => {
    //   block [0x82C0F474..0x82C0F4A0)
	// 82C0F474: 80C40000  lwz r6, 0(r4)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C0F478: 80E6008C  lwz r7, 0x8c(r6)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(140 as u32) ) } as u64;
	// 82C0F47C: 8168000C  lwz r11, 0xc(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C0F480: 814B008C  lwz r10, 0x8c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(140 as u32) ) } as u64;
	// 82C0F484: 7F075040  cmplw cr6, r7, r10
	ctx.cr[6].compare_u32(ctx.r[7].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82C0F488: 419A0028  beq cr6, 0x82c0f4b0
	if ctx.cr[6].eq {
		sub_82C0F4B0(ctx, base);
		return;
	}
	// 82C0F48C: 81260098  lwz r9, 0x98(r6)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(152 as u32) ) } as u64;
	// 82C0F490: 2F090002  cmpwi cr6, r9, 2
	ctx.cr[6].compare_i32(ctx.r[9].s32, 2, &mut ctx.xer);
	// 82C0F494: 409A000C  bne cr6, 0x82c0f4a0
	if !ctx.cr[6].eq {
		sub_82C0F4A0(ctx, base);
		return;
	}
	// 82C0F498: 7D6A3810  subfc r11, r10, r7
	ctx.xer.ca = ctx.r[7].u32 >= ctx.r[10].u32;
	ctx.r[11].s64 = ctx.r[7].s64 - ctx.r[10].s64;
	// 82C0F49C: 48000018  b 0x82c0f4b4
	sub_82C0F4B0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C0F4A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C0F4A0 size=16
    let mut pc: u32 = 0x82C0F4A0;
    'dispatch: loop {
        match pc {
            0x82C0F4A0 => {
    //   block [0x82C0F4A0..0x82C0F4B0)
	// 82C0F4A0: 2F090001  cmpwi cr6, r9, 1
	ctx.cr[6].compare_i32(ctx.r[9].s32, 1, &mut ctx.xer);
	// 82C0F4A4: 409A000C  bne cr6, 0x82c0f4b0
	if !ctx.cr[6].eq {
		sub_82C0F4B0(ctx, base);
		return;
	}
	// 82C0F4A8: 7D675010  subfc r11, r7, r10
	ctx.xer.ca = ctx.r[10].u32 >= ctx.r[7].u32;
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[7].s64;
	// 82C0F4AC: 48000008  b 0x82c0f4b4
	sub_82C0F4B0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C0F4B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82C0F4B0 size=36
    let mut pc: u32 = 0x82C0F4B0;
    'dispatch: loop {
        match pc {
            0x82C0F4B0 => {
    //   block [0x82C0F4B0..0x82C0F4D4)
	// 82C0F4B0: 7D6B3010  subfc r11, r11, r6
	ctx.xer.ca = ctx.r[6].u32 >= ctx.r[11].u32;
	ctx.r[11].s64 = ctx.r[6].s64 - ctx.r[11].s64;
	// 82C0F4B4: 7D4B5910  subfe r10, r11, r11
	let x = (!ctx.r[11].u32);
	let y = ctx.r[11].u32;
	let s = x.wrapping_add(y);
	let res = s.wrapping_add(ctx.xer.ca as u32);
	tmp.u8 = (s < x) as u8 | (res < s) as u8;
	ctx.r[10].u32 = res;
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	ctx.xer.ca = (tmp.u8 != 0);
	// 82C0F4B8: 554B07FE  clrlwi r11, r10, 0x1f
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0x00000001u64;
	// 82C0F4BC: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82C0F4C0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C0F4C4: 419A0010  beq cr6, 0x82c0f4d4
	if ctx.cr[6].eq {
		sub_82C0F4D4(ctx, base);
		return;
	}
	// 82C0F4C8: 7D034378  mr r3, r8
	ctx.r[3].u64 = ctx.r[8].u64;
	// 82C0F4CC: 81080000  lwz r8, 0(r8)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C0F4D0: 48000008  b 0x82c0f4d8
	sub_82C0F4D4(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C0F4D4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C0F4D4 size=20
    let mut pc: u32 = 0x82C0F4D4;
    'dispatch: loop {
        match pc {
            0x82C0F4D4 => {
    //   block [0x82C0F4D4..0x82C0F4E8)
	// 82C0F4D4: 81080008  lwz r8, 8(r8)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C0F4D8: 89680011  lbz r11, 0x11(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[8].u32.wrapping_add(17 as u32) ) } as u64;
	// 82C0F4DC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C0F4E0: 419AFF9C  beq cr6, 0x82c0f47c
	if ctx.cr[6].eq {
		sub_82C0F474(ctx, base);
		return;
	}
	// 82C0F4E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C0F4E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C0F4E8 size=64
    let mut pc: u32 = 0x82C0F4E8;
    'dispatch: loop {
        match pc {
            0x82C0F4E8 => {
    //   block [0x82C0F4E8..0x82C0F528)
	// 82C0F4E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C0F4EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C0F4F0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C0F4F4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C0F4F8: 80840010  lwz r4, 0x10(r4)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C0F4FC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C0F500: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C0F504: 814B0014  lwz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82C0F508: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C0F50C: 4E800421  bctrl
	ctx.lr = 0x82C0F510;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C0F510: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C0F514: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82C0F518: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C0F51C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C0F520: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C0F524: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C0F528(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82C0F528 size=204
    let mut pc: u32 = 0x82C0F528;
    'dispatch: loop {
        match pc {
            0x82C0F528 => {
    //   block [0x82C0F528..0x82C0F5F4)
	// 82C0F528: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C0F52C: 48099EE1  bl 0x82ca940c
	ctx.lr = 0x82C0F530;
	sub_82CA93D0(ctx, base);
	// 82C0F530: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C0F534: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C0F538: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82C0F53C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C0F540: 814B001C  lwz r10, 0x1c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82C0F544: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C0F548: 4E800421  bctrl
	ctx.lr = 0x82C0F54C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C0F54C: 5469063E  clrlwi r9, r3, 0x18
	ctx.r[9].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82C0F550: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82C0F554: 409A0098  bne cr6, 0x82c0f5ec
	if !ctx.cr[6].eq {
	pc = 0x82C0F5EC; continue 'dispatch;
	}
	// 82C0F558: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C0F55C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C0F560: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C0F564: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C0F568: 4E800421  bctrl
	ctx.lr = 0x82C0F56C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C0F56C: 813F0018  lwz r9, 0x18(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82C0F570: 3BDF0018  addi r30, r31, 0x18
	ctx.r[30].s64 = ctx.r[31].s64 + 24;
	// 82C0F574: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82C0F578: 409A0030  bne cr6, 0x82c0f5a8
	if !ctx.cr[6].eq {
	pc = 0x82C0F5A8; continue 'dispatch;
	}
	// 82C0F57C: 809F0010  lwz r4, 0x10(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C0F580: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C0F584: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C0F588: 814B0010  lwz r10, 0x10(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C0F58C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C0F590: 4E800421  bctrl
	ctx.lr = 0x82C0F594;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C0F594: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82C0F598: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82C0F59C: 4800272D  bl 0x82c11cc8
	ctx.lr = 0x82C0F5A0;
	sub_82C11CC8(ctx, base);
	// 82C0F5A0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C0F5A4: 4BFF6955  bl 0x82c05ef8
	ctx.lr = 0x82C0F5A8;
	sub_82C05EF8(ctx, base);
	// 82C0F5A8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C0F5AC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C0F5B0: 814B003C  lwz r10, 0x3c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 82C0F5B4: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C0F5B8: 4E800421  bctrl
	ctx.lr = 0x82C0F5BC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C0F5BC: 5469063E  clrlwi r9, r3, 0x18
	ctx.r[9].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82C0F5C0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82C0F5C4: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82C0F5C8: 409A0008  bne cr6, 0x82c0f5d0
	if !ctx.cr[6].eq {
	pc = 0x82C0F5D0; continue 'dispatch;
	}
	// 82C0F5CC: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82C0F5D0: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C0F5D4: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82C0F5D8: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C0F5DC: C02B0C18  lfs f1, 0xc18(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3096 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82C0F5E0: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C0F5E4: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82C0F5E8: 4E800421  bctrl
	ctx.lr = 0x82C0F5EC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C0F5EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82C0F5F0: 48099E6C  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C0F5F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C0F5F8 size=380
    let mut pc: u32 = 0x82C0F5F8;
    'dispatch: loop {
        match pc {
            0x82C0F5F8 => {
    //   block [0x82C0F5F8..0x82C0F774)
	// 82C0F5F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C0F5FC: 48099E05  bl 0x82ca9400
	ctx.lr = 0x82C0F600;
	sub_82CA93D0(ctx, base);
	// 82C0F600: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C0F604: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C0F608: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82C0F60C: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 82C0F610: 9B5F009E  stb r26, 0x9e(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(158 as u32), ctx.r[26].u8 ) };
	// 82C0F614: 897D0024  lbz r11, 0x24(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(36 as u32) ) } as u64;
	// 82C0F618: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C0F61C: 409A0150  bne cr6, 0x82c0f76c
	if !ctx.cr[6].eq {
	pc = 0x82C0F76C; continue 'dispatch;
	}
	// 82C0F620: 83DF0020  lwz r30, 0x20(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82C0F624: 3B7F0020  addi r27, r31, 0x20
	ctx.r[27].s64 = ctx.r[31].s64 + 32;
	// 82C0F628: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82C0F62C: 419A0140  beq cr6, 0x82c0f76c
	if ctx.cr[6].eq {
	pc = 0x82C0F76C; continue 'dispatch;
	}
	// 82C0F630: 817D001C  lwz r11, 0x1c(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(28 as u32) ) } as u64;
	// 82C0F634: 815E0014  lwz r10, 0x14(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82C0F638: 813E0018  lwz r9, 0x18(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 82C0F63C: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82C0F640: 917E0014  stw r11, 0x14(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82C0F644: 815D0020  lwz r10, 0x20(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(32 as u32) ) } as u64;
	// 82C0F648: 7D0A5A14  add r8, r10, r11
	ctx.r[8].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82C0F64C: 911E0014  stw r8, 0x14(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), ctx.r[8].u32 ) };
	// 82C0F650: 80FD001C  lwz r7, 0x1c(r29)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(28 as u32) ) } as u64;
	// 82C0F654: 7CC74850  subf r6, r7, r9
	ctx.r[6].s64 = ctx.r[9].s64 - ctx.r[7].s64;
	// 82C0F658: 90DE0018  stw r6, 0x18(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), ctx.r[6].u32 ) };
	// 82C0F65C: 80BD0020  lwz r5, 0x20(r29)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(32 as u32) ) } as u64;
	// 82C0F660: 7C853050  subf r4, r5, r6
	ctx.r[4].s64 = ctx.r[6].s64 - ctx.r[5].s64;
	// 82C0F664: 909E0018  stw r4, 0x18(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), ctx.r[4].u32 ) };
	// 82C0F668: E87F0060  ld r3, 0x60(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) };
	// 82C0F66C: 815D001C  lwz r10, 0x1c(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(28 as u32) ) } as u64;
	// 82C0F670: 817F0078  lwz r11, 0x78(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(120 as u32) ) } as u64;
	// 82C0F674: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82C0F678: 917F0078  stw r11, 0x78(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(120 as u32), ctx.r[11].u32 ) };
	// 82C0F67C: 815D001C  lwz r10, 0x1c(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(28 as u32) ) } as u64;
	// 82C0F680: 7D2A1850  subf r9, r10, r3
	ctx.r[9].s64 = ctx.r[3].s64 - ctx.r[10].s64;
	// 82C0F684: F93F0060  std r9, 0x60(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[9].u64 ) };
	// 82C0F688: 811D0020  lwz r8, 0x20(r29)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(32 as u32) ) } as u64;
	// 82C0F68C: 7D0707B4  extsw r7, r8
	ctx.r[7].s64 = ctx.r[8].s32 as i64;
	// 82C0F690: 7D674850  subf r11, r7, r9
	ctx.r[11].s64 = ctx.r[9].s64 - ctx.r[7].s64;
	// 82C0F694: F97F0060  std r11, 0x60(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[11].u64 ) };
	// 82C0F698: 815E0018  lwz r10, 0x18(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 82C0F69C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C0F6A0: 419A0010  beq cr6, 0x82c0f6b0
	if ctx.cr[6].eq {
	pc = 0x82C0F6B0; continue 'dispatch;
	}
	// 82C0F6A4: 813D0018  lwz r9, 0x18(r29)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(24 as u32) ) } as u64;
	// 82C0F6A8: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82C0F6AC: 40990058  ble cr6, 0x82c0f704
	if !ctx.cr[6].gt {
	pc = 0x82C0F704; continue 'dispatch;
	}
	// 82C0F6B0: 794A0020  clrldi r10, r10, 0x20
	ctx.r[10].u64 = ctx.r[10].u64 & 0x00000000FFFFFFFFu64;
	// 82C0F6B4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82C0F6B8: 7D2A5850  subf r9, r10, r11
	ctx.r[9].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82C0F6BC: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82C0F6C0: F93F0060  std r9, 0x60(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[9].u64 ) };
	// 82C0F6C4: 4BFFF67D  bl 0x82c0ed40
	ctx.lr = 0x82C0F6C8;
	sub_82C0ED40(ctx, base);
	// 82C0F6C8: 807E0010  lwz r3, 0x10(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C0F6CC: 3B9E000C  addi r28, r30, 0xc
	ctx.r[28].s64 = ctx.r[30].s64 + 12;
	// 82C0F6D0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C0F6D4: 419A0014  beq cr6, 0x82c0f6e8
	if ctx.cr[6].eq {
	pc = 0x82C0F6E8; continue 'dispatch;
	}
	// 82C0F6D8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C0F6DC: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C0F6E0: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C0F6E4: 4E800421  bctrl
	ctx.lr = 0x82C0F6E8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C0F6E8: 935C0004  stw r26, 4(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(4 as u32), ctx.r[26].u32 ) };
	// 82C0F6EC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82C0F6F0: 935C0000  stw r26, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[26].u32 ) };
	// 82C0F6F4: 4BC360BD  bl 0x828457b0
	ctx.lr = 0x82C0F6F8;
	sub_828457B0(ctx, base);
	// 82C0F6F8: 817F0070  lwz r11, 0x70(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(112 as u32) ) } as u64;
	// 82C0F6FC: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82C0F700: 917F0070  stw r11, 0x70(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(112 as u32), ctx.r[11].u32 ) };
	// 82C0F704: 80BF0078  lwz r5, 0x78(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(120 as u32) ) } as u64;
	// 82C0F708: 817F007C  lwz r11, 0x7c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(124 as u32) ) } as u64;
	// 82C0F70C: 7F055840  cmplw cr6, r5, r11
	ctx.cr[6].compare_u32(ctx.r[5].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82C0F710: 419A0010  beq cr6, 0x82c0f720
	if ctx.cr[6].eq {
	pc = 0x82C0F720; continue 'dispatch;
	}
	// 82C0F714: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C0F718: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82C0F71C: 409A0050  bne cr6, 0x82c0f76c
	if !ctx.cr[6].eq {
	pc = 0x82C0F76C; continue 'dispatch;
	}
	// 82C0F720: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C0F724: 809D0010  lwz r4, 0x10(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C0F728: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C0F72C: 814B002C  lwz r10, 0x2c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 82C0F730: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C0F734: 4E800421  bctrl
	ctx.lr = 0x82C0F738;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C0F738: E95F0068  ld r10, 0x68(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) };
	// 82C0F73C: 817F0078  lwz r11, 0x78(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(120 as u32) ) } as u64;
	// 82C0F740: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C0F744: 7D2B5214  add r9, r11, r10
	ctx.r[9].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82C0F748: F93F0068  std r9, 0x68(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(104 as u32), ctx.r[9].u64 ) };
	// 82C0F74C: 4BFFF6DD  bl 0x82c0ee28
	ctx.lr = 0x82C0F750;
	sub_82C0EE28(ctx, base);
	// 82C0F750: 817F0074  lwz r11, 0x74(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(116 as u32) ) } as u64;
	// 82C0F754: 935F0078  stw r26, 0x78(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(120 as u32), ctx.r[26].u32 ) };
	// 82C0F758: 390B0001  addi r8, r11, 1
	ctx.r[8].s64 = ctx.r[11].s64 + 1;
	// 82C0F75C: 7CE81B96  divwu r7, r8, r3
	ctx.r[7].u32 = ctx.r[8].u32 / ctx.r[3].u32;
	// 82C0F760: 7CC719D6  mullw r6, r7, r3
	ctx.r[6].s64 = (ctx.r[7].s32 as i64) * (ctx.r[3].s32 as i64);
	// 82C0F764: 7CA64050  subf r5, r6, r8
	ctx.r[5].s64 = ctx.r[8].s64 - ctx.r[6].s64;
	// 82C0F768: 90BF0074  stw r5, 0x74(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(116 as u32), ctx.r[5].u32 ) };
	// 82C0F76C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82C0F770: 48099CE0  b 0x82ca9450
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C0F778(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C0F778 size=336
    let mut pc: u32 = 0x82C0F778;
    'dispatch: loop {
        match pc {
            0x82C0F778 => {
    //   block [0x82C0F778..0x82C0F8C8)
	// 82C0F778: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C0F77C: 48099C81  bl 0x82ca93fc
	ctx.lr = 0x82C0F780;
	sub_82CA93D0(ctx, base);
	// 82C0F780: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C0F784: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82C0F788: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82C0F78C: 807E0010  lwz r3, 0x10(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C0F790: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C0F794: 814B0038  lwz r10, 0x38(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) } as u64;
	// 82C0F798: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C0F79C: 4E800421  bctrl
	ctx.lr = 0x82C0F7A0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C0F7A0: 4BFF3491  bl 0x82c02c30
	ctx.lr = 0x82C0F7A4;
	sub_82C02C30(ctx, base);
	// 82C0F7A4: 4BFF348D  bl 0x82c02c30
	ctx.lr = 0x82C0F7A8;
	sub_82C02C30(ctx, base);
	// 82C0F7A8: 813E0088  lwz r9, 0x88(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(136 as u32) ) } as u64;
	// 82C0F7AC: 8101006C  lwz r8, 0x6c(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(108 as u32) ) } as u64;
	// 82C0F7B0: 3B7E0020  addi r27, r30, 0x20
	ctx.r[27].s64 = ctx.r[30].s64 + 32;
	// 82C0F7B4: 80E10068  lwz r7, 0x68(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 82C0F7B8: 80C30010  lwz r6, 0x10(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C0F7BC: 7D674050  subf r11, r7, r8
	ctx.r[11].s64 = ctx.r[8].s64 - ctx.r[7].s64;
	// 82C0F7C0: 83FE0020  lwz r31, 0x20(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 82C0F7C4: 7F2649D6  mullw r25, r6, r9
	ctx.r[25].s64 = (ctx.r[6].s32 as i64) * (ctx.r[9].s32 as i64);
	// 82C0F7C8: 7CABCA15  add. r5, r11, r25
	ctx.r[5].u64 = ctx.r[11].u64 + ctx.r[25].u64;
	ctx.cr[0].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 82C0F7CC: 418200F4  beq 0x82c0f8c0
	if ctx.cr[0].eq {
	pc = 0x82C0F8C0; continue 'dispatch;
	}
	// 82C0F7D0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82C0F7D4: 419A00EC  beq cr6, 0x82c0f8c0
	if ctx.cr[6].eq {
	pc = 0x82C0F8C0; continue 'dispatch;
	}
	// 82C0F7D8: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 82C0F7DC: 7F5CD378  mr r28, r26
	ctx.r[28].u64 = ctx.r[26].u64;
	// 82C0F7E0: 817E007C  lwz r11, 0x7c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(124 as u32) ) } as u64;
	// 82C0F7E4: 7F1C5840  cmplw cr6, r28, r11
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82C0F7E8: 40980088  bge cr6, 0x82c0f870
	if !ctx.cr[6].lt {
	pc = 0x82C0F870; continue 'dispatch;
	}
	// 82C0F7EC: 815F0018  lwz r10, 0x18(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82C0F7F0: 7D7C5850  subf r11, r28, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[28].s64;
	// 82C0F7F4: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82C0F7F8: 40990008  ble cr6, 0x82c0f800
	if !ctx.cr[6].gt {
	pc = 0x82C0F800; continue 'dispatch;
	}
	// 82C0F7FC: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 82C0F800: 813F0014  lwz r9, 0x14(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82C0F804: 7D4B5051  subf. r10, r11, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82C0F808: 7F8BE214  add r28, r11, r28
	ctx.r[28].u64 = ctx.r[11].u64 + ctx.r[28].u64;
	// 82C0F80C: 7D2B4A14  add r9, r11, r9
	ctx.r[9].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82C0F810: 915F0018  stw r10, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[10].u32 ) };
	// 82C0F814: 913F0014  stw r9, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[9].u32 ) };
	// 82C0F818: 40820050  bne 0x82c0f868
	if !ctx.cr[0].eq {
	pc = 0x82C0F868; continue 'dispatch;
	}
	// 82C0F81C: 817E0070  lwz r11, 0x70(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(112 as u32) ) } as u64;
	// 82C0F820: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82C0F824: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82C0F828: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82C0F82C: 917E0070  stw r11, 0x70(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(112 as u32), ctx.r[11].u32 ) };
	// 82C0F830: 4BFFF511  bl 0x82c0ed40
	ctx.lr = 0x82C0F834;
	sub_82C0ED40(ctx, base);
	// 82C0F834: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C0F838: 3BBF000C  addi r29, r31, 0xc
	ctx.r[29].s64 = ctx.r[31].s64 + 12;
	// 82C0F83C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C0F840: 419A0014  beq cr6, 0x82c0f854
	if ctx.cr[6].eq {
	pc = 0x82C0F854; continue 'dispatch;
	}
	// 82C0F844: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C0F848: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C0F84C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C0F850: 4E800421  bctrl
	ctx.lr = 0x82C0F854;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C0F854: 935D0004  stw r26, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[26].u32 ) };
	// 82C0F858: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C0F85C: 935D0000  stw r26, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[26].u32 ) };
	// 82C0F860: 4BC35F51  bl 0x828457b0
	ctx.lr = 0x82C0F864;
	sub_828457B0(ctx, base);
	// 82C0F864: 83FB0000  lwz r31, 0(r27)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C0F868: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82C0F86C: 409AFF74  bne cr6, 0x82c0f7e0
	if !ctx.cr[6].eq {
	pc = 0x82C0F7E0; continue 'dispatch;
	}
	// 82C0F870: 807E0010  lwz r3, 0x10(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C0F874: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82C0F878: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82C0F87C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C0F880: 814B002C  lwz r10, 0x2c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 82C0F884: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C0F888: 4E800421  bctrl
	ctx.lr = 0x82C0F88C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C0F88C: 815E0074  lwz r10, 0x74(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(116 as u32) ) } as u64;
	// 82C0F890: E93E0068  ld r9, 0x68(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[30].u32.wrapping_add(104 as u32) ) };
	// 82C0F894: 7B8B0020  clrldi r11, r28, 0x20
	ctx.r[11].u64 = ctx.r[28].u64 & 0x00000000FFFFFFFFu64;
	// 82C0F898: 390A0001  addi r8, r10, 1
	ctx.r[8].s64 = ctx.r[10].s64 + 1;
	// 82C0F89C: E87E0060  ld r3, 0x60(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[30].u32.wrapping_add(96 as u32) ) };
	// 82C0F8A0: 7CEB4A14  add r7, r11, r9
	ctx.r[7].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82C0F8A4: 7CC8CB96  divwu r6, r8, r25
	ctx.r[6].u32 = ctx.r[8].u32 / ctx.r[25].u32;
	// 82C0F8A8: 7D6B1850  subf r11, r11, r3
	ctx.r[11].s64 = ctx.r[3].s64 - ctx.r[11].s64;
	// 82C0F8AC: F8FE0068  std r7, 0x68(r30)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[30].u32.wrapping_add(104 as u32), ctx.r[7].u64 ) };
	// 82C0F8B0: 7CA6C9D6  mullw r5, r6, r25
	ctx.r[5].s64 = (ctx.r[6].s32 as i64) * (ctx.r[25].s32 as i64);
	// 82C0F8B4: F97E0060  std r11, 0x60(r30)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[30].u32.wrapping_add(96 as u32), ctx.r[11].u64 ) };
	// 82C0F8B8: 7C854050  subf r4, r5, r8
	ctx.r[4].s64 = ctx.r[8].s64 - ctx.r[5].s64;
	// 82C0F8BC: 909E0074  stw r4, 0x74(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(116 as u32), ctx.r[4].u32 ) };
	// 82C0F8C0: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 82C0F8C4: 48099B88  b 0x82ca944c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C0F8C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C0F8C8 size=160
    let mut pc: u32 = 0x82C0F8C8;
    'dispatch: loop {
        match pc {
            0x82C0F8C8 => {
    //   block [0x82C0F8C8..0x82C0F968)
	// 82C0F8C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C0F8CC: 48099B39  bl 0x82ca9404
	ctx.lr = 0x82C0F8D0;
	sub_82CA93D0(ctx, base);
	// 82C0F8D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C0F8D4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C0F8D8: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82C0F8DC: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82C0F8E0: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82C0F8E4: 897F009D  lbz r11, 0x9d(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(157 as u32) ) } as u64;
	// 82C0F8E8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C0F8EC: 409A0074  bne cr6, 0x82c0f960
	if !ctx.cr[6].eq {
	pc = 0x82C0F960; continue 'dispatch;
	}
	// 82C0F8F0: 3860001C  li r3, 0x1c
	ctx.r[3].s64 = 28;
	// 82C0F8F4: 4B60F965  bl 0x8221f258
	ctx.lr = 0x82C0F8F8;
	sub_8221F258(ctx, base);
	// 82C0F8F8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C0F8FC: 419A0024  beq cr6, 0x82c0f920
	if ctx.cr[6].eq {
	pc = 0x82C0F920; continue 'dispatch;
	}
	// 82C0F900: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82C0F904: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82C0F908: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82C0F90C: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82C0F910: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82C0F914: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82C0F918: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82C0F91C: 48000008  b 0x82c0f924
	pc = 0x82C0F924; continue 'dispatch;
	// 82C0F920: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82C0F924: 937E0014  stw r27, 0x14(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), ctx.r[27].u32 ) };
	// 82C0F928: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82C0F92C: 93BE0018  stw r29, 0x18(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), ctx.r[29].u32 ) };
	// 82C0F930: 387E000C  addi r3, r30, 0xc
	ctx.r[3].s64 = ctx.r[30].s64 + 12;
	// 82C0F934: 48002395  bl 0x82c11cc8
	ctx.lr = 0x82C0F938;
	sub_82C11CC8(ctx, base);
	// 82C0F938: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82C0F93C: 387F0020  addi r3, r31, 0x20
	ctx.r[3].s64 = ctx.r[31].s64 + 32;
	// 82C0F940: 480485D1  bl 0x82c57f10
	ctx.lr = 0x82C0F944;
	sub_82C57F10(ctx, base);
	// 82C0F944: 813F0070  lwz r9, 0x70(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(112 as u32) ) } as u64;
	// 82C0F948: E95F0060  ld r10, 0x60(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) };
	// 82C0F94C: 7BAB0020  clrldi r11, r29, 0x20
	ctx.r[11].u64 = ctx.r[29].u64 & 0x00000000FFFFFFFFu64;
	// 82C0F950: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82C0F954: 7D0B5214  add r8, r11, r10
	ctx.r[8].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82C0F958: 913F0070  stw r9, 0x70(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(112 as u32), ctx.r[9].u32 ) };
	// 82C0F95C: F91F0060  std r8, 0x60(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[8].u64 ) };
	// 82C0F960: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82C0F964: 48099AF0  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C0F968(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C0F968 size=112
    let mut pc: u32 = 0x82C0F968;
    'dispatch: loop {
        match pc {
            0x82C0F968 => {
    //   block [0x82C0F968..0x82C0F9D8)
	// 82C0F968: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C0F96C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C0F970: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C0F974: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C0F978: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C0F97C: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82C0F980: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82C0F984: 419A0040  beq cr6, 0x82c0f9c4
	if ctx.cr[6].eq {
	pc = 0x82C0F9C4; continue 'dispatch;
	}
	// 82C0F988: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C0F98C: 814B0034  lwz r10, 0x34(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 82C0F990: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C0F994: 4E800421  bctrl
	ctx.lr = 0x82C0F998;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C0F998: 5469063E  clrlwi r9, r3, 0x18
	ctx.r[9].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82C0F99C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C0F9A0: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82C0F9A4: 409A001C  bne cr6, 0x82c0f9c0
	if !ctx.cr[6].eq {
	pc = 0x82C0F9C0; continue 'dispatch;
	}
	// 82C0F9A8: 4BFFF5C9  bl 0x82c0ef70
	ctx.lr = 0x82C0F9AC;
	sub_82C0EF70(ctx, base);
	// 82C0F9AC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82C0F9B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C0F9B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C0F9B8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C0F9BC: 4E800020  blr
	return;
	// 82C0F9C0: 4BFFFDB9  bl 0x82c0f778
	ctx.lr = 0x82C0F9C4;
	sub_82C0F778(ctx, base);
	// 82C0F9C4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82C0F9C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C0F9CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C0F9D0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C0F9D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C0F9D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C0F9D8 size=132
    let mut pc: u32 = 0x82C0F9D8;
    'dispatch: loop {
        match pc {
            0x82C0F9D8 => {
    //   block [0x82C0F9D8..0x82C0FA5C)
	// 82C0F9D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C0F9DC: 48099A31  bl 0x82ca940c
	ctx.lr = 0x82C0F9E0;
	sub_82CA93D0(ctx, base);
	// 82C0F9E0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C0F9E4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C0F9E8: 813F0048  lwz r9, 0x48(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 82C0F9EC: 815F0038  lwz r10, 0x38(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 82C0F9F0: 817F0040  lwz r11, 0x40(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as u64;
	// 82C0F9F4: 7D495214  add r10, r9, r10
	ctx.r[10].u64 = ctx.r[9].u64 + ctx.r[10].u64;
	// 82C0F9F8: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82C0F9FC: 40980058  bge cr6, 0x82c0fa54
	if !ctx.cr[6].lt {
	pc = 0x82C0FA54; continue 'dispatch;
	}
	// 82C0FA00: 7FCA5851  subf. r30, r10, r11
	ctx.r[30].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82C0FA04: 40810050  ble 0x82c0fa54
	if !ctx.cr[0].gt {
	pc = 0x82C0FA54; continue 'dispatch;
	}
	// 82C0FA08: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C0FA0C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C0FA10: 419A0044  beq cr6, 0x82c0fa54
	if ctx.cr[6].eq {
	pc = 0x82C0FA54; continue 'dispatch;
	}
	// 82C0FA14: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C0FA18: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C0FA1C: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82C0FA20: 409A0008  bne cr6, 0x82c0fa28
	if !ctx.cr[6].eq {
	pc = 0x82C0FA28; continue 'dispatch;
	}
	// 82C0FA24: 0FE00016  twui r0, 0x16
	// twui: trap word unsigned immediate — TODO: implement trap semantics
	// 82C0FA28: 83AB000C  lwz r29, 0xc(r11)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C0FA2C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82C0FA30: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82C0FA34: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82C0FA38: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C0FA3C: 814B0038  lwz r10, 0x38(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) } as u64;
	// 82C0FA40: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C0FA44: 4E800421  bctrl
	ctx.lr = 0x82C0FA48;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C0FA48: 813D0088  lwz r9, 0x88(r29)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(136 as u32) ) } as u64;
	// 82C0FA4C: 7FC9F051  subf. r30, r9, r30
	ctx.r[30].s64 = ctx.r[30].s64 - ctx.r[9].s64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82C0FA50: 4181FFB8  bgt 0x82c0fa08
	if ctx.cr[0].gt {
	pc = 0x82C0FA08; continue 'dispatch;
	}
	// 82C0FA54: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C0FA58: 48099A04  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C0FA60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82C0FA60 size=484
    let mut pc: u32 = 0x82C0FA60;
    'dispatch: loop {
        match pc {
            0x82C0FA60 => {
    //   block [0x82C0FA60..0x82C0FC44)
	// 82C0FA60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C0FA64: 480999A5  bl 0x82ca9408
	ctx.lr = 0x82C0FA68;
	sub_82CA93D0(ctx, base);
	// 82C0FA68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C0FA6C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82C0FA70: 3BA00BB8  li r29, 0xbb8
	ctx.r[29].s64 = 3000;
	// 82C0FA74: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 82C0FA78: 3B800064  li r28, 0x64
	ctx.r[28].s64 = 100;
	// 82C0FA7C: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82C0FA80: 815E0004  lwz r10, 4(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C0FA84: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C0FA88: 91210054  stw r9, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[9].u32 ) };
	// 82C0FA8C: 4800000C  b 0x82c0fa98
	pc = 0x82C0FA98; continue 'dispatch;
	// 82C0FA90: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82C0FA94: 81210054  lwz r9, 0x54(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82C0FA98: 815E0004  lwz r10, 4(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C0FA9C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C0FAA0: 419A000C  beq cr6, 0x82c0faac
	if ctx.cr[6].eq {
	pc = 0x82C0FAAC; continue 'dispatch;
	}
	// 82C0FAA4: 7F0BF040  cmplw cr6, r11, r30
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82C0FAA8: 419A0008  beq cr6, 0x82c0fab0
	if ctx.cr[6].eq {
	pc = 0x82C0FAB0; continue 'dispatch;
	}
	// 82C0FAAC: 0FE00016  twui r0, 0x16
	// twui: trap word unsigned immediate — TODO: implement trap semantics
	// 82C0FAB0: 7F095040  cmplw cr6, r9, r10
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82C0FAB4: 419A00A4  beq cr6, 0x82c0fb58
	if ctx.cr[6].eq {
	pc = 0x82C0FB58; continue 'dispatch;
	}
	// 82C0FAB8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C0FABC: 409A0008  bne cr6, 0x82c0fac4
	if !ctx.cr[6].eq {
	pc = 0x82C0FAC4; continue 'dispatch;
	}
	// 82C0FAC0: 0FE00016  twui r0, 0x16
	// twui: trap word unsigned immediate — TODO: implement trap semantics
	// 82C0FAC4: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C0FAC8: 7F095840  cmplw cr6, r9, r11
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82C0FACC: 409A0008  bne cr6, 0x82c0fad4
	if !ctx.cr[6].eq {
	pc = 0x82C0FAD4; continue 'dispatch;
	}
	// 82C0FAD0: 0FE00016  twui r0, 0x16
	// twui: trap word unsigned immediate — TODO: implement trap semantics
	// 82C0FAD4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C0FAD8: 83E9000C  lwz r31, 0xc(r9)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C0FADC: 4B6F064D  bl 0x82300128
	ctx.lr = 0x82C0FAE0;
	sub_82300128(ctx, base);
	// 82C0FAE0: 897F009C  lbz r11, 0x9c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(156 as u32) ) } as u64;
	// 82C0FAE4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C0FAE8: 419A0044  beq cr6, 0x82c0fb2c
	if ctx.cr[6].eq {
	pc = 0x82C0FB2C; continue 'dispatch;
	}
	// 82C0FAEC: 817F0098  lwz r11, 0x98(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(152 as u32) ) } as u64;
	// 82C0FAF0: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 82C0FAF4: 409A0018  bne cr6, 0x82c0fb0c
	if !ctx.cr[6].eq {
	pc = 0x82C0FB0C; continue 'dispatch;
	}
	// 82C0FAF8: 817F0090  lwz r11, 0x90(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(144 as u32) ) } as u64;
	// 82C0FAFC: 7D5D5810  subfc r10, r29, r11
	ctx.xer.ca = ctx.r[11].u32 >= ctx.r[29].u32;
	ctx.r[10].s64 = ctx.r[11].s64 - ctx.r[29].s64;
	// 82C0FB00: 7D6A5110  subfe r11, r10, r10
	let x = (!ctx.r[10].u32);
	let y = ctx.r[10].u32;
	let s = x.wrapping_add(y);
	let res = s.wrapping_add(ctx.xer.ca as u32);
	tmp.u8 = (s < x) as u8 | (res < s) as u8;
	ctx.r[11].u32 = res;
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	ctx.xer.ca = (tmp.u8 != 0);
	// 82C0FB04: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82C0FB08: 48000024  b 0x82c0fb2c
	pc = 0x82C0FB2C; continue 'dispatch;
	// 82C0FB0C: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82C0FB10: 409A0018  bne cr6, 0x82c0fb28
	if !ctx.cr[6].eq {
	pc = 0x82C0FB28; continue 'dispatch;
	}
	// 82C0FB14: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 82C0FB18: 7D5C5810  subfc r10, r28, r11
	ctx.xer.ca = ctx.r[11].u32 >= ctx.r[28].u32;
	ctx.r[10].s64 = ctx.r[11].s64 - ctx.r[28].s64;
	// 82C0FB1C: 7D2A5110  subfe r9, r10, r10
	let x = (!ctx.r[10].u32);
	let y = ctx.r[10].u32;
	let s = x.wrapping_add(y);
	let res = s.wrapping_add(ctx.xer.ca as u32);
	tmp.u8 = (s < x) as u8 | (res < s) as u8;
	ctx.r[9].u32 = res;
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	ctx.xer.ca = (tmp.u8 != 0);
	// 82C0FB20: 552B07FE  clrlwi r11, r9, 0x1f
	ctx.r[11].u64 = ctx.r[9].u32 as u64 & 0x00000001u64;
	// 82C0FB24: 48000008  b 0x82c0fb2c
	pc = 0x82C0FB2C; continue 'dispatch;
	// 82C0FB28: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82C0FB2C: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82C0FB30: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C0FB34: 419AFF5C  beq cr6, 0x82c0fa90
	if ctx.cr[6].eq {
	pc = 0x82C0FA90; continue 'dispatch;
	}
	// 82C0FB38: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C0FB3C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82C0FB40: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82C0FB44: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C0FB48: 814B0038  lwz r10, 0x38(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) } as u64;
	// 82C0FB4C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C0FB50: 4E800421  bctrl
	ctx.lr = 0x82C0FB54;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C0FB54: 4BFFFF3C  b 0x82c0fa90
	pc = 0x82C0FA90; continue 'dispatch;
	// 82C0FB58: 815E0010  lwz r10, 0x10(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C0FB5C: 3BDE000C  addi r30, r30, 0xc
	ctx.r[30].s64 = ctx.r[30].s64 + 12;
	// 82C0FB60: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 82C0FB64: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82C0FB68: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C0FB6C: 91210054  stw r9, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[9].u32 ) };
	// 82C0FB70: 4800000C  b 0x82c0fb7c
	pc = 0x82C0FB7C; continue 'dispatch;
	// 82C0FB74: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82C0FB78: 81210054  lwz r9, 0x54(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82C0FB7C: 815E0004  lwz r10, 4(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C0FB80: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C0FB84: 419A000C  beq cr6, 0x82c0fb90
	if ctx.cr[6].eq {
	pc = 0x82C0FB90; continue 'dispatch;
	}
	// 82C0FB88: 7F0BF040  cmplw cr6, r11, r30
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82C0FB8C: 419A0008  beq cr6, 0x82c0fb94
	if ctx.cr[6].eq {
	pc = 0x82C0FB94; continue 'dispatch;
	}
	// 82C0FB90: 0FE00016  twui r0, 0x16
	// twui: trap word unsigned immediate — TODO: implement trap semantics
	// 82C0FB94: 7F095040  cmplw cr6, r9, r10
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82C0FB98: 419A00A4  beq cr6, 0x82c0fc3c
	if ctx.cr[6].eq {
	pc = 0x82C0FC3C; continue 'dispatch;
	}
	// 82C0FB9C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C0FBA0: 409A0008  bne cr6, 0x82c0fba8
	if !ctx.cr[6].eq {
	pc = 0x82C0FBA8; continue 'dispatch;
	}
	// 82C0FBA4: 0FE00016  twui r0, 0x16
	// twui: trap word unsigned immediate — TODO: implement trap semantics
	// 82C0FBA8: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C0FBAC: 7F095840  cmplw cr6, r9, r11
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82C0FBB0: 409A0008  bne cr6, 0x82c0fbb8
	if !ctx.cr[6].eq {
	pc = 0x82C0FBB8; continue 'dispatch;
	}
	// 82C0FBB4: 0FE00016  twui r0, 0x16
	// twui: trap word unsigned immediate — TODO: implement trap semantics
	// 82C0FBB8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C0FBBC: 83E9000C  lwz r31, 0xc(r9)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C0FBC0: 4B6F0569  bl 0x82300128
	ctx.lr = 0x82C0FBC4;
	sub_82300128(ctx, base);
	// 82C0FBC4: 897F009C  lbz r11, 0x9c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(156 as u32) ) } as u64;
	// 82C0FBC8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C0FBCC: 419A0044  beq cr6, 0x82c0fc10
	if ctx.cr[6].eq {
	pc = 0x82C0FC10; continue 'dispatch;
	}
	// 82C0FBD0: 817F0098  lwz r11, 0x98(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(152 as u32) ) } as u64;
	// 82C0FBD4: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 82C0FBD8: 409A0018  bne cr6, 0x82c0fbf0
	if !ctx.cr[6].eq {
	pc = 0x82C0FBF0; continue 'dispatch;
	}
	// 82C0FBDC: 817F0090  lwz r11, 0x90(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(144 as u32) ) } as u64;
	// 82C0FBE0: 7D5D5810  subfc r10, r29, r11
	ctx.xer.ca = ctx.r[11].u32 >= ctx.r[29].u32;
	ctx.r[10].s64 = ctx.r[11].s64 - ctx.r[29].s64;
	// 82C0FBE4: 7D6A5110  subfe r11, r10, r10
	let x = (!ctx.r[10].u32);
	let y = ctx.r[10].u32;
	let s = x.wrapping_add(y);
	let res = s.wrapping_add(ctx.xer.ca as u32);
	tmp.u8 = (s < x) as u8 | (res < s) as u8;
	ctx.r[11].u32 = res;
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	ctx.xer.ca = (tmp.u8 != 0);
	// 82C0FBE8: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82C0FBEC: 48000024  b 0x82c0fc10
	pc = 0x82C0FC10; continue 'dispatch;
	// 82C0FBF0: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82C0FBF4: 409A0018  bne cr6, 0x82c0fc0c
	if !ctx.cr[6].eq {
	pc = 0x82C0FC0C; continue 'dispatch;
	}
	// 82C0FBF8: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 82C0FBFC: 7D5C5810  subfc r10, r28, r11
	ctx.xer.ca = ctx.r[11].u32 >= ctx.r[28].u32;
	ctx.r[10].s64 = ctx.r[11].s64 - ctx.r[28].s64;
	// 82C0FC00: 7D2A5110  subfe r9, r10, r10
	let x = (!ctx.r[10].u32);
	let y = ctx.r[10].u32;
	let s = x.wrapping_add(y);
	let res = s.wrapping_add(ctx.xer.ca as u32);
	tmp.u8 = (s < x) as u8 | (res < s) as u8;
	ctx.r[9].u32 = res;
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	ctx.xer.ca = (tmp.u8 != 0);
	// 82C0FC04: 552B07FE  clrlwi r11, r9, 0x1f
	ctx.r[11].u64 = ctx.r[9].u32 as u64 & 0x00000001u64;
	// 82C0FC08: 48000008  b 0x82c0fc10
	pc = 0x82C0FC10; continue 'dispatch;
	// 82C0FC0C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82C0FC10: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82C0FC14: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C0FC18: 409AFF5C  bne cr6, 0x82c0fb74
	if !ctx.cr[6].eq {
	pc = 0x82C0FB74; continue 'dispatch;
	}
	// 82C0FC1C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C0FC20: 38A003E8  li r5, 0x3e8
	ctx.r[5].s64 = 1000;
	// 82C0FC24: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82C0FC28: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C0FC2C: 814B0038  lwz r10, 0x38(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) } as u64;
	// 82C0FC30: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C0FC34: 4E800421  bctrl
	ctx.lr = 0x82C0FC38;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C0FC38: 4BFFFF3C  b 0x82c0fb74
	pc = 0x82C0FB74; continue 'dispatch;
	// 82C0FC3C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82C0FC40: 48099818  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C0FC48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C0FC48 size=304
    let mut pc: u32 = 0x82C0FC48;
    'dispatch: loop {
        match pc {
            0x82C0FC48 => {
    //   block [0x82C0FC48..0x82C0FD78)
	// 82C0FC48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C0FC4C: 480997C1  bl 0x82ca940c
	ctx.lr = 0x82C0FC50;
	sub_82CA93D0(ctx, base);
	// 82C0FC50: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C0FC54: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C0FC58: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82C0FC5C: 815F0040  lwz r10, 0x40(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as u64;
	// 82C0FC60: 817F0048  lwz r11, 0x48(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 82C0FC64: 7D4B5050  subf r10, r11, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 82C0FC68: 7F0A2040  cmplw cr6, r10, r4
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[4].u32, &mut ctx.xer);
	// 82C0FC6C: 41980014  blt cr6, 0x82c0fc80
	if ctx.cr[6].lt {
	pc = 0x82C0FC80; continue 'dispatch;
	}
	// 82C0FC70: 7D4B2214  add r10, r11, r4
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 82C0FC74: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82C0FC78: 915F0048  stw r10, 0x48(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[10].u32 ) };
	// 82C0FC7C: 48000010  b 0x82c0fc8c
	pc = 0x82C0FC8C; continue 'dispatch;
	// 82C0FC80: 7D2B5214  add r9, r11, r10
	ctx.r[9].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82C0FC84: 7D6A2050  subf r11, r10, r4
	ctx.r[11].s64 = ctx.r[4].s64 - ctx.r[10].s64;
	// 82C0FC88: 913F0048  stw r9, 0x48(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[9].u32 ) };
	// 82C0FC8C: 815F0044  lwz r10, 0x44(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(68 as u32) ) } as u64;
	// 82C0FC90: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82C0FC94: 917F0044  stw r11, 0x44(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(68 as u32), ctx.r[11].u32 ) };
	// 82C0FC98: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C0FC9C: 815D0008  lwz r10, 8(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C0FCA0: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82C0FCA4: 40990008  ble cr6, 0x82c0fcac
	if !ctx.cr[6].gt {
	pc = 0x82C0FCAC; continue 'dispatch;
	}
	// 82C0FCA8: 0FE00016  twui r0, 0x16
	// twui: trap word unsigned immediate — TODO: implement trap semantics
	// 82C0FCAC: 7D7E5B78  mr r30, r11
	ctx.r[30].u64 = ctx.r[11].u64;
	// 82C0FCB0: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C0FCB4: 815D0004  lwz r10, 4(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C0FCB8: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82C0FCBC: 40990008  ble cr6, 0x82c0fcc4
	if !ctx.cr[6].gt {
	pc = 0x82C0FCC4; continue 'dispatch;
	}
	// 82C0FCC0: 0FE00016  twui r0, 0x16
	// twui: trap word unsigned immediate — TODO: implement trap semantics
	// 82C0FCC4: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82C0FCC8: 419A0068  beq cr6, 0x82c0fd30
	if ctx.cr[6].eq {
	pc = 0x82C0FD30; continue 'dispatch;
	}
	// 82C0FCCC: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C0FCD0: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82C0FCD4: 41980008  blt cr6, 0x82c0fcdc
	if ctx.cr[6].lt {
	pc = 0x82C0FCDC; continue 'dispatch;
	}
	// 82C0FCD8: 0FE00016  twui r0, 0x16
	// twui: trap word unsigned immediate — TODO: implement trap semantics
	// 82C0FCDC: 807F0030  lwz r3, 0x30(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 82C0FCE0: 809E0000  lwz r4, 0(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C0FCE4: 8163000C  lwz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C0FCE8: 7F045840  cmplw cr6, r4, r11
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82C0FCEC: 41980018  blt cr6, 0x82c0fd04
	if ctx.cr[6].lt {
	pc = 0x82C0FD04; continue 'dispatch;
	}
	// 82C0FCF0: 81430004  lwz r10, 4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C0FCF4: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82C0FCF8: 7F0B2040  cmplw cr6, r11, r4
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[4].u32, &mut ctx.xer);
	// 82C0FCFC: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82C0FD00: 40980008  bge cr6, 0x82c0fd08
	if !ctx.cr[6].lt {
	pc = 0x82C0FD08; continue 'dispatch;
	}
	// 82C0FD04: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82C0FD08: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82C0FD0C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C0FD10: 419A0008  beq cr6, 0x82c0fd18
	if ctx.cr[6].eq {
	pc = 0x82C0FD18; continue 'dispatch;
	}
	// 82C0FD14: 4BFFF635  bl 0x82c0f348
	ctx.lr = 0x82C0FD18;
	sub_82C0F348(ctx, base);
	// 82C0FD18: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C0FD1C: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82C0FD20: 41980008  blt cr6, 0x82c0fd28
	if ctx.cr[6].lt {
	pc = 0x82C0FD28; continue 'dispatch;
	}
	// 82C0FD24: 0FE00016  twui r0, 0x16
	// twui: trap word unsigned immediate — TODO: implement trap semantics
	// 82C0FD28: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82C0FD2C: 4BFFFF84  b 0x82c0fcb0
	pc = 0x82C0FCB0; continue 'dispatch;
	// 82C0FD30: 80BD0008  lwz r5, 8(r29)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C0FD34: 807D0004  lwz r3, 4(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C0FD38: 7F032840  cmplw cr6, r3, r5
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[5].u32, &mut ctx.xer);
	// 82C0FD3C: 40990010  ble cr6, 0x82c0fd4c
	if !ctx.cr[6].gt {
	pc = 0x82C0FD4C; continue 'dispatch;
	}
	// 82C0FD40: 0FE00016  twui r0, 0x16
	// twui: trap word unsigned immediate — TODO: implement trap semantics
	// 82C0FD44: 0FE00016  twui r0, 0x16
	// twui: trap word unsigned immediate — TODO: implement trap semantics
	// 82C0FD48: 7F032840  cmplw cr6, r3, r5
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[5].u32, &mut ctx.xer);
	// 82C0FD4C: 419A0024  beq cr6, 0x82c0fd70
	if ctx.cr[6].eq {
	pc = 0x82C0FD70; continue 'dispatch;
	}
	// 82C0FD50: 7D652850  subf r11, r5, r5
	ctx.r[11].s64 = ctx.r[5].s64 - ctx.r[5].s64;
	// 82C0FD54: 7D6B1671  srawi. r11, r11, 2
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 2) as i64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82C0FD58: 5566103A  slwi r6, r11, 2
	ctx.r[6].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 82C0FD5C: 7FE61A14  add r31, r6, r3
	ctx.r[31].u64 = ctx.r[6].u64 + ctx.r[3].u64;
	// 82C0FD60: 4081000C  ble 0x82c0fd6c
	if !ctx.cr[0].gt {
	pc = 0x82C0FD6C; continue 'dispatch;
	}
	// 82C0FD64: 7CC43378  mr r4, r6
	ctx.r[4].u64 = ctx.r[6].u64;
	// 82C0FD68: 4809A2C1  bl 0x82caa028
	ctx.lr = 0x82C0FD6C;
	sub_82CAA028(ctx, base);
	// 82C0FD6C: 93FD0008  stw r31, 8(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(8 as u32), ctx.r[31].u32 ) };
	// 82C0FD70: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C0FD74: 480996E8  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C0FD78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C0FD78 size=264
    let mut pc: u32 = 0x82C0FD78;
    'dispatch: loop {
        match pc {
            0x82C0FD78 => {
    //   block [0x82C0FD78..0x82C0FE80)
	// 82C0FD78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C0FD7C: 4809968D  bl 0x82ca9408
	ctx.lr = 0x82C0FD80;
	sub_82CA93D0(ctx, base);
	// 82C0FD80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C0FD84: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82C0FD88: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C0FD8C: 3BDD0028  addi r30, r29, 0x28
	ctx.r[30].s64 = ctx.r[29].s64 + 40;
	// 82C0FD90: 394BAA88  addi r10, r11, -0x5578
	ctx.r[10].s64 = ctx.r[11].s64 + -21880;
	// 82C0FD94: 915D0000  stw r10, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82C0FD98: 817D002C  lwz r11, 0x2c(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(44 as u32) ) } as u64;
	// 82C0FD9C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C0FDA0: 419A0028  beq cr6, 0x82c0fdc8
	if ctx.cr[6].eq {
	pc = 0x82C0FDC8; continue 'dispatch;
	}
	// 82C0FDA4: 815E0008  lwz r10, 8(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C0FDA8: 7D2B5050  subf r9, r11, r10
	ctx.r[9].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 82C0FDAC: 7D2B1671  srawi. r11, r9, 2
	ctx.xer.ca = (ctx.r[9].s32 < 0) && ((ctx.r[9].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[9].s32 >> 2) as i64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82C0FDB0: 41820018  beq 0x82c0fdc8
	if ctx.cr[0].eq {
	pc = 0x82C0FDC8; continue 'dispatch;
	}
	// 82C0FDB4: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 82C0FDB8: 809D0088  lwz r4, 0x88(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(136 as u32) ) } as u64;
	// 82C0FDBC: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82C0FDC0: 806B4EEC  lwz r3, 0x4eec(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20204 as u32) ) } as u64;
	// 82C0FDC4: 4BFFFE85  bl 0x82c0fc48
	ctx.lr = 0x82C0FDC8;
	sub_82C0FC48(ctx, base);
	// 82C0FDC8: 807D0040  lwz r3, 0x40(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(64 as u32) ) } as u64;
	// 82C0FDCC: 3B9D003C  addi r28, r29, 0x3c
	ctx.r[28].s64 = ctx.r[29].s64 + 60;
	// 82C0FDD0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C0FDD4: 419A0014  beq cr6, 0x82c0fde8
	if ctx.cr[6].eq {
	pc = 0x82C0FDE8; continue 'dispatch;
	}
	// 82C0FDD8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C0FDDC: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C0FDE0: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C0FDE4: 4E800421  bctrl
	ctx.lr = 0x82C0FDE8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C0FDE8: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82C0FDEC: 93FC0000  stw r31, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82C0FDF0: 93FC0004  stw r31, 4(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82C0FDF4: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C0FDF8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C0FDFC: 419A0008  beq cr6, 0x82c0fe04
	if ctx.cr[6].eq {
	pc = 0x82C0FE04; continue 'dispatch;
	}
	// 82C0FE00: 4BC359B1  bl 0x828457b0
	ctx.lr = 0x82C0FE04;
	sub_828457B0(ctx, base);
	// 82C0FE04: 93FE0008  stw r31, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[31].u32 ) };
	// 82C0FE08: 3B9D0018  addi r28, r29, 0x18
	ctx.r[28].s64 = ctx.r[29].s64 + 24;
	// 82C0FE0C: 93FE000C  stw r31, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 82C0FE10: 93FE0004  stw r31, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82C0FE14: 807D001C  lwz r3, 0x1c(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(28 as u32) ) } as u64;
	// 82C0FE18: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C0FE1C: 419A0014  beq cr6, 0x82c0fe30
	if ctx.cr[6].eq {
	pc = 0x82C0FE30; continue 'dispatch;
	}
	// 82C0FE20: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C0FE24: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C0FE28: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C0FE2C: 4E800421  bctrl
	ctx.lr = 0x82C0FE30;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C0FE30: 93FC0004  stw r31, 4(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82C0FE34: 3BDD0010  addi r30, r29, 0x10
	ctx.r[30].s64 = ctx.r[29].s64 + 16;
	// 82C0FE38: 93FC0000  stw r31, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82C0FE3C: 807D0014  lwz r3, 0x14(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(20 as u32) ) } as u64;
	// 82C0FE40: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C0FE44: 419A0014  beq cr6, 0x82c0fe58
	if ctx.cr[6].eq {
	pc = 0x82C0FE58; continue 'dispatch;
	}
	// 82C0FE48: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C0FE4C: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C0FE50: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C0FE54: 4E800421  bctrl
	ctx.lr = 0x82C0FE58;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C0FE58: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C0FE5C: 93FE0004  stw r31, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82C0FE60: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82C0FE64: 93FE0000  stw r31, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82C0FE68: 392BA37C  addi r9, r11, -0x5c84
	ctx.r[9].s64 = ctx.r[11].s64 + -23684;
	// 82C0FE6C: 390AA9C8  addi r8, r10, -0x5638
	ctx.r[8].s64 = ctx.r[10].s64 + -22072;
	// 82C0FE70: 913D0004  stw r9, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82C0FE74: 911D0000  stw r8, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82C0FE78: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82C0FE7C: 480995DC  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C0FE80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C0FE80 size=80
    let mut pc: u32 = 0x82C0FE80;
    'dispatch: loop {
        match pc {
            0x82C0FE80 => {
    //   block [0x82C0FE80..0x82C0FED0)
	// 82C0FE80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C0FE84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C0FE88: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82C0FE8C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C0FE90: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C0FE94: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C0FE98: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82C0FE9C: 4BFFFEDD  bl 0x82c0fd78
	ctx.lr = 0x82C0FEA0;
	sub_82C0FD78(ctx, base);
	// 82C0FEA0: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82C0FEA4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C0FEA8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C0FEAC: 419A000C  beq cr6, 0x82c0feb8
	if ctx.cr[6].eq {
	pc = 0x82C0FEB8; continue 'dispatch;
	}
	// 82C0FEB0: 4BC35901  bl 0x828457b0
	ctx.lr = 0x82C0FEB4;
	sub_828457B0(ctx, base);
	// 82C0FEB4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C0FEB8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C0FEBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C0FEC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C0FEC4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82C0FEC8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C0FECC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C0FED0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C0FED0 size=748
    let mut pc: u32 = 0x82C0FED0;
    'dispatch: loop {
        match pc {
            0x82C0FED0 => {
    //   block [0x82C0FED0..0x82C101BC)
	// 82C0FED0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C0FED4: 48099529  bl 0x82ca93fc
	ctx.lr = 0x82C0FED8;
	sub_82CA93D0(ctx, base);
	// 82C0FED8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C0FEDC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82C0FEE0: 83260000  lwz r25, 0(r6)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C0FEE4: F88100A8  std r4, 0xa8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(168 as u32), ctx.r[4].u64 ) };
	// 82C0FEE8: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82C0FEEC: 815E0004  lwz r10, 4(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C0FEF0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C0FEF4: 409A000C  bne cr6, 0x82c0ff00
	if !ctx.cr[6].eq {
	pc = 0x82C0FF00; continue 'dispatch;
	}
	// 82C0FEF8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82C0FEFC: 48000010  b 0x82c0ff0c
	pc = 0x82C0FF0C; continue 'dispatch;
	// 82C0FF00: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C0FF04: 7D2A5850  subf r9, r10, r11
	ctx.r[9].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82C0FF08: 7D281670  srawi r8, r9, 2
	ctx.xer.ca = (ctx.r[9].s32 < 0) && ((ctx.r[9].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[8].s64 = (ctx.r[9].s32 >> 2) as i64;
	// 82C0FF0C: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 82C0FF10: 419A02A4  beq cr6, 0x82c101b4
	if ctx.cr[6].eq {
	pc = 0x82C101B4; continue 'dispatch;
	}
	// 82C0FF14: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C0FF18: 409A000C  bne cr6, 0x82c0ff24
	if !ctx.cr[6].eq {
	pc = 0x82C0FF24; continue 'dispatch;
	}
	// 82C0FF1C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82C0FF20: 48000010  b 0x82c0ff30
	pc = 0x82C0FF30; continue 'dispatch;
	// 82C0FF24: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C0FF28: 7D2A5850  subf r9, r10, r11
	ctx.r[9].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82C0FF2C: 7D2B1670  srawi r11, r9, 2
	ctx.xer.ca = (ctx.r[9].s32 < 0) && ((ctx.r[9].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[9].s32 >> 2) as i64;
	// 82C0FF30: 3D203FFF  lis r9, 0x3fff
	ctx.r[9].s64 = 1073676288;
	// 82C0FF34: 6129FFFF  ori r9, r9, 0xffff
	ctx.r[9].u64 = ctx.r[9].u64 | 65535;
	// 82C0FF38: 7CEB4850  subf r7, r11, r9
	ctx.r[7].s64 = ctx.r[9].s64 - ctx.r[11].s64;
	// 82C0FF3C: 7F07D840  cmplw cr6, r7, r27
	ctx.cr[6].compare_u32(ctx.r[7].u32, ctx.r[27].u32, &mut ctx.xer);
	// 82C0FF40: 40980010  bge cr6, 0x82c0ff50
	if !ctx.cr[6].lt {
	pc = 0x82C0FF50; continue 'dispatch;
	}
	// 82C0FF44: 4BFEF525  bl 0x82bff468
	ctx.lr = 0x82C0FF48;
	sub_82BFF468(ctx, base);
	// 82C0FF48: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82C0FF4C: 48099500  b 0x82ca944c
	sub_82CA9420(ctx, base);
	return;
	// 82C0FF50: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C0FF54: 409A000C  bne cr6, 0x82c0ff60
	if !ctx.cr[6].eq {
	pc = 0x82C0FF60; continue 'dispatch;
	}
	// 82C0FF58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82C0FF5C: 48000010  b 0x82c0ff6c
	pc = 0x82C0FF6C; continue 'dispatch;
	// 82C0FF60: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C0FF64: 7CEA5850  subf r7, r10, r11
	ctx.r[7].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82C0FF68: 7CEB1670  srawi r11, r7, 2
	ctx.xer.ca = (ctx.r[7].s32 < 0) && ((ctx.r[7].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[7].s32 >> 2) as i64;
	// 82C0FF6C: 7D6BDA14  add r11, r11, r27
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[27].u64;
	// 82C0FF70: 7F085840  cmplw cr6, r8, r11
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82C0FF74: 40980130  bge cr6, 0x82c100a4
	if !ctx.cr[6].lt {
	pc = 0x82C100A4; continue 'dispatch;
	}
	// 82C0FF78: 550BF87E  srwi r11, r8, 1
	ctx.r[11].u32 = ctx.r[8].u32.wrapping_shr(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82C0FF7C: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 82C0FF80: 7D2B4850  subf r9, r11, r9
	ctx.r[9].s64 = ctx.r[9].s64 - ctx.r[11].s64;
	// 82C0FF84: 7F094040  cmplw cr6, r9, r8
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82C0FF88: 41980008  blt cr6, 0x82c0ff90
	if ctx.cr[6].lt {
	pc = 0x82C0FF90; continue 'dispatch;
	}
	// 82C0FF8C: 7F4B4214  add r26, r11, r8
	ctx.r[26].u64 = ctx.r[11].u64 + ctx.r[8].u64;
	// 82C0FF90: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C0FF94: 409A000C  bne cr6, 0x82c0ffa0
	if !ctx.cr[6].eq {
	pc = 0x82C0FFA0; continue 'dispatch;
	}
	// 82C0FF98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82C0FF9C: 48000010  b 0x82c0ffac
	pc = 0x82C0FFAC; continue 'dispatch;
	// 82C0FFA0: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C0FFA4: 7D2A5850  subf r9, r10, r11
	ctx.r[9].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82C0FFA8: 7D2B1670  srawi r11, r9, 2
	ctx.xer.ca = (ctx.r[9].s32 < 0) && ((ctx.r[9].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[9].s32 >> 2) as i64;
	// 82C0FFAC: 7D6BDA14  add r11, r11, r27
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[27].u64;
	// 82C0FFB0: 7F1A5840  cmplw cr6, r26, r11
	ctx.cr[6].compare_u32(ctx.r[26].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82C0FFB4: 40980024  bge cr6, 0x82c0ffd8
	if !ctx.cr[6].lt {
	pc = 0x82C0FFD8; continue 'dispatch;
	}
	// 82C0FFB8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C0FFBC: 409A000C  bne cr6, 0x82c0ffc8
	if !ctx.cr[6].eq {
	pc = 0x82C0FFC8; continue 'dispatch;
	}
	// 82C0FFC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82C0FFC4: 48000010  b 0x82c0ffd4
	pc = 0x82C0FFD4; continue 'dispatch;
	// 82C0FFC8: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C0FFCC: 7D4A5850  subf r10, r10, r11
	ctx.r[10].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82C0FFD0: 7D4B1670  srawi r11, r10, 2
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[10].s32 >> 2) as i64;
	// 82C0FFD4: 7F4BDA14  add r26, r11, r27
	ctx.r[26].u64 = ctx.r[11].u64 + ctx.r[27].u64;
	// 82C0FFD8: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82C0FFDC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82C0FFE0: 4BE76039  bl 0x82a86018
	ctx.lr = 0x82C0FFE4;
	sub_82A86018(ctx, base);
	// 82C0FFE4: 80BE0004  lwz r5, 4(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C0FFE8: 83E100AC  lwz r31, 0xac(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(172 as u32) ) } as u64;
	// 82C0FFEC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82C0FFF0: 7D65F850  subf r11, r5, r31
	ctx.r[11].s64 = ctx.r[31].s64 - ctx.r[5].s64;
	// 82C0FFF4: 7D6B1671  srawi. r11, r11, 2
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 2) as i64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82C0FFF8: 5566103A  slwi r6, r11, 2
	ctx.r[6].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 82C0FFFC: 7F86EA14  add r28, r6, r29
	ctx.r[28].u64 = ctx.r[6].u64 + ctx.r[29].u64;
	// 82C10000: 4182000C  beq 0x82c1000c
	if ctx.cr[0].eq {
	pc = 0x82C1000C; continue 'dispatch;
	}
	// 82C10004: 7CC43378  mr r4, r6
	ctx.r[4].u64 = ctx.r[6].u64;
	// 82C10008: 4809A021  bl 0x82caa028
	ctx.lr = 0x82C1000C;
	sub_82CAA028(ctx, base);
	// 82C1000C: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 82C10010: 419A0018  beq cr6, 0x82c10028
	if ctx.cr[6].eq {
	pc = 0x82C10028; continue 'dispatch;
	}
	// 82C10014: 7F8BE378  mr r11, r28
	ctx.r[11].u64 = ctx.r[28].u64;
	// 82C10018: 7F6903A6  mtctr r27
	ctx.ctr.u64 = ctx.r[27].u64;
	// 82C1001C: 932B0000  stw r25, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[25].u32 ) };
	// 82C10020: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82C10024: 4200FFF8  bdnz 0x82c1001c
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82C1001C; continue 'dispatch;
	}
	// 82C10028: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C1002C: 7D5F5850  subf r10, r31, r11
	ctx.r[10].s64 = ctx.r[11].s64 - ctx.r[31].s64;
	// 82C10030: 7D4B1671  srawi. r11, r10, 2
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[10].s32 >> 2) as i64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82C10034: 4182001C  beq 0x82c10050
	if ctx.cr[0].eq {
	pc = 0x82C10050; continue 'dispatch;
	}
	// 82C10038: 5564103A  slwi r4, r11, 2
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82C1003C: 576B103A  slwi r11, r27, 2
	ctx.r[11].u32 = ctx.r[27].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82C10040: 7C862378  mr r6, r4
	ctx.r[6].u64 = ctx.r[4].u64;
	// 82C10044: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82C10048: 7C6BE214  add r3, r11, r28
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[28].u64;
	// 82C1004C: 48099FDD  bl 0x82caa028
	ctx.lr = 0x82C10050;
	sub_82CAA028(ctx, base);
	// 82C10050: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C10054: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C10058: 409A000C  bne cr6, 0x82c10064
	if !ctx.cr[6].eq {
	pc = 0x82C10064; continue 'dispatch;
	}
	// 82C1005C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82C10060: 48000010  b 0x82c10070
	pc = 0x82C10070; continue 'dispatch;
	// 82C10064: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C10068: 7D435850  subf r10, r3, r11
	ctx.r[10].s64 = ctx.r[11].s64 - ctx.r[3].s64;
	// 82C1006C: 7D4B1670  srawi r11, r10, 2
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[10].s32 >> 2) as i64;
	// 82C10070: 7FEBDA14  add r31, r11, r27
	ctx.r[31].u64 = ctx.r[11].u64 + ctx.r[27].u64;
	// 82C10074: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C10078: 419A0008  beq cr6, 0x82c10080
	if ctx.cr[6].eq {
	pc = 0x82C10080; continue 'dispatch;
	}
	// 82C1007C: 4BC35735  bl 0x828457b0
	ctx.lr = 0x82C10080;
	sub_828457B0(ctx, base);
	// 82C10080: 574B103A  slwi r11, r26, 2
	ctx.r[11].u32 = ctx.r[26].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82C10084: 93BE0004  stw r29, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82C10088: 57EA103A  slwi r10, r31, 2
	ctx.r[10].u32 = ctx.r[31].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82C1008C: 7D6BEA14  add r11, r11, r29
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 82C10090: 7D4AEA14  add r10, r10, r29
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[29].u64;
	// 82C10094: 917E000C  stw r11, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82C10098: 915E0008  stw r10, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82C1009C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82C100A0: 480993AC  b 0x82ca944c
	sub_82CA9420(ctx, base);
	return;
	// 82C100A4: 83E100AC  lwz r31, 0xac(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(172 as u32) ) } as u64;
	// 82C100A8: 839E0008  lwz r28, 8(r30)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C100AC: 7D7FE050  subf r11, r31, r28
	ctx.r[11].s64 = ctx.r[28].s64 - ctx.r[31].s64;
	// 82C100B0: 7D6B1670  srawi r11, r11, 2
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 2) as i64;
	// 82C100B4: 7F0BD840  cmplw cr6, r11, r27
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[27].u32, &mut ctx.xer);
	// 82C100B8: 40980088  bge cr6, 0x82c10140
	if !ctx.cr[6].lt {
	pc = 0x82C10140; continue 'dispatch;
	}
	// 82C100BC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C100C0: 419A001C  beq cr6, 0x82c100dc
	if ctx.cr[6].eq {
	pc = 0x82C100DC; continue 'dispatch;
	}
	// 82C100C4: 5564103A  slwi r4, r11, 2
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82C100C8: 576B103A  slwi r11, r27, 2
	ctx.r[11].u32 = ctx.r[27].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82C100CC: 7C862378  mr r6, r4
	ctx.r[6].u64 = ctx.r[4].u64;
	// 82C100D0: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82C100D4: 7C6BFA14  add r3, r11, r31
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82C100D8: 48099F51  bl 0x82caa028
	ctx.lr = 0x82C100DC;
	sub_82CAA028(ctx, base);
	// 82C100DC: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C100E0: 7D5F5850  subf r10, r31, r11
	ctx.r[10].s64 = ctx.r[11].s64 - ctx.r[31].s64;
	// 82C100E4: 7D491670  srawi r9, r10, 2
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[9].s64 = (ctx.r[10].s32 >> 2) as i64;
	// 82C100E8: 7D49D851  subf. r10, r9, r27
	ctx.r[10].s64 = ctx.r[27].s64 - ctx.r[9].s64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82C100EC: 4182001C  beq 0x82c10108
	if ctx.cr[0].eq {
	pc = 0x82C10108; continue 'dispatch;
	}
	// 82C100F0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C100F4: 419A0014  beq cr6, 0x82c10108
	if ctx.cr[6].eq {
	pc = 0x82C10108; continue 'dispatch;
	}
	// 82C100F8: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C100FC: 932B0000  stw r25, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[25].u32 ) };
	// 82C10100: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82C10104: 4200FFF8  bdnz 0x82c100fc
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82C100FC; continue 'dispatch;
	}
	// 82C10108: 815E0008  lwz r10, 8(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C1010C: 5769103A  slwi r9, r27, 2
	ctx.r[9].u32 = ctx.r[27].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82C10110: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 82C10114: 7D495214  add r10, r9, r10
	ctx.r[10].u64 = ctx.r[9].u64 + ctx.r[10].u64;
	// 82C10118: 7D295050  subf r9, r9, r10
	ctx.r[9].s64 = ctx.r[10].s64 - ctx.r[9].s64;
	// 82C1011C: 915E0008  stw r10, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82C10120: 7F1F4840  cmplw cr6, r31, r9
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82C10124: 419A0090  beq cr6, 0x82c101b4
	if ctx.cr[6].eq {
	pc = 0x82C101B4; continue 'dispatch;
	}
	// 82C10128: 932B0000  stw r25, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[25].u32 ) };
	// 82C1012C: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82C10130: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82C10134: 409AFFF4  bne cr6, 0x82c10128
	if !ctx.cr[6].eq {
	pc = 0x82C10128; continue 'dispatch;
	}
	// 82C10138: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82C1013C: 48099310  b 0x82ca944c
	sub_82CA9420(ctx, base);
	return;
	// 82C10140: 577B103A  slwi r27, r27, 2
	ctx.r[27].u32 = ctx.r[27].u32.wrapping_shl(2);
	ctx.r[27].u64 = ctx.r[27].u32 as u64;
	// 82C10144: 807E0008  lwz r3, 8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C10148: 7FBBE050  subf r29, r27, r28
	ctx.r[29].s64 = ctx.r[28].s64 - ctx.r[27].s64;
	// 82C1014C: 7D7DE050  subf r11, r29, r28
	ctx.r[11].s64 = ctx.r[28].s64 - ctx.r[29].s64;
	// 82C10150: 7D6B1671  srawi. r11, r11, 2
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 2) as i64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82C10154: 5566103A  slwi r6, r11, 2
	ctx.r[6].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 82C10158: 7F461A14  add r26, r6, r3
	ctx.r[26].u64 = ctx.r[6].u64 + ctx.r[3].u64;
	// 82C1015C: 41820010  beq 0x82c1016c
	if ctx.cr[0].eq {
	pc = 0x82C1016C; continue 'dispatch;
	}
	// 82C10160: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82C10164: 7CC43378  mr r4, r6
	ctx.r[4].u64 = ctx.r[6].u64;
	// 82C10168: 48099EC1  bl 0x82caa028
	ctx.lr = 0x82C1016C;
	sub_82CAA028(ctx, base);
	// 82C1016C: 7D7FE850  subf r11, r31, r29
	ctx.r[11].s64 = ctx.r[29].s64 - ctx.r[31].s64;
	// 82C10170: 935E0008  stw r26, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[26].u32 ) };
	// 82C10174: 7D6B1671  srawi. r11, r11, 2
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 2) as i64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82C10178: 4081001C  ble 0x82c10194
	if !ctx.cr[0].gt {
	pc = 0x82C10194; continue 'dispatch;
	}
	// 82C1017C: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82C10180: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82C10184: 7D665B78  mr r6, r11
	ctx.r[6].u64 = ctx.r[11].u64;
	// 82C10188: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82C1018C: 7C6BE050  subf r3, r11, r28
	ctx.r[3].s64 = ctx.r[28].s64 - ctx.r[11].s64;
	// 82C10190: 48099E99  bl 0x82caa028
	ctx.lr = 0x82C10194;
	sub_82CAA028(ctx, base);
	// 82C10194: 7D5BFA14  add r10, r27, r31
	ctx.r[10].u64 = ctx.r[27].u64 + ctx.r[31].u64;
	// 82C10198: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 82C1019C: 7F1F5040  cmplw cr6, r31, r10
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82C101A0: 419A0014  beq cr6, 0x82c101b4
	if ctx.cr[6].eq {
	pc = 0x82C101B4; continue 'dispatch;
	}
	// 82C101A4: 932B0000  stw r25, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[25].u32 ) };
	// 82C101A8: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82C101AC: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82C101B0: 409AFFF4  bne cr6, 0x82c101a4
	if !ctx.cr[6].eq {
	pc = 0x82C101A4; continue 'dispatch;
	}
	// 82C101B4: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82C101B8: 48099294  b 0x82ca944c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C101C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C101C0 size=292
    let mut pc: u32 = 0x82C101C0;
    'dispatch: loop {
        match pc {
            0x82C101C0 => {
    //   block [0x82C101C0..0x82C102E4)
	// 82C101C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C101C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C101C8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82C101CC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C101D0: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C101D4: 81430028  lwz r10, 0x28(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 82C101D8: 3BC30024  addi r30, r3, 0x24
	ctx.r[30].s64 = ctx.r[3].s64 + 36;
	// 82C101DC: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 82C101E0: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82C101E4: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C101E8: 91210054  stw r9, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[9].u32 ) };
	// 82C101EC: 815E0004  lwz r10, 4(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C101F0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C101F4: 419A000C  beq cr6, 0x82c10200
	if ctx.cr[6].eq {
	pc = 0x82C10200; continue 'dispatch;
	}
	// 82C101F8: 7F0BF040  cmplw cr6, r11, r30
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82C101FC: 419A0008  beq cr6, 0x82c10204
	if ctx.cr[6].eq {
	pc = 0x82C10204; continue 'dispatch;
	}
	// 82C10200: 0FE00016  twui r0, 0x16
	// twui: trap word unsigned immediate — TODO: implement trap semantics
	// 82C10204: 7F095040  cmplw cr6, r9, r10
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82C10208: 419A00C4  beq cr6, 0x82c102cc
	if ctx.cr[6].eq {
	pc = 0x82C102CC; continue 'dispatch;
	}
	// 82C1020C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C10210: 409A0008  bne cr6, 0x82c10218
	if !ctx.cr[6].eq {
	pc = 0x82C10218; continue 'dispatch;
	}
	// 82C10214: 0FE00016  twui r0, 0x16
	// twui: trap word unsigned immediate — TODO: implement trap semantics
	// 82C10218: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C1021C: 7F095840  cmplw cr6, r9, r11
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82C10220: 409A0008  bne cr6, 0x82c10228
	if !ctx.cr[6].eq {
	pc = 0x82C10228; continue 'dispatch;
	}
	// 82C10224: 0FE00016  twui r0, 0x16
	// twui: trap word unsigned immediate — TODO: implement trap semantics
	// 82C10228: 83E9000C  lwz r31, 0xc(r9)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C1022C: 897F009E  lbz r11, 0x9e(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(158 as u32) ) } as u64;
	// 82C10230: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C10234: 419A000C  beq cr6, 0x82c10240
	if ctx.cr[6].eq {
	pc = 0x82C10240; continue 'dispatch;
	}
	// 82C10238: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82C1023C: 48000030  b 0x82c1026c
	pc = 0x82C1026C; continue 'dispatch;
	// 82C10240: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C10244: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82C10248: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C1024C: 814B0038  lwz r10, 0x38(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) } as u64;
	// 82C10250: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C10254: 4E800421  bctrl
	ctx.lr = 0x82C10258;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C10258: 8121007C  lwz r9, 0x7c(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(124 as u32) ) } as u64;
	// 82C1025C: 81010078  lwz r8, 0x78(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(120 as u32) ) } as u64;
	// 82C10260: 7CE94050  subf r7, r9, r8
	ctx.r[7].s64 = ctx.r[8].s64 - ctx.r[9].s64;
	// 82C10264: 7CE60034  cntlzw r6, r7
	ctx.r[6].u64 = if ctx.r[7].u32 == 0 { 32 } else { ctx.r[7].u32.leading_zeros() as u64 };
	// 82C10268: 54CBDFFE  rlwinm r11, r6, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[6].u32 as u64 & 0x0000001Fu64;
	// 82C1026C: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82C10270: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C10274: 419A0044  beq cr6, 0x82c102b8
	if ctx.cr[6].eq {
	pc = 0x82C102B8; continue 'dispatch;
	}
	// 82C10278: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82C1027C: E8A10050  ld r5, 0x50(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82C10280: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82C10284: 480742AD  bl 0x82c84530
	ctx.lr = 0x82C10288;
	sub_82C84530(ctx, base);
	// 82C10288: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82C1028C: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C10290: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82C10294: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C10298: E92B0000  ld r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82C1029C: 810A0048  lwz r8, 0x48(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(72 as u32) ) } as u64;
	// 82C102A0: F9210050  std r9, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u64 ) };
	// 82C102A4: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	// 82C102A8: 4E800421  bctrl
	ctx.lr = 0x82C102AC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C102AC: 81210054  lwz r9, 0x54(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82C102B0: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82C102B4: 4BFFFF38  b 0x82c101ec
	pc = 0x82C101EC; continue 'dispatch;
	// 82C102B8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C102BC: 4B6EFE6D  bl 0x82300128
	ctx.lr = 0x82C102C0;
	sub_82300128(ctx, base);
	// 82C102C0: 81210054  lwz r9, 0x54(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82C102C4: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82C102C8: 4BFFFF24  b 0x82c101ec
	pc = 0x82C101EC; continue 'dispatch;
	// 82C102CC: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82C102D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C102D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C102D8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82C102DC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C102E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C102E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C102E8 size=228
    let mut pc: u32 = 0x82C102E8;
    'dispatch: loop {
        match pc {
            0x82C102E8 => {
    //   block [0x82C102E8..0x82C103CC)
	// 82C102E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C102EC: 4809911D  bl 0x82ca9408
	ctx.lr = 0x82C102F0;
	sub_82CA93D0(ctx, base);
	// 82C102F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C102F4: 3D603FFF  lis r11, 0x3fff
	ctx.r[11].s64 = 1073676288;
	// 82C102F8: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82C102FC: 616AFFFF  ori r10, r11, 0xffff
	ctx.r[10].u64 = ctx.r[11].u64 | 65535;
	// 82C10300: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C10304: 7F1C5040  cmplw cr6, r28, r10
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82C10308: 40990010  ble cr6, 0x82c10318
	if !ctx.cr[6].gt {
	pc = 0x82C10318; continue 'dispatch;
	}
	// 82C1030C: 4BFEF15D  bl 0x82bff468
	ctx.lr = 0x82C10310;
	sub_82BFF468(ctx, base);
	// 82C10310: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82C10314: 48099144  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
	// 82C10318: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C1031C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C10320: 419A0010  beq cr6, 0x82c10330
	if ctx.cr[6].eq {
	pc = 0x82C10330; continue 'dispatch;
	}
	// 82C10324: 815F000C  lwz r10, 0xc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C10328: 7D2B5050  subf r9, r11, r10
	ctx.r[9].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 82C1032C: 7D2B1670  srawi r11, r9, 2
	ctx.xer.ca = (ctx.r[9].s32 < 0) && ((ctx.r[9].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[9].s32 >> 2) as i64;
	// 82C10330: 7F0BE040  cmplw cr6, r11, r28
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[28].u32, &mut ctx.xer);
	// 82C10334: 40980090  bge cr6, 0x82c103c4
	if !ctx.cr[6].lt {
	pc = 0x82C103C4; continue 'dispatch;
	}
	// 82C10338: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82C1033C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C10340: 4BE75CD9  bl 0x82a86018
	ctx.lr = 0x82C10344;
	sub_82A86018(ctx, base);
	// 82C10344: 80BF0004  lwz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C10348: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C1034C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82C10350: 7F055840  cmplw cr6, r5, r11
	ctx.cr[6].compare_u32(ctx.r[5].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82C10354: 4099000C  ble cr6, 0x82c10360
	if !ctx.cr[6].gt {
	pc = 0x82C10360; continue 'dispatch;
	}
	// 82C10358: 0FE00016  twui r0, 0x16
	// twui: trap word unsigned immediate — TODO: implement trap semantics
	// 82C1035C: 0FE00016  twui r0, 0x16
	// twui: trap word unsigned immediate — TODO: implement trap semantics
	// 82C10360: 7D655850  subf r11, r5, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[5].s64;
	// 82C10364: 7D6B1671  srawi. r11, r11, 2
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 2) as i64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82C10368: 41820014  beq 0x82c1037c
	if ctx.cr[0].eq {
	pc = 0x82C1037C; continue 'dispatch;
	}
	// 82C1036C: 5564103A  slwi r4, r11, 2
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82C10370: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82C10374: 7C862378  mr r6, r4
	ctx.r[6].u64 = ctx.r[4].u64;
	// 82C10378: 48099CB1  bl 0x82caa028
	ctx.lr = 0x82C1037C;
	sub_82CAA028(ctx, base);
	// 82C1037C: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C10380: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C10384: 409A000C  bne cr6, 0x82c10390
	if !ctx.cr[6].eq {
	pc = 0x82C10390; continue 'dispatch;
	}
	// 82C10388: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82C1038C: 48000010  b 0x82c1039c
	pc = 0x82C1039C; continue 'dispatch;
	// 82C10390: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C10394: 7D435850  subf r10, r3, r11
	ctx.r[10].s64 = ctx.r[11].s64 - ctx.r[3].s64;
	// 82C10398: 7D5D1670  srawi r29, r10, 2
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[29].s64 = (ctx.r[10].s32 >> 2) as i64;
	// 82C1039C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C103A0: 419A0008  beq cr6, 0x82c103a8
	if ctx.cr[6].eq {
	pc = 0x82C103A8; continue 'dispatch;
	}
	// 82C103A4: 4BC3540D  bl 0x828457b0
	ctx.lr = 0x82C103A8;
	sub_828457B0(ctx, base);
	// 82C103A8: 578A103A  slwi r10, r28, 2
	ctx.r[10].u32 = ctx.r[28].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82C103AC: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82C103B0: 57AB103A  slwi r11, r29, 2
	ctx.r[11].u32 = ctx.r[29].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82C103B4: 7D4AF214  add r10, r10, r30
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[30].u64;
	// 82C103B8: 7D2BF214  add r9, r11, r30
	ctx.r[9].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82C103BC: 915F000C  stw r10, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 82C103C0: 913F0008  stw r9, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82C103C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82C103C8: 48099090  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C103D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82C103D0 size=440
    let mut pc: u32 = 0x82C103D0;
    'dispatch: loop {
        match pc {
            0x82C103D0 => {
    //   block [0x82C103D0..0x82C10588)
	// 82C103D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C103D4: 4809902D  bl 0x82ca9400
	ctx.lr = 0x82C103D8;
	sub_82CA93D0(ctx, base);
	// 82C103D8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C103DC: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82C103E0: 3B400001  li r26, 1
	ctx.r[26].s64 = 1;
	// 82C103E4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82C103E8: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82C103EC: 7F5FD378  mr r31, r26
	ctx.r[31].u64 = ctx.r[26].u64;
	// 82C103F0: 83DC0004  lwz r30, 4(r28)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C103F4: 811E0004  lwz r8, 4(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C103F8: 89680011  lbz r11, 0x11(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[8].u32.wrapping_add(17 as u32) ) } as u64;
	// 82C103FC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C10400: 409A0074  bne cr6, 0x82c10474
	if !ctx.cr[6].eq {
	pc = 0x82C10474; continue 'dispatch;
	}
	// 82C10404: 80DB0000  lwz r6, 0(r27)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C10408: 80E6008C  lwz r7, 0x8c(r6)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(140 as u32) ) } as u64;
	// 82C1040C: 8168000C  lwz r11, 0xc(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C10410: 7D1E4378  mr r30, r8
	ctx.r[30].u64 = ctx.r[8].u64;
	// 82C10414: 814B008C  lwz r10, 0x8c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(140 as u32) ) } as u64;
	// 82C10418: 7F075040  cmplw cr6, r7, r10
	ctx.cr[6].compare_u32(ctx.r[7].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82C1041C: 419A0028  beq cr6, 0x82c10444
	if ctx.cr[6].eq {
	pc = 0x82C10444; continue 'dispatch;
	}
	// 82C10420: 81260098  lwz r9, 0x98(r6)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(152 as u32) ) } as u64;
	// 82C10424: 2F090002  cmpwi cr6, r9, 2
	ctx.cr[6].compare_i32(ctx.r[9].s32, 2, &mut ctx.xer);
	// 82C10428: 409A000C  bne cr6, 0x82c10434
	if !ctx.cr[6].eq {
	pc = 0x82C10434; continue 'dispatch;
	}
	// 82C1042C: 7D6A3810  subfc r11, r10, r7
	ctx.xer.ca = ctx.r[7].u32 >= ctx.r[10].u32;
	ctx.r[11].s64 = ctx.r[7].s64 - ctx.r[10].s64;
	// 82C10430: 48000018  b 0x82c10448
	pc = 0x82C10448; continue 'dispatch;
	// 82C10434: 2F090001  cmpwi cr6, r9, 1
	ctx.cr[6].compare_i32(ctx.r[9].s32, 1, &mut ctx.xer);
	// 82C10438: 409A000C  bne cr6, 0x82c10444
	if !ctx.cr[6].eq {
	pc = 0x82C10444; continue 'dispatch;
	}
	// 82C1043C: 7D675010  subfc r11, r7, r10
	ctx.xer.ca = ctx.r[10].u32 >= ctx.r[7].u32;
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[7].s64;
	// 82C10440: 48000008  b 0x82c10448
	pc = 0x82C10448; continue 'dispatch;
	// 82C10444: 7D6B3010  subfc r11, r11, r6
	ctx.xer.ca = ctx.r[6].u32 >= ctx.r[11].u32;
	ctx.r[11].s64 = ctx.r[6].s64 - ctx.r[11].s64;
	// 82C10448: 7D4B5910  subfe r10, r11, r11
	let x = (!ctx.r[11].u32);
	let y = ctx.r[11].u32;
	let s = x.wrapping_add(y);
	let res = s.wrapping_add(ctx.xer.ca as u32);
	tmp.u8 = (s < x) as u8 | (res < s) as u8;
	ctx.r[10].u32 = res;
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	ctx.xer.ca = (tmp.u8 != 0);
	// 82C1044C: 555F07FE  clrlwi r31, r10, 0x1f
	ctx.r[31].u64 = ctx.r[10].u32 as u64 & 0x00000001u64;
	// 82C10450: 57EB063E  clrlwi r11, r31, 0x18
	ctx.r[11].u64 = ctx.r[31].u32 as u64 & 0x000000FFu64;
	// 82C10454: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C10458: 419A000C  beq cr6, 0x82c10464
	if ctx.cr[6].eq {
	pc = 0x82C10464; continue 'dispatch;
	}
	// 82C1045C: 81080000  lwz r8, 0(r8)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C10460: 48000008  b 0x82c10468
	pc = 0x82C10468; continue 'dispatch;
	// 82C10464: 81080008  lwz r8, 8(r8)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C10468: 89680011  lbz r11, 0x11(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[8].u32.wrapping_add(17 as u32) ) } as u64;
	// 82C1046C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C10470: 419AFF9C  beq cr6, 0x82c1040c
	if ctx.cr[6].eq {
	pc = 0x82C1040C; continue 'dispatch;
	}
	// 82C10474: 57EB063E  clrlwi r11, r31, 0x18
	ctx.r[11].u64 = ctx.r[31].u32 as u64 & 0x000000FFu64;
	// 82C10478: 93810050  stw r28, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[28].u32 ) };
	// 82C1047C: 93C10054  stw r30, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[30].u32 ) };
	// 82C10480: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C10484: 419A0054  beq cr6, 0x82c104d8
	if ctx.cr[6].eq {
	pc = 0x82C104D8; continue 'dispatch;
	}
	// 82C10488: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C1048C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C10490: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C10494: 7F1E5040  cmplw cr6, r30, r10
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82C10498: 409A003C  bne cr6, 0x82c104d4
	if !ctx.cr[6].eq {
	pc = 0x82C104D4; continue 'dispatch;
	}
	// 82C1049C: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 82C104A0: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82C104A4: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82C104A8: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82C104AC: 4807459D  bl 0x82c84a48
	ctx.lr = 0x82C104B0;
	sub_82C84A48(ctx, base);
	// 82C104B0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82C104B4: 9B5D0008  stb r26, 8(r29)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[29].u32.wrapping_add(8 as u32), ctx.r[26].u8 ) };
	// 82C104B8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82C104BC: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C104C0: 812B0004  lwz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C104C4: 915D0000  stw r10, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82C104C8: 913D0004  stw r9, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82C104CC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82C104D0: 48098F80  b 0x82ca9450
	sub_82CA9420(ctx, base);
	return;
	// 82C104D4: 4B846215  bl 0x824566e8
	ctx.lr = 0x82C104D8;
	sub_824566E8(ctx, base);
	// 82C104D8: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82C104DC: 815B0000  lwz r10, 0(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C104E0: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C104E4: 812A008C  lwz r9, 0x8c(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(140 as u32) ) } as u64;
	// 82C104E8: 810B008C  lwz r8, 0x8c(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(140 as u32) ) } as u64;
	// 82C104EC: 7F084840  cmplw cr6, r8, r9
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82C104F0: 419A0028  beq cr6, 0x82c10518
	if ctx.cr[6].eq {
	pc = 0x82C10518; continue 'dispatch;
	}
	// 82C104F4: 80EB0098  lwz r7, 0x98(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(152 as u32) ) } as u64;
	// 82C104F8: 2F070002  cmpwi cr6, r7, 2
	ctx.cr[6].compare_i32(ctx.r[7].s32, 2, &mut ctx.xer);
	// 82C104FC: 409A000C  bne cr6, 0x82c10508
	if !ctx.cr[6].eq {
	pc = 0x82C10508; continue 'dispatch;
	}
	// 82C10500: 7D694010  subfc r11, r9, r8
	ctx.xer.ca = ctx.r[8].u32 >= ctx.r[9].u32;
	ctx.r[11].s64 = ctx.r[8].s64 - ctx.r[9].s64;
	// 82C10504: 48000018  b 0x82c1051c
	pc = 0x82C1051C; continue 'dispatch;
	// 82C10508: 2F070001  cmpwi cr6, r7, 1
	ctx.cr[6].compare_i32(ctx.r[7].s32, 1, &mut ctx.xer);
	// 82C1050C: 409A000C  bne cr6, 0x82c10518
	if !ctx.cr[6].eq {
	pc = 0x82C10518; continue 'dispatch;
	}
	// 82C10510: 7D684810  subfc r11, r8, r9
	ctx.xer.ca = ctx.r[9].u32 >= ctx.r[8].u32;
	ctx.r[11].s64 = ctx.r[9].s64 - ctx.r[8].s64;
	// 82C10514: 48000008  b 0x82c1051c
	pc = 0x82C1051C; continue 'dispatch;
	// 82C10518: 7D6A5810  subfc r11, r10, r11
	ctx.xer.ca = ctx.r[11].u32 >= ctx.r[10].u32;
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82C1051C: 7D4B5910  subfe r10, r11, r11
	let x = (!ctx.r[11].u32);
	let y = ctx.r[11].u32;
	let s = x.wrapping_add(y);
	let res = s.wrapping_add(ctx.xer.ca as u32);
	tmp.u8 = (s < x) as u8 | (res < s) as u8;
	ctx.r[10].u32 = res;
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	ctx.xer.ca = (tmp.u8 != 0);
	// 82C10520: 554B07FE  clrlwi r11, r10, 0x1f
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0x00000001u64;
	// 82C10524: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82C10528: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C1052C: 419A0040  beq cr6, 0x82c1056c
	if ctx.cr[6].eq {
	pc = 0x82C1056C; continue 'dispatch;
	}
	// 82C10530: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 82C10534: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82C10538: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82C1053C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82C10540: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C10544: 48074505  bl 0x82c84a48
	ctx.lr = 0x82C10548;
	sub_82C84A48(ctx, base);
	// 82C10548: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82C1054C: 9B5D0008  stb r26, 8(r29)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[29].u32.wrapping_add(8 as u32), ctx.r[26].u8 ) };
	// 82C10550: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82C10554: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C10558: 812B0004  lwz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C1055C: 915D0000  stw r10, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82C10560: 913D0004  stw r9, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82C10564: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82C10568: 48098EE8  b 0x82ca9450
	sub_82CA9420(ctx, base);
	return;
	// 82C1056C: E9610050  ld r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82C10570: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82C10574: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82C10578: 995D0008  stb r10, 8(r29)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[29].u32.wrapping_add(8 as u32), ctx.r[10].u8 ) };
	// 82C1057C: F97D0000  std r11, 0(r29)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u64 ) };
	// 82C10580: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82C10584: 48098ECC  b 0x82ca9450
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C10588(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C10588 size=144
    let mut pc: u32 = 0x82C10588;
    'dispatch: loop {
        match pc {
            0x82C10588 => {
    //   block [0x82C10588..0x82C10618)
	// 82C10588: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C1058C: 48098E81  bl 0x82ca940c
	ctx.lr = 0x82C10590;
	sub_82CA93D0(ctx, base);
	// 82C10590: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C10594: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C10598: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82C1059C: 4BFFEEC5  bl 0x82c0f460
	ctx.lr = 0x82C105A0;
	sub_82C0F460(ctx, base);
	// 82C105A0: 9061005C  stw r3, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[3].u32 ) };
	// 82C105A4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82C105A8: 409A0008  bne cr6, 0x82c105b0
	if !ctx.cr[6].eq {
	pc = 0x82C105B0; continue 'dispatch;
	}
	// 82C105AC: 0FE00016  twui r0, 0x16
	// twui: trap word unsigned immediate — TODO: implement trap semantics
	// 82C105B0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82C105B4: 93E10058  stw r31, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[31].u32 ) };
	// 82C105B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C105BC: 4BFFEE1D  bl 0x82c0f3d8
	ctx.lr = 0x82C105C0;
	sub_82C0F3D8(ctx, base);
	// 82C105C0: 90610064  stw r3, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[3].u32 ) };
	// 82C105C4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82C105C8: 409A0008  bne cr6, 0x82c105d0
	if !ctx.cr[6].eq {
	pc = 0x82C105D0; continue 'dispatch;
	}
	// 82C105CC: 0FE00016  twui r0, 0x16
	// twui: trap word unsigned immediate — TODO: implement trap semantics
	// 82C105D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82C105D4: EBC10058  ld r30, 0x58(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 82C105D8: 93E10060  stw r31, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[31].u32 ) };
	// 82C105DC: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 82C105E0: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 82C105E4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82C105E8: EBA10060  ld r29, 0x60(r1)
	ctx.r[29].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) };
	// 82C105EC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82C105F0: 88C10050  lbz r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82C105F4: 48073A5D  bl 0x82c84050
	ctx.lr = 0x82C105F8;
	sub_82C84050(ctx, base);
	// 82C105F8: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82C105FC: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82C10600: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82C10604: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82C10608: 48074359  bl 0x82c84960
	ctx.lr = 0x82C1060C;
	sub_82C84960(ctx, base);
	// 82C1060C: 80610058  lwz r3, 0x58(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82C10610: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82C10614: 48098E48  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C10618(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C10618 size=220
    let mut pc: u32 = 0x82C10618;
    'dispatch: loop {
        match pc {
            0x82C10618 => {
    //   block [0x82C10618..0x82C106F4)
	// 82C10618: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C1061C: 48098DF1  bl 0x82ca940c
	ctx.lr = 0x82C10620;
	sub_82CA93D0(ctx, base);
	// 82C10620: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C10624: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82C10628: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82C1062C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82C10630: F88100A0  std r4, 0xa0(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(160 as u32), ctx.r[4].u64 ) };
	// 82C10634: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C10638: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C1063C: 419A0014  beq cr6, 0x82c10650
	if ctx.cr[6].eq {
	pc = 0x82C10650; continue 'dispatch;
	}
	// 82C10640: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C10644: 7D2B5050  subf r9, r11, r10
	ctx.r[9].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 82C10648: 7D291671  srawi. r9, r9, 2
	ctx.xer.ca = (ctx.r[9].s32 < 0) && ((ctx.r[9].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[9].s64 = (ctx.r[9].s32 >> 2) as i64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82C1064C: 4082000C  bne 0x82c10658
	if !ctx.cr[0].eq {
	pc = 0x82C10658; continue 'dispatch;
	}
	// 82C10650: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82C10654: 48000034  b 0x82c10688
	pc = 0x82C10688; continue 'dispatch;
	// 82C10658: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82C1065C: 40990008  ble cr6, 0x82c10664
	if !ctx.cr[6].gt {
	pc = 0x82C10664; continue 'dispatch;
	}
	// 82C10660: 0FE00016  twui r0, 0x16
	// twui: trap word unsigned immediate — TODO: implement trap semantics
	// 82C10664: 814100A0  lwz r10, 0xa0(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(160 as u32) ) } as u64;
	// 82C10668: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C1066C: 419A000C  beq cr6, 0x82c10678
	if ctx.cr[6].eq {
	pc = 0x82C10678; continue 'dispatch;
	}
	// 82C10670: 7F0AF840  cmplw cr6, r10, r31
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[31].u32, &mut ctx.xer);
	// 82C10674: 419A0008  beq cr6, 0x82c1067c
	if ctx.cr[6].eq {
	pc = 0x82C1067C; continue 'dispatch;
	}
	// 82C10678: 0FE00016  twui r0, 0x16
	// twui: trap word unsigned immediate — TODO: implement trap semantics
	// 82C1067C: 814100A4  lwz r10, 0xa4(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(164 as u32) ) } as u64;
	// 82C10680: 7D2B5050  subf r9, r11, r10
	ctx.r[9].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 82C10684: 7D3E1670  srawi r30, r9, 2
	ctx.xer.ca = (ctx.r[9].s32 < 0) && ((ctx.r[9].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[30].s64 = (ctx.r[9].s32 >> 2) as i64;
	// 82C10688: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82C1068C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C10690: 4BFFF841  bl 0x82c0fed0
	ctx.lr = 0x82C10694;
	sub_82C0FED0(ctx, base);
	// 82C10694: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C10698: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C1069C: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82C106A0: 40990008  ble cr6, 0x82c106a8
	if !ctx.cr[6].gt {
	pc = 0x82C106A8; continue 'dispatch;
	}
	// 82C106A4: 0FE00016  twui r0, 0x16
	// twui: trap word unsigned immediate — TODO: implement trap semantics
	// 82C106A8: 57CA103A  slwi r10, r30, 2
	ctx.r[10].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82C106AC: 813F0008  lwz r9, 8(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C106B0: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 82C106B4: 7D4A5A14  add r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82C106B8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82C106BC: E9010050  ld r8, 0x50(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82C106C0: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82C106C4: F9010050  std r8, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[8].u64 ) };
	// 82C106C8: 41990010  bgt cr6, 0x82c106d8
	if ctx.cr[6].gt {
	pc = 0x82C106D8; continue 'dispatch;
	}
	// 82C106CC: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C106D0: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82C106D4: 40980008  bge cr6, 0x82c106dc
	if !ctx.cr[6].lt {
	pc = 0x82C106DC; continue 'dispatch;
	}
	// 82C106D8: 0FE00016  twui r0, 0x16
	// twui: trap word unsigned immediate — TODO: implement trap semantics
	// 82C106DC: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 82C106E0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82C106E4: E9610050  ld r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82C106E8: F97D0000  std r11, 0(r29)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u64 ) };
	// 82C106EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82C106F0: 48098D6C  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C106F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C106F8 size=308
    let mut pc: u32 = 0x82C106F8;
    'dispatch: loop {
        match pc {
            0x82C106F8 => {
    //   block [0x82C106F8..0x82C10748)
	// 82C106F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C106FC: 48098D0D  bl 0x82ca9408
	ctx.lr = 0x82C10700;
	sub_82CA93D0(ctx, base);
	// 82C10700: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C10704: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82C10708: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C1070C: 397EFFFF  addi r11, r30, -1
	ctx.r[11].s64 = ctx.r[30].s64 + -1;
	// 82C10710: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82C10714: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82C10718: 2B0B0003  cmplwi cr6, r11, 3
	ctx.cr[6].compare_u32(ctx.r[11].u32, 3 as u32, &mut ctx.xer);
	// 82C1071C: 4199005C  bgt cr6, 0x82c10778
	if ctx.cr[6].gt {
	pc = 0x82C10778; continue 'dispatch;
	}
	// 82C10720: 3D8082C1  lis r12, -0x7d3f
	ctx.r[12].s64 = -2101280768;
	// 82C10724: 398C0738  addi r12, r12, 0x738
	ctx.r[12].s64 = ctx.r[12].s64 + 1848;
	// 82C10728: 5560103A  slwi r0, r11, 2
	ctx.r[0].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[0].u64 = ctx.r[0].u32 as u64;
	// 82C1072C: 7C0C002E  lwzx r0, r12, r0
	ctx.r[0].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[12].u32.wrapping_add(ctx.r[0].u32)) } as u64;
	// 82C10730: 7C0903A6  mtctr r0
	ctx.ctr.u64 = ctx.r[0].u64;
	// 82C10734: 4E800420  bctr
	match ctx.r[11].u64 {
		0 => {
	pc = 0x82C10748; continue 'dispatch;
		},
		1 => {
	pc = 0x82C10750; continue 'dispatch;
		},
		2 => {
	pc = 0x82C10758; continue 'dispatch;
		},
		3 => {
	pc = 0x82C10760; continue 'dispatch;
		},
		_ => unsafe { core::hint::unreachable_unchecked() },
	}
	// 82C10738: 82C10748  lwz r22, 0x748(r1)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(1864 as u32) ) } as u64;
	// 82C1073C: 82C10750  lwz r22, 0x750(r1)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(1872 as u32) ) } as u64;
	// 82C10740: 82C10758  lwz r22, 0x758(r1)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(1880 as u32) ) } as u64;
	// 82C10744: 82C10760  lwz r22, 0x760(r1)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(1888 as u32) ) } as u64;
            }
            0x82C10748 => {
    //   block [0x82C10748..0x82C10750)
	// 82C10748: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 82C1074C: 48000018  b 0x82c10764
	pc = 0x82C10764; continue 'dispatch;
            }
            0x82C10750 => {
    //   block [0x82C10750..0x82C10758)
	// 82C10750: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C10754: 48000010  b 0x82c10764
	pc = 0x82C10764; continue 'dispatch;
            }
            0x82C10758 => {
    //   block [0x82C10758..0x82C10760)
	// 82C10758: 387F0018  addi r3, r31, 0x18
	ctx.r[3].s64 = ctx.r[31].s64 + 24;
	// 82C1075C: 48000008  b 0x82c10764
	pc = 0x82C10764; continue 'dispatch;
            }
            0x82C10760 => {
    //   block [0x82C10760..0x82C107D0)
	// 82C10760: 387F0024  addi r3, r31, 0x24
	ctx.r[3].s64 = ctx.r[31].s64 + 36;
	// 82C10764: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C10768: 419A0010  beq cr6, 0x82c10778
	if ctx.cr[6].eq {
	pc = 0x82C10778; continue 'dispatch;
	}
	// 82C1076C: 93A10050  stw r29, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[29].u32 ) };
	// 82C10770: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82C10774: 4BFFFE15  bl 0x82c10588
	ctx.lr = 0x82C10778;
	sub_82C10588(ctx, base);
	// 82C10778: 2F1E0003  cmpwi cr6, r30, 3
	ctx.cr[6].compare_i32(ctx.r[30].s32, 3, &mut ctx.xer);
	// 82C1077C: 409A001C  bne cr6, 0x82c10798
	if !ctx.cr[6].eq {
	pc = 0x82C10798; continue 'dispatch;
	}
	// 82C10780: 357F0038  addic. r11, r31, 0x38
	ctx.xer.ca = (ctx.r[31].u32 > (!(56 as u32)));
	ctx.r[11].s64 = ctx.r[31].s64 + 56;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82C10784: 41820014  beq 0x82c10798
	if ctx.cr[0].eq {
	pc = 0x82C10798; continue 'dispatch;
	}
	// 82C10788: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C1078C: 813D0088  lwz r9, 0x88(r29)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(136 as u32) ) } as u64;
	// 82C10790: 7D095050  subf r8, r9, r10
	ctx.r[8].s64 = ctx.r[10].s64 - ctx.r[9].s64;
	// 82C10794: 910B0000  stw r8, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82C10798: 397CFFFF  addi r11, r28, -1
	ctx.r[11].s64 = ctx.r[28].s64 + -1;
	// 82C1079C: 939D0098  stw r28, 0x98(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(152 as u32), ctx.r[28].u32 ) };
	// 82C107A0: 2B0B0003  cmplwi cr6, r11, 3
	ctx.cr[6].compare_u32(ctx.r[11].u32, 3 as u32, &mut ctx.xer);
	// 82C107A4: 41990060  bgt cr6, 0x82c10804
	if ctx.cr[6].gt {
	pc = 0x82C10804; continue 'dispatch;
	}
	// 82C107A8: 3D8082C1  lis r12, -0x7d3f
	ctx.r[12].s64 = -2101280768;
	// 82C107AC: 398C07C0  addi r12, r12, 0x7c0
	ctx.r[12].s64 = ctx.r[12].s64 + 1984;
	// 82C107B0: 5560103A  slwi r0, r11, 2
	ctx.r[0].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[0].u64 = ctx.r[0].u32 as u64;
	// 82C107B4: 7C0C002E  lwzx r0, r12, r0
	ctx.r[0].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[12].u32.wrapping_add(ctx.r[0].u32)) } as u64;
	// 82C107B8: 7C0903A6  mtctr r0
	ctx.ctr.u64 = ctx.r[0].u64;
	// 82C107BC: 4E800420  bctr
	match ctx.r[11].u64 {
		0 => {
	pc = 0x82C107D0; continue 'dispatch;
		},
		1 => {
	pc = 0x82C107D8; continue 'dispatch;
		},
		2 => {
	pc = 0x82C107E0; continue 'dispatch;
		},
		3 => {
	pc = 0x82C107E8; continue 'dispatch;
		},
		_ => unsafe { core::hint::unreachable_unchecked() },
	}
	// 82C107C0: 82C107D0  lwz r22, 0x7d0(r1)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(2000 as u32) ) } as u64;
	// 82C107C4: 82C107D8  lwz r22, 0x7d8(r1)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(2008 as u32) ) } as u64;
	// 82C107C8: 82C107E0  lwz r22, 0x7e0(r1)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(2016 as u32) ) } as u64;
	// 82C107CC: 82C107E8  lwz r22, 0x7e8(r1)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(2024 as u32) ) } as u64;
            }
            0x82C107D0 => {
    //   block [0x82C107D0..0x82C107D8)
	// 82C107D0: 389F000C  addi r4, r31, 0xc
	ctx.r[4].s64 = ctx.r[31].s64 + 12;
	// 82C107D4: 48000018  b 0x82c107ec
	pc = 0x82C107EC; continue 'dispatch;
            }
            0x82C107D8 => {
    //   block [0x82C107D8..0x82C107E0)
	// 82C107D8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82C107DC: 48000010  b 0x82c107ec
	pc = 0x82C107EC; continue 'dispatch;
            }
            0x82C107E0 => {
    //   block [0x82C107E0..0x82C107E8)
	// 82C107E0: 389F0018  addi r4, r31, 0x18
	ctx.r[4].s64 = ctx.r[31].s64 + 24;
	// 82C107E4: 48000008  b 0x82c107ec
	pc = 0x82C107EC; continue 'dispatch;
            }
            0x82C107E8 => {
    //   block [0x82C107E8..0x82C1082C)
	// 82C107E8: 389F0024  addi r4, r31, 0x24
	ctx.r[4].s64 = ctx.r[31].s64 + 36;
	// 82C107EC: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82C107F0: 419A0014  beq cr6, 0x82c10804
	if ctx.cr[6].eq {
	pc = 0x82C10804; continue 'dispatch;
	}
	// 82C107F4: 93A10050  stw r29, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[29].u32 ) };
	// 82C107F8: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82C107FC: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82C10800: 4BFFFBD1  bl 0x82c103d0
	ctx.lr = 0x82C10804;
	sub_82C103D0(ctx, base);
	// 82C10804: 2F1C0003  cmpwi cr6, r28, 3
	ctx.cr[6].compare_i32(ctx.r[28].s32, 3, &mut ctx.xer);
	// 82C10808: 409A001C  bne cr6, 0x82c10824
	if !ctx.cr[6].eq {
	pc = 0x82C10824; continue 'dispatch;
	}
	// 82C1080C: 357F0038  addic. r11, r31, 0x38
	ctx.xer.ca = (ctx.r[31].u32 > (!(56 as u32)));
	ctx.r[11].s64 = ctx.r[31].s64 + 56;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82C10810: 41820014  beq 0x82c10824
	if ctx.cr[0].eq {
	pc = 0x82C10824; continue 'dispatch;
	}
	// 82C10814: 815D0088  lwz r10, 0x88(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(136 as u32) ) } as u64;
	// 82C10818: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C1081C: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82C10820: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82C10824: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82C10828: 48098C30  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C10830(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C10830 size=168
    let mut pc: u32 = 0x82C10830;
    'dispatch: loop {
        match pc {
            0x82C10830 => {
    //   block [0x82C10830..0x82C108D8)
	// 82C10830: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C10834: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C10838: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C1083C: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C10840: 7C862378  mr r6, r4
	ctx.r[6].u64 = ctx.r[4].u64;
	// 82C10844: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C10848: 409A000C  bne cr6, 0x82c10854
	if !ctx.cr[6].eq {
	pc = 0x82C10854; continue 'dispatch;
	}
	// 82C1084C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82C10850: 48000010  b 0x82c10860
	pc = 0x82C10860; continue 'dispatch;
	// 82C10854: 81430008  lwz r10, 8(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C10858: 7D2B5050  subf r9, r11, r10
	ctx.r[9].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 82C1085C: 7D291670  srawi r9, r9, 2
	ctx.xer.ca = (ctx.r[9].s32 < 0) && ((ctx.r[9].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[9].s64 = (ctx.r[9].s32 >> 2) as i64;
	// 82C10860: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C10864: 419A003C  beq cr6, 0x82c108a0
	if ctx.cr[6].eq {
	pc = 0x82C108A0; continue 'dispatch;
	}
	// 82C10868: 8143000C  lwz r10, 0xc(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C1086C: 7D0B5050  subf r8, r11, r10
	ctx.r[8].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 82C10870: 7D0A1670  srawi r10, r8, 2
	ctx.xer.ca = (ctx.r[8].s32 < 0) && ((ctx.r[8].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[8].s32 >> 2) as i64;
	// 82C10874: 7F095040  cmplw cr6, r9, r10
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82C10878: 40980028  bge cr6, 0x82c108a0
	if !ctx.cr[6].lt {
	pc = 0x82C108A0; continue 'dispatch;
	}
	// 82C1087C: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C10880: 81460000  lwz r10, 0(r6)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C10884: 392B0004  addi r9, r11, 4
	ctx.r[9].s64 = ctx.r[11].s64 + 4;
	// 82C10888: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82C1088C: 91230008  stw r9, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82C10890: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C10894: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C10898: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C1089C: 4E800020  blr
	return;
	// 82C108A0: 81430008  lwz r10, 8(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C108A4: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82C108A8: 40990008  ble cr6, 0x82c108b0
	if !ctx.cr[6].gt {
	pc = 0x82C108B0; continue 'dispatch;
	}
	// 82C108AC: 0FE00016  twui r0, 0x16
	// twui: trap word unsigned immediate — TODO: implement trap semantics
	// 82C108B0: 90610050  stw r3, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82C108B4: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82C108B8: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82C108BC: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 82C108C0: E8A10050  ld r5, 0x50(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82C108C4: 4BFFFD55  bl 0x82c10618
	ctx.lr = 0x82C108C8;
	sub_82C10618(ctx, base);
	// 82C108C8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C108CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C108D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C108D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C108D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C108D8 size=180
    let mut pc: u32 = 0x82C108D8;
    'dispatch: loop {
        match pc {
            0x82C108D8 => {
    //   block [0x82C108D8..0x82C1098C)
	// 82C108D8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C108DC: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82C108E0: 392BAA88  addi r9, r11, -0x5578
	ctx.r[9].s64 = ctx.r[11].s64 + -21880;
	// 82C108E4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82C108E8: 390AAA14  addi r8, r10, -0x55ec
	ctx.r[8].s64 = ctx.r[10].s64 + -21996;
	// 82C108EC: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82C108F0: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82C108F4: 38E0FFFF  li r7, -1
	ctx.r[7].s64 = -1;
	// 82C108F8: 91030004  stw r8, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82C108FC: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82C10900: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82C10904: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82C10908: 91630014  stw r11, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82C1090C: 91630018  stw r11, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82C10910: 9163001C  stw r11, 0x1c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82C10914: 91630020  stw r11, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82C10918: 91630024  stw r11, 0x24(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 82C1091C: 9163002C  stw r11, 0x2c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 82C10920: 91630030  stw r11, 0x30(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 82C10924: 91630034  stw r11, 0x34(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 82C10928: 9163003C  stw r11, 0x3c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(60 as u32), ctx.r[11].u32 ) };
	// 82C1092C: 91630040  stw r11, 0x40(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(64 as u32), ctx.r[11].u32 ) };
	// 82C10930: F9630060  std r11, 0x60(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(96 as u32), ctx.r[11].u64 ) };
	// 82C10934: F9630068  std r11, 0x68(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(104 as u32), ctx.r[11].u64 ) };
	// 82C10938: 91630070  stw r11, 0x70(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(112 as u32), ctx.r[11].u32 ) };
	// 82C1093C: 91630074  stw r11, 0x74(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82C10940: 91630078  stw r11, 0x78(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(120 as u32), ctx.r[11].u32 ) };
	// 82C10944: 9163007C  stw r11, 0x7c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(124 as u32), ctx.r[11].u32 ) };
	// 82C10948: 91630080  stw r11, 0x80(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 82C1094C: 91630084  stw r11, 0x84(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(132 as u32), ctx.r[11].u32 ) };
	// 82C10950: 91630088  stw r11, 0x88(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(136 as u32), ctx.r[11].u32 ) };
	// 82C10954: 90E3008C  stw r7, 0x8c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(140 as u32), ctx.r[7].u32 ) };
	// 82C10958: 91630090  stw r11, 0x90(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(144 as u32), ctx.r[11].u32 ) };
	// 82C1095C: 91630094  stw r11, 0x94(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(148 as u32), ctx.r[11].u32 ) };
	// 82C10960: 91630098  stw r11, 0x98(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(152 as u32), ctx.r[11].u32 ) };
	// 82C10964: 98C3009C  stb r6, 0x9c(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(156 as u32), ctx.r[6].u8 ) };
	// 82C10968: 9963009D  stb r11, 0x9d(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(157 as u32), ctx.r[11].u8 ) };
	// 82C1096C: 9963009E  stb r11, 0x9e(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(158 as u32), ctx.r[11].u8 ) };
	// 82C10970: 9963009F  stb r11, 0x9f(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(159 as u32), ctx.r[11].u8 ) };
	// 82C10974: F96300A0  std r11, 0xa0(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(160 as u32), ctx.r[11].u64 ) };
	// 82C10978: 916300A8  stw r11, 0xa8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(168 as u32), ctx.r[11].u32 ) };
	// 82C1097C: 996300AC  stb r11, 0xac(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(172 as u32), ctx.r[11].u8 ) };
	// 82C10980: 90630008  stw r3, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[3].u32 ) };
	// 82C10984: 90630038  stw r3, 0x38(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(56 as u32), ctx.r[3].u32 ) };
	// 82C10988: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C10990(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C10990 size=204
    let mut pc: u32 = 0x82C10990;
    'dispatch: loop {
        match pc {
            0x82C10990 => {
    //   block [0x82C10990..0x82C10A5C)
	// 82C10990: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C10994: 48098A71  bl 0x82ca9404
	ctx.lr = 0x82C10998;
	sub_82CA93D0(ctx, base);
	// 82C10998: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C1099C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C109A0: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82C109A4: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82C109A8: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 82C109AC: 4B86F88D  bl 0x82480238
	ctx.lr = 0x82C109B0;
	sub_82480238(ctx, base);
	// 82C109B0: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 82C109B4: 4B86F885  bl 0x82480238
	ctx.lr = 0x82C109B8;
	sub_82480238(ctx, base);
	// 82C109B8: 387F0018  addi r3, r31, 0x18
	ctx.r[3].s64 = ctx.r[31].s64 + 24;
	// 82C109BC: 4B86F87D  bl 0x82480238
	ctx.lr = 0x82C109C0;
	sub_82480238(ctx, base);
	// 82C109C0: 387F0024  addi r3, r31, 0x24
	ctx.r[3].s64 = ctx.r[31].s64 + 36;
	// 82C109C4: 4B86F875  bl 0x82480238
	ctx.lr = 0x82C109C8;
	sub_82480238(ctx, base);
	// 82C109C8: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82C109CC: 939F003C  stw r28, 0x3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[28].u32 ) };
	// 82C109D0: 937F0040  stw r27, 0x40(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), ctx.r[27].u32 ) };
	// 82C109D4: 939F0044  stw r28, 0x44(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(68 as u32), ctx.r[28].u32 ) };
	// 82C109D8: 93DF0038  stw r30, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[30].u32 ) };
	// 82C109DC: 93DF0030  stw r30, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[30].u32 ) };
	// 82C109E0: 937F0048  stw r27, 0x48(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[27].u32 ) };
	// 82C109E4: 93DF004C  stw r30, 0x4c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(76 as u32), ctx.r[30].u32 ) };
	// 82C109E8: 93BF0050  stw r29, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[29].u32 ) };
	// 82C109EC: 4BFF2245  bl 0x82c02c30
	ctx.lr = 0x82C109F0;
	sub_82C02C30(ctx, base);
	// 82C109F0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82C109F4: 3860001C  li r3, 0x1c
	ctx.r[3].s64 = 28;
	// 82C109F8: 814B0010  lwz r10, 0x10(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C109FC: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C10A00: 7D0A49D6  mullw r8, r10, r9
	ctx.r[8].s64 = (ctx.r[10].s32 as i64) * (ctx.r[9].s32 as i64);
	// 82C10A04: 911F0034  stw r8, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[8].u32 ) };
	// 82C10A08: 814B0018  lwz r10, 0x18(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82C10A0C: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82C10A10: 7CEA5A14  add r7, r10, r11
	ctx.r[7].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82C10A14: 7FA741D6  mullw r29, r7, r8
	ctx.r[29].s64 = (ctx.r[7].s32 as i64) * (ctx.r[8].s32 as i64);
	// 82C10A18: 4B60E841  bl 0x8221f258
	ctx.lr = 0x82C10A1C;
	sub_8221F258(ctx, base);
	// 82C10A1C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C10A20: 419A002C  beq cr6, 0x82c10a4c
	if ctx.cr[6].eq {
	pc = 0x82C10A4C; continue 'dispatch;
	}
	// 82C10A24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82C10A28: 80BF0034  lwz r5, 0x34(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 82C10A2C: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82C10A30: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82C10A34: 4BFFE7E5  bl 0x82c0f218
	ctx.lr = 0x82C10A38;
	sub_82C0F218(ctx, base);
	// 82C10A38: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82C10A3C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C10A40: 917F0030  stw r11, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 82C10A44: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82C10A48: 48098A0C  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
	// 82C10A4C: 93DF0030  stw r30, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[30].u32 ) };
	// 82C10A50: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C10A54: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82C10A58: 480989FC  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C10A60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C10A60 size=160
    let mut pc: u32 = 0x82C10A60;
    'dispatch: loop {
        match pc {
            0x82C10A60 => {
    //   block [0x82C10A60..0x82C10B00)
	// 82C10A60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C10A64: 480989A9  bl 0x82ca940c
	ctx.lr = 0x82C10A68;
	sub_82CA93D0(ctx, base);
	// 82C10A68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C10A6C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C10A70: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82C10A74: 815F0048  lwz r10, 0x48(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 82C10A78: 817F0044  lwz r11, 0x44(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(68 as u32) ) } as u64;
	// 82C10A7C: 7D2B5214  add r9, r11, r10
	ctx.r[9].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82C10A80: 7F092040  cmplw cr6, r9, r4
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[4].u32, &mut ctx.xer);
	// 82C10A84: 40980010  bge cr6, 0x82c10a94
	if !ctx.cr[6].lt {
	pc = 0x82C10A94; continue 'dispatch;
	}
	// 82C10A88: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82C10A8C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82C10A90: 480989CC  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
	// 82C10A94: 7F0B2040  cmplw cr6, r11, r4
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[4].u32, &mut ctx.xer);
	// 82C10A98: 41980010  blt cr6, 0x82c10aa8
	if ctx.cr[6].lt {
	pc = 0x82C10AA8; continue 'dispatch;
	}
	// 82C10A9C: 7D645850  subf r11, r4, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[4].s64;
	// 82C10AA0: 917F0044  stw r11, 0x44(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(68 as u32), ctx.r[11].u32 ) };
	// 82C10AA4: 4800001C  b 0x82c10ac0
	pc = 0x82C10AC0; continue 'dispatch;
	// 82C10AA8: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82C10AAC: 7D6B2051  subf. r11, r11, r4
	ctx.r[11].s64 = ctx.r[4].s64 - ctx.r[11].s64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82C10AB0: 913F0044  stw r9, 0x44(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(68 as u32), ctx.r[9].u32 ) };
	// 82C10AB4: 4182000C  beq 0x82c10ac0
	if ctx.cr[0].eq {
	pc = 0x82C10AC0; continue 'dispatch;
	}
	// 82C10AB8: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 82C10ABC: 917F0048  stw r11, 0x48(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[11].u32 ) };
	// 82C10AC0: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82C10AC4: 419A0030  beq cr6, 0x82c10af4
	if ctx.cr[6].eq {
	pc = 0x82C10AF4; continue 'dispatch;
	}
	// 82C10AC8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82C10ACC: 807F0030  lwz r3, 0x30(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 82C10AD0: 4BFFDCC9  bl 0x82c0e798
	ctx.lr = 0x82C10AD4;
	sub_82C0E798(ctx, base);
	// 82C10AD4: 90610050  stw r3, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82C10AD8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C10ADC: 419A0010  beq cr6, 0x82c10aec
	if ctx.cr[6].eq {
	pc = 0x82C10AEC; continue 'dispatch;
	}
	// 82C10AE0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82C10AE4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82C10AE8: 4BFFFD49  bl 0x82c10830
	ctx.lr = 0x82C10AEC;
	sub_82C10830(ctx, base);
	// 82C10AEC: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82C10AF0: 4082FFDC  bne 0x82c10acc
	if !ctx.cr[0].eq {
	pc = 0x82C10ACC; continue 'dispatch;
	}
	// 82C10AF4: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82C10AF8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82C10AFC: 48098960  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C10B00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C10B00 size=268
    let mut pc: u32 = 0x82C10B00;
    'dispatch: loop {
        match pc {
            0x82C10B00 => {
    //   block [0x82C10B00..0x82C10C0C)
	// 82C10B00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C10B04: 48098901  bl 0x82ca9404
	ctx.lr = 0x82C10B08;
	sub_82CA93D0(ctx, base);
	// 82C10B08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C10B0C: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82C10B10: 807B0010  lwz r3, 0x10(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C10B14: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C10B18: 814B0030  lwz r10, 0x30(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 82C10B1C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C10B20: 4E800421  bctrl
	ctx.lr = 0x82C10B24;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C10B24: 3B9B0020  addi r28, r27, 0x20
	ctx.r[28].s64 = ctx.r[27].s64 + 32;
	// 82C10B28: 83FB0020  lwz r31, 0x20(r27)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(32 as u32) ) } as u64;
	// 82C10B2C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82C10B30: 419A00B4  beq cr6, 0x82c10be4
	if ctx.cr[6].eq {
	pc = 0x82C10BE4; continue 'dispatch;
	}
	// 82C10B34: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82C10B38: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C10B3C: 7F0BE040  cmplw cr6, r11, r28
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[28].u32, &mut ctx.xer);
	// 82C10B40: 409A005C  bne cr6, 0x82c10b9c
	if !ctx.cr[6].eq {
	pc = 0x82C10B9C; continue 'dispatch;
	}
	// 82C10B44: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C10B48: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C10B4C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C10B50: 409A0024  bne cr6, 0x82c10b74
	if !ctx.cr[6].eq {
	pc = 0x82C10B74; continue 'dispatch;
	}
	// 82C10B54: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C10B58: 409A0010  bne cr6, 0x82c10b68
	if !ctx.cr[6].eq {
	pc = 0x82C10B68; continue 'dispatch;
	}
	// 82C10B5C: 93BC0000  stw r29, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82C10B60: 93BC0004  stw r29, 4(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82C10B64: 4800002C  b 0x82c10b90
	pc = 0x82C10B90; continue 'dispatch;
	// 82C10B68: 917C0000  stw r11, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82C10B6C: 93AB0004  stw r29, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82C10B70: 48000020  b 0x82c10b90
	pc = 0x82C10B90; continue 'dispatch;
	// 82C10B74: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C10B78: 409A0010  bne cr6, 0x82c10b88
	if !ctx.cr[6].eq {
	pc = 0x82C10B88; continue 'dispatch;
	}
	// 82C10B7C: 915C0004  stw r10, 4(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82C10B80: 93AA0000  stw r29, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82C10B84: 4800000C  b 0x82c10b90
	pc = 0x82C10B90; continue 'dispatch;
	// 82C10B88: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82C10B8C: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82C10B90: 93BF0004  stw r29, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82C10B94: 93BF0000  stw r29, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82C10B98: 93BF0008  stw r29, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 82C10B9C: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C10BA0: 3BDF000C  addi r30, r31, 0xc
	ctx.r[30].s64 = ctx.r[31].s64 + 12;
	// 82C10BA4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C10BA8: 419A0014  beq cr6, 0x82c10bbc
	if ctx.cr[6].eq {
	pc = 0x82C10BBC; continue 'dispatch;
	}
	// 82C10BAC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C10BB0: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C10BB4: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C10BB8: 4E800421  bctrl
	ctx.lr = 0x82C10BBC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C10BBC: 93BE0004  stw r29, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82C10BC0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C10BC4: 93BE0000  stw r29, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82C10BC8: 4BC34BE9  bl 0x828457b0
	ctx.lr = 0x82C10BCC;
	sub_828457B0(ctx, base);
	// 82C10BCC: 817B0070  lwz r11, 0x70(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(112 as u32) ) } as u64;
	// 82C10BD0: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82C10BD4: 917B0070  stw r11, 0x70(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(112 as u32), ctx.r[11].u32 ) };
	// 82C10BD8: 83FC0000  lwz r31, 0(r28)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C10BDC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82C10BE0: 409AFF58  bne cr6, 0x82c10b38
	if !ctx.cr[6].eq {
	pc = 0x82C10B38; continue 'dispatch;
	}
	// 82C10BE4: 80BB0098  lwz r5, 0x98(r27)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(152 as u32) ) } as u64;
	// 82C10BE8: 2F050004  cmpwi cr6, r5, 4
	ctx.cr[6].compare_i32(ctx.r[5].s32, 4, &mut ctx.xer);
	// 82C10BEC: 419A0018  beq cr6, 0x82c10c04
	if ctx.cr[6].eq {
	pc = 0x82C10C04; continue 'dispatch;
	}
	// 82C10BF0: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 82C10BF4: 38C00004  li r6, 4
	ctx.r[6].s64 = 4;
	// 82C10BF8: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82C10BFC: 806B4EEC  lwz r3, 0x4eec(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20204 as u32) ) } as u64;
	// 82C10C00: 4BFFFAF9  bl 0x82c106f8
	ctx.lr = 0x82C10C04;
	sub_82C106F8(ctx, base);
	// 82C10C04: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82C10C08: 4809884C  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C10C10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C10C10 size=388
    let mut pc: u32 = 0x82C10C10;
    'dispatch: loop {
        match pc {
            0x82C10C10 => {
    //   block [0x82C10C10..0x82C10D94)
	// 82C10C10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C10C14: 480987F9  bl 0x82ca940c
	ctx.lr = 0x82C10C18;
	sub_82CA93D0(ctx, base);
	// 82C10C18: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C10C1C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C10C20: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82C10C24: 3BA00001  li r29, 1
	ctx.r[29].s64 = 1;
	// 82C10C28: 897F00AC  lbz r11, 0xac(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(172 as u32) ) } as u64;
	// 82C10C2C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C10C30: 409A0084  bne cr6, 0x82c10cb4
	if !ctx.cr[6].eq {
	pc = 0x82C10CB4; continue 'dispatch;
	}
	// 82C10C34: 897F009D  lbz r11, 0x9d(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(157 as u32) ) } as u64;
	// 82C10C38: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C10C3C: 419A0078  beq cr6, 0x82c10cb4
	if ctx.cr[6].eq {
	pc = 0x82C10CB4; continue 'dispatch;
	}
	// 82C10C40: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C10C44: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C10C48: 814B0024  lwz r10, 0x24(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 82C10C4C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C10C50: 4E800421  bctrl
	ctx.lr = 0x82C10C54;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C10C54: 5469063E  clrlwi r9, r3, 0x18
	ctx.r[9].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82C10C58: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82C10C5C: 409A0058  bne cr6, 0x82c10cb4
	if !ctx.cr[6].eq {
	pc = 0x82C10CB4; continue 'dispatch;
	}
	// 82C10C60: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C10C64: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82C10C68: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C10C6C: 814B0038  lwz r10, 0x38(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) } as u64;
	// 82C10C70: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C10C74: 4E800421  bctrl
	ctx.lr = 0x82C10C78;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C10C78: E9410050  ld r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82C10C7C: E93F00A0  ld r9, 0xa0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(160 as u32) ) };
	// 82C10C80: 7F295040  cmpld cr6, r9, r10
	ctx.cr[6].compare_u64(ctx.r[9].u64, ctx.r[10].u64, &mut ctx.xer);
	// 82C10C84: 419A0018  beq cr6, 0x82c10c9c
	if ctx.cr[6].eq {
	pc = 0x82C10C9C; continue 'dispatch;
	}
	// 82C10C88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82C10C8C: F95F00A0  std r10, 0xa0(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(160 as u32), ctx.r[10].u64 ) };
	// 82C10C90: 917F00A8  stw r11, 0xa8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(168 as u32), ctx.r[11].u32 ) };
	// 82C10C94: 997F00AC  stb r11, 0xac(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(172 as u32), ctx.r[11].u8 ) };
	// 82C10C98: 4800001C  b 0x82c10cb4
	pc = 0x82C10CB4; continue 'dispatch;
	// 82C10C9C: 817F00A8  lwz r11, 0xa8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(168 as u32) ) } as u64;
	// 82C10CA0: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82C10CA4: 917F00A8  stw r11, 0xa8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(168 as u32), ctx.r[11].u32 ) };
	// 82C10CA8: 2B0B1B58  cmplwi cr6, r11, 0x1b58
	ctx.cr[6].compare_u32(ctx.r[11].u32, 7000 as u32, &mut ctx.xer);
	// 82C10CAC: 40990008  ble cr6, 0x82c10cb4
	if !ctx.cr[6].gt {
	pc = 0x82C10CB4; continue 'dispatch;
	}
	// 82C10CB0: 9BBF00AC  stb r29, 0xac(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(172 as u32), ctx.r[29].u8 ) };
	// 82C10CB4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82C10CB8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C10CBC: 4BFFE3C5  bl 0x82c0f080
	ctx.lr = 0x82C10CC0;
	sub_82C0F080(ctx, base);
	// 82C10CC0: 897F009E  lbz r11, 0x9e(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(158 as u32) ) } as u64;
	// 82C10CC4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C10CC8: 419A001C  beq cr6, 0x82c10ce4
	if ctx.cr[6].eq {
	pc = 0x82C10CE4; continue 'dispatch;
	}
	// 82C10CCC: 897F00AC  lbz r11, 0xac(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(172 as u32) ) } as u64;
	// 82C10CD0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C10CD4: 419A00B8  beq cr6, 0x82c10d8c
	if ctx.cr[6].eq {
	pc = 0x82C10D8C; continue 'dispatch;
	}
	// 82C10CD8: 9BBF005C  stb r29, 0x5c(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[29].u8 ) };
	// 82C10CDC: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82C10CE0: 4809877C  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
	// 82C10CE4: 897F009F  lbz r11, 0x9f(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(159 as u32) ) } as u64;
	// 82C10CE8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C10CEC: 419A0014  beq cr6, 0x82c10d00
	if ctx.cr[6].eq {
	pc = 0x82C10D00; continue 'dispatch;
	}
	// 82C10CF0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C10CF4: 4BFFFE0D  bl 0x82c10b00
	ctx.lr = 0x82C10CF8;
	sub_82C10B00(ctx, base);
	// 82C10CF8: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82C10CFC: 48098760  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
	// 82C10D00: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C10D04: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C10D08: 814B001C  lwz r10, 0x1c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82C10D0C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C10D10: 4E800421  bctrl
	ctx.lr = 0x82C10D14;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C10D14: 813F0098  lwz r9, 0x98(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(152 as u32) ) } as u64;
	// 82C10D18: 2F090003  cmpwi cr6, r9, 3
	ctx.cr[6].compare_i32(ctx.r[9].s32, 3, &mut ctx.xer);
	// 82C10D1C: 409A0068  bne cr6, 0x82c10d84
	if !ctx.cr[6].eq {
	pc = 0x82C10D84; continue 'dispatch;
	}
	// 82C10D20: 546B063E  clrlwi r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82C10D24: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C10D28: 419A005C  beq cr6, 0x82c10d84
	if ctx.cr[6].eq {
	pc = 0x82C10D84; continue 'dispatch;
	}
	// 82C10D2C: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82C10D30: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C10D34: 419A0020  beq cr6, 0x82c10d54
	if ctx.cr[6].eq {
	pc = 0x82C10D54; continue 'dispatch;
	}
	// 82C10D38: 806B000C  lwz r3, 0xc(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C10D3C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C10D40: 419A0014  beq cr6, 0x82c10d54
	if ctx.cr[6].eq {
	pc = 0x82C10D54; continue 'dispatch;
	}
	// 82C10D44: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C10D48: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C10D4C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C10D50: 4E800421  bctrl
	ctx.lr = 0x82C10D54;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C10D54: 3FC08333  lis r30, -0x7ccd
	ctx.r[30].s64 = -2093809664;
	// 82C10D58: 809F0088  lwz r4, 0x88(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(136 as u32) ) } as u64;
	// 82C10D5C: 38BF0028  addi r5, r31, 0x28
	ctx.r[5].s64 = ctx.r[31].s64 + 40;
	// 82C10D60: 807E4EEC  lwz r3, 0x4eec(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20204 as u32) ) } as u64;
	// 82C10D64: 4BFFEEE5  bl 0x82c0fc48
	ctx.lr = 0x82C10D68;
	sub_82C0FC48(ctx, base);
	// 82C10D68: 80BF0098  lwz r5, 0x98(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(152 as u32) ) } as u64;
	// 82C10D6C: 2F050001  cmpwi cr6, r5, 1
	ctx.cr[6].compare_i32(ctx.r[5].s32, 1, &mut ctx.xer);
	// 82C10D70: 419A0014  beq cr6, 0x82c10d84
	if ctx.cr[6].eq {
	pc = 0x82C10D84; continue 'dispatch;
	}
	// 82C10D74: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82C10D78: 807E4EEC  lwz r3, 0x4eec(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20204 as u32) ) } as u64;
	// 82C10D7C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82C10D80: 4BFFF979  bl 0x82c106f8
	ctx.lr = 0x82C10D84;
	sub_82C106F8(ctx, base);
	// 82C10D84: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C10D88: 4BFFEBE1  bl 0x82c0f968
	ctx.lr = 0x82C10D8C;
	sub_82C0F968(ctx, base);
	// 82C10D8C: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82C10D90: 480986CC  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C10D98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C10D98 size=208
    let mut pc: u32 = 0x82C10D98;
    'dispatch: loop {
        match pc {
            0x82C10D98 => {
    //   block [0x82C10D98..0x82C10E68)
	// 82C10D98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C10D9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C10DA0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82C10DA4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C10DA8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C10DAC: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82C10DB0: 81430044  lwz r10, 0x44(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(68 as u32) ) } as u64;
	// 82C10DB4: 7CC43378  mr r4, r6
	ctx.r[4].u64 = ctx.r[6].u64;
	// 82C10DB8: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82C10DBC: 7CE53B78  mr r5, r7
	ctx.r[5].u64 = ctx.r[7].u64;
	// 82C10DC0: 7F0A2040  cmplw cr6, r10, r4
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[4].u32, &mut ctx.xer);
	// 82C10DC4: 4198000C  blt cr6, 0x82c10dd0
	if ctx.cr[6].lt {
	pc = 0x82C10DD0; continue 'dispatch;
	}
	// 82C10DC8: 4BFFFC99  bl 0x82c10a60
	ctx.lr = 0x82C10DCC;
	sub_82C10A60(ctx, base);
	// 82C10DCC: 4800005C  b 0x82c10e28
	pc = 0x82C10E28; continue 'dispatch;
	// 82C10DD0: 81430008  lwz r10, 8(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C10DD4: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C10DD8: 419A004C  beq cr6, 0x82c10e24
	if ctx.cr[6].eq {
	pc = 0x82C10E24; continue 'dispatch;
	}
	// 82C10DDC: 81430048  lwz r10, 0x48(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(72 as u32) ) } as u64;
	// 82C10DE0: 7F045040  cmplw cr6, r4, r10
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82C10DE4: 41990040  bgt cr6, 0x82c10e24
	if ctx.cr[6].gt {
	pc = 0x82C10E24; continue 'dispatch;
	}
	// 82C10DE8: 81030004  lwz r8, 4(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C10DEC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82C10DF0: 5566063E  clrlwi r6, r11, 0x18
	ctx.r[6].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82C10DF4: 83E80000  lwz r31, 0(r8)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C10DF8: 813F000C  lwz r9, 0xc(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C10DFC: 7F1F4040  cmplw cr6, r31, r8
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82C10E00: 409A0008  bne cr6, 0x82c10e08
	if !ctx.cr[6].eq {
	pc = 0x82C10E08; continue 'dispatch;
	}
	// 82C10E04: 0FE00016  twui r0, 0x16
	// twui: trap word unsigned immediate — TODO: implement trap semantics
	// 82C10E08: 8169008C  lwz r11, 0x8c(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(140 as u32) ) } as u64;
	// 82C10E0C: 2B060000  cmplwi cr6, r6, 0
	ctx.cr[6].compare_u32(ctx.r[6].u32, 0 as u32, &mut ctx.xer);
	// 82C10E10: 419A0030  beq cr6, 0x82c10e40
	if ctx.cr[6].eq {
	pc = 0x82C10E40; continue 'dispatch;
	}
	// 82C10E14: 7F0BF040  cmplw cr6, r11, r30
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82C10E18: 40990030  ble cr6, 0x82c10e48
	if !ctx.cr[6].gt {
	pc = 0x82C10E48; continue 'dispatch;
	}
	// 82C10E1C: 7F0A2040  cmplw cr6, r10, r4
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[4].u32, &mut ctx.xer);
	// 82C10E20: 4098FFA8  bge cr6, 0x82c10dc8
	if !ctx.cr[6].lt {
	pc = 0x82C10DC8; continue 'dispatch;
	}
	// 82C10E24: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82C10E28: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C10E2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C10E30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C10E34: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82C10E38: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C10E3C: 4E800020  blr
	return;
	// 82C10E40: 7F0BF040  cmplw cr6, r11, r30
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82C10E44: 4098FFD8  bge cr6, 0x82c10e1c
	if !ctx.cr[6].lt {
	pc = 0x82C10E1C; continue 'dispatch;
	}
	// 82C10E48: 81690088  lwz r11, 0x88(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(136 as u32) ) } as u64;
	// 82C10E4C: 7F1F4040  cmplw cr6, r31, r8
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82C10E50: 7D4B5214  add r10, r11, r10
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82C10E54: 419AFFC8  beq cr6, 0x82c10e1c
	if ctx.cr[6].eq {
	pc = 0x82C10E1C; continue 'dispatch;
	}
	// 82C10E58: 7F0A2040  cmplw cr6, r10, r4
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[4].u32, &mut ctx.xer);
	// 82C10E5C: 4198FFA0  blt cr6, 0x82c10dfc
	if ctx.cr[6].lt {
	pc = 0x82C10DFC; continue 'dispatch;
	}
	// 82C10E60: 4BFFFC01  bl 0x82c10a60
	ctx.lr = 0x82C10E64;
	sub_82C10A60(ctx, base);
	// 82C10E64: 4BFFFFC4  b 0x82c10e28
	pc = 0x82C10E28; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C10E68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C10E68 size=144
    let mut pc: u32 = 0x82C10E68;
    'dispatch: loop {
        match pc {
            0x82C10E68 => {
    //   block [0x82C10E68..0x82C10EF8)
	// 82C10E68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C10E6C: 480985A1  bl 0x82ca940c
	ctx.lr = 0x82C10E70;
	sub_82CA93D0(ctx, base);
	// 82C10E70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C10E74: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82C10E78: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82C10E7C: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 82C10E80: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82C10E84: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C10E88: 814A0000  lwz r10, 0(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C10E8C: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 82C10E90: 813F0004  lwz r9, 4(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C10E94: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C10E98: 419A000C  beq cr6, 0x82c10ea4
	if ctx.cr[6].eq {
	pc = 0x82C10EA4; continue 'dispatch;
	}
	// 82C10E9C: 7F0BF840  cmplw cr6, r11, r31
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[31].u32, &mut ctx.xer);
	// 82C10EA0: 419A0008  beq cr6, 0x82c10ea8
	if ctx.cr[6].eq {
	pc = 0x82C10EA8; continue 'dispatch;
	}
	// 82C10EA4: 0FE00016  twui r0, 0x16
	// twui: trap word unsigned immediate — TODO: implement trap semantics
	// 82C10EA8: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82C10EAC: 419A0044  beq cr6, 0x82c10ef0
	if ctx.cr[6].eq {
	pc = 0x82C10EF0; continue 'dispatch;
	}
	// 82C10EB0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C10EB4: 409A0008  bne cr6, 0x82c10ebc
	if !ctx.cr[6].eq {
	pc = 0x82C10EBC; continue 'dispatch;
	}
	// 82C10EB8: 0FE00016  twui r0, 0x16
	// twui: trap word unsigned immediate — TODO: implement trap semantics
	// 82C10EBC: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C10EC0: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82C10EC4: 409A0008  bne cr6, 0x82c10ecc
	if !ctx.cr[6].eq {
	pc = 0x82C10ECC; continue 'dispatch;
	}
	// 82C10EC8: 0FE00016  twui r0, 0x16
	// twui: trap word unsigned immediate — TODO: implement trap semantics
	// 82C10ECC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C10ED0: 83AA000C  lwz r29, 0xc(r10)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C10ED4: 4B6EF255  bl 0x82300128
	ctx.lr = 0x82C10ED8;
	sub_82300128(ctx, base);
	// 82C10ED8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82C10EDC: 809E0050  lwz r4, 0x50(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(80 as u32) ) } as u64;
	// 82C10EE0: 4BFFFD31  bl 0x82c10c10
	ctx.lr = 0x82C10EE4;
	sub_82C10C10(ctx, base);
	// 82C10EE4: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82C10EE8: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82C10EEC: 4BFFFFA4  b 0x82c10e90
	pc = 0x82C10E90; continue 'dispatch;
	// 82C10EF0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82C10EF4: 48098568  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C10EF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C10EF8 size=216
    let mut pc: u32 = 0x82C10EF8;
    'dispatch: loop {
        match pc {
            0x82C10EF8 => {
    //   block [0x82C10EF8..0x82C10FD0)
	// 82C10EF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C10EFC: 4809850D  bl 0x82ca9408
	ctx.lr = 0x82C10F00;
	sub_82CA93D0(ctx, base);
	// 82C10F00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C10F04: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C10F08: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82C10F0C: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82C10F10: 897F009C  lbz r11, 0x9c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(156 as u32) ) } as u64;
	// 82C10F14: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C10F18: 419A00B0  beq cr6, 0x82c10fc8
	if ctx.cr[6].eq {
	pc = 0x82C10FC8; continue 'dispatch;
	}
	// 82C10F1C: 80BF0098  lwz r5, 0x98(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(152 as u32) ) } as u64;
	// 82C10F20: 2F050001  cmpwi cr6, r5, 1
	ctx.cr[6].compare_i32(ctx.r[5].s32, 1, &mut ctx.xer);
	// 82C10F24: 419A0010  beq cr6, 0x82c10f34
	if ctx.cr[6].eq {
	pc = 0x82C10F34; continue 'dispatch;
	}
	// 82C10F28: 2F050003  cmpwi cr6, r5, 3
	ctx.cr[6].compare_i32(ctx.r[5].s32, 3, &mut ctx.xer);
	// 82C10F2C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82C10F30: 409A0008  bne cr6, 0x82c10f38
	if !ctx.cr[6].eq {
	pc = 0x82C10F38; continue 'dispatch;
	}
	// 82C10F34: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82C10F38: 57AB063E  clrlwi r11, r29, 0x18
	ctx.r[11].u64 = ctx.r[29].u32 as u64 & 0x000000FFu64;
	// 82C10F3C: 554A063E  clrlwi r10, r10, 0x18
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	// 82C10F40: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82C10F44: 419A0084  beq cr6, 0x82c10fc8
	if ctx.cr[6].eq {
	pc = 0x82C10FC8; continue 'dispatch;
	}
	// 82C10F48: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C10F4C: 409A0044  bne cr6, 0x82c10f90
	if !ctx.cr[6].eq {
	pc = 0x82C10F90; continue 'dispatch;
	}
	// 82C10F50: 3FC08333  lis r30, -0x7ccd
	ctx.r[30].s64 = -2093809664;
	// 82C10F54: 80DF0088  lwz r6, 0x88(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(136 as u32) ) } as u64;
	// 82C10F58: 38FF0028  addi r7, r31, 0x28
	ctx.r[7].s64 = ctx.r[31].s64 + 40;
	// 82C10F5C: 80BF008C  lwz r5, 0x8c(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(140 as u32) ) } as u64;
	// 82C10F60: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82C10F64: 807E4EEC  lwz r3, 0x4eec(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20204 as u32) ) } as u64;
	// 82C10F68: 4BFFFE31  bl 0x82c10d98
	ctx.lr = 0x82C10F6C;
	sub_82C10D98(ctx, base);
	// 82C10F6C: 546B063E  clrlwi r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82C10F70: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C10F74: 419A0054  beq cr6, 0x82c10fc8
	if ctx.cr[6].eq {
	pc = 0x82C10FC8; continue 'dispatch;
	}
	// 82C10F78: 80BF0098  lwz r5, 0x98(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(152 as u32) ) } as u64;
	// 82C10F7C: 2F050002  cmpwi cr6, r5, 2
	ctx.cr[6].compare_i32(ctx.r[5].s32, 2, &mut ctx.xer);
	// 82C10F80: 419A002C  beq cr6, 0x82c10fac
	if ctx.cr[6].eq {
	pc = 0x82C10FAC; continue 'dispatch;
	}
	// 82C10F84: 807E4EEC  lwz r3, 0x4eec(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20204 as u32) ) } as u64;
	// 82C10F88: 38C00002  li r6, 2
	ctx.r[6].s64 = 2;
	// 82C10F8C: 48000018  b 0x82c10fa4
	pc = 0x82C10FA4; continue 'dispatch;
	// 82C10F90: 2F050003  cmpwi cr6, r5, 3
	ctx.cr[6].compare_i32(ctx.r[5].s32, 3, &mut ctx.xer);
	// 82C10F94: 419A0018  beq cr6, 0x82c10fac
	if ctx.cr[6].eq {
	pc = 0x82C10FAC; continue 'dispatch;
	}
	// 82C10F98: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 82C10F9C: 38C00003  li r6, 3
	ctx.r[6].s64 = 3;
	// 82C10FA0: 806B4EEC  lwz r3, 0x4eec(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20204 as u32) ) } as u64;
	// 82C10FA4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82C10FA8: 4BFFF751  bl 0x82c106f8
	ctx.lr = 0x82C10FAC;
	sub_82C106F8(ctx, base);
	// 82C10FAC: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C10FB0: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82C10FB4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82C10FB8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C10FBC: 814B0020  lwz r10, 0x20(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82C10FC0: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C10FC4: 4E800421  bctrl
	ctx.lr = 0x82C10FC8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C10FC8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82C10FCC: 4809848C  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C10FD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C10FD0 size=468
    let mut pc: u32 = 0x82C10FD0;
    'dispatch: loop {
        match pc {
            0x82C10FD0 => {
    //   block [0x82C10FD0..0x82C111A4)
	// 82C10FD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C10FD4: 48098431  bl 0x82ca9404
	ctx.lr = 0x82C10FD8;
	sub_82CA93D0(ctx, base);
	// 82C10FD8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C10FDC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82C10FE0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C10FE4: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C10FE8: 556A07FE  clrlwi r10, r11, 0x1f
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	// 82C10FEC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C10FF0: 409A0014  bne cr6, 0x82c11004
	if !ctx.cr[6].eq {
	pc = 0x82C11004; continue 'dispatch;
	}
	// 82C10FF4: 556B06B4  rlwinm r11, r11, 0, 0x1a, 0x1a
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82C10FF8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C10FFC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82C11000: 409A0008  bne cr6, 0x82c11008
	if !ctx.cr[6].eq {
	pc = 0x82C11008; continue 'dispatch;
	}
	// 82C11004: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82C11008: 556A063E  clrlwi r10, r11, 0x18
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82C1100C: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82C11010: 995F009C  stb r10, 0x9c(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(156 as u32), ctx.r[10].u8 ) };
	// 82C11014: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C11018: 556907FE  clrlwi r9, r11, 0x1f
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	// 82C1101C: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82C11020: 419A0008  beq cr6, 0x82c11028
	if ctx.cr[6].eq {
	pc = 0x82C11028; continue 'dispatch;
	}
	// 82C11024: 3B600001  li r27, 1
	ctx.r[27].s64 = 1;
	// 82C11028: 556907BC  rlwinm r9, r11, 0, 0x1e, 0x1e
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82C1102C: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82C11030: 419A0008  beq cr6, 0x82c11038
	if ctx.cr[6].eq {
	pc = 0x82C11038; continue 'dispatch;
	}
	// 82C11034: 637B0002  ori r27, r27, 2
	ctx.r[27].u64 = ctx.r[27].u64 | 2;
	// 82C11038: 5569077A  rlwinm r9, r11, 0, 0x1d, 0x1d
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82C1103C: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82C11040: 419A0008  beq cr6, 0x82c11048
	if ctx.cr[6].eq {
	pc = 0x82C11048; continue 'dispatch;
	}
	// 82C11044: 637B0004  ori r27, r27, 4
	ctx.r[27].u64 = ctx.r[27].u64 | 4;
	// 82C11048: 554A063E  clrlwi r10, r10, 0x18
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	// 82C1104C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C11050: 419A0014  beq cr6, 0x82c11064
	if ctx.cr[6].eq {
	pc = 0x82C11064; continue 'dispatch;
	}
	// 82C11054: 556A0738  rlwinm r10, r11, 0, 0x1c, 0x1c
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82C11058: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C1105C: 419A0008  beq cr6, 0x82c11064
	if ctx.cr[6].eq {
	pc = 0x82C11064; continue 'dispatch;
	}
	// 82C11060: 637B0008  ori r27, r27, 8
	ctx.r[27].u64 = ctx.r[27].u64 | 8;
	// 82C11064: 556A06F6  rlwinm r10, r11, 0, 0x1b, 0x1b
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82C11068: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C1106C: 419A0008  beq cr6, 0x82c11074
	if ctx.cr[6].eq {
	pc = 0x82C11074; continue 'dispatch;
	}
	// 82C11070: 637B0010  ori r27, r27, 0x10
	ctx.r[27].u64 = ctx.r[27].u64 | 16;
	// 82C11074: 556B06B4  rlwinm r11, r11, 0, 0x1a, 0x1a
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82C11078: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C1107C: 419A0008  beq cr6, 0x82c11084
	if ctx.cr[6].eq {
	pc = 0x82C11084; continue 'dispatch;
	}
	// 82C11080: 637B0020  ori r27, r27, 0x20
	ctx.r[27].u64 = ctx.r[27].u64 | 32;
	// 82C11084: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C11088: 917F0084  stw r11, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[11].u32 ) };
	// 82C1108C: 815E0004  lwz r10, 4(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C11090: 915F0088  stw r10, 0x88(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(136 as u32), ctx.r[10].u32 ) };
	// 82C11094: 813E000C  lwz r9, 0xc(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C11098: 913F008C  stw r9, 0x8c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(140 as u32), ctx.r[9].u32 ) };
	// 82C1109C: 4BFF1B95  bl 0x82c02c30
	ctx.lr = 0x82C110A0;
	sub_82C02C30(ctx, base);
	// 82C110A0: 809F0088  lwz r4, 0x88(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(136 as u32) ) } as u64;
	// 82C110A4: 8103000C  lwz r8, 0xc(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C110A8: 3B9F0028  addi r28, r31, 0x28
	ctx.r[28].s64 = ctx.r[31].s64 + 40;
	// 82C110AC: 5487083C  slwi r7, r4, 1
	ctx.r[7].u32 = ctx.r[4].u32.wrapping_shl(1);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82C110B0: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82C110B4: 7CC83B96  divwu r6, r8, r7
	ctx.r[6].u32 = ctx.r[8].u32 / ctx.r[7].u32;
	// 82C110B8: 7CA639D6  mullw r5, r6, r7
	ctx.r[5].s64 = (ctx.r[6].s32 as i64) * (ctx.r[7].s32 as i64);
	// 82C110BC: 7D654050  subf r11, r5, r8
	ctx.r[11].s64 = ctx.r[8].s64 - ctx.r[5].s64;
	// 82C110C0: 7D4B4050  subf r10, r11, r8
	ctx.r[10].s64 = ctx.r[8].s64 - ctx.r[11].s64;
	// 82C110C4: 915F007C  stw r10, 0x7c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(124 as u32), ctx.r[10].u32 ) };
	// 82C110C8: 4BFFF221  bl 0x82c102e8
	ctx.lr = 0x82C110CC;
	sub_82C102E8(ctx, base);
	// 82C110CC: 893F009C  lbz r9, 0x9c(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(156 as u32) ) } as u64;
	// 82C110D0: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82C110D4: 419A0028  beq cr6, 0x82c110fc
	if ctx.cr[6].eq {
	pc = 0x82C110FC; continue 'dispatch;
	}
	// 82C110D8: 576B0738  rlwinm r11, r27, 0, 0x1c, 0x1c
	ctx.r[11].u64 = ctx.r[27].u32 as u64 & 0xFFFFFFFFu64;
	// 82C110DC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C110E0: 419A001C  beq cr6, 0x82c110fc
	if ctx.cr[6].eq {
	pc = 0x82C110FC; continue 'dispatch;
	}
	// 82C110E4: 80BF0098  lwz r5, 0x98(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(152 as u32) ) } as u64;
	// 82C110E8: 2F050001  cmpwi cr6, r5, 1
	ctx.cr[6].compare_i32(ctx.r[5].s32, 1, &mut ctx.xer);
	// 82C110EC: 419A006C  beq cr6, 0x82c11158
	if ctx.cr[6].eq {
	pc = 0x82C11158; continue 'dispatch;
	}
	// 82C110F0: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 82C110F4: 806B4EEC  lwz r3, 0x4eec(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20204 as u32) ) } as u64;
	// 82C110F8: 48000054  b 0x82c1114c
	pc = 0x82C1114C; continue 'dispatch;
	// 82C110FC: 3FA08333  lis r29, -0x7ccd
	ctx.r[29].s64 = -2093809664;
	// 82C11100: 80DE0004  lwz r6, 4(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C11104: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 82C11108: 80BE000C  lwz r5, 0xc(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C1110C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82C11110: 807D4EEC  lwz r3, 0x4eec(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(20204 as u32) ) } as u64;
	// 82C11114: 4BFFFC85  bl 0x82c10d98
	ctx.lr = 0x82C11118;
	sub_82C10D98(ctx, base);
	// 82C11118: 546B063E  clrlwi r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82C1111C: 80BF0098  lwz r5, 0x98(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(152 as u32) ) } as u64;
	// 82C11120: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C11124: 419A0018  beq cr6, 0x82c1113c
	if ctx.cr[6].eq {
	pc = 0x82C1113C; continue 'dispatch;
	}
	// 82C11128: 2F050002  cmpwi cr6, r5, 2
	ctx.cr[6].compare_i32(ctx.r[5].s32, 2, &mut ctx.xer);
	// 82C1112C: 419A002C  beq cr6, 0x82c11158
	if ctx.cr[6].eq {
	pc = 0x82C11158; continue 'dispatch;
	}
	// 82C11130: 807D4EEC  lwz r3, 0x4eec(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(20204 as u32) ) } as u64;
	// 82C11134: 38C00002  li r6, 2
	ctx.r[6].s64 = 2;
	// 82C11138: 48000018  b 0x82c11150
	pc = 0x82C11150; continue 'dispatch;
	// 82C1113C: 637B0008  ori r27, r27, 8
	ctx.r[27].u64 = ctx.r[27].u64 | 8;
	// 82C11140: 2F050001  cmpwi cr6, r5, 1
	ctx.cr[6].compare_i32(ctx.r[5].s32, 1, &mut ctx.xer);
	// 82C11144: 419A0014  beq cr6, 0x82c11158
	if ctx.cr[6].eq {
	pc = 0x82C11158; continue 'dispatch;
	}
	// 82C11148: 807D4EEC  lwz r3, 0x4eec(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(20204 as u32) ) } as u64;
	// 82C1114C: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82C11150: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82C11154: 4BFFF5A5  bl 0x82c106f8
	ctx.lr = 0x82C11158;
	sub_82C106F8(ctx, base);
	// 82C11158: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 82C1115C: 811E0014  lwz r8, 0x14(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82C11160: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C11164: 80FE0010  lwz r7, 0x10(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C11168: 80BE0004  lwz r5, 4(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C1116C: 809E0000  lwz r4, 0(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C11170: 4BFF4DD9  bl 0x82c05f48
	ctx.lr = 0x82C11174;
	sub_82C05F48(ctx, base);
	// 82C11174: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82C11178: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 82C1117C: 48000B4D  bl 0x82c11cc8
	ctx.lr = 0x82C11180;
	sub_82C11CC8(ctx, base);
	// 82C11180: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82C11184: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C11188: 419A0014  beq cr6, 0x82c1119c
	if ctx.cr[6].eq {
	pc = 0x82C1119C; continue 'dispatch;
	}
	// 82C1118C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C11190: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C11194: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C11198: 4E800421  bctrl
	ctx.lr = 0x82C1119C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C1119C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82C111A0: 480982B4  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C111A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C111A8 size=152
    let mut pc: u32 = 0x82C111A8;
    'dispatch: loop {
        match pc {
            0x82C111A8 => {
    //   block [0x82C111A8..0x82C11240)
	// 82C111A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C111AC: 4809825D  bl 0x82ca9408
	ctx.lr = 0x82C111B0;
	sub_82CA93D0(ctx, base);
	// 82C111B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C111B4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C111B8: 386000B0  li r3, 0xb0
	ctx.r[3].s64 = 176;
	// 82C111BC: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82C111C0: 4B60E099  bl 0x8221f258
	ctx.lr = 0x82C111C4;
	sub_8221F258(ctx, base);
	// 82C111C4: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82C111C8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C111CC: 419A0010  beq cr6, 0x82c111dc
	if ctx.cr[6].eq {
	pc = 0x82C111DC; continue 'dispatch;
	}
	// 82C111D0: 4BFFF709  bl 0x82c108d8
	ctx.lr = 0x82C111D4;
	sub_82C108D8(ctx, base);
	// 82C111D4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82C111D8: 48000008  b 0x82c111e0
	pc = 0x82C111E0; continue 'dispatch;
	// 82C111DC: 7F9EE378  mr r30, r28
	ctx.r[30].u64 = ctx.r[28].u64;
	// 82C111E0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82C111E4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82C111E8: 4BFFFDE9  bl 0x82c10fd0
	ctx.lr = 0x82C111EC;
	sub_82C10FD0(ctx, base);
	// 82C111EC: 37BE0004  addic. r29, r30, 4
	ctx.xer.ca = (ctx.r[30].u32 > (!(4 as u32)));
	ctx.r[29].s64 = ctx.r[30].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82C111F0: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82C111F4: 939F0004  stw r28, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 82C111F8: 41820018  beq 0x82c11210
	if ctx.cr[0].eq {
	pc = 0x82C11210; continue 'dispatch;
	}
	// 82C111FC: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C11200: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82C11204: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C11208: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C1120C: 4E800421  bctrl
	ctx.lr = 0x82C11210;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C11210: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C11214: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C11218: 419A0014  beq cr6, 0x82c1122c
	if ctx.cr[6].eq {
	pc = 0x82C1122C; continue 'dispatch;
	}
	// 82C1121C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C11220: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C11224: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C11228: 4E800421  bctrl
	ctx.lr = 0x82C1122C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C1122C: 93BF0004  stw r29, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82C11230: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C11234: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82C11238: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82C1123C: 4809821C  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C11240(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C11240 size=144
    let mut pc: u32 = 0x82C11240;
    'dispatch: loop {
        match pc {
            0x82C11240 => {
    //   block [0x82C11240..0x82C112D0)
	// 82C11240: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C11244: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C11248: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C1124C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C11250: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C11254: 4BFF48D5  bl 0x82c05b28
	ctx.lr = 0x82C11258;
	sub_82C05B28(ctx, base);
	// 82C11258: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C1125C: 4BFFEF65  bl 0x82c101c0
	ctx.lr = 0x82C11260;
	sub_82C101C0(ctx, base);
	// 82C11260: 4803E609  bl 0x82c4f868
	ctx.lr = 0x82C11264;
	sub_82C4F868(ctx, base);
	// 82C11264: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C11268: 419A001C  beq cr6, 0x82c11284
	if ctx.cr[6].eq {
	pc = 0x82C11284; continue 'dispatch;
	}
	// 82C1126C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82C11270: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C11274: 4BFFE385  bl 0x82c0f5f8
	ctx.lr = 0x82C11278;
	sub_82C0F5F8(ctx, base);
	// 82C11278: 4803E5F1  bl 0x82c4f868
	ctx.lr = 0x82C1127C;
	sub_82C4F868(ctx, base);
	// 82C1127C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C11280: 409AFFEC  bne cr6, 0x82c1126c
	if !ctx.cr[6].eq {
	pc = 0x82C1126C; continue 'dispatch;
	}
	// 82C11284: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C11288: 4BFFE751  bl 0x82c0f9d8
	ctx.lr = 0x82C1128C;
	sub_82C0F9D8(ctx, base);
	// 82C1128C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82C11290: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C11294: 4BFFFBD5  bl 0x82c10e68
	ctx.lr = 0x82C11298;
	sub_82C10E68(ctx, base);
	// 82C11298: 389F000C  addi r4, r31, 0xc
	ctx.r[4].s64 = ctx.r[31].s64 + 12;
	// 82C1129C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C112A0: 4BFFFBC9  bl 0x82c10e68
	ctx.lr = 0x82C112A4;
	sub_82C10E68(ctx, base);
	// 82C112A4: 389F0018  addi r4, r31, 0x18
	ctx.r[4].s64 = ctx.r[31].s64 + 24;
	// 82C112A8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C112AC: 4BFFFBBD  bl 0x82c10e68
	ctx.lr = 0x82C112B0;
	sub_82C10E68(ctx, base);
	// 82C112B0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C112B4: 4BFFE7AD  bl 0x82c0fa60
	ctx.lr = 0x82C112B8;
	sub_82C0FA60(ctx, base);
	// 82C112B8: 4BFF48E9  bl 0x82c05ba0
	ctx.lr = 0x82C112BC;
	sub_82C05BA0(ctx, base);
	// 82C112BC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82C112C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C112C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C112C8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C112CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C112D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C112D0 size=12
    let mut pc: u32 = 0x82C112D0;
    'dispatch: loop {
        match pc {
            0x82C112D0 => {
    //   block [0x82C112D0..0x82C112DC)
	// 82C112D0: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 82C112D4: 806B4EEC  lwz r3, 0x4eec(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20204 as u32) ) } as u64;
	// 82C112D8: 4BFFFF68  b 0x82c11240
	sub_82C11240(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C112E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C112E0 size=136
    let mut pc: u32 = 0x82C112E0;
    'dispatch: loop {
        match pc {
            0x82C112E0 => {
    //   block [0x82C112E0..0x82C11368)
	// 82C112E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C112E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C112E8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C112EC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C112F0: 4BFF1941  bl 0x82c02c30
	ctx.lr = 0x82C112F4;
	sub_82C02C30(ctx, base);
	// 82C112F4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C112F8: 4803E2B1  bl 0x82c4f5a8
	ctx.lr = 0x82C112FC;
	sub_82C4F5A8(ctx, base);
	// 82C112FC: 38600054  li r3, 0x54
	ctx.r[3].s64 = 84;
	// 82C11300: 4B60DF59  bl 0x8221f258
	ctx.lr = 0x82C11304;
	sub_8221F258(ctx, base);
	// 82C11304: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C11308: 419A0020  beq cr6, 0x82c11328
	if ctx.cr[6].eq {
	pc = 0x82C11328; continue 'dispatch;
	}
	// 82C1130C: 80DF0018  lwz r6, 0x18(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82C11310: 80BF0014  lwz r5, 0x14(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82C11314: 809F0004  lwz r4, 4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C11318: 4BFFF679  bl 0x82c10990
	ctx.lr = 0x82C1131C;
	sub_82C10990(ctx, base);
	// 82C1131C: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 82C11320: 906B4EEC  stw r3, 0x4eec(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(20204 as u32), ctx.r[3].u32 ) };
	// 82C11324: 48000010  b 0x82c11334
	pc = 0x82C11334; continue 'dispatch;
	// 82C11328: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 82C1132C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82C11330: 916A4EEC  stw r11, 0x4eec(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(20204 as u32), ctx.r[11].u32 ) };
	// 82C11334: 3D40832D  lis r10, -0x7cd3
	ctx.r[10].s64 = -2094202880;
	// 82C11338: 3D6082C1  lis r11, -0x7d3f
	ctx.r[11].s64 = -2101280768;
	// 82C1133C: 392A61A0  addi r9, r10, 0x61a0
	ctx.r[9].s64 = ctx.r[10].s64 + 24992;
	// 82C11340: 396B12D0  addi r11, r11, 0x12d0
	ctx.r[11].s64 = ctx.r[11].s64 + 4816;
	// 82C11344: 7D234B78  mr r3, r9
	ctx.r[3].u64 = ctx.r[9].u64;
	// 82C11348: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82C1134C: 91690004  stw r11, 4(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82C11350: 4BFFD231  bl 0x82c0e580
	ctx.lr = 0x82C11354;
	sub_82C0E580(ctx, base);
	// 82C11354: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82C11358: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C1135C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C11360: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C11364: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C11368(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C11368 size=60
    let mut pc: u32 = 0x82C11368;
    'dispatch: loop {
        match pc {
            0x82C11368 => {
    //   block [0x82C11368..0x82C113A4)
	// 82C11368: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C1136C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C11370: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C11374: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C11378: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 82C1137C: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82C11380: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C11384: 808B4EEC  lwz r4, 0x4eec(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20204 as u32) ) } as u64;
	// 82C11388: 4BFFFE21  bl 0x82c111a8
	ctx.lr = 0x82C1138C;
	sub_82C111A8(ctx, base);
	// 82C1138C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C11390: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82C11394: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C11398: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C1139C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C113A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C113A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C113A8 size=16
    let mut pc: u32 = 0x82C113A8;
    'dispatch: loop {
        match pc {
            0x82C113A8 => {
    //   block [0x82C113A8..0x82C113B8)
	// 82C113A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82C113AC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82C113B0: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82C113B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C113B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C113B8 size=16
    let mut pc: u32 = 0x82C113B8;
    'dispatch: loop {
        match pc {
            0x82C113B8 => {
    //   block [0x82C113B8..0x82C113C8)
	// 82C113B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82C113BC: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82C113C0: 91670000  stw r11, 0(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82C113C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C113C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C113C8 size=16
    let mut pc: u32 = 0x82C113C8;
    'dispatch: loop {
        match pc {
            0x82C113C8 => {
    //   block [0x82C113C8..0x82C113D8)
	// 82C113C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82C113CC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82C113D0: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82C113D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C113D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C113D8 size=40
    let mut pc: u32 = 0x82C113D8;
    'dispatch: loop {
        match pc {
            0x82C113D8 => {
    //   block [0x82C113D8..0x82C11400)
	// 82C113D8: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82C113DC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82C113E0: 392AAAD4  addi r9, r10, -0x552c
	ctx.r[9].s64 = ctx.r[10].s64 + -21804;
	// 82C113E4: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82C113E8: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82C113EC: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82C113F0: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82C113F4: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82C113F8: 91630014  stw r11, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82C113FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C11400(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C11400 size=96
    let mut pc: u32 = 0x82C11400;
    'dispatch: loop {
        match pc {
            0x82C11400 => {
    //   block [0x82C11400..0x82C11460)
	// 82C11400: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C11404: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C11408: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82C1140C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C11410: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C11414: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C11418: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C1141C: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 82C11420: 394BAAE4  addi r10, r11, -0x551c
	ctx.r[10].s64 = ctx.r[11].s64 + -21788;
	// 82C11424: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82C11428: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82C1142C: 4803C775  bl 0x82c4dba0
	ctx.lr = 0x82C11430;
	sub_82C4DBA0(ctx, base);
	// 82C11430: 57C907FE  clrlwi r9, r30, 0x1f
	ctx.r[9].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82C11434: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C11438: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82C1143C: 419A000C  beq cr6, 0x82c11448
	if ctx.cr[6].eq {
	pc = 0x82C11448; continue 'dispatch;
	}
	// 82C11440: 4BC34371  bl 0x828457b0
	ctx.lr = 0x82C11444;
	sub_828457B0(ctx, base);
	// 82C11444: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C11448: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C1144C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C11450: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C11454: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82C11458: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C1145C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C11460(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C11460 size=88
    let mut pc: u32 = 0x82C11460;
    'dispatch: loop {
        match pc {
            0x82C11460 => {
    //   block [0x82C11460..0x82C114B8)
	// 82C11460: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C11464: 48097FA9  bl 0x82ca940c
	ctx.lr = 0x82C11468;
	sub_82CA93D0(ctx, base);
	// 82C11468: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C1146C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82C11470: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 82C11474: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82C11478: 4B60DDE1  bl 0x8221f258
	ctx.lr = 0x82C1147C;
	sub_8221F258(ctx, base);
	// 82C1147C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C11480: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82C11484: 419A0018  beq cr6, 0x82c1149c
	if ctx.cr[6].eq {
	pc = 0x82C1149C; continue 'dispatch;
	}
	// 82C11488: 4803C609  bl 0x82c4da90
	ctx.lr = 0x82C1148C;
	sub_82C4DA90(ctx, base);
	// 82C1148C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82C11490: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82C11494: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82C11498: 48000008  b 0x82c114a0
	pc = 0x82C114A0; continue 'dispatch;
	// 82C1149C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82C114A0: 93A50014  stw r29, 0x14(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(20 as u32), ctx.r[29].u32 ) };
	// 82C114A4: 389E0008  addi r4, r30, 8
	ctx.r[4].s64 = ctx.r[30].s64 + 8;
	// 82C114A8: 38640004  addi r3, r4, 4
	ctx.r[3].s64 = ctx.r[4].s64 + 4;
	// 82C114AC: 4803BE15  bl 0x82c4d2c0
	ctx.lr = 0x82C114B0;
	sub_82C4D2C0(ctx, base);
	// 82C114B0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C114B4: 48097FA8  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C114B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C114B8 size=288
    let mut pc: u32 = 0x82C114B8;
    'dispatch: loop {
        match pc {
            0x82C114B8 => {
    //   block [0x82C114B8..0x82C115D8)
	// 82C114B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C114BC: 48097F4D  bl 0x82ca9408
	ctx.lr = 0x82C114C0;
	sub_82CA93D0(ctx, base);
	// 82C114C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C114C4: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82C114C8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C114CC: 4B5DD66D  bl 0x821eeb38
	ctx.lr = 0x82C114D0;
	sub_821EEB38(ctx, base);
	// 82C114D0: 3FA08333  lis r29, -0x7ccd
	ctx.r[29].s64 = -2093809664;
	// 82C114D4: 817D4EF0  lwz r11, 0x4ef0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(20208 as u32) ) } as u64;
	// 82C114D8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C114DC: 419A00B8  beq cr6, 0x82c11594
	if ctx.cr[6].eq {
	pc = 0x82C11594; continue 'dispatch;
	}
	// 82C114E0: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 82C114E4: 419A00B0  beq cr6, 0x82c11594
	if ctx.cr[6].eq {
	pc = 0x82C11594; continue 'dispatch;
	}
	// 82C114E8: 386B000C  addi r3, r11, 0xc
	ctx.r[3].s64 = ctx.r[11].s64 + 12;
	// 82C114EC: 4803BACD  bl 0x82c4cfb8
	ctx.lr = 0x82C114F0;
	sub_82C4CFB8(ctx, base);
	// 82C114F0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C114F4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82C114F8: 419A009C  beq cr6, 0x82c11594
	if ctx.cr[6].eq {
	pc = 0x82C11594; continue 'dispatch;
	}
	// 82C114FC: 83DF0014  lwz r30, 0x14(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82C11500: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82C11504: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C11508: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C1150C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C11510: 4E800421  bctrl
	ctx.lr = 0x82C11514;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C11514: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C11518: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82C1151C: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82C11520: 409A000C  bne cr6, 0x82c1152c
	if !ctx.cr[6].eq {
	pc = 0x82C1152C; continue 'dispatch;
	}
	// 82C11524: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82C11528: 48000040  b 0x82c11568
	pc = 0x82C11568; continue 'dispatch;
	// 82C1152C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C11530: 419A0034  beq cr6, 0x82c11564
	if ctx.cr[6].eq {
	pc = 0x82C11564; continue 'dispatch;
	}
	// 82C11534: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C11538: 419A002C  beq cr6, 0x82c11564
	if ctx.cr[6].eq {
	pc = 0x82C11564; continue 'dispatch;
	}
	// 82C1153C: 812B0004  lwz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C11540: 810A0004  lwz r8, 4(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C11544: 7F094040  cmplw cr6, r9, r8
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82C11548: 409A001C  bne cr6, 0x82c11564
	if !ctx.cr[6].eq {
	pc = 0x82C11564; continue 'dispatch;
	}
	// 82C1154C: 808A0000  lwz r4, 0(r10)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C11550: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C11554: 4B65C2A5  bl 0x8226d7f8
	ctx.lr = 0x82C11558;
	sub_8226D7F8(ctx, base);
	// 82C11558: 7C6B0034  cntlzw r11, r3
	ctx.r[11].u64 = if ctx.r[3].u32 == 0 { 32 } else { ctx.r[3].u32.leading_zeros() as u64 };
	// 82C1155C: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82C11560: 48000008  b 0x82c11568
	pc = 0x82C11568; continue 'dispatch;
	// 82C11564: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82C11568: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82C1156C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C11570: 409A0038  bne cr6, 0x82c115a8
	if !ctx.cr[6].eq {
	pc = 0x82C115A8; continue 'dispatch;
	}
	// 82C11574: 817D4EF0  lwz r11, 0x4ef0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(20208 as u32) ) } as u64;
	// 82C11578: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82C1157C: 388B0008  addi r4, r11, 8
	ctx.r[4].s64 = ctx.r[11].s64 + 8;
	// 82C11580: 38640004  addi r3, r4, 4
	ctx.r[3].s64 = ctx.r[4].s64 + 4;
	// 82C11584: 4803BFC5  bl 0x82c4d548
	ctx.lr = 0x82C11588;
	sub_82C4D548(ctx, base);
	// 82C11588: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C1158C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82C11590: 409AFF6C  bne cr6, 0x82c114fc
	if !ctx.cr[6].eq {
	pc = 0x82C114FC; continue 'dispatch;
	}
	// 82C11594: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C11598: 4B603841  bl 0x82214dd8
	ctx.lr = 0x82C1159C;
	sub_82214DD8(ctx, base);
	// 82C1159C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82C115A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82C115A4: 48097EB4  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
	// 82C115A8: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C115AC: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82C115B0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82C115B4: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C115B8: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C115BC: 4E800421  bctrl
	ctx.lr = 0x82C115C0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C115C0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C115C4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C115C8: 4B603811  bl 0x82214dd8
	ctx.lr = 0x82C115CC;
	sub_82214DD8(ctx, base);
	// 82C115CC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C115D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82C115D4: 48097E84  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C115D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C115D8 size=8
    let mut pc: u32 = 0x82C115D8;
    'dispatch: loop {
        match pc {
            0x82C115D8 => {
    //   block [0x82C115D8..0x82C115E0)
	// 82C115D8: 80640014  lwz r3, 0x14(r4)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(20 as u32) ) } as u64;
	// 82C115DC: 4B8DD7B4  b 0x824eed90
	sub_824EED90(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C115E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C115E0 size=92
    let mut pc: u32 = 0x82C115E0;
    'dispatch: loop {
        match pc {
            0x82C115E0 => {
    //   block [0x82C115E0..0x82C1163C)
	// 82C115E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C115E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C115E8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C115EC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C115F0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C115F4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C115F8: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82C115FC: 392BAB0C  addi r9, r11, -0x54f4
	ctx.r[9].s64 = ctx.r[11].s64 + -21748;
	// 82C11600: 390AAAE4  addi r8, r10, -0x551c
	ctx.r[8].s64 = ctx.r[10].s64 + -21788;
	// 82C11604: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82C11608: 397F0008  addi r11, r31, 8
	ctx.r[11].s64 = ctx.r[31].s64 + 8;
	// 82C1160C: 911F0008  stw r8, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[8].u32 ) };
	// 82C11610: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 82C11614: 4803C495  bl 0x82c4daa8
	ctx.lr = 0x82C11618;
	sub_82C4DAA8(ctx, base);
	// 82C11618: 3CE08201  lis r7, -0x7dff
	ctx.r[7].s64 = -2113863680;
	// 82C1161C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C11620: 38C7AB0C  addi r6, r7, -0x54f4
	ctx.r[6].s64 = ctx.r[7].s64 + -21748;
	// 82C11624: 90DF0000  stw r6, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[6].u32 ) };
	// 82C11628: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82C1162C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C11630: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C11634: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C11638: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C11640(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C11640 size=112
    let mut pc: u32 = 0x82C11640;
    'dispatch: loop {
        match pc {
            0x82C11640 => {
    //   block [0x82C11640..0x82C116B0)
	// 82C11640: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C11644: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C11648: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82C1164C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C11650: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C11654: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C11658: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C1165C: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82C11660: 392BAB0C  addi r9, r11, -0x54f4
	ctx.r[9].s64 = ctx.r[11].s64 + -21748;
	// 82C11664: 390AAAE4  addi r8, r10, -0x551c
	ctx.r[8].s64 = ctx.r[10].s64 + -21788;
	// 82C11668: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82C1166C: 397F0008  addi r11, r31, 8
	ctx.r[11].s64 = ctx.r[31].s64 + 8;
	// 82C11670: 911F0008  stw r8, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[8].u32 ) };
	// 82C11674: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82C11678: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 82C1167C: 4803C525  bl 0x82c4dba0
	ctx.lr = 0x82C11680;
	sub_82C4DBA0(ctx, base);
	// 82C11680: 57C707FE  clrlwi r7, r30, 0x1f
	ctx.r[7].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82C11684: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C11688: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 82C1168C: 419A000C  beq cr6, 0x82c11698
	if ctx.cr[6].eq {
	pc = 0x82C11698; continue 'dispatch;
	}
	// 82C11690: 4BC34121  bl 0x828457b0
	ctx.lr = 0x82C11694;
	sub_828457B0(ctx, base);
	// 82C11694: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C11698: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C1169C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C116A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C116A4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82C116A8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C116AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C116B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C116B0 size=184
    let mut pc: u32 = 0x82C116B0;
    'dispatch: loop {
        match pc {
            0x82C116B0 => {
    //   block [0x82C116B0..0x82C11768)
	// 82C116B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C116B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C116B8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82C116BC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C116C0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C116C4: 3FE08333  lis r31, -0x7ccd
	ctx.r[31].s64 = -2093809664;
	// 82C116C8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82C116CC: 817F4EF0  lwz r11, 0x4ef0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20208 as u32) ) } as u64;
	// 82C116D0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C116D4: 409A0028  bne cr6, 0x82c116fc
	if !ctx.cr[6].eq {
	pc = 0x82C116FC; continue 'dispatch;
	}
	// 82C116D8: 38600014  li r3, 0x14
	ctx.r[3].s64 = 20;
	// 82C116DC: 4B60DB7D  bl 0x8221f258
	ctx.lr = 0x82C116E0;
	sub_8221F258(ctx, base);
	// 82C116E0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C116E4: 419A0010  beq cr6, 0x82c116f4
	if ctx.cr[6].eq {
	pc = 0x82C116F4; continue 'dispatch;
	}
	// 82C116E8: 4BFFFEF9  bl 0x82c115e0
	ctx.lr = 0x82C116EC;
	sub_82C115E0(ctx, base);
	// 82C116EC: 907F4EF0  stw r3, 0x4ef0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20208 as u32), ctx.r[3].u32 ) };
	// 82C116F0: 4800000C  b 0x82c116fc
	pc = 0x82C116FC; continue 'dispatch;
	// 82C116F4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82C116F8: 917F4EF0  stw r11, 0x4ef0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20208 as u32), ctx.r[11].u32 ) };
	// 82C116FC: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82C11700: 409A000C  bne cr6, 0x82c1170c
	if !ctx.cr[6].eq {
	pc = 0x82C1170C; continue 'dispatch;
	}
	// 82C11704: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82C11708: 48000048  b 0x82c11750
	pc = 0x82C11750; continue 'dispatch;
	// 82C1170C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C11710: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82C11714: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C11718: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C1171C: 4E800421  bctrl
	ctx.lr = 0x82C11720;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C11720: 817F4EF0  lwz r11, 0x4ef0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20208 as u32) ) } as u64;
	// 82C11724: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 82C11728: 388B0008  addi r4, r11, 8
	ctx.r[4].s64 = ctx.r[11].s64 + 8;
	// 82C1172C: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82C11730: 38640004  addi r3, r4, 4
	ctx.r[3].s64 = ctx.r[4].s64 + 4;
	// 82C11734: 4803BAA5  bl 0x82c4d1d8
	ctx.lr = 0x82C11738;
	sub_82C4D1D8(ctx, base);
	// 82C11738: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C1173C: 409AFFC8  bne cr6, 0x82c11704
	if !ctx.cr[6].eq {
	pc = 0x82C11704; continue 'dispatch;
	}
	// 82C11740: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82C11744: 807F4EF0  lwz r3, 0x4ef0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20208 as u32) ) } as u64;
	// 82C11748: 4BFFFD19  bl 0x82c11460
	ctx.lr = 0x82C1174C;
	sub_82C11460(ctx, base);
	// 82C1174C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82C11750: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C11754: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C11758: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C1175C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82C11760: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C11764: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C11768(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C11768 size=96
    let mut pc: u32 = 0x82C11768;
    'dispatch: loop {
        match pc {
            0x82C11768 => {
    //   block [0x82C11768..0x82C117C8)
	// 82C11768: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C1176C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C11770: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C11774: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C11778: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C1177C: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82C11780: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C11784: 419A0020  beq cr6, 0x82c117a4
	if ctx.cr[6].eq {
	pc = 0x82C117A4; continue 'dispatch;
	}
	// 82C11788: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C1178C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82C11790: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C11794: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C11798: 4E800421  bctrl
	ctx.lr = 0x82C1179C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C1179C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82C117A0: 913F0014  stw r9, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[9].u32 ) };
	// 82C117A4: 387F0018  addi r3, r31, 0x18
	ctx.r[3].s64 = ctx.r[31].s64 + 24;
	// 82C117A8: 4B603631  bl 0x82214dd8
	ctx.lr = 0x82C117AC;
	sub_82214DD8(ctx, base);
	// 82C117AC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C117B0: 4BF856C9  bl 0x82b96e78
	ctx.lr = 0x82C117B4;
	sub_82B96E78(ctx, base);
	// 82C117B4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82C117B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C117BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C117C0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C117C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C117C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C117C8 size=8
    let mut pc: u32 = 0x82C117C8;
    'dispatch: loop {
        match pc {
            0x82C117C8 => {
    //   block [0x82C117C8..0x82C117D0)
	// 82C117C8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C117CC: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C117D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C117D0 size=20
    let mut pc: u32 = 0x82C117D0;
    'dispatch: loop {
        match pc {
            0x82C117D0 => {
    //   block [0x82C117D0..0x82C117E4)
	// 82C117D0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C117D4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82C117D8: 814B0024  lwz r10, 0x24(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 82C117DC: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C117E0: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C117E4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C117E4 size=4
    let mut pc: u32 = 0x82C117E4;
    'dispatch: loop {
        match pc {
            0x82C117E4 => {
    //   block [0x82C117E4..0x82C117E8)
	// 82C117E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C117E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82C117E8 size=204
    let mut pc: u32 = 0x82C117E8;
    'dispatch: loop {
        match pc {
            0x82C117E8 => {
    //   block [0x82C117E8..0x82C118B4)
	// 82C117E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C117EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C117F0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82C117F4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C117F8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C117FC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82C11800: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C11804: 389E0004  addi r4, r30, 4
	ctx.r[4].s64 = ctx.r[30].s64 + 4;
	// 82C11808: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 82C1180C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C11810: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82C11814: 4B5DEA2D  bl 0x821f0240
	ctx.lr = 0x82C11818;
	sub_821F0240(ctx, base);
	// 82C11818: 815E0008  lwz r10, 8(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C1181C: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82C11820: 813E000C  lwz r9, 0xc(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C11824: 913F000C  stw r9, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 82C11828: 811E0010  lwz r8, 0x10(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C1182C: 911F0010  stw r8, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[8].u32 ) };
	// 82C11830: 80FE0014  lwz r7, 0x14(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82C11834: 90FF0014  stw r7, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[7].u32 ) };
	// 82C11838: 80DE0018  lwz r6, 0x18(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 82C1183C: 90DF0018  stw r6, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[6].u32 ) };
	// 82C11840: 80BE001C  lwz r5, 0x1c(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 82C11844: 90BF001C  stw r5, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[5].u32 ) };
	// 82C11848: 809E0020  lwz r4, 0x20(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 82C1184C: 909F0020  stw r4, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[4].u32 ) };
	// 82C11850: C01E0024  lfs f0, 0x24(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C11854: D01F0024  stfs f0, 0x24(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), tmp.u32 ) };
	// 82C11858: C1BE0028  lfs f13, 0x28(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82C1185C: D1BF0028  stfs f13, 0x28(r31)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), tmp.u32 ) };
	// 82C11860: 807E002C  lwz r3, 0x2c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(44 as u32) ) } as u64;
	// 82C11864: 907F002C  stw r3, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[3].u32 ) };
	// 82C11868: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C1186C: C19E0030  lfs f12, 0x30(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82C11870: D19F0030  stfs f12, 0x30(r31)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), tmp.u32 ) };
	// 82C11874: C17E0034  lfs f11, 0x34(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(52 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82C11878: D17F0034  stfs f11, 0x34(r31)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), tmp.u32 ) };
	// 82C1187C: 817E0038  lwz r11, 0x38(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(56 as u32) ) } as u64;
	// 82C11880: 917F0038  stw r11, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 82C11884: 815E003C  lwz r10, 0x3c(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(60 as u32) ) } as u64;
	// 82C11888: 915F003C  stw r10, 0x3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[10].u32 ) };
	// 82C1188C: 893E0040  lbz r9, 0x40(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(64 as u32) ) } as u64;
	// 82C11890: 993F0040  stb r9, 0x40(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), ctx.r[9].u8 ) };
	// 82C11894: 891E0041  lbz r8, 0x41(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(65 as u32) ) } as u64;
	// 82C11898: 991F0041  stb r8, 0x41(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(65 as u32), ctx.r[8].u8 ) };
	// 82C1189C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C118A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C118A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C118A8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82C118AC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C118B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C118B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C118B8 size=8
    let mut pc: u32 = 0x82C118B8;
    'dispatch: loop {
        match pc {
            0x82C118B8 => {
    //   block [0x82C118B8..0x82C118C0)
	// 82C118B8: 38630060  addi r3, r3, 0x60
	ctx.r[3].s64 = ctx.r[3].s64 + 96;
	// 82C118BC: 4B663B7C  b 0x82275438
	sub_82275438(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C118C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C118C0 size=52
    let mut pc: u32 = 0x82C118C0;
    'dispatch: loop {
        match pc {
            0x82C118C0 => {
    //   block [0x82C118C0..0x82C118F4)
	// 82C118C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C118C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C118C8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C118CC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C118D0: 38840060  addi r4, r4, 0x60
	ctx.r[4].s64 = ctx.r[4].s64 + 96;
	// 82C118D4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C118D8: 4B5DE969  bl 0x821f0240
	ctx.lr = 0x82C118DC;
	sub_821F0240(ctx, base);
	// 82C118DC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C118E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82C118E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C118E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C118EC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C118F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C118F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C118F8 size=8
    let mut pc: u32 = 0x82C118F8;
    'dispatch: loop {
        match pc {
            0x82C118F8 => {
    //   block [0x82C118F8..0x82C11900)
	// 82C118F8: 3863003C  addi r3, r3, 0x3c
	ctx.r[3].s64 = ctx.r[3].s64 + 60;
	// 82C118FC: 4B663B3C  b 0x82275438
	sub_82275438(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C11900(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C11900 size=52
    let mut pc: u32 = 0x82C11900;
    'dispatch: loop {
        match pc {
            0x82C11900 => {
    //   block [0x82C11900..0x82C11934)
	// 82C11900: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C11904: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C11908: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C1190C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C11910: 3884003C  addi r4, r4, 0x3c
	ctx.r[4].s64 = ctx.r[4].s64 + 60;
	// 82C11914: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C11918: 4B5DE929  bl 0x821f0240
	ctx.lr = 0x82C1191C;
	sub_821F0240(ctx, base);
	// 82C1191C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C11920: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82C11924: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C11928: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C1192C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C11930: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C11938(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C11938 size=20
    let mut pc: u32 = 0x82C11938;
    'dispatch: loop {
        match pc {
            0x82C11938 => {
    //   block [0x82C11938..0x82C1194C)
	// 82C11938: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C1193C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C11940: 409A000C  bne cr6, 0x82c1194c
	if !ctx.cr[6].eq {
		sub_82C1194C(ctx, base);
		return;
	}
	// 82C11944: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82C11948: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C1194C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C1194C size=20
    let mut pc: u32 = 0x82C1194C;
    'dispatch: loop {
        match pc {
            0x82C1194C => {
    //   block [0x82C1194C..0x82C11960)
	// 82C1194C: 80630004  lwz r3, 4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C11950: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C11954: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C11958: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C1195C: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C11960(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C11960 size=12
    let mut pc: u32 = 0x82C11960;
    'dispatch: loop {
        match pc {
            0x82C11960 => {
    //   block [0x82C11960..0x82C1196C)
	// 82C11960: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C11964: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C11968: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C1196C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C1196C size=20
    let mut pc: u32 = 0x82C1196C;
    'dispatch: loop {
        match pc {
            0x82C1196C => {
    //   block [0x82C1196C..0x82C11980)
	// 82C1196C: 5563003E  slwi r3, r11, 0
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82C11970: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C11974: 814B001C  lwz r10, 0x1c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82C11978: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C1197C: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C11980(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C11980 size=4
    let mut pc: u32 = 0x82C11980;
    'dispatch: loop {
        match pc {
            0x82C11980 => {
    //   block [0x82C11980..0x82C11984)
	// 82C11980: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C11988(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C11988 size=12
    let mut pc: u32 = 0x82C11988;
    'dispatch: loop {
        match pc {
            0x82C11988 => {
    //   block [0x82C11988..0x82C11994)
	// 82C11988: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C1198C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C11990: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C11994(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C11994 size=20
    let mut pc: u32 = 0x82C11994;
    'dispatch: loop {
        match pc {
            0x82C11994 => {
    //   block [0x82C11994..0x82C119A8)
	// 82C11994: 5563003E  slwi r3, r11, 0
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82C11998: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C1199C: 814B0020  lwz r10, 0x20(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82C119A0: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C119A4: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C119A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C119A8 size=4
    let mut pc: u32 = 0x82C119A8;
    'dispatch: loop {
        match pc {
            0x82C119A8 => {
    //   block [0x82C119A8..0x82C119AC)
	// 82C119A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C119B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C119B0 size=12
    let mut pc: u32 = 0x82C119B0;
    'dispatch: loop {
        match pc {
            0x82C119B0 => {
    //   block [0x82C119B0..0x82C119BC)
	// 82C119B0: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C119B4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C119B8: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C119BC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C119BC size=20
    let mut pc: u32 = 0x82C119BC;
    'dispatch: loop {
        match pc {
            0x82C119BC => {
    //   block [0x82C119BC..0x82C119D0)
	// 82C119BC: 5563003E  slwi r3, r11, 0
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82C119C0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C119C4: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C119C8: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C119CC: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C119D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C119D0 size=4
    let mut pc: u32 = 0x82C119D0;
    'dispatch: loop {
        match pc {
            0x82C119D0 => {
    //   block [0x82C119D0..0x82C119D4)
	// 82C119D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C119D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C119D8 size=12
    let mut pc: u32 = 0x82C119D8;
    'dispatch: loop {
        match pc {
            0x82C119D8 => {
    //   block [0x82C119D8..0x82C119E4)
	// 82C119D8: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C119DC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C119E0: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C119E4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C119E4 size=16
    let mut pc: u32 = 0x82C119E4;
    'dispatch: loop {
        match pc {
            0x82C119E4 => {
    //   block [0x82C119E4..0x82C119F4)
	// 82C119E4: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C119E8: 556A07FE  clrlwi r10, r11, 0x1f
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	// 82C119EC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C119F0: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C119F4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C119F4 size=20
    let mut pc: u32 = 0x82C119F4;
    'dispatch: loop {
        match pc {
            0x82C119F4 => {
    //   block [0x82C119F4..0x82C11A08)
	// 82C119F4: 80630004  lwz r3, 4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C119F8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C119FC: 814B0010  lwz r10, 0x10(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C11A00: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C11A04: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C11A08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C11A08 size=4
    let mut pc: u32 = 0x82C11A08;
    'dispatch: loop {
        match pc {
            0x82C11A08 => {
    //   block [0x82C11A08..0x82C11A0C)
	// 82C11A08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C11A10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C11A10 size=12
    let mut pc: u32 = 0x82C11A10;
    'dispatch: loop {
        match pc {
            0x82C11A10 => {
    //   block [0x82C11A10..0x82C11A1C)
	// 82C11A10: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C11A14: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C11A18: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C11A1C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C11A1C size=16
    let mut pc: u32 = 0x82C11A1C;
    'dispatch: loop {
        match pc {
            0x82C11A1C => {
    //   block [0x82C11A1C..0x82C11A2C)
	// 82C11A1C: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C11A20: 556A07FE  clrlwi r10, r11, 0x1f
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	// 82C11A24: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C11A28: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C11A2C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C11A2C size=20
    let mut pc: u32 = 0x82C11A2C;
    'dispatch: loop {
        match pc {
            0x82C11A2C => {
    //   block [0x82C11A2C..0x82C11A40)
	// 82C11A2C: 80630004  lwz r3, 4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C11A30: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C11A34: 814B0014  lwz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82C11A38: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C11A3C: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C11A40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C11A40 size=4
    let mut pc: u32 = 0x82C11A40;
    'dispatch: loop {
        match pc {
            0x82C11A40 => {
    //   block [0x82C11A40..0x82C11A44)
	// 82C11A40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C11A48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C11A48 size=12
    let mut pc: u32 = 0x82C11A48;
    'dispatch: loop {
        match pc {
            0x82C11A48 => {
    //   block [0x82C11A48..0x82C11A54)
	// 82C11A48: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C11A4C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C11A50: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C11A54(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C11A54 size=20
    let mut pc: u32 = 0x82C11A54;
    'dispatch: loop {
        match pc {
            0x82C11A54 => {
    //   block [0x82C11A54..0x82C11A68)
	// 82C11A54: 5563003E  slwi r3, r11, 0
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82C11A58: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C11A5C: 814B0018  lwz r10, 0x18(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82C11A60: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C11A64: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C11A68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C11A68 size=4
    let mut pc: u32 = 0x82C11A68;
    'dispatch: loop {
        match pc {
            0x82C11A68 => {
    //   block [0x82C11A68..0x82C11A6C)
	// 82C11A68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C11A70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C11A70 size=12
    let mut pc: u32 = 0x82C11A70;
    'dispatch: loop {
        match pc {
            0x82C11A70 => {
    //   block [0x82C11A70..0x82C11A7C)
	// 82C11A70: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C11A74: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C11A78: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C11A7C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C11A7C size=20
    let mut pc: u32 = 0x82C11A7C;
    'dispatch: loop {
        match pc {
            0x82C11A7C => {
    //   block [0x82C11A7C..0x82C11A90)
	// 82C11A7C: 5563003E  slwi r3, r11, 0
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82C11A80: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C11A84: 814B0028  lwz r10, 0x28(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 82C11A88: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C11A8C: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C11A90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C11A90 size=4
    let mut pc: u32 = 0x82C11A90;
    'dispatch: loop {
        match pc {
            0x82C11A90 => {
    //   block [0x82C11A90..0x82C11A94)
	// 82C11A90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C11A98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C11A98 size=12
    let mut pc: u32 = 0x82C11A98;
    'dispatch: loop {
        match pc {
            0x82C11A98 => {
    //   block [0x82C11A98..0x82C11AA4)
	// 82C11A98: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C11A9C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C11AA0: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C11AA4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C11AA4 size=20
    let mut pc: u32 = 0x82C11AA4;
    'dispatch: loop {
        match pc {
            0x82C11AA4 => {
    //   block [0x82C11AA4..0x82C11AB8)
	// 82C11AA4: 5563003E  slwi r3, r11, 0
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82C11AA8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C11AAC: 814B002C  lwz r10, 0x2c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 82C11AB0: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C11AB4: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C11AB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C11AB8 size=4
    let mut pc: u32 = 0x82C11AB8;
    'dispatch: loop {
        match pc {
            0x82C11AB8 => {
    //   block [0x82C11AB8..0x82C11ABC)
	// 82C11AB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C11AC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C11AC0 size=20
    let mut pc: u32 = 0x82C11AC0;
    'dispatch: loop {
        match pc {
            0x82C11AC0 => {
    //   block [0x82C11AC0..0x82C11AD4)
	// 82C11AC0: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C11AC4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C11AC8: 409A000C  bne cr6, 0x82c11ad4
	if !ctx.cr[6].eq {
		sub_82C11AD4(ctx, base);
		return;
	}
	// 82C11ACC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82C11AD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C11AD4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C11AD4 size=24
    let mut pc: u32 = 0x82C11AD4;
    'dispatch: loop {
        match pc {
            0x82C11AD4 => {
    //   block [0x82C11AD4..0x82C11AEC)
	// 82C11AD4: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C11AD8: 386B000C  addi r3, r11, 0xc
	ctx.r[3].s64 = ctx.r[11].s64 + 12;
	// 82C11ADC: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C11AE0: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C11AE4: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C11AE8: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C11AF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C11AF0 size=24
    let mut pc: u32 = 0x82C11AF0;
    'dispatch: loop {
        match pc {
            0x82C11AF0 => {
    //   block [0x82C11AF0..0x82C11B08)
	// 82C11AF0: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C11AF4: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C11AF8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82C11AFC: 409A000C  bne cr6, 0x82c11b08
	if !ctx.cr[6].eq {
		sub_82C11B08(ctx, base);
		return;
	}
	// 82C11B00: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82C11B04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C11B08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C11B08 size=20
    let mut pc: u32 = 0x82C11B08;
    'dispatch: loop {
        match pc {
            0x82C11B08 => {
    //   block [0x82C11B08..0x82C11B1C)
	// 82C11B08: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C11B0C: 386B000C  addi r3, r11, 0xc
	ctx.r[3].s64 = ctx.r[11].s64 + 12;
	// 82C11B10: 812A0004  lwz r9, 4(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C11B14: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82C11B18: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C11B20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C11B20 size=16
    let mut pc: u32 = 0x82C11B20;
    'dispatch: loop {
        match pc {
            0x82C11B20 => {
    //   block [0x82C11B20..0x82C11B30)
	// 82C11B20: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C11B24: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C11B28: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82C11B2C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C11B30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C11B30 size=8
    let mut pc: u32 = 0x82C11B30;
    'dispatch: loop {
        match pc {
            0x82C11B30 => {
    //   block [0x82C11B30..0x82C11B38)
	// 82C11B30: 48009770  b 0x82c1b2a0
	sub_82C1B2A0(ctx, base);
	return;
	// 82C11B34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C11B38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C11B38 size=28
    let mut pc: u32 = 0x82C11B38;
    'dispatch: loop {
        match pc {
            0x82C11B38 => {
    //   block [0x82C11B38..0x82C11B54)
	// 82C11B38: 8163000C  lwz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C11B3C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C11B40: 386B0020  addi r3, r11, 0x20
	ctx.r[3].s64 = ctx.r[11].s64 + 32;
	// 82C11B44: 814B0020  lwz r10, 0x20(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82C11B48: 812A0004  lwz r9, 4(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C11B4C: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82C11B50: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C11B58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C11B58 size=12
    let mut pc: u32 = 0x82C11B58;
    'dispatch: loop {
        match pc {
            0x82C11B58 => {
    //   block [0x82C11B58..0x82C11B64)
	// 82C11B58: 8163000C  lwz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C11B5C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C11B60: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C11B64(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C11B64 size=12
    let mut pc: u32 = 0x82C11B64;
    'dispatch: loop {
        match pc {
            0x82C11B64 => {
    //   block [0x82C11B64..0x82C11B70)
	// 82C11B64: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C11B68: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82C11B6C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C11B70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C11B70 size=8
    let mut pc: u32 = 0x82C11B70;
    'dispatch: loop {
        match pc {
            0x82C11B70 => {
    //   block [0x82C11B70..0x82C11B78)
	// 82C11B70: 48009730  b 0x82c1b2a0
	sub_82C1B2A0(ctx, base);
	return;
	// 82C11B74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C11B78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82C11B78 size=40
    let mut pc: u32 = 0x82C11B78;
    'dispatch: loop {
        match pc {
            0x82C11B78 => {
    //   block [0x82C11B78..0x82C11BA0)
	// 82C11B78: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82C11B7C: 39400250  li r10, 0x250
	ctx.r[10].s64 = 592;
	// 82C11B80: 39200260  li r9, 0x260
	ctx.r[9].s64 = 608;
	// 82C11B84: 39000270  li r8, 0x270
	ctx.r[8].s64 = 624;
	// 82C11B88: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82C11B8C: D02B0280  stfs f1, 0x280(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(640 as u32), tmp.u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C11BA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C11BA0 size=8
    let mut pc: u32 = 0x82C11BA0;
    'dispatch: loop {
        match pc {
            0x82C11BA0 => {
    //   block [0x82C11BA0..0x82C11BA8)
	// 82C11BA0: 90830284  stw r4, 0x284(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(644 as u32), ctx.r[4].u32 ) };
	// 82C11BA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C11BA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C11BA8 size=20
    let mut pc: u32 = 0x82C11BA8;
    'dispatch: loop {
        match pc {
            0x82C11BA8 => {
    //   block [0x82C11BA8..0x82C11BBC)
	// 82C11BA8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82C11BAC: 7CA62B78  mr r6, r5
	ctx.r[6].u64 = ctx.r[5].u64;
	// 82C11BB0: 386B0060  addi r3, r11, 0x60
	ctx.r[3].s64 = ctx.r[11].s64 + 96;
	// 82C11BB4: 808B0288  lwz r4, 0x288(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(648 as u32) ) } as u64;
	// 82C11BB8: 4803E498  b 0x82c50050
	sub_82C50050(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C11BC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C11BC0 size=24
    let mut pc: u32 = 0x82C11BC0;
    'dispatch: loop {
        match pc {
            0x82C11BC0 => {
    //   block [0x82C11BC0..0x82C11BD8)
	// 82C11BC0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82C11BC4: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82C11BC8: 7CC73378  mr r7, r6
	ctx.r[7].u64 = ctx.r[6].u64;
	// 82C11BCC: 386B0060  addi r3, r11, 0x60
	ctx.r[3].s64 = ctx.r[11].s64 + 96;
	// 82C11BD0: 808B0288  lwz r4, 0x288(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(648 as u32) ) } as u64;
	// 82C11BD4: 4803E4AC  b 0x82c50080
	sub_82C50080(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C11BD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C11BD8 size=8
    let mut pc: u32 = 0x82C11BD8;
    'dispatch: loop {
        match pc {
            0x82C11BD8 => {
    //   block [0x82C11BD8..0x82C11BE0)
	// 82C11BD8: 38630060  addi r3, r3, 0x60
	ctx.r[3].s64 = ctx.r[3].s64 + 96;
	// 82C11BDC: 4803E054  b 0x82c4fc30
	sub_82C4FC30(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C11BE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C11BE0 size=8
    let mut pc: u32 = 0x82C11BE0;
    'dispatch: loop {
        match pc {
            0x82C11BE0 => {
    //   block [0x82C11BE0..0x82C11BE8)
	// 82C11BE0: 38630060  addi r3, r3, 0x60
	ctx.r[3].s64 = ctx.r[3].s64 + 96;
	// 82C11BE4: 4803E054  b 0x82c4fc38
	sub_82C4FC38(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C11BE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C11BE8 size=8
    let mut pc: u32 = 0x82C11BE8;
    'dispatch: loop {
        match pc {
            0x82C11BE8 => {
    //   block [0x82C11BE8..0x82C11BF0)
	// 82C11BE8: 38630060  addi r3, r3, 0x60
	ctx.r[3].s64 = ctx.r[3].s64 + 96;
	// 82C11BEC: 480402D4  b 0x82c51ec0
	sub_82C51EC0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C11BF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C11BF0 size=20
    let mut pc: u32 = 0x82C11BF0;
    'dispatch: loop {
        match pc {
            0x82C11BF0 => {
    //   block [0x82C11BF0..0x82C11C04)
	// 82C11BF0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82C11BF4: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82C11BF8: 386B0060  addi r3, r11, 0x60
	ctx.r[3].s64 = ctx.r[11].s64 + 96;
	// 82C11BFC: 808B0288  lwz r4, 0x288(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(648 as u32) ) } as u64;
	// 82C11C00: 4803F000  b 0x82c50c00
	sub_82C50C00(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C11C08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82C11C08 size=192
    let mut pc: u32 = 0x82C11C08;
    'dispatch: loop {
        match pc {
            0x82C11C08 => {
    //   block [0x82C11C08..0x82C11CC8)
	// 82C11C08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C11C0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C11C10: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82C11C14: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C11C18: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C11C1C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82C11C20: 807E0284  lwz r3, 0x284(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(644 as u32) ) } as u64;
	// 82C11C24: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 82C11C28: 409A0084  bne cr6, 0x82c11cac
	if !ctx.cr[6].eq {
	pc = 0x82C11CAC; continue 'dispatch;
	}
	// 82C11C2C: 4BFF1005  bl 0x82c02c30
	ctx.lr = 0x82C11C30;
	sub_82C02C30(ctx, base);
	// 82C11C30: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 82C11C34: 3BE0FFFF  li r31, -1
	ctx.r[31].s64 = -1;
	// 82C11C38: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82C11C3C: 93E10060  stw r31, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[31].u32 ) };
	// 82C11C40: 39600250  li r11, 0x250
	ctx.r[11].s64 = 592;
	// 82C11C44: 93E10064  stw r31, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[31].u32 ) };
	// 82C11C48: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 82C11C4C: C0090C18  lfs f0, 0xc18(r9)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(3096 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C11C50: 9101007C  stw r8, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[8].u32 ) };
	// 82C11C54: D0010068  stfs f0, 0x68(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), tmp.u32 ) };
	// 82C11C58: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 82C11C5C: D001006C  stfs f0, 0x6c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), tmp.u32 ) };
	// 82C11C60: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 82C11C64: D0010070  stfs f0, 0x70(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), tmp.u32 ) };
	// 82C11C68: D0010074  stfs f0, 0x74(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), tmp.u32 ) };
	// 82C11C6C: D0010078  stfs f0, 0x78(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), tmp.u32 ) };
	// 82C11C70: 80630034  lwz r3, 0x34(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(52 as u32) ) } as u64;
	// 82C11C74: 13FE58C7  vcmpequd (lvx128) v31, v30, v11
	tmp.u32 = ctx.r[30].u32 + ctx.r[11].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C11CC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C11CC8 size=140
    let mut pc: u32 = 0x82C11CC8;
    'dispatch: loop {
        match pc {
            0x82C11CC8 => {
    //   block [0x82C11CC8..0x82C11D54)
	// 82C11CC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C11CCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C11CD0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82C11CD4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C11CD8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C11CDC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C11CE0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82C11CE4: 7F1EF840  cmplw cr6, r30, r31
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[31].u32, &mut ctx.xer);
	// 82C11CE8: 419A0050  beq cr6, 0x82c11d38
	if ctx.cr[6].eq {
	pc = 0x82C11D38; continue 'dispatch;
	}
	// 82C11CEC: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C11CF0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C11CF4: 419A0018  beq cr6, 0x82c11d0c
	if ctx.cr[6].eq {
	pc = 0x82C11D0C; continue 'dispatch;
	}
	// 82C11CF8: 5563003E  slwi r3, r11, 0
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82C11CFC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C11D00: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C11D04: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C11D08: 4E800421  bctrl
	ctx.lr = 0x82C11D0C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C11D0C: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C11D10: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C11D14: 419A0014  beq cr6, 0x82c11d28
	if ctx.cr[6].eq {
	pc = 0x82C11D28; continue 'dispatch;
	}
	// 82C11D18: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C11D1C: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C11D20: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C11D24: 4E800421  bctrl
	ctx.lr = 0x82C11D28;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C11D28: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C11D2C: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82C11D30: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C11D34: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82C11D38: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C11D3C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C11D40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C11D44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C11D48: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82C11D4C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C11D50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C11D58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C11D58 size=16
    let mut pc: u32 = 0x82C11D58;
    'dispatch: loop {
        match pc {
            0x82C11D58 => {
    //   block [0x82C11D58..0x82C11D68)
	// 82C11D58: 8164000C  lwz r11, 0xc(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C11D5C: 39040004  addi r8, r4, 4
	ctx.r[8].s64 = ctx.r[4].s64 + 4;
	// 82C11D60: 7F0B1840  cmplw cr6, r11, r3
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[3].u32, &mut ctx.xer);
	// 82C11D64: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C11D68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C11D68 size=52
    let mut pc: u32 = 0x82C11D68;
    'dispatch: loop {
        match pc {
            0x82C11D68 => {
    //   block [0x82C11D68..0x82C11D9C)
	// 82C11D68: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C11D6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82C11D70: 81480000  lwz r10, 0(r8)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C11D74: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C11D78: 409A003C  bne cr6, 0x82c11db4
	if !ctx.cr[6].eq {
		sub_82C11DB4(ctx, base);
		return;
	}
	// 82C11D7C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C11D80: 409A001C  bne cr6, 0x82c11d9c
	if !ctx.cr[6].eq {
		sub_82C11D9C(ctx, base);
		return;
	}
	// 82C11D84: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82C11D88: 91230004  stw r9, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82C11D8C: 91280004  stw r9, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82C11D90: 91280000  stw r9, 0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82C11D94: 91280008  stw r9, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82C11D98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C11D9C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C11D9C size=24
    let mut pc: u32 = 0x82C11D9C;
    'dispatch: loop {
        match pc {
            0x82C11D9C => {
    //   block [0x82C11D9C..0x82C11DB4)
	// 82C11D9C: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82C11DA0: 912A0008  stw r9, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82C11DA4: 91280004  stw r9, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82C11DA8: 91280000  stw r9, 0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82C11DAC: 91280008  stw r9, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82C11DB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C11DB4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C11DB4 size=32
    let mut pc: u32 = 0x82C11DB4;
    'dispatch: loop {
        match pc {
            0x82C11DB4 => {
    //   block [0x82C11DB4..0x82C11DD4)
	// 82C11DB4: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C11DB8: 409A001C  bne cr6, 0x82c11dd4
	if !ctx.cr[6].eq {
		sub_82C11DD4(ctx, base);
		return;
	}
	// 82C11DBC: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82C11DC0: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82C11DC4: 91280004  stw r9, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82C11DC8: 91280000  stw r9, 0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82C11DCC: 91280008  stw r9, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82C11DD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C11DD4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C11DD4 size=24
    let mut pc: u32 = 0x82C11DD4;
    'dispatch: loop {
        match pc {
            0x82C11DD4 => {
    //   block [0x82C11DD4..0x82C11DEC)
	// 82C11DD4: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82C11DD8: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82C11DDC: 91280004  stw r9, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82C11DE0: 91280000  stw r9, 0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82C11DE4: 91280008  stw r9, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82C11DE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C11DF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C11DF0 size=16
    let mut pc: u32 = 0x82C11DF0;
    'dispatch: loop {
        match pc {
            0x82C11DF0 => {
    //   block [0x82C11DF0..0x82C11E00)
	// 82C11DF0: 81440040  lwz r10, 0x40(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(64 as u32) ) } as u64;
	// 82C11DF4: 39640038  addi r11, r4, 0x38
	ctx.r[11].s64 = ctx.r[4].s64 + 56;
	// 82C11DF8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C11DFC: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C11E00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C11E00 size=12
    let mut pc: u32 = 0x82C11E00;
    'dispatch: loop {
        match pc {
            0x82C11E00 => {
    //   block [0x82C11E00..0x82C11E0C)
	// 82C11E00: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C11E04: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C11E08: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C11E0C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C11E0C size=12
    let mut pc: u32 = 0x82C11E0C;
    'dispatch: loop {
        match pc {
            0x82C11E0C => {
    //   block [0x82C11E0C..0x82C11E18)
	// 82C11E0C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C11E10: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C11E14: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C11E18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C11E18 size=36
    let mut pc: u32 = 0x82C11E18;
    'dispatch: loop {
        match pc {
            0x82C11E18 => {
    //   block [0x82C11E18..0x82C11E3C)
	// 82C11E18: 906B0008  stw r3, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[3].u32 ) };
	// 82C11E1C: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C11E20: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C11E24: 409A0018  bne cr6, 0x82c11e3c
	if !ctx.cr[6].eq {
		sub_82C11E3C(ctx, base);
		return;
	}
	// 82C11E28: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82C11E2C: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82C11E30: 90830000  stw r4, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 82C11E34: 90830004  stw r4, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 82C11E38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C11E3C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C11E3C size=32
    let mut pc: u32 = 0x82C11E3C;
    'dispatch: loop {
        match pc {
            0x82C11E3C => {
    //   block [0x82C11E3C..0x82C11E5C)
	// 82C11E3C: 81430004  lwz r10, 4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C11E40: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82C11E44: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82C11E48: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82C11E4C: 81030004  lwz r8, 4(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C11E50: 90880038  stw r4, 0x38(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(56 as u32), ctx.r[4].u32 ) };
	// 82C11E54: 90830004  stw r4, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 82C11E58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C11E60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C11E60 size=16
    let mut pc: u32 = 0x82C11E60;
    'dispatch: loop {
        match pc {
            0x82C11E60 => {
    //   block [0x82C11E60..0x82C11E70)
	// 82C11E60: 81640040  lwz r11, 0x40(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(64 as u32) ) } as u64;
	// 82C11E64: 39040038  addi r8, r4, 0x38
	ctx.r[8].s64 = ctx.r[4].s64 + 56;
	// 82C11E68: 7F0B1840  cmplw cr6, r11, r3
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[3].u32, &mut ctx.xer);
	// 82C11E6C: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C11E70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C11E70 size=52
    let mut pc: u32 = 0x82C11E70;
    'dispatch: loop {
        match pc {
            0x82C11E70 => {
    //   block [0x82C11E70..0x82C11EA4)
	// 82C11E70: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C11E74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82C11E78: 81480000  lwz r10, 0(r8)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C11E7C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C11E80: 409A003C  bne cr6, 0x82c11ebc
	if !ctx.cr[6].eq {
		sub_82C11EBC(ctx, base);
		return;
	}
	// 82C11E84: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C11E88: 409A001C  bne cr6, 0x82c11ea4
	if !ctx.cr[6].eq {
		sub_82C11EA4(ctx, base);
		return;
	}
	// 82C11E8C: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82C11E90: 91230004  stw r9, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82C11E94: 91280004  stw r9, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82C11E98: 91280000  stw r9, 0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82C11E9C: 91280008  stw r9, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82C11EA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C11EA4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C11EA4 size=24
    let mut pc: u32 = 0x82C11EA4;
    'dispatch: loop {
        match pc {
            0x82C11EA4 => {
    //   block [0x82C11EA4..0x82C11EBC)
	// 82C11EA4: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82C11EA8: 912A003C  stw r9, 0x3c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(60 as u32), ctx.r[9].u32 ) };
	// 82C11EAC: 91280004  stw r9, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82C11EB0: 91280000  stw r9, 0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82C11EB4: 91280008  stw r9, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82C11EB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C11EBC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C11EBC size=32
    let mut pc: u32 = 0x82C11EBC;
    'dispatch: loop {
        match pc {
            0x82C11EBC => {
    //   block [0x82C11EBC..0x82C11EDC)
	// 82C11EBC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C11EC0: 409A001C  bne cr6, 0x82c11edc
	if !ctx.cr[6].eq {
		sub_82C11EDC(ctx, base);
		return;
	}
	// 82C11EC4: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82C11EC8: 912B0038  stw r9, 0x38(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(56 as u32), ctx.r[9].u32 ) };
	// 82C11ECC: 91280004  stw r9, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82C11ED0: 91280000  stw r9, 0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82C11ED4: 91280008  stw r9, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82C11ED8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C11EDC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C11EDC size=24
    let mut pc: u32 = 0x82C11EDC;
    'dispatch: loop {
        match pc {
            0x82C11EDC => {
    //   block [0x82C11EDC..0x82C11EF4)
	// 82C11EDC: 914B0038  stw r10, 0x38(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(56 as u32), ctx.r[10].u32 ) };
	// 82C11EE0: 916A003C  stw r11, 0x3c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(60 as u32), ctx.r[11].u32 ) };
	// 82C11EE4: 91280004  stw r9, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82C11EE8: 91280000  stw r9, 0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82C11EEC: 91280008  stw r9, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82C11EF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C11EF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C11EF8 size=16
    let mut pc: u32 = 0x82C11EF8;
    'dispatch: loop {
        match pc {
            0x82C11EF8 => {
    //   block [0x82C11EF8..0x82C11F08)
	// 82C11EF8: 81440068  lwz r10, 0x68(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(104 as u32) ) } as u64;
	// 82C11EFC: 39640060  addi r11, r4, 0x60
	ctx.r[11].s64 = ctx.r[4].s64 + 96;
	// 82C11F00: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C11F04: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C11F08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C11F08 size=12
    let mut pc: u32 = 0x82C11F08;
    'dispatch: loop {
        match pc {
            0x82C11F08 => {
    //   block [0x82C11F08..0x82C11F14)
	// 82C11F08: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C11F0C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C11F10: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C11F14(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C11F14 size=12
    let mut pc: u32 = 0x82C11F14;
    'dispatch: loop {
        match pc {
            0x82C11F14 => {
    //   block [0x82C11F14..0x82C11F20)
	// 82C11F14: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C11F18: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C11F1C: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C11F20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C11F20 size=36
    let mut pc: u32 = 0x82C11F20;
    'dispatch: loop {
        match pc {
            0x82C11F20 => {
    //   block [0x82C11F20..0x82C11F44)
	// 82C11F20: 906B0008  stw r3, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[3].u32 ) };
	// 82C11F24: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C11F28: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C11F2C: 409A0018  bne cr6, 0x82c11f44
	if !ctx.cr[6].eq {
		sub_82C11F44(ctx, base);
		return;
	}
	// 82C11F30: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82C11F34: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82C11F38: 90830000  stw r4, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 82C11F3C: 90830004  stw r4, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 82C11F40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C11F44(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C11F44 size=32
    let mut pc: u32 = 0x82C11F44;
    'dispatch: loop {
        match pc {
            0x82C11F44 => {
    //   block [0x82C11F44..0x82C11F64)
	// 82C11F44: 81430004  lwz r10, 4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C11F48: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82C11F4C: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82C11F50: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82C11F54: 81030004  lwz r8, 4(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C11F58: 90880060  stw r4, 0x60(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(96 as u32), ctx.r[4].u32 ) };
	// 82C11F5C: 90830004  stw r4, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 82C11F60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C11F68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C11F68 size=16
    let mut pc: u32 = 0x82C11F68;
    'dispatch: loop {
        match pc {
            0x82C11F68 => {
    //   block [0x82C11F68..0x82C11F78)
	// 82C11F68: 81640068  lwz r11, 0x68(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(104 as u32) ) } as u64;
	// 82C11F6C: 39040060  addi r8, r4, 0x60
	ctx.r[8].s64 = ctx.r[4].s64 + 96;
	// 82C11F70: 7F0B1840  cmplw cr6, r11, r3
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[3].u32, &mut ctx.xer);
	// 82C11F74: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C11F78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C11F78 size=52
    let mut pc: u32 = 0x82C11F78;
    'dispatch: loop {
        match pc {
            0x82C11F78 => {
    //   block [0x82C11F78..0x82C11FAC)
	// 82C11F78: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C11F7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82C11F80: 81480000  lwz r10, 0(r8)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C11F84: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C11F88: 409A003C  bne cr6, 0x82c11fc4
	if !ctx.cr[6].eq {
		sub_82C11FC4(ctx, base);
		return;
	}
	// 82C11F8C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C11F90: 409A001C  bne cr6, 0x82c11fac
	if !ctx.cr[6].eq {
		sub_82C11FAC(ctx, base);
		return;
	}
	// 82C11F94: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82C11F98: 91230004  stw r9, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82C11F9C: 91280004  stw r9, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82C11FA0: 91280000  stw r9, 0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82C11FA4: 91280008  stw r9, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82C11FA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C11FAC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C11FAC size=24
    let mut pc: u32 = 0x82C11FAC;
    'dispatch: loop {
        match pc {
            0x82C11FAC => {
    //   block [0x82C11FAC..0x82C11FC4)
	// 82C11FAC: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82C11FB0: 912A0064  stw r9, 0x64(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(100 as u32), ctx.r[9].u32 ) };
	// 82C11FB4: 91280004  stw r9, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82C11FB8: 91280000  stw r9, 0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82C11FBC: 91280008  stw r9, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82C11FC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C11FC4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C11FC4 size=32
    let mut pc: u32 = 0x82C11FC4;
    'dispatch: loop {
        match pc {
            0x82C11FC4 => {
    //   block [0x82C11FC4..0x82C11FE4)
	// 82C11FC4: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C11FC8: 409A001C  bne cr6, 0x82c11fe4
	if !ctx.cr[6].eq {
		sub_82C11FE4(ctx, base);
		return;
	}
	// 82C11FCC: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82C11FD0: 912B0060  stw r9, 0x60(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(96 as u32), ctx.r[9].u32 ) };
	// 82C11FD4: 91280004  stw r9, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82C11FD8: 91280000  stw r9, 0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82C11FDC: 91280008  stw r9, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82C11FE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C11FE4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C11FE4 size=24
    let mut pc: u32 = 0x82C11FE4;
    'dispatch: loop {
        match pc {
            0x82C11FE4 => {
    //   block [0x82C11FE4..0x82C11FFC)
	// 82C11FE4: 914B0060  stw r10, 0x60(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(96 as u32), ctx.r[10].u32 ) };
	// 82C11FE8: 916A0064  stw r11, 0x64(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82C11FEC: 91280004  stw r9, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82C11FF0: 91280000  stw r9, 0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82C11FF4: 91280008  stw r9, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82C11FF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C12000(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C12000 size=16
    let mut pc: u32 = 0x82C12000;
    'dispatch: loop {
        match pc {
            0x82C12000 => {
    //   block [0x82C12000..0x82C12010)
	// 82C12000: 8164005C  lwz r11, 0x5c(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(92 as u32) ) } as u64;
	// 82C12004: 39040054  addi r8, r4, 0x54
	ctx.r[8].s64 = ctx.r[4].s64 + 84;
	// 82C12008: 7F0B1840  cmplw cr6, r11, r3
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[3].u32, &mut ctx.xer);
	// 82C1200C: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C12010(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C12010 size=52
    let mut pc: u32 = 0x82C12010;
    'dispatch: loop {
        match pc {
            0x82C12010 => {
    //   block [0x82C12010..0x82C12044)
	// 82C12010: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C12014: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82C12018: 81480000  lwz r10, 0(r8)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C1201C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C12020: 409A003C  bne cr6, 0x82c1205c
	if !ctx.cr[6].eq {
		sub_82C1205C(ctx, base);
		return;
	}
	// 82C12024: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C12028: 409A001C  bne cr6, 0x82c12044
	if !ctx.cr[6].eq {
		sub_82C12044(ctx, base);
		return;
	}
	// 82C1202C: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82C12030: 91230004  stw r9, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82C12034: 91280004  stw r9, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82C12038: 91280000  stw r9, 0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82C1203C: 91280008  stw r9, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82C12040: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C12044(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C12044 size=24
    let mut pc: u32 = 0x82C12044;
    'dispatch: loop {
        match pc {
            0x82C12044 => {
    //   block [0x82C12044..0x82C1205C)
	// 82C12044: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82C12048: 912A0058  stw r9, 0x58(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(88 as u32), ctx.r[9].u32 ) };
	// 82C1204C: 91280004  stw r9, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82C12050: 91280000  stw r9, 0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82C12054: 91280008  stw r9, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82C12058: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C1205C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C1205C size=32
    let mut pc: u32 = 0x82C1205C;
    'dispatch: loop {
        match pc {
            0x82C1205C => {
    //   block [0x82C1205C..0x82C1207C)
	// 82C1205C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C12060: 409A001C  bne cr6, 0x82c1207c
	if !ctx.cr[6].eq {
		sub_82C1207C(ctx, base);
		return;
	}
	// 82C12064: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82C12068: 912B0054  stw r9, 0x54(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(84 as u32), ctx.r[9].u32 ) };
	// 82C1206C: 91280004  stw r9, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82C12070: 91280000  stw r9, 0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82C12074: 91280008  stw r9, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82C12078: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C1207C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C1207C size=24
    let mut pc: u32 = 0x82C1207C;
    'dispatch: loop {
        match pc {
            0x82C1207C => {
    //   block [0x82C1207C..0x82C12094)
	// 82C1207C: 914B0054  stw r10, 0x54(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 82C12080: 916A0058  stw r11, 0x58(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 82C12084: 91280004  stw r9, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82C12088: 91280000  stw r9, 0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82C1208C: 91280008  stw r9, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82C12090: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C12098(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C12098 size=16
    let mut pc: u32 = 0x82C12098;
    'dispatch: loop {
        match pc {
            0x82C12098 => {
    //   block [0x82C12098..0x82C120A8)
	// 82C12098: 81640018  lwz r11, 0x18(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(24 as u32) ) } as u64;
	// 82C1209C: 39040010  addi r8, r4, 0x10
	ctx.r[8].s64 = ctx.r[4].s64 + 16;
	// 82C120A0: 7F0B1840  cmplw cr6, r11, r3
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[3].u32, &mut ctx.xer);
	// 82C120A4: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C120A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C120A8 size=52
    let mut pc: u32 = 0x82C120A8;
    'dispatch: loop {
        match pc {
            0x82C120A8 => {
    //   block [0x82C120A8..0x82C120DC)
	// 82C120A8: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C120AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82C120B0: 81480000  lwz r10, 0(r8)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C120B4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C120B8: 409A003C  bne cr6, 0x82c120f4
	if !ctx.cr[6].eq {
		sub_82C120F4(ctx, base);
		return;
	}
	// 82C120BC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C120C0: 409A001C  bne cr6, 0x82c120dc
	if !ctx.cr[6].eq {
		sub_82C120DC(ctx, base);
		return;
	}
	// 82C120C4: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82C120C8: 91230004  stw r9, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82C120CC: 91280004  stw r9, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82C120D0: 91280000  stw r9, 0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82C120D4: 91280008  stw r9, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82C120D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C120DC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C120DC size=24
    let mut pc: u32 = 0x82C120DC;
    'dispatch: loop {
        match pc {
            0x82C120DC => {
    //   block [0x82C120DC..0x82C120F4)
	// 82C120DC: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82C120E0: 912A0014  stw r9, 0x14(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(20 as u32), ctx.r[9].u32 ) };
	// 82C120E4: 91280004  stw r9, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82C120E8: 91280000  stw r9, 0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82C120EC: 91280008  stw r9, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82C120F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C120F4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C120F4 size=32
    let mut pc: u32 = 0x82C120F4;
    'dispatch: loop {
        match pc {
            0x82C120F4 => {
    //   block [0x82C120F4..0x82C12114)
	// 82C120F4: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C120F8: 409A001C  bne cr6, 0x82c12114
	if !ctx.cr[6].eq {
		sub_82C12114(ctx, base);
		return;
	}
	// 82C120FC: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82C12100: 912B0010  stw r9, 0x10(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[9].u32 ) };
	// 82C12104: 91280004  stw r9, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82C12108: 91280000  stw r9, 0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82C1210C: 91280008  stw r9, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82C12110: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C12114(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C12114 size=24
    let mut pc: u32 = 0x82C12114;
    'dispatch: loop {
        match pc {
            0x82C12114 => {
    //   block [0x82C12114..0x82C1212C)
	// 82C12114: 914B0010  stw r10, 0x10(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 82C12118: 916A0014  stw r11, 0x14(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82C1211C: 91280004  stw r9, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82C12120: 91280000  stw r9, 0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82C12124: 91280008  stw r9, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82C12128: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C12130(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C12130 size=52
    let mut pc: u32 = 0x82C12130;
    'dispatch: loop {
        match pc {
            0x82C12130 => {
    //   block [0x82C12130..0x82C12164)
	// 82C12130: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C12134: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C12138: 409A0008  bne cr6, 0x82c12140
	if !ctx.cr[6].eq {
	pc = 0x82C12140; continue 'dispatch;
	}
	// 82C1213C: 0FE00016  twui r0, 0x16
	// twui: trap word unsigned immediate — TODO: implement trap semantics
	// 82C12140: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C12144: 894B0029  lbz r10, 0x29(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(41 as u32) ) } as u64;
	// 82C12148: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C1214C: 419A0020  beq cr6, 0x82c1216c
	if ctx.cr[6].eq {
		sub_82C1216C(ctx, base);
		return;
	}
	// 82C12150: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C12154: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82C12158: 894B0029  lbz r10, 0x29(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(41 as u32) ) } as u64;
	// 82C1215C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C12160: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C12164(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C12164 size=8
    let mut pc: u32 = 0x82C12164;
    'dispatch: loop {
        match pc {
            0x82C12164 => {
    //   block [0x82C12164..0x82C1216C)
	// 82C12164: 0FE00016  twui r0, 0x16
	// twui: trap word unsigned immediate — TODO: implement trap semantics
	// 82C12168: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C1216C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C1216C size=60
    let mut pc: u32 = 0x82C1216C;
    'dispatch: loop {
        match pc {
            0x82C1216C => {
    //   block [0x82C1216C..0x82C121A8)
	// 82C1216C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C12170: 892A0029  lbz r9, 0x29(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(41 as u32) ) } as u64;
	// 82C12174: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82C12178: 409A0030  bne cr6, 0x82c121a8
	if !ctx.cr[6].eq {
		sub_82C121A8(ctx, base);
		return;
	}
	// 82C1217C: 816A0008  lwz r11, 8(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C12180: 892B0029  lbz r9, 0x29(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(41 as u32) ) } as u64;
	// 82C12184: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82C12188: 409A0018  bne cr6, 0x82c121a0
	if !ctx.cr[6].eq {
	pc = 0x82C121A0; continue 'dispatch;
	}
	// 82C1218C: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 82C12190: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C12194: 892B0029  lbz r9, 0x29(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(41 as u32) ) } as u64;
	// 82C12198: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82C1219C: 419AFFF0  beq cr6, 0x82c1218c
	if ctx.cr[6].eq {
	pc = 0x82C1218C; continue 'dispatch;
	}
	// 82C121A0: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82C121A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C121A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C121A8 size=80
    let mut pc: u32 = 0x82C121A8;
    'dispatch: loop {
        match pc {
            0x82C121A8 => {
    //   block [0x82C121A8..0x82C121F8)
	// 82C121A8: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C121AC: 894B0029  lbz r10, 0x29(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(41 as u32) ) } as u64;
	// 82C121B0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C121B4: 409A002C  bne cr6, 0x82c121e0
	if !ctx.cr[6].eq {
	pc = 0x82C121E0; continue 'dispatch;
	}
	// 82C121B8: 81430004  lwz r10, 4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C121BC: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C121C0: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82C121C4: 409A001C  bne cr6, 0x82c121e0
	if !ctx.cr[6].eq {
	pc = 0x82C121E0; continue 'dispatch;
	}
	// 82C121C8: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82C121CC: 556B003E  slwi r11, r11, 0
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82C121D0: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C121D4: 894B0029  lbz r10, 0x29(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(41 as u32) ) } as u64;
	// 82C121D8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C121DC: 419AFFDC  beq cr6, 0x82c121b8
	if ctx.cr[6].eq {
	pc = 0x82C121B8; continue 'dispatch;
	}
	// 82C121E0: 81430004  lwz r10, 4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C121E4: 892A0029  lbz r9, 0x29(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(41 as u32) ) } as u64;
	// 82C121E8: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82C121EC: 409AFF78  bne cr6, 0x82c12164
	if !ctx.cr[6].eq {
		sub_82C12164(ctx, base);
		return;
	}
	// 82C121F0: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82C121F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C121F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C121F8 size=40
    let mut pc: u32 = 0x82C121F8;
    'dispatch: loop {
        match pc {
            0x82C121F8 => {
    //   block [0x82C121F8..0x82C12220)
	// 82C121F8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C121FC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C12200: 409A0008  bne cr6, 0x82c12208
	if !ctx.cr[6].eq {
	pc = 0x82C12208; continue 'dispatch;
	}
	// 82C12204: 0FE00016  twui r0, 0x16
	// twui: trap word unsigned immediate — TODO: implement trap semantics
	// 82C12208: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C1220C: 894B0029  lbz r10, 0x29(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(41 as u32) ) } as u64;
	// 82C12210: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C12214: 419A000C  beq cr6, 0x82c12220
	if ctx.cr[6].eq {
		sub_82C12220(ctx, base);
		return;
	}
	// 82C12218: 0FE00016  twui r0, 0x16
	// twui: trap word unsigned immediate — TODO: implement trap semantics
	// 82C1221C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C12220(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C12220 size=60
    let mut pc: u32 = 0x82C12220;
    'dispatch: loop {
        match pc {
            0x82C12220 => {
    //   block [0x82C12220..0x82C1225C)
	// 82C12220: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C12224: 892A0029  lbz r9, 0x29(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(41 as u32) ) } as u64;
	// 82C12228: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82C1222C: 409A0030  bne cr6, 0x82c1225c
	if !ctx.cr[6].eq {
		sub_82C1225C(ctx, base);
		return;
	}
	// 82C12230: 816A0000  lwz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C12234: 892B0029  lbz r9, 0x29(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(41 as u32) ) } as u64;
	// 82C12238: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82C1223C: 409A0018  bne cr6, 0x82c12254
	if !ctx.cr[6].eq {
	pc = 0x82C12254; continue 'dispatch;
	}
	// 82C12240: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 82C12244: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C12248: 892B0029  lbz r9, 0x29(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(41 as u32) ) } as u64;
	// 82C1224C: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82C12250: 419AFFF0  beq cr6, 0x82c12240
	if ctx.cr[6].eq {
	pc = 0x82C12240; continue 'dispatch;
	}
	// 82C12254: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82C12258: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C1225C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C1225C size=64
    let mut pc: u32 = 0x82C1225C;
    'dispatch: loop {
        match pc {
            0x82C1225C => {
    //   block [0x82C1225C..0x82C1229C)
	// 82C1225C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C12260: 894B0029  lbz r10, 0x29(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(41 as u32) ) } as u64;
	// 82C12264: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C12268: 409A002C  bne cr6, 0x82c12294
	if !ctx.cr[6].eq {
	pc = 0x82C12294; continue 'dispatch;
	}
	// 82C1226C: 81430004  lwz r10, 4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C12270: 812B0008  lwz r9, 8(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C12274: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82C12278: 409A001C  bne cr6, 0x82c12294
	if !ctx.cr[6].eq {
	pc = 0x82C12294; continue 'dispatch;
	}
	// 82C1227C: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82C12280: 556B003E  slwi r11, r11, 0
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82C12284: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C12288: 894B0029  lbz r10, 0x29(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(41 as u32) ) } as u64;
	// 82C1228C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C12290: 419AFFDC  beq cr6, 0x82c1226c
	if ctx.cr[6].eq {
	pc = 0x82C1226C; continue 'dispatch;
	}
	// 82C12294: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82C12298: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C122A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82C122A0 size=212
    let mut pc: u32 = 0x82C122A0;
    'dispatch: loop {
        match pc {
            0x82C122A0 => {
    //   block [0x82C122A0..0x82C12374)
	// 82C122A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C122A4: 4809715D  bl 0x82ca9400
	ctx.lr = 0x82C122A8;
	sub_82CA93D0(ctx, base);
	// 82C122A8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C122AC: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 82C122B0: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82C122B4: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82C122B8: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 82C122BC: 9BDA0000  stb r30, 0(r26)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[30].u8 ) };
	// 82C122C0: 817C0008  lwz r11, 8(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C122C4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C122C8: 409A0010  bne cr6, 0x82c122d8
	if !ctx.cr[6].eq {
	pc = 0x82C122D8; continue 'dispatch;
	}
	// 82C122CC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82C122D0: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82C122D4: 4809717C  b 0x82ca9450
	sub_82CA9420(ctx, base);
	return;
	// 82C122D8: 37ABFFFF  addic. r29, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[29].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82C122DC: 41800040  blt 0x82c1231c
	if ctx.cr[0].lt {
	pc = 0x82C1231C; continue 'dispatch;
	}
	// 82C122E0: 7D7EEA14  add r11, r30, r29
	ctx.r[11].u64 = ctx.r[30].u64 + ctx.r[29].u64;
	// 82C122E4: 815C0000  lwz r10, 0(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C122E8: 7D690E70  srawi r9, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[9].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 82C122EC: 7FE90194  addze r31, r9
	tmp.s64 = ctx.r[9].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[9].u32);
	ctx.r[31].s64 = tmp.s64;
	// 82C122F0: 57E8103A  slwi r8, r31, 2
	ctx.r[8].u32 = ctx.r[31].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82C122F4: 7C68502E  lwzx r3, r8, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82C122F8: 4BF84B81  bl 0x82b96e78
	ctx.lr = 0x82C122FC;
	sub_82B96E78(ctx, base);
	// 82C122FC: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C12300: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C12304: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82C12308: 41990050  bgt cr6, 0x82c12358
	if ctx.cr[6].gt {
	pc = 0x82C12358; continue 'dispatch;
	}
	// 82C1230C: 40980054  bge cr6, 0x82c12360
	if !ctx.cr[6].lt {
	pc = 0x82C12360; continue 'dispatch;
	}
	// 82C12310: 3BDF0001  addi r30, r31, 1
	ctx.r[30].s64 = ctx.r[31].s64 + 1;
	// 82C12314: 7F1EE800  cmpw cr6, r30, r29
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[29].s32, &mut ctx.xer);
	// 82C12318: 4099FFC8  ble cr6, 0x82c122e0
	if !ctx.cr[6].gt {
	pc = 0x82C122E0; continue 'dispatch;
	}
	// 82C1231C: 7D7EEA14  add r11, r30, r29
	ctx.r[11].u64 = ctx.r[30].u64 + ctx.r[29].u64;
	// 82C12320: 815C0000  lwz r10, 0(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C12324: 7D690E70  srawi r9, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[9].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 82C12328: 7FE90194  addze r31, r9
	tmp.s64 = ctx.r[9].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[9].u32);
	ctx.r[31].s64 = tmp.s64;
	// 82C1232C: 57E8103A  slwi r8, r31, 2
	ctx.r[8].u32 = ctx.r[31].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82C12330: 7C68502E  lwzx r3, r8, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82C12334: 4BF84B45  bl 0x82b96e78
	ctx.lr = 0x82C12338;
	sub_82B96E78(ctx, base);
	// 82C12338: 80C30000  lwz r6, 0(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C1233C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C12340: 80FB0000  lwz r7, 0(r27)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C12344: 7F073040  cmplw cr6, r7, r6
	ctx.cr[6].compare_u32(ctx.r[7].u32, ctx.r[6].u32, &mut ctx.xer);
	// 82C12348: 41980008  blt cr6, 0x82c12350
	if ctx.cr[6].lt {
	pc = 0x82C12350; continue 'dispatch;
	}
	// 82C1234C: 387F0001  addi r3, r31, 1
	ctx.r[3].s64 = ctx.r[31].s64 + 1;
	// 82C12350: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82C12354: 480970FC  b 0x82ca9450
	sub_82CA9420(ctx, base);
	return;
	// 82C12358: 3BBFFFFF  addi r29, r31, -1
	ctx.r[29].s64 = ctx.r[31].s64 + -1;
	// 82C1235C: 4BFFFFB8  b 0x82c12314
	pc = 0x82C12314; continue 'dispatch;
	// 82C12360: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82C12364: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C12368: 997A0000  stb r11, 0(r26)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82C1236C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82C12370: 480970E0  b 0x82ca9450
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C12378(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C12378 size=92
    let mut pc: u32 = 0x82C12378;
    'dispatch: loop {
        match pc {
            0x82C12378 => {
    //   block [0x82C12378..0x82C123D4)
	// 82C12378: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C1237C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C12380: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82C12384: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C12388: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C1238C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C12390: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C12394: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82C12398: 394BAB10  addi r10, r11, -0x54f0
	ctx.r[10].s64 = ctx.r[11].s64 + -21744;
	// 82C1239C: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82C123A0: 4BFF3D61  bl 0x82c06100
	ctx.lr = 0x82C123A4;
	sub_82C06100(ctx, base);
	// 82C123A4: 57C907FE  clrlwi r9, r30, 0x1f
	ctx.r[9].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82C123A8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C123AC: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82C123B0: 419A000C  beq cr6, 0x82c123bc
	if ctx.cr[6].eq {
	pc = 0x82C123BC; continue 'dispatch;
	}
	// 82C123B4: 4BC333FD  bl 0x828457b0
	ctx.lr = 0x82C123B8;
	sub_828457B0(ctx, base);
	// 82C123B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C123BC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C123C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C123C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C123C8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82C123CC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C123D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C123D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82C123D8 size=144
    let mut pc: u32 = 0x82C123D8;
    'dispatch: loop {
        match pc {
            0x82C123D8 => {
    //   block [0x82C123D8..0x82C12468)
	// 82C123D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C123DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C123E0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82C123E4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C123E8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C123EC: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82C123F0: 3D408209  lis r10, -0x7df7
	ctx.r[10].s64 = -2113339392;
	// 82C123F4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82C123F8: 386AFFDF  addi r3, r10, -0x21
	ctx.r[3].s64 = ctx.r[10].s64 + -33;
	// 82C123FC: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82C12400: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82C12404: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C12408: 419A0008  beq cr6, 0x82c12410
	if ctx.cr[6].eq {
	pc = 0x82C12410; continue 'dispatch;
	}
	// 82C1240C: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C12410: 817E0018  lwz r11, 0x18(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 82C12414: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C12418: 419A0008  beq cr6, 0x82c12420
	if ctx.cr[6].eq {
	pc = 0x82C12420; continue 'dispatch;
	}
	// 82C1241C: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C12420: 4809A731  bl 0x82cacb50
	ctx.lr = 0x82C12424;
	sub_82CACB50(ctx, base);
	// 82C12424: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82C12428: 409A0028  bne cr6, 0x82c12450
	if !ctx.cr[6].eq {
	pc = 0x82C12450; continue 'dispatch;
	}
	// 82C1242C: 817E001C  lwz r11, 0x1c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 82C12430: 815F001C  lwz r10, 0x1c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82C12434: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82C12438: 4098000C  bge cr6, 0x82c12444
	if !ctx.cr[6].lt {
	pc = 0x82C12444; continue 'dispatch;
	}
	// 82C1243C: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82C12440: 48000010  b 0x82c12450
	pc = 0x82C12450; continue 'dispatch;
	// 82C12444: 7D6B5010  subfc r11, r11, r10
	ctx.xer.ca = ctx.r[10].u32 >= ctx.r[11].u32;
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 82C12448: 7D4B5910  subfe r10, r11, r11
	let x = (!ctx.r[11].u32);
	let y = ctx.r[11].u32;
	let s = x.wrapping_add(y);
	let res = s.wrapping_add(ctx.xer.ca as u32);
	tmp.u8 = (s < x) as u8 | (res < s) as u8;
	ctx.r[10].u32 = res;
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	ctx.xer.ca = (tmp.u8 != 0);
	// 82C1244C: 554307FE  clrlwi r3, r10, 0x1f
	ctx.r[3].u64 = ctx.r[10].u32 as u64 & 0x00000001u64;
	// 82C12450: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C12454: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C12458: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C1245C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82C12460: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C12464: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C12468(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C12468 size=12
    let mut pc: u32 = 0x82C12468;
    'dispatch: loop {
        match pc {
            0x82C12468 => {
    //   block [0x82C12468..0x82C12474)
	// 82C12468: 8063004C  lwz r3, 0x4c(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(76 as u32) ) } as u64;
	// 82C1246C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C12470: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C12474(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82C12474 size=52
    let mut pc: u32 = 0x82C12474;
    'dispatch: loop {
        match pc {
            0x82C12474 => {
    //   block [0x82C12474..0x82C124A8)
	// 82C12474: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82C12478: C00B0C18  lfs f0, 0xc18(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3096 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C1247C: FF010000  fcmpu cr6, f1, f0
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 82C12480: 41980014  blt cr6, 0x82c12494
	if ctx.cr[6].lt {
	pc = 0x82C12494; continue 'dispatch;
	}
	// 82C12484: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82C12488: C00B0C14  lfs f0, 0xc14(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3092 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C1248C: FF010000  fcmpu cr6, f1, f0
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 82C12490: 40990008  ble cr6, 0x82c12498
	if !ctx.cr[6].gt {
	pc = 0x82C12498; continue 'dispatch;
	}
	// 82C12494: FC200090  fmr f1, f0
	ctx.f[1].f64 = ctx.f[0].f64;
	// 82C12498: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C1249C: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C124A0: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C124A4: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C124A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C124A8 size=4
    let mut pc: u32 = 0x82C124A8;
    'dispatch: loop {
        match pc {
            0x82C124A8 => {
    //   block [0x82C124A8..0x82C124AC)
	// 82C124A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C124B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C124B0 size=12
    let mut pc: u32 = 0x82C124B0;
    'dispatch: loop {
        match pc {
            0x82C124B0 => {
    //   block [0x82C124B0..0x82C124BC)
	// 82C124B0: 8063004C  lwz r3, 0x4c(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(76 as u32) ) } as u64;
	// 82C124B4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C124B8: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C124BC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C124BC size=16
    let mut pc: u32 = 0x82C124BC;
    'dispatch: loop {
        match pc {
            0x82C124BC => {
    //   block [0x82C124BC..0x82C124CC)
	// 82C124BC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C124C0: 814B0010  lwz r10, 0x10(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C124C4: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C124C8: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C124CC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C124CC size=4
    let mut pc: u32 = 0x82C124CC;
    'dispatch: loop {
        match pc {
            0x82C124CC => {
    //   block [0x82C124CC..0x82C124D0)
	// 82C124CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C124D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C124D0 size=12
    let mut pc: u32 = 0x82C124D0;
    'dispatch: loop {
        match pc {
            0x82C124D0 => {
    //   block [0x82C124D0..0x82C124DC)
	// 82C124D0: 8063004C  lwz r3, 0x4c(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(76 as u32) ) } as u64;
	// 82C124D4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C124D8: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C124DC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C124DC size=16
    let mut pc: u32 = 0x82C124DC;
    'dispatch: loop {
        match pc {
            0x82C124DC => {
    //   block [0x82C124DC..0x82C124EC)
	// 82C124DC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C124E0: 814B0014  lwz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82C124E4: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C124E8: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C124EC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C124EC size=4
    let mut pc: u32 = 0x82C124EC;
    'dispatch: loop {
        match pc {
            0x82C124EC => {
    //   block [0x82C124EC..0x82C124F0)
	// 82C124EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C124F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C124F0 size=12
    let mut pc: u32 = 0x82C124F0;
    'dispatch: loop {
        match pc {
            0x82C124F0 => {
    //   block [0x82C124F0..0x82C124FC)
	// 82C124F0: 8063004C  lwz r3, 0x4c(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(76 as u32) ) } as u64;
	// 82C124F4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C124F8: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C124FC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C124FC size=16
    let mut pc: u32 = 0x82C124FC;
    'dispatch: loop {
        match pc {
            0x82C124FC => {
    //   block [0x82C124FC..0x82C1250C)
	// 82C124FC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C12500: 814B0018  lwz r10, 0x18(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82C12504: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C12508: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C1250C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C1250C size=4
    let mut pc: u32 = 0x82C1250C;
    'dispatch: loop {
        match pc {
            0x82C1250C => {
    //   block [0x82C1250C..0x82C12510)
	// 82C1250C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C12510(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C12510 size=12
    let mut pc: u32 = 0x82C12510;
    'dispatch: loop {
        match pc {
            0x82C12510 => {
    //   block [0x82C12510..0x82C1251C)
	// 82C12510: 8063004C  lwz r3, 0x4c(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(76 as u32) ) } as u64;
	// 82C12514: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C12518: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C1251C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C1251C size=16
    let mut pc: u32 = 0x82C1251C;
    'dispatch: loop {
        match pc {
            0x82C1251C => {
    //   block [0x82C1251C..0x82C1252C)
	// 82C1251C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C12520: 814B001C  lwz r10, 0x1c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82C12524: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C12528: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C1252C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C1252C size=4
    let mut pc: u32 = 0x82C1252C;
    'dispatch: loop {
        match pc {
            0x82C1252C => {
    //   block [0x82C1252C..0x82C12530)
	// 82C1252C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C12530(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C12530 size=12
    let mut pc: u32 = 0x82C12530;
    'dispatch: loop {
        match pc {
            0x82C12530 => {
    //   block [0x82C12530..0x82C1253C)
	// 82C12530: 8063004C  lwz r3, 0x4c(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(76 as u32) ) } as u64;
	// 82C12534: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C12538: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C1253C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C1253C size=16
    let mut pc: u32 = 0x82C1253C;
    'dispatch: loop {
        match pc {
            0x82C1253C => {
    //   block [0x82C1253C..0x82C1254C)
	// 82C1253C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C12540: 814B0020  lwz r10, 0x20(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82C12544: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C12548: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C1254C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C1254C size=4
    let mut pc: u32 = 0x82C1254C;
    'dispatch: loop {
        match pc {
            0x82C1254C => {
    //   block [0x82C1254C..0x82C12550)
	// 82C1254C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C12550(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C12550 size=24
    let mut pc: u32 = 0x82C12550;
    'dispatch: loop {
        match pc {
            0x82C12550 => {
    //   block [0x82C12550..0x82C12568)
	// 82C12550: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82C12554: 39400030  li r10, 0x30
	ctx.r[10].s64 = 48;
	// 82C12558: 806B0024  lwz r3, 0x24(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C12568(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C12568 size=16
    let mut pc: u32 = 0x82C12568;
    'dispatch: loop {
        match pc {
            0x82C12568 => {
    //   block [0x82C12568..0x82C12578)
	// 82C12568: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C1256C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C12570: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C12574: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C12578(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C12578 size=4
    let mut pc: u32 = 0x82C12578;
    'dispatch: loop {
        match pc {
            0x82C12578 => {
    //   block [0x82C12578..0x82C1257C)
	// 82C12578: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C12580(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82C12580 size=1660
    let mut pc: u32 = 0x82C12580;
    'dispatch: loop {
        match pc {
            0x82C12580 => {
    //   block [0x82C12580..0x82C12BFC)
	// 82C12580: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C12584: 48096E61  bl 0x82ca93e4
	ctx.lr = 0x82C12588;
	sub_82CA93D0(ctx, base);
	// 82C12588: DBE1FF88  stfd f31, -0x78(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-120 as u32), ctx.f[31].u64 ) };
	// 82C1258C: 9421FF00  stwu r1, -0x100(r1)
	ea = ctx.r[1].u32.wrapping_add(-256 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C12590: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82C12594: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C12598: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82C1259C: 4BFF3195  bl 0x82c05730
	ctx.lr = 0x82C125A0;
	sub_82C05730(ctx, base);
	// 82C125A0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C125A4: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82C125A8: 388BAC2C  addi r4, r11, -0x53d4
	ctx.r[4].s64 = ctx.r[11].s64 + -21460;
	// 82C125AC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C125B0: 3BCAAC1C  addi r30, r10, -0x53e4
	ctx.r[30].s64 = ctx.r[10].s64 + -21476;
	// 82C125B4: 3B9D0044  addi r28, r29, 0x44
	ctx.r[28].s64 = ctx.r[29].s64 + 68;
	// 82C125B8: 4BFF3241  bl 0x82c057f8
	ctx.lr = 0x82C125BC;
	sub_82C057F8(ctx, base);
	// 82C125BC: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82C125C0: 4BFF3201  bl 0x82c057c0
	ctx.lr = 0x82C125C4;
	sub_82C057C0(ctx, base);
	// 82C125C4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82C125C8: 4BFF3231  bl 0x82c057f8
	ctx.lr = 0x82C125CC;
	sub_82C057F8(ctx, base);
	// 82C125CC: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82C125D0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C125D4: 3BCB0C88  addi r30, r11, 0xc88
	ctx.r[30].s64 = ctx.r[11].s64 + 3208;
	// 82C125D8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82C125DC: 4BFF321D  bl 0x82c057f8
	ctx.lr = 0x82C125E0;
	sub_82C057F8(ctx, base);
	// 82C125E0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C125E4: 839F0000  lwz r28, 0(r31)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C125E8: 4BFF34D9  bl 0x82c05ac0
	ctx.lr = 0x82C125EC;
	sub_82C05AC0(ctx, base);
	// 82C125EC: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82C125F0: 813C0008  lwz r9, 8(r28)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C125F4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C125F8: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82C125FC: 4E800421  bctrl
	ctx.lr = 0x82C12600;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C12600: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C12604: 4BFF316D  bl 0x82c05770
	ctx.lr = 0x82C12608;
	sub_82C05770(ctx, base);
	// 82C12608: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C1260C: 4BFF3125  bl 0x82c05730
	ctx.lr = 0x82C12610;
	sub_82C05730(ctx, base);
	// 82C12610: 3D008201  lis r8, -0x7dff
	ctx.r[8].s64 = -2113863680;
	// 82C12614: 3CE08201  lis r7, -0x7dff
	ctx.r[7].s64 = -2113863680;
	// 82C12618: 3888AC0C  addi r4, r8, -0x53f4
	ctx.r[4].s64 = ctx.r[8].s64 + -21492;
	// 82C1261C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C12620: 3B87ABFC  addi r28, r7, -0x5404
	ctx.r[28].s64 = ctx.r[7].s64 + -21508;
	// 82C12624: 837D0008  lwz r27, 8(r29)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C12628: 4BFF31D1  bl 0x82c057f8
	ctx.lr = 0x82C1262C;
	sub_82C057F8(ctx, base);
	// 82C1262C: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82C12630: 4BFF3239  bl 0x82c05868
	ctx.lr = 0x82C12634;
	sub_82C05868(ctx, base);
	// 82C12634: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82C12638: 4BFF31C1  bl 0x82c057f8
	ctx.lr = 0x82C1263C;
	sub_82C057F8(ctx, base);
	// 82C1263C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82C12640: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C12644: 4BFF31B5  bl 0x82c057f8
	ctx.lr = 0x82C12648;
	sub_82C057F8(ctx, base);
	// 82C12648: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C1264C: 839F0000  lwz r28, 0(r31)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C12650: 4BFF3471  bl 0x82c05ac0
	ctx.lr = 0x82C12654;
	sub_82C05AC0(ctx, base);
	// 82C12654: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82C12658: 80DC0008  lwz r6, 8(r28)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C1265C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C12660: 7CC903A6  mtctr r6
	ctx.ctr.u64 = ctx.r[6].u64;
	// 82C12664: 4E800421  bctrl
	ctx.lr = 0x82C12668;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C12668: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C1266C: 4BFF3105  bl 0x82c05770
	ctx.lr = 0x82C12670;
	sub_82C05770(ctx, base);
	// 82C12670: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C12674: 4BFF30BD  bl 0x82c05730
	ctx.lr = 0x82C12678;
	sub_82C05730(ctx, base);
	// 82C12678: 3CA08201  lis r5, -0x7dff
	ctx.r[5].s64 = -2113863680;
	// 82C1267C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C12680: 3885ABEC  addi r4, r5, -0x5414
	ctx.r[4].s64 = ctx.r[5].s64 + -21524;
	// 82C12684: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C12688: 3B8BABD8  addi r28, r11, -0x5428
	ctx.r[28].s64 = ctx.r[11].s64 + -21544;
	// 82C1268C: 837D000C  lwz r27, 0xc(r29)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C12690: 4BFF3169  bl 0x82c057f8
	ctx.lr = 0x82C12694;
	sub_82C057F8(ctx, base);
	// 82C12694: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82C12698: 4BFF31D1  bl 0x82c05868
	ctx.lr = 0x82C1269C;
	sub_82C05868(ctx, base);
	// 82C1269C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82C126A0: 4BFF3159  bl 0x82c057f8
	ctx.lr = 0x82C126A4;
	sub_82C057F8(ctx, base);
	// 82C126A4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82C126A8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C126AC: 4BFF314D  bl 0x82c057f8
	ctx.lr = 0x82C126B0;
	sub_82C057F8(ctx, base);
	// 82C126B0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C126B4: 839F0000  lwz r28, 0(r31)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C126B8: 4BFF3409  bl 0x82c05ac0
	ctx.lr = 0x82C126BC;
	sub_82C05AC0(ctx, base);
	// 82C126BC: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82C126C0: 815C0008  lwz r10, 8(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C126C4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C126C8: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C126CC: 4E800421  bctrl
	ctx.lr = 0x82C126D0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C126D0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C126D4: 4BFF309D  bl 0x82c05770
	ctx.lr = 0x82C126D8;
	sub_82C05770(ctx, base);
	// 82C126D8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C126DC: 4BFF3055  bl 0x82c05730
	ctx.lr = 0x82C126E0;
	sub_82C05730(ctx, base);
	// 82C126E0: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 82C126E4: 3D008201  lis r8, -0x7dff
	ctx.r[8].s64 = -2113863680;
	// 82C126E8: 3889ABD0  addi r4, r9, -0x5430
	ctx.r[4].s64 = ctx.r[9].s64 + -21552;
	// 82C126EC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C126F0: 3B88ABC4  addi r28, r8, -0x543c
	ctx.r[28].s64 = ctx.r[8].s64 + -21564;
	// 82C126F4: 837D0010  lwz r27, 0x10(r29)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C126F8: 4BFF3101  bl 0x82c057f8
	ctx.lr = 0x82C126FC;
	sub_82C057F8(ctx, base);
	// 82C126FC: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82C12700: 4BFF3169  bl 0x82c05868
	ctx.lr = 0x82C12704;
	sub_82C05868(ctx, base);
	// 82C12704: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82C12708: 4BFF30F1  bl 0x82c057f8
	ctx.lr = 0x82C1270C;
	sub_82C057F8(ctx, base);
	// 82C1270C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82C12710: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C12714: 4BFF30E5  bl 0x82c057f8
	ctx.lr = 0x82C12718;
	sub_82C057F8(ctx, base);
	// 82C12718: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C1271C: 839F0000  lwz r28, 0(r31)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C12720: 4BFF33A1  bl 0x82c05ac0
	ctx.lr = 0x82C12724;
	sub_82C05AC0(ctx, base);
	// 82C12724: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82C12728: 80FC0008  lwz r7, 8(r28)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C1272C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C12730: 7CE903A6  mtctr r7
	ctx.ctr.u64 = ctx.r[7].u64;
	// 82C12734: 4E800421  bctrl
	ctx.lr = 0x82C12738;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C12738: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C1273C: 4BFF3035  bl 0x82c05770
	ctx.lr = 0x82C12740;
	sub_82C05770(ctx, base);
	// 82C12740: 80DD0014  lwz r6, 0x14(r29)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(20 as u32) ) } as u64;
	// 82C12744: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C12748: 2B060000  cmplwi cr6, r6, 0
	ctx.cr[6].compare_u32(ctx.r[6].u32, 0 as u32, &mut ctx.xer);
	// 82C1274C: 409A0060  bne cr6, 0x82c127ac
	if !ctx.cr[6].eq {
	pc = 0x82C127AC; continue 'dispatch;
	}
	// 82C12750: 4BFF2FE1  bl 0x82c05730
	ctx.lr = 0x82C12754;
	sub_82C05730(ctx, base);
	// 82C12754: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C12758: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82C1275C: 388BABB4  addi r4, r11, -0x544c
	ctx.r[4].s64 = ctx.r[11].s64 + -21580;
	// 82C12760: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C12764: 3B8AABA0  addi r28, r10, -0x5460
	ctx.r[28].s64 = ctx.r[10].s64 + -21600;
	// 82C12768: 4BFF3091  bl 0x82c057f8
	ctx.lr = 0x82C1276C;
	sub_82C057F8(ctx, base);
	// 82C1276C: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 82C12770: C0290C14  lfs f1, 0xc14(r9)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(3092 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82C12774: 4BFF322D  bl 0x82c059a0
	ctx.lr = 0x82C12778;
	sub_82C059A0(ctx, base);
	// 82C12778: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82C1277C: 4BFF307D  bl 0x82c057f8
	ctx.lr = 0x82C12780;
	sub_82C057F8(ctx, base);
	// 82C12780: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82C12784: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C12788: 4BFF3071  bl 0x82c057f8
	ctx.lr = 0x82C1278C;
	sub_82C057F8(ctx, base);
	// 82C1278C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C12790: 839F0000  lwz r28, 0(r31)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C12794: 4BFF332D  bl 0x82c05ac0
	ctx.lr = 0x82C12798;
	sub_82C05AC0(ctx, base);
	// 82C12798: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82C1279C: 811C0008  lwz r8, 8(r28)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C127A0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C127A4: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	// 82C127A8: 48000070  b 0x82c12818
	pc = 0x82C12818; continue 'dispatch;
	// 82C127AC: 4BFF2F85  bl 0x82c05730
	ctx.lr = 0x82C127B0;
	sub_82C05730(ctx, base);
	// 82C127B0: 807D0014  lwz r3, 0x14(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(20 as u32) ) } as u64;
	// 82C127B4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C127B8: 3B8BABA0  addi r28, r11, -0x5460
	ctx.r[28].s64 = ctx.r[11].s64 + -21600;
	// 82C127BC: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C127C0: 812A0014  lwz r9, 0x14(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 82C127C4: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82C127C8: 4E800421  bctrl
	ctx.lr = 0x82C127CC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C127CC: 3D008201  lis r8, -0x7dff
	ctx.r[8].s64 = -2113863680;
	// 82C127D0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C127D4: FFE00890  fmr f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82C127D8: 3888ABB4  addi r4, r8, -0x544c
	ctx.r[4].s64 = ctx.r[8].s64 + -21580;
	// 82C127DC: 4BFF301D  bl 0x82c057f8
	ctx.lr = 0x82C127E0;
	sub_82C057F8(ctx, base);
	// 82C127E0: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82C127E4: 4BFF31BD  bl 0x82c059a0
	ctx.lr = 0x82C127E8;
	sub_82C059A0(ctx, base);
	// 82C127E8: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82C127EC: 4BFF300D  bl 0x82c057f8
	ctx.lr = 0x82C127F0;
	sub_82C057F8(ctx, base);
	// 82C127F0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82C127F4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C127F8: 4BFF3001  bl 0x82c057f8
	ctx.lr = 0x82C127FC;
	sub_82C057F8(ctx, base);
	// 82C127FC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C12800: 839F0000  lwz r28, 0(r31)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C12804: 4BFF32BD  bl 0x82c05ac0
	ctx.lr = 0x82C12808;
	sub_82C05AC0(ctx, base);
	// 82C12808: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82C1280C: 80FC0008  lwz r7, 8(r28)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C12810: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C12814: 7CE903A6  mtctr r7
	ctx.ctr.u64 = ctx.r[7].u64;
	// 82C12818: 4E800421  bctrl
	ctx.lr = 0x82C1281C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C1281C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C12820: 4BFF2F51  bl 0x82c05770
	ctx.lr = 0x82C12824;
	sub_82C05770(ctx, base);
	// 82C12824: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C12828: 4BFF2F09  bl 0x82c05730
	ctx.lr = 0x82C1282C;
	sub_82C05730(ctx, base);
	// 82C1282C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C12830: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C12834: 3A6BAB90  addi r19, r11, -0x5470
	ctx.r[19].s64 = ctx.r[11].s64 + -21616;
	// 82C12838: 7E649B78  mr r4, r19
	ctx.r[4].u64 = ctx.r[19].u64;
	// 82C1283C: 4BFF2FBD  bl 0x82c057f8
	ctx.lr = 0x82C12840;
	sub_82C057F8(ctx, base);
	// 82C12840: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82C12844: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C12848: 4BFF2FB1  bl 0x82c057f8
	ctx.lr = 0x82C1284C;
	sub_82C057F8(ctx, base);
	// 82C1284C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C12850: 839F0000  lwz r28, 0(r31)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C12854: 4BFF326D  bl 0x82c05ac0
	ctx.lr = 0x82C12858;
	sub_82C05AC0(ctx, base);
	// 82C12858: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82C1285C: 817C0008  lwz r11, 8(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C12860: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C12864: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82C12868: 4E800421  bctrl
	ctx.lr = 0x82C1286C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C1286C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C12870: 4BFF2F01  bl 0x82c05770
	ctx.lr = 0x82C12874;
	sub_82C05770(ctx, base);
	// 82C12874: 815D0004  lwz r10, 4(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C12878: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C1287C: 419A001C  beq cr6, 0x82c12898
	if ctx.cr[6].eq {
	pc = 0x82C12898; continue 'dispatch;
	}
	// 82C12880: 5543003E  slwi r3, r10, 0
	ctx.r[3].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82C12884: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82C12888: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C1288C: 814B0024  lwz r10, 0x24(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 82C12890: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C12894: 4E800421  bctrl
	ctx.lr = 0x82C12898;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C12898: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C1289C: 4BFF2E95  bl 0x82c05730
	ctx.lr = 0x82C128A0;
	sub_82C05730(ctx, base);
	// 82C128A0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C128A4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C128A8: 3A8BAB80  addi r20, r11, -0x5480
	ctx.r[20].s64 = ctx.r[11].s64 + -21632;
	// 82C128AC: 7E84A378  mr r4, r20
	ctx.r[4].u64 = ctx.r[20].u64;
	// 82C128B0: 4BFF2F49  bl 0x82c057f8
	ctx.lr = 0x82C128B4;
	sub_82C057F8(ctx, base);
	// 82C128B4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82C128B8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C128BC: 4BFF2F3D  bl 0x82c057f8
	ctx.lr = 0x82C128C0;
	sub_82C057F8(ctx, base);
	// 82C128C0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C128C4: 839F0000  lwz r28, 0(r31)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C128C8: 4BFF31F9  bl 0x82c05ac0
	ctx.lr = 0x82C128CC;
	sub_82C05AC0(ctx, base);
	// 82C128CC: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82C128D0: 817C0008  lwz r11, 8(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C128D4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C128D8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82C128DC: 4E800421  bctrl
	ctx.lr = 0x82C128E0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C128E0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C128E4: 4BFF2E8D  bl 0x82c05770
	ctx.lr = 0x82C128E8;
	sub_82C05770(ctx, base);
	// 82C128E8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C128EC: 4BFF2E45  bl 0x82c05730
	ctx.lr = 0x82C128F0;
	sub_82C05730(ctx, base);
	// 82C128F0: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82C128F4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C128F8: 388AAB74  addi r4, r10, -0x548c
	ctx.r[4].s64 = ctx.r[10].s64 + -21644;
	// 82C128FC: 4BFF2EFD  bl 0x82c057f8
	ctx.lr = 0x82C12900;
	sub_82C057F8(ctx, base);
	// 82C12900: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82C12904: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C12908: 4BFF2EF1  bl 0x82c057f8
	ctx.lr = 0x82C1290C;
	sub_82C057F8(ctx, base);
	// 82C1290C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C12910: 839F0000  lwz r28, 0(r31)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C12914: 4BFF31AD  bl 0x82c05ac0
	ctx.lr = 0x82C12918;
	sub_82C05AC0(ctx, base);
	// 82C12918: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82C1291C: 813C0008  lwz r9, 8(r28)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C12920: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C12924: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82C12928: 4E800421  bctrl
	ctx.lr = 0x82C1292C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C1292C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C12930: 4BFF2E41  bl 0x82c05770
	ctx.lr = 0x82C12934;
	sub_82C05770(ctx, base);
	// 82C12934: 3B7D0024  addi r27, r29, 0x24
	ctx.r[27].s64 = ctx.r[29].s64 + 36;
	// 82C12938: 83BD0024  lwz r29, 0x24(r29)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(36 as u32) ) } as u64;
	// 82C1293C: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82C12940: 419A0264  beq cr6, 0x82c12ba4
	if ctx.cr[6].eq {
	pc = 0x82C12BA4; continue 'dispatch;
	}
	// 82C12944: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C12948: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82C1294C: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 82C12950: 3D008201  lis r8, -0x7dff
	ctx.r[8].s64 = -2113863680;
	// 82C12954: 3CE08201  lis r7, -0x7dff
	ctx.r[7].s64 = -2113863680;
	// 82C12958: 3CC08201  lis r6, -0x7dff
	ctx.r[6].s64 = -2113863680;
	// 82C1295C: 3B4BAB68  addi r26, r11, -0x5498
	ctx.r[26].s64 = ctx.r[11].s64 + -21656;
	// 82C12960: 3B2AAB5C  addi r25, r10, -0x54a4
	ctx.r[25].s64 = ctx.r[10].s64 + -21668;
	// 82C12964: 3B09AB50  addi r24, r9, -0x54b0
	ctx.r[24].s64 = ctx.r[9].s64 + -21680;
	// 82C12968: 3AE8AB48  addi r23, r8, -0x54b8
	ctx.r[23].s64 = ctx.r[8].s64 + -21688;
	// 82C1296C: 3AC7AB40  addi r22, r7, -0x54c0
	ctx.r[22].s64 = ctx.r[7].s64 + -21696;
	// 82C12970: 3AA6AB34  addi r21, r6, -0x54cc
	ctx.r[21].s64 = ctx.r[6].s64 + -21708;
	// 82C12974: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C12978: 4BFF2DB9  bl 0x82c05730
	ctx.lr = 0x82C1297C;
	sub_82C05730(ctx, base);
	// 82C1297C: 7EA4AB78  mr r4, r21
	ctx.r[4].u64 = ctx.r[21].u64;
	// 82C12980: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C12984: 4BFF2E75  bl 0x82c057f8
	ctx.lr = 0x82C12988;
	sub_82C057F8(ctx, base);
	// 82C12988: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82C1298C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C12990: 4BFF2E69  bl 0x82c057f8
	ctx.lr = 0x82C12994;
	sub_82C057F8(ctx, base);
	// 82C12994: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C12998: 839F0000  lwz r28, 0(r31)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C1299C: 4BFF3125  bl 0x82c05ac0
	ctx.lr = 0x82C129A0;
	sub_82C05AC0(ctx, base);
	// 82C129A0: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82C129A4: 817C0008  lwz r11, 8(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C129A8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C129AC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82C129B0: 4E800421  bctrl
	ctx.lr = 0x82C129B4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C129B4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82C129B8: 4BFF2DB9  bl 0x82c05770
	ctx.lr = 0x82C129BC;
	sub_82C05770(ctx, base);
	// 82C129BC: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82C129C0: 4BFF2D71  bl 0x82c05730
	ctx.lr = 0x82C129C4;
	sub_82C05730(ctx, base);
	// 82C129C4: 807D0010  lwz r3, 0x10(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C129C8: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C129CC: 812A0004  lwz r9, 4(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C129D0: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82C129D4: 4E800421  bctrl
	ctx.lr = 0x82C129D8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C129D8: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82C129DC: 7EC4B378  mr r4, r22
	ctx.r[4].u64 = ctx.r[22].u64;
	// 82C129E0: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82C129E4: 4BFF2E15  bl 0x82c057f8
	ctx.lr = 0x82C129E8;
	sub_82C057F8(ctx, base);
	// 82C129E8: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82C129EC: 4BFF2DD5  bl 0x82c057c0
	ctx.lr = 0x82C129F0;
	sub_82C057C0(ctx, base);
	// 82C129F0: 7EE4BB78  mr r4, r23
	ctx.r[4].u64 = ctx.r[23].u64;
	// 82C129F4: 4BFF2E05  bl 0x82c057f8
	ctx.lr = 0x82C129F8;
	sub_82C057F8(ctx, base);
	// 82C129F8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82C129FC: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82C12A00: 4BFF2DF9  bl 0x82c057f8
	ctx.lr = 0x82C12A04;
	sub_82C057F8(ctx, base);
	// 82C12A04: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82C12A08: 839F0000  lwz r28, 0(r31)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C12A0C: 4BFF30B5  bl 0x82c05ac0
	ctx.lr = 0x82C12A10;
	sub_82C05AC0(ctx, base);
	// 82C12A10: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82C12A14: 811C0008  lwz r8, 8(r28)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C12A18: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C12A1C: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	// 82C12A20: 4E800421  bctrl
	ctx.lr = 0x82C12A24;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C12A24: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82C12A28: 4BFF2D49  bl 0x82c05770
	ctx.lr = 0x82C12A2C;
	sub_82C05770(ctx, base);
	// 82C12A2C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82C12A30: 4BFF2D01  bl 0x82c05730
	ctx.lr = 0x82C12A34;
	sub_82C05730(ctx, base);
	// 82C12A34: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 82C12A38: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82C12A3C: 839D0014  lwz r28, 0x14(r29)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(20 as u32) ) } as u64;
	// 82C12A40: 4BFF2DB9  bl 0x82c057f8
	ctx.lr = 0x82C12A44;
	sub_82C057F8(ctx, base);
	// 82C12A44: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82C12A48: 4BFF2E21  bl 0x82c05868
	ctx.lr = 0x82C12A4C;
	sub_82C05868(ctx, base);
	// 82C12A4C: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 82C12A50: 4BFF2DA9  bl 0x82c057f8
	ctx.lr = 0x82C12A54;
	sub_82C057F8(ctx, base);
	// 82C12A54: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82C12A58: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82C12A5C: 4BFF2D9D  bl 0x82c057f8
	ctx.lr = 0x82C12A60;
	sub_82C057F8(ctx, base);
	// 82C12A60: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82C12A64: 839F0000  lwz r28, 0(r31)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C12A68: 4BFF3059  bl 0x82c05ac0
	ctx.lr = 0x82C12A6C;
	sub_82C05AC0(ctx, base);
	// 82C12A6C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82C12A70: 80FC0008  lwz r7, 8(r28)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C12A74: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C12A78: 7CE903A6  mtctr r7
	ctx.ctr.u64 = ctx.r[7].u64;
	// 82C12A7C: 4E800421  bctrl
	ctx.lr = 0x82C12A80;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C12A80: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82C12A84: 4BFF2CED  bl 0x82c05770
	ctx.lr = 0x82C12A88;
	sub_82C05770(ctx, base);
	// 82C12A88: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 82C12A8C: 4BFF2CA5  bl 0x82c05730
	ctx.lr = 0x82C12A90;
	sub_82C05730(ctx, base);
	// 82C12A90: 7E649B78  mr r4, r19
	ctx.r[4].u64 = ctx.r[19].u64;
	// 82C12A94: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 82C12A98: 4BFF2D61  bl 0x82c057f8
	ctx.lr = 0x82C12A9C;
	sub_82C057F8(ctx, base);
	// 82C12A9C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82C12AA0: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 82C12AA4: 4BFF2D55  bl 0x82c057f8
	ctx.lr = 0x82C12AA8;
	sub_82C057F8(ctx, base);
	// 82C12AA8: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 82C12AAC: 839F0000  lwz r28, 0(r31)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C12AB0: 4BFF3011  bl 0x82c05ac0
	ctx.lr = 0x82C12AB4;
	sub_82C05AC0(ctx, base);
	// 82C12AB4: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82C12AB8: 80DC0008  lwz r6, 8(r28)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C12ABC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C12AC0: 7CC903A6  mtctr r6
	ctx.ctr.u64 = ctx.r[6].u64;
	// 82C12AC4: 4E800421  bctrl
	ctx.lr = 0x82C12AC8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C12AC8: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 82C12ACC: 4BFF2CA5  bl 0x82c05770
	ctx.lr = 0x82C12AD0;
	sub_82C05770(ctx, base);
	// 82C12AD0: 807D0000  lwz r3, 0(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C12AD4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82C12AD8: 80A30000  lwz r5, 0(r3)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C12ADC: 8165000C  lwz r11, 0xc(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C12AE0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82C12AE4: 4E800421  bctrl
	ctx.lr = 0x82C12AE8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C12AE8: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82C12AEC: 4BFF2C45  bl 0x82c05730
	ctx.lr = 0x82C12AF0;
	sub_82C05730(ctx, base);
	// 82C12AF0: 7E84A378  mr r4, r20
	ctx.r[4].u64 = ctx.r[20].u64;
	// 82C12AF4: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82C12AF8: 4BFF2D01  bl 0x82c057f8
	ctx.lr = 0x82C12AFC;
	sub_82C057F8(ctx, base);
	// 82C12AFC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82C12B00: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82C12B04: 4BFF2CF5  bl 0x82c057f8
	ctx.lr = 0x82C12B08;
	sub_82C057F8(ctx, base);
	// 82C12B08: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82C12B0C: 839F0000  lwz r28, 0(r31)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C12B10: 4BFF2FB1  bl 0x82c05ac0
	ctx.lr = 0x82C12B14;
	sub_82C05AC0(ctx, base);
	// 82C12B14: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82C12B18: 815C0008  lwz r10, 8(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C12B1C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C12B20: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C12B24: 4E800421  bctrl
	ctx.lr = 0x82C12B28;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C12B28: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82C12B2C: 4BFF2C45  bl 0x82c05770
	ctx.lr = 0x82C12B30;
	sub_82C05770(ctx, base);
	// 82C12B30: 38610078  addi r3, r1, 0x78
	ctx.r[3].s64 = ctx.r[1].s64 + 120;
	// 82C12B34: 4BFF2BFD  bl 0x82c05730
	ctx.lr = 0x82C12B38;
	sub_82C05730(ctx, base);
	// 82C12B38: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82C12B3C: 38610078  addi r3, r1, 0x78
	ctx.r[3].s64 = ctx.r[1].s64 + 120;
	// 82C12B40: 4BFF2CB9  bl 0x82c057f8
	ctx.lr = 0x82C12B44;
	sub_82C057F8(ctx, base);
	// 82C12B44: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82C12B48: 38610078  addi r3, r1, 0x78
	ctx.r[3].s64 = ctx.r[1].s64 + 120;
	// 82C12B4C: 4BFF2CAD  bl 0x82c057f8
	ctx.lr = 0x82C12B50;
	sub_82C057F8(ctx, base);
	// 82C12B50: 38610078  addi r3, r1, 0x78
	ctx.r[3].s64 = ctx.r[1].s64 + 120;
	// 82C12B54: 839F0000  lwz r28, 0(r31)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C12B58: 4BFF2F69  bl 0x82c05ac0
	ctx.lr = 0x82C12B5C;
	sub_82C05AC0(ctx, base);
	// 82C12B5C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82C12B60: 813C0008  lwz r9, 8(r28)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C12B64: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C12B68: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82C12B6C: 4E800421  bctrl
	ctx.lr = 0x82C12B70;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C12B70: 38610078  addi r3, r1, 0x78
	ctx.r[3].s64 = ctx.r[1].s64 + 120;
	// 82C12B74: 4BFF2BFD  bl 0x82c05770
	ctx.lr = 0x82C12B78;
	sub_82C05770(ctx, base);
	// 82C12B78: 811D000C  lwz r8, 0xc(r29)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C12B7C: 7F08D840  cmplw cr6, r8, r27
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[27].u32, &mut ctx.xer);
	// 82C12B80: 409A0024  bne cr6, 0x82c12ba4
	if !ctx.cr[6].eq {
	pc = 0x82C12BA4; continue 'dispatch;
	}
	// 82C12B84: 83BD0004  lwz r29, 4(r29)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C12B88: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82C12B8C: 419A0018  beq cr6, 0x82c12ba4
	if ctx.cr[6].eq {
	pc = 0x82C12BA4; continue 'dispatch;
	}
	// 82C12B90: 817D000C  lwz r11, 0xc(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C12B94: 7F0BD840  cmplw cr6, r11, r27
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[27].u32, &mut ctx.xer);
	// 82C12B98: 409A000C  bne cr6, 0x82c12ba4
	if !ctx.cr[6].eq {
	pc = 0x82C12BA4; continue 'dispatch;
	}
	// 82C12B9C: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82C12BA0: 409AFDD4  bne cr6, 0x82c12974
	if !ctx.cr[6].eq {
	pc = 0x82C12974; continue 'dispatch;
	}
	// 82C12BA4: 38610078  addi r3, r1, 0x78
	ctx.r[3].s64 = ctx.r[1].s64 + 120;
	// 82C12BA8: 4BFF2B89  bl 0x82c05730
	ctx.lr = 0x82C12BAC;
	sub_82C05730(ctx, base);
	// 82C12BAC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C12BB0: 38610078  addi r3, r1, 0x78
	ctx.r[3].s64 = ctx.r[1].s64 + 120;
	// 82C12BB4: 388BAB28  addi r4, r11, -0x54d8
	ctx.r[4].s64 = ctx.r[11].s64 + -21720;
	// 82C12BB8: 4BFF2C41  bl 0x82c057f8
	ctx.lr = 0x82C12BBC;
	sub_82C057F8(ctx, base);
	// 82C12BBC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82C12BC0: 38610078  addi r3, r1, 0x78
	ctx.r[3].s64 = ctx.r[1].s64 + 120;
	// 82C12BC4: 4BFF2C35  bl 0x82c057f8
	ctx.lr = 0x82C12BC8;
	sub_82C057F8(ctx, base);
	// 82C12BC8: 38610078  addi r3, r1, 0x78
	ctx.r[3].s64 = ctx.r[1].s64 + 120;
	// 82C12BCC: 83DF0000  lwz r30, 0(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C12BD0: 4BFF2EF1  bl 0x82c05ac0
	ctx.lr = 0x82C12BD4;
	sub_82C05AC0(ctx, base);
	// 82C12BD4: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82C12BD8: 815E0008  lwz r10, 8(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C12BDC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C12BE0: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C12BE4: 4E800421  bctrl
	ctx.lr = 0x82C12BE8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C12BE8: 38610078  addi r3, r1, 0x78
	ctx.r[3].s64 = ctx.r[1].s64 + 120;
	// 82C12BEC: 4BFF2B85  bl 0x82c05770
	ctx.lr = 0x82C12BF0;
	sub_82C05770(ctx, base);
	// 82C12BF0: 38210100  addi r1, r1, 0x100
	ctx.r[1].s64 = ctx.r[1].s64 + 256;
	// 82C12BF4: CBE1FF88  lfd f31, -0x78(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-120 as u32) ) };
	// 82C12BF8: 4809683C  b 0x82ca9434
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C12C00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C12C00 size=460
    let mut pc: u32 = 0x82C12C00;
    'dispatch: loop {
        match pc {
            0x82C12C00 => {
    //   block [0x82C12C00..0x82C12DCC)
	// 82C12C00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C12C04: 480967FD  bl 0x82ca9400
	ctx.lr = 0x82C12C08;
	sub_82CA93D0(ctx, base);
	// 82C12C08: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C12C0C: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 82C12C10: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82C12C14: 3B7A0014  addi r27, r26, 0x14
	ctx.r[27].s64 = ctx.r[26].s64 + 20;
	// 82C12C18: 839A0014  lwz r28, 0x14(r26)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(20 as u32) ) } as u64;
	// 82C12C1C: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 82C12C20: 419A00EC  beq cr6, 0x82c12d0c
	if ctx.cr[6].eq {
	pc = 0x82C12D0C; continue 'dispatch;
	}
	// 82C12C24: 83DC0024  lwz r30, 0x24(r28)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(36 as u32) ) } as u64;
	// 82C12C28: 3BFC0024  addi r31, r28, 0x24
	ctx.r[31].s64 = ctx.r[28].s64 + 36;
	// 82C12C2C: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82C12C30: 419A00AC  beq cr6, 0x82c12cdc
	if ctx.cr[6].eq {
	pc = 0x82C12CDC; continue 'dispatch;
	}
	// 82C12C34: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C12C38: 393E0004  addi r9, r30, 4
	ctx.r[9].s64 = ctx.r[30].s64 + 4;
	// 82C12C3C: 7F0BF840  cmplw cr6, r11, r31
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[31].u32, &mut ctx.xer);
	// 82C12C40: 409A005C  bne cr6, 0x82c12c9c
	if !ctx.cr[6].eq {
	pc = 0x82C12C9C; continue 'dispatch;
	}
	// 82C12C44: 81490004  lwz r10, 4(r9)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C12C48: 81690000  lwz r11, 0(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C12C4C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C12C50: 409A0024  bne cr6, 0x82c12c74
	if !ctx.cr[6].eq {
	pc = 0x82C12C74; continue 'dispatch;
	}
	// 82C12C54: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C12C58: 409A0010  bne cr6, 0x82c12c68
	if !ctx.cr[6].eq {
	pc = 0x82C12C68; continue 'dispatch;
	}
	// 82C12C5C: 93BF0000  stw r29, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82C12C60: 93BF0004  stw r29, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82C12C64: 4800002C  b 0x82c12c90
	pc = 0x82C12C90; continue 'dispatch;
	// 82C12C68: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82C12C6C: 93AB0008  stw r29, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 82C12C70: 48000020  b 0x82c12c90
	pc = 0x82C12C90; continue 'dispatch;
	// 82C12C74: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C12C78: 409A0010  bne cr6, 0x82c12c88
	if !ctx.cr[6].eq {
	pc = 0x82C12C88; continue 'dispatch;
	}
	// 82C12C7C: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82C12C80: 93AA0004  stw r29, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82C12C84: 4800000C  b 0x82c12c90
	pc = 0x82C12C90; continue 'dispatch;
	// 82C12C88: 916A0004  stw r11, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82C12C8C: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82C12C90: 93A90004  stw r29, 4(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82C12C94: 93A90000  stw r29, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82C12C98: 93A90008  stw r29, 8(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 82C12C9C: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82C12CA0: 419A0030  beq cr6, 0x82c12cd0
	if ctx.cr[6].eq {
	pc = 0x82C12CD0; continue 'dispatch;
	}
	// 82C12CA4: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C12CA8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C12CAC: 419A001C  beq cr6, 0x82c12cc8
	if ctx.cr[6].eq {
	pc = 0x82C12CC8; continue 'dispatch;
	}
	// 82C12CB0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C12CB4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82C12CB8: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C12CBC: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C12CC0: 4E800421  bctrl
	ctx.lr = 0x82C12CC4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C12CC4: 93BE0000  stw r29, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82C12CC8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82C12CCC: 4BC32AE5  bl 0x828457b0
	ctx.lr = 0x82C12CD0;
	sub_828457B0(ctx, base);
	// 82C12CD0: 83DF0000  lwz r30, 0(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C12CD4: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82C12CD8: 409AFF5C  bne cr6, 0x82c12c34
	if !ctx.cr[6].eq {
	pc = 0x82C12C34; continue 'dispatch;
	}
	// 82C12CDC: 817C0040  lwz r11, 0x40(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(64 as u32) ) } as u64;
	// 82C12CE0: 7F0BD840  cmplw cr6, r11, r27
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[27].u32, &mut ctx.xer);
	// 82C12CE4: 409A0028  bne cr6, 0x82c12d0c
	if !ctx.cr[6].eq {
	pc = 0x82C12D0C; continue 'dispatch;
	}
	// 82C12CE8: 817C0038  lwz r11, 0x38(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(56 as u32) ) } as u64;
	// 82C12CEC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C12CF0: 419A0010  beq cr6, 0x82c12d00
	if ctx.cr[6].eq {
	pc = 0x82C12D00; continue 'dispatch;
	}
	// 82C12CF4: 814B0040  lwz r10, 0x40(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(64 as u32) ) } as u64;
	// 82C12CF8: 7F0AD840  cmplw cr6, r10, r27
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[27].u32, &mut ctx.xer);
	// 82C12CFC: 409A0010  bne cr6, 0x82c12d0c
	if !ctx.cr[6].eq {
	pc = 0x82C12D0C; continue 'dispatch;
	}
	// 82C12D00: 7D7C5B78  mr r28, r11
	ctx.r[28].u64 = ctx.r[11].u64;
	// 82C12D04: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C12D08: 409AFF1C  bne cr6, 0x82c12c24
	if !ctx.cr[6].eq {
	pc = 0x82C12C24; continue 'dispatch;
	}
	// 82C12D0C: 83DA000C  lwz r30, 0xc(r26)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C12D10: 3BFA000C  addi r31, r26, 0xc
	ctx.r[31].s64 = ctx.r[26].s64 + 12;
	// 82C12D14: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82C12D18: 419A00AC  beq cr6, 0x82c12dc4
	if ctx.cr[6].eq {
	pc = 0x82C12DC4; continue 'dispatch;
	}
	// 82C12D1C: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C12D20: 393E0004  addi r9, r30, 4
	ctx.r[9].s64 = ctx.r[30].s64 + 4;
	// 82C12D24: 7F0BF840  cmplw cr6, r11, r31
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[31].u32, &mut ctx.xer);
	// 82C12D28: 409A005C  bne cr6, 0x82c12d84
	if !ctx.cr[6].eq {
	pc = 0x82C12D84; continue 'dispatch;
	}
	// 82C12D2C: 81490004  lwz r10, 4(r9)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C12D30: 81690000  lwz r11, 0(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C12D34: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C12D38: 409A0024  bne cr6, 0x82c12d5c
	if !ctx.cr[6].eq {
	pc = 0x82C12D5C; continue 'dispatch;
	}
	// 82C12D3C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C12D40: 409A0010  bne cr6, 0x82c12d50
	if !ctx.cr[6].eq {
	pc = 0x82C12D50; continue 'dispatch;
	}
	// 82C12D44: 93BF0000  stw r29, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82C12D48: 93BF0004  stw r29, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82C12D4C: 4800002C  b 0x82c12d78
	pc = 0x82C12D78; continue 'dispatch;
	// 82C12D50: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82C12D54: 93AB0008  stw r29, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 82C12D58: 48000020  b 0x82c12d78
	pc = 0x82C12D78; continue 'dispatch;
	// 82C12D5C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C12D60: 409A0010  bne cr6, 0x82c12d70
	if !ctx.cr[6].eq {
	pc = 0x82C12D70; continue 'dispatch;
	}
	// 82C12D64: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82C12D68: 93AA0004  stw r29, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82C12D6C: 4800000C  b 0x82c12d78
	pc = 0x82C12D78; continue 'dispatch;
	// 82C12D70: 916A0004  stw r11, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82C12D74: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82C12D78: 93A90004  stw r29, 4(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82C12D7C: 93A90000  stw r29, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82C12D80: 93A90008  stw r29, 8(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 82C12D84: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82C12D88: 419A0030  beq cr6, 0x82c12db8
	if ctx.cr[6].eq {
	pc = 0x82C12DB8; continue 'dispatch;
	}
	// 82C12D8C: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C12D90: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C12D94: 419A001C  beq cr6, 0x82c12db0
	if ctx.cr[6].eq {
	pc = 0x82C12DB0; continue 'dispatch;
	}
	// 82C12D98: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C12D9C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82C12DA0: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C12DA4: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C12DA8: 4E800421  bctrl
	ctx.lr = 0x82C12DAC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C12DAC: 93BE0000  stw r29, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82C12DB0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82C12DB4: 4BC329FD  bl 0x828457b0
	ctx.lr = 0x82C12DB8;
	sub_828457B0(ctx, base);
	// 82C12DB8: 83DF0000  lwz r30, 0(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C12DBC: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82C12DC0: 409AFF5C  bne cr6, 0x82c12d1c
	if !ctx.cr[6].eq {
	pc = 0x82C12D1C; continue 'dispatch;
	}
	// 82C12DC4: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82C12DC8: 48096688  b 0x82ca9450
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C12DD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C12DD0 size=480
    let mut pc: u32 = 0x82C12DD0;
    'dispatch: loop {
        match pc {
            0x82C12DD0 => {
    //   block [0x82C12DD0..0x82C12FB0)
	// 82C12DD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C12DD4: 48096635  bl 0x82ca9408
	ctx.lr = 0x82C12DD8;
	sub_82CA93D0(ctx, base);
	// 82C12DD8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C12DDC: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82C12DE0: 3BDC0014  addi r30, r28, 0x14
	ctx.r[30].s64 = ctx.r[28].s64 + 20;
	// 82C12DE4: 807C0014  lwz r3, 0x14(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(20 as u32) ) } as u64;
	// 82C12DE8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C12DEC: 419A01BC  beq cr6, 0x82c12fa8
	if ctx.cr[6].eq {
	pc = 0x82C12FA8; continue 'dispatch;
	}
	// 82C12DF0: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82C12DF4: 81630040  lwz r11, 0x40(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(64 as u32) ) } as u64;
	// 82C12DF8: 38A30038  addi r5, r3, 0x38
	ctx.r[5].s64 = ctx.r[3].s64 + 56;
	// 82C12DFC: 7F0BF040  cmplw cr6, r11, r30
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82C12E00: 419A000C  beq cr6, 0x82c12e0c
	if ctx.cr[6].eq {
	pc = 0x82C12E0C; continue 'dispatch;
	}
	// 82C12E04: 7FFDFB78  mr r29, r31
	ctx.r[29].u64 = ctx.r[31].u64;
	// 82C12E08: 48000024  b 0x82c12e2c
	pc = 0x82C12E2C; continue 'dispatch;
	// 82C12E0C: 81650000  lwz r11, 0(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C12E10: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C12E14: 419A0014  beq cr6, 0x82c12e28
	if ctx.cr[6].eq {
	pc = 0x82C12E28; continue 'dispatch;
	}
	// 82C12E18: 814B0040  lwz r10, 0x40(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(64 as u32) ) } as u64;
	// 82C12E1C: 7FFDFB78  mr r29, r31
	ctx.r[29].u64 = ctx.r[31].u64;
	// 82C12E20: 7F0AF040  cmplw cr6, r10, r30
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82C12E24: 409A0008  bne cr6, 0x82c12e2c
	if !ctx.cr[6].eq {
	pc = 0x82C12E2C; continue 'dispatch;
	}
	// 82C12E28: 7D7D5B78  mr r29, r11
	ctx.r[29].u64 = ctx.r[11].u64;
	// 82C12E2C: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C12E30: 7D6A58F8  nor r10, r11, r11
	ctx.r[10].u64 = !(ctx.r[11].u64 | ctx.r[11].u64);
	// 82C12E34: 5549F7FE  rlwinm r9, r10, 0x1e, 0x1f, 0x1f
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0x00000003u64;
	// 82C12E38: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82C12E3C: 409A0160  bne cr6, 0x82c12f9c
	if !ctx.cr[6].eq {
	pc = 0x82C12F9C; continue 'dispatch;
	}
	// 82C12E40: 81030024  lwz r8, 0x24(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(36 as u32) ) } as u64;
	// 82C12E44: 38C30024  addi r6, r3, 0x24
	ctx.r[6].s64 = ctx.r[3].s64 + 36;
	// 82C12E48: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 82C12E4C: 419A00D8  beq cr6, 0x82c12f24
	if ctx.cr[6].eq {
	pc = 0x82C12F24; continue 'dispatch;
	}
	// 82C12E50: 38FC000C  addi r7, r28, 0xc
	ctx.r[7].s64 = ctx.r[28].s64 + 12;
	// 82C12E54: 8148000C  lwz r10, 0xc(r8)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C12E58: 39680004  addi r11, r8, 4
	ctx.r[11].s64 = ctx.r[8].s64 + 4;
	// 82C12E5C: 7F0A3040  cmplw cr6, r10, r6
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[6].u32, &mut ctx.xer);
	// 82C12E60: 409A005C  bne cr6, 0x82c12ebc
	if !ctx.cr[6].eq {
	pc = 0x82C12EBC; continue 'dispatch;
	}
	// 82C12E64: 812B0004  lwz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C12E68: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C12E6C: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82C12E70: 409A0024  bne cr6, 0x82c12e94
	if !ctx.cr[6].eq {
	pc = 0x82C12E94; continue 'dispatch;
	}
	// 82C12E74: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C12E78: 409A0010  bne cr6, 0x82c12e88
	if !ctx.cr[6].eq {
	pc = 0x82C12E88; continue 'dispatch;
	}
	// 82C12E7C: 93E60000  stw r31, 0(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82C12E80: 93E60004  stw r31, 4(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82C12E84: 4800002C  b 0x82c12eb0
	pc = 0x82C12EB0; continue 'dispatch;
	// 82C12E88: 91460000  stw r10, 0(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82C12E8C: 93EA0008  stw r31, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[31].u32 ) };
	// 82C12E90: 48000020  b 0x82c12eb0
	pc = 0x82C12EB0; continue 'dispatch;
	// 82C12E94: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C12E98: 409A0010  bne cr6, 0x82c12ea8
	if !ctx.cr[6].eq {
	pc = 0x82C12EA8; continue 'dispatch;
	}
	// 82C12E9C: 91260004  stw r9, 4(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82C12EA0: 93E90004  stw r31, 4(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82C12EA4: 4800000C  b 0x82c12eb0
	pc = 0x82C12EB0; continue 'dispatch;
	// 82C12EA8: 91490004  stw r10, 4(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82C12EAC: 912A0008  stw r9, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82C12EB0: 93EB0004  stw r31, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82C12EB4: 93EB0000  stw r31, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82C12EB8: 93EB0008  stw r31, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[31].u32 ) };
	// 82C12EBC: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C12EC0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C12EC4: 409A0054  bne cr6, 0x82c12f18
	if !ctx.cr[6].eq {
	pc = 0x82C12F18; continue 'dispatch;
	}
	// 82C12EC8: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C12ECC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C12ED0: 409A0048  bne cr6, 0x82c12f18
	if !ctx.cr[6].eq {
	pc = 0x82C12F18; continue 'dispatch;
	}
	// 82C12ED4: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C12ED8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C12EDC: 409A003C  bne cr6, 0x82c12f18
	if !ctx.cr[6].eq {
	pc = 0x82C12F18; continue 'dispatch;
	}
	// 82C12EE0: 90EB0008  stw r7, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[7].u32 ) };
	// 82C12EE4: 81470000  lwz r10, 0(r7)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C12EE8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C12EEC: 409A0014  bne cr6, 0x82c12f00
	if !ctx.cr[6].eq {
	pc = 0x82C12F00; continue 'dispatch;
	}
	// 82C12EF0: 93EB0004  stw r31, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82C12EF4: 93EB0000  stw r31, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82C12EF8: 91070000  stw r8, 0(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82C12EFC: 48000018  b 0x82c12f14
	pc = 0x82C12F14; continue 'dispatch;
	// 82C12F00: 81470004  lwz r10, 4(r7)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C12F04: 93EB0000  stw r31, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82C12F08: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82C12F0C: 81270004  lwz r9, 4(r7)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C12F10: 91090004  stw r8, 4(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82C12F14: 91070004  stw r8, 4(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82C12F18: 81060000  lwz r8, 0(r6)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C12F1C: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 82C12F20: 409AFF34  bne cr6, 0x82c12e54
	if !ctx.cr[6].eq {
	pc = 0x82C12E54; continue 'dispatch;
	}
	// 82C12F24: 81650008  lwz r11, 8(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C12F28: 7F0BF040  cmplw cr6, r11, r30
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82C12F2C: 409A005C  bne cr6, 0x82c12f88
	if !ctx.cr[6].eq {
	pc = 0x82C12F88; continue 'dispatch;
	}
	// 82C12F30: 81450004  lwz r10, 4(r5)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C12F34: 81650000  lwz r11, 0(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C12F38: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C12F3C: 409A0024  bne cr6, 0x82c12f60
	if !ctx.cr[6].eq {
	pc = 0x82C12F60; continue 'dispatch;
	}
	// 82C12F40: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C12F44: 409A0010  bne cr6, 0x82c12f54
	if !ctx.cr[6].eq {
	pc = 0x82C12F54; continue 'dispatch;
	}
	// 82C12F48: 93FE0000  stw r31, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82C12F4C: 93FE0004  stw r31, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82C12F50: 4800002C  b 0x82c12f7c
	pc = 0x82C12F7C; continue 'dispatch;
	// 82C12F54: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82C12F58: 93EB003C  stw r31, 0x3c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(60 as u32), ctx.r[31].u32 ) };
	// 82C12F5C: 48000020  b 0x82c12f7c
	pc = 0x82C12F7C; continue 'dispatch;
	// 82C12F60: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C12F64: 409A0010  bne cr6, 0x82c12f74
	if !ctx.cr[6].eq {
	pc = 0x82C12F74; continue 'dispatch;
	}
	// 82C12F68: 915E0004  stw r10, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82C12F6C: 93EA0038  stw r31, 0x38(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(56 as u32), ctx.r[31].u32 ) };
	// 82C12F70: 4800000C  b 0x82c12f7c
	pc = 0x82C12F7C; continue 'dispatch;
	// 82C12F74: 916A0038  stw r11, 0x38(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 82C12F78: 914B003C  stw r10, 0x3c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(60 as u32), ctx.r[10].u32 ) };
	// 82C12F7C: 93E50004  stw r31, 4(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82C12F80: 93E50000  stw r31, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82C12F84: 93E50008  stw r31, 8(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(8 as u32), ctx.r[31].u32 ) };
	// 82C12F88: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C12F8C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82C12F90: 814B0030  lwz r10, 0x30(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 82C12F94: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C12F98: 4E800421  bctrl
	ctx.lr = 0x82C12F9C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C12F9C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82C12FA0: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82C12FA4: 409AFE50  bne cr6, 0x82c12df4
	if !ctx.cr[6].eq {
	pc = 0x82C12DF4; continue 'dispatch;
	}
	// 82C12FA8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82C12FAC: 480964AC  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C12FB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C12FB0 size=116
    let mut pc: u32 = 0x82C12FB0;
    'dispatch: loop {
        match pc {
            0x82C12FB0 => {
    //   block [0x82C12FB0..0x82C13024)
	// 82C12FB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C12FB4: 48096455  bl 0x82ca9408
	ctx.lr = 0x82C12FB8;
	sub_82CA93D0(ctx, base);
	// 82C12FB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C12FBC: 83E30014  lwz r31, 0x14(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 82C12FC0: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82C12FC4: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82C12FC8: 3BC30014  addi r30, r3, 0x14
	ctx.r[30].s64 = ctx.r[3].s64 + 20;
	// 82C12FCC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82C12FD0: 419A004C  beq cr6, 0x82c1301c
	if ctx.cr[6].eq {
	pc = 0x82C1301C; continue 'dispatch;
	}
	// 82C12FD4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C12FD8: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82C12FDC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82C12FE0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C12FE4: 814B0020  lwz r10, 0x20(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82C12FE8: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C12FEC: 4E800421  bctrl
	ctx.lr = 0x82C12FF0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C12FF0: 813F0040  lwz r9, 0x40(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as u64;
	// 82C12FF4: 7F09F040  cmplw cr6, r9, r30
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82C12FF8: 409A0024  bne cr6, 0x82c1301c
	if !ctx.cr[6].eq {
	pc = 0x82C1301C; continue 'dispatch;
	}
	// 82C12FFC: 83FF0038  lwz r31, 0x38(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 82C13000: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82C13004: 419A0018  beq cr6, 0x82c1301c
	if ctx.cr[6].eq {
	pc = 0x82C1301C; continue 'dispatch;
	}
	// 82C13008: 817F0040  lwz r11, 0x40(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as u64;
	// 82C1300C: 7F0BF040  cmplw cr6, r11, r30
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82C13010: 409A000C  bne cr6, 0x82c1301c
	if !ctx.cr[6].eq {
	pc = 0x82C1301C; continue 'dispatch;
	}
	// 82C13014: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82C13018: 409AFFBC  bne cr6, 0x82c12fd4
	if !ctx.cr[6].eq {
	pc = 0x82C12FD4; continue 'dispatch;
	}
	// 82C1301C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82C13020: 48096438  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C13028(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C13028 size=12
    let mut pc: u32 = 0x82C13028;
    'dispatch: loop {
        match pc {
            0x82C13028 => {
    //   block [0x82C13028..0x82C13034)
	// 82C13028: 8163000C  lwz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C1302C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C13030: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C13034(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C13034 size=8
    let mut pc: u32 = 0x82C13034;
    'dispatch: loop {
        match pc {
            0x82C13034 => {
    //   block [0x82C13034..0x82C1303C)
	// 82C13034: 908B00A0  stw r4, 0xa0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(160 as u32), ctx.r[4].u32 ) };
	// 82C13038: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C13040(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C13040 size=12
    let mut pc: u32 = 0x82C13040;
    'dispatch: loop {
        match pc {
            0x82C13040 => {
    //   block [0x82C13040..0x82C1304C)
	// 82C13040: 8163000C  lwz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82C13044: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82C13048: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C1304C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C1304C size=12
    let mut pc: u32 = 0x82C1304C;
    'dispatch: loop {
        match pc {
            0x82C1304C => {
    //   block [0x82C1304C..0x82C13058)
	// 82C1304C: 814B00A4  lwz r10, 0xa4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(164 as u32) ) } as u64;
	// 82C13050: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C13054: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C13058(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C13058 size=8
    let mut pc: u32 = 0x82C13058;
    'dispatch: loop {
        match pc {
            0x82C13058 => {
    //   block [0x82C13058..0x82C13060)
	// 82C13058: 908B00A4  stw r4, 0xa4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(164 as u32), ctx.r[4].u32 ) };
	// 82C1305C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C13060(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82C13060 size=24
    let mut pc: u32 = 0x82C13060;
    'dispatch: loop {
        match pc {
            0x82C13060 => {
    //   block [0x82C13060..0x82C13078)
	// 82C13060: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C13064: 39400250  li r10, 0x250
	ctx.r[10].s64 = 592;
	// 82C13068: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82C1306C: 13EB50C7  vcmpequd (lvx128) v31, v11, v10
	tmp.u32 = ctx.r[11].u32 + ctx.r[10].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C13078(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82C13078 size=56
    let mut pc: u32 = 0x82C13078;
    'dispatch: loop {
        match pc {
            0x82C13078 => {
    //   block [0x82C13078..0x82C130B0)
	// 82C13078: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C1307C: 39400250  li r10, 0x250
	ctx.r[10].s64 = 592;
	// 82C13080: 39200260  li r9, 0x260
	ctx.r[9].s64 = 608;
	// 82C13084: 39000270  li r8, 0x270
	ctx.r[8].s64 = 624;
	// 82C13088: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82C1308C: 13EB50C7  vcmpequd (lvx128) v31, v11, v10
	tmp.u32 = ctx.r[11].u32 + ctx.r[10].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C130B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82C130B0 size=224
    let mut pc: u32 = 0x82C130B0;
    'dispatch: loop {
        match pc {
            0x82C130B0 => {
    //   block [0x82C130B0..0x82C13190)
	// 82C130B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C130B4: 48096359  bl 0x82ca940c
	ctx.lr = 0x82C130B8;
	sub_82CA93D0(ctx, base);
	// 82C130B8: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82C130BC: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82C130C0: 3921FFD0  addi r9, r1, -0x30
	ctx.r[9].s64 = ctx.r[1].s64 + -48;
	// 82C130C4: 3901FFD4  addi r8, r1, -0x2c
	ctx.r[8].s64 = ctx.r[1].s64 + -44;
	// 82C130C8: 38E1FFD4  addi r7, r1, -0x2c
	ctx.r[7].s64 = ctx.r[1].s64 + -44;
	// 82C130CC: C00B0C18  lfs f0, 0xc18(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3096 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C130D0: 3BE1FFD8  addi r31, r1, -0x28
	ctx.r[31].s64 = ctx.r[1].s64 + -40;
	// 82C130D4: D001FFD0  stfs f0, -0x30(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), tmp.u32 ) };
	// 82C130D8: 3BC1FFDC  addi r30, r1, -0x24
	ctx.r[30].s64 = ctx.r[1].s64 + -36;
	// 82C130DC: D001FFD4  stfs f0, -0x2c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-44 as u32), tmp.u32 ) };
	// 82C130E0: 3BA1FFDC  addi r29, r1, -0x24
	ctx.r[29].s64 = ctx.r[1].s64 + -36;
	// 82C130E4: 13E04C07  vcmpneb. (lvlx128) v31, v0, v9
	tmp.u32 = ctx.r[9].u32;
	// load shuffled into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82C130E8: 3921FFDC  addi r9, r1, -0x24
	ctx.r[9].s64 = ctx.r[1].s64 + -36;
	// 82C130EC: 13C04407  vcmpneb. (lvlx128) v30, v0, v8
	tmp.u32 = ctx.r[8].u32;
	// load shuffled into ctx.v[62] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82C130F0: 3901FFD8  addi r8, r1, -0x28
	ctx.r[8].s64 = ctx.r[1].s64 + -40;
	// 82C130F4: C1AA0C14  lfs f13, 0xc14(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(3092 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82C130F8: 3861FFD0  addi r3, r1, -0x30
	ctx.r[3].s64 = ctx.r[1].s64 + -48;
	// 82C130FC: D1A1FFD4  stfs f13, -0x2c(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-44 as u32), tmp.u32 ) };
	// 82C13100: 3961FFD4  addi r11, r1, -0x2c
	ctx.r[11].s64 = ctx.r[1].s64 + -44;
	// 82C13104: 13A03C07  vcmpneb. (lvlx128) v29, v0, v7
	tmp.u32 = ctx.r[7].u32;
	// load shuffled into ctx.v[61] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82C13108: 38E1FFD8  addi r7, r1, -0x28
	ctx.r[7].s64 = ctx.r[1].s64 + -40;
	// 82C1310C: D001FFD8  stfs f0, -0x28(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), tmp.u32 ) };
	// 82C13110: 3941FFD0  addi r10, r1, -0x30
	ctx.r[10].s64 = ctx.r[1].s64 + -48;
	// 82C13114: D1A1FFDC  stfs f13, -0x24(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-36 as u32), tmp.u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C13190(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82C13190 size=28
    let mut pc: u32 = 0x82C13190;
    'dispatch: loop {
        match pc {
            0x82C13190 => {
    //   block [0x82C13190..0x82C131AC)
	// 82C13190: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C13194: 81430004  lwz r10, 4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C13198: 7D6B2050  subf r11, r11, r4
	ctx.r[11].s64 = ctx.r[4].s64 - ctx.r[11].s64;
	// 82C1319C: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82C131A0: 4198000C  blt cr6, 0x82c131ac
	if ctx.cr[6].lt {
		sub_82C131AC(ctx, base);
		return;
	}
	// 82C131A4: C003000C  lfs f0, 0xc(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C131A8: 48000040  b 0x82c131e8
	sub_82C131AC(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C131AC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82C131AC size=100
    let mut pc: u32 = 0x82C131AC;
    'dispatch: loop {
        match pc {
            0x82C131AC => {
    //   block [0x82C131AC..0x82C13210)
	// 82C131AC: 794A0020  clrldi r10, r10, 0x20
	ctx.r[10].u64 = ctx.r[10].u64 & 0x00000000FFFFFFFFu64;
	// 82C131B0: C0030008  lfs f0, 8(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C131B4: 79690020  clrldi r9, r11, 0x20
	ctx.r[9].u64 = ctx.r[11].u64 & 0x00000000FFFFFFFFu64;
	// 82C131B8: C1A3000C  lfs f13, 0xc(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82C131BC: F941FFF0  std r10, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[10].u64 ) };
	// 82C131C0: C981FFF0  lfd f12, -0x10(r1)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C131C4: F921FFF0  std r9, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[9].u64 ) };
	// 82C131C8: C961FFF0  lfd f11, -0x10(r1)
	ctx.f[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C131CC: FD20669C  fcfid f9, f12
	ctx.f[9].f64 = (ctx.f[12].s64 as f64);
	// 82C131D0: FD405E9C  fcfid f10, f11
	ctx.f[10].f64 = (ctx.f[11].s64 as f64);
	// 82C131D4: FCC04818  frsp f6, f9
	ctx.f[6].f64 = (ctx.f[9].f64 as f32) as f64;
	// 82C131D8: ECED0028  fsubs f7, f13, f0
	ctx.f[7].f64 = (((ctx.f[13].f64 - ctx.f[0].f64) as f32) as f64);
	// 82C131DC: FD005018  frsp f8, f10
	ctx.f[8].f64 = (ctx.f[10].f64 as f32) as f64;
	// 82C131E0: ECA83024  fdivs f5, f8, f6
	ctx.f[5].f64 = ((ctx.f[8].f64 / ctx.f[6].f64) as f32) as f64;
	// 82C131E4: EC0501FA  fmadds f0, f5, f7, f0
	ctx.f[0].f64 = (((ctx.f[5].f64 * ctx.f[7].f64 + ctx.f[0].f64) as f32) as f64);
	// 82C131E8: D0030008  stfs f0, 8(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82C131EC: 90C30004  stw r6, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[6].u32 ) };
	// 82C131F0: C0050000  lfs f0, 0(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C131F4: 2B060000  cmplwi cr6, r6, 0
	ctx.cr[6].compare_u32(ctx.r[6].u32, 0 as u32, &mut ctx.xer);
	// 82C131F8: D003000C  stfs f0, 0xc(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 82C131FC: 409A000C  bne cr6, 0x82c13208
	if !ctx.cr[6].eq {
	pc = 0x82C13208; continue 'dispatch;
	}
	// 82C13200: C0050000  lfs f0, 0(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82C13204: D0030008  stfs f0, 8(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82C13208: 90830000  stw r4, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 82C1320C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C13210(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C13210 size=12
    let mut pc: u32 = 0x82C13210;
    'dispatch: loop {
        match pc {
            0x82C13210 => {
    //   block [0x82C13210..0x82C1321C)
	// 82C13210: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82C13214: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82C13218: 4BFFF1C0  b 0x82c123d8
	sub_82C123D8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C13220(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C13220 size=16
    let mut pc: u32 = 0x82C13220;
    'dispatch: loop {
        match pc {
            0x82C13220 => {
    //   block [0x82C13220..0x82C13230)
	// 82C13220: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C13224: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82C13228: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82C1322C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C13230(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C13230 size=16
    let mut pc: u32 = 0x82C13230;
    'dispatch: loop {
        match pc {
            0x82C13230 => {
    //   block [0x82C13230..0x82C13240)
	// 82C13230: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C13234: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82C13238: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82C1323C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C13240(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C13240 size=16
    let mut pc: u32 = 0x82C13240;
    'dispatch: loop {
        match pc {
            0x82C13240 => {
    //   block [0x82C13240..0x82C13250)
	// 82C13240: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C13244: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82C13248: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82C1324C: 4C820020  bnelr
	if !ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C13250(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C13250 size=12
    let mut pc: u32 = 0x82C13250;
    'dispatch: loop {
        match pc {
            0x82C13250 => {
    //   block [0x82C13250..0x82C1325C)
	// 82C13250: 80630004  lwz r3, 4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C13254: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C13258: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C1325C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C1325C size=20
    let mut pc: u32 = 0x82C1325C;
    'dispatch: loop {
        match pc {
            0x82C1325C => {
    //   block [0x82C1325C..0x82C13270)
	// 82C1325C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C13260: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82C13264: 814B0018  lwz r10, 0x18(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82C13268: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C1326C: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C13270(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C13270 size=4
    let mut pc: u32 = 0x82C13270;
    'dispatch: loop {
        match pc {
            0x82C13270 => {
    //   block [0x82C13270..0x82C13274)
	// 82C13270: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C13278(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C13278 size=16
    let mut pc: u32 = 0x82C13278;
    'dispatch: loop {
        match pc {
            0x82C13278 => {
    //   block [0x82C13278..0x82C13288)
	// 82C13278: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C1327C: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82C13280: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82C13284: 4C820020  bnelr
	if !ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C13288(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C13288 size=12
    let mut pc: u32 = 0x82C13288;
    'dispatch: loop {
        match pc {
            0x82C13288 => {
    //   block [0x82C13288..0x82C13294)
	// 82C13288: 80630004  lwz r3, 4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C1328C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C13290: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C13294(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C13294 size=20
    let mut pc: u32 = 0x82C13294;
    'dispatch: loop {
        match pc {
            0x82C13294 => {
    //   block [0x82C13294..0x82C132A8)
	// 82C13294: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C13298: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82C1329C: 814B0014  lwz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82C132A0: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C132A4: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C132A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C132A8 size=4
    let mut pc: u32 = 0x82C132A8;
    'dispatch: loop {
        match pc {
            0x82C132A8 => {
    //   block [0x82C132A8..0x82C132AC)
	// 82C132A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C132B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C132B0 size=16
    let mut pc: u32 = 0x82C132B0;
    'dispatch: loop {
        match pc {
            0x82C132B0 => {
    //   block [0x82C132B0..0x82C132C0)
	// 82C132B0: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C132B4: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82C132B8: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82C132BC: 4C820020  bnelr
	if !ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C132C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C132C0 size=20
    let mut pc: u32 = 0x82C132C0;
    'dispatch: loop {
        match pc {
            0x82C132C0 => {
    //   block [0x82C132C0..0x82C132D4)
	// 82C132C0: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C132C4: 814B0010  lwz r10, 0x10(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82C132C8: 61490002  ori r9, r10, 2
	ctx.r[9].u64 = ctx.r[10].u64 | 2;
	// 82C132CC: 912B0010  stw r9, 0x10(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[9].u32 ) };
	// 82C132D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C132D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C132D8 size=72
    let mut pc: u32 = 0x82C132D8;
    'dispatch: loop {
        match pc {
            0x82C132D8 => {
    //   block [0x82C132D8..0x82C13320)
	// 82C132D8: 81640008  lwz r11, 8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 82C132DC: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C132E0: 91440008  stw r10, 8(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82C132E4: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C132E8: 892A0029  lbz r9, 0x29(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(41 as u32) ) } as u64;
	// 82C132EC: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82C132F0: 409A0008  bne cr6, 0x82c132f8
	if !ctx.cr[6].eq {
	pc = 0x82C132F8; continue 'dispatch;
	}
	// 82C132F4: 908A0004  stw r4, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 82C132F8: 81440004  lwz r10, 4(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C132FC: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82C13300: 81430004  lwz r10, 4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C13304: 812A0004  lwz r9, 4(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C13308: 7F044840  cmplw cr6, r4, r9
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82C1330C: 409A0014  bne cr6, 0x82c13320
	if !ctx.cr[6].eq {
		sub_82C13320(ctx, base);
		return;
	}
	// 82C13310: 916A0004  stw r11, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82C13314: 908B0000  stw r4, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 82C13318: 91640004  stw r11, 4(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82C1331C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C13320(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C13320 size=32
    let mut pc: u32 = 0x82C13320;
    'dispatch: loop {
        match pc {
            0x82C13320 => {
    //   block [0x82C13320..0x82C13340)
	// 82C13320: 81440004  lwz r10, 4(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C13324: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C13328: 7F044840  cmplw cr6, r4, r9
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82C1332C: 409A0014  bne cr6, 0x82c13340
	if !ctx.cr[6].eq {
		sub_82C13340(ctx, base);
		return;
	}
	// 82C13330: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82C13334: 908B0000  stw r4, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 82C13338: 91640004  stw r11, 4(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82C1333C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C13340(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82C13340 size=16
    let mut pc: u32 = 0x82C13340;
    'dispatch: loop {
        match pc {
            0x82C13340 => {
    //   block [0x82C13340..0x82C13350)
	// 82C13340: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82C13344: 908B0000  stw r4, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 82C13348: 91640004  stw r11, 4(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82C1334C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C13350(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C13350 size=96
    let mut pc: u32 = 0x82C13350;
    'dispatch: loop {
        match pc {
            0x82C13350 => {
    //   block [0x82C13350..0x82C133B0)
	// 82C13350: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C13354: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C13358: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C1335C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C13360: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82C13364: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C13368: 4BFFEF39  bl 0x82c122a0
	ctx.lr = 0x82C1336C;
	sub_82C122A0(ctx, base);
	// 82C1336C: 89410050  lbz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82C13370: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82C13374: 409A001C  bne cr6, 0x82c13390
	if !ctx.cr[6].eq {
	pc = 0x82C13390; continue 'dispatch;
	}
	// 82C13378: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82C1337C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C13380: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C13384: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C13388: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C1338C: 4E800020  blr
	return;
	// 82C13390: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C13394: 546A103A  slwi r10, r3, 2
	ctx.r[10].u32 = ctx.r[3].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82C13398: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82C1339C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C133A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C133A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C133A8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C133AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C133B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C133B0 size=80
    let mut pc: u32 = 0x82C133B0;
    'dispatch: loop {
        match pc {
            0x82C133B0 => {
    //   block [0x82C133B0..0x82C13400)
	// 82C133B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C133B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C133B8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82C133BC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C133C0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C133C4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C133C8: 4B5E4439  bl 0x821f7800
	ctx.lr = 0x82C133CC;
	sub_821F7800(ctx, base);
	// 82C133CC: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82C133D0: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 82C133D4: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82C133D8: 4803A6B9  bl 0x82c4da90
	ctx.lr = 0x82C133DC;
	sub_82C4DA90(ctx, base);
	// 82C133DC: 93DF001C  stw r30, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[30].u32 ) };
	// 82C133E0: 93DF0020  stw r30, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[30].u32 ) };
	// 82C133E4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C133E8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C133EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C133F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C133F4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82C133F8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C133FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C13400(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C13400 size=124
    let mut pc: u32 = 0x82C13400;
    'dispatch: loop {
        match pc {
            0x82C13400 => {
    //   block [0x82C13400..0x82C1347C)
	// 82C13400: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C13404: 48095FFD  bl 0x82ca9400
	ctx.lr = 0x82C13408;
	sub_82CA93D0(ctx, base);
	// 82C13408: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C1340C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82C13410: 38600020  li r3, 0x20
	ctx.r[3].s64 = 32;
	// 82C13414: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82C13418: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82C1341C: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 82C13420: 4B60BE39  bl 0x8221f258
	ctx.lr = 0x82C13424;
	sub_8221F258(ctx, base);
	// 82C13424: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C13428: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82C1342C: 419A0020  beq cr6, 0x82c1344c
	if ctx.cr[6].eq {
	pc = 0x82C1344C; continue 'dispatch;
	}
	// 82C13430: 4803A661  bl 0x82c4da90
	ctx.lr = 0x82C13434;
	sub_82C4DA90(ctx, base);
	// 82C13434: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 82C13438: 387F0018  addi r3, r31, 0x18
	ctx.r[3].s64 = ctx.r[31].s64 + 24;
	// 82C1343C: 935F0014  stw r26, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[26].u32 ) };
	// 82C13440: 4B5E43C1  bl 0x821f7800
	ctx.lr = 0x82C13444;
	sub_821F7800(ctx, base);
	// 82C13444: 935F001C  stw r26, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[26].u32 ) };
	// 82C13448: 48000008  b 0x82c13450
	pc = 0x82C13450; continue 'dispatch;
	// 82C1344C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82C13450: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82C13454: 387F0018  addi r3, r31, 0x18
	ctx.r[3].s64 = ctx.r[31].s64 + 24;
	// 82C13458: 4B651D49  bl 0x822651a0
	ctx.lr = 0x82C1345C;
	sub_822651A0(ctx, base);
	// 82C1345C: 939F001C  stw r28, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[28].u32 ) };
	// 82C13460: 937F0014  stw r27, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[27].u32 ) };
	// 82C13464: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82C13468: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82C1346C: 387E0004  addi r3, r30, 4
	ctx.r[3].s64 = ctx.r[30].s64 + 4;
	// 82C13470: 48039E51  bl 0x82c4d2c0
	ctx.lr = 0x82C13474;
	sub_82C4D2C0(ctx, base);
	// 82C13474: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82C13478: 48095FD8  b 0x82ca9450
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C13480(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C13480 size=96
    let mut pc: u32 = 0x82C13480;
    'dispatch: loop {
        match pc {
            0x82C13480 => {
    //   block [0x82C13480..0x82C134E0)
	// 82C13480: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C13484: 48095F89  bl 0x82ca940c
	ctx.lr = 0x82C13488;
	sub_82CA93D0(ctx, base);
	// 82C13488: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C1348C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82C13490: 3BBE0004  addi r29, r30, 4
	ctx.r[29].s64 = ctx.r[30].s64 + 4;
	// 82C13494: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82C13498: 48039B21  bl 0x82c4cfb8
	ctx.lr = 0x82C1349C;
	sub_82C4CFB8(ctx, base);
	// 82C1349C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C134A0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82C134A4: 419A0034  beq cr6, 0x82c134d8
	if ctx.cr[6].eq {
	pc = 0x82C134D8; continue 'dispatch;
	}
	// 82C134A8: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82C134AC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C134B0: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C134B4: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C134B8: 4E800421  bctrl
	ctx.lr = 0x82C134BC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C134BC: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82C134C0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82C134C4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82C134C8: 4803A081  bl 0x82c4d548
	ctx.lr = 0x82C134CC;
	sub_82C4D548(ctx, base);
	// 82C134CC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C134D0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82C134D4: 409AFFD4  bne cr6, 0x82c134a8
	if !ctx.cr[6].eq {
	pc = 0x82C134A8; continue 'dispatch;
	}
	// 82C134D8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82C134DC: 48095F80  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C134E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C134E0 size=156
    let mut pc: u32 = 0x82C134E0;
    'dispatch: loop {
        match pc {
            0x82C134E0 => {
    //   block [0x82C134E0..0x82C1357C)
	// 82C134E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C134E4: 48095F25  bl 0x82ca9408
	ctx.lr = 0x82C134E8;
	sub_82CA93D0(ctx, base);
	// 82C134E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C134EC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82C134F0: 3BDD0004  addi r30, r29, 4
	ctx.r[30].s64 = ctx.r[29].s64 + 4;
	// 82C134F4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82C134F8: 48039AC1  bl 0x82c4cfb8
	ctx.lr = 0x82C134FC;
	sub_82C4CFB8(ctx, base);
	// 82C134FC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C13500: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82C13504: 419A0070  beq cr6, 0x82c13574
	if ctx.cr[6].eq {
	pc = 0x82C13574; continue 'dispatch;
	}
	// 82C13508: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82C1350C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82C13510: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82C13514: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82C13518: 4803A521  bl 0x82c4da38
	ctx.lr = 0x82C1351C;
	sub_82C4DA38(ctx, base);
	// 82C1351C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82C13520: 419A0040  beq cr6, 0x82c13560
	if ctx.cr[6].eq {
	pc = 0x82C13560; continue 'dispatch;
	}
	// 82C13524: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82C13528: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C1352C: 419A001C  beq cr6, 0x82c13548
	if ctx.cr[6].eq {
	pc = 0x82C13548; continue 'dispatch;
	}
	// 82C13530: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C13534: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82C13538: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82C1353C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C13540: 4E800421  bctrl
	ctx.lr = 0x82C13544;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C13544: 939F0014  stw r28, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[28].u32 ) };
	// 82C13548: 387F0018  addi r3, r31, 0x18
	ctx.r[3].s64 = ctx.r[31].s64 + 24;
	// 82C1354C: 4B60188D  bl 0x82214dd8
	ctx.lr = 0x82C13550;
	sub_82214DD8(ctx, base);
	// 82C13550: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C13554: 4BF83925  bl 0x82b96e78
	ctx.lr = 0x82C13558;
	sub_82B96E78(ctx, base);
	// 82C13558: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C1355C: 4BC32255  bl 0x828457b0
	ctx.lr = 0x82C13560;
	sub_828457B0(ctx, base);
	// 82C13560: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82C13564: 48039A55  bl 0x82c4cfb8
	ctx.lr = 0x82C13568;
	sub_82C4CFB8(ctx, base);
	// 82C13568: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C1356C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82C13570: 409AFF9C  bne cr6, 0x82c1350c
	if !ctx.cr[6].eq {
	pc = 0x82C1350C; continue 'dispatch;
	}
	// 82C13574: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82C13578: 48095EE0  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C13580(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C13580 size=160
    let mut pc: u32 = 0x82C13580;
    'dispatch: loop {
        match pc {
            0x82C13580 => {
    //   block [0x82C13580..0x82C13620)
	// 82C13580: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C13584: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C13588: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C1358C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C13590: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C13594: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82C13598: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 82C1359C: 394BACA8  addi r10, r11, -0x5358
	ctx.r[10].s64 = ctx.r[11].s64 + -21336;
	// 82C135A0: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82C135A4: 4B5C2965  bl 0x821d5f08
	ctx.lr = 0x82C135A8;
	sub_821D5F08(ctx, base);
	// 82C135A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82C135AC: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 82C135B0: 917F0048  stw r11, 0x48(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[11].u32 ) };
	// 82C135B4: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82C135B8: 917F004C  stw r11, 0x4c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(76 as u32), ctx.r[11].u32 ) };
	// 82C135BC: 38E9AC90  addi r7, r9, -0x5370
	ctx.r[7].s64 = ctx.r[9].s64 + -21360;
	// 82C135C0: 917F0050  stw r11, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82C135C4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C135C8: 917F0054  stw r11, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82C135CC: 917F0058  stw r11, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 82C135D0: 917F005C  stw r11, 0x5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82C135D4: 917F0060  stw r11, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 82C135D8: 917F0064  stw r11, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82C135DC: 917F0068  stw r11, 0x68(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 82C135E0: 90FF006C  stw r7, 0x6c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(108 as u32), ctx.r[7].u32 ) };
	// 82C135E4: 917F0070  stw r11, 0x70(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(112 as u32), ctx.r[11].u32 ) };
	// 82C135E8: 917F0074  stw r11, 0x74(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82C135EC: 917F0078  stw r11, 0x78(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(120 as u32), ctx.r[11].u32 ) };
	// 82C135F0: 917F007C  stw r11, 0x7c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(124 as u32), ctx.r[11].u32 ) };
	// 82C135F4: 917F0080  stw r11, 0x80(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 82C135F8: 917F0084  stw r11, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[11].u32 ) };
	// 82C135FC: 917F00A0  stw r11, 0xa0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(160 as u32), ctx.r[11].u32 ) };
	// 82C13600: 917F00A4  stw r11, 0xa4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(164 as u32), ctx.r[11].u32 ) };
	// 82C13604: 991F00A8  stb r8, 0xa8(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(168 as u32), ctx.r[8].u8 ) };
	// 82C13608: 93FF0070  stw r31, 0x70(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(112 as u32), ctx.r[31].u32 ) };
	// 82C1360C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82C13610: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C13614: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C13618: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C1361C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82C13620(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82C13620 size=84
    let mut pc: u32 = 0x82C13620;
    'dispatch: loop {
        match pc {
            0x82C13620 => {
    //   block [0x82C13620..0x82C13674)
	// 82C13620: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82C13624: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82C13628: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82C1362C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82C13630: 81640078  lwz r11, 0x78(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(120 as u32) ) } as u64;
	// 82C13634: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82C13638: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82C1363C: 8064007C  lwz r3, 0x7c(r4)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(124 as u32) ) } as u64;
	// 82C13640: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82C13644: 907F0004  stw r3, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 82C13648: 419A0014  beq cr6, 0x82c1365c
	if ctx.cr[6].eq {
	pc = 0x82C1365C; continue 'dispatch;
	}
	// 82C1364C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C13650: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82C13654: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82C13658: 4E800421  bctrl
	ctx.lr = 0x82C1365C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82C1365C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82C13660: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82C13664: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82C13668: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82C1366C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82C13670: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


