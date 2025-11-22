pub fn sub_825B18C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825B18C8 size=108
    let mut pc: u32 = 0x825B18C8;
    'dispatch: loop {
        match pc {
            0x825B18C8 => {
    //   block [0x825B18C8..0x825B1934)
	// 825B18C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B18CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825B18D0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825B18D4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825B18D8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B18DC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825B18E0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825B18E4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B18E8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825B18EC: 409A0018  bne cr6, 0x825b1904
	if !ctx.cr[6].eq {
	pc = 0x825B1904; continue 'dispatch;
	}
	// 825B18F0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825B18F4: 4BD21CE5  bl 0x822d35d8
	ctx.lr = 0x825B18F8;
	sub_822D35D8(ctx, base);
	// 825B18F8: 4BD0E709  bl 0x822c0000
	ctx.lr = 0x825B18FC;
	sub_822C0000(ctx, base);
	// 825B18FC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825B1900: 4BD20F11  bl 0x822d2810
	ctx.lr = 0x825B1904;
	sub_822D2810(ctx, base);
	// 825B1904: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B1908: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 825B190C: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 825B1910: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 825B1914: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825B1918: 4E800421  bctrl
	ctx.lr = 0x825B191C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825B191C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 825B1920: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825B1924: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825B1928: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825B192C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825B1930: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B1938(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825B1938 size=280
    let mut pc: u32 = 0x825B1938;
    'dispatch: loop {
        match pc {
            0x825B1938 => {
    //   block [0x825B1938..0x825B1A50)
	// 825B1938: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B193C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825B1940: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825B1944: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B1948: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825B194C: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 825B1950: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825B1954: 4098000C  bge cr6, 0x825b1960
	if !ctx.cr[6].lt {
	pc = 0x825B1960; continue 'dispatch;
	}
	// 825B1958: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 825B195C: 480000E0  b 0x825b1a3c
	pc = 0x825B1A3C; continue 'dispatch;
	// 825B1960: 38800080  li r4, 0x80
	ctx.r[4].s64 = 128;
	// 825B1964: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B1968: 4BFE9F21  bl 0x8259b888
	ctx.lr = 0x825B196C;
	sub_8259B888(ctx, base);
	// 825B196C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825B1970: 41820040  beq 0x825b19b0
	if ctx.cr[0].eq {
	pc = 0x825B19B0; continue 'dispatch;
	}
	// 825B1974: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 825B1978: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 825B197C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 825B1980: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825B1984: 915F0010  stw r10, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 825B1988: 409A000C  bne cr6, 0x825b1994
	if !ctx.cr[6].eq {
	pc = 0x825B1994; continue 'dispatch;
	}
	// 825B198C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825B1990: 48000010  b 0x825b19a0
	pc = 0x825B19A0; continue 'dispatch;
	// 825B1994: 813F0008  lwz r9, 8(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 825B1998: 7D6B4850  subf r11, r11, r9
	ctx.r[11].s64 = ctx.r[9].s64 - ctx.r[11].s64;
	// 825B199C: 7D6B1E70  srawi r11, r11, 3
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 3) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 3) as i64;
	// 825B19A0: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 825B19A4: 41980050  blt cr6, 0x825b19f4
	if ctx.cr[6].lt {
	pc = 0x825B19F4; continue 'dispatch;
	}
	// 825B19A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825B19AC: 48000044  b 0x825b19f0
	pc = 0x825B19F0; continue 'dispatch;
	// 825B19B0: 38800040  li r4, 0x40
	ctx.r[4].s64 = 64;
	// 825B19B4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B19B8: 4BFE9ED1  bl 0x8259b888
	ctx.lr = 0x825B19BC;
	sub_8259B888(ctx, base);
	// 825B19BC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825B19C0: 41820034  beq 0x825b19f4
	if ctx.cr[0].eq {
	pc = 0x825B19F4; continue 'dispatch;
	}
	// 825B19C4: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 825B19C8: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825B19CC: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 825B19D0: 40800024  bge 0x825b19f4
	if !ctx.cr[0].lt {
	pc = 0x825B19F4; continue 'dispatch;
	}
	// 825B19D4: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 825B19D8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825B19DC: 419A0010  beq cr6, 0x825b19ec
	if ctx.cr[6].eq {
	pc = 0x825B19EC; continue 'dispatch;
	}
	// 825B19E0: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 825B19E4: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 825B19E8: 7D6B1E70  srawi r11, r11, 3
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 3) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 3) as i64;
	// 825B19EC: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 825B19F0: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 825B19F4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 825B19F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B19FC: 4BFE9E8D  bl 0x8259b888
	ctx.lr = 0x825B1A00;
	sub_8259B888(ctx, base);
	// 825B1A00: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825B1A04: 41820020  beq 0x825b1a24
	if ctx.cr[0].eq {
	pc = 0x825B1A24; continue 'dispatch;
	}
	// 825B1A08: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 825B1A0C: 387F0018  addi r3, r31, 0x18
	ctx.r[3].s64 = ctx.r[31].s64 + 24;
	// 825B1A10: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 825B1A14: 554A1838  slwi r10, r10, 3
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 825B1A18: 7C8B5214  add r4, r11, r10
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 825B1A1C: 4BFFFEAD  bl 0x825b18c8
	ctx.lr = 0x825B1A20;
	sub_825B18C8(ctx, base);
	// 825B1A20: 4BFFFF38  b 0x825b1958
	pc = 0x825B1958; continue 'dispatch;
	// 825B1A24: 3880001A  li r4, 0x1a
	ctx.r[4].s64 = 26;
	// 825B1A28: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B1A2C: 4BFE9E5D  bl 0x8259b888
	ctx.lr = 0x825B1A30;
	sub_8259B888(ctx, base);
	// 825B1A30: 546B063E  clrlwi r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 825B1A34: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 825B1A38: 5563DFFE  rlwinm r3, r11, 0x1b, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 825B1A3C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825B1A40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825B1A44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825B1A48: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825B1A4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B1A50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825B1A50 size=36
    let mut pc: u32 = 0x825B1A50;
    'dispatch: loop {
        match pc {
            0x825B1A50 => {
    //   block [0x825B1A50..0x825B1A74)
	// 825B1A50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825B1A54: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 825B1A58: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 825B1A5C: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 825B1A60: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 825B1A64: 91430010  stw r10, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 825B1A68: 91630018  stw r11, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 825B1A6C: 91630038  stw r11, 0x38(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 825B1A70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B1A78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x825B1A78 size=104
    let mut pc: u32 = 0x825B1A78;
    'dispatch: loop {
        match pc {
            0x825B1A78 => {
    //   block [0x825B1A78..0x825B1AE0)
	// 825B1A78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B1A7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825B1A80: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825B1A84: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B1A88: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825B1A8C: 48595715  bl 0x82b471a0
	ctx.lr = 0x825B1A90;
	sub_82B471A0(ctx, base);
	// 825B1A90: 3CE08200  lis r7, -0x7e00
	ctx.r[7].s64 = -2113929216;
	// 825B1A94: 397F0038  addi r11, r31, 0x38
	ctx.r[11].s64 = ctx.r[31].s64 + 56;
	// 825B1A98: 393F0040  addi r9, r31, 0x40
	ctx.r[9].s64 = ctx.r[31].s64 + 64;
	// 825B1A9C: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 825B1AA0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825B1AA4: C00708A4  lfs f0, 0x8a4(r7)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825B1AA8: 990BFFFC  stb r8, -4(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(-4 as u32), ctx.r[8].u8 ) };
	// 825B1AAC: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 825B1AB0: 990B0000  stb r8, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u8 ) };
	// 825B1AB4: 990B0004  stb r8, 4(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u8 ) };
	// 825B1AB8: D0090000  stfs f0, 0(r9)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 825B1ABC: 39290004  addi r9, r9, 4
	ctx.r[9].s64 = ctx.r[9].s64 + 4;
	// 825B1AC0: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 825B1AC4: 4082FFE4  bne 0x825b1aa8
	if !ctx.cr[0].eq {
	pc = 0x825B1AA8; continue 'dispatch;
	}
	// 825B1AC8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B1ACC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825B1AD0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825B1AD4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825B1AD8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825B1ADC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B1AE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825B1AE0 size=12
    let mut pc: u32 = 0x825B1AE0;
    'dispatch: loop {
        match pc {
            0x825B1AE0 => {
    //   block [0x825B1AE0..0x825B1AEC)
	// 825B1AE0: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 825B1AE4: 55637FFE  rlwinm r3, r11, 0xf, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0001FFFFu64;
	// 825B1AE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B1AF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825B1AF0 size=28
    let mut pc: u32 = 0x825B1AF0;
    'dispatch: loop {
        match pc {
            0x825B1AF0 => {
    //   block [0x825B1AF0..0x825B1B0C)
	// 825B1AF0: 7D641A14  add r11, r4, r3
	ctx.r[11].u64 = ctx.r[4].u64 + ctx.r[3].u64;
	// 825B1AF4: 894B0034  lbz r10, 0x34(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 825B1AF8: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 825B1AFC: 40820010  bne 0x825b1b0c
	if !ctx.cr[0].eq {
		sub_825B1B0C(ctx, base);
		return;
	}
	// 825B1B00: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825B1B04: 994B0034  stb r10, 0x34(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(52 as u32), ctx.r[10].u8 ) };
	// 825B1B08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B1B0C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x825B1B0C size=44
    let mut pc: u32 = 0x825B1B0C;
    'dispatch: loop {
        match pc {
            0x825B1B0C => {
    //   block [0x825B1B0C..0x825B1B38)
	// 825B1B0C: 39440010  addi r10, r4, 0x10
	ctx.r[10].s64 = ctx.r[4].s64 + 16;
	// 825B1B10: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 825B1B14: 7C0A1C2E  lfsx f0, r10, r3
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[3].u32)) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825B1B18: EC01002A  fadds f0, f1, f0
	ctx.f[0].f64 = ((ctx.f[1].f64 + ctx.f[0].f64) as f32) as f64;
	// 825B1B1C: 7C0A1D2E  stfsx f0, r10, r3
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[3].u32), tmp.u32) };
	// 825B1B20: 892B0038  lbz r9, 0x38(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) } as u64;
	// 825B1B24: 28090000  cmplwi r9, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 825B1B28: 3D208204  lis r9, -0x7dfc
	ctx.r[9].s64 = -2113667072;
	// 825B1B2C: 4082000C  bne 0x825b1b38
	if !ctx.cr[0].eq {
		sub_825B1B38(ctx, base);
		return;
	}
	// 825B1B30: C1A9B918  lfs f13, -0x46e8(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-18152 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825B1B34: 4800000C  b 0x825b1b40
	sub_825B1B38(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B1B38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x825B1B38 size=16
    let mut pc: u32 = 0x825B1B38;
    'dispatch: loop {
        match pc {
            0x825B1B38 => {
    //   block [0x825B1B38..0x825B1B48)
	// 825B1B38: 3929B918  addi r9, r9, -0x46e8
	ctx.r[9].s64 = ctx.r[9].s64 + -18152;
	// 825B1B3C: C1A90004  lfs f13, 4(r9)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825B1B40: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 825B1B44: 4D980020  bltlr cr6
	if ctx.cr[6].lt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B1B48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x825B1B48 size=28
    let mut pc: u32 = 0x825B1B48;
    'dispatch: loop {
        match pc {
            0x825B1B48 => {
    //   block [0x825B1B48..0x825B1B64)
	// 825B1B48: 3D008200  lis r8, -0x7e00
	ctx.r[8].s64 = -2113929216;
	// 825B1B4C: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 825B1B50: C00808A4  lfs f0, 0x8a4(r8)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825B1B54: 7C0A1D2E  stfsx f0, r10, r3
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[3].u32), tmp.u32) };
	// 825B1B58: 992B0038  stb r9, 0x38(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(56 as u32), ctx.r[9].u8 ) };
	// 825B1B5C: 992B003C  stb r9, 0x3c(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(60 as u32), ctx.r[9].u8 ) };
	// 825B1B60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B1B68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x825B1B68 size=284
    let mut pc: u32 = 0x825B1B68;
    'dispatch: loop {
        match pc {
            0x825B1B68 => {
    //   block [0x825B1B68..0x825B1C84)
	// 825B1B68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B1B6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825B1B70: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825B1B74: DBE1FFE8  stfd f31, -0x18(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.f[31].u64 ) };
	// 825B1B78: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B1B7C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825B1B80: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 825B1B84: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825B1B88: 48010AC1  bl 0x825c2648
	ctx.lr = 0x825B1B8C;
	sub_825C2648(ctx, base);
	// 825B1B8C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 825B1B90: 38A00034  li r5, 0x34
	ctx.r[5].s64 = 52;
	// 825B1B94: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B1B98: 48BF6979  bl 0x831a8510
	ctx.lr = 0x825B1B9C;
	sub_831A8510(ctx, base);
	// 825B1B9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825B1BA0: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 825B1BA4: 98FF003C  stb r7, 0x3c(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[7].u8 ) };
	// 825B1BA8: 397F003C  addi r11, r31, 0x3c
	ctx.r[11].s64 = ctx.r[31].s64 + 60;
	// 825B1BAC: 98FF003D  stb r7, 0x3d(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(61 as u32), ctx.r[7].u8 ) };
	// 825B1BB0: 98FF003E  stb r7, 0x3e(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(62 as u32), ctx.r[7].u8 ) };
	// 825B1BB4: 98FF003F  stb r7, 0x3f(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(63 as u32), ctx.r[7].u8 ) };
	// 825B1BB8: C18A08A4  lfs f12, 0x8a4(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2212 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 825B1BBC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B1BC0: 556B0631  rlwinm. r11, r11, 0, 0x18, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825B1BC4: 41820018  beq 0x825b1bdc
	if ctx.cr[0].eq {
	pc = 0x825B1BDC; continue 'dispatch;
	}
	// 825B1BC8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 825B1BCC: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 825B1BD0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B1BD4: 4BFFFF1D  bl 0x825b1af0
	ctx.lr = 0x825B1BD8;
	sub_825B1AF0(ctx, base);
	// 825B1BD8: 48000010  b 0x825b1be8
	pc = 0x825B1BE8; continue 'dispatch;
	// 825B1BDC: 98FF0035  stb r7, 0x35(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(53 as u32), ctx.r[7].u8 ) };
	// 825B1BE0: D19F0044  stfs f12, 0x44(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(68 as u32), tmp.u32 ) };
	// 825B1BE4: 98FF0039  stb r7, 0x39(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(57 as u32), ctx.r[7].u8 ) };
	// 825B1BE8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B1BEC: 556B0673  rlwinm. r11, r11, 0, 0x19, 0x19
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825B1BF0: 41820018  beq 0x825b1c08
	if ctx.cr[0].eq {
	pc = 0x825B1C08; continue 'dispatch;
	}
	// 825B1BF4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 825B1BF8: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 825B1BFC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B1C00: 4BFFFEF1  bl 0x825b1af0
	ctx.lr = 0x825B1C04;
	sub_825B1AF0(ctx, base);
	// 825B1C04: 48000010  b 0x825b1c14
	pc = 0x825B1C14; continue 'dispatch;
	// 825B1C08: 98FF0034  stb r7, 0x34(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[7].u8 ) };
	// 825B1C0C: D19F0040  stfs f12, 0x40(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), tmp.u32 ) };
	// 825B1C10: 98FF0038  stb r7, 0x38(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[7].u8 ) };
	// 825B1C14: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B1C18: 556B05AD  rlwinm. r11, r11, 0, 0x16, 0x16
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825B1C1C: 41820018  beq 0x825b1c34
	if ctx.cr[0].eq {
	pc = 0x825B1C34; continue 'dispatch;
	}
	// 825B1C20: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 825B1C24: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 825B1C28: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B1C2C: 4BFFFEC5  bl 0x825b1af0
	ctx.lr = 0x825B1C30;
	sub_825B1AF0(ctx, base);
	// 825B1C30: 48000010  b 0x825b1c40
	pc = 0x825B1C40; continue 'dispatch;
	// 825B1C34: 98FF0036  stb r7, 0x36(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(54 as u32), ctx.r[7].u8 ) };
	// 825B1C38: D19F0048  stfs f12, 0x48(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), tmp.u32 ) };
	// 825B1C3C: 98FF003A  stb r7, 0x3a(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(58 as u32), ctx.r[7].u8 ) };
	// 825B1C40: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B1C44: 556B05EF  rlwinm. r11, r11, 0, 0x17, 0x17
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825B1C48: 41820018  beq 0x825b1c60
	if ctx.cr[0].eq {
	pc = 0x825B1C60; continue 'dispatch;
	}
	// 825B1C4C: 38800003  li r4, 3
	ctx.r[4].s64 = 3;
	// 825B1C50: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 825B1C54: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B1C58: 4BFFFE99  bl 0x825b1af0
	ctx.lr = 0x825B1C5C;
	sub_825B1AF0(ctx, base);
	// 825B1C5C: 48000010  b 0x825b1c6c
	pc = 0x825B1C6C; continue 'dispatch;
	// 825B1C60: 98FF0037  stb r7, 0x37(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(55 as u32), ctx.r[7].u8 ) };
	// 825B1C64: D19F004C  stfs f12, 0x4c(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(76 as u32), tmp.u32 ) };
	// 825B1C68: 98FF003B  stb r7, 0x3b(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(59 as u32), ctx.r[7].u8 ) };
	// 825B1C6C: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 825B1C70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825B1C74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825B1C78: CBE1FFE8  lfd f31, -0x18(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825B1C7C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825B1C80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B1C88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825B1C88 size=20
    let mut pc: u32 = 0x825B1C88;
    'dispatch: loop {
        match pc {
            0x825B1C88 => {
    //   block [0x825B1C88..0x825B1C9C)
	// 825B1C88: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 825B1C8C: 556B0673  rlwinm. r11, r11, 0, 0x19, 0x19
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825B1C90: 4182000C  beq 0x825b1c9c
	if ctx.cr[0].eq {
		sub_825B1C9C(ctx, base);
		return;
	}
	// 825B1C94: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 825B1C98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B1C9C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825B1C9C size=16
    let mut pc: u32 = 0x825B1C9C;
    'dispatch: loop {
        match pc {
            0x825B1C9C => {
    //   block [0x825B1C9C..0x825B1CAC)
	// 825B1C9C: 548B063F  clrlwi. r11, r4, 0x18
	ctx.r[11].u64 = ctx.r[4].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825B1CA0: 4182000C  beq 0x825b1cac
	if ctx.cr[0].eq {
		sub_825B1CAC(ctx, base);
		return;
	}
	// 825B1CA4: 8863003C  lbz r3, 0x3c(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(60 as u32) ) } as u64;
	// 825B1CA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B1CAC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825B1CAC size=8
    let mut pc: u32 = 0x825B1CAC;
    'dispatch: loop {
        match pc {
            0x825B1CAC => {
    //   block [0x825B1CAC..0x825B1CB4)
	// 825B1CAC: 88630038  lbz r3, 0x38(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(56 as u32) ) } as u64;
	// 825B1CB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B1CB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825B1CB8 size=20
    let mut pc: u32 = 0x825B1CB8;
    'dispatch: loop {
        match pc {
            0x825B1CB8 => {
    //   block [0x825B1CB8..0x825B1CCC)
	// 825B1CB8: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 825B1CBC: 556B0631  rlwinm. r11, r11, 0, 0x18, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825B1CC0: 4182000C  beq 0x825b1ccc
	if ctx.cr[0].eq {
		sub_825B1CCC(ctx, base);
		return;
	}
	// 825B1CC4: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 825B1CC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B1CCC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825B1CCC size=16
    let mut pc: u32 = 0x825B1CCC;
    'dispatch: loop {
        match pc {
            0x825B1CCC => {
    //   block [0x825B1CCC..0x825B1CDC)
	// 825B1CCC: 548B063F  clrlwi. r11, r4, 0x18
	ctx.r[11].u64 = ctx.r[4].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825B1CD0: 4182000C  beq 0x825b1cdc
	if ctx.cr[0].eq {
		sub_825B1CDC(ctx, base);
		return;
	}
	// 825B1CD4: 8863003D  lbz r3, 0x3d(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(61 as u32) ) } as u64;
	// 825B1CD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B1CDC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825B1CDC size=8
    let mut pc: u32 = 0x825B1CDC;
    'dispatch: loop {
        match pc {
            0x825B1CDC => {
    //   block [0x825B1CDC..0x825B1CE4)
	// 825B1CDC: 88630039  lbz r3, 0x39(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(57 as u32) ) } as u64;
	// 825B1CE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B1CE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825B1CE8 size=20
    let mut pc: u32 = 0x825B1CE8;
    'dispatch: loop {
        match pc {
            0x825B1CE8 => {
    //   block [0x825B1CE8..0x825B1CFC)
	// 825B1CE8: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 825B1CEC: 556B05AD  rlwinm. r11, r11, 0, 0x16, 0x16
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825B1CF0: 4182000C  beq 0x825b1cfc
	if ctx.cr[0].eq {
		sub_825B1CFC(ctx, base);
		return;
	}
	// 825B1CF4: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 825B1CF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B1CFC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825B1CFC size=16
    let mut pc: u32 = 0x825B1CFC;
    'dispatch: loop {
        match pc {
            0x825B1CFC => {
    //   block [0x825B1CFC..0x825B1D0C)
	// 825B1CFC: 548B063F  clrlwi. r11, r4, 0x18
	ctx.r[11].u64 = ctx.r[4].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825B1D00: 4182000C  beq 0x825b1d0c
	if ctx.cr[0].eq {
		sub_825B1D0C(ctx, base);
		return;
	}
	// 825B1D04: 8863003E  lbz r3, 0x3e(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(62 as u32) ) } as u64;
	// 825B1D08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B1D0C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825B1D0C size=8
    let mut pc: u32 = 0x825B1D0C;
    'dispatch: loop {
        match pc {
            0x825B1D0C => {
    //   block [0x825B1D0C..0x825B1D14)
	// 825B1D0C: 8863003A  lbz r3, 0x3a(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(58 as u32) ) } as u64;
	// 825B1D10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B1D18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825B1D18 size=20
    let mut pc: u32 = 0x825B1D18;
    'dispatch: loop {
        match pc {
            0x825B1D18 => {
    //   block [0x825B1D18..0x825B1D2C)
	// 825B1D18: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 825B1D1C: 556B05EF  rlwinm. r11, r11, 0, 0x17, 0x17
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825B1D20: 4182000C  beq 0x825b1d2c
	if ctx.cr[0].eq {
		sub_825B1D2C(ctx, base);
		return;
	}
	// 825B1D24: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 825B1D28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B1D2C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825B1D2C size=16
    let mut pc: u32 = 0x825B1D2C;
    'dispatch: loop {
        match pc {
            0x825B1D2C => {
    //   block [0x825B1D2C..0x825B1D3C)
	// 825B1D2C: 548B063F  clrlwi. r11, r4, 0x18
	ctx.r[11].u64 = ctx.r[4].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825B1D30: 4182000C  beq 0x825b1d3c
	if ctx.cr[0].eq {
		sub_825B1D3C(ctx, base);
		return;
	}
	// 825B1D34: 8863003F  lbz r3, 0x3f(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(63 as u32) ) } as u64;
	// 825B1D38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B1D3C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825B1D3C size=8
    let mut pc: u32 = 0x825B1D3C;
    'dispatch: loop {
        match pc {
            0x825B1D3C => {
    //   block [0x825B1D3C..0x825B1D44)
	// 825B1D3C: 8863003B  lbz r3, 0x3b(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(59 as u32) ) } as u64;
	// 825B1D40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B1D48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825B1D48 size=12
    let mut pc: u32 = 0x825B1D48;
    'dispatch: loop {
        match pc {
            0x825B1D48 => {
    //   block [0x825B1D48..0x825B1D54)
	// 825B1D48: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 825B1D4C: 996301C8  stb r11, 0x1c8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(456 as u32), ctx.r[11].u8 ) };
	// 825B1D50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B1D58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825B1D58 size=12
    let mut pc: u32 = 0x825B1D58;
    'dispatch: loop {
        match pc {
            0x825B1D58 => {
    //   block [0x825B1D58..0x825B1D64)
	// 825B1D58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825B1D5C: 996301C8  stb r11, 0x1c8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(456 as u32), ctx.r[11].u8 ) };
	// 825B1D60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B1D68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825B1D68 size=152
    let mut pc: u32 = 0x825B1D68;
    'dispatch: loop {
        match pc {
            0x825B1D68 => {
    //   block [0x825B1D68..0x825B1E00)
	// 825B1D68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B1D6C: 48BF6401  bl 0x831a816c
	ctx.lr = 0x825B1D70;
	sub_831A8130(ctx, base);
	// 825B1D70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B1D74: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825B1D78: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825B1D7C: 817E0108  lwz r11, 0x108(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(264 as u32) ) } as u64;
	// 825B1D80: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825B1D84: 419A0064  beq cr6, 0x825b1de8
	if ctx.cr[6].eq {
	pc = 0x825B1DE8; continue 'dispatch;
	}
	// 825B1D88: 389E0028  addi r4, r30, 0x28
	ctx.r[4].s64 = ctx.r[30].s64 + 40;
	// 825B1D8C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825B1D90: 488A4CD1  bl 0x82e56a60
	ctx.lr = 0x825B1D94;
	sub_82E56A60(ctx, base);
	// 825B1D94: 3D608328  lis r11, -0x7cd8
	ctx.r[11].s64 = -2094530560;
	// 825B1D98: 3D408325  lis r10, -0x7cdb
	ctx.r[10].s64 = -2094727168;
	// 825B1D9C: 809E0108  lwz r4, 0x108(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(264 as u32) ) } as u64;
	// 825B1DA0: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B1DA4: 3BCB773C  addi r30, r11, 0x773c
	ctx.r[30].s64 = ctx.r[11].s64 + 30524;
	// 825B1DA8: 3BAA2258  addi r29, r10, 0x2258
	ctx.r[29].s64 = ctx.r[10].s64 + 8792;
	// 825B1DAC: 488AF5B5  bl 0x82e61360
	ctx.lr = 0x825B1DB0;
	sub_82E61360(ctx, base);
	// 825B1DB0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 825B1DB4: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 825B1DB8: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 825B1DBC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825B1DC0: 48BF8189  bl 0x831a9f48
	ctx.lr = 0x825B1DC4;
	sub_831A9F48(ctx, base);
	// 825B1DC4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 825B1DC8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825B1DCC: 4883FEC5  bl 0x82df1c90
	ctx.lr = 0x825B1DD0;
	sub_82DF1C90(ctx, base);
	// 825B1DD0: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 825B1DD4: 419A0014  beq cr6, 0x825b1de8
	if ctx.cr[6].eq {
	pc = 0x825B1DE8; continue 'dispatch;
	}
	// 825B1DD8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 825B1DDC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B1DE0: 481E6EF1  bl 0x82798cd0
	ctx.lr = 0x825B1DE4;
	sub_82798CD0(ctx, base);
	// 825B1DE4: 48000010  b 0x825b1df4
	pc = 0x825B1DF4; continue 'dispatch;
	// 825B1DE8: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 825B1DEC: 816B853C  lwz r11, -0x7ac4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-31428 as u32) ) } as u64;
	// 825B1DF0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825B1DF4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B1DF8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825B1DFC: 48BF63C0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B1E00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x825B1E00 size=16
    let mut pc: u32 = 0x825B1E00;
    'dispatch: loop {
        match pc {
            0x825B1E00 => {
    //   block [0x825B1E00..0x825B1E10)
	// 825B1E00: 396000E0  li r11, 0xe0
	ctx.r[11].s64 = 224;
	// 825B1E04: 13E458C7  vcmpequd (lvx128) v31, v4, v11
	tmp.u32 = ctx.r[4].u32 + ctx.r[11].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B1E10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x825B1E10 size=72
    let mut pc: u32 = 0x825B1E10;
    'dispatch: loop {
        match pc {
            0x825B1E10 => {
    //   block [0x825B1E10..0x825B1E58)
	// 825B1E10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B1E14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825B1E18: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825B1E1C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825B1E20: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B1E24: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825B1E28: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825B1E2C: 807F00D8  lwz r3, 0xd8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(216 as u32) ) } as u64;
	// 825B1E30: 48000CE1  bl 0x825b2b10
	ctx.lr = 0x825B1E34;
	sub_825B2B10(ctx, base);
	// 825B1E34: 396000E0  li r11, 0xe0
	ctx.r[11].s64 = 224;
	// 825B1E38: 13E0F0C7  vcmpequd (lvx128) v31, v0, v30
	tmp.u32 = ctx.r[30].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B1E58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825B1E58 size=196
    let mut pc: u32 = 0x825B1E58;
    'dispatch: loop {
        match pc {
            0x825B1E58 => {
    //   block [0x825B1E58..0x825B1F1C)
	// 825B1E58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B1E5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825B1E60: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825B1E64: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825B1E68: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B1E6C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 825B1E70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825B1E74: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 825B1E78: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 825B1E7C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825B1E80: 4BD0EAB9  bl 0x822c0938
	ctx.lr = 0x825B1E84;
	sub_822C0938(ctx, base);
	// 825B1E84: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 825B1E88: 41820028  beq 0x825b1eb0
	if ctx.cr[0].eq {
	pc = 0x825B1EB0; continue 'dispatch;
	}
	// 825B1E8C: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 825B1E90: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 825B1E94: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825B1E98: 392BB93C  addi r9, r11, -0x46c4
	ctx.r[9].s64 = ctx.r[11].s64 + -18116;
	// 825B1E9C: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 825B1EA0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 825B1EA4: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 825B1EA8: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 825B1EAC: 48000008  b 0x825b1eb4
	pc = 0x825B1EB4; continue 'dispatch;
	// 825B1EB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825B1EB4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825B1EB8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825B1EBC: 409A0044  bne cr6, 0x825b1f00
	if !ctx.cr[6].eq {
	pc = 0x825B1F00; continue 'dispatch;
	}
	// 825B1EC0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 825B1EC4: 419A001C  beq cr6, 0x825b1ee0
	if ctx.cr[6].eq {
	pc = 0x825B1EE0; continue 'dispatch;
	}
	// 825B1EC8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B1ECC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 825B1ED0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B1ED4: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 825B1ED8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825B1EDC: 4E800421  bctrl
	ctx.lr = 0x825B1EE0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825B1EE0: 3D608329  lis r11, -0x7cd7
	ctx.r[11].s64 = -2094465024;
	// 825B1EE4: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 825B1EE8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825B1EEC: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 825B1EF0: 816B8B58  lwz r11, -0x74a8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-29864 as u32) ) } as u64;
	// 825B1EF4: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 825B1EF8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825B1EFC: 4BD0E105  bl 0x822c0000
	ctx.lr = 0x825B1F00;
	sub_822C0000(ctx, base);
	// 825B1F00: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825B1F04: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825B1F08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825B1F0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825B1F10: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825B1F14: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825B1F18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B1F20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825B1F20 size=172
    let mut pc: u32 = 0x825B1F20;
    'dispatch: loop {
        match pc {
            0x825B1F20 => {
    //   block [0x825B1F20..0x825B1FCC)
	// 825B1F20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B1F24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825B1F28: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825B1F2C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825B1F30: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B1F34: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 825B1F38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825B1F3C: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 825B1F40: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 825B1F44: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825B1F48: 4BD0E9F1  bl 0x822c0938
	ctx.lr = 0x825B1F4C;
	sub_822C0938(ctx, base);
	// 825B1F4C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 825B1F50: 41820028  beq 0x825b1f78
	if ctx.cr[0].eq {
	pc = 0x825B1F78; continue 'dispatch;
	}
	// 825B1F54: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 825B1F58: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 825B1F5C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825B1F60: 392BB950  addi r9, r11, -0x46b0
	ctx.r[9].s64 = ctx.r[11].s64 + -18096;
	// 825B1F64: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 825B1F68: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 825B1F6C: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 825B1F70: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 825B1F74: 48000008  b 0x825b1f7c
	pc = 0x825B1F7C; continue 'dispatch;
	// 825B1F78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825B1F7C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825B1F80: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825B1F84: 409A002C  bne cr6, 0x825b1fb0
	if !ctx.cr[6].eq {
	pc = 0x825B1FB0; continue 'dispatch;
	}
	// 825B1F88: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B1F8C: 4BD0E2DD  bl 0x822c0268
	ctx.lr = 0x825B1F90;
	sub_822C0268(ctx, base);
	// 825B1F90: 3D608329  lis r11, -0x7cd7
	ctx.r[11].s64 = -2094465024;
	// 825B1F94: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 825B1F98: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825B1F9C: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 825B1FA0: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 825B1FA4: 816B8B58  lwz r11, -0x74a8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-29864 as u32) ) } as u64;
	// 825B1FA8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825B1FAC: 4BD0E055  bl 0x822c0000
	ctx.lr = 0x825B1FB0;
	sub_822C0000(ctx, base);
	// 825B1FB0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825B1FB4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825B1FB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825B1FBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825B1FC0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825B1FC4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825B1FC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B1FD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825B1FD0 size=196
    let mut pc: u32 = 0x825B1FD0;
    'dispatch: loop {
        match pc {
            0x825B1FD0 => {
    //   block [0x825B1FD0..0x825B2094)
	// 825B1FD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B1FD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825B1FD8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825B1FDC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825B1FE0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B1FE4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 825B1FE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825B1FEC: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 825B1FF0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 825B1FF4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825B1FF8: 4BD0E941  bl 0x822c0938
	ctx.lr = 0x825B1FFC;
	sub_822C0938(ctx, base);
	// 825B1FFC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 825B2000: 41820028  beq 0x825b2028
	if ctx.cr[0].eq {
	pc = 0x825B2028; continue 'dispatch;
	}
	// 825B2004: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 825B2008: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 825B200C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825B2010: 392BB964  addi r9, r11, -0x469c
	ctx.r[9].s64 = ctx.r[11].s64 + -18076;
	// 825B2014: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 825B2018: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 825B201C: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 825B2020: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 825B2024: 48000008  b 0x825b202c
	pc = 0x825B202C; continue 'dispatch;
	// 825B2028: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825B202C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825B2030: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825B2034: 409A0044  bne cr6, 0x825b2078
	if !ctx.cr[6].eq {
	pc = 0x825B2078; continue 'dispatch;
	}
	// 825B2038: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 825B203C: 419A001C  beq cr6, 0x825b2058
	if ctx.cr[6].eq {
	pc = 0x825B2058; continue 'dispatch;
	}
	// 825B2040: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B2044: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 825B2048: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B204C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B2050: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825B2054: 4E800421  bctrl
	ctx.lr = 0x825B2058;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825B2058: 3D608329  lis r11, -0x7cd7
	ctx.r[11].s64 = -2094465024;
	// 825B205C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 825B2060: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825B2064: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 825B2068: 816B8B58  lwz r11, -0x74a8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-29864 as u32) ) } as u64;
	// 825B206C: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 825B2070: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825B2074: 4BD0DF8D  bl 0x822c0000
	ctx.lr = 0x825B2078;
	sub_822C0000(ctx, base);
	// 825B2078: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825B207C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825B2080: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825B2084: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825B2088: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825B208C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825B2090: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B2098(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825B2098 size=104
    let mut pc: u32 = 0x825B2098;
    'dispatch: loop {
        match pc {
            0x825B2098 => {
    //   block [0x825B2098..0x825B2100)
	// 825B2098: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B209C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825B20A0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825B20A4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825B20A8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B20AC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825B20B0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825B20B4: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 825B20B8: 4BF5D4B1  bl 0x8250f568
	ctx.lr = 0x825B20BC;
	sub_8250F568(ctx, base);
	// 825B20BC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B20C0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825B20C4: 388BFF40  addi r4, r11, -0xc0
	ctx.r[4].s64 = ctx.r[11].s64 + -192;
	// 825B20C8: 409A0008  bne cr6, 0x825b20d0
	if !ctx.cr[6].eq {
	pc = 0x825B20D0; continue 'dispatch;
	}
	// 825B20CC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 825B20D0: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 825B20D4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B20D8: 481F9B99  bl 0x827abc70
	ctx.lr = 0x825B20DC;
	sub_827ABC70(ctx, base);
	// 825B20DC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825B20E0: 4883FBB1  bl 0x82df1c90
	ctx.lr = 0x825B20E4;
	sub_82DF1C90(ctx, base);
	// 825B20E4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B20E8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825B20EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825B20F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825B20F4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825B20F8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825B20FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B2100(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x825B2100 size=120
    let mut pc: u32 = 0x825B2100;
    'dispatch: loop {
        match pc {
            0x825B2100 => {
    //   block [0x825B2100..0x825B2178)
	// 825B2100: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B2104: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825B2108: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825B210C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825B2110: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B2114: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 825B2118: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825B211C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825B2120: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 825B2124: C00B08A4  lfs f0, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825B2128: D01F0000  stfs f0, 0(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 825B212C: 48595075  bl 0x82b471a0
	ctx.lr = 0x825B2130;
	sub_82B471A0(ctx, base);
	// 825B2130: 93DF003C  stw r30, 0x3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[30].u32 ) };
	// 825B2134: 387F0040  addi r3, r31, 0x40
	ctx.r[3].s64 = ctx.r[31].s64 + 64;
	// 825B2138: 48840FB9  bl 0x82df30f0
	ctx.lr = 0x825B213C;
	sub_82DF30F0(ctx, base);
	// 825B213C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825B2140: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B2144: 917F0044  stw r11, 0x44(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(68 as u32), ctx.r[11].u32 ) };
	// 825B2148: 917F0048  stw r11, 0x48(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[11].u32 ) };
	// 825B214C: 997F004C  stb r11, 0x4c(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(76 as u32), ctx.r[11].u8 ) };
	// 825B2150: 997F004D  stb r11, 0x4d(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(77 as u32), ctx.r[11].u8 ) };
	// 825B2154: 997F004F  stb r11, 0x4f(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(79 as u32), ctx.r[11].u8 ) };
	// 825B2158: 917F0050  stw r11, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 825B215C: 917F0054  stw r11, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825B2160: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825B2164: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825B2168: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825B216C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825B2170: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825B2174: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B2178(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825B2178 size=92
    let mut pc: u32 = 0x825B2178;
    'dispatch: loop {
        match pc {
            0x825B2178 => {
    //   block [0x825B2178..0x825B21D4)
	// 825B2178: 816400CC  lwz r11, 0xcc(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(204 as u32) ) } as u64;
	// 825B217C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825B2180: 419A0078  beq cr6, 0x825b21f8
	if ctx.cr[6].eq {
		sub_825B21F8(ctx, base);
		return;
	}
	// 825B2184: 814400D0  lwz r10, 0xd0(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(208 as u32) ) } as u64;
	// 825B2188: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 825B218C: 7D6B1E71  srawi. r11, r11, 3
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 3) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 3) as i64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825B2190: 41820068  beq 0x825b21f8
	if ctx.cr[0].eq {
		sub_825B21F8(ctx, base);
		return;
	}
	// 825B2194: 816400CC  lwz r11, 0xcc(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(204 as u32) ) } as u64;
	// 825B2198: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825B219C: 419A0010  beq cr6, 0x825b21ac
	if ctx.cr[6].eq {
	pc = 0x825B21AC; continue 'dispatch;
	}
	// 825B21A0: 814400D0  lwz r10, 0xd0(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(208 as u32) ) } as u64;
	// 825B21A4: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 825B21A8: 7D6B1E70  srawi r11, r11, 3
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 3) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 3) as i64;
	// 825B21AC: 814400CC  lwz r10, 0xcc(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(204 as u32) ) } as u64;
	// 825B21B0: 556B1838  slwi r11, r11, 3
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 825B21B4: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 825B21B8: 394BFFF8  addi r10, r11, -8
	ctx.r[10].s64 = ctx.r[11].s64 + -8;
	// 825B21BC: 814BFFF8  lwz r10, -8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825B21C0: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 825B21C4: 816BFFFC  lwz r11, -4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-4 as u32) ) } as u64;
	// 825B21C8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825B21CC: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 825B21D0: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B21D4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825B21D4 size=36
    let mut pc: u32 = 0x825B21D4;
    'dispatch: loop {
        match pc {
            0x825B21D4 => {
    //   block [0x825B21D4..0x825B21F8)
	// 825B21D4: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 825B21D8: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 825B21DC: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 825B21E0: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 825B21E4: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 825B21E8: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 825B21EC: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 825B21F0: 4082FFE8  bne 0x825b21d8
	if !ctx.cr[0].eq {
	pc = 0x825B21D8; continue 'dispatch;
	}
	// 825B21F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B21F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825B21F8 size=16
    let mut pc: u32 = 0x825B21F8;
    'dispatch: loop {
        match pc {
            0x825B21F8 => {
    //   block [0x825B21F8..0x825B2208)
	// 825B21F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825B21FC: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825B2200: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 825B2204: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B2208(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825B2208 size=24
    let mut pc: u32 = 0x825B2208;
    'dispatch: loop {
        match pc {
            0x825B2208 => {
    //   block [0x825B2208..0x825B2220)
	// 825B2208: 816400F0  lwz r11, 0xf0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(240 as u32) ) } as u64;
	// 825B220C: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825B2210: 816400F4  lwz r11, 0xf4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(244 as u32) ) } as u64;
	// 825B2214: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825B2218: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 825B221C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B2220(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825B2220 size=36
    let mut pc: u32 = 0x825B2220;
    'dispatch: loop {
        match pc {
            0x825B2220 => {
    //   block [0x825B2220..0x825B2244)
	// 825B2220: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 825B2224: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 825B2228: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 825B222C: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 825B2230: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 825B2234: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 825B2238: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 825B223C: 4082FFE8  bne 0x825b2224
	if !ctx.cr[0].eq {
	pc = 0x825B2224; continue 'dispatch;
	}
	// 825B2240: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B2248(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825B2248 size=164
    let mut pc: u32 = 0x825B2248;
    'dispatch: loop {
        match pc {
            0x825B2248 => {
    //   block [0x825B2248..0x825B22EC)
	// 825B2248: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B224C: 48BF5F21  bl 0x831a816c
	ctx.lr = 0x825B2250;
	sub_831A8130(ctx, base);
	// 825B2250: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B2254: 81640004  lwz r11, 4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 825B2258: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 825B225C: 81440000  lwz r10, 0(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B2260: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825B2264: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825B2268: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 825B226C: 419A0024  beq cr6, 0x825b2290
	if ctx.cr[6].eq {
	pc = 0x825B2290; continue 'dispatch;
	}
	// 825B2270: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 825B2274: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 825B2278: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 825B227C: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 825B2280: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 825B2284: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 825B2288: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 825B228C: 4082FFE8  bne 0x825b2274
	if !ctx.cr[0].eq {
	pc = 0x825B2274; continue 'dispatch;
	}
	// 825B2290: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 825B2294: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 825B2298: 4BF5D231  bl 0x8250f4c8
	ctx.lr = 0x825B229C;
	sub_8250F4C8(ctx, base);
	// 825B229C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B22A0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825B22A4: 3BEBFFFC  addi r31, r11, -4
	ctx.r[31].s64 = ctx.r[11].s64 + -4;
	// 825B22A8: 409A0008  bne cr6, 0x825b22b0
	if !ctx.cr[6].eq {
	pc = 0x825B22B0; continue 'dispatch;
	}
	// 825B22AC: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 825B22B0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 825B22B4: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 825B22B8: 3BA10050  addi r29, r1, 0x50
	ctx.r[29].s64 = ctx.r[1].s64 + 80;
	// 825B22BC: 4BF5D25D  bl 0x8250f518
	ctx.lr = 0x825B22C0;
	sub_8250F518(ctx, base);
	// 825B22C0: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 825B22C4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B22C8: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 825B22CC: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 825B22D0: 4BF5B239  bl 0x8250d508
	ctx.lr = 0x825B22D4;
	sub_8250D508(ctx, base);
	// 825B22D4: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 825B22D8: 4883F9B9  bl 0x82df1c90
	ctx.lr = 0x825B22DC;
	sub_82DF1C90(ctx, base);
	// 825B22DC: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 825B22E0: 4883F9B1  bl 0x82df1c90
	ctx.lr = 0x825B22E4;
	sub_82DF1C90(ctx, base);
	// 825B22E4: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 825B22E8: 48BF5ED4  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B22F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825B22F0 size=32
    let mut pc: u32 = 0x825B22F0;
    'dispatch: loop {
        match pc {
            0x825B22F0 => {
    //   block [0x825B22F0..0x825B2310)
	// 825B22F0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 825B22F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825B22F8: 394B0174  addi r10, r11, 0x174
	ctx.r[10].s64 = ctx.r[11].s64 + 372;
	// 825B22FC: 912B0174  stw r9, 0x174(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(372 as u32), ctx.r[9].u32 ) };
	// 825B2300: 806B0178  lwz r3, 0x178(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(376 as u32) ) } as u64;
	// 825B2304: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825B2308: 912B0178  stw r9, 0x178(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(376 as u32), ctx.r[9].u32 ) };
	// 825B230C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B2310(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825B2310 size=8
    let mut pc: u32 = 0x825B2310;
    'dispatch: loop {
        match pc {
            0x825B2310 => {
    //   block [0x825B2310..0x825B2318)
	// 825B2310: 4BD0E580  b 0x822c0890
	sub_822C0890(ctx, base);
	return;
	// 825B2314: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B2318(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825B2318 size=112
    let mut pc: u32 = 0x825B2318;
    'dispatch: loop {
        match pc {
            0x825B2318 => {
    //   block [0x825B2318..0x825B2388)
	// 825B2318: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B231C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825B2320: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825B2324: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825B2328: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B232C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825B2330: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825B2334: 93C10050  stw r30, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 825B2338: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 825B233C: 4BFFFB1D  bl 0x825b1e58
	ctx.lr = 0x825B2340;
	sub_825B1E58(ctx, base);
	// 825B2340: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 825B2344: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 825B2348: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 825B234C: 4BD0DCB5  bl 0x822c0000
	ctx.lr = 0x825B2350;
	sub_822C0000(ctx, base);
	// 825B2350: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 825B2354: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 825B2358: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825B235C: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 825B2360: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825B2364: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 825B2368: 419A0008  beq cr6, 0x825b2370
	if ctx.cr[6].eq {
	pc = 0x825B2370; continue 'dispatch;
	}
	// 825B236C: 4BD0E525  bl 0x822c0890
	ctx.lr = 0x825B2370;
	sub_822C0890(ctx, base);
	// 825B2370: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825B2374: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825B2378: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825B237C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825B2380: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825B2384: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B2388(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825B2388 size=112
    let mut pc: u32 = 0x825B2388;
    'dispatch: loop {
        match pc {
            0x825B2388 => {
    //   block [0x825B2388..0x825B23F8)
	// 825B2388: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B238C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825B2390: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825B2394: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825B2398: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B239C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825B23A0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825B23A4: 93C10050  stw r30, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 825B23A8: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 825B23AC: 4BFFFB75  bl 0x825b1f20
	ctx.lr = 0x825B23B0;
	sub_825B1F20(ctx, base);
	// 825B23B0: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 825B23B4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 825B23B8: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 825B23BC: 4BD0DC45  bl 0x822c0000
	ctx.lr = 0x825B23C0;
	sub_822C0000(ctx, base);
	// 825B23C0: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 825B23C4: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 825B23C8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825B23CC: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 825B23D0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825B23D4: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 825B23D8: 419A0008  beq cr6, 0x825b23e0
	if ctx.cr[6].eq {
	pc = 0x825B23E0; continue 'dispatch;
	}
	// 825B23DC: 4BD0E4B5  bl 0x822c0890
	ctx.lr = 0x825B23E0;
	sub_822C0890(ctx, base);
	// 825B23E0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825B23E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825B23E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825B23EC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825B23F0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825B23F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B23F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825B23F8 size=112
    let mut pc: u32 = 0x825B23F8;
    'dispatch: loop {
        match pc {
            0x825B23F8 => {
    //   block [0x825B23F8..0x825B2468)
	// 825B23F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B23FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825B2400: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825B2404: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825B2408: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B240C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825B2410: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825B2414: 93C10050  stw r30, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 825B2418: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 825B241C: 4BF40F9D  bl 0x824f33b8
	ctx.lr = 0x825B2420;
	sub_824F33B8(ctx, base);
	// 825B2420: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 825B2424: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 825B2428: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 825B242C: 4BD0DBD5  bl 0x822c0000
	ctx.lr = 0x825B2430;
	sub_822C0000(ctx, base);
	// 825B2430: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 825B2434: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 825B2438: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825B243C: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 825B2440: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825B2444: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 825B2448: 419A0008  beq cr6, 0x825b2450
	if ctx.cr[6].eq {
	pc = 0x825B2450; continue 'dispatch;
	}
	// 825B244C: 4BD0E445  bl 0x822c0890
	ctx.lr = 0x825B2450;
	sub_822C0890(ctx, base);
	// 825B2450: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825B2454: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825B2458: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825B245C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825B2460: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825B2464: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B2468(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x825B2468 size=124
    let mut pc: u32 = 0x825B2468;
    'dispatch: loop {
        match pc {
            0x825B2468 => {
    //   block [0x825B2468..0x825B24E4)
	// 825B2468: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B246C: 48BF5D01  bl 0x831a816c
	ctx.lr = 0x825B2470;
	sub_831A8130(ctx, base);
	// 825B2470: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B2474: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 825B2478: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 825B247C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 825B2480: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825B2484: 388BA66C  addi r4, r11, -0x5994
	ctx.r[4].s64 = ctx.r[11].s64 + -22932;
	// 825B2488: 38A00089  li r5, 0x89
	ctx.r[5].s64 = 137;
	// 825B248C: 38600030  li r3, 0x30
	ctx.r[3].s64 = 48;
	// 825B2490: 4883FF59  bl 0x82df23e8
	ctx.lr = 0x825B2494;
	sub_82DF23E8(ctx, base);
	// 825B2494: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 825B2498: 41820018  beq 0x825b24b0
	if ctx.cr[0].eq {
	pc = 0x825B24B0; continue 'dispatch;
	}
	// 825B249C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825B24A0: C03F0000  lfs f1, 0(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 825B24A4: 4859F505  bl 0x82b519a8
	ctx.lr = 0x825B24A8;
	sub_82B519A8(ctx, base);
	// 825B24A8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825B24AC: 48000008  b 0x825b24b4
	pc = 0x825B24B4; continue 'dispatch;
	// 825B24B0: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 825B24B4: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 825B24B8: 3BDD0004  addi r30, r29, 4
	ctx.r[30].s64 = ctx.r[29].s64 + 4;
	// 825B24BC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825B24C0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825B24C4: 4BD4EE25  bl 0x823012e8
	ctx.lr = 0x825B24C8;
	sub_823012E8(ctx, base);
	// 825B24C8: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 825B24CC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825B24D0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825B24D4: 4BD0DB2D  bl 0x822c0000
	ctx.lr = 0x825B24D8;
	sub_822C0000(ctx, base);
	// 825B24D8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 825B24DC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825B24E0: 48BF5CDC  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B24E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825B24E8 size=112
    let mut pc: u32 = 0x825B24E8;
    'dispatch: loop {
        match pc {
            0x825B24E8 => {
    //   block [0x825B24E8..0x825B2558)
	// 825B24E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B24EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825B24F0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825B24F4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825B24F8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B24FC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825B2500: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825B2504: 93C10050  stw r30, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 825B2508: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 825B250C: 4BFFFAC5  bl 0x825b1fd0
	ctx.lr = 0x825B2510;
	sub_825B1FD0(ctx, base);
	// 825B2510: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 825B2514: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 825B2518: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 825B251C: 4BD0DAE5  bl 0x822c0000
	ctx.lr = 0x825B2520;
	sub_822C0000(ctx, base);
	// 825B2520: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 825B2524: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 825B2528: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825B252C: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 825B2530: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825B2534: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 825B2538: 419A0008  beq cr6, 0x825b2540
	if ctx.cr[6].eq {
	pc = 0x825B2540; continue 'dispatch;
	}
	// 825B253C: 4BD0E355  bl 0x822c0890
	ctx.lr = 0x825B2540;
	sub_822C0890(ctx, base);
	// 825B2540: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825B2544: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825B2548: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825B254C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825B2550: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825B2554: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B2558(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x825B2558 size=248
    let mut pc: u32 = 0x825B2558;
    'dispatch: loop {
        match pc {
            0x825B2558 => {
    //   block [0x825B2558..0x825B2650)
	// 825B2558: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B255C: 48BF5C11  bl 0x831a816c
	ctx.lr = 0x825B2560;
	sub_831A8130(ctx, base);
	// 825B2560: DBE1FFD8  stfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 825B2564: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B2568: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 825B256C: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 825B2570: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825B2574: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 825B2578: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 825B257C: 389F00E0  addi r4, r31, 0xe0
	ctx.r[4].s64 = ctx.r[31].s64 + 224;
	// 825B2580: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 825B2584: 4BFE97A5  bl 0x8259bd28
	ctx.lr = 0x825B2588;
	sub_8259BD28(ctx, base);
	// 825B2588: EC1F07F2  fmuls f0, f31, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = (((ctx.f[31].f64 * ctx.f[31].f64) as f32) as f64);
	// 825B258C: 83C10060  lwz r30, 0x60(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 825B2590: 83A10064  lwz r29, 0x64(r1)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 825B2594: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 825B2598: D01E0038  stfs f0, 0x38(r30)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(56 as u32), tmp.u32 ) };
	// 825B259C: 93C10058  stw r30, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[30].u32 ) };
	// 825B25A0: 93A1005C  stw r29, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[29].u32 ) };
	// 825B25A4: 419A0024  beq cr6, 0x825b25c8
	if ctx.cr[6].eq {
	pc = 0x825B25C8; continue 'dispatch;
	}
	// 825B25A8: 397D0004  addi r11, r29, 4
	ctx.r[11].s64 = ctx.r[29].s64 + 4;
	// 825B25AC: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 825B25B0: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 825B25B4: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 825B25B8: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 825B25BC: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 825B25C0: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 825B25C4: 4082FFE8  bne 0x825b25ac
	if !ctx.cr[0].eq {
	pc = 0x825B25AC; continue 'dispatch;
	}
	// 825B25C8: 3D608328  lis r11, -0x7cd8
	ctx.r[11].s64 = -2094530560;
	// 825B25CC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825B25D0: 808BD054  lwz r4, -0x2fac(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-12204 as u32) ) } as u64;
	// 825B25D4: 48841435  bl 0x82df3a08
	ctx.lr = 0x825B25D8;
	sub_82DF3A08(ctx, base);
	// 825B25D8: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 825B25DC: 38E10058  addi r7, r1, 0x58
	ctx.r[7].s64 = ctx.r[1].s64 + 88;
	// 825B25E0: 388BB978  addi r4, r11, -0x4688
	ctx.r[4].s64 = ctx.r[11].s64 + -18056;
	// 825B25E4: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 825B25E8: 38A003D6  li r5, 0x3d6
	ctx.r[5].s64 = 982;
	// 825B25EC: 387F0028  addi r3, r31, 0x28
	ctx.r[3].s64 = ctx.r[31].s64 + 40;
	// 825B25F0: 488A6B31  bl 0x82e59120
	ctx.lr = 0x825B25F4;
	sub_82E59120(ctx, base);
	// 825B25F4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825B25F8: 48840E31  bl 0x82df3428
	ctx.lr = 0x825B25FC;
	sub_82DF3428(ctx, base);
	// 825B25FC: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 825B2600: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825B2604: 419A0008  beq cr6, 0x825b260c
	if ctx.cr[6].eq {
	pc = 0x825B260C; continue 'dispatch;
	}
	// 825B2608: 4BD0E289  bl 0x822c0890
	ctx.lr = 0x825B260C;
	sub_822C0890(ctx, base);
	// 825B260C: 817E0030  lwz r11, 0x30(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 825B2610: 807F017C  lwz r3, 0x17c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(380 as u32) ) } as u64;
	// 825B2614: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825B2618: 917F0108  stw r11, 0x108(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(264 as u32), ctx.r[11].u32 ) };
	// 825B261C: 419A000C  beq cr6, 0x825b2628
	if ctx.cr[6].eq {
	pc = 0x825B2628; continue 'dispatch;
	}
	// 825B2620: 48001721  bl 0x825b3d40
	ctx.lr = 0x825B2624;
	sub_825B3D40(ctx, base);
	// 825B2624: 48000008  b 0x825b262c
	pc = 0x825B262C; continue 'dispatch;
	// 825B2628: 480016E9  bl 0x825b3d10
	ctx.lr = 0x825B262C;
	sub_825B3D10(ctx, base);
	// 825B262C: 83FE0030  lwz r31, 0x30(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 825B2630: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 825B2634: 419A000C  beq cr6, 0x825b2640
	if ctx.cr[6].eq {
	pc = 0x825B2640; continue 'dispatch;
	}
	// 825B2638: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 825B263C: 4BD0E255  bl 0x822c0890
	ctx.lr = 0x825B2640;
	sub_822C0890(ctx, base);
	// 825B2640: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B2644: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 825B2648: CBE1FFD8  lfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-40 as u32) ) };
	// 825B264C: 48BF5B70  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B2650(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825B2650 size=188
    let mut pc: u32 = 0x825B2650;
    'dispatch: loop {
        match pc {
            0x825B2650 => {
    //   block [0x825B2650..0x825B270C)
	// 825B2650: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B2654: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825B2658: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825B265C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825B2660: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B2664: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 825B2668: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 825B266C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825B2670: 388BB978  addi r4, r11, -0x4688
	ctx.r[4].s64 = ctx.r[11].s64 + -18056;
	// 825B2674: 38A00413  li r5, 0x413
	ctx.r[5].s64 = 1043;
	// 825B2678: 386000C4  li r3, 0xc4
	ctx.r[3].s64 = 196;
	// 825B267C: 4883FD6D  bl 0x82df23e8
	ctx.lr = 0x825B2680;
	sub_82DF23E8(ctx, base);
	// 825B2680: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 825B2684: 41820010  beq 0x825b2694
	if ctx.cr[0].eq {
	pc = 0x825B2694; continue 'dispatch;
	}
	// 825B2688: 48006759  bl 0x825b8de0
	ctx.lr = 0x825B268C;
	sub_825B8DE0(ctx, base);
	// 825B268C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 825B2690: 48000008  b 0x825b2698
	pc = 0x825B2698; continue 'dispatch;
	// 825B2694: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 825B2698: 387E0174  addi r3, r30, 0x174
	ctx.r[3].s64 = ctx.r[30].s64 + 372;
	// 825B269C: 4BFFFE4D  bl 0x825b24e8
	ctx.lr = 0x825B26A0;
	sub_825B24E8(ctx, base);
	// 825B26A0: 817E0174  lwz r11, 0x174(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(372 as u32) ) } as u64;
	// 825B26A4: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 825B26A8: 83FE0178  lwz r31, 0x178(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(376 as u32) ) } as u64;
	// 825B26AC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 825B26B0: 93E10054  stw r31, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[31].u32 ) };
	// 825B26B4: 419A0024  beq cr6, 0x825b26d8
	if ctx.cr[6].eq {
	pc = 0x825B26D8; continue 'dispatch;
	}
	// 825B26B8: 397F0004  addi r11, r31, 4
	ctx.r[11].s64 = ctx.r[31].s64 + 4;
	// 825B26BC: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 825B26C0: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 825B26C4: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 825B26C8: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 825B26CC: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 825B26D0: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 825B26D4: 4082FFE8  bne 0x825b26bc
	if !ctx.cr[0].eq {
	pc = 0x825B26BC; continue 'dispatch;
	}
	// 825B26D8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 825B26DC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825B26E0: 4BFFFB69  bl 0x825b2248
	ctx.lr = 0x825B26E4;
	sub_825B2248(ctx, base);
	// 825B26E4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 825B26E8: 419A000C  beq cr6, 0x825b26f4
	if ctx.cr[6].eq {
	pc = 0x825B26F4; continue 'dispatch;
	}
	// 825B26EC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B26F0: 4BD0E1A1  bl 0x822c0890
	ctx.lr = 0x825B26F4;
	sub_822C0890(ctx, base);
	// 825B26F4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825B26F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825B26FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825B2700: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825B2704: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825B2708: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B2710(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825B2710 size=104
    let mut pc: u32 = 0x825B2710;
    'dispatch: loop {
        match pc {
            0x825B2710 => {
    //   block [0x825B2710..0x825B2778)
	// 825B2710: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B2714: 48BF5A55  bl 0x831a8168
	ctx.lr = 0x825B2718;
	sub_831A8130(ctx, base);
	// 825B2718: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B271C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825B2720: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 825B2724: 7FDFF378  mr r31, r30
	ctx.r[31].u64 = ctx.r[30].u64;
	// 825B2728: 897E0019  lbz r11, 0x19(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(25 as u32) ) } as u64;
	// 825B272C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825B2730: 409A0040  bne cr6, 0x825b2770
	if !ctx.cr[6].eq {
	pc = 0x825B2770; continue 'dispatch;
	}
	// 825B2734: 3F808335  lis r28, -0x7ccb
	ctx.r[28].s64 = -2093678592;
	// 825B2738: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 825B273C: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 825B2740: 4BFFFFD1  bl 0x825b2710
	ctx.lr = 0x825B2744;
	sub_825B2710(ctx, base);
	// 825B2744: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 825B2748: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825B274C: 83FF0000  lwz r31, 0(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B2750: 4822A651  bl 0x827dcda0
	ctx.lr = 0x825B2754;
	sub_827DCDA0(ctx, base);
	// 825B2754: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 825B2758: 807C110C  lwz r3, 0x110c(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4364 as u32) ) } as u64;
	// 825B275C: 4883FA2D  bl 0x82df2188
	ctx.lr = 0x825B2760;
	sub_82DF2188(ctx, base);
	// 825B2760: 897F0019  lbz r11, 0x19(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(25 as u32) ) } as u64;
	// 825B2764: 7FFEFB78  mr r30, r31
	ctx.r[30].u64 = ctx.r[31].u64;
	// 825B2768: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825B276C: 419AFFCC  beq cr6, 0x825b2738
	if ctx.cr[6].eq {
	pc = 0x825B2738; continue 'dispatch;
	}
	// 825B2770: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825B2774: 48BF5A44  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B2778(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825B2778 size=84
    let mut pc: u32 = 0x825B2778;
    'dispatch: loop {
        match pc {
            0x825B2778 => {
    //   block [0x825B2778..0x825B27CC)
	// 825B2778: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B277C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825B2780: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825B2784: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B2788: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825B278C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 825B2790: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 825B2794: 4BFFFF7D  bl 0x825b2710
	ctx.lr = 0x825B2798;
	sub_825B2710(ctx, base);
	// 825B2798: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 825B279C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825B27A0: 914A0004  stw r10, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 825B27A4: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 825B27A8: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 825B27AC: 914A0000  stw r10, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 825B27B0: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 825B27B4: 916B0008  stw r11, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 825B27B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825B27BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825B27C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825B27C4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825B27C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B27D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825B27D0 size=84
    let mut pc: u32 = 0x825B27D0;
    'dispatch: loop {
        match pc {
            0x825B27D0 => {
    //   block [0x825B27D0..0x825B2824)
	// 825B27D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B27D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825B27D8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825B27DC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825B27E0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B27E4: 386300C8  addi r3, r3, 0xc8
	ctx.r[3].s64 = ctx.r[3].s64 + 200;
	// 825B27E8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 825B27EC: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 825B27F0: 48600919  bl 0x82bb3108
	ctx.lr = 0x825B27F4;
	sub_82BB3108(ctx, base);
	// 825B27F4: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B27F8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 825B27FC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B2800: 816B004C  lwz r11, 0x4c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(76 as u32) ) } as u64;
	// 825B2804: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825B2808: 4E800421  bctrl
	ctx.lr = 0x825B280C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825B280C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825B2810: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825B2814: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825B2818: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825B281C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825B2820: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B2828(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825B2828 size=188
    let mut pc: u32 = 0x825B2828;
    'dispatch: loop {
        match pc {
            0x825B2828 => {
    //   block [0x825B2828..0x825B28E4)
	// 825B2828: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B282C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825B2830: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825B2834: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825B2838: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B283C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 825B2840: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825B2844: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 825B2848: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 825B284C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825B2850: 4BD0E0E9  bl 0x822c0938
	ctx.lr = 0x825B2854;
	sub_822C0938(ctx, base);
	// 825B2854: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 825B2858: 41820028  beq 0x825b2880
	if ctx.cr[0].eq {
	pc = 0x825B2880; continue 'dispatch;
	}
	// 825B285C: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 825B2860: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 825B2864: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825B2868: 392BB928  addi r9, r11, -0x46d8
	ctx.r[9].s64 = ctx.r[11].s64 + -18136;
	// 825B286C: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 825B2870: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 825B2874: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 825B2878: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 825B287C: 48000008  b 0x825b2884
	pc = 0x825B2884; continue 'dispatch;
	// 825B2880: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825B2884: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825B2888: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825B288C: 409A003C  bne cr6, 0x825b28c8
	if !ctx.cr[6].eq {
	pc = 0x825B28C8; continue 'dispatch;
	}
	// 825B2890: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 825B2894: 419A0014  beq cr6, 0x825b28a8
	if ctx.cr[6].eq {
	pc = 0x825B28A8; continue 'dispatch;
	}
	// 825B2898: 387F001C  addi r3, r31, 0x1c
	ctx.r[3].s64 = ctx.r[31].s64 + 28;
	// 825B289C: 4853AA5D  bl 0x82aed2f8
	ctx.lr = 0x825B28A0;
	sub_82AED2F8(ctx, base);
	// 825B28A0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B28A4: 4BD0D9C5  bl 0x822c0268
	ctx.lr = 0x825B28A8;
	sub_822C0268(ctx, base);
	// 825B28A8: 3D608329  lis r11, -0x7cd7
	ctx.r[11].s64 = -2094465024;
	// 825B28AC: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 825B28B0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825B28B4: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 825B28B8: 816B8B58  lwz r11, -0x74a8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-29864 as u32) ) } as u64;
	// 825B28BC: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 825B28C0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825B28C4: 4BD0D73D  bl 0x822c0000
	ctx.lr = 0x825B28C8;
	sub_822C0000(ctx, base);
	// 825B28C8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825B28CC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825B28D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825B28D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825B28D8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825B28DC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825B28E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B28E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825B28E8 size=64
    let mut pc: u32 = 0x825B28E8;
    'dispatch: loop {
        match pc {
            0x825B28E8 => {
    //   block [0x825B28E8..0x825B2928)
	// 825B28E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B28EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825B28F0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825B28F4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B28F8: 83E3000C  lwz r31, 0xc(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 825B28FC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 825B2900: 419A0014  beq cr6, 0x825b2914
	if ctx.cr[6].eq {
	pc = 0x825B2914; continue 'dispatch;
	}
	// 825B2904: 387F001C  addi r3, r31, 0x1c
	ctx.r[3].s64 = ctx.r[31].s64 + 28;
	// 825B2908: 4853A9F1  bl 0x82aed2f8
	ctx.lr = 0x825B290C;
	sub_82AED2F8(ctx, base);
	// 825B290C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B2910: 4BD0D959  bl 0x822c0268
	ctx.lr = 0x825B2914;
	sub_822C0268(ctx, base);
	// 825B2914: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825B2918: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825B291C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825B2920: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825B2924: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B2928(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825B2928 size=112
    let mut pc: u32 = 0x825B2928;
    'dispatch: loop {
        match pc {
            0x825B2928 => {
    //   block [0x825B2928..0x825B2998)
	// 825B2928: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B292C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825B2930: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825B2934: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825B2938: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B293C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825B2940: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825B2944: 93C10050  stw r30, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 825B2948: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 825B294C: 4BFECAFD  bl 0x8259f448
	ctx.lr = 0x825B2950;
	sub_8259F448(ctx, base);
	// 825B2950: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 825B2954: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 825B2958: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 825B295C: 4BD0D6A5  bl 0x822c0000
	ctx.lr = 0x825B2960;
	sub_822C0000(ctx, base);
	// 825B2960: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 825B2964: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 825B2968: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825B296C: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 825B2970: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825B2974: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 825B2978: 419A0008  beq cr6, 0x825b2980
	if ctx.cr[6].eq {
	pc = 0x825B2980; continue 'dispatch;
	}
	// 825B297C: 4BD0DF15  bl 0x822c0890
	ctx.lr = 0x825B2980;
	sub_822C0890(ctx, base);
	// 825B2980: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825B2984: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825B2988: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825B298C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825B2990: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825B2994: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B2998(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825B2998 size=68
    let mut pc: u32 = 0x825B2998;
    'dispatch: loop {
        match pc {
            0x825B2998 => {
    //   block [0x825B2998..0x825B29DC)
	// 825B2998: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B299C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825B29A0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825B29A4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B29A8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825B29AC: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 825B29B0: 548A07FF  clrlwi. r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 825B29B4: 396BB9DC  addi r11, r11, -0x4624
	ctx.r[11].s64 = ctx.r[11].s64 + -17956;
	// 825B29B8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825B29BC: 41820008  beq 0x825b29c4
	if ctx.cr[0].eq {
	pc = 0x825B29C4; continue 'dispatch;
	}
	// 825B29C0: 4BD0D8A9  bl 0x822c0268
	ctx.lr = 0x825B29C4;
	sub_822C0268(ctx, base);
	// 825B29C4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B29C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825B29CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825B29D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825B29D4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825B29D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B29E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x825B29E0 size=172
    let mut pc: u32 = 0x825B29E0;
    'dispatch: loop {
        match pc {
            0x825B29E0 => {
    //   block [0x825B29E0..0x825B2A8C)
	// 825B29E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B29E4: 48BF5789  bl 0x831a816c
	ctx.lr = 0x825B29E8;
	sub_831A8130(ctx, base);
	// 825B29E8: DBE1FFD8  stfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 825B29EC: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B29F0: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 825B29F4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825B29F8: 54AB063F  clrlwi. r11, r5, 0x18
	ctx.r[11].u64 = ctx.r[5].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825B29FC: C3EA08A4  lfs f31, 0x8a4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2212 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 825B2A00: 41820064  beq 0x825b2a64
	if ctx.cr[0].eq {
	pc = 0x825B2A64; continue 'dispatch;
	}
	// 825B2A04: 7FA4FA14  add r29, r4, r31
	ctx.r[29].u64 = ctx.r[4].u64 + ctx.r[31].u64;
	// 825B2A08: 897D010C  lbz r11, 0x10c(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(268 as u32) ) } as u64;
	// 825B2A0C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 825B2A10: 4082001C  bne 0x825b2a2c
	if !ctx.cr[0].eq {
	pc = 0x825B2A2C; continue 'dispatch;
	}
	// 825B2A14: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 825B2A18: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 825B2A1C: 997D010C  stb r11, 0x10c(r29)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[29].u32.wrapping_add(268 as u32), ctx.r[11].u8 ) };
	// 825B2A20: 997D0112  stb r11, 0x112(r29)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[29].u32.wrapping_add(274 as u32), ctx.r[11].u8 ) };
	// 825B2A24: C3EA08A8  lfs f31, 0x8a8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2216 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 825B2A28: 48000054  b 0x825b2a7c
	pc = 0x825B2A7C; continue 'dispatch;
	// 825B2A2C: 39640046  addi r11, r4, 0x46
	ctx.r[11].s64 = ctx.r[4].s64 + 70;
	// 825B2A30: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B2A34: 557E103A  slwi r30, r11, 2
	ctx.r[30].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 825B2A38: 488A6D41  bl 0x82e59778
	ctx.lr = 0x825B2A3C;
	sub_82E59778(ctx, base);
	// 825B2A3C: 7C1EFC2E  lfsx f0, r30, r31
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[31].u32)) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825B2A40: EDA1002A  fadds f13, f1, f0
	ctx.f[13].f64 = ((ctx.f[1].f64 + ctx.f[0].f64) as f32) as f64;
	// 825B2A44: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 825B2A48: 7DBEFD2E  stfsx f13, r30, r31
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[30].u32.wrapping_add(ctx.r[31].u32), tmp.u32) };
	// 825B2A4C: C00B0A90  lfs f0, 0xa90(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2704 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825B2A50: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 825B2A54: 41980028  blt cr6, 0x825b2a7c
	if ctx.cr[6].lt {
	pc = 0x825B2A7C; continue 'dispatch;
	}
	// 825B2A58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825B2A5C: 997D010C  stb r11, 0x10c(r29)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[29].u32.wrapping_add(268 as u32), ctx.r[11].u8 ) };
	// 825B2A60: 4800001C  b 0x825b2a7c
	pc = 0x825B2A7C; continue 'dispatch;
	// 825B2A64: 39640046  addi r11, r4, 0x46
	ctx.r[11].s64 = ctx.r[4].s64 + 70;
	// 825B2A68: 7D44FA14  add r10, r4, r31
	ctx.r[10].u64 = ctx.r[4].u64 + ctx.r[31].u64;
	// 825B2A6C: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 825B2A70: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825B2A74: 992A010C  stb r9, 0x10c(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(268 as u32), ctx.r[9].u8 ) };
	// 825B2A78: 7FEBFD2E  stfsx f31, r11, r31
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32), tmp.u32) };
	// 825B2A7C: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 825B2A80: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825B2A84: CBE1FFD8  lfd f31, -0x28(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-40 as u32) ) };
	// 825B2A88: 48BF5734  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B2A90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x825B2A90 size=60
    let mut pc: u32 = 0x825B2A90;
    'dispatch: loop {
        match pc {
            0x825B2A90 => {
    //   block [0x825B2A90..0x825B2ACC)
	// 825B2A90: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 825B2A94: C0040000  lfs f0, 0(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825B2A98: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 825B2A9C: 3D208202  lis r9, -0x7dfe
	ctx.r[9].s64 = -2113798144;
	// 825B2AA0: 3D008200  lis r8, -0x7e00
	ctx.r[8].s64 = -2113929216;
	// 825B2AA4: 3CE08200  lis r7, -0x7e00
	ctx.r[7].s64 = -2113929216;
	// 825B2AA8: C1AB6150  lfs f13, 0x6150(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24912 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825B2AAC: C12A9534  lfs f9, -0x6acc(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-27340 as u32) ) };
	ctx.f[9].f64 = (tmp.f32 as f64);
	// 825B2AB0: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 825B2AB4: C1892534  lfs f12, 0x2534(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(9524 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 825B2AB8: C14808A4  lfs f10, 0x8a4(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(2212 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 825B2ABC: C16708A8  lfs f11, 0x8a8(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(2216 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 825B2AC0: 4198000C  blt cr6, 0x825b2acc
	if ctx.cr[6].lt {
		sub_825B2ACC(ctx, base);
		return;
	}
	// 825B2AC4: D1640000  stfs f11, 0(r4)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 825B2AC8: 48000018  b 0x825b2ae0
	sub_825B2ADC(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B2ACC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x825B2ACC size=16
    let mut pc: u32 = 0x825B2ACC;
    'dispatch: loop {
        match pc {
            0x825B2ACC => {
    //   block [0x825B2ACC..0x825B2ADC)
	// 825B2ACC: FF006000  fcmpu cr6, f0, f12
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[12].f64);
	// 825B2AD0: 4199000C  bgt cr6, 0x825b2adc
	if ctx.cr[6].gt {
		sub_825B2ADC(ctx, base);
		return;
	}
	// 825B2AD4: D1240000  stfs f9, 0(r4)
	tmp.f32 = (ctx.f[9].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 825B2AD8: 48000008  b 0x825b2ae0
	sub_825B2ADC(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B2ADC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x825B2ADC size=24
    let mut pc: u32 = 0x825B2ADC;
    'dispatch: loop {
        match pc {
            0x825B2ADC => {
    //   block [0x825B2ADC..0x825B2AF4)
	// 825B2ADC: D1440000  stfs f10, 0(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[10].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 825B2AE0: C0040008  lfs f0, 8(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825B2AE4: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 825B2AE8: 4198000C  blt cr6, 0x825b2af4
	if ctx.cr[6].lt {
		sub_825B2AF4(ctx, base);
		return;
	}
	// 825B2AEC: D1640008  stfs f11, 8(r4)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 825B2AF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B2AF4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x825B2AF4 size=16
    let mut pc: u32 = 0x825B2AF4;
    'dispatch: loop {
        match pc {
            0x825B2AF4 => {
    //   block [0x825B2AF4..0x825B2B04)
	// 825B2AF4: FF006000  fcmpu cr6, f0, f12
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[12].f64);
	// 825B2AF8: 4199000C  bgt cr6, 0x825b2b04
	if ctx.cr[6].gt {
		sub_825B2B04(ctx, base);
		return;
	}
	// 825B2AFC: D1240008  stfs f9, 8(r4)
	tmp.f32 = (ctx.f[9].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 825B2B00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B2B04(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x825B2B04 size=8
    let mut pc: u32 = 0x825B2B04;
    'dispatch: loop {
        match pc {
            0x825B2B04 => {
    //   block [0x825B2B04..0x825B2B0C)
	// 825B2B04: D1440008  stfs f10, 8(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[10].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 825B2B08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B2B10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x825B2B10 size=24
    let mut pc: u32 = 0x825B2B10;
    'dispatch: loop {
        match pc {
            0x825B2B10 => {
    //   block [0x825B2B10..0x825B2B28)
	// 825B2B10: 396300D0  addi r11, r3, 0xd0
	ctx.r[11].s64 = ctx.r[3].s64 + 208;
	// 825B2B14: 13E020C7  vcmpequd (lvx128) v31, v0, v4
	tmp.u32 = ctx.r[4].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B2B28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825B2B28 size=20
    let mut pc: u32 = 0x825B2B28;
    'dispatch: loop {
        match pc {
            0x825B2B28 => {
    //   block [0x825B2B28..0x825B2B3C)
	// 825B2B28: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B2B2C: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 825B2B30: 816A000C  lwz r11, 0xc(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 825B2B34: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825B2B38: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B2B3C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825B2B3C size=4
    let mut pc: u32 = 0x825B2B3C;
    'dispatch: loop {
        match pc {
            0x825B2B3C => {
    //   block [0x825B2B3C..0x825B2B40)
	// 825B2B3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B2B40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825B2B40 size=108
    let mut pc: u32 = 0x825B2B40;
    'dispatch: loop {
        match pc {
            0x825B2B40 => {
    //   block [0x825B2B40..0x825B2BAC)
	// 825B2B40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B2B44: 48BF5629  bl 0x831a816c
	ctx.lr = 0x825B2B48;
	sub_831A8130(ctx, base);
	// 825B2B48: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B2B4C: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 825B2B50: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 825B2B54: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 825B2B58: 57CB063F  clrlwi. r11, r30, 0x18
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825B2B5C: 41820038  beq 0x825b2b94
	if ctx.cr[0].eq {
	pc = 0x825B2B94; continue 'dispatch;
	}
	// 825B2B60: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B2B64: 48BF6E25  bl 0x831a9988
	ctx.lr = 0x825B2B68;
	sub_831A9988(ctx, base);
	// 825B2B68: 3D608325  lis r11, -0x7cdb
	ctx.r[11].s64 = -2094727168;
	// 825B2B6C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 825B2B70: 386B5BA8  addi r3, r11, 0x5ba8
	ctx.r[3].s64 = ctx.r[11].s64 + 23464;
	// 825B2B74: 48BF5585  bl 0x831a80f8
	ctx.lr = 0x825B2B78;
	sub_831A80F8(ctx, base);
	// 825B2B78: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825B2B7C: 41820018  beq 0x825b2b94
	if ctx.cr[0].eq {
	pc = 0x825B2B94; continue 'dispatch;
	}
	// 825B2B80: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825B2B84: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 825B2B88: 4BFECC01  bl 0x8259f788
	ctx.lr = 0x825B2B8C;
	sub_8259F788(ctx, base);
	// 825B2B8C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 825B2B90: 48000014  b 0x825b2ba4
	pc = 0x825B2BA4; continue 'dispatch;
	// 825B2B94: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 825B2B98: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825B2B9C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 825B2BA0: 48BCB8E9  bl 0x8317e488
	ctx.lr = 0x825B2BA4;
	sub_8317E488(ctx, base);
	// 825B2BA4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825B2BA8: 48BF5614  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B2BB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825B2BB0 size=196
    let mut pc: u32 = 0x825B2BB0;
    'dispatch: loop {
        match pc {
            0x825B2BB0 => {
    //   block [0x825B2BB0..0x825B2C74)
	// 825B2BB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B2BB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825B2BB8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825B2BBC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825B2BC0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B2BC4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 825B2BC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825B2BCC: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 825B2BD0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 825B2BD4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825B2BD8: 4BD0DD61  bl 0x822c0938
	ctx.lr = 0x825B2BDC;
	sub_822C0938(ctx, base);
	// 825B2BDC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 825B2BE0: 41820028  beq 0x825b2c08
	if ctx.cr[0].eq {
	pc = 0x825B2C08; continue 'dispatch;
	}
	// 825B2BE4: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 825B2BE8: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 825B2BEC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825B2BF0: 392BBA0C  addi r9, r11, -0x45f4
	ctx.r[9].s64 = ctx.r[11].s64 + -17908;
	// 825B2BF4: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 825B2BF8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 825B2BFC: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 825B2C00: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 825B2C04: 48000008  b 0x825b2c0c
	pc = 0x825B2C0C; continue 'dispatch;
	// 825B2C08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825B2C0C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825B2C10: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825B2C14: 409A0044  bne cr6, 0x825b2c58
	if !ctx.cr[6].eq {
	pc = 0x825B2C58; continue 'dispatch;
	}
	// 825B2C18: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 825B2C1C: 419A001C  beq cr6, 0x825b2c38
	if ctx.cr[6].eq {
	pc = 0x825B2C38; continue 'dispatch;
	}
	// 825B2C20: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B2C24: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 825B2C28: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B2C2C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B2C30: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825B2C34: 4E800421  bctrl
	ctx.lr = 0x825B2C38;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825B2C38: 3D608329  lis r11, -0x7cd7
	ctx.r[11].s64 = -2094465024;
	// 825B2C3C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 825B2C40: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825B2C44: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 825B2C48: 816B8CE4  lwz r11, -0x731c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-29468 as u32) ) } as u64;
	// 825B2C4C: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 825B2C50: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825B2C54: 4BD0D3AD  bl 0x822c0000
	ctx.lr = 0x825B2C58;
	sub_822C0000(ctx, base);
	// 825B2C58: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825B2C5C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825B2C60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825B2C64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825B2C68: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825B2C6C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825B2C70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B2C78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825B2C78 size=196
    let mut pc: u32 = 0x825B2C78;
    'dispatch: loop {
        match pc {
            0x825B2C78 => {
    //   block [0x825B2C78..0x825B2D3C)
	// 825B2C78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B2C7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825B2C80: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825B2C84: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825B2C88: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B2C8C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 825B2C90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825B2C94: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 825B2C98: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 825B2C9C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825B2CA0: 4BD0DC99  bl 0x822c0938
	ctx.lr = 0x825B2CA4;
	sub_822C0938(ctx, base);
	// 825B2CA4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 825B2CA8: 41820028  beq 0x825b2cd0
	if ctx.cr[0].eq {
	pc = 0x825B2CD0; continue 'dispatch;
	}
	// 825B2CAC: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 825B2CB0: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 825B2CB4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825B2CB8: 392BBA20  addi r9, r11, -0x45e0
	ctx.r[9].s64 = ctx.r[11].s64 + -17888;
	// 825B2CBC: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 825B2CC0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 825B2CC4: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 825B2CC8: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 825B2CCC: 48000008  b 0x825b2cd4
	pc = 0x825B2CD4; continue 'dispatch;
	// 825B2CD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825B2CD4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825B2CD8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825B2CDC: 409A0044  bne cr6, 0x825b2d20
	if !ctx.cr[6].eq {
	pc = 0x825B2D20; continue 'dispatch;
	}
	// 825B2CE0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 825B2CE4: 419A001C  beq cr6, 0x825b2d00
	if ctx.cr[6].eq {
	pc = 0x825B2D00; continue 'dispatch;
	}
	// 825B2CE8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B2CEC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 825B2CF0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B2CF4: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B2CF8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825B2CFC: 4E800421  bctrl
	ctx.lr = 0x825B2D00;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825B2D00: 3D608329  lis r11, -0x7cd7
	ctx.r[11].s64 = -2094465024;
	// 825B2D04: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 825B2D08: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825B2D0C: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 825B2D10: 816B8CE4  lwz r11, -0x731c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-29468 as u32) ) } as u64;
	// 825B2D14: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 825B2D18: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825B2D1C: 4BD0D2E5  bl 0x822c0000
	ctx.lr = 0x825B2D20;
	sub_822C0000(ctx, base);
	// 825B2D20: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825B2D24: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825B2D28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825B2D2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825B2D30: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825B2D34: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825B2D38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B2D40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x825B2D40 size=152
    let mut pc: u32 = 0x825B2D40;
    'dispatch: loop {
        match pc {
            0x825B2D40 => {
    //   block [0x825B2D40..0x825B2DD8)
	// 825B2D40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B2D44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825B2D48: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825B2D4C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B2D50: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825B2D54: 4BEB6A05  bl 0x82469758
	ctx.lr = 0x825B2D58;
	sub_82469758(ctx, base);
	// 825B2D58: 3D408208  lis r10, -0x7df8
	ctx.r[10].s64 = -2113404928;
	// 825B2D5C: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 825B2D60: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 825B2D64: 390BBA34  addi r8, r11, -0x45cc
	ctx.r[8].s64 = ctx.r[11].s64 + -17868;
	// 825B2D68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825B2D6C: 3CE08200  lis r7, -0x7e00
	ctx.r[7].s64 = -2113929216;
	// 825B2D70: C00A89AC  lfs f0, -0x7654(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-30292 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825B2D74: D01F00E0  stfs f0, 0xe0(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(224 as u32), tmp.u32 ) };
	// 825B2D78: 911F0000  stw r8, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 825B2D7C: 917F00F0  stw r11, 0xf0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(240 as u32), ctx.r[11].u32 ) };
	// 825B2D80: C00908A8  lfs f0, 0x8a8(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(2216 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825B2D84: 917F00F4  stw r11, 0xf4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(244 as u32), ctx.r[11].u32 ) };
	// 825B2D88: D01F0100  stfs f0, 0x100(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(256 as u32), tmp.u32 ) };
	// 825B2D8C: D01F0108  stfs f0, 0x108(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(264 as u32), tmp.u32 ) };
	// 825B2D90: 997F00F9  stb r11, 0xf9(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(249 as u32), ctx.r[11].u8 ) };
	// 825B2D94: C00708A4  lfs f0, 0x8a4(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825B2D98: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 825B2D9C: 997F0104  stb r11, 0x104(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(260 as u32), ctx.r[11].u8 ) };
	// 825B2DA0: 393F0118  addi r9, r31, 0x118
	ctx.r[9].s64 = ctx.r[31].s64 + 280;
	// 825B2DA4: 391F010C  addi r8, r31, 0x10c
	ctx.r[8].s64 = ctx.r[31].s64 + 268;
	// 825B2DA8: 7D6851AE  stbx r11, r8, r10
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[10].u32), ctx.r[11].u8) };
	// 825B2DAC: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 825B2DB0: D0090000  stfs f0, 0(r9)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 825B2DB4: 39290004  addi r9, r9, 4
	ctx.r[9].s64 = ctx.r[9].s64 + 4;
	// 825B2DB8: 2B0A0006  cmplwi cr6, r10, 6
	ctx.cr[6].compare_u32(ctx.r[10].u32, 6 as u32, &mut ctx.xer);
	// 825B2DBC: 4198FFEC  blt cr6, 0x825b2da8
	if ctx.cr[6].lt {
	pc = 0x825B2DA8; continue 'dispatch;
	}
	// 825B2DC0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B2DC4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825B2DC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825B2DCC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825B2DD0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825B2DD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B2DD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825B2DD8 size=100
    let mut pc: u32 = 0x825B2DD8;
    'dispatch: loop {
        match pc {
            0x825B2DD8 => {
    //   block [0x825B2DD8..0x825B2E3C)
	// 825B2DD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B2DDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825B2DE0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825B2DE4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B2DE8: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 825B2DEC: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 825B2DF0: 3BEB7BC0  addi r31, r11, 0x7bc0
	ctx.r[31].s64 = ctx.r[11].s64 + 31680;
	// 825B2DF4: 816A7BC4  lwz r11, 0x7bc4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(31684 as u32) ) } as u64;
	// 825B2DF8: 556907FF  clrlwi. r9, r11, 0x1f
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 825B2DFC: 40820028  bne 0x825b2e24
	if !ctx.cr[0].eq {
	pc = 0x825B2E24; continue 'dispatch;
	}
	// 825B2E00: 616B0001  ori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 | 1;
	// 825B2E04: 3D208204  lis r9, -0x7dfc
	ctx.r[9].s64 = -2113667072;
	// 825B2E08: 916A7BC4  stw r11, 0x7bc4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31684 as u32), ctx.r[11].u32 ) };
	// 825B2E0C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B2E10: 3889AA0C  addi r4, r9, -0x55f4
	ctx.r[4].s64 = ctx.r[9].s64 + -22004;
	// 825B2E14: 48840BF5  bl 0x82df3a08
	ctx.lr = 0x825B2E18;
	sub_82DF3A08(ctx, base);
	// 825B2E18: 3D608324  lis r11, -0x7cdc
	ctx.r[11].s64 = -2094792704;
	// 825B2E1C: 386BF750  addi r3, r11, -0x8b0
	ctx.r[3].s64 = ctx.r[11].s64 + -2224;
	// 825B2E20: 48BF56B9  bl 0x831a84d8
	ctx.lr = 0x825B2E24;
	sub_831A84D8(ctx, base);
	// 825B2E24: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B2E28: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825B2E2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825B2E30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825B2E34: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825B2E38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B2E40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x825B2E40 size=80
    let mut pc: u32 = 0x825B2E40;
    'dispatch: loop {
        match pc {
            0x825B2E40 => {
    //   block [0x825B2E40..0x825B2E90)
	// 825B2E40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B2E44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825B2E48: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825B2E4C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B2E50: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 825B2E54: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825B2E58: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825B2E5C: 808B167C  lwz r4, 0x167c(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(5756 as u32) ) } as u64;
	// 825B2E60: 4883ED99  bl 0x82df1bf8
	ctx.lr = 0x825B2E64;
	sub_82DF1BF8(ctx, base);
	// 825B2E64: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 825B2E68: 485943D9  bl 0x82b47240
	ctx.lr = 0x825B2E6C;
	sub_82B47240(ctx, base);
	// 825B2E6C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825B2E70: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825B2E74: 4883EE1D  bl 0x82df1c90
	ctx.lr = 0x825B2E78;
	sub_82DF1C90(ctx, base);
	// 825B2E78: C03F001C  lfs f1, 0x1c(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 825B2E7C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825B2E80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825B2E84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825B2E88: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825B2E8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B2E90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x825B2E90 size=80
    let mut pc: u32 = 0x825B2E90;
    'dispatch: loop {
        match pc {
            0x825B2E90 => {
    //   block [0x825B2E90..0x825B2EE0)
	// 825B2E90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B2E94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825B2E98: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825B2E9C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B2EA0: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 825B2EA4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825B2EA8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825B2EAC: 808B167C  lwz r4, 0x167c(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(5756 as u32) ) } as u64;
	// 825B2EB0: 4883ED49  bl 0x82df1bf8
	ctx.lr = 0x825B2EB4;
	sub_82DF1BF8(ctx, base);
	// 825B2EB4: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 825B2EB8: 48594389  bl 0x82b47240
	ctx.lr = 0x825B2EBC;
	sub_82B47240(ctx, base);
	// 825B2EBC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825B2EC0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825B2EC4: 4883EDCD  bl 0x82df1c90
	ctx.lr = 0x825B2EC8;
	sub_82DF1C90(ctx, base);
	// 825B2EC8: C03F0020  lfs f1, 0x20(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 825B2ECC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825B2ED0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825B2ED4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825B2ED8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825B2EDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B2EE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x825B2EE0 size=392
    let mut pc: u32 = 0x825B2EE0;
    'dispatch: loop {
        match pc {
            0x825B2EE0 => {
    //   block [0x825B2EE0..0x825B3068)
	// 825B2EE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B2EE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825B2EE8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825B2EEC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825B2EF0: DBC1FFD8  stfd f30, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[30].u64 ) };
	// 825B2EF4: DBE1FFE0  stfd f31, -0x20(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 825B2EF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B2EFC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825B2F00: 38800800  li r4, 0x800
	ctx.r[4].s64 = 2048;
	// 825B2F04: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825B2F08: 4BECE019  bl 0x82480f20
	ctx.lr = 0x825B2F0C;
	sub_82480F20(ctx, base);
	// 825B2F0C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825B2F10: 41820070  beq 0x825b2f80
	if ctx.cr[0].eq {
	pc = 0x825B2F80; continue 'dispatch;
	}
	// 825B2F14: 3D408208  lis r10, -0x7df8
	ctx.r[10].s64 = -2113404928;
	// 825B2F18: C01F00EC  lfs f0, 0xec(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(236 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825B2F1C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 825B2F20: C1AA89AC  lfs f13, -0x7654(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-30292 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825B2F24: C3EB08A8  lfs f31, 0x8a8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2216 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 825B2F28: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 825B2F2C: 41980010  blt cr6, 0x825b2f3c
	if ctx.cr[6].lt {
	pc = 0x825B2F3C; continue 'dispatch;
	}
	// 825B2F30: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 825B2F34: C1AB964C  lfs f13, -0x69b4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27060 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825B2F38: EFE00372  fmuls f31, f0, f13
	ctx.f[31].f64 = (((ctx.f[0].f64 * ctx.f[13].f64) as f32) as f64);
	// 825B2F3C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B2F40: 4BFFFF51  bl 0x825b2e90
	ctx.lr = 0x825B2F44;
	sub_825B2E90(ctx, base);
	// 825B2F44: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B2F48: FFC00890  fmr f30, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[30].f64 = ctx.f[1].f64;
	// 825B2F4C: 488A682D  bl 0x82e59778
	ctx.lr = 0x825B2F50;
	sub_82E59778(ctx, base);
	// 825B2F50: EDBE0072  fmuls f13, f30, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[13].f64 = (((ctx.f[30].f64 * ctx.f[1].f64) as f32) as f64);
	// 825B2F54: C19F00E0  lfs f12, 0xe0(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(224 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 825B2F58: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 825B2F5C: C17F00EC  lfs f11, 0xec(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(236 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 825B2F60: C00B9528  lfs f0, -0x6ad8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27352 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825B2F64: EDAD0332  fmuls f13, f13, f12
	ctx.f[13].f64 = (((ctx.f[13].f64 * ctx.f[12].f64) as f32) as f64);
	// 825B2F68: EDAD5FFC  fnmsubs f13, f13, f31, f11
	ctx.f[13].f64 = -(((ctx.f[13].f64 * ctx.f[31].f64 - ctx.f[11].f64) as f32) as f64);
	// 825B2F6C: D1BF00EC  stfs f13, 0xec(r31)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(236 as u32), tmp.u32 ) };
	// 825B2F70: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 825B2F74: 409800A0  bge cr6, 0x825b3014
	if !ctx.cr[6].lt {
	pc = 0x825B3014; continue 'dispatch;
	}
	// 825B2F78: D01F00EC  stfs f0, 0xec(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(236 as u32), tmp.u32 ) };
	// 825B2F7C: 48000098  b 0x825b3014
	pc = 0x825B3014; continue 'dispatch;
	// 825B2F80: 57CB063F  clrlwi. r11, r30, 0x18
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825B2F84: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B2F88: 41820038  beq 0x825b2fc0
	if ctx.cr[0].eq {
	pc = 0x825B2FC0; continue 'dispatch;
	}
	// 825B2F8C: 4BFFFEB5  bl 0x825b2e40
	ctx.lr = 0x825B2F90;
	sub_825B2E40(ctx, base);
	// 825B2F90: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B2F94: FFE00890  fmr f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].f64 = ctx.f[1].f64;
	// 825B2F98: 488A67E1  bl 0x82e59778
	ctx.lr = 0x825B2F9C;
	sub_82E59778(ctx, base);
	// 825B2F9C: EDBF0072  fmuls f13, f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[13].f64 = (((ctx.f[31].f64 * ctx.f[1].f64) as f32) as f64);
	// 825B2FA0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 825B2FA4: C19F00E8  lfs f12, 0xe8(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(232 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 825B2FA8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B2FAC: C00BA1C4  lfs f0, -0x5e3c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-24124 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825B2FB0: EC0D603C  fnmsubs f0, f13, f0, f12
	ctx.f[0].f64 = -(((ctx.f[13].f64 * ctx.f[0].f64 - ctx.f[12].f64) as f32) as f64);
	// 825B2FB4: D01F00E8  stfs f0, 0xe8(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(232 as u32), tmp.u32 ) };
	// 825B2FB8: 4BFFFED9  bl 0x825b2e90
	ctx.lr = 0x825B2FBC;
	sub_825B2E90(ctx, base);
	// 825B2FBC: 48000034  b 0x825b2ff0
	pc = 0x825B2FF0; continue 'dispatch;
	// 825B2FC0: 4BFECCE1  bl 0x8259fca0
	ctx.lr = 0x825B2FC4;
	sub_8259FCA0(ctx, base);
	// 825B2FC4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B2FC8: FFE00890  fmr f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].f64 = ctx.f[1].f64;
	// 825B2FCC: 488A67AD  bl 0x82e59778
	ctx.lr = 0x825B2FD0;
	sub_82E59778(ctx, base);
	// 825B2FD0: EDBF0072  fmuls f13, f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[13].f64 = (((ctx.f[31].f64 * ctx.f[1].f64) as f32) as f64);
	// 825B2FD4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 825B2FD8: C19F00E8  lfs f12, 0xe8(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(232 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 825B2FDC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B2FE0: C00BA1C4  lfs f0, -0x5e3c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-24124 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825B2FE4: EC0D603C  fnmsubs f0, f13, f0, f12
	ctx.f[0].f64 = -(((ctx.f[13].f64 * ctx.f[0].f64 - ctx.f[12].f64) as f32) as f64);
	// 825B2FE8: D01F00E8  stfs f0, 0xe8(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(232 as u32), tmp.u32 ) };
	// 825B2FEC: 4BECDFA5  bl 0x82480f90
	ctx.lr = 0x825B2FF0;
	sub_82480F90(ctx, base);
	// 825B2FF0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B2FF4: FFE00890  fmr f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].f64 = ctx.f[1].f64;
	// 825B2FF8: 488A6781  bl 0x82e59778
	ctx.lr = 0x825B2FFC;
	sub_82E59778(ctx, base);
	// 825B2FFC: EDBF0072  fmuls f13, f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[13].f64 = (((ctx.f[31].f64 * ctx.f[1].f64) as f32) as f64);
	// 825B3000: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 825B3004: C19F00E4  lfs f12, 0xe4(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(228 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 825B3008: C00B7BC4  lfs f0, 0x7bc4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(31684 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825B300C: EC0D603C  fnmsubs f0, f13, f0, f12
	ctx.f[0].f64 = -(((ctx.f[13].f64 * ctx.f[0].f64 - ctx.f[12].f64) as f32) as f64);
	// 825B3010: D01F00E4  stfs f0, 0xe4(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(228 as u32), tmp.u32 ) };
	// 825B3014: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 825B3018: C01F00E4  lfs f0, 0xe4(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(228 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825B301C: C1AB9584  lfs f13, -0x6a7c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27260 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825B3020: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 825B3024: 41990014  bgt cr6, 0x825b3038
	if ctx.cr[6].gt {
	pc = 0x825B3038; continue 'dispatch;
	}
	// 825B3028: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 825B302C: C1AB78A8  lfs f13, 0x78a8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(30888 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825B3030: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 825B3034: 40980008  bge cr6, 0x825b303c
	if !ctx.cr[6].lt {
	pc = 0x825B303C; continue 'dispatch;
	}
	// 825B3038: FC006890  fmr f0, f13
	ctx.f[0].f64 = ctx.f[13].f64;
	// 825B303C: D01F00E4  stfs f0, 0xe4(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(228 as u32), tmp.u32 ) };
	// 825B3040: 387F00E8  addi r3, r31, 0xe8
	ctx.r[3].s64 = ctx.r[31].s64 + 232;
	// 825B3044: 4BEE4DD5  bl 0x82497e18
	ctx.lr = 0x825B3048;
	sub_82497E18(ctx, base);
	// 825B3048: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825B304C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825B3050: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825B3054: CBC1FFD8  lfd f30, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-40 as u32) ) };
	// 825B3058: CBE1FFE0  lfd f31, -0x20(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 825B305C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825B3060: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825B3064: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B3068(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x825B3068 size=1852
    let mut pc: u32 = 0x825B3068;
    'dispatch: loop {
        match pc {
            0x825B3068 => {
    //   block [0x825B3068..0x825B37A4)
	// 825B3068: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B306C: 48BF5101  bl 0x831a816c
	ctx.lr = 0x825B3070;
	sub_831A8130(ctx, base);
	// 825B3070: DBC1FFD0  stfd f30, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[30].u64 ) };
	// 825B3074: DBE1FFD8  stfd f31, -0x28(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 825B3078: 9421FE00  stwu r1, -0x200(r1)
	ea = ctx.r[1].u32.wrapping_add(-512 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B307C: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 825B3080: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825B3084: 396B6910  addi r11, r11, 0x6910
	ctx.r[11].s64 = ctx.r[11].s64 + 26896;
	// 825B3088: 39210060  addi r9, r1, 0x60
	ctx.r[9].s64 = ctx.r[1].s64 + 96;
	// 825B308C: 39010050  addi r8, r1, 0x50
	ctx.r[8].s64 = ctx.r[1].s64 + 80;
	// 825B3090: 895F0104  lbz r10, 0x104(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(260 as u32) ) } as u64;
	// 825B3094: 13E058C7  vcmpequd (lvx128) v31, v0, v11
	tmp.u32 = ctx.r[11].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 825B3098: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B37A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825B37A8 size=112
    let mut pc: u32 = 0x825B37A8;
    'dispatch: loop {
        match pc {
            0x825B37A8 => {
    //   block [0x825B37A8..0x825B3818)
	// 825B37A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B37AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825B37B0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825B37B4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825B37B8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B37BC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825B37C0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825B37C4: 93C10050  stw r30, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 825B37C8: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 825B37CC: 4BFFF3E5  bl 0x825b2bb0
	ctx.lr = 0x825B37D0;
	sub_825B2BB0(ctx, base);
	// 825B37D0: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 825B37D4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 825B37D8: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 825B37DC: 4BD0C825  bl 0x822c0000
	ctx.lr = 0x825B37E0;
	sub_822C0000(ctx, base);
	// 825B37E0: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 825B37E4: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 825B37E8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825B37EC: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 825B37F0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825B37F4: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 825B37F8: 419A0008  beq cr6, 0x825b3800
	if ctx.cr[6].eq {
	pc = 0x825B3800; continue 'dispatch;
	}
	// 825B37FC: 4BD0D095  bl 0x822c0890
	ctx.lr = 0x825B3800;
	sub_822C0890(ctx, base);
	// 825B3800: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825B3804: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825B3808: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825B380C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825B3810: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825B3814: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B3818(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825B3818 size=112
    let mut pc: u32 = 0x825B3818;
    'dispatch: loop {
        match pc {
            0x825B3818 => {
    //   block [0x825B3818..0x825B3888)
	// 825B3818: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B381C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825B3820: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825B3824: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825B3828: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B382C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825B3830: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825B3834: 93C10050  stw r30, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 825B3838: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 825B383C: 4BFFF43D  bl 0x825b2c78
	ctx.lr = 0x825B3840;
	sub_825B2C78(ctx, base);
	// 825B3840: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 825B3844: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 825B3848: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 825B384C: 4BD0C7B5  bl 0x822c0000
	ctx.lr = 0x825B3850;
	sub_822C0000(ctx, base);
	// 825B3850: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 825B3854: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 825B3858: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825B385C: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 825B3860: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825B3864: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 825B3868: 419A0008  beq cr6, 0x825b3870
	if ctx.cr[6].eq {
	pc = 0x825B3870; continue 'dispatch;
	}
	// 825B386C: 4BD0D025  bl 0x822c0890
	ctx.lr = 0x825B3870;
	sub_822C0890(ctx, base);
	// 825B3870: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825B3874: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825B3878: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825B387C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825B3880: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825B3884: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B3888(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x825B3888 size=300
    let mut pc: u32 = 0x825B3888;
    'dispatch: loop {
        match pc {
            0x825B3888 => {
    //   block [0x825B3888..0x825B39B4)
	// 825B3888: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B388C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825B3890: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825B3894: DBC1FFE0  stfd f30, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[30].u64 ) };
	// 825B3898: DBE1FFE8  stfd f31, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.f[31].u64 ) };
	// 825B389C: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B38A0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825B38A4: 48BA0D5D  bl 0x83154600
	ctx.lr = 0x825B38A8;
	sub_83154600(ctx, base);
	// 825B38A8: 897F00F9  lbz r11, 0xf9(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(249 as u32) ) } as u64;
	// 825B38AC: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 825B38B0: 40820020  bne 0x825b38d0
	if !ctx.cr[0].eq {
	pc = 0x825B38D0; continue 'dispatch;
	}
	// 825B38B4: 38800800  li r4, 0x800
	ctx.r[4].s64 = 2048;
	// 825B38B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B38BC: 4BECD665  bl 0x82480f20
	ctx.lr = 0x825B38C0;
	sub_82480F20(ctx, base);
	// 825B38C0: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825B38C4: 4082000C  bne 0x825b38d0
	if !ctx.cr[0].eq {
	pc = 0x825B38D0; continue 'dispatch;
	}
	// 825B38C8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B38CC: 4BFFF79D  bl 0x825b3068
	ctx.lr = 0x825B38D0;
	sub_825B3068(ctx, base);
	// 825B38D0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 825B38D4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B38D8: 4BFFF609  bl 0x825b2ee0
	ctx.lr = 0x825B38DC;
	sub_825B2EE0(ctx, base);
	// 825B38DC: 38800008  li r4, 8
	ctx.r[4].s64 = 8;
	// 825B38E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B38E4: 4BECD63D  bl 0x82480f20
	ctx.lr = 0x825B38E8;
	sub_82480F20(ctx, base);
	// 825B38E8: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825B38EC: 41820010  beq 0x825b38fc
	if ctx.cr[0].eq {
	pc = 0x825B38FC; continue 'dispatch;
	}
	// 825B38F0: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 825B38F4: C00B08AC  lfs f0, 0x8ac(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2220 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825B38F8: 4800000C  b 0x825b3904
	pc = 0x825B3904; continue 'dispatch;
	// 825B38FC: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 825B3900: C00B89AC  lfs f0, -0x7654(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-30292 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825B3904: D01F00E0  stfs f0, 0xe0(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(224 as u32), tmp.u32 ) };
	// 825B3908: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825B390C: 38610090  addi r3, r1, 0x90
	ctx.r[3].s64 = ctx.r[1].s64 + 144;
	// 825B3910: C07F00E4  lfs f3, 0xe4(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(228 as u32) ) };
	ctx.f[3].f64 = (tmp.f32 as f64);
	// 825B3914: C05F00E8  lfs f2, 0xe8(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(232 as u32) ) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 825B3918: C03F00EC  lfs f1, 0xec(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(236 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 825B391C: 4BFEBD45  bl 0x8259f660
	ctx.lr = 0x825B3920;
	sub_8259F660(ctx, base);
	// 825B3920: 396000D0  li r11, 0xd0
	ctx.r[11].s64 = 208;
	// 825B3924: 13E018C7  vcmpequd (lvx128) v31, v0, v3
	tmp.u32 = ctx.r[3].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 825B3928: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 825B392C: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 825B3930: 39010080  addi r8, r1, 0x80
	ctx.r[8].s64 = ctx.r[1].s64 + 128;
	// 825B3934: 38E10070  addi r7, r1, 0x70
	ctx.r[7].s64 = ctx.r[1].s64 + 112;
	// 825B3938: 13DF58C7  vcmpequd (lvx128) v30, v31, v11
	tmp.u32 = ctx.r[31].u32 + ctx.r[11].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[62] using VectorMaskL[(tmp.u32 & 0xF)]
	// 825B393C: 38C10070  addi r6, r1, 0x70
	ctx.r[6].s64 = ctx.r[1].s64 + 112;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B39B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825B39B8 size=112
    let mut pc: u32 = 0x825B39B8;
    'dispatch: loop {
        match pc {
            0x825B39B8 => {
    //   block [0x825B39B8..0x825B3A28)
	// 825B39B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B39BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825B39C0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825B39C4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B39C8: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 825B39CC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825B39D0: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825B39D4: 388BBA68  addi r4, r11, -0x4598
	ctx.r[4].s64 = ctx.r[11].s64 + -17816;
	// 825B39D8: 38A001DE  li r5, 0x1de
	ctx.r[5].s64 = 478;
	// 825B39DC: 38600004  li r3, 4
	ctx.r[3].s64 = 4;
	// 825B39E0: 4BD0C9F9  bl 0x822c03d8
	ctx.lr = 0x825B39E4;
	sub_822C03D8(ctx, base);
	// 825B39E4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 825B39E8: 41820018  beq 0x825b3a00
	if ctx.cr[0].eq {
	pc = 0x825B3A00; continue 'dispatch;
	}
	// 825B39EC: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 825B39F0: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 825B39F4: 396BB9F4  addi r11, r11, -0x460c
	ctx.r[11].s64 = ctx.r[11].s64 + -17932;
	// 825B39F8: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825B39FC: 48000008  b 0x825b3a04
	pc = 0x825B3A04; continue 'dispatch;
	// 825B3A00: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 825B3A04: 387F00F0  addi r3, r31, 0xf0
	ctx.r[3].s64 = ctx.r[31].s64 + 240;
	// 825B3A08: 4BFFFE11  bl 0x825b3818
	ctx.lr = 0x825B3A0C;
	sub_825B3818(ctx, base);
	// 825B3A0C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825B3A10: 997F00F8  stb r11, 0xf8(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(248 as u32), ctx.r[11].u8 ) };
	// 825B3A14: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825B3A18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825B3A1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825B3A20: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825B3A24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B3A28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x825B3A28 size=372
    let mut pc: u32 = 0x825B3A28;
    'dispatch: loop {
        match pc {
            0x825B3A28 => {
    //   block [0x825B3A28..0x825B3B9C)
	// 825B3A28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B3A2C: 48BF4735  bl 0x831a8160
	ctx.lr = 0x825B3A30;
	sub_831A8130(ctx, base);
	// 825B3A30: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B3A34: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 825B3A38: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 825B3A3C: 93C10050  stw r30, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 825B3A40: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 825B3A44: 3B8BBA68  addi r28, r11, -0x4598
	ctx.r[28].s64 = ctx.r[11].s64 + -17816;
	// 825B3A48: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825B3A4C: 38A001CB  li r5, 0x1cb
	ctx.r[5].s64 = 459;
	// 825B3A50: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 825B3A54: 386000E0  li r3, 0xe0
	ctx.r[3].s64 = 224;
	// 825B3A58: 4883E991  bl 0x82df23e8
	ctx.lr = 0x825B3A5C;
	sub_82DF23E8(ctx, base);
	// 825B3A5C: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 825B3A60: 41820054  beq 0x825b3ab4
	if ctx.cr[0].eq {
	pc = 0x825B3AB4; continue 'dispatch;
	}
	// 825B3A64: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 825B3A68: 3BC00003  li r30, 3
	ctx.r[30].s64 = 3;
	// 825B3A6C: 48BA0B95  bl 0x83154600
	ctx.lr = 0x825B3A70;
	sub_83154600(ctx, base);
	// 825B3A70: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 825B3A74: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 825B3A78: 48BA0B89  bl 0x83154600
	ctx.lr = 0x825B3A7C;
	sub_83154600(ctx, base);
	// 825B3A7C: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 825B3A80: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 825B3A84: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 825B3A88: 4BF5BA41  bl 0x8250f4c8
	ctx.lr = 0x825B3A8C;
	sub_8250F4C8(ctx, base);
	// 825B3A8C: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 825B3A90: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 825B3A94: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 825B3A98: 4BF5BA81  bl 0x8250f518
	ctx.lr = 0x825B3A9C;
	sub_8250F518(ctx, base);
	// 825B3A9C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 825B3AA0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B3AA4: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 825B3AA8: 4BD314F9  bl 0x822e4fa0
	ctx.lr = 0x825B3AAC;
	sub_822E4FA0(ctx, base);
	// 825B3AAC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825B3AB0: 48000008  b 0x825b3ab8
	pc = 0x825B3AB8; continue 'dispatch;
	// 825B3AB4: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 825B3AB8: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 825B3ABC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825B3AC0: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 825B3AC4: 4BFEBDAD  bl 0x8259f870
	ctx.lr = 0x825B3AC8;
	sub_8259F870(ctx, base);
	// 825B3AC8: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 825B3ACC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825B3AD0: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 825B3AD4: 4BD0C52D  bl 0x822c0000
	ctx.lr = 0x825B3AD8;
	sub_822C0000(ctx, base);
	// 825B3AD8: 57CB07BD  rlwinm. r11, r30, 0, 0x1e, 0x1e
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825B3ADC: 41820010  beq 0x825b3aec
	if ctx.cr[0].eq {
	pc = 0x825B3AEC; continue 'dispatch;
	}
	// 825B3AE0: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 825B3AE4: 57DE07FA  rlwinm r30, r30, 0, 0x1f, 0x1d
	ctx.r[30].u64 = ctx.r[30].u32 as u64 & 0xFFFFFFFFu64;
	// 825B3AE8: 4883E1A9  bl 0x82df1c90
	ctx.lr = 0x825B3AEC;
	sub_82DF1C90(ctx, base);
	// 825B3AEC: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825B3AF0: 4182000C  beq 0x825b3afc
	if ctx.cr[0].eq {
	pc = 0x825B3AFC; continue 'dispatch;
	}
	// 825B3AF4: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 825B3AF8: 4883E199  bl 0x82df1c90
	ctx.lr = 0x825B3AFC;
	sub_82DF1C90(ctx, base);
	// 825B3AFC: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 825B3B00: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 825B3B04: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 825B3B08: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825B3B0C: 38A10070  addi r5, r1, 0x70
	ctx.r[5].s64 = ctx.r[1].s64 + 112;
	// 825B3B10: 389D00D0  addi r4, r29, 0xd0
	ctx.r[4].s64 = ctx.r[29].s64 + 208;
	// 825B3B14: C00B08A4  lfs f0, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825B3B18: C1AA8494  lfs f13, -0x7b6c(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-31596 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825B3B1C: D0010070  stfs f0, 0x70(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), tmp.u32 ) };
	// 825B3B20: D1A10074  stfs f13, 0x74(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), tmp.u32 ) };
	// 825B3B24: D0010078  stfs f0, 0x78(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), tmp.u32 ) };
	// 825B3B28: D001007C  stfs f0, 0x7c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), tmp.u32 ) };
	// 825B3B2C: 4BD30DA5  bl 0x822e48d0
	ctx.lr = 0x825B3B30;
	sub_822E48D0(ctx, base);
	// 825B3B30: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825B3B34: 41820048  beq 0x825b3b7c
	if ctx.cr[0].eq {
	pc = 0x825B3B7C; continue 'dispatch;
	}
	// 825B3B38: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 825B3B3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825B3B40: 38A001CD  li r5, 0x1cd
	ctx.r[5].s64 = 461;
	// 825B3B44: 3860000C  li r3, 0xc
	ctx.r[3].s64 = 12;
	// 825B3B48: 4BD0C891  bl 0x822c03d8
	ctx.lr = 0x825B3B4C;
	sub_822C03D8(ctx, base);
	// 825B3B4C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 825B3B50: 41820014  beq 0x825b3b64
	if ctx.cr[0].eq {
	pc = 0x825B3B64; continue 'dispatch;
	}
	// 825B3B54: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 825B3B58: 480064C9  bl 0x825ba020
	ctx.lr = 0x825B3B5C;
	sub_825BA020(ctx, base);
	// 825B3B5C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 825B3B60: 48000008  b 0x825b3b68
	pc = 0x825B3B68; continue 'dispatch;
	// 825B3B64: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 825B3B68: 387D00F0  addi r3, r29, 0xf0
	ctx.r[3].s64 = ctx.r[29].s64 + 240;
	// 825B3B6C: 4BFFFC3D  bl 0x825b37a8
	ctx.lr = 0x825B3B70;
	sub_825B37A8(ctx, base);
	// 825B3B70: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 825B3B74: 997D00F8  stb r11, 0xf8(r29)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[29].u32.wrapping_add(248 as u32), ctx.r[11].u8 ) };
	// 825B3B78: 4800000C  b 0x825b3b84
	pc = 0x825B3B84; continue 'dispatch;
	// 825B3B7C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 825B3B80: 4BFFFE39  bl 0x825b39b8
	ctx.lr = 0x825B3B84;
	sub_825B39B8(ctx, base);
	// 825B3B84: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 825B3B88: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825B3B8C: 419A0008  beq cr6, 0x825b3b94
	if ctx.cr[6].eq {
	pc = 0x825B3B94; continue 'dispatch;
	}
	// 825B3B90: 4BD0CD01  bl 0x822c0890
	ctx.lr = 0x825B3B94;
	sub_822C0890(ctx, base);
	// 825B3B94: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 825B3B98: 48BF4618  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B3BA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x825B3BA0 size=368
    let mut pc: u32 = 0x825B3BA0;
    'dispatch: loop {
        match pc {
            0x825B3BA0 => {
    //   block [0x825B3BA0..0x825B3D10)
	// 825B3BA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B3BA4: 48BF45C1  bl 0x831a8164
	ctx.lr = 0x825B3BA8;
	sub_831A8130(ctx, base);
	// 825B3BA8: 9421FF10  stwu r1, -0xf0(r1)
	ea = ctx.r[1].u32.wrapping_add(-240 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B3BAC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 825B3BB0: 48BA0A51  bl 0x83154600
	ctx.lr = 0x825B3BB4;
	sub_83154600(ctx, base);
	// 825B3BB4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825B3BB8: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 825B3BBC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825B3BC0: 4BEB11C9  bl 0x82464d88
	ctx.lr = 0x825B3BC4;
	sub_82464D88(ctx, base);
	// 825B3BC4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825B3BC8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825B3BCC: 4BEB11BD  bl 0x82464d88
	ctx.lr = 0x825B3BD0;
	sub_82464D88(ctx, base);
	// 825B3BD0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 825B3BD4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825B3BD8: 386100A0  addi r3, r1, 0xa0
	ctx.r[3].s64 = ctx.r[1].s64 + 160;
	// 825B3BDC: 4BEB11BD  bl 0x82464d98
	ctx.lr = 0x825B3BE0;
	sub_82464D98(ctx, base);
	// 825B3BE0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 825B3BE4: 13E0E8C7  vcmpequd (lvx128) v31, v0, v29
	tmp.u32 = ctx.r[29].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 825B3BE8: 39410080  addi r10, r1, 0x80
	ctx.r[10].s64 = ctx.r[1].s64 + 128;
	// 825B3BEC: 38810080  addi r4, r1, 0x80
	ctx.r[4].s64 = ctx.r[1].s64 + 128;
	// 825B3BF0: 386100B0  addi r3, r1, 0xb0
	ctx.r[3].s64 = ctx.r[1].s64 + 176;
	// 825B3BF4: 13C058C7  vcmpequd (lvx128) v30, v0, v11
	tmp.u32 = ctx.r[11].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[62] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B3D10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x825B3D10 size=44
    let mut pc: u32 = 0x825B3D10;
    'dispatch: loop {
        match pc {
            0x825B3D10 => {
    //   block [0x825B3D10..0x825B3D3C)
	// 825B3D10: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 825B3D14: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 825B3D18: 3D208208  lis r9, -0x7df8
	ctx.r[9].s64 = -2113404928;
	// 825B3D1C: C1AB08A8  lfs f13, 0x8a8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2216 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825B3D20: C00A08A4  lfs f0, 0x8a4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825B3D24: C1896150  lfs f12, 0x6150(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(24912 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 825B3D28: D1A300E4  stfs f13, 0xe4(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(228 as u32), tmp.u32 ) };
	// 825B3D2C: D00300E8  stfs f0, 0xe8(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(232 as u32), tmp.u32 ) };
	// 825B3D30: D00300EC  stfs f0, 0xec(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(236 as u32), tmp.u32 ) };
	// 825B3D34: D18300F0  stfs f12, 0xf0(r3)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(240 as u32), tmp.u32 ) };
	// 825B3D38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B3D40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x825B3D40 size=44
    let mut pc: u32 = 0x825B3D40;
    'dispatch: loop {
        match pc {
            0x825B3D40 => {
    //   block [0x825B3D40..0x825B3D6C)
	// 825B3D40: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 825B3D44: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 825B3D48: 3D208208  lis r9, -0x7df8
	ctx.r[9].s64 = -2113404928;
	// 825B3D4C: C00B08A8  lfs f0, 0x8a8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2216 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825B3D50: C1AA08A4  lfs f13, 0x8a4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2212 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825B3D54: C1896150  lfs f12, 0x6150(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(24912 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 825B3D58: D00300E4  stfs f0, 0xe4(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(228 as u32), tmp.u32 ) };
	// 825B3D5C: D00300E8  stfs f0, 0xe8(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(232 as u32), tmp.u32 ) };
	// 825B3D60: D1A300EC  stfs f13, 0xec(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(236 as u32), tmp.u32 ) };
	// 825B3D64: D18300F0  stfs f12, 0xf0(r3)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(240 as u32), tmp.u32 ) };
	// 825B3D68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B3D70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825B3D70 size=148
    let mut pc: u32 = 0x825B3D70;
    'dispatch: loop {
        match pc {
            0x825B3D70 => {
    //   block [0x825B3D70..0x825B3E04)
	// 825B3D70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B3D74: 48BF43F5  bl 0x831a8168
	ctx.lr = 0x825B3D78;
	sub_831A8130(ctx, base);
	// 825B3D78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B3D7C: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 825B3D80: 3BA00004  li r29, 4
	ctx.r[29].s64 = 4;
	// 825B3D84: 3BFC0028  addi r31, r28, 0x28
	ctx.r[31].s64 = ctx.r[28].s64 + 40;
	// 825B3D88: 397F0044  addi r11, r31, 0x44
	ctx.r[11].s64 = ctx.r[31].s64 + 68;
	// 825B3D8C: 3BCB0004  addi r30, r11, 4
	ctx.r[30].s64 = ctx.r[11].s64 + 4;
	// 825B3D90: 3BDEFFF8  addi r30, r30, -8
	ctx.r[30].s64 = ctx.r[30].s64 + -8;
	// 825B3D94: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B3D98: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825B3D9C: 419A0008  beq cr6, 0x825b3da4
	if ctx.cr[6].eq {
	pc = 0x825B3DA4; continue 'dispatch;
	}
	// 825B3DA0: 4BD0CAF1  bl 0x822c0890
	ctx.lr = 0x825B3DA4;
	sub_822C0890(ctx, base);
	// 825B3DA4: 37BDFFFF  addic. r29, r29, -1
	ctx.xer.ca = (ctx.r[29].u32 > (!(-1 as u32)));
	ctx.r[29].s64 = ctx.r[29].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 825B3DA8: 4080FFE8  bge 0x825b3d90
	if !ctx.cr[0].lt {
	pc = 0x825B3D90; continue 'dispatch;
	}
	// 825B3DAC: 397F001C  addi r11, r31, 0x1c
	ctx.r[11].s64 = ctx.r[31].s64 + 28;
	// 825B3DB0: 3BC00004  li r30, 4
	ctx.r[30].s64 = 4;
	// 825B3DB4: 3BEB0004  addi r31, r11, 4
	ctx.r[31].s64 = ctx.r[11].s64 + 4;
	// 825B3DB8: 3BFFFFF8  addi r31, r31, -8
	ctx.r[31].s64 = ctx.r[31].s64 + -8;
	// 825B3DBC: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B3DC0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825B3DC4: 419A0008  beq cr6, 0x825b3dcc
	if ctx.cr[6].eq {
	pc = 0x825B3DCC; continue 'dispatch;
	}
	// 825B3DC8: 4BD0CAC9  bl 0x822c0890
	ctx.lr = 0x825B3DCC;
	sub_822C0890(ctx, base);
	// 825B3DCC: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 825B3DD0: 4080FFE8  bge 0x825b3db8
	if !ctx.cr[0].lt {
	pc = 0x825B3DB8; continue 'dispatch;
	}
	// 825B3DD4: 807C0014  lwz r3, 0x14(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(20 as u32) ) } as u64;
	// 825B3DD8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825B3DDC: 419A0008  beq cr6, 0x825b3de4
	if ctx.cr[6].eq {
	pc = 0x825B3DE4; continue 'dispatch;
	}
	// 825B3DE0: 4BD0CAB1  bl 0x822c0890
	ctx.lr = 0x825B3DE4;
	sub_822C0890(ctx, base);
	// 825B3DE4: 807C000C  lwz r3, 0xc(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(12 as u32) ) } as u64;
	// 825B3DE8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825B3DEC: 419A0008  beq cr6, 0x825b3df4
	if ctx.cr[6].eq {
	pc = 0x825B3DF4; continue 'dispatch;
	}
	// 825B3DF0: 4BD0CAA1  bl 0x822c0890
	ctx.lr = 0x825B3DF4;
	sub_822C0890(ctx, base);
	// 825B3DF4: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 825B3DF8: 4885E0F9  bl 0x82e11ef0
	ctx.lr = 0x825B3DFC;
	sub_82E11EF0(ctx, base);
	// 825B3DFC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825B3E00: 48BF43B8  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B3E08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825B3E08 size=72
    let mut pc: u32 = 0x825B3E08;
    'dispatch: loop {
        match pc {
            0x825B3E08 => {
    //   block [0x825B3E08..0x825B3E50)
	// 825B3E08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B3E0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825B3E10: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825B3E14: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B3E18: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825B3E1C: 387F008C  addi r3, r31, 0x8c
	ctx.r[3].s64 = ctx.r[31].s64 + 140;
	// 825B3E20: 485394D9  bl 0x82aed2f8
	ctx.lr = 0x825B3E24;
	sub_82AED2F8(ctx, base);
	// 825B3E24: 807F0088  lwz r3, 0x88(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(136 as u32) ) } as u64;
	// 825B3E28: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825B3E2C: 419A0008  beq cr6, 0x825b3e34
	if ctx.cr[6].eq {
	pc = 0x825B3E34; continue 'dispatch;
	}
	// 825B3E30: 4BD0CA61  bl 0x822c0890
	ctx.lr = 0x825B3E34;
	sub_822C0890(ctx, base);
	// 825B3E34: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B3E38: 4BFFFF39  bl 0x825b3d70
	ctx.lr = 0x825B3E3C;
	sub_825B3D70(ctx, base);
	// 825B3E3C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825B3E40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825B3E44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825B3E48: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825B3E4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B3E50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825B3E50 size=196
    let mut pc: u32 = 0x825B3E50;
    'dispatch: loop {
        match pc {
            0x825B3E50 => {
    //   block [0x825B3E50..0x825B3F14)
	// 825B3E50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B3E54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825B3E58: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825B3E5C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825B3E60: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B3E64: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 825B3E68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825B3E6C: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 825B3E70: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 825B3E74: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825B3E78: 4BD0CAC1  bl 0x822c0938
	ctx.lr = 0x825B3E7C;
	sub_822C0938(ctx, base);
	// 825B3E7C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 825B3E80: 41820028  beq 0x825b3ea8
	if ctx.cr[0].eq {
	pc = 0x825B3EA8; continue 'dispatch;
	}
	// 825B3E84: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 825B3E88: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 825B3E8C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825B3E90: 392BBAD8  addi r9, r11, -0x4528
	ctx.r[9].s64 = ctx.r[11].s64 + -17704;
	// 825B3E94: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 825B3E98: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 825B3E9C: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 825B3EA0: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 825B3EA4: 48000008  b 0x825b3eac
	pc = 0x825B3EAC; continue 'dispatch;
	// 825B3EA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825B3EAC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825B3EB0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825B3EB4: 409A0044  bne cr6, 0x825b3ef8
	if !ctx.cr[6].eq {
	pc = 0x825B3EF8; continue 'dispatch;
	}
	// 825B3EB8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 825B3EBC: 419A001C  beq cr6, 0x825b3ed8
	if ctx.cr[6].eq {
	pc = 0x825B3ED8; continue 'dispatch;
	}
	// 825B3EC0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B3EC4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 825B3EC8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B3ECC: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B3ED0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825B3ED4: 4E800421  bctrl
	ctx.lr = 0x825B3ED8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825B3ED8: 3D608329  lis r11, -0x7cd7
	ctx.r[11].s64 = -2094465024;
	// 825B3EDC: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 825B3EE0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825B3EE4: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 825B3EE8: 816B8E80  lwz r11, -0x7180(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-29056 as u32) ) } as u64;
	// 825B3EEC: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 825B3EF0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825B3EF4: 4BD0C10D  bl 0x822c0000
	ctx.lr = 0x825B3EF8;
	sub_822C0000(ctx, base);
	// 825B3EF8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825B3EFC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825B3F00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825B3F04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825B3F08: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825B3F0C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825B3F10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B3F18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825B3F18 size=1024
    let mut pc: u32 = 0x825B3F18;
    'dispatch: loop {
        match pc {
            0x825B3F18 => {
    //   block [0x825B3F18..0x825B4318)
	// 825B3F18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B3F1C: 48BF423D  bl 0x831a8158
	ctx.lr = 0x825B3F20;
	sub_831A8130(ctx, base);
	// 825B3F20: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B3F24: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 825B3F28: 7C781B78  mr r24, r3
	ctx.r[24].u64 = ctx.r[3].u64;
	// 825B3F2C: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 825B3F30: 93E10104  stw r31, 0x104(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(260 as u32), ctx.r[31].u32 ) };
	// 825B3F34: 897F0039  lbz r11, 0x39(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(57 as u32) ) } as u64;
	// 825B3F38: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825B3F3C: 419A0048  beq cr6, 0x825b3f84
	if ctx.cr[6].eq {
	pc = 0x825B3F84; continue 'dispatch;
	}
	// 825B3F40: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 825B3F44: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825B3F48: 388B9620  addi r4, r11, -0x69e0
	ctx.r[4].s64 = ctx.r[11].s64 + -27104;
	// 825B3F4C: 4BD1197D  bl 0x822c58c8
	ctx.lr = 0x825B3F50;
	sub_822C58C8(ctx, base);
	// 825B3F50: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 825B3F54: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 825B3F58: 4BD15F59  bl 0x822c9eb0
	ctx.lr = 0x825B3F5C;
	sub_822C9EB0(ctx, base);
	// 825B3F5C: 4BD10355  bl 0x822c42b0
	ctx.lr = 0x825B3F60;
	sub_822C42B0(ctx, base);
	// 825B3F60: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 825B3F64: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 825B3F68: 396B9600  addi r11, r11, -0x6a00
	ctx.r[11].s64 = ctx.r[11].s64 + -27136;
	// 825B3F6C: 91610070  stw r11, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[11].u32 ) };
	// 825B3F70: 4BD11501  bl 0x822c5470
	ctx.lr = 0x825B3F74;
	sub_822C5470(ctx, base);
	// 825B3F74: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825B3F78: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 825B3F7C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825B3F80: 4BD10D61  bl 0x822c4ce0
	ctx.lr = 0x825B3F84;
	sub_822C4CE0(ctx, base);
	// 825B3F84: 38610104  addi r3, r1, 0x104
	ctx.r[3].s64 = ctx.r[1].s64 + 260;
	// 825B3F88: 7FFBFB78  mr r27, r31
	ctx.r[27].u64 = ctx.r[31].u64;
	// 825B3F8C: 48016855  bl 0x825ca7e0
	ctx.lr = 0x825B3F90;
	sub_825CA7E0(ctx, base);
	// 825B3F90: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B3F94: 894B0039  lbz r10, 0x39(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(57 as u32) ) } as u64;
	// 825B3F98: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 825B3F9C: 83210104  lwz r25, 0x104(r1)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(260 as u32) ) } as u64;
	// 825B3FA0: 419A000C  beq cr6, 0x825b3fac
	if ctx.cr[6].eq {
	pc = 0x825B3FAC; continue 'dispatch;
	}
	// 825B3FA4: 839B0008  lwz r28, 8(r27)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 825B3FA8: 48000028  b 0x825b3fd0
	pc = 0x825B3FD0; continue 'dispatch;
	// 825B3FAC: 815B0008  lwz r10, 8(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 825B3FB0: 894A0039  lbz r10, 0x39(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(57 as u32) ) } as u64;
	// 825B3FB4: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 825B3FB8: 419A000C  beq cr6, 0x825b3fc4
	if ctx.cr[6].eq {
	pc = 0x825B3FC4; continue 'dispatch;
	}
	// 825B3FBC: 7D7C5B78  mr r28, r11
	ctx.r[28].u64 = ctx.r[11].u64;
	// 825B3FC0: 48000010  b 0x825b3fd0
	pc = 0x825B3FD0; continue 'dispatch;
	// 825B3FC4: 83990008  lwz r28, 8(r25)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(8 as u32) ) } as u64;
	// 825B3FC8: 7F19D840  cmplw cr6, r25, r27
	ctx.cr[6].compare_u32(ctx.r[25].u32, ctx.r[27].u32, &mut ctx.xer);
	// 825B3FCC: 409A00DC  bne cr6, 0x825b40a8
	if !ctx.cr[6].eq {
	pc = 0x825B40A8; continue 'dispatch;
	}
	// 825B3FD0: 897C0039  lbz r11, 0x39(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(57 as u32) ) } as u64;
	// 825B3FD4: 83FB0004  lwz r31, 4(r27)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 825B3FD8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825B3FDC: 409A0008  bne cr6, 0x825b3fe4
	if !ctx.cr[6].eq {
	pc = 0x825B3FE4; continue 'dispatch;
	}
	// 825B3FE0: 93FC0004  stw r31, 4(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 825B3FE4: 817A0004  lwz r11, 4(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 825B3FE8: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 825B3FEC: 7F0AD840  cmplw cr6, r10, r27
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[27].u32, &mut ctx.xer);
	// 825B3FF0: 409A000C  bne cr6, 0x825b3ffc
	if !ctx.cr[6].eq {
	pc = 0x825B3FFC; continue 'dispatch;
	}
	// 825B3FF4: 938B0004  stw r28, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 825B3FF8: 4800001C  b 0x825b4014
	pc = 0x825B4014; continue 'dispatch;
	// 825B3FFC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B4000: 7F0BD840  cmplw cr6, r11, r27
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[27].u32, &mut ctx.xer);
	// 825B4004: 409A000C  bne cr6, 0x825b4010
	if !ctx.cr[6].eq {
	pc = 0x825B4010; continue 'dispatch;
	}
	// 825B4008: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 825B400C: 48000008  b 0x825b4014
	pc = 0x825B4014; continue 'dispatch;
	// 825B4010: 939F0008  stw r28, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[28].u32 ) };
	// 825B4014: 813A0004  lwz r9, 4(r26)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 825B4018: 81690000  lwz r11, 0(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B401C: 7F0BD840  cmplw cr6, r11, r27
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[27].u32, &mut ctx.xer);
	// 825B4020: 409A003C  bne cr6, 0x825b405c
	if !ctx.cr[6].eq {
	pc = 0x825B405C; continue 'dispatch;
	}
	// 825B4024: 897C0039  lbz r11, 0x39(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(57 as u32) ) } as u64;
	// 825B4028: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825B402C: 419A000C  beq cr6, 0x825b4038
	if ctx.cr[6].eq {
	pc = 0x825B4038; continue 'dispatch;
	}
	// 825B4030: 7FEAFB78  mr r10, r31
	ctx.r[10].u64 = ctx.r[31].u64;
	// 825B4034: 48000024  b 0x825b4058
	pc = 0x825B4058; continue 'dispatch;
	// 825B4038: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B403C: 7F8AE378  mr r10, r28
	ctx.r[10].u64 = ctx.r[28].u64;
	// 825B4040: 4800000C  b 0x825b404c
	pc = 0x825B404C; continue 'dispatch;
	// 825B4044: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 825B4048: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B404C: 890B0039  lbz r8, 0x39(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(57 as u32) ) } as u64;
	// 825B4050: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 825B4054: 419AFFF0  beq cr6, 0x825b4044
	if ctx.cr[6].eq {
	pc = 0x825B4044; continue 'dispatch;
	}
	// 825B4058: 91490000  stw r10, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 825B405C: 813A0004  lwz r9, 4(r26)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 825B4060: 81690008  lwz r11, 8(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) } as u64;
	// 825B4064: 7F0BD840  cmplw cr6, r11, r27
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[27].u32, &mut ctx.xer);
	// 825B4068: 409A00D4  bne cr6, 0x825b413c
	if !ctx.cr[6].eq {
	pc = 0x825B413C; continue 'dispatch;
	}
	// 825B406C: 897C0039  lbz r11, 0x39(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(57 as u32) ) } as u64;
	// 825B4070: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825B4074: 419A000C  beq cr6, 0x825b4080
	if ctx.cr[6].eq {
	pc = 0x825B4080; continue 'dispatch;
	}
	// 825B4078: 7FEAFB78  mr r10, r31
	ctx.r[10].u64 = ctx.r[31].u64;
	// 825B407C: 48000024  b 0x825b40a0
	pc = 0x825B40A0; continue 'dispatch;
	// 825B4080: 817C0008  lwz r11, 8(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 825B4084: 7F8AE378  mr r10, r28
	ctx.r[10].u64 = ctx.r[28].u64;
	// 825B4088: 4800000C  b 0x825b4094
	pc = 0x825B4094; continue 'dispatch;
	// 825B408C: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 825B4090: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 825B4094: 890B0039  lbz r8, 0x39(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(57 as u32) ) } as u64;
	// 825B4098: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 825B409C: 419AFFF0  beq cr6, 0x825b408c
	if ctx.cr[6].eq {
	pc = 0x825B408C; continue 'dispatch;
	}
	// 825B40A0: 91490008  stw r10, 8(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 825B40A4: 48000098  b 0x825b413c
	pc = 0x825B413C; continue 'dispatch;
	// 825B40A8: 932B0004  stw r25, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[25].u32 ) };
	// 825B40AC: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B40B0: 91790000  stw r11, 0(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825B40B4: 817B0008  lwz r11, 8(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 825B40B8: 7F195840  cmplw cr6, r25, r11
	ctx.cr[6].compare_u32(ctx.r[25].u32, ctx.r[11].u32, &mut ctx.xer);
	// 825B40BC: 409A000C  bne cr6, 0x825b40c8
	if !ctx.cr[6].eq {
	pc = 0x825B40C8; continue 'dispatch;
	}
	// 825B40C0: 7F3FCB78  mr r31, r25
	ctx.r[31].u64 = ctx.r[25].u64;
	// 825B40C4: 4800002C  b 0x825b40f0
	pc = 0x825B40F0; continue 'dispatch;
	// 825B40C8: 897C0039  lbz r11, 0x39(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(57 as u32) ) } as u64;
	// 825B40CC: 83F90004  lwz r31, 4(r25)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(4 as u32) ) } as u64;
	// 825B40D0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825B40D4: 409A0008  bne cr6, 0x825b40dc
	if !ctx.cr[6].eq {
	pc = 0x825B40DC; continue 'dispatch;
	}
	// 825B40D8: 93FC0004  stw r31, 4(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 825B40DC: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 825B40E0: 817B0008  lwz r11, 8(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 825B40E4: 91790008  stw r11, 8(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 825B40E8: 817B0008  lwz r11, 8(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 825B40EC: 932B0004  stw r25, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[25].u32 ) };
	// 825B40F0: 817A0004  lwz r11, 4(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 825B40F4: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 825B40F8: 7F0AD840  cmplw cr6, r10, r27
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[27].u32, &mut ctx.xer);
	// 825B40FC: 409A000C  bne cr6, 0x825b4108
	if !ctx.cr[6].eq {
	pc = 0x825B4108; continue 'dispatch;
	}
	// 825B4100: 932B0004  stw r25, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[25].u32 ) };
	// 825B4104: 48000020  b 0x825b4124
	pc = 0x825B4124; continue 'dispatch;
	// 825B4108: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 825B410C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B4110: 7F0AD840  cmplw cr6, r10, r27
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[27].u32, &mut ctx.xer);
	// 825B4114: 409A000C  bne cr6, 0x825b4120
	if !ctx.cr[6].eq {
	pc = 0x825B4120; continue 'dispatch;
	}
	// 825B4118: 932B0000  stw r25, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[25].u32 ) };
	// 825B411C: 48000008  b 0x825b4124
	pc = 0x825B4124; continue 'dispatch;
	// 825B4120: 932B0008  stw r25, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[25].u32 ) };
	// 825B4124: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 825B4128: 91790004  stw r11, 4(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 825B412C: 897B0038  lbz r11, 0x38(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[27].u32.wrapping_add(56 as u32) ) } as u64;
	// 825B4130: 89590038  lbz r10, 0x38(r25)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[25].u32.wrapping_add(56 as u32) ) } as u64;
	// 825B4134: 99790038  stb r11, 0x38(r25)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[25].u32.wrapping_add(56 as u32), ctx.r[11].u8 ) };
	// 825B4138: 995B0038  stb r10, 0x38(r27)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[27].u32.wrapping_add(56 as u32), ctx.r[10].u8 ) };
	// 825B413C: 897B0038  lbz r11, 0x38(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[27].u32.wrapping_add(56 as u32) ) } as u64;
	// 825B4140: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 825B4144: 409A0198  bne cr6, 0x825b42dc
	if !ctx.cr[6].eq {
	pc = 0x825B42DC; continue 'dispatch;
	}
	// 825B4148: 817A0004  lwz r11, 4(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 825B414C: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 825B4150: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 825B4154: 7F1C5840  cmplw cr6, r28, r11
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[11].u32, &mut ctx.xer);
	// 825B4158: 419A0180  beq cr6, 0x825b42d8
	if ctx.cr[6].eq {
	pc = 0x825B42D8; continue 'dispatch;
	}
	// 825B415C: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 825B4160: 897C0038  lbz r11, 0x38(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(56 as u32) ) } as u64;
	// 825B4164: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 825B4168: 409A0170  bne cr6, 0x825b42d8
	if !ctx.cr[6].eq {
	pc = 0x825B42D8; continue 'dispatch;
	}
	// 825B416C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B4170: 7F1C5840  cmplw cr6, r28, r11
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[11].u32, &mut ctx.xer);
	// 825B4174: 409A00A8  bne cr6, 0x825b421c
	if !ctx.cr[6].eq {
	pc = 0x825B421C; continue 'dispatch;
	}
	// 825B4178: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 825B417C: 894B0038  lbz r10, 0x38(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) } as u64;
	// 825B4180: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 825B4184: 409A001C  bne cr6, 0x825b41a0
	if !ctx.cr[6].eq {
	pc = 0x825B41A0; continue 'dispatch;
	}
	// 825B4188: 9BCB0038  stb r30, 0x38(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(56 as u32), ctx.r[30].u8 ) };
	// 825B418C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825B4190: 9BBF0038  stb r29, 0x38(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[29].u8 ) };
	// 825B4194: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 825B4198: 485F9C29  bl 0x82baddc0
	ctx.lr = 0x825B419C;
	sub_82BADDC0(ctx, base);
	// 825B419C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 825B41A0: 894B0039  lbz r10, 0x39(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(57 as u32) ) } as u64;
	// 825B41A4: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 825B41A8: 409A00C8  bne cr6, 0x825b4270
	if !ctx.cr[6].eq {
	pc = 0x825B4270; continue 'dispatch;
	}
	// 825B41AC: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B41B0: 894A0038  lbz r10, 0x38(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(56 as u32) ) } as u64;
	// 825B41B4: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 825B41B8: 409A0014  bne cr6, 0x825b41cc
	if !ctx.cr[6].eq {
	pc = 0x825B41CC; continue 'dispatch;
	}
	// 825B41BC: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 825B41C0: 894A0038  lbz r10, 0x38(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(56 as u32) ) } as u64;
	// 825B41C4: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 825B41C8: 419A00A4  beq cr6, 0x825b426c
	if ctx.cr[6].eq {
	pc = 0x825B426C; continue 'dispatch;
	}
	// 825B41CC: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 825B41D0: 894A0038  lbz r10, 0x38(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(56 as u32) ) } as u64;
	// 825B41D4: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 825B41D8: 409A0020  bne cr6, 0x825b41f8
	if !ctx.cr[6].eq {
	pc = 0x825B41F8; continue 'dispatch;
	}
	// 825B41DC: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B41E0: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 825B41E4: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 825B41E8: 9BCA0038  stb r30, 0x38(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(56 as u32), ctx.r[30].u8 ) };
	// 825B41EC: 9BAB0038  stb r29, 0x38(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(56 as u32), ctx.r[29].u8 ) };
	// 825B41F0: 485F9C39  bl 0x82bade28
	ctx.lr = 0x825B41F4;
	sub_82BADE28(ctx, base);
	// 825B41F4: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 825B41F8: 895F0038  lbz r10, 0x38(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 825B41FC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825B4200: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 825B4204: 994B0038  stb r10, 0x38(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(56 as u32), ctx.r[10].u8 ) };
	// 825B4208: 9BDF0038  stb r30, 0x38(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[30].u8 ) };
	// 825B420C: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 825B4210: 9BCB0038  stb r30, 0x38(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(56 as u32), ctx.r[30].u8 ) };
	// 825B4214: 485F9BAD  bl 0x82baddc0
	ctx.lr = 0x825B4218;
	sub_82BADDC0(ctx, base);
	// 825B4218: 480000C0  b 0x825b42d8
	pc = 0x825B42D8; continue 'dispatch;
	// 825B421C: 894B0038  lbz r10, 0x38(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) } as u64;
	// 825B4220: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 825B4224: 409A001C  bne cr6, 0x825b4240
	if !ctx.cr[6].eq {
	pc = 0x825B4240; continue 'dispatch;
	}
	// 825B4228: 9BCB0038  stb r30, 0x38(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(56 as u32), ctx.r[30].u8 ) };
	// 825B422C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825B4230: 9BBF0038  stb r29, 0x38(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[29].u8 ) };
	// 825B4234: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 825B4238: 485F9BF1  bl 0x82bade28
	ctx.lr = 0x825B423C;
	sub_82BADE28(ctx, base);
	// 825B423C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B4240: 894B0039  lbz r10, 0x39(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(57 as u32) ) } as u64;
	// 825B4244: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 825B4248: 409A0028  bne cr6, 0x825b4270
	if !ctx.cr[6].eq {
	pc = 0x825B4270; continue 'dispatch;
	}
	// 825B424C: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 825B4250: 894A0038  lbz r10, 0x38(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(56 as u32) ) } as u64;
	// 825B4254: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 825B4258: 409A0034  bne cr6, 0x825b428c
	if !ctx.cr[6].eq {
	pc = 0x825B428C; continue 'dispatch;
	}
	// 825B425C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B4260: 894A0038  lbz r10, 0x38(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(56 as u32) ) } as u64;
	// 825B4264: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 825B4268: 409A0024  bne cr6, 0x825b428c
	if !ctx.cr[6].eq {
	pc = 0x825B428C; continue 'dispatch;
	}
	// 825B426C: 9BAB0038  stb r29, 0x38(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(56 as u32), ctx.r[29].u8 ) };
	// 825B4270: 817A0004  lwz r11, 4(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 825B4274: 7FFCFB78  mr r28, r31
	ctx.r[28].u64 = ctx.r[31].u64;
	// 825B4278: 83FF0004  lwz r31, 4(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 825B427C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 825B4280: 7F1C5840  cmplw cr6, r28, r11
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[11].u32, &mut ctx.xer);
	// 825B4284: 409AFEDC  bne cr6, 0x825b4160
	if !ctx.cr[6].eq {
	pc = 0x825B4160; continue 'dispatch;
	}
	// 825B4288: 48000050  b 0x825b42d8
	pc = 0x825B42D8; continue 'dispatch;
	// 825B428C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B4290: 894A0038  lbz r10, 0x38(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(56 as u32) ) } as u64;
	// 825B4294: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 825B4298: 409A0020  bne cr6, 0x825b42b8
	if !ctx.cr[6].eq {
	pc = 0x825B42B8; continue 'dispatch;
	}
	// 825B429C: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 825B42A0: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 825B42A4: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 825B42A8: 9BCA0038  stb r30, 0x38(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(56 as u32), ctx.r[30].u8 ) };
	// 825B42AC: 9BAB0038  stb r29, 0x38(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(56 as u32), ctx.r[29].u8 ) };
	// 825B42B0: 485F9B11  bl 0x82baddc0
	ctx.lr = 0x825B42B4;
	sub_82BADDC0(ctx, base);
	// 825B42B4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B42B8: 895F0038  lbz r10, 0x38(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 825B42BC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825B42C0: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 825B42C4: 994B0038  stb r10, 0x38(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(56 as u32), ctx.r[10].u8 ) };
	// 825B42C8: 9BDF0038  stb r30, 0x38(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[30].u8 ) };
	// 825B42CC: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B42D0: 9BCB0038  stb r30, 0x38(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(56 as u32), ctx.r[30].u8 ) };
	// 825B42D4: 485F9B55  bl 0x82bade28
	ctx.lr = 0x825B42D8;
	sub_82BADE28(ctx, base);
	// 825B42D8: 9BDC0038  stb r30, 0x38(r28)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[28].u32.wrapping_add(56 as u32), ctx.r[30].u8 ) };
	// 825B42DC: 387B0018  addi r3, r27, 0x18
	ctx.r[3].s64 = ctx.r[27].s64 + 24;
	// 825B42E0: 4BD149D9  bl 0x822c8cb8
	ctx.lr = 0x825B42E4;
	sub_822C8CB8(ctx, base);
	// 825B42E4: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 825B42E8: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 825B42EC: 806B110C  lwz r3, 0x110c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4364 as u32) ) } as u64;
	// 825B42F0: 4883DE99  bl 0x82df2188
	ctx.lr = 0x825B42F4;
	sub_82DF2188(ctx, base);
	// 825B42F4: 817A0008  lwz r11, 8(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 825B42F8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825B42FC: 419A000C  beq cr6, 0x825b4308
	if ctx.cr[6].eq {
	pc = 0x825B4308; continue 'dispatch;
	}
	// 825B4300: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 825B4304: 917A0008  stw r11, 8(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 825B4308: 93380000  stw r25, 0(r24)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[24].u32.wrapping_add(0 as u32), ctx.r[25].u32 ) };
	// 825B430C: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 825B4310: 382100E0  addi r1, r1, 0xe0
	ctx.r[1].s64 = ctx.r[1].s64 + 224;
	// 825B4314: 48BF3E94  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B4318(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825B4318 size=132
    let mut pc: u32 = 0x825B4318;
    'dispatch: loop {
        match pc {
            0x825B4318 => {
    //   block [0x825B4318..0x825B439C)
	// 825B4318: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B431C: 48BF3E4D  bl 0x831a8168
	ctx.lr = 0x825B4320;
	sub_831A8130(ctx, base);
	// 825B4320: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B4324: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 825B4328: 90A100A4  stw r5, 0xa4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(164 as u32), ctx.r[5].u32 ) };
	// 825B432C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 825B4330: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 825B4334: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 825B4338: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B433C: 7F055040  cmplw cr6, r5, r10
	ctx.cr[6].compare_u32(ctx.r[5].u32, ctx.r[10].u32, &mut ctx.xer);
	// 825B4340: 409A0044  bne cr6, 0x825b4384
	if !ctx.cr[6].eq {
	pc = 0x825B4384; continue 'dispatch;
	}
	// 825B4344: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 825B4348: 409A003C  bne cr6, 0x825b4384
	if !ctx.cr[6].eq {
	pc = 0x825B4384; continue 'dispatch;
	}
	// 825B434C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B4350: 480174B1  bl 0x825cb800
	ctx.lr = 0x825B4354;
	sub_825CB800(ctx, base);
	// 825B4354: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 825B4358: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B435C: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825B4360: 48000030  b 0x825b4390
	pc = 0x825B4390; continue 'dispatch;
	// 825B4364: 386100A4  addi r3, r1, 0xa4
	ctx.r[3].s64 = ctx.r[1].s64 + 164;
	// 825B4368: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 825B436C: 48016475  bl 0x825ca7e0
	ctx.lr = 0x825B4370;
	sub_825CA7E0(ctx, base);
	// 825B4370: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 825B4374: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825B4378: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825B437C: 4BFFFB9D  bl 0x825b3f18
	ctx.lr = 0x825B4380;
	sub_825B3F18(ctx, base);
	// 825B4380: 80A100A4  lwz r5, 0xa4(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(164 as u32) ) } as u64;
	// 825B4384: 7F05F040  cmplw cr6, r5, r30
	ctx.cr[6].compare_u32(ctx.r[5].u32, ctx.r[30].u32, &mut ctx.xer);
	// 825B4388: 409AFFDC  bne cr6, 0x825b4364
	if !ctx.cr[6].eq {
	pc = 0x825B4364; continue 'dispatch;
	}
	// 825B438C: 90BD0000  stw r5, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[5].u32 ) };
	// 825B4390: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 825B4394: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825B4398: 48BF3E20  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B43A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825B43A0 size=120
    let mut pc: u32 = 0x825B43A0;
    'dispatch: loop {
        match pc {
            0x825B43A0 => {
    //   block [0x825B43A0..0x825B4418)
	// 825B43A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B43A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825B43A8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825B43AC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B43B0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825B43B4: 387F00F4  addi r3, r31, 0xf4
	ctx.r[3].s64 = ctx.r[31].s64 + 244;
	// 825B43B8: 4BD6CB39  bl 0x82320ef0
	ctx.lr = 0x825B43BC;
	sub_82320EF0(ctx, base);
	// 825B43BC: 387F00E8  addi r3, r31, 0xe8
	ctx.r[3].s64 = ctx.r[31].s64 + 232;
	// 825B43C0: 48017B01  bl 0x825cbec0
	ctx.lr = 0x825B43C4;
	sub_825CBEC0(ctx, base);
	// 825B43C4: 387F00C4  addi r3, r31, 0xc4
	ctx.r[3].s64 = ctx.r[31].s64 + 196;
	// 825B43C8: 4BEFCED9  bl 0x824b12a0
	ctx.lr = 0x825B43CC;
	sub_824B12A0(ctx, base);
	// 825B43CC: 387F00B4  addi r3, r31, 0xb4
	ctx.r[3].s64 = ctx.r[31].s64 + 180;
	// 825B43D0: 4BEFCED1  bl 0x824b12a0
	ctx.lr = 0x825B43D4;
	sub_824B12A0(ctx, base);
	// 825B43D4: 807F00B0  lwz r3, 0xb0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(176 as u32) ) } as u64;
	// 825B43D8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825B43DC: 419A0008  beq cr6, 0x825b43e4
	if ctx.cr[6].eq {
	pc = 0x825B43E4; continue 'dispatch;
	}
	// 825B43E0: 4BD0C4B1  bl 0x822c0890
	ctx.lr = 0x825B43E4;
	sub_822C0890(ctx, base);
	// 825B43E4: 807F00A8  lwz r3, 0xa8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(168 as u32) ) } as u64;
	// 825B43E8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825B43EC: 419A0008  beq cr6, 0x825b43f4
	if ctx.cr[6].eq {
	pc = 0x825B43F4; continue 'dispatch;
	}
	// 825B43F0: 4BD0C4A1  bl 0x822c0890
	ctx.lr = 0x825B43F4;
	sub_822C0890(ctx, base);
	// 825B43F4: 387F00A0  addi r3, r31, 0xa0
	ctx.r[3].s64 = ctx.r[31].s64 + 160;
	// 825B43F8: 4883F031  bl 0x82df3428
	ctx.lr = 0x825B43FC;
	sub_82DF3428(ctx, base);
	// 825B43FC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B4400: 48B0E6E9  bl 0x830c2ae8
	ctx.lr = 0x825B4404;
	sub_830C2AE8(ctx, base);
	// 825B4404: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825B4408: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825B440C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825B4410: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825B4414: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B4418(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825B4418 size=196
    let mut pc: u32 = 0x825B4418;
    'dispatch: loop {
        match pc {
            0x825B4418 => {
    //   block [0x825B4418..0x825B44DC)
	// 825B4418: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B441C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825B4420: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825B4424: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825B4428: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B442C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 825B4430: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825B4434: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 825B4438: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 825B443C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825B4440: 4BD0C4F9  bl 0x822c0938
	ctx.lr = 0x825B4444;
	sub_822C0938(ctx, base);
	// 825B4444: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 825B4448: 41820028  beq 0x825b4470
	if ctx.cr[0].eq {
	pc = 0x825B4470; continue 'dispatch;
	}
	// 825B444C: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 825B4450: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 825B4454: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825B4458: 392BBAEC  addi r9, r11, -0x4514
	ctx.r[9].s64 = ctx.r[11].s64 + -17684;
	// 825B445C: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 825B4460: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 825B4464: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 825B4468: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 825B446C: 48000008  b 0x825b4474
	pc = 0x825B4474; continue 'dispatch;
	// 825B4470: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825B4474: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825B4478: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825B447C: 409A0044  bne cr6, 0x825b44c0
	if !ctx.cr[6].eq {
	pc = 0x825B44C0; continue 'dispatch;
	}
	// 825B4480: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 825B4484: 419A001C  beq cr6, 0x825b44a0
	if ctx.cr[6].eq {
	pc = 0x825B44A0; continue 'dispatch;
	}
	// 825B4488: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B448C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 825B4490: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B4494: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B4498: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825B449C: 4E800421  bctrl
	ctx.lr = 0x825B44A0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825B44A0: 3D608329  lis r11, -0x7cd7
	ctx.r[11].s64 = -2094465024;
	// 825B44A4: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 825B44A8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825B44AC: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 825B44B0: 816B8EEC  lwz r11, -0x7114(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-28948 as u32) ) } as u64;
	// 825B44B4: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 825B44B8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825B44BC: 4BD0BB45  bl 0x822c0000
	ctx.lr = 0x825B44C0;
	sub_822C0000(ctx, base);
	// 825B44C0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825B44C4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825B44C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825B44CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825B44D0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825B44D4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825B44D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B44E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825B44E0 size=136
    let mut pc: u32 = 0x825B44E0;
    'dispatch: loop {
        match pc {
            0x825B44E0 => {
    //   block [0x825B44E0..0x825B4568)
	// 825B44E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B44E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825B44E8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825B44EC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B44F0: 89630128  lbz r11, 0x128(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(296 as u32) ) } as u64;
	// 825B44F4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 825B44F8: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 825B44FC: 4182000C  beq 0x825b4508
	if ctx.cr[0].eq {
	pc = 0x825B4508; continue 'dispatch;
	}
	// 825B4500: 80830124  lwz r4, 0x124(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(292 as u32) ) } as u64;
	// 825B4504: 4800003C  b 0x825b4540
	pc = 0x825B4540; continue 'dispatch;
	// 825B4508: 81430118  lwz r10, 0x118(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(280 as u32) ) } as u64;
	// 825B450C: 816300D4  lwz r11, 0xd4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(212 as u32) ) } as u64;
	// 825B4510: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 825B4514: 419A0034  beq cr6, 0x825b4548
	if ctx.cr[6].eq {
	pc = 0x825B4548; continue 'dispatch;
	}
	// 825B4518: 8123011C  lwz r9, 0x11c(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(284 as u32) ) } as u64;
	// 825B451C: 7D4A4850  subf r10, r10, r9
	ctx.r[10].s64 = ctx.r[9].s64 - ctx.r[10].s64;
	// 825B4520: 7D4A1670  srawi r10, r10, 2
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[10].s32 >> 2) as i64;
	// 825B4524: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 825B4528: 40980020  bge cr6, 0x825b4548
	if !ctx.cr[6].lt {
	pc = 0x825B4548; continue 'dispatch;
	}
	// 825B452C: 81430130  lwz r10, 0x130(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(304 as u32) ) } as u64;
	// 825B4530: 81230118  lwz r9, 0x118(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(280 as u32) ) } as u64;
	// 825B4534: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 825B4538: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 825B453C: 7C8B482E  lwzx r4, r11, r9
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 825B4540: 80630110  lwz r3, 0x110(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(272 as u32) ) } as u64;
	// 825B4544: 4BFF79AD  bl 0x825abef0
	ctx.lr = 0x825B4548;
	sub_825ABEF0(ctx, base);
	// 825B4548: 817F003C  lwz r11, 0x3c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) } as u64;
	// 825B454C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825B4550: 994B01C9  stb r10, 0x1c9(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(457 as u32), ctx.r[10].u8 ) };
	// 825B4554: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825B4558: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825B455C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825B4560: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825B4564: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B4568(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825B4568 size=196
    let mut pc: u32 = 0x825B4568;
    'dispatch: loop {
        match pc {
            0x825B4568 => {
    //   block [0x825B4568..0x825B462C)
	// 825B4568: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B456C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825B4570: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825B4574: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825B4578: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B457C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 825B4580: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825B4584: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 825B4588: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 825B458C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825B4590: 4BD0C3A9  bl 0x822c0938
	ctx.lr = 0x825B4594;
	sub_822C0938(ctx, base);
	// 825B4594: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 825B4598: 41820028  beq 0x825b45c0
	if ctx.cr[0].eq {
	pc = 0x825B45C0; continue 'dispatch;
	}
	// 825B459C: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 825B45A0: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 825B45A4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825B45A8: 392BBB10  addi r9, r11, -0x44f0
	ctx.r[9].s64 = ctx.r[11].s64 + -17648;
	// 825B45AC: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 825B45B0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 825B45B4: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 825B45B8: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 825B45BC: 48000008  b 0x825b45c4
	pc = 0x825B45C4; continue 'dispatch;
	// 825B45C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825B45C4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825B45C8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825B45CC: 409A0044  bne cr6, 0x825b4610
	if !ctx.cr[6].eq {
	pc = 0x825B4610; continue 'dispatch;
	}
	// 825B45D0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 825B45D4: 419A001C  beq cr6, 0x825b45f0
	if ctx.cr[6].eq {
	pc = 0x825B45F0; continue 'dispatch;
	}
	// 825B45D8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B45DC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 825B45E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B45E4: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B45E8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825B45EC: 4E800421  bctrl
	ctx.lr = 0x825B45F0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825B45F0: 3D608329  lis r11, -0x7cd7
	ctx.r[11].s64 = -2094465024;
	// 825B45F4: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 825B45F8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825B45FC: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 825B4600: 816B8F34  lwz r11, -0x70cc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-28876 as u32) ) } as u64;
	// 825B4604: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 825B4608: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825B460C: 4BD0B9F5  bl 0x822c0000
	ctx.lr = 0x825B4610;
	sub_822C0000(ctx, base);
	// 825B4610: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825B4614: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825B4618: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825B461C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825B4620: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825B4624: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825B4628: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B4630(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825B4630 size=776
    let mut pc: u32 = 0x825B4630;
    'dispatch: loop {
        match pc {
            0x825B4630 => {
    //   block [0x825B4630..0x825B4938)
	// 825B4630: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B4634: 48BF3B25  bl 0x831a8158
	ctx.lr = 0x825B4638;
	sub_831A8130(ctx, base);
	// 825B4638: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B463C: 3B200000  li r25, 0
	ctx.r[25].s64 = 0;
	// 825B4640: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825B4644: 7C982378  mr r24, r4
	ctx.r[24].u64 = ctx.r[4].u64;
	// 825B4648: 93210050  stw r25, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[25].u32 ) };
	// 825B464C: 480022E5  bl 0x825b6930
	ctx.lr = 0x825B4650;
	sub_825B6930(ctx, base);
	// 825B4650: 807F0110  lwz r3, 0x110(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(272 as u32) ) } as u64;
	// 825B4654: 4BFF6A75  bl 0x825ab0c8
	ctx.lr = 0x825B4658;
	sub_825AB0C8(ctx, base);
	// 825B4658: 817F0130  lwz r11, 0x130(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(304 as u32) ) } as u64;
	// 825B465C: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 825B4660: 3BAB0032  addi r29, r11, 0x32
	ctx.r[29].s64 = ctx.r[11].s64 + 50;
	// 825B4664: 7F1AE840  cmplw cr6, r26, r29
	ctx.cr[6].compare_u32(ctx.r[26].u32, ctx.r[29].u32, &mut ctx.xer);
	// 825B4668: 40980008  bge cr6, 0x825b4670
	if !ctx.cr[6].lt {
	pc = 0x825B4670; continue 'dispatch;
	}
	// 825B466C: 7F5DD378  mr r29, r26
	ctx.r[29].u64 = ctx.r[26].u64;
	// 825B4670: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 825B4674: 7D7C5B78  mr r28, r11
	ctx.r[28].u64 = ctx.r[11].u64;
	// 825B4678: 7F0BE840  cmplw cr6, r11, r29
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[29].u32, &mut ctx.xer);
	// 825B467C: 3B6ABB40  addi r27, r10, -0x44c0
	ctx.r[27].s64 = ctx.r[10].s64 + -17600;
	// 825B4680: 40980094  bge cr6, 0x825b4714
	if !ctx.cr[6].lt {
	pc = 0x825B4714; continue 'dispatch;
	}
	// 825B4684: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 825B4688: 809F0110  lwz r4, 0x110(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(272 as u32) ) } as u64;
	// 825B468C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825B4690: 4BFF6B21  bl 0x825ab1b0
	ctx.lr = 0x825B4694;
	sub_825AB1B0(ctx, base);
	// 825B4694: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 825B4698: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825B469C: 38A0004F  li r5, 0x4f
	ctx.r[5].s64 = 79;
	// 825B46A0: 386000D0  li r3, 0xd0
	ctx.r[3].s64 = 208;
	// 825B46A4: 4883DD45  bl 0x82df23e8
	ctx.lr = 0x825B46A8;
	sub_82DF23E8(ctx, base);
	// 825B46A8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 825B46AC: 41820014  beq 0x825b46c0
	if ctx.cr[0].eq {
	pc = 0x825B46C0; continue 'dispatch;
	}
	// 825B46B0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 825B46B4: 48003C05  bl 0x825b82b8
	ctx.lr = 0x825B46B8;
	sub_825B82B8(ctx, base);
	// 825B46B8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 825B46BC: 48000008  b 0x825b46c4
	pc = 0x825B46C4; continue 'dispatch;
	// 825B46C0: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 825B46C4: 93C10058  stw r30, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[30].u32 ) };
	// 825B46C8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 825B46CC: 3861005C  addi r3, r1, 0x5c
	ctx.r[3].s64 = ctx.r[1].s64 + 92;
	// 825B46D0: 4BFFFD49  bl 0x825b4418
	ctx.lr = 0x825B46D4;
	sub_825B4418(ctx, base);
	// 825B46D4: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 825B46D8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 825B46DC: 3861005C  addi r3, r1, 0x5c
	ctx.r[3].s64 = ctx.r[1].s64 + 92;
	// 825B46E0: 4BD0B921  bl 0x822c0000
	ctx.lr = 0x825B46E4;
	sub_822C0000(ctx, base);
	// 825B46E4: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 825B46E8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B46EC: 48002585  bl 0x825b6c70
	ctx.lr = 0x825B46F0;
	sub_825B6C70(ctx, base);
	// 825B46F0: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 825B46F4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825B46F8: 419A0008  beq cr6, 0x825b4700
	if ctx.cr[6].eq {
	pc = 0x825B4700; continue 'dispatch;
	}
	// 825B46FC: 4BD0C195  bl 0x822c0890
	ctx.lr = 0x825B4700;
	sub_822C0890(ctx, base);
	// 825B4700: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825B4704: 4883ED25  bl 0x82df3428
	ctx.lr = 0x825B4708;
	sub_82DF3428(ctx, base);
	// 825B4708: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 825B470C: 7F1CE840  cmplw cr6, r28, r29
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[29].u32, &mut ctx.xer);
	// 825B4710: 4198FF74  blt cr6, 0x825b4684
	if ctx.cr[6].lt {
	pc = 0x825B4684; continue 'dispatch;
	}
	// 825B4714: 7F1DD040  cmplw cr6, r29, r26
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[26].u32, &mut ctx.xer);
	// 825B4718: 409800C4  bge cr6, 0x825b47dc
	if !ctx.cr[6].lt {
	pc = 0x825B47DC; continue 'dispatch;
	}
	// 825B471C: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 825B4720: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825B4724: 38A00052  li r5, 0x52
	ctx.r[5].s64 = 82;
	// 825B4728: 386000D0  li r3, 0xd0
	ctx.r[3].s64 = 208;
	// 825B472C: 4883DCBD  bl 0x82df23e8
	ctx.lr = 0x825B4730;
	sub_82DF23E8(ctx, base);
	// 825B4730: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 825B4734: 4182002C  beq 0x825b4760
	if ctx.cr[0].eq {
	pc = 0x825B4760; continue 'dispatch;
	}
	// 825B4738: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 825B473C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825B4740: 388BBB30  addi r4, r11, -0x44d0
	ctx.r[4].s64 = ctx.r[11].s64 + -17616;
	// 825B4744: 4883F2C5  bl 0x82df3a08
	ctx.lr = 0x825B4748;
	sub_82DF3A08(ctx, base);
	// 825B4748: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 825B474C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825B4750: 3B200001  li r25, 1
	ctx.r[25].s64 = 1;
	// 825B4754: 48003B65  bl 0x825b82b8
	ctx.lr = 0x825B4758;
	sub_825B82B8(ctx, base);
	// 825B4758: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 825B475C: 48000008  b 0x825b4764
	pc = 0x825B4764; continue 'dispatch;
	// 825B4760: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 825B4764: 93C10060  stw r30, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[30].u32 ) };
	// 825B4768: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 825B476C: 38610064  addi r3, r1, 0x64
	ctx.r[3].s64 = ctx.r[1].s64 + 100;
	// 825B4770: 4BFFFCA9  bl 0x825b4418
	ctx.lr = 0x825B4774;
	sub_825B4418(ctx, base);
	// 825B4774: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 825B4778: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 825B477C: 38610064  addi r3, r1, 0x64
	ctx.r[3].s64 = ctx.r[1].s64 + 100;
	// 825B4780: 4BD0B881  bl 0x822c0000
	ctx.lr = 0x825B4784;
	sub_822C0000(ctx, base);
	// 825B4784: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 825B4788: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B478C: 48002535  bl 0x825b6cc0
	ctx.lr = 0x825B4790;
	sub_825B6CC0(ctx, base);
	// 825B4790: 80610064  lwz r3, 0x64(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 825B4794: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825B4798: 419A0008  beq cr6, 0x825b47a0
	if ctx.cr[6].eq {
	pc = 0x825B47A0; continue 'dispatch;
	}
	// 825B479C: 4BD0C0F5  bl 0x822c0890
	ctx.lr = 0x825B47A0;
	sub_822C0890(ctx, base);
	// 825B47A0: 572B07FF  clrlwi. r11, r25, 0x1f
	ctx.r[11].u64 = ctx.r[25].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825B47A4: 41820010  beq 0x825b47b4
	if ctx.cr[0].eq {
	pc = 0x825B47B4; continue 'dispatch;
	}
	// 825B47A8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825B47AC: 5739003C  rlwinm r25, r25, 0, 0, 0x1e
	ctx.r[25].u64 = ctx.r[25].u32 as u64 & 0xFFFFFFFFu64;
	// 825B47B0: 4883EC79  bl 0x82df3428
	ctx.lr = 0x825B47B4;
	sub_82DF3428(ctx, base);
	// 825B47B4: 8178000C  lwz r11, 0xc(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(12 as u32) ) } as u64;
	// 825B47B8: 556B05AD  rlwinm. r11, r11, 0, 0x16, 0x16
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825B47BC: 41820020  beq 0x825b47dc
	if ctx.cr[0].eq {
	pc = 0x825B47DC; continue 'dispatch;
	}
	// 825B47C0: 817F0130  lwz r11, 0x130(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(304 as u32) ) } as u64;
	// 825B47C4: 396B0032  addi r11, r11, 0x32
	ctx.r[11].s64 = ctx.r[11].s64 + 50;
	// 825B47C8: 917F0130  stw r11, 0x130(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(304 as u32), ctx.r[11].u32 ) };
	// 825B47CC: 7F0BD040  cmplw cr6, r11, r26
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[26].u32, &mut ctx.xer);
	// 825B47D0: 4198000C  blt cr6, 0x825b47dc
	if ctx.cr[6].lt {
	pc = 0x825B47DC; continue 'dispatch;
	}
	// 825B47D4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825B47D8: 917F0130  stw r11, 0x130(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(304 as u32), ctx.r[11].u32 ) };
	// 825B47DC: 817F0130  lwz r11, 0x130(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(304 as u32) ) } as u64;
	// 825B47E0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825B47E4: 419A00B0  beq cr6, 0x825b4894
	if ctx.cr[6].eq {
	pc = 0x825B4894; continue 'dispatch;
	}
	// 825B47E8: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 825B47EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825B47F0: 38A0005C  li r5, 0x5c
	ctx.r[5].s64 = 92;
	// 825B47F4: 386000D0  li r3, 0xd0
	ctx.r[3].s64 = 208;
	// 825B47F8: 4883DBF1  bl 0x82df23e8
	ctx.lr = 0x825B47FC;
	sub_82DF23E8(ctx, base);
	// 825B47FC: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 825B4800: 4182002C  beq 0x825b482c
	if ctx.cr[0].eq {
	pc = 0x825B482C; continue 'dispatch;
	}
	// 825B4804: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 825B4808: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825B480C: 388BBB20  addi r4, r11, -0x44e0
	ctx.r[4].s64 = ctx.r[11].s64 + -17632;
	// 825B4810: 4883F1F9  bl 0x82df3a08
	ctx.lr = 0x825B4814;
	sub_82DF3A08(ctx, base);
	// 825B4814: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 825B4818: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825B481C: 63390002  ori r25, r25, 2
	ctx.r[25].u64 = ctx.r[25].u64 | 2;
	// 825B4820: 48003A99  bl 0x825b82b8
	ctx.lr = 0x825B4824;
	sub_825B82B8(ctx, base);
	// 825B4824: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 825B4828: 48000008  b 0x825b4830
	pc = 0x825B4830; continue 'dispatch;
	// 825B482C: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 825B4830: 93C10068  stw r30, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[30].u32 ) };
	// 825B4834: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 825B4838: 3861006C  addi r3, r1, 0x6c
	ctx.r[3].s64 = ctx.r[1].s64 + 108;
	// 825B483C: 4BFFFBDD  bl 0x825b4418
	ctx.lr = 0x825B4840;
	sub_825B4418(ctx, base);
	// 825B4840: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 825B4844: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 825B4848: 3861006C  addi r3, r1, 0x6c
	ctx.r[3].s64 = ctx.r[1].s64 + 108;
	// 825B484C: 4BD0B7B5  bl 0x822c0000
	ctx.lr = 0x825B4850;
	sub_822C0000(ctx, base);
	// 825B4850: 38810068  addi r4, r1, 0x68
	ctx.r[4].s64 = ctx.r[1].s64 + 104;
	// 825B4854: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B4858: 48002469  bl 0x825b6cc0
	ctx.lr = 0x825B485C;
	sub_825B6CC0(ctx, base);
	// 825B485C: 8061006C  lwz r3, 0x6c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(108 as u32) ) } as u64;
	// 825B4860: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825B4864: 419A0008  beq cr6, 0x825b486c
	if ctx.cr[6].eq {
	pc = 0x825B486C; continue 'dispatch;
	}
	// 825B4868: 4BD0C029  bl 0x822c0890
	ctx.lr = 0x825B486C;
	sub_822C0890(ctx, base);
	// 825B486C: 572B07BD  rlwinm. r11, r25, 0, 0x1e, 0x1e
	ctx.r[11].u64 = ctx.r[25].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825B4870: 4182000C  beq 0x825b487c
	if ctx.cr[0].eq {
	pc = 0x825B487C; continue 'dispatch;
	}
	// 825B4874: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825B4878: 4883EBB1  bl 0x82df3428
	ctx.lr = 0x825B487C;
	sub_82DF3428(ctx, base);
	// 825B487C: 8178000C  lwz r11, 0xc(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(12 as u32) ) } as u64;
	// 825B4880: 556B05EF  rlwinm. r11, r11, 0, 0x17, 0x17
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825B4884: 41820010  beq 0x825b4894
	if ctx.cr[0].eq {
	pc = 0x825B4894; continue 'dispatch;
	}
	// 825B4888: 817F0130  lwz r11, 0x130(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(304 as u32) ) } as u64;
	// 825B488C: 396BFFCE  addi r11, r11, -0x32
	ctx.r[11].s64 = ctx.r[11].s64 + -50;
	// 825B4890: 917F0130  stw r11, 0x130(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(304 as u32), ctx.r[11].u32 ) };
	// 825B4894: 817F0130  lwz r11, 0x130(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(304 as u32) ) } as u64;
	// 825B4898: 815F012C  lwz r10, 0x12c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(300 as u32) ) } as u64;
	// 825B489C: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 825B48A0: 7F0BD040  cmplw cr6, r11, r26
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[26].u32, &mut ctx.xer);
	// 825B48A4: 4198000C  blt cr6, 0x825b48b0
	if ctx.cr[6].lt {
	pc = 0x825B48B0; continue 'dispatch;
	}
	// 825B48A8: 397AFFFF  addi r11, r26, -1
	ctx.r[11].s64 = ctx.r[26].s64 + -1;
	// 825B48AC: 917F012C  stw r11, 0x12c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(300 as u32), ctx.r[11].u32 ) };
	// 825B48B0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B48B4: 809F012C  lwz r4, 0x12c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(300 as u32) ) } as u64;
	// 825B48B8: 48001869  bl 0x825b6120
	ctx.lr = 0x825B48BC;
	sub_825B6120(ctx, base);
	// 825B48BC: 8178000C  lwz r11, 0xc(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(12 as u32) ) } as u64;
	// 825B48C0: 556B07BD  rlwinm. r11, r11, 0, 0x1e, 0x1e
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825B48C4: 4182000C  beq 0x825b48d0
	if ctx.cr[0].eq {
	pc = 0x825B48D0; continue 'dispatch;
	}
	// 825B48C8: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 825B48CC: 997F0128  stb r11, 0x128(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(296 as u32), ctx.r[11].u8 ) };
	// 825B48D0: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 825B48D4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B48D8: 48001DA9  bl 0x825b6680
	ctx.lr = 0x825B48DC;
	sub_825B6680(ctx, base);
	// 825B48DC: 817F012C  lwz r11, 0x12c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(300 as u32) ) } as u64;
	// 825B48E0: 83DF00D4  lwz r30, 0xd4(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(212 as u32) ) } as u64;
	// 825B48E4: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 825B48E8: 419A0048  beq cr6, 0x825b4930
	if ctx.cr[6].eq {
	pc = 0x825B4930; continue 'dispatch;
	}
	// 825B48EC: 817F0130  lwz r11, 0x130(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(304 as u32) ) } as u64;
	// 825B48F0: 815F0118  lwz r10, 0x118(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(280 as u32) ) } as u64;
	// 825B48F4: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 825B48F8: 807F0110  lwz r3, 0x110(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(272 as u32) ) } as u64;
	// 825B48FC: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 825B4900: 7C8B502E  lwzx r4, r11, r10
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 825B4904: 4BFF75ED  bl 0x825abef0
	ctx.lr = 0x825B4908;
	sub_825ABEF0(ctx, base);
	// 825B4908: 93DF012C  stw r30, 0x12c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(300 as u32), ctx.r[30].u32 ) };
	// 825B490C: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 825B4910: 8098003C  lwz r4, 0x3c(r24)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(60 as u32) ) } as u64;
	// 825B4914: 4BFFD8F5  bl 0x825b2208
	ctx.lr = 0x825B4918;
	sub_825B2208(ctx, base);
	// 825B4918: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B491C: 481E4C1D  bl 0x82799538
	ctx.lr = 0x825B4920;
	sub_82799538(ctx, base);
	// 825B4920: 80610074  lwz r3, 0x74(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 825B4924: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825B4928: 419A0008  beq cr6, 0x825b4930
	if ctx.cr[6].eq {
	pc = 0x825B4930; continue 'dispatch;
	}
	// 825B492C: 4BD0BF65  bl 0x822c0890
	ctx.lr = 0x825B4930;
	sub_822C0890(ctx, base);
	// 825B4930: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 825B4934: 48BF3874  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B4938(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825B4938 size=164
    let mut pc: u32 = 0x825B4938;
    'dispatch: loop {
        match pc {
            0x825B4938 => {
    //   block [0x825B4938..0x825B49DC)
	// 825B4938: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B493C: 48BF3825  bl 0x831a8160
	ctx.lr = 0x825B4940;
	sub_831A8130(ctx, base);
	// 825B4940: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B4944: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825B4948: 807F0110  lwz r3, 0x110(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(272 as u32) ) } as u64;
	// 825B494C: 4BFF675D  bl 0x825ab0a8
	ctx.lr = 0x825B4950;
	sub_825AB0A8(ctx, base);
	// 825B4950: 907F0124  stw r3, 0x124(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(292 as u32), ctx.r[3].u32 ) };
	// 825B4954: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 825B4958: 807F0110  lwz r3, 0x110(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(272 as u32) ) } as u64;
	// 825B495C: 4BFF676D  bl 0x825ab0c8
	ctx.lr = 0x825B4960;
	sub_825AB0C8(ctx, base);
	// 825B4960: 7C7B1B79  or. r27, r3, r3
	ctx.r[27].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 825B4964: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 825B4968: 41820054  beq 0x825b49bc
	if ctx.cr[0].eq {
	pc = 0x825B49BC; continue 'dispatch;
	}
	// 825B496C: 3B5F0114  addi r26, r31, 0x114
	ctx.r[26].s64 = ctx.r[31].s64 + 276;
	// 825B4970: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 825B4974: 807F0110  lwz r3, 0x110(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(272 as u32) ) } as u64;
	// 825B4978: 4BFF68B1  bl 0x825ab228
	ctx.lr = 0x825B497C;
	sub_825AB228(ctx, base);
	// 825B497C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 825B4980: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 825B4984: 93A10050  stw r29, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[29].u32 ) };
	// 825B4988: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 825B498C: 4BF04785  bl 0x824b9110
	ctx.lr = 0x825B4990;
	sub_824B9110(ctx, base);
	// 825B4990: 817F0124  lwz r11, 0x124(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(292 as u32) ) } as u64;
	// 825B4994: 7F0BE840  cmplw cr6, r11, r29
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[29].u32, &mut ctx.xer);
	// 825B4998: 409A0008  bne cr6, 0x825b49a0
	if !ctx.cr[6].eq {
	pc = 0x825B49A0; continue 'dispatch;
	}
	// 825B499C: 7FDCF378  mr r28, r30
	ctx.r[28].u64 = ctx.r[30].u64;
	// 825B49A0: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 825B49A4: 7F1ED840  cmplw cr6, r30, r27
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[27].u32, &mut ctx.xer);
	// 825B49A8: 4198FFC8  blt cr6, 0x825b4970
	if ctx.cr[6].lt {
	pc = 0x825B4970; continue 'dispatch;
	}
	// 825B49AC: 2B1C0032  cmplwi cr6, r28, 0x32
	ctx.cr[6].compare_u32(ctx.r[28].u32, 50 as u32, &mut ctx.xer);
	// 825B49B0: 4198000C  blt cr6, 0x825b49bc
	if ctx.cr[6].lt {
	pc = 0x825B49BC; continue 'dispatch;
	}
	// 825B49B4: 397CFFCE  addi r11, r28, -0x32
	ctx.r[11].s64 = ctx.r[28].s64 + -50;
	// 825B49B8: 917F0130  stw r11, 0x130(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(304 as u32), ctx.r[11].u32 ) };
	// 825B49BC: 817F0130  lwz r11, 0x130(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(304 as u32) ) } as u64;
	// 825B49C0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B49C4: 7C8BE050  subf r4, r11, r28
	ctx.r[4].s64 = ctx.r[28].s64 - ctx.r[11].s64;
	// 825B49C8: 48001759  bl 0x825b6120
	ctx.lr = 0x825B49CC;
	sub_825B6120(ctx, base);
	// 825B49CC: 817F00D4  lwz r11, 0xd4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(212 as u32) ) } as u64;
	// 825B49D0: 917F012C  stw r11, 0x12c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(300 as u32), ctx.r[11].u32 ) };
	// 825B49D4: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 825B49D8: 48BF37D8  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B49E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825B49E0 size=84
    let mut pc: u32 = 0x825B49E0;
    'dispatch: loop {
        match pc {
            0x825B49E0 => {
    //   block [0x825B49E0..0x825B4A34)
	// 825B49E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B49E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825B49E8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825B49EC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B49F0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825B49F4: 480023CD  bl 0x825b6dc0
	ctx.lr = 0x825B49F8;
	sub_825B6DC0(ctx, base);
	// 825B49F8: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 825B49FC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825B4A00: 394ABB94  addi r10, r10, -0x446c
	ctx.r[10].s64 = ctx.r[10].s64 + -17516;
	// 825B4A04: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B4A08: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 825B4A0C: 917F0118  stw r11, 0x118(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(280 as u32), ctx.r[11].u32 ) };
	// 825B4A10: 917F011C  stw r11, 0x11c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(284 as u32), ctx.r[11].u32 ) };
	// 825B4A14: 917F0120  stw r11, 0x120(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(288 as u32), ctx.r[11].u32 ) };
	// 825B4A18: 997F0128  stb r11, 0x128(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(296 as u32), ctx.r[11].u8 ) };
	// 825B4A1C: 917F0130  stw r11, 0x130(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(304 as u32), ctx.r[11].u32 ) };
	// 825B4A20: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825B4A24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825B4A28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825B4A2C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825B4A30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B4A38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825B4A38 size=88
    let mut pc: u32 = 0x825B4A38;
    'dispatch: loop {
        match pc {
            0x825B4A38 => {
    //   block [0x825B4A38..0x825B4A90)
	// 825B4A38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B4A3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825B4A40: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825B4A44: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825B4A48: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B4A4C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825B4A50: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825B4A54: 387F0114  addi r3, r31, 0x114
	ctx.r[3].s64 = ctx.r[31].s64 + 276;
	// 825B4A58: 4BEB55A9  bl 0x8246a000
	ctx.lr = 0x825B4A5C;
	sub_8246A000(ctx, base);
	// 825B4A5C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B4A60: 4BFFF941  bl 0x825b43a0
	ctx.lr = 0x825B4A64;
	sub_825B43A0(ctx, base);
	// 825B4A64: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825B4A68: 4182000C  beq 0x825b4a74
	if ctx.cr[0].eq {
	pc = 0x825B4A74; continue 'dispatch;
	}
	// 825B4A6C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B4A70: 4BD0B7F9  bl 0x822c0268
	ctx.lr = 0x825B4A74;
	sub_822C0268(ctx, base);
	// 825B4A74: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B4A78: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825B4A7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825B4A80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825B4A84: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825B4A88: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825B4A8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B4A90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x825B4A90 size=356
    let mut pc: u32 = 0x825B4A90;
    'dispatch: loop {
        match pc {
            0x825B4A90 => {
    //   block [0x825B4A90..0x825B4BF4)
	// 825B4A90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B4A94: 48BF36D5  bl 0x831a8168
	ctx.lr = 0x825B4A98;
	sub_831A8130(ctx, base);
	// 825B4A98: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B4A9C: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 825B4AA0: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 825B4AA4: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 825B4AA8: 3D008204  lis r8, -0x7dfc
	ctx.r[8].s64 = -2113667072;
	// 825B4AAC: 3CE08201  lis r7, -0x7dff
	ctx.r[7].s64 = -2113863680;
	// 825B4AB0: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 825B4AB4: C1ABBC10  lfs f13, -0x43f0(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-17392 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825B4AB8: C18ABC0C  lfs f12, -0x43f4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-17396 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 825B4ABC: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 825B4AC0: C00908A4  lfs f0, 0x8a4(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825B4AC4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 825B4AC8: C168BC08  lfs f11, -0x43f8(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(-17400 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 825B4ACC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825B4AD0: C147959C  lfs f10, -0x6a64(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(-27236 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 825B4AD4: 93810050  stw r28, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[28].u32 ) };
	// 825B4AD8: D1A10070  stfs f13, 0x70(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), tmp.u32 ) };
	// 825B4ADC: 388BBB40  addi r4, r11, -0x44c0
	ctx.r[4].s64 = ctx.r[11].s64 + -17600;
	// 825B4AE0: D1810074  stfs f12, 0x74(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), tmp.u32 ) };
	// 825B4AE4: 38A0001A  li r5, 0x1a
	ctx.r[5].s64 = 26;
	// 825B4AE8: D0010078  stfs f0, 0x78(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), tmp.u32 ) };
	// 825B4AEC: 38600140  li r3, 0x140
	ctx.r[3].s64 = 320;
	// 825B4AF0: D001007C  stfs f0, 0x7c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), tmp.u32 ) };
	// 825B4AF4: D1610060  stfs f11, 0x60(r1)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), tmp.u32 ) };
	// 825B4AF8: D1410064  stfs f10, 0x64(r1)
	tmp.f32 = (ctx.f[10].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), tmp.u32 ) };
	// 825B4AFC: D0010068  stfs f0, 0x68(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), tmp.u32 ) };
	// 825B4B00: D001006C  stfs f0, 0x6c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), tmp.u32 ) };
	// 825B4B04: 4BD0B8D5  bl 0x822c03d8
	ctx.lr = 0x825B4B08;
	sub_822C03D8(ctx, base);
	// 825B4B08: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 825B4B0C: 41820034  beq 0x825b4b40
	if ctx.cr[0].eq {
	pc = 0x825B4B40; continue 'dispatch;
	}
	// 825B4B10: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 825B4B14: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825B4B18: 388BBBF8  addi r4, r11, -0x4408
	ctx.r[4].s64 = ctx.r[11].s64 + -17416;
	// 825B4B1C: 4883EEED  bl 0x82df3a08
	ctx.lr = 0x825B4B20;
	sub_82DF3A08(ctx, base);
	// 825B4B20: 38C10060  addi r6, r1, 0x60
	ctx.r[6].s64 = ctx.r[1].s64 + 96;
	// 825B4B24: 38A10070  addi r5, r1, 0x70
	ctx.r[5].s64 = ctx.r[1].s64 + 112;
	// 825B4B28: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 825B4B2C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B4B30: 3B800001  li r28, 1
	ctx.r[28].s64 = 1;
	// 825B4B34: 4BFFFEAD  bl 0x825b49e0
	ctx.lr = 0x825B4B38;
	sub_825B49E0(ctx, base);
	// 825B4B38: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825B4B3C: 48000008  b 0x825b4b44
	pc = 0x825B4B44; continue 'dispatch;
	// 825B4B40: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 825B4B44: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 825B4B48: 3BDD0004  addi r30, r29, 4
	ctx.r[30].s64 = ctx.r[29].s64 + 4;
	// 825B4B4C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825B4B50: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825B4B54: 4BFFFA15  bl 0x825b4568
	ctx.lr = 0x825B4B58;
	sub_825B4568(ctx, base);
	// 825B4B58: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 825B4B5C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825B4B60: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825B4B64: 4BD0B49D  bl 0x822c0000
	ctx.lr = 0x825B4B68;
	sub_822C0000(ctx, base);
	// 825B4B68: 578B07FF  clrlwi. r11, r28, 0x1f
	ctx.r[11].u64 = ctx.r[28].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825B4B6C: 4182000C  beq 0x825b4b78
	if ctx.cr[0].eq {
	pc = 0x825B4B78; continue 'dispatch;
	}
	// 825B4B70: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825B4B74: 4883E8B5  bl 0x82df3428
	ctx.lr = 0x825B4B78;
	sub_82DF3428(ctx, base);
	// 825B4B78: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 825B4B7C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825B4B80: 388B1BC4  addi r4, r11, 0x1bc4
	ctx.r[4].s64 = ctx.r[11].s64 + 7108;
	// 825B4B84: 4883EE85  bl 0x82df3a08
	ctx.lr = 0x825B4B88;
	sub_82DF3A08(ctx, base);
	// 825B4B88: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B4B8C: 815D0000  lwz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B4B90: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825B4B94: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 825B4B98: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 825B4B9C: 419A0024  beq cr6, 0x825b4bc0
	if ctx.cr[6].eq {
	pc = 0x825B4BC0; continue 'dispatch;
	}
	// 825B4BA0: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 825B4BA4: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 825B4BA8: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 825B4BAC: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 825B4BB0: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 825B4BB4: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 825B4BB8: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 825B4BBC: 4082FFE8  bne 0x825b4ba4
	if !ctx.cr[0].eq {
	pc = 0x825B4BA4; continue 'dispatch;
	}
	// 825B4BC0: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 825B4BC4: 807D0000  lwz r3, 0(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B4BC8: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 825B4BCC: 480016CD  bl 0x825b6298
	ctx.lr = 0x825B4BD0;
	sub_825B6298(ctx, base);
	// 825B4BD0: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 825B4BD4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825B4BD8: 419A0008  beq cr6, 0x825b4be0
	if ctx.cr[6].eq {
	pc = 0x825B4BE0; continue 'dispatch;
	}
	// 825B4BDC: 4BD0BCB5  bl 0x822c0890
	ctx.lr = 0x825B4BE0;
	sub_822C0890(ctx, base);
	// 825B4BE0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825B4BE4: 4883E845  bl 0x82df3428
	ctx.lr = 0x825B4BE8;
	sub_82DF3428(ctx, base);
	// 825B4BE8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 825B4BEC: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 825B4BF0: 48BF35C8  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B4BF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825B4BF8 size=448
    let mut pc: u32 = 0x825B4BF8;
    'dispatch: loop {
        match pc {
            0x825B4BF8 => {
    //   block [0x825B4BF8..0x825B4DB8)
	// 825B4BF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B4BFC: 48BF3571  bl 0x831a816c
	ctx.lr = 0x825B4C00;
	sub_831A8130(ctx, base);
	// 825B4C00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B4C04: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 825B4C08: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 825B4C0C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 825B4C10: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825B4C14: 480015B5  bl 0x825b61c8
	ctx.lr = 0x825B4C18;
	sub_825B61C8(ctx, base);
	// 825B4C18: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 825B4C1C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B4C20: 816B0038  lwz r11, 0x38(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) } as u64;
	// 825B4C24: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825B4C28: 4E800421  bctrl
	ctx.lr = 0x825B4C2C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825B4C2C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825B4C30: 41820164  beq 0x825b4d94
	if ctx.cr[0].eq {
	pc = 0x825B4D94; continue 'dispatch;
	}
	// 825B4C34: 817E0110  lwz r11, 0x110(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(272 as u32) ) } as u64;
	// 825B4C38: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 825B4C3C: 419800B4  blt cr6, 0x825b4cf0
	if ctx.cr[6].lt {
	pc = 0x825B4CF0; continue 'dispatch;
	}
	// 825B4C40: 409A0160  bne cr6, 0x825b4da0
	if !ctx.cr[6].eq {
	pc = 0x825B4DA0; continue 'dispatch;
	}
	// 825B4C44: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 825B4C48: 3BA00001  li r29, 1
	ctx.r[29].s64 = 1;
	// 825B4C4C: 556A07BD  rlwinm. r10, r11, 0, 0x1e, 0x1e
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 825B4C50: 41820024  beq 0x825b4c74
	if ctx.cr[0].eq {
	pc = 0x825B4C74; continue 'dispatch;
	}
	// 825B4C54: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825B4C58: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825B4C5C: 917E0110  stw r11, 0x110(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(272 as u32), ctx.r[11].u32 ) };
	// 825B4C60: 48001471  bl 0x825b60d0
	ctx.lr = 0x825B4C64;
	sub_825B60D0(ctx, base);
	// 825B4C64: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 825B4C68: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B4C6C: 816B0034  lwz r11, 0x34(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 825B4C70: 48000034  b 0x825b4ca4
	pc = 0x825B4CA4; continue 'dispatch;
	// 825B4C74: 556A07FF  clrlwi. r10, r11, 0x1f
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 825B4C78: 41820018  beq 0x825b4c90
	if ctx.cr[0].eq {
	pc = 0x825B4C90; continue 'dispatch;
	}
	// 825B4C7C: 897F004E  lbz r11, 0x4e(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(78 as u32) ) } as u64;
	// 825B4C80: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 825B4C84: 4082011C  bne 0x825b4da0
	if !ctx.cr[0].eq {
	pc = 0x825B4DA0; continue 'dispatch;
	}
	// 825B4C88: 9BBF004E  stb r29, 0x4e(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(78 as u32), ctx.r[29].u8 ) };
	// 825B4C8C: 4BFFFFC8  b 0x825b4c54
	pc = 0x825B4C54; continue 'dispatch;
	// 825B4C90: 556B06F7  rlwinm. r11, r11, 0, 0x1b, 0x1b
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825B4C94: 41820020  beq 0x825b4cb4
	if ctx.cr[0].eq {
	pc = 0x825B4CB4; continue 'dispatch;
	}
	// 825B4C98: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 825B4C9C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B4CA0: 816B0040  lwz r11, 0x40(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(64 as u32) ) } as u64;
	// 825B4CA4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825B4CA8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825B4CAC: 4E800421  bctrl
	ctx.lr = 0x825B4CB0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825B4CB0: 4800000C  b 0x825b4cbc
	pc = 0x825B4CBC; continue 'dispatch;
	// 825B4CB4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825B4CB8: 997F004E  stb r11, 0x4e(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(78 as u32), ctx.r[11].u8 ) };
	// 825B4CBC: 9BBF004D  stb r29, 0x4d(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(77 as u32), ctx.r[29].u8 ) };
	// 825B4CC0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825B4CC4: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B4CC8: 816B0064  lwz r11, 0x64(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(100 as u32) ) } as u64;
	// 825B4CCC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825B4CD0: 4E800421  bctrl
	ctx.lr = 0x825B4CD4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825B4CD4: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 825B4CD8: 907F0054  stw r3, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 825B4CDC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825B4CE0: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 825B4CE4: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B4CE8: 816B0030  lwz r11, 0x30(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 825B4CEC: 48000088  b 0x825b4d74
	pc = 0x825B4D74; continue 'dispatch;
	// 825B4CF0: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 825B4CF4: 556A07FF  clrlwi. r10, r11, 0x1f
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 825B4CF8: 41820064  beq 0x825b4d5c
	if ctx.cr[0].eq {
	pc = 0x825B4D5C; continue 'dispatch;
	}
	// 825B4CFC: 897F004E  lbz r11, 0x4e(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(78 as u32) ) } as u64;
	// 825B4D00: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 825B4D04: 4082009C  bne 0x825b4da0
	if !ctx.cr[0].eq {
	pc = 0x825B4DA0; continue 'dispatch;
	}
	// 825B4D08: 3BA00001  li r29, 1
	ctx.r[29].s64 = 1;
	// 825B4D0C: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 825B4D10: 9BBF004E  stb r29, 0x4e(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(78 as u32), ctx.r[29].u8 ) };
	// 825B4D14: 48003625  bl 0x825b8338
	ctx.lr = 0x825B4D18;
	sub_825B8338(ctx, base);
	// 825B4D18: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825B4D1C: 41820010  beq 0x825b4d2c
	if ctx.cr[0].eq {
	pc = 0x825B4D2C; continue 'dispatch;
	}
	// 825B4D20: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825B4D24: 48001445  bl 0x825b6168
	ctx.lr = 0x825B4D28;
	sub_825B6168(ctx, base);
	// 825B4D28: 48000078  b 0x825b4da0
	pc = 0x825B4DA0; continue 'dispatch;
	// 825B4D2C: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 825B4D30: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825B4D34: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B4D38: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 825B4D3C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825B4D40: 4E800421  bctrl
	ctx.lr = 0x825B4D44;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825B4D44: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825B4D48: 41820058  beq 0x825b4da0
	if ctx.cr[0].eq {
	pc = 0x825B4DA0; continue 'dispatch;
	}
	// 825B4D4C: 93BE0110  stw r29, 0x110(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(272 as u32), ctx.r[29].u32 ) };
	// 825B4D50: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825B4D54: 48001325  bl 0x825b6078
	ctx.lr = 0x825B4D58;
	sub_825B6078(ctx, base);
	// 825B4D58: 48000048  b 0x825b4da0
	pc = 0x825B4DA0; continue 'dispatch;
	// 825B4D5C: 556B06F7  rlwinm. r11, r11, 0, 0x1b, 0x1b
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825B4D60: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825B4D64: 4182001C  beq 0x825b4d80
	if ctx.cr[0].eq {
	pc = 0x825B4D80; continue 'dispatch;
	}
	// 825B4D68: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 825B4D6C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B4D70: 816B0040  lwz r11, 0x40(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(64 as u32) ) } as u64;
	// 825B4D74: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825B4D78: 4E800421  bctrl
	ctx.lr = 0x825B4D7C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825B4D7C: 48000024  b 0x825b4da0
	pc = 0x825B4DA0; continue 'dispatch;
	// 825B4D80: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825B4D84: 480018FD  bl 0x825b6680
	ctx.lr = 0x825B4D88;
	sub_825B6680(ctx, base);
	// 825B4D88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825B4D8C: 997F004E  stb r11, 0x4e(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(78 as u32), ctx.r[11].u8 ) };
	// 825B4D90: 48000010  b 0x825b4da0
	pc = 0x825B4DA0; continue 'dispatch;
	// 825B4D94: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825B4D98: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825B4D9C: 480018E5  bl 0x825b6680
	ctx.lr = 0x825B4DA0;
	sub_825B6680(ctx, base);
	// 825B4DA0: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 825B4DA4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825B4DA8: 419A0008  beq cr6, 0x825b4db0
	if ctx.cr[6].eq {
	pc = 0x825B4DB0; continue 'dispatch;
	}
	// 825B4DAC: 4BD0BAE5  bl 0x822c0890
	ctx.lr = 0x825B4DB0;
	sub_822C0890(ctx, base);
	// 825B4DB0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825B4DB4: 48BF3408  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B4DB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825B4DB8 size=68
    let mut pc: u32 = 0x825B4DB8;
    'dispatch: loop {
        match pc {
            0x825B4DB8 => {
    //   block [0x825B4DB8..0x825B4DFC)
	// 825B4DB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B4DBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825B4DC0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825B4DC4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B4DC8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825B4DCC: 48001FF5  bl 0x825b6dc0
	ctx.lr = 0x825B4DD0;
	sub_825B6DC0(ctx, base);
	// 825B4DD0: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 825B4DD4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825B4DD8: 396BBC1C  addi r11, r11, -0x43e4
	ctx.r[11].s64 = ctx.r[11].s64 + -17380;
	// 825B4DDC: 915F0110  stw r10, 0x110(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(272 as u32), ctx.r[10].u32 ) };
	// 825B4DE0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B4DE4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825B4DE8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825B4DEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825B4DF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825B4DF4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825B4DF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B4E00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825B4E00 size=64
    let mut pc: u32 = 0x825B4E00;
    'dispatch: loop {
        match pc {
            0x825B4E00 => {
    //   block [0x825B4E00..0x825B4E40)
	// 825B4E00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B4E04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825B4E08: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825B4E0C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B4E10: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 825B4E14: 38830120  addi r4, r3, 0x120
	ctx.r[4].s64 = ctx.r[3].s64 + 288;
	// 825B4E18: 807F003C  lwz r3, 0x3c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) } as u64;
	// 825B4E1C: 4BFFCFF5  bl 0x825b1e10
	ctx.lr = 0x825B4E20;
	sub_825B1E10(ctx, base);
	// 825B4E20: 815F003C  lwz r10, 0x3c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) } as u64;
	// 825B4E24: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 825B4E28: 996A01C9  stb r11, 0x1c9(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(457 as u32), ctx.r[11].u8 ) };
	// 825B4E2C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825B4E30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825B4E34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825B4E38: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825B4E3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B4E40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825B4E40 size=84
    let mut pc: u32 = 0x825B4E40;
    'dispatch: loop {
        match pc {
            0x825B4E40 => {
    //   block [0x825B4E40..0x825B4E94)
	// 825B4E40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B4E44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825B4E48: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825B4E4C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B4E50: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825B4E54: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825B4E58: 909F0110  stw r4, 0x110(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(272 as u32), ctx.r[4].u32 ) };
	// 825B4E5C: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B4E60: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 825B4E64: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825B4E68: 4E800421  bctrl
	ctx.lr = 0x825B4E6C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825B4E6C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 825B4E70: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B4E74: 480014B5  bl 0x825b6328
	ctx.lr = 0x825B4E78;
	sub_825B6328(ctx, base);
	// 825B4E78: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825B4E7C: 4883E5AD  bl 0x82df3428
	ctx.lr = 0x825B4E80;
	sub_82DF3428(ctx, base);
	// 825B4E80: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825B4E84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825B4E88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825B4E8C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825B4E90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B4E98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825B4E98 size=196
    let mut pc: u32 = 0x825B4E98;
    'dispatch: loop {
        match pc {
            0x825B4E98 => {
    //   block [0x825B4E98..0x825B4F5C)
	// 825B4E98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B4E9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825B4EA0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825B4EA4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825B4EA8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B4EAC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 825B4EB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825B4EB4: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 825B4EB8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 825B4EBC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825B4EC0: 4BD0BA79  bl 0x822c0938
	ctx.lr = 0x825B4EC4;
	sub_822C0938(ctx, base);
	// 825B4EC4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 825B4EC8: 41820028  beq 0x825b4ef0
	if ctx.cr[0].eq {
	pc = 0x825B4EF0; continue 'dispatch;
	}
	// 825B4ECC: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 825B4ED0: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 825B4ED4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825B4ED8: 392BBC90  addi r9, r11, -0x4370
	ctx.r[9].s64 = ctx.r[11].s64 + -17264;
	// 825B4EDC: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 825B4EE0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 825B4EE4: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 825B4EE8: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 825B4EEC: 48000008  b 0x825b4ef4
	pc = 0x825B4EF4; continue 'dispatch;
	// 825B4EF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825B4EF4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825B4EF8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825B4EFC: 409A0044  bne cr6, 0x825b4f40
	if !ctx.cr[6].eq {
	pc = 0x825B4F40; continue 'dispatch;
	}
	// 825B4F00: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 825B4F04: 419A001C  beq cr6, 0x825b4f20
	if ctx.cr[6].eq {
	pc = 0x825B4F20; continue 'dispatch;
	}
	// 825B4F08: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B4F0C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 825B4F10: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B4F14: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B4F18: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825B4F1C: 4E800421  bctrl
	ctx.lr = 0x825B4F20;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825B4F20: 3D608329  lis r11, -0x7cd7
	ctx.r[11].s64 = -2094465024;
	// 825B4F24: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 825B4F28: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825B4F2C: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 825B4F30: 816B8FC4  lwz r11, -0x703c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-28732 as u32) ) } as u64;
	// 825B4F34: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 825B4F38: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825B4F3C: 4BD0B0C5  bl 0x822c0000
	ctx.lr = 0x825B4F40;
	sub_822C0000(ctx, base);
	// 825B4F40: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825B4F44: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825B4F48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825B4F4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825B4F50: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825B4F54: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825B4F58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B4F60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825B4F60 size=172
    let mut pc: u32 = 0x825B4F60;
    'dispatch: loop {
        match pc {
            0x825B4F60 => {
    //   block [0x825B4F60..0x825B500C)
	// 825B4F60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B4F64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825B4F68: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825B4F6C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825B4F70: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B4F74: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 825B4F78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825B4F7C: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 825B4F80: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 825B4F84: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825B4F88: 4BD0B9B1  bl 0x822c0938
	ctx.lr = 0x825B4F8C;
	sub_822C0938(ctx, base);
	// 825B4F8C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 825B4F90: 41820028  beq 0x825b4fb8
	if ctx.cr[0].eq {
	pc = 0x825B4FB8; continue 'dispatch;
	}
	// 825B4F94: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 825B4F98: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 825B4F9C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825B4FA0: 392BBCA4  addi r9, r11, -0x435c
	ctx.r[9].s64 = ctx.r[11].s64 + -17244;
	// 825B4FA4: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 825B4FA8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 825B4FAC: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 825B4FB0: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 825B4FB4: 48000008  b 0x825b4fbc
	pc = 0x825B4FBC; continue 'dispatch;
	// 825B4FB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825B4FBC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825B4FC0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825B4FC4: 409A002C  bne cr6, 0x825b4ff0
	if !ctx.cr[6].eq {
	pc = 0x825B4FF0; continue 'dispatch;
	}
	// 825B4FC8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B4FCC: 4BD0B29D  bl 0x822c0268
	ctx.lr = 0x825B4FD0;
	sub_822C0268(ctx, base);
	// 825B4FD0: 3D608329  lis r11, -0x7cd7
	ctx.r[11].s64 = -2094465024;
	// 825B4FD4: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 825B4FD8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825B4FDC: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 825B4FE0: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 825B4FE4: 816B8FC4  lwz r11, -0x703c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-28732 as u32) ) } as u64;
	// 825B4FE8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825B4FEC: 4BD0B015  bl 0x822c0000
	ctx.lr = 0x825B4FF0;
	sub_822C0000(ctx, base);
	// 825B4FF0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825B4FF4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825B4FF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825B4FFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825B5000: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825B5004: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825B5008: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B5010(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x825B5010 size=704
    let mut pc: u32 = 0x825B5010;
    'dispatch: loop {
        match pc {
            0x825B5010 => {
    //   block [0x825B5010..0x825B52D0)
	// 825B5010: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B5014: 48BF3149  bl 0x831a815c
	ctx.lr = 0x825B5018;
	sub_831A8130(ctx, base);
	// 825B5018: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B501C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825B5020: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 825B5024: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 825B5028: 833F00D4  lwz r25, 0xd4(r31)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(212 as u32) ) } as u64;
	// 825B502C: 93A10058  stw r29, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[29].u32 ) };
	// 825B5030: 48001901  bl 0x825b6930
	ctx.lr = 0x825B5034;
	sub_825B6930(ctx, base);
	// 825B5034: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 825B5038: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825B503C: 3B4BBCC0  addi r26, r11, -0x4340
	ctx.r[26].s64 = ctx.r[11].s64 + -17216;
	// 825B5040: 38A00097  li r5, 0x97
	ctx.r[5].s64 = 151;
	// 825B5044: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 825B5048: 386000D0  li r3, 0xd0
	ctx.r[3].s64 = 208;
	// 825B504C: 4883D39D  bl 0x82df23e8
	ctx.lr = 0x825B5050;
	sub_82DF23E8(ctx, base);
	// 825B5050: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 825B5054: 4182002C  beq 0x825b5080
	if ctx.cr[0].eq {
	pc = 0x825B5080; continue 'dispatch;
	}
	// 825B5058: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 825B505C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825B5060: 388BBCB4  addi r4, r11, -0x434c
	ctx.r[4].s64 = ctx.r[11].s64 + -17228;
	// 825B5064: 4883E9A5  bl 0x82df3a08
	ctx.lr = 0x825B5068;
	sub_82DF3A08(ctx, base);
	// 825B5068: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 825B506C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825B5070: 3BA00001  li r29, 1
	ctx.r[29].s64 = 1;
	// 825B5074: 48003245  bl 0x825b82b8
	ctx.lr = 0x825B5078;
	sub_825B82B8(ctx, base);
	// 825B5078: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 825B507C: 48000008  b 0x825b5084
	pc = 0x825B5084; continue 'dispatch;
	// 825B5080: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 825B5084: 93C10060  stw r30, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[30].u32 ) };
	// 825B5088: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 825B508C: 38610064  addi r3, r1, 0x64
	ctx.r[3].s64 = ctx.r[1].s64 + 100;
	// 825B5090: 4BFFF389  bl 0x825b4418
	ctx.lr = 0x825B5094;
	sub_825B4418(ctx, base);
	// 825B5094: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 825B5098: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 825B509C: 38610064  addi r3, r1, 0x64
	ctx.r[3].s64 = ctx.r[1].s64 + 100;
	// 825B50A0: 4BD0AF61  bl 0x822c0000
	ctx.lr = 0x825B50A4;
	sub_822C0000(ctx, base);
	// 825B50A4: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 825B50A8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B50AC: 48001945  bl 0x825b69f0
	ctx.lr = 0x825B50B0;
	sub_825B69F0(ctx, base);
	// 825B50B0: 80610064  lwz r3, 0x64(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 825B50B4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825B50B8: 419A0008  beq cr6, 0x825b50c0
	if ctx.cr[6].eq {
	pc = 0x825B50C0; continue 'dispatch;
	}
	// 825B50BC: 4BD0B7D5  bl 0x822c0890
	ctx.lr = 0x825B50C0;
	sub_822C0890(ctx, base);
	// 825B50C0: 57AB07FF  clrlwi. r11, r29, 0x1f
	ctx.r[11].u64 = ctx.r[29].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825B50C4: 4182000C  beq 0x825b50d0
	if ctx.cr[0].eq {
	pc = 0x825B50D0; continue 'dispatch;
	}
	// 825B50C8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825B50CC: 4883E35D  bl 0x82df3428
	ctx.lr = 0x825B50D0;
	sub_82DF3428(ctx, base);
	// 825B50D0: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 825B50D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825B50D8: 38A00099  li r5, 0x99
	ctx.r[5].s64 = 153;
	// 825B50DC: 38600020  li r3, 0x20
	ctx.r[3].s64 = 32;
	// 825B50E0: 4BD0B2F9  bl 0x822c03d8
	ctx.lr = 0x825B50E4;
	sub_822C03D8(ctx, base);
	// 825B50E4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 825B50E8: 41820028  beq 0x825b5110
	if ctx.cr[0].eq {
	pc = 0x825B5110; continue 'dispatch;
	}
	// 825B50EC: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 825B50F0: C03C0000  lfs f1, 0(r28)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 825B50F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825B50F8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825B50FC: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 825B5100: C04B08A8  lfs f2, 0x8a8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2216 as u32) ) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 825B5104: 480048CD  bl 0x825b99d0
	ctx.lr = 0x825B5108;
	sub_825B99D0(ctx, base);
	// 825B5108: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 825B510C: 48000008  b 0x825b5114
	pc = 0x825B5114; continue 'dispatch;
	// 825B5110: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 825B5114: 93C10070  stw r30, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[30].u32 ) };
	// 825B5118: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 825B511C: 38610074  addi r3, r1, 0x74
	ctx.r[3].s64 = ctx.r[1].s64 + 116;
	// 825B5120: 4BFFFE41  bl 0x825b4f60
	ctx.lr = 0x825B5124;
	sub_825B4F60(ctx, base);
	// 825B5124: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 825B5128: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 825B512C: 38610074  addi r3, r1, 0x74
	ctx.r[3].s64 = ctx.r[1].s64 + 116;
	// 825B5130: 4BD0AED1  bl 0x822c0000
	ctx.lr = 0x825B5134;
	sub_822C0000(ctx, base);
	// 825B5134: 807F0110  lwz r3, 0x110(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(272 as u32) ) } as u64;
	// 825B5138: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 825B513C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B5140: 816B0058  lwz r11, 0x58(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(88 as u32) ) } as u64;
	// 825B5144: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825B5148: 4E800421  bctrl
	ctx.lr = 0x825B514C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825B514C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 825B5150: 41820118  beq 0x825b5268
	if ctx.cr[0].eq {
	pc = 0x825B5268; continue 'dispatch;
	}
	// 825B5154: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 825B5158: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 825B515C: 3B6B1C1C  addi r27, r11, 0x1c1c
	ctx.r[27].s64 = ctx.r[11].s64 + 7196;
	// 825B5160: 3B8A9BC9  addi r28, r10, -0x6437
	ctx.r[28].s64 = ctx.r[10].s64 + -25655;
	// 825B5164: 807F0110  lwz r3, 0x110(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(272 as u32) ) } as u64;
	// 825B5168: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 825B516C: 80A10070  lwz r5, 0x70(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	// 825B5170: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B5174: 816B0064  lwz r11, 0x64(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(100 as u32) ) } as u64;
	// 825B5178: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825B517C: 4E800421  bctrl
	ctx.lr = 0x825B5180;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825B5180: 809F0110  lwz r4, 0x110(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(272 as u32) ) } as u64;
	// 825B5184: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 825B5188: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825B518C: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B5190: 816B0070  lwz r11, 0x70(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(112 as u32) ) } as u64;
	// 825B5194: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825B5198: 4E800421  bctrl
	ctx.lr = 0x825B519C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825B519C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 825B51A0: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 825B51A4: 4883E865  bl 0x82df3a08
	ctx.lr = 0x825B51A8;
	sub_82DF3A08(ctx, base);
	// 825B51A8: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 825B51AC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825B51B0: 4883E159  bl 0x82df3308
	ctx.lr = 0x825B51B4;
	sub_82DF3308(ctx, base);
	// 825B51B4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 825B51B8: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 825B51BC: 4883E26D  bl 0x82df3428
	ctx.lr = 0x825B51C0;
	sub_82DF3428(ctx, base);
	// 825B51C0: 57CB063F  clrlwi. r11, r30, 0x18
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825B51C4: 41820010  beq 0x825b51d4
	if ctx.cr[0].eq {
	pc = 0x825B51D4; continue 'dispatch;
	}
	// 825B51C8: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 825B51CC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825B51D0: 4883E6A9  bl 0x82df3878
	ctx.lr = 0x825B51D4;
	sub_82DF3878(ctx, base);
	// 825B51D4: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 825B51D8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825B51DC: 38A000A3  li r5, 0xa3
	ctx.r[5].s64 = 163;
	// 825B51E0: 386000D0  li r3, 0xd0
	ctx.r[3].s64 = 208;
	// 825B51E4: 4883D205  bl 0x82df23e8
	ctx.lr = 0x825B51E8;
	sub_82DF23E8(ctx, base);
	// 825B51E8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 825B51EC: 41820014  beq 0x825b5200
	if ctx.cr[0].eq {
	pc = 0x825B5200; continue 'dispatch;
	}
	// 825B51F0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 825B51F4: 480030C5  bl 0x825b82b8
	ctx.lr = 0x825B51F8;
	sub_825B82B8(ctx, base);
	// 825B51F8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 825B51FC: 48000008  b 0x825b5204
	pc = 0x825B5204; continue 'dispatch;
	// 825B5200: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 825B5204: 93C10068  stw r30, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[30].u32 ) };
	// 825B5208: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 825B520C: 3861006C  addi r3, r1, 0x6c
	ctx.r[3].s64 = ctx.r[1].s64 + 108;
	// 825B5210: 4BFFF209  bl 0x825b4418
	ctx.lr = 0x825B5214;
	sub_825B4418(ctx, base);
	// 825B5214: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 825B5218: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 825B521C: 3861006C  addi r3, r1, 0x6c
	ctx.r[3].s64 = ctx.r[1].s64 + 108;
	// 825B5220: 4BD0ADE1  bl 0x822c0000
	ctx.lr = 0x825B5224;
	sub_822C0000(ctx, base);
	// 825B5224: 38810068  addi r4, r1, 0x68
	ctx.r[4].s64 = ctx.r[1].s64 + 104;
	// 825B5228: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B522C: 480017C5  bl 0x825b69f0
	ctx.lr = 0x825B5230;
	sub_825B69F0(ctx, base);
	// 825B5230: 8061006C  lwz r3, 0x6c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(108 as u32) ) } as u64;
	// 825B5234: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825B5238: 419A0008  beq cr6, 0x825b5240
	if ctx.cr[6].eq {
	pc = 0x825B5240; continue 'dispatch;
	}
	// 825B523C: 4BD0B655  bl 0x822c0890
	ctx.lr = 0x825B5240;
	sub_822C0890(ctx, base);
	// 825B5240: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825B5244: 4883E1E5  bl 0x82df3428
	ctx.lr = 0x825B5248;
	sub_82DF3428(ctx, base);
	// 825B5248: 807F0110  lwz r3, 0x110(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(272 as u32) ) } as u64;
	// 825B524C: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 825B5250: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B5254: 816B0058  lwz r11, 0x58(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(88 as u32) ) } as u64;
	// 825B5258: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825B525C: 4E800421  bctrl
	ctx.lr = 0x825B5260;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825B5260: 7F1D1840  cmplw cr6, r29, r3
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[3].u32, &mut ctx.xer);
	// 825B5264: 4198FF00  blt cr6, 0x825b5164
	if ctx.cr[6].lt {
	pc = 0x825B5164; continue 'dispatch;
	}
	// 825B5268: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 825B526C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B5270: 48000EB1  bl 0x825b6120
	ctx.lr = 0x825B5274;
	sub_825B6120(ctx, base);
	// 825B5274: 2B1D0012  cmplwi cr6, r29, 0x12
	ctx.cr[6].compare_u32(ctx.r[29].u32, 18 as u32, &mut ctx.xer);
	// 825B5278: 41980040  blt cr6, 0x825b52b8
	if ctx.cr[6].lt {
	pc = 0x825B52B8; continue 'dispatch;
	}
	// 825B527C: 397DFFEF  addi r11, r29, -0x11
	ctx.r[11].s64 = ctx.r[29].s64 + -17;
	// 825B5280: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 825B5284: 796B0020  clrldi r11, r11, 0x20
	ctx.r[11].u64 = ctx.r[11].u64 & 0x00000000FFFFFFFFu64;
	// 825B5288: 392ABC84  addi r9, r10, -0x437c
	ctx.r[9].s64 = ctx.r[10].s64 + -17276;
	// 825B528C: F9610058  std r11, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u64 ) };
	// 825B5290: C8010058  lfd f0, 0x58(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 825B5294: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 825B5298: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 825B529C: FD600018  frsp f11, f0
	ctx.f[11].f64 = (ctx.f[0].f64 as f32) as f64;
	// 825B52A0: C00ABC84  lfs f0, -0x437c(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-17276 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825B52A4: C1AB9590  lfs f13, -0x6a70(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27248 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825B52A8: C1890004  lfs f12, 4(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 825B52AC: D01F0020  stfs f0, 0x20(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), tmp.u32 ) };
	// 825B52B0: EC0B637A  fmadds f0, f11, f13, f12
	ctx.f[0].f64 = (((ctx.f[11].f64 * ctx.f[13].f64 + ctx.f[12].f64) as f32) as f64);
	// 825B52B4: D01F0024  stfs f0, 0x24(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), tmp.u32 ) };
	// 825B52B8: 80610074  lwz r3, 0x74(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 825B52BC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825B52C0: 419A0008  beq cr6, 0x825b52c8
	if ctx.cr[6].eq {
	pc = 0x825B52C8; continue 'dispatch;
	}
	// 825B52C4: 4BD0B5CD  bl 0x822c0890
	ctx.lr = 0x825B52C8;
	sub_822C0890(ctx, base);
	// 825B52C8: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 825B52CC: 48BF2EE0  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B52D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825B52D0 size=76
    let mut pc: u32 = 0x825B52D0;
    'dispatch: loop {
        match pc {
            0x825B52D0 => {
    //   block [0x825B52D0..0x825B531C)
	// 825B52D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B52D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825B52D8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825B52DC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825B52E0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B52E4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825B52E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825B52EC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825B52F0: 917F0130  stw r11, 0x130(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(304 as u32), ctx.r[11].u32 ) };
	// 825B52F4: 48000DDD  bl 0x825b60d0
	ctx.lr = 0x825B52F8;
	sub_825B60D0(ctx, base);
	// 825B52F8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 825B52FC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B5300: 4BFFFD11  bl 0x825b5010
	ctx.lr = 0x825B5304;
	sub_825B5010(ctx, base);
	// 825B5304: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825B5308: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825B530C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825B5310: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825B5314: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825B5318: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B5320(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825B5320 size=112
    let mut pc: u32 = 0x825B5320;
    'dispatch: loop {
        match pc {
            0x825B5320 => {
    //   block [0x825B5320..0x825B5390)
	// 825B5320: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B5324: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825B5328: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825B532C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825B5330: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B5334: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825B5338: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825B533C: 48000D95  bl 0x825b60d0
	ctx.lr = 0x825B5340;
	sub_825B60D0(ctx, base);
	// 825B5340: 817F0130  lwz r11, 0x130(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(304 as u32) ) } as u64;
	// 825B5344: 2B0B0002  cmplwi cr6, r11, 2
	ctx.cr[6].compare_u32(ctx.r[11].u32, 2 as u32, &mut ctx.xer);
	// 825B5348: 409A001C  bne cr6, 0x825b5364
	if !ctx.cr[6].eq {
	pc = 0x825B5364; continue 'dispatch;
	}
	// 825B534C: 807F0110  lwz r3, 0x110(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(272 as u32) ) } as u64;
	// 825B5350: 809F0134  lwz r4, 0x134(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(308 as u32) ) } as u64;
	// 825B5354: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B5358: 816B0054  lwz r11, 0x54(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(84 as u32) ) } as u64;
	// 825B535C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825B5360: 4E800421  bctrl
	ctx.lr = 0x825B5364;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825B5364: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825B5368: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 825B536C: 917F0130  stw r11, 0x130(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(304 as u32), ctx.r[11].u32 ) };
	// 825B5370: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B5374: 4BFFFC9D  bl 0x825b5010
	ctx.lr = 0x825B5378;
	sub_825B5010(ctx, base);
	// 825B5378: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825B537C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825B5380: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825B5384: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825B5388: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825B538C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B5390(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x825B5390 size=292
    let mut pc: u32 = 0x825B5390;
    'dispatch: loop {
        match pc {
            0x825B5390 => {
    //   block [0x825B5390..0x825B54B4)
	// 825B5390: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B5394: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825B5398: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825B539C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825B53A0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B53A4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825B53A8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825B53AC: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 825B53B0: 809E003C  lwz r4, 0x3c(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(60 as u32) ) } as u64;
	// 825B53B4: 4BFFCA4D  bl 0x825b1e00
	ctx.lr = 0x825B53B8;
	sub_825B1E00(ctx, base);
	// 825B53B8: 39600120  li r11, 0x120
	ctx.r[11].s64 = 288;
	// 825B53BC: 13E018C7  vcmpequd (lvx128) v31, v0, v3
	tmp.u32 = ctx.r[3].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 825B53C0: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 825B53C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825B53C8: 388ABCC0  addi r4, r10, -0x4340
	ctx.r[4].s64 = ctx.r[10].s64 + -17216;
	// 825B53CC: 38A00039  li r5, 0x39
	ctx.r[5].s64 = 57;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B54B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x825B54B8 size=612
    let mut pc: u32 = 0x825B54B8;
    'dispatch: loop {
        match pc {
            0x825B54B8 => {
    //   block [0x825B54B8..0x825B571C)
	// 825B54B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B54BC: 48BF2CAD  bl 0x831a8168
	ctx.lr = 0x825B54C0;
	sub_831A8130(ctx, base);
	// 825B54C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B54C4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825B54C8: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 825B54CC: 817F0130  lwz r11, 0x130(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(304 as u32) ) } as u64;
	// 825B54D0: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 825B54D4: 419800D4  blt cr6, 0x825b55a8
	if ctx.cr[6].lt {
	pc = 0x825B55A8; continue 'dispatch;
	}
	// 825B54D8: 2B0B0003  cmplwi cr6, r11, 3
	ctx.cr[6].compare_u32(ctx.r[11].u32, 3 as u32, &mut ctx.xer);
	// 825B54DC: 40980238  bge cr6, 0x825b5714
	if !ctx.cr[6].lt {
	pc = 0x825B5714; continue 'dispatch;
	}
	// 825B54E0: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 825B54E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825B54E8: 388BBCC0  addi r4, r11, -0x4340
	ctx.r[4].s64 = ctx.r[11].s64 + -17216;
	// 825B54EC: 38A0007D  li r5, 0x7d
	ctx.r[5].s64 = 125;
	// 825B54F0: 38600020  li r3, 0x20
	ctx.r[3].s64 = 32;
	// 825B54F4: 4BD0AEE5  bl 0x822c03d8
	ctx.lr = 0x825B54F8;
	sub_822C03D8(ctx, base);
	// 825B54F8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 825B54FC: 41820028  beq 0x825b5524
	if ctx.cr[0].eq {
	pc = 0x825B5524; continue 'dispatch;
	}
	// 825B5500: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 825B5504: C03D0000  lfs f1, 0(r29)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 825B5508: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825B550C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825B5510: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 825B5514: C04B08A8  lfs f2, 0x8a8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2216 as u32) ) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 825B5518: 480044B9  bl 0x825b99d0
	ctx.lr = 0x825B551C;
	sub_825B99D0(ctx, base);
	// 825B551C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 825B5520: 48000008  b 0x825b5528
	pc = 0x825B5528; continue 'dispatch;
	// 825B5524: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 825B5528: 93C10050  stw r30, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 825B552C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 825B5530: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 825B5534: 4BFFFA2D  bl 0x825b4f60
	ctx.lr = 0x825B5538;
	sub_825B4F60(ctx, base);
	// 825B5538: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 825B553C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 825B5540: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 825B5544: 4BD0AABD  bl 0x822c0000
	ctx.lr = 0x825B5548;
	sub_822C0000(ctx, base);
	// 825B5548: 807F0110  lwz r3, 0x110(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(272 as u32) ) } as u64;
	// 825B554C: 83C10050  lwz r30, 0x50(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 825B5550: 809F0134  lwz r4, 0x134(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(308 as u32) ) } as u64;
	// 825B5554: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 825B5558: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B555C: 816B006C  lwz r11, 0x6c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(108 as u32) ) } as u64;
	// 825B5560: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825B5564: 4E800421  bctrl
	ctx.lr = 0x825B5568;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825B5568: 817E0018  lwz r11, 0x18(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 825B556C: 556A077B  rlwinm. r10, r11, 0, 0x1d, 0x1d
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 825B5570: 40820008  bne 0x825b5578
	if !ctx.cr[0].eq {
	pc = 0x825B5578; continue 'dispatch;
	}
	// 825B5574: 817D000C  lwz r11, 0xc(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 825B5578: 556A07FF  clrlwi. r10, r11, 0x1f
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 825B557C: 41820014  beq 0x825b5590
	if ctx.cr[0].eq {
	pc = 0x825B5590; continue 'dispatch;
	}
	// 825B5580: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 825B5584: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B5588: 4BFFFD49  bl 0x825b52d0
	ctx.lr = 0x825B558C;
	sub_825B52D0(ctx, base);
	// 825B558C: 48000100  b 0x825b568c
	pc = 0x825B568C; continue 'dispatch;
	// 825B5590: 556B07BD  rlwinm. r11, r11, 0, 0x1e, 0x1e
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825B5594: 418200F8  beq 0x825b568c
	if ctx.cr[0].eq {
	pc = 0x825B568C; continue 'dispatch;
	}
	// 825B5598: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 825B559C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B55A0: 4BFFFD81  bl 0x825b5320
	ctx.lr = 0x825B55A4;
	sub_825B5320(ctx, base);
	// 825B55A4: 480000E8  b 0x825b568c
	pc = 0x825B568C; continue 'dispatch;
	// 825B55A8: 817D000C  lwz r11, 0xc(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 825B55AC: 556A07FF  clrlwi. r10, r11, 0x1f
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 825B55B0: 418200F0  beq 0x825b56a0
	if ctx.cr[0].eq {
	pc = 0x825B56A0; continue 'dispatch;
	}
	// 825B55B4: 817F00D4  lwz r11, 0xd4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(212 as u32) ) } as u64;
	// 825B55B8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825B55BC: 409A0028  bne cr6, 0x825b55e4
	if !ctx.cr[6].eq {
	pc = 0x825B55E4; continue 'dispatch;
	}
	// 825B55C0: 807F0110  lwz r3, 0x110(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(272 as u32) ) } as u64;
	// 825B55C4: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 825B55C8: 917F0130  stw r11, 0x130(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(304 as u32), ctx.r[11].u32 ) };
	// 825B55CC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B55D0: 816B0050  lwz r11, 0x50(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(80 as u32) ) } as u64;
	// 825B55D4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825B55D8: 4E800421  bctrl
	ctx.lr = 0x825B55DC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825B55DC: 907F0134  stw r3, 0x134(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(308 as u32), ctx.r[3].u32 ) };
	// 825B55E0: 48000014  b 0x825b55f4
	pc = 0x825B55F4; continue 'dispatch;
	// 825B55E4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825B55E8: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 825B55EC: 915F0130  stw r10, 0x130(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(304 as u32), ctx.r[10].u32 ) };
	// 825B55F0: 917F0134  stw r11, 0x134(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(308 as u32), ctx.r[11].u32 ) };
	// 825B55F4: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 825B55F8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825B55FC: 388BBCC0  addi r4, r11, -0x4340
	ctx.r[4].s64 = ctx.r[11].s64 + -17216;
	// 825B5600: 38A0005F  li r5, 0x5f
	ctx.r[5].s64 = 95;
	// 825B5604: 38600020  li r3, 0x20
	ctx.r[3].s64 = 32;
	// 825B5608: 4BD0ADD1  bl 0x822c03d8
	ctx.lr = 0x825B560C;
	sub_822C03D8(ctx, base);
	// 825B560C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 825B5610: 41820028  beq 0x825b5638
	if ctx.cr[0].eq {
	pc = 0x825B5638; continue 'dispatch;
	}
	// 825B5614: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 825B5618: C03D0000  lfs f1, 0(r29)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 825B561C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825B5620: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825B5624: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 825B5628: C04B08A8  lfs f2, 0x8a8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2216 as u32) ) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 825B562C: 480043A5  bl 0x825b99d0
	ctx.lr = 0x825B5630;
	sub_825B99D0(ctx, base);
	// 825B5630: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 825B5634: 48000008  b 0x825b563c
	pc = 0x825B563C; continue 'dispatch;
	// 825B5638: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 825B563C: 93C10050  stw r30, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 825B5640: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 825B5644: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 825B5648: 4BFFF919  bl 0x825b4f60
	ctx.lr = 0x825B564C;
	sub_825B4F60(ctx, base);
	// 825B564C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 825B5650: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 825B5654: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 825B5658: 4BD0A9A9  bl 0x822c0000
	ctx.lr = 0x825B565C;
	sub_822C0000(ctx, base);
	// 825B565C: 807F0110  lwz r3, 0x110(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(272 as u32) ) } as u64;
	// 825B5660: 809F0134  lwz r4, 0x134(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(308 as u32) ) } as u64;
	// 825B5664: 80A10050  lwz r5, 0x50(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 825B5668: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B566C: 816B0068  lwz r11, 0x68(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(104 as u32) ) } as u64;
	// 825B5670: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825B5674: 4E800421  bctrl
	ctx.lr = 0x825B5678;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825B5678: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 825B567C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B5680: 4BFFF991  bl 0x825b5010
	ctx.lr = 0x825B5684;
	sub_825B5010(ctx, base);
	// 825B5684: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B5688: 480009F1  bl 0x825b6078
	ctx.lr = 0x825B568C;
	sub_825B6078(ctx, base);
	// 825B568C: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 825B5690: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825B5694: 419A0080  beq cr6, 0x825b5714
	if ctx.cr[6].eq {
	pc = 0x825B5714; continue 'dispatch;
	}
	// 825B5698: 4BD0B1F9  bl 0x822c0890
	ctx.lr = 0x825B569C;
	sub_822C0890(ctx, base);
	// 825B569C: 48000078  b 0x825b5714
	pc = 0x825B5714; continue 'dispatch;
	// 825B56A0: 556B06F7  rlwinm. r11, r11, 0, 0x1b, 0x1b
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825B56A4: 41820064  beq 0x825b5708
	if ctx.cr[0].eq {
	pc = 0x825B5708; continue 'dispatch;
	}
	// 825B56A8: 83DF00D4  lwz r30, 0xd4(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(212 as u32) ) } as u64;
	// 825B56AC: 2B1E0001  cmplwi cr6, r30, 1
	ctx.cr[6].compare_u32(ctx.r[30].u32, 1 as u32, &mut ctx.xer);
	// 825B56B0: 41980058  blt cr6, 0x825b5708
	if ctx.cr[6].lt {
	pc = 0x825B5708; continue 'dispatch;
	}
	// 825B56B4: 807F0110  lwz r3, 0x110(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(272 as u32) ) } as u64;
	// 825B56B8: 3B9EFFFF  addi r28, r30, -1
	ctx.r[28].s64 = ctx.r[30].s64 + -1;
	// 825B56BC: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 825B56C0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B56C4: 816B0054  lwz r11, 0x54(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(84 as u32) ) } as u64;
	// 825B56C8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825B56CC: 4E800421  bctrl
	ctx.lr = 0x825B56D0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825B56D0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 825B56D4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B56D8: 4BFFF939  bl 0x825b5010
	ctx.lr = 0x825B56DC;
	sub_825B5010(ctx, base);
	// 825B56DC: 807F0110  lwz r3, 0x110(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(272 as u32) ) } as u64;
	// 825B56E0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B56E4: 816B0058  lwz r11, 0x58(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(88 as u32) ) } as u64;
	// 825B56E8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825B56EC: 4E800421  bctrl
	ctx.lr = 0x825B56F0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825B56F0: 7F1E1840  cmplw cr6, r30, r3
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[3].u32, &mut ctx.xer);
	// 825B56F4: 40990020  ble cr6, 0x825b5714
	if !ctx.cr[6].gt {
	pc = 0x825B5714; continue 'dispatch;
	}
	// 825B56F8: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 825B56FC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B5700: 48000A21  bl 0x825b6120
	ctx.lr = 0x825B5704;
	sub_825B6120(ctx, base);
	// 825B5704: 48000010  b 0x825b5714
	pc = 0x825B5714; continue 'dispatch;
	// 825B5708: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 825B570C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B5710: 48000F71  bl 0x825b6680
	ctx.lr = 0x825B5714;
	sub_825B6680(ctx, base);
	// 825B5714: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825B5718: 48BF2AA0  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B5720(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825B5720 size=80
    let mut pc: u32 = 0x825B5720;
    'dispatch: loop {
        match pc {
            0x825B5720 => {
    //   block [0x825B5720..0x825B5770)
	// 825B5720: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B5724: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825B5728: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825B572C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B5730: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825B5734: 4800168D  bl 0x825b6dc0
	ctx.lr = 0x825B5738;
	sub_825B6DC0(ctx, base);
	// 825B5738: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 825B573C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825B5740: 394ABD24  addi r10, r10, -0x42dc
	ctx.r[10].s64 = ctx.r[10].s64 + -17116;
	// 825B5744: 997F0114  stb r11, 0x114(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(276 as u32), ctx.r[11].u8 ) };
	// 825B5748: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B574C: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 825B5750: 917F0118  stw r11, 0x118(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(280 as u32), ctx.r[11].u32 ) };
	// 825B5754: 917F011C  stw r11, 0x11c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(284 as u32), ctx.r[11].u32 ) };
	// 825B5758: 917F0130  stw r11, 0x130(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(304 as u32), ctx.r[11].u32 ) };
	// 825B575C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825B5760: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825B5764: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825B5768: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825B576C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B5770(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825B5770 size=96
    let mut pc: u32 = 0x825B5770;
    'dispatch: loop {
        match pc {
            0x825B5770 => {
    //   block [0x825B5770..0x825B57D0)
	// 825B5770: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B5774: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825B5778: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825B577C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825B5780: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B5784: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825B5788: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825B578C: 807F011C  lwz r3, 0x11c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(284 as u32) ) } as u64;
	// 825B5790: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825B5794: 419A0008  beq cr6, 0x825b579c
	if ctx.cr[6].eq {
	pc = 0x825B579C; continue 'dispatch;
	}
	// 825B5798: 4BD0B0F9  bl 0x822c0890
	ctx.lr = 0x825B579C;
	sub_822C0890(ctx, base);
	// 825B579C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B57A0: 4BFFEC01  bl 0x825b43a0
	ctx.lr = 0x825B57A4;
	sub_825B43A0(ctx, base);
	// 825B57A4: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825B57A8: 4182000C  beq 0x825b57b4
	if ctx.cr[0].eq {
	pc = 0x825B57B4; continue 'dispatch;
	}
	// 825B57AC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B57B0: 4BD0AAB9  bl 0x822c0268
	ctx.lr = 0x825B57B4;
	sub_822C0268(ctx, base);
	// 825B57B4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B57B8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825B57BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825B57C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825B57C4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825B57C8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825B57CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B57D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x825B57D0 size=356
    let mut pc: u32 = 0x825B57D0;
    'dispatch: loop {
        match pc {
            0x825B57D0 => {
    //   block [0x825B57D0..0x825B5934)
	// 825B57D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B57D4: 48BF2991  bl 0x831a8164
	ctx.lr = 0x825B57D8;
	sub_831A8130(ctx, base);
	// 825B57D8: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B57DC: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 825B57E0: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 825B57E4: 392BBC84  addi r9, r11, -0x437c
	ctx.r[9].s64 = ctx.r[11].s64 + -17276;
	// 825B57E8: 3D008201  lis r8, -0x7dff
	ctx.r[8].s64 = -2113863680;
	// 825B57EC: 3CE08201  lis r7, -0x7dff
	ctx.r[7].s64 = -2113863680;
	// 825B57F0: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 825B57F4: C1ABBC84  lfs f13, -0x437c(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-17276 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825B57F8: C00A08A4  lfs f0, 0x8a4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825B57FC: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 825B5800: C1890004  lfs f12, 4(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 825B5804: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 825B5808: C168DDF0  lfs f11, -0x2210(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(-8720 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 825B580C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825B5810: C147959C  lfs f10, -0x6a64(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(-27236 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 825B5814: 93810050  stw r28, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[28].u32 ) };
	// 825B5818: D1A10070  stfs f13, 0x70(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), tmp.u32 ) };
	// 825B581C: 388BBCC0  addi r4, r11, -0x4340
	ctx.r[4].s64 = ctx.r[11].s64 + -17216;
	// 825B5820: D1810074  stfs f12, 0x74(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), tmp.u32 ) };
	// 825B5824: 38A00025  li r5, 0x25
	ctx.r[5].s64 = 37;
	// 825B5828: D0010078  stfs f0, 0x78(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), tmp.u32 ) };
	// 825B582C: 38600140  li r3, 0x140
	ctx.r[3].s64 = 320;
	// 825B5830: D001007C  stfs f0, 0x7c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), tmp.u32 ) };
	// 825B5834: D1610060  stfs f11, 0x60(r1)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), tmp.u32 ) };
	// 825B5838: D1410064  stfs f10, 0x64(r1)
	tmp.f32 = (ctx.f[10].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), tmp.u32 ) };
	// 825B583C: D0010068  stfs f0, 0x68(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), tmp.u32 ) };
	// 825B5840: D001006C  stfs f0, 0x6c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), tmp.u32 ) };
	// 825B5844: 4BD0AB95  bl 0x822c03d8
	ctx.lr = 0x825B5848;
	sub_822C03D8(ctx, base);
	// 825B5848: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 825B584C: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 825B5850: 3B6BBD88  addi r27, r11, -0x4278
	ctx.r[27].s64 = ctx.r[11].s64 + -17016;
	// 825B5854: 41820030  beq 0x825b5884
	if ctx.cr[0].eq {
	pc = 0x825B5884; continue 'dispatch;
	}
	// 825B5858: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 825B585C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825B5860: 4883E1A9  bl 0x82df3a08
	ctx.lr = 0x825B5864;
	sub_82DF3A08(ctx, base);
	// 825B5864: 38C10060  addi r6, r1, 0x60
	ctx.r[6].s64 = ctx.r[1].s64 + 96;
	// 825B5868: 38A10070  addi r5, r1, 0x70
	ctx.r[5].s64 = ctx.r[1].s64 + 112;
	// 825B586C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 825B5870: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B5874: 3B800001  li r28, 1
	ctx.r[28].s64 = 1;
	// 825B5878: 4BFFFEA9  bl 0x825b5720
	ctx.lr = 0x825B587C;
	sub_825B5720(ctx, base);
	// 825B587C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825B5880: 48000008  b 0x825b5888
	pc = 0x825B5888; continue 'dispatch;
	// 825B5884: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 825B5888: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 825B588C: 3BDD0004  addi r30, r29, 4
	ctx.r[30].s64 = ctx.r[29].s64 + 4;
	// 825B5890: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825B5894: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825B5898: 4BFFF601  bl 0x825b4e98
	ctx.lr = 0x825B589C;
	sub_825B4E98(ctx, base);
	// 825B589C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 825B58A0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825B58A4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825B58A8: 4BD0A759  bl 0x822c0000
	ctx.lr = 0x825B58AC;
	sub_822C0000(ctx, base);
	// 825B58AC: 578B07FF  clrlwi. r11, r28, 0x1f
	ctx.r[11].u64 = ctx.r[28].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825B58B0: 4182000C  beq 0x825b58bc
	if ctx.cr[0].eq {
	pc = 0x825B58BC; continue 'dispatch;
	}
	// 825B58B4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825B58B8: 4883DB71  bl 0x82df3428
	ctx.lr = 0x825B58BC;
	sub_82DF3428(ctx, base);
	// 825B58BC: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 825B58C0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825B58C4: 4883E145  bl 0x82df3a08
	ctx.lr = 0x825B58C8;
	sub_82DF3A08(ctx, base);
	// 825B58C8: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B58CC: 815D0000  lwz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B58D0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825B58D4: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 825B58D8: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 825B58DC: 419A0024  beq cr6, 0x825b5900
	if ctx.cr[6].eq {
	pc = 0x825B5900; continue 'dispatch;
	}
	// 825B58E0: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 825B58E4: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 825B58E8: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 825B58EC: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 825B58F0: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 825B58F4: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 825B58F8: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 825B58FC: 4082FFE8  bne 0x825b58e4
	if !ctx.cr[0].eq {
	pc = 0x825B58E4; continue 'dispatch;
	}
	// 825B5900: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 825B5904: 807D0000  lwz r3, 0(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B5908: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 825B590C: 4800098D  bl 0x825b6298
	ctx.lr = 0x825B5910;
	sub_825B6298(ctx, base);
	// 825B5910: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 825B5914: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825B5918: 419A0008  beq cr6, 0x825b5920
	if ctx.cr[6].eq {
	pc = 0x825B5920; continue 'dispatch;
	}
	// 825B591C: 4BD0AF75  bl 0x822c0890
	ctx.lr = 0x825B5920;
	sub_822C0890(ctx, base);
	// 825B5920: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825B5924: 4883DB05  bl 0x82df3428
	ctx.lr = 0x825B5928;
	sub_82DF3428(ctx, base);
	// 825B5928: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 825B592C: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 825B5930: 48BF2884  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B5938(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825B5938 size=196
    let mut pc: u32 = 0x825B5938;
    'dispatch: loop {
        match pc {
            0x825B5938 => {
    //   block [0x825B5938..0x825B59FC)
	// 825B5938: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B593C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825B5940: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825B5944: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825B5948: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B594C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 825B5950: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825B5954: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 825B5958: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 825B595C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825B5960: 4BD0AFD9  bl 0x822c0938
	ctx.lr = 0x825B5964;
	sub_822C0938(ctx, base);
	// 825B5964: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 825B5968: 41820028  beq 0x825b5990
	if ctx.cr[0].eq {
	pc = 0x825B5990; continue 'dispatch;
	}
	// 825B596C: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 825B5970: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 825B5974: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825B5978: 392BBDA0  addi r9, r11, -0x4260
	ctx.r[9].s64 = ctx.r[11].s64 + -16992;
	// 825B597C: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 825B5980: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 825B5984: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 825B5988: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 825B598C: 48000008  b 0x825b5994
	pc = 0x825B5994; continue 'dispatch;
	// 825B5990: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825B5994: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825B5998: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825B599C: 409A0044  bne cr6, 0x825b59e0
	if !ctx.cr[6].eq {
	pc = 0x825B59E0; continue 'dispatch;
	}
	// 825B59A0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 825B59A4: 419A001C  beq cr6, 0x825b59c0
	if ctx.cr[6].eq {
	pc = 0x825B59C0; continue 'dispatch;
	}
	// 825B59A8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B59AC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 825B59B0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B59B4: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B59B8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825B59BC: 4E800421  bctrl
	ctx.lr = 0x825B59C0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825B59C0: 3D608329  lis r11, -0x7cd7
	ctx.r[11].s64 = -2094465024;
	// 825B59C4: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 825B59C8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825B59CC: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 825B59D0: 816B9088  lwz r11, -0x6f78(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-28536 as u32) ) } as u64;
	// 825B59D4: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 825B59D8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825B59DC: 4BD0A625  bl 0x822c0000
	ctx.lr = 0x825B59E0;
	sub_822C0000(ctx, base);
	// 825B59E0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825B59E4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825B59E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825B59EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825B59F0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825B59F4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825B59F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B5A00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825B5A00 size=112
    let mut pc: u32 = 0x825B5A00;
    'dispatch: loop {
        match pc {
            0x825B5A00 => {
    //   block [0x825B5A00..0x825B5A70)
	// 825B5A00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B5A04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825B5A08: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825B5A0C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825B5A10: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B5A14: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825B5A18: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825B5A1C: 93C10050  stw r30, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 825B5A20: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 825B5A24: 4BFEC8DD  bl 0x825a2300
	ctx.lr = 0x825B5A28;
	sub_825A2300(ctx, base);
	// 825B5A28: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 825B5A2C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 825B5A30: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 825B5A34: 4BD0A5CD  bl 0x822c0000
	ctx.lr = 0x825B5A38;
	sub_822C0000(ctx, base);
	// 825B5A38: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 825B5A3C: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 825B5A40: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825B5A44: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 825B5A48: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825B5A4C: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 825B5A50: 419A0008  beq cr6, 0x825b5a58
	if ctx.cr[6].eq {
	pc = 0x825B5A58; continue 'dispatch;
	}
	// 825B5A54: 4BD0AE3D  bl 0x822c0890
	ctx.lr = 0x825B5A58;
	sub_822C0890(ctx, base);
	// 825B5A58: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825B5A5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825B5A60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825B5A64: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825B5A68: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825B5A6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B5A70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825B5A70 size=16
    let mut pc: u32 = 0x825B5A70;
    'dispatch: loop {
        match pc {
            0x825B5A70 => {
    //   block [0x825B5A70..0x825B5A80)
	// 825B5A70: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 825B5A74: 38830120  addi r4, r3, 0x120
	ctx.r[4].s64 = ctx.r[3].s64 + 288;
	// 825B5A78: 806B003C  lwz r3, 0x3c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 825B5A7C: 4BFFC394  b 0x825b1e10
	sub_825B1E10(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B5A80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825B5A80 size=196
    let mut pc: u32 = 0x825B5A80;
    'dispatch: loop {
        match pc {
            0x825B5A80 => {
    //   block [0x825B5A80..0x825B5B44)
	// 825B5A80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B5A84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825B5A88: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825B5A8C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825B5A90: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B5A94: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 825B5A98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825B5A9C: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 825B5AA0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 825B5AA4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825B5AA8: 4BD0AE91  bl 0x822c0938
	ctx.lr = 0x825B5AAC;
	sub_822C0938(ctx, base);
	// 825B5AAC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 825B5AB0: 41820028  beq 0x825b5ad8
	if ctx.cr[0].eq {
	pc = 0x825B5AD8; continue 'dispatch;
	}
	// 825B5AB4: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 825B5AB8: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 825B5ABC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825B5AC0: 392BBDD0  addi r9, r11, -0x4230
	ctx.r[9].s64 = ctx.r[11].s64 + -16944;
	// 825B5AC4: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 825B5AC8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 825B5ACC: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 825B5AD0: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 825B5AD4: 48000008  b 0x825b5adc
	pc = 0x825B5ADC; continue 'dispatch;
	// 825B5AD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825B5ADC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825B5AE0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825B5AE4: 409A0044  bne cr6, 0x825b5b28
	if !ctx.cr[6].eq {
	pc = 0x825B5B28; continue 'dispatch;
	}
	// 825B5AE8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 825B5AEC: 419A001C  beq cr6, 0x825b5b08
	if ctx.cr[6].eq {
	pc = 0x825B5B08; continue 'dispatch;
	}
	// 825B5AF0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B5AF4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 825B5AF8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B5AFC: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B5B00: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825B5B04: 4E800421  bctrl
	ctx.lr = 0x825B5B08;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825B5B08: 3D608329  lis r11, -0x7cd7
	ctx.r[11].s64 = -2094465024;
	// 825B5B0C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 825B5B10: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825B5B14: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 825B5B18: 816B90E8  lwz r11, -0x6f18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-28440 as u32) ) } as u64;
	// 825B5B1C: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 825B5B20: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825B5B24: 4BD0A4DD  bl 0x822c0000
	ctx.lr = 0x825B5B28;
	sub_822C0000(ctx, base);
	// 825B5B28: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825B5B2C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825B5B30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825B5B34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825B5B38: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825B5B3C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825B5B40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B5B48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x825B5B48 size=280
    let mut pc: u32 = 0x825B5B48;
    'dispatch: loop {
        match pc {
            0x825B5B48 => {
    //   block [0x825B5B48..0x825B5C60)
	// 825B5B48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B5B4C: 48BF261D  bl 0x831a8168
	ctx.lr = 0x825B5B50;
	sub_831A8130(ctx, base);
	// 825B5B50: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B5B54: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 825B5B58: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 825B5B5C: 93A10050  stw r29, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[29].u32 ) };
	// 825B5B60: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 825B5B64: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 825B5B68: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825B5B6C: 388BBDE8  addi r4, r11, -0x4218
	ctx.r[4].s64 = ctx.r[11].s64 + -16920;
	// 825B5B70: 38A00026  li r5, 0x26
	ctx.r[5].s64 = 38;
	// 825B5B74: 386000D0  li r3, 0xd0
	ctx.r[3].s64 = 208;
	// 825B5B78: 4883C871  bl 0x82df23e8
	ctx.lr = 0x825B5B7C;
	sub_82DF23E8(ctx, base);
	// 825B5B7C: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 825B5B80: 4182002C  beq 0x825b5bac
	if ctx.cr[0].eq {
	pc = 0x825B5BAC; continue 'dispatch;
	}
	// 825B5B84: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 825B5B88: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825B5B8C: 388BBDE0  addi r4, r11, -0x4220
	ctx.r[4].s64 = ctx.r[11].s64 + -16928;
	// 825B5B90: 4883DE79  bl 0x82df3a08
	ctx.lr = 0x825B5B94;
	sub_82DF3A08(ctx, base);
	// 825B5B94: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 825B5B98: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B5B9C: 3BA00001  li r29, 1
	ctx.r[29].s64 = 1;
	// 825B5BA0: 48002719  bl 0x825b82b8
	ctx.lr = 0x825B5BA4;
	sub_825B82B8(ctx, base);
	// 825B5BA4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825B5BA8: 48000008  b 0x825b5bb0
	pc = 0x825B5BB0; continue 'dispatch;
	// 825B5BAC: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 825B5BB0: 93E10058  stw r31, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[31].u32 ) };
	// 825B5BB4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825B5BB8: 3861005C  addi r3, r1, 0x5c
	ctx.r[3].s64 = ctx.r[1].s64 + 92;
	// 825B5BBC: 4BFFE85D  bl 0x825b4418
	ctx.lr = 0x825B5BC0;
	sub_825B4418(ctx, base);
	// 825B5BC0: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 825B5BC4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825B5BC8: 3861005C  addi r3, r1, 0x5c
	ctx.r[3].s64 = ctx.r[1].s64 + 92;
	// 825B5BCC: 4BD0A435  bl 0x822c0000
	ctx.lr = 0x825B5BD0;
	sub_822C0000(ctx, base);
	// 825B5BD0: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 825B5BD4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825B5BD8: 48001099  bl 0x825b6c70
	ctx.lr = 0x825B5BDC;
	sub_825B6C70(ctx, base);
	// 825B5BDC: 8161005C  lwz r11, 0x5c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 825B5BE0: 907E0114  stw r3, 0x114(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(276 as u32), ctx.r[3].u32 ) };
	// 825B5BE4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825B5BE8: 419A000C  beq cr6, 0x825b5bf4
	if ctx.cr[6].eq {
	pc = 0x825B5BF4; continue 'dispatch;
	}
	// 825B5BEC: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 825B5BF0: 4BD0ACA1  bl 0x822c0890
	ctx.lr = 0x825B5BF4;
	sub_822C0890(ctx, base);
	// 825B5BF4: 57AB07FF  clrlwi. r11, r29, 0x1f
	ctx.r[11].u64 = ctx.r[29].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825B5BF8: 4182000C  beq 0x825b5c04
	if ctx.cr[0].eq {
	pc = 0x825B5C04; continue 'dispatch;
	}
	// 825B5BFC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825B5C00: 4883D829  bl 0x82df3428
	ctx.lr = 0x825B5C04;
	sub_82DF3428(ctx, base);
	// 825B5C04: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 825B5C08: 809C003C  lwz r4, 0x3c(r28)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(60 as u32) ) } as u64;
	// 825B5C0C: 4BFFC1F5  bl 0x825b1e00
	ctx.lr = 0x825B5C10;
	sub_825B1E00(ctx, base);
	// 825B5C10: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 825B5C14: 39400120  li r10, 0x120
	ctx.r[10].s64 = 288;
	// 825B5C18: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 825B5C1C: 13E058C7  vcmpequd (lvx128) v31, v0, v11
	tmp.u32 = ctx.r[11].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B5C60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825B5C60 size=100
    let mut pc: u32 = 0x825B5C60;
    'dispatch: loop {
        match pc {
            0x825B5C60 => {
    //   block [0x825B5C60..0x825B5CC4)
	// 825B5C60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B5C64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825B5C68: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825B5C6C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825B5C70: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B5C74: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825B5C78: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 825B5C7C: 817F0110  lwz r11, 0x110(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(272 as u32) ) } as u64;
	// 825B5C80: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 825B5C84: 409A0010  bne cr6, 0x825b5c94
	if !ctx.cr[6].eq {
	pc = 0x825B5C94; continue 'dispatch;
	}
	// 825B5C88: 387F0138  addi r3, r31, 0x138
	ctx.r[3].s64 = ctx.r[31].s64 + 312;
	// 825B5C8C: 809F0130  lwz r4, 0x130(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(304 as u32) ) } as u64;
	// 825B5C90: 4BFFBC39  bl 0x825b18c8
	ctx.lr = 0x825B5C94;
	sub_825B18C8(ctx, base);
	// 825B5C94: 817F0114  lwz r11, 0x114(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(276 as u32) ) } as u64;
	// 825B5C98: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 825B5C9C: 409A0010  bne cr6, 0x825b5cac
	if !ctx.cr[6].eq {
	pc = 0x825B5CAC; continue 'dispatch;
	}
	// 825B5CA0: 387F0138  addi r3, r31, 0x138
	ctx.r[3].s64 = ctx.r[31].s64 + 312;
	// 825B5CA4: 809F0130  lwz r4, 0x130(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(304 as u32) ) } as u64;
	// 825B5CA8: 4BFFBC21  bl 0x825b18c8
	ctx.lr = 0x825B5CAC;
	sub_825B18C8(ctx, base);
	// 825B5CAC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825B5CB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825B5CB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825B5CB8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825B5CBC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825B5CC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B5CC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825B5CC8 size=88
    let mut pc: u32 = 0x825B5CC8;
    'dispatch: loop {
        match pc {
            0x825B5CC8 => {
    //   block [0x825B5CC8..0x825B5D20)
	// 825B5CC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B5CCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825B5CD0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825B5CD4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825B5CD8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B5CDC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825B5CE0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825B5CE4: 387F0138  addi r3, r31, 0x138
	ctx.r[3].s64 = ctx.r[31].s64 + 312;
	// 825B5CE8: 4BD12FD1  bl 0x822c8cb8
	ctx.lr = 0x825B5CEC;
	sub_822C8CB8(ctx, base);
	// 825B5CEC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B5CF0: 4BFFE6B1  bl 0x825b43a0
	ctx.lr = 0x825B5CF4;
	sub_825B43A0(ctx, base);
	// 825B5CF4: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825B5CF8: 4182000C  beq 0x825b5d04
	if ctx.cr[0].eq {
	pc = 0x825B5D04; continue 'dispatch;
	}
	// 825B5CFC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B5D00: 4BD0A569  bl 0x822c0268
	ctx.lr = 0x825B5D04;
	sub_822C0268(ctx, base);
	// 825B5D04: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B5D08: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825B5D0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825B5D10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825B5D14: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825B5D18: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825B5D1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B5D20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x825B5D20 size=396
    let mut pc: u32 = 0x825B5D20;
    'dispatch: loop {
        match pc {
            0x825B5D20 => {
    //   block [0x825B5D20..0x825B5EAC)
	// 825B5D20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B5D24: 48BF2441  bl 0x831a8164
	ctx.lr = 0x825B5D28;
	sub_831A8130(ctx, base);
	// 825B5D28: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B5D2C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 825B5D30: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 825B5D34: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 825B5D38: 3D008204  lis r8, -0x7dfc
	ctx.r[8].s64 = -2113667072;
	// 825B5D3C: 3CE08200  lis r7, -0x7e00
	ctx.r[7].s64 = -2113929216;
	// 825B5D40: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 825B5D44: C1AB6218  lfs f13, 0x6218(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(25112 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825B5D48: C18A08AC  lfs f12, 0x8ac(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2220 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 825B5D4C: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 825B5D50: C00908A4  lfs f0, 0x8a4(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825B5D54: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 825B5D58: C168BEB4  lfs f11, -0x414c(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(-16716 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 825B5D5C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825B5D60: C147093C  lfs f10, 0x93c(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(2364 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 825B5D64: 93A10054  stw r29, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[29].u32 ) };
	// 825B5D68: D1A10070  stfs f13, 0x70(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), tmp.u32 ) };
	// 825B5D6C: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 825B5D70: D1810074  stfs f12, 0x74(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), tmp.u32 ) };
	// 825B5D74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825B5D78: D0010078  stfs f0, 0x78(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), tmp.u32 ) };
	// 825B5D7C: 388BBDE8  addi r4, r11, -0x4218
	ctx.r[4].s64 = ctx.r[11].s64 + -16920;
	// 825B5D80: D001007C  stfs f0, 0x7c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), tmp.u32 ) };
	// 825B5D84: 38A00013  li r5, 0x13
	ctx.r[5].s64 = 19;
	// 825B5D88: D1610060  stfs f11, 0x60(r1)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), tmp.u32 ) };
	// 825B5D8C: 38600160  li r3, 0x160
	ctx.r[3].s64 = 352;
	// 825B5D90: D1410064  stfs f10, 0x64(r1)
	tmp.f32 = (ctx.f[10].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), tmp.u32 ) };
	// 825B5D94: D0010068  stfs f0, 0x68(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), tmp.u32 ) };
	// 825B5D98: D001006C  stfs f0, 0x6c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), tmp.u32 ) };
	// 825B5D9C: 4BD0A63D  bl 0x822c03d8
	ctx.lr = 0x825B5DA0;
	sub_822C03D8(ctx, base);
	// 825B5DA0: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 825B5DA4: 41820054  beq 0x825b5df8
	if ctx.cr[0].eq {
	pc = 0x825B5DF8; continue 'dispatch;
	}
	// 825B5DA8: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 825B5DAC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825B5DB0: 388BBEAC  addi r4, r11, -0x4154
	ctx.r[4].s64 = ctx.r[11].s64 + -16724;
	// 825B5DB4: 4883DC55  bl 0x82df3a08
	ctx.lr = 0x825B5DB8;
	sub_82DF3A08(ctx, base);
	// 825B5DB8: 38C10060  addi r6, r1, 0x60
	ctx.r[6].s64 = ctx.r[1].s64 + 96;
	// 825B5DBC: 38A10070  addi r5, r1, 0x70
	ctx.r[5].s64 = ctx.r[1].s64 + 112;
	// 825B5DC0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 825B5DC4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B5DC8: 3BA00001  li r29, 1
	ctx.r[29].s64 = 1;
	// 825B5DCC: 48000FF5  bl 0x825b6dc0
	ctx.lr = 0x825B5DD0;
	sub_825B6DC0(ctx, base);
	// 825B5DD0: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 825B5DD4: 39402710  li r10, 0x2710
	ctx.r[10].s64 = 10000;
	// 825B5DD8: 937F0130  stw r27, 0x130(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(304 as u32), ctx.r[27].u32 ) };
	// 825B5DDC: 396BBE3C  addi r11, r11, -0x41c4
	ctx.r[11].s64 = ctx.r[11].s64 + -16836;
	// 825B5DE0: 915F0110  stw r10, 0x110(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(272 as u32), ctx.r[10].u32 ) };
	// 825B5DE4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 825B5DE8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825B5DEC: 387F0138  addi r3, r31, 0x138
	ctx.r[3].s64 = ctx.r[31].s64 + 312;
	// 825B5DF0: 4BFD6121  bl 0x8258bf10
	ctx.lr = 0x825B5DF4;
	sub_8258BF10(ctx, base);
	// 825B5DF4: 48000008  b 0x825b5dfc
	pc = 0x825B5DFC; continue 'dispatch;
	// 825B5DF8: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 825B5DFC: 93FC0000  stw r31, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 825B5E00: 3BDC0004  addi r30, r28, 4
	ctx.r[30].s64 = ctx.r[28].s64 + 4;
	// 825B5E04: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825B5E08: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825B5E0C: 4BFFFC75  bl 0x825b5a80
	ctx.lr = 0x825B5E10;
	sub_825B5A80(ctx, base);
	// 825B5E10: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 825B5E14: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825B5E18: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825B5E1C: 4BD0A1E5  bl 0x822c0000
	ctx.lr = 0x825B5E20;
	sub_822C0000(ctx, base);
	// 825B5E20: 57AB07FF  clrlwi. r11, r29, 0x1f
	ctx.r[11].u64 = ctx.r[29].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825B5E24: 4182000C  beq 0x825b5e30
	if ctx.cr[0].eq {
	pc = 0x825B5E30; continue 'dispatch;
	}
	// 825B5E28: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825B5E2C: 4883D5FD  bl 0x82df3428
	ctx.lr = 0x825B5E30;
	sub_82DF3428(ctx, base);
	// 825B5E30: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 825B5E34: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 825B5E38: 388BBEA0  addi r4, r11, -0x4160
	ctx.r[4].s64 = ctx.r[11].s64 + -16736;
	// 825B5E3C: 4883DBCD  bl 0x82df3a08
	ctx.lr = 0x825B5E40;
	sub_82DF3A08(ctx, base);
	// 825B5E40: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B5E44: 815C0000  lwz r10, 0(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B5E48: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825B5E4C: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 825B5E50: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 825B5E54: 419A0024  beq cr6, 0x825b5e78
	if ctx.cr[6].eq {
	pc = 0x825B5E78; continue 'dispatch;
	}
	// 825B5E58: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 825B5E5C: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 825B5E60: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 825B5E64: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 825B5E68: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 825B5E6C: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 825B5E70: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 825B5E74: 4082FFE8  bne 0x825b5e5c
	if !ctx.cr[0].eq {
	pc = 0x825B5E5C; continue 'dispatch;
	}
	// 825B5E78: 38A10054  addi r5, r1, 0x54
	ctx.r[5].s64 = ctx.r[1].s64 + 84;
	// 825B5E7C: 807C0000  lwz r3, 0(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B5E80: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 825B5E84: 48000415  bl 0x825b6298
	ctx.lr = 0x825B5E88;
	sub_825B6298(ctx, base);
	// 825B5E88: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 825B5E8C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825B5E90: 419A0008  beq cr6, 0x825b5e98
	if ctx.cr[6].eq {
	pc = 0x825B5E98; continue 'dispatch;
	}
	// 825B5E94: 4BD0A9FD  bl 0x822c0890
	ctx.lr = 0x825B5E98;
	sub_822C0890(ctx, base);
	// 825B5E98: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 825B5E9C: 4883D58D  bl 0x82df3428
	ctx.lr = 0x825B5EA0;
	sub_82DF3428(ctx, base);
	// 825B5EA0: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 825B5EA4: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 825B5EA8: 48BF230C  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B5EB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825B5EB0 size=196
    let mut pc: u32 = 0x825B5EB0;
    'dispatch: loop {
        match pc {
            0x825B5EB0 => {
    //   block [0x825B5EB0..0x825B5F74)
	// 825B5EB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B5EB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825B5EB8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825B5EBC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825B5EC0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B5EC4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 825B5EC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825B5ECC: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 825B5ED0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 825B5ED4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825B5ED8: 4BD0AA61  bl 0x822c0938
	ctx.lr = 0x825B5EDC;
	sub_822C0938(ctx, base);
	// 825B5EDC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 825B5EE0: 41820028  beq 0x825b5f08
	if ctx.cr[0].eq {
	pc = 0x825B5F08; continue 'dispatch;
	}
	// 825B5EE4: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 825B5EE8: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 825B5EEC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825B5EF0: 392BBEC8  addi r9, r11, -0x4138
	ctx.r[9].s64 = ctx.r[11].s64 + -16696;
	// 825B5EF4: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 825B5EF8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 825B5EFC: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 825B5F00: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 825B5F04: 48000008  b 0x825b5f0c
	pc = 0x825B5F0C; continue 'dispatch;
	// 825B5F08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825B5F0C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825B5F10: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825B5F14: 409A0044  bne cr6, 0x825b5f58
	if !ctx.cr[6].eq {
	pc = 0x825B5F58; continue 'dispatch;
	}
	// 825B5F18: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 825B5F1C: 419A001C  beq cr6, 0x825b5f38
	if ctx.cr[6].eq {
	pc = 0x825B5F38; continue 'dispatch;
	}
	// 825B5F20: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B5F24: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 825B5F28: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B5F2C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B5F30: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825B5F34: 4E800421  bctrl
	ctx.lr = 0x825B5F38;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825B5F38: 3D608329  lis r11, -0x7cd7
	ctx.r[11].s64 = -2094465024;
	// 825B5F3C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 825B5F40: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825B5F44: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 825B5F48: 816B917C  lwz r11, -0x6e84(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-28292 as u32) ) } as u64;
	// 825B5F4C: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 825B5F50: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825B5F54: 4BD0A0AD  bl 0x822c0000
	ctx.lr = 0x825B5F58;
	sub_822C0000(ctx, base);
	// 825B5F58: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825B5F5C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825B5F60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825B5F64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825B5F68: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825B5F6C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825B5F70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B5F78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825B5F78 size=52
    let mut pc: u32 = 0x825B5F78;
    'dispatch: loop {
        match pc {
            0x825B5F78 => {
    //   block [0x825B5F78..0x825B5FAC)
	// 825B5F78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B5F7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825B5F80: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825B5F84: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B5F88: 388400A0  addi r4, r4, 0xa0
	ctx.r[4].s64 = ctx.r[4].s64 + 160;
	// 825B5F8C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825B5F90: 4883DC71  bl 0x82df3c00
	ctx.lr = 0x825B5F94;
	sub_82DF3C00(ctx, base);
	// 825B5F94: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B5F98: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825B5F9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825B5FA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825B5FA4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825B5FA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B5FB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825B5FB0 size=8
    let mut pc: u32 = 0x825B5FB0;
    'dispatch: loop {
        match pc {
            0x825B5FB0 => {
    //   block [0x825B5FB0..0x825B5FB8)
	// 825B5FB0: 988300E4  stb r4, 0xe4(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(228 as u32), ctx.r[4].u8 ) };
	// 825B5FB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B5FB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825B5FB8 size=12
    let mut pc: u32 = 0x825B5FB8;
    'dispatch: loop {
        match pc {
            0x825B5FB8 => {
    //   block [0x825B5FB8..0x825B5FC4)
	// 825B5FB8: 816300B8  lwz r11, 0xb8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(184 as u32) ) } as u64;
	// 825B5FBC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825B5FC0: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B5FC4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825B5FC4 size=20
    let mut pc: u32 = 0x825B5FC4;
    'dispatch: loop {
        match pc {
            0x825B5FC4 => {
    //   block [0x825B5FC4..0x825B5FD8)
	// 825B5FC4: 814300BC  lwz r10, 0xbc(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(188 as u32) ) } as u64;
	// 825B5FC8: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 825B5FCC: 7D6B1E70  srawi r11, r11, 3
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 3) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 3) as i64;
	// 825B5FD0: 7F0B2040  cmplw cr6, r11, r4
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[4].u32, &mut ctx.xer);
	// 825B5FD4: 4C990020  blelr cr6
	if !ctx.cr[6].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B5FD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825B5FD8 size=48
    let mut pc: u32 = 0x825B5FD8;
    'dispatch: loop {
        match pc {
            0x825B5FD8 => {
    //   block [0x825B5FD8..0x825B6008)
	// 825B5FD8: 812300B8  lwz r9, 0xb8(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(184 as u32) ) } as u64;
	// 825B5FDC: 54881838  slwi r8, r4, 3
	ctx.r[8].u32 = ctx.r[4].u32.wrapping_shl(3);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 825B5FE0: 394000FF  li r10, 0xff
	ctx.r[10].s64 = 255;
	// 825B5FE4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825B5FE8: 9941FFF0  stb r10, -0x10(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[10].u8 ) };
	// 825B5FEC: 9941FFF1  stb r10, -0xf(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(-15 as u32), ctx.r[10].u8 ) };
	// 825B5FF0: 7D49402E  lwzx r10, r9, r8
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 825B5FF4: 9961FFF2  stb r11, -0xe(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(-14 as u32), ctx.r[11].u8 ) };
	// 825B5FF8: 9961FFF3  stb r11, -0xd(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(-13 as u32), ctx.r[11].u8 ) };
	// 825B5FFC: 8161FFF0  lwz r11, -0x10(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) } as u64;
	// 825B6000: 916A00B8  stw r11, 0xb8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(184 as u32), ctx.r[11].u32 ) };
	// 825B6004: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B6008(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825B6008 size=48
    let mut pc: u32 = 0x825B6008;
    'dispatch: loop {
        match pc {
            0x825B6008 => {
    //   block [0x825B6008..0x825B6038)
	// 825B6008: 814300AC  lwz r10, 0xac(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(172 as u32) ) } as u64;
	// 825B600C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825B6010: 392000FF  li r9, 0xff
	ctx.r[9].s64 = 255;
	// 825B6014: 9961FFF1  stb r11, -0xf(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(-15 as u32), ctx.r[11].u8 ) };
	// 825B6018: 9921FFF0  stb r9, -0x10(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[9].u8 ) };
	// 825B601C: 9961FFF2  stb r11, -0xe(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(-14 as u32), ctx.r[11].u8 ) };
	// 825B6020: 9961FFF3  stb r11, -0xd(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(-13 as u32), ctx.r[11].u8 ) };
	// 825B6024: 8101FFF0  lwz r8, -0x10(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) } as u64;
	// 825B6028: 910A00B8  stw r8, 0xb8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(184 as u32), ctx.r[8].u32 ) };
	// 825B602C: 814300B8  lwz r10, 0xb8(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(184 as u32) ) } as u64;
	// 825B6030: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 825B6034: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B6038(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825B6038 size=20
    let mut pc: u32 = 0x825B6038;
    'dispatch: loop {
        match pc {
            0x825B6038 => {
    //   block [0x825B6038..0x825B604C)
	// 825B6038: 810300BC  lwz r8, 0xbc(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(188 as u32) ) } as u64;
	// 825B603C: 7D4A4050  subf r10, r10, r8
	ctx.r[10].s64 = ctx.r[8].s64 - ctx.r[10].s64;
	// 825B6040: 7D4A1E70  srawi r10, r10, 3
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 3) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[10].s32 >> 3) as i64;
	// 825B6044: 7F0A2040  cmplw cr6, r10, r4
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[4].u32, &mut ctx.xer);
	// 825B6048: 4C990020  blelr cr6
	if !ctx.cr[6].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B604C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825B604C size=40
    let mut pc: u32 = 0x825B604C;
    'dispatch: loop {
        match pc {
            0x825B604C => {
    //   block [0x825B604C..0x825B6074)
	// 825B604C: 9961FFF3  stb r11, -0xd(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(-13 as u32), ctx.r[11].u8 ) };
	// 825B6050: 548A1838  slwi r10, r4, 3
	ctx.r[10].u32 = ctx.r[4].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 825B6054: 9921FFF0  stb r9, -0x10(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[9].u8 ) };
	// 825B6058: 812300B8  lwz r9, 0xb8(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(184 as u32) ) } as u64;
	// 825B605C: 9961FFF1  stb r11, -0xf(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(-15 as u32), ctx.r[11].u8 ) };
	// 825B6060: 9961FFF2  stb r11, -0xe(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(-14 as u32), ctx.r[11].u8 ) };
	// 825B6064: 7D69502E  lwzx r11, r9, r10
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 825B6068: 8141FFF0  lwz r10, -0x10(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) } as u64;
	// 825B606C: 914B00B8  stw r10, 0xb8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(184 as u32), ctx.r[10].u32 ) };
	// 825B6070: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B6078(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825B6078 size=84
    let mut pc: u32 = 0x825B6078;
    'dispatch: loop {
        match pc {
            0x825B6078 => {
    //   block [0x825B6078..0x825B60CC)
	// 825B6078: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B607C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825B6080: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B6084: 7C671B78  mr r7, r3
	ctx.r[7].u64 = ctx.r[3].u64;
	// 825B6088: 394000FF  li r10, 0xff
	ctx.r[10].s64 = 255;
	// 825B608C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825B6090: 99410050  stb r10, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u8 ) };
	// 825B6094: 99410051  stb r10, 0x51(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(81 as u32), ctx.r[10].u8 ) };
	// 825B6098: 812700AC  lwz r9, 0xac(r7)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(172 as u32) ) } as u64;
	// 825B609C: 99610052  stb r11, 0x52(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(82 as u32), ctx.r[11].u8 ) };
	// 825B60A0: 99610053  stb r11, 0x53(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(83 as u32), ctx.r[11].u8 ) };
	// 825B60A4: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 825B60A8: 916900B8  stw r11, 0xb8(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(184 as u32), ctx.r[11].u32 ) };
	// 825B60AC: 808700D4  lwz r4, 0xd4(r7)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(212 as u32) ) } as u64;
	// 825B60B0: 4BFFFF09  bl 0x825b5fb8
	ctx.lr = 0x825B60B4;
	sub_825B5FB8(ctx, base);
	// 825B60B4: 816700D4  lwz r11, 0xd4(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(212 as u32) ) } as u64;
	// 825B60B8: 91670104  stw r11, 0x104(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(260 as u32), ctx.r[11].u32 ) };
	// 825B60BC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825B60C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825B60C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825B60C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B60D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825B60D0 size=80
    let mut pc: u32 = 0x825B60D0;
    'dispatch: loop {
        match pc {
            0x825B60D0 => {
    //   block [0x825B60D0..0x825B6120)
	// 825B60D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B60D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825B60D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B60DC: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 825B60E0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825B60E4: 394000FF  li r10, 0xff
	ctx.r[10].s64 = 255;
	// 825B60E8: 98E10051  stb r7, 0x51(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(81 as u32), ctx.r[7].u8 ) };
	// 825B60EC: 99410050  stb r10, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u8 ) };
	// 825B60F0: 816600AC  lwz r11, 0xac(r6)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(172 as u32) ) } as u64;
	// 825B60F4: 98E10052  stb r7, 0x52(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(82 as u32), ctx.r[7].u8 ) };
	// 825B60F8: 98E10053  stb r7, 0x53(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(83 as u32), ctx.r[7].u8 ) };
	// 825B60FC: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 825B6100: 914B00B8  stw r10, 0xb8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(184 as u32), ctx.r[10].u32 ) };
	// 825B6104: 808600D4  lwz r4, 0xd4(r6)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(212 as u32) ) } as u64;
	// 825B6108: 4BFFFF01  bl 0x825b6008
	ctx.lr = 0x825B610C;
	sub_825B6008(ctx, base);
	// 825B610C: 90E60104  stw r7, 0x104(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(260 as u32), ctx.r[7].u32 ) };
	// 825B6110: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825B6114: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825B6118: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825B611C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B6120(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x825B6120 size=68
    let mut pc: u32 = 0x825B6120;
    'dispatch: loop {
        match pc {
            0x825B6120 => {
    //   block [0x825B6120..0x825B6164)
	// 825B6120: 39640001  addi r11, r4, 1
	ctx.r[11].s64 = ctx.r[4].s64 + 1;
	// 825B6124: 814300AC  lwz r10, 0xac(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(172 as u32) ) } as u64;
	// 825B6128: 3D208204  lis r9, -0x7dfc
	ctx.r[9].s64 = -2113667072;
	// 825B612C: 908300D4  stw r4, 0xd4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(212 as u32), ctx.r[4].u32 ) };
	// 825B6130: 796B0020  clrldi r11, r11, 0x20
	ctx.r[11].u64 = ctx.r[11].u64 & 0x00000000FFFFFFFFu64;
	// 825B6134: F961FFF0  std r11, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[11].u64 ) };
	// 825B6138: C801FFF0  lfd f0, -0x10(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825B613C: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 825B6140: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 825B6144: FD800018  frsp f12, f0
	ctx.f[12].f64 = (ctx.f[0].f64 as f32) as f64;
	// 825B6148: C1A9BED8  lfs f13, -0x4128(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-16680 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825B614C: C00B89AC  lfs f0, -0x7654(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-30292 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825B6150: D00A0030  stfs f0, 0x30(r10)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(48 as u32), tmp.u32 ) };
	// 825B6154: EDAC0372  fmuls f13, f12, f13
	ctx.f[13].f64 = (((ctx.f[12].f64 * ctx.f[13].f64) as f32) as f64);
	// 825B6158: EC0D002A  fadds f0, f13, f0
	ctx.f[0].f64 = ((ctx.f[13].f64 + ctx.f[0].f64) as f32) as f64;
	// 825B615C: D00A0034  stfs f0, 0x34(r10)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(52 as u32), tmp.u32 ) };
	// 825B6160: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B6168(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825B6168 size=92
    let mut pc: u32 = 0x825B6168;
    'dispatch: loop {
        match pc {
            0x825B6168 => {
    //   block [0x825B6168..0x825B61C4)
	// 825B6168: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B616C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825B6170: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825B6174: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B6178: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825B617C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825B6180: 4BD12B99  bl 0x822c8d18
	ctx.lr = 0x825B6184;
	sub_822C8D18(ctx, base);
	// 825B6184: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B6188: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825B618C: 386BFFFC  addi r3, r11, -4
	ctx.r[3].s64 = ctx.r[11].s64 + -4;
	// 825B6190: 409A0008  bne cr6, 0x825b6198
	if !ctx.cr[6].eq {
	pc = 0x825B6198; continue 'dispatch;
	}
	// 825B6194: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 825B6198: 809F00E0  lwz r4, 0xe0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(224 as u32) ) } as u64;
	// 825B619C: 48B0A0D5  bl 0x830c0270
	ctx.lr = 0x825B61A0;
	sub_830C0270(ctx, base);
	// 825B61A0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825B61A4: 4883BAED  bl 0x82df1c90
	ctx.lr = 0x825B61A8;
	sub_82DF1C90(ctx, base);
	// 825B61A8: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 825B61AC: 997F00DC  stb r11, 0xdc(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(220 as u32), ctx.r[11].u8 ) };
	// 825B61B0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825B61B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825B61B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825B61BC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825B61C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B61C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825B61C8 size=40
    let mut pc: u32 = 0x825B61C8;
    'dispatch: loop {
        match pc {
            0x825B61C8 => {
    //   block [0x825B61C8..0x825B61F0)
	// 825B61C8: 816400D4  lwz r11, 0xd4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(212 as u32) ) } as u64;
	// 825B61CC: 814400C8  lwz r10, 0xc8(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(200 as u32) ) } as u64;
	// 825B61D0: 556B1838  slwi r11, r11, 3
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 825B61D4: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 825B61D8: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B61DC: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 825B61E0: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 825B61E4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825B61E8: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 825B61EC: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B61F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825B61F0 size=36
    let mut pc: u32 = 0x825B61F0;
    'dispatch: loop {
        match pc {
            0x825B61F0 => {
    //   block [0x825B61F0..0x825B6214)
	// 825B61F0: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 825B61F4: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 825B61F8: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 825B61FC: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 825B6200: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 825B6204: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 825B6208: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 825B620C: 4082FFE8  bne 0x825b61f4
	if !ctx.cr[0].eq {
	pc = 0x825B61F4; continue 'dispatch;
	}
	// 825B6210: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B6218(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825B6218 size=44
    let mut pc: u32 = 0x825B6218;
    'dispatch: loop {
        match pc {
            0x825B6218 => {
    //   block [0x825B6218..0x825B6244)
	// 825B6218: 81440004  lwz r10, 4(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 825B621C: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 825B6220: 892B0039  lbz r9, 0x39(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(57 as u32) ) } as u64;
	// 825B6224: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 825B6228: 409A0030  bne cr6, 0x825b6258
	if !ctx.cr[6].eq {
		sub_825B6244(ctx, base);
		return;
	}
	// 825B622C: 81250000  lwz r9, 0(r5)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B6230: 810B0010  lwz r8, 0x10(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 825B6234: 7F084840  cmplw cr6, r8, r9
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[9].u32, &mut ctx.xer);
	// 825B6238: 4098000C  bge cr6, 0x825b6244
	if !ctx.cr[6].lt {
		sub_825B6244(ctx, base);
		return;
	}
	// 825B623C: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 825B6240: 4800000C  b 0x825b624c
	sub_825B6244(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B6244(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825B6244 size=60
    let mut pc: u32 = 0x825B6244;
    'dispatch: loop {
        match pc {
            0x825B6244 => {
    //   block [0x825B6244..0x825B6280)
	// 825B6244: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 825B6248: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B624C: 890B0039  lbz r8, 0x39(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(57 as u32) ) } as u64;
	// 825B6250: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 825B6254: 419AFFDC  beq cr6, 0x825b6230
	if ctx.cr[6].eq {
		sub_825B6218(ctx, base);
		return;
	}
	// 825B6258: 81640004  lwz r11, 4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 825B625C: 9141FFF0  stw r10, -0x10(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[10].u32 ) };
	// 825B6260: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 825B6264: 419A001C  beq cr6, 0x825b6280
	if ctx.cr[6].eq {
		sub_825B6280(ctx, base);
		return;
	}
	// 825B6268: 81250000  lwz r9, 0(r5)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B626C: 814A0010  lwz r10, 0x10(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 825B6270: 7F095040  cmplw cr6, r9, r10
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[10].u32, &mut ctx.xer);
	// 825B6274: 4198000C  blt cr6, 0x825b6280
	if ctx.cr[6].lt {
		sub_825B6280(ctx, base);
		return;
	}
	// 825B6278: 3961FFF0  addi r11, r1, -0x10
	ctx.r[11].s64 = ctx.r[1].s64 + -16;
	// 825B627C: 4800000C  b 0x825b6288
	sub_825B6280(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B6280(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825B6280 size=20
    let mut pc: u32 = 0x825B6280;
    'dispatch: loop {
        match pc {
            0x825B6280 => {
    //   block [0x825B6280..0x825B6294)
	// 825B6280: 9161FFF4  stw r11, -0xc(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-12 as u32), ctx.r[11].u32 ) };
	// 825B6284: 3961FFF4  addi r11, r1, -0xc
	ctx.r[11].s64 = ctx.r[1].s64 + -12;
	// 825B6288: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B628C: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825B6290: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B6298(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825B6298 size=140
    let mut pc: u32 = 0x825B6298;
    'dispatch: loop {
        match pc {
            0x825B6298 => {
    //   block [0x825B6298..0x825B6324)
	// 825B6298: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B629C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825B62A0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825B62A4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825B62A8: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B62AC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825B62B0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825B62B4: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 825B62B8: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 825B62BC: 488434CD  bl 0x82df9788
	ctx.lr = 0x825B62C0;
	sub_82DF9788(ctx, base);
	// 825B62C0: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 825B62C4: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B62C8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825B62CC: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825B62D0: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 825B62D4: 419A0024  beq cr6, 0x825b62f8
	if ctx.cr[6].eq {
	pc = 0x825B62F8; continue 'dispatch;
	}
	// 825B62D8: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 825B62DC: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 825B62E0: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 825B62E4: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 825B62E8: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 825B62EC: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 825B62F0: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 825B62F4: 4082FFE8  bne 0x825b62dc
	if !ctx.cr[0].eq {
	pc = 0x825B62DC; continue 'dispatch;
	}
	// 825B62F8: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 825B62FC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 825B6300: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B6304: 48B0C67D  bl 0x830c2980
	ctx.lr = 0x825B6308;
	sub_830C2980(ctx, base);
	// 825B6308: 907F00E0  stw r3, 0xe0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(224 as u32), ctx.r[3].u32 ) };
	// 825B630C: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 825B6310: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825B6314: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825B6318: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825B631C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825B6320: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B6328(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825B6328 size=112
    let mut pc: u32 = 0x825B6328;
    'dispatch: loop {
        match pc {
            0x825B6328 => {
    //   block [0x825B6328..0x825B6398)
	// 825B6328: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B632C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825B6330: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825B6334: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825B6338: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B633C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825B6340: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825B6344: 387F00A0  addi r3, r31, 0xa0
	ctx.r[3].s64 = ctx.r[31].s64 + 160;
	// 825B6348: 4883D889  bl 0x82df3bd0
	ctx.lr = 0x825B634C;
	sub_82DF3BD0(ctx, base);
	// 825B634C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 825B6350: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825B6354: 48843435  bl 0x82df9788
	ctx.lr = 0x825B6358;
	sub_82DF9788(ctx, base);
	// 825B6358: 817F00A4  lwz r11, 0xa4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 825B635C: 38C0FFFF  li r6, -1
	ctx.r[6].s64 = -1;
	// 825B6360: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825B6364: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 825B6368: 386B0090  addi r3, r11, 0x90
	ctx.r[3].s64 = ctx.r[11].s64 + 144;
	// 825B636C: 4BD12DC5  bl 0x822c9130
	ctx.lr = 0x825B6370;
	sub_822C9130(ctx, base);
	// 825B6370: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825B6374: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 825B6378: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825B637C: 4BD12AB5  bl 0x822c8e30
	ctx.lr = 0x825B6380;
	sub_822C8E30(ctx, base);
	// 825B6380: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 825B6384: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825B6388: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825B638C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825B6390: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825B6394: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B6398(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825B6398 size=336
    let mut pc: u32 = 0x825B6398;
    'dispatch: loop {
        match pc {
            0x825B6398 => {
    //   block [0x825B6398..0x825B64E8)
	// 825B6398: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B639C: 48BF1DC5  bl 0x831a8160
	ctx.lr = 0x825B63A0;
	sub_831A8130(ctx, base);
	// 825B63A0: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B63A4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825B63A8: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 825B63AC: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 825B63B0: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 825B63B4: 817F00C8  lwz r11, 0xc8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(200 as u32) ) } as u64;
	// 825B63B8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825B63BC: 419A0124  beq cr6, 0x825b64e0
	if ctx.cr[6].eq {
	pc = 0x825B64E0; continue 'dispatch;
	}
	// 825B63C0: 815F00CC  lwz r10, 0xcc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(204 as u32) ) } as u64;
	// 825B63C4: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 825B63C8: 7D6B1E70  srawi r11, r11, 3
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 3) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 3) as i64;
	// 825B63CC: 7F1D5840  cmplw cr6, r29, r11
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[11].u32, &mut ctx.xer);
	// 825B63D0: 40980110  bge cr6, 0x825b64e0
	if !ctx.cr[6].lt {
	pc = 0x825B64E0; continue 'dispatch;
	}
	// 825B63D4: 817F00C8  lwz r11, 0xc8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(200 as u32) ) } as u64;
	// 825B63D8: 57BE1838  slwi r30, r29, 3
	ctx.r[30].u32 = ctx.r[29].u32.wrapping_shl(3);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 825B63DC: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 825B63E0: 7C8BF02E  lwzx r4, r11, r30
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 825B63E4: 48016C85  bl 0x825cd068
	ctx.lr = 0x825B63E8;
	sub_825CD068(ctx, base);
	// 825B63E8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 825B63EC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825B63F0: 388B9BC9  addi r4, r11, -0x6437
	ctx.r[4].s64 = ctx.r[11].s64 + -25655;
	// 825B63F4: 4883D615  bl 0x82df3a08
	ctx.lr = 0x825B63F8;
	sub_82DF3A08(ctx, base);
	// 825B63F8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 825B63FC: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 825B6400: 4883CEA1  bl 0x82df32a0
	ctx.lr = 0x825B6404;
	sub_82DF32A0(ctx, base);
	// 825B6404: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 825B6408: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825B640C: 4883D01D  bl 0x82df3428
	ctx.lr = 0x825B6410;
	sub_82DF3428(ctx, base);
	// 825B6410: 574B063F  clrlwi. r11, r26, 0x18
	ctx.r[11].u64 = ctx.r[26].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825B6414: 4182004C  beq 0x825b6460
	if ctx.cr[0].eq {
	pc = 0x825B6460; continue 'dispatch;
	}
	// 825B6418: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 825B641C: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 825B6420: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 825B6424: 388BBEE4  addi r4, r11, -0x411c
	ctx.r[4].s64 = ctx.r[11].s64 + -16668;
	// 825B6428: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 825B642C: 3B8ABEE0  addi r28, r10, -0x4120
	ctx.r[28].s64 = ctx.r[10].s64 + -16672;
	// 825B6430: 4883D871  bl 0x82df3ca0
	ctx.lr = 0x825B6434;
	sub_82DF3CA0(ctx, base);
	// 825B6434: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 825B6438: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825B643C: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 825B6440: 4883D8B9  bl 0x82df3cf8
	ctx.lr = 0x825B6444;
	sub_82DF3CF8(ctx, base);
	// 825B6444: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 825B6448: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 825B644C: 4883D465  bl 0x82df38b0
	ctx.lr = 0x825B6450;
	sub_82DF38B0(ctx, base);
	// 825B6450: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825B6454: 4883CFD5  bl 0x82df3428
	ctx.lr = 0x825B6458;
	sub_82DF3428(ctx, base);
	// 825B6458: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 825B645C: 4883CFCD  bl 0x82df3428
	ctx.lr = 0x825B6460;
	sub_82DF3428(ctx, base);
	// 825B6460: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 825B6464: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 825B6468: 48843321  bl 0x82df9788
	ctx.lr = 0x825B646C;
	sub_82DF9788(ctx, base);
	// 825B646C: 817F00B8  lwz r11, 0xb8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(184 as u32) ) } as u64;
	// 825B6470: 38C0FFFF  li r6, -1
	ctx.r[6].s64 = -1;
	// 825B6474: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825B6478: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 825B647C: 7D7E582E  lwzx r11, r30, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 825B6480: 386B0090  addi r3, r11, 0x90
	ctx.r[3].s64 = ctx.r[11].s64 + 144;
	// 825B6484: 4BD12CAD  bl 0x822c9130
	ctx.lr = 0x825B6488;
	sub_822C9130(ctx, base);
	// 825B6488: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825B648C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 825B6490: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 825B6494: 4BD1299D  bl 0x822c8e30
	ctx.lr = 0x825B6498;
	sub_822C8E30(ctx, base);
	// 825B6498: 817F0104  lwz r11, 0x104(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(260 as u32) ) } as u64;
	// 825B649C: 7F1D5840  cmplw cr6, r29, r11
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[11].u32, &mut ctx.xer);
	// 825B64A0: 419A0038  beq cr6, 0x825b64d8
	if ctx.cr[6].eq {
	pc = 0x825B64D8; continue 'dispatch;
	}
	// 825B64A4: 815F00B8  lwz r10, 0xb8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(184 as u32) ) } as u64;
	// 825B64A8: 392000FF  li r9, 0xff
	ctx.r[9].s64 = 255;
	// 825B64AC: 576B063F  clrlwi. r11, r27, 0x18
	ctx.r[11].u64 = ctx.r[27].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825B64B0: 7D5E502E  lwzx r10, r30, r10
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 825B64B4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825B64B8: 99210050  stb r9, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u8 ) };
	// 825B64BC: 40820008  bne 0x825b64c4
	if !ctx.cr[0].eq {
	pc = 0x825B64C4; continue 'dispatch;
	}
	// 825B64C0: 39600080  li r11, 0x80
	ctx.r[11].s64 = 128;
	// 825B64C4: 99610051  stb r11, 0x51(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(81 as u32), ctx.r[11].u8 ) };
	// 825B64C8: 99610052  stb r11, 0x52(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(82 as u32), ctx.r[11].u8 ) };
	// 825B64CC: 99610053  stb r11, 0x53(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(83 as u32), ctx.r[11].u8 ) };
	// 825B64D0: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 825B64D4: 916A00B8  stw r11, 0xb8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(184 as u32), ctx.r[11].u32 ) };
	// 825B64D8: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 825B64DC: 4883CF4D  bl 0x82df3428
	ctx.lr = 0x825B64E0;
	sub_82DF3428(ctx, base);
	// 825B64E0: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 825B64E4: 48BF1CCC  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B64E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825B64E8 size=244
    let mut pc: u32 = 0x825B64E8;
    'dispatch: loop {
        match pc {
            0x825B64E8 => {
    //   block [0x825B64E8..0x825B65DC)
	// 825B64E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B64EC: 48BF1C79  bl 0x831a8164
	ctx.lr = 0x825B64F0;
	sub_831A8130(ctx, base);
	// 825B64F0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B64F4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825B64F8: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 825B64FC: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 825B6500: 93A100AC  stw r29, 0xac(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(172 as u32), ctx.r[29].u32 ) };
	// 825B6504: 897F00E5  lbz r11, 0xe5(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(229 as u32) ) } as u64;
	// 825B6508: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 825B650C: 40820020  bne 0x825b652c
	if !ctx.cr[0].eq {
	pc = 0x825B652C; continue 'dispatch;
	}
	// 825B6510: 817F00C8  lwz r11, 0xc8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(200 as u32) ) } as u64;
	// 825B6514: 57AA1838  slwi r10, r29, 3
	ctx.r[10].u32 = ctx.r[29].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 825B6518: 7C6B502E  lwzx r3, r11, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 825B651C: 48001E1D  bl 0x825b8338
	ctx.lr = 0x825B6520;
	sub_825B8338(ctx, base);
	// 825B6520: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825B6524: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825B6528: 41820008  beq 0x825b6530
	if ctx.cr[0].eq {
	pc = 0x825B6530; continue 'dispatch;
	}
	// 825B652C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 825B6530: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825B6534: 7D7B5B78  mr r27, r11
	ctx.r[27].u64 = ctx.r[11].u64;
	// 825B6538: 4182000C  beq 0x825b6544
	if ctx.cr[0].eq {
	pc = 0x825B6544; continue 'dispatch;
	}
	// 825B653C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B6540: 4BFFFC29  bl 0x825b6168
	ctx.lr = 0x825B6544;
	sub_825B6168(ctx, base);
	// 825B6544: 817F00C8  lwz r11, 0xc8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(200 as u32) ) } as u64;
	// 825B6548: 57BE1838  slwi r30, r29, 3
	ctx.r[30].u32 = ctx.r[29].u32.wrapping_shl(3);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 825B654C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 825B6550: 7C6BF02E  lwzx r3, r11, r30
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 825B6554: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B6558: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 825B655C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825B6560: 4E800421  bctrl
	ctx.lr = 0x825B6564;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825B6564: 817F00C8  lwz r11, 0xc8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(200 as u32) ) } as u64;
	// 825B6568: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 825B656C: 7CABF214  add r5, r11, r30
	ctx.r[5].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 825B6570: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 825B6574: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B6578: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B657C: 816B005C  lwz r11, 0x5c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(92 as u32) ) } as u64;
	// 825B6580: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825B6584: 4E800421  bctrl
	ctx.lr = 0x825B6588;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825B6588: 38A100AC  addi r5, r1, 0xac
	ctx.r[5].s64 = ctx.r[1].s64 + 172;
	// 825B658C: 389F00E8  addi r4, r31, 0xe8
	ctx.r[4].s64 = ctx.r[31].s64 + 232;
	// 825B6590: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825B6594: 4BFFFC85  bl 0x825b6218
	ctx.lr = 0x825B6598;
	sub_825B6218(ctx, base);
	// 825B6598: 815F00EC  lwz r10, 0xec(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(236 as u32) ) } as u64;
	// 825B659C: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 825B65A0: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 825B65A4: 419A0010  beq cr6, 0x825b65b4
	if ctx.cr[6].eq {
	pc = 0x825B65B4; continue 'dispatch;
	}
	// 825B65A8: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 825B65AC: 386B0018  addi r3, r11, 0x18
	ctx.r[3].s64 = ctx.r[11].s64 + 24;
	// 825B65B0: 4BFFB319  bl 0x825b18c8
	ctx.lr = 0x825B65B4;
	sub_825B18C8(ctx, base);
	// 825B65B4: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 825B65B8: 419A001C  beq cr6, 0x825b65d4
	if ctx.cr[6].eq {
	pc = 0x825B65D4; continue 'dispatch;
	}
	// 825B65BC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B65C0: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 825B65C4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B65C8: 816B0058  lwz r11, 0x58(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(88 as u32) ) } as u64;
	// 825B65CC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825B65D0: 4E800421  bctrl
	ctx.lr = 0x825B65D4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825B65D4: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 825B65D8: 48BF1BDC  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B65E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825B65E0 size=160
    let mut pc: u32 = 0x825B65E0;
    'dispatch: loop {
        match pc {
            0x825B65E0 => {
    //   block [0x825B65E0..0x825B6680)
	// 825B65E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B65E4: 48BF1B81  bl 0x831a8164
	ctx.lr = 0x825B65E8;
	sub_831A8130(ctx, base);
	// 825B65E8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B65EC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 825B65F0: 83FE00C8  lwz r31, 0xc8(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(200 as u32) ) } as u64;
	// 825B65F4: 48000078  b 0x825b666c
	pc = 0x825B666C; continue 'dispatch;
	// 825B65F8: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B65FC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B6600: 816B0038  lwz r11, 0x38(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) } as u64;
	// 825B6604: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825B6608: 4E800421  bctrl
	ctx.lr = 0x825B660C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825B660C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825B6610: 41820058  beq 0x825b6668
	if ctx.cr[0].eq {
	pc = 0x825B6668; continue 'dispatch;
	}
	// 825B6614: 83BF0000  lwz r29, 0(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B6618: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 825B661C: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B6620: 839D00C8  lwz r28, 0xc8(r29)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(200 as u32) ) } as u64;
	// 825B6624: 816B0048  lwz r11, 0x48(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(72 as u32) ) } as u64;
	// 825B6628: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825B662C: 4E800421  bctrl
	ctx.lr = 0x825B6630;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825B6630: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B6634: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 825B6638: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 825B663C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825B6640: 816B003C  lwz r11, 0x3c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 825B6644: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825B6648: 4E800421  bctrl
	ctx.lr = 0x825B664C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825B664C: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 825B6650: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 825B6654: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825B6658: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 825B665C: 4BFFFD3D  bl 0x825b6398
	ctx.lr = 0x825B6660;
	sub_825B6398(ctx, base);
	// 825B6660: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825B6664: 4883CDC5  bl 0x82df3428
	ctx.lr = 0x825B6668;
	sub_82DF3428(ctx, base);
	// 825B6668: 3BFF0008  addi r31, r31, 8
	ctx.r[31].s64 = ctx.r[31].s64 + 8;
	// 825B666C: 817E00CC  lwz r11, 0xcc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(204 as u32) ) } as u64;
	// 825B6670: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 825B6674: 409AFF84  bne cr6, 0x825b65f8
	if !ctx.cr[6].eq {
	pc = 0x825B65F8; continue 'dispatch;
	}
	// 825B6678: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 825B667C: 48BF1B38  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B6680(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x825B6680 size=684
    let mut pc: u32 = 0x825B6680;
    'dispatch: loop {
        match pc {
            0x825B6680 => {
    //   block [0x825B6680..0x825B692C)
	// 825B6680: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B6684: 48BF1AE5  bl 0x831a8168
	ctx.lr = 0x825B6688;
	sub_831A8130(ctx, base);
	// 825B6688: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B668C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825B6690: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825B6694: 897E004C  lbz r11, 0x4c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(76 as u32) ) } as u64;
	// 825B6698: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 825B669C: 41820028  beq 0x825b66c4
	if ctx.cr[0].eq {
	pc = 0x825B66C4; continue 'dispatch;
	}
	// 825B66A0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B66A4: 816B0060  lwz r11, 0x60(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(96 as u32) ) } as u64;
	// 825B66A8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825B66AC: 4E800421  bctrl
	ctx.lr = 0x825B66B0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825B66B0: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825B66B4: 4182007C  beq 0x825b6730
	if ctx.cr[0].eq {
	pc = 0x825B6730; continue 'dispatch;
	}
	// 825B66B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825B66BC: 997E004C  stb r11, 0x4c(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(76 as u32), ctx.r[11].u8 ) };
	// 825B66C0: 48000264  b 0x825b6924
	pc = 0x825B6924; continue 'dispatch;
	// 825B66C4: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 825B66C8: 556A07FF  clrlwi. r10, r11, 0x1f
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 825B66CC: 41820050  beq 0x825b671c
	if ctx.cr[0].eq {
	pc = 0x825B671C; continue 'dispatch;
	}
	// 825B66D0: 897E004E  lbz r11, 0x4e(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(78 as u32) ) } as u64;
	// 825B66D4: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 825B66D8: 4082024C  bne 0x825b6924
	if !ctx.cr[0].eq {
	pc = 0x825B6924; continue 'dispatch;
	}
	// 825B66DC: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 825B66E0: 997E004E  stb r11, 0x4e(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(78 as u32), ctx.r[11].u8 ) };
	// 825B66E4: 817F00C8  lwz r11, 0xc8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(200 as u32) ) } as u64;
	// 825B66E8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825B66EC: 419A0238  beq cr6, 0x825b6924
	if ctx.cr[6].eq {
	pc = 0x825B6924; continue 'dispatch;
	}
	// 825B66F0: 815F00CC  lwz r10, 0xcc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(204 as u32) ) } as u64;
	// 825B66F4: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 825B66F8: 7D6B1E71  srawi. r11, r11, 3
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 3) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 3) as i64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825B66FC: 41820228  beq 0x825b6924
	if ctx.cr[0].eq {
	pc = 0x825B6924; continue 'dispatch;
	}
	// 825B6700: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 825B6704: 809F00D4  lwz r4, 0xd4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(212 as u32) ) } as u64;
	// 825B6708: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B670C: 4BFFFDDD  bl 0x825b64e8
	ctx.lr = 0x825B6710;
	sub_825B64E8(ctx, base);
	// 825B6710: 387E0004  addi r3, r30, 4
	ctx.r[3].s64 = ctx.r[30].s64 + 4;
	// 825B6714: 48590ADD  bl 0x82b471f0
	ctx.lr = 0x825B6718;
	sub_82B471F0(ctx, base);
	// 825B6718: 4800020C  b 0x825b6924
	pc = 0x825B6924; continue 'dispatch;
	// 825B671C: 556B07BD  rlwinm. r11, r11, 0, 0x1e, 0x1e
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825B6720: 41820034  beq 0x825b6754
	if ctx.cr[0].eq {
	pc = 0x825B6754; continue 'dispatch;
	}
	// 825B6724: 897F00E4  lbz r11, 0xe4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(228 as u32) ) } as u64;
	// 825B6728: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 825B672C: 408201F8  bne 0x825b6924
	if !ctx.cr[0].eq {
	pc = 0x825B6924; continue 'dispatch;
	}
	// 825B6730: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B6734: 4BFFFA35  bl 0x825b6168
	ctx.lr = 0x825B6738;
	sub_825B6168(ctx, base);
	// 825B6738: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B673C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 825B6740: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B6744: 816B0058  lwz r11, 0x58(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(88 as u32) ) } as u64;
	// 825B6748: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825B674C: 4E800421  bctrl
	ctx.lr = 0x825B6750;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825B6750: 480001D4  b 0x825b6924
	pc = 0x825B6924; continue 'dispatch;
	// 825B6754: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 825B6758: 807E0038  lwz r3, 0x38(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(56 as u32) ) } as u64;
	// 825B675C: 4BFFB55D  bl 0x825b1cb8
	ctx.lr = 0x825B6760;
	sub_825B1CB8(ctx, base);
	// 825B6760: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825B6764: 418200C0  beq 0x825b6824
	if ctx.cr[0].eq {
	pc = 0x825B6824; continue 'dispatch;
	}
	// 825B6768: 817F00C8  lwz r11, 0xc8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(200 as u32) ) } as u64;
	// 825B676C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825B6770: 419A01B4  beq cr6, 0x825b6924
	if ctx.cr[6].eq {
	pc = 0x825B6924; continue 'dispatch;
	}
	// 825B6774: 815F00CC  lwz r10, 0xcc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(204 as u32) ) } as u64;
	// 825B6778: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 825B677C: 7D6B1E71  srawi. r11, r11, 3
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 3) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 3) as i64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825B6780: 418201A4  beq 0x825b6924
	if ctx.cr[0].eq {
	pc = 0x825B6924; continue 'dispatch;
	}
	// 825B6784: 817F00D4  lwz r11, 0xd4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(212 as u32) ) } as u64;
	// 825B6788: 3BDF00D4  addi r30, r31, 0xd4
	ctx.r[30].s64 = ctx.r[31].s64 + 212;
	// 825B678C: 3B9F00F4  addi r28, r31, 0xf4
	ctx.r[28].s64 = ctx.r[31].s64 + 244;
	// 825B6790: 3BAB0001  addi r29, r11, 1
	ctx.r[29].s64 = ctx.r[11].s64 + 1;
	// 825B6794: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825B6798: 93BF00D4  stw r29, 0xd4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(212 as u32), ctx.r[29].u32 ) };
	// 825B679C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 825B67A0: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 825B67A4: 4BDBD96D  bl 0x82374110
	ctx.lr = 0x825B67A8;
	sub_82374110(ctx, base);
	// 825B67A8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B67AC: 815F00F8  lwz r10, 0xf8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(248 as u32) ) } as u64;
	// 825B67B0: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 825B67B4: 419A002C  beq cr6, 0x825b67e0
	if ctx.cr[6].eq {
	pc = 0x825B67E0; continue 'dispatch;
	}
	// 825B67B8: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 825B67BC: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 825B67C0: 93BE0000  stw r29, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 825B67C4: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 825B67C8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825B67CC: 4BDBD945  bl 0x82374110
	ctx.lr = 0x825B67D0;
	sub_82374110(ctx, base);
	// 825B67D0: 817F00F8  lwz r11, 0xf8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(248 as u32) ) } as u64;
	// 825B67D4: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B67D8: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 825B67DC: 409AFFDC  bne cr6, 0x825b67b8
	if !ctx.cr[6].eq {
	pc = 0x825B67B8; continue 'dispatch;
	}
	// 825B67E0: 817F00C8  lwz r11, 0xc8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(200 as u32) ) } as u64;
	// 825B67E4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825B67E8: 419A0010  beq cr6, 0x825b67f8
	if ctx.cr[6].eq {
	pc = 0x825B67F8; continue 'dispatch;
	}
	// 825B67EC: 815F00CC  lwz r10, 0xcc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(204 as u32) ) } as u64;
	// 825B67F0: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 825B67F4: 7D6B1E70  srawi r11, r11, 3
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 3) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 3) as i64;
	// 825B67F8: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B67FC: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 825B6800: 4198000C  blt cr6, 0x825b680c
	if ctx.cr[6].lt {
	pc = 0x825B680C; continue 'dispatch;
	}
	// 825B6804: 817F00D8  lwz r11, 0xd8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(216 as u32) ) } as u64;
	// 825B6808: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825B680C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B6810: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 825B6814: 796B0020  clrldi r11, r11, 0x20
	ctx.r[11].u64 = ctx.r[11].u64 & 0x00000000FFFFFFFFu64;
	// 825B6818: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 825B681C: C8010050  lfd f0, 0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 825B6820: 480000CC  b 0x825b68ec
	pc = 0x825B68EC; continue 'dispatch;
	// 825B6824: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 825B6828: 807E0038  lwz r3, 0x38(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(56 as u32) ) } as u64;
	// 825B682C: 4BFFB45D  bl 0x825b1c88
	ctx.lr = 0x825B6830;
	sub_825B1C88(ctx, base);
	// 825B6830: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825B6834: 418200E8  beq 0x825b691c
	if ctx.cr[0].eq {
	pc = 0x825B691C; continue 'dispatch;
	}
	// 825B6838: 817F00C8  lwz r11, 0xc8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(200 as u32) ) } as u64;
	// 825B683C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825B6840: 419A00E4  beq cr6, 0x825b6924
	if ctx.cr[6].eq {
	pc = 0x825B6924; continue 'dispatch;
	}
	// 825B6844: 815F00CC  lwz r10, 0xcc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(204 as u32) ) } as u64;
	// 825B6848: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 825B684C: 7D6B1E71  srawi. r11, r11, 3
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 3) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 3) as i64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825B6850: 418200D4  beq 0x825b6924
	if ctx.cr[0].eq {
	pc = 0x825B6924; continue 'dispatch;
	}
	// 825B6854: 815F00D8  lwz r10, 0xd8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(216 as u32) ) } as u64;
	// 825B6858: 3BDF00D4  addi r30, r31, 0xd4
	ctx.r[30].s64 = ctx.r[31].s64 + 212;
	// 825B685C: 817F00D4  lwz r11, 0xd4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(212 as u32) ) } as u64;
	// 825B6860: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 825B6864: 409A001C  bne cr6, 0x825b6880
	if !ctx.cr[6].eq {
	pc = 0x825B6880; continue 'dispatch;
	}
	// 825B6868: 817F00C8  lwz r11, 0xc8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(200 as u32) ) } as u64;
	// 825B686C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825B6870: 419A0010  beq cr6, 0x825b6880
	if ctx.cr[6].eq {
	pc = 0x825B6880; continue 'dispatch;
	}
	// 825B6874: 815F00CC  lwz r10, 0xcc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(204 as u32) ) } as u64;
	// 825B6878: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 825B687C: 7D6B1E70  srawi r11, r11, 3
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 3) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 3) as i64;
	// 825B6880: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 825B6884: 3BBF00F4  addi r29, r31, 0xf4
	ctx.r[29].s64 = ctx.r[31].s64 + 244;
	// 825B6888: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825B688C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 825B6890: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 825B6894: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825B6898: 4BDBD879  bl 0x82374110
	ctx.lr = 0x825B689C;
	sub_82374110(ctx, base);
	// 825B689C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B68A0: 815F00F8  lwz r10, 0xf8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(248 as u32) ) } as u64;
	// 825B68A4: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 825B68A8: 419A0030  beq cr6, 0x825b68d8
	if ctx.cr[6].eq {
	pc = 0x825B68D8; continue 'dispatch;
	}
	// 825B68AC: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B68B0: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 825B68B4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 825B68B8: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 825B68BC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825B68C0: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825B68C4: 4BDBD84D  bl 0x82374110
	ctx.lr = 0x825B68C8;
	sub_82374110(ctx, base);
	// 825B68C8: 817F00F8  lwz r11, 0xf8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(248 as u32) ) } as u64;
	// 825B68CC: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B68D0: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 825B68D4: 409AFFD8  bne cr6, 0x825b68ac
	if !ctx.cr[6].eq {
	pc = 0x825B68AC; continue 'dispatch;
	}
	// 825B68D8: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B68DC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 825B68E0: 796B0020  clrldi r11, r11, 0x20
	ctx.r[11].u64 = ctx.r[11].u64 & 0x00000000FFFFFFFFu64;
	// 825B68E4: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 825B68E8: C8010050  lfd f0, 0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 825B68EC: FD80069C  fcfid f12, f0
	ctx.f[12].f64 = (ctx.f[0].s64 as f64);
	// 825B68F0: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 825B68F4: 3D208208  lis r9, -0x7df8
	ctx.r[9].s64 = -2113404928;
	// 825B68F8: 811F00AC  lwz r8, 0xac(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(172 as u32) ) } as u64;
	// 825B68FC: C1AABED8  lfs f13, -0x4128(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-16680 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825B6900: C00989AC  lfs f0, -0x7654(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-30292 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825B6904: D0080030  stfs f0, 0x30(r8)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(48 as u32), tmp.u32 ) };
	// 825B6908: FD806018  frsp f12, f12
	ctx.f[12].f64 = (ctx.f[12].f64 as f32) as f64;
	// 825B690C: EDAC0372  fmuls f13, f12, f13
	ctx.f[13].f64 = (((ctx.f[12].f64 * ctx.f[13].f64) as f32) as f64);
	// 825B6910: EC0D002A  fadds f0, f13, f0
	ctx.f[0].f64 = ((ctx.f[13].f64 + ctx.f[0].f64) as f32) as f64;
	// 825B6914: D0080034  stfs f0, 0x34(r8)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(52 as u32), tmp.u32 ) };
	// 825B6918: 4800000C  b 0x825b6924
	pc = 0x825B6924; continue 'dispatch;
	// 825B691C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825B6920: 997E004E  stb r11, 0x4e(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(78 as u32), ctx.r[11].u8 ) };
	// 825B6924: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825B6928: 48BF1890  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B6930(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x825B6930 size=192
    let mut pc: u32 = 0x825B6930;
    'dispatch: loop {
        match pc {
            0x825B6930 => {
    //   block [0x825B6930..0x825B69F0)
	// 825B6930: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B6934: 48BF1839  bl 0x831a816c
	ctx.lr = 0x825B6938;
	sub_831A8130(ctx, base);
	// 825B6938: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B693C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 825B6940: 83FE00B8  lwz r31, 0xb8(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(184 as u32) ) } as u64;
	// 825B6944: 817E00BC  lwz r11, 0xbc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(188 as u32) ) } as u64;
	// 825B6948: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 825B694C: 419A0050  beq cr6, 0x825b699c
	if ctx.cr[6].eq {
	pc = 0x825B699C; continue 'dispatch;
	}
	// 825B6950: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 825B6954: 3BAB9D20  addi r29, r11, -0x62e0
	ctx.r[29].s64 = ctx.r[11].s64 + -25312;
	// 825B6958: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 825B695C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 825B6960: 4BD12991  bl 0x822c92f0
	ctx.lr = 0x825B6964;
	sub_822C92F0(ctx, base);
	// 825B6964: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B6968: 38C0FFFF  li r6, -1
	ctx.r[6].s64 = -1;
	// 825B696C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825B6970: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 825B6974: 386B0090  addi r3, r11, 0x90
	ctx.r[3].s64 = ctx.r[11].s64 + 144;
	// 825B6978: 4BD127B9  bl 0x822c9130
	ctx.lr = 0x825B697C;
	sub_822C9130(ctx, base);
	// 825B697C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825B6980: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 825B6984: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 825B6988: 4BD124A9  bl 0x822c8e30
	ctx.lr = 0x825B698C;
	sub_822C8E30(ctx, base);
	// 825B698C: 817E00BC  lwz r11, 0xbc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(188 as u32) ) } as u64;
	// 825B6990: 3BFF0008  addi r31, r31, 8
	ctx.r[31].s64 = ctx.r[31].s64 + 8;
	// 825B6994: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 825B6998: 409AFFC0  bne cr6, 0x825b6958
	if !ctx.cr[6].eq {
	pc = 0x825B6958; continue 'dispatch;
	}
	// 825B699C: 387E00C4  addi r3, r30, 0xc4
	ctx.r[3].s64 = ctx.r[30].s64 + 196;
	// 825B69A0: 4BD10209  bl 0x822c6ba8
	ctx.lr = 0x825B69A4;
	sub_822C6BA8(ctx, base);
	// 825B69A4: 817E00D8  lwz r11, 0xd8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(216 as u32) ) } as u64;
	// 825B69A8: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 825B69AC: 392B0001  addi r9, r11, 1
	ctx.r[9].s64 = ctx.r[11].s64 + 1;
	// 825B69B0: 3D008208  lis r8, -0x7df8
	ctx.r[8].s64 = -2113404928;
	// 825B69B4: 79290020  clrldi r9, r9, 0x20
	ctx.r[9].u64 = ctx.r[9].u64 & 0x00000000FFFFFFFFu64;
	// 825B69B8: 917E00D4  stw r11, 0xd4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(212 as u32), ctx.r[11].u32 ) };
	// 825B69BC: F9210050  std r9, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u64 ) };
	// 825B69C0: C8010050  lfd f0, 0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 825B69C4: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 825B69C8: C1AABED8  lfs f13, -0x4128(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-16680 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825B69CC: FD800018  frsp f12, f0
	ctx.f[12].f64 = (ctx.f[0].f64 as f32) as f64;
	// 825B69D0: 817E00AC  lwz r11, 0xac(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(172 as u32) ) } as u64;
	// 825B69D4: C00889AC  lfs f0, -0x7654(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(-30292 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825B69D8: D00B0030  stfs f0, 0x30(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(48 as u32), tmp.u32 ) };
	// 825B69DC: EDAC0372  fmuls f13, f12, f13
	ctx.f[13].f64 = (((ctx.f[12].f64 * ctx.f[13].f64) as f32) as f64);
	// 825B69E0: EC0D002A  fadds f0, f13, f0
	ctx.f[0].f64 = ((ctx.f[13].f64 + ctx.f[0].f64) as f32) as f64;
	// 825B69E4: D00B0034  stfs f0, 0x34(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(52 as u32), tmp.u32 ) };
	// 825B69E8: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 825B69EC: 48BF17D0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B69F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x825B69F0 size=636
    let mut pc: u32 = 0x825B69F0;
    'dispatch: loop {
        match pc {
            0x825B69F0 => {
    //   block [0x825B69F0..0x825B6C6C)
	// 825B69F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B69F4: 48BF176D  bl 0x831a8160
	ctx.lr = 0x825B69F8;
	sub_831A8130(ctx, base);
	// 825B69F8: 9421FEE0  stwu r1, -0x120(r1)
	ea = ctx.r[1].u32.wrapping_add(-288 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B69FC: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 825B6A00: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 825B6A04: 387C00C4  addi r3, r28, 0xc4
	ctx.r[3].s64 = ctx.r[28].s64 + 196;
	// 825B6A08: 817C00C8  lwz r11, 0xc8(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(200 as u32) ) } as u64;
	// 825B6A0C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825B6A10: 409A000C  bne cr6, 0x825b6a1c
	if !ctx.cr[6].eq {
	pc = 0x825B6A1C; continue 'dispatch;
	}
	// 825B6A14: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 825B6A18: 48000010  b 0x825b6a28
	pc = 0x825B6A28; continue 'dispatch;
	// 825B6A1C: 81430008  lwz r10, 8(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 825B6A20: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 825B6A24: 7D7A1E70  srawi r26, r11, 3
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 3) - 1)) != 0);
	ctx.r[26].s64 = (ctx.r[11].s32 >> 3) as i64;
	// 825B6A28: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B6A2C: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 825B6A30: 934B00C8  stw r26, 0xc8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(200 as u32), ctx.r[26].u32 ) };
	// 825B6A34: 485FC6D5  bl 0x82bb3108
	ctx.lr = 0x825B6A38;
	sub_82BB3108(ctx, base);
	// 825B6A38: 817C00B8  lwz r11, 0xb8(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(184 as u32) ) } as u64;
	// 825B6A3C: 3BDC00B4  addi r30, r28, 0xb4
	ctx.r[30].s64 = ctx.r[28].s64 + 180;
	// 825B6A40: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825B6A44: 419A006C  beq cr6, 0x825b6ab0
	if ctx.cr[6].eq {
	pc = 0x825B6AB0; continue 'dispatch;
	}
	// 825B6A48: 815E0008  lwz r10, 8(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 825B6A4C: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 825B6A50: 7D6B1E70  srawi r11, r11, 3
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 3) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 3) as i64;
	// 825B6A54: 7F0BD040  cmplw cr6, r11, r26
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[26].u32, &mut ctx.xer);
	// 825B6A58: 40990058  ble cr6, 0x825b6ab0
	if !ctx.cr[6].gt {
	pc = 0x825B6AB0; continue 'dispatch;
	}
	// 825B6A5C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825B6A60: 809B0000  lwz r4, 0(r27)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B6A64: 48016605  bl 0x825cd068
	ctx.lr = 0x825B6A68;
	sub_825CD068(ctx, base);
	// 825B6A68: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 825B6A6C: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 825B6A70: 48842D19  bl 0x82df9788
	ctx.lr = 0x825B6A74;
	sub_82DF9788(ctx, base);
	// 825B6A74: 815C00B8  lwz r10, 0xb8(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(184 as u32) ) } as u64;
	// 825B6A78: 574B1838  slwi r11, r26, 3
	ctx.r[11].u32 = ctx.r[26].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 825B6A7C: 38C0FFFF  li r6, -1
	ctx.r[6].s64 = -1;
	// 825B6A80: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825B6A84: 38810080  addi r4, r1, 0x80
	ctx.r[4].s64 = ctx.r[1].s64 + 128;
	// 825B6A88: 7D6A582E  lwzx r11, r10, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 825B6A8C: 386B0090  addi r3, r11, 0x90
	ctx.r[3].s64 = ctx.r[11].s64 + 144;
	// 825B6A90: 4BD126A1  bl 0x822c9130
	ctx.lr = 0x825B6A94;
	sub_822C9130(ctx, base);
	// 825B6A94: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825B6A98: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 825B6A9C: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 825B6AA0: 4BD12391  bl 0x822c8e30
	ctx.lr = 0x825B6AA4;
	sub_822C8E30(ctx, base);
	// 825B6AA4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825B6AA8: 4883C981  bl 0x82df3428
	ctx.lr = 0x825B6AAC;
	sub_82DF3428(ctx, base);
	// 825B6AAC: 480001A0  b 0x825b6c4c
	pc = 0x825B6C4C; continue 'dispatch;
	// 825B6AB0: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 825B6AB4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825B6AB8: 409A000C  bne cr6, 0x825b6ac4
	if !ctx.cr[6].eq {
	pc = 0x825B6AC4; continue 'dispatch;
	}
	// 825B6ABC: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 825B6AC0: 48000010  b 0x825b6ad0
	pc = 0x825B6AD0; continue 'dispatch;
	// 825B6AC4: 815E0008  lwz r10, 8(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 825B6AC8: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 825B6ACC: 7D7F1E70  srawi r31, r11, 3
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 3) - 1)) != 0);
	ctx.r[31].s64 = (ctx.r[11].s32 >> 3) as i64;
	// 825B6AD0: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 825B6AD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825B6AD8: 388BBEF8  addi r4, r11, -0x4108
	ctx.r[4].s64 = ctx.r[11].s64 + -16648;
	// 825B6ADC: 38A000C3  li r5, 0xc3
	ctx.r[5].s64 = 195;
	// 825B6AE0: 386000C0  li r3, 0xc0
	ctx.r[3].s64 = 192;
	// 825B6AE4: 4BD098F5  bl 0x822c03d8
	ctx.lr = 0x825B6AE8;
	sub_822C03D8(ctx, base);
	// 825B6AE8: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 825B6AEC: 41820098  beq 0x825b6b84
	if ctx.cr[0].eq {
	pc = 0x825B6B84; continue 'dispatch;
	}
	// 825B6AF0: 397F0001  addi r11, r31, 1
	ctx.r[11].s64 = ctx.r[31].s64 + 1;
	// 825B6AF4: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 825B6AF8: 796B0020  clrldi r11, r11, 0x20
	ctx.r[11].u64 = ctx.r[11].u64 & 0x00000000FFFFFFFFu64;
	// 825B6AFC: 3D208208  lis r9, -0x7df8
	ctx.r[9].s64 = -2113404928;
	// 825B6B00: F9610058  std r11, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u64 ) };
	// 825B6B04: C8010058  lfd f0, 0x58(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 825B6B08: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 825B6B0C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 825B6B10: FD400018  frsp f10, f0
	ctx.f[10].f64 = (ctx.f[0].f64 as f32) as f64;
	// 825B6B14: 3D008200  lis r8, -0x7e00
	ctx.r[8].s64 = -2113929216;
	// 825B6B18: 3CE08202  lis r7, -0x7dfe
	ctx.r[7].s64 = -2113798144;
	// 825B6B1C: C18ABED8  lfs f12, -0x4128(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-16680 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 825B6B20: C00B959C  lfs f0, -0x6a64(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27236 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825B6B24: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 825B6B28: C16989AC  lfs f11, -0x7654(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-30292 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 825B6B2C: 386100C0  addi r3, r1, 0xc0
	ctx.r[3].s64 = ctx.r[1].s64 + 192;
	// 825B6B30: D0010060  stfs f0, 0x60(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), tmp.u32 ) };
	// 825B6B34: 388ABEE8  addi r4, r10, -0x4118
	ctx.r[4].s64 = ctx.r[10].s64 + -16664;
	// 825B6B38: C00808A4  lfs f0, 0x8a4(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825B6B3C: C1A76218  lfs f13, 0x6218(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(25112 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825B6B40: D0010068  stfs f0, 0x68(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), tmp.u32 ) };
	// 825B6B44: D001006C  stfs f0, 0x6c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), tmp.u32 ) };
	// 825B6B48: D1A10070  stfs f13, 0x70(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), tmp.u32 ) };
	// 825B6B4C: ED8A5B3A  fmadds f12, f10, f12, f11
	ctx.f[12].f64 = (((ctx.f[10].f64 * ctx.f[12].f64 + ctx.f[11].f64) as f32) as f64);
	// 825B6B50: D1A10074  stfs f13, 0x74(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), tmp.u32 ) };
	// 825B6B54: D0010078  stfs f0, 0x78(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), tmp.u32 ) };
	// 825B6B58: D001007C  stfs f0, 0x7c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), tmp.u32 ) };
	// 825B6B5C: D1810064  stfs f12, 0x64(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), tmp.u32 ) };
	// 825B6B60: 4BD12791  bl 0x822c92f0
	ctx.lr = 0x825B6B64;
	sub_822C92F0(ctx, base);
	// 825B6B64: 7C671B78  mr r7, r3
	ctx.r[7].u64 = ctx.r[3].u64;
	// 825B6B68: 38C10060  addi r6, r1, 0x60
	ctx.r[6].s64 = ctx.r[1].s64 + 96;
	// 825B6B6C: 38A10070  addi r5, r1, 0x70
	ctx.r[5].s64 = ctx.r[1].s64 + 112;
	// 825B6B70: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 825B6B74: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 825B6B78: 48B115F1  bl 0x830c8168
	ctx.lr = 0x825B6B7C;
	sub_830C8168(ctx, base);
	// 825B6B7C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825B6B80: 48000008  b 0x825b6b88
	pc = 0x825B6B88; continue 'dispatch;
	// 825B6B84: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 825B6B88: 93E10058  stw r31, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[31].u32 ) };
	// 825B6B8C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825B6B90: 3861005C  addi r3, r1, 0x5c
	ctx.r[3].s64 = ctx.r[1].s64 + 92;
	// 825B6B94: 4BD12AA5  bl 0x822c9638
	ctx.lr = 0x825B6B98;
	sub_822C9638(ctx, base);
	// 825B6B98: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 825B6B9C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825B6BA0: 3861005C  addi r3, r1, 0x5c
	ctx.r[3].s64 = ctx.r[1].s64 + 92;
	// 825B6BA4: 4BD0945D  bl 0x822c0000
	ctx.lr = 0x825B6BA8;
	sub_822C0000(ctx, base);
	// 825B6BA8: 83E1005C  lwz r31, 0x5c(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 825B6BAC: 83A10058  lwz r29, 0x58(r1)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 825B6BB0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 825B6BB4: 93E10054  stw r31, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[31].u32 ) };
	// 825B6BB8: 93A10050  stw r29, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[29].u32 ) };
	// 825B6BBC: 419A0024  beq cr6, 0x825b6be0
	if ctx.cr[6].eq {
	pc = 0x825B6BE0; continue 'dispatch;
	}
	// 825B6BC0: 397F0004  addi r11, r31, 4
	ctx.r[11].s64 = ctx.r[31].s64 + 4;
	// 825B6BC4: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 825B6BC8: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 825B6BCC: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 825B6BD0: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 825B6BD4: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 825B6BD8: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 825B6BDC: 4082FFE8  bne 0x825b6bc4
	if !ctx.cr[0].eq {
	pc = 0x825B6BC4; continue 'dispatch;
	}
	// 825B6BE0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 825B6BE4: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 825B6BE8: 48B0CD31  bl 0x830c3918
	ctx.lr = 0x825B6BEC;
	sub_830C3918(ctx, base);
	// 825B6BEC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825B6BF0: 809B0000  lwz r4, 0(r27)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B6BF4: 48016475  bl 0x825cd068
	ctx.lr = 0x825B6BF8;
	sub_825CD068(ctx, base);
	// 825B6BF8: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 825B6BFC: 386100A0  addi r3, r1, 0xa0
	ctx.r[3].s64 = ctx.r[1].s64 + 160;
	// 825B6C00: 48842B89  bl 0x82df9788
	ctx.lr = 0x825B6C04;
	sub_82DF9788(ctx, base);
	// 825B6C04: 38C0FFFF  li r6, -1
	ctx.r[6].s64 = -1;
	// 825B6C08: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825B6C0C: 388100A0  addi r4, r1, 0xa0
	ctx.r[4].s64 = ctx.r[1].s64 + 160;
	// 825B6C10: 387D0090  addi r3, r29, 0x90
	ctx.r[3].s64 = ctx.r[29].s64 + 144;
	// 825B6C14: 4BD1251D  bl 0x822c9130
	ctx.lr = 0x825B6C18;
	sub_822C9130(ctx, base);
	// 825B6C18: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825B6C1C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 825B6C20: 386100A0  addi r3, r1, 0xa0
	ctx.r[3].s64 = ctx.r[1].s64 + 160;
	// 825B6C24: 4BD1220D  bl 0x822c8e30
	ctx.lr = 0x825B6C28;
	sub_822C8E30(ctx, base);
	// 825B6C28: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825B6C2C: 4883C7FD  bl 0x82df3428
	ctx.lr = 0x825B6C30;
	sub_82DF3428(ctx, base);
	// 825B6C30: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 825B6C34: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825B6C38: 485FC4D1  bl 0x82bb3108
	ctx.lr = 0x825B6C3C;
	sub_82BB3108(ctx, base);
	// 825B6C3C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 825B6C40: 419A000C  beq cr6, 0x825b6c4c
	if ctx.cr[6].eq {
	pc = 0x825B6C4C; continue 'dispatch;
	}
	// 825B6C44: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B6C48: 4BD09C49  bl 0x822c0890
	ctx.lr = 0x825B6C4C;
	sub_822C0890(ctx, base);
	// 825B6C4C: 807B0000  lwz r3, 0(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B6C50: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B6C54: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 825B6C58: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825B6C5C: 4E800421  bctrl
	ctx.lr = 0x825B6C60;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825B6C60: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 825B6C64: 38210120  addi r1, r1, 0x120
	ctx.r[1].s64 = ctx.r[1].s64 + 288;
	// 825B6C68: 48BF1548  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B6C70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825B6C70 size=76
    let mut pc: u32 = 0x825B6C70;
    'dispatch: loop {
        match pc {
            0x825B6C70 => {
    //   block [0x825B6C70..0x825B6CBC)
	// 825B6C70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B6C74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825B6C78: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825B6C7C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825B6C80: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B6C84: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825B6C88: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 825B6C8C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825B6C90: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B6C94: 4800169D  bl 0x825b8330
	ctx.lr = 0x825B6C98;
	sub_825B8330(ctx, base);
	// 825B6C98: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 825B6C9C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B6CA0: 4BFFFD51  bl 0x825b69f0
	ctx.lr = 0x825B6CA4;
	sub_825B69F0(ctx, base);
	// 825B6CA4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825B6CA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825B6CAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825B6CB0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825B6CB4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825B6CB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B6CC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825B6CC0 size=80
    let mut pc: u32 = 0x825B6CC0;
    'dispatch: loop {
        match pc {
            0x825B6CC0 => {
    //   block [0x825B6CC0..0x825B6D10)
	// 825B6CC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B6CC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825B6CC8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825B6CCC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825B6CD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B6CD4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825B6CD8: 4BFFFD19  bl 0x825b69f0
	ctx.lr = 0x825B6CDC;
	sub_825B69F0(ctx, base);
	// 825B6CDC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 825B6CE0: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 825B6CE4: 93C10050  stw r30, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 825B6CE8: 389F00F4  addi r4, r31, 0xf4
	ctx.r[4].s64 = ctx.r[31].s64 + 244;
	// 825B6CEC: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 825B6CF0: 4BF99C69  bl 0x82550958
	ctx.lr = 0x825B6CF4;
	sub_82550958(ctx, base);
	// 825B6CF4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825B6CF8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825B6CFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825B6D00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825B6D04: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825B6D08: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825B6D0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B6D10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825B6D10 size=172
    let mut pc: u32 = 0x825B6D10;
    'dispatch: loop {
        match pc {
            0x825B6D10 => {
    //   block [0x825B6D10..0x825B6DBC)
	// 825B6D10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B6D14: 48BF1455  bl 0x831a8168
	ctx.lr = 0x825B6D18;
	sub_831A8130(ctx, base);
	// 825B6D18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B6D1C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825B6D20: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 825B6D24: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 825B6D28: 4BFFFCC9  bl 0x825b69f0
	ctx.lr = 0x825B6D2C;
	sub_825B69F0(ctx, base);
	// 825B6D2C: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B6D30: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 825B6D34: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 825B6D38: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 825B6D3C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B6D40: 816A0044  lwz r11, 0x44(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(68 as u32) ) } as u64;
	// 825B6D44: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825B6D48: 4E800421  bctrl
	ctx.lr = 0x825B6D4C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825B6D4C: 809D0000  lwz r4, 0(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B6D50: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825B6D54: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B6D58: 816B003C  lwz r11, 0x3c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 825B6D5C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825B6D60: 4E800421  bctrl
	ctx.lr = 0x825B6D64;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825B6D64: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 825B6D68: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 825B6D6C: 388B9BC9  addi r4, r11, -0x6437
	ctx.r[4].s64 = ctx.r[11].s64 + -25655;
	// 825B6D70: 4883CC99  bl 0x82df3a08
	ctx.lr = 0x825B6D74;
	sub_82DF3A08(ctx, base);
	// 825B6D74: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 825B6D78: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825B6D7C: 4883C525  bl 0x82df32a0
	ctx.lr = 0x825B6D80;
	sub_82DF32A0(ctx, base);
	// 825B6D80: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 825B6D84: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 825B6D88: 4883C6A1  bl 0x82df3428
	ctx.lr = 0x825B6D8C;
	sub_82DF3428(ctx, base);
	// 825B6D8C: 57AB063F  clrlwi. r11, r29, 0x18
	ctx.r[11].u64 = ctx.r[29].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825B6D90: 41820018  beq 0x825b6da8
	if ctx.cr[0].eq {
	pc = 0x825B6DA8; continue 'dispatch;
	}
	// 825B6D94: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 825B6D98: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 825B6D9C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 825B6DA0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B6DA4: 4BFFF5F5  bl 0x825b6398
	ctx.lr = 0x825B6DA8;
	sub_825B6398(ctx, base);
	// 825B6DA8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825B6DAC: 4883C67D  bl 0x82df3428
	ctx.lr = 0x825B6DB0;
	sub_82DF3428(ctx, base);
	// 825B6DB0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825B6DB4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825B6DB8: 48BF1400  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B6DC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x825B6DC0 size=808
    let mut pc: u32 = 0x825B6DC0;
    'dispatch: loop {
        match pc {
            0x825B6DC0 => {
    //   block [0x825B6DC0..0x825B70E8)
	// 825B6DC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B6DC4: 48BF1395  bl 0x831a8158
	ctx.lr = 0x825B6DC8;
	sub_831A8130(ctx, base);
	// 825B6DC8: DBA1FFA0  stfd f29, -0x60(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-96 as u32), ctx.f[29].u64 ) };
	// 825B6DCC: DBC1FFA8  stfd f30, -0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-88 as u32), ctx.f[30].u64 ) };
	// 825B6DD0: DBE1FFB0  stfd f31, -0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-80 as u32), ctx.f[31].u64 ) };
	// 825B6DD4: 9421FEA0  stwu r1, -0x160(r1)
	ea = ctx.r[1].u32.wrapping_add(-352 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B6DD8: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 825B6DDC: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 825B6DE0: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 825B6DE4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825B6DE8: 48B0BDA1  bl 0x830c2b88
	ctx.lr = 0x825B6DEC;
	sub_830C2B88(ctx, base);
	// 825B6DEC: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 825B6DF0: 387F00A0  addi r3, r31, 0xa0
	ctx.r[3].s64 = ctx.r[31].s64 + 160;
	// 825B6DF4: 396BBF4C  addi r11, r11, -0x40b4
	ctx.r[11].s64 = ctx.r[11].s64 + -16564;
	// 825B6DF8: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 825B6DFC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825B6E00: 4883CE01  bl 0x82df3c00
	ctx.lr = 0x825B6E04;
	sub_82DF3C00(ctx, base);
	// 825B6E04: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 825B6E08: 387F00E8  addi r3, r31, 0xe8
	ctx.r[3].s64 = ctx.r[31].s64 + 232;
	// 825B6E0C: 93DF00A4  stw r30, 0xa4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(164 as u32), ctx.r[30].u32 ) };
	// 825B6E10: 3BBF00A4  addi r29, r31, 0xa4
	ctx.r[29].s64 = ctx.r[31].s64 + 164;
	// 825B6E14: 93DF00A8  stw r30, 0xa8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(168 as u32), ctx.r[30].u32 ) };
	// 825B6E18: 3B9F00AC  addi r28, r31, 0xac
	ctx.r[28].s64 = ctx.r[31].s64 + 172;
	// 825B6E1C: 93DF00AC  stw r30, 0xac(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(172 as u32), ctx.r[30].u32 ) };
	// 825B6E20: 93DF00B0  stw r30, 0xb0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(176 as u32), ctx.r[30].u32 ) };
	// 825B6E24: 93DF00B8  stw r30, 0xb8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(184 as u32), ctx.r[30].u32 ) };
	// 825B6E28: 93DF00BC  stw r30, 0xbc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(188 as u32), ctx.r[30].u32 ) };
	// 825B6E2C: 93DF00C0  stw r30, 0xc0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(192 as u32), ctx.r[30].u32 ) };
	// 825B6E30: 93DF00C8  stw r30, 0xc8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(200 as u32), ctx.r[30].u32 ) };
	// 825B6E34: 93DF00CC  stw r30, 0xcc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(204 as u32), ctx.r[30].u32 ) };
	// 825B6E38: 93DF00D0  stw r30, 0xd0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(208 as u32), ctx.r[30].u32 ) };
	// 825B6E3C: 93DF00D4  stw r30, 0xd4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(212 as u32), ctx.r[30].u32 ) };
	// 825B6E40: 93DF00D8  stw r30, 0xd8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(216 as u32), ctx.r[30].u32 ) };
	// 825B6E44: 9BDF00DC  stb r30, 0xdc(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(220 as u32), ctx.r[30].u8 ) };
	// 825B6E48: 9BDF00E4  stb r30, 0xe4(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(228 as u32), ctx.r[30].u8 ) };
	// 825B6E4C: 9BDF00E5  stb r30, 0xe5(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(229 as u32), ctx.r[30].u8 ) };
	// 825B6E50: 485F8B01  bl 0x82baf950
	ctx.lr = 0x825B6E54;
	sub_82BAF950(ctx, base);
	// 825B6E54: 387F00F4  addi r3, r31, 0xf4
	ctx.r[3].s64 = ctx.r[31].s64 + 244;
	// 825B6E58: 4BE7E1B9  bl 0x82435010
	ctx.lr = 0x825B6E5C;
	sub_82435010(ctx, base);
	// 825B6E5C: 396000FF  li r11, 0xff
	ctx.r[11].s64 = 255;
	// 825B6E60: 394000C8  li r10, 0xc8
	ctx.r[10].s64 = 200;
	// 825B6E64: 93DF0104  stw r30, 0x104(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(260 as u32), ctx.r[30].u32 ) };
	// 825B6E68: 9BDF0090  stb r30, 0x90(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(144 as u32), ctx.r[30].u8 ) };
	// 825B6E6C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 825B6E70: 99410050  stb r10, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u8 ) };
	// 825B6E74: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B6E78: 99610051  stb r11, 0x51(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(81 as u32), ctx.r[11].u8 ) };
	// 825B6E7C: 99610052  stb r11, 0x52(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(82 as u32), ctx.r[11].u8 ) };
	// 825B6E80: 99610053  stb r11, 0x53(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(83 as u32), ctx.r[11].u8 ) };
	// 825B6E84: 48B0C21D  bl 0x830c30a0
	ctx.lr = 0x825B6E88;
	sub_830C30A0(ctx, base);
	// 825B6E88: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 825B6E8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825B6E90: 3B2BBEF8  addi r25, r11, -0x4108
	ctx.r[25].s64 = ctx.r[11].s64 + -16648;
	// 825B6E94: 38A00037  li r5, 0x37
	ctx.r[5].s64 = 55;
	// 825B6E98: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 825B6E9C: 386000C0  li r3, 0xc0
	ctx.r[3].s64 = 192;
	// 825B6EA0: 4BD09539  bl 0x822c03d8
	ctx.lr = 0x825B6EA4;
	sub_822C03D8(ctx, base);
	// 825B6EA4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825B6EA8: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 825B6EAC: 3D008208  lis r8, -0x7df8
	ctx.r[8].s64 = -2113404928;
	// 825B6EB0: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 825B6EB4: 7C7B1B79  or. r27, r3, r3
	ctx.r[27].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 825B6EB8: C3AA6218  lfs f29, 0x6218(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(25112 as u32) ) };
	ctx.f[29].f64 = (tmp.f32 as f64);
	// 825B6EBC: 3B0BBEE8  addi r24, r11, -0x4118
	ctx.r[24].s64 = ctx.r[11].s64 + -16664;
	// 825B6EC0: C3E908A4  lfs f31, 0x8a4(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(2212 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 825B6EC4: C3C889AC  lfs f30, -0x7654(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(-30292 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 825B6EC8: 41820050  beq 0x825b6f18
	if ctx.cr[0].eq {
	pc = 0x825B6F18; continue 'dispatch;
	}
	// 825B6ECC: D3C10080  stfs f30, 0x80(r1)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), tmp.u32 ) };
	// 825B6ED0: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 825B6ED4: D3C10084  stfs f30, 0x84(r1)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), tmp.u32 ) };
	// 825B6ED8: 386100E0  addi r3, r1, 0xe0
	ctx.r[3].s64 = ctx.r[1].s64 + 224;
	// 825B6EDC: D3E10088  stfs f31, 0x88(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(136 as u32), tmp.u32 ) };
	// 825B6EE0: D3E1008C  stfs f31, 0x8c(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(140 as u32), tmp.u32 ) };
	// 825B6EE4: D3A10090  stfs f29, 0x90(r1)
	tmp.f32 = (ctx.f[29].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(144 as u32), tmp.u32 ) };
	// 825B6EE8: D3A10094  stfs f29, 0x94(r1)
	tmp.f32 = (ctx.f[29].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(148 as u32), tmp.u32 ) };
	// 825B6EEC: D3E10098  stfs f31, 0x98(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(152 as u32), tmp.u32 ) };
	// 825B6EF0: D3E1009C  stfs f31, 0x9c(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(156 as u32), tmp.u32 ) };
	// 825B6EF4: 4BD123FD  bl 0x822c92f0
	ctx.lr = 0x825B6EF8;
	sub_822C92F0(ctx, base);
	// 825B6EF8: 7C671B78  mr r7, r3
	ctx.r[7].u64 = ctx.r[3].u64;
	// 825B6EFC: 38C10080  addi r6, r1, 0x80
	ctx.r[6].s64 = ctx.r[1].s64 + 128;
	// 825B6F00: 38A10090  addi r5, r1, 0x90
	ctx.r[5].s64 = ctx.r[1].s64 + 144;
	// 825B6F04: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825B6F08: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 825B6F0C: 48B1125D  bl 0x830c8168
	ctx.lr = 0x825B6F10;
	sub_830C8168(ctx, base);
	// 825B6F10: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 825B6F14: 48000008  b 0x825b6f1c
	pc = 0x825B6F1C; continue 'dispatch;
	// 825B6F18: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 825B6F1C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 825B6F20: 4BD17AC1  bl 0x822ce9e0
	ctx.lr = 0x825B6F24;
	sub_822CE9E0(ctx, base);
	// 825B6F24: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 825B6F28: 815D0000  lwz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B6F2C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825B6F30: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 825B6F34: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 825B6F38: 419A0024  beq cr6, 0x825b6f5c
	if ctx.cr[6].eq {
	pc = 0x825B6F5C; continue 'dispatch;
	}
	// 825B6F3C: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 825B6F40: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 825B6F44: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 825B6F48: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 825B6F4C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 825B6F50: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 825B6F54: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 825B6F58: 4082FFE8  bne 0x825b6f40
	if !ctx.cr[0].eq {
	pc = 0x825B6F40; continue 'dispatch;
	}
	// 825B6F5C: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 825B6F60: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B6F64: 48B0C9B5  bl 0x830c3918
	ctx.lr = 0x825B6F68;
	sub_830C3918(ctx, base);
	// 825B6F68: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 825B6F6C: 386100A0  addi r3, r1, 0xa0
	ctx.r[3].s64 = ctx.r[1].s64 + 160;
	// 825B6F70: 48842819  bl 0x82df9788
	ctx.lr = 0x825B6F74;
	sub_82DF9788(ctx, base);
	// 825B6F74: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B6F78: 3B60FFFF  li r27, -1
	ctx.r[27].s64 = -1;
	// 825B6F7C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825B6F80: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 825B6F84: 388100A0  addi r4, r1, 0xa0
	ctx.r[4].s64 = ctx.r[1].s64 + 160;
	// 825B6F88: 386B0090  addi r3, r11, 0x90
	ctx.r[3].s64 = ctx.r[11].s64 + 144;
	// 825B6F8C: 4BD121A5  bl 0x822c9130
	ctx.lr = 0x825B6F90;
	sub_822C9130(ctx, base);
	// 825B6F90: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825B6F94: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 825B6F98: 386100A0  addi r3, r1, 0xa0
	ctx.r[3].s64 = ctx.r[1].s64 + 160;
	// 825B6F9C: 4BD11E95  bl 0x822c8e30
	ctx.lr = 0x825B6FA0;
	sub_822C8E30(ctx, base);
	// 825B6FA0: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 825B6FA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825B6FA8: 38A0003C  li r5, 0x3c
	ctx.r[5].s64 = 60;
	// 825B6FAC: 386000C0  li r3, 0xc0
	ctx.r[3].s64 = 192;
	// 825B6FB0: 4BD09429  bl 0x822c03d8
	ctx.lr = 0x825B6FB4;
	sub_822C03D8(ctx, base);
	// 825B6FB4: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 825B6FB8: 41820058  beq 0x825b7010
	if ctx.cr[0].eq {
	pc = 0x825B7010; continue 'dispatch;
	}
	// 825B6FBC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 825B6FC0: D3C10060  stfs f30, 0x60(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), tmp.u32 ) };
	// 825B6FC4: D3E10068  stfs f31, 0x68(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), tmp.u32 ) };
	// 825B6FC8: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 825B6FCC: D3E1006C  stfs f31, 0x6c(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), tmp.u32 ) };
	// 825B6FD0: 386100E0  addi r3, r1, 0xe0
	ctx.r[3].s64 = ctx.r[1].s64 + 224;
	// 825B6FD4: D3A10070  stfs f29, 0x70(r1)
	tmp.f32 = (ctx.f[29].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), tmp.u32 ) };
	// 825B6FD8: D3A10074  stfs f29, 0x74(r1)
	tmp.f32 = (ctx.f[29].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), tmp.u32 ) };
	// 825B6FDC: C00BE4E4  lfs f0, -0x1b1c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-6940 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825B6FE0: D0010064  stfs f0, 0x64(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), tmp.u32 ) };
	// 825B6FE4: D3E10078  stfs f31, 0x78(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), tmp.u32 ) };
	// 825B6FE8: D3E1007C  stfs f31, 0x7c(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), tmp.u32 ) };
	// 825B6FEC: 4BD12305  bl 0x822c92f0
	ctx.lr = 0x825B6FF0;
	sub_822C92F0(ctx, base);
	// 825B6FF0: 7C671B78  mr r7, r3
	ctx.r[7].u64 = ctx.r[3].u64;
	// 825B6FF4: 38C10060  addi r6, r1, 0x60
	ctx.r[6].s64 = ctx.r[1].s64 + 96;
	// 825B6FF8: 38A10070  addi r5, r1, 0x70
	ctx.r[5].s64 = ctx.r[1].s64 + 112;
	// 825B6FFC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825B7000: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 825B7004: 48B11165  bl 0x830c8168
	ctx.lr = 0x825B7008;
	sub_830C8168(ctx, base);
	// 825B7008: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 825B700C: 48000008  b 0x825b7014
	pc = 0x825B7014; continue 'dispatch;
	// 825B7010: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 825B7014: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 825B7018: 4BD179C9  bl 0x822ce9e0
	ctx.lr = 0x825B701C;
	sub_822CE9E0(ctx, base);
	// 825B701C: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 825B7020: 815C0000  lwz r10, 0(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B7024: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825B7028: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 825B702C: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 825B7030: 419A0024  beq cr6, 0x825b7054
	if ctx.cr[6].eq {
	pc = 0x825B7054; continue 'dispatch;
	}
	// 825B7034: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 825B7038: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 825B703C: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 825B7040: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 825B7044: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 825B7048: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 825B704C: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 825B7050: 4082FFE8  bne 0x825b7038
	if !ctx.cr[0].eq {
	pc = 0x825B7038; continue 'dispatch;
	}
	// 825B7054: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 825B7058: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B705C: 48B0C8BD  bl 0x830c3918
	ctx.lr = 0x825B7060;
	sub_830C3918(ctx, base);
	// 825B7060: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 825B7064: 386100C0  addi r3, r1, 0xc0
	ctx.r[3].s64 = ctx.r[1].s64 + 192;
	// 825B7068: 388BBF40  addi r4, r11, -0x40c0
	ctx.r[4].s64 = ctx.r[11].s64 + -16576;
	// 825B706C: 4BD12285  bl 0x822c92f0
	ctx.lr = 0x825B7070;
	sub_822C92F0(ctx, base);
	// 825B7070: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B7074: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 825B7078: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825B707C: 388100C0  addi r4, r1, 0xc0
	ctx.r[4].s64 = ctx.r[1].s64 + 192;
	// 825B7080: 386B0090  addi r3, r11, 0x90
	ctx.r[3].s64 = ctx.r[11].s64 + 144;
	// 825B7084: 4BD120AD  bl 0x822c9130
	ctx.lr = 0x825B7088;
	sub_822C9130(ctx, base);
	// 825B7088: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825B708C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 825B7090: 386100C0  addi r3, r1, 0xc0
	ctx.r[3].s64 = ctx.r[1].s64 + 192;
	// 825B7094: 4BD11D9D  bl 0x822c8e30
	ctx.lr = 0x825B7098;
	sub_822C8E30(ctx, base);
	// 825B7098: 817F00D4  lwz r11, 0xd4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(212 as u32) ) } as u64;
	// 825B709C: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 825B70A0: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 825B70A4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B70A8: 796B0020  clrldi r11, r11, 0x20
	ctx.r[11].u64 = ctx.r[11].u64 & 0x00000000FFFFFFFFu64;
	// 825B70AC: F9610058  std r11, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u64 ) };
	// 825B70B0: C8010058  lfd f0, 0x58(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 825B70B4: FDA0069C  fcfid f13, f0
	ctx.f[13].f64 = (ctx.f[0].s64 as f64);
	// 825B70B8: C00ABED8  lfs f0, -0x4128(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-16680 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825B70BC: FDA06818  frsp f13, f13
	ctx.f[13].f64 = (ctx.f[13].f64 as f32) as f64;
	// 825B70C0: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B70C4: D3CB0030  stfs f30, 0x30(r11)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(48 as u32), tmp.u32 ) };
	// 825B70C8: EC0D0032  fmuls f0, f13, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 825B70CC: EC00F02A  fadds f0, f0, f30
	ctx.f[0].f64 = ((ctx.f[0].f64 + ctx.f[30].f64) as f32) as f64;
	// 825B70D0: D00B0034  stfs f0, 0x34(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(52 as u32), tmp.u32 ) };
	// 825B70D4: 38210160  addi r1, r1, 0x160
	ctx.r[1].s64 = ctx.r[1].s64 + 352;
	// 825B70D8: CBA1FFA0  lfd f29, -0x60(r1)
	ctx.f[29].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-96 as u32) ) };
	// 825B70DC: CBC1FFA8  lfd f30, -0x58(r1)
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-88 as u32) ) };
	// 825B70E0: CBE1FFB0  lfd f31, -0x50(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-80 as u32) ) };
	// 825B70E4: 48BF10C4  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B70E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825B70E8 size=76
    let mut pc: u32 = 0x825B70E8;
    'dispatch: loop {
        match pc {
            0x825B70E8 => {
    //   block [0x825B70E8..0x825B7134)
	// 825B70E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B70EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825B70F0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825B70F4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825B70F8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B70FC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825B7100: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825B7104: 4BFFD29D  bl 0x825b43a0
	ctx.lr = 0x825B7108;
	sub_825B43A0(ctx, base);
	// 825B7108: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825B710C: 4182000C  beq 0x825b7118
	if ctx.cr[0].eq {
	pc = 0x825B7118; continue 'dispatch;
	}
	// 825B7110: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B7114: 4BD09155  bl 0x822c0268
	ctx.lr = 0x825B7118;
	sub_822C0268(ctx, base);
	// 825B7118: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B711C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825B7120: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825B7124: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825B7128: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825B712C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825B7130: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B7138(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825B7138 size=92
    let mut pc: u32 = 0x825B7138;
    'dispatch: loop {
        match pc {
            0x825B7138 => {
    //   block [0x825B7138..0x825B7194)
	// 825B7138: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B713C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825B7140: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825B7144: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825B7148: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B714C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825B7150: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825B7154: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825B7158: 4883CAA9  bl 0x82df3c00
	ctx.lr = 0x825B715C;
	sub_82DF3C00(ctx, base);
	// 825B715C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B7160: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 825B7164: 4883CA9D  bl 0x82df3c00
	ctx.lr = 0x825B7168;
	sub_82DF3C00(ctx, base);
	// 825B7168: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825B716C: 4883C2BD  bl 0x82df3428
	ctx.lr = 0x825B7170;
	sub_82DF3428(ctx, base);
	// 825B7170: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825B7174: 4883C2B5  bl 0x82df3428
	ctx.lr = 0x825B7178;
	sub_82DF3428(ctx, base);
	// 825B7178: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B717C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825B7180: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825B7184: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825B7188: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825B718C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825B7190: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B7198(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825B7198 size=64
    let mut pc: u32 = 0x825B7198;
    'dispatch: loop {
        match pc {
            0x825B7198 => {
    //   block [0x825B7198..0x825B71D8)
	// 825B7198: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B719C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825B71A0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825B71A4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B71A8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 825B71AC: 38830130  addi r4, r3, 0x130
	ctx.r[4].s64 = ctx.r[3].s64 + 304;
	// 825B71B0: 807F003C  lwz r3, 0x3c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) } as u64;
	// 825B71B4: 4BFFAC5D  bl 0x825b1e10
	ctx.lr = 0x825B71B8;
	sub_825B1E10(ctx, base);
	// 825B71B8: 815F003C  lwz r10, 0x3c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) } as u64;
	// 825B71BC: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 825B71C0: 996A01C9  stb r11, 0x1c9(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(457 as u32), ctx.r[11].u8 ) };
	// 825B71C4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825B71C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825B71CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825B71D0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825B71D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B71D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825B71D8 size=84
    let mut pc: u32 = 0x825B71D8;
    'dispatch: loop {
        match pc {
            0x825B71D8 => {
    //   block [0x825B71D8..0x825B722C)
	// 825B71D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B71DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825B71E0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825B71E4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B71E8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825B71EC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825B71F0: 909F0120  stw r4, 0x120(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(288 as u32), ctx.r[4].u32 ) };
	// 825B71F4: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B71F8: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 825B71FC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825B7200: 4E800421  bctrl
	ctx.lr = 0x825B7204;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825B7204: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 825B7208: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B720C: 4BFFF11D  bl 0x825b6328
	ctx.lr = 0x825B7210;
	sub_825B6328(ctx, base);
	// 825B7210: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825B7214: 4883C215  bl 0x82df3428
	ctx.lr = 0x825B7218;
	sub_82DF3428(ctx, base);
	// 825B7218: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825B721C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825B7220: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825B7224: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825B7228: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B7230(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825B7230 size=136
    let mut pc: u32 = 0x825B7230;
    'dispatch: loop {
        match pc {
            0x825B7230 => {
    //   block [0x825B7230..0x825B72B8)
	// 825B7230: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B7234: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825B7238: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825B723C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825B7240: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B7244: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 825B7248: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 825B724C: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 825B7250: 409A0020  bne cr6, 0x825b7270
	if !ctx.cr[6].eq {
	pc = 0x825B7270; continue 'dispatch;
	}
	// 825B7254: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 825B7258: 419A0048  beq cr6, 0x825b72a0
	if ctx.cr[6].eq {
	pc = 0x825B72A0; continue 'dispatch;
	}
	// 825B725C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B7260: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825B7264: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 825B7268: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 825B726C: 48000034  b 0x825b72a0
	pc = 0x825B72A0; continue 'dispatch;
	// 825B7270: 2F050001  cmpwi cr6, r5, 1
	ctx.cr[6].compare_i32(ctx.r[5].s32, 1, &mut ctx.xer);
	// 825B7274: 419A002C  beq cr6, 0x825b72a0
	if ctx.cr[6].eq {
	pc = 0x825B72A0; continue 'dispatch;
	}
	// 825B7278: 3D608329  lis r11, -0x7cd7
	ctx.r[11].s64 = -2094465024;
	// 825B727C: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B7280: 388B9200  addi r4, r11, -0x6e00
	ctx.r[4].s64 = ctx.r[11].s64 + -28160;
	// 825B7284: 48BF0E75  bl 0x831a80f8
	ctx.lr = 0x825B7288;
	sub_831A80F8(ctx, base);
	// 825B7288: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825B728C: 4182000C  beq 0x825b7298
	if ctx.cr[0].eq {
	pc = 0x825B7298; continue 'dispatch;
	}
	// 825B7290: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 825B7294: 4800000C  b 0x825b72a0
	pc = 0x825B72A0; continue 'dispatch;
	// 825B7298: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825B729C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825B72A0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825B72A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825B72A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825B72AC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825B72B0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825B72B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B72B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825B72B8 size=72
    let mut pc: u32 = 0x825B72B8;
    'dispatch: loop {
        match pc {
            0x825B72B8 => {
    //   block [0x825B72B8..0x825B7300)
	// 825B72B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B72BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825B72C0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825B72C4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B72C8: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 825B72CC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825B72D0: 816B853C  lwz r11, -0x7ac4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-31428 as u32) ) } as u64;
	// 825B72D4: 7F045840  cmplw cr6, r4, r11
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[11].u32, &mut ctx.xer);
	// 825B72D8: 419A0014  beq cr6, 0x825b72ec
	if ctx.cr[6].eq {
	pc = 0x825B72EC; continue 'dispatch;
	}
	// 825B72DC: 807F0120  lwz r3, 0x120(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(288 as u32) ) } as u64;
	// 825B72E0: 4BFF3889  bl 0x825aab68
	ctx.lr = 0x825B72E4;
	sub_825AAB68(ctx, base);
	// 825B72E4: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 825B72E8: 997F0124  stb r11, 0x124(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(292 as u32), ctx.r[11].u8 ) };
	// 825B72EC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825B72F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825B72F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825B72F8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825B72FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B7300(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825B7300 size=72
    let mut pc: u32 = 0x825B7300;
    'dispatch: loop {
        match pc {
            0x825B7300 => {
    //   block [0x825B7300..0x825B7348)
	// 825B7300: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B7304: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825B7308: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825B730C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B7310: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 825B7314: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825B7318: 816B853C  lwz r11, -0x7ac4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-31428 as u32) ) } as u64;
	// 825B731C: 7F045840  cmplw cr6, r4, r11
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[11].u32, &mut ctx.xer);
	// 825B7320: 419A0014  beq cr6, 0x825b7334
	if ctx.cr[6].eq {
	pc = 0x825B7334; continue 'dispatch;
	}
	// 825B7324: 807F0120  lwz r3, 0x120(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(288 as u32) ) } as u64;
	// 825B7328: 4BFF34C9  bl 0x825aa7f0
	ctx.lr = 0x825B732C;
	sub_825AA7F0(ctx, base);
	// 825B732C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 825B7330: 997F0124  stb r11, 0x124(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(292 as u32), ctx.r[11].u8 ) };
	// 825B7334: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825B7338: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825B733C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825B7340: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825B7344: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B7348(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825B7348 size=196
    let mut pc: u32 = 0x825B7348;
    'dispatch: loop {
        match pc {
            0x825B7348 => {
    //   block [0x825B7348..0x825B740C)
	// 825B7348: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B734C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825B7350: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825B7354: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825B7358: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B735C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 825B7360: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825B7364: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 825B7368: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 825B736C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825B7370: 4BD095C9  bl 0x822c0938
	ctx.lr = 0x825B7374;
	sub_822C0938(ctx, base);
	// 825B7374: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 825B7378: 41820028  beq 0x825b73a0
	if ctx.cr[0].eq {
	pc = 0x825B73A0; continue 'dispatch;
	}
	// 825B737C: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 825B7380: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 825B7384: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825B7388: 392BC044  addi r9, r11, -0x3fbc
	ctx.r[9].s64 = ctx.r[11].s64 + -16316;
	// 825B738C: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 825B7390: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 825B7394: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 825B7398: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 825B739C: 48000008  b 0x825b73a4
	pc = 0x825B73A4; continue 'dispatch;
	// 825B73A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825B73A4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825B73A8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825B73AC: 409A0044  bne cr6, 0x825b73f0
	if !ctx.cr[6].eq {
	pc = 0x825B73F0; continue 'dispatch;
	}
	// 825B73B0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 825B73B4: 419A001C  beq cr6, 0x825b73d0
	if ctx.cr[6].eq {
	pc = 0x825B73D0; continue 'dispatch;
	}
	// 825B73B8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B73BC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 825B73C0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B73C4: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B73C8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825B73CC: 4E800421  bctrl
	ctx.lr = 0x825B73D0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825B73D0: 3D608329  lis r11, -0x7cd7
	ctx.r[11].s64 = -2094465024;
	// 825B73D4: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 825B73D8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825B73DC: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 825B73E0: 816B91FC  lwz r11, -0x6e04(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-28164 as u32) ) } as u64;
	// 825B73E4: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 825B73E8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825B73EC: 4BD08C15  bl 0x822c0000
	ctx.lr = 0x825B73F0;
	sub_822C0000(ctx, base);
	// 825B73F0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825B73F4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825B73F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825B73FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825B7400: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825B7404: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825B7408: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B7410(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825B7410 size=196
    let mut pc: u32 = 0x825B7410;
    'dispatch: loop {
        match pc {
            0x825B7410 => {
    //   block [0x825B7410..0x825B74D4)
	// 825B7410: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B7414: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825B7418: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825B741C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825B7420: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B7424: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 825B7428: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825B742C: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 825B7430: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 825B7434: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825B7438: 4BD09501  bl 0x822c0938
	ctx.lr = 0x825B743C;
	sub_822C0938(ctx, base);
	// 825B743C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 825B7440: 41820028  beq 0x825b7468
	if ctx.cr[0].eq {
	pc = 0x825B7468; continue 'dispatch;
	}
	// 825B7444: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 825B7448: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 825B744C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825B7450: 392BC058  addi r9, r11, -0x3fa8
	ctx.r[9].s64 = ctx.r[11].s64 + -16296;
	// 825B7454: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 825B7458: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 825B745C: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 825B7460: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 825B7464: 48000008  b 0x825b746c
	pc = 0x825B746C; continue 'dispatch;
	// 825B7468: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825B746C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825B7470: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825B7474: 409A0044  bne cr6, 0x825b74b8
	if !ctx.cr[6].eq {
	pc = 0x825B74B8; continue 'dispatch;
	}
	// 825B7478: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 825B747C: 419A001C  beq cr6, 0x825b7498
	if ctx.cr[6].eq {
	pc = 0x825B7498; continue 'dispatch;
	}
	// 825B7480: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B7484: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 825B7488: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B748C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B7490: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825B7494: 4E800421  bctrl
	ctx.lr = 0x825B7498;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825B7498: 3D608329  lis r11, -0x7cd7
	ctx.r[11].s64 = -2094465024;
	// 825B749C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 825B74A0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825B74A4: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 825B74A8: 816B91FC  lwz r11, -0x6e04(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-28164 as u32) ) } as u64;
	// 825B74AC: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 825B74B0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825B74B4: 4BD08B4D  bl 0x822c0000
	ctx.lr = 0x825B74B8;
	sub_822C0000(ctx, base);
	// 825B74B8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825B74BC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825B74C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825B74C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825B74C8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825B74CC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825B74D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B74D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825B74D8 size=72
    let mut pc: u32 = 0x825B74D8;
    'dispatch: loop {
        match pc {
            0x825B74D8 => {
    //   block [0x825B74D8..0x825B7520)
	// 825B74D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B74DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825B74E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B74E4: 2F050003  cmpwi cr6, r5, 3
	ctx.cr[6].compare_i32(ctx.r[5].s32, 3, &mut ctx.xer);
	// 825B74E8: 419A001C  beq cr6, 0x825b7504
	if ctx.cr[6].eq {
	pc = 0x825B7504; continue 'dispatch;
	}
	// 825B74EC: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 825B74F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825B74F4: 994B0000  stb r10, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 825B74F8: 88C10050  lbz r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 825B74FC: 4BFFFD35  bl 0x825b7230
	ctx.lr = 0x825B7500;
	sub_825B7230(ctx, base);
	// 825B7500: 48000010  b 0x825b7510
	pc = 0x825B7510; continue 'dispatch;
	// 825B7504: 3D608329  lis r11, -0x7cd7
	ctx.r[11].s64 = -2094465024;
	// 825B7508: 396B9200  addi r11, r11, -0x6e00
	ctx.r[11].s64 = ctx.r[11].s64 + -28160;
	// 825B750C: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825B7510: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825B7514: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825B7518: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825B751C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B7520(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825B7520 size=136
    let mut pc: u32 = 0x825B7520;
    'dispatch: loop {
        match pc {
            0x825B7520 => {
    //   block [0x825B7520..0x825B75A8)
	// 825B7520: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B7524: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825B7528: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825B752C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B7530: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825B7534: 80A40020  lwz r5, 0x20(r4)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(32 as u32) ) } as u64;
	// 825B7538: 81660000  lwz r11, 0(r6)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B753C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825B7540: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825B7544: 4E800421  bctrl
	ctx.lr = 0x825B7548;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825B7548: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B754C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825B7550: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 825B7554: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825B7558: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 825B755C: 419A0024  beq cr6, 0x825b7580
	if ctx.cr[6].eq {
	pc = 0x825B7580; continue 'dispatch;
	}
	// 825B7560: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 825B7564: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 825B7568: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 825B756C: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 825B7570: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 825B7574: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 825B7578: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 825B757C: 4082FFE8  bne 0x825b7564
	if !ctx.cr[0].eq {
	pc = 0x825B7564; continue 'dispatch;
	}
	// 825B7580: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 825B7584: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825B7588: 419A0008  beq cr6, 0x825b7590
	if ctx.cr[6].eq {
	pc = 0x825B7590; continue 'dispatch;
	}
	// 825B758C: 4BD09305  bl 0x822c0890
	ctx.lr = 0x825B7590;
	sub_822C0890(ctx, base);
	// 825B7590: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B7594: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825B7598: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825B759C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825B75A0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825B75A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B75A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825B75A8 size=80
    let mut pc: u32 = 0x825B75A8;
    'dispatch: loop {
        match pc {
            0x825B75A8 => {
    //   block [0x825B75A8..0x825B75F8)
	// 825B75A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B75AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825B75B0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825B75B4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B75B8: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 825B75BC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825B75C0: 7C862378  mr r6, r4
	ctx.r[6].u64 = ctx.r[4].u64;
	// 825B75C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825B75C8: 38E10051  addi r7, r1, 0x51
	ctx.r[7].s64 = ctx.r[1].s64 + 81;
	// 825B75CC: 994B0000  stb r10, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 825B75D0: 38860008  addi r4, r6, 8
	ctx.r[4].s64 = ctx.r[6].s64 + 8;
	// 825B75D4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825B75D8: 88A10050  lbz r5, 0x50(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 825B75DC: 4BFFFF45  bl 0x825b7520
	ctx.lr = 0x825B75E0;
	sub_825B7520(ctx, base);
	// 825B75E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B75E4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825B75E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825B75EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825B75F0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825B75F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B75F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825B75F8 size=128
    let mut pc: u32 = 0x825B75F8;
    'dispatch: loop {
        match pc {
            0x825B75F8 => {
    //   block [0x825B75F8..0x825B7678)
	// 825B75F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B75FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825B7600: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825B7604: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B7608: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825B760C: 80840000  lwz r4, 0(r4)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B7610: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825B7614: 4BFFFF95  bl 0x825b75a8
	ctx.lr = 0x825B7618;
	sub_825B75A8(ctx, base);
	// 825B7618: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B761C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825B7620: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 825B7624: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825B7628: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 825B762C: 419A0024  beq cr6, 0x825b7650
	if ctx.cr[6].eq {
	pc = 0x825B7650; continue 'dispatch;
	}
	// 825B7630: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 825B7634: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 825B7638: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 825B763C: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 825B7640: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 825B7644: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 825B7648: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 825B764C: 4082FFE8  bne 0x825b7634
	if !ctx.cr[0].eq {
	pc = 0x825B7634; continue 'dispatch;
	}
	// 825B7650: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 825B7654: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825B7658: 419A0008  beq cr6, 0x825b7660
	if ctx.cr[6].eq {
	pc = 0x825B7660; continue 'dispatch;
	}
	// 825B765C: 4BD09235  bl 0x822c0890
	ctx.lr = 0x825B7660;
	sub_822C0890(ctx, base);
	// 825B7660: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B7664: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825B7668: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825B766C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825B7670: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825B7674: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B7678(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825B7678 size=96
    let mut pc: u32 = 0x825B7678;
    'dispatch: loop {
        match pc {
            0x825B7678 => {
    //   block [0x825B7678..0x825B76D8)
	// 825B7678: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B767C: 48BF0AF1  bl 0x831a816c
	ctx.lr = 0x825B7680;
	sub_831A8130(ctx, base);
	// 825B7680: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B7684: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825B7688: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 825B768C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825B7690: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 825B7694: 4BFD487D  bl 0x8258bf10
	ctx.lr = 0x825B7698;
	sub_8258BF10(ctx, base);
	// 825B7698: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 825B769C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825B76A0: 4BFD4871  bl 0x8258bf10
	ctx.lr = 0x825B76A4;
	sub_8258BF10(ctx, base);
	// 825B76A4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B76A8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 825B76AC: 4BFD4865  bl 0x8258bf10
	ctx.lr = 0x825B76B0;
	sub_8258BF10(ctx, base);
	// 825B76B0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825B76B4: 4BD11605  bl 0x822c8cb8
	ctx.lr = 0x825B76B8;
	sub_822C8CB8(ctx, base);
	// 825B76B8: 93BF0020  stw r29, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[29].u32 ) };
	// 825B76BC: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 825B76C0: 4BD115F9  bl 0x822c8cb8
	ctx.lr = 0x825B76C4;
	sub_822C8CB8(ctx, base);
	// 825B76C4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825B76C8: 4BD115F1  bl 0x822c8cb8
	ctx.lr = 0x825B76CC;
	sub_822C8CB8(ctx, base);
	// 825B76CC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B76D0: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 825B76D4: 48BF0AE8  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B76D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825B76D8 size=84
    let mut pc: u32 = 0x825B76D8;
    'dispatch: loop {
        match pc {
            0x825B76D8 => {
    //   block [0x825B76D8..0x825B772C)
	// 825B76D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B76DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825B76E0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825B76E4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825B76E8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B76EC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825B76F0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825B76F4: 389E0008  addi r4, r30, 8
	ctx.r[4].s64 = ctx.r[30].s64 + 8;
	// 825B76F8: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 825B76FC: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B7700: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825B7704: 4BFD480D  bl 0x8258bf10
	ctx.lr = 0x825B7708;
	sub_8258BF10(ctx, base);
	// 825B7708: 817E0028  lwz r11, 0x28(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 825B770C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B7710: 917F0028  stw r11, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 825B7714: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825B7718: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825B771C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825B7720: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825B7724: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825B7728: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B7730(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825B7730 size=116
    let mut pc: u32 = 0x825B7730;
    'dispatch: loop {
        match pc {
            0x825B7730 => {
    //   block [0x825B7730..0x825B77A4)
	// 825B7730: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B7734: 48BF0A35  bl 0x831a8168
	ctx.lr = 0x825B7738;
	sub_831A8130(ctx, base);
	// 825B7738: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B773C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 825B7740: 90C10050  stw r6, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[6].u32 ) };
	// 825B7744: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825B7748: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825B774C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 825B7750: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 825B7754: 3B810050  addi r28, r1, 0x50
	ctx.r[28].s64 = ctx.r[1].s64 + 80;
	// 825B7758: 4BFD47B9  bl 0x8258bf10
	ctx.lr = 0x825B775C;
	sub_8258BF10(ctx, base);
	// 825B775C: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 825B7760: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 825B7764: 80BC0000  lwz r5, 0(r28)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B7768: 4BFFFF11  bl 0x825b7678
	ctx.lr = 0x825B776C;
	sub_825B7678(ctx, base);
	// 825B776C: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 825B7770: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 825B7774: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 825B7778: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 825B777C: 4BFD4795  bl 0x8258bf10
	ctx.lr = 0x825B7780;
	sub_8258BF10(ctx, base);
	// 825B7780: 817E0020  lwz r11, 0x20(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 825B7784: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 825B7788: 917F0028  stw r11, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 825B778C: 4BD1152D  bl 0x822c8cb8
	ctx.lr = 0x825B7790;
	sub_822C8CB8(ctx, base);
	// 825B7790: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 825B7794: 4BD11525  bl 0x822c8cb8
	ctx.lr = 0x825B7798;
	sub_822C8CB8(ctx, base);
	// 825B7798: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B779C: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 825B77A0: 48BF0A18  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B77A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825B77A8 size=128
    let mut pc: u32 = 0x825B77A8;
    'dispatch: loop {
        match pc {
            0x825B77A8 => {
    //   block [0x825B77A8..0x825B7828)
	// 825B77A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B77AC: 48BF09C1  bl 0x831a816c
	ctx.lr = 0x825B77B0;
	sub_831A8130(ctx, base);
	// 825B77B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B77B4: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 825B77B8: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 825B77BC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 825B77C0: 3BEB7BC8  addi r31, r11, 0x7bc8
	ctx.r[31].s64 = ctx.r[11].s64 + 31688;
	// 825B77C4: 816A7BD0  lwz r11, 0x7bd0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(31696 as u32) ) } as u64;
	// 825B77C8: 556907FF  clrlwi. r9, r11, 0x1f
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 825B77CC: 40820024  bne 0x825b77f0
	if !ctx.cr[0].eq {
	pc = 0x825B77F0; continue 'dispatch;
	}
	// 825B77D0: 3D208256  lis r9, -0x7daa
	ctx.r[9].s64 = -2108293120;
	// 825B77D4: 3D00825B  lis r8, -0x7da5
	ctx.r[8].s64 = -2107965440;
	// 825B77D8: 616B0001  ori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 | 1;
	// 825B77DC: 3929DA70  addi r9, r9, -0x2590
	ctx.r[9].s64 = ctx.r[9].s64 + -9616;
	// 825B77E0: 390874D8  addi r8, r8, 0x74d8
	ctx.r[8].s64 = ctx.r[8].s64 + 29912;
	// 825B77E4: 916A7BD0  stw r11, 0x7bd0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31696 as u32), ctx.r[11].u32 ) };
	// 825B77E8: 913F0004  stw r9, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 825B77EC: 911F0000  stw r8, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 825B77F0: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 825B77F4: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 825B77F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B77FC: 38BE0008  addi r5, r30, 8
	ctx.r[5].s64 = ctx.r[30].s64 + 8;
	// 825B7800: 9BAB0000  stb r29, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u8 ) };
	// 825B7804: 88C10050  lbz r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 825B7808: 4801B119  bl 0x825d2920
	ctx.lr = 0x825B780C;
	sub_825D2920(ctx, base);
	// 825B780C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825B7810: 4182000C  beq 0x825b781c
	if ctx.cr[0].eq {
	pc = 0x825B781C; continue 'dispatch;
	}
	// 825B7814: 93FE0000  stw r31, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 825B7818: 48000008  b 0x825b7820
	pc = 0x825B7820; continue 'dispatch;
	// 825B781C: 93BE0000  stw r29, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 825B7820: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825B7824: 48BF0998  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B7828(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825B7828 size=128
    let mut pc: u32 = 0x825B7828;
    'dispatch: loop {
        match pc {
            0x825B7828 => {
    //   block [0x825B7828..0x825B78A8)
	// 825B7828: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B782C: 48BF093D  bl 0x831a8168
	ctx.lr = 0x825B7830;
	sub_831A8130(ctx, base);
	// 825B7830: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B7834: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 825B7838: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 825B783C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 825B7840: 48217999  bl 0x827cf1d8
	ctx.lr = 0x825B7844;
	sub_827CF1D8(ctx, base);
	// 825B7844: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825B7848: 40820058  bne 0x825b78a0
	if !ctx.cr[0].eq {
	pc = 0x825B78A0; continue 'dispatch;
	}
	// 825B784C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 825B7850: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825B7854: 4BFFFE85  bl 0x825b76d8
	ctx.lr = 0x825B7858;
	sub_825B76D8(ctx, base);
	// 825B7858: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 825B785C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 825B7860: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 825B7864: 4BD2416D  bl 0x822db9d0
	ctx.lr = 0x825B7868;
	sub_822DB9D0(ctx, base);
	// 825B7868: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 825B786C: 41820010  beq 0x825b787c
	if ctx.cr[0].eq {
	pc = 0x825B787C; continue 'dispatch;
	}
	// 825B7870: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 825B7874: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B7878: 4BFFFE61  bl 0x825b76d8
	ctx.lr = 0x825B787C;
	sub_825B76D8(ctx, base);
	// 825B787C: 93FC0000  stw r31, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 825B7880: 387E0008  addi r3, r30, 8
	ctx.r[3].s64 = ctx.r[30].s64 + 8;
	// 825B7884: 4BD11435  bl 0x822c8cb8
	ctx.lr = 0x825B7888;
	sub_822C8CB8(ctx, base);
	// 825B7888: 3BE00001  li r31, 1
	ctx.r[31].s64 = 1;
	// 825B788C: 387D0008  addi r3, r29, 8
	ctx.r[3].s64 = ctx.r[29].s64 + 8;
	// 825B7890: 4BD11429  bl 0x822c8cb8
	ctx.lr = 0x825B7894;
	sub_822C8CB8(ctx, base);
	// 825B7894: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B7898: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 825B789C: 48BF091C  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
	// 825B78A0: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 825B78A4: 4BFFFFE8  b 0x825b788c
	pc = 0x825B788C; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B78A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825B78A8 size=88
    let mut pc: u32 = 0x825B78A8;
    'dispatch: loop {
        match pc {
            0x825B78A8 => {
    //   block [0x825B78A8..0x825B7900)
	// 825B78A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B78AC: 48BF08C1  bl 0x831a816c
	ctx.lr = 0x825B78B0;
	sub_831A8130(ctx, base);
	// 825B78B0: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B78B4: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 825B78B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825B78BC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 825B78C0: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 825B78C4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 825B78C8: 994B0000  stb r10, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 825B78CC: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 825B78D0: 4BFFFE09  bl 0x825b76d8
	ctx.lr = 0x825B78D4;
	sub_825B76D8(ctx, base);
	// 825B78D4: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 825B78D8: 88C10050  lbz r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 825B78DC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825B78E0: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 825B78E4: 4BFFFF45  bl 0x825b7828
	ctx.lr = 0x825B78E8;
	sub_825B7828(ctx, base);
	// 825B78E8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 825B78EC: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 825B78F0: 4BD113C9  bl 0x822c8cb8
	ctx.lr = 0x825B78F4;
	sub_822C8CB8(ctx, base);
	// 825B78F4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825B78F8: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 825B78FC: 48BF08C0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B7900(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825B7900 size=148
    let mut pc: u32 = 0x825B7900;
    'dispatch: loop {
        match pc {
            0x825B7900 => {
    //   block [0x825B7900..0x825B7994)
	// 825B7900: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B7904: 48BF0869  bl 0x831a816c
	ctx.lr = 0x825B7908;
	sub_831A8130(ctx, base);
	// 825B7908: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B790C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825B7910: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825B7914: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 825B7918: 409A0030  bne cr6, 0x825b7948
	if !ctx.cr[6].eq {
	pc = 0x825B7948; continue 'dispatch;
	}
	// 825B791C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 825B7920: 83BF0000  lwz r29, 0(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B7924: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 825B7928: 4BD240A9  bl 0x822db9d0
	ctx.lr = 0x825B792C;
	sub_822DB9D0(ctx, base);
	// 825B792C: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 825B7930: 41820010  beq 0x825b7940
	if ctx.cr[0].eq {
	pc = 0x825B7940; continue 'dispatch;
	}
	// 825B7934: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 825B7938: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B793C: 4BFFFD9D  bl 0x825b76d8
	ctx.lr = 0x825B7940;
	sub_825B76D8(ctx, base);
	// 825B7940: 93FE0000  stw r31, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 825B7944: 48000028  b 0x825b796c
	pc = 0x825B796C; continue 'dispatch;
	// 825B7948: 2F050001  cmpwi cr6, r5, 1
	ctx.cr[6].compare_i32(ctx.r[5].s32, 1, &mut ctx.xer);
	// 825B794C: 409A0028  bne cr6, 0x825b7974
	if !ctx.cr[6].eq {
	pc = 0x825B7974; continue 'dispatch;
	}
	// 825B7950: 83FE0000  lwz r31, 0(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B7954: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 825B7958: 4BD11361  bl 0x822c8cb8
	ctx.lr = 0x825B795C;
	sub_822C8CB8(ctx, base);
	// 825B795C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B7960: 4BD08909  bl 0x822c0268
	ctx.lr = 0x825B7964;
	sub_822C0268(ctx, base);
	// 825B7964: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825B7968: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825B796C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825B7970: 48BF084C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 825B7974: 3D608329  lis r11, -0x7cd7
	ctx.r[11].s64 = -2094465024;
	// 825B7978: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B797C: 388B9340  addi r4, r11, -0x6cc0
	ctx.r[4].s64 = ctx.r[11].s64 + -27840;
	// 825B7980: 48BF0779  bl 0x831a80f8
	ctx.lr = 0x825B7984;
	sub_831A80F8(ctx, base);
	// 825B7984: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825B7988: 4182FFDC  beq 0x825b7964
	if ctx.cr[0].eq {
	pc = 0x825B7964; continue 'dispatch;
	}
	// 825B798C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B7990: 4BFFFFD8  b 0x825b7968
	pc = 0x825B7968; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B7998(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825B7998 size=72
    let mut pc: u32 = 0x825B7998;
    'dispatch: loop {
        match pc {
            0x825B7998 => {
    //   block [0x825B7998..0x825B79E0)
	// 825B7998: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B799C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825B79A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B79A4: 2F050003  cmpwi cr6, r5, 3
	ctx.cr[6].compare_i32(ctx.r[5].s32, 3, &mut ctx.xer);
	// 825B79A8: 419A001C  beq cr6, 0x825b79c4
	if ctx.cr[6].eq {
	pc = 0x825B79C4; continue 'dispatch;
	}
	// 825B79AC: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 825B79B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825B79B4: 994B0000  stb r10, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 825B79B8: 88C10050  lbz r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 825B79BC: 4BFFFF45  bl 0x825b7900
	ctx.lr = 0x825B79C0;
	sub_825B7900(ctx, base);
	// 825B79C0: 48000010  b 0x825b79d0
	pc = 0x825B79D0; continue 'dispatch;
	// 825B79C4: 3D608329  lis r11, -0x7cd7
	ctx.r[11].s64 = -2094465024;
	// 825B79C8: 396B9340  addi r11, r11, -0x6cc0
	ctx.r[11].s64 = ctx.r[11].s64 + -27840;
	// 825B79CC: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825B79D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825B79D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825B79D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825B79DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B79E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825B79E0 size=100
    let mut pc: u32 = 0x825B79E0;
    'dispatch: loop {
        match pc {
            0x825B79E0 => {
    //   block [0x825B79E0..0x825B7A44)
	// 825B79E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B79E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825B79E8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825B79EC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825B79F0: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B79F4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 825B79F8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825B79FC: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 825B7A00: 4BFFFCD9  bl 0x825b76d8
	ctx.lr = 0x825B7A04;
	sub_825B76D8(ctx, base);
	// 825B7A04: 3D60825B  lis r11, -0x7da5
	ctx.r[11].s64 = -2107965440;
	// 825B7A08: 3D40825B  lis r10, -0x7da5
	ctx.r[10].s64 = -2107965440;
	// 825B7A0C: 396B75F8  addi r11, r11, 0x75f8
	ctx.r[11].s64 = ctx.r[11].s64 + 30200;
	// 825B7A10: 394A7998  addi r10, r10, 0x7998
	ctx.r[10].s64 = ctx.r[10].s64 + 31128;
	// 825B7A14: 917E0004  stw r11, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 825B7A18: 38630008  addi r3, r3, 8
	ctx.r[3].s64 = ctx.r[3].s64 + 8;
	// 825B7A1C: 915E0000  stw r10, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 825B7A20: 4BD11299  bl 0x822c8cb8
	ctx.lr = 0x825B7A24;
	sub_822C8CB8(ctx, base);
	// 825B7A24: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 825B7A28: 4BD11291  bl 0x822c8cb8
	ctx.lr = 0x825B7A2C;
	sub_822C8CB8(ctx, base);
	// 825B7A2C: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 825B7A30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825B7A34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825B7A38: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825B7A3C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825B7A40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B7A48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825B7A48 size=96
    let mut pc: u32 = 0x825B7A48;
    'dispatch: loop {
        match pc {
            0x825B7A48 => {
    //   block [0x825B7A48..0x825B7AA8)
	// 825B7A48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B7A4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825B7A50: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825B7A54: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825B7A58: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B7A5C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 825B7A60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825B7A64: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825B7A68: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 825B7A6C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825B7A70: 917E0004  stw r11, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 825B7A74: 4BFFFC65  bl 0x825b76d8
	ctx.lr = 0x825B7A78;
	sub_825B76D8(ctx, base);
	// 825B7A78: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 825B7A7C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825B7A80: 4BFFFF61  bl 0x825b79e0
	ctx.lr = 0x825B7A84;
	sub_825B79E0(ctx, base);
	// 825B7A84: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 825B7A88: 4BD11231  bl 0x822c8cb8
	ctx.lr = 0x825B7A8C;
	sub_822C8CB8(ctx, base);
	// 825B7A8C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825B7A90: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 825B7A94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825B7A98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825B7A9C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825B7AA0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825B7AA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B7AA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825B7AA8 size=144
    let mut pc: u32 = 0x825B7AA8;
    'dispatch: loop {
        match pc {
            0x825B7AA8 => {
    //   block [0x825B7AA8..0x825B7B38)
	// 825B7AA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B7AAC: 48BF06BD  bl 0x831a8168
	ctx.lr = 0x825B7AB0;
	sub_831A8130(ctx, base);
	// 825B7AB0: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B7AB4: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 825B7AB8: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 825B7ABC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 825B7AC0: 3BEB7BD4  addi r31, r11, 0x7bd4
	ctx.r[31].s64 = ctx.r[11].s64 + 31700;
	// 825B7AC4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825B7AC8: 816A7BDC  lwz r11, 0x7bdc(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(31708 as u32) ) } as u64;
	// 825B7ACC: 556907FF  clrlwi. r9, r11, 0x1f
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 825B7AD0: 40820020  bne 0x825b7af0
	if !ctx.cr[0].eq {
	pc = 0x825B7AF0; continue 'dispatch;
	}
	// 825B7AD4: 616B0001  ori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 | 1;
	// 825B7AD8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825B7ADC: 916A7BDC  stw r11, 0x7bdc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31708 as u32), ctx.r[11].u32 ) };
	// 825B7AE0: 4BFFFBF9  bl 0x825b76d8
	ctx.lr = 0x825B7AE4;
	sub_825B76D8(ctx, base);
	// 825B7AE4: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 825B7AE8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B7AEC: 4BFFFF5D  bl 0x825b7a48
	ctx.lr = 0x825B7AF0;
	sub_825B7A48(ctx, base);
	// 825B7AF0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 825B7AF4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825B7AF8: 3B9D0008  addi r28, r29, 8
	ctx.r[28].s64 = ctx.r[29].s64 + 8;
	// 825B7AFC: 4BFFFBDD  bl 0x825b76d8
	ctx.lr = 0x825B7B00;
	sub_825B76D8(ctx, base);
	// 825B7B00: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 825B7B04: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B7B08: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 825B7B0C: 4BFFFD9D  bl 0x825b78a8
	ctx.lr = 0x825B7B10;
	sub_825B78A8(ctx, base);
	// 825B7B10: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825B7B14: 4182000C  beq 0x825b7b20
	if ctx.cr[0].eq {
	pc = 0x825B7B20; continue 'dispatch;
	}
	// 825B7B18: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 825B7B1C: 4800000C  b 0x825b7b28
	pc = 0x825B7B28; continue 'dispatch;
	// 825B7B20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825B7B24: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825B7B28: 387E0008  addi r3, r30, 8
	ctx.r[3].s64 = ctx.r[30].s64 + 8;
	// 825B7B2C: 4BD1118D  bl 0x822c8cb8
	ctx.lr = 0x825B7B30;
	sub_822C8CB8(ctx, base);
	// 825B7B30: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 825B7B34: 48BF0684  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B7B38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825B7B38 size=96
    let mut pc: u32 = 0x825B7B38;
    'dispatch: loop {
        match pc {
            0x825B7B38 => {
    //   block [0x825B7B38..0x825B7B98)
	// 825B7B38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B7B3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825B7B40: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825B7B44: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825B7B48: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B7B4C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825B7B50: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825B7B54: 807F012C  lwz r3, 0x12c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(300 as u32) ) } as u64;
	// 825B7B58: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825B7B5C: 419A0008  beq cr6, 0x825b7b64
	if ctx.cr[6].eq {
	pc = 0x825B7B64; continue 'dispatch;
	}
	// 825B7B60: 4BD08D31  bl 0x822c0890
	ctx.lr = 0x825B7B64;
	sub_822C0890(ctx, base);
	// 825B7B64: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B7B68: 4BFFC839  bl 0x825b43a0
	ctx.lr = 0x825B7B6C;
	sub_825B43A0(ctx, base);
	// 825B7B6C: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825B7B70: 4182000C  beq 0x825b7b7c
	if ctx.cr[0].eq {
	pc = 0x825B7B7C; continue 'dispatch;
	}
	// 825B7B74: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B7B78: 4BD086F1  bl 0x822c0268
	ctx.lr = 0x825B7B7C;
	sub_822C0268(ctx, base);
	// 825B7B7C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B7B80: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825B7B84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825B7B88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825B7B8C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825B7B90: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825B7B94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B7B98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825B7B98 size=92
    let mut pc: u32 = 0x825B7B98;
    'dispatch: loop {
        match pc {
            0x825B7B98 => {
    //   block [0x825B7B98..0x825B7BF4)
	// 825B7B98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B7B9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825B7BA0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825B7BA4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825B7BA8: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B7BAC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 825B7BB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825B7BB4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825B7BB8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 825B7BBC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825B7BC0: 4BFFFB19  bl 0x825b76d8
	ctx.lr = 0x825B7BC4;
	sub_825B76D8(ctx, base);
	// 825B7BC4: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 825B7BC8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825B7BCC: 4BFFFEDD  bl 0x825b7aa8
	ctx.lr = 0x825B7BD0;
	sub_825B7AA8(ctx, base);
	// 825B7BD0: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 825B7BD4: 4BD110E5  bl 0x822c8cb8
	ctx.lr = 0x825B7BD8;
	sub_822C8CB8(ctx, base);
	// 825B7BD8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825B7BDC: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 825B7BE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825B7BE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825B7BE8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825B7BEC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825B7BF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B7BF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x825B7BF8 size=384
    let mut pc: u32 = 0x825B7BF8;
    'dispatch: loop {
        match pc {
            0x825B7BF8 => {
    //   block [0x825B7BF8..0x825B7D78)
	// 825B7BF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B7BFC: 48BF0569  bl 0x831a8164
	ctx.lr = 0x825B7C00;
	sub_831A8130(ctx, base);
	// 825B7C00: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B7C04: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 825B7C08: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 825B7C0C: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 825B7C10: 3D008204  lis r8, -0x7dfc
	ctx.r[8].s64 = -2113667072;
	// 825B7C14: 3CE08201  lis r7, -0x7dff
	ctx.r[7].s64 = -2113863680;
	// 825B7C18: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 825B7C1C: C1ABBC10  lfs f13, -0x43f0(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-17392 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825B7C20: C18A9690  lfs f12, -0x6970(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-26992 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 825B7C24: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 825B7C28: 7FDDF378  mr r29, r30
	ctx.r[29].u64 = ctx.r[30].u64;
	// 825B7C2C: C00908A4  lfs f0, 0x8a4(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825B7C30: C168BC08  lfs f11, -0x43f8(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(-17400 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 825B7C34: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 825B7C38: C147959C  lfs f10, -0x6a64(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(-27236 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 825B7C3C: 93A10054  stw r29, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[29].u32 ) };
	// 825B7C40: D1A10070  stfs f13, 0x70(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), tmp.u32 ) };
	// 825B7C44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825B7C48: D1810074  stfs f12, 0x74(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), tmp.u32 ) };
	// 825B7C4C: 388BC0D8  addi r4, r11, -0x3f28
	ctx.r[4].s64 = ctx.r[11].s64 + -16168;
	// 825B7C50: D0010078  stfs f0, 0x78(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), tmp.u32 ) };
	// 825B7C54: 38A00019  li r5, 0x19
	ctx.r[5].s64 = 25;
	// 825B7C58: D001007C  stfs f0, 0x7c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), tmp.u32 ) };
	// 825B7C5C: 38600140  li r3, 0x140
	ctx.r[3].s64 = 320;
	// 825B7C60: D1610060  stfs f11, 0x60(r1)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), tmp.u32 ) };
	// 825B7C64: D1410064  stfs f10, 0x64(r1)
	tmp.f32 = (ctx.f[10].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), tmp.u32 ) };
	// 825B7C68: D0010068  stfs f0, 0x68(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), tmp.u32 ) };
	// 825B7C6C: D001006C  stfs f0, 0x6c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), tmp.u32 ) };
	// 825B7C70: 4BD08769  bl 0x822c03d8
	ctx.lr = 0x825B7C74;
	sub_822C03D8(ctx, base);
	// 825B7C74: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 825B7C78: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 825B7C7C: 3B6B1BC4  addi r27, r11, 0x1bc4
	ctx.r[27].s64 = ctx.r[11].s64 + 7108;
	// 825B7C80: 41820048  beq 0x825b7cc8
	if ctx.cr[0].eq {
	pc = 0x825B7CC8; continue 'dispatch;
	}
	// 825B7C84: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 825B7C88: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825B7C8C: 4883BD7D  bl 0x82df3a08
	ctx.lr = 0x825B7C90;
	sub_82DF3A08(ctx, base);
	// 825B7C90: 38C10060  addi r6, r1, 0x60
	ctx.r[6].s64 = ctx.r[1].s64 + 96;
	// 825B7C94: 38A10070  addi r5, r1, 0x70
	ctx.r[5].s64 = ctx.r[1].s64 + 112;
	// 825B7C98: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 825B7C9C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B7CA0: 3BA00001  li r29, 1
	ctx.r[29].s64 = 1;
	// 825B7CA4: 4BFFD115  bl 0x825b4db8
	ctx.lr = 0x825B7CA8;
	sub_825B4DB8(ctx, base);
	// 825B7CA8: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 825B7CAC: 397F0128  addi r11, r31, 0x128
	ctx.r[11].s64 = ctx.r[31].s64 + 296;
	// 825B7CB0: 9BDF0124  stb r30, 0x124(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(292 as u32), ctx.r[30].u8 ) };
	// 825B7CB4: 396AC06C  addi r11, r10, -0x3f94
	ctx.r[11].s64 = ctx.r[10].s64 + -16276;
	// 825B7CB8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825B7CBC: 93DF0128  stw r30, 0x128(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(296 as u32), ctx.r[30].u32 ) };
	// 825B7CC0: 93DF012C  stw r30, 0x12c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(300 as u32), ctx.r[30].u32 ) };
	// 825B7CC4: 48000008  b 0x825b7ccc
	pc = 0x825B7CCC; continue 'dispatch;
	// 825B7CC8: 7FDFF378  mr r31, r30
	ctx.r[31].u64 = ctx.r[30].u64;
	// 825B7CCC: 93FC0000  stw r31, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 825B7CD0: 3BDC0004  addi r30, r28, 4
	ctx.r[30].s64 = ctx.r[28].s64 + 4;
	// 825B7CD4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825B7CD8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825B7CDC: 4BFFF66D  bl 0x825b7348
	ctx.lr = 0x825B7CE0;
	sub_825B7348(ctx, base);
	// 825B7CE0: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 825B7CE4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825B7CE8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825B7CEC: 4BD08315  bl 0x822c0000
	ctx.lr = 0x825B7CF0;
	sub_822C0000(ctx, base);
	// 825B7CF0: 57AB07FF  clrlwi. r11, r29, 0x1f
	ctx.r[11].u64 = ctx.r[29].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825B7CF4: 4182000C  beq 0x825b7d00
	if ctx.cr[0].eq {
	pc = 0x825B7D00; continue 'dispatch;
	}
	// 825B7CF8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825B7CFC: 4883B72D  bl 0x82df3428
	ctx.lr = 0x825B7D00;
	sub_82DF3428(ctx, base);
	// 825B7D00: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 825B7D04: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 825B7D08: 4883BD01  bl 0x82df3a08
	ctx.lr = 0x825B7D0C;
	sub_82DF3A08(ctx, base);
	// 825B7D0C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B7D10: 815C0000  lwz r10, 0(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B7D14: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825B7D18: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 825B7D1C: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 825B7D20: 419A0024  beq cr6, 0x825b7d44
	if ctx.cr[6].eq {
	pc = 0x825B7D44; continue 'dispatch;
	}
	// 825B7D24: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 825B7D28: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 825B7D2C: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 825B7D30: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 825B7D34: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 825B7D38: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 825B7D3C: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 825B7D40: 4082FFE8  bne 0x825b7d28
	if !ctx.cr[0].eq {
	pc = 0x825B7D28; continue 'dispatch;
	}
	// 825B7D44: 38A10054  addi r5, r1, 0x54
	ctx.r[5].s64 = ctx.r[1].s64 + 84;
	// 825B7D48: 807C0000  lwz r3, 0(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B7D4C: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 825B7D50: 4BFFE549  bl 0x825b6298
	ctx.lr = 0x825B7D54;
	sub_825B6298(ctx, base);
	// 825B7D54: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 825B7D58: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825B7D5C: 419A0008  beq cr6, 0x825b7d64
	if ctx.cr[6].eq {
	pc = 0x825B7D64; continue 'dispatch;
	}
	// 825B7D60: 4BD08B31  bl 0x822c0890
	ctx.lr = 0x825B7D64;
	sub_822C0890(ctx, base);
	// 825B7D64: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 825B7D68: 4883B6C1  bl 0x82df3428
	ctx.lr = 0x825B7D6C;
	sub_82DF3428(ctx, base);
	// 825B7D6C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 825B7D70: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 825B7D74: 48BF0440  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B7D78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825B7D78 size=596
    let mut pc: u32 = 0x825B7D78;
    'dispatch: loop {
        match pc {
            0x825B7D78 => {
    //   block [0x825B7D78..0x825B7FCC)
	// 825B7D78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B7D7C: 48BF03DD  bl 0x831a8158
	ctx.lr = 0x825B7D80;
	sub_831A8130(ctx, base);
	// 825B7D80: 9421FEA0  stwu r1, -0x160(r1)
	ea = ctx.r[1].u32.wrapping_add(-352 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B7D84: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 825B7D88: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825B7D8C: 7F7DDB78  mr r29, r27
	ctx.r[29].u64 = ctx.r[27].u64;
	// 825B7D90: 7C992378  mr r25, r4
	ctx.r[25].u64 = ctx.r[4].u64;
	// 825B7D94: 93A10050  stw r29, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[29].u32 ) };
	// 825B7D98: 4BFFEB99  bl 0x825b6930
	ctx.lr = 0x825B7D9C;
	sub_825B6930(ctx, base);
	// 825B7D9C: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 825B7DA0: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825B7DA4: 3B4BC0D8  addi r26, r11, -0x3f28
	ctx.r[26].s64 = ctx.r[11].s64 + -16168;
	// 825B7DA8: 38A0004F  li r5, 0x4f
	ctx.r[5].s64 = 79;
	// 825B7DAC: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 825B7DB0: 386000F8  li r3, 0xf8
	ctx.r[3].s64 = 248;
	// 825B7DB4: 4883A635  bl 0x82df23e8
	ctx.lr = 0x825B7DB8;
	sub_82DF23E8(ctx, base);
	// 825B7DB8: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 825B7DBC: 41820054  beq 0x825b7e10
	if ctx.cr[0].eq {
	pc = 0x825B7E10; continue 'dispatch;
	}
	// 825B7DC0: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 825B7DC4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825B7DC8: 388BBCB4  addi r4, r11, -0x434c
	ctx.r[4].s64 = ctx.r[11].s64 + -17228;
	// 825B7DCC: 4883BC3D  bl 0x82df3a08
	ctx.lr = 0x825B7DD0;
	sub_82DF3A08(ctx, base);
	// 825B7DD0: 3D60825B  lis r11, -0x7da5
	ctx.r[11].s64 = -2107965440;
	// 825B7DD4: 386100A0  addi r3, r1, 0xa0
	ctx.r[3].s64 = ctx.r[1].s64 + 160;
	// 825B7DD8: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 825B7DDC: 396B72B8  addi r11, r11, 0x72b8
	ctx.r[11].s64 = ctx.r[11].s64 + 29368;
	// 825B7DE0: 936100A0  stw r27, 0xa0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(160 as u32), ctx.r[27].u32 ) };
	// 825B7DE4: 3BA00001  li r29, 1
	ctx.r[29].s64 = 1;
	// 825B7DE8: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 825B7DEC: E8810058  ld r4, 0x58(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 825B7DF0: 4BFFF9B9  bl 0x825b77a8
	ctx.lr = 0x825B7DF4;
	sub_825B77A8(ctx, base);
	// 825B7DF4: 38DF0128  addi r6, r31, 0x128
	ctx.r[6].s64 = ctx.r[31].s64 + 296;
	// 825B7DF8: 38A100A0  addi r5, r1, 0xa0
	ctx.r[5].s64 = ctx.r[1].s64 + 160;
	// 825B7DFC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 825B7E00: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825B7E04: 4800030D  bl 0x825b8110
	ctx.lr = 0x825B7E08;
	sub_825B8110(ctx, base);
	// 825B7E08: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 825B7E0C: 48000008  b 0x825b7e14
	pc = 0x825B7E14; continue 'dispatch;
	// 825B7E10: 7F7EDB78  mr r30, r27
	ctx.r[30].u64 = ctx.r[27].u64;
	// 825B7E14: 93C10060  stw r30, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[30].u32 ) };
	// 825B7E18: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 825B7E1C: 38610064  addi r3, r1, 0x64
	ctx.r[3].s64 = ctx.r[1].s64 + 100;
	// 825B7E20: 4BFFF5F1  bl 0x825b7410
	ctx.lr = 0x825B7E24;
	sub_825B7410(ctx, base);
	// 825B7E24: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 825B7E28: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 825B7E2C: 38610064  addi r3, r1, 0x64
	ctx.r[3].s64 = ctx.r[1].s64 + 100;
	// 825B7E30: 4BD081D1  bl 0x822c0000
	ctx.lr = 0x825B7E34;
	sub_822C0000(ctx, base);
	// 825B7E34: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 825B7E38: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B7E3C: 4BFFEBB5  bl 0x825b69f0
	ctx.lr = 0x825B7E40;
	sub_825B69F0(ctx, base);
	// 825B7E40: 80610064  lwz r3, 0x64(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 825B7E44: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825B7E48: 419A0008  beq cr6, 0x825b7e50
	if ctx.cr[6].eq {
	pc = 0x825B7E50; continue 'dispatch;
	}
	// 825B7E4C: 4BD08A45  bl 0x822c0890
	ctx.lr = 0x825B7E50;
	sub_822C0890(ctx, base);
	// 825B7E50: 57AB07FF  clrlwi. r11, r29, 0x1f
	ctx.r[11].u64 = ctx.r[29].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825B7E54: 41820010  beq 0x825b7e64
	if ctx.cr[0].eq {
	pc = 0x825B7E64; continue 'dispatch;
	}
	// 825B7E58: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825B7E5C: 57BD003C  rlwinm r29, r29, 0, 0, 0x1e
	ctx.r[29].u64 = ctx.r[29].u32 as u64 & 0xFFFFFFFFu64;
	// 825B7E60: 4883B5C9  bl 0x82df3428
	ctx.lr = 0x825B7E64;
	sub_82DF3428(ctx, base);
	// 825B7E64: 807F0120  lwz r3, 0x120(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(288 as u32) ) } as u64;
	// 825B7E68: 7F7CDB78  mr r28, r27
	ctx.r[28].u64 = ctx.r[27].u64;
	// 825B7E6C: 4BFF278D  bl 0x825aa5f8
	ctx.lr = 0x825B7E70;
	sub_825AA5F8(ctx, base);
	// 825B7E70: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 825B7E74: 41820150  beq 0x825b7fc4
	if ctx.cr[0].eq {
	pc = 0x825B7FC4; continue 'dispatch;
	}
	// 825B7E78: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 825B7E7C: 809F0120  lwz r4, 0x120(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(288 as u32) ) } as u64;
	// 825B7E80: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825B7E84: 4BFF279D  bl 0x825aa620
	ctx.lr = 0x825B7E88;
	sub_825AA620(ctx, base);
	// 825B7E88: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 825B7E8C: 80A10050  lwz r5, 0x50(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 825B7E90: 8099003C  lwz r4, 0x3c(r25)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(60 as u32) ) } as u64;
	// 825B7E94: 4BFFA205  bl 0x825b2098
	ctx.lr = 0x825B7E98;
	sub_825B2098(ctx, base);
	// 825B7E98: 81610070  lwz r11, 0x70(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	// 825B7E9C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825B7EA0: 419A00F4  beq cr6, 0x825b7f94
	if ctx.cr[6].eq {
	pc = 0x825B7F94; continue 'dispatch;
	}
	// 825B7EA4: 3D60825B  lis r11, -0x7da5
	ctx.r[11].s64 = -2107965440;
	// 825B7EA8: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 825B7EAC: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 825B7EB0: 93610080  stw r27, 0x80(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[27].u32 ) };
	// 825B7EB4: 396B7300  addi r11, r11, 0x7300
	ctx.r[11].s64 = ctx.r[11].s64 + 29440;
	// 825B7EB8: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 825B7EBC: E8810058  ld r4, 0x58(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 825B7EC0: 4BFFF8E9  bl 0x825b77a8
	ctx.lr = 0x825B7EC4;
	sub_825B77A8(ctx, base);
	// 825B7EC4: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 825B7EC8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825B7ECC: 38A00058  li r5, 0x58
	ctx.r[5].s64 = 88;
	// 825B7ED0: 386000F0  li r3, 0xf0
	ctx.r[3].s64 = 240;
	// 825B7ED4: 4883A515  bl 0x82df23e8
	ctx.lr = 0x825B7ED8;
	sub_82DF23E8(ctx, base);
	// 825B7ED8: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 825B7EDC: 41820058  beq 0x825b7f34
	if ctx.cr[0].eq {
	pc = 0x825B7F34; continue 'dispatch;
	}
	// 825B7EE0: 38810080  addi r4, r1, 0x80
	ctx.r[4].s64 = ctx.r[1].s64 + 128;
	// 825B7EE4: 83010050  lwz r24, 0x50(r1)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 825B7EE8: 386100A0  addi r3, r1, 0xa0
	ctx.r[3].s64 = ctx.r[1].s64 + 160;
	// 825B7EEC: 4BFD4025  bl 0x8258bf10
	ctx.lr = 0x825B7EF0;
	sub_8258BF10(ctx, base);
	// 825B7EF0: 3D60825B  lis r11, -0x7da5
	ctx.r[11].s64 = -2107965440;
	// 825B7EF4: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 825B7EF8: 388B5D20  addi r4, r11, 0x5d20
	ctx.r[4].s64 = ctx.r[11].s64 + 23840;
	// 825B7EFC: 386100E0  addi r3, r1, 0xe0
	ctx.r[3].s64 = ctx.r[1].s64 + 224;
	// 825B7F00: 7F06C378  mr r6, r24
	ctx.r[6].u64 = ctx.r[24].u64;
	// 825B7F04: 4BFFF82D  bl 0x825b7730
	ctx.lr = 0x825B7F08;
	sub_825B7730(ctx, base);
	// 825B7F08: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 825B7F0C: 386100C0  addi r3, r1, 0xc0
	ctx.r[3].s64 = ctx.r[1].s64 + 192;
	// 825B7F10: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825B7F14: 4BFFFC85  bl 0x825b7b98
	ctx.lr = 0x825B7F18;
	sub_825B7B98(ctx, base);
	// 825B7F18: 38A100C0  addi r5, r1, 0xc0
	ctx.r[5].s64 = ctx.r[1].s64 + 192;
	// 825B7F1C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825B7F20: 80810070  lwz r4, 0x70(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	// 825B7F24: 63BD0002  ori r29, r29, 2
	ctx.r[29].u64 = ctx.r[29].u64 | 2;
	// 825B7F28: 48000B89  bl 0x825b8ab0
	ctx.lr = 0x825B7F2C;
	sub_825B8AB0(ctx, base);
	// 825B7F2C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 825B7F30: 48000008  b 0x825b7f38
	pc = 0x825B7F38; continue 'dispatch;
	// 825B7F34: 7F7EDB78  mr r30, r27
	ctx.r[30].u64 = ctx.r[27].u64;
	// 825B7F38: 93C10068  stw r30, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[30].u32 ) };
	// 825B7F3C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 825B7F40: 3861006C  addi r3, r1, 0x6c
	ctx.r[3].s64 = ctx.r[1].s64 + 108;
	// 825B7F44: 4BFFBF0D  bl 0x825b3e50
	ctx.lr = 0x825B7F48;
	sub_825B3E50(ctx, base);
	// 825B7F48: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 825B7F4C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 825B7F50: 3861006C  addi r3, r1, 0x6c
	ctx.r[3].s64 = ctx.r[1].s64 + 108;
	// 825B7F54: 4BD080AD  bl 0x822c0000
	ctx.lr = 0x825B7F58;
	sub_822C0000(ctx, base);
	// 825B7F58: 38810068  addi r4, r1, 0x68
	ctx.r[4].s64 = ctx.r[1].s64 + 104;
	// 825B7F5C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B7F60: 4BFFEA91  bl 0x825b69f0
	ctx.lr = 0x825B7F64;
	sub_825B69F0(ctx, base);
	// 825B7F64: 8061006C  lwz r3, 0x6c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(108 as u32) ) } as u64;
	// 825B7F68: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825B7F6C: 419A0008  beq cr6, 0x825b7f74
	if ctx.cr[6].eq {
	pc = 0x825B7F74; continue 'dispatch;
	}
	// 825B7F70: 4BD08921  bl 0x822c0890
	ctx.lr = 0x825B7F74;
	sub_822C0890(ctx, base);
	// 825B7F74: 57AB07BD  rlwinm. r11, r29, 0, 0x1e, 0x1e
	ctx.r[11].u64 = ctx.r[29].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825B7F78: 41820010  beq 0x825b7f88
	if ctx.cr[0].eq {
	pc = 0x825B7F88; continue 'dispatch;
	}
	// 825B7F7C: 386100C0  addi r3, r1, 0xc0
	ctx.r[3].s64 = ctx.r[1].s64 + 192;
	// 825B7F80: 57BD07FA  rlwinm r29, r29, 0, 0x1f, 0x1d
	ctx.r[29].u64 = ctx.r[29].u32 as u64 & 0xFFFFFFFFu64;
	// 825B7F84: 4BD10D35  bl 0x822c8cb8
	ctx.lr = 0x825B7F88;
	sub_822C8CB8(ctx, base);
	// 825B7F88: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 825B7F8C: 4BD10D2D  bl 0x822c8cb8
	ctx.lr = 0x825B7F90;
	sub_822C8CB8(ctx, base);
	// 825B7F90: 48000010  b 0x825b7fa0
	pc = 0x825B7FA0; continue 'dispatch;
	// 825B7F94: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 825B7F98: 807F0120  lwz r3, 0x120(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(288 as u32) ) } as u64;
	// 825B7F9C: 4BFF2855  bl 0x825aa7f0
	ctx.lr = 0x825B7FA0;
	sub_825AA7F0(ctx, base);
	// 825B7FA0: 80610074  lwz r3, 0x74(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 825B7FA4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825B7FA8: 419A0008  beq cr6, 0x825b7fb0
	if ctx.cr[6].eq {
	pc = 0x825B7FB0; continue 'dispatch;
	}
	// 825B7FAC: 4BD088E5  bl 0x822c0890
	ctx.lr = 0x825B7FB0;
	sub_822C0890(ctx, base);
	// 825B7FB0: 807F0120  lwz r3, 0x120(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(288 as u32) ) } as u64;
	// 825B7FB4: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 825B7FB8: 4BFF2641  bl 0x825aa5f8
	ctx.lr = 0x825B7FBC;
	sub_825AA5F8(ctx, base);
	// 825B7FBC: 7F1C1840  cmplw cr6, r28, r3
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[3].u32, &mut ctx.xer);
	// 825B7FC0: 4198FEB8  blt cr6, 0x825b7e78
	if ctx.cr[6].lt {
	pc = 0x825B7E78; continue 'dispatch;
	}
	// 825B7FC4: 38210160  addi r1, r1, 0x160
	ctx.r[1].s64 = ctx.r[1].s64 + 352;
	// 825B7FC8: 48BF01E0  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B7FD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x825B7FD0 size=228
    let mut pc: u32 = 0x825B7FD0;
    'dispatch: loop {
        match pc {
            0x825B7FD0 => {
    //   block [0x825B7FD0..0x825B80B4)
	// 825B7FD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B7FD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825B7FD8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825B7FDC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825B7FE0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B7FE4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825B7FE8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825B7FEC: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 825B7FF0: 809E003C  lwz r4, 0x3c(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(60 as u32) ) } as u64;
	// 825B7FF4: 4BFF9E0D  bl 0x825b1e00
	ctx.lr = 0x825B7FF8;
	sub_825B1E00(ctx, base);
	// 825B7FF8: 39600130  li r11, 0x130
	ctx.r[11].s64 = 304;
	// 825B7FFC: 13E018C7  vcmpequd (lvx128) v31, v0, v3
	tmp.u32 = ctx.r[3].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 825B8000: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 825B8004: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825B8008: 388AC0D8  addi r4, r10, -0x3f28
	ctx.r[4].s64 = ctx.r[10].s64 + -16168;
	// 825B800C: 38A0002C  li r5, 0x2c
	ctx.r[5].s64 = 44;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B80B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825B80B8 size=88
    let mut pc: u32 = 0x825B80B8;
    'dispatch: loop {
        match pc {
            0x825B80B8 => {
    //   block [0x825B80B8..0x825B8110)
	// 825B80B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B80BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825B80C0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825B80C4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825B80C8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B80CC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825B80D0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825B80D4: 4BFFCB25  bl 0x825b4bf8
	ctx.lr = 0x825B80D8;
	sub_825B4BF8(ctx, base);
	// 825B80D8: 897F0124  lbz r11, 0x124(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(292 as u32) ) } as u64;
	// 825B80DC: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 825B80E0: 41820018  beq 0x825b80f8
	if ctx.cr[0].eq {
	pc = 0x825B80F8; continue 'dispatch;
	}
	// 825B80E4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825B80E8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 825B80EC: 997F0124  stb r11, 0x124(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(292 as u32), ctx.r[11].u8 ) };
	// 825B80F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B80F4: 4BFFFC85  bl 0x825b7d78
	ctx.lr = 0x825B80F8;
	sub_825B7D78(ctx, base);
	// 825B80F8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825B80FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825B8100: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825B8104: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825B8108: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825B810C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B8110(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825B8110 size=144
    let mut pc: u32 = 0x825B8110;
    'dispatch: loop {
        match pc {
            0x825B8110 => {
    //   block [0x825B8110..0x825B81A0)
	// 825B8110: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B8114: 48BF0059  bl 0x831a816c
	ctx.lr = 0x825B8118;
	sub_831A8130(ctx, base);
	// 825B8118: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B811C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825B8120: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 825B8124: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 825B8128: 48000191  bl 0x825b82b8
	ctx.lr = 0x825B812C;
	sub_825B82B8(ctx, base);
	// 825B812C: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 825B8130: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 825B8134: 396BC13C  addi r11, r11, -0x3ec4
	ctx.r[11].s64 = ctx.r[11].s64 + -16068;
	// 825B8138: 394AC128  addi r10, r10, -0x3ed8
	ctx.r[10].s64 = ctx.r[10].s64 + -16088;
	// 825B813C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825B8140: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 825B8144: 915F0028  stw r10, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[10].u32 ) };
	// 825B8148: 387F00D0  addi r3, r31, 0xd0
	ctx.r[3].s64 = ctx.r[31].s64 + 208;
	// 825B814C: 4BFD3DC5  bl 0x8258bf10
	ctx.lr = 0x825B8150;
	sub_8258BF10(ctx, base);
	// 825B8150: 397F00F0  addi r11, r31, 0xf0
	ctx.r[11].s64 = ctx.r[31].s64 + 240;
	// 825B8154: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B8158: 917F00F0  stw r11, 0xf0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(240 as u32), ctx.r[11].u32 ) };
	// 825B815C: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 825B8160: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825B8164: 917F00F4  stw r11, 0xf4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(244 as u32), ctx.r[11].u32 ) };
	// 825B8168: 419A0024  beq cr6, 0x825b818c
	if ctx.cr[6].eq {
	pc = 0x825B818C; continue 'dispatch;
	}
	// 825B816C: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 825B8170: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 825B8174: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 825B8178: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 825B817C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 825B8180: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 825B8184: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 825B8188: 4082FFE8  bne 0x825b8170
	if !ctx.cr[0].eq {
	pc = 0x825B8170; continue 'dispatch;
	}
	// 825B818C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825B8190: 4BD10B29  bl 0x822c8cb8
	ctx.lr = 0x825B8194;
	sub_822C8CB8(ctx, base);
	// 825B8194: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B8198: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825B819C: 48BF0020  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B81A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825B81A0 size=8
    let mut pc: u32 = 0x825B81A0;
    'dispatch: loop {
        match pc {
            0x825B81A0 => {
    //   block [0x825B81A0..0x825B81A8)
	// 825B81A0: 3863FFD8  addi r3, r3, -0x28
	ctx.r[3].s64 = ctx.r[3].s64 + -40;
	// 825B81A4: 48000054  b 0x825b81f8
	sub_825B81F8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B81A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825B81A8 size=80
    let mut pc: u32 = 0x825B81A8;
    'dispatch: loop {
        match pc {
            0x825B81A8 => {
    //   block [0x825B81A8..0x825B81F8)
	// 825B81A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B81AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825B81B0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825B81B4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B81B8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825B81BC: 807F00F4  lwz r3, 0xf4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(244 as u32) ) } as u64;
	// 825B81C0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825B81C4: 419A0008  beq cr6, 0x825b81cc
	if ctx.cr[6].eq {
	pc = 0x825B81CC; continue 'dispatch;
	}
	// 825B81C8: 4BD086C9  bl 0x822c0890
	ctx.lr = 0x825B81CC;
	sub_822C0890(ctx, base);
	// 825B81CC: 387F00D0  addi r3, r31, 0xd0
	ctx.r[3].s64 = ctx.r[31].s64 + 208;
	// 825B81D0: 4BD10AE9  bl 0x822c8cb8
	ctx.lr = 0x825B81D4;
	sub_822C8CB8(ctx, base);
	// 825B81D4: 387F00C0  addi r3, r31, 0xc0
	ctx.r[3].s64 = ctx.r[31].s64 + 192;
	// 825B81D8: 4883B251  bl 0x82df3428
	ctx.lr = 0x825B81DC;
	sub_82DF3428(ctx, base);
	// 825B81DC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B81E0: 4BF58FB9  bl 0x82511198
	ctx.lr = 0x825B81E4;
	sub_82511198(ctx, base);
	// 825B81E4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825B81E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825B81EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825B81F0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825B81F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B81F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825B81F8 size=76
    let mut pc: u32 = 0x825B81F8;
    'dispatch: loop {
        match pc {
            0x825B81F8 => {
    //   block [0x825B81F8..0x825B8244)
	// 825B81F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B81FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825B8200: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825B8204: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825B8208: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B820C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825B8210: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825B8214: 4BFFFF95  bl 0x825b81a8
	ctx.lr = 0x825B8218;
	sub_825B81A8(ctx, base);
	// 825B8218: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825B821C: 4182000C  beq 0x825b8228
	if ctx.cr[0].eq {
	pc = 0x825B8228; continue 'dispatch;
	}
	// 825B8220: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B8224: 4883A1B5  bl 0x82df23d8
	ctx.lr = 0x825B8228;
	sub_82DF23D8(ctx, base);
	// 825B8228: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B822C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825B8230: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825B8234: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825B8238: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825B823C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825B8240: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B8248(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825B8248 size=112
    let mut pc: u32 = 0x825B8248;
    'dispatch: loop {
        match pc {
            0x825B8248 => {
    //   block [0x825B8248..0x825B82B8)
	// 825B8248: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B824C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825B8250: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825B8254: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825B8258: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B825C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 825B8260: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 825B8264: 807E00F0  lwz r3, 0xf0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(240 as u32) ) } as u64;
	// 825B8268: 48000EC1  bl 0x825b9128
	ctx.lr = 0x825B826C;
	sub_825B9128(ctx, base);
	// 825B826C: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 825B8270: 556B07FF  clrlwi. r11, r11, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825B8274: 4182002C  beq 0x825b82a0
	if ctx.cr[0].eq {
	pc = 0x825B82A0; continue 'dispatch;
	}
	// 825B8278: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825B827C: 809F003C  lwz r4, 0x3c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) } as u64;
	// 825B8280: 4BFF9AE9  bl 0x825b1d68
	ctx.lr = 0x825B8284;
	sub_825B1D68(ctx, base);
	// 825B8284: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 825B8288: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 825B828C: 816B853C  lwz r11, -0x7ac4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-31428 as u32) ) } as u64;
	// 825B8290: 7F045840  cmplw cr6, r4, r11
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[11].u32, &mut ctx.xer);
	// 825B8294: 419A000C  beq cr6, 0x825b82a0
	if ctx.cr[6].eq {
	pc = 0x825B82A0; continue 'dispatch;
	}
	// 825B8298: 387E00D0  addi r3, r30, 0xd0
	ctx.r[3].s64 = ctx.r[30].s64 + 208;
	// 825B829C: 4BFF962D  bl 0x825b18c8
	ctx.lr = 0x825B82A0;
	sub_825B18C8(ctx, base);
	// 825B82A0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825B82A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825B82A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825B82AC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825B82B0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825B82B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B82B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825B82B8 size=116
    let mut pc: u32 = 0x825B82B8;
    'dispatch: loop {
        match pc {
            0x825B82B8 => {
    //   block [0x825B82B8..0x825B832C)
	// 825B82B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B82BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825B82C0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825B82C4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825B82C8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B82CC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825B82D0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825B82D4: 4BF58E1D  bl 0x825110f0
	ctx.lr = 0x825B82D8;
	sub_825110F0(ctx, base);
	// 825B82D8: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 825B82DC: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 825B82E0: 396BC1A4  addi r11, r11, -0x3e5c
	ctx.r[11].s64 = ctx.r[11].s64 + -15964;
	// 825B82E4: 394AC18C  addi r10, r10, -0x3e74
	ctx.r[10].s64 = ctx.r[10].s64 + -15988;
	// 825B82E8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825B82EC: 387F00C0  addi r3, r31, 0xc0
	ctx.r[3].s64 = ctx.r[31].s64 + 192;
	// 825B82F0: 915F0028  stw r10, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[10].u32 ) };
	// 825B82F4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 825B82F8: 4883B909  bl 0x82df3c00
	ctx.lr = 0x825B82FC;
	sub_82DF3C00(ctx, base);
	// 825B82FC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825B8300: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B8304: 997F00C4  stb r11, 0xc4(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(196 as u32), ctx.r[11].u8 ) };
	// 825B8308: 997F00C5  stb r11, 0xc5(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(197 as u32), ctx.r[11].u8 ) };
	// 825B830C: 997F00CC  stb r11, 0xcc(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(204 as u32), ctx.r[11].u8 ) };
	// 825B8310: 997F00CD  stb r11, 0xcd(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(205 as u32), ctx.r[11].u8 ) };
	// 825B8314: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825B8318: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825B831C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825B8320: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825B8324: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825B8328: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B8330(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825B8330 size=8
    let mut pc: u32 = 0x825B8330;
    'dispatch: loop {
        match pc {
            0x825B8330 => {
    //   block [0x825B8330..0x825B8338)
	// 825B8330: 988300C4  stb r4, 0xc4(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(196 as u32), ctx.r[4].u8 ) };
	// 825B8334: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B8338(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825B8338 size=32
    let mut pc: u32 = 0x825B8338;
    'dispatch: loop {
        match pc {
            0x825B8338 => {
    //   block [0x825B8338..0x825B8358)
	// 825B8338: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 825B833C: 894B00CC  lbz r10, 0xcc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(204 as u32) ) } as u64;
	// 825B8340: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 825B8344: 41820014  beq 0x825b8358
	if ctx.cr[0].eq {
		sub_825B8358(ctx, base);
		return;
	}
	// 825B8348: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825B834C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 825B8350: 994B00CC  stb r10, 0xcc(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(204 as u32), ctx.r[10].u8 ) };
	// 825B8354: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B8358(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825B8358 size=8
    let mut pc: u32 = 0x825B8358;
    'dispatch: loop {
        match pc {
            0x825B8358 => {
    //   block [0x825B8358..0x825B8360)
	// 825B8358: 886B00C4  lbz r3, 0xc4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(196 as u32) ) } as u64;
	// 825B835C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B8360(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825B8360 size=8
    let mut pc: u32 = 0x825B8360;
    'dispatch: loop {
        match pc {
            0x825B8360 => {
    //   block [0x825B8360..0x825B8368)
	// 825B8360: 3863FFD8  addi r3, r3, -0x28
	ctx.r[3].s64 = ctx.r[3].s64 + -40;
	// 825B8364: 48000004  b 0x825b8368
	sub_825B8368(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B8368(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825B8368 size=88
    let mut pc: u32 = 0x825B8368;
    'dispatch: loop {
        match pc {
            0x825B8368 => {
    //   block [0x825B8368..0x825B83C0)
	// 825B8368: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B836C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825B8370: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825B8374: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825B8378: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B837C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825B8380: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825B8384: 387F00C0  addi r3, r31, 0xc0
	ctx.r[3].s64 = ctx.r[31].s64 + 192;
	// 825B8388: 4883B0A1  bl 0x82df3428
	ctx.lr = 0x825B838C;
	sub_82DF3428(ctx, base);
	// 825B838C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B8390: 4BF58E09  bl 0x82511198
	ctx.lr = 0x825B8394;
	sub_82511198(ctx, base);
	// 825B8394: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825B8398: 4182000C  beq 0x825b83a4
	if ctx.cr[0].eq {
	pc = 0x825B83A4; continue 'dispatch;
	}
	// 825B839C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B83A0: 4883A039  bl 0x82df23d8
	ctx.lr = 0x825B83A4;
	sub_82DF23D8(ctx, base);
	// 825B83A4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B83A8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825B83AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825B83B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825B83B4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825B83B8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825B83BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B83C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825B83C0 size=12
    let mut pc: u32 = 0x825B83C0;
    'dispatch: loop {
        match pc {
            0x825B83C0 => {
    //   block [0x825B83C0..0x825B83CC)
	// 825B83C0: 808300D8  lwz r4, 0xd8(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(216 as u32) ) } as u64;
	// 825B83C4: 806300D0  lwz r3, 0xd0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(208 as u32) ) } as u64;
	// 825B83C8: 4BFE8700  b 0x825a0ac8
	sub_825A0AC8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B83D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825B83D0 size=56
    let mut pc: u32 = 0x825B83D0;
    'dispatch: loop {
        match pc {
            0x825B83D0 => {
    //   block [0x825B83D0..0x825B8408)
	// 825B83D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B83D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825B83D8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825B83DC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B83E0: 80A400D8  lwz r5, 0xd8(r4)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(216 as u32) ) } as u64;
	// 825B83E4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825B83E8: 808400D0  lwz r4, 0xd0(r4)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(208 as u32) ) } as u64;
	// 825B83EC: 4BFE888D  bl 0x825a0c78
	ctx.lr = 0x825B83F0;
	sub_825A0C78(ctx, base);
	// 825B83F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B83F4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825B83F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825B83FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825B8400: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825B8404: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B8408(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825B8408 size=16
    let mut pc: u32 = 0x825B8408;
    'dispatch: loop {
        match pc {
            0x825B8408 => {
    //   block [0x825B8408..0x825B8418)
	// 825B8408: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 825B840C: 806B00D0  lwz r3, 0xd0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(208 as u32) ) } as u64;
	// 825B8410: 808B00D8  lwz r4, 0xd8(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(216 as u32) ) } as u64;
	// 825B8414: 4BFE9064  b 0x825a1478
	sub_825A1478(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B8418(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825B8418 size=96
    let mut pc: u32 = 0x825B8418;
    'dispatch: loop {
        match pc {
            0x825B8418 => {
    //   block [0x825B8418..0x825B8478)
	// 825B8418: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B841C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825B8420: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825B8424: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B8428: 8084003C  lwz r4, 0x3c(r4)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(60 as u32) ) } as u64;
	// 825B842C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825B8430: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 825B8434: 419A0030  beq cr6, 0x825b8464
	if ctx.cr[6].eq {
	pc = 0x825B8464; continue 'dispatch;
	}
	// 825B8438: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825B843C: 4BFF9DCD  bl 0x825b2208
	ctx.lr = 0x825B8440;
	sub_825B2208(ctx, base);
	// 825B8440: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 825B8444: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825B8448: 419A000C  beq cr6, 0x825b8454
	if ctx.cr[6].eq {
	pc = 0x825B8454; continue 'dispatch;
	}
	// 825B844C: 809F00D8  lwz r4, 0xd8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(216 as u32) ) } as u64;
	// 825B8450: 481E16E9  bl 0x82799b38
	ctx.lr = 0x825B8454;
	sub_82799B38(ctx, base);
	// 825B8454: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 825B8458: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825B845C: 419A0008  beq cr6, 0x825b8464
	if ctx.cr[6].eq {
	pc = 0x825B8464; continue 'dispatch;
	}
	// 825B8460: 4BD08431  bl 0x822c0890
	ctx.lr = 0x825B8464;
	sub_822C0890(ctx, base);
	// 825B8464: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825B8468: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825B846C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825B8470: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825B8474: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B8478(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825B8478 size=188
    let mut pc: u32 = 0x825B8478;
    'dispatch: loop {
        match pc {
            0x825B8478 => {
    //   block [0x825B8478..0x825B8534)
	// 825B8478: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B847C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825B8480: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825B8484: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825B8488: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B848C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 825B8490: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825B8494: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 825B8498: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 825B849C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825B84A0: 4BD08499  bl 0x822c0938
	ctx.lr = 0x825B84A4;
	sub_822C0938(ctx, base);
	// 825B84A4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 825B84A8: 41820028  beq 0x825b84d0
	if ctx.cr[0].eq {
	pc = 0x825B84D0; continue 'dispatch;
	}
	// 825B84AC: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 825B84B0: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 825B84B4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825B84B8: 392BC1F4  addi r9, r11, -0x3e0c
	ctx.r[9].s64 = ctx.r[11].s64 + -15884;
	// 825B84BC: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 825B84C0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 825B84C4: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 825B84C8: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 825B84CC: 48000008  b 0x825b84d4
	pc = 0x825B84D4; continue 'dispatch;
	// 825B84D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825B84D4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825B84D8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825B84DC: 409A003C  bne cr6, 0x825b8518
	if !ctx.cr[6].eq {
	pc = 0x825B8518; continue 'dispatch;
	}
	// 825B84E0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 825B84E4: 419A0014  beq cr6, 0x825b84f8
	if ctx.cr[6].eq {
	pc = 0x825B84F8; continue 'dispatch;
	}
	// 825B84E8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B84EC: 48014FED  bl 0x825cd4d8
	ctx.lr = 0x825B84F0;
	sub_825CD4D8(ctx, base);
	// 825B84F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B84F4: 4BD07D75  bl 0x822c0268
	ctx.lr = 0x825B84F8;
	sub_822C0268(ctx, base);
	// 825B84F8: 3D608329  lis r11, -0x7cd7
	ctx.r[11].s64 = -2094465024;
	// 825B84FC: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 825B8500: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825B8504: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 825B8508: 816B9514  lwz r11, -0x6aec(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27372 as u32) ) } as u64;
	// 825B850C: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 825B8510: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825B8514: 4BD07AED  bl 0x822c0000
	ctx.lr = 0x825B8518;
	sub_822C0000(ctx, base);
	// 825B8518: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825B851C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825B8520: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825B8524: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825B8528: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825B852C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825B8530: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B8538(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825B8538 size=64
    let mut pc: u32 = 0x825B8538;
    'dispatch: loop {
        match pc {
            0x825B8538 => {
    //   block [0x825B8538..0x825B8578)
	// 825B8538: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B853C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825B8540: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825B8544: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B8548: 83E3000C  lwz r31, 0xc(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 825B854C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 825B8550: 419A0014  beq cr6, 0x825b8564
	if ctx.cr[6].eq {
	pc = 0x825B8564; continue 'dispatch;
	}
	// 825B8554: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B8558: 48014F81  bl 0x825cd4d8
	ctx.lr = 0x825B855C;
	sub_825CD4D8(ctx, base);
	// 825B855C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B8560: 4BD07D09  bl 0x822c0268
	ctx.lr = 0x825B8564;
	sub_822C0268(ctx, base);
	// 825B8564: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825B8568: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825B856C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825B8570: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825B8574: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B8578(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825B8578 size=164
    let mut pc: u32 = 0x825B8578;
    'dispatch: loop {
        match pc {
            0x825B8578 => {
    //   block [0x825B8578..0x825B861C)
	// 825B8578: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B857C: 48BEFBED  bl 0x831a8168
	ctx.lr = 0x825B8580;
	sub_831A8130(ctx, base);
	// 825B8580: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B8584: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825B8588: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 825B858C: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 825B8590: 4BFFFD29  bl 0x825b82b8
	ctx.lr = 0x825B8594;
	sub_825B82B8(ctx, base);
	// 825B8594: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 825B8598: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 825B859C: 396BC21C  addi r11, r11, -0x3de4
	ctx.r[11].s64 = ctx.r[11].s64 + -15844;
	// 825B85A0: 394AC208  addi r10, r10, -0x3df8
	ctx.r[10].s64 = ctx.r[10].s64 + -15864;
	// 825B85A4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825B85A8: 397F00D0  addi r11, r31, 0xd0
	ctx.r[11].s64 = ctx.r[31].s64 + 208;
	// 825B85AC: 915F0028  stw r10, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[10].u32 ) };
	// 825B85B0: 3BDD0004  addi r30, r29, 4
	ctx.r[30].s64 = ctx.r[29].s64 + 4;
	// 825B85B4: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B85B8: 917F00D0  stw r11, 0xd0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(208 as u32), ctx.r[11].u32 ) };
	// 825B85BC: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 825B85C0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825B85C4: 917F00D4  stw r11, 0xd4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(212 as u32), ctx.r[11].u32 ) };
	// 825B85C8: 419A0024  beq cr6, 0x825b85ec
	if ctx.cr[6].eq {
	pc = 0x825B85EC; continue 'dispatch;
	}
	// 825B85CC: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 825B85D0: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 825B85D4: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 825B85D8: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 825B85DC: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 825B85E0: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 825B85E4: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 825B85E8: 4082FFE8  bne 0x825b85d0
	if !ctx.cr[0].eq {
	pc = 0x825B85D0; continue 'dispatch;
	}
	// 825B85EC: 939F00D8  stw r28, 0xd8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(216 as u32), ctx.r[28].u32 ) };
	// 825B85F0: 387F00DC  addi r3, r31, 0xdc
	ctx.r[3].s64 = ctx.r[31].s64 + 220;
	// 825B85F4: 4883AAFD  bl 0x82df30f0
	ctx.lr = 0x825B85F8;
	sub_82DF30F0(ctx, base);
	// 825B85F8: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 825B85FC: 917F00E0  stw r11, 0xe0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(224 as u32), ctx.r[11].u32 ) };
	// 825B8600: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B8604: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825B8608: 419A0008  beq cr6, 0x825b8610
	if ctx.cr[6].eq {
	pc = 0x825B8610; continue 'dispatch;
	}
	// 825B860C: 4BD08285  bl 0x822c0890
	ctx.lr = 0x825B8610;
	sub_822C0890(ctx, base);
	// 825B8610: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B8614: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825B8618: 48BEFBA0  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B8620(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825B8620 size=8
    let mut pc: u32 = 0x825B8620;
    'dispatch: loop {
        match pc {
            0x825B8620 => {
    //   block [0x825B8620..0x825B8628)
	// 825B8620: 3863FFD8  addi r3, r3, -0x28
	ctx.r[3].s64 = ctx.r[3].s64 + -40;
	// 825B8624: 48000054  b 0x825b8678
	sub_825B8678(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B8628(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825B8628 size=80
    let mut pc: u32 = 0x825B8628;
    'dispatch: loop {
        match pc {
            0x825B8628 => {
    //   block [0x825B8628..0x825B8678)
	// 825B8628: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B862C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825B8630: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825B8634: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B8638: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825B863C: 387F00DC  addi r3, r31, 0xdc
	ctx.r[3].s64 = ctx.r[31].s64 + 220;
	// 825B8640: 4883ADE9  bl 0x82df3428
	ctx.lr = 0x825B8644;
	sub_82DF3428(ctx, base);
	// 825B8644: 807F00D4  lwz r3, 0xd4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(212 as u32) ) } as u64;
	// 825B8648: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825B864C: 419A0008  beq cr6, 0x825b8654
	if ctx.cr[6].eq {
	pc = 0x825B8654; continue 'dispatch;
	}
	// 825B8650: 4BD08241  bl 0x822c0890
	ctx.lr = 0x825B8654;
	sub_822C0890(ctx, base);
	// 825B8654: 387F00C0  addi r3, r31, 0xc0
	ctx.r[3].s64 = ctx.r[31].s64 + 192;
	// 825B8658: 4883ADD1  bl 0x82df3428
	ctx.lr = 0x825B865C;
	sub_82DF3428(ctx, base);
	// 825B865C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B8660: 4BF58B39  bl 0x82511198
	ctx.lr = 0x825B8664;
	sub_82511198(ctx, base);
	// 825B8664: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825B8668: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825B866C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825B8670: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825B8674: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B8678(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825B8678 size=76
    let mut pc: u32 = 0x825B8678;
    'dispatch: loop {
        match pc {
            0x825B8678 => {
    //   block [0x825B8678..0x825B86C4)
	// 825B8678: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B867C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825B8680: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825B8684: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825B8688: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B868C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825B8690: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825B8694: 4BFFFF95  bl 0x825b8628
	ctx.lr = 0x825B8698;
	sub_825B8628(ctx, base);
	// 825B8698: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825B869C: 4182000C  beq 0x825b86a8
	if ctx.cr[0].eq {
	pc = 0x825B86A8; continue 'dispatch;
	}
	// 825B86A0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B86A4: 48839D35  bl 0x82df23d8
	ctx.lr = 0x825B86A8;
	sub_82DF23D8(ctx, base);
	// 825B86A8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B86AC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825B86B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825B86B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825B86B8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825B86BC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825B86C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B86C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x825B86C8 size=448
    let mut pc: u32 = 0x825B86C8;
    'dispatch: loop {
        match pc {
            0x825B86C8 => {
    //   block [0x825B86C8..0x825B8888)
	// 825B86C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B86CC: 48BEFAA1  bl 0x831a816c
	ctx.lr = 0x825B86D0;
	sub_831A8130(ctx, base);
	// 825B86D0: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B86D4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825B86D8: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 825B86DC: 817E003C  lwz r11, 0x3c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(60 as u32) ) } as u64;
	// 825B86E0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825B86E4: 419A00EC  beq cr6, 0x825b87d0
	if ctx.cr[6].eq {
	pc = 0x825B87D0; continue 'dispatch;
	}
	// 825B86E8: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 825B86EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825B86F0: 388BC268  addi r4, r11, -0x3d98
	ctx.r[4].s64 = ctx.r[11].s64 + -15768;
	// 825B86F4: 38A0002F  li r5, 0x2f
	ctx.r[5].s64 = 47;
	// 825B86F8: 38600020  li r3, 0x20
	ctx.r[3].s64 = 32;
	// 825B86FC: 4BD07CDD  bl 0x822c03d8
	ctx.lr = 0x825B8700;
	sub_822C03D8(ctx, base);
	// 825B8700: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 825B8704: 41820028  beq 0x825b872c
	if ctx.cr[0].eq {
	pc = 0x825B872C; continue 'dispatch;
	}
	// 825B8708: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 825B870C: 80BD00C8  lwz r5, 0xc8(r29)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(200 as u32) ) } as u64;
	// 825B8710: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825B8714: C03E0000  lfs f1, 0(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 825B8718: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 825B871C: C04B08A8  lfs f2, 0x8a8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2216 as u32) ) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 825B8720: 480012B1  bl 0x825b99d0
	ctx.lr = 0x825B8724;
	sub_825B99D0(ctx, base);
	// 825B8724: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825B8728: 48000008  b 0x825b8730
	pc = 0x825B8730; continue 'dispatch;
	// 825B872C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 825B8730: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 825B8734: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825B8738: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 825B873C: 4BFFC825  bl 0x825b4f60
	ctx.lr = 0x825B8740;
	sub_825B4F60(ctx, base);
	// 825B8740: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 825B8744: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825B8748: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 825B874C: 4BD078B5  bl 0x822c0000
	ctx.lr = 0x825B8750;
	sub_822C0000(ctx, base);
	// 825B8750: 80A10050  lwz r5, 0x50(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 825B8754: 807D00D0  lwz r3, 0xd0(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(208 as u32) ) } as u64;
	// 825B8758: 809D00D8  lwz r4, 0xd8(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(216 as u32) ) } as u64;
	// 825B875C: 4BFE832D  bl 0x825a0a88
	ctx.lr = 0x825B8760;
	sub_825A0A88(ctx, base);
	// 825B8760: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825B8764: 809E003C  lwz r4, 0x3c(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(60 as u32) ) } as u64;
	// 825B8768: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 825B876C: 4BFF9A9D  bl 0x825b2208
	ctx.lr = 0x825B8770;
	sub_825B2208(ctx, base);
	// 825B8770: 80610058  lwz r3, 0x58(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 825B8774: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825B8778: 419A0038  beq cr6, 0x825b87b0
	if ctx.cr[6].eq {
	pc = 0x825B87B0; continue 'dispatch;
	}
	// 825B877C: 809D00D8  lwz r4, 0xd8(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(216 as u32) ) } as u64;
	// 825B8780: 481E1359  bl 0x82799ad8
	ctx.lr = 0x825B8784;
	sub_82799AD8(ctx, base);
	// 825B8784: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 825B8788: 809E003C  lwz r4, 0x3c(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(60 as u32) ) } as u64;
	// 825B878C: 4BF56DDD  bl 0x8250f568
	ctx.lr = 0x825B8790;
	sub_8250F568(ctx, base);
	// 825B8790: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B8794: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825B8798: 386BFF40  addi r3, r11, -0xc0
	ctx.r[3].s64 = ctx.r[11].s64 + -192;
	// 825B879C: 409A0008  bne cr6, 0x825b87a4
	if !ctx.cr[6].eq {
	pc = 0x825B87A4; continue 'dispatch;
	}
	// 825B87A0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 825B87A4: 481F3575  bl 0x827abd18
	ctx.lr = 0x825B87A8;
	sub_827ABD18(ctx, base);
	// 825B87A8: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 825B87AC: 488394E5  bl 0x82df1c90
	ctx.lr = 0x825B87B0;
	sub_82DF1C90(ctx, base);
	// 825B87B0: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 825B87B4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825B87B8: 419A0008  beq cr6, 0x825b87c0
	if ctx.cr[6].eq {
	pc = 0x825B87C0; continue 'dispatch;
	}
	// 825B87BC: 4BD080D5  bl 0x822c0890
	ctx.lr = 0x825B87C0;
	sub_822C0890(ctx, base);
	// 825B87C0: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 825B87C4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825B87C8: 419A00B4  beq cr6, 0x825b887c
	if ctx.cr[6].eq {
	pc = 0x825B887C; continue 'dispatch;
	}
	// 825B87CC: 480000AC  b 0x825b8878
	pc = 0x825B8878; continue 'dispatch;
	// 825B87D0: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 825B87D4: 4858E9CD  bl 0x82b471a0
	ctx.lr = 0x825B87D8;
	sub_82B471A0(ctx, base);
	// 825B87D8: 389E0004  addi r4, r30, 4
	ctx.r[4].s64 = ctx.r[30].s64 + 4;
	// 825B87DC: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 825B87E0: 38A00034  li r5, 0x34
	ctx.r[5].s64 = 52;
	// 825B87E4: 48BEFD2D  bl 0x831a8510
	ctx.lr = 0x825B87E8;
	sub_831A8510(ctx, base);
	// 825B87E8: 817E0038  lwz r11, 0x38(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(56 as u32) ) } as u64;
	// 825B87EC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825B87F0: 3D208204  lis r9, -0x7dfc
	ctx.r[9].s64 = -2113667072;
	// 825B87F4: 914100A4  stw r10, 0xa4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(164 as u32), ctx.r[10].u32 ) };
	// 825B87F8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825B87FC: 3889C268  addi r4, r9, -0x3d98
	ctx.r[4].s64 = ctx.r[9].s64 + -15768;
	// 825B8800: 916100A8  stw r11, 0xa8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(168 as u32), ctx.r[11].u32 ) };
	// 825B8804: 38A00040  li r5, 0x40
	ctx.r[5].s64 = 64;
	// 825B8808: 3860000C  li r3, 0xc
	ctx.r[3].s64 = 12;
	// 825B880C: 4BD07BCD  bl 0x822c03d8
	ctx.lr = 0x825B8810;
	sub_822C03D8(ctx, base);
	// 825B8810: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 825B8814: 4182001C  beq 0x825b8830
	if ctx.cr[0].eq {
	pc = 0x825B8830; continue 'dispatch;
	}
	// 825B8818: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 825B881C: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 825B8820: C02B08A8  lfs f1, 0x8a8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2216 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 825B8824: 48014D85  bl 0x825cd5a8
	ctx.lr = 0x825B8828;
	sub_825CD5A8(ctx, base);
	// 825B8828: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825B882C: 48000008  b 0x825b8834
	pc = 0x825B8834; continue 'dispatch;
	// 825B8830: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 825B8834: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 825B8838: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825B883C: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 825B8840: 4BFFFC39  bl 0x825b8478
	ctx.lr = 0x825B8844;
	sub_825B8478(ctx, base);
	// 825B8844: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 825B8848: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825B884C: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 825B8850: 4BD077B1  bl 0x822c0000
	ctx.lr = 0x825B8854;
	sub_822C0000(ctx, base);
	// 825B8854: 80A10050  lwz r5, 0x50(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 825B8858: 809D00D8  lwz r4, 0xd8(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(216 as u32) ) } as u64;
	// 825B885C: 807D00D0  lwz r3, 0xd0(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(208 as u32) ) } as u64;
	// 825B8860: 4BFE8229  bl 0x825a0a88
	ctx.lr = 0x825B8864;
	sub_825A0A88(ctx, base);
	// 825B8864: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 825B8868: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825B886C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825B8870: 419A000C  beq cr6, 0x825b887c
	if ctx.cr[6].eq {
	pc = 0x825B887C; continue 'dispatch;
	}
	// 825B8874: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 825B8878: 4BD08019  bl 0x822c0890
	ctx.lr = 0x825B887C;
	sub_822C0890(ctx, base);
	// 825B887C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B8880: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 825B8884: 48BEF938  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B8888(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x825B8888 size=360
    let mut pc: u32 = 0x825B8888;
    'dispatch: loop {
        match pc {
            0x825B8888 => {
    //   block [0x825B8888..0x825B89F0)
	// 825B8888: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B888C: 48BEF8E1  bl 0x831a816c
	ctx.lr = 0x825B8890;
	sub_831A8130(ctx, base);
	// 825B8890: DBE1FFD8  stfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 825B8894: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B8898: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 825B889C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 825B88A0: 817F003C  lwz r11, 0x3c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) } as u64;
	// 825B88A4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825B88A8: 419A00A0  beq cr6, 0x825b8948
	if ctx.cr[6].eq {
	pc = 0x825B8948; continue 'dispatch;
	}
	// 825B88AC: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 825B88B0: 816B01C4  lwz r11, 0x1c4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(452 as u32) ) } as u64;
	// 825B88B4: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 825B88B8: C3EA08A8  lfs f31, 0x8a8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2216 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 825B88BC: 419A0020  beq cr6, 0x825b88dc
	if ctx.cr[6].eq {
	pc = 0x825B88DC; continue 'dispatch;
	}
	// 825B88C0: 2B0B0002  cmplwi cr6, r11, 2
	ctx.cr[6].compare_u32(ctx.r[11].u32, 2 as u32, &mut ctx.xer);
	// 825B88C4: 419A0020  beq cr6, 0x825b88e4
	if ctx.cr[6].eq {
	pc = 0x825B88E4; continue 'dispatch;
	}
	// 825B88C8: 2B0B0003  cmplwi cr6, r11, 3
	ctx.cr[6].compare_u32(ctx.r[11].u32, 3 as u32, &mut ctx.xer);
	// 825B88CC: 409A0018  bne cr6, 0x825b88e4
	if !ctx.cr[6].eq {
	pc = 0x825B88E4; continue 'dispatch;
	}
	// 825B88D0: 3D408208  lis r10, -0x7df8
	ctx.r[10].s64 = -2113404928;
	// 825B88D4: C3EA89AC  lfs f31, -0x7654(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-30292 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 825B88D8: 4800000C  b 0x825b88e4
	pc = 0x825B88E4; continue 'dispatch;
	// 825B88DC: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 825B88E0: C3EA964C  lfs f31, -0x69b4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-27060 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 825B88E4: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 825B88E8: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 825B88EC: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 825B88F0: 388AC268  addi r4, r10, -0x3d98
	ctx.r[4].s64 = ctx.r[10].s64 + -15768;
	// 825B88F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825B88F8: 38A0005C  li r5, 0x5c
	ctx.r[5].s64 = 92;
	// 825B88FC: 38600020  li r3, 0x20
	ctx.r[3].s64 = 32;
	// 825B8900: 697E0001  xori r30, r11, 1
	ctx.r[30].u64 = ctx.r[11].u64 ^ 1;
	// 825B8904: 4BD07AD5  bl 0x822c03d8
	ctx.lr = 0x825B8908;
	sub_822C03D8(ctx, base);
	// 825B8908: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 825B890C: 41820024  beq 0x825b8930
	if ctx.cr[0].eq {
	pc = 0x825B8930; continue 'dispatch;
	}
	// 825B8910: 7FC8F378  mr r8, r30
	ctx.r[8].u64 = ctx.r[30].u64;
	// 825B8914: 80BD00C8  lwz r5, 0xc8(r29)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(200 as u32) ) } as u64;
	// 825B8918: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825B891C: C03F0000  lfs f1, 0(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 825B8920: FC40F890  fmr f2, f31
	ctx.f[2].f64 = ctx.f[31].f64;
	// 825B8924: 480010AD  bl 0x825b99d0
	ctx.lr = 0x825B8928;
	sub_825B99D0(ctx, base);
	// 825B8928: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825B892C: 48000008  b 0x825b8934
	pc = 0x825B8934; continue 'dispatch;
	// 825B8930: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 825B8934: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 825B8938: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825B893C: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 825B8940: 4BFFC621  bl 0x825b4f60
	ctx.lr = 0x825B8944;
	sub_825B4F60(ctx, base);
	// 825B8944: 48000070  b 0x825b89b4
	pc = 0x825B89B4; continue 'dispatch;
	// 825B8948: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 825B894C: 4858E855  bl 0x82b471a0
	ctx.lr = 0x825B8950;
	sub_82B471A0(ctx, base);
	// 825B8950: 389F0004  addi r4, r31, 4
	ctx.r[4].s64 = ctx.r[31].s64 + 4;
	// 825B8954: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 825B8958: 38A00034  li r5, 0x34
	ctx.r[5].s64 = 52;
	// 825B895C: 48BEFBB5  bl 0x831a8510
	ctx.lr = 0x825B8960;
	sub_831A8510(ctx, base);
	// 825B8960: 817F0038  lwz r11, 0x38(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 825B8964: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 825B8968: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 825B896C: 93E10094  stw r31, 0x94(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(148 as u32), ctx.r[31].u32 ) };
	// 825B8970: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825B8974: 388AC268  addi r4, r10, -0x3d98
	ctx.r[4].s64 = ctx.r[10].s64 + -15768;
	// 825B8978: 91610098  stw r11, 0x98(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(152 as u32), ctx.r[11].u32 ) };
	// 825B897C: 38A00066  li r5, 0x66
	ctx.r[5].s64 = 102;
	// 825B8980: 3860000C  li r3, 0xc
	ctx.r[3].s64 = 12;
	// 825B8984: 4BD07A55  bl 0x822c03d8
	ctx.lr = 0x825B8988;
	sub_822C03D8(ctx, base);
	// 825B8988: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 825B898C: 41820018  beq 0x825b89a4
	if ctx.cr[0].eq {
	pc = 0x825B89A4; continue 'dispatch;
	}
	// 825B8990: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 825B8994: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 825B8998: C02B08A8  lfs f1, 0x8a8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2216 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 825B899C: 48014C0D  bl 0x825cd5a8
	ctx.lr = 0x825B89A0;
	sub_825CD5A8(ctx, base);
	// 825B89A0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825B89A4: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 825B89A8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825B89AC: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 825B89B0: 4BFFFAC9  bl 0x825b8478
	ctx.lr = 0x825B89B4;
	sub_825B8478(ctx, base);
	// 825B89B4: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 825B89B8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825B89BC: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 825B89C0: 4BD07641  bl 0x822c0000
	ctx.lr = 0x825B89C4;
	sub_822C0000(ctx, base);
	// 825B89C4: 80A10050  lwz r5, 0x50(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 825B89C8: 809D00D8  lwz r4, 0xd8(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(216 as u32) ) } as u64;
	// 825B89CC: 807D00D0  lwz r3, 0xd0(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(208 as u32) ) } as u64;
	// 825B89D0: 4BFE80D9  bl 0x825a0aa8
	ctx.lr = 0x825B89D4;
	sub_825A0AA8(ctx, base);
	// 825B89D4: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 825B89D8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825B89DC: 419A0008  beq cr6, 0x825b89e4
	if ctx.cr[6].eq {
	pc = 0x825B89E4; continue 'dispatch;
	}
	// 825B89E0: 4BD07EB1  bl 0x822c0890
	ctx.lr = 0x825B89E4;
	sub_822C0890(ctx, base);
	// 825B89E4: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 825B89E8: CBE1FFD8  lfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-40 as u32) ) };
	// 825B89EC: 48BEF7D0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B89F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x825B89F0 size=188
    let mut pc: u32 = 0x825B89F0;
    'dispatch: loop {
        match pc {
            0x825B89F0 => {
    //   block [0x825B89F0..0x825B8AAC)
	// 825B89F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B89F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825B89F8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825B89FC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825B8A00: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B8A04: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 825B8A08: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 825B8A0C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 825B8A10: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825B8A14: 388BC268  addi r4, r11, -0x3d98
	ctx.r[4].s64 = ctx.r[11].s64 + -15768;
	// 825B8A18: 38A00086  li r5, 0x86
	ctx.r[5].s64 = 134;
	// 825B8A1C: 38600020  li r3, 0x20
	ctx.r[3].s64 = 32;
	// 825B8A20: 4BD079B9  bl 0x822c03d8
	ctx.lr = 0x825B8A24;
	sub_822C03D8(ctx, base);
	// 825B8A24: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 825B8A28: 41820028  beq 0x825b8a50
	if ctx.cr[0].eq {
	pc = 0x825B8A50; continue 'dispatch;
	}
	// 825B8A2C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 825B8A30: 80BE00C8  lwz r5, 0xc8(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(200 as u32) ) } as u64;
	// 825B8A34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825B8A38: C03F0000  lfs f1, 0(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 825B8A3C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825B8A40: C04B08A8  lfs f2, 0x8a8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2216 as u32) ) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 825B8A44: 48000F8D  bl 0x825b99d0
	ctx.lr = 0x825B8A48;
	sub_825B99D0(ctx, base);
	// 825B8A48: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825B8A4C: 48000008  b 0x825b8a54
	pc = 0x825B8A54; continue 'dispatch;
	// 825B8A50: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 825B8A54: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 825B8A58: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825B8A5C: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 825B8A60: 4BFFC501  bl 0x825b4f60
	ctx.lr = 0x825B8A64;
	sub_825B4F60(ctx, base);
	// 825B8A64: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 825B8A68: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825B8A6C: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 825B8A70: 4BD07591  bl 0x822c0000
	ctx.lr = 0x825B8A74;
	sub_822C0000(ctx, base);
	// 825B8A74: 80A10050  lwz r5, 0x50(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 825B8A78: 809E00D8  lwz r4, 0xd8(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(216 as u32) ) } as u64;
	// 825B8A7C: 807E00D0  lwz r3, 0xd0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(208 as u32) ) } as u64;
	// 825B8A80: 4BFE7FE9  bl 0x825a0a68
	ctx.lr = 0x825B8A84;
	sub_825A0A68(ctx, base);
	// 825B8A84: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 825B8A88: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825B8A8C: 419A0008  beq cr6, 0x825b8a94
	if ctx.cr[6].eq {
	pc = 0x825B8A94; continue 'dispatch;
	}
	// 825B8A90: 4BD07E01  bl 0x822c0890
	ctx.lr = 0x825B8A94;
	sub_822C0890(ctx, base);
	// 825B8A94: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825B8A98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825B8A9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825B8AA0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825B8AA4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825B8AA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B8AB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825B8AB0 size=96
    let mut pc: u32 = 0x825B8AB0;
    'dispatch: loop {
        match pc {
            0x825B8AB0 => {
    //   block [0x825B8AB0..0x825B8B10)
	// 825B8AB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B8AB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825B8AB8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825B8ABC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825B8AC0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B8AC4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825B8AC8: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 825B8ACC: 4BFFF7ED  bl 0x825b82b8
	ctx.lr = 0x825B8AD0;
	sub_825B82B8(ctx, base);
	// 825B8AD0: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 825B8AD4: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 825B8AD8: 396BC2D4  addi r11, r11, -0x3d2c
	ctx.r[11].s64 = ctx.r[11].s64 + -15660;
	// 825B8ADC: 394AC2C0  addi r10, r10, -0x3d40
	ctx.r[10].s64 = ctx.r[10].s64 + -15680;
	// 825B8AE0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825B8AE4: 387F00D0  addi r3, r31, 0xd0
	ctx.r[3].s64 = ctx.r[31].s64 + 208;
	// 825B8AE8: 915F0028  stw r10, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[10].u32 ) };
	// 825B8AEC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 825B8AF0: 4BFD3421  bl 0x8258bf10
	ctx.lr = 0x825B8AF4;
	sub_8258BF10(ctx, base);
	// 825B8AF4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B8AF8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825B8AFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825B8B00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825B8B04: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825B8B08: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825B8B0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B8B10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825B8B10 size=8
    let mut pc: u32 = 0x825B8B10;
    'dispatch: loop {
        match pc {
            0x825B8B10 => {
    //   block [0x825B8B10..0x825B8B18)
	// 825B8B10: 3863FFD8  addi r3, r3, -0x28
	ctx.r[3].s64 = ctx.r[3].s64 + -40;
	// 825B8B14: 48000004  b 0x825b8b18
	sub_825B8B18(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B8B18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825B8B18 size=96
    let mut pc: u32 = 0x825B8B18;
    'dispatch: loop {
        match pc {
            0x825B8B18 => {
    //   block [0x825B8B18..0x825B8B78)
	// 825B8B18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B8B1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825B8B20: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825B8B24: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825B8B28: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B8B2C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825B8B30: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825B8B34: 387F00D0  addi r3, r31, 0xd0
	ctx.r[3].s64 = ctx.r[31].s64 + 208;
	// 825B8B38: 4BD10181  bl 0x822c8cb8
	ctx.lr = 0x825B8B3C;
	sub_822C8CB8(ctx, base);
	// 825B8B3C: 387F00C0  addi r3, r31, 0xc0
	ctx.r[3].s64 = ctx.r[31].s64 + 192;
	// 825B8B40: 4883A8E9  bl 0x82df3428
	ctx.lr = 0x825B8B44;
	sub_82DF3428(ctx, base);
	// 825B8B44: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B8B48: 4BF58651  bl 0x82511198
	ctx.lr = 0x825B8B4C;
	sub_82511198(ctx, base);
	// 825B8B4C: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825B8B50: 4182000C  beq 0x825b8b5c
	if ctx.cr[0].eq {
	pc = 0x825B8B5C; continue 'dispatch;
	}
	// 825B8B54: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B8B58: 48839881  bl 0x82df23d8
	ctx.lr = 0x825B8B5C;
	sub_82DF23D8(ctx, base);
	// 825B8B5C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B8B60: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825B8B64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825B8B68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825B8B6C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825B8B70: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825B8B74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B8B78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825B8B78 size=276
    let mut pc: u32 = 0x825B8B78;
    'dispatch: loop {
        match pc {
            0x825B8B78 => {
    //   block [0x825B8B78..0x825B8C8C)
	// 825B8B78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B8B7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825B8B80: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825B8B84: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825B8B88: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B8B8C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 825B8B90: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 825B8B94: 389E00D0  addi r4, r30, 0xd0
	ctx.r[4].s64 = ctx.r[30].s64 + 208;
	// 825B8B98: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 825B8B9C: 481FD005  bl 0x827b5ba0
	ctx.lr = 0x825B8BA0;
	sub_827B5BA0(ctx, base);
	// 825B8BA0: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 825B8BA4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 825B8BA8: 917F0050  stw r11, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 825B8BAC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825B8BB0: 480144B9  bl 0x825cd068
	ctx.lr = 0x825B8BB4;
	sub_825CD068(ctx, base);
	// 825B8BB4: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 825B8BB8: 387F0040  addi r3, r31, 0x40
	ctx.r[3].s64 = ctx.r[31].s64 + 64;
	// 825B8BBC: 4883B015  bl 0x82df3bd0
	ctx.lr = 0x825B8BC0;
	sub_82DF3BD0(ctx, base);
	// 825B8BC0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825B8BC4: 4883A865  bl 0x82df3428
	ctx.lr = 0x825B8BC8;
	sub_82DF3428(ctx, base);
	// 825B8BC8: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 825B8BCC: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 825B8BD0: 807F003C  lwz r3, 0x3c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) } as u64;
	// 825B8BD4: 4BFF9BFD  bl 0x825b27d0
	ctx.lr = 0x825B8BD8;
	sub_825B27D0(ctx, base);
	// 825B8BD8: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 825B8BDC: 809F003C  lwz r4, 0x3c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) } as u64;
	// 825B8BE0: 4BF56989  bl 0x8250f568
	ctx.lr = 0x825B8BE4;
	sub_8250F568(ctx, base);
	// 825B8BE4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B8BE8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825B8BEC: 388BFF40  addi r4, r11, -0xc0
	ctx.r[4].s64 = ctx.r[11].s64 + -192;
	// 825B8BF0: 409A0008  bne cr6, 0x825b8bf8
	if !ctx.cr[6].eq {
	pc = 0x825B8BF8; continue 'dispatch;
	}
	// 825B8BF4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 825B8BF8: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 825B8BFC: 481F22E5  bl 0x827aaee0
	ctx.lr = 0x825B8C00;
	sub_827AAEE0(ctx, base);
	// 825B8C00: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 825B8C04: 4883908D  bl 0x82df1c90
	ctx.lr = 0x825B8C08;
	sub_82DF1C90(ctx, base);
	// 825B8C08: 81610068  lwz r11, 0x68(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 825B8C0C: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 825B8C10: 7D7F5B78  mr r31, r11
	ctx.r[31].u64 = ctx.r[11].u64;
	// 825B8C14: 480D0A75  bl 0x82689688
	ctx.lr = 0x825B8C18;
	sub_82689688(ctx, base);
	// 825B8C18: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 825B8C1C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825B8C20: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 825B8C24: 481EEC45  bl 0x827a7868
	ctx.lr = 0x825B8C28;
	sub_827A7868(ctx, base);
	// 825B8C28: 81610060  lwz r11, 0x60(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 825B8C2C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825B8C30: 419A0010  beq cr6, 0x825b8c40
	if ctx.cr[6].eq {
	pc = 0x825B8C40; continue 'dispatch;
	}
	// 825B8C34: 388B000C  addi r4, r11, 0xc
	ctx.r[4].s64 = ctx.r[11].s64 + 12;
	// 825B8C38: 80610058  lwz r3, 0x58(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 825B8C3C: 48B0A465  bl 0x830c30a0
	ctx.lr = 0x825B8C40;
	sub_830C30A0(ctx, base);
	// 825B8C40: 80610064  lwz r3, 0x64(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 825B8C44: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825B8C48: 419A0008  beq cr6, 0x825b8c50
	if ctx.cr[6].eq {
	pc = 0x825B8C50; continue 'dispatch;
	}
	// 825B8C4C: 4BD07C45  bl 0x822c0890
	ctx.lr = 0x825B8C50;
	sub_822C0890(ctx, base);
	// 825B8C50: 8061006C  lwz r3, 0x6c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(108 as u32) ) } as u64;
	// 825B8C54: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825B8C58: 419A0008  beq cr6, 0x825b8c60
	if ctx.cr[6].eq {
	pc = 0x825B8C60; continue 'dispatch;
	}
	// 825B8C5C: 4BD07C35  bl 0x822c0890
	ctx.lr = 0x825B8C60;
	sub_822C0890(ctx, base);
	// 825B8C60: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 825B8C64: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825B8C68: 419A0008  beq cr6, 0x825b8c70
	if ctx.cr[6].eq {
	pc = 0x825B8C70; continue 'dispatch;
	}
	// 825B8C6C: 4BD07C25  bl 0x822c0890
	ctx.lr = 0x825B8C70;
	sub_822C0890(ctx, base);
	// 825B8C70: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 825B8C74: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 825B8C78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825B8C7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825B8C80: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825B8C84: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825B8C88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B8C90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825B8C90 size=196
    let mut pc: u32 = 0x825B8C90;
    'dispatch: loop {
        match pc {
            0x825B8C90 => {
    //   block [0x825B8C90..0x825B8D54)
	// 825B8C90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B8C94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825B8C98: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825B8C9C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825B8CA0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B8CA4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 825B8CA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825B8CAC: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 825B8CB0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 825B8CB4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825B8CB8: 4BD07C81  bl 0x822c0938
	ctx.lr = 0x825B8CBC;
	sub_822C0938(ctx, base);
	// 825B8CBC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 825B8CC0: 41820028  beq 0x825b8ce8
	if ctx.cr[0].eq {
	pc = 0x825B8CE8; continue 'dispatch;
	}
	// 825B8CC4: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 825B8CC8: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 825B8CCC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825B8CD0: 392BC324  addi r9, r11, -0x3cdc
	ctx.r[9].s64 = ctx.r[11].s64 + -15580;
	// 825B8CD4: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 825B8CD8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 825B8CDC: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 825B8CE0: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 825B8CE4: 48000008  b 0x825b8cec
	pc = 0x825B8CEC; continue 'dispatch;
	// 825B8CE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825B8CEC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825B8CF0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825B8CF4: 409A0044  bne cr6, 0x825b8d38
	if !ctx.cr[6].eq {
	pc = 0x825B8D38; continue 'dispatch;
	}
	// 825B8CF8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 825B8CFC: 419A001C  beq cr6, 0x825b8d18
	if ctx.cr[6].eq {
	pc = 0x825B8D18; continue 'dispatch;
	}
	// 825B8D00: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B8D04: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 825B8D08: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B8D0C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B8D10: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825B8D14: 4E800421  bctrl
	ctx.lr = 0x825B8D18;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825B8D18: 3D608329  lis r11, -0x7cd7
	ctx.r[11].s64 = -2094465024;
	// 825B8D1C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 825B8D20: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825B8D24: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 825B8D28: 816B95C4  lwz r11, -0x6a3c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27196 as u32) ) } as u64;
	// 825B8D2C: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 825B8D30: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825B8D34: 4BD072CD  bl 0x822c0000
	ctx.lr = 0x825B8D38;
	sub_822C0000(ctx, base);
	// 825B8D38: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825B8D3C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825B8D40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825B8D44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825B8D48: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825B8D4C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825B8D50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B8D58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825B8D58 size=136
    let mut pc: u32 = 0x825B8D58;
    'dispatch: loop {
        match pc {
            0x825B8D58 => {
    //   block [0x825B8D58..0x825B8DE0)
	// 825B8D58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B8D5C: 48BEF40D  bl 0x831a8168
	ctx.lr = 0x825B8D60;
	sub_831A8130(ctx, base);
	// 825B8D60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B8D64: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 825B8D68: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 825B8D6C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 825B8D70: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 825B8D74: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 825B8D78: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825B8D7C: 388BA66C  addi r4, r11, -0x5994
	ctx.r[4].s64 = ctx.r[11].s64 + -22932;
	// 825B8D80: 38A00089  li r5, 0x89
	ctx.r[5].s64 = 137;
	// 825B8D84: 38600050  li r3, 0x50
	ctx.r[3].s64 = 80;
	// 825B8D88: 48839661  bl 0x82df23e8
	ctx.lr = 0x825B8D8C;
	sub_82DF23E8(ctx, base);
	// 825B8D8C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 825B8D90: 4182001C  beq 0x825b8dac
	if ctx.cr[0].eq {
	pc = 0x825B8DAC; continue 'dispatch;
	}
	// 825B8D94: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 825B8D98: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 825B8D9C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825B8DA0: 485A1181  bl 0x82b59f20
	ctx.lr = 0x825B8DA4;
	sub_82B59F20(ctx, base);
	// 825B8DA4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825B8DA8: 48000008  b 0x825b8db0
	pc = 0x825B8DB0; continue 'dispatch;
	// 825B8DAC: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 825B8DB0: 93FC0000  stw r31, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 825B8DB4: 3BDC0004  addi r30, r28, 4
	ctx.r[30].s64 = ctx.r[28].s64 + 4;
	// 825B8DB8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825B8DBC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825B8DC0: 4BFFFED1  bl 0x825b8c90
	ctx.lr = 0x825B8DC4;
	sub_825B8C90(ctx, base);
	// 825B8DC4: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 825B8DC8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825B8DCC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825B8DD0: 4BD07231  bl 0x822c0000
	ctx.lr = 0x825B8DD4;
	sub_822C0000(ctx, base);
	// 825B8DD4: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 825B8DD8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825B8DDC: 48BEF3DC  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B8DE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x825B8DE0 size=84
    let mut pc: u32 = 0x825B8DE0;
    'dispatch: loop {
        match pc {
            0x825B8DE0 => {
    //   block [0x825B8DE0..0x825B8E34)
	// 825B8DE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B8DE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825B8DE8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825B8DEC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B8DF0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825B8DF4: 4BF582FD  bl 0x825110f0
	ctx.lr = 0x825B8DF8;
	sub_825110F0(ctx, base);
	// 825B8DF8: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 825B8DFC: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 825B8E00: 3D208204  lis r9, -0x7dfc
	ctx.r[9].s64 = -2113667072;
	// 825B8E04: 394AC364  addi r10, r10, -0x3c9c
	ctx.r[10].s64 = ctx.r[10].s64 + -15516;
	// 825B8E08: 3929C350  addi r9, r9, -0x3cb0
	ctx.r[9].s64 = ctx.r[9].s64 + -15536;
	// 825B8E0C: C00B08A4  lfs f0, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825B8E10: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 825B8E14: D01F00C0  stfs f0, 0xc0(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(192 as u32), tmp.u32 ) };
	// 825B8E18: 913F0028  stw r9, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[9].u32 ) };
	// 825B8E1C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B8E20: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825B8E24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825B8E28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825B8E2C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825B8E30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B8E38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825B8E38 size=24
    let mut pc: u32 = 0x825B8E38;
    'dispatch: loop {
        match pc {
            0x825B8E38 => {
    //   block [0x825B8E38..0x825B8E50)
	// 825B8E38: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 825B8E3C: 814B7BE4  lwz r10, 0x7be4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(31716 as u32) ) } as u64;
	// 825B8E40: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 825B8E44: 914B7BE4  stw r10, 0x7be4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(31716 as u32), ctx.r[10].u32 ) };
	// 825B8E48: 2B0A0002  cmplwi cr6, r10, 2
	ctx.cr[6].compare_u32(ctx.r[10].u32, 2 as u32, &mut ctx.xer);
	// 825B8E4C: 4D980020  bltlr cr6
	if ctx.cr[6].lt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B8E50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825B8E50 size=12
    let mut pc: u32 = 0x825B8E50;
    'dispatch: loop {
        match pc {
            0x825B8E50 => {
    //   block [0x825B8E50..0x825B8E5C)
	// 825B8E50: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825B8E54: 914B7BE4  stw r10, 0x7be4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(31716 as u32), ctx.r[10].u32 ) };
	// 825B8E58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B8E60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825B8E60 size=196
    let mut pc: u32 = 0x825B8E60;
    'dispatch: loop {
        match pc {
            0x825B8E60 => {
    //   block [0x825B8E60..0x825B8F24)
	// 825B8E60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B8E64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825B8E68: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825B8E6C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825B8E70: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B8E74: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 825B8E78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825B8E7C: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 825B8E80: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 825B8E84: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825B8E88: 4BD07AB1  bl 0x822c0938
	ctx.lr = 0x825B8E8C;
	sub_822C0938(ctx, base);
	// 825B8E8C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 825B8E90: 41820028  beq 0x825b8eb8
	if ctx.cr[0].eq {
	pc = 0x825B8EB8; continue 'dispatch;
	}
	// 825B8E94: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 825B8E98: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 825B8E9C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825B8EA0: 392BC398  addi r9, r11, -0x3c68
	ctx.r[9].s64 = ctx.r[11].s64 + -15464;
	// 825B8EA4: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 825B8EA8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 825B8EAC: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 825B8EB0: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 825B8EB4: 48000008  b 0x825b8ebc
	pc = 0x825B8EBC; continue 'dispatch;
	// 825B8EB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825B8EBC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825B8EC0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825B8EC4: 409A0044  bne cr6, 0x825b8f08
	if !ctx.cr[6].eq {
	pc = 0x825B8F08; continue 'dispatch;
	}
	// 825B8EC8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 825B8ECC: 419A001C  beq cr6, 0x825b8ee8
	if ctx.cr[6].eq {
	pc = 0x825B8EE8; continue 'dispatch;
	}
	// 825B8ED0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B8ED4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 825B8ED8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B8EDC: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B8EE0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825B8EE4: 4E800421  bctrl
	ctx.lr = 0x825B8EE8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825B8EE8: 3D608329  lis r11, -0x7cd7
	ctx.r[11].s64 = -2094465024;
	// 825B8EEC: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 825B8EF0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825B8EF4: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 825B8EF8: 816B9640  lwz r11, -0x69c0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27072 as u32) ) } as u64;
	// 825B8EFC: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 825B8F00: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825B8F04: 4BD070FD  bl 0x822c0000
	ctx.lr = 0x825B8F08;
	sub_822C0000(ctx, base);
	// 825B8F08: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825B8F0C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825B8F10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825B8F14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825B8F18: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825B8F1C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825B8F20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B8F28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x825B8F28 size=380
    let mut pc: u32 = 0x825B8F28;
    'dispatch: loop {
        match pc {
            0x825B8F28 => {
    //   block [0x825B8F28..0x825B90A4)
	// 825B8F28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B8F2C: 48BEF241  bl 0x831a816c
	ctx.lr = 0x825B8F30;
	sub_831A8130(ctx, base);
	// 825B8F30: DBE1FFD8  stfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 825B8F34: 9421FEC0  stwu r1, -0x140(r1)
	ea = ctx.r[1].u32.wrapping_add(-320 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B8F38: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825B8F3C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 825B8F40: 419A0158  beq cr6, 0x825b9098
	if ctx.cr[6].eq {
	pc = 0x825B9098; continue 'dispatch;
	}
	// 825B8F44: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 825B8F48: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 825B8F4C: 388BC3E0  addi r4, r11, -0x3c20
	ctx.r[4].s64 = ctx.r[11].s64 + -15392;
	// 825B8F50: 4883AAB9  bl 0x82df3a08
	ctx.lr = 0x825B8F54;
	sub_82DF3A08(ctx, base);
	// 825B8F54: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 825B8F58: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825B8F5C: 388BC3D0  addi r4, r11, -0x3c30
	ctx.r[4].s64 = ctx.r[11].s64 + -15408;
	// 825B8F60: 4883AAA9  bl 0x82df3a08
	ctx.lr = 0x825B8F64;
	sub_82DF3A08(ctx, base);
	// 825B8F64: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 825B8F68: 3D408329  lis r10, -0x7cd7
	ctx.r[10].s64 = -2094465024;
	// 825B8F6C: 3D208204  lis r9, -0x7dfc
	ctx.r[9].s64 = -2113667072;
	// 825B8F70: 3BCA9638  addi r30, r10, -0x69c8
	ctx.r[30].s64 = ctx.r[10].s64 + -27080;
	// 825B8F74: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 825B8F78: C3EB964C  lfs f31, -0x69b4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27060 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 825B8F7C: 38BE0004  addi r5, r30, 4
	ctx.r[5].s64 = ctx.r[30].s64 + 4;
	// 825B8F80: 38610090  addi r3, r1, 0x90
	ctx.r[3].s64 = ctx.r[1].s64 + 144;
	// 825B8F84: FC60F890  fmr f3, f31
	ctx.f[3].f64 = ctx.f[31].f64;
	// 825B8F88: C049DD6C  lfs f2, -0x2294(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-8852 as u32) ) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 825B8F8C: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 825B8F90: 4BFEA319  bl 0x825a32a8
	ctx.lr = 0x825B8F94;
	sub_825A32A8(ctx, base);
	// 825B8F94: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 825B8F98: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 825B8F9C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B8FA0: 4BFE87D1  bl 0x825a1770
	ctx.lr = 0x825B8FA4;
	sub_825A1770(ctx, base);
	// 825B8FA4: 386100C8  addi r3, r1, 0xc8
	ctx.r[3].s64 = ctx.r[1].s64 + 200;
	// 825B8FA8: 4883A481  bl 0x82df3428
	ctx.lr = 0x825B8FAC;
	sub_82DF3428(ctx, base);
	// 825B8FAC: 386100A8  addi r3, r1, 0xa8
	ctx.r[3].s64 = ctx.r[1].s64 + 168;
	// 825B8FB0: 4BD0FD09  bl 0x822c8cb8
	ctx.lr = 0x825B8FB4;
	sub_822C8CB8(ctx, base);
	// 825B8FB4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825B8FB8: 4883A471  bl 0x82df3428
	ctx.lr = 0x825B8FBC;
	sub_82DF3428(ctx, base);
	// 825B8FBC: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 825B8FC0: 4883A469  bl 0x82df3428
	ctx.lr = 0x825B8FC4;
	sub_82DF3428(ctx, base);
	// 825B8FC4: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 825B8FC8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825B8FCC: 388BC3C4  addi r4, r11, -0x3c3c
	ctx.r[4].s64 = ctx.r[11].s64 + -15420;
	// 825B8FD0: 4883AA39  bl 0x82df3a08
	ctx.lr = 0x825B8FD4;
	sub_82DF3A08(ctx, base);
	// 825B8FD4: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 825B8FD8: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 825B8FDC: 3BABC3B8  addi r29, r11, -0x3c48
	ctx.r[29].s64 = ctx.r[11].s64 + -15432;
	// 825B8FE0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 825B8FE4: 4883AA25  bl 0x82df3a08
	ctx.lr = 0x825B8FE8;
	sub_82DF3A08(ctx, base);
	// 825B8FE8: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 825B8FEC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 825B8FF0: 38AB7BE0  addi r5, r11, 0x7be0
	ctx.r[5].s64 = ctx.r[11].s64 + 31712;
	// 825B8FF4: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 825B8FF8: 4BFEA059  bl 0x825a3050
	ctx.lr = 0x825B8FFC;
	sub_825A3050(ctx, base);
	// 825B8FFC: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 825B9000: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 825B9004: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B9008: 4BFE9181  bl 0x825a2188
	ctx.lr = 0x825B900C;
	sub_825A2188(ctx, base);
	// 825B900C: 38610088  addi r3, r1, 0x88
	ctx.r[3].s64 = ctx.r[1].s64 + 136;
	// 825B9010: 4883A419  bl 0x82df3428
	ctx.lr = 0x825B9014;
	sub_82DF3428(ctx, base);
	// 825B9014: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 825B9018: 4BD0FCA1  bl 0x822c8cb8
	ctx.lr = 0x825B901C;
	sub_822C8CB8(ctx, base);
	// 825B901C: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 825B9020: 4883A409  bl 0x82df3428
	ctx.lr = 0x825B9024;
	sub_82DF3428(ctx, base);
	// 825B9024: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825B9028: 4883A401  bl 0x82df3428
	ctx.lr = 0x825B902C;
	sub_82DF3428(ctx, base);
	// 825B902C: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 825B9030: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825B9034: 388BC3A8  addi r4, r11, -0x3c58
	ctx.r[4].s64 = ctx.r[11].s64 + -15448;
	// 825B9038: 4883A9D1  bl 0x82df3a08
	ctx.lr = 0x825B903C;
	sub_82DF3A08(ctx, base);
	// 825B903C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 825B9040: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 825B9044: 4883A9C5  bl 0x82df3a08
	ctx.lr = 0x825B9048;
	sub_82DF3A08(ctx, base);
	// 825B9048: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 825B904C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 825B9050: FC60F890  fmr f3, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[3].f64 = ctx.f[31].f64;
	// 825B9054: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 825B9058: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 825B905C: 386100D0  addi r3, r1, 0xd0
	ctx.r[3].s64 = ctx.r[1].s64 + 208;
	// 825B9060: C04B9A8C  lfs f2, -0x6574(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-25972 as u32) ) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 825B9064: 4BFEA245  bl 0x825a32a8
	ctx.lr = 0x825B9068;
	sub_825A32A8(ctx, base);
	// 825B9068: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 825B906C: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 825B9070: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B9074: 4BFE86FD  bl 0x825a1770
	ctx.lr = 0x825B9078;
	sub_825A1770(ctx, base);
	// 825B9078: 38610108  addi r3, r1, 0x108
	ctx.r[3].s64 = ctx.r[1].s64 + 264;
	// 825B907C: 4883A3AD  bl 0x82df3428
	ctx.lr = 0x825B9080;
	sub_82DF3428(ctx, base);
	// 825B9080: 386100E8  addi r3, r1, 0xe8
	ctx.r[3].s64 = ctx.r[1].s64 + 232;
	// 825B9084: 4BD0FC35  bl 0x822c8cb8
	ctx.lr = 0x825B9088;
	sub_822C8CB8(ctx, base);
	// 825B9088: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 825B908C: 4883A39D  bl 0x82df3428
	ctx.lr = 0x825B9090;
	sub_82DF3428(ctx, base);
	// 825B9090: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825B9094: 4883A395  bl 0x82df3428
	ctx.lr = 0x825B9098;
	sub_82DF3428(ctx, base);
	// 825B9098: 38210140  addi r1, r1, 0x140
	ctx.r[1].s64 = ctx.r[1].s64 + 320;
	// 825B909C: CBE1FFD8  lfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-40 as u32) ) };
	// 825B90A0: 48BEF11C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B90A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825B90A8 size=124
    let mut pc: u32 = 0x825B90A8;
    'dispatch: loop {
        match pc {
            0x825B90A8 => {
    //   block [0x825B90A8..0x825B9124)
	// 825B90A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B90AC: 48BEF0C1  bl 0x831a816c
	ctx.lr = 0x825B90B0;
	sub_831A8130(ctx, base);
	// 825B90B0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B90B4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 825B90B8: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 825B90BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825B90C0: 388BA66C  addi r4, r11, -0x5994
	ctx.r[4].s64 = ctx.r[11].s64 + -22932;
	// 825B90C4: 38A00073  li r5, 0x73
	ctx.r[5].s64 = 115;
	// 825B90C8: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 825B90CC: 4883931D  bl 0x82df23e8
	ctx.lr = 0x825B90D0;
	sub_82DF23E8(ctx, base);
	// 825B90D0: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 825B90D4: 4182001C  beq 0x825b90f0
	if ctx.cr[0].eq {
	pc = 0x825B90F0; continue 'dispatch;
	}
	// 825B90D8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B90DC: 488A05A5  bl 0x82e59680
	ctx.lr = 0x825B90E0;
	sub_82E59680(ctx, base);
	// 825B90E0: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 825B90E4: 396BC390  addi r11, r11, -0x3c70
	ctx.r[11].s64 = ctx.r[11].s64 + -15472;
	// 825B90E8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825B90EC: 48000008  b 0x825b90f4
	pc = 0x825B90F4; continue 'dispatch;
	// 825B90F0: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 825B90F4: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 825B90F8: 3BDD0004  addi r30, r29, 4
	ctx.r[30].s64 = ctx.r[29].s64 + 4;
	// 825B90FC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825B9100: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825B9104: 4BFFFD5D  bl 0x825b8e60
	ctx.lr = 0x825B9108;
	sub_825B8E60(ctx, base);
	// 825B9108: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 825B910C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825B9110: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825B9114: 4BD06EED  bl 0x822c0000
	ctx.lr = 0x825B9118;
	sub_822C0000(ctx, base);
	// 825B9118: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 825B911C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825B9120: 48BEF09C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B9128(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x825B9128 size=1800
    let mut pc: u32 = 0x825B9128;
    'dispatch: loop {
        match pc {
            0x825B9128 => {
    //   block [0x825B9128..0x825B9830)
	// 825B9128: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B912C: 48BEF02D  bl 0x831a8158
	ctx.lr = 0x825B9130;
	sub_831A8130(ctx, base);
	// 825B9130: DBC1FFA8  stfd f30, -0x58(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-88 as u32), ctx.f[30].u64 ) };
	// 825B9134: DBE1FFB0  stfd f31, -0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-80 as u32), ctx.f[31].u64 ) };
	// 825B9138: 9421FDC0  stwu r1, -0x240(r1)
	ea = ctx.r[1].u32.wrapping_add(-576 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B913C: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 825B9140: 3D608329  lis r11, -0x7cd7
	ctx.r[11].s64 = -2094465024;
	// 825B9144: 3BAA7BE0  addi r29, r10, 0x7be0
	ctx.r[29].s64 = ctx.r[10].s64 + 31712;
	// 825B9148: 3B8B9638  addi r28, r11, -0x69c8
	ctx.r[28].s64 = ctx.r[11].s64 + -27080;
	// 825B914C: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 825B9150: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 825B9154: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 825B9158: C3DC0004  lfs f30, 4(r28)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 825B915C: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 825B9160: 409A000C  bne cr6, 0x825b916c
	if !ctx.cr[6].eq {
	pc = 0x825B916C; continue 'dispatch;
	}
	// 825B9164: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 825B9168: C3CB6218  lfs f30, 0x6218(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(25112 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 825B916C: 807F003C  lwz r3, 0x3c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) } as u64;
	// 825B9170: FC20F090  fmr f1, f30
	ctx.f[1].f64 = ctx.f[30].f64;
	// 825B9174: 4BFF93E5  bl 0x825b2558
	ctx.lr = 0x825B9178;
	sub_825B2558(ctx, base);
	// 825B9178: 7C7A1B79  or. r26, r3, r3
	ctx.r[26].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[26].s32, 0, &mut ctx.xer);
	// 825B917C: 418206A4  beq 0x825b9820
	if ctx.cr[0].eq {
	pc = 0x825B9820; continue 'dispatch;
	}
	// 825B9180: 396100B0  addi r11, r1, 0xb0
	ctx.r[11].s64 = ctx.r[1].s64 + 176;
	// 825B9184: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 825B9188: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825B918C: 38610148  addi r3, r1, 0x148
	ctx.r[3].s64 = ctx.r[1].s64 + 328;
	// 825B9190: 4BD64041  bl 0x8231d1d0
	ctx.lr = 0x825B9194;
	sub_8231D1D0(ctx, base);
	// 825B9194: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B9198: 916100A8  stw r11, 0xa8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(168 as u32), ctx.r[11].u32 ) };
	// 825B919C: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 825B91A0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825B91A4: 916100AC  stw r11, 0xac(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(172 as u32), ctx.r[11].u32 ) };
	// 825B91A8: 419A0024  beq cr6, 0x825b91cc
	if ctx.cr[6].eq {
	pc = 0x825B91CC; continue 'dispatch;
	}
	// 825B91AC: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 825B91B0: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 825B91B4: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 825B91B8: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 825B91BC: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 825B91C0: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 825B91C4: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 825B91C8: 4082FFE8  bne 0x825b91b0
	if !ctx.cr[0].eq {
	pc = 0x825B91B0; continue 'dispatch;
	}
	// 825B91CC: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 825B91D0: 3B3B0028  addi r25, r27, 0x28
	ctx.r[25].s64 = ctx.r[27].s64 + 40;
	// 825B91D4: 3B0BC3F0  addi r24, r11, -0x3c10
	ctx.r[24].s64 = ctx.r[11].s64 + -15376;
	// 825B91D8: 38E100A8  addi r7, r1, 0xa8
	ctx.r[7].s64 = ctx.r[1].s64 + 168;
	// 825B91DC: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 825B91E0: 7F46D378  mr r6, r26
	ctx.r[6].u64 = ctx.r[26].u64;
	// 825B91E4: 38A0003D  li r5, 0x3d
	ctx.r[5].s64 = 61;
	// 825B91E8: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 825B91EC: 4889DDFD  bl 0x82e56fe8
	ctx.lr = 0x825B91F0;
	sub_82E56FE8(ctx, base);
	// 825B91F0: 806100AC  lwz r3, 0xac(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(172 as u32) ) } as u64;
	// 825B91F4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825B91F8: 419A0008  beq cr6, 0x825b9200
	if ctx.cr[6].eq {
	pc = 0x825B9200; continue 'dispatch;
	}
	// 825B91FC: 4BD07695  bl 0x822c0890
	ctx.lr = 0x825B9200;
	sub_822C0890(ctx, base);
	// 825B9200: 8061014C  lwz r3, 0x14c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(332 as u32) ) } as u64;
	// 825B9204: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825B9208: 419A0008  beq cr6, 0x825b9210
	if ctx.cr[6].eq {
	pc = 0x825B9210; continue 'dispatch;
	}
	// 825B920C: 4BD07685  bl 0x822c0890
	ctx.lr = 0x825B9210;
	sub_822C0890(ctx, base);
	// 825B9210: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 825B9214: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 825B9218: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 825B921C: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 825B9220: C3EA08A4  lfs f31, 0x8a4(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2212 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 825B9224: 409A0088  bne cr6, 0x825b92ac
	if !ctx.cr[6].eq {
	pc = 0x825B92AC; continue 'dispatch;
	}
	// 825B9228: 396000FF  li r11, 0xff
	ctx.r[11].s64 = 255;
	// 825B922C: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 825B9230: 99610070  stb r11, 0x70(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[11].u8 ) };
	// 825B9234: 38610108  addi r3, r1, 0x108
	ctx.r[3].s64 = ctx.r[1].s64 + 264;
	// 825B9238: 99610071  stb r11, 0x71(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(113 as u32), ctx.r[11].u8 ) };
	// 825B923C: 99610072  stb r11, 0x72(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(114 as u32), ctx.r[11].u8 ) };
	// 825B9240: 99610073  stb r11, 0x73(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(115 as u32), ctx.r[11].u8 ) };
	// 825B9244: 4BF56285  bl 0x8250f4c8
	ctx.lr = 0x825B9248;
	sub_8250F4C8(ctx, base);
	// 825B9248: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B924C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825B9250: 388BFFFC  addi r4, r11, -4
	ctx.r[4].s64 = ctx.r[11].s64 + -4;
	// 825B9254: 409A0008  bne cr6, 0x825b925c
	if !ctx.cr[6].eq {
	pc = 0x825B925C; continue 'dispatch;
	}
	// 825B9258: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 825B925C: 386100F8  addi r3, r1, 0xf8
	ctx.r[3].s64 = ctx.r[1].s64 + 248;
	// 825B9260: 4BF4F751  bl 0x825089b0
	ctx.lr = 0x825B9264;
	sub_825089B0(ctx, base);
	// 825B9264: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B9268: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825B926C: 386BFF34  addi r3, r11, -0xcc
	ctx.r[3].s64 = ctx.r[11].s64 + -204;
	// 825B9270: 409A0008  bne cr6, 0x825b9278
	if !ctx.cr[6].eq {
	pc = 0x825B9278; continue 'dispatch;
	}
	// 825B9274: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 825B9278: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 825B927C: 3D408332  lis r10, -0x7cce
	ctx.r[10].s64 = -2093875200;
	// 825B9280: 38C10070  addi r6, r1, 0x70
	ctx.r[6].s64 = ctx.r[1].s64 + 112;
	// 825B9284: 38EABA80  addi r7, r10, -0x4580
	ctx.r[7].s64 = ctx.r[10].s64 + -17792;
	// 825B9288: 388100B0  addi r4, r1, 0xb0
	ctx.r[4].s64 = ctx.r[1].s64 + 176;
	// 825B928C: C02B9524  lfs f1, -0x6adc(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27356 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 825B9290: 4BD0E7D9  bl 0x822c7a68
	ctx.lr = 0x825B9294;
	sub_822C7A68(ctx, base);
	// 825B9294: 386100F8  addi r3, r1, 0xf8
	ctx.r[3].s64 = ctx.r[1].s64 + 248;
	// 825B9298: 488389F9  bl 0x82df1c90
	ctx.lr = 0x825B929C;
	sub_82DF1C90(ctx, base);
	// 825B929C: 38610108  addi r3, r1, 0x108
	ctx.r[3].s64 = ctx.r[1].s64 + 264;
	// 825B92A0: 488389F1  bl 0x82df1c90
	ctx.lr = 0x825B92A4;
	sub_82DF1C90(ctx, base);
	// 825B92A4: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 825B92A8: 480000A4  b 0x825b934c
	pc = 0x825B934C; continue 'dispatch;
	// 825B92AC: C01F0000  lfs f0, 0(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825B92B0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 825B92B4: EDA007B2  fmuls f13, f0, f30
	ctx.f[13].f64 = (((ctx.f[0].f64 * ctx.f[30].f64) as f32) as f64);
	// 825B92B8: C19B00C0  lfs f12, 0xc0(r27)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(192 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 825B92BC: C00B9524  lfs f0, -0x6adc(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27356 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825B92C0: EC0D603C  fnmsubs f0, f13, f0, f12
	ctx.f[0].f64 = -(((ctx.f[13].f64 * ctx.f[0].f64 - ctx.f[12].f64) as f32) as f64);
	// 825B92C4: D01B00C0  stfs f0, 0xc0(r27)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(192 as u32), tmp.u32 ) };
	// 825B92C8: FF00F800  fcmpu cr6, f0, f31
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[31].f64);
	// 825B92CC: 41990008  bgt cr6, 0x825b92d4
	if ctx.cr[6].gt {
	pc = 0x825B92D4; continue 'dispatch;
	}
	// 825B92D0: D3DB00C0  stfs f30, 0xc0(r27)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(192 as u32), tmp.u32 ) };
	// 825B92D4: 396000FF  li r11, 0xff
	ctx.r[11].s64 = 255;
	// 825B92D8: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 825B92DC: 99610050  stb r11, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u8 ) };
	// 825B92E0: 386100D0  addi r3, r1, 0xd0
	ctx.r[3].s64 = ctx.r[1].s64 + 208;
	// 825B92E4: 99610051  stb r11, 0x51(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(81 as u32), ctx.r[11].u8 ) };
	// 825B92E8: 99610052  stb r11, 0x52(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(82 as u32), ctx.r[11].u8 ) };
	// 825B92EC: 99610053  stb r11, 0x53(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(83 as u32), ctx.r[11].u8 ) };
	// 825B92F0: 4BF561D9  bl 0x8250f4c8
	ctx.lr = 0x825B92F4;
	sub_8250F4C8(ctx, base);
	// 825B92F4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B92F8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825B92FC: 388BFFFC  addi r4, r11, -4
	ctx.r[4].s64 = ctx.r[11].s64 + -4;
	// 825B9300: 409A0008  bne cr6, 0x825b9308
	if !ctx.cr[6].eq {
	pc = 0x825B9308; continue 'dispatch;
	}
	// 825B9304: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 825B9308: 38610118  addi r3, r1, 0x118
	ctx.r[3].s64 = ctx.r[1].s64 + 280;
	// 825B930C: 4BF4F6A5  bl 0x825089b0
	ctx.lr = 0x825B9310;
	sub_825089B0(ctx, base);
	// 825B9310: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B9314: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825B9318: 386BFF34  addi r3, r11, -0xcc
	ctx.r[3].s64 = ctx.r[11].s64 + -204;
	// 825B931C: 409A0008  bne cr6, 0x825b9324
	if !ctx.cr[6].eq {
	pc = 0x825B9324; continue 'dispatch;
	}
	// 825B9320: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 825B9324: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 825B9328: C03B00C0  lfs f1, 0xc0(r27)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(192 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 825B932C: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 825B9330: 38EBBA80  addi r7, r11, -0x4580
	ctx.r[7].s64 = ctx.r[11].s64 + -17792;
	// 825B9334: 388100B0  addi r4, r1, 0xb0
	ctx.r[4].s64 = ctx.r[1].s64 + 176;
	// 825B9338: 4BD0E731  bl 0x822c7a68
	ctx.lr = 0x825B933C;
	sub_822C7A68(ctx, base);
	// 825B933C: 38610118  addi r3, r1, 0x118
	ctx.r[3].s64 = ctx.r[1].s64 + 280;
	// 825B9340: 48838951  bl 0x82df1c90
	ctx.lr = 0x825B9344;
	sub_82DF1C90(ctx, base);
	// 825B9344: 386100D0  addi r3, r1, 0xd0
	ctx.r[3].s64 = ctx.r[1].s64 + 208;
	// 825B9348: 48838949  bl 0x82df1c90
	ctx.lr = 0x825B934C;
	sub_82DF1C90(ctx, base);
	// 825B934C: 897D0000  lbz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B9350: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 825B9354: 4082000C  bne 0x825b9360
	if !ctx.cr[0].eq {
	pc = 0x825B9360; continue 'dispatch;
	}
	// 825B9358: 57CB063F  clrlwi. r11, r30, 0x18
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 825B935C: 41820218  beq 0x825b9574
	if ctx.cr[0].eq {
	pc = 0x825B9574; continue 'dispatch;
	}
	// 825B9360: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 825B9364: C3DC0000  lfs f30, 0(r28)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 825B9368: 394100B0  addi r10, r1, 0xb0
	ctx.r[10].s64 = ctx.r[1].s64 + 176;
	// 825B936C: D3E1006C  stfs f31, 0x6c(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), tmp.u32 ) };
	// 825B9370: 39210060  addi r9, r1, 0x60
	ctx.r[9].s64 = ctx.r[1].s64 + 96;
	// 825B9374: D3C10060  stfs f30, 0x60(r1)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), tmp.u32 ) };
	// 825B9378: D3E10064  stfs f31, 0x64(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), tmp.u32 ) };
	// 825B937C: 39010180  addi r8, r1, 0x180
	ctx.r[8].s64 = ctx.r[1].s64 + 384;
	// 825B9380: D3E10068  stfs f31, 0x68(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), tmp.u32 ) };
	// 825B9384: 38E101D0  addi r7, r1, 0x1d0
	ctx.r[7].s64 = ctx.r[1].s64 + 464;
	// 825B9388: 13C058C7  vcmpequd (lvx128) v30, v0, v11
	tmp.u32 = ctx.r[11].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[62] using VectorMaskL[(tmp.u32 & 0xF)]
	// 825B938C: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 825B9390: 13E050C7  vcmpequd (lvx128) v31, v0, v10
	tmp.u32 = ctx.r[10].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 825B9394: 386100E0  addi r3, r1, 0xe0
	ctx.r[3].s64 = ctx.r[1].s64 + 224;
	// 825B9398: D3C10060  stfs f30, 0x60(r1)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), tmp.u32 ) };
	// 825B939C: D3E10064  stfs f31, 0x64(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), tmp.u32 ) };
	// 825B93A0: D3E10068  stfs f31, 0x68(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), tmp.u32 ) };
	// 825B93A4: D3E1006C  stfs f31, 0x6c(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), tmp.u32 ) };
	// 825B93A8: 13A048C7  vcmpequd (lvx128) v29, v0, v9
	tmp.u32 = ctx.r[9].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[61] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B9830(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825B9830 size=16
    let mut pc: u32 = 0x825B9830;
    'dispatch: loop {
        match pc {
            0x825B9830 => {
    //   block [0x825B9830..0x825B9840)
	// 825B9830: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 825B9834: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 825B9838: 5563B7FE  rlwinm r3, r11, 0x16, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x000003FFu64;
	// 825B983C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B9840(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825B9840 size=16
    let mut pc: u32 = 0x825B9840;
    'dispatch: loop {
        match pc {
            0x825B9840 => {
    //   block [0x825B9840..0x825B9850)
	// 825B9840: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 825B9844: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 825B9848: 5563D7FE  rlwinm r3, r11, 0x1a, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000003Fu64;
	// 825B984C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B9850(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825B9850 size=16
    let mut pc: u32 = 0x825B9850;
    'dispatch: loop {
        match pc {
            0x825B9850 => {
    //   block [0x825B9850..0x825B9860)
	// 825B9850: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 825B9854: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 825B9858: 5563CFFE  rlwinm r3, r11, 0x19, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000007Fu64;
	// 825B985C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B9860(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825B9860 size=16
    let mut pc: u32 = 0x825B9860;
    'dispatch: loop {
        match pc {
            0x825B9860 => {
    //   block [0x825B9860..0x825B9870)
	// 825B9860: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 825B9864: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 825B9868: 5563BFFE  rlwinm r3, r11, 0x17, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x000001FFu64;
	// 825B986C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B9870(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825B9870 size=16
    let mut pc: u32 = 0x825B9870;
    'dispatch: loop {
        match pc {
            0x825B9870 => {
    //   block [0x825B9870..0x825B9880)
	// 825B9870: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 825B9874: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 825B9878: 5563C7FE  rlwinm r3, r11, 0x18, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 825B987C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B9880(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825B9880 size=16
    let mut pc: u32 = 0x825B9880;
    'dispatch: loop {
        match pc {
            0x825B9880 => {
    //   block [0x825B9880..0x825B9890)
	// 825B9880: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 825B9884: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 825B9888: 556307FE  clrlwi r3, r11, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	// 825B988C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B9890(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825B9890 size=16
    let mut pc: u32 = 0x825B9890;
    'dispatch: loop {
        match pc {
            0x825B9890 => {
    //   block [0x825B9890..0x825B98A0)
	// 825B9890: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 825B9894: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 825B9898: 5563FFFE  rlwinm r3, r11, 0x1f, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	// 825B989C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B98A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825B98A0 size=16
    let mut pc: u32 = 0x825B98A0;
    'dispatch: loop {
        match pc {
            0x825B98A0 => {
    //   block [0x825B98A0..0x825B98B0)
	// 825B98A0: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 825B98A4: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 825B98A8: 5563EFFE  rlwinm r3, r11, 0x1d, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x00000007u64;
	// 825B98AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B98B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825B98B0 size=16
    let mut pc: u32 = 0x825B98B0;
    'dispatch: loop {
        match pc {
            0x825B98B0 => {
    //   block [0x825B98B0..0x825B98C0)
	// 825B98B0: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 825B98B4: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 825B98B8: 5563E7FE  rlwinm r3, r11, 0x1c, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000000Fu64;
	// 825B98BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B98C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825B98C0 size=16
    let mut pc: u32 = 0x825B98C0;
    'dispatch: loop {
        match pc {
            0x825B98C0 => {
    //   block [0x825B98C0..0x825B98D0)
	// 825B98C0: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 825B98C4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 825B98C8: 806B0038  lwz r3, 0x38(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) } as u64;
	// 825B98CC: 4BFF841C  b 0x825b1ce8
	sub_825B1CE8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B98D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825B98D0 size=12
    let mut pc: u32 = 0x825B98D0;
    'dispatch: loop {
        match pc {
            0x825B98D0 => {
    //   block [0x825B98D0..0x825B98DC)
	// 825B98D0: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 825B98D4: 806B003C  lwz r3, 0x3c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 825B98D8: 4BFF8D78  b 0x825b2650
	sub_825B2650(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B98E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825B98E0 size=12
    let mut pc: u32 = 0x825B98E0;
    'dispatch: loop {
        match pc {
            0x825B98E0 => {
    //   block [0x825B98E0..0x825B98EC)
	// 825B98E0: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 825B98E4: 806B003C  lwz r3, 0x3c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 825B98E8: 4BFF8A08  b 0x825b22f0
	sub_825B22F0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B98F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825B98F0 size=12
    let mut pc: u32 = 0x825B98F0;
    'dispatch: loop {
        match pc {
            0x825B98F0 => {
    //   block [0x825B98F0..0x825B98FC)
	// 825B98F0: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 825B98F4: 806B003C  lwz r3, 0x3c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 825B98F8: 4BFF8450  b 0x825b1d48
	sub_825B1D48(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B9900(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825B9900 size=12
    let mut pc: u32 = 0x825B9900;
    'dispatch: loop {
        match pc {
            0x825B9900 => {
    //   block [0x825B9900..0x825B990C)
	// 825B9900: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 825B9904: 806B003C  lwz r3, 0x3c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 825B9908: 4BFF8450  b 0x825b1d58
	sub_825B1D58(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B9910(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825B9910 size=56
    let mut pc: u32 = 0x825B9910;
    'dispatch: loop {
        match pc {
            0x825B9910 => {
    //   block [0x825B9910..0x825B9948)
	// 825B9910: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B9914: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825B9918: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825B991C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B9920: 81640004  lwz r11, 4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 825B9924: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825B9928: 808B003C  lwz r4, 0x3c(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 825B992C: 4BFF843D  bl 0x825b1d68
	ctx.lr = 0x825B9930;
	sub_825B1D68(ctx, base);
	// 825B9930: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B9934: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825B9938: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825B993C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825B9940: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825B9944: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B9948(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825B9948 size=56
    let mut pc: u32 = 0x825B9948;
    'dispatch: loop {
        match pc {
            0x825B9948 => {
    //   block [0x825B9948..0x825B9980)
	// 825B9948: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B994C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825B9950: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825B9954: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B9958: 81640004  lwz r11, 4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 825B995C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825B9960: 808B003C  lwz r4, 0x3c(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 825B9964: 4BFF849D  bl 0x825b1e00
	ctx.lr = 0x825B9968;
	sub_825B1E00(ctx, base);
	// 825B9968: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B996C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825B9970: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825B9974: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825B9978: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825B997C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B9980(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825B9980 size=12
    let mut pc: u32 = 0x825B9980;
    'dispatch: loop {
        match pc {
            0x825B9980 => {
    //   block [0x825B9980..0x825B998C)
	// 825B9980: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 825B9984: 806B003C  lwz r3, 0x3c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 825B9988: 4BFF8488  b 0x825b1e10
	sub_825B1E10(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B9990(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825B9990 size=32
    let mut pc: u32 = 0x825B9990;
    'dispatch: loop {
        match pc {
            0x825B9990 => {
    //   block [0x825B9990..0x825B99B0)
	// 825B9990: 8163001C  lwz r11, 0x1c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 825B9994: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825B9998: 419A0018  beq cr6, 0x825b99b0
	if ctx.cr[6].eq {
		sub_825B99B0(ctx, base);
		return;
	}
	// 825B999C: 5563003E  slwi r3, r11, 0
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 825B99A0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B99A4: 816B0060  lwz r11, 0x60(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(96 as u32) ) } as u64;
	// 825B99A8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825B99AC: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B99B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825B99B0 size=8
    let mut pc: u32 = 0x825B99B0;
    'dispatch: loop {
        match pc {
            0x825B99B0 => {
    //   block [0x825B99B0..0x825B99B8)
	// 825B99B0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 825B99B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B99B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x825B99B8 size=20
    let mut pc: u32 = 0x825B99B8;
    'dispatch: loop {
        match pc {
            0x825B99B8 => {
    //   block [0x825B99B8..0x825B99CC)
	// 825B99B8: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 825B99BC: 396B6910  addi r11, r11, 0x6910
	ctx.r[11].s64 = ctx.r[11].s64 + 26896;
	// 825B99C0: 13E058C7  vcmpequd (lvx128) v31, v0, v11
	tmp.u32 = ctx.r[11].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B99D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x825B99D0 size=48
    let mut pc: u32 = 0x825B99D0;
    'dispatch: loop {
        match pc {
            0x825B99D0 => {
    //   block [0x825B99D0..0x825B9A00)
	// 825B99D0: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 825B99D4: D023000C  stfs f1, 0xc(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 825B99D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825B99DC: D0430010  stfs f2, 0x10(r3)
	tmp.f32 = (ctx.f[2].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 825B99E0: 394AC44C  addi r10, r10, -0x3bb4
	ctx.r[10].s64 = ctx.r[10].s64 + -15284;
	// 825B99E4: 90830004  stw r4, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 825B99E8: 90A30008  stw r5, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[5].u32 ) };
	// 825B99EC: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 825B99F0: 99030014  stb r8, 0x14(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[8].u8 ) };
	// 825B99F4: 91630018  stw r11, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 825B99F8: 9163001C  stw r11, 0x1c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 825B99FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B9A00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825B9A00 size=8
    let mut pc: u32 = 0x825B9A00;
    'dispatch: loop {
        match pc {
            0x825B9A00 => {
    //   block [0x825B9A00..0x825B9A08)
	// 825B9A00: 9083001C  stw r4, 0x1c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[4].u32 ) };
	// 825B9A04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B9A08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825B9A08 size=112
    let mut pc: u32 = 0x825B9A08;
    'dispatch: loop {
        match pc {
            0x825B9A08 => {
    //   block [0x825B9A08..0x825B9A78)
	// 825B9A08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B9A0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825B9A10: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825B9A14: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825B9A18: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B9A1C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 825B9A20: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825B9A24: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 825B9A28: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 825B9A2C: 808B003C  lwz r4, 0x3c(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 825B9A30: 4BFF8749  bl 0x825b2178
	ctx.lr = 0x825B9A34;
	sub_825B2178(ctx, base);
	// 825B9A34: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 825B9A38: 809E0008  lwz r4, 8(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 825B9A3C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 825B9A40: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 825B9A44: 4BFFC955  bl 0x825b6398
	ctx.lr = 0x825B9A48;
	sub_825B6398(ctx, base);
	// 825B9A48: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 825B9A4C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825B9A50: 419A0008  beq cr6, 0x825b9a58
	if ctx.cr[6].eq {
	pc = 0x825B9A58; continue 'dispatch;
	}
	// 825B9A54: 4BD06E3D  bl 0x822c0890
	ctx.lr = 0x825B9A58;
	sub_822C0890(ctx, base);
	// 825B9A58: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B9A5C: 488399CD  bl 0x82df3428
	ctx.lr = 0x825B9A60;
	sub_82DF3428(ctx, base);
	// 825B9A60: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825B9A64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825B9A68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825B9A6C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825B9A70: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825B9A74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B9A78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825B9A78 size=120
    let mut pc: u32 = 0x825B9A78;
    'dispatch: loop {
        match pc {
            0x825B9A78 => {
    //   block [0x825B9A78..0x825B9AF0)
	// 825B9A78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B9A7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825B9A80: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825B9A84: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B9A88: 3D608336  lis r11, -0x7cca
	ctx.r[11].s64 = -2093613056;
	// 825B9A8C: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 825B9A90: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825B9A94: 816B853C  lwz r11, -0x7ac4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-31428 as u32) ) } as u64;
	// 825B9A98: 7F055840  cmplw cr6, r5, r11
	ctx.cr[6].compare_u32(ctx.r[5].u32, ctx.r[11].u32, &mut ctx.xer);
	// 825B9A9C: 419A0040  beq cr6, 0x825b9adc
	if ctx.cr[6].eq {
	pc = 0x825B9ADC; continue 'dispatch;
	}
	// 825B9AA0: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 825B9AA4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825B9AA8: 808B003C  lwz r4, 0x3c(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 825B9AAC: 4BFF85ED  bl 0x825b2098
	ctx.lr = 0x825B9AB0;
	sub_825B2098(ctx, base);
	// 825B9AB0: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 825B9AB4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825B9AB8: 419A0014  beq cr6, 0x825b9acc
	if ctx.cr[6].eq {
	pc = 0x825B9ACC; continue 'dispatch;
	}
	// 825B9ABC: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 825B9AC0: 388B0010  addi r4, r11, 0x10
	ctx.r[4].s64 = ctx.r[11].s64 + 16;
	// 825B9AC4: 806A003C  lwz r3, 0x3c(r10)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(60 as u32) ) } as u64;
	// 825B9AC8: 4BFF8349  bl 0x825b1e10
	ctx.lr = 0x825B9ACC;
	sub_825B1E10(ctx, base);
	// 825B9ACC: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 825B9AD0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825B9AD4: 419A0008  beq cr6, 0x825b9adc
	if ctx.cr[6].eq {
	pc = 0x825B9ADC; continue 'dispatch;
	}
	// 825B9AD8: 4BD06DB9  bl 0x822c0890
	ctx.lr = 0x825B9ADC;
	sub_822C0890(ctx, base);
	// 825B9ADC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825B9AE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825B9AE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825B9AE8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825B9AEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B9AF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825B9AF0 size=12
    let mut pc: u32 = 0x825B9AF0;
    'dispatch: loop {
        match pc {
            0x825B9AF0 => {
    //   block [0x825B9AF0..0x825B9AFC)
	// 825B9AF0: 80640004  lwz r3, 4(r4)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 825B9AF4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825B9AF8: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B9AFC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825B9AFC size=8
    let mut pc: u32 = 0x825B9AFC;
    'dispatch: loop {
        match pc {
            0x825B9AFC => {
    //   block [0x825B9AFC..0x825B9B04)
	// 825B9AFC: 4BD06D94  b 0x822c0890
	sub_822C0890(ctx, base);
	return;
	// 825B9B00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B9B08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825B9B08 size=180
    let mut pc: u32 = 0x825B9B08;
    'dispatch: loop {
        match pc {
            0x825B9B08 => {
    //   block [0x825B9B08..0x825B9BBC)
	// 825B9B08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B9B0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825B9B10: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825B9B14: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825B9B18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B9B1C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825B9B20: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825B9B24: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825B9B28: 4BFFE0D1  bl 0x825b7bf8
	ctx.lr = 0x825B9B2C;
	sub_825B7BF8(ctx, base);
	// 825B9B2C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 825B9B30: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 825B9B34: 4BFFD6A5  bl 0x825b71d8
	ctx.lr = 0x825B9B38;
	sub_825B71D8(ctx, base);
	// 825B9B38: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 825B9B3C: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 825B9B40: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825B9B44: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 825B9B48: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 825B9B4C: 419A0024  beq cr6, 0x825b9b70
	if ctx.cr[6].eq {
	pc = 0x825B9B70; continue 'dispatch;
	}
	// 825B9B50: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 825B9B54: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 825B9B58: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 825B9B5C: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 825B9B60: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 825B9B64: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 825B9B68: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 825B9B6C: 4082FFE8  bne 0x825b9b54
	if !ctx.cr[0].eq {
	pc = 0x825B9B54; continue 'dispatch;
	}
	// 825B9B70: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 825B9B74: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 825B9B78: 7D655B78  mr r5, r11
	ctx.r[5].u64 = ctx.r[11].u64;
	// 825B9B7C: 806B003C  lwz r3, 0x3c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 825B9B80: 4BFF8C51  bl 0x825b27d0
	ctx.lr = 0x825B9B84;
	sub_825B27D0(ctx, base);
	// 825B9B84: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 825B9B88: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825B9B8C: 419A0008  beq cr6, 0x825b9b94
	if ctx.cr[6].eq {
	pc = 0x825B9B94; continue 'dispatch;
	}
	// 825B9B90: 4BD06D01  bl 0x822c0890
	ctx.lr = 0x825B9B94;
	sub_822C0890(ctx, base);
	// 825B9B94: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 825B9B98: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825B9B9C: 419A0008  beq cr6, 0x825b9ba4
	if ctx.cr[6].eq {
	pc = 0x825B9BA4; continue 'dispatch;
	}
	// 825B9BA0: 4BD06CF1  bl 0x822c0890
	ctx.lr = 0x825B9BA4;
	sub_822C0890(ctx, base);
	// 825B9BA4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825B9BA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825B9BAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825B9BB0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825B9BB4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825B9BB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B9BC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825B9BC0 size=180
    let mut pc: u32 = 0x825B9BC0;
    'dispatch: loop {
        match pc {
            0x825B9BC0 => {
    //   block [0x825B9BC0..0x825B9C74)
	// 825B9BC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B9BC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825B9BC8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825B9BCC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825B9BD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B9BD4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825B9BD8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825B9BDC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825B9BE0: 4BFFBBF1  bl 0x825b57d0
	ctx.lr = 0x825B9BE4;
	sub_825B57D0(ctx, base);
	// 825B9BE4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 825B9BE8: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 825B9BEC: 4BFFB255  bl 0x825b4e40
	ctx.lr = 0x825B9BF0;
	sub_825B4E40(ctx, base);
	// 825B9BF0: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 825B9BF4: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 825B9BF8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825B9BFC: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 825B9C00: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 825B9C04: 419A0024  beq cr6, 0x825b9c28
	if ctx.cr[6].eq {
	pc = 0x825B9C28; continue 'dispatch;
	}
	// 825B9C08: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 825B9C0C: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 825B9C10: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 825B9C14: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 825B9C18: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 825B9C1C: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 825B9C20: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 825B9C24: 4082FFE8  bne 0x825b9c0c
	if !ctx.cr[0].eq {
	pc = 0x825B9C0C; continue 'dispatch;
	}
	// 825B9C28: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 825B9C2C: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 825B9C30: 7D655B78  mr r5, r11
	ctx.r[5].u64 = ctx.r[11].u64;
	// 825B9C34: 806B003C  lwz r3, 0x3c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 825B9C38: 4BFF8B99  bl 0x825b27d0
	ctx.lr = 0x825B9C3C;
	sub_825B27D0(ctx, base);
	// 825B9C3C: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 825B9C40: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825B9C44: 419A0008  beq cr6, 0x825b9c4c
	if ctx.cr[6].eq {
	pc = 0x825B9C4C; continue 'dispatch;
	}
	// 825B9C48: 4BD06C49  bl 0x822c0890
	ctx.lr = 0x825B9C4C;
	sub_822C0890(ctx, base);
	// 825B9C4C: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 825B9C50: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825B9C54: 419A0008  beq cr6, 0x825b9c5c
	if ctx.cr[6].eq {
	pc = 0x825B9C5C; continue 'dispatch;
	}
	// 825B9C58: 4BD06C39  bl 0x822c0890
	ctx.lr = 0x825B9C5C;
	sub_822C0890(ctx, base);
	// 825B9C5C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825B9C60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825B9C64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825B9C68: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825B9C6C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825B9C70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B9C78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825B9C78 size=112
    let mut pc: u32 = 0x825B9C78;
    'dispatch: loop {
        match pc {
            0x825B9C78 => {
    //   block [0x825B9C78..0x825B9CE8)
	// 825B9C78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B9C7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825B9C80: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825B9C84: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B9C88: 81640004  lwz r11, 4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 825B9C8C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825B9C90: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825B9C94: 808B003C  lwz r4, 0x3c(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 825B9C98: 4BFF8401  bl 0x825b2098
	ctx.lr = 0x825B9C9C;
	sub_825B2098(ctx, base);
	// 825B9C9C: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 825B9CA0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B9CA4: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 825B9CA8: 419A000C  beq cr6, 0x825b9cb4
	if ctx.cr[6].eq {
	pc = 0x825B9CB4; continue 'dispatch;
	}
	// 825B9CAC: 48839F55  bl 0x82df3c00
	ctx.lr = 0x825B9CB0;
	sub_82DF3C00(ctx, base);
	// 825B9CB0: 48000010  b 0x825b9cc0
	pc = 0x825B9CC0; continue 'dispatch;
	// 825B9CB4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 825B9CB8: 388B9BC9  addi r4, r11, -0x6437
	ctx.r[4].s64 = ctx.r[11].s64 + -25655;
	// 825B9CBC: 48839D4D  bl 0x82df3a08
	ctx.lr = 0x825B9CC0;
	sub_82DF3A08(ctx, base);
	// 825B9CC0: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 825B9CC4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825B9CC8: 419A0008  beq cr6, 0x825b9cd0
	if ctx.cr[6].eq {
	pc = 0x825B9CD0; continue 'dispatch;
	}
	// 825B9CCC: 4BD06BC5  bl 0x822c0890
	ctx.lr = 0x825B9CD0;
	sub_822C0890(ctx, base);
	// 825B9CD0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B9CD4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825B9CD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825B9CDC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825B9CE0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825B9CE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B9CE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825B9CE8 size=176
    let mut pc: u32 = 0x825B9CE8;
    'dispatch: loop {
        match pc {
            0x825B9CE8 => {
    //   block [0x825B9CE8..0x825B9D98)
	// 825B9CE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B9CEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825B9CF0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825B9CF4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825B9CF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B9CFC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825B9D00: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825B9D04: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825B9D08: 4BFFAD89  bl 0x825b4a90
	ctx.lr = 0x825B9D0C;
	sub_825B4A90(ctx, base);
	// 825B9D0C: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 825B9D10: 93CB0110  stw r30, 0x110(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(272 as u32), ctx.r[30].u32 ) };
	// 825B9D14: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 825B9D18: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 825B9D1C: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 825B9D20: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825B9D24: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 825B9D28: 419A0024  beq cr6, 0x825b9d4c
	if ctx.cr[6].eq {
	pc = 0x825B9D4C; continue 'dispatch;
	}
	// 825B9D2C: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 825B9D30: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 825B9D34: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 825B9D38: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 825B9D3C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 825B9D40: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 825B9D44: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 825B9D48: 4082FFE8  bne 0x825b9d30
	if !ctx.cr[0].eq {
	pc = 0x825B9D30; continue 'dispatch;
	}
	// 825B9D4C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 825B9D50: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 825B9D54: 7D655B78  mr r5, r11
	ctx.r[5].u64 = ctx.r[11].u64;
	// 825B9D58: 806B003C  lwz r3, 0x3c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 825B9D5C: 4BFF8A75  bl 0x825b27d0
	ctx.lr = 0x825B9D60;
	sub_825B27D0(ctx, base);
	// 825B9D60: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 825B9D64: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825B9D68: 419A0008  beq cr6, 0x825b9d70
	if ctx.cr[6].eq {
	pc = 0x825B9D70; continue 'dispatch;
	}
	// 825B9D6C: 4BD06B25  bl 0x822c0890
	ctx.lr = 0x825B9D70;
	sub_822C0890(ctx, base);
	// 825B9D70: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 825B9D74: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825B9D78: 419A0008  beq cr6, 0x825b9d80
	if ctx.cr[6].eq {
	pc = 0x825B9D80; continue 'dispatch;
	}
	// 825B9D7C: 4BD06B15  bl 0x822c0890
	ctx.lr = 0x825B9D80;
	sub_822C0890(ctx, base);
	// 825B9D80: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825B9D84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825B9D88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825B9D8C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825B9D90: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825B9D94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B9D98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825B9D98 size=172
    let mut pc: u32 = 0x825B9D98;
    'dispatch: loop {
        match pc {
            0x825B9D98 => {
    //   block [0x825B9D98..0x825B9E44)
	// 825B9D98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B9D9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825B9DA0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825B9DA4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825B9DA8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B9DAC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 825B9DB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825B9DB4: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 825B9DB8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 825B9DBC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825B9DC0: 4BD06B79  bl 0x822c0938
	ctx.lr = 0x825B9DC4;
	sub_822C0938(ctx, base);
	// 825B9DC4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 825B9DC8: 41820028  beq 0x825b9df0
	if ctx.cr[0].eq {
	pc = 0x825B9DF0; continue 'dispatch;
	}
	// 825B9DCC: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 825B9DD0: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 825B9DD4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825B9DD8: 392BC4D0  addi r9, r11, -0x3b30
	ctx.r[9].s64 = ctx.r[11].s64 + -15152;
	// 825B9DDC: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 825B9DE0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 825B9DE4: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 825B9DE8: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 825B9DEC: 48000008  b 0x825b9df4
	pc = 0x825B9DF4; continue 'dispatch;
	// 825B9DF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825B9DF4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825B9DF8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825B9DFC: 409A002C  bne cr6, 0x825b9e28
	if !ctx.cr[6].eq {
	pc = 0x825B9E28; continue 'dispatch;
	}
	// 825B9E00: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825B9E04: 4BD06465  bl 0x822c0268
	ctx.lr = 0x825B9E08;
	sub_822C0268(ctx, base);
	// 825B9E08: 3D608329  lis r11, -0x7cd7
	ctx.r[11].s64 = -2094465024;
	// 825B9E0C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 825B9E10: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825B9E14: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 825B9E18: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 825B9E1C: 816B9750  lwz r11, -0x68b0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-26800 as u32) ) } as u64;
	// 825B9E20: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825B9E24: 4BD061DD  bl 0x822c0000
	ctx.lr = 0x825B9E28;
	sub_822C0000(ctx, base);
	// 825B9E28: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 825B9E2C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825B9E30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825B9E34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825B9E38: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825B9E3C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825B9E40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B9E48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825B9E48 size=100
    let mut pc: u32 = 0x825B9E48;
    'dispatch: loop {
        match pc {
            0x825B9E48 => {
    //   block [0x825B9E48..0x825B9EAC)
	// 825B9E48: 3963000C  addi r11, r3, 0xc
	ctx.r[11].s64 = ctx.r[3].s64 + 12;
	// 825B9E4C: 90830000  stw r4, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 825B9E50: 90A30004  stw r5, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 825B9E54: 90C30008  stw r6, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[6].u32 ) };
	// 825B9E58: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 825B9E5C: 81670000  lwz r11, 0(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 825B9E60: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 825B9E64: 81670004  lwz r11, 4(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(4 as u32) ) } as u64;
	// 825B9E68: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 825B9E6C: 81670008  lwz r11, 8(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(8 as u32) ) } as u64;
	// 825B9E70: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825B9E74: 91630014  stw r11, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 825B9E78: 419A0024  beq cr6, 0x825b9e9c
	if ctx.cr[6].eq {
	pc = 0x825B9E9C; continue 'dispatch;
	}
	// 825B9E7C: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 825B9E80: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 825B9E84: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 825B9E88: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 825B9E8C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 825B9E90: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 825B9E94: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 825B9E98: 4082FFE8  bne 0x825b9e80
	if !ctx.cr[0].eq {
	pc = 0x825B9E80; continue 'dispatch;
	}
	// 825B9E9C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825B9EA0: 99030018  stb r8, 0x18(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[8].u8 ) };
	// 825B9EA4: 99630019  stb r11, 0x19(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(25 as u32), ctx.r[11].u8 ) };
	// 825B9EA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B9EB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x825B9EB0 size=72
    let mut pc: u32 = 0x825B9EB0;
    'dispatch: loop {
        match pc {
            0x825B9EB0 => {
    //   block [0x825B9EB0..0x825B9EF8)
	// 825B9EB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B9EB4: 48BEE2B9  bl 0x831a816c
	ctx.lr = 0x825B9EB8;
	sub_831A8130(ctx, base);
	// 825B9EB8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B9EBC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825B9EC0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825B9EC4: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 825B9EC8: 4BD2A311  bl 0x822e41d8
	ctx.lr = 0x825B9ECC;
	sub_822E41D8(ctx, base);
	// 825B9ECC: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 825B9ED0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 825B9ED4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 825B9ED8: C02B08A8  lfs f1, 0x8a8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2216 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 825B9EDC: 4BD165ED  bl 0x822d04c8
	ctx.lr = 0x825B9EE0;
	sub_822D04C8(ctx, base);
	// 825B9EE0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 825B9EE4: 4BD1604D  bl 0x822cff30
	ctx.lr = 0x825B9EE8;
	sub_822CFF30(ctx, base);
	// 825B9EE8: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 825B9EEC: 4BD2A77D  bl 0x822e4668
	ctx.lr = 0x825B9EF0;
	sub_822E4668(ctx, base);
	// 825B9EF0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825B9EF4: 48BEE2C8  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B9EF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825B9EF8 size=8
    let mut pc: u32 = 0x825B9EF8;
    'dispatch: loop {
        match pc {
            0x825B9EF8 => {
    //   block [0x825B9EF8..0x825B9F00)
	// 825B9EF8: 80630004  lwz r3, 4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 825B9EFC: 4BD2A54C  b 0x822e4448
	sub_822E4448(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B9F00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825B9F00 size=80
    let mut pc: u32 = 0x825B9F00;
    'dispatch: loop {
        match pc {
            0x825B9F00 => {
    //   block [0x825B9F00..0x825B9F50)
	// 825B9F00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B9F04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825B9F08: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825B9F0C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B9F10: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825B9F14: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 825B9F18: 396BC4E4  addi r11, r11, -0x3b1c
	ctx.r[11].s64 = ctx.r[11].s64 + -15132;
	// 825B9F1C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825B9F20: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 825B9F24: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825B9F28: 419A0008  beq cr6, 0x825b9f30
	if ctx.cr[6].eq {
	pc = 0x825B9F30; continue 'dispatch;
	}
	// 825B9F2C: 4BD06965  bl 0x822c0890
	ctx.lr = 0x825B9F30;
	sub_822C0890(ctx, base);
	// 825B9F30: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 825B9F34: 396BB9DC  addi r11, r11, -0x4624
	ctx.r[11].s64 = ctx.r[11].s64 + -17956;
	// 825B9F38: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825B9F3C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825B9F40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825B9F44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825B9F48: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825B9F4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825B9F50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x825B9F50 size=204
    let mut pc: u32 = 0x825B9F50;
    'dispatch: loop {
        match pc {
            0x825B9F50 => {
    //   block [0x825B9F50..0x825BA01C)
	// 825B9F50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825B9F54: 48BEE215  bl 0x831a8168
	ctx.lr = 0x825B9F58;
	sub_831A8130(ctx, base);
	// 825B9F58: DBE1FFD0  stfd f31, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[31].u64 ) };
	// 825B9F5C: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825B9F60: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825B9F64: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 825B9F68: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825B9F6C: 7D044378  mr r4, r8
	ctx.r[4].u64 = ctx.r[8].u64;
	// 825B9F70: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 825B9F74: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 825B9F78: 7CFC3B78  mr r28, r7
	ctx.r[28].u64 = ctx.r[7].u64;
	// 825B9F7C: 488C2255  bl 0x82e7c1d0
	ctx.lr = 0x825B9F80;
	sub_82E7C1D0(ctx, base);
	// 825B9F80: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 825B9F84: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 825B9F88: 38A10090  addi r5, r1, 0x90
	ctx.r[5].s64 = ctx.r[1].s64 + 144;
	// 825B9F8C: 388100A0  addi r4, r1, 0xa0
	ctx.r[4].s64 = ctx.r[1].s64 + 160;
	// 825B9F90: 4BD2A4B9  bl 0x822e4448
	ctx.lr = 0x825B9F94;
	sub_822E4448(ctx, base);
	// 825B9F94: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 825B9F98: 39410070  addi r10, r1, 0x70
	ctx.r[10].s64 = ctx.r[1].s64 + 112;
	// 825B9F9C: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 825B9FA0: 39210050  addi r9, r1, 0x50
	ctx.r[9].s64 = ctx.r[1].s64 + 80;
	// 825B9FA4: 39010080  addi r8, r1, 0x80
	ctx.r[8].s64 = ctx.r[1].s64 + 128;
	// 825B9FA8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825B9FAC: C00B08A4  lfs f0, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825B9FB0: 13E050C7  vcmpequd (lvx128) v31, v0, v10
	tmp.u32 = ctx.r[10].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 825B9FB4: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 825B9FB8: 13C048C7  vcmpequd (lvx128) v30, v0, v9
	tmp.u32 = ctx.r[9].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[62] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825BA020(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825BA020 size=40
    let mut pc: u32 = 0x825BA020;
    'dispatch: loop {
        match pc {
            0x825BA020 => {
    //   block [0x825BA020..0x825BA048)
	// 825BA020: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 825BA024: 39630004  addi r11, r3, 4
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	// 825BA028: 396AC4E4  addi r11, r10, -0x3b1c
	ctx.r[11].s64 = ctx.r[10].s64 + -15132;
	// 825BA02C: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825BA030: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 825BA034: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 825BA038: 81640004  lwz r11, 4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 825BA03C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825BA040: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 825BA044: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


