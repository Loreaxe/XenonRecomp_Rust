pub fn sub_82DDEBB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DDEBB0 size=108
    let mut pc: u32 = 0x82DDEBB0;
    'dispatch: loop {
        match pc {
            0x82DDEBB0 => {
    //   block [0x82DDEBB0..0x82DDEC04)
	// 82DDEBB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DDEBB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DDEBB8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DDEBBC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DDEBC0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DDEBC4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DDEBC8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DDEBCC: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DDEBD0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDEBD4: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 82DDEBD8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DDEBDC: 4E800421  bctrl
	ctx.lr = 0x82DDEBE0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DDEBE0: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DDEBE4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DDEBE8: 419A001C  beq cr6, 0x82ddec04
	if ctx.cr[6].eq {
	pc = 0x82DDEC04; continue 'dispatch;
	}
	// 82DDEBEC: 5563003E  slwi r3, r11, 0
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82DDEBF0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82DDEBF4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDEBF8: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 82DDEBFC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DDEC00: 4E800421  bctrl
	ctx.lr = 0x82DDEC04;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x82DDEC04 => {
    //   block [0x82DDEC04..0x82DDEC1C)
	// 82DDEC04: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DDEC08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DDEC0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DDEC10: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DDEC14: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DDEC18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDEC20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DDEC20 size=140
    let mut pc: u32 = 0x82DDEC20;
    'dispatch: loop {
        match pc {
            0x82DDEC20 => {
    //   block [0x82DDEC20..0x82DDEC8C)
	// 82DDEC20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DDEC24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DDEC28: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DDEC2C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DDEC30: DBC1FFD8  stfd f30, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[30].u64 ) };
	// 82DDEC34: DBE1FFE0  stfd f31, -0x20(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 82DDEC38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DDEC3C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DDEC40: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82DDEC44: FFC01090  fmr f30, f2
	ctx.f[30].f64 = ctx.f[2].f64;
	// 82DDEC48: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82DDEC4C: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DDEC50: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDEC54: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 82DDEC58: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DDEC5C: 4E800421  bctrl
	ctx.lr = 0x82DDEC60;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DDEC60: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DDEC64: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DDEC68: 419A0024  beq cr6, 0x82ddec8c
	if ctx.cr[6].eq {
	pc = 0x82DDEC8C; continue 'dispatch;
	}
	// 82DDEC6C: 5563003E  slwi r3, r11, 0
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82DDEC70: FC40F090  fmr f2, f30
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[2].f64 = ctx.f[30].f64;
	// 82DDEC74: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82DDEC78: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82DDEC7C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDEC80: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 82DDEC84: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DDEC88: 4E800421  bctrl
	ctx.lr = 0x82DDEC8C;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x82DDEC8C => {
    //   block [0x82DDEC8C..0x82DDECAC)
	// 82DDEC8C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DDEC90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DDEC94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DDEC98: CBC1FFD8  lfd f30, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-40 as u32) ) };
	// 82DDEC9C: CBE1FFE0  lfd f31, -0x20(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 82DDECA0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DDECA4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DDECA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDECB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DDECB0 size=32
    let mut pc: u32 = 0x82DDECB0;
    'dispatch: loop {
        match pc {
            0x82DDECB0 => {
    //   block [0x82DDECB0..0x82DDECD0)
	// 82DDECB0: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DDECB4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DDECB8: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 82DDECBC: 5563003E  slwi r3, r11, 0
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82DDECC0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDECC4: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 82DDECC8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DDECCC: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDECD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DDECD0 size=4
    let mut pc: u32 = 0x82DDECD0;
    'dispatch: loop {
        match pc {
            0x82DDECD0 => {
    //   block [0x82DDECD0..0x82DDECD4)
	// 82DDECD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDECD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DDECD8 size=32
    let mut pc: u32 = 0x82DDECD8;
    'dispatch: loop {
        match pc {
            0x82DDECD8 => {
    //   block [0x82DDECD8..0x82DDECF8)
	// 82DDECD8: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DDECDC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DDECE0: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 82DDECE4: 5563003E  slwi r3, r11, 0
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82DDECE8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDECEC: 816B0030  lwz r11, 0x30(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 82DDECF0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DDECF4: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDECF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DDECF8 size=4
    let mut pc: u32 = 0x82DDECF8;
    'dispatch: loop {
        match pc {
            0x82DDECF8 => {
    //   block [0x82DDECF8..0x82DDECFC)
	// 82DDECF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDED00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DDED00 size=32
    let mut pc: u32 = 0x82DDED00;
    'dispatch: loop {
        match pc {
            0x82DDED00 => {
    //   block [0x82DDED00..0x82DDED20)
	// 82DDED00: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DDED04: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DDED08: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 82DDED0C: 5563003E  slwi r3, r11, 0
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82DDED10: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDED14: 816B0034  lwz r11, 0x34(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 82DDED18: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DDED1C: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDED20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DDED20 size=4
    let mut pc: u32 = 0x82DDED20;
    'dispatch: loop {
        match pc {
            0x82DDED20 => {
    //   block [0x82DDED20..0x82DDED24)
	// 82DDED20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDED28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DDED28 size=156
    let mut pc: u32 = 0x82DDED28;
    'dispatch: loop {
        match pc {
            0x82DDED28 => {
    //   block [0x82DDED28..0x82DDEDBC)
	// 82DDED28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DDED2C: 4BECA6D9  bl 0x82ca9404
	ctx.lr = 0x82DDED30;
	sub_82CA93D0(ctx, base);
	// 82DDED30: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DDED34: 81640008  lwz r11, 8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DDED38: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DDED3C: 83640000  lwz r27, 0(r4)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDED40: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82DDED44: 9081005C  stw r4, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[4].u32 ) };
	// 82DDED48: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82DDED4C: 7CFC3B78  mr r28, r7
	ctx.r[28].u64 = ctx.r[7].u64;
	// 82DDED50: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82DDED54: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 82DDED58: 817B0010  lwz r11, 0x10(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DDED5C: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DDED60: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82DDED64: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82DDED68: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82DDED6C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDED70: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82DDED74: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DDED78: 4E800421  bctrl
	ctx.lr = 0x82DDED7C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DDED7C: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DDED80: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DDED84: 419A0038  beq cr6, 0x82ddedbc
	if ctx.cr[6].eq {
	pc = 0x82DDEDBC; continue 'dispatch;
	}
	// 82DDED88: 5563003E  slwi r3, r11, 0
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82DDED8C: 817B0018  lwz r11, 0x18(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DDED90: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 82DDED94: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82DDED98: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82DDED9C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82DDEDA0: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82DDEDA4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DDEDA8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82DDEDAC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDEDB0: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82DDEDB4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DDEDB8: 4E800421  bctrl
	ctx.lr = 0x82DDEDBC;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x82DDEDBC => {
    //   block [0x82DDEDBC..0x82DDEDC4)
	// 82DDEDBC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82DDEDC0: 4BECA694  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDEDC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DDEDC8 size=212
    let mut pc: u32 = 0x82DDEDC8;
    'dispatch: loop {
        match pc {
            0x82DDEDC8 => {
    //   block [0x82DDEDC8..0x82DDEE44)
	// 82DDEDC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DDEDCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DDEDD0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DDEDD4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DDEDD8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82DDEDDC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DDEDE0: 394A3E3C  addi r10, r10, 0x3e3c
	ctx.r[10].s64 = ctx.r[10].s64 + 15932;
	// 82DDEDE4: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82DDEDE8: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82DDEDEC: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82DDEDF0: 90FF0008  stw r7, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[7].u32 ) };
	// 82DDEDF4: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 82DDEDF8: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82DDEDFC: B13F0006  sth r9, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 82DDEE00: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDEE04: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82DDEE08: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DDEE0C: 81040000  lwz r8, 0(r4)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDEE10: 80C50010  lwz r6, 0x10(r5)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DDEE14: 81450000  lwz r10, 0(r5)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDEE18: 2F060000  cmpwi cr6, r6, 0
	ctx.cr[6].compare_i32(ctx.r[6].s32, 0, &mut ctx.xer);
	// 82DDEE1C: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 82DDEE20: 81690010  lwz r11, 0x10(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DDEE24: 3920FFFF  li r9, -1
	ctx.r[9].s64 = -1;
	// 82DDEE28: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82DDEE2C: 91210054  stw r9, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[9].u32 ) };
	// 82DDEE30: 8128000C  lwz r9, 0xc(r8)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DDEE34: 390A05A0  addi r8, r10, 0x5a0
	ctx.r[8].s64 = ctx.r[10].s64 + 1440;
	// 82DDEE38: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DDEE3C: 409A0008  bne cr6, 0x82ddee44
	if !ctx.cr[6].eq {
	pc = 0x82DDEE44; continue 'dispatch;
	}
	// 82DDEE40: 390A01A0  addi r8, r10, 0x1a0
	ctx.r[8].s64 = ctx.r[10].s64 + 416;
	pc = 0x82DDEE44; continue 'dispatch;
            }
            0x82DDEE44 => {
    //   block [0x82DDEE44..0x82DDEE9C)
	// 82DDEE44: 556B2834  slwi r11, r11, 5
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(5);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DDEE48: 7CE63B78  mr r6, r7
	ctx.r[6].u64 = ctx.r[7].u64;
	// 82DDEE4C: 7D6B4214  add r11, r11, r8
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[8].u64;
	// 82DDEE50: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DDEE54: 7D6B48AE  lbzx r11, r11, r9
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82DDEE58: 5569103E  rotlwi r9, r11, 2
	ctx.r[9].u64 = ((ctx.r[11].u32).rotate_left(2)) as u64;
	// 82DDEE5C: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82DDEE60: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DDEE64: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82DDEE68: 816B09A0  lwz r11, 0x9a0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2464 as u32) ) } as u64;
	// 82DDEE6C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DDEE70: 4E800421  bctrl
	ctx.lr = 0x82DDEE74;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DDEE74: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82DDEE78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82DDEE7C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DDEE80: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82DDEE84: 915F0010  stw r10, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 82DDEE88: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DDEE8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DDEE90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DDEE94: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DDEE98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDEEA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DDEEA0 size=88
    let mut pc: u32 = 0x82DDEEA0;
    'dispatch: loop {
        match pc {
            0x82DDEEA0 => {
    //   block [0x82DDEEA0..0x82DDEEF8)
	// 82DDEEA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DDEEA4: 4BECA565  bl 0x82ca9408
	ctx.lr = 0x82DDEEA8;
	sub_82CA93D0(ctx, base);
	// 82DDEEA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DDEEAC: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDEEB0: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82DDEEB4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DDEEB8: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82DDEEBC: 38A0001E  li r5, 0x1e
	ctx.r[5].s64 = 30;
	// 82DDEEC0: 38800014  li r4, 0x14
	ctx.r[4].s64 = 20;
	// 82DDEEC4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DDEEC8: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DDEECC: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82DDEED0: 4BF76379  bl 0x82d55248
	ctx.lr = 0x82DDEED4;
	sub_82D55248(ctx, base);
	// 82DDEED4: 39600014  li r11, 0x14
	ctx.r[11].s64 = 20;
	// 82DDEED8: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 82DDEEDC: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82DDEEE0: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82DDEEE4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DDEEE8: B1630004  sth r11, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 82DDEEEC: 4BFFFEDD  bl 0x82ddedc8
	ctx.lr = 0x82DDEEF0;
	sub_82DDEDC8(ctx, base);
	// 82DDEEF0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DDEEF4: 4BECA564  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDEEF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DDEEF8 size=496
    let mut pc: u32 = 0x82DDEEF8;
    'dispatch: loop {
        match pc {
            0x82DDEEF8 => {
    //   block [0x82DDEEF8..0x82DDEF58)
	// 82DDEEF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DDEEFC: 4BECA4FD  bl 0x82ca93f8
	ctx.lr = 0x82DDEF00;
	sub_82CA93D0(ctx, base);
	// 82DDEF00: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DDEF04: 832D0000  lwz r25, 0(r13)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDEF08: 3B400008  li r26, 8
	ctx.r[26].s64 = 8;
	// 82DDEF0C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DDEF10: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82DDEF14: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82DDEF18: 7CF83B78  mr r24, r7
	ctx.r[24].u64 = ctx.r[7].u64;
	// 82DDEF1C: 7D5AC82E  lwzx r10, r26, r25
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[25].u32)) } as u64;
	// 82DDEF20: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DDEF24: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DDEF28: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DDEF2C: 4098002C  bge cr6, 0x82ddef58
	if !ctx.cr[6].lt {
	pc = 0x82DDEF58; continue 'dispatch;
	}
	// 82DDEF30: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DDEF34: 3D008203  lis r8, -0x7dfd
	ctx.r[8].s64 = -2113732608;
	// 82DDEF38: 39293E84  addi r9, r9, 0x3e84
	ctx.r[9].s64 = ctx.r[9].s64 + 16004;
	// 82DDEF3C: 39083E74  addi r8, r8, 0x3e74
	ctx.r[8].s64 = ctx.r[8].s64 + 15988;
	// 82DDEF40: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DDEF44: 910B000C  stw r8, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 82DDEF48: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82DDEF4C: 390B0010  addi r8, r11, 0x10
	ctx.r[8].s64 = ctx.r[11].s64 + 16;
	// 82DDEF50: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82DDEF54: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	pc = 0x82DDEF58; continue 'dispatch;
            }
            0x82DDEF58 => {
    //   block [0x82DDEF58..0x82DDEFEC)
	// 82DDEF58: 81440008  lwz r10, 8(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DDEF5C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DDEF60: 83840000  lwz r28, 0(r4)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDEF64: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82DDEF68: 9081006C  stw r4, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[4].u32 ) };
	// 82DDEF6C: 396B360C  addi r11, r11, 0x360c
	ctx.r[11].s64 = ctx.r[11].s64 + 13836;
	// 82DDEF70: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DDEF74: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 82DDEF78: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82DDEF7C: 91410068  stw r10, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[10].u32 ) };
	// 82DDEF80: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82DDEF84: 815C0010  lwz r10, 0x10(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DDEF88: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82DDEF8C: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82DDEF90: 9B610054  stb r27, 0x54(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[27].u8 ) };
	// 82DDEF94: 91410060  stw r10, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[10].u32 ) };
	// 82DDEF98: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 82DDEF9C: 91410064  stw r10, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[10].u32 ) };
	// 82DDEFA0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDEFA4: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DDEFA8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DDEFAC: 4E800421  bctrl
	ctx.lr = 0x82DDEFB0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DDEFB0: 89610054  lbz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82DDEFB4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DDEFB8: 419A00D4  beq cr6, 0x82ddf08c
	if ctx.cr[6].eq {
	pc = 0x82DDF08C; continue 'dispatch;
	}
	// 82DDEFBC: 7D5AC82E  lwzx r10, r26, r25
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[25].u32)) } as u64;
	// 82DDEFC0: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DDEFC4: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DDEFC8: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DDEFCC: 40980020  bge cr6, 0x82ddefec
	if !ctx.cr[6].lt {
	pc = 0x82DDEFEC; continue 'dispatch;
	}
	// 82DDEFD0: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DDEFD4: 392936A4  addi r9, r9, 0x36a4
	ctx.r[9].s64 = ctx.r[9].s64 + 13988;
	// 82DDEFD8: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DDEFDC: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82DDEFE0: 390B000C  addi r8, r11, 0xc
	ctx.r[8].s64 = ctx.r[11].s64 + 12;
	// 82DDEFE4: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82DDEFE8: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
            }
            0x82DDEFEC => {
    //   block [0x82DDEFEC..0x82DDF02C)
	// 82DDEFEC: 817C0018  lwz r11, 0x18(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DDEFF0: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DDEFF4: 93610064  stw r27, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[27].u32 ) };
	// 82DDEFF8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DDEFFC: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 82DDF000: 409A0064  bne cr6, 0x82ddf064
	if !ctx.cr[6].eq {
	pc = 0x82DDF064; continue 'dispatch;
	}
	// 82DDF004: 815D0000  lwz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDF008: 811E0010  lwz r8, 0x10(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DDF00C: 813E0000  lwz r9, 0(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDF010: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82DDF014: 80DF0008  lwz r6, 8(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DDF018: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DDF01C: 810A000C  lwz r8, 0xc(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DDF020: 394905A0  addi r10, r9, 0x5a0
	ctx.r[10].s64 = ctx.r[9].s64 + 1440;
	// 82DDF024: 409A0008  bne cr6, 0x82ddf02c
	if !ctx.cr[6].eq {
	pc = 0x82DDF02C; continue 'dispatch;
	}
	// 82DDF028: 394901A0  addi r10, r9, 0x1a0
	ctx.r[10].s64 = ctx.r[9].s64 + 416;
	pc = 0x82DDF02C; continue 'dispatch;
            }
            0x82DDF02C => {
    //   block [0x82DDF02C..0x82DDF064)
	// 82DDF02C: 556B2834  slwi r11, r11, 5
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(5);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DDF030: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82DDF034: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82DDF038: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82DDF03C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82DDF040: 7D6B40AE  lbzx r11, r11, r8
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 82DDF044: 556A103E  rotlwi r10, r11, 2
	ctx.r[10].u64 = ((ctx.r[11].u32).rotate_left(2)) as u64;
	// 82DDF048: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82DDF04C: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DDF050: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82DDF054: 816B09A0  lwz r11, 0x9a0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2464 as u32) ) } as u64;
	// 82DDF058: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DDF05C: 4E800421  bctrl
	ctx.lr = 0x82DDF060;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DDF060: 907F0010  stw r3, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[3].u32 ) };
            }
            0x82DDF064 => {
    //   block [0x82DDF064..0x82DDF08C)
	// 82DDF064: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DDF068: 7F07C378  mr r7, r24
	ctx.r[7].u64 = ctx.r[24].u64;
	// 82DDF06C: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82DDF070: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82DDF074: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82DDF078: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDF07C: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DDF080: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DDF084: 4E800421  bctrl
	ctx.lr = 0x82DDF088;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DDF088: 48000028  b 0x82ddf0b0
	pc = 0x82DDF0B0; continue 'dispatch;
            }
            0x82DDF08C => {
    //   block [0x82DDF08C..0x82DDF0B0)
	// 82DDF08C: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DDF090: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82DDF094: 419A001C  beq cr6, 0x82ddf0b0
	if ctx.cr[6].eq {
	pc = 0x82DDF0B0; continue 'dispatch;
	}
	// 82DDF098: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDF09C: 80980004  lwz r4, 4(r24)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DDF0A0: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82DDF0A4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DDF0A8: 4E800421  bctrl
	ctx.lr = 0x82DDF0AC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DDF0AC: 937F0010  stw r27, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[27].u32 ) };
            }
            0x82DDF0B0 => {
    //   block [0x82DDF0B0..0x82DDF0E0)
	// 82DDF0B0: 7D5AC82E  lwzx r10, r26, r25
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[25].u32)) } as u64;
	// 82DDF0B4: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DDF0B8: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DDF0BC: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DDF0C0: 40980020  bge cr6, 0x82ddf0e0
	if !ctx.cr[6].lt {
	pc = 0x82DDF0E0; continue 'dispatch;
	}
	// 82DDF0C4: 3D208202  lis r9, -0x7dfe
	ctx.r[9].s64 = -2113798144;
	// 82DDF0C8: 39296468  addi r9, r9, 0x6468
	ctx.r[9].s64 = ctx.r[9].s64 + 25704;
	// 82DDF0CC: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DDF0D0: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82DDF0D4: 390B000C  addi r8, r11, 0xc
	ctx.r[8].s64 = ctx.r[11].s64 + 12;
	// 82DDF0D8: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82DDF0DC: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	pc = 0x82DDF0E0; continue 'dispatch;
            }
            0x82DDF0E0 => {
    //   block [0x82DDF0E0..0x82DDF0E8)
	// 82DDF0E0: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 82DDF0E4: 4BECA364  b 0x82ca9448
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDF0E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DDF0E8 size=484
    let mut pc: u32 = 0x82DDF0E8;
    'dispatch: loop {
        match pc {
            0x82DDF0E8 => {
    //   block [0x82DDF0E8..0x82DDF14C)
	// 82DDF0E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DDF0EC: 4BECA309  bl 0x82ca93f4
	ctx.lr = 0x82DDF0F0;
	sub_82CA93D0(ctx, base);
	// 82DDF0F0: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DDF0F4: 832D0000  lwz r25, 0(r13)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDF0F8: 3B400008  li r26, 8
	ctx.r[26].s64 = 8;
	// 82DDF0FC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DDF100: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82DDF104: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82DDF108: 7CF83B78  mr r24, r7
	ctx.r[24].u64 = ctx.r[7].u64;
	// 82DDF10C: 7D5AC82E  lwzx r10, r26, r25
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[25].u32)) } as u64;
	// 82DDF110: 7D174378  mr r23, r8
	ctx.r[23].u64 = ctx.r[8].u64;
	// 82DDF114: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DDF118: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DDF11C: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DDF120: 4098002C  bge cr6, 0x82ddf14c
	if !ctx.cr[6].lt {
	pc = 0x82DDF14C; continue 'dispatch;
	}
	// 82DDF124: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DDF128: 3D008203  lis r8, -0x7dfd
	ctx.r[8].s64 = -2113732608;
	// 82DDF12C: 39293E84  addi r9, r9, 0x3e84
	ctx.r[9].s64 = ctx.r[9].s64 + 16004;
	// 82DDF130: 39083E74  addi r8, r8, 0x3e74
	ctx.r[8].s64 = ctx.r[8].s64 + 15988;
	// 82DDF134: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DDF138: 910B000C  stw r8, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 82DDF13C: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82DDF140: 390B0010  addi r8, r11, 0x10
	ctx.r[8].s64 = ctx.r[11].s64 + 16;
	// 82DDF144: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82DDF148: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	pc = 0x82DDF14C; continue 'dispatch;
            }
            0x82DDF14C => {
    //   block [0x82DDF14C..0x82DDF1F4)
	// 82DDF14C: 81440008  lwz r10, 8(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DDF150: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DDF154: 83840000  lwz r28, 0(r4)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDF158: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82DDF15C: 396B0E8C  addi r11, r11, 0xe8c
	ctx.r[11].s64 = ctx.r[11].s64 + 3724;
	// 82DDF160: 9081005C  stw r4, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[4].u32 ) };
	// 82DDF164: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DDF168: 39010060  addi r8, r1, 0x60
	ctx.r[8].s64 = ctx.r[1].s64 + 96;
	// 82DDF16C: 38E10060  addi r7, r1, 0x60
	ctx.r[7].s64 = ctx.r[1].s64 + 96;
	// 82DDF170: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 82DDF174: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82DDF178: 815C0010  lwz r10, 0x10(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DDF17C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82DDF180: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 82DDF184: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DDF188: 9B610068  stb r27, 0x68(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[27].u8 ) };
	// 82DDF18C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82DDF190: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82DDF194: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 82DDF198: C00B0C64  lfs f0, 0xc64(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3172 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DDF19C: D001008C  stfs f0, 0x8c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(140 as u32), tmp.u32 ) };
	// 82DDF1A0: D0010064  stfs f0, 0x64(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), tmp.u32 ) };
	// 82DDF1A4: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 82DDF1A8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDF1AC: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DDF1B0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DDF1B4: 4E800421  bctrl
	ctx.lr = 0x82DDF1B8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DDF1B8: 89610068  lbz r11, 0x68(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 82DDF1BC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DDF1C0: 419A00D4  beq cr6, 0x82ddf294
	if ctx.cr[6].eq {
	pc = 0x82DDF294; continue 'dispatch;
	}
	// 82DDF1C4: 7D5AC82E  lwzx r10, r26, r25
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[25].u32)) } as u64;
	// 82DDF1C8: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DDF1CC: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DDF1D0: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DDF1D4: 40980020  bge cr6, 0x82ddf1f4
	if !ctx.cr[6].lt {
	pc = 0x82DDF1F4; continue 'dispatch;
	}
	// 82DDF1D8: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DDF1DC: 392936A4  addi r9, r9, 0x36a4
	ctx.r[9].s64 = ctx.r[9].s64 + 13988;
	// 82DDF1E0: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DDF1E4: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82DDF1E8: 390B000C  addi r8, r11, 0xc
	ctx.r[8].s64 = ctx.r[11].s64 + 12;
	// 82DDF1EC: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82DDF1F0: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
            }
            0x82DDF1F4 => {
    //   block [0x82DDF1F4..0x82DDF234)
	// 82DDF1F4: 817C0018  lwz r11, 0x18(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DDF1F8: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DDF1FC: 93610054  stw r27, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[27].u32 ) };
	// 82DDF200: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DDF204: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82DDF208: 409A0064  bne cr6, 0x82ddf26c
	if !ctx.cr[6].eq {
	pc = 0x82DDF26C; continue 'dispatch;
	}
	// 82DDF20C: 815D0000  lwz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDF210: 811E0010  lwz r8, 0x10(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DDF214: 813E0000  lwz r9, 0(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDF218: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82DDF21C: 80DF0008  lwz r6, 8(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DDF220: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DDF224: 810A000C  lwz r8, 0xc(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DDF228: 394905A0  addi r10, r9, 0x5a0
	ctx.r[10].s64 = ctx.r[9].s64 + 1440;
	// 82DDF22C: 409A0008  bne cr6, 0x82ddf234
	if !ctx.cr[6].eq {
	pc = 0x82DDF234; continue 'dispatch;
	}
	// 82DDF230: 394901A0  addi r10, r9, 0x1a0
	ctx.r[10].s64 = ctx.r[9].s64 + 416;
	pc = 0x82DDF234; continue 'dispatch;
            }
            0x82DDF234 => {
    //   block [0x82DDF234..0x82DDF26C)
	// 82DDF234: 556B2834  slwi r11, r11, 5
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(5);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DDF238: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82DDF23C: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82DDF240: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82DDF244: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DDF248: 7D6B40AE  lbzx r11, r11, r8
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 82DDF24C: 556A103E  rotlwi r10, r11, 2
	ctx.r[10].u64 = ((ctx.r[11].u32).rotate_left(2)) as u64;
	// 82DDF250: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82DDF254: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DDF258: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82DDF25C: 816B09A0  lwz r11, 0x9a0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2464 as u32) ) } as u64;
	// 82DDF260: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DDF264: 4E800421  bctrl
	ctx.lr = 0x82DDF268;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DDF268: 907F0010  stw r3, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[3].u32 ) };
            }
            0x82DDF26C => {
    //   block [0x82DDF26C..0x82DDF294)
	// 82DDF26C: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DDF270: 7EE8BB78  mr r8, r23
	ctx.r[8].u64 = ctx.r[23].u64;
	// 82DDF274: 7F07C378  mr r7, r24
	ctx.r[7].u64 = ctx.r[24].u64;
	// 82DDF278: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82DDF27C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82DDF280: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82DDF284: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDF288: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DDF28C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DDF290: 4E800421  bctrl
	ctx.lr = 0x82DDF294;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x82DDF294 => {
    //   block [0x82DDF294..0x82DDF2C4)
	// 82DDF294: 7D5AC82E  lwzx r10, r26, r25
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[25].u32)) } as u64;
	// 82DDF298: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DDF29C: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DDF2A0: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DDF2A4: 40980020  bge cr6, 0x82ddf2c4
	if !ctx.cr[6].lt {
	pc = 0x82DDF2C4; continue 'dispatch;
	}
	// 82DDF2A8: 3D208202  lis r9, -0x7dfe
	ctx.r[9].s64 = -2113798144;
	// 82DDF2AC: 39296468  addi r9, r9, 0x6468
	ctx.r[9].s64 = ctx.r[9].s64 + 25704;
	// 82DDF2B0: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DDF2B4: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82DDF2B8: 390B000C  addi r8, r11, 0xc
	ctx.r[8].s64 = ctx.r[11].s64 + 12;
	// 82DDF2BC: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82DDF2C0: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	pc = 0x82DDF2C4; continue 'dispatch;
            }
            0x82DDF2C4 => {
    //   block [0x82DDF2C4..0x82DDF2CC)
	// 82DDF2C4: 382100E0  addi r1, r1, 0xe0
	ctx.r[1].s64 = ctx.r[1].s64 + 224;
	// 82DDF2C8: 4BECA17C  b 0x82ca9444
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDF2D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DDF2D0 size=456
    let mut pc: u32 = 0x82DDF2D0;
    'dispatch: loop {
        match pc {
            0x82DDF2D0 => {
    //   block [0x82DDF2D0..0x82DDF330)
	// 82DDF2D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DDF2D4: 4BECA121  bl 0x82ca93f4
	ctx.lr = 0x82DDF2D8;
	sub_82CA93D0(ctx, base);
	// 82DDF2D8: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DDF2DC: 82ED0000  lwz r23, 0(r13)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDF2E0: 3B000008  li r24, 8
	ctx.r[24].s64 = 8;
	// 82DDF2E4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DDF2E8: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82DDF2EC: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 82DDF2F0: 7CF93B78  mr r25, r7
	ctx.r[25].u64 = ctx.r[7].u64;
	// 82DDF2F4: 7D58B82E  lwzx r10, r24, r23
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[24].u32.wrapping_add(ctx.r[23].u32)) } as u64;
	// 82DDF2F8: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DDF2FC: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DDF300: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DDF304: 4098002C  bge cr6, 0x82ddf330
	if !ctx.cr[6].lt {
	pc = 0x82DDF330; continue 'dispatch;
	}
	// 82DDF308: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DDF30C: 3D008203  lis r8, -0x7dfd
	ctx.r[8].s64 = -2113732608;
	// 82DDF310: 39293E84  addi r9, r9, 0x3e84
	ctx.r[9].s64 = ctx.r[9].s64 + 16004;
	// 82DDF314: 39083E74  addi r8, r8, 0x3e74
	ctx.r[8].s64 = ctx.r[8].s64 + 15988;
	// 82DDF318: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DDF31C: 910B000C  stw r8, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 82DDF320: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82DDF324: 390B0010  addi r8, r11, 0x10
	ctx.r[8].s64 = ctx.r[11].s64 + 16;
	// 82DDF328: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82DDF32C: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	pc = 0x82DDF330; continue 'dispatch;
            }
            0x82DDF330 => {
    //   block [0x82DDF330..0x82DDF400)
	// 82DDF330: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82DDF334: 83A30000  lwz r29, 0(r3)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDF338: 9061005C  stw r3, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[3].u32 ) };
	// 82DDF33C: 3B80FFFF  li r28, -1
	ctx.r[28].s64 = -1;
	// 82DDF340: 392A0E8C  addi r9, r10, 0xe8c
	ctx.r[9].s64 = ctx.r[10].s64 + 3724;
	// 82DDF344: 81430008  lwz r10, 8(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DDF348: 811E0000  lwz r8, 0(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDF34C: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82DDF350: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDF354: 38E10060  addi r7, r1, 0x60
	ctx.r[7].s64 = ctx.r[1].s64 + 96;
	// 82DDF358: 38C10060  addi r6, r1, 0x60
	ctx.r[6].s64 = ctx.r[1].s64 + 96;
	// 82DDF35C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82DDF360: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 82DDF364: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82DDF368: 815D0010  lwz r10, 0x10(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DDF36C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DDF370: 93810054  stw r28, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[28].u32 ) };
	// 82DDF374: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82DDF378: 814A000C  lwz r10, 0xc(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DDF37C: 8388000C  lwz r28, 0xc(r8)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DDF380: 394A000D  addi r10, r10, 0xd
	ctx.r[10].s64 = ctx.r[10].s64 + 13;
	// 82DDF384: 554A2834  slwi r10, r10, 5
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(5);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DDF388: 7D4A5A14  add r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82DDF38C: 7D4AE0AE  lbzx r10, r10, r28
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 82DDF390: 5548103E  rotlwi r8, r10, 2
	ctx.r[8].u64 = ((ctx.r[10].u32).rotate_left(2)) as u64;
	// 82DDF394: 7D4A4214  add r10, r10, r8
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[8].u64;
	// 82DDF398: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DDF39C: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82DDF3A0: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82DDF3A4: 816B09AC  lwz r11, 0x9ac(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2476 as u32) ) } as u64;
	// 82DDF3A8: C00A0C64  lfs f0, 0xc64(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(3172 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DDF3AC: 91210060  stw r9, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[9].u32 ) };
	// 82DDF3B0: D001008C  stfs f0, 0x8c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(140 as u32), tmp.u32 ) };
	// 82DDF3B4: 9B610068  stb r27, 0x68(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[27].u8 ) };
	// 82DDF3B8: D0010064  stfs f0, 0x64(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), tmp.u32 ) };
	// 82DDF3BC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DDF3C0: 4E800421  bctrl
	ctx.lr = 0x82DDF3C4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DDF3C4: 89610068  lbz r11, 0x68(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 82DDF3C8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DDF3CC: 419A0088  beq cr6, 0x82ddf454
	if ctx.cr[6].eq {
	pc = 0x82DDF454; continue 'dispatch;
	}
	// 82DDF3D0: 7D58B82E  lwzx r10, r24, r23
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[24].u32.wrapping_add(ctx.r[23].u32)) } as u64;
	// 82DDF3D4: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DDF3D8: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DDF3DC: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DDF3E0: 40980020  bge cr6, 0x82ddf400
	if !ctx.cr[6].lt {
	pc = 0x82DDF400; continue 'dispatch;
	}
	// 82DDF3E4: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DDF3E8: 392936A4  addi r9, r9, 0x36a4
	ctx.r[9].s64 = ctx.r[9].s64 + 13988;
	// 82DDF3EC: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DDF3F0: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82DDF3F4: 390B000C  addi r8, r11, 0xc
	ctx.r[8].s64 = ctx.r[11].s64 + 12;
	// 82DDF3F8: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82DDF3FC: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
            }
            0x82DDF400 => {
    //   block [0x82DDF400..0x82DDF454)
	// 82DDF400: 815D0018  lwz r10, 0x18(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DDF404: 7F27CB78  mr r7, r25
	ctx.r[7].u64 = ctx.r[25].u64;
	// 82DDF408: 93610054  stw r27, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[27].u32 ) };
	// 82DDF40C: 7F46D378  mr r6, r26
	ctx.r[6].u64 = ctx.r[26].u64;
	// 82DDF410: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDF414: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82DDF418: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82DDF41C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82DDF420: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82DDF424: 814A000C  lwz r10, 0xc(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DDF428: 394A000D  addi r10, r10, 0xd
	ctx.r[10].s64 = ctx.r[10].s64 + 13;
	// 82DDF42C: 554A2834  slwi r10, r10, 5
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(5);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DDF430: 7D4A5A14  add r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82DDF434: 7D4AE0AE  lbzx r10, r10, r28
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 82DDF438: 5549103E  rotlwi r9, r10, 2
	ctx.r[9].u64 = ((ctx.r[10].u32).rotate_left(2)) as u64;
	// 82DDF43C: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82DDF440: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DDF444: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82DDF448: 816B09AC  lwz r11, 0x9ac(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2476 as u32) ) } as u64;
	// 82DDF44C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DDF450: 4E800421  bctrl
	ctx.lr = 0x82DDF454;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x82DDF454 => {
    //   block [0x82DDF454..0x82DDF490)
	// 82DDF454: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DDF458: 7D58B82E  lwzx r10, r24, r23
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[24].u32.wrapping_add(ctx.r[23].u32)) } as u64;
	// 82DDF45C: 396B3250  addi r11, r11, 0x3250
	ctx.r[11].s64 = ctx.r[11].s64 + 12880;
	// 82DDF460: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 82DDF464: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DDF468: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DDF46C: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DDF470: 40980020  bge cr6, 0x82ddf490
	if !ctx.cr[6].lt {
	pc = 0x82DDF490; continue 'dispatch;
	}
	// 82DDF474: 3D208202  lis r9, -0x7dfe
	ctx.r[9].s64 = -2113798144;
	// 82DDF478: 39296468  addi r9, r9, 0x6468
	ctx.r[9].s64 = ctx.r[9].s64 + 25704;
	// 82DDF47C: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DDF480: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82DDF484: 390B000C  addi r8, r11, 0xc
	ctx.r[8].s64 = ctx.r[11].s64 + 12;
	// 82DDF488: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82DDF48C: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	pc = 0x82DDF490; continue 'dispatch;
            }
            0x82DDF490 => {
    //   block [0x82DDF490..0x82DDF498)
	// 82DDF490: 382100E0  addi r1, r1, 0xe0
	ctx.r[1].s64 = ctx.r[1].s64 + 224;
	// 82DDF494: 4BEC9FB0  b 0x82ca9444
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDF498(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DDF498 size=468
    let mut pc: u32 = 0x82DDF498;
    'dispatch: loop {
        match pc {
            0x82DDF498 => {
    //   block [0x82DDF498..0x82DDF4F8)
	// 82DDF498: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DDF49C: 4BEC9F5D  bl 0x82ca93f8
	ctx.lr = 0x82DDF4A0;
	sub_82CA93D0(ctx, base);
	// 82DDF4A0: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DDF4A4: 832D0000  lwz r25, 0(r13)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDF4A8: 3B400008  li r26, 8
	ctx.r[26].s64 = 8;
	// 82DDF4AC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DDF4B0: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82DDF4B4: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82DDF4B8: 7CF83B78  mr r24, r7
	ctx.r[24].u64 = ctx.r[7].u64;
	// 82DDF4BC: 7D5AC82E  lwzx r10, r26, r25
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[25].u32)) } as u64;
	// 82DDF4C0: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DDF4C4: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DDF4C8: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DDF4CC: 4098002C  bge cr6, 0x82ddf4f8
	if !ctx.cr[6].lt {
	pc = 0x82DDF4F8; continue 'dispatch;
	}
	// 82DDF4D0: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DDF4D4: 3D008203  lis r8, -0x7dfd
	ctx.r[8].s64 = -2113732608;
	// 82DDF4D8: 39293E84  addi r9, r9, 0x3e84
	ctx.r[9].s64 = ctx.r[9].s64 + 16004;
	// 82DDF4DC: 39083E74  addi r8, r8, 0x3e74
	ctx.r[8].s64 = ctx.r[8].s64 + 15988;
	// 82DDF4E0: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DDF4E4: 910B000C  stw r8, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 82DDF4E8: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82DDF4EC: 390B0010  addi r8, r11, 0x10
	ctx.r[8].s64 = ctx.r[11].s64 + 16;
	// 82DDF4F0: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82DDF4F4: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	pc = 0x82DDF4F8; continue 'dispatch;
            }
            0x82DDF4F8 => {
    //   block [0x82DDF4F8..0x82DDF58C)
	// 82DDF4F8: 81440008  lwz r10, 8(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DDF4FC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DDF500: 83840000  lwz r28, 0(r4)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDF504: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82DDF508: 9081006C  stw r4, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[4].u32 ) };
	// 82DDF50C: 396B360C  addi r11, r11, 0x360c
	ctx.r[11].s64 = ctx.r[11].s64 + 13836;
	// 82DDF510: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DDF514: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 82DDF518: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82DDF51C: 91410068  stw r10, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[10].u32 ) };
	// 82DDF520: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82DDF524: 815C0010  lwz r10, 0x10(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DDF528: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82DDF52C: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82DDF530: 9B610054  stb r27, 0x54(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[27].u8 ) };
	// 82DDF534: 91410060  stw r10, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[10].u32 ) };
	// 82DDF538: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 82DDF53C: 91410064  stw r10, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[10].u32 ) };
	// 82DDF540: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDF544: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DDF548: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DDF54C: 4E800421  bctrl
	ctx.lr = 0x82DDF550;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DDF550: 89610054  lbz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82DDF554: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DDF558: 419A00D0  beq cr6, 0x82ddf628
	if ctx.cr[6].eq {
	pc = 0x82DDF628; continue 'dispatch;
	}
	// 82DDF55C: 7D5AC82E  lwzx r10, r26, r25
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[25].u32)) } as u64;
	// 82DDF560: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DDF564: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DDF568: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DDF56C: 40980020  bge cr6, 0x82ddf58c
	if !ctx.cr[6].lt {
	pc = 0x82DDF58C; continue 'dispatch;
	}
	// 82DDF570: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DDF574: 392936A4  addi r9, r9, 0x36a4
	ctx.r[9].s64 = ctx.r[9].s64 + 13988;
	// 82DDF578: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DDF57C: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82DDF580: 390B000C  addi r8, r11, 0xc
	ctx.r[8].s64 = ctx.r[11].s64 + 12;
	// 82DDF584: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82DDF588: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
            }
            0x82DDF58C => {
    //   block [0x82DDF58C..0x82DDF5CC)
	// 82DDF58C: 817C0018  lwz r11, 0x18(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DDF590: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DDF594: 93610064  stw r27, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[27].u32 ) };
	// 82DDF598: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DDF59C: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 82DDF5A0: 409A0064  bne cr6, 0x82ddf604
	if !ctx.cr[6].eq {
	pc = 0x82DDF604; continue 'dispatch;
	}
	// 82DDF5A4: 815D0000  lwz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDF5A8: 811E0010  lwz r8, 0x10(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DDF5AC: 813E0000  lwz r9, 0(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDF5B0: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82DDF5B4: 80DF0008  lwz r6, 8(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DDF5B8: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DDF5BC: 810A000C  lwz r8, 0xc(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DDF5C0: 394905A0  addi r10, r9, 0x5a0
	ctx.r[10].s64 = ctx.r[9].s64 + 1440;
	// 82DDF5C4: 409A0008  bne cr6, 0x82ddf5cc
	if !ctx.cr[6].eq {
	pc = 0x82DDF5CC; continue 'dispatch;
	}
	// 82DDF5C8: 394901A0  addi r10, r9, 0x1a0
	ctx.r[10].s64 = ctx.r[9].s64 + 416;
	pc = 0x82DDF5CC; continue 'dispatch;
            }
            0x82DDF5CC => {
    //   block [0x82DDF5CC..0x82DDF604)
	// 82DDF5CC: 556B2834  slwi r11, r11, 5
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(5);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DDF5D0: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82DDF5D4: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82DDF5D8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82DDF5DC: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82DDF5E0: 7D6B40AE  lbzx r11, r11, r8
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 82DDF5E4: 556A103E  rotlwi r10, r11, 2
	ctx.r[10].u64 = ((ctx.r[11].u32).rotate_left(2)) as u64;
	// 82DDF5E8: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82DDF5EC: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DDF5F0: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82DDF5F4: 816B09A0  lwz r11, 0x9a0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2464 as u32) ) } as u64;
	// 82DDF5F8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DDF5FC: 4E800421  bctrl
	ctx.lr = 0x82DDF600;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DDF600: 907F0010  stw r3, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[3].u32 ) };
            }
            0x82DDF604 => {
    //   block [0x82DDF604..0x82DDF628)
	// 82DDF604: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DDF608: 7F07C378  mr r7, r24
	ctx.r[7].u64 = ctx.r[24].u64;
	// 82DDF60C: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82DDF610: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82DDF614: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82DDF618: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDF61C: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DDF620: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DDF624: 4E800421  bctrl
	ctx.lr = 0x82DDF628;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x82DDF628 => {
    //   block [0x82DDF628..0x82DDF664)
	// 82DDF628: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DDF62C: 7D5AC82E  lwzx r10, r26, r25
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[25].u32)) } as u64;
	// 82DDF630: 396B325C  addi r11, r11, 0x325c
	ctx.r[11].s64 = ctx.r[11].s64 + 12892;
	// 82DDF634: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82DDF638: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DDF63C: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DDF640: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DDF644: 40980020  bge cr6, 0x82ddf664
	if !ctx.cr[6].lt {
	pc = 0x82DDF664; continue 'dispatch;
	}
	// 82DDF648: 3D208202  lis r9, -0x7dfe
	ctx.r[9].s64 = -2113798144;
	// 82DDF64C: 39296468  addi r9, r9, 0x6468
	ctx.r[9].s64 = ctx.r[9].s64 + 25704;
	// 82DDF650: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DDF654: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82DDF658: 390B000C  addi r8, r11, 0xc
	ctx.r[8].s64 = ctx.r[11].s64 + 12;
	// 82DDF65C: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82DDF660: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	pc = 0x82DDF664; continue 'dispatch;
            }
            0x82DDF664 => {
    //   block [0x82DDF664..0x82DDF66C)
	// 82DDF664: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 82DDF668: 4BEC9DE0  b 0x82ca9448
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDF670(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DDF670 size=428
    let mut pc: u32 = 0x82DDF670;
    'dispatch: loop {
        match pc {
            0x82DDF670 => {
    //   block [0x82DDF670..0x82DDF6CC)
	// 82DDF670: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DDF674: 4BEC9D85  bl 0x82ca93f8
	ctx.lr = 0x82DDF678;
	sub_82CA93D0(ctx, base);
	// 82DDF678: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DDF67C: 830D0000  lwz r24, 0(r13)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDF680: 3B200008  li r25, 8
	ctx.r[25].s64 = 8;
	// 82DDF684: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DDF688: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82DDF68C: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 82DDF690: 7D59C02E  lwzx r10, r25, r24
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[25].u32.wrapping_add(ctx.r[24].u32)) } as u64;
	// 82DDF694: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DDF698: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DDF69C: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DDF6A0: 4098002C  bge cr6, 0x82ddf6cc
	if !ctx.cr[6].lt {
	pc = 0x82DDF6CC; continue 'dispatch;
	}
	// 82DDF6A4: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DDF6A8: 3D008203  lis r8, -0x7dfd
	ctx.r[8].s64 = -2113732608;
	// 82DDF6AC: 39293E84  addi r9, r9, 0x3e84
	ctx.r[9].s64 = ctx.r[9].s64 + 16004;
	// 82DDF6B0: 39083E74  addi r8, r8, 0x3e74
	ctx.r[8].s64 = ctx.r[8].s64 + 15988;
	// 82DDF6B4: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DDF6B8: 910B000C  stw r8, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 82DDF6BC: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82DDF6C0: 390B0010  addi r8, r11, 0x10
	ctx.r[8].s64 = ctx.r[11].s64 + 16;
	// 82DDF6C4: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82DDF6C8: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	pc = 0x82DDF6CC; continue 'dispatch;
            }
            0x82DDF6CC => {
    //   block [0x82DDF6CC..0x82DDF788)
	// 82DDF6CC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82DDF6D0: 83A30000  lwz r29, 0(r3)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDF6D4: 9061006C  stw r3, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[3].u32 ) };
	// 82DDF6D8: 38E0FFFF  li r7, -1
	ctx.r[7].s64 = -1;
	// 82DDF6DC: 392A360C  addi r9, r10, 0x360c
	ctx.r[9].s64 = ctx.r[10].s64 + 13836;
	// 82DDF6E0: 81430008  lwz r10, 8(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DDF6E4: 811E0000  lwz r8, 0(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDF6E8: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82DDF6EC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDF6F0: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82DDF6F4: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82DDF6F8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82DDF6FC: 91410068  stw r10, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[10].u32 ) };
	// 82DDF700: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82DDF704: 815D0010  lwz r10, 0x10(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DDF708: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 82DDF70C: 91410060  stw r10, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[10].u32 ) };
	// 82DDF710: 814A000C  lwz r10, 0xc(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DDF714: 8388000C  lwz r28, 0xc(r8)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DDF718: 394A000D  addi r10, r10, 0xd
	ctx.r[10].s64 = ctx.r[10].s64 + 13;
	// 82DDF71C: 554A2834  slwi r10, r10, 5
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(5);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DDF720: 7D4A5A14  add r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82DDF724: 7D4AE0AE  lbzx r10, r10, r28
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 82DDF728: 5548103E  rotlwi r8, r10, 2
	ctx.r[8].u64 = ((ctx.r[10].u32).rotate_left(2)) as u64;
	// 82DDF72C: 7D4A4214  add r10, r10, r8
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[8].u64;
	// 82DDF730: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DDF734: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82DDF738: 816B09A4  lwz r11, 0x9a4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2468 as u32) ) } as u64;
	// 82DDF73C: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 82DDF740: 9B610054  stb r27, 0x54(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[27].u8 ) };
	// 82DDF744: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DDF748: 4E800421  bctrl
	ctx.lr = 0x82DDF74C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DDF74C: 89610054  lbz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82DDF750: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DDF754: 419A0084  beq cr6, 0x82ddf7d8
	if ctx.cr[6].eq {
	pc = 0x82DDF7D8; continue 'dispatch;
	}
	// 82DDF758: 7D59C02E  lwzx r10, r25, r24
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[25].u32.wrapping_add(ctx.r[24].u32)) } as u64;
	// 82DDF75C: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DDF760: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DDF764: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DDF768: 40980020  bge cr6, 0x82ddf788
	if !ctx.cr[6].lt {
	pc = 0x82DDF788; continue 'dispatch;
	}
	// 82DDF76C: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DDF770: 392936A4  addi r9, r9, 0x36a4
	ctx.r[9].s64 = ctx.r[9].s64 + 13988;
	// 82DDF774: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DDF778: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82DDF77C: 390B000C  addi r8, r11, 0xc
	ctx.r[8].s64 = ctx.r[11].s64 + 12;
	// 82DDF780: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82DDF784: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
            }
            0x82DDF788 => {
    //   block [0x82DDF788..0x82DDF7D8)
	// 82DDF788: 815D0018  lwz r10, 0x18(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DDF78C: 7F46D378  mr r6, r26
	ctx.r[6].u64 = ctx.r[26].u64;
	// 82DDF790: 93610064  stw r27, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[27].u32 ) };
	// 82DDF794: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82DDF798: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDF79C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82DDF7A0: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82DDF7A4: 91410060  stw r10, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[10].u32 ) };
	// 82DDF7A8: 814A000C  lwz r10, 0xc(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DDF7AC: 394A000D  addi r10, r10, 0xd
	ctx.r[10].s64 = ctx.r[10].s64 + 13;
	// 82DDF7B0: 554A2834  slwi r10, r10, 5
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(5);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DDF7B4: 7D4A5A14  add r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82DDF7B8: 7D4AE0AE  lbzx r10, r10, r28
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 82DDF7BC: 5549103E  rotlwi r9, r10, 2
	ctx.r[9].u64 = ((ctx.r[10].u32).rotate_left(2)) as u64;
	// 82DDF7C0: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82DDF7C4: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DDF7C8: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82DDF7CC: 816B09A8  lwz r11, 0x9a8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2472 as u32) ) } as u64;
	// 82DDF7D0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DDF7D4: 4E800421  bctrl
	ctx.lr = 0x82DDF7D8;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x82DDF7D8 => {
    //   block [0x82DDF7D8..0x82DDF814)
	// 82DDF7D8: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DDF7DC: 7D59C02E  lwzx r10, r25, r24
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[25].u32.wrapping_add(ctx.r[24].u32)) } as u64;
	// 82DDF7E0: 396B325C  addi r11, r11, 0x325c
	ctx.r[11].s64 = ctx.r[11].s64 + 12892;
	// 82DDF7E4: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82DDF7E8: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DDF7EC: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DDF7F0: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DDF7F4: 40980020  bge cr6, 0x82ddf814
	if !ctx.cr[6].lt {
	pc = 0x82DDF814; continue 'dispatch;
	}
	// 82DDF7F8: 3D208202  lis r9, -0x7dfe
	ctx.r[9].s64 = -2113798144;
	// 82DDF7FC: 39296468  addi r9, r9, 0x6468
	ctx.r[9].s64 = ctx.r[9].s64 + 25704;
	// 82DDF800: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DDF804: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82DDF808: 390B000C  addi r8, r11, 0xc
	ctx.r[8].s64 = ctx.r[11].s64 + 12;
	// 82DDF80C: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82DDF810: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	pc = 0x82DDF814; continue 'dispatch;
            }
            0x82DDF814 => {
    //   block [0x82DDF814..0x82DDF81C)
	// 82DDF814: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 82DDF818: 4BEC9C30  b 0x82ca9448
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDF820(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DDF820 size=468
    let mut pc: u32 = 0x82DDF820;
    'dispatch: loop {
        match pc {
            0x82DDF820 => {
    //   block [0x82DDF820..0x82DDF880)
	// 82DDF820: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DDF824: 4BEC9BD5  bl 0x82ca93f8
	ctx.lr = 0x82DDF828;
	sub_82CA93D0(ctx, base);
	// 82DDF828: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DDF82C: 832D0000  lwz r25, 0(r13)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDF830: 3B400008  li r26, 8
	ctx.r[26].s64 = 8;
	// 82DDF834: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DDF838: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82DDF83C: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82DDF840: 7CF83B78  mr r24, r7
	ctx.r[24].u64 = ctx.r[7].u64;
	// 82DDF844: 7D5AC82E  lwzx r10, r26, r25
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[25].u32)) } as u64;
	// 82DDF848: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DDF84C: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DDF850: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DDF854: 4098002C  bge cr6, 0x82ddf880
	if !ctx.cr[6].lt {
	pc = 0x82DDF880; continue 'dispatch;
	}
	// 82DDF858: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DDF85C: 3D008203  lis r8, -0x7dfd
	ctx.r[8].s64 = -2113732608;
	// 82DDF860: 39293E84  addi r9, r9, 0x3e84
	ctx.r[9].s64 = ctx.r[9].s64 + 16004;
	// 82DDF864: 39083E74  addi r8, r8, 0x3e74
	ctx.r[8].s64 = ctx.r[8].s64 + 15988;
	// 82DDF868: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DDF86C: 910B000C  stw r8, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 82DDF870: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82DDF874: 390B0010  addi r8, r11, 0x10
	ctx.r[8].s64 = ctx.r[11].s64 + 16;
	// 82DDF878: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82DDF87C: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	pc = 0x82DDF880; continue 'dispatch;
            }
            0x82DDF880 => {
    //   block [0x82DDF880..0x82DDF914)
	// 82DDF880: 81440008  lwz r10, 8(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DDF884: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DDF888: 83840000  lwz r28, 0(r4)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDF88C: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82DDF890: 9081006C  stw r4, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[4].u32 ) };
	// 82DDF894: 396B360C  addi r11, r11, 0x360c
	ctx.r[11].s64 = ctx.r[11].s64 + 13836;
	// 82DDF898: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DDF89C: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 82DDF8A0: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82DDF8A4: 91410068  stw r10, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[10].u32 ) };
	// 82DDF8A8: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82DDF8AC: 815C0010  lwz r10, 0x10(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DDF8B0: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82DDF8B4: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82DDF8B8: 9B610054  stb r27, 0x54(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[27].u8 ) };
	// 82DDF8BC: 91410060  stw r10, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[10].u32 ) };
	// 82DDF8C0: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 82DDF8C4: 91410064  stw r10, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[10].u32 ) };
	// 82DDF8C8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDF8CC: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DDF8D0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DDF8D4: 4E800421  bctrl
	ctx.lr = 0x82DDF8D8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DDF8D8: 89610054  lbz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82DDF8DC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DDF8E0: 419A00D0  beq cr6, 0x82ddf9b0
	if ctx.cr[6].eq {
	pc = 0x82DDF9B0; continue 'dispatch;
	}
	// 82DDF8E4: 7D5AC82E  lwzx r10, r26, r25
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[25].u32)) } as u64;
	// 82DDF8E8: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DDF8EC: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DDF8F0: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DDF8F4: 40980020  bge cr6, 0x82ddf914
	if !ctx.cr[6].lt {
	pc = 0x82DDF914; continue 'dispatch;
	}
	// 82DDF8F8: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DDF8FC: 392936A4  addi r9, r9, 0x36a4
	ctx.r[9].s64 = ctx.r[9].s64 + 13988;
	// 82DDF900: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DDF904: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82DDF908: 390B000C  addi r8, r11, 0xc
	ctx.r[8].s64 = ctx.r[11].s64 + 12;
	// 82DDF90C: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82DDF910: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
            }
            0x82DDF914 => {
    //   block [0x82DDF914..0x82DDF954)
	// 82DDF914: 817C0018  lwz r11, 0x18(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DDF918: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DDF91C: 93610064  stw r27, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[27].u32 ) };
	// 82DDF920: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DDF924: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 82DDF928: 409A0064  bne cr6, 0x82ddf98c
	if !ctx.cr[6].eq {
	pc = 0x82DDF98C; continue 'dispatch;
	}
	// 82DDF92C: 815D0000  lwz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDF930: 811E0010  lwz r8, 0x10(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DDF934: 813E0000  lwz r9, 0(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDF938: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82DDF93C: 80DF0008  lwz r6, 8(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DDF940: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DDF944: 810A000C  lwz r8, 0xc(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DDF948: 394905A0  addi r10, r9, 0x5a0
	ctx.r[10].s64 = ctx.r[9].s64 + 1440;
	// 82DDF94C: 409A0008  bne cr6, 0x82ddf954
	if !ctx.cr[6].eq {
	pc = 0x82DDF954; continue 'dispatch;
	}
	// 82DDF950: 394901A0  addi r10, r9, 0x1a0
	ctx.r[10].s64 = ctx.r[9].s64 + 416;
	pc = 0x82DDF954; continue 'dispatch;
            }
            0x82DDF954 => {
    //   block [0x82DDF954..0x82DDF98C)
	// 82DDF954: 556B2834  slwi r11, r11, 5
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(5);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DDF958: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82DDF95C: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82DDF960: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82DDF964: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82DDF968: 7D6B40AE  lbzx r11, r11, r8
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 82DDF96C: 556A103E  rotlwi r10, r11, 2
	ctx.r[10].u64 = ((ctx.r[11].u32).rotate_left(2)) as u64;
	// 82DDF970: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82DDF974: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DDF978: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82DDF97C: 816B09A0  lwz r11, 0x9a0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2464 as u32) ) } as u64;
	// 82DDF980: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DDF984: 4E800421  bctrl
	ctx.lr = 0x82DDF988;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DDF988: 907F0010  stw r3, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[3].u32 ) };
            }
            0x82DDF98C => {
    //   block [0x82DDF98C..0x82DDF9B0)
	// 82DDF98C: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DDF990: 7F07C378  mr r7, r24
	ctx.r[7].u64 = ctx.r[24].u64;
	// 82DDF994: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82DDF998: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82DDF99C: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82DDF9A0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDF9A4: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DDF9A8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DDF9AC: 4E800421  bctrl
	ctx.lr = 0x82DDF9B0;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x82DDF9B0 => {
    //   block [0x82DDF9B0..0x82DDF9EC)
	// 82DDF9B0: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DDF9B4: 7D5AC82E  lwzx r10, r26, r25
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[25].u32)) } as u64;
	// 82DDF9B8: 396B325C  addi r11, r11, 0x325c
	ctx.r[11].s64 = ctx.r[11].s64 + 12892;
	// 82DDF9BC: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82DDF9C0: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DDF9C4: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DDF9C8: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DDF9CC: 40980020  bge cr6, 0x82ddf9ec
	if !ctx.cr[6].lt {
	pc = 0x82DDF9EC; continue 'dispatch;
	}
	// 82DDF9D0: 3D208202  lis r9, -0x7dfe
	ctx.r[9].s64 = -2113798144;
	// 82DDF9D4: 39296468  addi r9, r9, 0x6468
	ctx.r[9].s64 = ctx.r[9].s64 + 25704;
	// 82DDF9D8: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DDF9DC: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82DDF9E0: 390B000C  addi r8, r11, 0xc
	ctx.r[8].s64 = ctx.r[11].s64 + 12;
	// 82DDF9E4: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82DDF9E8: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	pc = 0x82DDF9EC; continue 'dispatch;
            }
            0x82DDF9EC => {
    //   block [0x82DDF9EC..0x82DDF9F4)
	// 82DDF9EC: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 82DDF9F0: 4BEC9A58  b 0x82ca9448
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDF9F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DDF9F8 size=428
    let mut pc: u32 = 0x82DDF9F8;
    'dispatch: loop {
        match pc {
            0x82DDF9F8 => {
    //   block [0x82DDF9F8..0x82DDFA54)
	// 82DDF9F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DDF9FC: 4BEC99FD  bl 0x82ca93f8
	ctx.lr = 0x82DDFA00;
	sub_82CA93D0(ctx, base);
	// 82DDFA00: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DDFA04: 830D0000  lwz r24, 0(r13)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDFA08: 3B200008  li r25, 8
	ctx.r[25].s64 = 8;
	// 82DDFA0C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DDFA10: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82DDFA14: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 82DDFA18: 7D59C02E  lwzx r10, r25, r24
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[25].u32.wrapping_add(ctx.r[24].u32)) } as u64;
	// 82DDFA1C: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DDFA20: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DDFA24: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DDFA28: 4098002C  bge cr6, 0x82ddfa54
	if !ctx.cr[6].lt {
	pc = 0x82DDFA54; continue 'dispatch;
	}
	// 82DDFA2C: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DDFA30: 3D008203  lis r8, -0x7dfd
	ctx.r[8].s64 = -2113732608;
	// 82DDFA34: 39293E84  addi r9, r9, 0x3e84
	ctx.r[9].s64 = ctx.r[9].s64 + 16004;
	// 82DDFA38: 39083E74  addi r8, r8, 0x3e74
	ctx.r[8].s64 = ctx.r[8].s64 + 15988;
	// 82DDFA3C: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DDFA40: 910B000C  stw r8, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 82DDFA44: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82DDFA48: 390B0010  addi r8, r11, 0x10
	ctx.r[8].s64 = ctx.r[11].s64 + 16;
	// 82DDFA4C: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82DDFA50: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	pc = 0x82DDFA54; continue 'dispatch;
            }
            0x82DDFA54 => {
    //   block [0x82DDFA54..0x82DDFB10)
	// 82DDFA54: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82DDFA58: 83A30000  lwz r29, 0(r3)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDFA5C: 9061006C  stw r3, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[3].u32 ) };
	// 82DDFA60: 38E0FFFF  li r7, -1
	ctx.r[7].s64 = -1;
	// 82DDFA64: 392A360C  addi r9, r10, 0x360c
	ctx.r[9].s64 = ctx.r[10].s64 + 13836;
	// 82DDFA68: 81430008  lwz r10, 8(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DDFA6C: 811E0000  lwz r8, 0(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDFA70: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82DDFA74: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDFA78: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82DDFA7C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82DDFA80: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82DDFA84: 91410068  stw r10, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[10].u32 ) };
	// 82DDFA88: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82DDFA8C: 815D0010  lwz r10, 0x10(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DDFA90: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 82DDFA94: 91410060  stw r10, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[10].u32 ) };
	// 82DDFA98: 814A000C  lwz r10, 0xc(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DDFA9C: 8388000C  lwz r28, 0xc(r8)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DDFAA0: 394A000D  addi r10, r10, 0xd
	ctx.r[10].s64 = ctx.r[10].s64 + 13;
	// 82DDFAA4: 554A2834  slwi r10, r10, 5
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(5);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DDFAA8: 7D4A5A14  add r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82DDFAAC: 7D4AE0AE  lbzx r10, r10, r28
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 82DDFAB0: 5548103E  rotlwi r8, r10, 2
	ctx.r[8].u64 = ((ctx.r[10].u32).rotate_left(2)) as u64;
	// 82DDFAB4: 7D4A4214  add r10, r10, r8
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[8].u64;
	// 82DDFAB8: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DDFABC: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82DDFAC0: 816B09A4  lwz r11, 0x9a4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2468 as u32) ) } as u64;
	// 82DDFAC4: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 82DDFAC8: 9B610054  stb r27, 0x54(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[27].u8 ) };
	// 82DDFACC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DDFAD0: 4E800421  bctrl
	ctx.lr = 0x82DDFAD4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DDFAD4: 89610054  lbz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82DDFAD8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DDFADC: 419A0084  beq cr6, 0x82ddfb60
	if ctx.cr[6].eq {
	pc = 0x82DDFB60; continue 'dispatch;
	}
	// 82DDFAE0: 7D59C02E  lwzx r10, r25, r24
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[25].u32.wrapping_add(ctx.r[24].u32)) } as u64;
	// 82DDFAE4: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DDFAE8: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DDFAEC: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DDFAF0: 40980020  bge cr6, 0x82ddfb10
	if !ctx.cr[6].lt {
	pc = 0x82DDFB10; continue 'dispatch;
	}
	// 82DDFAF4: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DDFAF8: 392936A4  addi r9, r9, 0x36a4
	ctx.r[9].s64 = ctx.r[9].s64 + 13988;
	// 82DDFAFC: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DDFB00: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82DDFB04: 390B000C  addi r8, r11, 0xc
	ctx.r[8].s64 = ctx.r[11].s64 + 12;
	// 82DDFB08: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82DDFB0C: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
            }
            0x82DDFB10 => {
    //   block [0x82DDFB10..0x82DDFB60)
	// 82DDFB10: 815D0018  lwz r10, 0x18(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DDFB14: 7F46D378  mr r6, r26
	ctx.r[6].u64 = ctx.r[26].u64;
	// 82DDFB18: 93610064  stw r27, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[27].u32 ) };
	// 82DDFB1C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82DDFB20: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDFB24: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82DDFB28: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82DDFB2C: 91410060  stw r10, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[10].u32 ) };
	// 82DDFB30: 814A000C  lwz r10, 0xc(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DDFB34: 394A000D  addi r10, r10, 0xd
	ctx.r[10].s64 = ctx.r[10].s64 + 13;
	// 82DDFB38: 554A2834  slwi r10, r10, 5
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(5);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DDFB3C: 7D4A5A14  add r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82DDFB40: 7D4AE0AE  lbzx r10, r10, r28
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 82DDFB44: 5549103E  rotlwi r9, r10, 2
	ctx.r[9].u64 = ((ctx.r[10].u32).rotate_left(2)) as u64;
	// 82DDFB48: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82DDFB4C: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DDFB50: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82DDFB54: 816B09A4  lwz r11, 0x9a4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2468 as u32) ) } as u64;
	// 82DDFB58: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DDFB5C: 4E800421  bctrl
	ctx.lr = 0x82DDFB60;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x82DDFB60 => {
    //   block [0x82DDFB60..0x82DDFB9C)
	// 82DDFB60: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DDFB64: 7D59C02E  lwzx r10, r25, r24
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[25].u32.wrapping_add(ctx.r[24].u32)) } as u64;
	// 82DDFB68: 396B325C  addi r11, r11, 0x325c
	ctx.r[11].s64 = ctx.r[11].s64 + 12892;
	// 82DDFB6C: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82DDFB70: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DDFB74: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DDFB78: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DDFB7C: 40980020  bge cr6, 0x82ddfb9c
	if !ctx.cr[6].lt {
	pc = 0x82DDFB9C; continue 'dispatch;
	}
	// 82DDFB80: 3D208202  lis r9, -0x7dfe
	ctx.r[9].s64 = -2113798144;
	// 82DDFB84: 39296468  addi r9, r9, 0x6468
	ctx.r[9].s64 = ctx.r[9].s64 + 25704;
	// 82DDFB88: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DDFB8C: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82DDFB90: 390B000C  addi r8, r11, 0xc
	ctx.r[8].s64 = ctx.r[11].s64 + 12;
	// 82DDFB94: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82DDFB98: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	pc = 0x82DDFB9C; continue 'dispatch;
            }
            0x82DDFB9C => {
    //   block [0x82DDFB9C..0x82DDFBA4)
	// 82DDFB9C: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 82DDFBA0: 4BEC98A8  b 0x82ca9448
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDFBA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DDFBA8 size=108
    let mut pc: u32 = 0x82DDFBA8;
    'dispatch: loop {
        match pc {
            0x82DDFBA8 => {
    //   block [0x82DDFBA8..0x82DDFC14)
	// 82DDFBA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DDFBAC: 4BEC9859  bl 0x82ca9404
	ctx.lr = 0x82DDFBB0;
	sub_82CA93D0(ctx, base);
	// 82DDFBB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DDFBB4: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDFBB8: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82DDFBBC: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82DDFBC0: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82DDFBC4: 38A0001E  li r5, 0x1e
	ctx.r[5].s64 = 30;
	// 82DDFBC8: 38800014  li r4, 0x14
	ctx.r[4].s64 = 20;
	// 82DDFBCC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DDFBD0: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DDFBD4: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 82DDFBD8: 4BF75671  bl 0x82d55248
	ctx.lr = 0x82DDFBDC;
	sub_82D55248(ctx, base);
	// 82DDFBDC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DDFBE0: 39600014  li r11, 0x14
	ctx.r[11].s64 = 20;
	// 82DDFBE4: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 82DDFBE8: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82DDFBEC: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82DDFBF0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82DDFBF4: B17F0004  sth r11, 4(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 82DDFBF8: 4BFFF1D1  bl 0x82ddedc8
	ctx.lr = 0x82DDFBFC;
	sub_82DDEDC8(ctx, base);
	// 82DDFBFC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DDFC00: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DDFC04: 396B3E98  addi r11, r11, 0x3e98
	ctx.r[11].s64 = ctx.r[11].s64 + 16024;
	// 82DDFC08: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DDFC0C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DDFC10: 4BEC9844  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDFC18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DDFC18 size=204
    let mut pc: u32 = 0x82DDFC18;
    'dispatch: loop {
        match pc {
            0x82DDFC18 => {
    //   block [0x82DDFC18..0x82DDFCE4)
	// 82DDFC18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DDFC1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DDFC20: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DDFC24: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DDFC28: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DDFC2C: 3D0082DE  lis r8, -0x7d22
	ctx.r[8].s64 = -2099380224;
	// 82DDFC30: 3D2082DE  lis r9, -0x7d22
	ctx.r[9].s64 = -2099380224;
	// 82DDFC34: 3D4082DE  lis r10, -0x7d22
	ctx.r[10].s64 = -2099380224;
	// 82DDFC38: 3D6082DE  lis r11, -0x7d22
	ctx.r[11].s64 = -2099380224;
	// 82DDFC3C: 3908FBA8  addi r8, r8, -0x458
	ctx.r[8].s64 = ctx.r[8].s64 + -1112;
	// 82DDFC40: 3929FCE8  addi r9, r9, -0x318
	ctx.r[9].s64 = ctx.r[9].s64 + -792;
	// 82DDFC44: 394AFD38  addi r10, r10, -0x2c8
	ctx.r[10].s64 = ctx.r[10].s64 + -712;
	// 82DDFC48: 396BFD88  addi r11, r11, -0x278
	ctx.r[11].s64 = ctx.r[11].s64 + -632;
	// 82DDFC4C: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 82DDFC50: 38C0001A  li r6, 0x1a
	ctx.r[6].s64 = 26;
	// 82DDFC54: 91010050  stw r8, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[8].u32 ) };
	// 82DDFC58: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 82DDFC5C: 91210054  stw r9, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[9].u32 ) };
	// 82DDFC60: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82DDFC64: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 82DDFC68: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DDFC6C: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82DDFC70: 9BC10060  stb r30, 0x60(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[30].u8 ) };
	// 82DDFC74: 9BC10061  stb r30, 0x61(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(97 as u32), ctx.r[30].u8 ) };
	// 82DDFC78: 4BFE14F1  bl 0x82dc1168
	ctx.lr = 0x82DDFC7C;
	sub_82DC1168(ctx, base);
	// 82DDFC7C: 3D6082DE  lis r11, -0x7d22
	ctx.r[11].s64 = -2099380224;
	// 82DDFC80: 9BC10081  stb r30, 0x81(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(129 as u32), ctx.r[30].u8 ) };
	// 82DDFC84: 3D0082DE  lis r8, -0x7d22
	ctx.r[8].s64 = -2099380224;
	// 82DDFC88: 396BF2D0  addi r11, r11, -0xd30
	ctx.r[11].s64 = ctx.r[11].s64 + -3376;
	// 82DDFC8C: 3D2082DE  lis r9, -0x7d22
	ctx.r[9].s64 = -2099380224;
	// 82DDFC90: 3D4082DE  lis r10, -0x7d22
	ctx.r[10].s64 = -2099380224;
	// 82DDFC94: 3908EEA0  addi r8, r8, -0x1160
	ctx.r[8].s64 = ctx.r[8].s64 + -4448;
	// 82DDFC98: 3929F9F8  addi r9, r9, -0x608
	ctx.r[9].s64 = ctx.r[9].s64 + -1544;
	// 82DDFC9C: 9161007C  stw r11, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[11].u32 ) };
	// 82DDFCA0: 394AF670  addi r10, r10, -0x990
	ctx.r[10].s64 = ctx.r[10].s64 + -2448;
	// 82DDFCA4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DDFCA8: 38C0FFFF  li r6, -1
	ctx.r[6].s64 = -1;
	// 82DDFCAC: 38A0001A  li r5, 0x1a
	ctx.r[5].s64 = 26;
	// 82DDFCB0: 91010070  stw r8, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[8].u32 ) };
	// 82DDFCB4: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 82DDFCB8: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82DDFCBC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DDFCC0: 91410078  stw r10, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[10].u32 ) };
	// 82DDFCC4: 99610080  stb r11, 0x80(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[11].u8 ) };
	// 82DDFCC8: 4BFE14A1  bl 0x82dc1168
	ctx.lr = 0x82DDFCCC;
	sub_82DC1168(ctx, base);
	// 82DDFCCC: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82DDFCD0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DDFCD4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DDFCD8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DDFCDC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DDFCE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDFCE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DDFCE8 size=76
    let mut pc: u32 = 0x82DDFCE8;
    'dispatch: loop {
        match pc {
            0x82DDFCE8 => {
    //   block [0x82DDFCE8..0x82DDFD34)
	// 82DDFCE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DDFCEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DDFCF0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DDFCF4: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82DDFCF8: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DDFCFC: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82DDFD00: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82DDFD04: 7CCA3378  mr r10, r6
	ctx.r[10].u64 = ctx.r[6].u64;
	// 82DDFD08: 39293280  addi r9, r9, 0x3280
	ctx.r[9].s64 = ctx.r[9].s64 + 12928;
	// 82DDFD0C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DDFD10: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82DDFD14: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 82DDFD18: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 82DDFD1C: 99610054  stb r11, 0x54(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u8 ) };
	// 82DDFD20: 4BFFFCD9  bl 0x82ddf9f8
	ctx.lr = 0x82DDFD24;
	sub_82DDF9F8(ctx, base);
	// 82DDFD24: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DDFD28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DDFD2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DDFD30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDFD38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DDFD38 size=80
    let mut pc: u32 = 0x82DDFD38;
    'dispatch: loop {
        match pc {
            0x82DDFD38 => {
    //   block [0x82DDFD38..0x82DDFD88)
	// 82DDFD38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DDFD3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DDFD40: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DDFD44: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DDFD48: 3D008200  lis r8, -0x7e00
	ctx.r[8].s64 = -2113929216;
	// 82DDFD4C: 7CCA3378  mr r10, r6
	ctx.r[10].u64 = ctx.r[6].u64;
	// 82DDFD50: 39293268  addi r9, r9, 0x3268
	ctx.r[9].s64 = ctx.r[9].s64 + 12904;
	// 82DDFD54: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82DDFD58: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82DDFD5C: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82DDFD60: C0080C64  lfs f0, 0xc64(r8)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(3172 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DDFD64: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82DDFD68: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82DDFD6C: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 82DDFD70: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 82DDFD74: 4BFFF8FD  bl 0x82ddf670
	ctx.lr = 0x82DDFD78;
	sub_82DDF670(ctx, base);
	// 82DDFD78: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DDFD7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DDFD80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DDFD84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDFD88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DDFD88 size=232
    let mut pc: u32 = 0x82DDFD88;
    'dispatch: loop {
        match pc {
            0x82DDFD88 => {
    //   block [0x82DDFD88..0x82DDFDB4)
	// 82DDFD88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DDFD8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DDFD90: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DDFD94: 9421FED0  stwu r1, -0x130(r1)
	ea = ctx.r[1].u32.wrapping_add(-304 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DDFD98: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DDFD9C: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82DDFDA0: 7CE83B78  mr r8, r7
	ctx.r[8].u64 = ctx.r[7].u64;
	// 82DDFDA4: 394100B0  addi r10, r1, 0xb0
	ctx.r[10].s64 = ctx.r[1].s64 + 176;
	// 82DDFDA8: 7CAB2B78  mr r11, r5
	ctx.r[11].u64 = ctx.r[5].u64;
	// 82DDFDAC: 3920000E  li r9, 0xe
	ctx.r[9].s64 = 14;
	// 82DDFDB0: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	pc = 0x82DDFDB4; continue 'dispatch;
            }
            0x82DDFDB4 => {
    //   block [0x82DDFDB4..0x82DDFE70)
	// 82DDFDB4: E92B0000  ld r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82DDFDB8: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82DDFDBC: F92A0000  std r9, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u64 ) };
	// 82DDFDC0: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 82DDFDC4: 4200FFF0  bdnz 0x82ddfdb4
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82DDFDB4; continue 'dispatch;
	}
	// 82DDFDC8: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DDFDCC: 39650050  addi r11, r5, 0x50
	ctx.r[11].s64 = ctx.r[5].s64 + 80;
	// 82DDFDD0: 39292AF0  addi r9, r9, 0x2af0
	ctx.r[9].s64 = ctx.r[9].s64 + 10992;
	// 82DDFDD4: 3CE08200  lis r7, -0x7e00
	ctx.r[7].s64 = -2113929216;
	// 82DDFDD8: 39410060  addi r10, r1, 0x60
	ctx.r[10].s64 = ctx.r[1].s64 + 96;
	// 82DDFDDC: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDFE70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DDFE70 size=16
    let mut pc: u32 = 0x82DDFE70;
    'dispatch: loop {
        match pc {
            0x82DDFE70 => {
    //   block [0x82DDFE70..0x82DDFE80)
	// 82DDFE70: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82DDFE74: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82DDFE78: 7D655B78  mr r5, r11
	ctx.r[5].u64 = ctx.r[11].u64;
	// 82DDFE7C: 4BFFEEAC  b 0x82dded28
	sub_82DDED28(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDFE80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DDFE80 size=76
    let mut pc: u32 = 0x82DDFE80;
    'dispatch: loop {
        match pc {
            0x82DDFE80 => {
    //   block [0x82DDFE80..0x82DDFECC)
	// 82DDFE80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DDFE84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DDFE88: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DDFE8C: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82DDFE90: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DDFE94: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82DDFE98: 7D655B78  mr r5, r11
	ctx.r[5].u64 = ctx.r[11].u64;
	// 82DDFE9C: 7CEA3B78  mr r10, r7
	ctx.r[10].u64 = ctx.r[7].u64;
	// 82DDFEA0: 39293280  addi r9, r9, 0x3280
	ctx.r[9].s64 = ctx.r[9].s64 + 12928;
	// 82DDFEA4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DDFEA8: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 82DDFEAC: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 82DDFEB0: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 82DDFEB4: 99610054  stb r11, 0x54(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u8 ) };
	// 82DDFEB8: 4BFFF969  bl 0x82ddf820
	ctx.lr = 0x82DDFEBC;
	sub_82DDF820(ctx, base);
	// 82DDFEBC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DDFEC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DDFEC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DDFEC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDFED0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DDFED0 size=80
    let mut pc: u32 = 0x82DDFED0;
    'dispatch: loop {
        match pc {
            0x82DDFED0 => {
    //   block [0x82DDFED0..0x82DDFF20)
	// 82DDFED0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DDFED4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DDFED8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DDFEDC: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DDFEE0: 3D008200  lis r8, -0x7e00
	ctx.r[8].s64 = -2113929216;
	// 82DDFEE4: 7CEA3B78  mr r10, r7
	ctx.r[10].u64 = ctx.r[7].u64;
	// 82DDFEE8: 39293268  addi r9, r9, 0x3268
	ctx.r[9].s64 = ctx.r[9].s64 + 12904;
	// 82DDFEEC: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82DDFEF0: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82DDFEF4: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 82DDFEF8: C0080C64  lfs f0, 0xc64(r8)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(3172 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DDFEFC: 7D655B78  mr r5, r11
	ctx.r[5].u64 = ctx.r[11].u64;
	// 82DDFF00: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82DDFF04: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 82DDFF08: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 82DDFF0C: 4BFFF58D  bl 0x82ddf498
	ctx.lr = 0x82DDFF10;
	sub_82DDF498(ctx, base);
	// 82DDFF10: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DDFF14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DDFF18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DDFF1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDFF20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DDFF20 size=176
    let mut pc: u32 = 0x82DDFF20;
    'dispatch: loop {
        match pc {
            0x82DDFF20 => {
    //   block [0x82DDFF20..0x82DDFFA0)
	// 82DDFF20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DDFF24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DDFF28: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DDFF2C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DDFF30: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 82DDFF34: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DDFF38: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 82DDFF3C: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82DDFF40: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82DDFF44: 7D655B78  mr r5, r11
	ctx.r[5].u64 = ctx.r[11].u64;
	// 82DDFF48: 83FE0000  lwz r31, 0(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDFF4C: C3FE3030  lfs f31, 0x3030(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12336 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82DDFF50: 4BFFEFA9  bl 0x82ddeef8
	ctx.lr = 0x82DDFF54;
	sub_82DDEEF8(ctx, base);
	// 82DDFF54: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DDFF58: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82DDFF5C: 40980044  bge cr6, 0x82ddffa0
	if !ctx.cr[6].lt {
	pc = 0x82DDFFA0; continue 'dispatch;
	}
	// 82DDFF60: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DDFF64: 394BFC80  addi r10, r11, -0x380
	ctx.r[10].s64 = ctx.r[11].s64 + -896;
	// 82DDFF68: 397F0010  addi r11, r31, 0x10
	ctx.r[11].s64 = ctx.r[31].s64 + 16;
	pc = 0x82DDFFA0; continue 'dispatch;
            }
            0x82DDFFA0 => {
    //   block [0x82DDFFA0..0x82DDFFD0)
	// 82DDFFA0: C01E3030  lfs f0, 0x3030(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12336 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DDFFA4: FF1F0000  fcmpu cr6, f31, f0
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[0].f64);
	// 82DDFFA8: 419A000C  beq cr6, 0x82ddffb4
	if ctx.cr[6].eq {
	pc = 0x82DDFFB4; continue 'dispatch;
	}
	// 82DDFFAC: 387E3010  addi r3, r30, 0x3010
	ctx.r[3].s64 = ctx.r[30].s64 + 12304;
	// 82DDFFB0: 4BFEAFD9  bl 0x82dcaf88
	ctx.lr = 0x82DDFFB4;
	sub_82DCAF88(ctx, base);
	// 82DDFFB4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DDFFB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DDFFBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DDFFC0: CBE1FFE0  lfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 82DDFFC4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DDFFC8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DDFFCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DDFFD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DDFFD0 size=240
    let mut pc: u32 = 0x82DDFFD0;
    'dispatch: loop {
        match pc {
            0x82DDFFD0 => {
    //   block [0x82DDFFD0..0x82DE0000)
	// 82DDFFD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DDFFD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DDFFD8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DDFFDC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DDFFE0: 9421FEC0  stwu r1, -0x140(r1)
	ea = ctx.r[1].u32.wrapping_add(-320 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DDFFE4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DDFFE8: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82DDFFEC: 7D1F4378  mr r31, r8
	ctx.r[31].u64 = ctx.r[8].u64;
	// 82DDFFF0: 394100B0  addi r10, r1, 0xb0
	ctx.r[10].s64 = ctx.r[1].s64 + 176;
	// 82DDFFF4: 7CCB3378  mr r11, r6
	ctx.r[11].u64 = ctx.r[6].u64;
	// 82DDFFF8: 3920000E  li r9, 0xe
	ctx.r[9].s64 = 14;
	// 82DDFFFC: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	pc = 0x82DE0000; continue 'dispatch;
            }
            0x82DE0000 => {
    //   block [0x82DE0000..0x82DE00C0)
	// 82DE0000: E92B0000  ld r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82DE0004: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82DE0008: F92A0000  std r9, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u64 ) };
	// 82DE000C: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 82DE0010: 4200FFF0  bdnz 0x82de0000
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82DE0000; continue 'dispatch;
	}
	// 82DE0014: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DE0018: 39660050  addi r11, r6, 0x50
	ctx.r[11].s64 = ctx.r[6].s64 + 80;
	// 82DE001C: 39292AF0  addi r9, r9, 0x2af0
	ctx.r[9].s64 = ctx.r[9].s64 + 10992;
	// 82DE0020: 3D008200  lis r8, -0x7e00
	ctx.r[8].s64 = -2113929216;
	// 82DE0024: 39410060  addi r10, r1, 0x60
	ctx.r[10].s64 = ctx.r[1].s64 + 96;
	// 82DE0028: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE00C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DE00C0 size=324
    let mut pc: u32 = 0x82DE00C0;
    'dispatch: loop {
        match pc {
            0x82DE00C0 => {
    //   block [0x82DE00C0..0x82DE0204)
	// 82DE00C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DE00C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DE00C8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DE00CC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DE00D0: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82DE00D4: 38800003  li r4, 3
	ctx.r[4].s64 = 3;
	// 82DE00D8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DE00DC: 4BFE1445  bl 0x82dc1520
	ctx.lr = 0x82DE00E0;
	sub_82DC1520(ctx, base);
	// 82DE00E0: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82DE00E4: 38800005  li r4, 5
	ctx.r[4].s64 = 5;
	// 82DE00E8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DE00EC: 4BFE1435  bl 0x82dc1520
	ctx.lr = 0x82DE00F0;
	sub_82DC1520(ctx, base);
	// 82DE00F0: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82DE00F4: 38800006  li r4, 6
	ctx.r[4].s64 = 6;
	// 82DE00F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DE00FC: 4BFE1425  bl 0x82dc1520
	ctx.lr = 0x82DE0100;
	sub_82DC1520(ctx, base);
	// 82DE0100: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82DE0104: 38800007  li r4, 7
	ctx.r[4].s64 = 7;
	// 82DE0108: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DE010C: 4BFE1415  bl 0x82dc1520
	ctx.lr = 0x82DE0110;
	sub_82DC1520(ctx, base);
	// 82DE0110: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82DE0114: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 82DE0118: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DE011C: 4BFE1405  bl 0x82dc1520
	ctx.lr = 0x82DE0120;
	sub_82DC1520(ctx, base);
	// 82DE0120: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82DE0124: 38800008  li r4, 8
	ctx.r[4].s64 = 8;
	// 82DE0128: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DE012C: 4BFE13F5  bl 0x82dc1520
	ctx.lr = 0x82DE0130;
	sub_82DC1520(ctx, base);
	// 82DE0130: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82DE0134: 38800009  li r4, 9
	ctx.r[4].s64 = 9;
	// 82DE0138: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DE013C: 4BFE13E5  bl 0x82dc1520
	ctx.lr = 0x82DE0140;
	sub_82DC1520(ctx, base);
	// 82DE0140: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82DE0144: 3880000D  li r4, 0xd
	ctx.r[4].s64 = 13;
	// 82DE0148: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DE014C: 4BFE13D5  bl 0x82dc1520
	ctx.lr = 0x82DE0150;
	sub_82DC1520(ctx, base);
	// 82DE0150: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82DE0154: 3880000E  li r4, 0xe
	ctx.r[4].s64 = 14;
	// 82DE0158: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DE015C: 4BFE13C5  bl 0x82dc1520
	ctx.lr = 0x82DE0160;
	sub_82DC1520(ctx, base);
	// 82DE0160: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82DE0164: 38800011  li r4, 0x11
	ctx.r[4].s64 = 17;
	// 82DE0168: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DE016C: 4BFE13B5  bl 0x82dc1520
	ctx.lr = 0x82DE0170;
	sub_82DC1520(ctx, base);
	// 82DE0170: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 82DE0174: 38800014  li r4, 0x14
	ctx.r[4].s64 = 20;
	// 82DE0178: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DE017C: 4BFE13A5  bl 0x82dc1520
	ctx.lr = 0x82DE0180;
	sub_82DC1520(ctx, base);
	// 82DE0180: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 82DE0184: 3880000A  li r4, 0xa
	ctx.r[4].s64 = 10;
	// 82DE0188: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DE018C: 4BFE1395  bl 0x82dc1520
	ctx.lr = 0x82DE0190;
	sub_82DC1520(ctx, base);
	// 82DE0190: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 82DE0194: 3880000F  li r4, 0xf
	ctx.r[4].s64 = 15;
	// 82DE0198: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DE019C: 4BFE1385  bl 0x82dc1520
	ctx.lr = 0x82DE01A0;
	sub_82DC1520(ctx, base);
	// 82DE01A0: 38A00015  li r5, 0x15
	ctx.r[5].s64 = 21;
	// 82DE01A4: 3880000B  li r4, 0xb
	ctx.r[4].s64 = 11;
	// 82DE01A8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DE01AC: 4BFE1375  bl 0x82dc1520
	ctx.lr = 0x82DE01B0;
	sub_82DC1520(ctx, base);
	// 82DE01B0: 38A00015  li r5, 0x15
	ctx.r[5].s64 = 21;
	// 82DE01B4: 3880000C  li r4, 0xc
	ctx.r[4].s64 = 12;
	// 82DE01B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DE01BC: 4BFE1365  bl 0x82dc1520
	ctx.lr = 0x82DE01C0;
	sub_82DC1520(ctx, base);
	// 82DE01C0: 38A00019  li r5, 0x19
	ctx.r[5].s64 = 25;
	// 82DE01C4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82DE01C8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DE01CC: 4BFE1355  bl 0x82dc1520
	ctx.lr = 0x82DE01D0;
	sub_82DC1520(ctx, base);
	// 82DE01D0: 38A00017  li r5, 0x17
	ctx.r[5].s64 = 23;
	// 82DE01D4: 3880001B  li r4, 0x1b
	ctx.r[4].s64 = 27;
	// 82DE01D8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DE01DC: 4BFE1345  bl 0x82dc1520
	ctx.lr = 0x82DE01E0;
	sub_82DC1520(ctx, base);
	// 82DE01E0: 38A00017  li r5, 0x17
	ctx.r[5].s64 = 23;
	// 82DE01E4: 38800018  li r4, 0x18
	ctx.r[4].s64 = 24;
	// 82DE01E8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DE01EC: 4BFE1335  bl 0x82dc1520
	ctx.lr = 0x82DE01F0;
	sub_82DC1520(ctx, base);
	// 82DE01F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DE01F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DE01F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DE01FC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DE0200: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE0208(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DE0208 size=580
    let mut pc: u32 = 0x82DE0208;
    'dispatch: loop {
        match pc {
            0x82DE0208 => {
    //   block [0x82DE0208..0x82DE02B4)
	// 82DE0208: 39630001  addi r11, r3, 1
	ctx.r[11].s64 = ctx.r[3].s64 + 1;
	// 82DE020C: 2B0B0021  cmplwi cr6, r11, 0x21
	ctx.cr[6].compare_u32(ctx.r[11].u32, 33 as u32, &mut ctx.xer);
	// 82DE0210: 4199023C  bgt cr6, 0x82de044c
	if ctx.cr[6].gt {
		sub_82DE044C(ctx, base);
		return;
	}
	// 82DE0214: 3D8082DE  lis r12, -0x7d22
	ctx.r[12].s64 = -2099380224;
	// 82DE0218: 398C022C  addi r12, r12, 0x22c
	ctx.r[12].s64 = ctx.r[12].s64 + 556;
	// 82DE021C: 5560103A  slwi r0, r11, 2
	ctx.r[0].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[0].u64 = ctx.r[0].u32 as u64;
	// 82DE0220: 7C0C002E  lwzx r0, r12, r0
	ctx.r[0].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[12].u32.wrapping_add(ctx.r[0].u32)) } as u64;
	// 82DE0224: 7C0903A6  mtctr r0
	ctx.ctr.u64 = ctx.r[0].u64;
	// 82DE0228: 4E800420  bctr
	match ctx.r[11].u64 {
		0 => {
	pc = 0x82DE02C0; continue 'dispatch;
		},
		1 => {
	pc = 0x82DE02B4; continue 'dispatch;
		},
		2 => {
	pc = 0x82DE02CC; continue 'dispatch;
		},
		3 => {
	pc = 0x82DE02D8; continue 'dispatch;
		},
		4 => {
	pc = 0x82DE02F0; continue 'dispatch;
		},
		5 => {
	pc = 0x82DE0320; continue 'dispatch;
		},
		6 => {
	pc = 0x82DE02FC; continue 'dispatch;
		},
		7 => {
	pc = 0x82DE0308; continue 'dispatch;
		},
		8 => {
	pc = 0x82DE0314; continue 'dispatch;
		},
		9 => {
	pc = 0x82DE032C; continue 'dispatch;
		},
		10 => {
	pc = 0x82DE0338; continue 'dispatch;
		},
		11 => {
	pc = 0x82DE035C; continue 'dispatch;
		},
		12 => {
	pc = 0x82DE03D4; continue 'dispatch;
		},
		13 => {
	pc = 0x82DE0368; continue 'dispatch;
		},
		14 => {
	pc = 0x82DE03EC; continue 'dispatch;
		},
		15 => {
	pc = 0x82DE03F8; continue 'dispatch;
		},
		16 => {
	pc = 0x82DE0404; continue 'dispatch;
		},
		17 => {
	pc = 0x82DE0410; continue 'dispatch;
		},
		18 => {
	pc = 0x82DE0344; continue 'dispatch;
		},
		19 => {
	pc = 0x82DE0350; continue 'dispatch;
		},
		20 => {
	pc = 0x82DE0374; continue 'dispatch;
		},
		21 => {
	pc = 0x82DE0380; continue 'dispatch;
		},
		22 => {
	pc = 0x82DE02E4; continue 'dispatch;
		},
		23 => {
	pc = 0x82DE038C; continue 'dispatch;
		},
		24 => {
	pc = 0x82DE0398; continue 'dispatch;
		},
		25 => {
	pc = 0x82DE03A4; continue 'dispatch;
		},
		26 => {
	pc = 0x82DE03B0; continue 'dispatch;
		},
		27 => {
	pc = 0x82DE03BC; continue 'dispatch;
		},
		28 => {
	pc = 0x82DE03C8; continue 'dispatch;
		},
		29 => {
	pc = 0x82DE03E0; continue 'dispatch;
		},
		30 => {
	pc = 0x82DE041C; continue 'dispatch;
		},
		31 => {
	pc = 0x82DE0428; continue 'dispatch;
		},
		32 => {
	pc = 0x82DE0434; continue 'dispatch;
		},
		33 => {
	pc = 0x82DE0440; continue 'dispatch;
		},
		_ => unsafe { core::hint::unreachable_unchecked() },
	}
	// 82DE022C: 82DE02C0  lwz r22, 0x2c0(r30)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(704 as u32) ) } as u64;
	// 82DE0230: 82DE02B4  lwz r22, 0x2b4(r30)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(692 as u32) ) } as u64;
	// 82DE0234: 82DE02CC  lwz r22, 0x2cc(r30)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(716 as u32) ) } as u64;
	// 82DE0238: 82DE02D8  lwz r22, 0x2d8(r30)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(728 as u32) ) } as u64;
	// 82DE023C: 82DE02F0  lwz r22, 0x2f0(r30)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(752 as u32) ) } as u64;
	// 82DE0240: 82DE0320  lwz r22, 0x320(r30)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(800 as u32) ) } as u64;
	// 82DE0244: 82DE02FC  lwz r22, 0x2fc(r30)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(764 as u32) ) } as u64;
	// 82DE0248: 82DE0308  lwz r22, 0x308(r30)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(776 as u32) ) } as u64;
	// 82DE024C: 82DE0314  lwz r22, 0x314(r30)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(788 as u32) ) } as u64;
	// 82DE0250: 82DE032C  lwz r22, 0x32c(r30)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(812 as u32) ) } as u64;
	// 82DE0254: 82DE0338  lwz r22, 0x338(r30)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(824 as u32) ) } as u64;
	// 82DE0258: 82DE035C  lwz r22, 0x35c(r30)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(860 as u32) ) } as u64;
	// 82DE025C: 82DE03D4  lwz r22, 0x3d4(r30)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(980 as u32) ) } as u64;
	// 82DE0260: 82DE0368  lwz r22, 0x368(r30)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(872 as u32) ) } as u64;
	// 82DE0264: 82DE03EC  lwz r22, 0x3ec(r30)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(1004 as u32) ) } as u64;
	// 82DE0268: 82DE03F8  lwz r22, 0x3f8(r30)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(1016 as u32) ) } as u64;
	// 82DE026C: 82DE0404  lwz r22, 0x404(r30)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(1028 as u32) ) } as u64;
	// 82DE0270: 82DE0410  lwz r22, 0x410(r30)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(1040 as u32) ) } as u64;
	// 82DE0274: 82DE0344  lwz r22, 0x344(r30)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(836 as u32) ) } as u64;
	// 82DE0278: 82DE0350  lwz r22, 0x350(r30)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(848 as u32) ) } as u64;
	// 82DE027C: 82DE0374  lwz r22, 0x374(r30)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(884 as u32) ) } as u64;
	// 82DE0280: 82DE0380  lwz r22, 0x380(r30)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(896 as u32) ) } as u64;
	// 82DE0284: 82DE02E4  lwz r22, 0x2e4(r30)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(740 as u32) ) } as u64;
	// 82DE0288: 82DE038C  lwz r22, 0x38c(r30)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(908 as u32) ) } as u64;
	// 82DE028C: 82DE0398  lwz r22, 0x398(r30)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(920 as u32) ) } as u64;
	// 82DE0290: 82DE03A4  lwz r22, 0x3a4(r30)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(932 as u32) ) } as u64;
	// 82DE0294: 82DE03B0  lwz r22, 0x3b0(r30)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(944 as u32) ) } as u64;
	// 82DE0298: 82DE03BC  lwz r22, 0x3bc(r30)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(956 as u32) ) } as u64;
	// 82DE029C: 82DE03C8  lwz r22, 0x3c8(r30)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(968 as u32) ) } as u64;
	// 82DE02A0: 82DE03E0  lwz r22, 0x3e0(r30)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(992 as u32) ) } as u64;
	// 82DE02A4: 82DE041C  lwz r22, 0x41c(r30)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(1052 as u32) ) } as u64;
	// 82DE02A8: 82DE0428  lwz r22, 0x428(r30)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(1064 as u32) ) } as u64;
	// 82DE02AC: 82DE0434  lwz r22, 0x434(r30)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(1076 as u32) ) } as u64;
	// 82DE02B0: 82DE0440  lwz r22, 0x440(r30)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(1088 as u32) ) } as u64;
            }
            0x82DE02B4 => {
    //   block [0x82DE02B4..0x82DE02C0)
	// 82DE02B4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DE02B8: 386B4190  addi r3, r11, 0x4190
	ctx.r[3].s64 = ctx.r[11].s64 + 16784;
	// 82DE02BC: 4E800020  blr
	return;
            }
            0x82DE02C0 => {
    //   block [0x82DE02C0..0x82DE02CC)
	// 82DE02C0: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DE02C4: 386B4180  addi r3, r11, 0x4180
	ctx.r[3].s64 = ctx.r[11].s64 + 16768;
	// 82DE02C8: 4E800020  blr
	return;
            }
            0x82DE02CC => {
    //   block [0x82DE02CC..0x82DE02D8)
	// 82DE02CC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DE02D0: 386B4170  addi r3, r11, 0x4170
	ctx.r[3].s64 = ctx.r[11].s64 + 16752;
	// 82DE02D4: 4E800020  blr
	return;
            }
            0x82DE02D8 => {
    //   block [0x82DE02D8..0x82DE02E4)
	// 82DE02D8: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DE02DC: 386B415C  addi r3, r11, 0x415c
	ctx.r[3].s64 = ctx.r[11].s64 + 16732;
	// 82DE02E0: 4E800020  blr
	return;
            }
            0x82DE02E4 => {
    //   block [0x82DE02E4..0x82DE02F0)
	// 82DE02E4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DE02E8: 386B4148  addi r3, r11, 0x4148
	ctx.r[3].s64 = ctx.r[11].s64 + 16712;
	// 82DE02EC: 4E800020  blr
	return;
            }
            0x82DE02F0 => {
    //   block [0x82DE02F0..0x82DE02FC)
	// 82DE02F0: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DE02F4: 386B4138  addi r3, r11, 0x4138
	ctx.r[3].s64 = ctx.r[11].s64 + 16696;
	// 82DE02F8: 4E800020  blr
	return;
            }
            0x82DE02FC => {
    //   block [0x82DE02FC..0x82DE0308)
	// 82DE02FC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DE0300: 386B4124  addi r3, r11, 0x4124
	ctx.r[3].s64 = ctx.r[11].s64 + 16676;
	// 82DE0304: 4E800020  blr
	return;
            }
            0x82DE0308 => {
    //   block [0x82DE0308..0x82DE0314)
	// 82DE0308: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DE030C: 386B4114  addi r3, r11, 0x4114
	ctx.r[3].s64 = ctx.r[11].s64 + 16660;
	// 82DE0310: 4E800020  blr
	return;
            }
            0x82DE0314 => {
    //   block [0x82DE0314..0x82DE0320)
	// 82DE0314: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DE0318: 386B4100  addi r3, r11, 0x4100
	ctx.r[3].s64 = ctx.r[11].s64 + 16640;
	// 82DE031C: 4E800020  blr
	return;
            }
            0x82DE0320 => {
    //   block [0x82DE0320..0x82DE032C)
	// 82DE0320: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DE0324: 386B40EC  addi r3, r11, 0x40ec
	ctx.r[3].s64 = ctx.r[11].s64 + 16620;
	// 82DE0328: 4E800020  blr
	return;
            }
            0x82DE032C => {
    //   block [0x82DE032C..0x82DE0338)
	// 82DE032C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DE0330: 386B40D0  addi r3, r11, 0x40d0
	ctx.r[3].s64 = ctx.r[11].s64 + 16592;
	// 82DE0334: 4E800020  blr
	return;
            }
            0x82DE0338 => {
    //   block [0x82DE0338..0x82DE0344)
	// 82DE0338: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DE033C: 386B40B0  addi r3, r11, 0x40b0
	ctx.r[3].s64 = ctx.r[11].s64 + 16560;
	// 82DE0340: 4E800020  blr
	return;
            }
            0x82DE0344 => {
    //   block [0x82DE0344..0x82DE0350)
	// 82DE0344: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DE0348: 386B4098  addi r3, r11, 0x4098
	ctx.r[3].s64 = ctx.r[11].s64 + 16536;
	// 82DE034C: 4E800020  blr
	return;
            }
            0x82DE0350 => {
    //   block [0x82DE0350..0x82DE035C)
	// 82DE0350: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DE0354: 386B4080  addi r3, r11, 0x4080
	ctx.r[3].s64 = ctx.r[11].s64 + 16512;
	// 82DE0358: 4E800020  blr
	return;
            }
            0x82DE035C => {
    //   block [0x82DE035C..0x82DE0368)
	// 82DE035C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DE0360: 386B4070  addi r3, r11, 0x4070
	ctx.r[3].s64 = ctx.r[11].s64 + 16496;
	// 82DE0364: 4E800020  blr
	return;
            }
            0x82DE0368 => {
    //   block [0x82DE0368..0x82DE0374)
	// 82DE0368: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DE036C: 386B4058  addi r3, r11, 0x4058
	ctx.r[3].s64 = ctx.r[11].s64 + 16472;
	// 82DE0370: 4E800020  blr
	return;
            }
            0x82DE0374 => {
    //   block [0x82DE0374..0x82DE0380)
	// 82DE0374: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DE0378: 386B4040  addi r3, r11, 0x4040
	ctx.r[3].s64 = ctx.r[11].s64 + 16448;
	// 82DE037C: 4E800020  blr
	return;
            }
            0x82DE0380 => {
    //   block [0x82DE0380..0x82DE038C)
	// 82DE0380: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DE0384: 386B4020  addi r3, r11, 0x4020
	ctx.r[3].s64 = ctx.r[11].s64 + 16416;
	// 82DE0388: 4E800020  blr
	return;
            }
            0x82DE038C => {
    //   block [0x82DE038C..0x82DE0398)
	// 82DE038C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DE0390: 386B400C  addi r3, r11, 0x400c
	ctx.r[3].s64 = ctx.r[11].s64 + 16396;
	// 82DE0394: 4E800020  blr
	return;
            }
            0x82DE0398 => {
    //   block [0x82DE0398..0x82DE03A4)
	// 82DE0398: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DE039C: 386B3FF4  addi r3, r11, 0x3ff4
	ctx.r[3].s64 = ctx.r[11].s64 + 16372;
	// 82DE03A0: 4E800020  blr
	return;
            }
            0x82DE03A4 => {
    //   block [0x82DE03A4..0x82DE03B0)
	// 82DE03A4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DE03A8: 386B3FD4  addi r3, r11, 0x3fd4
	ctx.r[3].s64 = ctx.r[11].s64 + 16340;
	// 82DE03AC: 4E800020  blr
	return;
            }
            0x82DE03B0 => {
    //   block [0x82DE03B0..0x82DE03BC)
	// 82DE03B0: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DE03B4: 386B3FC0  addi r3, r11, 0x3fc0
	ctx.r[3].s64 = ctx.r[11].s64 + 16320;
	// 82DE03B8: 4E800020  blr
	return;
            }
            0x82DE03BC => {
    //   block [0x82DE03BC..0x82DE03C8)
	// 82DE03BC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DE03C0: 386B3FB4  addi r3, r11, 0x3fb4
	ctx.r[3].s64 = ctx.r[11].s64 + 16308;
	// 82DE03C4: 4E800020  blr
	return;
            }
            0x82DE03C8 => {
    //   block [0x82DE03C8..0x82DE03D4)
	// 82DE03C8: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DE03CC: 386B3FA4  addi r3, r11, 0x3fa4
	ctx.r[3].s64 = ctx.r[11].s64 + 16292;
	// 82DE03D0: 4E800020  blr
	return;
            }
            0x82DE03D4 => {
    //   block [0x82DE03D4..0x82DE03E0)
	// 82DE03D4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DE03D8: 386B3F94  addi r3, r11, 0x3f94
	ctx.r[3].s64 = ctx.r[11].s64 + 16276;
	// 82DE03DC: 4E800020  blr
	return;
            }
            0x82DE03E0 => {
    //   block [0x82DE03E0..0x82DE03EC)
	// 82DE03E0: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DE03E4: 386B3F80  addi r3, r11, 0x3f80
	ctx.r[3].s64 = ctx.r[11].s64 + 16256;
	// 82DE03E8: 4E800020  blr
	return;
            }
            0x82DE03EC => {
    //   block [0x82DE03EC..0x82DE03F8)
	// 82DE03EC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DE03F0: 386B3F64  addi r3, r11, 0x3f64
	ctx.r[3].s64 = ctx.r[11].s64 + 16228;
	// 82DE03F4: 4E800020  blr
	return;
            }
            0x82DE03F8 => {
    //   block [0x82DE03F8..0x82DE0404)
	// 82DE03F8: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DE03FC: 386B3F48  addi r3, r11, 0x3f48
	ctx.r[3].s64 = ctx.r[11].s64 + 16200;
	// 82DE0400: 4E800020  blr
	return;
            }
            0x82DE0404 => {
    //   block [0x82DE0404..0x82DE0410)
	// 82DE0404: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DE0408: 386B3F30  addi r3, r11, 0x3f30
	ctx.r[3].s64 = ctx.r[11].s64 + 16176;
	// 82DE040C: 4E800020  blr
	return;
            }
            0x82DE0410 => {
    //   block [0x82DE0410..0x82DE041C)
	// 82DE0410: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DE0414: 386B3F1C  addi r3, r11, 0x3f1c
	ctx.r[3].s64 = ctx.r[11].s64 + 16156;
	// 82DE0418: 4E800020  blr
	return;
            }
            0x82DE041C => {
    //   block [0x82DE041C..0x82DE0428)
	// 82DE041C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DE0420: 386B3F00  addi r3, r11, 0x3f00
	ctx.r[3].s64 = ctx.r[11].s64 + 16128;
	// 82DE0424: 4E800020  blr
	return;
            }
            0x82DE0428 => {
    //   block [0x82DE0428..0x82DE0434)
	// 82DE0428: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DE042C: 386B3EF0  addi r3, r11, 0x3ef0
	ctx.r[3].s64 = ctx.r[11].s64 + 16112;
	// 82DE0430: 4E800020  blr
	return;
            }
            0x82DE0434 => {
    //   block [0x82DE0434..0x82DE0440)
	// 82DE0434: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DE0438: 386B3EE0  addi r3, r11, 0x3ee0
	ctx.r[3].s64 = ctx.r[11].s64 + 16096;
	// 82DE043C: 4E800020  blr
	return;
            }
            0x82DE0440 => {
    //   block [0x82DE0440..0x82DE044C)
	// 82DE0440: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DE0444: 386B3ED0  addi r3, r11, 0x3ed0
	ctx.r[3].s64 = ctx.r[11].s64 + 16080;
	// 82DE0448: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE044C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DE044C size=8
    let mut pc: u32 = 0x82DE044C;
    'dispatch: loop {
        match pc {
            0x82DE044C => {
    //   block [0x82DE044C..0x82DE0454)
	// 82DE044C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82DE0450: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE0458(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82DE0458 size=308
    let mut pc: u32 = 0x82DE0458;
    'dispatch: loop {
        match pc {
            0x82DE0458 => {
    //   block [0x82DE0458..0x82DE04D8)
	// 82DE0458: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DE045C: 4BEC8FAD  bl 0x82ca9408
	ctx.lr = 0x82DE0460;
	sub_82CA93D0(ctx, base);
	// 82DE0460: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 82DE0464: 816A0024  lwz r11, 0x24(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(36 as u32) ) } as u64;
	// 82DE0468: 812A0034  lwz r9, 0x34(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(52 as u32) ) } as u64;
	// 82DE046C: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82DE0470: 810B0014  lwz r8, 0x14(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DE0474: 812B0018  lwz r9, 0x18(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DE0478: 806B000C  lwz r3, 0xc(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DE047C: 7CE821D6  mullw r7, r8, r4
	ctx.r[7].s32 = ((ctx.r[8].s32 as i64 * ctx.r[4].s32 as i64) as i32);
	ctx.r[7].s64 = ctx.r[7].s32 as i64;
	// 82DE0480: 810B0000  lwz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE0484: 7D262038  and r6, r9, r4
	ctx.r[6].u64 = ctx.r[9].u64 & ctx.r[4].u64;
	// 82DE0488: 7CE71A14  add r7, r7, r3
	ctx.r[7].u64 = ctx.r[7].u64 + ctx.r[3].u64;
	// 82DE048C: 812B0004  lwz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DE0490: 68C30001  xori r3, r6, 1
	ctx.r[3].u64 = ctx.r[6].u64 ^ 1;
	// 82DE0494: 38C60001  addi r6, r6, 1
	ctx.r[6].s64 = ctx.r[6].s64 + 1;
	// 82DE0498: 38630001  addi r3, r3, 1
	ctx.r[3].s64 = ctx.r[3].s64 + 1;
	// 82DE049C: 54C6083C  slwi r6, r6, 1
	ctx.r[6].u32 = ctx.r[6].u32.wrapping_shl(1);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 82DE04A0: 5463083C  slwi r3, r3, 1
	ctx.r[3].u32 = ctx.r[3].u32.wrapping_shl(1);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82DE04A4: A3E70000  lhz r31, 0(r7)
	ctx.r[31].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE04A8: 7FC63A2E  lhzx r30, r6, r7
	ctx.r[30].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[6].u32.wrapping_add(ctx.r[7].u32)) } as u64;
	// 82DE04AC: 7CDF49D6  mullw r6, r31, r9
	ctx.r[6].s32 = ((ctx.r[31].s32 as i64 * ctx.r[9].s32 as i64) as i32);
	ctx.r[6].s64 = ctx.r[6].s32 as i64;
	// 82DE04B0: 7C633A2E  lhzx r3, r3, r7
	ctx.r[3].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[3].u32.wrapping_add(ctx.r[7].u32)) } as u64;
	// 82DE04B4: 7CFE49D6  mullw r7, r30, r9
	ctx.r[7].s32 = ((ctx.r[30].s32 as i64 * ctx.r[9].s32 as i64) as i32);
	ctx.r[7].s64 = ctx.r[7].s32 as i64;
	// 82DE04B8: 7D2349D6  mullw r9, r3, r9
	ctx.r[9].s32 = ((ctx.r[3].s32 as i64 * ctx.r[9].s32 as i64) as i32);
	ctx.r[9].s64 = ctx.r[9].s32 as i64;
	// 82DE04BC: 7FE64214  add r31, r6, r8
	ctx.r[31].u64 = ctx.r[6].u64 + ctx.r[8].u64;
	// 82DE04C0: 7FC74214  add r30, r7, r8
	ctx.r[30].u64 = ctx.r[7].u64 + ctx.r[8].u64;
	// 82DE04C4: 7FA94214  add r29, r9, r8
	ctx.r[29].u64 = ctx.r[9].u64 + ctx.r[8].u64;
	// 82DE04C8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82DE04CC: 409A000C  bne cr6, 0x82de04d8
	if !ctx.cr[6].eq {
	pc = 0x82DE04D8; continue 'dispatch;
	}
	// 82DE04D0: 7CC73378  mr r7, r6
	ctx.r[7].u64 = ctx.r[6].u64;
	// 82DE04D4: 48000018  b 0x82de04ec
	pc = 0x82DE04EC; continue 'dispatch;
            }
            0x82DE04D8 => {
    //   block [0x82DE04D8..0x82DE04EC)
	// 82DE04D8: 816B0034  lwz r11, 0x34(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 82DE04DC: 812A0030  lwz r9, 0x30(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(48 as u32) ) } as u64;
	// 82DE04E0: 7D6B2214  add r11, r11, r4
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 82DE04E4: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DE04E8: 7CEB4A2E  lhzx r7, r11, r9
	ctx.r[7].u64 = unsafe { crate::rt::load_u16(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	pc = 0x82DE04EC; continue 'dispatch;
            }
            0x82DE04EC => {
    //   block [0x82DE04EC..0x82DE0544)
	// 82DE04EC: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 82DE04F0: 419A0054  beq cr6, 0x82de0544
	if ctx.cr[6].eq {
	pc = 0x82DE0544; continue 'dispatch;
	}
	// 82DE04F4: 3D008200  lis r8, -0x7e00
	ctx.r[8].s64 = -2113929216;
	// 82DE04F8: 892A003C  lbz r9, 0x3c(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(60 as u32) ) } as u64;
	// 82DE04FC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82DE0500: C00A0040  lfs f0, 0x40(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(64 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DE0504: 39084CA4  addi r8, r8, 0x4ca4
	ctx.r[8].s64 = ctx.r[8].s64 + 19620;
	// 82DE0508: D0050010  stfs f0, 0x10(r5)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 82DE050C: 3B800005  li r28, 5
	ctx.r[28].s64 = 5;
	// 82DE0510: 90C50008  stw r6, 8(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(8 as u32), ctx.r[6].u32 ) };
	// 82DE0514: 39650050  addi r11, r5, 0x50
	ctx.r[11].s64 = ctx.r[5].s64 + 80;
	// 82DE0518: B0E50014  sth r7, 0x14(r5)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[5].u32.wrapping_add(20 as u32), ctx.r[7].u16 ) };
	// 82DE051C: 99250016  stb r9, 0x16(r5)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[5].u32.wrapping_add(22 as u32), ctx.r[9].u8 ) };
	// 82DE0520: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 82DE0524: B0850006  sth r4, 6(r5)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[5].u32.wrapping_add(6 as u32), ctx.r[4].u16 ) };
	// 82DE0528: 91050000  stw r8, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82DE052C: 9385000C  stw r28, 0xc(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(12 as u32), ctx.r[28].u32 ) };
	pc = 0x82DE0544; continue 'dispatch;
            }
            0x82DE0544 => {
    //   block [0x82DE0544..0x82DE058C)
	// 82DE0544: 7CC33378  mr r3, r6
	ctx.r[3].u64 = ctx.r[6].u64;
	// 82DE0548: 396A0010  addi r11, r10, 0x10
	ctx.r[11].s64 = ctx.r[10].s64 + 16;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE0590(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DE0590 size=144
    let mut pc: u32 = 0x82DE0590;
    'dispatch: loop {
        match pc {
            0x82DE0590 => {
    //   block [0x82DE0590..0x82DE0620)
	// 82DE0590: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DE0594: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DE0598: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DE059C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DE05A0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DE05A4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82DE05A8: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DE05AC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DE05B0: 388B41A4  addi r4, r11, 0x41a4
	ctx.r[4].s64 = ctx.r[11].s64 + 16804;
	// 82DE05B4: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82DE05B8: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE05BC: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82DE05C0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DE05C4: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DE05C8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DE05CC: 4E800421  bctrl
	ctx.lr = 0x82DE05D0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DE05D0: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE05D4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DE05D8: 80DE0010  lwz r6, 0x10(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DE05DC: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82DE05E0: 388B2AB4  addi r4, r11, 0x2ab4
	ctx.r[4].s64 = ctx.r[11].s64 + 10932;
	// 82DE05E4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DE05E8: 816A000C  lwz r11, 0xc(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DE05EC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DE05F0: 4E800421  bctrl
	ctx.lr = 0x82DE05F4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DE05F4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE05F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DE05FC: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DE0600: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DE0604: 4E800421  bctrl
	ctx.lr = 0x82DE0608;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DE0608: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DE060C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DE0610: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DE0614: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DE0618: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DE061C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE0620(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DE0620 size=8
    let mut pc: u32 = 0x82DE0620;
    'dispatch: loop {
        match pc {
            0x82DE0620 => {
    //   block [0x82DE0620..0x82DE0628)
	// 82DE0620: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82DE0624: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE0628(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DE0628 size=72
    let mut pc: u32 = 0x82DE0628;
    'dispatch: loop {
        match pc {
            0x82DE0628 => {
    //   block [0x82DE0628..0x82DE0648)
	// 82DE0628: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82DE062C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82DE0630: 814BFFE8  lwz r10, -0x18(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-24 as u32) ) } as u64;
	// 82DE0634: 554ABA7E  srwi r10, r10, 9
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shr(9);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DE0638: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82DE063C: 4C990020  blelr cr6
	if !ctx.cr[6].gt { return; }
	// 82DE0640: 816BFFE4  lwz r11, -0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-28 as u32) ) } as u64;
	// 82DE0644: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	pc = 0x82DE0648; continue 'dispatch;
            }
            0x82DE0648 => {
    //   block [0x82DE0648..0x82DE0670)
	// 82DE0648: 892BFFFF  lbz r9, -1(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(-1 as u32) ) } as u64;
	// 82DE064C: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82DE0650: 890B0000  lbz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE0654: 396B0200  addi r11, r11, 0x200
	ctx.r[11].s64 = ctx.r[11].s64 + 512;
	// 82DE0658: 5529403E  rotlwi r9, r9, 8
	ctx.r[9].u64 = ((ctx.r[9].u32).rotate_left(8)) as u64;
	// 82DE065C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82DE0660: 7D294378  or r9, r9, r8
	ctx.r[9].u64 = ctx.r[9].u64 | ctx.r[8].u64;
	// 82DE0664: 7C691A14  add r3, r9, r3
	ctx.r[3].u64 = ctx.r[9].u64 + ctx.r[3].u64;
	// 82DE0668: 409AFFE0  bne cr6, 0x82de0648
	if !ctx.cr[6].eq {
	pc = 0x82DE0648; continue 'dispatch;
	}
	// 82DE066C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE0670(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DE0670 size=92
    let mut pc: u32 = 0x82DE0670;
    'dispatch: loop {
        match pc {
            0x82DE0670 => {
    //   block [0x82DE0670..0x82DE069C)
	// 82DE0670: 8143FFE4  lwz r10, -0x1c(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-28 as u32) ) } as u64;
	// 82DE0674: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DE0678: 8123FFE8  lwz r9, -0x18(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-24 as u32) ) } as u64;
	// 82DE067C: 5529BA7E  srwi r9, r9, 9
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shr(9);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82DE0680: 890A0003  lbz r8, 3(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(3 as u32) ) } as u64;
	// 82DE0684: 88EA0004  lbz r7, 4(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DE0688: 5508403E  rotlwi r8, r8, 8
	ctx.r[8].u64 = ((ctx.r[8].u32).rotate_left(8)) as u64;
	// 82DE068C: 7D083B78  or r8, r8, r7
	ctx.r[8].u64 = ctx.r[8].u64 | ctx.r[7].u64;
	// 82DE0690: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82DE0694: 41990030  bgt cr6, 0x82de06c4
	if ctx.cr[6].gt {
	pc = 0x82DE06C4; continue 'dispatch;
	}
	// 82DE0698: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	pc = 0x82DE069C; continue 'dispatch;
            }
            0x82DE069C => {
    //   block [0x82DE069C..0x82DE06C4)
	// 82DE069C: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82DE06A0: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82DE06A4: 394A0200  addi r10, r10, 0x200
	ctx.r[10].s64 = ctx.r[10].s64 + 512;
	// 82DE06A8: 419A0024  beq cr6, 0x82de06cc
	if ctx.cr[6].eq {
		sub_82DE06CC(ctx, base);
		return;
	}
	// 82DE06AC: 890AFFFF  lbz r8, -1(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(-1 as u32) ) } as u64;
	// 82DE06B0: 88EA0000  lbz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE06B4: 5508403E  rotlwi r8, r8, 8
	ctx.r[8].u64 = ((ctx.r[8].u32).rotate_left(8)) as u64;
	// 82DE06B8: 7D083B78  or r8, r8, r7
	ctx.r[8].u64 = ctx.r[8].u64 | ctx.r[7].u64;
	// 82DE06BC: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82DE06C0: 4099FFDC  ble cr6, 0x82de069c
	if !ctx.cr[6].gt {
	pc = 0x82DE069C; continue 'dispatch;
	}
	pc = 0x82DE06C4; continue 'dispatch;
            }
            0x82DE06C4 => {
    //   block [0x82DE06C4..0x82DE06CC)
	// 82DE06C4: 5563402E  slwi r3, r11, 8
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(8);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82DE06C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE06CC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DE06CC size=8
    let mut pc: u32 = 0x82DE06CC;
    'dispatch: loop {
        match pc {
            0x82DE06CC => {
    //   block [0x82DE06CC..0x82DE06D4)
	// 82DE06CC: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82DE06D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE06D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DE06D8 size=116
    let mut pc: u32 = 0x82DE06D8;
    'dispatch: loop {
        match pc {
            0x82DE06D8 => {
    //   block [0x82DE06D8..0x82DE0718)
	// 82DE06D8: 548A063E  clrlwi r10, r4, 0x18
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x000000FFu64;
	// 82DE06DC: 8123FFE4  lwz r9, -0x1c(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-28 as u32) ) } as u64;
	// 82DE06E0: 548BC23E  srwi r11, r4, 8
	ctx.r[11].u32 = ctx.r[4].u32.wrapping_shr(8);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DE06E4: 80E3FFE8  lwz r7, -0x18(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-24 as u32) ) } as u64;
	// 82DE06E8: 390A0001  addi r8, r10, 1
	ctx.r[8].s64 = ctx.r[10].s64 + 1;
	// 82DE06EC: 556A482C  slwi r10, r11, 9
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(9);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DE06F0: 54E7BA7E  srwi r7, r7, 9
	ctx.r[7].u32 = ctx.r[7].u32.wrapping_shr(9);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82DE06F4: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82DE06F8: 892A0003  lbz r9, 3(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(3 as u32) ) } as u64;
	// 82DE06FC: 88CA0004  lbz r6, 4(r10)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DE0700: 5529403E  rotlwi r9, r9, 8
	ctx.r[9].u64 = ((ctx.r[9].u32).rotate_left(8)) as u64;
	// 82DE0704: 7D293378  or r9, r9, r6
	ctx.r[9].u64 = ctx.r[9].u64 | ctx.r[6].u64;
	// 82DE0708: 7F084800  cmpw cr6, r8, r9
	ctx.cr[6].compare_i32(ctx.r[8].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82DE070C: 41980034  blt cr6, 0x82de0740
	if ctx.cr[6].lt {
	pc = 0x82DE0740; continue 'dispatch;
	}
	// 82DE0710: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82DE0714: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	pc = 0x82DE0718; continue 'dispatch;
            }
            0x82DE0718 => {
    //   block [0x82DE0718..0x82DE0740)
	// 82DE0718: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82DE071C: 394A0200  addi r10, r10, 0x200
	ctx.r[10].s64 = ctx.r[10].s64 + 512;
	// 82DE0720: 7F0B3800  cmpw cr6, r11, r7
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[7].s32, &mut ctx.xer);
	// 82DE0724: 419A0028  beq cr6, 0x82de074c
	if ctx.cr[6].eq {
		sub_82DE074C(ctx, base);
		return;
	}
	// 82DE0728: 892AFFFF  lbz r9, -1(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(-1 as u32) ) } as u64;
	// 82DE072C: 88CA0000  lbz r6, 0(r10)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE0730: 5529403E  rotlwi r9, r9, 8
	ctx.r[9].u64 = ((ctx.r[9].u32).rotate_left(8)) as u64;
	// 82DE0734: 7D293378  or r9, r9, r6
	ctx.r[9].u64 = ctx.r[9].u64 | ctx.r[6].u64;
	// 82DE0738: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82DE073C: 4099FFDC  ble cr6, 0x82de0718
	if !ctx.cr[6].gt {
	pc = 0x82DE0718; continue 'dispatch;
	}
	pc = 0x82DE0740; continue 'dispatch;
            }
            0x82DE0740 => {
    //   block [0x82DE0740..0x82DE074C)
	// 82DE0740: 556B402E  slwi r11, r11, 8
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(8);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DE0744: 7D634378  or r3, r11, r8
	ctx.r[3].u64 = ctx.r[11].u64 | ctx.r[8].u64;
	// 82DE0748: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE074C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DE074C size=8
    let mut pc: u32 = 0x82DE074C;
    'dispatch: loop {
        match pc {
            0x82DE074C => {
    //   block [0x82DE074C..0x82DE0754)
	// 82DE074C: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82DE0750: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE0758(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DE0758 size=228
    let mut pc: u32 = 0x82DE0758;
    'dispatch: loop {
        match pc {
            0x82DE0758 => {
    //   block [0x82DE0758..0x82DE07B0)
	// 82DE0758: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DE075C: 4BEC8CAD  bl 0x82ca9408
	ctx.lr = 0x82DE0760;
	sub_82CA93D0(ctx, base);
	// 82DE0760: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DE0764: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82DE0768: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DE076C: 578B063E  clrlwi r11, r28, 0x18
	ctx.r[11].u64 = ctx.r[28].u32 as u64 & 0x000000FFu64;
	// 82DE0770: 2B0B0006  cmplwi cr6, r11, 6
	ctx.cr[6].compare_u32(ctx.r[11].u32, 6 as u32, &mut ctx.xer);
	// 82DE0774: 997E0074  stb r11, 0x74(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(116 as u32), ctx.r[11].u8 ) };
	// 82DE0778: 419A00BC  beq cr6, 0x82de0834
	if ctx.cr[6].eq {
	pc = 0x82DE0834; continue 'dispatch;
	}
	// 82DE077C: 817E0030  lwz r11, 0x30(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 82DE0780: 3BBE0030  addi r29, r30, 0x30
	ctx.r[29].s64 = ctx.r[30].s64 + 48;
	// 82DE0784: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82DE0788: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DE078C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DE0790: 4E800421  bctrl
	ctx.lr = 0x82DE0794;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DE0794: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DE0798: 2F1FFFFF  cmpwi cr6, r31, -1
	ctx.cr[6].compare_i32(ctx.r[31].s32, -1, &mut ctx.xer);
	// 82DE079C: 419A0098  beq cr6, 0x82de0834
	if ctx.cr[6].eq {
	pc = 0x82DE0834; continue 'dispatch;
	}
	// 82DE07A0: 397CFFFB  addi r11, r28, -5
	ctx.r[11].s64 = ctx.r[28].s64 + -5;
	// 82DE07A4: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82DE07A8: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82DE07AC: 697C0001  xori r28, r11, 1
	ctx.r[28].u64 = ctx.r[11].u64 ^ 1;
            }
            0x82DE07B0 => {
    //   block [0x82DE07B0..0x82DE0834)
	// 82DE07B0: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82DE07B4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82DE07B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DE07BC: 480022B5  bl 0x82de2a70
	ctx.lr = 0x82DE07C0;
	sub_82DE2A70(ctx, base);
	// 82DE07C0: 815E0014  lwz r10, 0x14(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DE07C4: 57EB082C  rlwinm r11, r31, 1, 0, 0x16
	ctx.r[11].u64 = ctx.r[31].u32 as u64 & 0x7FFFFFFFu64;
	// 82DE07C8: 7C691B78  mr r9, r3
	ctx.r[9].u64 = ctx.r[3].u64;
	// 82DE07CC: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82DE07D0: 57EA063E  clrlwi r10, r31, 0x18
	ctx.r[10].u64 = ctx.r[31].u32 as u64 & 0x000000FFu64;
	// 82DE07D4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DE07D8: 394A0010  addi r10, r10, 0x10
	ctx.r[10].s64 = ctx.r[10].s64 + 16;
	// 82DE07DC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82DE07E0: 5548083C  slwi r8, r10, 1
	ctx.r[8].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82DE07E4: 894B0001  lbz r10, 1(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(1 as u32) ) } as u64;
	// 82DE07E8: 88EB0002  lbz r7, 2(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(2 as u32) ) } as u64;
	// 82DE07EC: 554A403E  rotlwi r10, r10, 8
	ctx.r[10].u64 = ((ctx.r[10].u32).rotate_left(8)) as u64;
	// 82DE07F0: 7D4A3B78  or r10, r10, r7
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[7].u64;
	// 82DE07F4: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82DE07F8: 894B000C  lbz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DE07FC: 88EB000D  lbz r7, 0xd(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(13 as u32) ) } as u64;
	// 82DE0800: 554A083E  rotlwi r10, r10, 1
	ctx.r[10].u64 = ((ctx.r[10].u32).rotate_left(1)) as u64;
	// 82DE0804: 7D4A3A14  add r10, r10, r7
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[7].u64;
	// 82DE0808: 5547083C  slwi r7, r10, 1
	ctx.r[7].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82DE080C: 7D4A3A14  add r10, r10, r7
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[7].u64;
	// 82DE0810: 7D4A4214  add r10, r10, r8
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[8].u64;
	// 82DE0814: 7D2A5B2E  sthx r9, r10, r11
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32), ctx.r[9].u16) };
	// 82DE0818: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE081C: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DE0820: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DE0824: 4E800421  bctrl
	ctx.lr = 0x82DE0828;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DE0828: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DE082C: 2F1FFFFF  cmpwi cr6, r31, -1
	ctx.cr[6].compare_i32(ctx.r[31].s32, -1, &mut ctx.xer);
	// 82DE0830: 409AFF80  bne cr6, 0x82de07b0
	if !ctx.cr[6].eq {
	pc = 0x82DE07B0; continue 'dispatch;
	}
            }
            0x82DE0834 => {
    //   block [0x82DE0834..0x82DE083C)
	// 82DE0834: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DE0838: 4BEC8C20  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE0840(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DE0840 size=64
    let mut pc: u32 = 0x82DE0840;
    'dispatch: loop {
        match pc {
            0x82DE0840 => {
    //   block [0x82DE0840..0x82DE085C)
	// 82DE0840: 81230088  lwz r9, 0x88(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(136 as u32) ) } as u64;
	// 82DE0844: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82DE0848: 419A0030  beq cr6, 0x82de0878
	if ctx.cr[6].eq {
	pc = 0x82DE0878; continue 'dispatch;
	}
	// 82DE084C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82DE0850: 40990028  ble cr6, 0x82de0878
	if !ctx.cr[6].gt {
	pc = 0x82DE0878; continue 'dispatch;
	}
	// 82DE0854: 81030084  lwz r8, 0x84(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(132 as u32) ) } as u64;
	// 82DE0858: 39680004  addi r11, r8, 4
	ctx.r[11].s64 = ctx.r[8].s64 + 4;
	pc = 0x82DE085C; continue 'dispatch;
            }
            0x82DE085C => {
    //   block [0x82DE085C..0x82DE0878)
	// 82DE085C: 80EB0000  lwz r7, 0(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE0860: 7F072040  cmplw cr6, r7, r4
	ctx.cr[6].compare_u32(ctx.r[7].u32, ctx.r[4].u32, &mut ctx.xer);
	// 82DE0864: 419A001C  beq cr6, 0x82de0880
	if ctx.cr[6].eq {
		sub_82DE0880(ctx, base);
		return;
	}
	// 82DE0868: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82DE086C: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82DE0870: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82DE0874: 4198FFE8  blt cr6, 0x82de085c
	if ctx.cr[6].lt {
	pc = 0x82DE085C; continue 'dispatch;
	}
	pc = 0x82DE0878; continue 'dispatch;
            }
            0x82DE0878 => {
    //   block [0x82DE0878..0x82DE0880)
	// 82DE0878: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82DE087C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE0880(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DE0880 size=12
    let mut pc: u32 = 0x82DE0880;
    'dispatch: loop {
        match pc {
            0x82DE0880 => {
    //   block [0x82DE0880..0x82DE088C)
	// 82DE0880: 554B1838  slwi r11, r10, 3
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DE0884: 7C6B402E  lwzx r3, r11, r8
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 82DE0888: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE0890(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DE0890 size=44
    let mut pc: u32 = 0x82DE0890;
    'dispatch: loop {
        match pc {
            0x82DE0890 => {
    //   block [0x82DE0890..0x82DE08BC)
	// 82DE0890: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DE0894: 39400020  li r10, 0x20
	ctx.r[10].s64 = 32;
	// 82DE0898: 3920000C  li r9, 0xc
	ctx.r[9].s64 = 12;
	// 82DE089C: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE08C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DE08C0 size=408
    let mut pc: u32 = 0x82DE08C0;
    'dispatch: loop {
        match pc {
            0x82DE08C0 => {
    //   block [0x82DE08C0..0x82DE0A58)
	// 82DE08C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DE08C4: 4BEC8B45  bl 0x82ca9408
	ctx.lr = 0x82DE08C8;
	sub_82CA93D0(ctx, base);
	// 82DE08C8: 9421FD30  stwu r1, -0x2d0(r1)
	ea = ctx.r[1].u32.wrapping_add(-720 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DE08CC: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 82DE08D0: 39410070  addi r10, r1, 0x70
	ctx.r[10].s64 = ctx.r[1].s64 + 112;
	// 82DE08D4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82DE08D8: 3BFD0030  addi r31, r29, 0x30
	ctx.r[31].s64 = ctx.r[29].s64 + 48;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE0A58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DE0A58 size=20
    let mut pc: u32 = 0x82DE0A58;
    'dispatch: loop {
        match pc {
            0x82DE0A58 => {
    //   block [0x82DE0A58..0x82DE0A6C)
	// 82DE0A58: 81630048  lwz r11, 0x48(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(72 as u32) ) } as u64;
	// 82DE0A5C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DE0A60: 409A000C  bne cr6, 0x82de0a6c
	if !ctx.cr[6].eq {
		sub_82DE0A6C(ctx, base);
		return;
	}
	// 82DE0A64: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82DE0A68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE0A6C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DE0A6C size=100
    let mut pc: u32 = 0x82DE0A6C;
    'dispatch: loop {
        match pc {
            0x82DE0A6C => {
    //   block [0x82DE0A6C..0x82DE0AD0)
	// 82DE0A6C: 8143FFE4  lwz r10, -0x1c(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-28 as u32) ) } as u64;
	// 82DE0A70: 548B082C  rlwinm r11, r4, 1, 0, 0x16
	ctx.r[11].u64 = ctx.r[4].u32 as u64 & 0x7FFFFFFFu64;
	// 82DE0A74: 5488063E  clrlwi r8, r4, 0x18
	ctx.r[8].u64 = ctx.r[4].u32 as u64 & 0x000000FFu64;
	// 82DE0A78: 80C3004C  lwz r6, 0x4c(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(76 as u32) ) } as u64;
	// 82DE0A7C: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82DE0A80: 80A30048  lwz r5, 0x48(r3)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(72 as u32) ) } as u64;
	// 82DE0A84: 894B0001  lbz r10, 1(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(1 as u32) ) } as u64;
	// 82DE0A88: 892B0002  lbz r9, 2(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(2 as u32) ) } as u64;
	// 82DE0A8C: 554A403E  rotlwi r10, r10, 8
	ctx.r[10].u64 = ((ctx.r[10].u32).rotate_left(8)) as u64;
	// 82DE0A90: 7D4A4B78  or r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[9].u64;
	// 82DE0A94: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82DE0A98: 894B000C  lbz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DE0A9C: 892B000D  lbz r9, 0xd(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(13 as u32) ) } as u64;
	// 82DE0AA0: 5547083E  rotlwi r7, r10, 1
	ctx.r[7].u64 = ((ctx.r[10].u32).rotate_left(1)) as u64;
	// 82DE0AA4: 7D4A3A14  add r10, r10, r7
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[7].u64;
	// 82DE0AA8: 5527103E  rotlwi r7, r9, 2
	ctx.r[7].u64 = ((ctx.r[9].u32).rotate_left(2)) as u64;
	// 82DE0AAC: 554A083C  slwi r10, r10, 1
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DE0AB0: 7D293A14  add r9, r9, r7
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[7].u64;
	// 82DE0AB4: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82DE0AB8: 7D4A4214  add r10, r10, r8
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[8].u64;
	// 82DE0ABC: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82DE0AC0: 896B0020  lbz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82DE0AC4: 7D6B31D6  mullw r11, r11, r6
	ctx.r[11].s32 = ((ctx.r[11].s32 as i64 * ctx.r[6].s32 as i64) as i32);
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 82DE0AC8: 7C6B282E  lwzx r3, r11, r5
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[5].u32)) } as u64;
	// 82DE0ACC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE0AD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DE0AD0 size=120
    let mut pc: u32 = 0x82DE0AD0;
    'dispatch: loop {
        match pc {
            0x82DE0AD0 => {
    //   block [0x82DE0AD0..0x82DE0B48)
	// 82DE0AD0: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DE0AD4: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82DE0AD8: 396B00C4  addi r11, r11, 0xc4
	ctx.r[11].s64 = ctx.r[11].s64 + 196;
	// 82DE0ADC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82DE0AE0: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DE0AE4: 394A41E0  addi r10, r10, 0x41e0
	ctx.r[10].s64 = ctx.r[10].s64 + 16864;
	// 82DE0AE8: B0E30006  sth r7, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[7].u16 ) };
	// 82DE0AEC: 392941BC  addi r9, r9, 0x41bc
	ctx.r[9].s64 = ctx.r[9].s64 + 16828;
	// 82DE0AF0: 91630030  stw r11, 0x30(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 82DE0AF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82DE0AF8: 3CE08000  lis r7, -0x8000
	ctx.r[7].s64 = -2147483648;
	// 82DE0AFC: 3960000C  li r11, 0xc
	ctx.r[11].s64 = 12;
	// 82DE0B00: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82DE0B04: 2F040001  cmpwi cr6, r4, 1
	ctx.cr[6].compare_i32(ctx.r[4].s32, 1, &mut ctx.xer);
	// 82DE0B08: 91230030  stw r9, 0x30(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(48 as u32), ctx.r[9].u32 ) };
	// 82DE0B0C: 91030084  stw r8, 0x84(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(132 as u32), ctx.r[8].u32 ) };
	// 82DE0B10: 91030088  stw r8, 0x88(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(136 as u32), ctx.r[8].u32 ) };
	// 82DE0B14: 90E3008C  stw r7, 0x8c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(140 as u32), ctx.r[7].u32 ) };
	// 82DE0B18: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82DE0B1C: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
	// 82DE0B20: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DE0B24: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 82DE0B28: 39200020  li r9, 0x20
	ctx.r[9].s64 = 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE0B48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DE0B48 size=128
    let mut pc: u32 = 0x82DE0B48;
    'dispatch: loop {
        match pc {
            0x82DE0B48 => {
    //   block [0x82DE0B48..0x82DE0BA0)
	// 82DE0B48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DE0B4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DE0B50: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DE0B54: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DE0B58: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DE0B5C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82DE0B60: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DE0B64: 396B41E0  addi r11, r11, 0x41e0
	ctx.r[11].s64 = ctx.r[11].s64 + 16864;
	// 82DE0B68: 394A41BC  addi r10, r10, 0x41bc
	ctx.r[10].s64 = ctx.r[10].s64 + 16828;
	// 82DE0B6C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DE0B70: 915F0030  stw r10, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[10].u32 ) };
	// 82DE0B74: 817F008C  lwz r11, 0x8c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(140 as u32) ) } as u64;
	// 82DE0B78: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82DE0B7C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82DE0B80: 409A0020  bne cr6, 0x82de0ba0
	if !ctx.cr[6].eq {
	pc = 0x82DE0BA0; continue 'dispatch;
	}
	// 82DE0B84: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE0B88: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82DE0B8C: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82DE0B90: 809F0084  lwz r4, 0x84(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 82DE0B94: 55651838  slwi r5, r11, 3
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82DE0B98: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82DE0B9C: 4BF7472D  bl 0x82d552c8
	ctx.lr = 0x82DE0BA0;
	sub_82D552C8(ctx, base);
	pc = 0x82DE0BA0; continue 'dispatch;
            }
            0x82DE0BA0 => {
    //   block [0x82DE0BA0..0x82DE0BC8)
	// 82DE0BA0: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DE0BA4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DE0BA8: 396B00C4  addi r11, r11, 0xc4
	ctx.r[11].s64 = ctx.r[11].s64 + 196;
	// 82DE0BAC: 917F0030  stw r11, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 82DE0BB0: 4BFD2CD1  bl 0x82db3880
	ctx.lr = 0x82DE0BB4;
	sub_82DB3880(ctx, base);
	// 82DE0BB4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DE0BB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DE0BBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DE0BC0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DE0BC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE0BC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DE0BC8 size=24
    let mut pc: u32 = 0x82DE0BC8;
    'dispatch: loop {
        match pc {
            0x82DE0BC8 => {
    //   block [0x82DE0BC8..0x82DE0BE0)
	// 82DE0BC8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82DE0BCC: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82DE0BD0: 7CC73378  mr r7, r6
	ctx.r[7].u64 = ctx.r[6].u64;
	// 82DE0BD4: 38AB0050  addi r5, r11, 0x50
	ctx.r[5].s64 = ctx.r[11].s64 + 80;
	// 82DE0BD8: 388B0040  addi r4, r11, 0x40
	ctx.r[4].s64 = ctx.r[11].s64 + 64;
	// 82DE0BDC: 4B4B9214  b 0x82299df0
	sub_82299DF0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE0BE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DE0BE0 size=268
    let mut pc: u32 = 0x82DE0BE0;
    'dispatch: loop {
        match pc {
            0x82DE0BE0 => {
    //   block [0x82DE0BE0..0x82DE0C34)
	// 82DE0BE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DE0BE4: 4BEC8821  bl 0x82ca9404
	ctx.lr = 0x82DE0BE8;
	sub_82CA93D0(ctx, base);
	// 82DE0BE8: 9421FED0  stwu r1, -0x130(r1)
	ea = ctx.r[1].u32.wrapping_add(-304 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DE0BEC: 836D0000  lwz r27, 0(r13)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE0BF0: 3B800008  li r28, 8
	ctx.r[28].s64 = 8;
	// 82DE0BF4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DE0BF8: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82DE0BFC: 7CA72B78  mr r7, r5
	ctx.r[7].u64 = ctx.r[5].u64;
	// 82DE0C00: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 82DE0C04: 7D5CD82E  lwzx r10, r28, r27
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[28].u32.wrapping_add(ctx.r[27].u32)) } as u64;
	// 82DE0C08: 812A0004  lwz r9, 4(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DE0C0C: 810A000C  lwz r8, 0xc(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DE0C10: 7F094040  cmplw cr6, r9, r8
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82DE0C14: 40980020  bge cr6, 0x82de0c34
	if !ctx.cr[6].lt {
	pc = 0x82DE0C34; continue 'dispatch;
	}
	// 82DE0C18: 3D008203  lis r8, -0x7dfd
	ctx.r[8].s64 = -2113732608;
	// 82DE0C1C: 39082AC8  addi r8, r8, 0x2ac8
	ctx.r[8].s64 = ctx.r[8].s64 + 10952;
	// 82DE0C20: 91090000  stw r8, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82DE0C24: 7D0C42E6  mftb r8, 0x10c
	ctx.r[8].u64 = crate::rt::rdtsc_u64();
	// 82DE0C28: 38C9000C  addi r6, r9, 0xc
	ctx.r[6].s64 = ctx.r[9].s64 + 12;
	// 82DE0C2C: 91090004  stw r8, 4(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82DE0C30: 90CA0004  stw r6, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[6].u32 ) };
	pc = 0x82DE0C34; continue 'dispatch;
            }
            0x82DE0C34 => {
    //   block [0x82DE0C34..0x82DE0CEC)
	// 82DE0C34: 815F0040  lwz r10, 0x40(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as u64;
	// 82DE0C38: 38800020  li r4, 0x20
	ctx.r[4].s64 = 32;
	// 82DE0C3C: 392100CF  addi r9, r1, 0xcf
	ctx.r[9].s64 = ctx.r[1].s64 + 207;
	// 82DE0C40: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82DE0C44: 55260036  rlwinm r6, r9, 0, 0, 0x1b
	ctx.r[6].u64 = ctx.r[9].u32 as u64 & 0xFFFFFFFFu64;
	// 82DE0C48: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 82DE0C4C: 38AB0030  addi r5, r11, 0x30
	ctx.r[5].s64 = ctx.r[11].s64 + 48;
	// 82DE0C50: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82DE0C54: 915F0040  stw r10, 0x40(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), ctx.r[10].u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE0CF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DE0CF0 size=232
    let mut pc: u32 = 0x82DE0CF0;
    'dispatch: loop {
        match pc {
            0x82DE0CF0 => {
    //   block [0x82DE0CF0..0x82DE0D3C)
	// 82DE0CF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DE0CF4: 4BEC8711  bl 0x82ca9404
	ctx.lr = 0x82DE0CF8;
	sub_82CA93D0(ctx, base);
	// 82DE0CF8: 9421FEC0  stwu r1, -0x140(r1)
	ea = ctx.r[1].u32.wrapping_add(-320 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DE0CFC: 83AD0000  lwz r29, 0(r13)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE0D00: 3BC00008  li r30, 8
	ctx.r[30].s64 = 8;
	// 82DE0D04: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 82DE0D08: 7CC83378  mr r8, r6
	ctx.r[8].u64 = ctx.r[6].u64;
	// 82DE0D0C: 7D7EE82E  lwzx r11, r30, r29
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 82DE0D10: 812B0004  lwz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DE0D14: 80EB000C  lwz r7, 0xc(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DE0D18: 7F093840  cmplw cr6, r9, r7
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[7].u32, &mut ctx.xer);
	// 82DE0D1C: 40980020  bge cr6, 0x82de0d3c
	if !ctx.cr[6].lt {
	pc = 0x82DE0D3C; continue 'dispatch;
	}
	// 82DE0D20: 3CE08203  lis r7, -0x7dfd
	ctx.r[7].s64 = -2113732608;
	// 82DE0D24: 38E72AC8  addi r7, r7, 0x2ac8
	ctx.r[7].s64 = ctx.r[7].s64 + 10952;
	// 82DE0D28: 90E90000  stw r7, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82DE0D2C: 7CEC42E6  mftb r7, 0x10c
	ctx.r[7].u64 = crate::rt::rdtsc_u64();
	// 82DE0D30: 38C9000C  addi r6, r9, 0xc
	ctx.r[6].s64 = ctx.r[9].s64 + 12;
	// 82DE0D34: 90E90004  stw r7, 4(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82DE0D38: 90CB0004  stw r6, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[6].u32 ) };
	pc = 0x82DE0D3C; continue 'dispatch;
            }
            0x82DE0D3C => {
    //   block [0x82DE0D3C..0x82DE0DD8)
	// 82DE0D3C: 38C00020  li r6, 0x20
	ctx.r[6].s64 = 32;
	// 82DE0D40: 80E50008  lwz r7, 8(r5)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(8 as u32) ) } as u64;
	// 82DE0D44: 396100DF  addi r11, r1, 0xdf
	ctx.r[11].s64 = ctx.r[1].s64 + 223;
	// 82DE0D48: 812A0018  lwz r9, 0x18(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DE0D4C: 3B600010  li r27, 0x10
	ctx.r[27].s64 = 16;
	// 82DE0D50: 838A0014  lwz r28, 0x14(r10)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DE0D54: 556B0036  rlwinm r11, r11, 0, 0, 0x1b
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82DE0D58: 90A1005C  stw r5, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[5].u32 ) };
	// 82DE0D5C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82DE0D60: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE0DD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82DE0DD8 size=752
    let mut pc: u32 = 0x82DE0DD8;
    'dispatch: loop {
        match pc {
            0x82DE0DD8 => {
    //   block [0x82DE0DD8..0x82DE0E68)
	// 82DE0DD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DE0DDC: 4BEC862D  bl 0x82ca9408
	ctx.lr = 0x82DE0DE0;
	sub_82CA93D0(ctx, base);
	// 82DE0DE0: 38E3FFD0  addi r7, r3, -0x30
	ctx.r[7].s64 = ctx.r[3].s64 + -48;
	// 82DE0DE4: 548B082C  rlwinm r11, r4, 1, 0, 0x16
	ctx.r[11].u64 = ctx.r[4].u32 as u64 & 0x7FFFFFFFu64;
	// 82DE0DE8: 5486063E  clrlwi r6, r4, 0x18
	ctx.r[6].u64 = ctx.r[4].u32 as u64 & 0x000000FFu64;
	// 82DE0DEC: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 82DE0DF0: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82DE0DF4: 81470014  lwz r10, 0x14(r7)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(20 as u32) ) } as u64;
	// 82DE0DF8: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82DE0DFC: 894B0001  lbz r10, 1(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(1 as u32) ) } as u64;
	// 82DE0E00: 892B0002  lbz r9, 2(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(2 as u32) ) } as u64;
	// 82DE0E04: 554A403E  rotlwi r10, r10, 8
	ctx.r[10].u64 = ((ctx.r[10].u32).rotate_left(8)) as u64;
	// 82DE0E08: 7D4A4B78  or r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[9].u64;
	// 82DE0E0C: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82DE0E10: 419A0058  beq cr6, 0x82de0e68
	if ctx.cr[6].eq {
	pc = 0x82DE0E68; continue 'dispatch;
	}
	// 82DE0E14: 3D408330  lis r10, -0x7cd0
	ctx.r[10].s64 = -2094006272;
	// 82DE0E18: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 82DE0E1C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82DE0E20: 39294CA4  addi r9, r9, 0x4ca4
	ctx.r[9].s64 = ctx.r[9].s64 + 19620;
	// 82DE0E24: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 82DE0E28: C00AB264  lfs f0, -0x4d9c(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-19868 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DE0E2C: 38800006  li r4, 6
	ctx.r[4].s64 = 6;
	// 82DE0E30: 39450050  addi r10, r5, 0x50
	ctx.r[10].s64 = ctx.r[5].s64 + 80;
	// 82DE0E34: D0050010  stfs f0, 0x10(r5)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 82DE0E38: B0650006  sth r3, 6(r5)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[5].u32.wrapping_add(6 as u32), ctx.r[3].u16 ) };
	// 82DE0E3C: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 82DE0E40: 93E50008  stw r31, 8(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(8 as u32), ctx.r[31].u32 ) };
	// 82DE0E44: 9105000C  stw r8, 0xc(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 82DE0E48: 91250000  stw r9, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DE0E4C: B3E50014  sth r31, 0x14(r5)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[5].u32.wrapping_add(20 as u32), ctx.r[31].u16 ) };
	// 82DE0E50: 98850016  stb r4, 0x16(r5)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[5].u32.wrapping_add(22 as u32), ctx.r[4].u8 ) };
	pc = 0x82DE0E68; continue 'dispatch;
            }
            0x82DE0E68 => {
    //   block [0x82DE0E68..0x82DE10C8)
	// 82DE0E68: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DE0E6C: 3D008200  lis r8, -0x7e00
	ctx.r[8].s64 = -2113929216;
	// 82DE0E70: C00B0000  lfs f0, 0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DE0E74: D001FFB0  stfs f0, -0x50(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-80 as u32), tmp.u32 ) };
	// 82DE0E78: 892B000C  lbz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DE0E7C: C00B0004  lfs f0, 4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DE0E80: 394B0020  addi r10, r11, 0x20
	ctx.r[10].s64 = ctx.r[11].s64 + 32;
	// 82DE0E84: D001FFB4  stfs f0, -0x4c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-76 as u32), tmp.u32 ) };
	// 82DE0E88: 3BC00020  li r30, 0x20
	ctx.r[30].s64 = 32;
	// 82DE0E8C: C1AB0008  lfs f13, 8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DE0E90: 3BA00030  li r29, 0x30
	ctx.r[29].s64 = 48;
	// 82DE0E94: C0080C18  lfs f0, 0xc18(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(3096 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DE0E98: 5528083E  rotlwi r8, r9, 1
	ctx.r[8].u64 = ((ctx.r[9].u32).rotate_left(1)) as u64;
	// 82DE0E9C: D1A1FFB8  stfs f13, -0x48(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-72 as u32), tmp.u32 ) };
	// 82DE0EA0: C1AB0010  lfs f13, 0x10(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DE0EA4: 7D094214  add r8, r9, r8
	ctx.r[8].u64 = ctx.r[9].u64 + ctx.r[8].u64;
	// 82DE0EA8: D1A1FFA0  stfs f13, -0x60(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-96 as u32), tmp.u32 ) };
	// 82DE0EAC: C1AB0014  lfs f13, 0x14(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DE0EB0: 5508083C  slwi r8, r8, 1
	ctx.r[8].u32 = ctx.r[8].u32.wrapping_shl(1);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82DE0EB4: D001FFBC  stfs f0, -0x44(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-68 as u32), tmp.u32 ) };
	// 82DE0EB8: 3921FFB0  addi r9, r1, -0x50
	ctx.r[9].s64 = ctx.r[1].s64 + -80;
	// 82DE0EBC: D1A1FFA4  stfs f13, -0x5c(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-92 as u32), tmp.u32 ) };
	// 82DE0EC0: C1AB0018  lfs f13, 0x18(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DE0EC4: D1A1FFA8  stfs f13, -0x58(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-88 as u32), tmp.u32 ) };
	// 82DE0EC8: D001FFAC  stfs f0, -0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-84 as u32), tmp.u32 ) };
	// 82DE0ECC: 38A1FFA0  addi r5, r1, -0x60
	ctx.r[5].s64 = ctx.r[1].s64 + -96;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE10C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DE10C8 size=220
    let mut pc: u32 = 0x82DE10C8;
    'dispatch: loop {
        match pc {
            0x82DE10C8 => {
    //   block [0x82DE10C8..0x82DE11A4)
	// 82DE10C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DE10CC: 4BEC833D  bl 0x82ca9408
	ctx.lr = 0x82DE10D0;
	sub_82CA93D0(ctx, base);
	// 82DE10D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DE10D4: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82DE10D8: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82DE10DC: 3880000C  li r4, 0xc
	ctx.r[4].s64 = 12;
	// 82DE10E0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DE10E4: 4BFD5795  bl 0x82db6878
	ctx.lr = 0x82DE10E8;
	sub_82DB6878(ctx, base);
	// 82DE10E8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82DE10EC: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DE10F0: 394A00C4  addi r10, r10, 0xc4
	ctx.r[10].s64 = ctx.r[10].s64 + 196;
	// 82DE10F4: 392941E0  addi r9, r9, 0x41e0
	ctx.r[9].s64 = ctx.r[9].s64 + 16864;
	// 82DE10F8: 3D008203  lis r8, -0x7dfd
	ctx.r[8].s64 = -2113732608;
	// 82DE10FC: 3BDF0084  addi r30, r31, 0x84
	ctx.r[30].s64 = ctx.r[31].s64 + 132;
	// 82DE1100: 390841BC  addi r8, r8, 0x41bc
	ctx.r[8].s64 = ctx.r[8].s64 + 16828;
	// 82DE1104: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82DE1108: 915F0030  stw r10, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[10].u32 ) };
	// 82DE110C: 3CE08000  lis r7, -0x8000
	ctx.r[7].s64 = -2147483648;
	// 82DE1110: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DE1114: 3D408330  lis r10, -0x7cd0
	ctx.r[10].s64 = -2094006272;
	// 82DE1118: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 82DE111C: 397F0060  addi r11, r31, 0x60
	ctx.r[11].s64 = ctx.r[31].s64 + 96;
	// 82DE1120: 911F0030  stw r8, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[8].u32 ) };
	// 82DE1124: 93BE0000  stw r29, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82DE1128: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DE112C: 93BE0004  stw r29, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82DE1130: 90FE0008  stw r7, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[7].u32 ) };
	// 82DE1134: 93BF0078  stw r29, 0x78(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(120 as u32), ctx.r[29].u32 ) };
	// 82DE1138: C00AB264  lfs f0, -0x4d9c(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-19868 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DE113C: D01F0070  stfs f0, 0x70(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(112 as u32), tmp.u32 ) };
	// 82DE1140: 993F0074  stb r9, 0x74(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(116 as u32), ctx.r[9].u8 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE11A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DE11A8 size=256
    let mut pc: u32 = 0x82DE11A8;
    'dispatch: loop {
        match pc {
            0x82DE11A8 => {
    //   block [0x82DE11A8..0x82DE11C0)
	// 82DE11A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DE11AC: 4BEC825D  bl 0x82ca9408
	ctx.lr = 0x82DE11B0;
	sub_82CA93D0(ctx, base);
	// 82DE11B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DE11B4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DE11B8: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82DE11BC: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	pc = 0x82DE11C0; continue 'dispatch;
            }
            0x82DE11C0 => {
    //   block [0x82DE11C0..0x82DE11E0)
	// 82DE11C0: 7D64EA14  add r11, r4, r29
	ctx.r[11].u64 = ctx.r[4].u64 + ctx.r[29].u64;
	// 82DE11C4: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82DE11C8: 7D6B0E70  srawi r11, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 82DE11CC: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82DE11D0: 556B1838  slwi r11, r11, 3
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DE11D4: 7D6BF02A  ldx r11, r11, r30
	ctx.r[11].u64 = unsafe { crate::rt::load_u64(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) };
	// 82DE11D8: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 82DE11DC: 81010054  lwz r8, 0x54(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	pc = 0x82DE11E0; continue 'dispatch;
            }
            0x82DE11E0 => {
    //   block [0x82DE11E0..0x82DE11F8)
	// 82DE11E0: 57EB1838  slwi r11, r31, 3
	ctx.r[11].u32 = ctx.r[31].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DE11E4: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82DE11E8: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82DE11EC: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE11F0: 7F0A4040  cmplw cr6, r10, r8
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82DE11F4: 40980018  bge cr6, 0x82de120c
	if !ctx.cr[6].lt {
	pc = 0x82DE120C; continue 'dispatch;
	}
	pc = 0x82DE11F8; continue 'dispatch;
            }
            0x82DE11F8 => {
    //   block [0x82DE11F8..0x82DE120C)
	// 82DE11F8: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82DE11FC: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82DE1200: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE1204: 7F0A4040  cmplw cr6, r10, r8
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82DE1208: 4198FFF0  blt cr6, 0x82de11f8
	if ctx.cr[6].lt {
	pc = 0x82DE11F8; continue 'dispatch;
	}
	pc = 0x82DE120C; continue 'dispatch;
            }
            0x82DE120C => {
    //   block [0x82DE120C..0x82DE1224)
	// 82DE120C: 54AB1838  slwi r11, r5, 3
	ctx.r[11].u32 = ctx.r[5].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DE1210: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82DE1214: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82DE1218: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE121C: 7F085040  cmplw cr6, r8, r10
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82DE1220: 40980018  bge cr6, 0x82de1238
	if !ctx.cr[6].lt {
	pc = 0x82DE1238; continue 'dispatch;
	}
	pc = 0x82DE1224; continue 'dispatch;
            }
            0x82DE1224 => {
    //   block [0x82DE1224..0x82DE1238)
	// 82DE1224: 396BFFF8  addi r11, r11, -8
	ctx.r[11].s64 = ctx.r[11].s64 + -8;
	// 82DE1228: 38A5FFFF  addi r5, r5, -1
	ctx.r[5].s64 = ctx.r[5].s64 + -1;
	// 82DE122C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE1230: 7F085040  cmplw cr6, r8, r10
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82DE1234: 4198FFF0  blt cr6, 0x82de1224
	if ctx.cr[6].lt {
	pc = 0x82DE1224; continue 'dispatch;
	}
	pc = 0x82DE1238; continue 'dispatch;
            }
            0x82DE1238 => {
    //   block [0x82DE1238..0x82DE126C)
	// 82DE1238: 7F05F800  cmpw cr6, r5, r31
	ctx.cr[6].compare_i32(ctx.r[5].s32, ctx.r[31].s32, &mut ctx.xer);
	// 82DE123C: 41980040  blt cr6, 0x82de127c
	if ctx.cr[6].lt {
	pc = 0x82DE127C; continue 'dispatch;
	}
	// 82DE1240: 419A002C  beq cr6, 0x82de126c
	if ctx.cr[6].eq {
	pc = 0x82DE126C; continue 'dispatch;
	}
	// 82DE1244: 57EB1838  slwi r11, r31, 3
	ctx.r[11].u32 = ctx.r[31].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DE1248: 54AA1838  slwi r10, r5, 3
	ctx.r[10].u32 = ctx.r[5].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DE124C: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82DE1250: 7D4AF214  add r10, r10, r30
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[30].u64;
	// 82DE1254: 80EB0000  lwz r7, 0(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE1258: E92A0000  ld r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	// 82DE125C: 90EA0000  stw r7, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82DE1260: 80EB0004  lwz r7, 4(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DE1264: 90EA0004  stw r7, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82DE1268: F92B0000  std r9, 0(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u64 ) };
	pc = 0x82DE126C; continue 'dispatch;
            }
            0x82DE126C => {
    //   block [0x82DE126C..0x82DE127C)
	// 82DE126C: 38A5FFFF  addi r5, r5, -1
	ctx.r[5].s64 = ctx.r[5].s64 + -1;
	// 82DE1270: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82DE1274: 7F1F2800  cmpw cr6, r31, r5
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[5].s32, &mut ctx.xer);
	// 82DE1278: 4099FF68  ble cr6, 0x82de11e0
	if !ctx.cr[6].gt {
	pc = 0x82DE11E0; continue 'dispatch;
	}
	pc = 0x82DE127C; continue 'dispatch;
            }
            0x82DE127C => {
    //   block [0x82DE127C..0x82DE1290)
	// 82DE127C: 7F042800  cmpw cr6, r4, r5
	ctx.cr[6].compare_i32(ctx.r[4].s32, ctx.r[5].s32, &mut ctx.xer);
	// 82DE1280: 40980010  bge cr6, 0x82de1290
	if !ctx.cr[6].lt {
	pc = 0x82DE1290; continue 'dispatch;
	}
	// 82DE1284: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82DE1288: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DE128C: 4BFFFF1D  bl 0x82de11a8
	ctx.lr = 0x82DE1290;
	sub_82DE11A8(ctx, base);
	pc = 0x82DE1290; continue 'dispatch;
            }
            0x82DE1290 => {
    //   block [0x82DE1290..0x82DE12A0)
	// 82DE1290: 7F1FE800  cmpw cr6, r31, r29
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[29].s32, &mut ctx.xer);
	// 82DE1294: 4098000C  bge cr6, 0x82de12a0
	if !ctx.cr[6].lt {
	pc = 0x82DE12A0; continue 'dispatch;
	}
	// 82DE1298: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DE129C: 4BFFFF24  b 0x82de11c0
	pc = 0x82DE11C0; continue 'dispatch;
            }
            0x82DE12A0 => {
    //   block [0x82DE12A0..0x82DE12A8)
	// 82DE12A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82DE12A4: 4BEC81B4  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE12A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DE12A8 size=20
    let mut pc: u32 = 0x82DE12A8;
    'dispatch: loop {
        match pc {
            0x82DE12A8 => {
    //   block [0x82DE12A8..0x82DE12BC)
	// 82DE12A8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82DE12AC: 38630030  addi r3, r3, 0x30
	ctx.r[3].s64 = ctx.r[3].s64 + 48;
	// 82DE12B0: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
	// 82DE12B4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82DE12B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE12C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DE12C0 size=8
    let mut pc: u32 = 0x82DE12C0;
    'dispatch: loop {
        match pc {
            0x82DE12C0 => {
    //   block [0x82DE12C0..0x82DE12C8)
	// 82DE12C0: 3863FFD0  addi r3, r3, -0x30
	ctx.r[3].s64 = ctx.r[3].s64 + -48;
	// 82DE12C4: 48000004  b 0x82de12c8
	sub_82DE12C8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE12C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DE12C8 size=100
    let mut pc: u32 = 0x82DE12C8;
    'dispatch: loop {
        match pc {
            0x82DE12C8 => {
    //   block [0x82DE12C8..0x82DE1310)
	// 82DE12C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DE12CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DE12D0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82DE12D4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DE12D8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DE12DC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DE12E0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82DE12E4: 4BFFF865  bl 0x82de0b48
	ctx.lr = 0x82DE12E8;
	sub_82DE0B48(ctx, base);
	// 82DE12E8: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82DE12EC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DE12F0: 419A0020  beq cr6, 0x82de1310
	if ctx.cr[6].eq {
	pc = 0x82DE1310; continue 'dispatch;
	}
	// 82DE12F4: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE12F8: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82DE12FC: 38C00026  li r6, 0x26
	ctx.r[6].s64 = 38;
	// 82DE1300: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DE1304: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82DE1308: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82DE130C: 4BF73FBD  bl 0x82d552c8
	ctx.lr = 0x82DE1310;
	sub_82D552C8(ctx, base);
	pc = 0x82DE1310; continue 'dispatch;
            }
            0x82DE1310 => {
    //   block [0x82DE1310..0x82DE132C)
	// 82DE1310: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DE1314: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DE1318: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DE131C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DE1320: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DE1324: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DE1328: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE1330(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DE1330 size=8
    let mut pc: u32 = 0x82DE1330;
    'dispatch: loop {
        match pc {
            0x82DE1330 => {
    //   block [0x82DE1330..0x82DE1338)
	// 82DE1330: 80630010  lwz r3, 0x10(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DE1334: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE1338(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DE1338 size=96
    let mut pc: u32 = 0x82DE1338;
    'dispatch: loop {
        match pc {
            0x82DE1338 => {
    //   block [0x82DE1338..0x82DE1398)
	// 82DE1338: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DE133C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82DE1340: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82DE1344: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DE1348: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82DE134C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DE1350: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 82DE1354: 388B4220  addi r4, r11, 0x4220
	ctx.r[4].s64 = ctx.r[11].s64 + 16928;
	// 82DE1358: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82DE135C: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE1360: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DE1364: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DE1368: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DE136C: 4E800421  bctrl
	ctx.lr = 0x82DE1370;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DE1370: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE1374: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DE1378: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82DE137C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82DE1380: 4E800421  bctrl
	ctx.lr = 0x82DE1384;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82DE1384: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82DE1388: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DE138C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DE1390: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DE1394: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE1398(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82DE1398 size=388
    let mut pc: u32 = 0x82DE1398;
    'dispatch: loop {
        match pc {
            0x82DE1398 => {
    //   block [0x82DE1398..0x82DE13C4)
	// 82DE1398: FBC1FFF0  std r30, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[30].u64 ) };
	// 82DE139C: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 82DE13A0: C0090010  lfs f0, 0x10(r9)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(16 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DE13A4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DE13A8: D001FFE0  stfs f0, -0x20(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), tmp.u32 ) };
	// 82DE13AC: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82DE13B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DE13B4: 2F070004  cmpwi cr6, r7, 4
	ctx.cr[6].compare_i32(ctx.r[7].s32, 4, &mut ctx.xer);
	// 82DE13B8: 41980070  blt cr6, 0x82de1428
	if ctx.cr[6].lt {
	pc = 0x82DE1428; continue 'dispatch;
	}
	// 82DE13BC: 3887FFFD  addi r4, r7, -3
	ctx.r[4].s64 = ctx.r[7].s64 + -3;
	// 82DE13C0: 39450008  addi r10, r5, 8
	ctx.r[10].s64 = ctx.r[5].s64 + 8;
	pc = 0x82DE13C4; continue 'dispatch;
            }
            0x82DE13C4 => {
    //   block [0x82DE13C4..0x82DE13D8)
	// 82DE13C4: C1AAFFF8  lfs f13, -8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-8 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DE13C8: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 82DE13CC: 4098000C  bge cr6, 0x82de13d8
	if !ctx.cr[6].lt {
	pc = 0x82DE13D8; continue 'dispatch;
	}
	// 82DE13D0: FC006890  fmr f0, f13
	ctx.f[0].f64 = ctx.f[13].f64;
	// 82DE13D4: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	pc = 0x82DE13D8; continue 'dispatch;
            }
            0x82DE13D8 => {
    //   block [0x82DE13D8..0x82DE13EC)
	// 82DE13D8: C1AAFFFC  lfs f13, -4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DE13DC: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 82DE13E0: 4098000C  bge cr6, 0x82de13ec
	if !ctx.cr[6].lt {
	pc = 0x82DE13EC; continue 'dispatch;
	}
	// 82DE13E4: FC006890  fmr f0, f13
	ctx.f[0].f64 = ctx.f[13].f64;
	// 82DE13E8: 386B0001  addi r3, r11, 1
	ctx.r[3].s64 = ctx.r[11].s64 + 1;
	pc = 0x82DE13EC; continue 'dispatch;
            }
            0x82DE13EC => {
    //   block [0x82DE13EC..0x82DE1400)
	// 82DE13EC: C1AA0000  lfs f13, 0(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DE13F0: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 82DE13F4: 4098000C  bge cr6, 0x82de1400
	if !ctx.cr[6].lt {
	pc = 0x82DE1400; continue 'dispatch;
	}
	// 82DE13F8: FC006890  fmr f0, f13
	ctx.f[0].f64 = ctx.f[13].f64;
	// 82DE13FC: 386B0002  addi r3, r11, 2
	ctx.r[3].s64 = ctx.r[11].s64 + 2;
	pc = 0x82DE1400; continue 'dispatch;
            }
            0x82DE1400 => {
    //   block [0x82DE1400..0x82DE1414)
	// 82DE1400: C1AA0004  lfs f13, 4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DE1404: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 82DE1408: 4098000C  bge cr6, 0x82de1414
	if !ctx.cr[6].lt {
	pc = 0x82DE1414; continue 'dispatch;
	}
	// 82DE140C: FC006890  fmr f0, f13
	ctx.f[0].f64 = ctx.f[13].f64;
	// 82DE1410: 386B0003  addi r3, r11, 3
	ctx.r[3].s64 = ctx.r[11].s64 + 3;
	pc = 0x82DE1414; continue 'dispatch;
            }
            0x82DE1414 => {
    //   block [0x82DE1414..0x82DE1428)
	// 82DE1414: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82DE1418: 394A0010  addi r10, r10, 0x10
	ctx.r[10].s64 = ctx.r[10].s64 + 16;
	// 82DE141C: 7F0B2000  cmpw cr6, r11, r4
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[4].s32, &mut ctx.xer);
	// 82DE1420: 4198FFA4  blt cr6, 0x82de13c4
	if ctx.cr[6].lt {
	pc = 0x82DE13C4; continue 'dispatch;
	}
	// 82DE1424: D001FFE0  stfs f0, -0x20(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), tmp.u32 ) };
	pc = 0x82DE1428; continue 'dispatch;
            }
            0x82DE1428 => {
    //   block [0x82DE1428..0x82DE1438)
	// 82DE1428: 7F0B3800  cmpw cr6, r11, r7
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[7].s32, &mut ctx.xer);
	// 82DE142C: 40980034  bge cr6, 0x82de1460
	if !ctx.cr[6].lt {
	pc = 0x82DE1460; continue 'dispatch;
	}
	// 82DE1430: 556A103A  slwi r10, r11, 2
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82DE1434: 7D4A2A14  add r10, r10, r5
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[5].u64;
	pc = 0x82DE1438; continue 'dispatch;
            }
            0x82DE1438 => {
    //   block [0x82DE1438..0x82DE144C)
	// 82DE1438: C1AA0000  lfs f13, 0(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DE143C: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 82DE1440: 4098000C  bge cr6, 0x82de144c
	if !ctx.cr[6].lt {
	pc = 0x82DE144C; continue 'dispatch;
	}
	// 82DE1444: FC006890  fmr f0, f13
	ctx.f[0].f64 = ctx.f[13].f64;
	// 82DE1448: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	pc = 0x82DE144C; continue 'dispatch;
            }
            0x82DE144C => {
    //   block [0x82DE144C..0x82DE1460)
	// 82DE144C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82DE1450: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82DE1454: 7F0B3800  cmpw cr6, r11, r7
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[7].s32, &mut ctx.xer);
	// 82DE1458: 4198FFE0  blt cr6, 0x82de1438
	if ctx.cr[6].lt {
	pc = 0x82DE1438; continue 'dispatch;
	}
	// 82DE145C: D001FFE0  stfs f0, -0x20(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), tmp.u32 ) };
	pc = 0x82DE1460; continue 'dispatch;
            }
            0x82DE1460 => {
    //   block [0x82DE1460..0x82DE150C)
	// 82DE1460: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 82DE1464: 419A00A8  beq cr6, 0x82de150c
	if ctx.cr[6].eq {
	pc = 0x82DE150C; continue 'dispatch;
	}
	// 82DE1468: 81690040  lwz r11, 0x40(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(64 as u32) ) } as u64;
	// 82DE146C: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 82DE1470: 38A1FFE0  addi r5, r1, -0x20
	ctx.r[5].s64 = ctx.r[1].s64 + -32;
	// 82DE1474: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82DE1478: 5467103A  slwi r7, r3, 2
	ctx.r[7].u32 = ctx.r[3].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82DE147C: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DE1480: 38800010  li r4, 0x10
	ctx.r[4].s64 = 16;
	pc = 0x82DE150C; continue 'dispatch;
            }
            0x82DE150C => {
    //   block [0x82DE150C..0x82DE151C)
	// 82DE150C: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82DE1510: EBC1FFF0  ld r30, -0x10(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DE1514: EBE1FFF8  ld r31, -8(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) };
	// 82DE1518: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE1520(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DE1520 size=92
    let mut pc: u32 = 0x82DE1520;
    'dispatch: loop {
        match pc {
            0x82DE1520 => {
    //   block [0x82DE1520..0x82DE1574)
	// 82DE1520: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82DE1524: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82DE1528: 396B2314  addi r11, r11, 0x2314
	ctx.r[11].s64 = ctx.r[11].s64 + 8980;
	// 82DE152C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82DE1530: 38E00012  li r7, 0x12
	ctx.r[7].s64 = 18;
	// 82DE1534: 7C8A2378  mr r10, r4
	ctx.r[10].u64 = ctx.r[4].u64;
	// 82DE1538: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 82DE153C: B1230006  sth r9, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 82DE1540: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DE1544: 91030008  stw r8, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[8].u32 ) };
	// 82DE1548: 90E3000C  stw r7, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[7].u32 ) };
	// 82DE154C: 40990028  ble cr6, 0x82de1574
	if !ctx.cr[6].gt {
	pc = 0x82DE1574; continue 'dispatch;
	}
	// 82DE1550: 39230020  addi r9, r3, 0x20
	ctx.r[9].s64 = ctx.r[3].s64 + 32;
	// 82DE1554: 7CAB2B78  mr r11, r5
	ctx.r[11].u64 = ctx.r[5].u64;
	// 82DE1558: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	pc = 0x82DE1574; continue 'dispatch;
            }
            0x82DE1574 => {
    //   block [0x82DE1574..0x82DE157C)
	// 82DE1574: 90A30010  stw r5, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[5].u32 ) };
	// 82DE1578: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE1580(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DE1580 size=372
    let mut pc: u32 = 0x82DE1580;
    'dispatch: loop {
        match pc {
            0x82DE1580 => {
    //   block [0x82DE1580..0x82DE16F4)
	// 82DE1580: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DE1584: 4BEC7E75  bl 0x82ca93f8
	ctx.lr = 0x82DE1588;
	sub_82CA93D0(ctx, base);
	// 82DE1588: 39040030  addi r8, r4, 0x30
	ctx.r[8].s64 = ctx.r[4].s64 + 48;
	// 82DE158C: EB640000  ld r27, 0(r4)
	ctx.r[27].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
	// 82DE1590: 39440010  addi r10, r4, 0x10
	ctx.r[10].s64 = ctx.r[4].s64 + 16;
	// 82DE1594: 83A30010  lwz r29, 0x10(r3)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DE1598: 39240020  addi r9, r4, 0x20
	ctx.r[9].s64 = ctx.r[4].s64 + 32;
	// 82DE159C: E8840008  ld r4, 8(r4)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) };
	// 82DE15A0: 38E1FF20  addi r7, r1, -0xe0
	ctx.r[7].s64 = ctx.r[1].s64 + -224;
	// 82DE15A4: 3BC1FEF0  addi r30, r1, -0x110
	ctx.r[30].s64 = ctx.r[1].s64 + -272;
	// 82DE15A8: EB080000  ld r24, 0(r8)
	ctx.r[24].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) };
	// 82DE15AC: 38A1FF10  addi r5, r1, -0xf0
	ctx.r[5].s64 = ctx.r[1].s64 + -240;
	// 82DE15B0: E9080008  ld r8, 8(r8)
	ctx.r[8].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[8].u32.wrapping_add(8 as u32) ) };
	// 82DE15B4: 3BE1FF00  addi r31, r1, -0x100
	ctx.r[31].s64 = ctx.r[1].s64 + -256;
	// 82DE15B8: EB4A0000  ld r26, 0(r10)
	ctx.r[26].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	// 82DE15BC: 397DFFFF  addi r11, r29, -1
	ctx.r[11].s64 = ctx.r[29].s64 + -1;
	// 82DE15C0: FB670000  std r27, 0(r7)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), ctx.r[27].u64 ) };
	// 82DE15C4: 3B81FF30  addi r28, r1, -0xd0
	ctx.r[28].s64 = ctx.r[1].s64 + -208;
	// 82DE15C8: F8870008  std r4, 8(r7)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[7].u32.wrapping_add(8 as u32), ctx.r[4].u64 ) };
	// 82DE15CC: FB1E0000  std r24, 0(r30)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[24].u64 ) };
	// 82DE15D0: F91E0008  std r8, 8(r30)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[8].u64 ) };
	// 82DE15D4: 3901FF20  addi r8, r1, -0xe0
	ctx.r[8].s64 = ctx.r[1].s64 + -224;
	// 82DE15D8: E94A0008  ld r10, 8(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) };
	// 82DE15DC: FB450000  std r26, 0(r5)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[26].u64 ) };
	// 82DE15E0: EB290000  ld r25, 0(r9)
	ctx.r[25].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) };
	// 82DE15E4: E9290008  ld r9, 8(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE16F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DE16F8 size=68
    let mut pc: u32 = 0x82DE16F8;
    'dispatch: loop {
        match pc {
            0x82DE16F8 => {
    //   block [0x82DE16F8..0x82DE173C)
	// 82DE16F8: 7C681B78  mr r8, r3
	ctx.r[8].u64 = ctx.r[3].u64;
	// 82DE16FC: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82DE1700: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82DE1704: 81680010  lwz r11, 0x10(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DE1708: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82DE170C: 4C990020  blelr cr6
	if !ctx.cr[6].gt { return; }
	// 82DE1710: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 82DE1714: 39680020  addi r11, r8, 0x20
	ctx.r[11].s64 = ctx.r[8].s64 + 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE1740(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82DE1740 size=424
    let mut pc: u32 = 0x82DE1740;
    'dispatch: loop {
        match pc {
            0x82DE1740 => {
    //   block [0x82DE1740..0x82DE1790)
	// 82DE1740: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DE1744: 4BEC7CB5  bl 0x82ca93f8
	ctx.lr = 0x82DE1748;
	sub_82CA93D0(ctx, base);
	// 82DE1748: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE174C: 39600008  li r11, 8
	ctx.r[11].s64 = 8;
	// 82DE1750: 7CE83B78  mr r8, r7
	ctx.r[8].u64 = ctx.r[7].u64;
	// 82DE1754: 7F8B5214  add r28, r11, r10
	ctx.r[28].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82DE1758: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82DE175C: 7C892378  mr r9, r4
	ctx.r[9].u64 = ctx.r[4].u64;
	// 82DE1760: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE1764: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DE1768: 80EB000C  lwz r7, 0xc(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DE176C: 7F0A3840  cmplw cr6, r10, r7
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[7].u32, &mut ctx.xer);
	// 82DE1770: 40980020  bge cr6, 0x82de1790
	if !ctx.cr[6].lt {
	pc = 0x82DE1790; continue 'dispatch;
	}
	// 82DE1774: 3CE08203  lis r7, -0x7dfd
	ctx.r[7].s64 = -2113732608;
	// 82DE1778: 38E7422C  addi r7, r7, 0x422c
	ctx.r[7].s64 = ctx.r[7].s64 + 16940;
	// 82DE177C: 90EA0000  stw r7, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82DE1780: 7CEC42E6  mftb r7, 0x10c
	ctx.r[7].u64 = crate::rt::rdtsc_u64();
	// 82DE1784: 388A000C  addi r4, r10, 0xc
	ctx.r[4].s64 = ctx.r[10].s64 + 12;
	// 82DE1788: 90EA0004  stw r7, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82DE178C: 908B0004  stw r4, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	pc = 0x82DE1790; continue 'dispatch;
            }
            0x82DE1790 => {
    //   block [0x82DE1790..0x82DE18B4)
	// 82DE1790: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82DE1794: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82DE1798: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 82DE179C: 40990118  ble cr6, 0x82de18b4
	if !ctx.cr[6].gt {
	pc = 0x82DE18B4; continue 'dispatch;
	}
	// 82DE17A0: 7FA83050  subf r29, r8, r6
	ctx.r[29].s64 = ctx.r[6].s64 - ctx.r[8].s64;
	// 82DE17A4: 3C808200  lis r4, -0x7e00
	ctx.r[4].s64 = -2113929216;
	// 82DE17A8: 3CC08200  lis r6, -0x7e00
	ctx.r[6].s64 = -2113929216;
	// 82DE17AC: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82DE17B0: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82DE17B4: 38FF0010  addi r7, r31, 0x10
	ctx.r[7].s64 = ctx.r[31].s64 + 16;
	// 82DE17B8: C1240BFC  lfs f9, 0xbfc(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(3068 as u32) ) };
	ctx.f[9].f64 = (tmp.f32 as f64);
	// 82DE17BC: C1460A4C  lfs f10, 0xa4c(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(2636 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 82DE17C0: C16A0C18  lfs f11, 0xc18(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(3096 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82DE17C4: C10B0C4C  lfs f8, 0xc4c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3148 as u32) ) };
	ctx.f[8].f64 = (tmp.f32 as f64);
	// 82DE17C8: EB7F0000  ld r27, 0(r31)
	ctx.r[27].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) };
	// 82DE17CC: 3961FF90  addi r11, r1, -0x70
	ctx.r[11].s64 = ctx.r[1].s64 + -112;
	// 82DE17D0: EB5F0008  ld r26, 8(r31)
	ctx.r[26].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) };
	// 82DE17D4: 3941FFA0  addi r10, r1, -0x60
	ctx.r[10].s64 = ctx.r[1].s64 + -96;
	// 82DE17D8: EB270000  ld r25, 0(r7)
	ctx.r[25].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) };
	pc = 0x82DE18B4; continue 'dispatch;
            }
            0x82DE18B4 => {
    //   block [0x82DE18B4..0x82DE18E8)
	// 82DE18B4: 815C0000  lwz r10, 0(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE18B8: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DE18BC: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DE18C0: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DE18C4: 40980020  bge cr6, 0x82de18e4
	if !ctx.cr[6].lt {
	pc = 0x82DE18E4; continue 'dispatch;
	}
	// 82DE18C8: 3D208202  lis r9, -0x7dfe
	ctx.r[9].s64 = -2113798144;
	// 82DE18CC: 392964CC  addi r9, r9, 0x64cc
	ctx.r[9].s64 = ctx.r[9].s64 + 25804;
	// 82DE18D0: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DE18D4: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82DE18D8: 390B000C  addi r8, r11, 0xc
	ctx.r[8].s64 = ctx.r[11].s64 + 12;
	// 82DE18DC: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82DE18E0: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82DE18E4: 4BEC7B64  b 0x82ca9448
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE18E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82DE18E8 size=124
    let mut pc: u32 = 0x82DE18E8;
    'dispatch: loop {
        match pc {
            0x82DE18E8 => {
    //   block [0x82DE18E8..0x82DE1964)
	// 82DE18E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82DE18EC: 4BEC7B19  bl 0x82ca9404
	ctx.lr = 0x82DE18F0;
	sub_82CA93D0(ctx, base);
	// 82DE18F0: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82DE18F4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82DE18F8: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82DE18FC: 3B9F0020  addi r28, r31, 0x20
	ctx.r[28].s64 = ctx.r[31].s64 + 32;
	// 82DE1900: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82DE1904: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 82DE1908: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 82DE190C: 80BF0010  lwz r5, 0x10(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DE1910: 38C10070  addi r6, r1, 0x70
	ctx.r[6].s64 = ctx.r[1].s64 + 112;
	// 82DE1914: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82DE1918: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82DE191C: 4BFFFE25  bl 0x82de1740
	ctx.lr = 0x82DE1920;
	sub_82DE1740(ctx, base);
	// 82DE1920: 7C671B78  mr r7, r3
	ctx.r[7].u64 = ctx.r[3].u64;
	// 82DE1924: 7F69DB78  mr r9, r27
	ctx.r[9].u64 = ctx.r[27].u64;
	// 82DE1928: 809F0010  lwz r4, 0x10(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DE192C: 7FA8EB78  mr r8, r29
	ctx.r[8].u64 = ctx.r[29].u64;
	// 82DE1930: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82DE1934: 38A10070  addi r5, r1, 0x70
	ctx.r[5].s64 = ctx.r[1].s64 + 112;
	// 82DE1938: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82DE193C: 4BFFFA5D  bl 0x82de1398
	ctx.lr = 0x82DE1940;
	sub_82DE1398(ctx, base);
	// 82DE1940: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82DE1944: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82DE1948: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82DE194C: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82DE1950: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82DE1954: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 82DE1958: 997E0000  stb r11, 0(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82DE195C: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 82DE1960: 4BEC7AF4  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE1968(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DE1968 size=320
    let mut pc: u32 = 0x82DE1968;
    'dispatch: loop {
        match pc {
            0x82DE1968 => {
    //   block [0x82DE1968..0x82DE1A9C)
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
	pc = 0x82DE1A9C; continue 'dispatch;
            }
            0x82DE1A9C => {
    //   block [0x82DE1A9C..0x82DE1AA8)
	// 82DE1A9C: 38210160  addi r1, r1, 0x160
	ctx.r[1].s64 = ctx.r[1].s64 + 352;
	// 82DE1AA0: CBE1FF88  lfd f31, -0x78(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-120 as u32) ) };
	// 82DE1AA4: 4BEC7990  b 0x82ca9434
	sub_82CA9420(ctx, base);
	return;
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
    //   block [0x82DE1B08..0x82DE1B9C)
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
            }
            0x82DE1B9C => {
    //   block [0x82DE1B9C..0x82DE1BB4)
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
    // ---- function 0x82DE1D20 size=104
    let mut pc: u32 = 0x82DE1D20;
    'dispatch: loop {
        match pc {
            0x82DE1D20 => {
    //   block [0x82DE1D20..0x82DE1D88)
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
    //   block [0x82DE1EC0..0x82DE1F00)
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
	pc = 0x82DE1F00; continue 'dispatch;
            }
            0x82DE1F00 => {
    //   block [0x82DE1F00..0x82DE2058)
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
    //   block [0x82DE20A0..0x82DE20BC)
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
	pc = 0x82DE20BC; continue 'dispatch;
            }
            0x82DE20BC => {
    //   block [0x82DE20BC..0x82DE20FC)
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
	pc = 0x82DE20FC; continue 'dispatch;
            }
            0x82DE20FC => {
    //   block [0x82DE20FC..0x82DE2104)
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
    // ---- function 0x82DE2108 size=68
    let mut pc: u32 = 0x82DE2108;
    'dispatch: loop {
        match pc {
            0x82DE2108 => {
    //   block [0x82DE2108..0x82DE211C)
	// 82DE2108: 81430010  lwz r10, 0x10(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82DE210C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DE2110: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82DE2114: 4C990020  blelr cr6
	if !ctx.cr[6].gt { return; }
	// 82DE2118: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	pc = 0x82DE211C; continue 'dispatch;
            }
            0x82DE211C => {
    //   block [0x82DE211C..0x82DE214C)
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
    //   block [0x82DE2150..0x82DE2168)
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
	pc = 0x82DE2168; continue 'dispatch;
            }
            0x82DE2168 => {
    //   block [0x82DE2168..0x82DE2184)
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
	pc = 0x82DE2184; continue 'dispatch;
            }
            0x82DE2184 => {
    //   block [0x82DE2184..0x82DE2188)
	// 82DE2184: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	pc = 0x82DE2188; continue 'dispatch;
            }
            0x82DE2188 => {
    //   block [0x82DE2188..0x82DE21A0)
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
    //   block [0x82DE21A0..0x82DE22F4)
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
            0x82DE22F4 => {
    //   block [0x82DE22F4..0x82DE230C)
	// 82DE22F4: 7EE5BB78  mr r5, r23
	ctx.r[5].u64 = ctx.r[23].u64;
	// 82DE22F8: 93DF0018  stw r30, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[30].u32 ) };
	// 82DE22FC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82DE2300: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 82DE2304: 4804251D  bl 0x82e24820
	ctx.lr = 0x82DE2308;
	sub_82E24820(ctx, base);
	// 82DE2308: 933F0018  stw r25, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[25].u32 ) };
	pc = 0x82DE230C; continue 'dispatch;
            }
            0x82DE230C => {
    //   block [0x82DE230C..0x82DE231C)
	// 82DE230C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82DE2310: 382102F0  addi r1, r1, 0x2f0
	ctx.r[1].s64 = ctx.r[1].s64 + 752;
	// 82DE2314: CBE1FFA8  lfd f31, -0x58(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-88 as u32) ) };
	// 82DE2318: 4BEC712C  b 0x82ca9444
	sub_82CA9420(ctx, base);
	return;
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
    //   block [0x82DE23B0..0x82DE2408)
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
	pc = 0x82DE2408; continue 'dispatch;
            }
            0x82DE2408 => {
    //   block [0x82DE2408..0x82DE2434)
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
    //   block [0x82DE2438..0x82DE2474)
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
	pc = 0x82DE2474; continue 'dispatch;
            }
            0x82DE2474 => {
    //   block [0x82DE2474..0x82DE24BC)
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
    //   block [0x82DE27F0..0x82DE281C)
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
	pc = 0x82DE281C; continue 'dispatch;
            }
            0x82DE281C => {
    //   block [0x82DE281C..0x82DE2858)
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
            }
            0x82DE2858 => {
    //   block [0x82DE2858..0x82DE2874)
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
	pc = 0x82DE2874; continue 'dispatch;
            }
            0x82DE2874 => {
    //   block [0x82DE2874..0x82DE2884)
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
    //   block [0x82DE2A70..0x82DE2AEC)
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
            }
            0x82DE2AEC => {
    //   block [0x82DE2AEC..0x82DE2AF4)
	// 82DE2AEC: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82DE2AF0: B3E10050  sth r31, 0x50(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u16 ) };
	pc = 0x82DE2AF4; continue 'dispatch;
            }
            0x82DE2AF4 => {
    //   block [0x82DE2AF4..0x82DE2B28)
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
    //   block [0x82DE2C40..0x82DE2C98)
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
	pc = 0x82DE2C98; continue 'dispatch;
            }
            0x82DE2C98 => {
    //   block [0x82DE2C98..0x82DE2CF4)
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
    // ---- function 0x82DE2E30 size=20
    let mut pc: u32 = 0x82DE2E30;
    'dispatch: loop {
        match pc {
            0x82DE2E30 => {
    //   block [0x82DE2E30..0x82DE2E44)
	// 82DE2E30: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82DE2E34: 38630014  addi r3, r3, 0x14
	ctx.r[3].s64 = ctx.r[3].s64 + 20;
	// 82DE2E38: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
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
    // ---- function 0x82DE2E50 size=28
    let mut pc: u32 = 0x82DE2E50;
    'dispatch: loop {
        match pc {
            0x82DE2E50 => {
    //   block [0x82DE2E50..0x82DE2E6C)
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
    // ---- function 0x82DE2EA8 size=80
    let mut pc: u32 = 0x82DE2EA8;
    'dispatch: loop {
        match pc {
            0x82DE2EA8 => {
    //   block [0x82DE2EA8..0x82DE2EF8)
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
    //   block [0x82DE2EF8..0x82DE2F9C)
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
            0x82DE2F9C => {
    //   block [0x82DE2F9C..0x82DE2FA4)
	// 82DE2F9C: 38210290  addi r1, r1, 0x290
	ctx.r[1].s64 = ctx.r[1].s64 + 656;
	// 82DE2FA0: 4BEC64AC  b 0x82ca944c
	sub_82CA9420(ctx, base);
	return;
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
    //   block [0x82DE30E8..0x82DE31C4)
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
            0x82DE31C4 => {
    //   block [0x82DE31C4..0x82DE31D8)
	// 82DE31C4: 672B3F00  oris r11, r25, 0x3f00
	ctx.r[11].u64 = ctx.r[25].u64 | 1056964608;
	// 82DE31C8: 9178000C  stw r11, 0xc(r24)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[24].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82DE31CC: 382102D0  addi r1, r1, 0x2d0
	ctx.r[1].s64 = ctx.r[1].s64 + 720;
	// 82DE31D0: CBE1FFA8  lfd f31, -0x58(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-88 as u32) ) };
	// 82DE31D4: 4BEC6270  b 0x82ca9444
	sub_82CA9420(ctx, base);
	return;
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
    //   block [0x82DE31D8..0x82DE322C)
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
	pc = 0x82DE322C; continue 'dispatch;
            }
            0x82DE322C => {
    //   block [0x82DE322C..0x82DE3250)
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
	pc = 0x82DE3250; continue 'dispatch;
            }
            0x82DE3250 => {
    //   block [0x82DE3250..0x82DE3270)
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
	pc = 0x82DE3270; continue 'dispatch;
            }
            0x82DE3270 => {
    //   block [0x82DE3270..0x82DE32A0)
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
            }
            0x82DE32A0 => {
    //   block [0x82DE32A0..0x82DE32F4)
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
            }
            0x82DE32F4 => {
    //   block [0x82DE32F4..0x82DE3308)
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
	pc = 0x82DE3308; continue 'dispatch;
            }
            0x82DE3308 => {
    //   block [0x82DE3308..0x82DE3328)
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
	pc = 0x82DE3328; continue 'dispatch;
            }
            0x82DE3328 => {
    //   block [0x82DE3328..0x82DE3358)
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
	pc = 0x82DE3358; continue 'dispatch;
            }
            0x82DE3358 => {
    //   block [0x82DE3358..0x82DE3378)
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
    //   block [0x82DE3378..0x82DE33CC)
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
	pc = 0x82DE33CC; continue 'dispatch;
            }
            0x82DE33CC => {
    //   block [0x82DE33CC..0x82DE33E0)
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
	pc = 0x82DE33E0; continue 'dispatch;
            }
            0x82DE33E0 => {
    //   block [0x82DE33E0..0x82DE3400)
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
	pc = 0x82DE3400; continue 'dispatch;
            }
            0x82DE3400 => {
    //   block [0x82DE3400..0x82DE3430)
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
            }
            0x82DE3430 => {
    //   block [0x82DE3430..0x82DE3484)
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
            }
            0x82DE3484 => {
    //   block [0x82DE3484..0x82DE3498)
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
	pc = 0x82DE3498; continue 'dispatch;
            }
            0x82DE3498 => {
    //   block [0x82DE3498..0x82DE34C8)
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
	pc = 0x82DE34C8; continue 'dispatch;
            }
            0x82DE34C8 => {
    //   block [0x82DE34C8..0x82DE34D0)
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
    //   block [0x82DE34D8..0x82DE34FC)
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
	pc = 0x82DE34FC; continue 'dispatch;
            }
            0x82DE34FC => {
    //   block [0x82DE34FC..0x82DE353C)
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
	pc = 0x82DE353C; continue 'dispatch;
            }
            0x82DE353C => {
    //   block [0x82DE353C..0x82DE3554)
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
    // ---- function 0x82DE36E8 size=28
    let mut pc: u32 = 0x82DE36E8;
    'dispatch: loop {
        match pc {
            0x82DE36E8 => {
    //   block [0x82DE36E8..0x82DE3704)
	// 82DE36E8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82DE36EC: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
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
    //   block [0x82DE3760..0x82DE37CC)
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
            }
            0x82DE37CC => {
    //   block [0x82DE37CC..0x82DE37D0)
	// 82DE37CC: 48034E6D  bl 0x82e18638
	ctx.lr = 0x82DE37D0;
	sub_82E18638(ctx, base);
	pc = 0x82DE37D0; continue 'dispatch;
            }
            0x82DE37D0 => {
    //   block [0x82DE37D0..0x82DE3850)
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
    //   block [0x82DE38F8..0x82DE3958)
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
	pc = 0x82DE3958; continue 'dispatch;
            }
            0x82DE3958 => {
    //   block [0x82DE3958..0x82DE3988)
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
	pc = 0x82DE3988; continue 'dispatch;
            }
            0x82DE3988 => {
    //   block [0x82DE3988..0x82DE3A50)
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
            }
            0x82DE3A50 => {
    //   block [0x82DE3A50..0x82DE3AC0)
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
	pc = 0x82DE3AC0; continue 'dispatch;
            }
            0x82DE3AC0 => {
    //   block [0x82DE3AC0..0x82DE3B24)
	// 82DE3AC0: 397B0020  addi r11, r27, 0x20
	ctx.r[11].s64 = ctx.r[27].s64 + 32;
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
    //   block [0x82DE3B28..0x82DE3B78)
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
	pc = 0x82DE3B78; continue 'dispatch;
            }
            0x82DE3B78 => {
    //   block [0x82DE3B78..0x82DE3BB4)
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
            }
            0x82DE3BB4 => {
    //   block [0x82DE3BB4..0x82DE3BB8)
	// 82DE3BB4: 48034A85  bl 0x82e18638
	ctx.lr = 0x82DE3BB8;
	sub_82E18638(ctx, base);
	pc = 0x82DE3BB8; continue 'dispatch;
            }
            0x82DE3BB8 => {
    //   block [0x82DE3BB8..0x82DE3C7C)
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
            }
            0x82DE3C7C => {
    //   block [0x82DE3C7C..0x82DE3CA8)
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
	pc = 0x82DE3CA8; continue 'dispatch;
            }
            0x82DE3CA8 => {
    //   block [0x82DE3CA8..0x82DE3CC4)
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
            }
            0x82DE3CC4 => {
    //   block [0x82DE3CC4..0x82DE3CCC)
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
    //   block [0x82DE3CD0..0x82DE3D24)
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
	pc = 0x82DE3D24; continue 'dispatch;
            }
            0x82DE3D24 => {
    //   block [0x82DE3D24..0x82DE3D60)
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
            }
            0x82DE3D60 => {
    //   block [0x82DE3D60..0x82DE3D64)
	// 82DE3D60: 480348D9  bl 0x82e18638
	ctx.lr = 0x82DE3D64;
	sub_82E18638(ctx, base);
	pc = 0x82DE3D64; continue 'dispatch;
            }
            0x82DE3D64 => {
    //   block [0x82DE3D64..0x82DE3EEC)
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
            0x82DE3EEC => {
    //   block [0x82DE3EEC..0x82DE3FB4)
	// 82DE3EEC: 396100A0  addi r11, r1, 0xa0
	ctx.r[11].s64 = ctx.r[1].s64 + 160;
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
    //   block [0x82DE3FB8..0x82DE4098)
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
            }
            0x82DE4098 => {
    //   block [0x82DE4098..0x82DE41B8)
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
    //   block [0x82DE41B8..0x82DE4210)
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
	pc = 0x82DE4210; continue 'dispatch;
            }
            0x82DE4210 => {
    //   block [0x82DE4210..0x82DE42A4)
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
	pc = 0x82DE42A4; continue 'dispatch;
            }
            0x82DE42A4 => {
    //   block [0x82DE42A4..0x82DE42DC)
	// 82DE42A4: 7D5DE02E  lwzx r10, r29, r28
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 82DE42A8: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DE42AC: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DE42B0: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DE42B4: 40980020  bge cr6, 0x82de42d4
	if !ctx.cr[6].lt {
	pc = 0x82DE42D4; continue 'dispatch;
	}
	// 82DE42B8: 3D208202  lis r9, -0x7dfe
	ctx.r[9].s64 = -2113798144;
	// 82DE42BC: 392964CC  addi r9, r9, 0x64cc
	ctx.r[9].s64 = ctx.r[9].s64 + 25804;
	// 82DE42C0: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DE42C4: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82DE42C8: 390B000C  addi r8, r11, 0xc
	ctx.r[8].s64 = ctx.r[11].s64 + 12;
	// 82DE42CC: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82DE42D0: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82DE42D4: 38210140  addi r1, r1, 0x140
	ctx.r[1].s64 = ctx.r[1].s64 + 320;
	// 82DE42D8: 4BEC5174  b 0x82ca944c
	sub_82CA9420(ctx, base);
	return;
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
    // ---- function 0x82DE4330 size=56
    let mut pc: u32 = 0x82DE4330;
    'dispatch: loop {
        match pc {
            0x82DE4330 => {
    //   block [0x82DE4330..0x82DE4348)
	// 82DE4330: 89230032  lbz r9, 0x32(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(50 as u32) ) } as u64;
	// 82DE4334: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82DE4338: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82DE433C: 4C990020  blelr cr6
	if !ctx.cr[6].gt { return; }
	// 82DE4340: 5488043E  clrlwi r8, r4, 0x10
	ctx.r[8].u64 = ctx.r[4].u32 as u64 & 0x0000FFFFu64;
	// 82DE4344: 39630036  addi r11, r3, 0x36
	ctx.r[11].s64 = ctx.r[3].s64 + 54;
	pc = 0x82DE4348; continue 'dispatch;
            }
            0x82DE4348 => {
    //   block [0x82DE4348..0x82DE4368)
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
    // ---- function 0x82DE4378 size=52
    let mut pc: u32 = 0x82DE4378;
    'dispatch: loop {
        match pc {
            0x82DE4378 => {
    //   block [0x82DE4378..0x82DE438C)
	// 82DE4378: 89230032  lbz r9, 0x32(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(50 as u32) ) } as u64;
	// 82DE437C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82DE4380: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82DE4384: 4C990020  blelr cr6
	if !ctx.cr[6].gt { return; }
	// 82DE4388: 39430036  addi r10, r3, 0x36
	ctx.r[10].s64 = ctx.r[3].s64 + 54;
	pc = 0x82DE438C; continue 'dispatch;
            }
            0x82DE438C => {
    //   block [0x82DE438C..0x82DE43AC)
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
    // ---- function 0x82DE43C0 size=60
    let mut pc: u32 = 0x82DE43C0;
    'dispatch: loop {
        match pc {
            0x82DE43C0 => {
    //   block [0x82DE43C0..0x82DE43DC)
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
	// 82DE43D4: 5488043E  clrlwi r8, r4, 0x10
	ctx.r[8].u64 = ctx.r[4].u32 as u64 & 0x0000FFFFu64;
	// 82DE43D8: 39630034  addi r11, r3, 0x34
	ctx.r[11].s64 = ctx.r[3].s64 + 52;
	pc = 0x82DE43DC; continue 'dispatch;
            }
            0x82DE43DC => {
    //   block [0x82DE43DC..0x82DE43FC)
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
    //   block [0x82DE4450..0x82DE44C4)
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
            }
            0x82DE44C4 => {
    //   block [0x82DE44C4..0x82DE44EC)
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
    //   block [0x82DE4618..0x82DE466C)
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
	pc = 0x82DE466C; continue 'dispatch;
            }
            0x82DE466C => {
    //   block [0x82DE466C..0x82DE476C)
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
	pc = 0x82DE476C; continue 'dispatch;
            }
            0x82DE476C => {
    //   block [0x82DE476C..0x82DE47E0)
	// 82DE476C: 815C0000  lwz r10, 0(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE4770: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DE4774: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DE4778: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DE477C: 40980020  bge cr6, 0x82de479c
	if !ctx.cr[6].lt {
	pc = 0x82DE479C; continue 'dispatch;
	}
	// 82DE4780: 3D208203  lis r9, -0x7dfd
	ctx.r[9].s64 = -2113732608;
	// 82DE4784: 39293700  addi r9, r9, 0x3700
	ctx.r[9].s64 = ctx.r[9].s64 + 14080;
	// 82DE4788: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DE478C: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82DE4790: 390B000C  addi r8, r11, 0xc
	ctx.r[8].s64 = ctx.r[11].s64 + 12;
	// 82DE4794: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82DE4798: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82DE479C: C0060054  lfs f0, 0x54(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(84 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82DE47A0: D01B0000  stfs f0, 0(r27)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82DE47A4: 4BFFFDF5  bl 0x82de4598
	ctx.lr = 0x82DE47A8;
	sub_82DE4598(ctx, base);
	// 82DE47A8: 815C0000  lwz r10, 0(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE47AC: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DE47B0: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82DE47B4: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82DE47B8: 40980020  bge cr6, 0x82de47d8
	if !ctx.cr[6].lt {
	pc = 0x82DE47D8; continue 'dispatch;
	}
	// 82DE47BC: 3D208202  lis r9, -0x7dfe
	ctx.r[9].s64 = -2113798144;
	// 82DE47C0: 39296468  addi r9, r9, 0x6468
	ctx.r[9].s64 = ctx.r[9].s64 + 25704;
	// 82DE47C4: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82DE47C8: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82DE47CC: 390B000C  addi r8, r11, 0xc
	ctx.r[8].s64 = ctx.r[11].s64 + 12;
	// 82DE47D0: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82DE47D4: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82DE47D8: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 82DE47DC: 4BEC4C70  b 0x82ca944c
	sub_82CA9420(ctx, base);
	return;
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
    // ---- function 0x82DE4E50 size=72
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
	pc = 0x82DE4E6C; continue 'dispatch;
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
	pc = 0x82DE4E6C; continue 'dispatch;
            }
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
    //   block [0x82DE4F10..0x82DE4F84)
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
            }
            0x82DE4F84 => {
    //   block [0x82DE4F84..0x82DE4F90)
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
    //   block [0x82DE4F90..0x82DE4FAC)
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
	pc = 0x82DE4FAC; continue 'dispatch;
            }
            0x82DE4FAC => {
    //   block [0x82DE4FAC..0x82DE4FC8)
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
		crate::recompiler::externs::call(ctx, base, 0x82DE4FE4);
		return;
	}
	pc = 0x82DE4FC8; continue 'dispatch;
            }
            0x82DE4FC8 => {
    //   block [0x82DE4FC8..0x82DE4FD8)
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
	pc = 0x82DE4FD8; continue 'dispatch;
            }
            0x82DE4FD8 => {
    //   block [0x82DE4FD8..0x82DE4FE4)
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
    //   block [0x82DE51B0..0x82DE5320)
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
	pc = 0x82DE5320; continue 'dispatch;
            }
            0x82DE5320 => {
    //   block [0x82DE5320..0x82DE5328)
	// 82DE5320: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DE5324: 4BEC4118  b 0x82ca943c
	sub_82CA9420(ctx, base);
	return;
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
    //   block [0x82DE65E0..0x82DE6680)
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
	pc = 0x82DE6680; continue 'dispatch;
            }
            0x82DE6680 => {
    //   block [0x82DE6680..0x82DE6688)
	// 82DE6680: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 82DE6684: 4BEC2DC8  b 0x82ca944c
	sub_82CA9420(ctx, base);
	return;
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
    //   block [0x82DE6688..0x82DE66C0)
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
	pc = 0x82DE66C0; continue 'dispatch;
            }
            0x82DE66C0 => {
    //   block [0x82DE66C0..0x82DE66D8)
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
	pc = 0x82DE66D8; continue 'dispatch;
            }
            0x82DE66D8 => {
    //   block [0x82DE66D8..0x82DE66EC)
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
            }
            0x82DE66EC => {
    //   block [0x82DE66EC..0x82DE671C)
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
            }
            0x82DE671C => {
    //   block [0x82DE671C..0x82DE6724)
	// 82DE671C: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82DE6720: 48002901  bl 0x82de9020
	ctx.lr = 0x82DE6724;
	sub_82DE9020(ctx, base);
	pc = 0x82DE6724; continue 'dispatch;
            }
            0x82DE6724 => {
    //   block [0x82DE6724..0x82DE673C)
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
    // ---- function 0x82DE6C30 size=272
    let mut pc: u32 = 0x82DE6C30;
    'dispatch: loop {
        match pc {
            0x82DE6C30 => {
    //   block [0x82DE6C30..0x82DE6D40)
	// 82DE6C30: 81240030  lwz r9, 0x30(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(48 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82DE6D40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82DE6D40 size=272
    let mut pc: u32 = 0x82DE6D40;
    'dispatch: loop {
        match pc {
            0x82DE6D40 => {
    //   block [0x82DE6D40..0x82DE6E50)
	// 82DE6D40: 81240030  lwz r9, 0x30(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(48 as u32) ) } as u64;
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
    //   block [0x82DE7160..0x82DE7298)
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
	pc = 0x82DE7298; continue 'dispatch;
            }
            0x82DE7298 => {
    //   block [0x82DE7298..0x82DE72B4)
	// 82DE7298: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82DE729C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82DE72A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DE72A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DE72A8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82DE72AC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82DE72B0: 4E800020  blr
	return;
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
    //   block [0x82DE7790..0x82DE7800)
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
            }
            0x82DE7800 => {
    //   block [0x82DE7800..0x82DE782C)
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
    //   block [0x82DE7830..0x82DE7B78)
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
	pc = 0x82DE7B78; continue 'dispatch;
            }
            0x82DE7B78 => {
    //   block [0x82DE7B78..0x82DE7B9C)
	// 82DE7B78: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE7B7C: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 82DE7B80: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82DE7B84: 390B0010  addi r8, r11, 0x10
	ctx.r[8].s64 = ctx.r[11].s64 + 16;
	// 82DE7B88: 994B0003  stb r10, 3(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(3 as u32), ctx.r[10].u8 ) };
	// 82DE7B8C: 992B0004  stb r9, 4(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u8 ) };
	// 82DE7B90: 911B0000  stw r8, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82DE7B94: 382100F0  addi r1, r1, 0xf0
	ctx.r[1].s64 = ctx.r[1].s64 + 240;
	// 82DE7B98: 4BEC18AC  b 0x82ca9444
	sub_82CA9420(ctx, base);
	return;
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
    //   block [0x82DE8050..0x82DE80CC)
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
	pc = 0x82DE80CC; continue 'dispatch;
            }
            0x82DE80CC => {
    //   block [0x82DE80CC..0x82DE80F8)
	// 82DE80CC: 81670000  lwz r11, 0(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE80D0: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 82DE80D4: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82DE80D8: 390B0010  addi r8, r11, 0x10
	ctx.r[8].s64 = ctx.r[11].s64 + 16;
	// 82DE80DC: 994B0003  stb r10, 3(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(3 as u32), ctx.r[10].u8 ) };
	// 82DE80E0: 992B0004  stb r9, 4(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u8 ) };
	// 82DE80E4: 91070000  stw r8, 0(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82DE80E8: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82DE80EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82DE80F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82DE80F4: 4E800020  blr
	return;
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


pub fn sub_82DE8750(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82DE8750 size=608
    let mut pc: u32 = 0x82DE8750;
    'dispatch: loop {
        match pc {
            0x82DE8750 => {
    //   block [0x82DE8750..0x82DE878C)
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
	pc = 0x82DE878C; continue 'dispatch;
            }
            0x82DE878C => {
    //   block [0x82DE878C..0x82DE87C8)
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
	pc = 0x82DE87C8; continue 'dispatch;
            }
            0x82DE87C8 => {
    //   block [0x82DE87C8..0x82DE87DC)
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
            }
            0x82DE87DC => {
    //   block [0x82DE87DC..0x82DE8818)
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
	pc = 0x82DE8818; continue 'dispatch;
            }
            0x82DE8818 => {
    //   block [0x82DE8818..0x82DE884C)
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
	pc = 0x82DE884C; continue 'dispatch;
            }
            0x82DE884C => {
    //   block [0x82DE884C..0x82DE8884)
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
	pc = 0x82DE8884; continue 'dispatch;
            }
            0x82DE8884 => {
    //   block [0x82DE8884..0x82DE88BC)
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
	pc = 0x82DE88BC; continue 'dispatch;
            }
            0x82DE88BC => {
    //   block [0x82DE88BC..0x82DE88F4)
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
	pc = 0x82DE88F4; continue 'dispatch;
            }
            0x82DE88F4 => {
    //   block [0x82DE88F4..0x82DE8908)
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
	pc = 0x82DE8908; continue 'dispatch;
            }
            0x82DE8908 => {
    //   block [0x82DE8908..0x82DE8924)
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
	pc = 0x82DE8924; continue 'dispatch;
            }
            0x82DE8924 => {
    //   block [0x82DE8924..0x82DE8958)
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
	pc = 0x82DE8958; continue 'dispatch;
            }
            0x82DE8958 => {
    //   block [0x82DE8958..0x82DE896C)
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
	pc = 0x82DE896C; continue 'dispatch;
            }
            0x82DE896C => {
    //   block [0x82DE896C..0x82DE8990)
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
            }
            0x82DE8990 => {
    //   block [0x82DE8990..0x82DE8998)
	// 82DE8990: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82DE8994: 4800068D  bl 0x82de9020
	ctx.lr = 0x82DE8998;
	sub_82DE9020(ctx, base);
	pc = 0x82DE8998; continue 'dispatch;
            }
            0x82DE8998 => {
    //   block [0x82DE8998..0x82DE89B0)
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
    //   block [0x82DE89B0..0x82DE89E0)
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
	pc = 0x82DE89E0; continue 'dispatch;
            }
            0x82DE89E0 => {
    //   block [0x82DE89E0..0x82DE8A7C)
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
	pc = 0x82DE8A7C; continue 'dispatch;
            }
            0x82DE8A7C => {
    //   block [0x82DE8A7C..0x82DE8A9C)
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
	pc = 0x82DE8A9C; continue 'dispatch;
            }
            0x82DE8A9C => {
    //   block [0x82DE8A9C..0x82DE8B98)
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
    //   block [0x82DE8B98..0x82DE8BD0)
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
	pc = 0x82DE8BD0; continue 'dispatch;
            }
            0x82DE8BD0 => {
    //   block [0x82DE8BD0..0x82DE8CEC)
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
	pc = 0x82DE8CEC; continue 'dispatch;
            }
            0x82DE8CEC => {
    //   block [0x82DE8CEC..0x82DE8E3C)
	// 82DE8CEC: 81640034  lwz r11, 0x34(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(52 as u32) ) } as u64;
	// 82DE8CF0: 38E10060  addi r7, r1, 0x60
	ctx.r[7].s64 = ctx.r[1].s64 + 96;
	// 82DE8CF4: 8B6B0000  lbz r27, 0(r11)
	ctx.r[27].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
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
    //   block [0x82DE8EB0..0x82DE8F14)
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
	pc = 0x82DE8F14; continue 'dispatch;
            }
            0x82DE8F14 => {
    //   block [0x82DE8F14..0x82DE8FFC)
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
	pc = 0x82DE8FFC; continue 'dispatch;
            }
            0x82DE8FFC => {
    //   block [0x82DE8FFC..0x82DE9020)
	// 82DE8FFC: 81790000  lwz r11, 0(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DE9000: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 82DE9004: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82DE9008: 390B0010  addi r8, r11, 0x10
	ctx.r[8].s64 = ctx.r[11].s64 + 16;
	// 82DE900C: 994B0003  stb r10, 3(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(3 as u32), ctx.r[10].u8 ) };
	// 82DE9010: 992B0004  stb r9, 4(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u8 ) };
	// 82DE9014: 91190000  stw r8, 0(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82DE9018: 382100F0  addi r1, r1, 0xf0
	ctx.r[1].s64 = ctx.r[1].s64 + 240;
	// 82DE901C: 4BEC0428  b 0x82ca9444
	sub_82CA9420(ctx, base);
	return;
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
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82DEB2E0 size=488
    let mut pc: u32 = 0x82DEB2E0;
    'dispatch: loop {
        match pc {
            0x82DEB2E0 => {
    //   block [0x82DEB2E0..0x82DEB31C)
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
    //   block [0x82DEB31C..0x82DEB364)
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
	pc = 0x82DEB364; continue 'dispatch;
            }
            0x82DEB364 => {
    //   block [0x82DEB364..0x82DEB36C)
	// 82DEB364: EDA0582A  fadds f13, f0, f11
	ctx.f[13].f64 = ((ctx.f[0].f64 + ctx.f[11].f64) as f32) as f64;
	// 82DEB368: 48000008  b 0x82deb370
	pc = 0x82DEB370; continue 'dispatch;
            }
            0x82DEB36C => {
    //   block [0x82DEB36C..0x82DEB370)
	// 82DEB36C: FDA06090  fmr f13, f12
	ctx.f[13].f64 = ctx.f[12].f64;
	pc = 0x82DEB370; continue 'dispatch;
            }
            0x82DEB370 => {
    //   block [0x82DEB370..0x82DEB394)
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
            }
            0x82DEB394 => {
    //   block [0x82DEB394..0x82DEB3A0)
	// 82DEB394: FF0D6000  fcmpu cr6, f13, f12
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[12].f64);
	// 82DEB398: 40990008  ble cr6, 0x82deb3a0
	if !ctx.cr[6].gt {
	pc = 0x82DEB3A0; continue 'dispatch;
	}
	// 82DEB39C: FDA06090  fmr f13, f12
	ctx.f[13].f64 = ctx.f[12].f64;
	pc = 0x82DEB3A0; continue 'dispatch;
            }
            0x82DEB3A0 => {
    //   block [0x82DEB3A0..0x82DEB3C8)
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
    //   block [0x82DEB3C8..0x82DEB3FC)
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
            }
            0x82DEB3FC => {
    //   block [0x82DEB3FC..0x82DEB410)
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
    //   block [0x82DEB410..0x82DEB45C)
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
            }
            0x82DEB45C => {
    //   block [0x82DEB45C..0x82DEB46C)
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
            }
            0x82DEB46C => {
    //   block [0x82DEB46C..0x82DEB470)
	// 82DEB46C: FD400090  fmr f10, f0
	ctx.f[10].f64 = ctx.f[0].f64;
	pc = 0x82DEB470; continue 'dispatch;
            }
            0x82DEB470 => {
    //   block [0x82DEB470..0x82DEB48C)
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
            }
            0x82DEB48C => {
    //   block [0x82DEB48C..0x82DEB498)
	// 82DEB48C: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 82DEB490: 41990008  bgt cr6, 0x82deb498
	if ctx.cr[6].gt {
	pc = 0x82DEB498; continue 'dispatch;
	}
	// 82DEB494: FDA00090  fmr f13, f0
	ctx.f[13].f64 = ctx.f[0].f64;
	pc = 0x82DEB498; continue 'dispatch;
            }
            0x82DEB498 => {
    //   block [0x82DEB498..0x82DEB4A0)
	// 82DEB498: C183000C  lfs f12, 0xc(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82DEB49C: C0030010  lfs f0, 0x10(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	pc = 0x82DEB4A0; continue 'dispatch;
            }
            0x82DEB4A0 => {
    //   block [0x82DEB4A0..0x82DEB4BC)
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
    //   block [0x82DEBF50..0x82DEC080)
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
	pc = 0x82DEC080; continue 'dispatch;
            }
            0x82DEC080 => {
    //   block [0x82DEC080..0x82DEC084)
	// 82DEC080: 4BEBD3D0  b 0x82ca9450
	sub_82CA9420(ctx, base);
	return;
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
    //   block [0x82DEC088..0x82DEC0E4)
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
	ctx.r[6].s32 = ((ctx.r[5].s32 as i64 * 68 as i64) as i32);
	ctx.r[6].s64 = ctx.r[6].s32 as i64;
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
	pc = 0x82DEC0E4; continue 'dispatch;
            }
            0x82DEC0E4 => {
    //   block [0x82DEC0E4..0x82DEC0FC)
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
	pc = 0x82DEC0FC; continue 'dispatch;
            }
            0x82DEC0FC => {
    //   block [0x82DEC0FC..0x82DEC250)
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
	pc = 0x82DEC250; continue 'dispatch;
            }
            0x82DEC250 => {
    //   block [0x82DEC250..0x82DEC264)
	// 82DEC250: 81790000  lwz r11, 0(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DEC254: 815A0000  lwz r10, 0(r26)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DEC258: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82DEC25C: 917A0000  stw r11, 0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82DEC260: 4BEBD1E8  b 0x82ca9448
	sub_82CA9420(ctx, base);
	return;
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
    //   block [0x82DEC338..0x82DEC580)
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
	ctx.r[10].s32 = ((ctx.r[11].s32 as i64 * 60 as i64) as i32);
	ctx.r[10].s64 = ctx.r[10].s32 as i64;
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
	pc = 0x82DEC580; continue 'dispatch;
            }
            0x82DEC580 => {
    //   block [0x82DEC580..0x82DEC6B0)
	// 82DEC580: 395FFFFF  addi r10, r31, -1
	ctx.r[10].s64 = ctx.r[31].s64 + -1;
	// 82DEC584: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82DEC588: 41980124  blt cr6, 0x82dec6ac
	if ctx.cr[6].lt {
	pc = 0x82DEC6AC; continue 'dispatch;
	}
	// 82DEC58C: 81210034  lwz r9, 0x34(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(52 as u32) ) } as u64;
	// 82DEC590: 5548083C  slwi r8, r10, 1
	ctx.r[8].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82DEC594: 554B103A  slwi r11, r10, 2
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82DEC598: 7FB14850  subf r29, r17, r9
	ctx.r[29].s64 = ctx.r[9].s64 - ctx.r[17].s64;
	// 82DEC59C: 7D0A4214  add r8, r10, r8
	ctx.r[8].u64 = ctx.r[10].u64 + ctx.r[8].u64;
	// 82DEC5A0: 5549083C  slwi r9, r10, 1
	ctx.r[9].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82DEC5A4: 5508103A  slwi r8, r8, 2
	ctx.r[8].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82DEC5A8: 7D2A4A14  add r9, r10, r9
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82DEC5AC: 7D082214  add r8, r8, r4
	ctx.r[8].u64 = ctx.r[8].u64 + ctx.r[4].u64;
	// 82DEC5B0: 55292036  slwi r9, r9, 4
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(4);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82DEC5B4: 7FD19050  subf r30, r17, r18
	ctx.r[30].s64 = ctx.r[18].s64 - ctx.r[17].s64;
	// 82DEC5B8: 7D6B8A14  add r11, r11, r17
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[17].u64;
	// 82DEC5BC: 7D293A14  add r9, r9, r7
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[7].u64;
	// 82DEC5C0: 38C80008  addi r6, r8, 8
	ctx.r[6].s64 = ctx.r[8].s64 + 8;
	// 82DEC5C4: 810B0000  lwz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DEC5C8: 38890020  addi r4, r9, 0x20
	ctx.r[4].s64 = ctx.r[9].s64 + 32;
	// 82DEC5CC: 3861FF18  addi r3, r1, -0xe8
	ctx.r[3].s64 = ctx.r[1].s64 + -232;
	// 82DEC5D0: C1A60000  lfs f13, 0(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82DEC5D4: 7D082A14  add r8, r8, r5
	ctx.r[8].u64 = ctx.r[8].u64 + ctx.r[5].u64;
	// 82DEC5D8: 7D9E5C2E  lfsx f12, r30, r11
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82DEC5DC: EC0D603C  fnmsubs f0, f13, f0, f12
	ctx.f[0].f64 = -(((ctx.f[13].f64 * ctx.f[0].f64 - ctx.f[12].f64) as f32) as f64);
	// 82DEC5E0: D001FF18  stfs f0, -0xe8(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-232 as u32), tmp.u32 ) };
	// 82DEC5E4: 80EB0004  lwz r7, 4(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
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
	ctx.r[11].s32 = ((ctx.r[11].s32 as i64 * 304 as i64) as i32);
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
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
    // ---- function 0x82DECE20 size=244
    //   switch @ 0x82DECE7C: r10 with 4 label(s)
    //       case  0 → 0x82DECEB8
    //       case  1 → 0x82DECE90
    //       case  2 → 0x82DECE90
    //       case  3 → 0x82DECF10
    let mut pc: u32 = 0x82DECE20;
    'dispatch: loop {
        match pc {
            0x82DECE20 => {
    //   block [0x82DECE20..0x82DECE4C)
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
	pc = 0x82DECE4C; continue 'dispatch;
            }
            0x82DECE4C => {
    //   block [0x82DECE4C..0x82DECE90)
	// 82DECE4C: 7F0B3840  cmplw cr6, r11, r7
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[7].u32, &mut ctx.xer);
	// 82DECE50: 4C980020  bgelr cr6
	if !ctx.cr[6].lt { return; }
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
	ctx.r[8].s32 = ((ctx.r[8].s32 as i64 * 28 as i64) as i32);
	ctx.r[8].s64 = ctx.r[8].s32 as i64;
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
    //   block [0x82DED5E0..0x82DED694)
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
	ctx.r[9].s32 = ((ctx.r[11].s32 as i64 * 1264 as i64) as i32);
	ctx.r[9].s64 = ctx.r[9].s32 as i64;
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
	ctx.r[8].s32 = ((ctx.r[11].s32 as i64 * 240 as i64) as i32);
	ctx.r[8].s64 = ctx.r[8].s32 as i64;
	// 82DED624: 39290040  addi r9, r9, 0x40
	ctx.r[9].s64 = ctx.r[9].s64 + 64;
	// 82DED628: 7D083214  add r8, r8, r6
	ctx.r[8].u64 = ctx.r[8].u64 + ctx.r[6].u64;
	// 82DED62C: 1CEB04F4  mulli r7, r11, 0x4f4
	ctx.r[7].s32 = ((ctx.r[11].s32 as i64 * 1268 as i64) as i32);
	ctx.r[7].s64 = ctx.r[7].s32 as i64;
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
	ctx.r[5].s32 = ((ctx.r[11].s32 as i64 * 1200 as i64) as i32);
	ctx.r[5].s64 = ctx.r[5].s32 as i64;
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
	ctx.r[3].s32 = ((ctx.r[11].s32 as i64 * 1232 as i64) as i32);
	ctx.r[3].s64 = ctx.r[3].s32 as i64;
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
	pc = 0x82DED694; continue 'dispatch;
            }
            0x82DED694 => {
    //   block [0x82DED694..0x82DED69C)
	// 82DED694: 57E93032  slwi r9, r31, 6
	ctx.r[9].u32 = ctx.r[31].u32.wrapping_shl(6);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82DED698: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	pc = 0x82DED69C; continue 'dispatch;
            }
            0x82DED69C => {
    //   block [0x82DED69C..0x82DED6DC)
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
	pc = 0x82DED6DC; continue 'dispatch;
            }
            0x82DED6DC => {
    //   block [0x82DED6DC..0x82DED704)
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
	pc = 0x82DED704; continue 'dispatch;
            }
            0x82DED704 => {
    //   block [0x82DED704..0x82DED710)
	// 82DED704: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82DED708: 57BC3032  slwi r28, r29, 6
	ctx.r[28].u32 = ctx.r[29].u32.wrapping_shl(6);
	ctx.r[28].u64 = ctx.r[28].u32 as u64;
	// 82DED70C: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	pc = 0x82DED710; continue 'dispatch;
            }
            0x82DED710 => {
    //   block [0x82DED710..0x82DED740)
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
            }
            0x82DED740 => {
    //   block [0x82DED740..0x82DED758)
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
	pc = 0x82DED758; continue 'dispatch;
            }
            0x82DED758 => {
    //   block [0x82DED758..0x82DED77C)
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
	pc = 0x82DED77C; continue 'dispatch;
            }
            0x82DED77C => {
    //   block [0x82DED77C..0x82DED7BC)
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
	pc = 0x82DED7BC; continue 'dispatch;
            }
            0x82DED7BC => {
    //   block [0x82DED7BC..0x82DED81C)
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
	pc = 0x82DED81C; continue 'dispatch;
            }
            0x82DED81C => {
    //   block [0x82DED81C..0x82DED858)
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
	pc = 0x82DED858; continue 'dispatch;
            }
            0x82DED858 => {
    //   block [0x82DED858..0x82DED8B8)
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
	pc = 0x82DED8B8; continue 'dispatch;
            }
            0x82DED8B8 => {
    //   block [0x82DED8B8..0x82DEDC80)
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
    //   block [0x82DEF7F0..0x82DEF854)
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
            }
            0x82DEF854 => {
    //   block [0x82DEF854..0x82DEF8DC)
	// 82DEF854: 1001038C  vspltisw v0, 1
	for i in 0..4 {
		ctx.v[0].u32[i] = 1;
	}
	// 82DEF858: 39370010  addi r9, r23, 0x10
	ctx.r[9].s64 = ctx.r[23].s64 + 16;
	pc = 0x82DEF8DC; continue 'dispatch;
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
	ctx.r[8].s32 = ((ctx.r[8].s32 as i64 * 28 as i64) as i32);
	ctx.r[8].s64 = ctx.r[8].s32 as i64;
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
    //   block [0x82DF1868..0x82DF1888)
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
	pc = 0x82DF1888; continue 'dispatch;
            }
            0x82DF1888 => {
    //   block [0x82DF1888..0x82DF18A0)
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
    //   block [0x82DF3778..0x82DF37D4)
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
	pc = 0x82DF37D4; continue 'dispatch;
            }
            0x82DF37D4 => {
    //   block [0x82DF37D4..0x82DF37EC)
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
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82DF4358 size=164
    let mut pc: u32 = 0x82DF4358;
    'dispatch: loop {
        match pc {
            0x82DF4358 => {
    //   block [0x82DF4358..0x82DF4388)
	// 82DF4358: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF435C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DF4360: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
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
	pc = 0x82DF4388; continue 'dispatch;
            }
            0x82DF4388 => {
    //   block [0x82DF4388..0x82DF43E0)
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
	pc = 0x82DF43E0; continue 'dispatch;
            }
            0x82DF43E0 => {
    //   block [0x82DF43E0..0x82DF43FC)
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
    // ---- function 0x82DF4400 size=92
    let mut pc: u32 = 0x82DF4400;
    'dispatch: loop {
        match pc {
            0x82DF4400 => {
    //   block [0x82DF4400..0x82DF4448)
	// 82DF4400: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF4404: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82DF4408: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
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
	pc = 0x82DF4448; continue 'dispatch;
            }
            0x82DF4448 => {
    //   block [0x82DF4448..0x82DF445C)
	// 82DF4448: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 82DF444C: 396B0010  addi r11, r11, 0x10
	ctx.r[11].s64 = ctx.r[11].s64 + 16;
	// 82DF4450: 2F0A0006  cmpwi cr6, r10, 6
	ctx.cr[6].compare_i32(ctx.r[10].s32, 6, &mut ctx.xer);
	// 82DF4454: 4198FFC0  blt cr6, 0x82df4414
	if ctx.cr[6].lt {
	pc = 0x82DF4414; continue 'dispatch;
	}
	// 82DF4458: 4E800020  blr
	return;
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
		crate::recompiler::externs::call(ctx, base, 0x82DF44AC);
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
    //   block [0x82DF48D8..0x82DF4BB0)
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
	pc = 0x82DF4BB0; continue 'dispatch;
            }
            0x82DF4BB0 => {
    //   block [0x82DF4BB0..0x82DF4BB4)
	// 82DF4BB0: 4BEB4870  b 0x82ca9420
	sub_82CA9420(ctx, base);
	return;
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
    //   block [0x82DF5B40..0x82DF5B64)
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
	pc = 0x82DF5B64; continue 'dispatch;
            }
            0x82DF5B64 => {
    //   block [0x82DF5B64..0x82DF5C24)
	// 82DF5B64: 419A00C0  beq cr6, 0x82df5c24
	if ctx.cr[6].eq {
	pc = 0x82DF5C24; continue 'dispatch;
	}
	// 82DF5B68: 1D6A03C0  mulli r11, r10, 0x3c0
	ctx.r[11].s32 = ((ctx.r[10].s32 as i64 * 960 as i64) as i32);
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
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
	pc = 0x82DF5C24; continue 'dispatch;
            }
            0x82DF5C24 => {
    //   block [0x82DF5C24..0x82DF5D20)
	// 82DF5C24: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
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
    //   block [0x82DF5D20..0x82DF5D98)
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
	ctx.r[11].s32 = ((ctx.r[10].s32 as i64 * 1332 as i64) as i32);
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 82DF5D7C: 38EB0053  addi r7, r11, 0x53
	ctx.r[7].s64 = ctx.r[11].s64 + 83;
	// 82DF5D80: 1D6A04F0  mulli r11, r10, 0x4f0
	ctx.r[11].s32 = ((ctx.r[10].s32 as i64 * 1264 as i64) as i32);
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
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
	pc = 0x82DF5D98; continue 'dispatch;
            }
            0x82DF5D98 => {
    //   block [0x82DF5D98..0x82DF5DB4)
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
	pc = 0x82DF5DB4; continue 'dispatch;
            }
            0x82DF5DB4 => {
    //   block [0x82DF5DB4..0x82DF5DD0)
	// 82DF5DB4: A17F0004  lhz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82DF5DB8: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82DF5DBC: 1D6B04F4  mulli r11, r11, 0x4f4
	ctx.r[11].s32 = ((ctx.r[11].s32 as i64 * 1268 as i64) as i32);
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
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
	pc = 0x82DF5DD0; continue 'dispatch;
            }
            0x82DF5DD0 => {
    //   block [0x82DF5DD0..0x82DF5DF4)
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
	pc = 0x82DF5DF4; continue 'dispatch;
            }
            0x82DF5DF4 => {
    //   block [0x82DF5DF4..0x82DF5E70)
	// 82DF5DF4: 1D1D03C0  mulli r8, r29, 0x3c0
	ctx.r[8].s32 = ((ctx.r[29].s32 as i64 * 960 as i64) as i32);
	ctx.r[8].s64 = ctx.r[8].s32 as i64;
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
	ctx.r[10].s32 = ((ctx.r[11].s32 as i64 * 240 as i64) as i32);
	ctx.r[10].s64 = ctx.r[10].s32 as i64;
	// 82DF5E24: 80BE0024  lwz r5, 0x24(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 82DF5E28: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82DF5E2C: 7CFD4214  add r7, r29, r8
	ctx.r[7].u64 = ctx.r[29].u64 + ctx.r[8].u64;
	// 82DF5E30: 1D0B04F0  mulli r8, r11, 0x4f0
	ctx.r[8].s32 = ((ctx.r[11].s32 as i64 * 1264 as i64) as i32);
	ctx.r[8].s64 = ctx.r[8].s32 as i64;
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


