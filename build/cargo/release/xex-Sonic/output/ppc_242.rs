pub fn sub_8303B09C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8303B09C size=72
    let mut pc: u32 = 0x8303B09C;
    'dispatch: loop {
        match pc {
            0x8303B09C => {
    //   block [0x8303B09C..0x8303B0E4)
	// 8303B09C: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 8303B0A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8303B0A4: 28090000  cmplwi r9, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8303B0A8: 41820024  beq 0x8303b0cc
	if ctx.cr[0].eq {
	pc = 0x8303B0CC; continue 'dispatch;
	}
	// 8303B0AC: 814A0008  lwz r10, 8(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 8303B0B0: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303B0B4: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 8303B0B8: 409A002C  bne cr6, 0x8303b0e4
	if !ctx.cr[6].eq {
		sub_8303B0E4(ctx, base);
		return;
	}
	// 8303B0BC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8303B0C0: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 8303B0C4: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 8303B0C8: 4198FFE8  blt cr6, 0x8303b0b0
	if ctx.cr[6].lt {
	pc = 0x8303B0B0; continue 'dispatch;
	}
	// 8303B0CC: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8303B0D0: 556B063E  clrlwi r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 8303B0D4: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 8303B0D8: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8303B0DC: 5563063E  clrlwi r3, r11, 0x18
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 8303B0E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303B0E4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8303B0E4 size=8
    let mut pc: u32 = 0x8303B0E4;
    'dispatch: loop {
        match pc {
            0x8303B0E4 => {
    //   block [0x8303B0E4..0x8303B0EC)
	// 8303B0E4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8303B0E8: 4BFFFFE8  b 0x8303b0d0
	sub_8303B09C(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303B0F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8303B0F0 size=96
    let mut pc: u32 = 0x8303B0F0;
    'dispatch: loop {
        match pc {
            0x8303B0F0 => {
    //   block [0x8303B0F0..0x8303B150)
	// 8303B0F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8303B0F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8303B0F8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8303B0FC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8303B100: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8303B104: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8303B108: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8303B10C: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 8303B110: 90FF0000  stw r7, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 8303B114: 98BF0004  stb r5, 4(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[5].u8 ) };
	// 8303B118: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8303B11C: 909F000C  stw r4, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[4].u32 ) };
	// 8303B120: 909F0010  stw r4, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[4].u32 ) };
	// 8303B124: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 8303B128: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 8303B12C: 48011745  bl 0x8304c870
	ctx.lr = 0x8303B130;
	sub_8304C870(ctx, base);
	// 8303B130: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8303B134: 93DF0018  stw r30, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[30].u32 ) };
	// 8303B138: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8303B13C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8303B140: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8303B144: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8303B148: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8303B14C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303B150(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8303B150 size=88
    let mut pc: u32 = 0x8303B150;
    'dispatch: loop {
        match pc {
            0x8303B150 => {
    //   block [0x8303B150..0x8303B1A8)
	// 8303B150: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8303B154: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8303B158: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8303B15C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8303B160: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8303B164: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8303B168: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8303B16C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8303B170: 4BFFFE41  bl 0x8303afb0
	ctx.lr = 0x8303B174;
	sub_8303AFB0(ctx, base);
	// 8303B174: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8303B178: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8303B17C: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8303B180: 7FCB512E  stwx r30, r11, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[30].u32) };
	// 8303B184: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8303B188: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8303B18C: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8303B190: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8303B194: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8303B198: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8303B19C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8303B1A0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8303B1A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303B1A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8303B1A8 size=8
    let mut pc: u32 = 0x8303B1A8;
    'dispatch: loop {
        match pc {
            0x8303B1A8 => {
    //   block [0x8303B1A8..0x8303B1B0)
	// 8303B1A8: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8303B1AC: 8215D260  lwz r16, -0x2da0(r21)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-11680 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303B1B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8303B1B0 size=100
    let mut pc: u32 = 0x8303B1B0;
    'dispatch: loop {
        match pc {
            0x8303B1B0 => {
    //   block [0x8303B1B0..0x8303B214)
	// 8303B1B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8303B1B4: 4816CFB9  bl 0x831a816c
	ctx.lr = 0x8303B1B8;
	sub_831A8130(ctx, base);
	// 8303B1B8: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 8303B1BC: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8303B1C0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8303B1C4: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 8303B1C8: 93DF0094  stw r30, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[30].u32 ) };
	// 8303B1CC: 90DE0000  stw r6, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[6].u32 ) };
	// 8303B1D0: 98BE0004  stb r5, 4(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[5].u8 ) };
	// 8303B1D4: 93BE0008  stw r29, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 8303B1D8: 909E000C  stw r4, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[4].u32 ) };
	// 8303B1DC: 93BE0010  stw r29, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[29].u32 ) };
	// 8303B1E0: 4BFA6F41  bl 0x82fe2120
	ctx.lr = 0x8303B1E4;
	sub_82FE2120(ctx, base);
	// 8303B1E4: 38600004  li r3, 4
	ctx.r[3].s64 = 4;
	// 8303B1E8: 809E0000  lwz r4, 0(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303B1EC: 4BF9D0AD  bl 0x82fd8298
	ctx.lr = 0x8303B1F0;
	sub_82FD8298(ctx, base);
	// 8303B1F0: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 8303B1F4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8303B1F8: 4182000C  beq 0x8303b204
	if ctx.cr[0].eq {
	pc = 0x8303B204; continue 'dispatch;
	}
	// 8303B1FC: 4BFD4765  bl 0x8300f960
	ctx.lr = 0x8303B200;
	sub_8300F960(ctx, base);
	// 8303B200: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8303B204: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8303B208: 93BE0010  stw r29, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[29].u32 ) };
	// 8303B20C: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 8303B210: 4816CFAC  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303B214(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8303B214 size=48
    let mut pc: u32 = 0x8303B214;
    'dispatch: loop {
        match pc {
            0x8303B214 => {
    //   block [0x8303B214..0x8303B244)
	// 8303B214: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 8303B218: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8303B21C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8303B220: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8303B224: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 8303B228: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303B22C: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8303B230: 4BF9D0B1  bl 0x82fd82e0
	ctx.lr = 0x8303B234;
	sub_82FD82E0(ctx, base);
	// 8303B234: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8303B238: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8303B23C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8303B240: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303B248(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8303B248 size=8
    let mut pc: u32 = 0x8303B248;
    'dispatch: loop {
        match pc {
            0x8303B248 => {
    //   block [0x8303B248..0x8303B250)
	// 8303B248: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8303B24C: 8215D2A8  lwz r16, -0x2d58(r21)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-11608 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303B250(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8303B250 size=224
    let mut pc: u32 = 0x8303B250;
    'dispatch: loop {
        match pc {
            0x8303B250 => {
    //   block [0x8303B250..0x8303B330)
	// 8303B250: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8303B254: 4816CF11  bl 0x831a8164
	ctx.lr = 0x8303B258;
	sub_831A8130(ctx, base);
	// 8303B258: 3BE1FF70  addi r31, r1, -0x90
	ctx.r[31].s64 = ctx.r[1].s64 + -144;
	// 8303B25C: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8303B260: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 8303B264: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8303B268: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 8303B26C: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 8303B270: 93BF00AC  stw r29, 0xac(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(172 as u32), ctx.r[29].u32 ) };
	// 8303B274: 93BC0000  stw r29, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 8303B278: 937C0004  stw r27, 4(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(4 as u32), ctx.r[27].u32 ) };
	// 8303B27C: 937C0008  stw r27, 8(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(8 as u32), ctx.r[27].u32 ) };
	// 8303B280: 4BF9D019  bl 0x82fd8298
	ctx.lr = 0x8303B284;
	sub_82FD8298(ctx, base);
	// 8303B284: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8303B288: 93DF0050  stw r30, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 8303B28C: 4182002C  beq 0x8303b2b8
	if ctx.cr[0].eq {
	pc = 0x8303B2B8; continue 'dispatch;
	}
	// 8303B290: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 8303B294: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8303B298: 38800014  li r4, 0x14
	ctx.r[4].s64 = 20;
	// 8303B29C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8303B2A0: 48011551  bl 0x8304c7f0
	ctx.lr = 0x8303B2A4;
	sub_8304C7F0(ctx, base);
	// 8303B2A4: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 8303B2A8: 394B2990  addi r10, r11, 0x2990
	ctx.r[10].s64 = ctx.r[11].s64 + 10640;
	// 8303B2AC: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 8303B2B0: 915E0000  stw r10, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8303B2B4: 48000008  b 0x8303b2bc
	pc = 0x8303B2BC; continue 'dispatch;
	// 8303B2B8: 7F6BDB78  mr r11, r27
	ctx.r[11].u64 = ctx.r[27].u64;
	// 8303B2BC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8303B2C0: 3860001C  li r3, 0x1c
	ctx.r[3].s64 = 28;
	// 8303B2C4: 917C0008  stw r11, 8(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8303B2C8: 4BF9CFD1  bl 0x82fd8298
	ctx.lr = 0x8303B2CC;
	sub_82FD8298(ctx, base);
	// 8303B2CC: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8303B2D0: 93DF0050  stw r30, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 8303B2D4: 41820048  beq 0x8303b31c
	if ctx.cr[0].eq {
	pc = 0x8303B31C; continue 'dispatch;
	}
	// 8303B2D8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8303B2DC: 38600004  li r3, 4
	ctx.r[3].s64 = 4;
	// 8303B2E0: 4BF9CFB9  bl 0x82fd8298
	ctx.lr = 0x8303B2E4;
	sub_82FD8298(ctx, base);
	// 8303B2E4: 907F0054  stw r3, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 8303B2E8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8303B2EC: 41820010  beq 0x8303b2fc
	if ctx.cr[0].eq {
	pc = 0x8303B2FC; continue 'dispatch;
	}
	// 8303B2F0: 4BFC42A1  bl 0x82fff590
	ctx.lr = 0x8303B2F4;
	sub_82FFF590(ctx, base);
	// 8303B2F4: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 8303B2F8: 48000008  b 0x8303b300
	pc = 0x8303B300; continue 'dispatch;
	// 8303B2FC: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 8303B300: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 8303B304: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8303B308: 3880006D  li r4, 0x6d
	ctx.r[4].s64 = 109;
	// 8303B30C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8303B310: 4BFFFDE1  bl 0x8303b0f0
	ctx.lr = 0x8303B314;
	sub_8303B0F0(ctx, base);
	// 8303B314: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8303B318: 48000008  b 0x8303b320
	pc = 0x8303B320; continue 'dispatch;
	// 8303B31C: 7F6BDB78  mr r11, r27
	ctx.r[11].u64 = ctx.r[27].u64;
	// 8303B320: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8303B324: 917C0004  stw r11, 4(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8303B328: 383F0090  addi r1, r31, 0x90
	ctx.r[1].s64 = ctx.r[31].s64 + 144;
	// 8303B32C: 4816CE88  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303B330(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8303B330 size=44
    let mut pc: u32 = 0x8303B330;
    'dispatch: loop {
        match pc {
            0x8303B330 => {
    //   block [0x8303B330..0x8303B35C)
	// 8303B330: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 8303B334: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8303B338: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8303B33C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8303B340: 809F00AC  lwz r4, 0xac(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(172 as u32) ) } as u64;
	// 8303B344: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8303B348: 4BF9CF99  bl 0x82fd82e0
	ctx.lr = 0x8303B34C;
	sub_82FD82E0(ctx, base);
	// 8303B34C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8303B350: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8303B354: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8303B358: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303B35C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8303B35C size=44
    let mut pc: u32 = 0x8303B35C;
    'dispatch: loop {
        match pc {
            0x8303B35C => {
    //   block [0x8303B35C..0x8303B388)
	// 8303B35C: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 8303B360: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8303B364: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8303B368: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8303B36C: 809F00AC  lwz r4, 0xac(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(172 as u32) ) } as u64;
	// 8303B370: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8303B374: 4BF9CF6D  bl 0x82fd82e0
	ctx.lr = 0x8303B378;
	sub_82FD82E0(ctx, base);
	// 8303B378: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8303B37C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8303B380: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8303B384: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303B388(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8303B388 size=44
    let mut pc: u32 = 0x8303B388;
    'dispatch: loop {
        match pc {
            0x8303B388 => {
    //   block [0x8303B388..0x8303B3B4)
	// 8303B388: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 8303B38C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8303B390: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8303B394: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8303B398: 809F00AC  lwz r4, 0xac(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(172 as u32) ) } as u64;
	// 8303B39C: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 8303B3A0: 4BF9CF41  bl 0x82fd82e0
	ctx.lr = 0x8303B3A4;
	sub_82FD82E0(ctx, base);
	// 8303B3A4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8303B3A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8303B3AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8303B3B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303B3B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8303B3B8 size=8
    let mut pc: u32 = 0x8303B3B8;
    'dispatch: loop {
        match pc {
            0x8303B3B8 => {
    //   block [0x8303B3B8..0x8303B3C0)
	// 8303B3B8: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8303B3BC: 8215D308  lwz r16, -0x2cf8(r21)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-11512 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303B3C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8303B3C0 size=108
    let mut pc: u32 = 0x8303B3C0;
    'dispatch: loop {
        match pc {
            0x8303B3C0 => {
    //   block [0x8303B3C0..0x8303B42C)
	// 8303B3C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8303B3C4: 4816CDA5  bl 0x831a8168
	ctx.lr = 0x8303B3C8;
	sub_831A8130(ctx, base);
	// 8303B3C8: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 8303B3CC: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8303B3D0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8303B3D4: 38600024  li r3, 0x24
	ctx.r[3].s64 = 36;
	// 8303B3D8: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8303B3DC: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 8303B3E0: 809E0000  lwz r4, 0(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303B3E4: 93DF0094  stw r30, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[30].u32 ) };
	// 8303B3E8: 4BF9CEB1  bl 0x82fd8298
	ctx.lr = 0x8303B3EC;
	sub_82FD8298(ctx, base);
	// 8303B3EC: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 8303B3F0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8303B3F4: 4182001C  beq 0x8303b410
	if ctx.cr[0].eq {
	pc = 0x8303B410; continue 'dispatch;
	}
	// 8303B3F8: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 8303B3FC: 80DE0000  lwz r6, 0(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303B400: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8303B404: 48044845  bl 0x8307fc48
	ctx.lr = 0x8303B408;
	sub_8307FC48(ctx, base);
	// 8303B408: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8303B40C: 48000008  b 0x8303b414
	pc = 0x8303B414; continue 'dispatch;
	// 8303B410: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 8303B414: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8303B418: 807E0008  lwz r3, 8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8303B41C: 4BFFFD35  bl 0x8303b150
	ctx.lr = 0x8303B420;
	sub_8303B150(ctx, base);
	// 8303B420: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8303B424: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 8303B428: 4816CD90  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303B42C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8303B42C size=48
    let mut pc: u32 = 0x8303B42C;
    'dispatch: loop {
        match pc {
            0x8303B42C => {
    //   block [0x8303B42C..0x8303B45C)
	// 8303B42C: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 8303B430: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8303B434: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8303B438: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8303B43C: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 8303B440: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303B444: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8303B448: 4BF9CE99  bl 0x82fd82e0
	ctx.lr = 0x8303B44C;
	sub_82FD82E0(ctx, base);
	// 8303B44C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8303B450: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8303B454: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8303B458: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303B460(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8303B460 size=148
    let mut pc: u32 = 0x8303B460;
    'dispatch: loop {
        match pc {
            0x8303B460 => {
    //   block [0x8303B460..0x8303B4F4)
	// 8303B460: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8303B464: 4816CD05  bl 0x831a8168
	ctx.lr = 0x8303B468;
	sub_831A8130(ctx, base);
	// 8303B468: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8303B46C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8303B470: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 8303B474: 83DD0008  lwz r30, 8(r29)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 8303B478: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8303B47C: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8303B480: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8303B484: 40990050  ble cr6, 0x8303b4d4
	if !ctx.cr[6].gt {
	pc = 0x8303B4D4; continue 'dispatch;
	}
	// 8303B488: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8303B48C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8303B490: 4BFF13E1  bl 0x8302c870
	ctx.lr = 0x8303B494;
	sub_8302C870(ctx, base);
	// 8303B494: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8303B498: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8303B49C: 41820028  beq 0x8303b4c4
	if ctx.cr[0].eq {
	pc = 0x8303B4C4; continue 'dispatch;
	}
	// 8303B4A0: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8303B4A4: 806B004C  lwz r3, 0x4c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(76 as u32) ) } as u64;
	// 8303B4A8: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8303B4AC: 4BFBEFB5  bl 0x82ffa460
	ctx.lr = 0x8303B4B0;
	sub_82FFA460(ctx, base);
	// 8303B4B0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8303B4B4: 41820010  beq 0x8303b4c4
	if ctx.cr[0].eq {
	pc = 0x8303B4C4; continue 'dispatch;
	}
	// 8303B4B8: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303B4BC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8303B4C0: 4082002C  bne 0x8303b4ec
	if !ctx.cr[0].eq {
	pc = 0x8303B4EC; continue 'dispatch;
	}
	// 8303B4C4: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8303B4C8: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 8303B4CC: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8303B4D0: 4198FFB8  blt cr6, 0x8303b488
	if ctx.cr[6].lt {
	pc = 0x8303B488; continue 'dispatch;
	}
	// 8303B4D4: 817D0090  lwz r11, 0x90(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(144 as u32) ) } as u64;
	// 8303B4D8: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8303B4DC: 4182000C  beq 0x8303b4e8
	if ctx.cr[0].eq {
	pc = 0x8303B4E8; continue 'dispatch;
	}
	// 8303B4E0: 7D7D5B78  mr r29, r11
	ctx.r[29].u64 = ctx.r[11].u64;
	// 8303B4E4: 4BFFFF90  b 0x8303b474
	pc = 0x8303B474; continue 'dispatch;
	// 8303B4E8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8303B4EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8303B4F0: 4816CCC8  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303B4F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8303B4F8 size=108
    let mut pc: u32 = 0x8303B4F8;
    'dispatch: loop {
        match pc {
            0x8303B4F8 => {
    //   block [0x8303B4F8..0x8303B564)
	// 8303B4F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8303B4FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8303B500: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8303B504: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8303B508: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8303B50C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8303B510: 83FE0004  lwz r31, 4(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8303B514: 281F0000  cmplwi r31, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8303B518: 41820014  beq 0x8303b52c
	if ctx.cr[0].eq {
	pc = 0x8303B52C; continue 'dispatch;
	}
	// 8303B51C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8303B520: 4BFAB749  bl 0x82fe6c68
	ctx.lr = 0x8303B524;
	sub_82FE6C68(ctx, base);
	// 8303B524: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8303B528: 4BF9CDB9  bl 0x82fd82e0
	ctx.lr = 0x8303B52C;
	sub_82FD82E0(ctx, base);
	// 8303B52C: 807E0008  lwz r3, 8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8303B530: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8303B534: 41820018  beq 0x8303b54c
	if ctx.cr[0].eq {
	pc = 0x8303B54C; continue 'dispatch;
	}
	// 8303B538: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303B53C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8303B540: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303B544: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8303B548: 4E800421  bctrl
	ctx.lr = 0x8303B54C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8303B54C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8303B550: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8303B554: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8303B558: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8303B55C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8303B560: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303B568(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8303B568 size=8
    let mut pc: u32 = 0x8303B568;
    'dispatch: loop {
        match pc {
            0x8303B568 => {
    //   block [0x8303B568..0x8303B570)
	// 8303B568: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8303B56C: 8215D358  lwz r16, -0x2ca8(r21)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-11432 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303B570(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8303B570 size=416
    let mut pc: u32 = 0x8303B570;
    'dispatch: loop {
        match pc {
            0x8303B570 => {
    //   block [0x8303B570..0x8303B710)
	// 8303B570: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8303B574: 4816CBE5  bl 0x831a8158
	ctx.lr = 0x8303B578;
	sub_831A8130(ctx, base);
	// 8303B578: 3BE1FF60  addi r31, r1, -0xa0
	ctx.r[31].s64 = ctx.r[1].s64 + -160;
	// 8303B57C: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8303B580: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8303B584: 7CB92B78  mr r25, r5
	ctx.r[25].u64 = ctx.r[5].u64;
	// 8303B588: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 8303B58C: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 8303B590: 93BF00B4  stw r29, 0xb4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(180 as u32), ctx.r[29].u32 ) };
	// 8303B594: 4BFD261D  bl 0x8300dbb0
	ctx.lr = 0x8303B598;
	sub_8300DBB0(ctx, base);
	// 8303B598: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8303B59C: 40820168  bne 0x8303b704
	if !ctx.cr[0].eq {
	pc = 0x8303B704; continue 'dispatch;
	}
	// 8303B5A0: 817A0010  lwz r11, 0x10(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(16 as u32) ) } as u64;
	// 8303B5A4: 3B000000  li r24, 0
	ctx.r[24].s64 = 0;
	// 8303B5A8: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 8303B5AC: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8303B5B0: 4182000C  beq 0x8303b5bc
	if ctx.cr[0].eq {
	pc = 0x8303B5BC; continue 'dispatch;
	}
	// 8303B5B4: 836B0008  lwz r27, 8(r11)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8303B5B8: 48000008  b 0x8303b5c0
	pc = 0x8303B5C0; continue 'dispatch;
	// 8303B5BC: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 8303B5C0: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 8303B5C4: 419A0098  beq cr6, 0x8303b65c
	if ctx.cr[6].eq {
	pc = 0x8303B65C; continue 'dispatch;
	}
	// 8303B5C8: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 8303B5CC: 809D0000  lwz r4, 0(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303B5D0: 4BF9CCC9  bl 0x82fd8298
	ctx.lr = 0x8303B5D4;
	sub_82FD8298(ctx, base);
	// 8303B5D4: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8303B5D8: 93DF0050  stw r30, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 8303B5DC: 4182002C  beq 0x8303b608
	if ctx.cr[0].eq {
	pc = 0x8303B608; continue 'dispatch;
	}
	// 8303B5E0: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8303B5E4: 80DD0000  lwz r6, 0(r29)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303B5E8: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8303B5EC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8303B5F0: 4BF97451  bl 0x82fd2a40
	ctx.lr = 0x8303B5F4;
	sub_82FD2A40(ctx, base);
	// 8303B5F4: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 8303B5F8: 7FDCF378  mr r28, r30
	ctx.r[28].u64 = ctx.r[30].u64;
	// 8303B5FC: 396B6C68  addi r11, r11, 0x6c68
	ctx.r[11].s64 = ctx.r[11].s64 + 27752;
	// 8303B600: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8303B604: 48000008  b 0x8303b60c
	pc = 0x8303B60C; continue 'dispatch;
	// 8303B608: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 8303B60C: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8303B610: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 8303B614: 419A0048  beq cr6, 0x8303b65c
	if ctx.cr[6].eq {
	pc = 0x8303B65C; continue 'dispatch;
	}
	// 8303B618: 807A0010  lwz r3, 0x10(r26)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(16 as u32) ) } as u64;
	// 8303B61C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8303B620: 41820010  beq 0x8303b630
	if ctx.cr[0].eq {
	pc = 0x8303B630; continue 'dispatch;
	}
	// 8303B624: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8303B628: 4BFF1249  bl 0x8302c870
	ctx.lr = 0x8303B62C;
	sub_8302C870(ctx, base);
	// 8303B62C: 48000008  b 0x8303b634
	pc = 0x8303B634; continue 'dispatch;
	// 8303B630: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8303B634: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8303B638: 809D0000  lwz r4, 0(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303B63C: 806B0008  lwz r3, 8(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8303B640: 4BF95541  bl 0x82fd0b80
	ctx.lr = 0x8303B644;
	sub_82FD0B80(ctx, base);
	// 8303B644: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8303B648: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8303B64C: 4BFFFB05  bl 0x8303b150
	ctx.lr = 0x8303B650;
	sub_8303B150(ctx, base);
	// 8303B650: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 8303B654: 7F1ED840  cmplw cr6, r30, r27
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[27].u32, &mut ctx.xer);
	// 8303B658: 4198FFC0  blt cr6, 0x8303b618
	if ctx.cr[6].lt {
	pc = 0x8303B618; continue 'dispatch;
	}
	// 8303B65C: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303B660: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 8303B664: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 8303B668: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8303B66C: 4E800421  bctrl
	ctx.lr = 0x8303B670;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8303B670: 7C6B0734  extsh r11, r3
	ctx.r[11].s64 = ctx.r[3].s16 as i64;
	// 8303B674: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 8303B678: 409A0018  bne cr6, 0x8303b690
	if !ctx.cr[6].eq {
	pc = 0x8303B690; continue 'dispatch;
	}
	// 8303B67C: 7F25CB78  mr r5, r25
	ctx.r[5].u64 = ctx.r[25].u64;
	// 8303B680: 809A001C  lwz r4, 0x1c(r26)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(28 as u32) ) } as u64;
	// 8303B684: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8303B688: 4BFFFEE9  bl 0x8303b570
	ctx.lr = 0x8303B68C;
	sub_8303B570(ctx, base);
	// 8303B68C: 7C781B78  mr r24, r3
	ctx.r[24].u64 = ctx.r[3].u64;
	// 8303B690: 38600024  li r3, 0x24
	ctx.r[3].s64 = 36;
	// 8303B694: 809D0000  lwz r4, 0(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303B698: 4BF9CC01  bl 0x82fd8298
	ctx.lr = 0x8303B69C;
	sub_82FD8298(ctx, base);
	// 8303B69C: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8303B6A0: 93DF0050  stw r30, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 8303B6A4: 41820040  beq 0x8303b6e4
	if ctx.cr[0].eq {
	pc = 0x8303B6E4; continue 'dispatch;
	}
	// 8303B6A8: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 8303B6AC: 837D0000  lwz r27, 0(r29)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303B6B0: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 8303B6B4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8303B6B8: 4BFFFDA9  bl 0x8303b460
	ctx.lr = 0x8303B6BC;
	sub_8303B460(ctx, base);
	// 8303B6BC: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 8303B6C0: 7F05C378  mr r5, r24
	ctx.r[5].u64 = ctx.r[24].u64;
	// 8303B6C4: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 8303B6C8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8303B6CC: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 8303B6D0: 7F28CB78  mr r8, r25
	ctx.r[8].u64 = ctx.r[25].u64;
	// 8303B6D4: 7F69DB78  mr r9, r27
	ctx.r[9].u64 = ctx.r[27].u64;
	// 8303B6D8: 48044761  bl 0x8307fe38
	ctx.lr = 0x8303B6DC;
	sub_8307FE38(ctx, base);
	// 8303B6DC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8303B6E0: 48000008  b 0x8303b6e8
	pc = 0x8303B6E8; continue 'dispatch;
	// 8303B6E4: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8303B6E8: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8303B6EC: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 8303B6F0: 807D0004  lwz r3, 4(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 8303B6F4: 4BFF1755  bl 0x8302ce48
	ctx.lr = 0x8303B6F8;
	sub_8302CE48(ctx, base);
	// 8303B6F8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8303B6FC: 807D0008  lwz r3, 8(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 8303B700: 4BFFFA51  bl 0x8303b150
	ctx.lr = 0x8303B704;
	sub_8303B150(ctx, base);
	// 8303B704: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8303B708: 383F00A0  addi r1, r31, 0xa0
	ctx.r[1].s64 = ctx.r[31].s64 + 160;
	// 8303B70C: 4816CA9C  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303B710(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8303B710 size=48
    let mut pc: u32 = 0x8303B710;
    'dispatch: loop {
        match pc {
            0x8303B710 => {
    //   block [0x8303B710..0x8303B740)
	// 8303B710: 3BECFF60  addi r31, r12, -0xa0
	ctx.r[31].s64 = ctx.r[12].s64 + -160;
	// 8303B714: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8303B718: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8303B71C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8303B720: 817F00B4  lwz r11, 0xb4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(180 as u32) ) } as u64;
	// 8303B724: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303B728: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8303B72C: 4BF9CBB5  bl 0x82fd82e0
	ctx.lr = 0x8303B730;
	sub_82FD82E0(ctx, base);
	// 8303B730: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8303B734: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8303B738: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8303B73C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303B740(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8303B740 size=48
    let mut pc: u32 = 0x8303B740;
    'dispatch: loop {
        match pc {
            0x8303B740 => {
    //   block [0x8303B740..0x8303B770)
	// 8303B740: 3BECFF60  addi r31, r12, -0xa0
	ctx.r[31].s64 = ctx.r[12].s64 + -160;
	// 8303B744: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8303B748: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8303B74C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8303B750: 817F00B4  lwz r11, 0xb4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(180 as u32) ) } as u64;
	// 8303B754: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303B758: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8303B75C: 4BF9CB85  bl 0x82fd82e0
	ctx.lr = 0x8303B760;
	sub_82FD82E0(ctx, base);
	// 8303B760: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8303B764: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8303B768: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8303B76C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303B770(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8303B770 size=8
    let mut pc: u32 = 0x8303B770;
    'dispatch: loop {
        match pc {
            0x8303B770 => {
    //   block [0x8303B770..0x8303B778)
	// 8303B770: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8303B774: 8215D3B0  lwz r16, -0x2c50(r21)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-11344 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303B778(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8303B778 size=172
    let mut pc: u32 = 0x8303B778;
    'dispatch: loop {
        match pc {
            0x8303B778 => {
    //   block [0x8303B778..0x8303B824)
	// 8303B778: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8303B77C: 4816C9E5  bl 0x831a8160
	ctx.lr = 0x8303B780;
	sub_831A8130(ctx, base);
	// 8303B780: 3BE1FF70  addi r31, r1, -0x90
	ctx.r[31].s64 = ctx.r[1].s64 + -144;
	// 8303B784: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8303B788: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8303B78C: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 8303B790: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8303B794: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8303B798: 93DF00A4  stw r30, 0xa4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(164 as u32), ctx.r[30].u32 ) };
	// 8303B79C: 4BFD2415  bl 0x8300dbb0
	ctx.lr = 0x8303B7A0;
	sub_8300DBB0(ctx, base);
	// 8303B7A0: 7C7B1B79  or. r27, r3, r3
	ctx.r[27].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 8303B7A4: 40820074  bne 0x8303b818
	if !ctx.cr[0].eq {
	pc = 0x8303B818; continue 'dispatch;
	}
	// 8303B7A8: 3860001C  li r3, 0x1c
	ctx.r[3].s64 = 28;
	// 8303B7AC: 809E0000  lwz r4, 0(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303B7B0: 4BF9CAE9  bl 0x82fd8298
	ctx.lr = 0x8303B7B4;
	sub_82FD8298(ctx, base);
	// 8303B7B4: 7C7B1B79  or. r27, r3, r3
	ctx.r[27].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 8303B7B8: 937F0050  stw r27, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[27].u32 ) };
	// 8303B7BC: 41820038  beq 0x8303b7f4
	if ctx.cr[0].eq {
	pc = 0x8303B7F4; continue 'dispatch;
	}
	// 8303B7C0: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8303B7C4: 835E0000  lwz r26, 0(r30)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303B7C8: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8303B7CC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8303B7D0: 4BFFFC91  bl 0x8303b460
	ctx.lr = 0x8303B7D4;
	sub_8303B460(ctx, base);
	// 8303B7D4: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 8303B7D8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8303B7DC: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 8303B7E0: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 8303B7E4: 7F47D378  mr r7, r26
	ctx.r[7].u64 = ctx.r[26].u64;
	// 8303B7E8: 48044781  bl 0x8307ff68
	ctx.lr = 0x8303B7EC;
	sub_8307FF68(ctx, base);
	// 8303B7EC: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 8303B7F0: 48000008  b 0x8303b7f8
	pc = 0x8303B7F8; continue 'dispatch;
	// 8303B7F4: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 8303B7F8: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 8303B7FC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8303B800: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8303B804: 7F9BE378  mr r27, r28
	ctx.r[27].u64 = ctx.r[28].u64;
	// 8303B808: 4BFF1641  bl 0x8302ce48
	ctx.lr = 0x8303B80C;
	sub_8302CE48(ctx, base);
	// 8303B80C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8303B810: 807E0008  lwz r3, 8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8303B814: 4BFFF93D  bl 0x8303b150
	ctx.lr = 0x8303B818;
	sub_8303B150(ctx, base);
	// 8303B818: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 8303B81C: 383F0090  addi r1, r31, 0x90
	ctx.r[1].s64 = ctx.r[31].s64 + 144;
	// 8303B820: 4816C990  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303B824(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8303B824 size=48
    let mut pc: u32 = 0x8303B824;
    'dispatch: loop {
        match pc {
            0x8303B824 => {
    //   block [0x8303B824..0x8303B854)
	// 8303B824: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 8303B828: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8303B82C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8303B830: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8303B834: 817F00A4  lwz r11, 0xa4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 8303B838: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303B83C: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8303B840: 4BF9CAA1  bl 0x82fd82e0
	ctx.lr = 0x8303B844;
	sub_82FD82E0(ctx, base);
	// 8303B844: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8303B848: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8303B84C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8303B850: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303B858(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8303B858 size=8
    let mut pc: u32 = 0x8303B858;
    'dispatch: loop {
        match pc {
            0x8303B858 => {
    //   block [0x8303B858..0x8303B860)
	// 8303B858: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8303B85C: 8215D3F8  lwz r16, -0x2c08(r21)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-11272 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303B860(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8303B860 size=140
    let mut pc: u32 = 0x8303B860;
    'dispatch: loop {
        match pc {
            0x8303B860 => {
    //   block [0x8303B860..0x8303B8EC)
	// 8303B860: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8303B864: 4816C901  bl 0x831a8164
	ctx.lr = 0x8303B868;
	sub_831A8130(ctx, base);
	// 8303B868: 3BE1FF70  addi r31, r1, -0x90
	ctx.r[31].s64 = ctx.r[1].s64 + -144;
	// 8303B86C: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8303B870: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 8303B874: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8303B878: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8303B87C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8303B880: 80BB0048  lwz r5, 0x48(r27)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(72 as u32) ) } as u64;
	// 8303B884: 93DF00A4  stw r30, 0xa4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(164 as u32), ctx.r[30].u32 ) };
	// 8303B888: 28050000  cmplwi r5, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8303B88C: 40820008  bne 0x8303b894
	if !ctx.cr[0].eq {
	pc = 0x8303B894; continue 'dispatch;
	}
	// 8303B890: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 8303B894: 4BFFFBCD  bl 0x8303b460
	ctx.lr = 0x8303B898;
	sub_8303B460(ctx, base);
	// 8303B898: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 8303B89C: 38600024  li r3, 0x24
	ctx.r[3].s64 = 36;
	// 8303B8A0: 809E0000  lwz r4, 0(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303B8A4: 4BF9C9F5  bl 0x82fd8298
	ctx.lr = 0x8303B8A8;
	sub_82FD8298(ctx, base);
	// 8303B8A8: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 8303B8AC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8303B8B0: 41820020  beq 0x8303b8d0
	if ctx.cr[0].eq {
	pc = 0x8303B8D0; continue 'dispatch;
	}
	// 8303B8B4: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 8303B8B8: 80FE0000  lwz r7, 0(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303B8BC: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 8303B8C0: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8303B8C4: 4804486D  bl 0x83080130
	ctx.lr = 0x8303B8C8;
	sub_83080130(ctx, base);
	// 8303B8C8: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8303B8CC: 48000008  b 0x8303b8d4
	pc = 0x8303B8D4; continue 'dispatch;
	// 8303B8D0: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 8303B8D4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8303B8D8: 807E0008  lwz r3, 8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8303B8DC: 4BFFF875  bl 0x8303b150
	ctx.lr = 0x8303B8E0;
	sub_8303B150(ctx, base);
	// 8303B8E0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8303B8E4: 383F0090  addi r1, r31, 0x90
	ctx.r[1].s64 = ctx.r[31].s64 + 144;
	// 8303B8E8: 4816C8CC  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303B8EC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8303B8EC size=48
    let mut pc: u32 = 0x8303B8EC;
    'dispatch: loop {
        match pc {
            0x8303B8EC => {
    //   block [0x8303B8EC..0x8303B91C)
	// 8303B8EC: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 8303B8F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8303B8F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8303B8F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8303B8FC: 817F00A4  lwz r11, 0xa4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 8303B900: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303B904: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8303B908: 4BF9C9D9  bl 0x82fd82e0
	ctx.lr = 0x8303B90C;
	sub_82FD82E0(ctx, base);
	// 8303B90C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8303B910: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8303B914: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8303B918: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303B920(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8303B920 size=8
    let mut pc: u32 = 0x8303B920;
    'dispatch: loop {
        match pc {
            0x8303B920 => {
    //   block [0x8303B920..0x8303B928)
	// 8303B920: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8303B924: 8215D440  lwz r16, -0x2bc0(r21)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-11200 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303B928(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8303B928 size=136
    let mut pc: u32 = 0x8303B928;
    'dispatch: loop {
        match pc {
            0x8303B928 => {
    //   block [0x8303B928..0x8303B9B0)
	// 8303B928: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8303B92C: 4816C835  bl 0x831a8160
	ctx.lr = 0x8303B930;
	sub_831A8130(ctx, base);
	// 8303B930: 3BE1FF70  addi r31, r1, -0x90
	ctx.r[31].s64 = ctx.r[1].s64 + -144;
	// 8303B934: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8303B938: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8303B93C: 38600024  li r3, 0x24
	ctx.r[3].s64 = 36;
	// 8303B940: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8303B944: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 8303B948: 809E0000  lwz r4, 0(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303B94C: 93DF00A4  stw r30, 0xa4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(164 as u32), ctx.r[30].u32 ) };
	// 8303B950: 4BF9C949  bl 0x82fd8298
	ctx.lr = 0x8303B954;
	sub_82FD8298(ctx, base);
	// 8303B954: 7C7B1B79  or. r27, r3, r3
	ctx.r[27].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 8303B958: 937F0050  stw r27, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[27].u32 ) };
	// 8303B95C: 41820038  beq 0x8303b994
	if ctx.cr[0].eq {
	pc = 0x8303B994; continue 'dispatch;
	}
	// 8303B960: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8303B964: 835E0000  lwz r26, 0(r30)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303B968: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8303B96C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8303B970: 4BFFFAF1  bl 0x8303b460
	ctx.lr = 0x8303B974;
	sub_8303B460(ctx, base);
	// 8303B974: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 8303B978: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8303B97C: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 8303B980: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 8303B984: 7F47D378  mr r7, r26
	ctx.r[7].u64 = ctx.r[26].u64;
	// 8303B988: 48044AA9  bl 0x83080430
	ctx.lr = 0x8303B98C;
	sub_83080430(ctx, base);
	// 8303B98C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8303B990: 48000008  b 0x8303b998
	pc = 0x8303B998; continue 'dispatch;
	// 8303B994: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 8303B998: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8303B99C: 807E0008  lwz r3, 8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8303B9A0: 4BFFF7B1  bl 0x8303b150
	ctx.lr = 0x8303B9A4;
	sub_8303B150(ctx, base);
	// 8303B9A4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8303B9A8: 383F0090  addi r1, r31, 0x90
	ctx.r[1].s64 = ctx.r[31].s64 + 144;
	// 8303B9AC: 4816C804  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303B9B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8303B9B0 size=48
    let mut pc: u32 = 0x8303B9B0;
    'dispatch: loop {
        match pc {
            0x8303B9B0 => {
    //   block [0x8303B9B0..0x8303B9E0)
	// 8303B9B0: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 8303B9B4: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8303B9B8: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8303B9BC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8303B9C0: 817F00A4  lwz r11, 0xa4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 8303B9C4: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303B9C8: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8303B9CC: 4BF9C915  bl 0x82fd82e0
	ctx.lr = 0x8303B9D0;
	sub_82FD82E0(ctx, base);
	// 8303B9D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8303B9D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8303B9D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8303B9DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303B9E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8303B9E0 size=8
    let mut pc: u32 = 0x8303B9E0;
    'dispatch: loop {
        match pc {
            0x8303B9E0 => {
    //   block [0x8303B9E0..0x8303B9E8)
	// 8303B9E0: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8303B9E4: 8215D488  lwz r16, -0x2b78(r21)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-11128 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303B9E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8303B9E8 size=120
    let mut pc: u32 = 0x8303B9E8;
    'dispatch: loop {
        match pc {
            0x8303B9E8 => {
    //   block [0x8303B9E8..0x8303BA60)
	// 8303B9E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8303B9EC: 4816C779  bl 0x831a8164
	ctx.lr = 0x8303B9F0;
	sub_831A8130(ctx, base);
	// 8303B9F0: 3BE1FF70  addi r31, r1, -0x90
	ctx.r[31].s64 = ctx.r[1].s64 + -144;
	// 8303B9F4: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8303B9F8: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8303B9FC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8303BA00: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 8303BA04: 93BF00A4  stw r29, 0xa4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(164 as u32), ctx.r[29].u32 ) };
	// 8303BA08: 4BFFFF21  bl 0x8303b928
	ctx.lr = 0x8303BA0C;
	sub_8303B928(ctx, base);
	// 8303BA0C: 7C7B1B79  or. r27, r3, r3
	ctx.r[27].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 8303BA10: 41820044  beq 0x8303ba54
	if ctx.cr[0].eq {
	pc = 0x8303BA54; continue 'dispatch;
	}
	// 8303BA14: 38600024  li r3, 0x24
	ctx.r[3].s64 = 36;
	// 8303BA18: 809D0000  lwz r4, 0(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303BA1C: 4BF9C87D  bl 0x82fd8298
	ctx.lr = 0x8303BA20;
	sub_82FD8298(ctx, base);
	// 8303BA20: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 8303BA24: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8303BA28: 41820024  beq 0x8303ba4c
	if ctx.cr[0].eq {
	pc = 0x8303BA4C; continue 'dispatch;
	}
	// 8303BA2C: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 8303BA30: 811E0024  lwz r8, 0x24(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 8303BA34: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 8303BA38: 80FE0020  lwz r7, 0x20(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 8303BA3C: 38800009  li r4, 9
	ctx.r[4].s64 = 9;
	// 8303BA40: 813D0000  lwz r9, 0(r29)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303BA44: 48044C7D  bl 0x830806c0
	ctx.lr = 0x8303BA48;
	sub_830806C0(ctx, base);
	// 8303BA48: 48000008  b 0x8303ba50
	pc = 0x8303BA50; continue 'dispatch;
	// 8303BA4C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8303BA50: 48000008  b 0x8303ba58
	pc = 0x8303BA58; continue 'dispatch;
	// 8303BA54: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8303BA58: 383F0090  addi r1, r31, 0x90
	ctx.r[1].s64 = ctx.r[31].s64 + 144;
	// 8303BA5C: 4816C758  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303BA60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8303BA60 size=48
    let mut pc: u32 = 0x8303BA60;
    'dispatch: loop {
        match pc {
            0x8303BA60 => {
    //   block [0x8303BA60..0x8303BA90)
	// 8303BA60: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 8303BA64: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8303BA68: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8303BA6C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8303BA70: 817F00A4  lwz r11, 0xa4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 8303BA74: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303BA78: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8303BA7C: 4BF9C865  bl 0x82fd82e0
	ctx.lr = 0x8303BA80;
	sub_82FD82E0(ctx, base);
	// 8303BA80: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8303BA84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8303BA88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8303BA8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303BA90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8303BA90 size=8
    let mut pc: u32 = 0x8303BA90;
    'dispatch: loop {
        match pc {
            0x8303BA90 => {
    //   block [0x8303BA90..0x8303BA98)
	// 8303BA90: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8303BA94: 8215D500  lwz r16, -0x2b00(r21)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-11008 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303BA98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8303BA98 size=1984
    let mut pc: u32 = 0x8303BA98;
    'dispatch: loop {
        match pc {
            0x8303BA98 => {
    //   block [0x8303BA98..0x8303C258)
	// 8303BA98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8303BA9C: 4816C695  bl 0x831a8130
	ctx.lr = 0x8303BAA0;
	sub_831A8130(ctx, base);
	// 8303BAA0: 3BE1FEC0  addi r31, r1, -0x140
	ctx.r[31].s64 = ctx.r[1].s64 + -320;
	// 8303BAA4: 9421FEC0  stwu r1, -0x140(r1)
	ea = ctx.r[1].u32.wrapping_add(-320 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8303BAA8: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 8303BAAC: 7CB82B78  mr r24, r5
	ctx.r[24].u64 = ctx.r[5].u64;
	// 8303BAB0: 3AC00000  li r22, 0
	ctx.r[22].s64 = 0;
	// 8303BAB4: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 8303BAB8: 7CD73378  mr r23, r6
	ctx.r[23].u64 = ctx.r[6].u64;
	// 8303BABC: 833B0014  lwz r25, 0x14(r27)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(20 as u32) ) } as u64;
	// 8303BAC0: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 8303BAC4: 937F015C  stw r27, 0x15c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(348 as u32), ctx.r[27].u32 ) };
	// 8303BAC8: 7EDAB378  mr r26, r22
	ctx.r[26].u64 = ctx.r[22].u64;
	// 8303BACC: 931F0164  stw r24, 0x164(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(356 as u32), ctx.r[24].u32 ) };
	// 8303BAD0: 83BB0010  lwz r29, 0x10(r27)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(16 as u32) ) } as u64;
	// 8303BAD4: 809C0000  lwz r4, 0(r28)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303BAD8: 939F0154  stw r28, 0x154(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(340 as u32), ctx.r[28].u32 ) };
	// 8303BADC: 92FF016C  stw r23, 0x16c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(364 as u32), ctx.r[23].u32 ) };
	// 8303BAE0: 933F0054  stw r25, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[25].u32 ) };
	// 8303BAE4: 92DF0050  stw r22, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[22].u32 ) };
	// 8303BAE8: 92DF0058  stw r22, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[22].u32 ) };
	// 8303BAEC: 92DF005C  stw r22, 0x5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[22].u32 ) };
	// 8303BAF0: 92DF0064  stw r22, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[22].u32 ) };
	// 8303BAF4: 4BF9C7A5  bl 0x82fd8298
	ctx.lr = 0x8303BAF8;
	sub_82FD8298(ctx, base);
	// 8303BAF8: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8303BAFC: 93DF0068  stw r30, 0x68(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(104 as u32), ctx.r[30].u32 ) };
	// 8303BB00: 4182002C  beq 0x8303bb2c
	if ctx.cr[0].eq {
	pc = 0x8303BB2C; continue 'dispatch;
	}
	// 8303BB04: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8303BB08: 80DC0000  lwz r6, 0(r28)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303BB0C: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 8303BB10: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8303BB14: 48010CDD  bl 0x8304c7f0
	ctx.lr = 0x8303BB18;
	sub_8304C7F0(ctx, base);
	// 8303BB18: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 8303BB1C: 93DF0060  stw r30, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[30].u32 ) };
	// 8303BB20: 396B2990  addi r11, r11, 0x2990
	ctx.r[11].s64 = ctx.r[11].s64 + 10640;
	// 8303BB24: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8303BB28: 48000008  b 0x8303bb30
	pc = 0x8303BB30; continue 'dispatch;
	// 8303BB2C: 92DF0060  stw r22, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[22].u32 ) };
	// 8303BB30: 7F6BDB78  mr r11, r27
	ctx.r[11].u64 = ctx.r[27].u64;
	// 8303BB34: 814B0010  lwz r10, 0x10(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 8303BB38: 55490739  rlwinm. r9, r10, 0, 0x1c, 0x1c
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8303BB3C: 40820064  bne 0x8303bba0
	if !ctx.cr[0].eq {
	pc = 0x8303BBA0; continue 'dispatch;
	}
	// 8303BB40: 554A06F7  rlwinm. r10, r10, 0, 0x1b, 0x1b
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8303BB44: 4082005C  bne 0x8303bba0
	if !ctx.cr[0].eq {
	pc = 0x8303BBA0; continue 'dispatch;
	}
	// 8303BB48: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 8303BB4C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8303BB50: 4082FFE4  bne 0x8303bb34
	if !ctx.cr[0].eq {
	pc = 0x8303BB34; continue 'dispatch;
	}
	// 8303BB54: 7ECBB378  mr r11, r22
	ctx.r[11].u64 = ctx.r[22].u64;
	// 8303BB58: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8303BB5C: 41820054  beq 0x8303bbb0
	if ctx.cr[0].eq {
	pc = 0x8303BBB0; continue 'dispatch;
	}
	// 8303BB60: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 8303BB64: 809C0000  lwz r4, 0(r28)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303BB68: 4BF9C731  bl 0x82fd8298
	ctx.lr = 0x8303BB6C;
	sub_82FD8298(ctx, base);
	// 8303BB6C: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8303BB70: 93DF0068  stw r30, 0x68(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(104 as u32), ctx.r[30].u32 ) };
	// 8303BB74: 41820034  beq 0x8303bba8
	if ctx.cr[0].eq {
	pc = 0x8303BBA8; continue 'dispatch;
	}
	// 8303BB78: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8303BB7C: 80DC0000  lwz r6, 0(r28)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303BB80: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 8303BB84: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8303BB88: 48010C69  bl 0x8304c7f0
	ctx.lr = 0x8303BB8C;
	sub_8304C7F0(ctx, base);
	// 8303BB8C: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 8303BB90: 7FCAF378  mr r10, r30
	ctx.r[10].u64 = ctx.r[30].u64;
	// 8303BB94: 396B2990  addi r11, r11, 0x2990
	ctx.r[11].s64 = ctx.r[11].s64 + 10640;
	// 8303BB98: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8303BB9C: 48000010  b 0x8303bbac
	pc = 0x8303BBAC; continue 'dispatch;
	// 8303BBA0: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8303BBA4: 4BFFFFB4  b 0x8303bb58
	pc = 0x8303BB58; continue 'dispatch;
	// 8303BBA8: 7ECAB378  mr r10, r22
	ctx.r[10].u64 = ctx.r[22].u64;
	// 8303BBAC: 915F005C  stw r10, 0x5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[10].u32 ) };
	// 8303BBB0: 57AB06F7  rlwinm. r11, r29, 0, 0x1b, 0x1b
	ctx.r[11].u64 = ctx.r[29].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8303BBB4: 418200A4  beq 0x8303bc58
	if ctx.cr[0].eq {
	pc = 0x8303BC58; continue 'dispatch;
	}
	// 8303BBB8: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303BBBC: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 8303BBC0: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 8303BBC4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8303BBC8: 4E800421  bctrl
	ctx.lr = 0x8303BBCC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8303BBCC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8303BBD0: 572B06F7  rlwinm. r11, r25, 0, 0x1b, 0x1b
	ctx.r[11].u64 = ctx.r[25].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8303BBD4: 3B200800  li r25, 0x800
	ctx.r[25].s64 = 2048;
	// 8303BBD8: 4182000C  beq 0x8303bbe4
	if ctx.cr[0].eq {
	pc = 0x8303BBE4; continue 'dispatch;
	}
	// 8303BBDC: 3B400001  li r26, 1
	ctx.r[26].s64 = 1;
	// 8303BBE0: 933F0058  stw r25, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[25].u32 ) };
	// 8303BBE4: 38600024  li r3, 0x24
	ctx.r[3].s64 = 36;
	// 8303BBE8: 809C0000  lwz r4, 0(r28)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303BBEC: 4BF9C6AD  bl 0x82fd8298
	ctx.lr = 0x8303BBF0;
	sub_82FD8298(ctx, base);
	// 8303BBF0: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8303BBF4: 93DF0068  stw r30, 0x68(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(104 as u32), ctx.r[30].u32 ) };
	// 8303BBF8: 41820040  beq 0x8303bc38
	if ctx.cr[0].eq {
	pc = 0x8303BC38; continue 'dispatch;
	}
	// 8303BBFC: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8303BC00: 82BC0000  lwz r21, 0(r28)
	ctx.r[21].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303BC04: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 8303BC08: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8303BC0C: 4BFFF855  bl 0x8303b460
	ctx.lr = 0x8303BC10;
	sub_8303B460(ctx, base);
	// 8303BC10: 7C671B78  mr r7, r3
	ctx.r[7].u64 = ctx.r[3].u64;
	// 8303BC14: 7F46D378  mr r6, r26
	ctx.r[6].u64 = ctx.r[26].u64;
	// 8303BC18: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8303BC1C: 38800800  li r4, 0x800
	ctx.r[4].s64 = 2048;
	// 8303BC20: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8303BC24: 7F08C378  mr r8, r24
	ctx.r[8].u64 = ctx.r[24].u64;
	// 8303BC28: 7EA9AB78  mr r9, r21
	ctx.r[9].u64 = ctx.r[21].u64;
	// 8303BC2C: 48045115  bl 0x83080d40
	ctx.lr = 0x8303BC30;
	sub_83080D40(ctx, base);
	// 8303BC30: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8303BC34: 48000008  b 0x8303bc3c
	pc = 0x8303BC3C; continue 'dispatch;
	// 8303BC38: 7EDEB378  mr r30, r22
	ctx.r[30].u64 = ctx.r[22].u64;
	// 8303BC3C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8303BC40: 807C0008  lwz r3, 8(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 8303BC44: 4BFFF50D  bl 0x8303b150
	ctx.lr = 0x8303BC48;
	sub_8303B150(ctx, base);
	// 8303BC48: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8303BC4C: 807F005C  lwz r3, 0x5c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 8303BC50: 4BFFF501  bl 0x8303b150
	ctx.lr = 0x8303BC54;
	sub_8303B150(ctx, base);
	// 8303BC54: 933F0050  stw r25, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[25].u32 ) };
	// 8303BC58: 809B0020  lwz r4, 0x20(r27)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(32 as u32) ) } as u64;
	// 8303BC5C: 28040000  cmplwi r4, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8303BC60: 4182040C  beq 0x8303c06c
	if ctx.cr[0].eq {
	pc = 0x8303C06C; continue 'dispatch;
	}
	// 8303BC64: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8303BC68: 80DC0000  lwz r6, 0(r28)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303BC6C: 387F0090  addi r3, r31, 0x90
	ctx.r[3].s64 = ctx.r[31].s64 + 144;
	// 8303BC70: 4BFAAED1  bl 0x82fe6b40
	ctx.lr = 0x8303BC74;
	sub_82FE6B40(ctx, base);
	// 8303BC74: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 8303BC78: 3B2B6C68  addi r25, r11, 0x6c68
	ctx.r[25].s64 = ctx.r[11].s64 + 27752;
	// 8303BC7C: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 8303BC80: 3AEB32C8  addi r23, r11, 0x32c8
	ctx.r[23].s64 = ctx.r[11].s64 + 13000;
	// 8303BC84: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8303BC88: 3ACBCF74  addi r22, r11, -0x308c
	ctx.r[22].s64 = ctx.r[11].s64 + -12428;
	// 8303BC8C: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8303BC90: 3AABCE48  addi r21, r11, -0x31b8
	ctx.r[21].s64 = ctx.r[11].s64 + -12728;
	// 8303BC94: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8303BC98: 3A8BCFAC  addi r20, r11, -0x3054
	ctx.r[20].s64 = ctx.r[11].s64 + -12372;
	// 8303BC9C: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8303BCA0: 3A6BCF94  addi r19, r11, -0x306c
	ctx.r[19].s64 = ctx.r[11].s64 + -12396;
	// 8303BCA4: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8303BCA8: 3A4BCF00  addi r18, r11, -0x3100
	ctx.r[18].s64 = ctx.r[11].s64 + -12544;
	// 8303BCAC: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8303BCB0: 3A2BCF4C  addi r17, r11, -0x30b4
	ctx.r[17].s64 = ctx.r[11].s64 + -12468;
	// 8303BCB4: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8303BCB8: 3A0BCEB8  addi r16, r11, -0x3148
	ctx.r[16].s64 = ctx.r[11].s64 + -12616;
	// 8303BCBC: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8303BCC0: 39EBCF14  addi r15, r11, -0x30ec
	ctx.r[15].s64 = ctx.r[11].s64 + -12524;
	// 8303BCC4: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8303BCC8: 39CBCF30  addi r14, r11, -0x30d0
	ctx.r[14].s64 = ctx.r[11].s64 + -12496;
	// 8303BCCC: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8303BCD0: 396BCEC8  addi r11, r11, -0x3138
	ctx.r[11].s64 = ctx.r[11].s64 + -12600;
	// 8303BCD4: 917F0068  stw r11, 0x68(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 8303BCD8: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8303BCDC: 3B0BCEE4  addi r24, r11, -0x311c
	ctx.r[24].s64 = ctx.r[11].s64 + -12572;
	// 8303BCE0: 48000008  b 0x8303bce8
	pc = 0x8303BCE8; continue 'dispatch;
	// 8303BCE4: 837F015C  lwz r27, 0x15c(r31)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(348 as u32) ) } as u64;
	// 8303BCE8: 817F0098  lwz r11, 0x98(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(152 as u32) ) } as u64;
	// 8303BCEC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8303BCF0: 409A001C  bne cr6, 0x8303bd0c
	if !ctx.cr[6].eq {
	pc = 0x8303BD0C; continue 'dispatch;
	}
	// 8303BCF4: 817F00A0  lwz r11, 0xa0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(160 as u32) ) } as u64;
	// 8303BCF8: 815F009C  lwz r10, 0x9c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(156 as u32) ) } as u64;
	// 8303BCFC: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8303BD00: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8303BD04: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8303BD08: 419A0008  beq cr6, 0x8303bd10
	if ctx.cr[6].eq {
	pc = 0x8303BD10; continue 'dispatch;
	}
	// 8303BD0C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8303BD10: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8303BD14: 41820344  beq 0x8303c058
	if ctx.cr[0].eq {
	pc = 0x8303C058; continue 'dispatch;
	}
	// 8303BD18: 387F0090  addi r3, r31, 0x90
	ctx.r[3].s64 = ctx.r[31].s64 + 144;
	// 8303BD1C: 4BFD1C5D  bl 0x8300d978
	ctx.lr = 0x8303BD20;
	sub_8300D978(ctx, base);
	// 8303BD20: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8303BD24: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8303BD28: 809F0164  lwz r4, 0x164(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(356 as u32) ) } as u64;
	// 8303BD2C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8303BD30: 83DD0008  lwz r30, 8(r29)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 8303BD34: 4BFFF72D  bl 0x8303b460
	ctx.lr = 0x8303BD38;
	sub_8303B460(ctx, base);
	// 8303BD38: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 8303BD3C: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 8303BD40: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8303BD44: 4BF97EFD  bl 0x82fd3c40
	ctx.lr = 0x8303BD48;
	sub_82FD3C40(ctx, base);
	// 8303BD48: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8303BD4C: 41820014  beq 0x8303bd60
	if ctx.cr[0].eq {
	pc = 0x8303BD60; continue 'dispatch;
	}
	// 8303BD50: 817F0054  lwz r11, 0x54(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 8303BD54: 3BC00020  li r30, 0x20
	ctx.r[30].s64 = 32;
	// 8303BD58: 557ADFFE  rlwinm r26, r11, 0x1b, 0x1f, 0x1f
	ctx.r[26].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8303BD5C: 48000144  b 0x8303bea0
	pc = 0x8303BEA0; continue 'dispatch;
	// 8303BD60: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8303BD64: 809F0068  lwz r4, 0x68(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) } as u64;
	// 8303BD68: 4BF97ED9  bl 0x82fd3c40
	ctx.lr = 0x8303BD6C;
	sub_82FD3C40(ctx, base);
	// 8303BD6C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8303BD70: 41820014  beq 0x8303bd84
	if ctx.cr[0].eq {
	pc = 0x8303BD84; continue 'dispatch;
	}
	// 8303BD74: 817F0054  lwz r11, 0x54(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 8303BD78: 3BC00040  li r30, 0x40
	ctx.r[30].s64 = 64;
	// 8303BD7C: 557AD7FE  rlwinm r26, r11, 0x1a, 0x1f, 0x1f
	ctx.r[26].u64 = ctx.r[11].u32 as u64 & 0x0000003Fu64;
	// 8303BD80: 48000120  b 0x8303bea0
	pc = 0x8303BEA0; continue 'dispatch;
	// 8303BD84: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8303BD88: 7DC47378  mr r4, r14
	ctx.r[4].u64 = ctx.r[14].u64;
	// 8303BD8C: 4BF97EB5  bl 0x82fd3c40
	ctx.lr = 0x8303BD90;
	sub_82FD3C40(ctx, base);
	// 8303BD90: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8303BD94: 41820014  beq 0x8303bda8
	if ctx.cr[0].eq {
	pc = 0x8303BDA8; continue 'dispatch;
	}
	// 8303BD98: 817F0054  lwz r11, 0x54(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 8303BD9C: 3BC00100  li r30, 0x100
	ctx.r[30].s64 = 256;
	// 8303BDA0: 557ACFFE  rlwinm r26, r11, 0x19, 0x1f, 0x1f
	ctx.r[26].u64 = ctx.r[11].u32 as u64 & 0x0000007Fu64;
	// 8303BDA4: 480000FC  b 0x8303bea0
	pc = 0x8303BEA0; continue 'dispatch;
	// 8303BDA8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8303BDAC: 7DE47B78  mr r4, r15
	ctx.r[4].u64 = ctx.r[15].u64;
	// 8303BDB0: 4BF97E91  bl 0x82fd3c40
	ctx.lr = 0x8303BDB4;
	sub_82FD3C40(ctx, base);
	// 8303BDB4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8303BDB8: 41820014  beq 0x8303bdcc
	if ctx.cr[0].eq {
	pc = 0x8303BDCC; continue 'dispatch;
	}
	// 8303BDBC: 817F0054  lwz r11, 0x54(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 8303BDC0: 3BC00080  li r30, 0x80
	ctx.r[30].s64 = 128;
	// 8303BDC4: 557AC7FE  rlwinm r26, r11, 0x18, 0x1f, 0x1f
	ctx.r[26].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 8303BDC8: 480000D8  b 0x8303bea0
	pc = 0x8303BEA0; continue 'dispatch;
	// 8303BDCC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8303BDD0: 7E048378  mr r4, r16
	ctx.r[4].u64 = ctx.r[16].u64;
	// 8303BDD4: 4BF97E6D  bl 0x82fd3c40
	ctx.lr = 0x8303BDD8;
	sub_82FD3C40(ctx, base);
	// 8303BDD8: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8303BDDC: 41820014  beq 0x8303bdf0
	if ctx.cr[0].eq {
	pc = 0x8303BDF0; continue 'dispatch;
	}
	// 8303BDE0: 817F0054  lwz r11, 0x54(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 8303BDE4: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 8303BDE8: 557A07FE  clrlwi r26, r11, 0x1f
	ctx.r[26].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	// 8303BDEC: 480000B4  b 0x8303bea0
	pc = 0x8303BEA0; continue 'dispatch;
	// 8303BDF0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8303BDF4: 7E248B78  mr r4, r17
	ctx.r[4].u64 = ctx.r[17].u64;
	// 8303BDF8: 4BF97E49  bl 0x82fd3c40
	ctx.lr = 0x8303BDFC;
	sub_82FD3C40(ctx, base);
	// 8303BDFC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8303BE00: 41820014  beq 0x8303be14
	if ctx.cr[0].eq {
	pc = 0x8303BE14; continue 'dispatch;
	}
	// 8303BE04: 817F0054  lwz r11, 0x54(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 8303BE08: 3BC00002  li r30, 2
	ctx.r[30].s64 = 2;
	// 8303BE0C: 557AFFFE  rlwinm r26, r11, 0x1f, 0x1f, 0x1f
	ctx.r[26].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	// 8303BE10: 48000090  b 0x8303bea0
	pc = 0x8303BEA0; continue 'dispatch;
	// 8303BE14: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8303BE18: 7E449378  mr r4, r18
	ctx.r[4].u64 = ctx.r[18].u64;
	// 8303BE1C: 4BF97E25  bl 0x82fd3c40
	ctx.lr = 0x8303BE20;
	sub_82FD3C40(ctx, base);
	// 8303BE20: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8303BE24: 41820014  beq 0x8303be38
	if ctx.cr[0].eq {
	pc = 0x8303BE38; continue 'dispatch;
	}
	// 8303BE28: 817F0054  lwz r11, 0x54(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 8303BE2C: 3BC00004  li r30, 4
	ctx.r[30].s64 = 4;
	// 8303BE30: 557AF7FE  rlwinm r26, r11, 0x1e, 0x1f, 0x1f
	ctx.r[26].u64 = ctx.r[11].u32 as u64 & 0x00000003u64;
	// 8303BE34: 4800006C  b 0x8303bea0
	pc = 0x8303BEA0; continue 'dispatch;
	// 8303BE38: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8303BE3C: 7E649B78  mr r4, r19
	ctx.r[4].u64 = ctx.r[19].u64;
	// 8303BE40: 4BF97E01  bl 0x82fd3c40
	ctx.lr = 0x8303BE44;
	sub_82FD3C40(ctx, base);
	// 8303BE44: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8303BE48: 41820014  beq 0x8303be5c
	if ctx.cr[0].eq {
	pc = 0x8303BE5C; continue 'dispatch;
	}
	// 8303BE4C: 817F0054  lwz r11, 0x54(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 8303BE50: 3BC00200  li r30, 0x200
	ctx.r[30].s64 = 512;
	// 8303BE54: 557ABFFE  rlwinm r26, r11, 0x17, 0x1f, 0x1f
	ctx.r[26].u64 = ctx.r[11].u32 as u64 & 0x000001FFu64;
	// 8303BE58: 48000048  b 0x8303bea0
	pc = 0x8303BEA0; continue 'dispatch;
	// 8303BE5C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8303BE60: 7E84A378  mr r4, r20
	ctx.r[4].u64 = ctx.r[20].u64;
	// 8303BE64: 4BF97DDD  bl 0x82fd3c40
	ctx.lr = 0x8303BE68;
	sub_82FD3C40(ctx, base);
	// 8303BE68: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8303BE6C: 41820014  beq 0x8303be80
	if ctx.cr[0].eq {
	pc = 0x8303BE80; continue 'dispatch;
	}
	// 8303BE70: 817F0054  lwz r11, 0x54(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 8303BE74: 3BC00400  li r30, 0x400
	ctx.r[30].s64 = 1024;
	// 8303BE78: 557AB7FE  rlwinm r26, r11, 0x16, 0x1f, 0x1f
	ctx.r[26].u64 = ctx.r[11].u32 as u64 & 0x000003FFu64;
	// 8303BE7C: 48000024  b 0x8303bea0
	pc = 0x8303BEA0; continue 'dispatch;
	// 8303BE80: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8303BE84: 7EA4AB78  mr r4, r21
	ctx.r[4].u64 = ctx.r[21].u64;
	// 8303BE88: 4BF97DB9  bl 0x82fd3c40
	ctx.lr = 0x8303BE8C;
	sub_82FD3C40(ctx, base);
	// 8303BE8C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8303BE90: 4182008C  beq 0x8303bf1c
	if ctx.cr[0].eq {
	pc = 0x8303BF1C; continue 'dispatch;
	}
	// 8303BE94: 817F0054  lwz r11, 0x54(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 8303BE98: 3BC00010  li r30, 0x10
	ctx.r[30].s64 = 16;
	// 8303BE9C: 557A97FE  rlwinm r26, r11, 0x12, 0x1f, 0x1f
	ctx.r[26].u64 = ctx.r[11].u32 as u64 & 0x00003FFFu64;
	// 8303BEA0: 38600024  li r3, 0x24
	ctx.r[3].s64 = 36;
	// 8303BEA4: 809C0000  lwz r4, 0(r28)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303BEA8: 4BF9C3F1  bl 0x82fd8298
	ctx.lr = 0x8303BEAC;
	sub_82FD8298(ctx, base);
	// 8303BEAC: 907F006C  stw r3, 0x6c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(108 as u32), ctx.r[3].u32 ) };
	// 8303BEB0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8303BEB4: 41820028  beq 0x8303bedc
	if ctx.cr[0].eq {
	pc = 0x8303BEDC; continue 'dispatch;
	}
	// 8303BEB8: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 8303BEBC: 80BD0010  lwz r5, 0x10(r29)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 8303BEC0: 7F46D378  mr r6, r26
	ctx.r[6].u64 = ctx.r[26].u64;
	// 8303BEC4: 811F0164  lwz r8, 0x164(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(356 as u32) ) } as u64;
	// 8303BEC8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8303BECC: 813C0000  lwz r9, 0(r28)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303BED0: 48044D21  bl 0x83080bf0
	ctx.lr = 0x8303BED4;
	sub_83080BF0(ctx, base);
	// 8303BED4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8303BED8: 48000008  b 0x8303bee0
	pc = 0x8303BEE0; continue 'dispatch;
	// 8303BEDC: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 8303BEE0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8303BEE4: 807C0008  lwz r3, 8(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 8303BEE8: 4BFFF269  bl 0x8303b150
	ctx.lr = 0x8303BEEC;
	sub_8303B150(ctx, base);
	// 8303BEEC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8303BEF0: 807F0060  lwz r3, 0x60(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 8303BEF4: 4BFFF25D  bl 0x8303b150
	ctx.lr = 0x8303BEF8;
	sub_8303B150(ctx, base);
	// 8303BEF8: 815F0050  lwz r10, 0x50(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8303BEFC: 7FCA5378  or r10, r30, r10
	ctx.r[10].u64 = ctx.r[30].u64 | ctx.r[10].u64;
	// 8303BF00: 915F0050  stw r10, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 8303BF04: 574B063F  clrlwi. r11, r26, 0x18
	ctx.r[11].u64 = ctx.r[26].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8303BF08: 4182FDDC  beq 0x8303bce4
	if ctx.cr[0].eq {
	pc = 0x8303BCE4; continue 'dispatch;
	}
	// 8303BF0C: 817F0058  lwz r11, 0x58(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 8303BF10: 7FCB5B78  or r11, r30, r11
	ctx.r[11].u64 = ctx.r[30].u64 | ctx.r[11].u64;
	// 8303BF14: 917F0058  stw r11, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 8303BF18: 48000138  b 0x8303c050
	pc = 0x8303C050; continue 'dispatch;
	// 8303BF1C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8303BF20: 7EC4B378  mr r4, r22
	ctx.r[4].u64 = ctx.r[22].u64;
	// 8303BF24: 4BF97D1D  bl 0x82fd3c40
	ctx.lr = 0x8303BF28;
	sub_82FD3C40(ctx, base);
	// 8303BF28: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8303BF2C: 4182FDB8  beq 0x8303bce4
	if ctx.cr[0].eq {
	pc = 0x8303BCE4; continue 'dispatch;
	}
	// 8303BF30: 817F015C  lwz r11, 0x15c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(348 as u32) ) } as u64;
	// 8303BF34: 387F0070  addi r3, r31, 0x70
	ctx.r[3].s64 = ctx.r[31].s64 + 112;
	// 8303BF38: 7EE5BB78  mr r5, r23
	ctx.r[5].u64 = ctx.r[23].u64;
	// 8303BF3C: 80DC0000  lwz r6, 0(r28)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303BF40: 808B0024  lwz r4, 0x24(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 8303BF44: 4BFF8725  bl 0x83034668
	ctx.lr = 0x8303BF48;
	sub_83034668(ctx, base);
	// 8303BF48: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 8303BF4C: 809C0000  lwz r4, 0(r28)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303BF50: 4BF9C349  bl 0x82fd8298
	ctx.lr = 0x8303BF54;
	sub_82FD8298(ctx, base);
	// 8303BF54: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8303BF58: 93DF006C  stw r30, 0x6c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(108 as u32), ctx.r[30].u32 ) };
	// 8303BF5C: 4182002C  beq 0x8303bf88
	if ctx.cr[0].eq {
	pc = 0x8303BF88; continue 'dispatch;
	}
	// 8303BF60: 387F0070  addi r3, r31, 0x70
	ctx.r[3].s64 = ctx.r[31].s64 + 112;
	// 8303BF64: 83BC0000  lwz r29, 0(r28)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303BF68: 4BFF8489  bl 0x830343f0
	ctx.lr = 0x8303BF6C;
	sub_830343F0(ctx, base);
	// 8303BF6C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8303BF70: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 8303BF74: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8303BF78: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8303BF7C: 4BF96AC5  bl 0x82fd2a40
	ctx.lr = 0x8303BF80;
	sub_82FD2A40(ctx, base);
	// 8303BF80: 933E0000  stw r25, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[25].u32 ) };
	// 8303BF84: 48000008  b 0x8303bf8c
	pc = 0x8303BF8C; continue 'dispatch;
	// 8303BF88: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8303BF8C: 387F0070  addi r3, r31, 0x70
	ctx.r[3].s64 = ctx.r[31].s64 + 112;
	// 8303BF90: 93DF0064  stw r30, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[30].u32 ) };
	// 8303BF94: 48000028  b 0x8303bfbc
	pc = 0x8303BFBC; continue 'dispatch;
	// 8303BF98: 387F0070  addi r3, r31, 0x70
	ctx.r[3].s64 = ctx.r[31].s64 + 112;
	// 8303BF9C: 83BC0000  lwz r29, 0(r28)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303BFA0: 4BFF8869  bl 0x83034808
	ctx.lr = 0x8303BFA4;
	sub_83034808(ctx, base);
	// 8303BFA4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8303BFA8: 4BF94BD9  bl 0x82fd0b80
	ctx.lr = 0x8303BFAC;
	sub_82FD0B80(ctx, base);
	// 8303BFAC: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8303BFB0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8303BFB4: 4BFFF19D  bl 0x8303b150
	ctx.lr = 0x8303BFB8;
	sub_8303B150(ctx, base);
	// 8303BFB8: 387F0070  addi r3, r31, 0x70
	ctx.r[3].s64 = ctx.r[31].s64 + 112;
	// 8303BFBC: 4BFF84CD  bl 0x83034488
	ctx.lr = 0x8303BFC0;
	sub_83034488(ctx, base);
	// 8303BFC0: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8303BFC4: 4082FFD4  bne 0x8303bf98
	if !ctx.cr[0].eq {
	pc = 0x8303BF98; continue 'dispatch;
	}
	// 8303BFC8: 817F0054  lwz r11, 0x54(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 8303BFCC: 556B0739  rlwinm. r11, r11, 0, 0x1c, 0x1c
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8303BFD0: 41820014  beq 0x8303bfe4
	if ctx.cr[0].eq {
	pc = 0x8303BFE4; continue 'dispatch;
	}
	// 8303BFD4: 817F0058  lwz r11, 0x58(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 8303BFD8: 3B400001  li r26, 1
	ctx.r[26].s64 = 1;
	// 8303BFDC: 616B0008  ori r11, r11, 8
	ctx.r[11].u64 = ctx.r[11].u64 | 8;
	// 8303BFE0: 917F0058  stw r11, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 8303BFE4: 38600024  li r3, 0x24
	ctx.r[3].s64 = 36;
	// 8303BFE8: 809C0000  lwz r4, 0(r28)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303BFEC: 4BF9C2AD  bl 0x82fd8298
	ctx.lr = 0x8303BFF0;
	sub_82FD8298(ctx, base);
	// 8303BFF0: 907F006C  stw r3, 0x6c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(108 as u32), ctx.r[3].u32 ) };
	// 8303BFF4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8303BFF8: 41820028  beq 0x8303c020
	if ctx.cr[0].eq {
	pc = 0x8303C020; continue 'dispatch;
	}
	// 8303BFFC: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 8303C000: 811F0164  lwz r8, 0x164(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(356 as u32) ) } as u64;
	// 8303C004: 7F46D378  mr r6, r26
	ctx.r[6].u64 = ctx.r[26].u64;
	// 8303C008: 813C0000  lwz r9, 0(r28)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303C00C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8303C010: 38800008  li r4, 8
	ctx.r[4].s64 = 8;
	// 8303C014: 48044D2D  bl 0x83080d40
	ctx.lr = 0x8303C018;
	sub_83080D40(ctx, base);
	// 8303C018: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8303C01C: 48000008  b 0x8303c024
	pc = 0x8303C024; continue 'dispatch;
	// 8303C020: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8303C024: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8303C028: 807C0008  lwz r3, 8(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 8303C02C: 4BFFF125  bl 0x8303b150
	ctx.lr = 0x8303C030;
	sub_8303B150(ctx, base);
	// 8303C030: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8303C034: 807F005C  lwz r3, 0x5c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 8303C038: 4BFFF119  bl 0x8303b150
	ctx.lr = 0x8303C03C;
	sub_8303B150(ctx, base);
	// 8303C03C: 817F0050  lwz r11, 0x50(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8303C040: 616B0008  ori r11, r11, 8
	ctx.r[11].u64 = ctx.r[11].u64 | 8;
	// 8303C044: 917F0050  stw r11, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8303C048: 387F0070  addi r3, r31, 0x70
	ctx.r[3].s64 = ctx.r[31].s64 + 112;
	// 8303C04C: 4BFF8435  bl 0x83034480
	ctx.lr = 0x8303C050;
	sub_83034480(ctx, base);
	// 8303C050: 837F015C  lwz r27, 0x15c(r31)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(348 as u32) ) } as u64;
	// 8303C054: 4BFFFC94  b 0x8303bce8
	pc = 0x8303BCE8; continue 'dispatch;
	// 8303C058: 387F0090  addi r3, r31, 0x90
	ctx.r[3].s64 = ctx.r[31].s64 + 144;
	// 8303C05C: 4BFD216D  bl 0x8300e1c8
	ctx.lr = 0x8303C060;
	sub_8300E1C8(ctx, base);
	// 8303C060: 82FF016C  lwz r23, 0x16c(r31)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(364 as u32) ) } as u64;
	// 8303C064: 831F0164  lwz r24, 0x164(r31)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(356 as u32) ) } as u64;
	// 8303C068: 3AC00000  li r22, 0
	ctx.r[22].s64 = 0;
	// 8303C06C: 83BF0050  lwz r29, 0x50(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8303C070: 57AB06F7  rlwinm. r11, r29, 0, 0x1b, 0x1b
	ctx.r[11].u64 = ctx.r[29].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8303C074: 40820078  bne 0x8303c0ec
	if !ctx.cr[0].eq {
	pc = 0x8303C0EC; continue 'dispatch;
	}
	// 8303C078: 38600024  li r3, 0x24
	ctx.r[3].s64 = 36;
	// 8303C07C: 809C0000  lwz r4, 0(r28)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303C080: 4BF9C219  bl 0x82fd8298
	ctx.lr = 0x8303C084;
	sub_82FD8298(ctx, base);
	// 8303C084: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8303C088: 93DF006C  stw r30, 0x6c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(108 as u32), ctx.r[30].u32 ) };
	// 8303C08C: 4182003C  beq 0x8303c0c8
	if ctx.cr[0].eq {
	pc = 0x8303C0C8; continue 'dispatch;
	}
	// 8303C090: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 8303C094: A09B000A  lhz r4, 0xa(r27)
	ctx.r[4].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[27].u32.wrapping_add(10 as u32) ) } as u64;
	// 8303C098: 837C0000  lwz r27, 0(r28)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303C09C: 48004E75  bl 0x83040f10
	ctx.lr = 0x8303C0A0;
	sub_83040F10(ctx, base);
	// 8303C0A0: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 8303C0A4: 38800010  li r4, 0x10
	ctx.r[4].s64 = 16;
	// 8303C0A8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8303C0AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8303C0B0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8303C0B4: 7F08C378  mr r8, r24
	ctx.r[8].u64 = ctx.r[24].u64;
	// 8303C0B8: 7F69DB78  mr r9, r27
	ctx.r[9].u64 = ctx.r[27].u64;
	// 8303C0BC: 48044B35  bl 0x83080bf0
	ctx.lr = 0x8303C0C0;
	sub_83080BF0(ctx, base);
	// 8303C0C0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8303C0C4: 48000008  b 0x8303c0cc
	pc = 0x8303C0CC; continue 'dispatch;
	// 8303C0C8: 7EDEB378  mr r30, r22
	ctx.r[30].u64 = ctx.r[22].u64;
	// 8303C0CC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8303C0D0: 807C0008  lwz r3, 8(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 8303C0D4: 4BFFF07D  bl 0x8303b150
	ctx.lr = 0x8303C0D8;
	sub_8303B150(ctx, base);
	// 8303C0D8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8303C0DC: 807F0060  lwz r3, 0x60(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 8303C0E0: 4BFFF071  bl 0x8303b150
	ctx.lr = 0x8303C0E4;
	sub_8303B150(ctx, base);
	// 8303C0E4: 63AB0010  ori r11, r29, 0x10
	ctx.r[11].u64 = ctx.r[29].u64 | 16;
	// 8303C0E8: 917F0050  stw r11, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8303C0EC: 81770000  lwz r11, 0(r23)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303C0F0: 7EE3BB78  mr r3, r23
	ctx.r[3].u64 = ctx.r[23].u64;
	// 8303C0F4: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 8303C0F8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8303C0FC: 4E800421  bctrl
	ctx.lr = 0x8303C100;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8303C100: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8303C104: 41820130  beq 0x8303c234
	if ctx.cr[0].eq {
	pc = 0x8303C234; continue 'dispatch;
	}
	// 8303C108: 81770000  lwz r11, 0(r23)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303C10C: 7EE3BB78  mr r3, r23
	ctx.r[3].u64 = ctx.r[23].u64;
	// 8303C110: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 8303C114: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8303C118: 4E800421  bctrl
	ctx.lr = 0x8303C11C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8303C11C: 81630014  lwz r11, 0x14(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 8303C120: 2F0B0010  cmpwi cr6, r11, 0x10
	ctx.cr[6].compare_i32(ctx.r[11].s32, 16, &mut ctx.xer);
	// 8303C124: 409A0110  bne cr6, 0x8303c234
	if !ctx.cr[6].eq {
	pc = 0x8303C234; continue 'dispatch;
	}
	// 8303C128: 81770000  lwz r11, 0(r23)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303C12C: 7EE3BB78  mr r3, r23
	ctx.r[3].u64 = ctx.r[23].u64;
	// 8303C130: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 8303C134: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8303C138: 4E800421  bctrl
	ctx.lr = 0x8303C13C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8303C13C: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 8303C140: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 8303C144: 83BB0030  lwz r29, 0x30(r27)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(48 as u32) ) } as u64;
	// 8303C148: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 8303C14C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8303C150: 40990064  ble cr6, 0x8303c1b4
	if !ctx.cr[6].gt {
	pc = 0x8303C1B4; continue 'dispatch;
	}
	// 8303C154: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8303C158: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8303C15C: 4BFF0715  bl 0x8302c870
	ctx.lr = 0x8303C160;
	sub_8302C870(ctx, base);
	// 8303C160: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8303C164: 815F0050  lwz r10, 0x50(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8303C168: 817E0014  lwz r11, 0x14(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 8303C16C: 7D695039  and. r9, r11, r10
	ctx.r[9].u64 = ctx.r[11].u64 & ctx.r[10].u64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8303C170: 40820034  bne 0x8303c1a4
	if !ctx.cr[0].eq {
	pc = 0x8303C1A4; continue 'dispatch;
	}
	// 8303C174: 7D6B5378  or r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 8303C178: 807F0060  lwz r3, 0x60(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 8303C17C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8303C180: 917F0050  stw r11, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8303C184: 4BFFEFCD  bl 0x8303b150
	ctx.lr = 0x8303C188;
	sub_8303B150(ctx, base);
	// 8303C188: 897E0018  lbz r11, 0x18(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 8303C18C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8303C190: 41820014  beq 0x8303c1a4
	if ctx.cr[0].eq {
	pc = 0x8303C1A4; continue 'dispatch;
	}
	// 8303C194: 815F0058  lwz r10, 0x58(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 8303C198: 817E0014  lwz r11, 0x14(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 8303C19C: 7D6B5378  or r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 8303C1A0: 917F0058  stw r11, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 8303C1A4: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 8303C1A8: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 8303C1AC: 7F1C5840  cmplw cr6, r28, r11
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8303C1B0: 4198FFA4  blt cr6, 0x8303c154
	if ctx.cr[6].lt {
	pc = 0x8303C154; continue 'dispatch;
	}
	// 8303C1B4: 839B0034  lwz r28, 0x34(r27)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(52 as u32) ) } as u64;
	// 8303C1B8: 2C1C0000  cmpwi r28, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8303C1BC: 41820074  beq 0x8303c230
	if ctx.cr[0].eq {
	pc = 0x8303C230; continue 'dispatch;
	}
	// 8303C1C0: 817C0008  lwz r11, 8(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 8303C1C4: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 8303C1C8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8303C1CC: 40990064  ble cr6, 0x8303c230
	if !ctx.cr[6].gt {
	pc = 0x8303C230; continue 'dispatch;
	}
	// 8303C1D0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8303C1D4: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8303C1D8: 4BFF0699  bl 0x8302c870
	ctx.lr = 0x8303C1DC;
	sub_8302C870(ctx, base);
	// 8303C1DC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8303C1E0: 815F0050  lwz r10, 0x50(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8303C1E4: 817E0014  lwz r11, 0x14(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 8303C1E8: 7D695039  and. r9, r11, r10
	ctx.r[9].u64 = ctx.r[11].u64 & ctx.r[10].u64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8303C1EC: 40820034  bne 0x8303c220
	if !ctx.cr[0].eq {
	pc = 0x8303C220; continue 'dispatch;
	}
	// 8303C1F0: 7D6B5378  or r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 8303C1F4: 807F005C  lwz r3, 0x5c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 8303C1F8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8303C1FC: 917F0050  stw r11, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8303C200: 4BFFEF51  bl 0x8303b150
	ctx.lr = 0x8303C204;
	sub_8303B150(ctx, base);
	// 8303C204: 897E0018  lbz r11, 0x18(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 8303C208: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8303C20C: 41820014  beq 0x8303c220
	if ctx.cr[0].eq {
	pc = 0x8303C220; continue 'dispatch;
	}
	// 8303C210: 815F0058  lwz r10, 0x58(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 8303C214: 817E0014  lwz r11, 0x14(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 8303C218: 7D6B5378  or r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 8303C21C: 917F0058  stw r11, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 8303C220: 817C0008  lwz r11, 8(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 8303C224: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 8303C228: 7F1D5840  cmplw cr6, r29, r11
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8303C22C: 4198FFA4  blt cr6, 0x8303c1d0
	if ctx.cr[6].lt {
	pc = 0x8303C1D0; continue 'dispatch;
	}
	// 8303C230: 82FF016C  lwz r23, 0x16c(r31)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(364 as u32) ) } as u64;
	// 8303C234: 7EE3BB78  mr r3, r23
	ctx.r[3].u64 = ctx.r[23].u64;
	// 8303C238: 811F0064  lwz r8, 0x64(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(100 as u32) ) } as u64;
	// 8303C23C: 80FF005C  lwz r7, 0x5c(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 8303C240: 80DF0060  lwz r6, 0x60(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 8303C244: 80BF0058  lwz r5, 0x58(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 8303C248: 809F0050  lwz r4, 0x50(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8303C24C: 48044795  bl 0x830809e0
	ctx.lr = 0x8303C250;
	sub_830809E0(ctx, base);
	// 8303C250: 383F0140  addi r1, r31, 0x140
	ctx.r[1].s64 = ctx.r[31].s64 + 320;
	// 8303C254: 4816BF2C  b 0x831a8180
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303C258(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8303C258 size=48
    let mut pc: u32 = 0x8303C258;
    'dispatch: loop {
        match pc {
            0x8303C258 => {
    //   block [0x8303C258..0x8303C288)
	// 8303C258: 3BECFEC0  addi r31, r12, -0x140
	ctx.r[31].s64 = ctx.r[12].s64 + -320;
	// 8303C25C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8303C260: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8303C264: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8303C268: 817F0154  lwz r11, 0x154(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(340 as u32) ) } as u64;
	// 8303C26C: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303C270: 807F0068  lwz r3, 0x68(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) } as u64;
	// 8303C274: 4BF9C06D  bl 0x82fd82e0
	ctx.lr = 0x8303C278;
	sub_82FD82E0(ctx, base);
	// 8303C278: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8303C27C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8303C280: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8303C284: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303C288(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8303C288 size=48
    let mut pc: u32 = 0x8303C288;
    'dispatch: loop {
        match pc {
            0x8303C288 => {
    //   block [0x8303C288..0x8303C2B8)
	// 8303C288: 3BECFEC0  addi r31, r12, -0x140
	ctx.r[31].s64 = ctx.r[12].s64 + -320;
	// 8303C28C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8303C290: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8303C294: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8303C298: 817F0154  lwz r11, 0x154(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(340 as u32) ) } as u64;
	// 8303C29C: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303C2A0: 807F0068  lwz r3, 0x68(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) } as u64;
	// 8303C2A4: 4BF9C03D  bl 0x82fd82e0
	ctx.lr = 0x8303C2A8;
	sub_82FD82E0(ctx, base);
	// 8303C2A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8303C2AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8303C2B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8303C2B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303C2B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8303C2B8 size=48
    let mut pc: u32 = 0x8303C2B8;
    'dispatch: loop {
        match pc {
            0x8303C2B8 => {
    //   block [0x8303C2B8..0x8303C2E8)
	// 8303C2B8: 3BECFEC0  addi r31, r12, -0x140
	ctx.r[31].s64 = ctx.r[12].s64 + -320;
	// 8303C2BC: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8303C2C0: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8303C2C4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8303C2C8: 817F0154  lwz r11, 0x154(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(340 as u32) ) } as u64;
	// 8303C2CC: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303C2D0: 807F0068  lwz r3, 0x68(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) } as u64;
	// 8303C2D4: 4BF9C00D  bl 0x82fd82e0
	ctx.lr = 0x8303C2D8;
	sub_82FD82E0(ctx, base);
	// 8303C2D8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8303C2DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8303C2E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8303C2E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303C2E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8303C2E8 size=40
    let mut pc: u32 = 0x8303C2E8;
    'dispatch: loop {
        match pc {
            0x8303C2E8 => {
    //   block [0x8303C2E8..0x8303C310)
	// 8303C2E8: 3BECFEC0  addi r31, r12, -0x140
	ctx.r[31].s64 = ctx.r[12].s64 + -320;
	// 8303C2EC: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8303C2F0: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8303C2F4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8303C2F8: 387F0090  addi r3, r31, 0x90
	ctx.r[3].s64 = ctx.r[31].s64 + 144;
	// 8303C2FC: 4BFD1ECD  bl 0x8300e1c8
	ctx.lr = 0x8303C300;
	sub_8300E1C8(ctx, base);
	// 8303C300: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8303C304: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8303C308: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8303C30C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303C310(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8303C310 size=48
    let mut pc: u32 = 0x8303C310;
    'dispatch: loop {
        match pc {
            0x8303C310 => {
    //   block [0x8303C310..0x8303C340)
	// 8303C310: 3BECFEC0  addi r31, r12, -0x140
	ctx.r[31].s64 = ctx.r[12].s64 + -320;
	// 8303C314: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8303C318: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8303C31C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8303C320: 817F0154  lwz r11, 0x154(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(340 as u32) ) } as u64;
	// 8303C324: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303C328: 807F006C  lwz r3, 0x6c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(108 as u32) ) } as u64;
	// 8303C32C: 4BF9BFB5  bl 0x82fd82e0
	ctx.lr = 0x8303C330;
	sub_82FD82E0(ctx, base);
	// 8303C330: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8303C334: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8303C338: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8303C33C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303C340(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8303C340 size=40
    let mut pc: u32 = 0x8303C340;
    'dispatch: loop {
        match pc {
            0x8303C340 => {
    //   block [0x8303C340..0x8303C368)
	// 8303C340: 3BECFEC0  addi r31, r12, -0x140
	ctx.r[31].s64 = ctx.r[12].s64 + -320;
	// 8303C344: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8303C348: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8303C34C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8303C350: 387F0070  addi r3, r31, 0x70
	ctx.r[3].s64 = ctx.r[31].s64 + 112;
	// 8303C354: 4BFF812D  bl 0x83034480
	ctx.lr = 0x8303C358;
	sub_83034480(ctx, base);
	// 8303C358: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8303C35C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8303C360: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8303C364: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303C368(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8303C368 size=48
    let mut pc: u32 = 0x8303C368;
    'dispatch: loop {
        match pc {
            0x8303C368 => {
    //   block [0x8303C368..0x8303C398)
	// 8303C368: 3BECFEC0  addi r31, r12, -0x140
	ctx.r[31].s64 = ctx.r[12].s64 + -320;
	// 8303C36C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8303C370: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8303C374: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8303C378: 817F0154  lwz r11, 0x154(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(340 as u32) ) } as u64;
	// 8303C37C: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303C380: 807F006C  lwz r3, 0x6c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(108 as u32) ) } as u64;
	// 8303C384: 4BF9BF5D  bl 0x82fd82e0
	ctx.lr = 0x8303C388;
	sub_82FD82E0(ctx, base);
	// 8303C388: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8303C38C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8303C390: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8303C394: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303C398(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8303C398 size=48
    let mut pc: u32 = 0x8303C398;
    'dispatch: loop {
        match pc {
            0x8303C398 => {
    //   block [0x8303C398..0x8303C3C8)
	// 8303C398: 3BECFEC0  addi r31, r12, -0x140
	ctx.r[31].s64 = ctx.r[12].s64 + -320;
	// 8303C39C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8303C3A0: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8303C3A4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8303C3A8: 817F0154  lwz r11, 0x154(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(340 as u32) ) } as u64;
	// 8303C3AC: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303C3B0: 807F006C  lwz r3, 0x6c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(108 as u32) ) } as u64;
	// 8303C3B4: 4BF9BF2D  bl 0x82fd82e0
	ctx.lr = 0x8303C3B8;
	sub_82FD82E0(ctx, base);
	// 8303C3B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8303C3BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8303C3C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8303C3C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303C3C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8303C3C8 size=48
    let mut pc: u32 = 0x8303C3C8;
    'dispatch: loop {
        match pc {
            0x8303C3C8 => {
    //   block [0x8303C3C8..0x8303C3F8)
	// 8303C3C8: 3BECFEC0  addi r31, r12, -0x140
	ctx.r[31].s64 = ctx.r[12].s64 + -320;
	// 8303C3CC: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8303C3D0: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8303C3D4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8303C3D8: 817F0154  lwz r11, 0x154(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(340 as u32) ) } as u64;
	// 8303C3DC: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303C3E0: 807F006C  lwz r3, 0x6c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(108 as u32) ) } as u64;
	// 8303C3E4: 4BF9BEFD  bl 0x82fd82e0
	ctx.lr = 0x8303C3E8;
	sub_82FD82E0(ctx, base);
	// 8303C3E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8303C3EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8303C3F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8303C3F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303C3F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8303C3F8 size=8
    let mut pc: u32 = 0x8303C3F8;
    'dispatch: loop {
        match pc {
            0x8303C3F8 => {
    //   block [0x8303C3F8..0x8303C400)
	// 8303C3F8: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8303C3FC: 8215D5E8  lwz r16, -0x2a18(r21)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-10776 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303C400(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8303C400 size=704
    let mut pc: u32 = 0x8303C400;
    'dispatch: loop {
        match pc {
            0x8303C400 => {
    //   block [0x8303C400..0x8303C6C0)
	// 8303C400: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8303C404: 4816BD45  bl 0x831a8148
	ctx.lr = 0x8303C408;
	sub_831A8130(ctx, base);
	// 8303C408: 3BE1FF30  addi r31, r1, -0xd0
	ctx.r[31].s64 = ctx.r[1].s64 + -208;
	// 8303C40C: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8303C410: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 8303C414: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8303C418: 7C962378  mr r22, r4
	ctx.r[22].u64 = ctx.r[4].u64;
	// 8303C41C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8303C420: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 8303C424: 935F00E4  stw r26, 0xe4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(228 as u32), ctx.r[26].u32 ) };
	// 8303C428: 4BFD1789  bl 0x8300dbb0
	ctx.lr = 0x8303C42C;
	sub_8300DBB0(ctx, base);
	// 8303C42C: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8303C430: 40820284  bne 0x8303c6b4
	if !ctx.cr[0].eq {
	pc = 0x8303C6B4; continue 'dispatch;
	}
	// 8303C434: 81760018  lwz r11, 0x18(r22)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(24 as u32) ) } as u64;
	// 8303C438: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 8303C43C: 8336001C  lwz r25, 0x1c(r22)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(28 as u32) ) } as u64;
	// 8303C440: 3AE00000  li r23, 0
	ctx.r[23].s64 = 0;
	// 8303C444: 3AA00001  li r21, 1
	ctx.r[21].s64 = 1;
	// 8303C448: 3A800000  li r20, 0
	ctx.r[20].s64 = 0;
	// 8303C44C: 2F0B0019  cmpwi cr6, r11, 0x19
	ctx.cr[6].compare_i32(ctx.r[11].s32, 25, &mut ctx.xer);
	// 8303C450: 409A00D4  bne cr6, 0x8303c524
	if !ctx.cr[6].eq {
	pc = 0x8303C524; continue 'dispatch;
	}
	// 8303C454: 83160048  lwz r24, 0x48(r22)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(72 as u32) ) } as u64;
	// 8303C458: 3AA00003  li r21, 3
	ctx.r[21].s64 = 3;
	// 8303C45C: 83980008  lwz r28, 8(r24)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(8 as u32) ) } as u64;
	// 8303C460: 281C0000  cmplwi r28, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8303C464: 4182008C  beq 0x8303c4f0
	if ctx.cr[0].eq {
	pc = 0x8303C4F0; continue 'dispatch;
	}
	// 8303C468: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 8303C46C: 809A0000  lwz r4, 0(r26)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303C470: 4BF9BE29  bl 0x82fd8298
	ctx.lr = 0x8303C474;
	sub_82FD8298(ctx, base);
	// 8303C474: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8303C478: 93DF0060  stw r30, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[30].u32 ) };
	// 8303C47C: 4182002C  beq 0x8303c4a8
	if ctx.cr[0].eq {
	pc = 0x8303C4A8; continue 'dispatch;
	}
	// 8303C480: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8303C484: 80DA0000  lwz r6, 0(r26)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303C488: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8303C48C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8303C490: 48010361  bl 0x8304c7f0
	ctx.lr = 0x8303C494;
	sub_8304C7F0(ctx, base);
	// 8303C494: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 8303C498: 7FDBF378  mr r27, r30
	ctx.r[27].u64 = ctx.r[30].u64;
	// 8303C49C: 396B2990  addi r11, r11, 0x2990
	ctx.r[11].s64 = ctx.r[11].s64 + 10640;
	// 8303C4A0: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8303C4A4: 48000008  b 0x8303c4ac
	pc = 0x8303C4AC; continue 'dispatch;
	// 8303C4A8: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 8303C4AC: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8303C4B0: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 8303C4B4: 419A003C  beq cr6, 0x8303c4f0
	if ctx.cr[6].eq {
	pc = 0x8303C4F0; continue 'dispatch;
	}
	// 8303C4B8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8303C4BC: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 8303C4C0: 4BFF03B1  bl 0x8302c870
	ctx.lr = 0x8303C4C4;
	sub_8302C870(ctx, base);
	// 8303C4C4: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8303C4C8: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 8303C4CC: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8303C4D0: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8303C4D4: 4BFFFF2D  bl 0x8303c400
	ctx.lr = 0x8303C4D8;
	sub_8303C400(ctx, base);
	// 8303C4D8: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8303C4DC: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 8303C4E0: 4BFFEC71  bl 0x8303b150
	ctx.lr = 0x8303C4E4;
	sub_8303B150(ctx, base);
	// 8303C4E4: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 8303C4E8: 7F1EE040  cmplw cr6, r30, r28
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[28].u32, &mut ctx.xer);
	// 8303C4EC: 4198FFCC  blt cr6, 0x8303c4b8
	if ctx.cr[6].lt {
	pc = 0x8303C4B8; continue 'dispatch;
	}
	// 8303C4F0: 2B190000  cmplwi cr6, r25, 0
	ctx.cr[6].compare_u32(ctx.r[25].u32, 0 as u32, &mut ctx.xer);
	// 8303C4F4: 419A001C  beq cr6, 0x8303c510
	if ctx.cr[6].eq {
	pc = 0x8303C510; continue 'dispatch;
	}
	// 8303C4F8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8303C4FC: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8303C500: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 8303C504: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 8303C508: 4BFFFEF9  bl 0x8303c400
	ctx.lr = 0x8303C50C;
	sub_8303C400(ctx, base);
	// 8303C50C: 48000108  b 0x8303c614
	pc = 0x8303C614; continue 'dispatch;
	// 8303C510: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8303C514: 38ABCC98  addi r5, r11, -0x3368
	ctx.r[5].s64 = ctx.r[11].s64 + -13160;
	// 8303C518: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8303C51C: 388BD8FC  addi r4, r11, -0x2704
	ctx.r[4].s64 = ctx.r[11].s64 + -9988;
	// 8303C520: 480000EC  b 0x8303c60c
	pc = 0x8303C60C; continue 'dispatch;
	// 8303C524: 2F0B0018  cmpwi cr6, r11, 0x18
	ctx.cr[6].compare_i32(ctx.r[11].s32, 24, &mut ctx.xer);
	// 8303C528: 409A0080  bne cr6, 0x8303c5a8
	if !ctx.cr[6].eq {
	pc = 0x8303C5A8; continue 'dispatch;
	}
	// 8303C52C: 81790018  lwz r11, 0x18(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(24 as u32) ) } as u64;
	// 8303C530: 3AA00002  li r21, 2
	ctx.r[21].s64 = 2;
	// 8303C534: 2F0B0018  cmpwi cr6, r11, 0x18
	ctx.cr[6].compare_i32(ctx.r[11].s32, 24, &mut ctx.xer);
	// 8303C538: 409A0038  bne cr6, 0x8303c570
	if !ctx.cr[6].eq {
	pc = 0x8303C570; continue 'dispatch;
	}
	// 8303C53C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8303C540: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8303C544: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 8303C548: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 8303C54C: 4BFFFEB5  bl 0x8303c400
	ctx.lr = 0x8303C550;
	sub_8303C400(ctx, base);
	// 8303C550: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8303C554: 817E0028  lwz r11, 0x28(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 8303C558: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 8303C55C: 409A000C  bne cr6, 0x8303c568
	if !ctx.cr[6].eq {
	pc = 0x8303C568; continue 'dispatch;
	}
	// 8303C560: 82FE003C  lwz r23, 0x3c(r30)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(60 as u32) ) } as u64;
	// 8303C564: 480000B4  b 0x8303c618
	pc = 0x8303C618; continue 'dispatch;
	// 8303C568: 3AE00000  li r23, 0
	ctx.r[23].s64 = 0;
	// 8303C56C: 480000AC  b 0x8303c618
	pc = 0x8303C618; continue 'dispatch;
	// 8303C570: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8303C574: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8303C578: 38ABCC98  addi r5, r11, -0x3368
	ctx.r[5].s64 = ctx.r[11].s64 + -13160;
	// 8303C57C: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8303C580: 388BD8FC  addi r4, r11, -0x2704
	ctx.r[4].s64 = ctx.r[11].s64 + -9988;
	// 8303C584: 4BFD1B4D  bl 0x8300e0d0
	ctx.lr = 0x8303C588;
	sub_8300E0D0(ctx, base);
	// 8303C588: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8303C58C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8303C590: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8303C594: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 8303C598: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 8303C59C: 4BFFFE65  bl 0x8303c400
	ctx.lr = 0x8303C5A0;
	sub_8303C400(ctx, base);
	// 8303C5A0: 7C771B78  mr r23, r3
	ctx.r[23].u64 = ctx.r[3].u64;
	// 8303C5A4: 48000074  b 0x8303c618
	pc = 0x8303C618; continue 'dispatch;
	// 8303C5A8: 578B063F  clrlwi. r11, r28, 0x18
	ctx.r[11].u64 = ctx.r[28].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8303C5AC: 40820050  bne 0x8303c5fc
	if !ctx.cr[0].eq {
	pc = 0x8303C5FC; continue 'dispatch;
	}
	// 8303C5B0: 2B190000  cmplwi cr6, r25, 0
	ctx.cr[6].compare_u32(ctx.r[25].u32, 0 as u32, &mut ctx.xer);
	// 8303C5B4: 419A0028  beq cr6, 0x8303c5dc
	if ctx.cr[6].eq {
	pc = 0x8303C5DC; continue 'dispatch;
	}
	// 8303C5B8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8303C5BC: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8303C5C0: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 8303C5C4: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 8303C5C8: 4BFFFE39  bl 0x8303c400
	ctx.lr = 0x8303C5CC;
	sub_8303C400(ctx, base);
	// 8303C5CC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8303C5D0: 817E0028  lwz r11, 0x28(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 8303C5D4: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8303C5D8: 4BFFFF84  b 0x8303c55c
	pc = 0x8303C55C; continue 'dispatch;
	// 8303C5DC: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8303C5E0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8303C5E4: 38ABCC98  addi r5, r11, -0x3368
	ctx.r[5].s64 = ctx.r[11].s64 + -13160;
	// 8303C5E8: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8303C5EC: 388BD8FC  addi r4, r11, -0x2704
	ctx.r[4].s64 = ctx.r[11].s64 + -9988;
	// 8303C5F0: 4BFD1AE1  bl 0x8300e0d0
	ctx.lr = 0x8303C5F4;
	sub_8300E0D0(ctx, base);
	// 8303C5F4: 3A800001  li r20, 1
	ctx.r[20].s64 = 1;
	// 8303C5F8: 4800001C  b 0x8303c614
	pc = 0x8303C614; continue 'dispatch;
	// 8303C5FC: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8303C600: 38ABCC98  addi r5, r11, -0x3368
	ctx.r[5].s64 = ctx.r[11].s64 + -13160;
	// 8303C604: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8303C608: 388BD5D4  addi r4, r11, -0x2a2c
	ctx.r[4].s64 = ctx.r[11].s64 + -10796;
	// 8303C60C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8303C610: 4BFD1AC1  bl 0x8300e0d0
	ctx.lr = 0x8303C614;
	sub_8300E0D0(ctx, base);
	// 8303C614: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8303C618: 38600048  li r3, 0x48
	ctx.r[3].s64 = 72;
	// 8303C61C: 809A0000  lwz r4, 0(r26)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303C620: 4BF9BC79  bl 0x82fd8298
	ctx.lr = 0x8303C624;
	sub_82FD8298(ctx, base);
	// 8303C624: 7C7C1B79  or. r28, r3, r3
	ctx.r[28].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 8303C628: 939F0060  stw r28, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[28].u32 ) };
	// 8303C62C: 41820048  beq 0x8303c674
	if ctx.cr[0].eq {
	pc = 0x8303C674; continue 'dispatch;
	}
	// 8303C630: 7EC5B378  mr r5, r22
	ctx.r[5].u64 = ctx.r[22].u64;
	// 8303C634: 833A0000  lwz r25, 0(r26)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303C638: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8303C63C: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 8303C640: 4BFFEE21  bl 0x8303b460
	ctx.lr = 0x8303C644;
	sub_8303B460(ctx, base);
	// 8303C644: 7C691B78  mr r9, r3
	ctx.r[9].u64 = ctx.r[3].u64;
	// 8303C648: 7F68DB78  mr r8, r27
	ctx.r[8].u64 = ctx.r[27].u64;
	// 8303C64C: 93210054  stw r25, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[25].u32 ) };
	// 8303C650: 7EE7BB78  mr r7, r23
	ctx.r[7].u64 = ctx.r[23].u64;
	// 8303C654: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 8303C658: 7EA5AB78  mr r5, r21
	ctx.r[5].u64 = ctx.r[21].u64;
	// 8303C65C: 7EC4B378  mr r4, r22
	ctx.r[4].u64 = ctx.r[22].u64;
	// 8303C660: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8303C664: 7FAAEB78  mr r10, r29
	ctx.r[10].u64 = ctx.r[29].u64;
	// 8303C668: 480443E9  bl 0x83080a50
	ctx.lr = 0x8303C66C;
	sub_83080A50(ctx, base);
	// 8303C66C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8303C670: 48000008  b 0x8303c678
	pc = 0x8303C678; continue 'dispatch;
	// 8303C674: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8303C678: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8303C67C: 7EC4B378  mr r4, r22
	ctx.r[4].u64 = ctx.r[22].u64;
	// 8303C680: 807A0004  lwz r3, 4(r26)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 8303C684: 4BFF07C5  bl 0x8302ce48
	ctx.lr = 0x8303C688;
	sub_8302CE48(ctx, base);
	// 8303C688: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8303C68C: 807A0008  lwz r3, 8(r26)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 8303C690: 4BFFEAC1  bl 0x8303b150
	ctx.lr = 0x8303C694;
	sub_8303B150(ctx, base);
	// 8303C694: 568B063F  clrlwi. r11, r20, 0x18
	ctx.r[11].u64 = ctx.r[20].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8303C698: 41820008  beq 0x8303c6a0
	if ctx.cr[0].eq {
	pc = 0x8303C6A0; continue 'dispatch;
	}
	// 8303C69C: 93DE003C  stw r30, 0x3c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(60 as u32), ctx.r[30].u32 ) };
	// 8303C6A0: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 8303C6A4: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8303C6A8: 7EC4B378  mr r4, r22
	ctx.r[4].u64 = ctx.r[22].u64;
	// 8303C6AC: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 8303C6B0: 4BFFF3E9  bl 0x8303ba98
	ctx.lr = 0x8303C6B4;
	sub_8303BA98(ctx, base);
	// 8303C6B4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8303C6B8: 383F00D0  addi r1, r31, 0xd0
	ctx.r[1].s64 = ctx.r[31].s64 + 208;
	// 8303C6BC: 4816BADC  b 0x831a8198
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303C6C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8303C6C0 size=48
    let mut pc: u32 = 0x8303C6C0;
    'dispatch: loop {
        match pc {
            0x8303C6C0 => {
    //   block [0x8303C6C0..0x8303C6F0)
	// 8303C6C0: 3BECFF30  addi r31, r12, -0xd0
	ctx.r[31].s64 = ctx.r[12].s64 + -208;
	// 8303C6C4: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8303C6C8: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8303C6CC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8303C6D0: 817F00E4  lwz r11, 0xe4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(228 as u32) ) } as u64;
	// 8303C6D4: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303C6D8: 807F0060  lwz r3, 0x60(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 8303C6DC: 4BF9BC05  bl 0x82fd82e0
	ctx.lr = 0x8303C6E0;
	sub_82FD82E0(ctx, base);
	// 8303C6E0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8303C6E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8303C6E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8303C6EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303C6F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8303C6F0 size=48
    let mut pc: u32 = 0x8303C6F0;
    'dispatch: loop {
        match pc {
            0x8303C6F0 => {
    //   block [0x8303C6F0..0x8303C720)
	// 8303C6F0: 3BECFF30  addi r31, r12, -0xd0
	ctx.r[31].s64 = ctx.r[12].s64 + -208;
	// 8303C6F4: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8303C6F8: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8303C6FC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8303C700: 817F00E4  lwz r11, 0xe4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(228 as u32) ) } as u64;
	// 8303C704: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303C708: 807F0060  lwz r3, 0x60(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 8303C70C: 4BF9BBD5  bl 0x82fd82e0
	ctx.lr = 0x8303C710;
	sub_82FD82E0(ctx, base);
	// 8303C710: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8303C714: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8303C718: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8303C71C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303C720(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8303C720 size=8
    let mut pc: u32 = 0x8303C720;
    'dispatch: loop {
        match pc {
            0x8303C720 => {
    //   block [0x8303C720..0x8303C728)
	// 8303C720: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8303C724: 8215D640  lwz r16, -0x29c0(r21)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-10688 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303C728(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8303C728 size=304
    let mut pc: u32 = 0x8303C728;
    'dispatch: loop {
        match pc {
            0x8303C728 => {
    //   block [0x8303C728..0x8303C858)
	// 8303C728: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8303C72C: 4816BA29  bl 0x831a8154
	ctx.lr = 0x8303C730;
	sub_831A8130(ctx, base);
	// 8303C730: 3BE1FF50  addi r31, r1, -0xb0
	ctx.r[31].s64 = ctx.r[1].s64 + -176;
	// 8303C734: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8303C738: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 8303C73C: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 8303C740: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8303C744: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 8303C748: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 8303C74C: 939F00C4  stw r28, 0xc4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(196 as u32), ctx.r[28].u32 ) };
	// 8303C750: 4BFD1461  bl 0x8300dbb0
	ctx.lr = 0x8303C754;
	sub_8300DBB0(ctx, base);
	// 8303C754: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 8303C758: 4182002C  beq 0x8303c784
	if ctx.cr[0].eq {
	pc = 0x8303C784; continue 'dispatch;
	}
	// 8303C75C: 817D0020  lwz r11, 0x20(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(32 as u32) ) } as u64;
	// 8303C760: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 8303C764: 409A00E8  bne cr6, 0x8303c84c
	if !ctx.cr[6].eq {
	pc = 0x8303C84C; continue 'dispatch;
	}
	// 8303C768: 817D0024  lwz r11, 0x24(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(36 as u32) ) } as u64;
	// 8303C76C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8303C770: 409A00DC  bne cr6, 0x8303c84c
	if !ctx.cr[6].eq {
	pc = 0x8303C84C; continue 'dispatch;
	}
	// 8303C774: 2B1A0000  cmplwi cr6, r26, 0
	ctx.cr[6].compare_u32(ctx.r[26].u32, 0 as u32, &mut ctx.xer);
	// 8303C778: 419A00D4  beq cr6, 0x8303c84c
	if ctx.cr[6].eq {
	pc = 0x8303C84C; continue 'dispatch;
	}
	// 8303C77C: 935D0024  stw r26, 0x24(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(36 as u32), ctx.r[26].u32 ) };
	// 8303C780: 480000CC  b 0x8303c84c
	pc = 0x8303C84C; continue 'dispatch;
	// 8303C784: 809E002C  lwz r4, 0x2c(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(44 as u32) ) } as u64;
	// 8303C788: 3B000000  li r24, 0
	ctx.r[24].s64 = 0;
	// 8303C78C: 28040000  cmplwi r4, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8303C790: 41820018  beq 0x8303c7a8
	if ctx.cr[0].eq {
	pc = 0x8303C7A8; continue 'dispatch;
	}
	// 8303C794: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8303C798: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 8303C79C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8303C7A0: 4BFFFC61  bl 0x8303c400
	ctx.lr = 0x8303C7A4;
	sub_8303C400(ctx, base);
	// 8303C7A4: 7C781B78  mr r24, r3
	ctx.r[24].u64 = ctx.r[3].u64;
	// 8303C7A8: 817E0044  lwz r11, 0x44(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(68 as u32) ) } as u64;
	// 8303C7AC: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 8303C7B0: 3B200000  li r25, 0
	ctx.r[25].s64 = 0;
	// 8303C7B4: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8303C7B8: 409A000C  bne cr6, 0x8303c7c4
	if !ctx.cr[6].eq {
	pc = 0x8303C7C4; continue 'dispatch;
	}
	// 8303C7BC: 3BA00001  li r29, 1
	ctx.r[29].s64 = 1;
	// 8303C7C0: 48000014  b 0x8303c7d4
	pc = 0x8303C7D4; continue 'dispatch;
	// 8303C7C4: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 8303C7C8: 409A000C  bne cr6, 0x8303c7d4
	if !ctx.cr[6].eq {
	pc = 0x8303C7D4; continue 'dispatch;
	}
	// 8303C7CC: 3BA00002  li r29, 2
	ctx.r[29].s64 = 2;
	// 8303C7D0: 7F59D378  mr r25, r26
	ctx.r[25].u64 = ctx.r[26].u64;
	// 8303C7D4: 38600028  li r3, 0x28
	ctx.r[3].s64 = 40;
	// 8303C7D8: 809C0000  lwz r4, 0(r28)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303C7DC: 4BF9BABD  bl 0x82fd8298
	ctx.lr = 0x8303C7E0;
	sub_82FD8298(ctx, base);
	// 8303C7E0: 7C7A1B79  or. r26, r3, r3
	ctx.r[26].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[26].s32, 0, &mut ctx.xer);
	// 8303C7E4: 935F0050  stw r26, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[26].u32 ) };
	// 8303C7E8: 41820044  beq 0x8303c82c
	if ctx.cr[0].eq {
	pc = 0x8303C82C; continue 'dispatch;
	}
	// 8303C7EC: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8303C7F0: 82FC0000  lwz r23, 0(r28)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303C7F4: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8303C7F8: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8303C7FC: 4BFFEC65  bl 0x8303b460
	ctx.lr = 0x8303C800;
	sub_8303B460(ctx, base);
	// 8303C800: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 8303C804: 7F05C378  mr r5, r24
	ctx.r[5].u64 = ctx.r[24].u64;
	// 8303C808: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8303C80C: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 8303C810: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 8303C814: 7FA8EB78  mr r8, r29
	ctx.r[8].u64 = ctx.r[29].u64;
	// 8303C818: 7F29CB78  mr r9, r25
	ctx.r[9].u64 = ctx.r[25].u64;
	// 8303C81C: 7EEABB78  mr r10, r23
	ctx.r[10].u64 = ctx.r[23].u64;
	// 8303C820: 48044651  bl 0x83080e70
	ctx.lr = 0x8303C824;
	sub_83080E70(ctx, base);
	// 8303C824: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8303C828: 48000008  b 0x8303c830
	pc = 0x8303C830; continue 'dispatch;
	// 8303C82C: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 8303C830: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8303C834: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8303C838: 807C0004  lwz r3, 4(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 8303C83C: 4BFF060D  bl 0x8302ce48
	ctx.lr = 0x8303C840;
	sub_8302CE48(ctx, base);
	// 8303C840: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8303C844: 807C0008  lwz r3, 8(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 8303C848: 4BFFE909  bl 0x8303b150
	ctx.lr = 0x8303C84C;
	sub_8303B150(ctx, base);
	// 8303C84C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8303C850: 383F00B0  addi r1, r31, 0xb0
	ctx.r[1].s64 = ctx.r[31].s64 + 176;
	// 8303C854: 4816B950  b 0x831a81a4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303C858(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8303C858 size=48
    let mut pc: u32 = 0x8303C858;
    'dispatch: loop {
        match pc {
            0x8303C858 => {
    //   block [0x8303C858..0x8303C888)
	// 8303C858: 3BECFF50  addi r31, r12, -0xb0
	ctx.r[31].s64 = ctx.r[12].s64 + -176;
	// 8303C85C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8303C860: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8303C864: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8303C868: 817F00C4  lwz r11, 0xc4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(196 as u32) ) } as u64;
	// 8303C86C: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303C870: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8303C874: 4BF9BA6D  bl 0x82fd82e0
	ctx.lr = 0x8303C878;
	sub_82FD82E0(ctx, base);
	// 8303C878: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8303C87C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8303C880: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8303C884: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303C888(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8303C888 size=8
    let mut pc: u32 = 0x8303C888;
    'dispatch: loop {
        match pc {
            0x8303C888 => {
    //   block [0x8303C888..0x8303C890)
	// 8303C888: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8303C88C: 8215D690  lwz r16, -0x2970(r21)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-10608 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303C890(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8303C890 size=884
    let mut pc: u32 = 0x8303C890;
    'dispatch: loop {
        match pc {
            0x8303C890 => {
    //   block [0x8303C890..0x8303CC04)
	// 8303C890: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8303C894: 4816B8B1  bl 0x831a8144
	ctx.lr = 0x8303C898;
	sub_831A8130(ctx, base);
	// 8303C898: 3BE1FF20  addi r31, r1, -0xe0
	ctx.r[31].s64 = ctx.r[1].s64 + -224;
	// 8303C89C: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8303C8A0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8303C8A4: 7CB82B78  mr r24, r5
	ctx.r[24].u64 = ctx.r[5].u64;
	// 8303C8A8: 7C972378  mr r23, r4
	ctx.r[23].u64 = ctx.r[4].u64;
	// 8303C8AC: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 8303C8B0: 93BF00F4  stw r29, 0xf4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(244 as u32), ctx.r[29].u32 ) };
	// 8303C8B4: 4BFD12FD  bl 0x8300dbb0
	ctx.lr = 0x8303C8B8;
	sub_8300DBB0(ctx, base);
	// 8303C8B8: 7C791B79  or. r25, r3, r3
	ctx.r[25].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[25].s32, 0, &mut ctx.xer);
	// 8303C8BC: 4082033C  bne 0x8303cbf8
	if !ctx.cr[0].eq {
	pc = 0x8303CBF8; continue 'dispatch;
	}
	// 8303C8C0: 80970040  lwz r4, 0x40(r23)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(64 as u32) ) } as u64;
	// 8303C8C4: 3AC00000  li r22, 0
	ctx.r[22].s64 = 0;
	// 8303C8C8: 3B200000  li r25, 0
	ctx.r[25].s64 = 0;
	// 8303C8CC: 3AA00000  li r21, 0
	ctx.r[21].s64 = 0;
	// 8303C8D0: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 8303C8D4: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 8303C8D8: 28040000  cmplwi r4, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8303C8DC: 41820014  beq 0x8303c8f0
	if ctx.cr[0].eq {
	pc = 0x8303C8F0; continue 'dispatch;
	}
	// 8303C8E0: 7F05C378  mr r5, r24
	ctx.r[5].u64 = ctx.r[24].u64;
	// 8303C8E4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8303C8E8: 4BFFEF79  bl 0x8303b860
	ctx.lr = 0x8303C8EC;
	sub_8303B860(ctx, base);
	// 8303C8EC: 7C761B78  mr r22, r3
	ctx.r[22].u64 = ctx.r[3].u64;
	// 8303C8F0: 81770020  lwz r11, 0x20(r23)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(32 as u32) ) } as u64;
	// 8303C8F4: 2F0B0005  cmpwi cr6, r11, 5
	ctx.cr[6].compare_i32(ctx.r[11].s32, 5, &mut ctx.xer);
	// 8303C8F8: 409A0024  bne cr6, 0x8303c91c
	if !ctx.cr[6].eq {
	pc = 0x8303C91C; continue 'dispatch;
	}
	// 8303C8FC: 80970034  lwz r4, 0x34(r23)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(52 as u32) ) } as u64;
	// 8303C900: 28040000  cmplwi r4, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8303C904: 41820018  beq 0x8303c91c
	if ctx.cr[0].eq {
	pc = 0x8303C91C; continue 'dispatch;
	}
	// 8303C908: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8303C90C: 7F05C378  mr r5, r24
	ctx.r[5].u64 = ctx.r[24].u64;
	// 8303C910: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8303C914: 4BFFFAED  bl 0x8303c400
	ctx.lr = 0x8303C918;
	sub_8303C400(ctx, base);
	// 8303C918: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 8303C91C: 7EE3BB78  mr r3, r23
	ctx.r[3].u64 = ctx.r[23].u64;
	// 8303C920: 4BFFE769  bl 0x8303b088
	ctx.lr = 0x8303C924;
	sub_8303B088(ctx, base);
	// 8303C924: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8303C928: 4182006C  beq 0x8303c994
	if ctx.cr[0].eq {
	pc = 0x8303C994; continue 'dispatch;
	}
	// 8303C92C: 7EE3BB78  mr r3, r23
	ctx.r[3].u64 = ctx.r[23].u64;
	// 8303C930: 48003901  bl 0x83040230
	ctx.lr = 0x8303C934;
	sub_83040230(ctx, base);
	// 8303C934: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303C938: 816B0030  lwz r11, 0x30(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 8303C93C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8303C940: 4E800421  bctrl
	ctx.lr = 0x8303C944;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8303C944: 7C741B78  mr r20, r3
	ctx.r[20].u64 = ctx.r[3].u64;
	// 8303C948: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 8303C94C: 809D0000  lwz r4, 0(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303C950: 4BF9B949  bl 0x82fd8298
	ctx.lr = 0x8303C954;
	sub_82FD8298(ctx, base);
	// 8303C954: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8303C958: 93DF0060  stw r30, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[30].u32 ) };
	// 8303C95C: 4182002C  beq 0x8303c988
	if ctx.cr[0].eq {
	pc = 0x8303C988; continue 'dispatch;
	}
	// 8303C960: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8303C964: 80DD0000  lwz r6, 0(r29)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303C968: 7E84A378  mr r4, r20
	ctx.r[4].u64 = ctx.r[20].u64;
	// 8303C96C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8303C970: 4800FE81  bl 0x8304c7f0
	ctx.lr = 0x8303C974;
	sub_8304C7F0(ctx, base);
	// 8303C974: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 8303C978: 7FCAF378  mr r10, r30
	ctx.r[10].u64 = ctx.r[30].u64;
	// 8303C97C: 396B2990  addi r11, r11, 0x2990
	ctx.r[11].s64 = ctx.r[11].s64 + 10640;
	// 8303C980: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8303C984: 48000008  b 0x8303c98c
	pc = 0x8303C98C; continue 'dispatch;
	// 8303C988: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8303C98C: 7D555378  mr r21, r10
	ctx.r[21].u64 = ctx.r[10].u64;
	// 8303C990: 48000008  b 0x8303c998
	pc = 0x8303C998; continue 'dispatch;
	// 8303C994: 829F0060  lwz r20, 0x60(r31)
	ctx.r[20].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 8303C998: 80970038  lwz r4, 0x38(r23)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(56 as u32) ) } as u64;
	// 8303C99C: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 8303C9A0: 7F04B840  cmplw cr6, r4, r23
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[23].u32, &mut ctx.xer);
	// 8303C9A4: 409A000C  bne cr6, 0x8303c9b0
	if !ctx.cr[6].eq {
	pc = 0x8303C9B0; continue 'dispatch;
	}
	// 8303C9A8: 3B600001  li r27, 1
	ctx.r[27].s64 = 1;
	// 8303C9AC: 48000058  b 0x8303ca04
	pc = 0x8303CA04; continue 'dispatch;
	// 8303C9B0: 28040000  cmplwi r4, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8303C9B4: 41820014  beq 0x8303c9c8
	if ctx.cr[0].eq {
	pc = 0x8303C9C8; continue 'dispatch;
	}
	// 8303C9B8: 7F05C378  mr r5, r24
	ctx.r[5].u64 = ctx.r[24].u64;
	// 8303C9BC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8303C9C0: 4BFFFED1  bl 0x8303c890
	ctx.lr = 0x8303C9C4;
	sub_8303C890(ctx, base);
	// 8303C9C4: 4800003C  b 0x8303ca00
	pc = 0x8303CA00; continue 'dispatch;
	// 8303C9C8: 80970030  lwz r4, 0x30(r23)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(48 as u32) ) } as u64;
	// 8303C9CC: 28040000  cmplwi r4, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8303C9D0: 41820018  beq 0x8303c9e8
	if ctx.cr[0].eq {
	pc = 0x8303C9E8; continue 'dispatch;
	}
	// 8303C9D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8303C9D8: 7F05C378  mr r5, r24
	ctx.r[5].u64 = ctx.r[24].u64;
	// 8303C9DC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8303C9E0: 4BFFFA21  bl 0x8303c400
	ctx.lr = 0x8303C9E4;
	sub_8303C400(ctx, base);
	// 8303C9E4: 4800001C  b 0x8303ca00
	pc = 0x8303CA00; continue 'dispatch;
	// 8303C9E8: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8303C9EC: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 8303C9F0: 38ABCC98  addi r5, r11, -0x3368
	ctx.r[5].s64 = ctx.r[11].s64 + -13160;
	// 8303C9F4: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8303C9F8: 388BD5D4  addi r4, r11, -0x2a2c
	ctx.r[4].s64 = ctx.r[11].s64 + -10796;
	// 8303C9FC: 4BFD16D5  bl 0x8300e0d0
	ctx.lr = 0x8303CA00;
	sub_8300E0D0(ctx, base);
	// 8303CA00: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 8303CA04: 8097003C  lwz r4, 0x3c(r23)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(60 as u32) ) } as u64;
	// 8303CA08: 28040000  cmplwi r4, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8303CA0C: 41820014  beq 0x8303ca20
	if ctx.cr[0].eq {
	pc = 0x8303CA20; continue 'dispatch;
	}
	// 8303CA10: 7F05C378  mr r5, r24
	ctx.r[5].u64 = ctx.r[24].u64;
	// 8303CA14: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8303CA18: 48000969  bl 0x8303d380
	ctx.lr = 0x8303CA1C;
	sub_8303D380(ctx, base);
	// 8303CA1C: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 8303CA20: 3860003C  li r3, 0x3c
	ctx.r[3].s64 = 60;
	// 8303CA24: 809D0000  lwz r4, 0(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303CA28: 4BF9B871  bl 0x82fd8298
	ctx.lr = 0x8303CA2C;
	sub_82FD8298(ctx, base);
	// 8303CA2C: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8303CA30: 93DF0060  stw r30, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[30].u32 ) };
	// 8303CA34: 4182004C  beq 0x8303ca80
	if ctx.cr[0].eq {
	pc = 0x8303CA80; continue 'dispatch;
	}
	// 8303CA38: 7EE5BB78  mr r5, r23
	ctx.r[5].u64 = ctx.r[23].u64;
	// 8303CA3C: 827D0000  lwz r19, 0(r29)
	ctx.r[19].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303CA40: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 8303CA44: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8303CA48: 4BFFEA19  bl 0x8303b460
	ctx.lr = 0x8303CA4C;
	sub_8303B460(ctx, base);
	// 8303CA4C: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 8303CA50: 7F49D378  mr r9, r26
	ctx.r[9].u64 = ctx.r[26].u64;
	// 8303CA54: 93010054  stw r24, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[24].u32 ) };
	// 8303CA58: 7F88E378  mr r8, r28
	ctx.r[8].u64 = ctx.r[28].u64;
	// 8303CA5C: 9261005C  stw r19, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[19].u32 ) };
	// 8303CA60: 7EA7AB78  mr r7, r21
	ctx.r[7].u64 = ctx.r[21].u64;
	// 8303CA64: 7F26CB78  mr r6, r25
	ctx.r[6].u64 = ctx.r[25].u64;
	// 8303CA68: 7EC5B378  mr r5, r22
	ctx.r[5].u64 = ctx.r[22].u64;
	// 8303CA6C: 7EE4BB78  mr r4, r23
	ctx.r[4].u64 = ctx.r[23].u64;
	// 8303CA70: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8303CA74: 4804474D  bl 0x830811c0
	ctx.lr = 0x8303CA78;
	sub_830811C0(ctx, base);
	// 8303CA78: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 8303CA7C: 48000008  b 0x8303ca84
	pc = 0x8303CA84; continue 'dispatch;
	// 8303CA80: 3B200000  li r25, 0
	ctx.r[25].s64 = 0;
	// 8303CA84: 7F25CB78  mr r5, r25
	ctx.r[5].u64 = ctx.r[25].u64;
	// 8303CA88: 7EE4BB78  mr r4, r23
	ctx.r[4].u64 = ctx.r[23].u64;
	// 8303CA8C: 807D0004  lwz r3, 4(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 8303CA90: 4BFF03B9  bl 0x8302ce48
	ctx.lr = 0x8303CA94;
	sub_8302CE48(ctx, base);
	// 8303CA94: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 8303CA98: 807D0008  lwz r3, 8(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 8303CA9C: 4BFFE6B5  bl 0x8303b150
	ctx.lr = 0x8303CAA0;
	sub_8303B150(ctx, base);
	// 8303CAA0: 576B063F  clrlwi. r11, r27, 0x18
	ctx.r[11].u64 = ctx.r[27].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8303CAA4: 41820008  beq 0x8303caac
	if ctx.cr[0].eq {
	pc = 0x8303CAAC; continue 'dispatch;
	}
	// 8303CAA8: 9339001C  stw r25, 0x1c(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(28 as u32), ctx.r[25].u32 ) };
	// 8303CAAC: 7EE3BB78  mr r3, r23
	ctx.r[3].u64 = ctx.r[23].u64;
	// 8303CAB0: 4BFFE5D9  bl 0x8303b088
	ctx.lr = 0x8303CAB4;
	sub_8303B088(ctx, base);
	// 8303CAB4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8303CAB8: 418200C4  beq 0x8303cb7c
	if ctx.cr[0].eq {
	pc = 0x8303CB7C; continue 'dispatch;
	}
	// 8303CABC: 7EE3BB78  mr r3, r23
	ctx.r[3].u64 = ctx.r[23].u64;
	// 8303CAC0: 48003771  bl 0x83040230
	ctx.lr = 0x8303CAC4;
	sub_83040230(ctx, base);
	// 8303CAC4: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 8303CAC8: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 8303CACC: 2B140000  cmplwi cr6, r20, 0
	ctx.cr[6].compare_u32(ctx.r[20].u32, 0 as u32, &mut ctx.xer);
	// 8303CAD0: 419A00AC  beq cr6, 0x8303cb7c
	if ctx.cr[6].eq {
	pc = 0x8303CB7C; continue 'dispatch;
	}
	// 8303CAD4: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303CAD8: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8303CADC: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 8303CAE0: 816B0038  lwz r11, 0x38(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) } as u64;
	// 8303CAE4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8303CAE8: 4E800421  bctrl
	ctx.lr = 0x8303CAEC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8303CAEC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8303CAF0: 7F05C378  mr r5, r24
	ctx.r[5].u64 = ctx.r[24].u64;
	// 8303CAF4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8303CAF8: 809E0048  lwz r4, 0x48(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(72 as u32) ) } as u64;
	// 8303CAFC: 2C040000  cmpwi r4, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8303CB00: 41820024  beq 0x8303cb24
	if ctx.cr[0].eq {
	pc = 0x8303CB24; continue 'dispatch;
	}
	// 8303CB04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8303CB08: 4BFFFC21  bl 0x8303c728
	ctx.lr = 0x8303CB0C;
	sub_8303C728(ctx, base);
	// 8303CB0C: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 8303CB10: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8303CB14: 807D0004  lwz r3, 4(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 8303CB18: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 8303CB1C: 4BFF032D  bl 0x8302ce48
	ctx.lr = 0x8303CB20;
	sub_8302CE48(ctx, base);
	// 8303CB20: 48000014  b 0x8303cb34
	pc = 0x8303CB34; continue 'dispatch;
	// 8303CB24: 7F26CB78  mr r6, r25
	ctx.r[6].u64 = ctx.r[25].u64;
	// 8303CB28: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8303CB2C: 4BFFFBFD  bl 0x8303c728
	ctx.lr = 0x8303CB30;
	sub_8303C728(ctx, base);
	// 8303CB30: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 8303CB34: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8303CB38: 2F0B0008  cmpwi cr6, r11, 8
	ctx.cr[6].compare_i32(ctx.r[11].s32, 8, &mut ctx.xer);
	// 8303CB3C: 419A0034  beq cr6, 0x8303cb70
	if ctx.cr[6].eq {
	pc = 0x8303CB70; continue 'dispatch;
	}
	// 8303CB40: 7F05C378  mr r5, r24
	ctx.r[5].u64 = ctx.r[24].u64;
	// 8303CB44: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8303CB48: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8303CB4C: 4BFFE875  bl 0x8303b3c0
	ctx.lr = 0x8303CB50;
	sub_8303B3C0(ctx, base);
	// 8303CB50: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 8303CB54: 7EA3AB78  mr r3, r21
	ctx.r[3].u64 = ctx.r[21].u64;
	// 8303CB58: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8303CB5C: 4BFFE5F5  bl 0x8303b150
	ctx.lr = 0x8303CB60;
	sub_8303B150(ctx, base);
	// 8303CB60: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 8303CB64: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8303CB68: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8303CB6C: 4BFFE3ED  bl 0x8303af58
	ctx.lr = 0x8303CB70;
	sub_8303AF58(ctx, base);
	// 8303CB70: 3B7B0001  addi r27, r27, 1
	ctx.r[27].s64 = ctx.r[27].s64 + 1;
	// 8303CB74: 7F1BA040  cmplw cr6, r27, r20
	ctx.cr[6].compare_u32(ctx.r[27].u32, ctx.r[20].u32, &mut ctx.xer);
	// 8303CB78: 4198FF5C  blt cr6, 0x8303cad4
	if ctx.cr[6].lt {
	pc = 0x8303CAD4; continue 'dispatch;
	}
	// 8303CB7C: 81770048  lwz r11, 0x48(r23)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(72 as u32) ) } as u64;
	// 8303CB80: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8303CB84: 4182000C  beq 0x8303cb90
	if ctx.cr[0].eq {
	pc = 0x8303CB90; continue 'dispatch;
	}
	// 8303CB88: 838B0008  lwz r28, 8(r11)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8303CB8C: 48000008  b 0x8303cb94
	pc = 0x8303CB94; continue 'dispatch;
	// 8303CB90: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 8303CB94: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8303CB98: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 8303CB9C: 419A005C  beq cr6, 0x8303cbf8
	if ctx.cr[6].eq {
	pc = 0x8303CBF8; continue 'dispatch;
	}
	// 8303CBA0: 80770048  lwz r3, 0x48(r23)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(72 as u32) ) } as u64;
	// 8303CBA4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8303CBA8: 4082000C  bne 0x8303cbb4
	if !ctx.cr[0].eq {
	pc = 0x8303CBB4; continue 'dispatch;
	}
	// 8303CBAC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8303CBB0: 48000010  b 0x8303cbc0
	pc = 0x8303CBC0; continue 'dispatch;
	// 8303CBB4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8303CBB8: 4BFEFCB9  bl 0x8302c870
	ctx.lr = 0x8303CBBC;
	sub_8302C870(ctx, base);
	// 8303CBBC: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8303CBC0: 81640024  lwz r11, 0x24(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(36 as u32) ) } as u64;
	// 8303CBC4: 81570018  lwz r10, 0x18(r23)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(24 as u32) ) } as u64;
	// 8303CBC8: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 8303CBCC: 409A0020  bne cr6, 0x8303cbec
	if !ctx.cr[6].eq {
	pc = 0x8303CBEC; continue 'dispatch;
	}
	// 8303CBD0: 8164001C  lwz r11, 0x1c(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(28 as u32) ) } as u64;
	// 8303CBD4: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 8303CBD8: 409A0014  bne cr6, 0x8303cbec
	if !ctx.cr[6].eq {
	pc = 0x8303CBEC; continue 'dispatch;
	}
	// 8303CBDC: 7F26CB78  mr r6, r25
	ctx.r[6].u64 = ctx.r[25].u64;
	// 8303CBE0: 7F05C378  mr r5, r24
	ctx.r[5].u64 = ctx.r[24].u64;
	// 8303CBE4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8303CBE8: 480002A9  bl 0x8303ce90
	ctx.lr = 0x8303CBEC;
	sub_8303CE90(ctx, base);
	// 8303CBEC: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 8303CBF0: 7F1EE040  cmplw cr6, r30, r28
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[28].u32, &mut ctx.xer);
	// 8303CBF4: 4198FFAC  blt cr6, 0x8303cba0
	if ctx.cr[6].lt {
	pc = 0x8303CBA0; continue 'dispatch;
	}
	// 8303CBF8: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 8303CBFC: 383F00E0  addi r1, r31, 0xe0
	ctx.r[1].s64 = ctx.r[31].s64 + 224;
	// 8303CC00: 4816B594  b 0x831a8194
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303CC04(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8303CC04 size=48
    let mut pc: u32 = 0x8303CC04;
    'dispatch: loop {
        match pc {
            0x8303CC04 => {
    //   block [0x8303CC04..0x8303CC34)
	// 8303CC04: 3BECFF20  addi r31, r12, -0xe0
	ctx.r[31].s64 = ctx.r[12].s64 + -224;
	// 8303CC08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8303CC0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8303CC10: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8303CC14: 817F00F4  lwz r11, 0xf4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(244 as u32) ) } as u64;
	// 8303CC18: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303CC1C: 807F0060  lwz r3, 0x60(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 8303CC20: 4BF9B6C1  bl 0x82fd82e0
	ctx.lr = 0x8303CC24;
	sub_82FD82E0(ctx, base);
	// 8303CC24: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8303CC28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8303CC2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8303CC30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303CC34(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8303CC34 size=48
    let mut pc: u32 = 0x8303CC34;
    'dispatch: loop {
        match pc {
            0x8303CC34 => {
    //   block [0x8303CC34..0x8303CC64)
	// 8303CC34: 3BECFF20  addi r31, r12, -0xe0
	ctx.r[31].s64 = ctx.r[12].s64 + -224;
	// 8303CC38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8303CC3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8303CC40: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8303CC44: 817F00F4  lwz r11, 0xf4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(244 as u32) ) } as u64;
	// 8303CC48: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303CC4C: 807F0060  lwz r3, 0x60(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 8303CC50: 4BF9B691  bl 0x82fd82e0
	ctx.lr = 0x8303CC54;
	sub_82FD82E0(ctx, base);
	// 8303CC54: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8303CC58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8303CC5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8303CC60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303CC68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8303CC68 size=8
    let mut pc: u32 = 0x8303CC68;
    'dispatch: loop {
        match pc {
            0x8303CC68 => {
    //   block [0x8303CC68..0x8303CC70)
	// 8303CC68: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8303CC6C: 8215D6F0  lwz r16, -0x2910(r21)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-10512 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303CC70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8303CC70 size=436
    let mut pc: u32 = 0x8303CC70;
    'dispatch: loop {
        match pc {
            0x8303CC70 => {
    //   block [0x8303CC70..0x8303CE24)
	// 8303CC70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8303CC74: 4816B4DD  bl 0x831a8150
	ctx.lr = 0x8303CC78;
	sub_831A8130(ctx, base);
	// 8303CC78: 3BE1FF50  addi r31, r1, -0xb0
	ctx.r[31].s64 = ctx.r[1].s64 + -176;
	// 8303CC7C: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8303CC80: 7C972378  mr r23, r4
	ctx.r[23].u64 = ctx.r[4].u64;
	// 8303CC84: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8303CC88: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 8303CC8C: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 8303CC90: 3AC00000  li r22, 0
	ctx.r[22].s64 = 0;
	// 8303CC94: 81770010  lwz r11, 0x10(r23)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(16 as u32) ) } as u64;
	// 8303CC98: 93DF00C4  stw r30, 0xc4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(196 as u32), ctx.r[30].u32 ) };
	// 8303CC9C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8303CCA0: 4182000C  beq 0x8303ccac
	if ctx.cr[0].eq {
	pc = 0x8303CCAC; continue 'dispatch;
	}
	// 8303CCA4: 830B0008  lwz r24, 8(r11)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8303CCA8: 48000008  b 0x8303ccb0
	pc = 0x8303CCB0; continue 'dispatch;
	// 8303CCAC: 3B000000  li r24, 0
	ctx.r[24].s64 = 0;
	// 8303CCB0: 2B180000  cmplwi cr6, r24, 0
	ctx.cr[6].compare_u32(ctx.r[24].u32, 0 as u32, &mut ctx.xer);
	// 8303CCB4: 419A00E4  beq cr6, 0x8303cd98
	if ctx.cr[6].eq {
	pc = 0x8303CD98; continue 'dispatch;
	}
	// 8303CCB8: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 8303CCBC: 809E0000  lwz r4, 0(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303CCC0: 4BF9B5D9  bl 0x82fd8298
	ctx.lr = 0x8303CCC4;
	sub_82FD8298(ctx, base);
	// 8303CCC4: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 8303CCC8: 93BF0050  stw r29, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[29].u32 ) };
	// 8303CCCC: 4182002C  beq 0x8303ccf8
	if ctx.cr[0].eq {
	pc = 0x8303CCF8; continue 'dispatch;
	}
	// 8303CCD0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8303CCD4: 80DE0000  lwz r6, 0(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303CCD8: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 8303CCDC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8303CCE0: 4800FB11  bl 0x8304c7f0
	ctx.lr = 0x8303CCE4;
	sub_8304C7F0(ctx, base);
	// 8303CCE4: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 8303CCE8: 7FBAEB78  mr r26, r29
	ctx.r[26].u64 = ctx.r[29].u64;
	// 8303CCEC: 396B2990  addi r11, r11, 0x2990
	ctx.r[11].s64 = ctx.r[11].s64 + 10640;
	// 8303CCF0: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8303CCF4: 48000008  b 0x8303ccfc
	pc = 0x8303CCFC; continue 'dispatch;
	// 8303CCF8: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 8303CCFC: 3B200000  li r25, 0
	ctx.r[25].s64 = 0;
	// 8303CD00: 2B180000  cmplwi cr6, r24, 0
	ctx.cr[6].compare_u32(ctx.r[24].u32, 0 as u32, &mut ctx.xer);
	// 8303CD04: 419A0094  beq cr6, 0x8303cd98
	if ctx.cr[6].eq {
	pc = 0x8303CD98; continue 'dispatch;
	}
	// 8303CD08: 80770010  lwz r3, 0x10(r23)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(16 as u32) ) } as u64;
	// 8303CD0C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8303CD10: 41820014  beq 0x8303cd24
	if ctx.cr[0].eq {
	pc = 0x8303CD24; continue 'dispatch;
	}
	// 8303CD14: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 8303CD18: 4BFEFB59  bl 0x8302c870
	ctx.lr = 0x8303CD1C;
	sub_8302C870(ctx, base);
	// 8303CD1C: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 8303CD20: 48000008  b 0x8303cd28
	pc = 0x8303CD28; continue 'dispatch;
	// 8303CD24: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 8303CD28: 809C0048  lwz r4, 0x48(r28)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(72 as u32) ) } as u64;
	// 8303CD2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8303CD30: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 8303CD34: 28040000  cmplwi r4, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8303CD38: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8303CD3C: 40820008  bne 0x8303cd44
	if !ctx.cr[0].eq {
	pc = 0x8303CD44; continue 'dispatch;
	}
	// 8303CD40: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8303CD44: 4BFFF9E5  bl 0x8303c728
	ctx.lr = 0x8303CD48;
	sub_8303C728(ctx, base);
	// 8303CD48: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8303CD4C: 419A0040  beq cr6, 0x8303cd8c
	if ctx.cr[6].eq {
	pc = 0x8303CD8C; continue 'dispatch;
	}
	// 8303CD50: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 8303CD54: 2F0B0008  cmpwi cr6, r11, 8
	ctx.cr[6].compare_i32(ctx.r[11].s32, 8, &mut ctx.xer);
	// 8303CD58: 419A0034  beq cr6, 0x8303cd8c
	if ctx.cr[6].eq {
	pc = 0x8303CD8C; continue 'dispatch;
	}
	// 8303CD5C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8303CD60: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 8303CD64: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8303CD68: 4BFFE659  bl 0x8303b3c0
	ctx.lr = 0x8303CD6C;
	sub_8303B3C0(ctx, base);
	// 8303CD6C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8303CD70: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 8303CD74: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8303CD78: 4BFFE3D9  bl 0x8303b150
	ctx.lr = 0x8303CD7C;
	sub_8303B150(ctx, base);
	// 8303CD7C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8303CD80: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8303CD84: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8303CD88: 4BFFE1D1  bl 0x8303af58
	ctx.lr = 0x8303CD8C;
	sub_8303AF58(ctx, base);
	// 8303CD8C: 3B390001  addi r25, r25, 1
	ctx.r[25].s64 = ctx.r[25].s64 + 1;
	// 8303CD90: 7F19C040  cmplw cr6, r25, r24
	ctx.cr[6].compare_u32(ctx.r[25].u32, ctx.r[24].u32, &mut ctx.xer);
	// 8303CD94: 4198FF74  blt cr6, 0x8303cd08
	if ctx.cr[6].lt {
	pc = 0x8303CD08; continue 'dispatch;
	}
	// 8303CD98: 80970018  lwz r4, 0x18(r23)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(24 as u32) ) } as u64;
	// 8303CD9C: 28040000  cmplwi r4, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8303CDA0: 41820014  beq 0x8303cdb4
	if ctx.cr[0].eq {
	pc = 0x8303CDB4; continue 'dispatch;
	}
	// 8303CDA4: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 8303CDA8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8303CDAC: 4BFFEAB5  bl 0x8303b860
	ctx.lr = 0x8303CDB0;
	sub_8303B860(ctx, base);
	// 8303CDB0: 7C761B78  mr r22, r3
	ctx.r[22].u64 = ctx.r[3].u64;
	// 8303CDB4: 38600024  li r3, 0x24
	ctx.r[3].s64 = 36;
	// 8303CDB8: 809E0000  lwz r4, 0(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303CDBC: 4BF9B4DD  bl 0x82fd8298
	ctx.lr = 0x8303CDC0;
	sub_82FD8298(ctx, base);
	// 8303CDC0: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 8303CDC4: 93BF0050  stw r29, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[29].u32 ) };
	// 8303CDC8: 41820040  beq 0x8303ce08
	if ctx.cr[0].eq {
	pc = 0x8303CE08; continue 'dispatch;
	}
	// 8303CDCC: 7EE5BB78  mr r5, r23
	ctx.r[5].u64 = ctx.r[23].u64;
	// 8303CDD0: 839E0000  lwz r28, 0(r30)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303CDD4: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8303CDD8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8303CDDC: 4BFFE685  bl 0x8303b460
	ctx.lr = 0x8303CDE0;
	sub_8303B460(ctx, base);
	// 8303CDE0: 7C671B78  mr r7, r3
	ctx.r[7].u64 = ctx.r[3].u64;
	// 8303CDE4: 7EC6B378  mr r6, r22
	ctx.r[6].u64 = ctx.r[22].u64;
	// 8303CDE8: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 8303CDEC: 7EE4BB78  mr r4, r23
	ctx.r[4].u64 = ctx.r[23].u64;
	// 8303CDF0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8303CDF4: 7F68DB78  mr r8, r27
	ctx.r[8].u64 = ctx.r[27].u64;
	// 8303CDF8: 7F89E378  mr r9, r28
	ctx.r[9].u64 = ctx.r[28].u64;
	// 8303CDFC: 48044565  bl 0x83081360
	ctx.lr = 0x8303CE00;
	sub_83081360(ctx, base);
	// 8303CE00: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8303CE04: 48000008  b 0x8303ce0c
	pc = 0x8303CE0C; continue 'dispatch;
	// 8303CE08: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 8303CE0C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8303CE10: 807E0008  lwz r3, 8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8303CE14: 4BFFE33D  bl 0x8303b150
	ctx.lr = 0x8303CE18;
	sub_8303B150(ctx, base);
	// 8303CE18: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8303CE1C: 383F00B0  addi r1, r31, 0xb0
	ctx.r[1].s64 = ctx.r[31].s64 + 176;
	// 8303CE20: 4816B380  b 0x831a81a0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303CE24(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8303CE24 size=48
    let mut pc: u32 = 0x8303CE24;
    'dispatch: loop {
        match pc {
            0x8303CE24 => {
    //   block [0x8303CE24..0x8303CE54)
	// 8303CE24: 3BECFF50  addi r31, r12, -0xb0
	ctx.r[31].s64 = ctx.r[12].s64 + -176;
	// 8303CE28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8303CE2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8303CE30: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8303CE34: 817F00C4  lwz r11, 0xc4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(196 as u32) ) } as u64;
	// 8303CE38: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303CE3C: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8303CE40: 4BF9B4A1  bl 0x82fd82e0
	ctx.lr = 0x8303CE44;
	sub_82FD82E0(ctx, base);
	// 8303CE44: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8303CE48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8303CE4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8303CE50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303CE54(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8303CE54 size=48
    let mut pc: u32 = 0x8303CE54;
    'dispatch: loop {
        match pc {
            0x8303CE54 => {
    //   block [0x8303CE54..0x8303CE84)
	// 8303CE54: 3BECFF50  addi r31, r12, -0xb0
	ctx.r[31].s64 = ctx.r[12].s64 + -176;
	// 8303CE58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8303CE5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8303CE60: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8303CE64: 817F00C4  lwz r11, 0xc4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(196 as u32) ) } as u64;
	// 8303CE68: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303CE6C: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8303CE70: 4BF9B471  bl 0x82fd82e0
	ctx.lr = 0x8303CE74;
	sub_82FD82E0(ctx, base);
	// 8303CE74: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8303CE78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8303CE7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8303CE80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303CE88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8303CE88 size=8
    let mut pc: u32 = 0x8303CE88;
    'dispatch: loop {
        match pc {
            0x8303CE88 => {
    //   block [0x8303CE88..0x8303CE90)
	// 8303CE88: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8303CE8C: 8215D750  lwz r16, -0x28b0(r21)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-10416 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303CE90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8303CE90 size=640
    let mut pc: u32 = 0x8303CE90;
    'dispatch: loop {
        match pc {
            0x8303CE90 => {
    //   block [0x8303CE90..0x8303D110)
	// 8303CE90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8303CE94: 4816B2B5  bl 0x831a8148
	ctx.lr = 0x8303CE98;
	sub_831A8130(ctx, base);
	// 8303CE98: 3BE1FF30  addi r31, r1, -0xd0
	ctx.r[31].s64 = ctx.r[1].s64 + -208;
	// 8303CE9C: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8303CEA0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8303CEA4: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 8303CEA8: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 8303CEAC: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 8303CEB0: 7CD53378  mr r21, r6
	ctx.r[21].u64 = ctx.r[6].u64;
	// 8303CEB4: 93BF00E4  stw r29, 0xe4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(228 as u32), ctx.r[29].u32 ) };
	// 8303CEB8: 4BFD0CF9  bl 0x8300dbb0
	ctx.lr = 0x8303CEBC;
	sub_8300DBB0(ctx, base);
	// 8303CEBC: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8303CEC0: 41820020  beq 0x8303cee0
	if ctx.cr[0].eq {
	pc = 0x8303CEE0; continue 'dispatch;
	}
	// 8303CEC4: 817E0024  lwz r11, 0x24(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 8303CEC8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8303CECC: 409A0238  bne cr6, 0x8303d104
	if !ctx.cr[6].eq {
	pc = 0x8303D104; continue 'dispatch;
	}
	// 8303CED0: 2B150000  cmplwi cr6, r21, 0
	ctx.cr[6].compare_u32(ctx.r[21].u32, 0 as u32, &mut ctx.xer);
	// 8303CED4: 419A0230  beq cr6, 0x8303d104
	if ctx.cr[6].eq {
	pc = 0x8303D104; continue 'dispatch;
	}
	// 8303CED8: 92BE0024  stw r21, 0x24(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(36 as u32), ctx.r[21].u32 ) };
	// 8303CEDC: 48000228  b 0x8303d104
	pc = 0x8303D104; continue 'dispatch;
	// 8303CEE0: 809B0050  lwz r4, 0x50(r27)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(80 as u32) ) } as u64;
	// 8303CEE4: 3AE00000  li r23, 0
	ctx.r[23].s64 = 0;
	// 8303CEE8: 3AC00000  li r22, 0
	ctx.r[22].s64 = 0;
	// 8303CEEC: 3B200000  li r25, 0
	ctx.r[25].s64 = 0;
	// 8303CEF0: 28040000  cmplwi r4, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8303CEF4: 41820018  beq 0x8303cf0c
	if ctx.cr[0].eq {
	pc = 0x8303CF0C; continue 'dispatch;
	}
	// 8303CEF8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8303CEFC: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 8303CF00: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8303CF04: 4BFFFF8D  bl 0x8303ce90
	ctx.lr = 0x8303CF08;
	sub_8303CE90(ctx, base);
	// 8303CF08: 7C771B78  mr r23, r3
	ctx.r[23].u64 = ctx.r[3].u64;
	// 8303CF0C: 817B0038  lwz r11, 0x38(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(56 as u32) ) } as u64;
	// 8303CF10: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8303CF14: 40820024  bne 0x8303cf38
	if !ctx.cr[0].eq {
	pc = 0x8303CF38; continue 'dispatch;
	}
	// 8303CF18: 809B0020  lwz r4, 0x20(r27)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(32 as u32) ) } as u64;
	// 8303CF1C: 28040000  cmplwi r4, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8303CF20: 41820018  beq 0x8303cf38
	if ctx.cr[0].eq {
	pc = 0x8303CF38; continue 'dispatch;
	}
	// 8303CF24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8303CF28: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 8303CF2C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8303CF30: 4BFFF4D1  bl 0x8303c400
	ctx.lr = 0x8303CF34;
	sub_8303C400(ctx, base);
	// 8303CF34: 7C761B78  mr r22, r3
	ctx.r[22].u64 = ctx.r[3].u64;
	// 8303CF38: 817B0048  lwz r11, 0x48(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(72 as u32) ) } as u64;
	// 8303CF3C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8303CF40: 4182000C  beq 0x8303cf4c
	if ctx.cr[0].eq {
	pc = 0x8303CF4C; continue 'dispatch;
	}
	// 8303CF44: 830B0008  lwz r24, 8(r11)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8303CF48: 48000008  b 0x8303cf50
	pc = 0x8303CF50; continue 'dispatch;
	// 8303CF4C: 3B000000  li r24, 0
	ctx.r[24].s64 = 0;
	// 8303CF50: 2B180000  cmplwi cr6, r24, 0
	ctx.cr[6].compare_u32(ctx.r[24].u32, 0 as u32, &mut ctx.xer);
	// 8303CF54: 419A00CC  beq cr6, 0x8303d020
	if ctx.cr[6].eq {
	pc = 0x8303D020; continue 'dispatch;
	}
	// 8303CF58: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 8303CF5C: 809D0000  lwz r4, 0(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303CF60: 4BF9B339  bl 0x82fd8298
	ctx.lr = 0x8303CF64;
	sub_82FD8298(ctx, base);
	// 8303CF64: 907F0060  stw r3, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[3].u32 ) };
	// 8303CF68: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8303CF6C: 41820024  beq 0x8303cf90
	if ctx.cr[0].eq {
	pc = 0x8303CF90; continue 'dispatch;
	}
	// 8303CF70: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8303CF74: 80DA007C  lwz r6, 0x7c(r26)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(124 as u32) ) } as u64;
	// 8303CF78: 38A0001D  li r5, 0x1d
	ctx.r[5].s64 = 29;
	// 8303CF7C: 811D0000  lwz r8, 0(r29)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303CF80: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 8303CF84: 4BFD0CAD  bl 0x8300dc30
	ctx.lr = 0x8303CF88;
	sub_8300DC30(ctx, base);
	// 8303CF88: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 8303CF8C: 48000008  b 0x8303cf94
	pc = 0x8303CF94; continue 'dispatch;
	// 8303CF90: 3B200000  li r25, 0
	ctx.r[25].s64 = 0;
	// 8303CF94: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 8303CF98: 2B180000  cmplwi cr6, r24, 0
	ctx.cr[6].compare_u32(ctx.r[24].u32, 0 as u32, &mut ctx.xer);
	// 8303CF9C: 419A0084  beq cr6, 0x8303d020
	if ctx.cr[6].eq {
	pc = 0x8303D020; continue 'dispatch;
	}
	// 8303CFA0: 807B0048  lwz r3, 0x48(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(72 as u32) ) } as u64;
	// 8303CFA4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8303CFA8: 41820014  beq 0x8303cfbc
	if ctx.cr[0].eq {
	pc = 0x8303CFBC; continue 'dispatch;
	}
	// 8303CFAC: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8303CFB0: 4BFEF8C1  bl 0x8302c870
	ctx.lr = 0x8303CFB4;
	sub_8302C870(ctx, base);
	// 8303CFB4: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8303CFB8: 48000008  b 0x8303cfc0
	pc = 0x8303CFC0; continue 'dispatch;
	// 8303CFBC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8303CFC0: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 8303CFC4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8303CFC8: 4BFFE5A9  bl 0x8303b570
	ctx.lr = 0x8303CFCC;
	sub_8303B570(ctx, base);
	// 8303CFCC: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8303CFD0: 41820044  beq 0x8303d014
	if ctx.cr[0].eq {
	pc = 0x8303D014; continue 'dispatch;
	}
	// 8303CFD4: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303CFD8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8303CFDC: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8303CFE0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8303CFE4: 4E800421  bctrl
	ctx.lr = 0x8303CFE8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8303CFE8: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303CFEC: 7C741B78  mr r20, r3
	ctx.r[20].u64 = ctx.r[3].u64;
	// 8303CFF0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8303CFF4: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8303CFF8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8303CFFC: 4E800421  bctrl
	ctx.lr = 0x8303D000;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8303D000: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 8303D004: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8303D008: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 8303D00C: 7E86A378  mr r6, r20
	ctx.r[6].u64 = ctx.r[20].u64;
	// 8303D010: 4BFD0D39  bl 0x8300dd48
	ctx.lr = 0x8303D014;
	sub_8300DD48(ctx, base);
	// 8303D014: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 8303D018: 7F1CC040  cmplw cr6, r28, r24
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[24].u32, &mut ctx.xer);
	// 8303D01C: 4198FF84  blt cr6, 0x8303cfa0
	if ctx.cr[6].lt {
	pc = 0x8303CFA0; continue 'dispatch;
	}
	// 8303D020: 817B001C  lwz r11, 0x1c(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(28 as u32) ) } as u64;
	// 8303D024: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 8303D028: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 8303D02C: 409A000C  bne cr6, 0x8303d038
	if !ctx.cr[6].eq {
	pc = 0x8303D038; continue 'dispatch;
	}
	// 8303D030: 3B800002  li r28, 2
	ctx.r[28].s64 = 2;
	// 8303D034: 48000010  b 0x8303d044
	pc = 0x8303D044; continue 'dispatch;
	// 8303D038: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8303D03C: 409A0008  bne cr6, 0x8303d044
	if !ctx.cr[6].eq {
	pc = 0x8303D044; continue 'dispatch;
	}
	// 8303D040: 3B800001  li r28, 1
	ctx.r[28].s64 = 1;
	// 8303D044: 38600034  li r3, 0x34
	ctx.r[3].s64 = 52;
	// 8303D048: 809D0000  lwz r4, 0(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303D04C: 4BF9B24D  bl 0x82fd8298
	ctx.lr = 0x8303D050;
	sub_82FD8298(ctx, base);
	// 8303D050: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8303D054: 93DF0060  stw r30, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[30].u32 ) };
	// 8303D058: 4182004C  beq 0x8303d0a4
	if ctx.cr[0].eq {
	pc = 0x8303D0A4; continue 'dispatch;
	}
	// 8303D05C: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 8303D060: 831D0000  lwz r24, 0(r29)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303D064: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 8303D068: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8303D06C: 4BFFE3F5  bl 0x8303b460
	ctx.lr = 0x8303D070;
	sub_8303B460(ctx, base);
	// 8303D070: 7C671B78  mr r7, r3
	ctx.r[7].u64 = ctx.r[3].u64;
	// 8303D074: 7EE6BB78  mr r6, r23
	ctx.r[6].u64 = ctx.r[23].u64;
	// 8303D078: 92A10054  stw r21, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[21].u32 ) };
	// 8303D07C: 7EC5B378  mr r5, r22
	ctx.r[5].u64 = ctx.r[22].u64;
	// 8303D080: 9301005C  stw r24, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[24].u32 ) };
	// 8303D084: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8303D088: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8303D08C: 7F28CB78  mr r8, r25
	ctx.r[8].u64 = ctx.r[25].u64;
	// 8303D090: 7F49D378  mr r9, r26
	ctx.r[9].u64 = ctx.r[26].u64;
	// 8303D094: 7F8AE378  mr r10, r28
	ctx.r[10].u64 = ctx.r[28].u64;
	// 8303D098: 4BFAF2E9  bl 0x82fec380
	ctx.lr = 0x8303D09C;
	sub_82FEC380(ctx, base);
	// 8303D09C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8303D0A0: 48000008  b 0x8303d0a8
	pc = 0x8303D0A8; continue 'dispatch;
	// 8303D0A4: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8303D0A8: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8303D0AC: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8303D0B0: 807D0004  lwz r3, 4(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 8303D0B4: 4BFEFD95  bl 0x8302ce48
	ctx.lr = 0x8303D0B8;
	sub_8302CE48(ctx, base);
	// 8303D0B8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8303D0BC: 807D0008  lwz r3, 8(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 8303D0C0: 4BFFE091  bl 0x8303b150
	ctx.lr = 0x8303D0C4;
	sub_8303B150(ctx, base);
	// 8303D0C4: 809B0038  lwz r4, 0x38(r27)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(56 as u32) ) } as u64;
	// 8303D0C8: 2C040000  cmpwi r4, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8303D0CC: 41820014  beq 0x8303d0e0
	if ctx.cr[0].eq {
	pc = 0x8303D0E0; continue 'dispatch;
	}
	// 8303D0D0: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 8303D0D4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8303D0D8: 4BFFF7B9  bl 0x8303c890
	ctx.lr = 0x8303D0DC;
	sub_8303C890(ctx, base);
	// 8303D0DC: 48000024  b 0x8303d100
	pc = 0x8303D100; continue 'dispatch;
	// 8303D0E0: 2B160000  cmplwi cr6, r22, 0
	ctx.cr[6].compare_u32(ctx.r[22].u32, 0 as u32, &mut ctx.xer);
	// 8303D0E4: 409A0020  bne cr6, 0x8303d104
	if !ctx.cr[6].eq {
	pc = 0x8303D104; continue 'dispatch;
	}
	// 8303D0E8: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8303D0EC: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 8303D0F0: 38ABCC98  addi r5, r11, -0x3368
	ctx.r[5].s64 = ctx.r[11].s64 + -13160;
	// 8303D0F4: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8303D0F8: 388BD5D4  addi r4, r11, -0x2a2c
	ctx.r[4].s64 = ctx.r[11].s64 + -10796;
	// 8303D0FC: 4BFD0FD5  bl 0x8300e0d0
	ctx.lr = 0x8303D100;
	sub_8300E0D0(ctx, base);
	// 8303D100: 907E0020  stw r3, 0x20(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(32 as u32), ctx.r[3].u32 ) };
	// 8303D104: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8303D108: 383F00D0  addi r1, r31, 0xd0
	ctx.r[1].s64 = ctx.r[31].s64 + 208;
	// 8303D10C: 4816B08C  b 0x831a8198
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303D110(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8303D110 size=48
    let mut pc: u32 = 0x8303D110;
    'dispatch: loop {
        match pc {
            0x8303D110 => {
    //   block [0x8303D110..0x8303D140)
	// 8303D110: 3BECFF30  addi r31, r12, -0xd0
	ctx.r[31].s64 = ctx.r[12].s64 + -208;
	// 8303D114: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8303D118: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8303D11C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8303D120: 817F00E4  lwz r11, 0xe4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(228 as u32) ) } as u64;
	// 8303D124: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303D128: 807F0060  lwz r3, 0x60(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 8303D12C: 4BF9B1B5  bl 0x82fd82e0
	ctx.lr = 0x8303D130;
	sub_82FD82E0(ctx, base);
	// 8303D130: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8303D134: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8303D138: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8303D13C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303D140(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8303D140 size=48
    let mut pc: u32 = 0x8303D140;
    'dispatch: loop {
        match pc {
            0x8303D140 => {
    //   block [0x8303D140..0x8303D170)
	// 8303D140: 3BECFF30  addi r31, r12, -0xd0
	ctx.r[31].s64 = ctx.r[12].s64 + -208;
	// 8303D144: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8303D148: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8303D14C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8303D150: 817F00E4  lwz r11, 0xe4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(228 as u32) ) } as u64;
	// 8303D154: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303D158: 807F0060  lwz r3, 0x60(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 8303D15C: 4BF9B185  bl 0x82fd82e0
	ctx.lr = 0x8303D160;
	sub_82FD82E0(ctx, base);
	// 8303D160: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8303D164: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8303D168: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8303D16C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303D170(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8303D170 size=8
    let mut pc: u32 = 0x8303D170;
    'dispatch: loop {
        match pc {
            0x8303D170 => {
    //   block [0x8303D170..0x8303D178)
	// 8303D170: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8303D174: 8215D7A8  lwz r16, -0x2858(r21)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-10328 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303D178(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8303D178 size=136
    let mut pc: u32 = 0x8303D178;
    'dispatch: loop {
        match pc {
            0x8303D178 => {
    //   block [0x8303D178..0x8303D200)
	// 8303D178: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8303D17C: 4816AFE9  bl 0x831a8164
	ctx.lr = 0x8303D180;
	sub_831A8130(ctx, base);
	// 8303D180: 3BE1FF70  addi r31, r1, -0x90
	ctx.r[31].s64 = ctx.r[1].s64 + -144;
	// 8303D184: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8303D188: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8303D18C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8303D190: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 8303D194: 809D000C  lwz r4, 0xc(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 8303D198: 93DF00A4  stw r30, 0xa4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(164 as u32), ctx.r[30].u32 ) };
	// 8303D19C: 28040000  cmplwi r4, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8303D1A0: 41820054  beq 0x8303d1f4
	if ctx.cr[0].eq {
	pc = 0x8303D1F4; continue 'dispatch;
	}
	// 8303D1A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8303D1A8: 4BFFFCE9  bl 0x8303ce90
	ctx.lr = 0x8303D1AC;
	sub_8303CE90(ctx, base);
	// 8303D1AC: 7C7C1B79  or. r28, r3, r3
	ctx.r[28].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 8303D1B0: 41820044  beq 0x8303d1f4
	if ctx.cr[0].eq {
	pc = 0x8303D1F4; continue 'dispatch;
	}
	// 8303D1B4: 38600024  li r3, 0x24
	ctx.r[3].s64 = 36;
	// 8303D1B8: 809E0000  lwz r4, 0(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303D1BC: 4BF9B0DD  bl 0x82fd8298
	ctx.lr = 0x8303D1C0;
	sub_82FD8298(ctx, base);
	// 8303D1C0: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 8303D1C4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8303D1C8: 41820024  beq 0x8303d1ec
	if ctx.cr[0].eq {
	pc = 0x8303D1EC; continue 'dispatch;
	}
	// 8303D1CC: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 8303D1D0: 811D0024  lwz r8, 0x24(r29)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(36 as u32) ) } as u64;
	// 8303D1D4: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 8303D1D8: 80FD0020  lwz r7, 0x20(r29)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(32 as u32) ) } as u64;
	// 8303D1DC: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 8303D1E0: 813E0000  lwz r9, 0(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303D1E4: 480434DD  bl 0x830806c0
	ctx.lr = 0x8303D1E8;
	sub_830806C0(ctx, base);
	// 8303D1E8: 48000008  b 0x8303d1f0
	pc = 0x8303D1F0; continue 'dispatch;
	// 8303D1EC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8303D1F0: 48000008  b 0x8303d1f8
	pc = 0x8303D1F8; continue 'dispatch;
	// 8303D1F4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8303D1F8: 383F0090  addi r1, r31, 0x90
	ctx.r[1].s64 = ctx.r[31].s64 + 144;
	// 8303D1FC: 4816AFB8  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303D200(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8303D200 size=48
    let mut pc: u32 = 0x8303D200;
    'dispatch: loop {
        match pc {
            0x8303D200 => {
    //   block [0x8303D200..0x8303D230)
	// 8303D200: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 8303D204: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8303D208: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8303D20C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8303D210: 817F00A4  lwz r11, 0xa4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 8303D214: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303D218: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8303D21C: 4BF9B0C5  bl 0x82fd82e0
	ctx.lr = 0x8303D220;
	sub_82FD82E0(ctx, base);
	// 8303D220: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8303D224: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8303D228: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8303D22C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303D230(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8303D230 size=124
    let mut pc: u32 = 0x8303D230;
    'dispatch: loop {
        match pc {
            0x8303D230 => {
    //   block [0x8303D230..0x8303D2AC)
	// 8303D230: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8303D234: 4816AF35  bl 0x831a8168
	ctx.lr = 0x8303D238;
	sub_831A8130(ctx, base);
	// 8303D238: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8303D23C: 81640018  lwz r11, 0x18(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(24 as u32) ) } as u64;
	// 8303D240: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8303D244: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 8303D248: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 8303D24C: 4800002C  b 0x8303d278
	pc = 0x8303D278; continue 'dispatch;
	// 8303D250: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 8303D254: 83E40014  lwz r31, 0x14(r4)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(20 as u32) ) } as u64;
	// 8303D258: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 8303D25C: 80840010  lwz r4, 0x10(r4)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(16 as u32) ) } as u64;
	// 8303D260: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8303D264: 4BFFFFCD  bl 0x8303d230
	ctx.lr = 0x8303D268;
	sub_8303D230(ctx, base);
	// 8303D268: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8303D26C: 419A0038  beq cr6, 0x8303d2a4
	if ctx.cr[6].eq {
	pc = 0x8303D2A4; continue 'dispatch;
	}
	// 8303D270: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 8303D274: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8303D278: 2F0B0009  cmpwi cr6, r11, 9
	ctx.cr[6].compare_i32(ctx.r[11].s32, 9, &mut ctx.xer);
	// 8303D27C: 419AFFD4  beq cr6, 0x8303d250
	if ctx.cr[6].eq {
	pc = 0x8303D250; continue 'dispatch;
	}
	// 8303D280: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8303D284: 409A0020  bne cr6, 0x8303d2a4
	if !ctx.cr[6].eq {
	pc = 0x8303D2A4; continue 'dispatch;
	}
	// 8303D288: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8303D28C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8303D290: 4BFFFEE9  bl 0x8303d178
	ctx.lr = 0x8303D294;
	sub_8303D178(ctx, base);
	// 8303D294: 7C641B79  or. r4, r3, r3
	ctx.r[4].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 8303D298: 4182000C  beq 0x8303d2a4
	if ctx.cr[0].eq {
	pc = 0x8303D2A4; continue 'dispatch;
	}
	// 8303D29C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8303D2A0: 4BFFDEB1  bl 0x8303b150
	ctx.lr = 0x8303D2A4;
	sub_8303B150(ctx, base);
	// 8303D2A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8303D2A8: 4816AF10  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303D2B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8303D2B0 size=200
    let mut pc: u32 = 0x8303D2B0;
    'dispatch: loop {
        match pc {
            0x8303D2B0 => {
    //   block [0x8303D2B0..0x8303D378)
	// 8303D2B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8303D2B4: 4816AEB5  bl 0x831a8168
	ctx.lr = 0x8303D2B8;
	sub_831A8130(ctx, base);
	// 8303D2B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8303D2BC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8303D2C0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8303D2C4: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 8303D2C8: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 8303D2CC: 48000030  b 0x8303d2fc
	pc = 0x8303D2FC; continue 'dispatch;
	// 8303D2D0: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 8303D2D4: 2F0B0005  cmpwi cr6, r11, 5
	ctx.cr[6].compare_i32(ctx.r[11].s32, 5, &mut ctx.xer);
	// 8303D2D8: 419A000C  beq cr6, 0x8303d2e4
	if ctx.cr[6].eq {
	pc = 0x8303D2E4; continue 'dispatch;
	}
	// 8303D2DC: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 8303D2E0: 409A0028  bne cr6, 0x8303d308
	if !ctx.cr[6].eq {
	pc = 0x8303D308; continue 'dispatch;
	}
	// 8303D2E4: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 8303D2E8: 809F0010  lwz r4, 0x10(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8303D2EC: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 8303D2F0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8303D2F4: 4BFFFFBD  bl 0x8303d2b0
	ctx.lr = 0x8303D2F8;
	sub_8303D2B0(ctx, base);
	// 8303D2F8: 83FF0014  lwz r31, 0x14(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 8303D2FC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8303D300: 409AFFD0  bne cr6, 0x8303d2d0
	if !ctx.cr[6].eq {
	pc = 0x8303D2D0; continue 'dispatch;
	}
	// 8303D304: 4800006C  b 0x8303d370
	pc = 0x8303D370; continue 'dispatch;
	// 8303D308: 556A073E  clrlwi r10, r11, 0x1c
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x0000000Fu64;
	// 8303D30C: 2F0A0006  cmpwi cr6, r10, 6
	ctx.cr[6].compare_i32(ctx.r[10].s32, 6, &mut ctx.xer);
	// 8303D310: 419A0040  beq cr6, 0x8303d350
	if ctx.cr[6].eq {
	pc = 0x8303D350; continue 'dispatch;
	}
	// 8303D314: 2F0A0007  cmpwi cr6, r10, 7
	ctx.cr[6].compare_i32(ctx.r[10].s32, 7, &mut ctx.xer);
	// 8303D318: 419A0038  beq cr6, 0x8303d350
	if ctx.cr[6].eq {
	pc = 0x8303D350; continue 'dispatch;
	}
	// 8303D31C: 2F0A0008  cmpwi cr6, r10, 8
	ctx.cr[6].compare_i32(ctx.r[10].s32, 8, &mut ctx.xer);
	// 8303D320: 419A0030  beq cr6, 0x8303d350
	if ctx.cr[6].eq {
	pc = 0x8303D350; continue 'dispatch;
	}
	// 8303D324: 2F0B0014  cmpwi cr6, r11, 0x14
	ctx.cr[6].compare_i32(ctx.r[11].s32, 20, &mut ctx.xer);
	// 8303D328: 419A0028  beq cr6, 0x8303d350
	if ctx.cr[6].eq {
	pc = 0x8303D350; continue 'dispatch;
	}
	// 8303D32C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8303D330: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8303D334: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8303D338: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8303D33C: 409A000C  bne cr6, 0x8303d348
	if !ctx.cr[6].eq {
	pc = 0x8303D348; continue 'dispatch;
	}
	// 8303D340: 4BFFFE39  bl 0x8303d178
	ctx.lr = 0x8303D344;
	sub_8303D178(ctx, base);
	// 8303D344: 4800001C  b 0x8303d360
	pc = 0x8303D360; continue 'dispatch;
	// 8303D348: 48000039  bl 0x8303d380
	ctx.lr = 0x8303D34C;
	sub_8303D380(ctx, base);
	// 8303D34C: 48000014  b 0x8303d360
	pc = 0x8303D360; continue 'dispatch;
	// 8303D350: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8303D354: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8303D358: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8303D35C: 4BFFE68D  bl 0x8303b9e8
	ctx.lr = 0x8303D360;
	sub_8303B9E8(ctx, base);
	// 8303D360: 7C641B79  or. r4, r3, r3
	ctx.r[4].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 8303D364: 4182000C  beq 0x8303d370
	if ctx.cr[0].eq {
	pc = 0x8303D370; continue 'dispatch;
	}
	// 8303D368: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8303D36C: 4BFFDDE5  bl 0x8303b150
	ctx.lr = 0x8303D370;
	sub_8303B150(ctx, base);
	// 8303D370: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8303D374: 4816AE44  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303D378(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8303D378 size=8
    let mut pc: u32 = 0x8303D378;
    'dispatch: loop {
        match pc {
            0x8303D378 => {
    //   block [0x8303D378..0x8303D380)
	// 8303D378: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8303D37C: 8215D800  lwz r16, -0x2800(r21)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-10240 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303D380(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8303D380 size=496
    let mut pc: u32 = 0x8303D380;
    'dispatch: loop {
        match pc {
            0x8303D380 => {
    //   block [0x8303D380..0x8303D570)
	// 8303D380: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8303D384: 4816ADD9  bl 0x831a815c
	ctx.lr = 0x8303D388;
	sub_831A8130(ctx, base);
	// 8303D388: 3BE1FF60  addi r31, r1, -0xa0
	ctx.r[31].s64 = ctx.r[1].s64 + -160;
	// 8303D38C: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8303D390: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8303D394: 7C992378  mr r25, r4
	ctx.r[25].u64 = ctx.r[4].u64;
	// 8303D398: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 8303D39C: 2B190000  cmplwi cr6, r25, 0
	ctx.cr[6].compare_u32(ctx.r[25].u32, 0 as u32, &mut ctx.xer);
	// 8303D3A0: 93DF00B4  stw r30, 0xb4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(180 as u32), ctx.r[30].u32 ) };
	// 8303D3A4: 409A000C  bne cr6, 0x8303d3b0
	if !ctx.cr[6].eq {
	pc = 0x8303D3B0; continue 'dispatch;
	}
	// 8303D3A8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8303D3AC: 480001BC  b 0x8303d568
	pc = 0x8303D568; continue 'dispatch;
	// 8303D3B0: 83590018  lwz r26, 0x18(r25)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(24 as u32) ) } as u64;
	// 8303D3B4: 2F1A0009  cmpwi cr6, r26, 9
	ctx.cr[6].compare_i32(ctx.r[26].s32, 9, &mut ctx.xer);
	// 8303D3B8: 419A0014  beq cr6, 0x8303d3cc
	if ctx.cr[6].eq {
	pc = 0x8303D3CC; continue 'dispatch;
	}
	// 8303D3BC: 2F1A0024  cmpwi cr6, r26, 0x24
	ctx.cr[6].compare_i32(ctx.r[26].s32, 36, &mut ctx.xer);
	// 8303D3C0: 419A000C  beq cr6, 0x8303d3cc
	if ctx.cr[6].eq {
	pc = 0x8303D3CC; continue 'dispatch;
	}
	// 8303D3C4: 2F1A0015  cmpwi cr6, r26, 0x15
	ctx.cr[6].compare_i32(ctx.r[26].s32, 21, &mut ctx.xer);
	// 8303D3C8: 409AFFE0  bne cr6, 0x8303d3a8
	if !ctx.cr[6].eq {
	pc = 0x8303D3A8; continue 'dispatch;
	}
	// 8303D3CC: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 8303D3D0: 809E0000  lwz r4, 0(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303D3D4: 4BF9AEC5  bl 0x82fd8298
	ctx.lr = 0x8303D3D8;
	sub_82FD8298(ctx, base);
	// 8303D3D8: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 8303D3DC: 93BF0050  stw r29, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[29].u32 ) };
	// 8303D3E0: 41820028  beq 0x8303d408
	if ctx.cr[0].eq {
	pc = 0x8303D408; continue 'dispatch;
	}
	// 8303D3E4: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8303D3E8: 80DE0000  lwz r6, 0(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303D3EC: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 8303D3F0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8303D3F4: 4800F3FD  bl 0x8304c7f0
	ctx.lr = 0x8303D3F8;
	sub_8304C7F0(ctx, base);
	// 8303D3F8: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 8303D3FC: 396B2990  addi r11, r11, 0x2990
	ctx.r[11].s64 = ctx.r[11].s64 + 10640;
	// 8303D400: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8303D404: 48000008  b 0x8303d40c
	pc = 0x8303D40C; continue 'dispatch;
	// 8303D408: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 8303D40C: 7F25CB78  mr r5, r25
	ctx.r[5].u64 = ctx.r[25].u64;
	// 8303D410: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8303D414: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8303D418: 4BFFE049  bl 0x8303b460
	ctx.lr = 0x8303D41C;
	sub_8303B460(ctx, base);
	// 8303D41C: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 8303D420: 2F1A0009  cmpwi cr6, r26, 9
	ctx.cr[6].compare_i32(ctx.r[26].s32, 9, &mut ctx.xer);
	// 8303D424: 409A005C  bne cr6, 0x8303d480
	if !ctx.cr[6].eq {
	pc = 0x8303D480; continue 'dispatch;
	}
	// 8303D428: 38600020  li r3, 0x20
	ctx.r[3].s64 = 32;
	// 8303D42C: 809E0000  lwz r4, 0(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303D430: 4BF9AE69  bl 0x82fd8298
	ctx.lr = 0x8303D434;
	sub_82FD8298(ctx, base);
	// 8303D434: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 8303D438: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8303D43C: 41820024  beq 0x8303d460
	if ctx.cr[0].eq {
	pc = 0x8303D460; continue 'dispatch;
	}
	// 8303D440: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 8303D444: 811E0000  lwz r8, 0(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303D448: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 8303D44C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8303D450: 38800003  li r4, 3
	ctx.r[4].s64 = 3;
	// 8303D454: 48044075  bl 0x830814c8
	ctx.lr = 0x8303D458;
	sub_830814C8(ctx, base);
	// 8303D458: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8303D45C: 48000008  b 0x8303d464
	pc = 0x8303D464; continue 'dispatch;
	// 8303D460: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8303D464: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 8303D468: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8303D46C: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 8303D470: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8303D474: 7D7B5B78  mr r27, r11
	ctx.r[27].u64 = ctx.r[11].u64;
	// 8303D478: 4BFFFDB9  bl 0x8303d230
	ctx.lr = 0x8303D47C;
	sub_8303D230(ctx, base);
	// 8303D47C: 480000B0  b 0x8303d52c
	pc = 0x8303D52C; continue 'dispatch;
	// 8303D480: 2F1A0024  cmpwi cr6, r26, 0x24
	ctx.cr[6].compare_i32(ctx.r[26].s32, 36, &mut ctx.xer);
	// 8303D484: 409A0044  bne cr6, 0x8303d4c8
	if !ctx.cr[6].eq {
	pc = 0x8303D4C8; continue 'dispatch;
	}
	// 8303D488: 38600020  li r3, 0x20
	ctx.r[3].s64 = 32;
	// 8303D48C: 809E0000  lwz r4, 0(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303D490: 4BF9AE09  bl 0x82fd8298
	ctx.lr = 0x8303D494;
	sub_82FD8298(ctx, base);
	// 8303D494: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 8303D498: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8303D49C: 41820020  beq 0x8303d4bc
	if ctx.cr[0].eq {
	pc = 0x8303D4BC; continue 'dispatch;
	}
	// 8303D4A0: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 8303D4A4: 811E0000  lwz r8, 0(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303D4A8: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 8303D4AC: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8303D4B0: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 8303D4B4: 48044015  bl 0x830814c8
	ctx.lr = 0x8303D4B8;
	sub_830814C8(ctx, base);
	// 8303D4B8: 48000008  b 0x8303d4c0
	pc = 0x8303D4C0; continue 'dispatch;
	// 8303D4BC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8303D4C0: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 8303D4C4: 48000040  b 0x8303d504
	pc = 0x8303D504; continue 'dispatch;
	// 8303D4C8: 38600020  li r3, 0x20
	ctx.r[3].s64 = 32;
	// 8303D4CC: 809E0000  lwz r4, 0(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303D4D0: 4BF9ADC9  bl 0x82fd8298
	ctx.lr = 0x8303D4D4;
	sub_82FD8298(ctx, base);
	// 8303D4D4: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 8303D4D8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8303D4DC: 41820020  beq 0x8303d4fc
	if ctx.cr[0].eq {
	pc = 0x8303D4FC; continue 'dispatch;
	}
	// 8303D4E0: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 8303D4E4: 811E0000  lwz r8, 0(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303D4E8: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 8303D4EC: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8303D4F0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8303D4F4: 48043FD5  bl 0x830814c8
	ctx.lr = 0x8303D4F8;
	sub_830814C8(ctx, base);
	// 8303D4F8: 48000008  b 0x8303d500
	pc = 0x8303D500; continue 'dispatch;
	// 8303D4FC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8303D500: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 8303D504: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 8303D508: 80990010  lwz r4, 0x10(r25)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(16 as u32) ) } as u64;
	// 8303D50C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8303D510: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8303D514: 4BFFFD9D  bl 0x8303d2b0
	ctx.lr = 0x8303D518;
	sub_8303D2B0(ctx, base);
	// 8303D518: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 8303D51C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8303D520: 80990014  lwz r4, 0x14(r25)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(20 as u32) ) } as u64;
	// 8303D524: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8303D528: 4BFFFD89  bl 0x8303d2b0
	ctx.lr = 0x8303D52C;
	sub_8303D2B0(ctx, base);
	// 8303D52C: 38600024  li r3, 0x24
	ctx.r[3].s64 = 36;
	// 8303D530: 809E0000  lwz r4, 0(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303D534: 4BF9AD65  bl 0x82fd8298
	ctx.lr = 0x8303D538;
	sub_82FD8298(ctx, base);
	// 8303D538: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 8303D53C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8303D540: 41820024  beq 0x8303d564
	if ctx.cr[0].eq {
	pc = 0x8303D564; continue 'dispatch;
	}
	// 8303D544: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 8303D548: 81190024  lwz r8, 0x24(r25)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(36 as u32) ) } as u64;
	// 8303D54C: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 8303D550: 80F90020  lwz r7, 0x20(r25)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(32 as u32) ) } as u64;
	// 8303D554: 38800006  li r4, 6
	ctx.r[4].s64 = 6;
	// 8303D558: 813E0000  lwz r9, 0(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303D55C: 48043165  bl 0x830806c0
	ctx.lr = 0x8303D560;
	sub_830806C0(ctx, base);
	// 8303D560: 48000008  b 0x8303d568
	pc = 0x8303D568; continue 'dispatch;
	// 8303D564: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8303D568: 383F00A0  addi r1, r31, 0xa0
	ctx.r[1].s64 = ctx.r[31].s64 + 160;
	// 8303D56C: 4816AC40  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303D570(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8303D570 size=48
    let mut pc: u32 = 0x8303D570;
    'dispatch: loop {
        match pc {
            0x8303D570 => {
    //   block [0x8303D570..0x8303D5A0)
	// 8303D570: 3BECFF60  addi r31, r12, -0xa0
	ctx.r[31].s64 = ctx.r[12].s64 + -160;
	// 8303D574: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8303D578: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8303D57C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8303D580: 817F00B4  lwz r11, 0xb4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(180 as u32) ) } as u64;
	// 8303D584: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303D588: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8303D58C: 4BF9AD55  bl 0x82fd82e0
	ctx.lr = 0x8303D590;
	sub_82FD82E0(ctx, base);
	// 8303D590: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8303D594: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8303D598: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8303D59C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303D5A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8303D5A0 size=48
    let mut pc: u32 = 0x8303D5A0;
    'dispatch: loop {
        match pc {
            0x8303D5A0 => {
    //   block [0x8303D5A0..0x8303D5D0)
	// 8303D5A0: 3BECFF60  addi r31, r12, -0xa0
	ctx.r[31].s64 = ctx.r[12].s64 + -160;
	// 8303D5A4: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8303D5A8: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8303D5AC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8303D5B0: 817F00B4  lwz r11, 0xb4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(180 as u32) ) } as u64;
	// 8303D5B4: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303D5B8: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8303D5BC: 4BF9AD25  bl 0x82fd82e0
	ctx.lr = 0x8303D5C0;
	sub_82FD82E0(ctx, base);
	// 8303D5C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8303D5C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8303D5C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8303D5CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303D5D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8303D5D0 size=48
    let mut pc: u32 = 0x8303D5D0;
    'dispatch: loop {
        match pc {
            0x8303D5D0 => {
    //   block [0x8303D5D0..0x8303D600)
	// 8303D5D0: 3BECFF60  addi r31, r12, -0xa0
	ctx.r[31].s64 = ctx.r[12].s64 + -160;
	// 8303D5D4: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8303D5D8: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8303D5DC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8303D5E0: 817F00B4  lwz r11, 0xb4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(180 as u32) ) } as u64;
	// 8303D5E4: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303D5E8: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8303D5EC: 4BF9ACF5  bl 0x82fd82e0
	ctx.lr = 0x8303D5F0;
	sub_82FD82E0(ctx, base);
	// 8303D5F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8303D5F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8303D5F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8303D5FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303D600(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8303D600 size=48
    let mut pc: u32 = 0x8303D600;
    'dispatch: loop {
        match pc {
            0x8303D600 => {
    //   block [0x8303D600..0x8303D630)
	// 8303D600: 3BECFF60  addi r31, r12, -0xa0
	ctx.r[31].s64 = ctx.r[12].s64 + -160;
	// 8303D604: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8303D608: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8303D60C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8303D610: 817F00B4  lwz r11, 0xb4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(180 as u32) ) } as u64;
	// 8303D614: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303D618: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8303D61C: 4BF9ACC5  bl 0x82fd82e0
	ctx.lr = 0x8303D620;
	sub_82FD82E0(ctx, base);
	// 8303D620: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8303D624: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8303D628: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8303D62C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303D630(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8303D630 size=48
    let mut pc: u32 = 0x8303D630;
    'dispatch: loop {
        match pc {
            0x8303D630 => {
    //   block [0x8303D630..0x8303D660)
	// 8303D630: 3BECFF60  addi r31, r12, -0xa0
	ctx.r[31].s64 = ctx.r[12].s64 + -160;
	// 8303D634: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8303D638: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8303D63C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8303D640: 817F00B4  lwz r11, 0xb4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(180 as u32) ) } as u64;
	// 8303D644: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303D648: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8303D64C: 4BF9AC95  bl 0x82fd82e0
	ctx.lr = 0x8303D650;
	sub_82FD82E0(ctx, base);
	// 8303D650: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8303D654: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8303D658: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8303D65C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303D660(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8303D660 size=8
    let mut pc: u32 = 0x8303D660;
    'dispatch: loop {
        match pc {
            0x8303D660 => {
    //   block [0x8303D660..0x8303D668)
	// 8303D660: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8303D664: 8215D880  lwz r16, -0x2780(r21)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-10112 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303D668(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8303D668 size=232
    let mut pc: u32 = 0x8303D668;
    'dispatch: loop {
        match pc {
            0x8303D668 => {
    //   block [0x8303D668..0x8303D750)
	// 8303D668: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8303D66C: 4816AAF1  bl 0x831a815c
	ctx.lr = 0x8303D670;
	sub_831A8130(ctx, base);
	// 8303D670: 3BE1FF60  addi r31, r1, -0xa0
	ctx.r[31].s64 = ctx.r[1].s64 + -160;
	// 8303D674: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8303D678: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8303D67C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8303D680: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 8303D684: 93DF00B4  stw r30, 0xb4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(180 as u32), ctx.r[30].u32 ) };
	// 8303D688: 809D0014  lwz r4, 0x14(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(20 as u32) ) } as u64;
	// 8303D68C: 4BFFFCF5  bl 0x8303d380
	ctx.lr = 0x8303D690;
	sub_8303D380(ctx, base);
	// 8303D690: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 8303D694: 809E0000  lwz r4, 0(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303D698: 38600020  li r3, 0x20
	ctx.r[3].s64 = 32;
	// 8303D69C: 4BF9ABFD  bl 0x82fd8298
	ctx.lr = 0x8303D6A0;
	sub_82FD8298(ctx, base);
	// 8303D6A0: 7C7C1B79  or. r28, r3, r3
	ctx.r[28].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 8303D6A4: 939F0050  stw r28, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[28].u32 ) };
	// 8303D6A8: 4182003C  beq 0x8303d6e4
	if ctx.cr[0].eq {
	pc = 0x8303D6E4; continue 'dispatch;
	}
	// 8303D6AC: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8303D6B0: 833E0000  lwz r25, 0(r30)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303D6B4: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8303D6B8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8303D6BC: 4BFFDDA5  bl 0x8303b460
	ctx.lr = 0x8303D6C0;
	sub_8303B460(ctx, base);
	// 8303D6C0: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 8303D6C4: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 8303D6C8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8303D6CC: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8303D6D0: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 8303D6D4: 7F28CB78  mr r8, r25
	ctx.r[8].u64 = ctx.r[25].u64;
	// 8303D6D8: 48043F31  bl 0x83081608
	ctx.lr = 0x8303D6DC;
	sub_83081608(ctx, base);
	// 8303D6DC: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 8303D6E0: 48000008  b 0x8303d6e8
	pc = 0x8303D6E8; continue 'dispatch;
	// 8303D6E4: 3B200000  li r25, 0
	ctx.r[25].s64 = 0;
	// 8303D6E8: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 8303D6EC: 807E0008  lwz r3, 8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8303D6F0: 4BFFDA61  bl 0x8303b150
	ctx.lr = 0x8303D6F4;
	sub_8303B150(ctx, base);
	// 8303D6F4: 817D0018  lwz r11, 0x18(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(24 as u32) ) } as u64;
	// 8303D6F8: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 8303D6FC: 834B0008  lwz r26, 8(r11)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8303D700: 281A0000  cmplwi r26, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8303D704: 41820040  beq 0x8303d744
	if ctx.cr[0].eq {
	pc = 0x8303D744; continue 'dispatch;
	}
	// 8303D708: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8303D70C: 807D0018  lwz r3, 0x18(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(24 as u32) ) } as u64;
	// 8303D710: 4BFEF161  bl 0x8302c870
	ctx.lr = 0x8303D714;
	sub_8302C870(ctx, base);
	// 8303D714: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8303D718: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 8303D71C: 81440024  lwz r10, 0x24(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(36 as u32) ) } as u64;
	// 8303D720: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8303D724: 409A0014  bne cr6, 0x8303d738
	if !ctx.cr[6].eq {
	pc = 0x8303D738; continue 'dispatch;
	}
	// 8303D728: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8303D72C: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 8303D730: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8303D734: 4BFFF75D  bl 0x8303ce90
	ctx.lr = 0x8303D738;
	sub_8303CE90(ctx, base);
	// 8303D738: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 8303D73C: 7F1CD040  cmplw cr6, r28, r26
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[26].u32, &mut ctx.xer);
	// 8303D740: 4198FFC8  blt cr6, 0x8303d708
	if ctx.cr[6].lt {
	pc = 0x8303D708; continue 'dispatch;
	}
	// 8303D744: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 8303D748: 383F00A0  addi r1, r31, 0xa0
	ctx.r[1].s64 = ctx.r[31].s64 + 160;
	// 8303D74C: 4816AA60  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303D750(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8303D750 size=48
    let mut pc: u32 = 0x8303D750;
    'dispatch: loop {
        match pc {
            0x8303D750 => {
    //   block [0x8303D750..0x8303D780)
	// 8303D750: 3BECFF60  addi r31, r12, -0xa0
	ctx.r[31].s64 = ctx.r[12].s64 + -160;
	// 8303D754: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8303D758: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8303D75C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8303D760: 817F00B4  lwz r11, 0xb4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(180 as u32) ) } as u64;
	// 8303D764: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303D768: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8303D76C: 4BF9AB75  bl 0x82fd82e0
	ctx.lr = 0x8303D770;
	sub_82FD82E0(ctx, base);
	// 8303D770: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8303D774: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8303D778: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8303D77C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303D780(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8303D780 size=68
    let mut pc: u32 = 0x8303D780;
    'dispatch: loop {
        match pc {
            0x8303D780 => {
    //   block [0x8303D780..0x8303D7C4)
	// 8303D780: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8303D784: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8303D788: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8303D78C: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 8303D790: 419A0020  beq cr6, 0x8303d7b0
	if ctx.cr[6].eq {
	pc = 0x8303D7B0; continue 'dispatch;
	}
	// 8303D794: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8303D798: 80630050  lwz r3, 0x50(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(80 as u32) ) } as u64;
	// 8303D79C: 4BFBCCC5  bl 0x82ffa460
	ctx.lr = 0x8303D7A0;
	sub_82FFA460(ctx, base);
	// 8303D7A0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8303D7A4: 4182000C  beq 0x8303d7b0
	if ctx.cr[0].eq {
	pc = 0x8303D7B0; continue 'dispatch;
	}
	// 8303D7A8: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303D7AC: 48000008  b 0x8303d7b4
	pc = 0x8303D7B4; continue 'dispatch;
	// 8303D7B0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8303D7B4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8303D7B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8303D7BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8303D7C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303D7C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8303D7C8 size=8
    let mut pc: u32 = 0x8303D7C8;
    'dispatch: loop {
        match pc {
            0x8303D7C8 => {
    //   block [0x8303D7C8..0x8303D7D0)
	// 8303D7C8: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8303D7CC: 8215D8D8  lwz r16, -0x2728(r21)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-10024 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303D7D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8303D7D0 size=360
    let mut pc: u32 = 0x8303D7D0;
    'dispatch: loop {
        match pc {
            0x8303D7D0 => {
    //   block [0x8303D7D0..0x8303D938)
	// 8303D7D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8303D7D4: 4816A989  bl 0x831a815c
	ctx.lr = 0x8303D7D8;
	sub_831A8130(ctx, base);
	// 8303D7D8: 3BE1FF60  addi r31, r1, -0xa0
	ctx.r[31].s64 = ctx.r[1].s64 + -160;
	// 8303D7DC: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8303D7E0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8303D7E4: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 8303D7E8: 3B200000  li r25, 0
	ctx.r[25].s64 = 0;
	// 8303D7EC: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 8303D7F0: 90BE0004  stw r5, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 8303D7F4: 937E0000  stw r27, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 8303D7F8: 909E0008  stw r4, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[4].u32 ) };
	// 8303D7FC: 933E0044  stw r25, 0x44(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(68 as u32), ctx.r[25].u32 ) };
	// 8303D800: 81650000  lwz r11, 0(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303D804: 93DF00B4  stw r30, 0xb4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(180 as u32), ctx.r[30].u32 ) };
	// 8303D808: 937F00CC  stw r27, 0xcc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(204 as u32), ctx.r[27].u32 ) };
	// 8303D80C: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 8303D810: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8303D814: 4E800421  bctrl
	ctx.lr = 0x8303D818;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8303D818: 3B800001  li r28, 1
	ctx.r[28].s64 = 1;
	// 8303D81C: 3BBE0048  addi r29, r30, 0x48
	ctx.r[29].s64 = ctx.r[30].s64 + 72;
	// 8303D820: 907E0080  stw r3, 0x80(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(128 as u32), ctx.r[3].u32 ) };
	// 8303D824: 3B40000E  li r26, 0xe
	ctx.r[26].s64 = 14;
	// 8303D828: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 8303D82C: 419A00A0  beq cr6, 0x8303d8cc
	if ctx.cr[6].eq {
	pc = 0x8303D8CC; continue 'dispatch;
	}
	// 8303D830: 2B1C0003  cmplwi cr6, r28, 3
	ctx.cr[6].compare_u32(ctx.r[28].u32, 3 as u32, &mut ctx.xer);
	// 8303D834: 4099001C  ble cr6, 0x8303d850
	if !ctx.cr[6].gt {
	pc = 0x8303D850; continue 'dispatch;
	}
	// 8303D838: 2B1C0004  cmplwi cr6, r28, 4
	ctx.cr[6].compare_u32(ctx.r[28].u32, 4 as u32, &mut ctx.xer);
	// 8303D83C: 40990090  ble cr6, 0x8303d8cc
	if !ctx.cr[6].gt {
	pc = 0x8303D8CC; continue 'dispatch;
	}
	// 8303D840: 2B1C0006  cmplwi cr6, r28, 6
	ctx.cr[6].compare_u32(ctx.r[28].u32, 6 as u32, &mut ctx.xer);
	// 8303D844: 4099000C  ble cr6, 0x8303d850
	if !ctx.cr[6].gt {
	pc = 0x8303D850; continue 'dispatch;
	}
	// 8303D848: 2B1C000B  cmplwi cr6, r28, 0xb
	ctx.cr[6].compare_u32(ctx.r[28].u32, 11 as u32, &mut ctx.xer);
	// 8303D84C: 409A0080  bne cr6, 0x8303d8cc
	if !ctx.cr[6].eq {
	pc = 0x8303D8CC; continue 'dispatch;
	}
	// 8303D850: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 8303D854: 809E0000  lwz r4, 0(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303D858: 4BF9AA41  bl 0x82fd8298
	ctx.lr = 0x8303D85C;
	sub_82FD8298(ctx, base);
	// 8303D85C: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 8303D860: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8303D864: 41820028  beq 0x8303d88c
	if ctx.cr[0].eq {
	pc = 0x8303D88C; continue 'dispatch;
	}
	// 8303D868: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8303D86C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8303D870: 38A0001D  li r5, 0x1d
	ctx.r[5].s64 = 29;
	// 8303D874: 811E0000  lwz r8, 0(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303D878: 38800014  li r4, 0x14
	ctx.r[4].s64 = 20;
	// 8303D87C: 80CB007C  lwz r6, 0x7c(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(124 as u32) ) } as u64;
	// 8303D880: 4BFD03B1  bl 0x8300dc30
	ctx.lr = 0x8303D884;
	sub_8300DC30(ctx, base);
	// 8303D884: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8303D888: 48000008  b 0x8303d890
	pc = 0x8303D890; continue 'dispatch;
	// 8303D88C: 7F2BCB78  mr r11, r25
	ctx.r[11].u64 = ctx.r[25].u64;
	// 8303D890: 3860001C  li r3, 0x1c
	ctx.r[3].s64 = 28;
	// 8303D894: 917DFFC4  stw r11, -0x3c(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(-60 as u32), ctx.r[11].u32 ) };
	// 8303D898: 809E0000  lwz r4, 0(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303D89C: 4BF9A9FD  bl 0x82fd8298
	ctx.lr = 0x8303D8A0;
	sub_82FD8298(ctx, base);
	// 8303D8A0: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 8303D8A4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8303D8A8: 41820018  beq 0x8303d8c0
	if ctx.cr[0].eq {
	pc = 0x8303D8C0; continue 'dispatch;
	}
	// 8303D8AC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8303D8B0: 80DE0000  lwz r6, 0(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303D8B4: 3880001D  li r4, 0x1d
	ctx.r[4].s64 = 29;
	// 8303D8B8: 4800F6A9  bl 0x8304cf60
	ctx.lr = 0x8303D8BC;
	sub_8304CF60(ctx, base);
	// 8303D8BC: 48000008  b 0x8303d8c4
	pc = 0x8303D8C4; continue 'dispatch;
	// 8303D8C0: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 8303D8C4: 907D0000  stw r3, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 8303D8C8: 4800000C  b 0x8303d8d4
	pc = 0x8303D8D4; continue 'dispatch;
	// 8303D8CC: 933DFFC4  stw r25, -0x3c(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(-60 as u32), ctx.r[25].u32 ) };
	// 8303D8D0: 933D0000  stw r25, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[25].u32 ) };
	// 8303D8D4: 375AFFFF  addic. r26, r26, -1
	ctx.xer.ca = (ctx.r[26].u32 > (!(-1 as u32)));
	ctx.r[26].s64 = ctx.r[26].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[26].s32, 0, &mut ctx.xer);
	// 8303D8D8: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 8303D8DC: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 8303D8E0: 4082FF48  bne 0x8303d828
	if !ctx.cr[0].eq {
	pc = 0x8303D828; continue 'dispatch;
	}
	// 8303D8E4: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8303D8E8: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 8303D8EC: 4BF9A9AD  bl 0x82fd8298
	ctx.lr = 0x8303D8F0;
	sub_82FD8298(ctx, base);
	// 8303D8F0: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 8303D8F4: 93BF0050  stw r29, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[29].u32 ) };
	// 8303D8F8: 4182002C  beq 0x8303d924
	if ctx.cr[0].eq {
	pc = 0x8303D924; continue 'dispatch;
	}
	// 8303D8FC: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 8303D900: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8303D904: 38800005  li r4, 5
	ctx.r[4].s64 = 5;
	// 8303D908: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8303D90C: 4800EEE5  bl 0x8304c7f0
	ctx.lr = 0x8303D910;
	sub_8304C7F0(ctx, base);
	// 8303D910: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 8303D914: 7FAAEB78  mr r10, r29
	ctx.r[10].u64 = ctx.r[29].u64;
	// 8303D918: 396B2990  addi r11, r11, 0x2990
	ctx.r[11].s64 = ctx.r[11].s64 + 10640;
	// 8303D91C: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8303D920: 48000008  b 0x8303d928
	pc = 0x8303D928; continue 'dispatch;
	// 8303D924: 7F2ACB78  mr r10, r25
	ctx.r[10].u64 = ctx.r[25].u64;
	// 8303D928: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8303D92C: 915E0044  stw r10, 0x44(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(68 as u32), ctx.r[10].u32 ) };
	// 8303D930: 383F00A0  addi r1, r31, 0xa0
	ctx.r[1].s64 = ctx.r[31].s64 + 160;
	// 8303D934: 4816A878  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303D938(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8303D938 size=48
    let mut pc: u32 = 0x8303D938;
    'dispatch: loop {
        match pc {
            0x8303D938 => {
    //   block [0x8303D938..0x8303D968)
	// 8303D938: 3BECFF60  addi r31, r12, -0xa0
	ctx.r[31].s64 = ctx.r[12].s64 + -160;
	// 8303D93C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8303D940: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8303D944: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8303D948: 817F00B4  lwz r11, 0xb4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(180 as u32) ) } as u64;
	// 8303D94C: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303D950: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8303D954: 4BF9A98D  bl 0x82fd82e0
	ctx.lr = 0x8303D958;
	sub_82FD82E0(ctx, base);
	// 8303D958: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8303D95C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8303D960: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8303D964: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303D968(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8303D968 size=48
    let mut pc: u32 = 0x8303D968;
    'dispatch: loop {
        match pc {
            0x8303D968 => {
    //   block [0x8303D968..0x8303D998)
	// 8303D968: 3BECFF60  addi r31, r12, -0xa0
	ctx.r[31].s64 = ctx.r[12].s64 + -160;
	// 8303D96C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8303D970: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8303D974: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8303D978: 817F00B4  lwz r11, 0xb4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(180 as u32) ) } as u64;
	// 8303D97C: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303D980: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8303D984: 4BF9A95D  bl 0x82fd82e0
	ctx.lr = 0x8303D988;
	sub_82FD82E0(ctx, base);
	// 8303D988: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8303D98C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8303D990: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8303D994: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303D998(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8303D998 size=44
    let mut pc: u32 = 0x8303D998;
    'dispatch: loop {
        match pc {
            0x8303D998 => {
    //   block [0x8303D998..0x8303D9C4)
	// 8303D998: 3BECFF60  addi r31, r12, -0xa0
	ctx.r[31].s64 = ctx.r[12].s64 + -160;
	// 8303D99C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8303D9A0: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8303D9A4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8303D9A8: 809F00CC  lwz r4, 0xcc(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(204 as u32) ) } as u64;
	// 8303D9AC: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8303D9B0: 4BF9A931  bl 0x82fd82e0
	ctx.lr = 0x8303D9B4;
	sub_82FD82E0(ctx, base);
	// 8303D9B4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8303D9B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8303D9BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8303D9C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303D9C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8303D9C8 size=8
    let mut pc: u32 = 0x8303D9C8;
    'dispatch: loop {
        match pc {
            0x8303D9C8 => {
    //   block [0x8303D9C8..0x8303D9D0)
	// 8303D9C8: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8303D9CC: 8215D948  lwz r16, -0x26b8(r21)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-9912 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303D9D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8303D9D0 size=340
    let mut pc: u32 = 0x8303D9D0;
    'dispatch: loop {
        match pc {
            0x8303D9D0 => {
    //   block [0x8303D9D0..0x8303DB24)
	// 8303D9D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8303D9D4: 4816A789  bl 0x831a815c
	ctx.lr = 0x8303D9D8;
	sub_831A8130(ctx, base);
	// 8303D9D8: 3BE1FF60  addi r31, r1, -0xa0
	ctx.r[31].s64 = ctx.r[1].s64 + -160;
	// 8303D9DC: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8303D9E0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8303D9E4: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 8303D9E8: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 8303D9EC: 3B800001  li r28, 1
	ctx.r[28].s64 = 1;
	// 8303D9F0: 3BBE0048  addi r29, r30, 0x48
	ctx.r[29].s64 = ctx.r[30].s64 + 72;
	// 8303D9F4: 93DF00B4  stw r30, 0xb4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(180 as u32), ctx.r[30].u32 ) };
	// 8303D9F8: 3B20000E  li r25, 0xe
	ctx.r[25].s64 = 14;
	// 8303D9FC: 935F00CC  stw r26, 0xcc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(204 as u32), ctx.r[26].u32 ) };
	// 8303DA00: 935E0000  stw r26, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[26].u32 ) };
	// 8303DA04: 937E0004  stw r27, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[27].u32 ) };
	// 8303DA08: 90BE0080  stw r5, 0x80(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(128 as u32), ctx.r[5].u32 ) };
	// 8303DA0C: 909E0008  stw r4, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[4].u32 ) };
	// 8303DA10: 937E0044  stw r27, 0x44(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(68 as u32), ctx.r[27].u32 ) };
	// 8303DA14: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 8303DA18: 419A00A0  beq cr6, 0x8303dab8
	if ctx.cr[6].eq {
	pc = 0x8303DAB8; continue 'dispatch;
	}
	// 8303DA1C: 2B1C0003  cmplwi cr6, r28, 3
	ctx.cr[6].compare_u32(ctx.r[28].u32, 3 as u32, &mut ctx.xer);
	// 8303DA20: 4099001C  ble cr6, 0x8303da3c
	if !ctx.cr[6].gt {
	pc = 0x8303DA3C; continue 'dispatch;
	}
	// 8303DA24: 2B1C0004  cmplwi cr6, r28, 4
	ctx.cr[6].compare_u32(ctx.r[28].u32, 4 as u32, &mut ctx.xer);
	// 8303DA28: 40990090  ble cr6, 0x8303dab8
	if !ctx.cr[6].gt {
	pc = 0x8303DAB8; continue 'dispatch;
	}
	// 8303DA2C: 2B1C0006  cmplwi cr6, r28, 6
	ctx.cr[6].compare_u32(ctx.r[28].u32, 6 as u32, &mut ctx.xer);
	// 8303DA30: 4099000C  ble cr6, 0x8303da3c
	if !ctx.cr[6].gt {
	pc = 0x8303DA3C; continue 'dispatch;
	}
	// 8303DA34: 2B1C000B  cmplwi cr6, r28, 0xb
	ctx.cr[6].compare_u32(ctx.r[28].u32, 11 as u32, &mut ctx.xer);
	// 8303DA38: 409A0080  bne cr6, 0x8303dab8
	if !ctx.cr[6].eq {
	pc = 0x8303DAB8; continue 'dispatch;
	}
	// 8303DA3C: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 8303DA40: 809E0000  lwz r4, 0(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303DA44: 4BF9A855  bl 0x82fd8298
	ctx.lr = 0x8303DA48;
	sub_82FD8298(ctx, base);
	// 8303DA48: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 8303DA4C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8303DA50: 41820028  beq 0x8303da78
	if ctx.cr[0].eq {
	pc = 0x8303DA78; continue 'dispatch;
	}
	// 8303DA54: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8303DA58: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8303DA5C: 38A0001D  li r5, 0x1d
	ctx.r[5].s64 = 29;
	// 8303DA60: 811E0000  lwz r8, 0(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303DA64: 38800014  li r4, 0x14
	ctx.r[4].s64 = 20;
	// 8303DA68: 80CB007C  lwz r6, 0x7c(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(124 as u32) ) } as u64;
	// 8303DA6C: 4BFD01C5  bl 0x8300dc30
	ctx.lr = 0x8303DA70;
	sub_8300DC30(ctx, base);
	// 8303DA70: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8303DA74: 48000008  b 0x8303da7c
	pc = 0x8303DA7C; continue 'dispatch;
	// 8303DA78: 7F6BDB78  mr r11, r27
	ctx.r[11].u64 = ctx.r[27].u64;
	// 8303DA7C: 3860001C  li r3, 0x1c
	ctx.r[3].s64 = 28;
	// 8303DA80: 917DFFC4  stw r11, -0x3c(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(-60 as u32), ctx.r[11].u32 ) };
	// 8303DA84: 809E0000  lwz r4, 0(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303DA88: 4BF9A811  bl 0x82fd8298
	ctx.lr = 0x8303DA8C;
	sub_82FD8298(ctx, base);
	// 8303DA8C: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 8303DA90: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8303DA94: 41820018  beq 0x8303daac
	if ctx.cr[0].eq {
	pc = 0x8303DAAC; continue 'dispatch;
	}
	// 8303DA98: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8303DA9C: 80DE0000  lwz r6, 0(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303DAA0: 3880001D  li r4, 0x1d
	ctx.r[4].s64 = 29;
	// 8303DAA4: 4800F4BD  bl 0x8304cf60
	ctx.lr = 0x8303DAA8;
	sub_8304CF60(ctx, base);
	// 8303DAA8: 48000008  b 0x8303dab0
	pc = 0x8303DAB0; continue 'dispatch;
	// 8303DAAC: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 8303DAB0: 907D0000  stw r3, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 8303DAB4: 4800000C  b 0x8303dac0
	pc = 0x8303DAC0; continue 'dispatch;
	// 8303DAB8: 937DFFC4  stw r27, -0x3c(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(-60 as u32), ctx.r[27].u32 ) };
	// 8303DABC: 937D0000  stw r27, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 8303DAC0: 3739FFFF  addic. r25, r25, -1
	ctx.xer.ca = (ctx.r[25].u32 > (!(-1 as u32)));
	ctx.r[25].s64 = ctx.r[25].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[25].s32, 0, &mut ctx.xer);
	// 8303DAC4: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 8303DAC8: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 8303DACC: 4082FF48  bne 0x8303da14
	if !ctx.cr[0].eq {
	pc = 0x8303DA14; continue 'dispatch;
	}
	// 8303DAD0: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 8303DAD4: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 8303DAD8: 4BF9A7C1  bl 0x82fd8298
	ctx.lr = 0x8303DADC;
	sub_82FD8298(ctx, base);
	// 8303DADC: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 8303DAE0: 93BF0050  stw r29, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[29].u32 ) };
	// 8303DAE4: 4182002C  beq 0x8303db10
	if ctx.cr[0].eq {
	pc = 0x8303DB10; continue 'dispatch;
	}
	// 8303DAE8: 7F46D378  mr r6, r26
	ctx.r[6].u64 = ctx.r[26].u64;
	// 8303DAEC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8303DAF0: 38800005  li r4, 5
	ctx.r[4].s64 = 5;
	// 8303DAF4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8303DAF8: 4800ECF9  bl 0x8304c7f0
	ctx.lr = 0x8303DAFC;
	sub_8304C7F0(ctx, base);
	// 8303DAFC: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 8303DB00: 7FAAEB78  mr r10, r29
	ctx.r[10].u64 = ctx.r[29].u64;
	// 8303DB04: 396B2990  addi r11, r11, 0x2990
	ctx.r[11].s64 = ctx.r[11].s64 + 10640;
	// 8303DB08: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8303DB0C: 48000008  b 0x8303db14
	pc = 0x8303DB14; continue 'dispatch;
	// 8303DB10: 7F6ADB78  mr r10, r27
	ctx.r[10].u64 = ctx.r[27].u64;
	// 8303DB14: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8303DB18: 915E0044  stw r10, 0x44(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(68 as u32), ctx.r[10].u32 ) };
	// 8303DB1C: 383F00A0  addi r1, r31, 0xa0
	ctx.r[1].s64 = ctx.r[31].s64 + 160;
	// 8303DB20: 4816A68C  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303DB24(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8303DB24 size=48
    let mut pc: u32 = 0x8303DB24;
    'dispatch: loop {
        match pc {
            0x8303DB24 => {
    //   block [0x8303DB24..0x8303DB54)
	// 8303DB24: 3BECFF60  addi r31, r12, -0xa0
	ctx.r[31].s64 = ctx.r[12].s64 + -160;
	// 8303DB28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8303DB2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8303DB30: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8303DB34: 817F00B4  lwz r11, 0xb4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(180 as u32) ) } as u64;
	// 8303DB38: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303DB3C: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8303DB40: 4BF9A7A1  bl 0x82fd82e0
	ctx.lr = 0x8303DB44;
	sub_82FD82E0(ctx, base);
	// 8303DB44: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8303DB48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8303DB4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8303DB50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303DB54(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8303DB54 size=48
    let mut pc: u32 = 0x8303DB54;
    'dispatch: loop {
        match pc {
            0x8303DB54 => {
    //   block [0x8303DB54..0x8303DB84)
	// 8303DB54: 3BECFF60  addi r31, r12, -0xa0
	ctx.r[31].s64 = ctx.r[12].s64 + -160;
	// 8303DB58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8303DB5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8303DB60: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8303DB64: 817F00B4  lwz r11, 0xb4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(180 as u32) ) } as u64;
	// 8303DB68: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303DB6C: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8303DB70: 4BF9A771  bl 0x82fd82e0
	ctx.lr = 0x8303DB74;
	sub_82FD82E0(ctx, base);
	// 8303DB74: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8303DB78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8303DB7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8303DB80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303DB84(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8303DB84 size=44
    let mut pc: u32 = 0x8303DB84;
    'dispatch: loop {
        match pc {
            0x8303DB84 => {
    //   block [0x8303DB84..0x8303DBB0)
	// 8303DB84: 3BECFF60  addi r31, r12, -0xa0
	ctx.r[31].s64 = ctx.r[12].s64 + -160;
	// 8303DB88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8303DB8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8303DB90: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8303DB94: 809F00CC  lwz r4, 0xcc(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(204 as u32) ) } as u64;
	// 8303DB98: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8303DB9C: 4BF9A745  bl 0x82fd82e0
	ctx.lr = 0x8303DBA0;
	sub_82FD82E0(ctx, base);
	// 8303DBA0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8303DBA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8303DBA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8303DBAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303DBB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8303DBB0 size=180
    let mut pc: u32 = 0x8303DBB0;
    'dispatch: loop {
        match pc {
            0x8303DBB0 => {
    //   block [0x8303DBB0..0x8303DC64)
	// 8303DBB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8303DBB4: 4816A5B1  bl 0x831a8164
	ctx.lr = 0x8303DBB8;
	sub_831A8130(ctx, base);
	// 8303DBB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8303DBBC: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 8303DBC0: 3BE00001  li r31, 1
	ctx.r[31].s64 = 1;
	// 8303DBC4: 3BDB0048  addi r30, r27, 0x48
	ctx.r[30].s64 = ctx.r[27].s64 + 72;
	// 8303DBC8: 3B80000E  li r28, 0xe
	ctx.r[28].s64 = 14;
	// 8303DBCC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8303DBD0: 419A005C  beq cr6, 0x8303dc2c
	if ctx.cr[6].eq {
	pc = 0x8303DC2C; continue 'dispatch;
	}
	// 8303DBD4: 2B1F0003  cmplwi cr6, r31, 3
	ctx.cr[6].compare_u32(ctx.r[31].u32, 3 as u32, &mut ctx.xer);
	// 8303DBD8: 4099001C  ble cr6, 0x8303dbf4
	if !ctx.cr[6].gt {
	pc = 0x8303DBF4; continue 'dispatch;
	}
	// 8303DBDC: 2B1F0004  cmplwi cr6, r31, 4
	ctx.cr[6].compare_u32(ctx.r[31].u32, 4 as u32, &mut ctx.xer);
	// 8303DBE0: 4099004C  ble cr6, 0x8303dc2c
	if !ctx.cr[6].gt {
	pc = 0x8303DC2C; continue 'dispatch;
	}
	// 8303DBE4: 2B1F0006  cmplwi cr6, r31, 6
	ctx.cr[6].compare_u32(ctx.r[31].u32, 6 as u32, &mut ctx.xer);
	// 8303DBE8: 4099000C  ble cr6, 0x8303dbf4
	if !ctx.cr[6].gt {
	pc = 0x8303DBF4; continue 'dispatch;
	}
	// 8303DBEC: 2B1F000B  cmplwi cr6, r31, 0xb
	ctx.cr[6].compare_u32(ctx.r[31].u32, 11 as u32, &mut ctx.xer);
	// 8303DBF0: 409A003C  bne cr6, 0x8303dc2c
	if !ctx.cr[6].eq {
	pc = 0x8303DC2C; continue 'dispatch;
	}
	// 8303DBF4: 83BEFFC4  lwz r29, -0x3c(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-60 as u32) ) } as u64;
	// 8303DBF8: 281D0000  cmplwi r29, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8303DBFC: 41820014  beq 0x8303dc10
	if ctx.cr[0].eq {
	pc = 0x8303DC10; continue 'dispatch;
	}
	// 8303DC00: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8303DC04: 4BFAE8B5  bl 0x82fec4b8
	ctx.lr = 0x8303DC08;
	sub_82FEC4B8(ctx, base);
	// 8303DC08: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8303DC0C: 4BF9A6D5  bl 0x82fd82e0
	ctx.lr = 0x8303DC10;
	sub_82FD82E0(ctx, base);
	// 8303DC10: 83BE0000  lwz r29, 0(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303DC14: 281D0000  cmplwi r29, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8303DC18: 41820014  beq 0x8303dc2c
	if ctx.cr[0].eq {
	pc = 0x8303DC2C; continue 'dispatch;
	}
	// 8303DC1C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8303DC20: 4BFA9049  bl 0x82fe6c68
	ctx.lr = 0x8303DC24;
	sub_82FE6C68(ctx, base);
	// 8303DC24: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8303DC28: 4BF9A6B9  bl 0x82fd82e0
	ctx.lr = 0x8303DC2C;
	sub_82FD82E0(ctx, base);
	// 8303DC2C: 379CFFFF  addic. r28, r28, -1
	ctx.xer.ca = (ctx.r[28].u32 > (!(-1 as u32)));
	ctx.r[28].s64 = ctx.r[28].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 8303DC30: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 8303DC34: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 8303DC38: 4082FF94  bne 0x8303dbcc
	if !ctx.cr[0].eq {
	pc = 0x8303DBCC; continue 'dispatch;
	}
	// 8303DC3C: 807B0044  lwz r3, 0x44(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(68 as u32) ) } as u64;
	// 8303DC40: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8303DC44: 41820018  beq 0x8303dc5c
	if ctx.cr[0].eq {
	pc = 0x8303DC5C; continue 'dispatch;
	}
	// 8303DC48: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303DC4C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8303DC50: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303DC54: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8303DC58: 4E800421  bctrl
	ctx.lr = 0x8303DC5C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8303DC5C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8303DC60: 4816A554  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303DC68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8303DC68 size=16
    let mut pc: u32 = 0x8303DC68;
    'dispatch: loop {
        match pc {
            0x8303DC68 => {
    //   block [0x8303DC68..0x8303DC78)
	// 8303DC68: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 8303DC6C: 396BD9B0  addi r11, r11, -0x2650
	ctx.r[11].s64 = ctx.r[11].s64 + -9808;
	// 8303DC70: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8303DC74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303DC78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8303DC78 size=68
    let mut pc: u32 = 0x8303DC78;
    'dispatch: loop {
        match pc {
            0x8303DC78 => {
    //   block [0x8303DC78..0x8303DCBC)
	// 8303DC78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8303DC7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8303DC80: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8303DC84: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8303DC88: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 8303DC8C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8303DC90: 396BD9B0  addi r11, r11, -0x2650
	ctx.r[11].s64 = ctx.r[11].s64 + -9808;
	// 8303DC94: 548A07FF  clrlwi. r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8303DC98: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8303DC9C: 41820008  beq 0x8303dca4
	if ctx.cr[0].eq {
	pc = 0x8303DCA4; continue 'dispatch;
	}
	// 8303DCA0: 4BF9A641  bl 0x82fd82e0
	ctx.lr = 0x8303DCA4;
	sub_82FD82E0(ctx, base);
	// 8303DCA4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8303DCA8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8303DCAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8303DCB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8303DCB4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8303DCB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303DCC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8303DCC0 size=328
    let mut pc: u32 = 0x8303DCC0;
    'dispatch: loop {
        match pc {
            0x8303DCC0 => {
    //   block [0x8303DCC0..0x8303DE08)
	// 8303DCC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8303DCC4: 4816A4A9  bl 0x831a816c
	ctx.lr = 0x8303DCC8;
	sub_831A8130(ctx, base);
	// 8303DCC8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8303DCCC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8303DCD0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8303DCD4: 807F0068  lwz r3, 0x68(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) } as u64;
	// 8303DCD8: 809F0024  lwz r4, 0x24(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 8303DCDC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303DCE0: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8303DCE4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8303DCE8: 4E800421  bctrl
	ctx.lr = 0x8303DCEC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8303DCEC: 807F0068  lwz r3, 0x68(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) } as u64;
	// 8303DCF0: 809F0028  lwz r4, 0x28(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 8303DCF4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303DCF8: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8303DCFC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8303DD00: 4E800421  bctrl
	ctx.lr = 0x8303DD04;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8303DD04: 807F0068  lwz r3, 0x68(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) } as u64;
	// 8303DD08: 809F002C  lwz r4, 0x2c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 8303DD0C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303DD10: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8303DD14: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8303DD18: 4E800421  bctrl
	ctx.lr = 0x8303DD1C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8303DD1C: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8303DD20: 419A00D0  beq cr6, 0x8303ddf0
	if ctx.cr[6].eq {
	pc = 0x8303DDF0; continue 'dispatch;
	}
	// 8303DD24: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8303DD28: 809F0068  lwz r4, 0x68(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) } as u64;
	// 8303DD2C: 4BF92E55  bl 0x82fd0b80
	ctx.lr = 0x8303DD30;
	sub_82FD0B80(ctx, base);
	// 8303DD30: 3880002C  li r4, 0x2c
	ctx.r[4].s64 = 44;
	// 8303DD34: 907F0024  stw r3, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[3].u32 ) };
	// 8303DD38: 4BF94079  bl 0x82fd1db0
	ctx.lr = 0x8303DD3C;
	sub_82FD1DB0(ctx, base);
	// 8303DD3C: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 8303DD40: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8303DD44: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8303DD48: 41820034  beq 0x8303dd7c
	if ctx.cr[0].eq {
	pc = 0x8303DD7C; continue 'dispatch;
	}
	// 8303DD4C: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303DD50: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8303DD54: 41820028  beq 0x8303dd7c
	if ctx.cr[0].eq {
	pc = 0x8303DD7C; continue 'dispatch;
	}
	// 8303DD58: 394B0002  addi r10, r11, 2
	ctx.r[10].s64 = ctx.r[11].s64 + 2;
	// 8303DD5C: 48000008  b 0x8303dd64
	pc = 0x8303DD64; continue 'dispatch;
	// 8303DD60: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 8303DD64: A12A0000  lhz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303DD68: 28090000  cmplwi r9, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8303DD6C: 4082FFF4  bne 0x8303dd60
	if !ctx.cr[0].eq {
	pc = 0x8303DD60; continue 'dispatch;
	}
	// 8303DD70: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 8303DD74: 7D7D0E70  srawi r29, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[29].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 8303DD78: 48000008  b 0x8303dd80
	pc = 0x8303DD80; continue 'dispatch;
	// 8303DD7C: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 8303DD80: 7D7EE850  subf r11, r30, r29
	ctx.r[11].s64 = ctx.r[29].s64 - ctx.r[30].s64;
	// 8303DD84: 807F0068  lwz r3, 0x68(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) } as u64;
	// 8303DD88: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8303DD8C: 5564083C  slwi r4, r11, 1
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 8303DD90: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303DD94: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8303DD98: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8303DD9C: 4E800421  bctrl
	ctx.lr = 0x8303DDA0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8303DDA0: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 8303DDA4: 38BE0001  addi r5, r30, 1
	ctx.r[5].s64 = ctx.r[30].s64 + 1;
	// 8303DDA8: 80FF0068  lwz r7, 0x68(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) } as u64;
	// 8303DDAC: 809F0024  lwz r4, 0x24(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 8303DDB0: 907F0028  stw r3, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[3].u32 ) };
	// 8303DDB4: 4BF942D5  bl 0x82fd2088
	ctx.lr = 0x8303DDB8;
	sub_82FD2088(ctx, base);
	// 8303DDB8: 807F0068  lwz r3, 0x68(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) } as u64;
	// 8303DDBC: 397E0001  addi r11, r30, 1
	ctx.r[11].s64 = ctx.r[30].s64 + 1;
	// 8303DDC0: 5564083C  slwi r4, r11, 1
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 8303DDC4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303DDC8: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8303DDCC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8303DDD0: 4E800421  bctrl
	ctx.lr = 0x8303DDD4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8303DDD4: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 8303DDD8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8303DDDC: 80FF0068  lwz r7, 0x68(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) } as u64;
	// 8303DDE0: 809F0024  lwz r4, 0x24(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 8303DDE4: 907F002C  stw r3, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[3].u32 ) };
	// 8303DDE8: 4BF942A1  bl 0x82fd2088
	ctx.lr = 0x8303DDEC;
	sub_82FD2088(ctx, base);
	// 8303DDEC: 48000014  b 0x8303de00
	pc = 0x8303DE00; continue 'dispatch;
	// 8303DDF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8303DDF4: 917F002C  stw r11, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 8303DDF8: 917F0028  stw r11, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 8303DDFC: 917F0024  stw r11, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 8303DE00: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8303DE04: 4816A3B8  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303DE08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8303DE08 size=88
    let mut pc: u32 = 0x8303DE08;
    'dispatch: loop {
        match pc {
            0x8303DE08 => {
    //   block [0x8303DE08..0x8303DE60)
	// 8303DE08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8303DE0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8303DE10: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8303DE14: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8303DE18: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8303DE1C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8303DE20: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8303DE24: 807F0040  lwz r3, 0x40(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as u64;
	// 8303DE28: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8303DE2C: 41820018  beq 0x8303de44
	if ctx.cr[0].eq {
	pc = 0x8303DE44; continue 'dispatch;
	}
	// 8303DE30: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303DE34: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8303DE38: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303DE3C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8303DE40: 4E800421  bctrl
	ctx.lr = 0x8303DE44;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8303DE44: 93DF0040  stw r30, 0x40(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), ctx.r[30].u32 ) };
	// 8303DE48: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8303DE4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8303DE50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8303DE54: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8303DE58: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8303DE5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303DE60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8303DE60 size=128
    let mut pc: u32 = 0x8303DE60;
    'dispatch: loop {
        match pc {
            0x8303DE60 => {
    //   block [0x8303DE60..0x8303DEE0)
	// 8303DE60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8303DE64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8303DE68: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8303DE6C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8303DE70: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8303DE74: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8303DE78: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8303DE7C: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8303DE80: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8303DE84: 41820018  beq 0x8303de9c
	if ctx.cr[0].eq {
	pc = 0x8303DE9C; continue 'dispatch;
	}
	// 8303DE88: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303DE8C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8303DE90: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303DE94: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8303DE98: 4E800421  bctrl
	ctx.lr = 0x8303DE9C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8303DE9C: 809F0054  lwz r4, 0x54(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 8303DEA0: 93DF0050  stw r30, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 8303DEA4: 28040000  cmplwi r4, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8303DEA8: 41820020  beq 0x8303dec8
	if ctx.cr[0].eq {
	pc = 0x8303DEC8; continue 'dispatch;
	}
	// 8303DEAC: 807F0068  lwz r3, 0x68(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) } as u64;
	// 8303DEB0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303DEB4: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8303DEB8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8303DEBC: 4E800421  bctrl
	ctx.lr = 0x8303DEC0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8303DEC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8303DEC4: 917F0054  stw r11, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8303DEC8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8303DECC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8303DED0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8303DED4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8303DED8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8303DEDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303DEE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8303DEE0 size=152
    let mut pc: u32 = 0x8303DEE0;
    'dispatch: loop {
        match pc {
            0x8303DEE0 => {
    //   block [0x8303DEE0..0x8303DF78)
	// 8303DEE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8303DEE4: 4816A289  bl 0x831a816c
	ctx.lr = 0x8303DEE8;
	sub_831A8130(ctx, base);
	// 8303DEE8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8303DEEC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8303DEF0: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8303DEF4: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 8303DEF8: 815F0018  lwz r10, 0x18(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 8303DEFC: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8303DF00: 409A0054  bne cr6, 0x8303df54
	if !ctx.cr[6].eq {
	pc = 0x8303DF54; continue 'dispatch;
	}
	// 8303DF04: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8303DF08: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8303DF0C: 5564103A  slwi r4, r11, 2
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 8303DF10: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 8303DF14: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303DF18: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8303DF1C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8303DF20: 4E800421  bctrl
	ctx.lr = 0x8303DF24;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8303DF24: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 8303DF28: 809F0010  lwz r4, 0x10(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8303DF2C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8303DF30: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 8303DF34: 4816A5DD  bl 0x831a8510
	ctx.lr = 0x8303DF38;
	sub_831A8510(ctx, base);
	// 8303DF38: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8303DF3C: 809F0010  lwz r4, 0x10(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8303DF40: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303DF44: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8303DF48: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8303DF4C: 4E800421  bctrl
	ctx.lr = 0x8303DF50;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8303DF50: 93DF0010  stw r30, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[30].u32 ) };
	// 8303DF54: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 8303DF58: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8303DF5C: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8303DF60: 7FAA592E  stwx r29, r10, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32), ctx.r[29].u32) };
	// 8303DF64: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 8303DF68: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8303DF6C: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 8303DF70: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8303DF74: 4816A248  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303DF78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8303DF78 size=8
    let mut pc: u32 = 0x8303DF78;
    'dispatch: loop {
        match pc {
            0x8303DF78 => {
    //   block [0x8303DF78..0x8303DF80)
	// 8303DF78: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8303DF7C: 8215D9E8  lwz r16, -0x2618(r21)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-9752 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303DF80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8303DF80 size=168
    let mut pc: u32 = 0x8303DF80;
    'dispatch: loop {
        match pc {
            0x8303DF80 => {
    //   block [0x8303DF80..0x8303E028)
	// 8303DF80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8303DF84: 4816A1E5  bl 0x831a8168
	ctx.lr = 0x8303DF88;
	sub_831A8130(ctx, base);
	// 8303DF88: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 8303DF8C: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8303DF90: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8303DF94: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 8303DF98: 93DF0094  stw r30, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[30].u32 ) };
	// 8303DF9C: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 8303DFA0: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 8303DFA4: 90DE0004  stw r6, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[6].u32 ) };
	// 8303DFA8: 396BD9C8  addi r11, r11, -0x2638
	ctx.r[11].s64 = ctx.r[11].s64 + -9784;
	// 8303DFAC: 54AA063F  clrlwi. r10, r5, 0x18
	ctx.r[10].u64 = ctx.r[5].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8303DFB0: 93BE000C  stw r29, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[29].u32 ) };
	// 8303DFB4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8303DFB8: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8303DFBC: 93BE0010  stw r29, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[29].u32 ) };
	// 8303DFC0: 93BE0014  stw r29, 0x14(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), ctx.r[29].u32 ) };
	// 8303DFC4: 93BE0018  stw r29, 0x18(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), ctx.r[29].u32 ) };
	// 8303DFC8: 93BE0008  stw r29, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 8303DFCC: 997E001C  stb r11, 0x1c(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(28 as u32), ctx.r[11].u8 ) };
	// 8303DFD0: 997E001D  stb r11, 0x1d(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(29 as u32), ctx.r[11].u8 ) };
	// 8303DFD4: 917E0020  stw r11, 0x20(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 8303DFD8: 917E0024  stw r11, 0x24(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 8303DFDC: 4182003C  beq 0x8303e018
	if ctx.cr[0].eq {
	pc = 0x8303E018; continue 'dispatch;
	}
	// 8303DFE0: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 8303DFE4: 419A0038  beq cr6, 0x8303e01c
	if ctx.cr[6].eq {
	pc = 0x8303E01C; continue 'dispatch;
	}
	// 8303DFE8: 7CC43378  mr r4, r6
	ctx.r[4].u64 = ctx.r[6].u64;
	// 8303DFEC: 38600024  li r3, 0x24
	ctx.r[3].s64 = 36;
	// 8303DFF0: 4BF9A2A9  bl 0x82fd8298
	ctx.lr = 0x8303DFF4;
	sub_82FD8298(ctx, base);
	// 8303DFF4: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 8303DFF8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8303DFFC: 41820010  beq 0x8303e00c
	if ctx.cr[0].eq {
	pc = 0x8303E00C; continue 'dispatch;
	}
	// 8303E000: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8303E004: 4BFA01ED  bl 0x82fde1f0
	ctx.lr = 0x8303E008;
	sub_82FDE1F0(ctx, base);
	// 8303E008: 48000008  b 0x8303e010
	pc = 0x8303E010; continue 'dispatch;
	// 8303E00C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8303E010: 907E0008  stw r3, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[3].u32 ) };
	// 8303E014: 48000008  b 0x8303e01c
	pc = 0x8303E01C; continue 'dispatch;
	// 8303E018: 939E0008  stw r28, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[28].u32 ) };
	// 8303E01C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8303E020: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 8303E024: 4816A194  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303E028(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8303E028 size=40
    let mut pc: u32 = 0x8303E028;
    'dispatch: loop {
        match pc {
            0x8303E028 => {
    //   block [0x8303E028..0x8303E050)
	// 8303E028: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 8303E02C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8303E030: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8303E034: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8303E038: 807F0094  lwz r3, 0x94(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 8303E03C: 4800E725  bl 0x8304c760
	ctx.lr = 0x8303E040;
	sub_8304C760(ctx, base);
	// 8303E040: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8303E044: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8303E048: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8303E04C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303E050(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8303E050 size=48
    let mut pc: u32 = 0x8303E050;
    'dispatch: loop {
        match pc {
            0x8303E050 => {
    //   block [0x8303E050..0x8303E080)
	// 8303E050: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 8303E054: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8303E058: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8303E05C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8303E060: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 8303E064: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8303E068: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8303E06C: 4BF9A275  bl 0x82fd82e0
	ctx.lr = 0x8303E070;
	sub_82FD82E0(ctx, base);
	// 8303E070: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8303E074: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8303E078: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8303E07C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303E080(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8303E080 size=8
    let mut pc: u32 = 0x8303E080;
    'dispatch: loop {
        match pc {
            0x8303E080 => {
    //   block [0x8303E080..0x8303E088)
	// 8303E080: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8303E084: 8215DA30  lwz r16, -0x25d0(r21)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-9680 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303E088(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8303E088 size=200
    let mut pc: u32 = 0x8303E088;
    'dispatch: loop {
        match pc {
            0x8303E088 => {
    //   block [0x8303E088..0x8303E150)
	// 8303E088: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8303E08C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8303E090: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8303E094: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8303E098: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 8303E09C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8303E0A0: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 8303E0A4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8303E0A8: 396BD9C8  addi r11, r11, -0x2638
	ctx.r[11].s64 = ctx.r[11].s64 + -9784;
	// 8303E0AC: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 8303E0B0: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8303E0B4: 897E001C  lbz r11, 0x1c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 8303E0B8: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8303E0BC: 41820024  beq 0x8303e0e0
	if ctx.cr[0].eq {
	pc = 0x8303E0E0; continue 'dispatch;
	}
	// 8303E0C0: 807E0010  lwz r3, 0x10(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 8303E0C4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8303E0C8: 41820018  beq 0x8303e0e0
	if ctx.cr[0].eq {
	pc = 0x8303E0E0; continue 'dispatch;
	}
	// 8303E0CC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303E0D0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8303E0D4: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303E0D8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8303E0DC: 4E800421  bctrl
	ctx.lr = 0x8303E0E0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8303E0E0: 897E001D  lbz r11, 0x1d(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(29 as u32) ) } as u64;
	// 8303E0E4: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8303E0E8: 41820024  beq 0x8303e10c
	if ctx.cr[0].eq {
	pc = 0x8303E10C; continue 'dispatch;
	}
	// 8303E0EC: 807E0014  lwz r3, 0x14(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 8303E0F0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8303E0F4: 41820018  beq 0x8303e10c
	if ctx.cr[0].eq {
	pc = 0x8303E10C; continue 'dispatch;
	}
	// 8303E0F8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303E0FC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8303E100: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303E104: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8303E108: 4E800421  bctrl
	ctx.lr = 0x8303E10C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8303E10C: 807E0008  lwz r3, 8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8303E110: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8303E114: 41820018  beq 0x8303e12c
	if ctx.cr[0].eq {
	pc = 0x8303E12C; continue 'dispatch;
	}
	// 8303E118: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303E11C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8303E120: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303E124: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8303E128: 4E800421  bctrl
	ctx.lr = 0x8303E12C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8303E12C: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8303E130: 396BA93C  addi r11, r11, -0x56c4
	ctx.r[11].s64 = ctx.r[11].s64 + -22212;
	// 8303E134: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8303E138: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 8303E13C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8303E140: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8303E144: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8303E148: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8303E14C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303E150(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8303E150 size=40
    let mut pc: u32 = 0x8303E150;
    'dispatch: loop {
        match pc {
            0x8303E150 => {
    //   block [0x8303E150..0x8303E178)
	// 8303E150: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 8303E154: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8303E158: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8303E15C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8303E160: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 8303E164: 4800E5FD  bl 0x8304c760
	ctx.lr = 0x8303E168;
	sub_8304C760(ctx, base);
	// 8303E168: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8303E16C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8303E170: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8303E174: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303E178(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8303E178 size=100
    let mut pc: u32 = 0x8303E178;
    'dispatch: loop {
        match pc {
            0x8303E178 => {
    //   block [0x8303E178..0x8303E1DC)
	// 8303E178: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8303E17C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8303E180: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8303E184: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8303E188: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8303E18C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8303E190: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8303E194: 897F001C  lbz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 8303E198: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8303E19C: 41820024  beq 0x8303e1c0
	if ctx.cr[0].eq {
	pc = 0x8303E1C0; continue 'dispatch;
	}
	// 8303E1A0: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8303E1A4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8303E1A8: 41820018  beq 0x8303e1c0
	if ctx.cr[0].eq {
	pc = 0x8303E1C0; continue 'dispatch;
	}
	// 8303E1AC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303E1B0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8303E1B4: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303E1B8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8303E1BC: 4E800421  bctrl
	ctx.lr = 0x8303E1C0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8303E1C0: 93DF0010  stw r30, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[30].u32 ) };
	// 8303E1C4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8303E1C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8303E1CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8303E1D0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8303E1D4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8303E1D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303E1E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8303E1E0 size=100
    let mut pc: u32 = 0x8303E1E0;
    'dispatch: loop {
        match pc {
            0x8303E1E0 => {
    //   block [0x8303E1E0..0x8303E244)
	// 8303E1E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8303E1E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8303E1E8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8303E1EC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8303E1F0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8303E1F4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8303E1F8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8303E1FC: 897F001D  lbz r11, 0x1d(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(29 as u32) ) } as u64;
	// 8303E200: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8303E204: 41820024  beq 0x8303e228
	if ctx.cr[0].eq {
	pc = 0x8303E228; continue 'dispatch;
	}
	// 8303E208: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 8303E20C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8303E210: 41820018  beq 0x8303e228
	if ctx.cr[0].eq {
	pc = 0x8303E228; continue 'dispatch;
	}
	// 8303E214: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303E218: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8303E21C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303E220: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8303E224: 4E800421  bctrl
	ctx.lr = 0x8303E228;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8303E228: 93DF0014  stw r30, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[30].u32 ) };
	// 8303E22C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8303E230: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8303E234: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8303E238: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8303E23C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8303E240: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303E248(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8303E248 size=8
    let mut pc: u32 = 0x8303E248;
    'dispatch: loop {
        match pc {
            0x8303E248 => {
    //   block [0x8303E248..0x8303E250)
	// 8303E248: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8303E24C: 8215DAA0  lwz r16, -0x2560(r21)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-9568 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303E250(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8303E250 size=324
    let mut pc: u32 = 0x8303E250;
    'dispatch: loop {
        match pc {
            0x8303E250 => {
    //   block [0x8303E250..0x8303E394)
	// 8303E250: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8303E254: 48169F05  bl 0x831a8158
	ctx.lr = 0x8303E258;
	sub_831A8130(ctx, base);
	// 8303E258: 3BE1FF60  addi r31, r1, -0xa0
	ctx.r[31].s64 = ctx.r[1].s64 + -160;
	// 8303E25C: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8303E260: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8303E264: 7D1D4378  mr r29, r8
	ctx.r[29].u64 = ctx.r[8].u64;
	// 8303E268: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 8303E26C: 7CD83378  mr r24, r6
	ctx.r[24].u64 = ctx.r[6].u64;
	// 8303E270: 93DF00B4  stw r30, 0xb4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(180 as u32), ctx.r[30].u32 ) };
	// 8303E274: 93BF00DC  stw r29, 0xdc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(220 as u32), ctx.r[29].u32 ) };
	// 8303E278: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 8303E27C: 3B200000  li r25, 0
	ctx.r[25].s64 = 0;
	// 8303E280: 90FE000C  stw r7, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[7].u32 ) };
	// 8303E284: 396BDA60  addi r11, r11, -0x25a0
	ctx.r[11].s64 = ctx.r[11].s64 + -9632;
	// 8303E288: 989E0010  stb r4, 0x10(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[4].u8 ) };
	// 8303E28C: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 8303E290: 93BE0014  stw r29, 0x14(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), ctx.r[29].u32 ) };
	// 8303E294: 3F408217  lis r26, -0x7de9
	ctx.r[26].s64 = -2112421888;
	// 8303E298: 933E0004  stw r25, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[25].u32 ) };
	// 8303E29C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8303E2A0: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8303E2A4: 933E0008  stw r25, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[25].u32 ) };
	// 8303E2A8: 3B6B8158  addi r27, r11, -0x7ea8
	ctx.r[27].s64 = ctx.r[11].s64 + -32424;
	// 8303E2AC: 419A0034  beq cr6, 0x8303e2e0
	if ctx.cr[6].eq {
	pc = 0x8303E2E0; continue 'dispatch;
	}
	// 8303E2B0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8303E2B4: 38600024  li r3, 0x24
	ctx.r[3].s64 = 36;
	// 8303E2B8: 4BF99FE1  bl 0x82fd8298
	ctx.lr = 0x8303E2BC;
	sub_82FD8298(ctx, base);
	// 8303E2BC: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 8303E2C0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8303E2C4: 41820010  beq 0x8303e2d4
	if ctx.cr[0].eq {
	pc = 0x8303E2D4; continue 'dispatch;
	}
	// 8303E2C8: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8303E2CC: 4BF9FF25  bl 0x82fde1f0
	ctx.lr = 0x8303E2D0;
	sub_82FDE1F0(ctx, base);
	// 8303E2D0: 48000008  b 0x8303e2d8
	pc = 0x8303E2D8; continue 'dispatch;
	// 8303E2D4: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 8303E2D8: 907E0004  stw r3, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 8303E2DC: 4800003C  b 0x8303e318
	pc = 0x8303E318; continue 'dispatch;
	// 8303E2E0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8303E2E4: 38600024  li r3, 0x24
	ctx.r[3].s64 = 36;
	// 8303E2E8: 4BF99FB1  bl 0x82fd8298
	ctx.lr = 0x8303E2EC;
	sub_82FD8298(ctx, base);
	// 8303E2EC: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 8303E2F0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8303E2F4: 4182001C  beq 0x8303e310
	if ctx.cr[0].eq {
	pc = 0x8303E310; continue 'dispatch;
	}
	// 8303E2F8: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 8303E2FC: 80DA9770  lwz r6, -0x6890(r26)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-26768 as u32) ) } as u64;
	// 8303E300: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 8303E304: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8303E308: 4BFA0921  bl 0x82fdec28
	ctx.lr = 0x8303E30C;
	sub_82FDEC28(ctx, base);
	// 8303E30C: 48000008  b 0x8303e314
	pc = 0x8303E314; continue 'dispatch;
	// 8303E310: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 8303E314: 907E0004  stw r3, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 8303E318: 2B180000  cmplwi cr6, r24, 0
	ctx.cr[6].compare_u32(ctx.r[24].u32, 0 as u32, &mut ctx.xer);
	// 8303E31C: 419A0034  beq cr6, 0x8303e350
	if ctx.cr[6].eq {
	pc = 0x8303E350; continue 'dispatch;
	}
	// 8303E320: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8303E324: 38600024  li r3, 0x24
	ctx.r[3].s64 = 36;
	// 8303E328: 4BF99F71  bl 0x82fd8298
	ctx.lr = 0x8303E32C;
	sub_82FD8298(ctx, base);
	// 8303E32C: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 8303E330: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8303E334: 41820010  beq 0x8303e344
	if ctx.cr[0].eq {
	pc = 0x8303E344; continue 'dispatch;
	}
	// 8303E338: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 8303E33C: 4BF9FEB5  bl 0x82fde1f0
	ctx.lr = 0x8303E340;
	sub_82FDE1F0(ctx, base);
	// 8303E340: 48000008  b 0x8303e348
	pc = 0x8303E348; continue 'dispatch;
	// 8303E344: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 8303E348: 907E0008  stw r3, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[3].u32 ) };
	// 8303E34C: 4800003C  b 0x8303e388
	pc = 0x8303E388; continue 'dispatch;
	// 8303E350: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8303E354: 38600024  li r3, 0x24
	ctx.r[3].s64 = 36;
	// 8303E358: 4BF99F41  bl 0x82fd8298
	ctx.lr = 0x8303E35C;
	sub_82FD8298(ctx, base);
	// 8303E35C: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 8303E360: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8303E364: 4182001C  beq 0x8303e380
	if ctx.cr[0].eq {
	pc = 0x8303E380; continue 'dispatch;
	}
	// 8303E368: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 8303E36C: 80DA9770  lwz r6, -0x6890(r26)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-26768 as u32) ) } as u64;
	// 8303E370: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 8303E374: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8303E378: 4BFA08B1  bl 0x82fdec28
	ctx.lr = 0x8303E37C;
	sub_82FDEC28(ctx, base);
	// 8303E37C: 48000008  b 0x8303e384
	pc = 0x8303E384; continue 'dispatch;
	// 8303E380: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 8303E384: 907E0008  stw r3, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[3].u32 ) };
	// 8303E388: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8303E38C: 383F00A0  addi r1, r31, 0xa0
	ctx.r[1].s64 = ctx.r[31].s64 + 160;
	// 8303E390: 48169E18  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303E394(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8303E394 size=40
    let mut pc: u32 = 0x8303E394;
    'dispatch: loop {
        match pc {
            0x8303E394 => {
    //   block [0x8303E394..0x8303E3BC)
	// 8303E394: 3BECFF60  addi r31, r12, -0xa0
	ctx.r[31].s64 = ctx.r[12].s64 + -160;
	// 8303E398: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8303E39C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8303E3A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8303E3A4: 807F00B4  lwz r3, 0xb4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(180 as u32) ) } as u64;
	// 8303E3A8: 4BFFF8C1  bl 0x8303dc68
	ctx.lr = 0x8303E3AC;
	sub_8303DC68(ctx, base);
	// 8303E3AC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8303E3B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8303E3B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8303E3B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303E3BC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8303E3BC size=44
    let mut pc: u32 = 0x8303E3BC;
    'dispatch: loop {
        match pc {
            0x8303E3BC => {
    //   block [0x8303E3BC..0x8303E3E8)
	// 8303E3BC: 3BECFF60  addi r31, r12, -0xa0
	ctx.r[31].s64 = ctx.r[12].s64 + -160;
	// 8303E3C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8303E3C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8303E3C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8303E3CC: 809F00DC  lwz r4, 0xdc(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(220 as u32) ) } as u64;
	// 8303E3D0: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8303E3D4: 4BF99F0D  bl 0x82fd82e0
	ctx.lr = 0x8303E3D8;
	sub_82FD82E0(ctx, base);
	// 8303E3D8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8303E3DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8303E3E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8303E3E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303E3E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8303E3E8 size=44
    let mut pc: u32 = 0x8303E3E8;
    'dispatch: loop {
        match pc {
            0x8303E3E8 => {
    //   block [0x8303E3E8..0x8303E414)
	// 8303E3E8: 3BECFF60  addi r31, r12, -0xa0
	ctx.r[31].s64 = ctx.r[12].s64 + -160;
	// 8303E3EC: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8303E3F0: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8303E3F4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8303E3F8: 809F00DC  lwz r4, 0xdc(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(220 as u32) ) } as u64;
	// 8303E3FC: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8303E400: 4BF99EE1  bl 0x82fd82e0
	ctx.lr = 0x8303E404;
	sub_82FD82E0(ctx, base);
	// 8303E404: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8303E408: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8303E40C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8303E410: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303E414(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8303E414 size=44
    let mut pc: u32 = 0x8303E414;
    'dispatch: loop {
        match pc {
            0x8303E414 => {
    //   block [0x8303E414..0x8303E440)
	// 8303E414: 3BECFF60  addi r31, r12, -0xa0
	ctx.r[31].s64 = ctx.r[12].s64 + -160;
	// 8303E418: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8303E41C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8303E420: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8303E424: 809F00DC  lwz r4, 0xdc(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(220 as u32) ) } as u64;
	// 8303E428: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8303E42C: 4BF99EB5  bl 0x82fd82e0
	ctx.lr = 0x8303E430;
	sub_82FD82E0(ctx, base);
	// 8303E430: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8303E434: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8303E438: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8303E43C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303E440(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8303E440 size=44
    let mut pc: u32 = 0x8303E440;
    'dispatch: loop {
        match pc {
            0x8303E440 => {
    //   block [0x8303E440..0x8303E46C)
	// 8303E440: 3BECFF60  addi r31, r12, -0xa0
	ctx.r[31].s64 = ctx.r[12].s64 + -160;
	// 8303E444: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8303E448: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8303E44C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8303E450: 809F00DC  lwz r4, 0xdc(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(220 as u32) ) } as u64;
	// 8303E454: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8303E458: 4BF99E89  bl 0x82fd82e0
	ctx.lr = 0x8303E45C;
	sub_82FD82E0(ctx, base);
	// 8303E45C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8303E460: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8303E464: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8303E468: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303E470(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8303E470 size=8
    let mut pc: u32 = 0x8303E470;
    'dispatch: loop {
        match pc {
            0x8303E470 => {
    //   block [0x8303E470..0x8303E478)
	// 8303E470: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8303E474: 8215DB18  lwz r16, -0x24e8(r21)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-9448 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303E478(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8303E478 size=144
    let mut pc: u32 = 0x8303E478;
    'dispatch: loop {
        match pc {
            0x8303E478 => {
    //   block [0x8303E478..0x8303E508)
	// 8303E478: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8303E47C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8303E480: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8303E484: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8303E488: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 8303E48C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8303E490: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 8303E494: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8303E498: 396BDA60  addi r11, r11, -0x25a0
	ctx.r[11].s64 = ctx.r[11].s64 + -9632;
	// 8303E49C: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 8303E4A0: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8303E4A4: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8303E4A8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8303E4AC: 41820018  beq 0x8303e4c4
	if ctx.cr[0].eq {
	pc = 0x8303E4C4; continue 'dispatch;
	}
	// 8303E4B0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303E4B4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8303E4B8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303E4BC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8303E4C0: 4E800421  bctrl
	ctx.lr = 0x8303E4C4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8303E4C4: 807E0008  lwz r3, 8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8303E4C8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8303E4CC: 41820018  beq 0x8303e4e4
	if ctx.cr[0].eq {
	pc = 0x8303E4E4; continue 'dispatch;
	}
	// 8303E4D0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303E4D4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8303E4D8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303E4DC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8303E4E0: 4E800421  bctrl
	ctx.lr = 0x8303E4E4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8303E4E4: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 8303E4E8: 396BD9B0  addi r11, r11, -0x2650
	ctx.r[11].s64 = ctx.r[11].s64 + -9808;
	// 8303E4EC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8303E4F0: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 8303E4F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8303E4F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8303E4FC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8303E500: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8303E504: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303E508(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8303E508 size=40
    let mut pc: u32 = 0x8303E508;
    'dispatch: loop {
        match pc {
            0x8303E508 => {
    //   block [0x8303E508..0x8303E530)
	// 8303E508: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 8303E50C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8303E510: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8303E514: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8303E518: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 8303E51C: 4BFFF74D  bl 0x8303dc68
	ctx.lr = 0x8303E520;
	sub_8303DC68(ctx, base);
	// 8303E520: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8303E524: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8303E528: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8303E52C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303E530(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8303E530 size=156
    let mut pc: u32 = 0x8303E530;
    'dispatch: loop {
        match pc {
            0x8303E530 => {
    //   block [0x8303E530..0x8303E5CC)
	// 8303E530: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 8303E534: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8303E538: 394BDB48  addi r10, r11, -0x24b8
	ctx.r[10].s64 = ctx.r[11].s64 + -9400;
	// 8303E53C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8303E540: 99230006  stb r9, 6(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[9].u8 ) };
	// 8303E544: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8303E548: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 8303E54C: 99630004  stb r11, 4(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u8 ) };
	// 8303E550: 99630005  stb r11, 5(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(5 as u32), ctx.r[11].u8 ) };
	// 8303E554: 99630007  stb r11, 7(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(7 as u32), ctx.r[11].u8 ) };
	// 8303E558: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 8303E55C: 91430018  stw r10, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[10].u32 ) };
	// 8303E560: 3D408217  lis r10, -0x7de9
	ctx.r[10].s64 = -2112421888;
	// 8303E564: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 8303E568: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 8303E56C: 91630014  stw r11, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 8303E570: 814A9770  lwz r10, -0x6890(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-26768 as u32) ) } as u64;
	// 8303E574: 91630020  stw r11, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 8303E578: 91630024  stw r11, 0x24(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 8303E57C: 91630028  stw r11, 0x28(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 8303E580: 9163002C  stw r11, 0x2c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 8303E584: 9143001C  stw r10, 0x1c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[10].u32 ) };
	// 8303E588: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 8303E58C: 91630030  stw r11, 0x30(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 8303E590: 91630034  stw r11, 0x34(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 8303E594: 91630038  stw r11, 0x38(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 8303E598: 9163003C  stw r11, 0x3c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(60 as u32), ctx.r[11].u32 ) };
	// 8303E59C: 91630040  stw r11, 0x40(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(64 as u32), ctx.r[11].u32 ) };
	// 8303E5A0: 91630044  stw r11, 0x44(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(68 as u32), ctx.r[11].u32 ) };
	// 8303E5A4: 91630048  stw r11, 0x48(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(72 as u32), ctx.r[11].u32 ) };
	// 8303E5A8: 9163004C  stw r11, 0x4c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(76 as u32), ctx.r[11].u32 ) };
	// 8303E5AC: 91630050  stw r11, 0x50(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8303E5B0: 91630054  stw r11, 0x54(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8303E5B4: 91630058  stw r11, 0x58(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 8303E5B8: 9163005C  stw r11, 0x5c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8303E5BC: 91430060  stw r10, 0x60(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(96 as u32), ctx.r[10].u32 ) };
	// 8303E5C0: 91630064  stw r11, 0x64(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8303E5C4: 90830068  stw r4, 0x68(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(104 as u32), ctx.r[4].u32 ) };
	// 8303E5C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303E5D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8303E5D0 size=112
    let mut pc: u32 = 0x8303E5D0;
    'dispatch: loop {
        match pc {
            0x8303E5D0 => {
    //   block [0x8303E5D0..0x8303E640)
	// 8303E5D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8303E5D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8303E5D8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8303E5DC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8303E5E0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8303E5E4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8303E5E8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8303E5EC: 807F003C  lwz r3, 0x3c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) } as u64;
	// 8303E5F0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8303E5F4: 41820024  beq 0x8303e618
	if ctx.cr[0].eq {
	pc = 0x8303E618; continue 'dispatch;
	}
	// 8303E5F8: 897F0006  lbz r11, 6(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(6 as u32) ) } as u64;
	// 8303E5FC: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8303E600: 41820018  beq 0x8303e618
	if ctx.cr[0].eq {
	pc = 0x8303E618; continue 'dispatch;
	}
	// 8303E604: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303E608: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8303E60C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303E610: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8303E614: 4E800421  bctrl
	ctx.lr = 0x8303E618;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8303E618: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8303E61C: 93DF003C  stw r30, 0x3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[30].u32 ) };
	// 8303E620: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8303E624: 4BFFF83D  bl 0x8303de60
	ctx.lr = 0x8303E628;
	sub_8303DE60(ctx, base);
	// 8303E628: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8303E62C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8303E630: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8303E634: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8303E638: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8303E63C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303E640(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8303E640 size=88
    let mut pc: u32 = 0x8303E640;
    'dispatch: loop {
        match pc {
            0x8303E640 => {
    //   block [0x8303E640..0x8303E698)
	// 8303E640: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8303E644: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8303E648: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8303E64C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8303E650: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8303E654: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8303E658: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8303E65C: 807F0064  lwz r3, 0x64(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(100 as u32) ) } as u64;
	// 8303E660: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8303E664: 41820018  beq 0x8303e67c
	if ctx.cr[0].eq {
	pc = 0x8303E67C; continue 'dispatch;
	}
	// 8303E668: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303E66C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8303E670: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303E674: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8303E678: 4E800421  bctrl
	ctx.lr = 0x8303E67C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8303E67C: 93DF0064  stw r30, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[30].u32 ) };
	// 8303E680: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8303E684: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8303E688: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8303E68C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8303E690: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8303E694: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303E698(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8303E698 size=8
    let mut pc: u32 = 0x8303E698;
    'dispatch: loop {
        match pc {
            0x8303E698 => {
    //   block [0x8303E698..0x8303E6A0)
	// 8303E698: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8303E69C: 8215DB60  lwz r16, -0x24a0(r21)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-9376 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303E6A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8303E6A0 size=188
    let mut pc: u32 = 0x8303E6A0;
    'dispatch: loop {
        match pc {
            0x8303E6A0 => {
    //   block [0x8303E6A0..0x8303E75C)
	// 8303E6A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8303E6A4: 48169AC5  bl 0x831a8168
	ctx.lr = 0x8303E6A8;
	sub_831A8130(ctx, base);
	// 8303E6A8: 3BE1FF60  addi r31, r1, -0xa0
	ctx.r[31].s64 = ctx.r[1].s64 + -160;
	// 8303E6AC: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8303E6B0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8303E6B4: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 8303E6B8: 817E0020  lwz r11, 0x20(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 8303E6BC: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8303E6C0: 409A0010  bne cr6, 0x8303e6d0
	if !ctx.cr[6].eq {
	pc = 0x8303E6D0; continue 'dispatch;
	}
	// 8303E6C4: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 8303E6C8: 386B78D8  addi r3, r11, 0x78d8
	ctx.r[3].s64 = ctx.r[11].s64 + 30936;
	// 8303E6CC: 48000014  b 0x8303e6e0
	pc = 0x8303E6E0; continue 'dispatch;
	// 8303E6D0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8303E6D4: 409A001C  bne cr6, 0x8303e6f0
	if !ctx.cr[6].eq {
	pc = 0x8303E6F0; continue 'dispatch;
	}
	// 8303E6D8: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 8303E6DC: 386B7950  addi r3, r11, 0x7950
	ctx.r[3].s64 = ctx.r[11].s64 + 31056;
	// 8303E6E0: 809E0068  lwz r4, 0x68(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(104 as u32) ) } as u64;
	// 8303E6E4: 4BF9249D  bl 0x82fd0b80
	ctx.lr = 0x8303E6E8;
	sub_82FD0B80(ctx, base);
	// 8303E6E8: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8303E6EC: 48000064  b 0x8303e750
	pc = 0x8303E750; continue 'dispatch;
	// 8303E6F0: 839E003C  lwz r28, 0x3c(r30)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(60 as u32) ) } as u64;
	// 8303E6F4: 281C0000  cmplwi r28, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8303E6F8: 41820058  beq 0x8303e750
	if ctx.cr[0].eq {
	pc = 0x8303E750; continue 'dispatch;
	}
	// 8303E6FC: 388003FF  li r4, 0x3ff
	ctx.r[4].s64 = 1023;
	// 8303E700: 80BE0068  lwz r5, 0x68(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(104 as u32) ) } as u64;
	// 8303E704: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 8303E708: 4BFA0751  bl 0x82fdee58
	ctx.lr = 0x8303E70C;
	sub_82FDEE58(ctx, base);
	// 8303E70C: 389F0050  addi r4, r31, 0x50
	ctx.r[4].s64 = ctx.r[31].s64 + 80;
	// 8303E710: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8303E714: 48044415  bl 0x83082b28
	ctx.lr = 0x8303E718;
	sub_83082B28(ctx, base);
	// 8303E718: 817F0054  lwz r11, 0x54(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 8303E71C: 815F0068  lwz r10, 0x68(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) } as u64;
	// 8303E720: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8303E724: 7FAB532E  sthx r29, r11, r10
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[29].u16) };
	// 8303E728: 807F0068  lwz r3, 0x68(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) } as u64;
	// 8303E72C: 809E0068  lwz r4, 0x68(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(104 as u32) ) } as u64;
	// 8303E730: 4BF92451  bl 0x82fd0b80
	ctx.lr = 0x8303E734;
	sub_82FD0B80(ctx, base);
	// 8303E734: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8303E738: 807F005C  lwz r3, 0x5c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 8303E73C: 809F0068  lwz r4, 0x68(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) } as u64;
	// 8303E740: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303E744: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8303E748: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8303E74C: 4E800421  bctrl
	ctx.lr = 0x8303E750;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8303E750: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8303E754: 383F00A0  addi r1, r31, 0xa0
	ctx.r[1].s64 = ctx.r[31].s64 + 160;
	// 8303E758: 48169A60  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303E75C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8303E75C size=40
    let mut pc: u32 = 0x8303E75C;
    'dispatch: loop {
        match pc {
            0x8303E75C => {
    //   block [0x8303E75C..0x8303E784)
	// 8303E75C: 3BECFF60  addi r31, r12, -0xa0
	ctx.r[31].s64 = ctx.r[12].s64 + -160;
	// 8303E760: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8303E764: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8303E768: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8303E76C: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 8303E770: 4BFA0769  bl 0x82fdeed8
	ctx.lr = 0x8303E774;
	sub_82FDEED8(ctx, base);
	// 8303E774: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8303E778: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8303E77C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8303E780: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303E788(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8303E788 size=8
    let mut pc: u32 = 0x8303E788;
    'dispatch: loop {
        match pc {
            0x8303E788 => {
    //   block [0x8303E788..0x8303E790)
	// 8303E788: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8303E78C: 8215DC28  lwz r16, -0x23d8(r21)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-9176 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303E790(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8303E790 size=860
    let mut pc: u32 = 0x8303E790;
    'dispatch: loop {
        match pc {
            0x8303E790 => {
    //   block [0x8303E790..0x8303EAEC)
	// 8303E790: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8303E794: 481699D1  bl 0x831a8164
	ctx.lr = 0x8303E798;
	sub_831A8130(ctx, base);
	// 8303E798: 3BE1FF50  addi r31, r1, -0xb0
	ctx.r[31].s64 = ctx.r[1].s64 + -176;
	// 8303E79C: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8303E7A0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8303E7A4: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8303E7A8: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 8303E7AC: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 8303E7B0: 93DF00C4  stw r30, 0xc4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(196 as u32), ctx.r[30].u32 ) };
	// 8303E7B4: 409A0030  bne cr6, 0x8303e7e4
	if !ctx.cr[6].eq {
	pc = 0x8303E7E4; continue 'dispatch;
	}
	// 8303E7B8: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 8303E7BC: 80FE0068  lwz r7, 0x68(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(104 as u32) ) } as u64;
	// 8303E7C0: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 8303E7C4: 388BDBA0  addi r4, r11, -0x2460
	ctx.r[4].s64 = ctx.r[11].s64 + -9312;
	// 8303E7C8: 38A00282  li r5, 0x282
	ctx.r[5].s64 = 642;
	// 8303E7CC: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 8303E7D0: 4BF92889  bl 0x82fd1058
	ctx.lr = 0x8303E7D4;
	sub_82FD1058(ctx, base);
	// 8303E7D4: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 8303E7D8: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 8303E7DC: 388BC3EC  addi r4, r11, -0x3c14
	ctx.r[4].s64 = ctx.r[11].s64 + -15380;
	// 8303E7E0: 48172449  bl 0x831b0c28
	ctx.lr = 0x8303E7E4;
	sub_831B0C28(ctx, base);
	// 8303E7E4: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 8303E7E8: 839D0018  lwz r28, 0x18(r29)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(24 as u32) ) } as u64;
	// 8303E7EC: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8303E7F0: 41820044  beq 0x8303e834
	if ctx.cr[0].eq {
	pc = 0x8303E834; continue 'dispatch;
	}
	// 8303E7F4: 3D408217  lis r10, -0x7de9
	ctx.r[10].s64 = -2112421888;
	// 8303E7F8: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 8303E7FC: 814A9774  lwz r10, -0x688c(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-26764 as u32) ) } as u64;
	// 8303E800: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 8303E804: 409A0030  bne cr6, 0x8303e834
	if !ctx.cr[6].eq {
	pc = 0x8303E834; continue 'dispatch;
	}
	// 8303E808: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 8303E80C: 80FE0068  lwz r7, 0x68(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(104 as u32) ) } as u64;
	// 8303E810: 38C00012  li r6, 0x12
	ctx.r[6].s64 = 18;
	// 8303E814: 388BDBA0  addi r4, r11, -0x2460
	ctx.r[4].s64 = ctx.r[11].s64 + -9312;
	// 8303E818: 38A0028B  li r5, 0x28b
	ctx.r[5].s64 = 651;
	// 8303E81C: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 8303E820: 4BF92839  bl 0x82fd1058
	ctx.lr = 0x8303E824;
	sub_82FD1058(ctx, base);
	// 8303E824: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 8303E828: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 8303E82C: 388BC3EC  addi r4, r11, -0x3c14
	ctx.r[4].s64 = ctx.r[11].s64 + -15380;
	// 8303E830: 481723F9  bl 0x831b0c28
	ctx.lr = 0x8303E834;
	sub_831B0C28(ctx, base);
	// 8303E834: 578B073E  clrlwi r11, r28, 0x1c
	ctx.r[11].u64 = ctx.r[28].u32 as u64 & 0x0000000Fu64;
	// 8303E838: 2F0B0006  cmpwi cr6, r11, 6
	ctx.cr[6].compare_i32(ctx.r[11].s32, 6, &mut ctx.xer);
	// 8303E83C: 419A0274  beq cr6, 0x8303eab0
	if ctx.cr[6].eq {
	pc = 0x8303EAB0; continue 'dispatch;
	}
	// 8303E840: 2F0B0007  cmpwi cr6, r11, 7
	ctx.cr[6].compare_i32(ctx.r[11].s32, 7, &mut ctx.xer);
	// 8303E844: 419A026C  beq cr6, 0x8303eab0
	if ctx.cr[6].eq {
	pc = 0x8303EAB0; continue 'dispatch;
	}
	// 8303E848: 2F0B0008  cmpwi cr6, r11, 8
	ctx.cr[6].compare_i32(ctx.r[11].s32, 8, &mut ctx.xer);
	// 8303E84C: 419A0264  beq cr6, 0x8303eab0
	if ctx.cr[6].eq {
	pc = 0x8303EAB0; continue 'dispatch;
	}
	// 8303E850: 576A063F  clrlwi. r10, r27, 0x18
	ctx.r[10].u64 = ctx.r[27].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8303E854: 4182008C  beq 0x8303e8e0
	if ctx.cr[0].eq {
	pc = 0x8303E8E0; continue 'dispatch;
	}
	// 8303E858: 2F1C0009  cmpwi cr6, r28, 9
	ctx.cr[6].compare_i32(ctx.r[28].s32, 9, &mut ctx.xer);
	// 8303E85C: 409A0038  bne cr6, 0x8303e894
	if !ctx.cr[6].eq {
	pc = 0x8303E894; continue 'dispatch;
	}
	// 8303E860: 3860001C  li r3, 0x1c
	ctx.r[3].s64 = 28;
	// 8303E864: 809E0068  lwz r4, 0x68(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(104 as u32) ) } as u64;
	// 8303E868: 4BF99A31  bl 0x82fd8298
	ctx.lr = 0x8303E86C;
	sub_82FD8298(ctx, base);
	// 8303E86C: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 8303E870: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8303E874: 41820018  beq 0x8303e88c
	if ctx.cr[0].eq {
	pc = 0x8303E88C; continue 'dispatch;
	}
	// 8303E878: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8303E87C: 80DE0068  lwz r6, 0x68(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(104 as u32) ) } as u64;
	// 8303E880: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8303E884: 48048195  bl 0x83086a18
	ctx.lr = 0x8303E888;
	sub_83086A18(ctx, base);
	// 8303E888: 48000008  b 0x8303e890
	pc = 0x8303E890; continue 'dispatch;
	// 8303E88C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8303E890: 48000254  b 0x8303eae4
	pc = 0x8303EAE4; continue 'dispatch;
	// 8303E894: 2F1C0001  cmpwi cr6, r28, 1
	ctx.cr[6].compare_i32(ctx.r[28].s32, 1, &mut ctx.xer);
	// 8303E898: 409A0218  bne cr6, 0x8303eab0
	if !ctx.cr[6].eq {
	pc = 0x8303EAB0; continue 'dispatch;
	}
	// 8303E89C: 817D0010  lwz r11, 0x10(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 8303E8A0: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 8303E8A4: 2F0B0009  cmpwi cr6, r11, 9
	ctx.cr[6].compare_i32(ctx.r[11].s32, 9, &mut ctx.xer);
	// 8303E8A8: 409A0208  bne cr6, 0x8303eab0
	if !ctx.cr[6].eq {
	pc = 0x8303EAB0; continue 'dispatch;
	}
	// 8303E8AC: 3860001C  li r3, 0x1c
	ctx.r[3].s64 = 28;
	// 8303E8B0: 809E0068  lwz r4, 0x68(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(104 as u32) ) } as u64;
	// 8303E8B4: 4BF999E5  bl 0x82fd8298
	ctx.lr = 0x8303E8B8;
	sub_82FD8298(ctx, base);
	// 8303E8B8: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 8303E8BC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8303E8C0: 41820018  beq 0x8303e8d8
	if ctx.cr[0].eq {
	pc = 0x8303E8D8; continue 'dispatch;
	}
	// 8303E8C4: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8303E8C8: 809D0010  lwz r4, 0x10(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 8303E8CC: 80DE0068  lwz r6, 0x68(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(104 as u32) ) } as u64;
	// 8303E8D0: 48048149  bl 0x83086a18
	ctx.lr = 0x8303E8D4;
	sub_83086A18(ctx, base);
	// 8303E8D4: 48000008  b 0x8303e8dc
	pc = 0x8303E8DC; continue 'dispatch;
	// 8303E8D8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8303E8DC: 48000208  b 0x8303eae4
	pc = 0x8303EAE4; continue 'dispatch;
	// 8303E8E0: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 8303E8E4: 409A0040  bne cr6, 0x8303e924
	if !ctx.cr[6].eq {
	pc = 0x8303E924; continue 'dispatch;
	}
	// 8303E8E8: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 8303E8EC: 809E0068  lwz r4, 0x68(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(104 as u32) ) } as u64;
	// 8303E8F0: 4BF999A9  bl 0x82fd8298
	ctx.lr = 0x8303E8F4;
	sub_82FD8298(ctx, base);
	// 8303E8F4: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 8303E8F8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8303E8FC: 41820020  beq 0x8303e91c
	if ctx.cr[0].eq {
	pc = 0x8303E91C; continue 'dispatch;
	}
	// 8303E900: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8303E904: 80BD0008  lwz r5, 8(r29)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 8303E908: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8303E90C: 811E0068  lwz r8, 0x68(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(104 as u32) ) } as u64;
	// 8303E910: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8303E914: 4BFFF93D  bl 0x8303e250
	ctx.lr = 0x8303E918;
	sub_8303E250(ctx, base);
	// 8303E918: 48000008  b 0x8303e920
	pc = 0x8303E920; continue 'dispatch;
	// 8303E91C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8303E920: 480001C4  b 0x8303eae4
	pc = 0x8303EAE4; continue 'dispatch;
	// 8303E924: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 8303E928: 419A011C  beq cr6, 0x8303ea44
	if ctx.cr[6].eq {
	pc = 0x8303EA44; continue 'dispatch;
	}
	// 8303E92C: 2F0B0005  cmpwi cr6, r11, 5
	ctx.cr[6].compare_i32(ctx.r[11].s32, 5, &mut ctx.xer);
	// 8303E930: 419A0114  beq cr6, 0x8303ea44
	if ctx.cr[6].eq {
	pc = 0x8303EA44; continue 'dispatch;
	}
	// 8303E934: 2F1C0003  cmpwi cr6, r28, 3
	ctx.cr[6].compare_i32(ctx.r[28].s32, 3, &mut ctx.xer);
	// 8303E938: 419A007C  beq cr6, 0x8303e9b4
	if ctx.cr[6].eq {
	pc = 0x8303E9B4; continue 'dispatch;
	}
	// 8303E93C: 2F1C0002  cmpwi cr6, r28, 2
	ctx.cr[6].compare_i32(ctx.r[28].s32, 2, &mut ctx.xer);
	// 8303E940: 419A0074  beq cr6, 0x8303e9b4
	if ctx.cr[6].eq {
	pc = 0x8303E9B4; continue 'dispatch;
	}
	// 8303E944: 2F1C0001  cmpwi cr6, r28, 1
	ctx.cr[6].compare_i32(ctx.r[28].s32, 1, &mut ctx.xer);
	// 8303E948: 419A006C  beq cr6, 0x8303e9b4
	if ctx.cr[6].eq {
	pc = 0x8303E9B4; continue 'dispatch;
	}
	// 8303E94C: 2F1C0009  cmpwi cr6, r28, 9
	ctx.cr[6].compare_i32(ctx.r[28].s32, 9, &mut ctx.xer);
	// 8303E950: 409A0038  bne cr6, 0x8303e988
	if !ctx.cr[6].eq {
	pc = 0x8303E988; continue 'dispatch;
	}
	// 8303E954: 3860001C  li r3, 0x1c
	ctx.r[3].s64 = 28;
	// 8303E958: 809E0068  lwz r4, 0x68(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(104 as u32) ) } as u64;
	// 8303E95C: 4BF9993D  bl 0x82fd8298
	ctx.lr = 0x8303E960;
	sub_82FD8298(ctx, base);
	// 8303E960: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 8303E964: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8303E968: 41820018  beq 0x8303e980
	if ctx.cr[0].eq {
	pc = 0x8303E980; continue 'dispatch;
	}
	// 8303E96C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8303E970: 80DE0068  lwz r6, 0x68(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(104 as u32) ) } as u64;
	// 8303E974: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8303E978: 480480A1  bl 0x83086a18
	ctx.lr = 0x8303E97C;
	sub_83086A18(ctx, base);
	// 8303E97C: 48000008  b 0x8303e984
	pc = 0x8303E984; continue 'dispatch;
	// 8303E980: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8303E984: 48000160  b 0x8303eae4
	pc = 0x8303EAE4; continue 'dispatch;
	// 8303E988: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 8303E98C: 80FE0068  lwz r7, 0x68(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(104 as u32) ) } as u64;
	// 8303E990: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 8303E994: 388BDBA0  addi r4, r11, -0x2460
	ctx.r[4].s64 = ctx.r[11].s64 + -9312;
	// 8303E998: 38A002E6  li r5, 0x2e6
	ctx.r[5].s64 = 742;
	// 8303E99C: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 8303E9A0: 4BF926B9  bl 0x82fd1058
	ctx.lr = 0x8303E9A4;
	sub_82FD1058(ctx, base);
	// 8303E9A4: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 8303E9A8: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 8303E9AC: 388BC3EC  addi r4, r11, -0x3c14
	ctx.r[4].s64 = ctx.r[11].s64 + -15380;
	// 8303E9B0: 48172279  bl 0x831b0c28
	ctx.lr = 0x8303E9B4;
	sub_831B0C28(ctx, base);
	// 8303E9B4: 817D0010  lwz r11, 0x10(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 8303E9B8: 814B0018  lwz r10, 0x18(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 8303E9BC: 2C0A0000  cmpwi r10, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8303E9C0: 40820044  bne 0x8303ea04
	if !ctx.cr[0].eq {
	pc = 0x8303EA04; continue 'dispatch;
	}
	// 8303E9C4: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 8303E9C8: 809E0068  lwz r4, 0x68(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(104 as u32) ) } as u64;
	// 8303E9CC: 4BF998CD  bl 0x82fd8298
	ctx.lr = 0x8303E9D0;
	sub_82FD8298(ctx, base);
	// 8303E9D0: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 8303E9D4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8303E9D8: 41820024  beq 0x8303e9fc
	if ctx.cr[0].eq {
	pc = 0x8303E9FC; continue 'dispatch;
	}
	// 8303E9DC: 817D0010  lwz r11, 0x10(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 8303E9E0: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 8303E9E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8303E9E8: 811E0068  lwz r8, 0x68(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(104 as u32) ) } as u64;
	// 8303E9EC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8303E9F0: 80AB0008  lwz r5, 8(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8303E9F4: 4BFFF85D  bl 0x8303e250
	ctx.lr = 0x8303E9F8;
	sub_8303E250(ctx, base);
	// 8303E9F8: 48000008  b 0x8303ea00
	pc = 0x8303EA00; continue 'dispatch;
	// 8303E9FC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8303EA00: 480000E4  b 0x8303eae4
	pc = 0x8303EAE4; continue 'dispatch;
	// 8303EA04: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 8303EA08: 2F0B0009  cmpwi cr6, r11, 9
	ctx.cr[6].compare_i32(ctx.r[11].s32, 9, &mut ctx.xer);
	// 8303EA0C: 409A00A4  bne cr6, 0x8303eab0
	if !ctx.cr[6].eq {
	pc = 0x8303EAB0; continue 'dispatch;
	}
	// 8303EA10: 3860001C  li r3, 0x1c
	ctx.r[3].s64 = 28;
	// 8303EA14: 809E0068  lwz r4, 0x68(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(104 as u32) ) } as u64;
	// 8303EA18: 4BF99881  bl 0x82fd8298
	ctx.lr = 0x8303EA1C;
	sub_82FD8298(ctx, base);
	// 8303EA1C: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 8303EA20: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8303EA24: 41820018  beq 0x8303ea3c
	if ctx.cr[0].eq {
	pc = 0x8303EA3C; continue 'dispatch;
	}
	// 8303EA28: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8303EA2C: 809D0010  lwz r4, 0x10(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 8303EA30: 80DE0068  lwz r6, 0x68(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(104 as u32) ) } as u64;
	// 8303EA34: 48047FE5  bl 0x83086a18
	ctx.lr = 0x8303EA38;
	sub_83086A18(ctx, base);
	// 8303EA38: 48000008  b 0x8303ea40
	pc = 0x8303EA40; continue 'dispatch;
	// 8303EA3C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8303EA40: 480000A4  b 0x8303eae4
	pc = 0x8303EAE4; continue 'dispatch;
	// 8303EA44: 817D0010  lwz r11, 0x10(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 8303EA48: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 8303EA4C: 2C0B0000  cmpwi r11, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8303EA50: 40820060  bne 0x8303eab0
	if !ctx.cr[0].eq {
	pc = 0x8303EAB0; continue 'dispatch;
	}
	// 8303EA54: 817D0014  lwz r11, 0x14(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(20 as u32) ) } as u64;
	// 8303EA58: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8303EA5C: 41820054  beq 0x8303eab0
	if ctx.cr[0].eq {
	pc = 0x8303EAB0; continue 'dispatch;
	}
	// 8303EA60: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 8303EA64: 2C0B0000  cmpwi r11, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8303EA68: 40820048  bne 0x8303eab0
	if !ctx.cr[0].eq {
	pc = 0x8303EAB0; continue 'dispatch;
	}
	// 8303EA6C: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 8303EA70: 809E0068  lwz r4, 0x68(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(104 as u32) ) } as u64;
	// 8303EA74: 4BF99825  bl 0x82fd8298
	ctx.lr = 0x8303EA78;
	sub_82FD8298(ctx, base);
	// 8303EA78: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 8303EA7C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8303EA80: 41820028  beq 0x8303eaa8
	if ctx.cr[0].eq {
	pc = 0x8303EAA8; continue 'dispatch;
	}
	// 8303EA84: 817D0014  lwz r11, 0x14(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(20 as u32) ) } as u64;
	// 8303EA88: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 8303EA8C: 815D0010  lwz r10, 0x10(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 8303EA90: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8303EA94: 811E0068  lwz r8, 0x68(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(104 as u32) ) } as u64;
	// 8303EA98: 80CB0008  lwz r6, 8(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8303EA9C: 80AA0008  lwz r5, 8(r10)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 8303EAA0: 4BFFF7B1  bl 0x8303e250
	ctx.lr = 0x8303EAA4;
	sub_8303E250(ctx, base);
	// 8303EAA4: 48000008  b 0x8303eaac
	pc = 0x8303EAAC; continue 'dispatch;
	// 8303EAA8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8303EAAC: 48000038  b 0x8303eae4
	pc = 0x8303EAE4; continue 'dispatch;
	// 8303EAB0: 38600044  li r3, 0x44
	ctx.r[3].s64 = 68;
	// 8303EAB4: 809E0068  lwz r4, 0x68(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(104 as u32) ) } as u64;
	// 8303EAB8: 4BF997E1  bl 0x82fd8298
	ctx.lr = 0x8303EABC;
	sub_82FD8298(ctx, base);
	// 8303EABC: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 8303EAC0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8303EAC4: 4182001C  beq 0x8303eae0
	if ctx.cr[0].eq {
	pc = 0x8303EAE0; continue 'dispatch;
	}
	// 8303EAC8: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 8303EACC: 80FE0068  lwz r7, 0x68(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(104 as u32) ) } as u64;
	// 8303EAD0: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8303EAD4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8303EAD8: 48047791  bl 0x83086268
	ctx.lr = 0x8303EADC;
	sub_83086268(ctx, base);
	// 8303EADC: 48000008  b 0x8303eae4
	pc = 0x8303EAE4; continue 'dispatch;
	// 8303EAE0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8303EAE4: 383F00B0  addi r1, r31, 0xb0
	ctx.r[1].s64 = ctx.r[31].s64 + 176;
	// 8303EAE8: 481696CC  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303EAEC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8303EAEC size=48
    let mut pc: u32 = 0x8303EAEC;
    'dispatch: loop {
        match pc {
            0x8303EAEC => {
    //   block [0x8303EAEC..0x8303EB1C)
	// 8303EAEC: 3BECFF50  addi r31, r12, -0xb0
	ctx.r[31].s64 = ctx.r[12].s64 + -176;
	// 8303EAF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8303EAF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8303EAF8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8303EAFC: 817F00C4  lwz r11, 0xc4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(196 as u32) ) } as u64;
	// 8303EB00: 808B0068  lwz r4, 0x68(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(104 as u32) ) } as u64;
	// 8303EB04: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8303EB08: 4BF997D9  bl 0x82fd82e0
	ctx.lr = 0x8303EB0C;
	sub_82FD82E0(ctx, base);
	// 8303EB0C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8303EB10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8303EB14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8303EB18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303EB1C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8303EB1C size=48
    let mut pc: u32 = 0x8303EB1C;
    'dispatch: loop {
        match pc {
            0x8303EB1C => {
    //   block [0x8303EB1C..0x8303EB4C)
	// 8303EB1C: 3BECFF50  addi r31, r12, -0xb0
	ctx.r[31].s64 = ctx.r[12].s64 + -176;
	// 8303EB20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8303EB24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8303EB28: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8303EB2C: 817F00C4  lwz r11, 0xc4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(196 as u32) ) } as u64;
	// 8303EB30: 808B0068  lwz r4, 0x68(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(104 as u32) ) } as u64;
	// 8303EB34: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8303EB38: 4BF997A9  bl 0x82fd82e0
	ctx.lr = 0x8303EB3C;
	sub_82FD82E0(ctx, base);
	// 8303EB3C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8303EB40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8303EB44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8303EB48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303EB4C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8303EB4C size=48
    let mut pc: u32 = 0x8303EB4C;
    'dispatch: loop {
        match pc {
            0x8303EB4C => {
    //   block [0x8303EB4C..0x8303EB7C)
	// 8303EB4C: 3BECFF50  addi r31, r12, -0xb0
	ctx.r[31].s64 = ctx.r[12].s64 + -176;
	// 8303EB50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8303EB54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8303EB58: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8303EB5C: 817F00C4  lwz r11, 0xc4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(196 as u32) ) } as u64;
	// 8303EB60: 808B0068  lwz r4, 0x68(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(104 as u32) ) } as u64;
	// 8303EB64: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8303EB68: 4BF99779  bl 0x82fd82e0
	ctx.lr = 0x8303EB6C;
	sub_82FD82E0(ctx, base);
	// 8303EB6C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8303EB70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8303EB74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8303EB78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303EB7C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8303EB7C size=48
    let mut pc: u32 = 0x8303EB7C;
    'dispatch: loop {
        match pc {
            0x8303EB7C => {
    //   block [0x8303EB7C..0x8303EBAC)
	// 8303EB7C: 3BECFF50  addi r31, r12, -0xb0
	ctx.r[31].s64 = ctx.r[12].s64 + -176;
	// 8303EB80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8303EB84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8303EB88: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8303EB8C: 817F00C4  lwz r11, 0xc4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(196 as u32) ) } as u64;
	// 8303EB90: 808B0068  lwz r4, 0x68(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(104 as u32) ) } as u64;
	// 8303EB94: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8303EB98: 4BF99749  bl 0x82fd82e0
	ctx.lr = 0x8303EB9C;
	sub_82FD82E0(ctx, base);
	// 8303EB9C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8303EBA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8303EBA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8303EBA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303EBAC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8303EBAC size=48
    let mut pc: u32 = 0x8303EBAC;
    'dispatch: loop {
        match pc {
            0x8303EBAC => {
    //   block [0x8303EBAC..0x8303EBDC)
	// 8303EBAC: 3BECFF50  addi r31, r12, -0xb0
	ctx.r[31].s64 = ctx.r[12].s64 + -176;
	// 8303EBB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8303EBB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8303EBB8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8303EBBC: 817F00C4  lwz r11, 0xc4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(196 as u32) ) } as u64;
	// 8303EBC0: 808B0068  lwz r4, 0x68(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(104 as u32) ) } as u64;
	// 8303EBC4: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8303EBC8: 4BF99719  bl 0x82fd82e0
	ctx.lr = 0x8303EBCC;
	sub_82FD82E0(ctx, base);
	// 8303EBCC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8303EBD0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8303EBD4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8303EBD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303EBDC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8303EBDC size=48
    let mut pc: u32 = 0x8303EBDC;
    'dispatch: loop {
        match pc {
            0x8303EBDC => {
    //   block [0x8303EBDC..0x8303EC0C)
	// 8303EBDC: 3BECFF50  addi r31, r12, -0xb0
	ctx.r[31].s64 = ctx.r[12].s64 + -176;
	// 8303EBE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8303EBE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8303EBE8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8303EBEC: 817F00C4  lwz r11, 0xc4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(196 as u32) ) } as u64;
	// 8303EBF0: 808B0068  lwz r4, 0x68(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(104 as u32) ) } as u64;
	// 8303EBF4: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8303EBF8: 4BF996E9  bl 0x82fd82e0
	ctx.lr = 0x8303EBFC;
	sub_82FD82E0(ctx, base);
	// 8303EBFC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8303EC00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8303EC04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8303EC08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303EC0C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8303EC0C size=48
    let mut pc: u32 = 0x8303EC0C;
    'dispatch: loop {
        match pc {
            0x8303EC0C => {
    //   block [0x8303EC0C..0x8303EC3C)
	// 8303EC0C: 3BECFF50  addi r31, r12, -0xb0
	ctx.r[31].s64 = ctx.r[12].s64 + -176;
	// 8303EC10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8303EC14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8303EC18: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8303EC1C: 817F00C4  lwz r11, 0xc4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(196 as u32) ) } as u64;
	// 8303EC20: 808B0068  lwz r4, 0x68(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(104 as u32) ) } as u64;
	// 8303EC24: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8303EC28: 4BF996B9  bl 0x82fd82e0
	ctx.lr = 0x8303EC2C;
	sub_82FD82E0(ctx, base);
	// 8303EC2C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8303EC30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8303EC34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8303EC38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303EC3C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8303EC3C size=48
    let mut pc: u32 = 0x8303EC3C;
    'dispatch: loop {
        match pc {
            0x8303EC3C => {
    //   block [0x8303EC3C..0x8303EC6C)
	// 8303EC3C: 3BECFF50  addi r31, r12, -0xb0
	ctx.r[31].s64 = ctx.r[12].s64 + -176;
	// 8303EC40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8303EC44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8303EC48: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8303EC4C: 817F00C4  lwz r11, 0xc4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(196 as u32) ) } as u64;
	// 8303EC50: 808B0068  lwz r4, 0x68(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(104 as u32) ) } as u64;
	// 8303EC54: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8303EC58: 4BF99689  bl 0x82fd82e0
	ctx.lr = 0x8303EC5C;
	sub_82FD82E0(ctx, base);
	// 8303EC5C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8303EC60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8303EC64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8303EC68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303EC70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8303EC70 size=1204
    let mut pc: u32 = 0x8303EC70;
    'dispatch: loop {
        match pc {
            0x8303EC70 => {
    //   block [0x8303EC70..0x8303F124)
	// 8303EC70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8303EC74: 481694DD  bl 0x831a8150
	ctx.lr = 0x8303EC78;
	sub_831A8130(ctx, base);
	// 8303EC78: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8303EC7C: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 8303EC80: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8303EC84: 7CB72B78  mr r23, r5
	ctx.r[23].u64 = ctx.r[5].u64;
	// 8303EC88: 7CD63378  mr r22, r6
	ctx.r[22].u64 = ctx.r[6].u64;
	// 8303EC8C: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 8303EC90: 409A000C  bne cr6, 0x8303ec9c
	if !ctx.cr[6].eq {
	pc = 0x8303EC9C; continue 'dispatch;
	}
	// 8303EC94: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8303EC98: 48000484  b 0x8303f11c
	pc = 0x8303F11C; continue 'dispatch;
	// 8303EC9C: 7F7CDB78  mr r28, r27
	ctx.r[28].u64 = ctx.r[27].u64;
	// 8303ECA0: 2F170001  cmpwi cr6, r23, 1
	ctx.cr[6].compare_i32(ctx.r[23].s32, 1, &mut ctx.xer);
	// 8303ECA4: 409A000C  bne cr6, 0x8303ecb0
	if !ctx.cr[6].eq {
	pc = 0x8303ECB0; continue 'dispatch;
	}
	// 8303ECA8: 2F160001  cmpwi cr6, r22, 1
	ctx.cr[6].compare_i32(ctx.r[22].s32, 1, &mut ctx.xer);
	// 8303ECAC: 419A046C  beq cr6, 0x8303f118
	if ctx.cr[6].eq {
	pc = 0x8303F118; continue 'dispatch;
	}
	// 8303ECB0: 2F170000  cmpwi cr6, r23, 0
	ctx.cr[6].compare_i32(ctx.r[23].s32, 0, &mut ctx.xer);
	// 8303ECB4: 409A00B0  bne cr6, 0x8303ed64
	if !ctx.cr[6].eq {
	pc = 0x8303ED64; continue 'dispatch;
	}
	// 8303ECB8: 809D0068  lwz r4, 0x68(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(104 as u32) ) } as u64;
	// 8303ECBC: 2F160001  cmpwi cr6, r22, 1
	ctx.cr[6].compare_i32(ctx.r[22].s32, 1, &mut ctx.xer);
	// 8303ECC0: 38600028  li r3, 0x28
	ctx.r[3].s64 = 40;
	// 8303ECC4: 409A0058  bne cr6, 0x8303ed1c
	if !ctx.cr[6].eq {
	pc = 0x8303ED1C; continue 'dispatch;
	}
	// 8303ECC8: 4BF995D1  bl 0x82fd8298
	ctx.lr = 0x8303ECCC;
	sub_82FD8298(ctx, base);
	// 8303ECCC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8303ECD0: 418200D4  beq 0x8303eda4
	if ctx.cr[0].eq {
	pc = 0x8303EDA4; continue 'dispatch;
	}
	// 8303ECD4: 3D408216  lis r10, -0x7dea
	ctx.r[10].s64 = -2112487424;
	// 8303ECD8: 817D0068  lwz r11, 0x68(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(104 as u32) ) } as u64;
	// 8303ECDC: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8303ECE0: 93630010  stw r27, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[27].u32 ) };
	// 8303ECE4: 394AD9C8  addi r10, r10, -0x2638
	ctx.r[10].s64 = ctx.r[10].s64 + -9784;
	// 8303ECE8: 3BE00001  li r31, 1
	ctx.r[31].s64 = 1;
	// 8303ECEC: 93C30008  stw r30, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 8303ECF0: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8303ECF4: 93C3000C  stw r30, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[30].u32 ) };
	// 8303ECF8: 93C30014  stw r30, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[30].u32 ) };
	// 8303ECFC: 93E30018  stw r31, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[31].u32 ) };
	// 8303ED00: 9BE3001C  stb r31, 0x1c(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[31].u8 ) };
	// 8303ED04: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 8303ED08: 9BE3001D  stb r31, 0x1d(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(29 as u32), ctx.r[31].u8 ) };
	// 8303ED0C: 93E30020  stw r31, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[31].u32 ) };
	// 8303ED10: 93E30024  stw r31, 0x24(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), ctx.r[31].u32 ) };
	// 8303ED14: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8303ED18: 48000400  b 0x8303f118
	pc = 0x8303F118; continue 'dispatch;
	// 8303ED1C: 2F16FFFF  cmpwi cr6, r22, -1
	ctx.cr[6].compare_i32(ctx.r[22].s32, -1, &mut ctx.xer);
	// 8303ED20: 409A0164  bne cr6, 0x8303ee84
	if !ctx.cr[6].eq {
	pc = 0x8303EE84; continue 'dispatch;
	}
	// 8303ED24: 4BF99575  bl 0x82fd8298
	ctx.lr = 0x8303ED28;
	sub_82FD8298(ctx, base);
	// 8303ED28: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8303ED2C: 41820078  beq 0x8303eda4
	if ctx.cr[0].eq {
	pc = 0x8303EDA4; continue 'dispatch;
	}
	// 8303ED30: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8303ED34: 3D408216  lis r10, -0x7dea
	ctx.r[10].s64 = -2112487424;
	// 8303ED38: 817D0068  lwz r11, 0x68(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(104 as u32) ) } as u64;
	// 8303ED3C: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8303ED40: 394AD9C8  addi r10, r10, -0x2638
	ctx.r[10].s64 = ctx.r[10].s64 + -9784;
	// 8303ED44: 3BE00001  li r31, 1
	ctx.r[31].s64 = 1;
	// 8303ED48: 93C30008  stw r30, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 8303ED4C: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8303ED50: 93C3000C  stw r30, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[30].u32 ) };
	// 8303ED54: 93630010  stw r27, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[27].u32 ) };
	// 8303ED58: 93C30014  stw r30, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[30].u32 ) };
	// 8303ED5C: 91230018  stw r9, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[9].u32 ) };
	// 8303ED60: 4BFFFFA0  b 0x8303ed00
	pc = 0x8303ED00; continue 'dispatch;
	// 8303ED64: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 8303ED68: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8303ED6C: 3BE00001  li r31, 1
	ctx.r[31].s64 = 1;
	// 8303ED70: 3B000005  li r24, 5
	ctx.r[24].s64 = 5;
	// 8303ED74: 2F170001  cmpwi cr6, r23, 1
	ctx.cr[6].compare_i32(ctx.r[23].s32, 1, &mut ctx.xer);
	// 8303ED78: 3B4BD9C8  addi r26, r11, -0x2638
	ctx.r[26].s64 = ctx.r[11].s64 + -9784;
	// 8303ED7C: 409A0030  bne cr6, 0x8303edac
	if !ctx.cr[6].eq {
	pc = 0x8303EDAC; continue 'dispatch;
	}
	// 8303ED80: 2F16FFFF  cmpwi cr6, r22, -1
	ctx.cr[6].compare_i32(ctx.r[22].s32, -1, &mut ctx.xer);
	// 8303ED84: 409A0288  bne cr6, 0x8303f00c
	if !ctx.cr[6].eq {
	pc = 0x8303F00C; continue 'dispatch;
	}
	// 8303ED88: 38600028  li r3, 0x28
	ctx.r[3].s64 = 40;
	// 8303ED8C: 809D0068  lwz r4, 0x68(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(104 as u32) ) } as u64;
	// 8303ED90: 4BF99509  bl 0x82fd8298
	ctx.lr = 0x8303ED94;
	sub_82FD8298(ctx, base);
	// 8303ED94: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8303ED98: 4182000C  beq 0x8303eda4
	if ctx.cr[0].eq {
	pc = 0x8303EDA4; continue 'dispatch;
	}
	// 8303ED9C: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8303EDA0: 4BFFFF94  b 0x8303ed34
	pc = 0x8303ED34; continue 'dispatch;
	// 8303EDA4: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 8303EDA8: 48000370  b 0x8303f118
	pc = 0x8303F118; continue 'dispatch;
	// 8303EDAC: 2F16FFFF  cmpwi cr6, r22, -1
	ctx.cr[6].compare_i32(ctx.r[22].s32, -1, &mut ctx.xer);
	// 8303EDB0: 409A019C  bne cr6, 0x8303ef4c
	if !ctx.cr[6].eq {
	pc = 0x8303EF4C; continue 'dispatch;
	}
	// 8303EDB4: 38600028  li r3, 0x28
	ctx.r[3].s64 = 40;
	// 8303EDB8: 809D0068  lwz r4, 0x68(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(104 as u32) ) } as u64;
	// 8303EDBC: 4BF994DD  bl 0x82fd8298
	ctx.lr = 0x8303EDC0;
	sub_82FD8298(ctx, base);
	// 8303EDC0: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 8303EDC4: 3BE00001  li r31, 1
	ctx.r[31].s64 = 1;
	// 8303EDC8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8303EDCC: 3B4BD9C8  addi r26, r11, -0x2638
	ctx.r[26].s64 = ctx.r[11].s64 + -9784;
	// 8303EDD0: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8303EDD4: 41820040  beq 0x8303ee14
	if ctx.cr[0].eq {
	pc = 0x8303EE14; continue 'dispatch;
	}
	// 8303EDD8: 817D0068  lwz r11, 0x68(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(104 as u32) ) } as u64;
	// 8303EDDC: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 8303EDE0: 93C30008  stw r30, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 8303EDE4: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 8303EDE8: 93430000  stw r26, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[26].u32 ) };
	// 8303EDEC: 93C3000C  stw r30, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[30].u32 ) };
	// 8303EDF0: 93630010  stw r27, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[27].u32 ) };
	// 8303EDF4: 93C30014  stw r30, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[30].u32 ) };
	// 8303EDF8: 91430018  stw r10, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[10].u32 ) };
	// 8303EDFC: 9BE3001C  stb r31, 0x1c(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[31].u8 ) };
	// 8303EE00: 9BE3001D  stb r31, 0x1d(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(29 as u32), ctx.r[31].u8 ) };
	// 8303EE04: 93E30020  stw r31, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[31].u32 ) };
	// 8303EE08: 93E30024  stw r31, 0x24(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), ctx.r[31].u32 ) };
	// 8303EE0C: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8303EE10: 48000008  b 0x8303ee18
	pc = 0x8303EE18; continue 'dispatch;
	// 8303EE14: 7FDCF378  mr r28, r30
	ctx.r[28].u64 = ctx.r[30].u64;
	// 8303EE18: 3577FFFF  addic. r11, r23, -1
	ctx.xer.ca = (ctx.r[23].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[23].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8303EE1C: 408102FC  ble 0x8303f118
	if !ctx.cr[0].gt {
	pc = 0x8303F118; continue 'dispatch;
	}
	// 8303EE20: 7D795B78  mr r25, r11
	ctx.r[25].u64 = ctx.r[11].u64;
	// 8303EE24: 3B000005  li r24, 5
	ctx.r[24].s64 = 5;
	// 8303EE28: 38600028  li r3, 0x28
	ctx.r[3].s64 = 40;
	// 8303EE2C: 809D0068  lwz r4, 0x68(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(104 as u32) ) } as u64;
	// 8303EE30: 4BF99469  bl 0x82fd8298
	ctx.lr = 0x8303EE34;
	sub_82FD8298(ctx, base);
	// 8303EE34: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8303EE38: 4182003C  beq 0x8303ee74
	if ctx.cr[0].eq {
	pc = 0x8303EE74; continue 'dispatch;
	}
	// 8303EE3C: 817D0068  lwz r11, 0x68(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(104 as u32) ) } as u64;
	// 8303EE40: 93830014  stw r28, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[28].u32 ) };
	// 8303EE44: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 8303EE48: 93C30008  stw r30, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 8303EE4C: 93430000  stw r26, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[26].u32 ) };
	// 8303EE50: 93C3000C  stw r30, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[30].u32 ) };
	// 8303EE54: 93630010  stw r27, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[27].u32 ) };
	// 8303EE58: 93030018  stw r24, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[24].u32 ) };
	// 8303EE5C: 9BC3001C  stb r30, 0x1c(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[30].u8 ) };
	// 8303EE60: 9BE3001D  stb r31, 0x1d(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(29 as u32), ctx.r[31].u8 ) };
	// 8303EE64: 93E30020  stw r31, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[31].u32 ) };
	// 8303EE68: 93E30024  stw r31, 0x24(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), ctx.r[31].u32 ) };
	// 8303EE6C: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8303EE70: 48000008  b 0x8303ee78
	pc = 0x8303EE78; continue 'dispatch;
	// 8303EE74: 7FDCF378  mr r28, r30
	ctx.r[28].u64 = ctx.r[30].u64;
	// 8303EE78: 3739FFFF  addic. r25, r25, -1
	ctx.xer.ca = (ctx.r[25].u32 > (!(-1 as u32)));
	ctx.r[25].s64 = ctx.r[25].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[25].s32, 0, &mut ctx.xer);
	// 8303EE7C: 4082FFAC  bne 0x8303ee28
	if !ctx.cr[0].eq {
	pc = 0x8303EE28; continue 'dispatch;
	}
	// 8303EE80: 48000298  b 0x8303f118
	pc = 0x8303F118; continue 'dispatch;
	// 8303EE84: 4BF99415  bl 0x82fd8298
	ctx.lr = 0x8303EE88;
	sub_82FD8298(ctx, base);
	// 8303EE88: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 8303EE8C: 3BE00001  li r31, 1
	ctx.r[31].s64 = 1;
	// 8303EE90: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8303EE94: 3B4BD9C8  addi r26, r11, -0x2638
	ctx.r[26].s64 = ctx.r[11].s64 + -9784;
	// 8303EE98: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8303EE9C: 4182003C  beq 0x8303eed8
	if ctx.cr[0].eq {
	pc = 0x8303EED8; continue 'dispatch;
	}
	// 8303EEA0: 817D0068  lwz r11, 0x68(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(104 as u32) ) } as u64;
	// 8303EEA4: 93630010  stw r27, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[27].u32 ) };
	// 8303EEA8: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 8303EEAC: 93C30008  stw r30, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 8303EEB0: 93430000  stw r26, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[26].u32 ) };
	// 8303EEB4: 93C3000C  stw r30, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[30].u32 ) };
	// 8303EEB8: 93C30014  stw r30, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[30].u32 ) };
	// 8303EEBC: 93E30018  stw r31, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[31].u32 ) };
	// 8303EEC0: 9BE3001C  stb r31, 0x1c(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[31].u8 ) };
	// 8303EEC4: 9BE3001D  stb r31, 0x1d(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(29 as u32), ctx.r[31].u8 ) };
	// 8303EEC8: 93E30020  stw r31, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[31].u32 ) };
	// 8303EECC: 93E30024  stw r31, 0x24(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), ctx.r[31].u32 ) };
	// 8303EED0: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8303EED4: 48000008  b 0x8303eedc
	pc = 0x8303EEDC; continue 'dispatch;
	// 8303EED8: 7FDBF378  mr r27, r30
	ctx.r[27].u64 = ctx.r[30].u64;
	// 8303EEDC: 3576FFFF  addic. r11, r22, -1
	ctx.xer.ca = (ctx.r[22].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[22].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8303EEE0: 7F7CDB78  mr r28, r27
	ctx.r[28].u64 = ctx.r[27].u64;
	// 8303EEE4: 40810234  ble 0x8303f118
	if !ctx.cr[0].gt {
	pc = 0x8303F118; continue 'dispatch;
	}
	// 8303EEE8: 7D795B78  mr r25, r11
	ctx.r[25].u64 = ctx.r[11].u64;
	// 8303EEEC: 3B000005  li r24, 5
	ctx.r[24].s64 = 5;
	// 8303EEF0: 38600028  li r3, 0x28
	ctx.r[3].s64 = 40;
	// 8303EEF4: 809D0068  lwz r4, 0x68(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(104 as u32) ) } as u64;
	// 8303EEF8: 4BF993A1  bl 0x82fd8298
	ctx.lr = 0x8303EEFC;
	sub_82FD8298(ctx, base);
	// 8303EEFC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8303EF00: 4182003C  beq 0x8303ef3c
	if ctx.cr[0].eq {
	pc = 0x8303EF3C; continue 'dispatch;
	}
	// 8303EF04: 817D0068  lwz r11, 0x68(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(104 as u32) ) } as u64;
	// 8303EF08: 93830010  stw r28, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[28].u32 ) };
	// 8303EF0C: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 8303EF10: 93C30008  stw r30, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 8303EF14: 93430000  stw r26, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[26].u32 ) };
	// 8303EF18: 93C3000C  stw r30, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[30].u32 ) };
	// 8303EF1C: 93630014  stw r27, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[27].u32 ) };
	// 8303EF20: 93030018  stw r24, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[24].u32 ) };
	// 8303EF24: 9BE3001C  stb r31, 0x1c(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[31].u8 ) };
	// 8303EF28: 9BC3001D  stb r30, 0x1d(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(29 as u32), ctx.r[30].u8 ) };
	// 8303EF2C: 93E30020  stw r31, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[31].u32 ) };
	// 8303EF30: 93E30024  stw r31, 0x24(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), ctx.r[31].u32 ) };
	// 8303EF34: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8303EF38: 48000008  b 0x8303ef40
	pc = 0x8303EF40; continue 'dispatch;
	// 8303EF3C: 7FDCF378  mr r28, r30
	ctx.r[28].u64 = ctx.r[30].u64;
	// 8303EF40: 3739FFFF  addic. r25, r25, -1
	ctx.xer.ca = (ctx.r[25].u32 > (!(-1 as u32)));
	ctx.r[25].s64 = ctx.r[25].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[25].s32, 0, &mut ctx.xer);
	// 8303EF44: 4082FFAC  bne 0x8303eef0
	if !ctx.cr[0].eq {
	pc = 0x8303EEF0; continue 'dispatch;
	}
	// 8303EF48: 480001D0  b 0x8303f118
	pc = 0x8303F118; continue 'dispatch;
	// 8303EF4C: 2F170001  cmpwi cr6, r23, 1
	ctx.cr[6].compare_i32(ctx.r[23].s32, 1, &mut ctx.xer);
	// 8303EF50: 409900BC  ble cr6, 0x8303f00c
	if !ctx.cr[6].gt {
	pc = 0x8303F00C; continue 'dispatch;
	}
	// 8303EF54: 38600028  li r3, 0x28
	ctx.r[3].s64 = 40;
	// 8303EF58: 809D0068  lwz r4, 0x68(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(104 as u32) ) } as u64;
	// 8303EF5C: 4BF9933D  bl 0x82fd8298
	ctx.lr = 0x8303EF60;
	sub_82FD8298(ctx, base);
	// 8303EF60: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8303EF64: 4182003C  beq 0x8303efa0
	if ctx.cr[0].eq {
	pc = 0x8303EFA0; continue 'dispatch;
	}
	// 8303EF68: 817D0068  lwz r11, 0x68(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(104 as u32) ) } as u64;
	// 8303EF6C: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 8303EF70: 93C30008  stw r30, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 8303EF74: 93430000  stw r26, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[26].u32 ) };
	// 8303EF78: 93C3000C  stw r30, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[30].u32 ) };
	// 8303EF7C: 93630010  stw r27, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[27].u32 ) };
	// 8303EF80: 93630014  stw r27, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[27].u32 ) };
	// 8303EF84: 93030018  stw r24, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[24].u32 ) };
	// 8303EF88: 9BE3001C  stb r31, 0x1c(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[31].u8 ) };
	// 8303EF8C: 9BC3001D  stb r30, 0x1d(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(29 as u32), ctx.r[30].u8 ) };
	// 8303EF90: 93E30020  stw r31, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[31].u32 ) };
	// 8303EF94: 93E30024  stw r31, 0x24(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), ctx.r[31].u32 ) };
	// 8303EF98: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8303EF9C: 48000008  b 0x8303efa4
	pc = 0x8303EFA4; continue 'dispatch;
	// 8303EFA0: 7FDCF378  mr r28, r30
	ctx.r[28].u64 = ctx.r[30].u64;
	// 8303EFA4: 3977FFFF  addi r11, r23, -1
	ctx.r[11].s64 = ctx.r[23].s64 + -1;
	// 8303EFA8: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8303EFAC: 40990060  ble cr6, 0x8303f00c
	if !ctx.cr[6].gt {
	pc = 0x8303F00C; continue 'dispatch;
	}
	// 8303EFB0: 3B2BFFFF  addi r25, r11, -1
	ctx.r[25].s64 = ctx.r[11].s64 + -1;
	// 8303EFB4: 38600028  li r3, 0x28
	ctx.r[3].s64 = 40;
	// 8303EFB8: 809D0068  lwz r4, 0x68(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(104 as u32) ) } as u64;
	// 8303EFBC: 4BF992DD  bl 0x82fd8298
	ctx.lr = 0x8303EFC0;
	sub_82FD8298(ctx, base);
	// 8303EFC0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8303EFC4: 4182003C  beq 0x8303f000
	if ctx.cr[0].eq {
	pc = 0x8303F000; continue 'dispatch;
	}
	// 8303EFC8: 817D0068  lwz r11, 0x68(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(104 as u32) ) } as u64;
	// 8303EFCC: 93830010  stw r28, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[28].u32 ) };
	// 8303EFD0: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 8303EFD4: 93C30008  stw r30, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 8303EFD8: 93430000  stw r26, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[26].u32 ) };
	// 8303EFDC: 93C3000C  stw r30, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[30].u32 ) };
	// 8303EFE0: 93630014  stw r27, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[27].u32 ) };
	// 8303EFE4: 93030018  stw r24, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[24].u32 ) };
	// 8303EFE8: 9BE3001C  stb r31, 0x1c(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[31].u8 ) };
	// 8303EFEC: 9BC3001D  stb r30, 0x1d(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(29 as u32), ctx.r[30].u8 ) };
	// 8303EFF0: 93E30020  stw r31, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[31].u32 ) };
	// 8303EFF4: 93E30024  stw r31, 0x24(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), ctx.r[31].u32 ) };
	// 8303EFF8: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8303EFFC: 48000008  b 0x8303f004
	pc = 0x8303F004; continue 'dispatch;
	// 8303F000: 7FDCF378  mr r28, r30
	ctx.r[28].u64 = ctx.r[30].u64;
	// 8303F004: 3739FFFF  addic. r25, r25, -1
	ctx.xer.ca = (ctx.r[25].u32 > (!(-1 as u32)));
	ctx.r[25].s64 = ctx.r[25].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[25].s32, 0, &mut ctx.xer);
	// 8303F008: 4082FFAC  bne 0x8303efb4
	if !ctx.cr[0].eq {
	pc = 0x8303EFB4; continue 'dispatch;
	}
	// 8303F00C: 7F37B051  subf. r25, r23, r22
	ctx.r[25].s64 = ctx.r[22].s64 - ctx.r[23].s64;
	ctx.cr[0].compare_i32(ctx.r[25].s32, 0, &mut ctx.xer);
	// 8303F010: 40810108  ble 0x8303f118
	if !ctx.cr[0].gt {
	pc = 0x8303F118; continue 'dispatch;
	}
	// 8303F014: 38600028  li r3, 0x28
	ctx.r[3].s64 = 40;
	// 8303F018: 809D0068  lwz r4, 0x68(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(104 as u32) ) } as u64;
	// 8303F01C: 4BF9927D  bl 0x82fd8298
	ctx.lr = 0x8303F020;
	sub_82FD8298(ctx, base);
	// 8303F020: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8303F024: 4182003C  beq 0x8303f060
	if ctx.cr[0].eq {
	pc = 0x8303F060; continue 'dispatch;
	}
	// 8303F028: 817D0068  lwz r11, 0x68(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(104 as u32) ) } as u64;
	// 8303F02C: 93630010  stw r27, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[27].u32 ) };
	// 8303F030: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 8303F034: 93C30008  stw r30, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 8303F038: 93430000  stw r26, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[26].u32 ) };
	// 8303F03C: 93C3000C  stw r30, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[30].u32 ) };
	// 8303F040: 93C30014  stw r30, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[30].u32 ) };
	// 8303F044: 93E30018  stw r31, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[31].u32 ) };
	// 8303F048: 9BC3001C  stb r30, 0x1c(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[30].u8 ) };
	// 8303F04C: 9BE3001D  stb r31, 0x1d(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(29 as u32), ctx.r[31].u8 ) };
	// 8303F050: 93E30020  stw r31, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[31].u32 ) };
	// 8303F054: 93E30024  stw r31, 0x24(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), ctx.r[31].u32 ) };
	// 8303F058: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8303F05C: 48000008  b 0x8303f064
	pc = 0x8303F064; continue 'dispatch;
	// 8303F060: 7FDBF378  mr r27, r30
	ctx.r[27].u64 = ctx.r[30].u64;
	// 8303F064: 38600028  li r3, 0x28
	ctx.r[3].s64 = 40;
	// 8303F068: 809D0068  lwz r4, 0x68(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(104 as u32) ) } as u64;
	// 8303F06C: 4BF9922D  bl 0x82fd8298
	ctx.lr = 0x8303F070;
	sub_82FD8298(ctx, base);
	// 8303F070: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8303F074: 4182003C  beq 0x8303f0b0
	if ctx.cr[0].eq {
	pc = 0x8303F0B0; continue 'dispatch;
	}
	// 8303F078: 817D0068  lwz r11, 0x68(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(104 as u32) ) } as u64;
	// 8303F07C: 93830010  stw r28, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[28].u32 ) };
	// 8303F080: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 8303F084: 93C30008  stw r30, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 8303F088: 93430000  stw r26, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[26].u32 ) };
	// 8303F08C: 93C3000C  stw r30, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[30].u32 ) };
	// 8303F090: 93630014  stw r27, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[27].u32 ) };
	// 8303F094: 93030018  stw r24, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[24].u32 ) };
	// 8303F098: 9BE3001C  stb r31, 0x1c(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[31].u8 ) };
	// 8303F09C: 9BE3001D  stb r31, 0x1d(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(29 as u32), ctx.r[31].u8 ) };
	// 8303F0A0: 93E30020  stw r31, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[31].u32 ) };
	// 8303F0A4: 93E30024  stw r31, 0x24(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), ctx.r[31].u32 ) };
	// 8303F0A8: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8303F0AC: 48000008  b 0x8303f0b4
	pc = 0x8303F0B4; continue 'dispatch;
	// 8303F0B0: 7FDCF378  mr r28, r30
	ctx.r[28].u64 = ctx.r[30].u64;
	// 8303F0B4: 2F190001  cmpwi cr6, r25, 1
	ctx.cr[6].compare_i32(ctx.r[25].s32, 1, &mut ctx.xer);
	// 8303F0B8: 40990060  ble cr6, 0x8303f118
	if !ctx.cr[6].gt {
	pc = 0x8303F118; continue 'dispatch;
	}
	// 8303F0BC: 3B39FFFF  addi r25, r25, -1
	ctx.r[25].s64 = ctx.r[25].s64 + -1;
	// 8303F0C0: 38600028  li r3, 0x28
	ctx.r[3].s64 = 40;
	// 8303F0C4: 809D0068  lwz r4, 0x68(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(104 as u32) ) } as u64;
	// 8303F0C8: 4BF991D1  bl 0x82fd8298
	ctx.lr = 0x8303F0CC;
	sub_82FD8298(ctx, base);
	// 8303F0CC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8303F0D0: 4182003C  beq 0x8303f10c
	if ctx.cr[0].eq {
	pc = 0x8303F10C; continue 'dispatch;
	}
	// 8303F0D4: 817D0068  lwz r11, 0x68(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(104 as u32) ) } as u64;
	// 8303F0D8: 93830010  stw r28, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[28].u32 ) };
	// 8303F0DC: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 8303F0E0: 93C30008  stw r30, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 8303F0E4: 93430000  stw r26, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[26].u32 ) };
	// 8303F0E8: 93C3000C  stw r30, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[30].u32 ) };
	// 8303F0EC: 93630014  stw r27, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[27].u32 ) };
	// 8303F0F0: 93030018  stw r24, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[24].u32 ) };
	// 8303F0F4: 9BE3001C  stb r31, 0x1c(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[31].u8 ) };
	// 8303F0F8: 9BC3001D  stb r30, 0x1d(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(29 as u32), ctx.r[30].u8 ) };
	// 8303F0FC: 93E30020  stw r31, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[31].u32 ) };
	// 8303F100: 93E30024  stw r31, 0x24(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), ctx.r[31].u32 ) };
	// 8303F104: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8303F108: 48000008  b 0x8303f110
	pc = 0x8303F110; continue 'dispatch;
	// 8303F10C: 7FDCF378  mr r28, r30
	ctx.r[28].u64 = ctx.r[30].u64;
	// 8303F110: 3739FFFF  addic. r25, r25, -1
	ctx.xer.ca = (ctx.r[25].u32 > (!(-1 as u32)));
	ctx.r[25].s64 = ctx.r[25].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[25].s32, 0, &mut ctx.xer);
	// 8303F114: 4082FFAC  bne 0x8303f0c0
	if !ctx.cr[0].eq {
	pc = 0x8303F0C0; continue 'dispatch;
	}
	// 8303F118: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8303F11C: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 8303F120: 48169080  b 0x831a81a0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303F128(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8303F128 size=188
    let mut pc: u32 = 0x8303F128;
    'dispatch: loop {
        match pc {
            0x8303F128 => {
    //   block [0x8303F128..0x8303F1E4)
	// 8303F128: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8303F12C: 48169041  bl 0x831a816c
	ctx.lr = 0x8303F130;
	sub_831A8130(ctx, base);
	// 8303F130: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8303F134: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8303F138: 807F0068  lwz r3, 0x68(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) } as u64;
	// 8303F13C: 817F0060  lwz r11, 0x60(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 8303F140: 557E083C  slwi r30, r11, 1
	ctx.r[30].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 8303F144: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303F148: 57C4103A  slwi r4, r30, 2
	ctx.r[4].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 8303F14C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8303F150: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8303F154: 4E800421  bctrl
	ctx.lr = 0x8303F158;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8303F158: 815F0060  lwz r10, 0x60(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 8303F15C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8303F160: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8303F164: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8303F168: 40990028  ble cr6, 0x8303f190
	if !ctx.cr[6].gt {
	pc = 0x8303F190; continue 'dispatch;
	}
	// 8303F16C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8303F170: 813F0058  lwz r9, 0x58(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 8303F174: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8303F178: 7D29502E  lwzx r9, r9, r10
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 8303F17C: 7D2AE92E  stwx r9, r10, r29
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[29].u32), ctx.r[9].u32) };
	// 8303F180: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 8303F184: 813F0060  lwz r9, 0x60(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 8303F188: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 8303F18C: 4198FFE4  blt cr6, 0x8303f170
	if ctx.cr[6].lt {
	pc = 0x8303F170; continue 'dispatch;
	}
	// 8303F190: 7F0BF040  cmplw cr6, r11, r30
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[30].u32, &mut ctx.xer);
	// 8303F194: 40980028  bge cr6, 0x8303f1bc
	if !ctx.cr[6].lt {
	pc = 0x8303F1BC; continue 'dispatch;
	}
	// 8303F198: 556A103A  slwi r10, r11, 2
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8303F19C: 7D6BF051  subf. r11, r11, r30
	ctx.r[11].s64 = ctx.r[30].s64 - ctx.r[11].s64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8303F1A0: 7D4AEA14  add r10, r10, r29
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[29].u64;
	// 8303F1A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8303F1A8: 41820014  beq 0x8303f1bc
	if ctx.cr[0].eq {
	pc = 0x8303F1BC; continue 'dispatch;
	}
	// 8303F1AC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8303F1B0: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8303F1B4: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 8303F1B8: 4200FFF8  bdnz 0x8303f1b0
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x8303F1B0; continue 'dispatch;
	}
	// 8303F1BC: 807F0068  lwz r3, 0x68(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) } as u64;
	// 8303F1C0: 809F0058  lwz r4, 0x58(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 8303F1C4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303F1C8: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8303F1CC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8303F1D0: 4E800421  bctrl
	ctx.lr = 0x8303F1D4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8303F1D4: 93BF0058  stw r29, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[29].u32 ) };
	// 8303F1D8: 93DF0060  stw r30, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[30].u32 ) };
	// 8303F1DC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8303F1E0: 48168FDC  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303F1E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8303F1E8 size=76
    let mut pc: u32 = 0x8303F1E8;
    'dispatch: loop {
        match pc {
            0x8303F1E8 => {
    //   block [0x8303F1E8..0x8303F234)
	// 8303F1E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8303F1EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8303F1F0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8303F1F4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8303F1F8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8303F1FC: 3860006C  li r3, 0x6c
	ctx.r[3].s64 = 108;
	// 8303F200: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8303F204: 4BF99095  bl 0x82fd8298
	ctx.lr = 0x8303F208;
	sub_82FD8298(ctx, base);
	// 8303F208: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8303F20C: 41820010  beq 0x8303f21c
	if ctx.cr[0].eq {
	pc = 0x8303F21C; continue 'dispatch;
	}
	// 8303F210: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8303F214: 4BFFF31D  bl 0x8303e530
	ctx.lr = 0x8303F218;
	sub_8303E530(ctx, base);
	// 8303F218: 48000008  b 0x8303f220
	pc = 0x8303F220; continue 'dispatch;
	// 8303F21C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8303F220: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8303F224: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8303F228: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8303F22C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8303F230: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303F238(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8303F238 size=12
    let mut pc: u32 = 0x8303F238;
    'dispatch: loop {
        match pc {
            0x8303F238 => {
    //   block [0x8303F238..0x8303F244)
	// 8303F238: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 8303F23C: 386B32CC  addi r3, r11, 0x32cc
	ctx.r[3].s64 = ctx.r[11].s64 + 13004;
	// 8303F240: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303F248(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8303F248 size=752
    let mut pc: u32 = 0x8303F248;
    'dispatch: loop {
        match pc {
            0x8303F248 => {
    //   block [0x8303F248..0x8303F538)
	// 8303F248: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8303F24C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8303F250: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8303F254: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8303F258: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8303F25C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8303F260: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8303F264: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8303F268: A97E0000  lha r11, 0(r30)
	ctx.r[11].s64 = (unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as i16) as i64;
	// 8303F26C: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 8303F270: 556BDFFF  rlwinm. r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8303F274: 41820124  beq 0x8303f398
	if ctx.cr[0].eq {
	pc = 0x8303F398; continue 'dispatch;
	}
	// 8303F278: 889F0004  lbz r4, 4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8303F27C: 4BFB9F85  bl 0x82ff9200
	ctx.lr = 0x8303F280;
	sub_82FF9200(ctx, base);
	// 8303F280: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8303F284: 889F0005  lbz r4, 5(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(5 as u32) ) } as u64;
	// 8303F288: 4BFB9F79  bl 0x82ff9200
	ctx.lr = 0x8303F28C;
	sub_82FF9200(ctx, base);
	// 8303F28C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8303F290: 889F0006  lbz r4, 6(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(6 as u32) ) } as u64;
	// 8303F294: 4BFB9F6D  bl 0x82ff9200
	ctx.lr = 0x8303F298;
	sub_82FF9200(ctx, base);
	// 8303F298: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8303F29C: 889F0007  lbz r4, 7(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(7 as u32) ) } as u64;
	// 8303F2A0: 4BFB9F61  bl 0x82ff9200
	ctx.lr = 0x8303F2A4;
	sub_82FF9200(ctx, base);
	// 8303F2A4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8303F2A8: 889F0008  lbz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8303F2AC: 4BFB9F55  bl 0x82ff9200
	ctx.lr = 0x8303F2B0;
	sub_82FF9200(ctx, base);
	// 8303F2B0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8303F2B4: 809F000C  lwz r4, 0xc(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8303F2B8: 4BFBA041  bl 0x82ff92f8
	ctx.lr = 0x8303F2BC;
	sub_82FF92F8(ctx, base);
	// 8303F2BC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8303F2C0: 809F0010  lwz r4, 0x10(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8303F2C4: 4BFBA035  bl 0x82ff92f8
	ctx.lr = 0x8303F2C8;
	sub_82FF92F8(ctx, base);
	// 8303F2C8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8303F2CC: 809F0014  lwz r4, 0x14(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 8303F2D0: 4BFBA029  bl 0x82ff92f8
	ctx.lr = 0x8303F2D4;
	sub_82FF92F8(ctx, base);
	// 8303F2D4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8303F2D8: 809F0018  lwz r4, 0x18(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 8303F2DC: 4BFBA01D  bl 0x82ff92f8
	ctx.lr = 0x8303F2E0;
	sub_82FF92F8(ctx, base);
	// 8303F2E0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8303F2E4: 809F001C  lwz r4, 0x1c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 8303F2E8: 4BFBA011  bl 0x82ff92f8
	ctx.lr = 0x8303F2EC;
	sub_82FF92F8(ctx, base);
	// 8303F2EC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8303F2F0: 809F0020  lwz r4, 0x20(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 8303F2F4: 4BFBA005  bl 0x82ff92f8
	ctx.lr = 0x8303F2F8;
	sub_82FF92F8(ctx, base);
	// 8303F2F8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8303F2FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8303F300: 809F0024  lwz r4, 0x24(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 8303F304: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8303F308: 4BFBA5F9  bl 0x82ff9900
	ctx.lr = 0x8303F30C;
	sub_82FF9900(ctx, base);
	// 8303F30C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8303F310: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8303F314: 809F0028  lwz r4, 0x28(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 8303F318: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8303F31C: 4BFBA5E5  bl 0x82ff9900
	ctx.lr = 0x8303F320;
	sub_82FF9900(ctx, base);
	// 8303F320: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8303F324: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8303F328: 809F002C  lwz r4, 0x2c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 8303F32C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8303F330: 4BFBA5D1  bl 0x82ff9900
	ctx.lr = 0x8303F334;
	sub_82FF9900(ctx, base);
	// 8303F334: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8303F338: 809F0030  lwz r4, 0x30(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 8303F33C: 48001F2D  bl 0x83041268
	ctx.lr = 0x8303F340;
	sub_83041268(ctx, base);
	// 8303F340: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8303F344: 809F0034  lwz r4, 0x34(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 8303F348: 48001F21  bl 0x83041268
	ctx.lr = 0x8303F34C;
	sub_83041268(ctx, base);
	// 8303F34C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8303F350: 809F0038  lwz r4, 0x38(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 8303F354: 4BFBA8AD  bl 0x82ff9c00
	ctx.lr = 0x8303F358;
	sub_82FF9C00(ctx, base);
	// 8303F358: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8303F35C: 809F003C  lwz r4, 0x3c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) } as u64;
	// 8303F360: 4BFBA8A1  bl 0x82ff9c00
	ctx.lr = 0x8303F364;
	sub_82FF9C00(ctx, base);
	// 8303F364: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8303F368: 809F0040  lwz r4, 0x40(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as u64;
	// 8303F36C: 4BFBA895  bl 0x82ff9c00
	ctx.lr = 0x8303F370;
	sub_82FF9C00(ctx, base);
	// 8303F370: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8303F374: 809F0044  lwz r4, 0x44(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(68 as u32) ) } as u64;
	// 8303F378: 4BFBA889  bl 0x82ff9c00
	ctx.lr = 0x8303F37C;
	sub_82FF9C00(ctx, base);
	// 8303F37C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8303F380: 807F0048  lwz r3, 0x48(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 8303F384: 4800D7B5  bl 0x8304cb38
	ctx.lr = 0x8303F388;
	sub_8304CB38(ctx, base);
	// 8303F388: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8303F38C: 807F004C  lwz r3, 0x4c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 8303F390: 48011119  bl 0x830504a8
	ctx.lr = 0x8303F394;
	sub_830504A8(ctx, base);
	// 8303F394: 4800018C  b 0x8303f520
	pc = 0x8303F520; continue 'dispatch;
	// 8303F398: 389F0004  addi r4, r31, 4
	ctx.r[4].s64 = ctx.r[31].s64 + 4;
	// 8303F39C: 4BFBA0E5  bl 0x82ff9480
	ctx.lr = 0x8303F3A0;
	sub_82FF9480(ctx, base);
	// 8303F3A0: 389F0005  addi r4, r31, 5
	ctx.r[4].s64 = ctx.r[31].s64 + 5;
	// 8303F3A4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8303F3A8: 4BFBA0D9  bl 0x82ff9480
	ctx.lr = 0x8303F3AC;
	sub_82FF9480(ctx, base);
	// 8303F3AC: 389F0006  addi r4, r31, 6
	ctx.r[4].s64 = ctx.r[31].s64 + 6;
	// 8303F3B0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8303F3B4: 4BFBA0CD  bl 0x82ff9480
	ctx.lr = 0x8303F3B8;
	sub_82FF9480(ctx, base);
	// 8303F3B8: 389F0007  addi r4, r31, 7
	ctx.r[4].s64 = ctx.r[31].s64 + 7;
	// 8303F3BC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8303F3C0: 4BFBA0C1  bl 0x82ff9480
	ctx.lr = 0x8303F3C4;
	sub_82FF9480(ctx, base);
	// 8303F3C4: 389F0008  addi r4, r31, 8
	ctx.r[4].s64 = ctx.r[31].s64 + 8;
	// 8303F3C8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8303F3CC: 4BFBA0B5  bl 0x82ff9480
	ctx.lr = 0x8303F3D0;
	sub_82FF9480(ctx, base);
	// 8303F3D0: 389F000C  addi r4, r31, 0xc
	ctx.r[4].s64 = ctx.r[31].s64 + 12;
	// 8303F3D4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8303F3D8: 4BFBA1A1  bl 0x82ff9578
	ctx.lr = 0x8303F3DC;
	sub_82FF9578(ctx, base);
	// 8303F3DC: 389F0010  addi r4, r31, 0x10
	ctx.r[4].s64 = ctx.r[31].s64 + 16;
	// 8303F3E0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8303F3E4: 4BFBA195  bl 0x82ff9578
	ctx.lr = 0x8303F3E8;
	sub_82FF9578(ctx, base);
	// 8303F3E8: 389F0014  addi r4, r31, 0x14
	ctx.r[4].s64 = ctx.r[31].s64 + 20;
	// 8303F3EC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8303F3F0: 4BFBA189  bl 0x82ff9578
	ctx.lr = 0x8303F3F4;
	sub_82FF9578(ctx, base);
	// 8303F3F4: 389F0018  addi r4, r31, 0x18
	ctx.r[4].s64 = ctx.r[31].s64 + 24;
	// 8303F3F8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8303F3FC: 4BFBA17D  bl 0x82ff9578
	ctx.lr = 0x8303F400;
	sub_82FF9578(ctx, base);
	// 8303F400: 389F001C  addi r4, r31, 0x1c
	ctx.r[4].s64 = ctx.r[31].s64 + 28;
	// 8303F404: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8303F408: 4BFBA171  bl 0x82ff9578
	ctx.lr = 0x8303F40C;
	sub_82FF9578(ctx, base);
	// 8303F40C: 389F0020  addi r4, r31, 0x20
	ctx.r[4].s64 = ctx.r[31].s64 + 32;
	// 8303F410: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8303F414: 4BFBA165  bl 0x82ff9578
	ctx.lr = 0x8303F418;
	sub_82FF9578(ctx, base);
	// 8303F418: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8303F41C: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 8303F420: 38A10054  addi r5, r1, 0x54
	ctx.r[5].s64 = ctx.r[1].s64 + 84;
	// 8303F424: 389F0024  addi r4, r31, 0x24
	ctx.r[4].s64 = ctx.r[31].s64 + 36;
	// 8303F428: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8303F42C: 4BFBA6FD  bl 0x82ff9b28
	ctx.lr = 0x8303F430;
	sub_82FF9B28(ctx, base);
	// 8303F430: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8303F434: 38C10054  addi r6, r1, 0x54
	ctx.r[6].s64 = ctx.r[1].s64 + 84;
	// 8303F438: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8303F43C: 389F0028  addi r4, r31, 0x28
	ctx.r[4].s64 = ctx.r[31].s64 + 40;
	// 8303F440: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8303F444: 4BFBA6E5  bl 0x82ff9b28
	ctx.lr = 0x8303F448;
	sub_82FF9B28(ctx, base);
	// 8303F448: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8303F44C: 38C10054  addi r6, r1, 0x54
	ctx.r[6].s64 = ctx.r[1].s64 + 84;
	// 8303F450: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8303F454: 389F002C  addi r4, r31, 0x2c
	ctx.r[4].s64 = ctx.r[31].s64 + 44;
	// 8303F458: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8303F45C: 4BFBA6CD  bl 0x82ff9b28
	ctx.lr = 0x8303F460;
	sub_82FF9B28(ctx, base);
	// 8303F460: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8303F464: 48001ECD  bl 0x83041330
	ctx.lr = 0x8303F468;
	sub_83041330(ctx, base);
	// 8303F468: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8303F46C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8303F470: 917F0030  stw r11, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 8303F474: 48001EBD  bl 0x83041330
	ctx.lr = 0x8303F478;
	sub_83041330(ctx, base);
	// 8303F478: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 8303F47C: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 8303F480: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8303F484: 388B32CC  addi r4, r11, 0x32cc
	ctx.r[4].s64 = ctx.r[11].s64 + 13004;
	// 8303F488: 915F0034  stw r10, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[10].u32 ) };
	// 8303F48C: 4BFBA835  bl 0x82ff9cc0
	ctx.lr = 0x8303F490;
	sub_82FF9CC0(ctx, base);
	// 8303F490: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 8303F494: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 8303F498: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8303F49C: 388B34D0  addi r4, r11, 0x34d0
	ctx.r[4].s64 = ctx.r[11].s64 + 13520;
	// 8303F4A0: 915F0038  stw r10, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[10].u32 ) };
	// 8303F4A4: 4BFBA81D  bl 0x82ff9cc0
	ctx.lr = 0x8303F4A8;
	sub_82FF9CC0(ctx, base);
	// 8303F4A8: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 8303F4AC: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 8303F4B0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8303F4B4: 388B34C0  addi r4, r11, 0x34c0
	ctx.r[4].s64 = ctx.r[11].s64 + 13504;
	// 8303F4B8: 915F003C  stw r10, 0x3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[10].u32 ) };
	// 8303F4BC: 4BFBA805  bl 0x82ff9cc0
	ctx.lr = 0x8303F4C0;
	sub_82FF9CC0(ctx, base);
	// 8303F4C0: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 8303F4C4: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 8303F4C8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8303F4CC: 388B34C8  addi r4, r11, 0x34c8
	ctx.r[4].s64 = ctx.r[11].s64 + 13512;
	// 8303F4D0: 915F0040  stw r10, 0x40(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), ctx.r[10].u32 ) };
	// 8303F4D4: 4BFBA7ED  bl 0x82ff9cc0
	ctx.lr = 0x8303F4D8;
	sub_82FF9CC0(ctx, base);
	// 8303F4D8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8303F4DC: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 8303F4E0: 917F0044  stw r11, 0x44(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(68 as u32), ctx.r[11].u32 ) };
	// 8303F4E4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8303F4E8: 38800008  li r4, 8
	ctx.r[4].s64 = 8;
	// 8303F4EC: 387F0048  addi r3, r31, 0x48
	ctx.r[3].s64 = ctx.r[31].s64 + 72;
	// 8303F4F0: 4800E319  bl 0x8304d808
	ctx.lr = 0x8303F4F4;
	sub_8304D808(ctx, base);
	// 8303F4F4: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 8303F4F8: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8303F4FC: 3880001D  li r4, 0x1d
	ctx.r[4].s64 = 29;
	// 8303F500: 387F004C  addi r3, r31, 0x4c
	ctx.r[3].s64 = ctx.r[31].s64 + 76;
	// 8303F504: 4800F915  bl 0x8304ee18
	ctx.lr = 0x8303F508;
	sub_8304EE18(ctx, base);
	// 8303F508: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8303F50C: 917F0054  stw r11, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8303F510: 917F0064  stw r11, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8303F514: 917F0058  stw r11, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 8303F518: 917F0060  stw r11, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 8303F51C: 917F005C  stw r11, 0x5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8303F520: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8303F524: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8303F528: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8303F52C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8303F530: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8303F534: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303F538(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8303F538 size=76
    let mut pc: u32 = 0x8303F538;
    'dispatch: loop {
        match pc {
            0x8303F538 => {
    //   block [0x8303F538..0x8303F584)
	// 8303F538: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8303F53C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8303F540: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8303F544: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8303F548: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8303F54C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8303F550: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8303F554: 4BFFEB35  bl 0x8303e088
	ctx.lr = 0x8303F558;
	sub_8303E088(ctx, base);
	// 8303F558: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8303F55C: 4182000C  beq 0x8303f568
	if ctx.cr[0].eq {
	pc = 0x8303F568; continue 'dispatch;
	}
	// 8303F560: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8303F564: 4BF98D7D  bl 0x82fd82e0
	ctx.lr = 0x8303F568;
	sub_82FD82E0(ctx, base);
	// 8303F568: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8303F56C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8303F570: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8303F574: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8303F578: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8303F57C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8303F580: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303F588(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8303F588 size=76
    let mut pc: u32 = 0x8303F588;
    'dispatch: loop {
        match pc {
            0x8303F588 => {
    //   block [0x8303F588..0x8303F5D4)
	// 8303F588: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8303F58C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8303F590: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8303F594: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8303F598: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8303F59C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8303F5A0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8303F5A4: 4BFFEED5  bl 0x8303e478
	ctx.lr = 0x8303F5A8;
	sub_8303E478(ctx, base);
	// 8303F5A8: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8303F5AC: 4182000C  beq 0x8303f5b8
	if ctx.cr[0].eq {
	pc = 0x8303F5B8; continue 'dispatch;
	}
	// 8303F5B0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8303F5B4: 4BF98D2D  bl 0x82fd82e0
	ctx.lr = 0x8303F5B8;
	sub_82FD82E0(ctx, base);
	// 8303F5B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8303F5BC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8303F5C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8303F5C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8303F5C8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8303F5CC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8303F5D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303F5D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8303F5D8 size=136
    let mut pc: u32 = 0x8303F5D8;
    'dispatch: loop {
        match pc {
            0x8303F5D8 => {
    //   block [0x8303F5D8..0x8303F660)
	// 8303F5D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8303F5DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8303F5E0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8303F5E4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8303F5E8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8303F5EC: 3FE08339  lis r31, -0x7cc7
	ctx.r[31].s64 = -2093416448;
	// 8303F5F0: 807FB9B0  lwz r3, -0x4650(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-18000 as u32) ) } as u64;
	// 8303F5F4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8303F5F8: 419A0018  beq cr6, 0x8303f610
	if ctx.cr[6].eq {
	pc = 0x8303F610; continue 'dispatch;
	}
	// 8303F5FC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303F600: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8303F604: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303F608: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8303F60C: 4E800421  bctrl
	ctx.lr = 0x8303F610;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8303F610: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8303F614: 917FB9B0  stw r11, -0x4650(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-18000 as u32), ctx.r[11].u32 ) };
	// 8303F618: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 8303F61C: 3BEBB9A8  addi r31, r11, -0x4658
	ctx.r[31].s64 = ctx.r[11].s64 + -18008;
	// 8303F620: 83DF0004  lwz r30, 4(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8303F624: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8303F628: 419A0014  beq cr6, 0x8303f63c
	if ctx.cr[6].eq {
	pc = 0x8303F63C; continue 'dispatch;
	}
	// 8303F62C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8303F630: 4BFB6159  bl 0x82ff5788
	ctx.lr = 0x8303F634;
	sub_82FF5788(ctx, base);
	// 8303F634: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8303F638: 4BF98CA9  bl 0x82fd82e0
	ctx.lr = 0x8303F63C;
	sub_82FD82E0(ctx, base);
	// 8303F63C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8303F640: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8303F644: 997F0000  stb r11, 0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 8303F648: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8303F64C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8303F650: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8303F654: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8303F658: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8303F65C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303F660(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8303F660 size=8
    let mut pc: u32 = 0x8303F660;
    'dispatch: loop {
        match pc {
            0x8303F660 => {
    //   block [0x8303F660..0x8303F668)
	// 8303F660: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8303F664: 8215DD00  lwz r16, -0x2300(r21)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-8960 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303F668(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8303F668 size=708
    let mut pc: u32 = 0x8303F668;
    'dispatch: loop {
        match pc {
            0x8303F668 => {
    //   block [0x8303F668..0x8303F92C)
	// 8303F668: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8303F66C: 48168AE5  bl 0x831a8150
	ctx.lr = 0x8303F670;
	sub_831A8130(ctx, base);
	// 8303F670: 3BE1FE40  addi r31, r1, -0x1c0
	ctx.r[31].s64 = ctx.r[1].s64 + -448;
	// 8303F674: 9421FE40  stwu r1, -0x1c0(r1)
	ea = ctx.r[1].u32.wrapping_add(-448 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8303F678: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 8303F67C: 7C761B78  mr r22, r3
	ctx.r[22].u64 = ctx.r[3].u64;
	// 8303F680: 3AEBB9A8  addi r23, r11, -0x4658
	ctx.r[23].s64 = ctx.r[11].s64 + -18008;
	// 8303F684: 3F408339  lis r26, -0x7cc7
	ctx.r[26].s64 = -2093416448;
	// 8303F688: 89770000  lbz r11, 0(r23)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[23].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303F68C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8303F690: 40820290  bne 0x8303f920
	if !ctx.cr[0].eq {
	pc = 0x8303F920; continue 'dispatch;
	}
	// 8303F694: 80970004  lwz r4, 4(r23)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(4 as u32) ) } as u64;
	// 8303F698: 3B000000  li r24, 0
	ctx.r[24].s64 = 0;
	// 8303F69C: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 8303F6A0: 409A0050  bne cr6, 0x8303f6f0
	if !ctx.cr[6].eq {
	pc = 0x8303F6F0; continue 'dispatch;
	}
	// 8303F6A4: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 8303F6A8: 387F0054  addi r3, r31, 0x54
	ctx.r[3].s64 = ctx.r[31].s64 + 84;
	// 8303F6AC: 808BB7EC  lwz r4, -0x4814(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18452 as u32) ) } as u64;
	// 8303F6B0: 4BFB6129  bl 0x82ff57d8
	ctx.lr = 0x8303F6B4;
	sub_82FF57D8(ctx, base);
	// 8303F6B4: 81770004  lwz r11, 4(r23)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(4 as u32) ) } as u64;
	// 8303F6B8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8303F6BC: 409A0028  bne cr6, 0x8303f6e4
	if !ctx.cr[6].eq {
	pc = 0x8303F6E4; continue 'dispatch;
	}
	// 8303F6C0: 38600004  li r3, 4
	ctx.r[3].s64 = 4;
	// 8303F6C4: 4BF98B85  bl 0x82fd8248
	ctx.lr = 0x8303F6C8;
	sub_82FD8248(ctx, base);
	// 8303F6C8: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 8303F6CC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8303F6D0: 4182000C  beq 0x8303f6dc
	if ctx.cr[0].eq {
	pc = 0x8303F6DC; continue 'dispatch;
	}
	// 8303F6D4: 4BFB6075  bl 0x82ff5748
	ctx.lr = 0x8303F6D8;
	sub_82FF5748(ctx, base);
	// 8303F6D8: 48000008  b 0x8303f6e0
	pc = 0x8303F6E0; continue 'dispatch;
	// 8303F6DC: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 8303F6E0: 90770004  stw r3, 4(r23)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[23].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 8303F6E4: 387F0054  addi r3, r31, 0x54
	ctx.r[3].s64 = ctx.r[31].s64 + 84;
	// 8303F6E8: 4BFB6129  bl 0x82ff5810
	ctx.lr = 0x8303F6EC;
	sub_82FF5810(ctx, base);
	// 8303F6EC: 80970004  lwz r4, 4(r23)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(4 as u32) ) } as u64;
	// 8303F6F0: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 8303F6F4: 4BFB60E5  bl 0x82ff57d8
	ctx.lr = 0x8303F6F8;
	sub_82FF57D8(ctx, base);
	// 8303F6F8: 89770000  lbz r11, 0(r23)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[23].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303F6FC: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8303F700: 40820218  bne 0x8303f918
	if !ctx.cr[0].eq {
	pc = 0x8303F918; continue 'dispatch;
	}
	// 8303F704: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8303F708: 388BCC98  addi r4, r11, -0x3368
	ctx.r[4].s64 = ctx.r[11].s64 + -13160;
	// 8303F70C: A1640000  lhz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303F710: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8303F714: 41820030  beq 0x8303f744
	if ctx.cr[0].eq {
	pc = 0x8303F744; continue 'dispatch;
	}
	// 8303F718: A1640002  lhz r11, 2(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(2 as u32) ) } as u64;
	// 8303F71C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8303F720: 39640002  addi r11, r4, 2
	ctx.r[11].s64 = ctx.r[4].s64 + 2;
	// 8303F724: 41820014  beq 0x8303f738
	if ctx.cr[0].eq {
	pc = 0x8303F738; continue 'dispatch;
	}
	// 8303F728: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 8303F72C: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303F730: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8303F734: 4082FFF4  bne 0x8303f728
	if !ctx.cr[0].eq {
	pc = 0x8303F728; continue 'dispatch;
	}
	// 8303F738: 7D645850  subf r11, r4, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[4].s64;
	// 8303F73C: 7D7E0E70  srawi r30, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[30].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 8303F740: 48000008  b 0x8303f748
	pc = 0x8303F748; continue 'dispatch;
	// 8303F744: 7F1EC378  mr r30, r24
	ctx.r[30].u64 = ctx.r[24].u64;
	// 8303F748: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 8303F74C: 4BF9241D  bl 0x82fd1b68
	ctx.lr = 0x8303F750;
	sub_82FD1B68(ctx, base);
	// 8303F750: 3D408214  lis r10, -0x7dec
	ctx.r[10].s64 = -2112618496;
	// 8303F754: 57CB083C  slwi r11, r30, 1
	ctx.r[11].u32 = ctx.r[30].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8303F758: 393F0060  addi r9, r31, 0x60
	ctx.r[9].s64 = ctx.r[31].s64 + 96;
	// 8303F75C: 3900002C  li r8, 0x2c
	ctx.r[8].s64 = 44;
	// 8303F760: 388AD5D4  addi r4, r10, -0x2a2c
	ctx.r[4].s64 = ctx.r[10].s64 + -10796;
	// 8303F764: 395F0062  addi r10, r31, 0x62
	ctx.r[10].s64 = ctx.r[31].s64 + 98;
	// 8303F768: 7C6B5214  add r3, r11, r10
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8303F76C: 7D0B4B2E  sthx r8, r11, r9
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32), ctx.r[8].u16) };
	// 8303F770: 4BF923F9  bl 0x82fd1b68
	ctx.lr = 0x8303F774;
	sub_82FD1B68(ctx, base);
	// 8303F774: 3860006C  li r3, 0x6c
	ctx.r[3].s64 = 108;
	// 8303F778: 4BF98AD1  bl 0x82fd8248
	ctx.lr = 0x8303F77C;
	sub_82FD8248(ctx, base);
	// 8303F77C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8303F780: 3F808339  lis r28, -0x7cc7
	ctx.r[28].s64 = -2093416448;
	// 8303F784: 41820014  beq 0x8303f798
	if ctx.cr[0].eq {
	pc = 0x8303F798; continue 'dispatch;
	}
	// 8303F788: 809CB7E8  lwz r4, -0x4818(r28)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 8303F78C: 4BFFEDA5  bl 0x8303e530
	ctx.lr = 0x8303F790;
	sub_8303E530(ctx, base);
	// 8303F790: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8303F794: 48000008  b 0x8303f79c
	pc = 0x8303F79C; continue 'dispatch;
	// 8303F798: 7F0BC378  mr r11, r24
	ctx.r[11].u64 = ctx.r[24].u64;
	// 8303F79C: 38600028  li r3, 0x28
	ctx.r[3].s64 = 40;
	// 8303F7A0: 917AB9B0  stw r11, -0x4650(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(-18000 as u32), ctx.r[11].u32 ) };
	// 8303F7A4: 4BF98AA5  bl 0x82fd8248
	ctx.lr = 0x8303F7A8;
	sub_82FD8248(ctx, base);
	// 8303F7A8: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8303F7AC: 93DF0054  stw r30, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[30].u32 ) };
	// 8303F7B0: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8303F7B4: 3B2B8158  addi r25, r11, -0x7ea8
	ctx.r[25].s64 = ctx.r[11].s64 + -32424;
	// 8303F7B8: 41820050  beq 0x8303f808
	if ctx.cr[0].eq {
	pc = 0x8303F808; continue 'dispatch;
	}
	// 8303F7BC: 38600024  li r3, 0x24
	ctx.r[3].s64 = 36;
	// 8303F7C0: 4BF98A89  bl 0x82fd8248
	ctx.lr = 0x8303F7C4;
	sub_82FD8248(ctx, base);
	// 8303F7C4: 907F0058  stw r3, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[3].u32 ) };
	// 8303F7C8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8303F7CC: 41820020  beq 0x8303f7ec
	if ctx.cr[0].eq {
	pc = 0x8303F7EC; continue 'dispatch;
	}
	// 8303F7D0: 7EC6B378  mr r6, r22
	ctx.r[6].u64 = ctx.r[22].u64;
	// 8303F7D4: 80FCB7E8  lwz r7, -0x4818(r28)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 8303F7D8: 7F25CB78  mr r5, r25
	ctx.r[5].u64 = ctx.r[25].u64;
	// 8303F7DC: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 8303F7E0: 4BF9F449  bl 0x82fdec28
	ctx.lr = 0x8303F7E4;
	sub_82FDEC28(ctx, base);
	// 8303F7E4: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8303F7E8: 48000008  b 0x8303f7f0
	pc = 0x8303F7F0; continue 'dispatch;
	// 8303F7EC: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 8303F7F0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8303F7F4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8303F7F8: 80DCB7E8  lwz r6, -0x4818(r28)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 8303F7FC: 4BFFE785  bl 0x8303df80
	ctx.lr = 0x8303F800;
	sub_8303DF80(ctx, base);
	// 8303F800: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8303F804: 48000008  b 0x8303f80c
	pc = 0x8303F80C; continue 'dispatch;
	// 8303F808: 7F1EC378  mr r30, r24
	ctx.r[30].u64 = ctx.r[24].u64;
	// 8303F80C: 39600016  li r11, 0x16
	ctx.r[11].s64 = 22;
	// 8303F810: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 8303F814: 931E0020  stw r24, 0x20(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(32 as u32), ctx.r[24].u32 ) };
	// 8303F818: 38600028  li r3, 0x28
	ctx.r[3].s64 = 40;
	// 8303F81C: 917E0018  stw r11, 0x18(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 8303F820: 915E0024  stw r10, 0x24(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(36 as u32), ctx.r[10].u32 ) };
	// 8303F824: 4BF98A25  bl 0x82fd8248
	ctx.lr = 0x8303F828;
	sub_82FD8248(ctx, base);
	// 8303F828: 3BA00001  li r29, 1
	ctx.r[29].s64 = 1;
	// 8303F82C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8303F830: 41820048  beq 0x8303f878
	if ctx.cr[0].eq {
	pc = 0x8303F878; continue 'dispatch;
	}
	// 8303F834: 3D408216  lis r10, -0x7dea
	ctx.r[10].s64 = -2112487424;
	// 8303F838: 817CB7E8  lwz r11, -0x4818(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 8303F83C: 39200015  li r9, 0x15
	ctx.r[9].s64 = 21;
	// 8303F840: 93030008  stw r24, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[24].u32 ) };
	// 8303F844: 394AD9C8  addi r10, r10, -0x2638
	ctx.r[10].s64 = ctx.r[10].s64 + -9784;
	// 8303F848: 9303000C  stw r24, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[24].u32 ) };
	// 8303F84C: 93C30010  stw r30, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[30].u32 ) };
	// 8303F850: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 8303F854: 93030014  stw r24, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[24].u32 ) };
	// 8303F858: 9BA3001C  stb r29, 0x1c(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[29].u8 ) };
	// 8303F85C: 91230018  stw r9, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[9].u32 ) };
	// 8303F860: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8303F864: 9BA3001D  stb r29, 0x1d(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(29 as u32), ctx.r[29].u8 ) };
	// 8303F868: 93A30020  stw r29, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[29].u32 ) };
	// 8303F86C: 93A30024  stw r29, 0x24(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), ctx.r[29].u32 ) };
	// 8303F870: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8303F874: 48000008  b 0x8303f87c
	pc = 0x8303F87C; continue 'dispatch;
	// 8303F878: 7F1BC378  mr r27, r24
	ctx.r[27].u64 = ctx.r[24].u64;
	// 8303F87C: 3860004C  li r3, 0x4c
	ctx.r[3].s64 = 76;
	// 8303F880: 4BF989C9  bl 0x82fd8248
	ctx.lr = 0x8303F884;
	sub_82FD8248(ctx, base);
	// 8303F884: 907F0058  stw r3, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[3].u32 ) };
	// 8303F888: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8303F88C: 41820028  beq 0x8303f8b4
	if ctx.cr[0].eq {
	pc = 0x8303F8B4; continue 'dispatch;
	}
	// 8303F890: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 8303F894: 813CB7E8  lwz r9, -0x4818(r28)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(-18456 as u32) ) } as u64;
	// 8303F898: 38E0000B  li r7, 0xb
	ctx.r[7].s64 = 11;
	// 8303F89C: 7EC6B378  mr r6, r22
	ctx.r[6].u64 = ctx.r[22].u64;
	// 8303F8A0: 7F25CB78  mr r5, r25
	ctx.r[5].u64 = ctx.r[25].u64;
	// 8303F8A4: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 8303F8A8: 48041FC1  bl 0x83081868
	ctx.lr = 0x8303F8AC;
	sub_83081868(ctx, base);
	// 8303F8AC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8303F8B0: 48000008  b 0x8303f8b8
	pc = 0x8303F8B8; continue 'dispatch;
	// 8303F8B4: 7F1EC378  mr r30, r24
	ctx.r[30].u64 = ctx.r[24].u64;
	// 8303F8B8: 389F0060  addi r4, r31, 0x60
	ctx.r[4].s64 = ctx.r[31].s64 + 96;
	// 8303F8BC: 807AB9B0  lwz r3, -0x4650(r26)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-18000 as u32) ) } as u64;
	// 8303F8C0: 4BFFE401  bl 0x8303dcc0
	ctx.lr = 0x8303F8C4;
	sub_8303DCC0(ctx, base);
	// 8303F8C4: 817AB9B0  lwz r11, -0x4650(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-18000 as u32) ) } as u64;
	// 8303F8C8: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 8303F8CC: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8303F8D0: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8303F8D4: 916B0038  stw r11, 0x38(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 8303F8D8: 817AB9B0  lwz r11, -0x4650(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-18000 as u32) ) } as u64;
	// 8303F8DC: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 8303F8E0: 817AB9B0  lwz r11, -0x4650(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-18000 as u32) ) } as u64;
	// 8303F8E4: 912B0020  stw r9, 0x20(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[9].u32 ) };
	// 8303F8E8: 807AB9B0  lwz r3, -0x4650(r26)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-18000 as u32) ) } as u64;
	// 8303F8EC: 4BFFECE5  bl 0x8303e5d0
	ctx.lr = 0x8303F8F0;
	sub_8303E5D0(ctx, base);
	// 8303F8F0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8303F8F4: 807AB9B0  lwz r3, -0x4650(r26)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-18000 as u32) ) } as u64;
	// 8303F8F8: 4BFFE511  bl 0x8303de08
	ctx.lr = 0x8303F8FC;
	sub_8303DE08(ctx, base);
	// 8303F8FC: 3D608304  lis r11, -0x7cfc
	ctx.r[11].s64 = -2096889856;
	// 8303F900: 3D408339  lis r10, -0x7cc7
	ctx.r[10].s64 = -2093416448;
	// 8303F904: 388BF5D8  addi r4, r11, -0xa28
	ctx.r[4].s64 = ctx.r[11].s64 + -2600;
	// 8303F908: 386AB9B4  addi r3, r10, -0x464c
	ctx.r[3].s64 = ctx.r[10].s64 + -17996;
	// 8303F90C: 4BFB822D  bl 0x82ff7b38
	ctx.lr = 0x8303F910;
	sub_82FF7B38(ctx, base);
	// 8303F910: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 8303F914: 99770000  stb r11, 0(r23)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[23].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 8303F918: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 8303F91C: 4BFB5EF5  bl 0x82ff5810
	ctx.lr = 0x8303F920;
	sub_82FF5810(ctx, base);
	// 8303F920: 807AB9B0  lwz r3, -0x4650(r26)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-18000 as u32) ) } as u64;
	// 8303F924: 383F01C0  addi r1, r31, 0x1c0
	ctx.r[1].s64 = ctx.r[31].s64 + 448;
	// 8303F928: 48168878  b 0x831a81a0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303F92C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8303F92C size=40
    let mut pc: u32 = 0x8303F92C;
    'dispatch: loop {
        match pc {
            0x8303F92C => {
    //   block [0x8303F92C..0x8303F954)
	// 8303F92C: 3BECFE40  addi r31, r12, -0x1c0
	ctx.r[31].s64 = ctx.r[12].s64 + -448;
	// 8303F930: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8303F934: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8303F938: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8303F93C: 387F0054  addi r3, r31, 0x54
	ctx.r[3].s64 = ctx.r[31].s64 + 84;
	// 8303F940: 4BFB5ED1  bl 0x82ff5810
	ctx.lr = 0x8303F944;
	sub_82FF5810(ctx, base);
	// 8303F944: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8303F948: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8303F94C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8303F950: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303F954(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8303F954 size=40
    let mut pc: u32 = 0x8303F954;
    'dispatch: loop {
        match pc {
            0x8303F954 => {
    //   block [0x8303F954..0x8303F97C)
	// 8303F954: 3BECFE40  addi r31, r12, -0x1c0
	ctx.r[31].s64 = ctx.r[12].s64 + -448;
	// 8303F958: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8303F95C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8303F960: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8303F964: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8303F968: 4BF98979  bl 0x82fd82e0
	ctx.lr = 0x8303F96C;
	sub_82FD82E0(ctx, base);
	// 8303F96C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8303F970: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8303F974: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8303F978: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303F97C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8303F97C size=40
    let mut pc: u32 = 0x8303F97C;
    'dispatch: loop {
        match pc {
            0x8303F97C => {
    //   block [0x8303F97C..0x8303F9A4)
	// 8303F97C: 3BECFE40  addi r31, r12, -0x1c0
	ctx.r[31].s64 = ctx.r[12].s64 + -448;
	// 8303F980: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8303F984: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8303F988: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8303F98C: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 8303F990: 4BFB5E81  bl 0x82ff5810
	ctx.lr = 0x8303F994;
	sub_82FF5810(ctx, base);
	// 8303F994: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8303F998: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8303F99C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8303F9A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303F9A4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8303F9A4 size=40
    let mut pc: u32 = 0x8303F9A4;
    'dispatch: loop {
        match pc {
            0x8303F9A4 => {
    //   block [0x8303F9A4..0x8303F9CC)
	// 8303F9A4: 3BECFE40  addi r31, r12, -0x1c0
	ctx.r[31].s64 = ctx.r[12].s64 + -448;
	// 8303F9A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8303F9AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8303F9B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8303F9B4: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 8303F9B8: 4BF98929  bl 0x82fd82e0
	ctx.lr = 0x8303F9BC;
	sub_82FD82E0(ctx, base);
	// 8303F9BC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8303F9C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8303F9C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8303F9C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303F9CC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8303F9CC size=40
    let mut pc: u32 = 0x8303F9CC;
    'dispatch: loop {
        match pc {
            0x8303F9CC => {
    //   block [0x8303F9CC..0x8303F9F4)
	// 8303F9CC: 3BECFE40  addi r31, r12, -0x1c0
	ctx.r[31].s64 = ctx.r[12].s64 + -448;
	// 8303F9D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8303F9D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8303F9D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8303F9DC: 807F0058  lwz r3, 0x58(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 8303F9E0: 4BF98901  bl 0x82fd82e0
	ctx.lr = 0x8303F9E4;
	sub_82FD82E0(ctx, base);
	// 8303F9E4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8303F9E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8303F9EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8303F9F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303F9F4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8303F9F4 size=40
    let mut pc: u32 = 0x8303F9F4;
    'dispatch: loop {
        match pc {
            0x8303F9F4 => {
    //   block [0x8303F9F4..0x8303FA1C)
	// 8303F9F4: 3BECFE40  addi r31, r12, -0x1c0
	ctx.r[31].s64 = ctx.r[12].s64 + -448;
	// 8303F9F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8303F9FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8303FA00: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8303FA04: 807F0058  lwz r3, 0x58(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 8303FA08: 4BF988D9  bl 0x82fd82e0
	ctx.lr = 0x8303FA0C;
	sub_82FD82E0(ctx, base);
	// 8303FA0C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8303FA10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8303FA14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8303FA18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303FA20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8303FA20 size=64
    let mut pc: u32 = 0x8303FA20;
    'dispatch: loop {
        match pc {
            0x8303FA20 => {
    //   block [0x8303FA20..0x8303FA60)
	// 8303FA20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8303FA24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8303FA28: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8303FA2C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8303FA30: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8303FA34: 817F0054  lwz r11, 0x54(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 8303FA38: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8303FA3C: 409A000C  bne cr6, 0x8303fa48
	if !ctx.cr[6].eq {
	pc = 0x8303FA48; continue 'dispatch;
	}
	// 8303FA40: 4BFFEC61  bl 0x8303e6a0
	ctx.lr = 0x8303FA44;
	sub_8303E6A0(ctx, base);
	// 8303FA44: 907F0054  stw r3, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 8303FA48: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 8303FA4C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8303FA50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8303FA54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8303FA58: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8303FA5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303FA60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8303FA60 size=8
    let mut pc: u32 = 0x8303FA60;
    'dispatch: loop {
        match pc {
            0x8303FA60 => {
    //   block [0x8303FA60..0x8303FA68)
	// 8303FA60: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8303FA64: 8215DD90  lwz r16, -0x2270(r21)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-8816 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303FA68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8303FA68 size=208
    let mut pc: u32 = 0x8303FA68;
    'dispatch: loop {
        match pc {
            0x8303FA68 => {
    //   block [0x8303FA68..0x8303FB38)
	// 8303FA68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8303FA6C: 48168701  bl 0x831a816c
	ctx.lr = 0x8303FA70;
	sub_831A8130(ctx, base);
	// 8303FA70: 3BE1FF60  addi r31, r1, -0xa0
	ctx.r[31].s64 = ctx.r[1].s64 + -160;
	// 8303FA74: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8303FA78: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8303FA7C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8303FA80: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8303FA84: 817E0020  lwz r11, 0x20(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 8303FA88: 93DF00B4  stw r30, 0xb4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(180 as u32), ctx.r[30].u32 ) };
	// 8303FA8C: 2F0B0005  cmpwi cr6, r11, 5
	ctx.cr[6].compare_i32(ctx.r[11].s32, 5, &mut ctx.xer);
	// 8303FA90: 419A0040  beq cr6, 0x8303fad0
	if ctx.cr[6].eq {
	pc = 0x8303FAD0; continue 'dispatch;
	}
	// 8303FA94: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 8303FA98: 409A0040  bne cr6, 0x8303fad8
	if !ctx.cr[6].eq {
	pc = 0x8303FAD8; continue 'dispatch;
	}
	// 8303FA9C: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 8303FAA0: 809E0068  lwz r4, 0x68(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(104 as u32) ) } as u64;
	// 8303FAA4: 4BF987F5  bl 0x82fd8298
	ctx.lr = 0x8303FAA8;
	sub_82FD8298(ctx, base);
	// 8303FAA8: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 8303FAAC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8303FAB0: 4182001C  beq 0x8303facc
	if ctx.cr[0].eq {
	pc = 0x8303FACC; continue 'dispatch;
	}
	// 8303FAB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8303FAB8: 80FE0068  lwz r7, 0x68(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(104 as u32) ) } as u64;
	// 8303FABC: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8303FAC0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8303FAC4: 4804790D  bl 0x830873d0
	ctx.lr = 0x8303FAC8;
	sub_830873D0(ctx, base);
	// 8303FAC8: 48000008  b 0x8303fad0
	pc = 0x8303FAD0; continue 'dispatch;
	// 8303FACC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8303FAD0: 383F00A0  addi r1, r31, 0xa0
	ctx.r[1].s64 = ctx.r[31].s64 + 160;
	// 8303FAD4: 481686E8  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 8303FAD8: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 8303FADC: 409A0014  bne cr6, 0x8303faf0
	if !ctx.cr[6].eq {
	pc = 0x8303FAF0; continue 'dispatch;
	}
	// 8303FAE0: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8303FAE4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8303FAE8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8303FAEC: 48000018  b 0x8303fb04
	pc = 0x8303FB04; continue 'dispatch;
	// 8303FAF0: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 8303FAF4: 409A0018  bne cr6, 0x8303fb0c
	if !ctx.cr[6].eq {
	pc = 0x8303FB0C; continue 'dispatch;
	}
	// 8303FAF8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8303FAFC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8303FB00: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8303FB04: 4BFFEC8D  bl 0x8303e790
	ctx.lr = 0x8303FB08;
	sub_8303E790(ctx, base);
	// 8303FB08: 4BFFFFC8  b 0x8303fad0
	pc = 0x8303FAD0; continue 'dispatch;
	// 8303FB0C: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 8303FB10: 80FE0068  lwz r7, 0x68(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(104 as u32) ) } as u64;
	// 8303FB14: 38C00011  li r6, 0x11
	ctx.r[6].s64 = 17;
	// 8303FB18: 388BDBA0  addi r4, r11, -0x2460
	ctx.r[4].s64 = ctx.r[11].s64 + -9312;
	// 8303FB1C: 38A00276  li r5, 0x276
	ctx.r[5].s64 = 630;
	// 8303FB20: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 8303FB24: 4BF91535  bl 0x82fd1058
	ctx.lr = 0x8303FB28;
	sub_82FD1058(ctx, base);
	// 8303FB28: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 8303FB2C: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 8303FB30: 388BC3EC  addi r4, r11, -0x3c14
	ctx.r[4].s64 = ctx.r[11].s64 + -15380;
	// 8303FB34: 481710F5  bl 0x831b0c28
	ctx.lr = 0x8303FB38;
	sub_831B0C28(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303FB38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8303FB38 size=48
    let mut pc: u32 = 0x8303FB38;
    'dispatch: loop {
        match pc {
            0x8303FB38 => {
    //   block [0x8303FB38..0x8303FB68)
	// 8303FB38: 3BECFF60  addi r31, r12, -0xa0
	ctx.r[31].s64 = ctx.r[12].s64 + -160;
	// 8303FB3C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8303FB40: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8303FB44: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8303FB48: 817F00B4  lwz r11, 0xb4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(180 as u32) ) } as u64;
	// 8303FB4C: 808B0068  lwz r4, 0x68(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(104 as u32) ) } as u64;
	// 8303FB50: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8303FB54: 4BF9878D  bl 0x82fd82e0
	ctx.lr = 0x8303FB58;
	sub_82FD82E0(ctx, base);
	// 8303FB58: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8303FB5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8303FB60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8303FB64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303FB68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8303FB68 size=408
    let mut pc: u32 = 0x8303FB68;
    'dispatch: loop {
        match pc {
            0x8303FB68 => {
    //   block [0x8303FB68..0x8303FD00)
	// 8303FB68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8303FB6C: 481685E9  bl 0x831a8154
	ctx.lr = 0x8303FB70;
	sub_831A8130(ctx, base);
	// 8303FB70: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8303FB74: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8303FB78: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8303FB7C: 7CB72B78  mr r23, r5
	ctx.r[23].u64 = ctx.r[5].u64;
	// 8303FB80: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8303FB84: 409A000C  bne cr6, 0x8303fb90
	if !ctx.cr[6].eq {
	pc = 0x8303FB90; continue 'dispatch;
	}
	// 8303FB88: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8303FB8C: 4800016C  b 0x8303fcf8
	pc = 0x8303FCF8; continue 'dispatch;
	// 8303FB90: 83BF0018  lwz r29, 0x18(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 8303FB94: 56EB063F  clrlwi. r11, r23, 0x18
	ctx.r[11].u64 = ctx.r[23].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8303FB98: 41820058  beq 0x8303fbf0
	if ctx.cr[0].eq {
	pc = 0x8303FBF0; continue 'dispatch;
	}
	// 8303FB9C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8303FBA0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8303FBA4: 419A004C  beq cr6, 0x8303fbf0
	if ctx.cr[6].eq {
	pc = 0x8303FBF0; continue 'dispatch;
	}
	// 8303FBA8: 817E0060  lwz r11, 0x60(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(96 as u32) ) } as u64;
	// 8303FBAC: 815E005C  lwz r10, 0x5c(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(92 as u32) ) } as u64;
	// 8303FBB0: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8303FBB4: 409A000C  bne cr6, 0x8303fbc0
	if !ctx.cr[6].eq {
	pc = 0x8303FBC0; continue 'dispatch;
	}
	// 8303FBB8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8303FBBC: 4BFFF56D  bl 0x8303f128
	ctx.lr = 0x8303FBC0;
	sub_8303F128(ctx, base);
	// 8303FBC0: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8303FBC4: 815E005C  lwz r10, 0x5c(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(92 as u32) ) } as u64;
	// 8303FBC8: 813E0058  lwz r9, 0x58(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(88 as u32) ) } as u64;
	// 8303FBCC: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8303FBD0: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 8303FBD4: 7D6A492E  stwx r11, r10, r9
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32), ctx.r[11].u32) };
	// 8303FBD8: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8303FBDC: 815E005C  lwz r10, 0x5c(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(92 as u32) ) } as u64;
	// 8303FBE0: 914B0020  stw r10, 0x20(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 8303FBE4: 817E005C  lwz r11, 0x5c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(92 as u32) ) } as u64;
	// 8303FBE8: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8303FBEC: 917E005C  stw r11, 0x5c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8303FBF0: 57AB073E  clrlwi r11, r29, 0x1c
	ctx.r[11].u64 = ctx.r[29].u32 as u64 & 0x0000000Fu64;
	// 8303FBF4: 833F0020  lwz r25, 0x20(r31)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 8303FBF8: 831F0024  lwz r24, 0x24(r31)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 8303FBFC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8303FC00: 2F0B0006  cmpwi cr6, r11, 6
	ctx.cr[6].compare_i32(ctx.r[11].s32, 6, &mut ctx.xer);
	// 8303FC04: 419A00E0  beq cr6, 0x8303fce4
	if ctx.cr[6].eq {
	pc = 0x8303FCE4; continue 'dispatch;
	}
	// 8303FC08: 2F0B0007  cmpwi cr6, r11, 7
	ctx.cr[6].compare_i32(ctx.r[11].s32, 7, &mut ctx.xer);
	// 8303FC0C: 419A00D8  beq cr6, 0x8303fce4
	if ctx.cr[6].eq {
	pc = 0x8303FCE4; continue 'dispatch;
	}
	// 8303FC10: 2F0B0008  cmpwi cr6, r11, 8
	ctx.cr[6].compare_i32(ctx.r[11].s32, 8, &mut ctx.xer);
	// 8303FC14: 419A00D0  beq cr6, 0x8303fce4
	if ctx.cr[6].eq {
	pc = 0x8303FCE4; continue 'dispatch;
	}
	// 8303FC18: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 8303FC1C: 419A00C8  beq cr6, 0x8303fce4
	if ctx.cr[6].eq {
	pc = 0x8303FCE4; continue 'dispatch;
	}
	// 8303FC20: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 8303FC24: 419A0014  beq cr6, 0x8303fc38
	if ctx.cr[6].eq {
	pc = 0x8303FC38; continue 'dispatch;
	}
	// 8303FC28: 2F1D0009  cmpwi cr6, r29, 9
	ctx.cr[6].compare_i32(ctx.r[29].s32, 9, &mut ctx.xer);
	// 8303FC2C: 419A000C  beq cr6, 0x8303fc38
	if ctx.cr[6].eq {
	pc = 0x8303FC38; continue 'dispatch;
	}
	// 8303FC30: 2F0B0005  cmpwi cr6, r11, 5
	ctx.cr[6].compare_i32(ctx.r[11].s32, 5, &mut ctx.xer);
	// 8303FC34: 409A00C4  bne cr6, 0x8303fcf8
	if !ctx.cr[6].eq {
	pc = 0x8303FCF8; continue 'dispatch;
	}
	// 8303FC38: 83BF0010  lwz r29, 0x10(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8303FC3C: 7EE5BB78  mr r5, r23
	ctx.r[5].u64 = ctx.r[23].u64;
	// 8303FC40: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8303FC44: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8303FC48: 4BFFFF21  bl 0x8303fb68
	ctx.lr = 0x8303FC4C;
	sub_8303FB68(ctx, base);
	// 8303FC4C: 839F0014  lwz r28, 0x14(r31)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 8303FC50: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8303FC54: 281C0000  cmplwi r28, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8303FC58: 40820040  bne 0x8303fc98
	if !ctx.cr[0].eq {
	pc = 0x8303FC98; continue 'dispatch;
	}
	// 8303FC5C: 7F06C378  mr r6, r24
	ctx.r[6].u64 = ctx.r[24].u64;
	// 8303FC60: 7F25CB78  mr r5, r25
	ctx.r[5].u64 = ctx.r[25].u64;
	// 8303FC64: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8303FC68: 4BFFF009  bl 0x8303ec70
	ctx.lr = 0x8303FC6C;
	sub_8303EC70(ctx, base);
	// 8303FC6C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8303FC70: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8303FC74: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8303FC78: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8303FC7C: 997F001C  stb r11, 0x1c(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u8 ) };
	// 8303FC80: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303FC84: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303FC88: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8303FC8C: 4E800421  bctrl
	ctx.lr = 0x8303FC90;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8303FC90: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8303FC94: 48000064  b 0x8303fcf8
	pc = 0x8303FCF8; continue 'dispatch;
	// 8303FC98: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 8303FC9C: 3B600001  li r27, 1
	ctx.r[27].s64 = 1;
	// 8303FCA0: 7F04E840  cmplw cr6, r4, r29
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[29].u32, &mut ctx.xer);
	// 8303FCA4: 419A0014  beq cr6, 0x8303fcb8
	if ctx.cr[6].eq {
	pc = 0x8303FCB8; continue 'dispatch;
	}
	// 8303FCA8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8303FCAC: 9B5F001C  stb r26, 0x1c(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[26].u8 ) };
	// 8303FCB0: 4BFFE4C9  bl 0x8303e178
	ctx.lr = 0x8303FCB4;
	sub_8303E178(ctx, base);
	// 8303FCB4: 9B7F001C  stb r27, 0x1c(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[27].u8 ) };
	// 8303FCB8: 7EE5BB78  mr r5, r23
	ctx.r[5].u64 = ctx.r[23].u64;
	// 8303FCBC: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8303FCC0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8303FCC4: 4BFFFEA5  bl 0x8303fb68
	ctx.lr = 0x8303FCC8;
	sub_8303FB68(ctx, base);
	// 8303FCC8: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8303FCCC: 7F04E040  cmplw cr6, r4, r28
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[28].u32, &mut ctx.xer);
	// 8303FCD0: 419A0014  beq cr6, 0x8303fce4
	if ctx.cr[6].eq {
	pc = 0x8303FCE4; continue 'dispatch;
	}
	// 8303FCD4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8303FCD8: 9B5F001D  stb r26, 0x1d(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(29 as u32), ctx.r[26].u8 ) };
	// 8303FCDC: 4BFFE505  bl 0x8303e1e0
	ctx.lr = 0x8303FCE0;
	sub_8303E1E0(ctx, base);
	// 8303FCE0: 9B7F001D  stb r27, 0x1d(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(29 as u32), ctx.r[27].u8 ) };
	// 8303FCE4: 7F06C378  mr r6, r24
	ctx.r[6].u64 = ctx.r[24].u64;
	// 8303FCE8: 7F25CB78  mr r5, r25
	ctx.r[5].u64 = ctx.r[25].u64;
	// 8303FCEC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8303FCF0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8303FCF4: 4BFFEF7D  bl 0x8303ec70
	ctx.lr = 0x8303FCF8;
	sub_8303EC70(ctx, base);
	// 8303FCF8: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 8303FCFC: 481684A8  b 0x831a81a4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303FD00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8303FD00 size=188
    let mut pc: u32 = 0x8303FD00;
    'dispatch: loop {
        match pc {
            0x8303FD00 => {
    //   block [0x8303FD00..0x8303FDBC)
	// 8303FD00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8303FD04: 48168461  bl 0x831a8164
	ctx.lr = 0x8303FD08;
	sub_831A8130(ctx, base);
	// 8303FD08: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8303FD0C: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 8303FD10: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 8303FD14: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8303FD18: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8303FD1C: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 8303FD20: 48028EE1  bl 0x83068c00
	ctx.lr = 0x8303FD24;
	sub_83068C00(ctx, base);
	// 8303FD24: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8303FD28: 41820040  beq 0x8303fd68
	if ctx.cr[0].eq {
	pc = 0x8303FD68; continue 'dispatch;
	}
	// 8303FD2C: 897F0004  lbz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8303FD30: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8303FD34: 41820024  beq 0x8303fd58
	if ctx.cr[0].eq {
	pc = 0x8303FD58; continue 'dispatch;
	}
	// 8303FD38: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303FD3C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8303FD40: 41820018  beq 0x8303fd58
	if ctx.cr[0].eq {
	pc = 0x8303FD58; continue 'dispatch;
	}
	// 8303FD44: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303FD48: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8303FD4C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303FD50: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8303FD54: 4E800421  bctrl
	ctx.lr = 0x8303FD58;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8303FD58: 937E0000  stw r27, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 8303FD5C: 93BE0008  stw r29, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 8303FD60: 939E000C  stw r28, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[28].u32 ) };
	// 8303FD64: 48000050  b 0x8303fdb4
	pc = 0x8303FDB4; continue 'dispatch;
	// 8303FD68: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 8303FD6C: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303FD70: 4BF98529  bl 0x82fd8298
	ctx.lr = 0x8303FD74;
	sub_82FD8298(ctx, base);
	// 8303FD74: 81210050  lwz r9, 0x50(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8303FD78: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8303FD7C: 41820028  beq 0x8303fda4
	if ctx.cr[0].eq {
	pc = 0x8303FDA4; continue 'dispatch;
	}
	// 8303FD80: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8303FD84: 5528103A  slwi r8, r9, 2
	ctx.r[8].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 8303FD88: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 8303FD8C: 7D68582E  lwzx r11, r8, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8303FD90: 93630000  stw r27, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 8303FD94: 93A30008  stw r29, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 8303FD98: 9383000C  stw r28, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[28].u32 ) };
	// 8303FD9C: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8303FDA0: 48000008  b 0x8303fda8
	pc = 0x8303FDA8; continue 'dispatch;
	// 8303FDA4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8303FDA8: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8303FDAC: 5529103A  slwi r9, r9, 2
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 8303FDB0: 7D49592E  stwx r10, r9, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32), ctx.r[10].u32) };
	// 8303FDB4: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8303FDB8: 481683FC  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303FDC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8303FDC0 size=8
    let mut pc: u32 = 0x8303FDC0;
    'dispatch: loop {
        match pc {
            0x8303FDC0 => {
    //   block [0x8303FDC0..0x8303FDC8)
	// 8303FDC0: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8303FDC4: 8215DDF0  lwz r16, -0x2210(r21)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-8720 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303FDC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8303FDC8 size=164
    let mut pc: u32 = 0x8303FDC8;
    'dispatch: loop {
        match pc {
            0x8303FDC8 => {
    //   block [0x8303FDC8..0x8303FE6C)
	// 8303FDC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8303FDCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8303FDD0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8303FDD4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8303FDD8: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 8303FDDC: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8303FDE0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8303FDE4: 7CC73378  mr r7, r6
	ctx.r[7].u64 = ctx.r[6].u64;
	// 8303FDE8: 93DF0094  stw r30, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[30].u32 ) };
	// 8303FDEC: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 8303FDF0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8303FDF4: 98BE0004  stb r5, 4(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[5].u8 ) };
	// 8303FDF8: 396BDDD8  addi r11, r11, -0x2228
	ctx.r[11].s64 = ctx.r[11].s64 + -8744;
	// 8303FDFC: 909E0010  stw r4, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[4].u32 ) };
	// 8303FE00: 3920FFFF  li r9, -1
	ctx.r[9].s64 = -1;
	// 8303FE04: 90FE0014  stw r7, 0x14(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), ctx.r[7].u32 ) };
	// 8303FE08: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 8303FE0C: 915E0008  stw r10, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8303FE10: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8303FE14: 913E000C  stw r9, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 8303FE18: 915E0018  stw r10, 0x18(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), ctx.r[10].u32 ) };
	// 8303FE1C: 409A002C  bne cr6, 0x8303fe48
	if !ctx.cr[6].eq {
	pc = 0x8303FE48; continue 'dispatch;
	}
	// 8303FE20: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8303FE24: 38C0000F  li r6, 0xf
	ctx.r[6].s64 = 15;
	// 8303FE28: 388BAB5C  addi r4, r11, -0x54a4
	ctx.r[4].s64 = ctx.r[11].s64 + -21668;
	// 8303FE2C: 38A001B8  li r5, 0x1b8
	ctx.r[5].s64 = 440;
	// 8303FE30: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 8303FE34: 4BFA10BD  bl 0x82fe0ef0
	ctx.lr = 0x8303FE38;
	sub_82FE0EF0(ctx, base);
	// 8303FE38: 3D608225  lis r11, -0x7ddb
	ctx.r[11].s64 = -2111504384;
	// 8303FE3C: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 8303FE40: 388BC654  addi r4, r11, -0x39ac
	ctx.r[4].s64 = ctx.r[11].s64 + -14764;
	// 8303FE44: 48170DE5  bl 0x831b0c28
	ctx.lr = 0x8303FE48;
	sub_831B0C28(ctx, base);
	// 8303FE48: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8303FE4C: 4BFA256D  bl 0x82fe23b8
	ctx.lr = 0x8303FE50;
	sub_82FE23B8(ctx, base);
	// 8303FE50: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8303FE54: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 8303FE58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8303FE5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8303FE60: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8303FE64: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8303FE68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303FE6C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8303FE6C size=40
    let mut pc: u32 = 0x8303FE6C;
    'dispatch: loop {
        match pc {
            0x8303FE6C => {
    //   block [0x8303FE6C..0x8303FE94)
	// 8303FE6C: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 8303FE70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8303FE74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8303FE78: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8303FE7C: 807F0094  lwz r3, 0x94(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 8303FE80: 4800C8E1  bl 0x8304c760
	ctx.lr = 0x8303FE84;
	sub_8304C760(ctx, base);
	// 8303FE84: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8303FE88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8303FE8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8303FE90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303FE98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8303FE98 size=116
    let mut pc: u32 = 0x8303FE98;
    'dispatch: loop {
        match pc {
            0x8303FE98 => {
    //   block [0x8303FE98..0x8303FF0C)
	// 8303FE98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8303FE9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8303FEA0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8303FEA4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8303FEA8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8303FEAC: 809F0018  lwz r4, 0x18(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 8303FEB0: 28040000  cmplwi r4, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8303FEB4: 4182002C  beq 0x8303fee0
	if ctx.cr[0].eq {
	pc = 0x8303FEE0; continue 'dispatch;
	}
	// 8303FEB8: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8303FEBC: 80DF0014  lwz r6, 0x14(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 8303FEC0: 806B0010  lwz r3, 0x10(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 8303FEC4: 80AB000C  lwz r5, 0xc(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8303FEC8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303FECC: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8303FED0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8303FED4: 4E800421  bctrl
	ctx.lr = 0x8303FED8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8303FED8: 907F000C  stw r3, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[3].u32 ) };
	// 8303FEDC: 4800000C  b 0x8303fee8
	pc = 0x8303FEE8; continue 'dispatch;
	// 8303FEE0: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 8303FEE4: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 8303FEE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8303FEEC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8303FEF0: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8303FEF4: 4BFA24C5  bl 0x82fe23b8
	ctx.lr = 0x8303FEF8;
	sub_82FE23B8(ctx, base);
	// 8303FEF8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8303FEFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8303FF00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8303FF04: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8303FF08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303FF10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8303FF10 size=8
    let mut pc: u32 = 0x8303FF10;
    'dispatch: loop {
        match pc {
            0x8303FF10 => {
    //   block [0x8303FF10..0x8303FF18)
	// 8303FF10: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8303FF14: 8215DE28  lwz r16, -0x21d8(r21)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-8664 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303FF18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8303FF18 size=108
    let mut pc: u32 = 0x8303FF18;
    'dispatch: loop {
        match pc {
            0x8303FF18 => {
    //   block [0x8303FF18..0x8303FF84)
	// 8303FF18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8303FF1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8303FF20: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8303FF24: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8303FF28: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 8303FF2C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8303FF30: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8303FF34: 38600014  li r3, 0x14
	ctx.r[3].s64 = 20;
	// 8303FF38: 809E0068  lwz r4, 0x68(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(104 as u32) ) } as u64;
	// 8303FF3C: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 8303FF40: 4BF98359  bl 0x82fd8298
	ctx.lr = 0x8303FF44;
	sub_82FD8298(ctx, base);
	// 8303FF44: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 8303FF48: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8303FF4C: 41820018  beq 0x8303ff64
	if ctx.cr[0].eq {
	pc = 0x8303FF64; continue 'dispatch;
	}
	// 8303FF50: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8303FF54: 80DE0068  lwz r6, 0x68(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(104 as u32) ) } as u64;
	// 8303FF58: 3880001D  li r4, 0x1d
	ctx.r[4].s64 = 29;
	// 8303FF5C: 4BFFB255  bl 0x8303b1b0
	ctx.lr = 0x8303FF60;
	sub_8303B1B0(ctx, base);
	// 8303FF60: 48000008  b 0x8303ff68
	pc = 0x8303FF68; continue 'dispatch;
	// 8303FF64: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8303FF68: 907E004C  stw r3, 0x4c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(76 as u32), ctx.r[3].u32 ) };
	// 8303FF6C: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 8303FF70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8303FF74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8303FF78: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8303FF7C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8303FF80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303FF84(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8303FF84 size=48
    let mut pc: u32 = 0x8303FF84;
    'dispatch: loop {
        match pc {
            0x8303FF84 => {
    //   block [0x8303FF84..0x8303FFB4)
	// 8303FF84: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 8303FF88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8303FF8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8303FF90: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8303FF94: 817F0084  lwz r11, 0x84(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 8303FF98: 808B0068  lwz r4, 0x68(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(104 as u32) ) } as u64;
	// 8303FF9C: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8303FFA0: 4BF98341  bl 0x82fd82e0
	ctx.lr = 0x8303FFA4;
	sub_82FD82E0(ctx, base);
	// 8303FFA4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8303FFA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8303FFAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8303FFB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303FFB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8303FFB8 size=8
    let mut pc: u32 = 0x8303FFB8;
    'dispatch: loop {
        match pc {
            0x8303FFB8 => {
    //   block [0x8303FFB8..0x8303FFC0)
	// 8303FFB8: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8303FFBC: 8215DE60  lwz r16, -0x21a0(r21)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-8608 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8303FFC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8303FFC0 size=192
    let mut pc: u32 = 0x8303FFC0;
    'dispatch: loop {
        match pc {
            0x8303FFC0 => {
    //   block [0x8303FFC0..0x83040080)
	// 8303FFC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8303FFC4: 481681A5  bl 0x831a8168
	ctx.lr = 0x8303FFC8;
	sub_831A8130(ctx, base);
	// 8303FFC8: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 8303FFCC: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8303FFD0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8303FFD4: 38600028  li r3, 0x28
	ctx.r[3].s64 = 40;
	// 8303FFD8: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 8303FFDC: 809E0068  lwz r4, 0x68(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(104 as u32) ) } as u64;
	// 8303FFE0: 93DF0094  stw r30, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[30].u32 ) };
	// 8303FFE4: 4BF982B5  bl 0x82fd8298
	ctx.lr = 0x8303FFE8;
	sub_82FD8298(ctx, base);
	// 8303FFE8: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 8303FFEC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8303FFF0: 41820014  beq 0x83040004
	if ctx.cr[0].eq {
	pc = 0x83040004; continue 'dispatch;
	}
	// 8303FFF4: 809E003C  lwz r4, 0x3c(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(60 as u32) ) } as u64;
	// 8303FFF8: 48042719  bl 0x83082710
	ctx.lr = 0x8303FFFC;
	sub_83082710(ctx, base);
	// 8303FFFC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83040000: 48000008  b 0x83040008
	pc = 0x83040008; continue 'dispatch;
	// 83040004: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 83040008: 578B063F  clrlwi. r11, r28, 0x18
	ctx.r[11].u64 = ctx.r[28].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8304000C: 41820024  beq 0x83040030
	if ctx.cr[0].eq {
	pc = 0x83040030; continue 'dispatch;
	}
	// 83040010: 807E0068  lwz r3, 0x68(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(104 as u32) ) } as u64;
	// 83040014: 817E0060  lwz r11, 0x60(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(96 as u32) ) } as u64;
	// 83040018: 5564103A  slwi r4, r11, 2
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 8304001C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83040020: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83040024: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83040028: 4E800421  bctrl
	ctx.lr = 0x8304002C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8304002C: 907E0058  stw r3, 0x58(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(88 as u32), ctx.r[3].u32 ) };
	// 83040030: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 83040034: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83040038: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8304003C: 4BFFFB2D  bl 0x8303fb68
	ctx.lr = 0x83040040;
	sub_8303FB68(ctx, base);
	// 83040040: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83040044: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83040048: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8304004C: 4BFFFA1D  bl 0x8303fa68
	ctx.lr = 0x83040050;
	sub_8303FA68(ctx, base);
	// 83040050: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83040054: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 83040058: 419A001C  beq cr6, 0x83040074
	if ctx.cr[6].eq {
	pc = 0x83040074; continue 'dispatch;
	}
	// 8304005C: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 83040060: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83040064: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83040068: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304006C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83040070: 4E800421  bctrl
	ctx.lr = 0x83040074;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83040074: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83040078: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 8304007C: 4816813C  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83040080(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83040080 size=48
    let mut pc: u32 = 0x83040080;
    'dispatch: loop {
        match pc {
            0x83040080 => {
    //   block [0x83040080..0x830400B0)
	// 83040080: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 83040084: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83040088: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8304008C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83040090: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 83040094: 808B0068  lwz r4, 0x68(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(104 as u32) ) } as u64;
	// 83040098: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8304009C: 4BF98245  bl 0x82fd82e0
	ctx.lr = 0x830400A0;
	sub_82FD82E0(ctx, base);
	// 830400A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830400A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830400A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830400AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830400B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830400B0 size=160
    let mut pc: u32 = 0x830400B0;
    'dispatch: loop {
        match pc {
            0x830400B0 => {
    //   block [0x830400B0..0x83040150)
	// 830400B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830400B4: 481680B1  bl 0x831a8164
	ctx.lr = 0x830400B8;
	sub_831A8130(ctx, base);
	// 830400B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830400BC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830400C0: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 830400C4: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 830400C8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830400CC: 4099007C  ble cr6, 0x83040148
	if !ctx.cr[6].gt {
	pc = 0x83040148; continue 'dispatch;
	}
	// 830400D0: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 830400D4: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 830400D8: 7FEBE82E  lwzx r31, r11, r29
	ctx.r[31].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 830400DC: 281F0000  cmplwi r31, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830400E0: 41820048  beq 0x83040128
	if ctx.cr[0].eq {
	pc = 0x83040128; continue 'dispatch;
	}
	// 830400E4: 897E0004  lbz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830400E8: 839F0004  lwz r28, 4(r31)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830400EC: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830400F0: 41820024  beq 0x83040114
	if ctx.cr[0].eq {
	pc = 0x83040114; continue 'dispatch;
	}
	// 830400F4: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830400F8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830400FC: 41820018  beq 0x83040114
	if ctx.cr[0].eq {
	pc = 0x83040114; continue 'dispatch;
	}
	// 83040100: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83040104: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83040108: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304010C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83040110: 4E800421  bctrl
	ctx.lr = 0x83040114;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83040114: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83040118: 4BF981C9  bl 0x82fd82e0
	ctx.lr = 0x8304011C;
	sub_82FD82E0(ctx, base);
	// 8304011C: 7F9FE378  mr r31, r28
	ctx.r[31].u64 = ctx.r[28].u64;
	// 83040120: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 83040124: 409AFFC0  bne cr6, 0x830400e4
	if !ctx.cr[6].eq {
	pc = 0x830400E4; continue 'dispatch;
	}
	// 83040128: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8304012C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83040130: 3B7B0001  addi r27, r27, 1
	ctx.r[27].s64 = ctx.r[27].s64 + 1;
	// 83040134: 7D4BE92E  stwx r10, r11, r29
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[29].u32), ctx.r[10].u32) };
	// 83040138: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 8304013C: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 83040140: 7F1B5840  cmplw cr6, r27, r11
	ctx.cr[6].compare_u32(ctx.r[27].u32, ctx.r[11].u32, &mut ctx.xer);
	// 83040144: 4198FF90  blt cr6, 0x830400d4
	if ctx.cr[6].lt {
	pc = 0x830400D4; continue 'dispatch;
	}
	// 83040148: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8304014C: 48168068  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83040150(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83040150 size=8
    let mut pc: u32 = 0x83040150;
    'dispatch: loop {
        match pc {
            0x83040150 => {
    //   block [0x83040150..0x83040158)
	// 83040150: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83040154: 8215DEA8  lwz r16, -0x2158(r21)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-8536 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83040158(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83040158 size=156
    let mut pc: u32 = 0x83040158;
    'dispatch: loop {
        match pc {
            0x83040158 => {
    //   block [0x83040158..0x830401F4)
	// 83040158: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304015C: 48168011  bl 0x831a816c
	ctx.lr = 0x83040160;
	sub_831A8130(ctx, base);
	// 83040160: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 83040164: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83040168: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8304016C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 83040170: 817E004C  lwz r11, 0x4c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(76 as u32) ) } as u64;
	// 83040174: 93DF0094  stw r30, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[30].u32 ) };
	// 83040178: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8304017C: 409A0008  bne cr6, 0x83040184
	if !ctx.cr[6].eq {
	pc = 0x83040184; continue 'dispatch;
	}
	// 83040180: 4BFFFD99  bl 0x8303ff18
	ctx.lr = 0x83040184;
	sub_8303FF18(ctx, base);
	// 83040184: 815E001C  lwz r10, 0x1c(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 83040188: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 8304018C: 817D0028  lwz r11, 0x28(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(40 as u32) ) } as u64;
	// 83040190: 915D0024  stw r10, 0x24(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(36 as u32), ctx.r[10].u32 ) };
	// 83040194: 807E004C  lwz r3, 0x4c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(76 as u32) ) } as u64;
	// 83040198: 80AB0020  lwz r5, 0x20(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 8304019C: 808B0010  lwz r4, 0x10(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 830401A0: 4BFFFB61  bl 0x8303fd00
	ctx.lr = 0x830401A4;
	sub_8303FD00(ctx, base);
	// 830401A4: 817E0044  lwz r11, 0x44(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(68 as u32) ) } as u64;
	// 830401A8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830401AC: 409A0034  bne cr6, 0x830401e0
	if !ctx.cr[6].eq {
	pc = 0x830401E0; continue 'dispatch;
	}
	// 830401B0: 3860001C  li r3, 0x1c
	ctx.r[3].s64 = 28;
	// 830401B4: 809E0068  lwz r4, 0x68(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(104 as u32) ) } as u64;
	// 830401B8: 4BF980E1  bl 0x82fd8298
	ctx.lr = 0x830401BC;
	sub_82FD8298(ctx, base);
	// 830401BC: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 830401C0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830401C4: 41820014  beq 0x830401d8
	if ctx.cr[0].eq {
	pc = 0x830401D8; continue 'dispatch;
	}
	// 830401C8: 80BE0068  lwz r5, 0x68(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(104 as u32) ) } as u64;
	// 830401CC: 809E004C  lwz r4, 0x4c(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(76 as u32) ) } as u64;
	// 830401D0: 48042229  bl 0x830823f8
	ctx.lr = 0x830401D4;
	sub_830823F8(ctx, base);
	// 830401D4: 48000008  b 0x830401dc
	pc = 0x830401DC; continue 'dispatch;
	// 830401D8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830401DC: 907E0044  stw r3, 0x44(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(68 as u32), ctx.r[3].u32 ) };
	// 830401E0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 830401E4: 807E0044  lwz r3, 0x44(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(68 as u32) ) } as u64;
	// 830401E8: 4BFFDCF9  bl 0x8303dee0
	ctx.lr = 0x830401EC;
	sub_8303DEE0(ctx, base);
	// 830401EC: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 830401F0: 48167FCC  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830401F4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830401F4 size=48
    let mut pc: u32 = 0x830401F4;
    'dispatch: loop {
        match pc {
            0x830401F4 => {
    //   block [0x830401F4..0x83040224)
	// 830401F4: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 830401F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830401FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83040200: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83040204: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 83040208: 808B0068  lwz r4, 0x68(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(104 as u32) ) } as u64;
	// 8304020C: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 83040210: 4BF980D1  bl 0x82fd82e0
	ctx.lr = 0x83040214;
	sub_82FD82E0(ctx, base);
	// 83040214: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83040218: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8304021C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83040220: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83040228(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83040228 size=8
    let mut pc: u32 = 0x83040228;
    'dispatch: loop {
        match pc {
            0x83040228 => {
    //   block [0x83040228..0x83040230)
	// 83040228: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8304022C: 8215DEF0  lwz r16, -0x2110(r21)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-8464 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83040230(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83040230 size=156
    let mut pc: u32 = 0x83040230;
    'dispatch: loop {
        match pc {
            0x83040230 => {
    //   block [0x83040230..0x830402CC)
	// 83040230: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83040234: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83040238: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8304023C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83040240: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 83040244: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83040248: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8304024C: 817E0044  lwz r11, 0x44(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(68 as u32) ) } as u64;
	// 83040250: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 83040254: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83040258: 409A0044  bne cr6, 0x8304029c
	if !ctx.cr[6].eq {
	pc = 0x8304029C; continue 'dispatch;
	}
	// 8304025C: 817E004C  lwz r11, 0x4c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(76 as u32) ) } as u64;
	// 83040260: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83040264: 409A0008  bne cr6, 0x8304026c
	if !ctx.cr[6].eq {
	pc = 0x8304026C; continue 'dispatch;
	}
	// 83040268: 4BFFFCB1  bl 0x8303ff18
	ctx.lr = 0x8304026C;
	sub_8303FF18(ctx, base);
	// 8304026C: 3860001C  li r3, 0x1c
	ctx.r[3].s64 = 28;
	// 83040270: 809E0068  lwz r4, 0x68(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(104 as u32) ) } as u64;
	// 83040274: 4BF98025  bl 0x82fd8298
	ctx.lr = 0x83040278;
	sub_82FD8298(ctx, base);
	// 83040278: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 8304027C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83040280: 41820014  beq 0x83040294
	if ctx.cr[0].eq {
	pc = 0x83040294; continue 'dispatch;
	}
	// 83040284: 80BE0068  lwz r5, 0x68(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(104 as u32) ) } as u64;
	// 83040288: 809E004C  lwz r4, 0x4c(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(76 as u32) ) } as u64;
	// 8304028C: 4804216D  bl 0x830823f8
	ctx.lr = 0x83040290;
	sub_830823F8(ctx, base);
	// 83040290: 48000008  b 0x83040298
	pc = 0x83040298; continue 'dispatch;
	// 83040294: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83040298: 907E0044  stw r3, 0x44(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(68 as u32), ctx.r[3].u32 ) };
	// 8304029C: 807E0044  lwz r3, 0x44(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(68 as u32) ) } as u64;
	// 830402A0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830402A4: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 830402A8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830402AC: 4E800421  bctrl
	ctx.lr = 0x830402B0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830402B0: 807E0044  lwz r3, 0x44(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(68 as u32) ) } as u64;
	// 830402B4: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 830402B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830402BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830402C0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830402C4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830402C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830402CC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830402CC size=48
    let mut pc: u32 = 0x830402CC;
    'dispatch: loop {
        match pc {
            0x830402CC => {
    //   block [0x830402CC..0x830402FC)
	// 830402CC: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 830402D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830402D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830402D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830402DC: 817F0084  lwz r11, 0x84(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 830402E0: 808B0068  lwz r4, 0x68(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(104 as u32) ) } as u64;
	// 830402E4: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 830402E8: 4BF97FF9  bl 0x82fd82e0
	ctx.lr = 0x830402EC;
	sub_82FD82E0(ctx, base);
	// 830402EC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830402F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830402F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830402F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83040300(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83040300 size=8
    let mut pc: u32 = 0x83040300;
    'dispatch: loop {
        match pc {
            0x83040300 => {
    //   block [0x83040300..0x83040308)
	// 83040300: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83040304: 8215DF38  lwz r16, -0x20c8(r21)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-8392 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83040308(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83040308 size=320
    let mut pc: u32 = 0x83040308;
    'dispatch: loop {
        match pc {
            0x83040308 => {
    //   block [0x83040308..0x83040448)
	// 83040308: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304030C: 48167E4D  bl 0x831a8158
	ctx.lr = 0x83040310;
	sub_831A8130(ctx, base);
	// 83040310: 3BE1FF60  addi r31, r1, -0xa0
	ctx.r[31].s64 = ctx.r[1].s64 + -160;
	// 83040314: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83040318: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8304031C: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 83040320: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 83040324: 7CFA3B78  mr r26, r7
	ctx.r[26].u64 = ctx.r[7].u64;
	// 83040328: 7D194378  mr r25, r8
	ctx.r[25].u64 = ctx.r[8].u64;
	// 8304032C: 807E004C  lwz r3, 0x4c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(76 as u32) ) } as u64;
	// 83040330: 7D384B78  mr r24, r9
	ctx.r[24].u64 = ctx.r[9].u64;
	// 83040334: 93DF00B4  stw r30, 0xb4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(180 as u32), ctx.r[30].u32 ) };
	// 83040338: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 8304033C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83040340: 41820024  beq 0x83040364
	if ctx.cr[0].eq {
	pc = 0x83040364; continue 'dispatch;
	}
	// 83040344: 38DF0050  addi r6, r31, 0x50
	ctx.r[6].s64 = ctx.r[31].s64 + 80;
	// 83040348: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8304034C: 480288B5  bl 0x83068c00
	ctx.lr = 0x83040350;
	sub_83068C00(ctx, base);
	// 83040350: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83040354: 41820010  beq 0x83040364
	if ctx.cr[0].eq {
	pc = 0x83040364; continue 'dispatch;
	}
	// 83040358: 83630000  lwz r27, 0(r3)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304035C: 281B0000  cmplwi r27, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83040360: 408200D4  bne 0x83040434
	if !ctx.cr[0].eq {
	pc = 0x83040434; continue 'dispatch;
	}
	// 83040364: 2F190000  cmpwi cr6, r25, 0
	ctx.cr[6].compare_i32(ctx.r[25].s32, 0, &mut ctx.xer);
	// 83040368: 409A00CC  bne cr6, 0x83040434
	if !ctx.cr[6].eq {
	pc = 0x83040434; continue 'dispatch;
	}
	// 8304036C: 817E004C  lwz r11, 0x4c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(76 as u32) ) } as u64;
	// 83040370: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83040374: 409A000C  bne cr6, 0x83040380
	if !ctx.cr[6].eq {
	pc = 0x83040380; continue 'dispatch;
	}
	// 83040378: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8304037C: 4BFFFB9D  bl 0x8303ff18
	ctx.lr = 0x83040380;
	sub_8303FF18(ctx, base);
	// 83040380: 3860004C  li r3, 0x4c
	ctx.r[3].s64 = 76;
	// 83040384: 809E0068  lwz r4, 0x68(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(104 as u32) ) } as u64;
	// 83040388: 4BF97F11  bl 0x82fd8298
	ctx.lr = 0x8304038C;
	sub_82FD8298(ctx, base);
	// 8304038C: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 83040390: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83040394: 41820028  beq 0x830403bc
	if ctx.cr[0].eq {
	pc = 0x830403BC; continue 'dispatch;
	}
	// 83040398: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8304039C: 813E0068  lwz r9, 0x68(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(104 as u32) ) } as u64;
	// 830403A0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 830403A4: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 830403A8: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 830403AC: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 830403B0: 480414B9  bl 0x83081868
	ctx.lr = 0x830403B4;
	sub_83081868(ctx, base);
	// 830403B4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 830403B8: 48000008  b 0x830403c0
	pc = 0x830403C0; continue 'dispatch;
	// 830403BC: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 830403C0: 817E001C  lwz r11, 0x1c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 830403C4: 815D0028  lwz r10, 0x28(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(40 as u32) ) } as u64;
	// 830403C8: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 830403CC: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 830403D0: 7FBBEB78  mr r27, r29
	ctx.r[27].u64 = ctx.r[29].u64;
	// 830403D4: 917D0024  stw r11, 0x24(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 830403D8: 808A0010  lwz r4, 0x10(r10)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 830403DC: 807E004C  lwz r3, 0x4c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(76 as u32) ) } as u64;
	// 830403E0: 4BFFF921  bl 0x8303fd00
	ctx.lr = 0x830403E4;
	sub_8303FD00(ctx, base);
	// 830403E4: 817E0044  lwz r11, 0x44(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(68 as u32) ) } as u64;
	// 830403E8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830403EC: 409A0034  bne cr6, 0x83040420
	if !ctx.cr[6].eq {
	pc = 0x83040420; continue 'dispatch;
	}
	// 830403F0: 3860001C  li r3, 0x1c
	ctx.r[3].s64 = 28;
	// 830403F4: 809E0068  lwz r4, 0x68(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(104 as u32) ) } as u64;
	// 830403F8: 4BF97EA1  bl 0x82fd8298
	ctx.lr = 0x830403FC;
	sub_82FD8298(ctx, base);
	// 830403FC: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 83040400: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83040404: 41820014  beq 0x83040418
	if ctx.cr[0].eq {
	pc = 0x83040418; continue 'dispatch;
	}
	// 83040408: 80BE0068  lwz r5, 0x68(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(104 as u32) ) } as u64;
	// 8304040C: 809E004C  lwz r4, 0x4c(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(76 as u32) ) } as u64;
	// 83040410: 48041FE9  bl 0x830823f8
	ctx.lr = 0x83040414;
	sub_830823F8(ctx, base);
	// 83040414: 48000008  b 0x8304041c
	pc = 0x8304041C; continue 'dispatch;
	// 83040418: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8304041C: 907E0044  stw r3, 0x44(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(68 as u32), ctx.r[3].u32 ) };
	// 83040420: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83040424: 807E0044  lwz r3, 0x44(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(68 as u32) ) } as u64;
	// 83040428: 4BFFDAB9  bl 0x8303dee0
	ctx.lr = 0x8304042C;
	sub_8303DEE0(ctx, base);
	// 8304042C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83040430: 48000008  b 0x83040438
	pc = 0x83040438; continue 'dispatch;
	// 83040434: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83040438: 99780000  stb r11, 0(r24)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[24].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 8304043C: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 83040440: 383F00A0  addi r1, r31, 0xa0
	ctx.r[1].s64 = ctx.r[31].s64 + 160;
	// 83040444: 48167D64  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83040448(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83040448 size=48
    let mut pc: u32 = 0x83040448;
    'dispatch: loop {
        match pc {
            0x83040448 => {
    //   block [0x83040448..0x83040478)
	// 83040448: 3BECFF60  addi r31, r12, -0xa0
	ctx.r[31].s64 = ctx.r[12].s64 + -160;
	// 8304044C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83040450: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83040454: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83040458: 817F00B4  lwz r11, 0xb4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(180 as u32) ) } as u64;
	// 8304045C: 808B0068  lwz r4, 0x68(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(104 as u32) ) } as u64;
	// 83040460: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 83040464: 4BF97E7D  bl 0x82fd82e0
	ctx.lr = 0x83040468;
	sub_82FD82E0(ctx, base);
	// 83040468: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8304046C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83040470: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83040474: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83040478(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83040478 size=48
    let mut pc: u32 = 0x83040478;
    'dispatch: loop {
        match pc {
            0x83040478 => {
    //   block [0x83040478..0x830404A8)
	// 83040478: 3BECFF60  addi r31, r12, -0xa0
	ctx.r[31].s64 = ctx.r[12].s64 + -160;
	// 8304047C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83040480: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83040484: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83040488: 817F00B4  lwz r11, 0xb4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(180 as u32) ) } as u64;
	// 8304048C: 808B0068  lwz r4, 0x68(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(104 as u32) ) } as u64;
	// 83040490: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 83040494: 4BF97E4D  bl 0x82fd82e0
	ctx.lr = 0x83040498;
	sub_82FD82E0(ctx, base);
	// 83040498: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8304049C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830404A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830404A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830404A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830404A8 size=124
    let mut pc: u32 = 0x830404A8;
    'dispatch: loop {
        match pc {
            0x830404A8 => {
    //   block [0x830404A8..0x83040524)
	// 830404A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830404AC: 48167CB9  bl 0x831a8164
	ctx.lr = 0x830404B0;
	sub_831A8130(ctx, base);
	// 830404B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830404B4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830404B8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 830404BC: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 830404C0: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 830404C4: 7CFB3B78  mr r27, r7
	ctx.r[27].u64 = ctx.r[7].u64;
	// 830404C8: 817F003C  lwz r11, 0x3c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) } as u64;
	// 830404CC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830404D0: 419A004C  beq cr6, 0x8304051c
	if ctx.cr[6].eq {
	pc = 0x8304051C; continue 'dispatch;
	}
	// 830404D4: 817F0050  lwz r11, 0x50(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 830404D8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830404DC: 409A0040  bne cr6, 0x8304051c
	if !ctx.cr[6].eq {
	pc = 0x8304051C; continue 'dispatch;
	}
	// 830404E0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 830404E4: 4BFFFADD  bl 0x8303ffc0
	ctx.lr = 0x830404E8;
	sub_8303FFC0(ctx, base);
	// 830404E8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830404EC: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 830404F0: 4182002C  beq 0x8304051c
	if ctx.cr[0].eq {
	pc = 0x8304051C; continue 'dispatch;
	}
	// 830404F4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830404F8: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 830404FC: 813F0028  lwz r9, 0x28(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 83040500: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 83040504: 811F0058  lwz r8, 0x58(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 83040508: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8304050C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83040510: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 83040514: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83040518: 4E800421  bctrl
	ctx.lr = 0x8304051C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8304051C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83040520: 48167C94  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83040528(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83040528 size=100
    let mut pc: u32 = 0x83040528;
    'dispatch: loop {
        match pc {
            0x83040528 => {
    //   block [0x83040528..0x8304058C)
	// 83040528: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304052C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83040530: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83040534: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83040538: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8304053C: 4BFFFB75  bl 0x830400b0
	ctx.lr = 0x83040540;
	sub_830400B0(ctx, base);
	// 83040540: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83040544: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83040548: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304054C: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83040550: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83040554: 4E800421  bctrl
	ctx.lr = 0x83040558;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83040558: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8304055C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83040560: 41820018  beq 0x83040578
	if ctx.cr[0].eq {
	pc = 0x83040578; continue 'dispatch;
	}
	// 83040564: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83040568: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8304056C: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83040570: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83040574: 4E800421  bctrl
	ctx.lr = 0x83040578;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83040578: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8304057C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83040580: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83040584: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83040588: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83040590(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83040590 size=8
    let mut pc: u32 = 0x83040590;
    'dispatch: loop {
        match pc {
            0x83040590 => {
    //   block [0x83040590..0x83040598)
	// 83040590: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83040594: 8215DF90  lwz r16, -0x2070(r21)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-8304 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83040598(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83040598 size=96
    let mut pc: u32 = 0x83040598;
    'dispatch: loop {
        match pc {
            0x83040598 => {
    //   block [0x83040598..0x830405F8)
	// 83040598: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304059C: 48167BD1  bl 0x831a816c
	ctx.lr = 0x830405A0;
	sub_831A8130(ctx, base);
	// 830405A0: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 830405A4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830405A8: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 830405AC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830405B0: 396BDDD8  addi r11, r11, -0x2228
	ctx.r[11].s64 = ctx.r[11].s64 + -8744;
	// 830405B4: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 830405B8: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830405BC: 897E0004  lbz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830405C0: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830405C4: 41820020  beq 0x830405e4
	if ctx.cr[0].eq {
	pc = 0x830405E4; continue 'dispatch;
	}
	// 830405C8: 83BE0010  lwz r29, 0x10(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 830405CC: 281D0000  cmplwi r29, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830405D0: 41820014  beq 0x830405e4
	if ctx.cr[0].eq {
	pc = 0x830405E4; continue 'dispatch;
	}
	// 830405D4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830405D8: 4BFFFF51  bl 0x83040528
	ctx.lr = 0x830405DC;
	sub_83040528(ctx, base);
	// 830405DC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830405E0: 4BF97D01  bl 0x82fd82e0
	ctx.lr = 0x830405E4;
	sub_82FD82E0(ctx, base);
	// 830405E4: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 830405E8: 396BA93C  addi r11, r11, -0x56c4
	ctx.r[11].s64 = ctx.r[11].s64 + -22212;
	// 830405EC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830405F0: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 830405F4: 48167BC8  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830405F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830405F8 size=40
    let mut pc: u32 = 0x830405F8;
    'dispatch: loop {
        match pc {
            0x830405F8 => {
    //   block [0x830405F8..0x83040620)
	// 830405F8: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 830405FC: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83040600: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83040604: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83040608: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 8304060C: 4800C155  bl 0x8304c760
	ctx.lr = 0x83040610;
	sub_8304C760(ctx, base);
	// 83040610: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83040614: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83040618: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8304061C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83040620(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83040620 size=76
    let mut pc: u32 = 0x83040620;
    'dispatch: loop {
        match pc {
            0x83040620 => {
    //   block [0x83040620..0x8304066C)
	// 83040620: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83040624: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83040628: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8304062C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83040630: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83040634: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83040638: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8304063C: 4BFFFF5D  bl 0x83040598
	ctx.lr = 0x83040640;
	sub_83040598(ctx, base);
	// 83040640: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83040644: 4182000C  beq 0x83040650
	if ctx.cr[0].eq {
	pc = 0x83040650; continue 'dispatch;
	}
	// 83040648: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8304064C: 4BF97C95  bl 0x82fd82e0
	ctx.lr = 0x83040650;
	sub_82FD82E0(ctx, base);
	// 83040650: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83040654: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83040658: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8304065C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83040660: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83040664: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83040668: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83040670(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83040670 size=8
    let mut pc: u32 = 0x83040670;
    'dispatch: loop {
        match pc {
            0x83040670 => {
    //   block [0x83040670..0x83040678)
	// 83040670: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83040674: 8215DFC8  lwz r16, -0x2038(r21)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-8248 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83040678(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83040678 size=408
    let mut pc: u32 = 0x83040678;
    'dispatch: loop {
        match pc {
            0x83040678 => {
    //   block [0x83040678..0x83040810)
	// 83040678: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304067C: 48167AF1  bl 0x831a816c
	ctx.lr = 0x83040680;
	sub_831A8130(ctx, base);
	// 83040680: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 83040684: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83040688: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 8304068C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83040690: 396BDB48  addi r11, r11, -0x24b8
	ctx.r[11].s64 = ctx.r[11].s64 + -9400;
	// 83040694: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 83040698: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8304069C: 807E0068  lwz r3, 0x68(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(104 as u32) ) } as u64;
	// 830406A0: 809E0024  lwz r4, 0x24(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 830406A4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830406A8: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 830406AC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830406B0: 4E800421  bctrl
	ctx.lr = 0x830406B4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830406B4: 807E0068  lwz r3, 0x68(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(104 as u32) ) } as u64;
	// 830406B8: 809E0028  lwz r4, 0x28(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 830406BC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830406C0: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 830406C4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830406C8: 4E800421  bctrl
	ctx.lr = 0x830406CC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830406CC: 807E0068  lwz r3, 0x68(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(104 as u32) ) } as u64;
	// 830406D0: 809E002C  lwz r4, 0x2c(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(44 as u32) ) } as u64;
	// 830406D4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830406D8: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 830406DC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830406E0: 4E800421  bctrl
	ctx.lr = 0x830406E4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830406E4: 897E0006  lbz r11, 6(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(6 as u32) ) } as u64;
	// 830406E8: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830406EC: 41820024  beq 0x83040710
	if ctx.cr[0].eq {
	pc = 0x83040710; continue 'dispatch;
	}
	// 830406F0: 807E003C  lwz r3, 0x3c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(60 as u32) ) } as u64;
	// 830406F4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830406F8: 41820018  beq 0x83040710
	if ctx.cr[0].eq {
	pc = 0x83040710; continue 'dispatch;
	}
	// 830406FC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83040700: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83040704: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83040708: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8304070C: 4E800421  bctrl
	ctx.lr = 0x83040710;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83040710: 807E0040  lwz r3, 0x40(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(64 as u32) ) } as u64;
	// 83040714: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83040718: 41820018  beq 0x83040730
	if ctx.cr[0].eq {
	pc = 0x83040730; continue 'dispatch;
	}
	// 8304071C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83040720: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83040724: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83040728: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8304072C: 4E800421  bctrl
	ctx.lr = 0x83040730;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83040730: 83BE004C  lwz r29, 0x4c(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(76 as u32) ) } as u64;
	// 83040734: 281D0000  cmplwi r29, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83040738: 41820014  beq 0x8304074c
	if ctx.cr[0].eq {
	pc = 0x8304074C; continue 'dispatch;
	}
	// 8304073C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83040740: 4BFFFDE9  bl 0x83040528
	ctx.lr = 0x83040744;
	sub_83040528(ctx, base);
	// 83040744: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83040748: 4BF97B99  bl 0x82fd82e0
	ctx.lr = 0x8304074C;
	sub_82FD82E0(ctx, base);
	// 8304074C: 807E0044  lwz r3, 0x44(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(68 as u32) ) } as u64;
	// 83040750: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83040754: 41820018  beq 0x8304076c
	if ctx.cr[0].eq {
	pc = 0x8304076C; continue 'dispatch;
	}
	// 83040758: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304075C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83040760: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83040764: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83040768: 4E800421  bctrl
	ctx.lr = 0x8304076C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8304076C: 807E0048  lwz r3, 0x48(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(72 as u32) ) } as u64;
	// 83040770: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83040774: 41820018  beq 0x8304078c
	if ctx.cr[0].eq {
	pc = 0x8304078C; continue 'dispatch;
	}
	// 83040778: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304077C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83040780: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83040784: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83040788: 4E800421  bctrl
	ctx.lr = 0x8304078C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8304078C: 807E0064  lwz r3, 0x64(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(100 as u32) ) } as u64;
	// 83040790: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83040794: 41820018  beq 0x830407ac
	if ctx.cr[0].eq {
	pc = 0x830407AC; continue 'dispatch;
	}
	// 83040798: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304079C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 830407A0: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830407A4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830407A8: 4E800421  bctrl
	ctx.lr = 0x830407AC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830407AC: 807E0050  lwz r3, 0x50(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(80 as u32) ) } as u64;
	// 830407B0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830407B4: 41820018  beq 0x830407cc
	if ctx.cr[0].eq {
	pc = 0x830407CC; continue 'dispatch;
	}
	// 830407B8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830407BC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 830407C0: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830407C4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830407C8: 4E800421  bctrl
	ctx.lr = 0x830407CC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830407CC: 807E0068  lwz r3, 0x68(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(104 as u32) ) } as u64;
	// 830407D0: 809E0054  lwz r4, 0x54(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(84 as u32) ) } as u64;
	// 830407D4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830407D8: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 830407DC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830407E0: 4E800421  bctrl
	ctx.lr = 0x830407E4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830407E4: 807E0068  lwz r3, 0x68(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(104 as u32) ) } as u64;
	// 830407E8: 809E0058  lwz r4, 0x58(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(88 as u32) ) } as u64;
	// 830407EC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830407F0: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 830407F4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830407F8: 4E800421  bctrl
	ctx.lr = 0x830407FC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830407FC: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83040800: 396BA93C  addi r11, r11, -0x56c4
	ctx.r[11].s64 = ctx.r[11].s64 + -22212;
	// 83040804: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83040808: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 8304080C: 481679B0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83040810(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83040810 size=40
    let mut pc: u32 = 0x83040810;
    'dispatch: loop {
        match pc {
            0x83040810 => {
    //   block [0x83040810..0x83040838)
	// 83040810: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 83040814: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83040818: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8304081C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83040820: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 83040824: 4800BF3D  bl 0x8304c760
	ctx.lr = 0x83040828;
	sub_8304C760(ctx, base);
	// 83040828: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8304082C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83040830: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83040834: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83040838(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83040838 size=8
    let mut pc: u32 = 0x83040838;
    'dispatch: loop {
        match pc {
            0x83040838 => {
    //   block [0x83040838..0x83040840)
	// 83040838: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8304083C: 8215E000  lwz r16, -0x2000(r21)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-8192 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83040840(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83040840 size=156
    let mut pc: u32 = 0x83040840;
    'dispatch: loop {
        match pc {
            0x83040840 => {
    //   block [0x83040840..0x830408DC)
	// 83040840: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83040844: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83040848: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8304084C: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 83040850: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83040854: 8083004C  lwz r4, 0x4c(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(76 as u32) ) } as u64;
	// 83040858: 28040000  cmplwi r4, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8304085C: 4082000C  bne 0x83040868
	if !ctx.cr[0].eq {
	pc = 0x83040868; continue 'dispatch;
	}
	// 83040860: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83040864: 48000064  b 0x830408c8
	pc = 0x830408C8; continue 'dispatch;
	// 83040868: 80C30068  lwz r6, 0x68(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(104 as u32) ) } as u64;
	// 8304086C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83040870: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 83040874: 4BFFF555  bl 0x8303fdc8
	ctx.lr = 0x83040878;
	sub_8303FDC8(ctx, base);
	// 83040878: 817F0058  lwz r11, 0x58(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 8304087C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83040880: 409A001C  bne cr6, 0x8304089c
	if !ctx.cr[6].eq {
	pc = 0x8304089C; continue 'dispatch;
	}
	// 83040884: 817F0060  lwz r11, 0x60(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 83040888: 815F005C  lwz r10, 0x5c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 8304088C: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 83040890: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 83040894: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83040898: 419A0008  beq cr6, 0x830408a0
	if ctx.cr[6].eq {
	pc = 0x830408A0; continue 'dispatch;
	}
	// 8304089C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 830408A0: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830408A4: 41820018  beq 0x830408bc
	if ctx.cr[0].eq {
	pc = 0x830408BC; continue 'dispatch;
	}
	// 830408A8: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 830408AC: 4BFA338D  bl 0x82fe3c38
	ctx.lr = 0x830408B0;
	sub_82FE3C38(ctx, base);
	// 830408B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830408B4: 99630010  stb r11, 0x10(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u8 ) };
	// 830408B8: 4BFFFFC0  b 0x83040878
	pc = 0x83040878; continue 'dispatch;
	// 830408BC: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 830408C0: 4BFFFCD9  bl 0x83040598
	ctx.lr = 0x830408C4;
	sub_83040598(ctx, base);
	// 830408C4: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 830408C8: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 830408CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830408D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830408D4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830408D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830408DC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830408DC size=40
    let mut pc: u32 = 0x830408DC;
    'dispatch: loop {
        match pc {
            0x830408DC => {
    //   block [0x830408DC..0x83040904)
	// 830408DC: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 830408E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830408E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830408E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830408EC: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 830408F0: 4BFFFCA9  bl 0x83040598
	ctx.lr = 0x830408F4;
	sub_83040598(ctx, base);
	// 830408F4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830408F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830408FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83040900: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83040908(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83040908 size=76
    let mut pc: u32 = 0x83040908;
    'dispatch: loop {
        match pc {
            0x83040908 => {
    //   block [0x83040908..0x83040954)
	// 83040908: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304090C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83040910: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83040914: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83040918: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304091C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83040920: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83040924: 4BFFFD55  bl 0x83040678
	ctx.lr = 0x83040928;
	sub_83040678(ctx, base);
	// 83040928: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8304092C: 4182000C  beq 0x83040938
	if ctx.cr[0].eq {
	pc = 0x83040938; continue 'dispatch;
	}
	// 83040930: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83040934: 4BF979AD  bl 0x82fd82e0
	ctx.lr = 0x83040938;
	sub_82FD82E0(ctx, base);
	// 83040938: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8304093C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83040940: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83040944: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83040948: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8304094C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83040950: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83040958(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83040958 size=8
    let mut pc: u32 = 0x83040958;
    'dispatch: loop {
        match pc {
            0x83040958 => {
    //   block [0x83040958..0x83040960)
	// 83040958: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8304095C: 8215E080  lwz r16, -0x1f80(r21)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-8064 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83040960(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83040960 size=140
    let mut pc: u32 = 0x83040960;
    'dispatch: loop {
        match pc {
            0x83040960 => {
    //   block [0x83040960..0x830409EC)
	// 83040960: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83040964: 48167801  bl 0x831a8164
	ctx.lr = 0x83040968;
	sub_831A8130(ctx, base);
	// 83040968: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 8304096C: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83040970: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83040974: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83040978: 396BA93C  addi r11, r11, -0x56c4
	ctx.r[11].s64 = ctx.r[11].s64 + -22212;
	// 8304097C: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 83040980: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 83040984: 93DF0094  stw r30, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[30].u32 ) };
	// 83040988: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8304098C: 3B9E0004  addi r28, r30, 4
	ctx.r[28].s64 = ctx.r[30].s64 + 4;
	// 83040990: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 83040994: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83040998: 3880000C  li r4, 0xc
	ctx.r[4].s64 = 12;
	// 8304099C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 830409A0: 4BFEEB11  bl 0x8302f4b0
	ctx.lr = 0x830409A4;
	sub_8302F4B0(ctx, base);
	// 830409A4: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 830409A8: 3D408216  lis r10, -0x7dea
	ctx.r[10].s64 = -2112487424;
	// 830409AC: 396BE05C  addi r11, r11, -0x1fa4
	ctx.r[11].s64 = ctx.r[11].s64 + -8100;
	// 830409B0: 394AE048  addi r10, r10, -0x1fb8
	ctx.r[10].s64 = ctx.r[10].s64 + -8120;
	// 830409B4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 830409B8: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 830409BC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830409C0: 915C0000  stw r10, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 830409C4: 4BF901BD  bl 0x82fd0b80
	ctx.lr = 0x830409C8;
	sub_82FD0B80(ctx, base);
	// 830409C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830409CC: 907E0018  stw r3, 0x18(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), ctx.r[3].u32 ) };
	// 830409D0: 917E001C  stw r11, 0x1c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 830409D4: 917E0020  stw r11, 0x20(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 830409D8: 917E0024  stw r11, 0x24(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 830409DC: 917E0028  stw r11, 0x28(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 830409E0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830409E4: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 830409E8: 481677CC  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830409EC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830409EC size=40
    let mut pc: u32 = 0x830409EC;
    'dispatch: loop {
        match pc {
            0x830409EC => {
    //   block [0x830409EC..0x83040A14)
	// 830409EC: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 830409F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830409F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830409F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830409FC: 807F0094  lwz r3, 0x94(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 83040A00: 4800BD61  bl 0x8304c760
	ctx.lr = 0x83040A04;
	sub_8304C760(ctx, base);
	// 83040A04: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83040A08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83040A0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83040A10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83040A14(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83040A14 size=44
    let mut pc: u32 = 0x83040A14;
    'dispatch: loop {
        match pc {
            0x83040A14 => {
    //   block [0x83040A14..0x83040A40)
	// 83040A14: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 83040A18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83040A1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83040A20: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83040A24: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 83040A28: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 83040A2C: 4BFEEAED  bl 0x8302f518
	ctx.lr = 0x83040A30;
	sub_8302F518(ctx, base);
	// 83040A30: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83040A34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83040A38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83040A3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83040A40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83040A40 size=8
    let mut pc: u32 = 0x83040A40;
    'dispatch: loop {
        match pc {
            0x83040A40 => {
    //   block [0x83040A40..0x83040A48)
	// 83040A40: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83040A44: 8215E0C8  lwz r16, -0x1f38(r21)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-7992 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83040A48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83040A48 size=120
    let mut pc: u32 = 0x83040A48;
    'dispatch: loop {
        match pc {
            0x83040A48 => {
    //   block [0x83040A48..0x83040AC0)
	// 83040A48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83040A4C: 48167721  bl 0x831a816c
	ctx.lr = 0x83040A50;
	sub_831A8130(ctx, base);
	// 83040A50: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 83040A54: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83040A58: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83040A5C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83040A60: 396BA93C  addi r11, r11, -0x56c4
	ctx.r[11].s64 = ctx.r[11].s64 + -22212;
	// 83040A64: 7C862378  mr r6, r4
	ctx.r[6].u64 = ctx.r[4].u64;
	// 83040A68: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 83040A6C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83040A70: 3BBE0004  addi r29, r30, 4
	ctx.r[29].s64 = ctx.r[30].s64 + 4;
	// 83040A74: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83040A78: 3880000C  li r4, 0xc
	ctx.r[4].s64 = 12;
	// 83040A7C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83040A80: 4BFEEA31  bl 0x8302f4b0
	ctx.lr = 0x83040A84;
	sub_8302F4B0(ctx, base);
	// 83040A84: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 83040A88: 394BE05C  addi r10, r11, -0x1fa4
	ctx.r[10].s64 = ctx.r[11].s64 + -8100;
	// 83040A8C: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 83040A90: 392BE048  addi r9, r11, -0x1fb8
	ctx.r[9].s64 = ctx.r[11].s64 + -8120;
	// 83040A94: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83040A98: 915E0000  stw r10, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83040A9C: 913D0000  stw r9, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 83040AA0: 917E0018  stw r11, 0x18(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 83040AA4: 917E001C  stw r11, 0x1c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 83040AA8: 917E0020  stw r11, 0x20(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 83040AAC: 917E0024  stw r11, 0x24(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 83040AB0: 917E0028  stw r11, 0x28(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 83040AB4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83040AB8: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 83040ABC: 48167700  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83040AC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83040AC0 size=40
    let mut pc: u32 = 0x83040AC0;
    'dispatch: loop {
        match pc {
            0x83040AC0 => {
    //   block [0x83040AC0..0x83040AE8)
	// 83040AC0: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 83040AC4: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83040AC8: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83040ACC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83040AD0: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 83040AD4: 4800BC8D  bl 0x8304c760
	ctx.lr = 0x83040AD8;
	sub_8304C760(ctx, base);
	// 83040AD8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83040ADC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83040AE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83040AE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83040AE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83040AE8 size=8
    let mut pc: u32 = 0x83040AE8;
    'dispatch: loop {
        match pc {
            0x83040AE8 => {
    //   block [0x83040AE8..0x83040AF0)
	// 83040AE8: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83040AEC: 8215E108  lwz r16, -0x1ef8(r21)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-7928 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83040AF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83040AF0 size=160
    let mut pc: u32 = 0x83040AF0;
    'dispatch: loop {
        match pc {
            0x83040AF0 => {
    //   block [0x83040AF0..0x83040B90)
	// 83040AF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83040AF4: 48167679  bl 0x831a816c
	ctx.lr = 0x83040AF8;
	sub_831A8130(ctx, base);
	// 83040AF8: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 83040AFC: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83040B00: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83040B04: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 83040B08: 3D408216  lis r10, -0x7dea
	ctx.r[10].s64 = -2112487424;
	// 83040B0C: 3BBE0004  addi r29, r30, 4
	ctx.r[29].s64 = ctx.r[30].s64 + 4;
	// 83040B10: 396BE05C  addi r11, r11, -0x1fa4
	ctx.r[11].s64 = ctx.r[11].s64 + -8100;
	// 83040B14: 394AE048  addi r10, r10, -0x1fb8
	ctx.r[10].s64 = ctx.r[10].s64 + -8120;
	// 83040B18: 93DF0094  stw r30, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[30].u32 ) };
	// 83040B1C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83040B20: 915D0000  stw r10, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83040B24: 807E0010  lwz r3, 0x10(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 83040B28: 809E0018  lwz r4, 0x18(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 83040B2C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83040B30: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83040B34: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83040B38: 4E800421  bctrl
	ctx.lr = 0x83040B3C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83040B3C: 807E001C  lwz r3, 0x1c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 83040B40: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83040B44: 41820018  beq 0x83040b5c
	if ctx.cr[0].eq {
	pc = 0x83040B5C; continue 'dispatch;
	}
	// 83040B48: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83040B4C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83040B50: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83040B54: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83040B58: 4E800421  bctrl
	ctx.lr = 0x83040B5C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83040B5C: 807E0010  lwz r3, 0x10(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 83040B60: 809E0020  lwz r4, 0x20(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 83040B64: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83040B68: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83040B6C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83040B70: 4E800421  bctrl
	ctx.lr = 0x83040B74;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83040B74: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83040B78: 4BFEE9A1  bl 0x8302f518
	ctx.lr = 0x83040B7C;
	sub_8302F518(ctx, base);
	// 83040B7C: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83040B80: 396BA93C  addi r11, r11, -0x56c4
	ctx.r[11].s64 = ctx.r[11].s64 + -22212;
	// 83040B84: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83040B88: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 83040B8C: 48167630  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83040B90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83040B90 size=40
    let mut pc: u32 = 0x83040B90;
    'dispatch: loop {
        match pc {
            0x83040B90 => {
    //   block [0x83040B90..0x83040BB8)
	// 83040B90: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 83040B94: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83040B98: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83040B9C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83040BA0: 807F0094  lwz r3, 0x94(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 83040BA4: 4800BBBD  bl 0x8304c760
	ctx.lr = 0x83040BA8;
	sub_8304C760(ctx, base);
	// 83040BA8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83040BAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83040BB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83040BB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83040BB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83040BB8 size=76
    let mut pc: u32 = 0x83040BB8;
    'dispatch: loop {
        match pc {
            0x83040BB8 => {
    //   block [0x83040BB8..0x83040C04)
	// 83040BB8: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 83040BBC: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83040BC0: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83040BC4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83040BC8: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 83040BCC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83040BD0: 419A0014  beq cr6, 0x83040be4
	if ctx.cr[6].eq {
	pc = 0x83040BE4; continue 'dispatch;
	}
	// 83040BD4: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 83040BD8: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 83040BDC: 917F0050  stw r11, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 83040BE0: 4800000C  b 0x83040bec
	pc = 0x83040BEC; continue 'dispatch;
	// 83040BE4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83040BE8: 917F0050  stw r11, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 83040BEC: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 83040BF0: 4BFEE929  bl 0x8302f518
	ctx.lr = 0x83040BF4;
	sub_8302F518(ctx, base);
	// 83040BF4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83040BF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83040BFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83040C00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83040C08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83040C08 size=4
    let mut pc: u32 = 0x83040C08;
    'dispatch: loop {
        match pc {
            0x83040C08 => {
    //   block [0x83040C08..0x83040C0C)
	// 83040C08: 48000008  b 0x83040c10
	sub_83040C0C(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83040C0C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83040C0C size=24
    let mut pc: u32 = 0x83040C0C;
    'dispatch: loop {
        match pc {
            0x83040C0C => {
    //   block [0x83040C0C..0x83040C24)
	// 83040C0C: 8063001C  lwz r3, 0x1c(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 83040C10: 8163001C  lwz r11, 0x1c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 83040C14: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83040C18: 409AFFF4  bne cr6, 0x83040c0c
	if !ctx.cr[6].eq {
	pc = 0x83040C0C; continue 'dispatch;
	}
	// 83040C1C: 9083001C  stw r4, 0x1c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[4].u32 ) };
	// 83040C20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83040C28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83040C28 size=116
    let mut pc: u32 = 0x83040C28;
    'dispatch: loop {
        match pc {
            0x83040C28 => {
    //   block [0x83040C28..0x83040C9C)
	// 83040C28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83040C2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83040C30: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83040C34: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83040C38: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83040C3C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83040C40: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83040C44: 809F0020  lwz r4, 0x20(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 83040C48: 28040000  cmplwi r4, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83040C4C: 41820020  beq 0x83040c6c
	if ctx.cr[0].eq {
	pc = 0x83040C6C; continue 'dispatch;
	}
	// 83040C50: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 83040C54: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83040C58: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83040C5C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83040C60: 4E800421  bctrl
	ctx.lr = 0x83040C64;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83040C64: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83040C68: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 83040C6C: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 83040C70: 419A0014  beq cr6, 0x83040c84
	if ctx.cr[6].eq {
	pc = 0x83040C84; continue 'dispatch;
	}
	// 83040C74: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83040C78: 809F0010  lwz r4, 0x10(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 83040C7C: 4BF8FF05  bl 0x82fd0b80
	ctx.lr = 0x83040C80;
	sub_82FD0B80(ctx, base);
	// 83040C80: 907F0020  stw r3, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[3].u32 ) };
	// 83040C84: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83040C88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83040C8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83040C90: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83040C94: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83040C98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83040CA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83040CA0 size=8
    let mut pc: u32 = 0x83040CA0;
    'dispatch: loop {
        match pc {
            0x83040CA0 => {
    //   block [0x83040CA0..0x83040CA8)
	// 83040CA0: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83040CA4: 8215E148  lwz r16, -0x1eb8(r21)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-7864 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83040CA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83040CA8 size=96
    let mut pc: u32 = 0x83040CA8;
    'dispatch: loop {
        match pc {
            0x83040CA8 => {
    //   block [0x83040CA8..0x83040D08)
	// 83040CA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83040CAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83040CB0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83040CB4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83040CB8: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 83040CBC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83040CC0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83040CC4: 3860002C  li r3, 0x2c
	ctx.r[3].s64 = 44;
	// 83040CC8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83040CCC: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 83040CD0: 4BF975C9  bl 0x82fd8298
	ctx.lr = 0x83040CD4;
	sub_82FD8298(ctx, base);
	// 83040CD4: 907F0050  stw r3, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 83040CD8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83040CDC: 41820010  beq 0x83040cec
	if ctx.cr[0].eq {
	pc = 0x83040CEC; continue 'dispatch;
	}
	// 83040CE0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83040CE4: 4BFFFD65  bl 0x83040a48
	ctx.lr = 0x83040CE8;
	sub_83040A48(ctx, base);
	// 83040CE8: 48000008  b 0x83040cf0
	pc = 0x83040CF0; continue 'dispatch;
	// 83040CEC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83040CF0: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 83040CF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83040CF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83040CFC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83040D00: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83040D04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83040D08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83040D08 size=44
    let mut pc: u32 = 0x83040D08;
    'dispatch: loop {
        match pc {
            0x83040D08 => {
    //   block [0x83040D08..0x83040D34)
	// 83040D08: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 83040D0C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83040D10: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83040D14: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83040D18: 809F0084  lwz r4, 0x84(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 83040D1C: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 83040D20: 4BF975C1  bl 0x82fd82e0
	ctx.lr = 0x83040D24;
	sub_82FD82E0(ctx, base);
	// 83040D24: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83040D28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83040D2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83040D30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83040D38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83040D38 size=12
    let mut pc: u32 = 0x83040D38;
    'dispatch: loop {
        match pc {
            0x83040D38 => {
    //   block [0x83040D38..0x83040D44)
	// 83040D38: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 83040D3C: 386B32D4  addi r3, r11, 0x32d4
	ctx.r[3].s64 = ctx.r[11].s64 + 13012;
	// 83040D40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83040D48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83040D48 size=240
    let mut pc: u32 = 0x83040D48;
    'dispatch: loop {
        match pc {
            0x83040D48 => {
    //   block [0x83040D48..0x83040E38)
	// 83040D48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83040D4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83040D50: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83040D54: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83040D58: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83040D5C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 83040D60: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83040D64: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83040D68: A97F0000  lha r11, 0(r31)
	ctx.r[11].s64 = (unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as i16) as i64;
	// 83040D6C: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 83040D70: 556BDFFF  rlwinm. r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83040D74: 41820050  beq 0x83040dc4
	if ctx.cr[0].eq {
	pc = 0x83040DC4; continue 'dispatch;
	}
	// 83040D78: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 83040D7C: 809E0018  lwz r4, 0x18(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 83040D80: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83040D84: 4BFB8B7D  bl 0x82ff9900
	ctx.lr = 0x83040D88;
	sub_82FF9900(ctx, base);
	// 83040D88: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83040D8C: 809E001C  lwz r4, 0x1c(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 83040D90: 4BFB8E71  bl 0x82ff9c00
	ctx.lr = 0x83040D94;
	sub_82FF9C00(ctx, base);
	// 83040D94: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 83040D98: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83040D9C: 809E0020  lwz r4, 0x20(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 83040DA0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83040DA4: 4BFB8B5D  bl 0x82ff9900
	ctx.lr = 0x83040DA8;
	sub_82FF9900(ctx, base);
	// 83040DA8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83040DAC: 809E0024  lwz r4, 0x24(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 83040DB0: 4BFB8549  bl 0x82ff92f8
	ctx.lr = 0x83040DB4;
	sub_82FF92F8(ctx, base);
	// 83040DB4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83040DB8: 809E0028  lwz r4, 0x28(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 83040DBC: 4BFB853D  bl 0x82ff92f8
	ctx.lr = 0x83040DC0;
	sub_82FF92F8(ctx, base);
	// 83040DC0: 48000060  b 0x83040e20
	pc = 0x83040E20; continue 'dispatch;
	// 83040DC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83040DC8: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 83040DCC: 38A10054  addi r5, r1, 0x54
	ctx.r[5].s64 = ctx.r[1].s64 + 84;
	// 83040DD0: 389E0018  addi r4, r30, 0x18
	ctx.r[4].s64 = ctx.r[30].s64 + 24;
	// 83040DD4: 4BFB8D55  bl 0x82ff9b28
	ctx.lr = 0x83040DD8;
	sub_82FF9B28(ctx, base);
	// 83040DD8: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 83040DDC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83040DE0: 388B32D4  addi r4, r11, 0x32d4
	ctx.r[4].s64 = ctx.r[11].s64 + 13012;
	// 83040DE4: 4BFB8EDD  bl 0x82ff9cc0
	ctx.lr = 0x83040DE8;
	sub_82FF9CC0(ctx, base);
	// 83040DE8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83040DEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83040DF0: 38C10054  addi r6, r1, 0x54
	ctx.r[6].s64 = ctx.r[1].s64 + 84;
	// 83040DF4: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 83040DF8: 389E0020  addi r4, r30, 0x20
	ctx.r[4].s64 = ctx.r[30].s64 + 32;
	// 83040DFC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83040E00: 917E001C  stw r11, 0x1c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 83040E04: 4BFB8D25  bl 0x82ff9b28
	ctx.lr = 0x83040E08;
	sub_82FF9B28(ctx, base);
	// 83040E08: 389E0024  addi r4, r30, 0x24
	ctx.r[4].s64 = ctx.r[30].s64 + 36;
	// 83040E0C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83040E10: 4BFB8769  bl 0x82ff9578
	ctx.lr = 0x83040E14;
	sub_82FF9578(ctx, base);
	// 83040E14: 389E0028  addi r4, r30, 0x28
	ctx.r[4].s64 = ctx.r[30].s64 + 40;
	// 83040E18: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83040E1C: 4BFB875D  bl 0x82ff9578
	ctx.lr = 0x83040E20;
	sub_82FF9578(ctx, base);
	// 83040E20: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83040E24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83040E28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83040E2C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83040E30: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83040E34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83040E38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83040E38 size=8
    let mut pc: u32 = 0x83040E38;
    'dispatch: loop {
        match pc {
            0x83040E38 => {
    //   block [0x83040E38..0x83040E40)
	// 83040E38: 3863FFFC  addi r3, r3, -4
	ctx.r[3].s64 = ctx.r[3].s64 + -4;
	// 83040E3C: 48000004  b 0x83040e40
	sub_83040E40(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83040E40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83040E40 size=76
    let mut pc: u32 = 0x83040E40;
    'dispatch: loop {
        match pc {
            0x83040E40 => {
    //   block [0x83040E40..0x83040E8C)
	// 83040E40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83040E44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83040E48: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83040E4C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83040E50: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83040E54: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83040E58: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83040E5C: 4BFFFC95  bl 0x83040af0
	ctx.lr = 0x83040E60;
	sub_83040AF0(ctx, base);
	// 83040E60: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83040E64: 4182000C  beq 0x83040e70
	if ctx.cr[0].eq {
	pc = 0x83040E70; continue 'dispatch;
	}
	// 83040E68: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83040E6C: 4BF97475  bl 0x82fd82e0
	ctx.lr = 0x83040E70;
	sub_82FD82E0(ctx, base);
	// 83040E70: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83040E74: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83040E78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83040E7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83040E80: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83040E84: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83040E88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83040E90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83040E90 size=112
    let mut pc: u32 = 0x83040E90;
    'dispatch: loop {
        match pc {
            0x83040E90 => {
    //   block [0x83040E90..0x83040F00)
	// 83040E90: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 83040E94: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 83040E98: 91030004  stw r8, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 83040E9C: 3BE00002  li r31, 2
	ctx.r[31].s64 = 2;
	// 83040EA0: 90C3000C  stw r6, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[6].u32 ) };
	// 83040EA4: 392BE18C  addi r9, r11, -0x1e74
	ctx.r[9].s64 = ctx.r[11].s64 + -7796;
	// 83040EA8: 90E30018  stw r7, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[7].u32 ) };
	// 83040EAC: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83040EB0: 9083001C  stw r4, 0x1c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[4].u32 ) };
	// 83040EB4: 90A30020  stw r5, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[5].u32 ) };
	// 83040EB8: 394B8158  addi r10, r11, -0x7ea8
	ctx.r[10].s64 = ctx.r[11].s64 + -32424;
	// 83040EBC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83040EC0: B3E3000A  sth r31, 0xa(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(10 as u32), ctx.r[31].u16 ) };
	// 83040EC4: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 83040EC8: 91430030  stw r10, 0x30(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(48 as u32), ctx.r[10].u32 ) };
	// 83040ECC: 99630008  stb r11, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u8 ) };
	// 83040ED0: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 83040ED4: 91630014  stw r11, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 83040ED8: 91630024  stw r11, 0x24(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 83040EDC: 91630028  stw r11, 0x28(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 83040EE0: 9163002C  stw r11, 0x2c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 83040EE4: 91430034  stw r10, 0x34(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(52 as u32), ctx.r[10].u32 ) };
	// 83040EE8: 91630038  stw r11, 0x38(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 83040EEC: 9963003C  stb r11, 0x3c(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(60 as u32), ctx.r[11].u8 ) };
	// 83040EF0: 9963003D  stb r11, 0x3d(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(61 as u32), ctx.r[11].u8 ) };
	// 83040EF4: 9963003E  stb r11, 0x3e(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(62 as u32), ctx.r[11].u8 ) };
	// 83040EF8: EBE1FFF8  ld r31, -8(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) };
	// 83040EFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83040F00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83040F00 size=12
    let mut pc: u32 = 0x83040F00;
    'dispatch: loop {
        match pc {
            0x83040F00 => {
    //   block [0x83040F00..0x83040F0C)
	// 83040F00: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 83040F04: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 83040F08: 4BF90AC0  b 0x82fd19c8
	sub_82FD19C8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83040F10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83040F10 size=36
    let mut pc: u32 = 0x83040F10;
    'dispatch: loop {
        match pc {
            0x83040F10 => {
    //   block [0x83040F10..0x83040F34)
	// 83040F10: 7C8B0734  extsh r11, r4
	ctx.r[11].s64 = ctx.r[4].s16 as i64;
	// 83040F14: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 83040F18: 41980028  blt cr6, 0x83040f40
	if ctx.cr[6].lt {
		sub_83040F40(ctx, base);
		return;
	}
	// 83040F1C: 419A0018  beq cr6, 0x83040f34
	if ctx.cr[6].eq {
		sub_83040F34(ctx, base);
		return;
	}
	// 83040F20: 2B0B0003  cmplwi cr6, r11, 3
	ctx.cr[6].compare_u32(ctx.r[11].u32, 3 as u32, &mut ctx.xer);
	// 83040F24: 4098001C  bge cr6, 0x83040f40
	if !ctx.cr[6].lt {
		sub_83040F40(ctx, base);
		return;
	}
	// 83040F28: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83040F2C: 386BD5F8  addi r3, r11, -0x2a08
	ctx.r[3].s64 = ctx.r[11].s64 + -10760;
	// 83040F30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83040F34(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83040F34 size=12
    let mut pc: u32 = 0x83040F34;
    'dispatch: loop {
        match pc {
            0x83040F34 => {
    //   block [0x83040F34..0x83040F40)
	// 83040F34: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83040F38: 386BD60C  addi r3, r11, -0x29f4
	ctx.r[3].s64 = ctx.r[11].s64 + -10740;
	// 83040F3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83040F40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83040F40 size=12
    let mut pc: u32 = 0x83040F40;
    'dispatch: loop {
        match pc {
            0x83040F40 => {
    //   block [0x83040F40..0x83040F4C)
	// 83040F40: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83040F44: 386BD5E4  addi r3, r11, -0x2a1c
	ctx.r[3].s64 = ctx.r[11].s64 + -10780;
	// 83040F48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83040F50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83040F50 size=348
    let mut pc: u32 = 0x83040F50;
    'dispatch: loop {
        match pc {
            0x83040F50 => {
    //   block [0x83040F50..0x830410AC)
	// 83040F50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83040F54: 4816720D  bl 0x831a8160
	ctx.lr = 0x83040F58;
	sub_831A8130(ctx, base);
	// 83040F58: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83040F5C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83040F60: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 83040F64: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 83040F68: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 83040F6C: 809F002C  lwz r4, 0x2c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 83040F70: 28040000  cmplwi r4, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83040F74: 4182001C  beq 0x83040f90
	if ctx.cr[0].eq {
	pc = 0x83040F90; continue 'dispatch;
	}
	// 83040F78: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83040F7C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83040F80: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83040F84: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83040F88: 4E800421  bctrl
	ctx.lr = 0x83040F8C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83040F8C: 935F002C  stw r26, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[26].u32 ) };
	// 83040F90: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 83040F94: 409A0020  bne cr6, 0x83040fb4
	if !ctx.cr[6].eq {
	pc = 0x83040FB4; continue 'dispatch;
	}
	// 83040F98: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 83040F9C: 409A0048  bne cr6, 0x83040fe4
	if !ctx.cr[6].eq {
	pc = 0x83040FE4; continue 'dispatch;
	}
	// 83040FA0: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83040FA4: 396B8158  addi r11, r11, -0x7ea8
	ctx.r[11].s64 = ctx.r[11].s64 + -32424;
	// 83040FA8: 917F0030  stw r11, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 83040FAC: 917F0034  stw r11, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 83040FB0: 480000F4  b 0x830410a4
	pc = 0x830410A4; continue 'dispatch;
	// 83040FB4: A17B0000  lhz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 83040FB8: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83040FBC: 41820028  beq 0x83040fe4
	if ctx.cr[0].eq {
	pc = 0x83040FE4; continue 'dispatch;
	}
	// 83040FC0: 397B0002  addi r11, r27, 2
	ctx.r[11].s64 = ctx.r[27].s64 + 2;
	// 83040FC4: 48000008  b 0x83040fcc
	pc = 0x83040FCC; continue 'dispatch;
	// 83040FC8: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 83040FCC: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83040FD0: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83040FD4: 4082FFF4  bne 0x83040fc8
	if !ctx.cr[0].eq {
	pc = 0x83040FC8; continue 'dispatch;
	}
	// 83040FD8: 7D7B5850  subf r11, r27, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[27].s64;
	// 83040FDC: 7D7C0E70  srawi r28, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[28].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 83040FE0: 48000008  b 0x83040fe8
	pc = 0x83040FE8; continue 'dispatch;
	// 83040FE4: 7F5CD378  mr r28, r26
	ctx.r[28].u64 = ctx.r[26].u64;
	// 83040FE8: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 83040FEC: 419A0034  beq cr6, 0x83041020
	if ctx.cr[6].eq {
	pc = 0x83041020; continue 'dispatch;
	}
	// 83040FF0: A17D0000  lhz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 83040FF4: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83040FF8: 41820028  beq 0x83041020
	if ctx.cr[0].eq {
	pc = 0x83041020; continue 'dispatch;
	}
	// 83040FFC: 397D0002  addi r11, r29, 2
	ctx.r[11].s64 = ctx.r[29].s64 + 2;
	// 83041000: 48000008  b 0x83041008
	pc = 0x83041008; continue 'dispatch;
	// 83041004: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 83041008: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304100C: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83041010: 4082FFF4  bne 0x83041004
	if !ctx.cr[0].eq {
	pc = 0x83041004; continue 'dispatch;
	}
	// 83041014: 7D7D5850  subf r11, r29, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[29].s64;
	// 83041018: 7D7E0E70  srawi r30, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[30].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 8304101C: 48000008  b 0x83041024
	pc = 0x83041024; continue 'dispatch;
	// 83041020: 7F5ED378  mr r30, r26
	ctx.r[30].u64 = ctx.r[26].u64;
	// 83041024: 7D7EE214  add r11, r30, r28
	ctx.r[11].u64 = ctx.r[30].u64 + ctx.r[28].u64;
	// 83041028: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8304102C: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 83041030: 5564083C  slwi r4, r11, 1
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 83041034: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83041038: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8304103C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83041040: 4E800421  bctrl
	ctx.lr = 0x83041044;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83041044: 57DE083C  slwi r30, r30, 1
	ctx.r[30].u32 = ctx.r[30].u32.wrapping_shl(1);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 83041048: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 8304104C: 907F002C  stw r3, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[3].u32 ) };
	// 83041050: 7D7E1A14  add r11, r30, r3
	ctx.r[11].u64 = ctx.r[30].u64 + ctx.r[3].u64;
	// 83041054: 907F0034  stw r3, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[3].u32 ) };
	// 83041058: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 8304105C: 917F0030  stw r11, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 83041060: 419A0014  beq cr6, 0x83041074
	if ctx.cr[6].eq {
	pc = 0x83041074; continue 'dispatch;
	}
	// 83041064: 38BE0002  addi r5, r30, 2
	ctx.r[5].s64 = ctx.r[30].s64 + 2;
	// 83041068: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8304106C: 481674A5  bl 0x831a8510
	ctx.lr = 0x83041070;
	sub_831A8510(ctx, base);
	// 83041070: 48000008  b 0x83041078
	pc = 0x83041078; continue 'dispatch;
	// 83041074: B3430000  sth r26, 0(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[26].u16 ) };
	// 83041078: 817F002C  lwz r11, 0x2c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 8304107C: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 83041080: 7D7E5A14  add r11, r30, r11
	ctx.r[11].u64 = ctx.r[30].u64 + ctx.r[11].u64;
	// 83041084: 419A001C  beq cr6, 0x830410a0
	if ctx.cr[6].eq {
	pc = 0x830410A0; continue 'dispatch;
	}
	// 83041088: 395C0001  addi r10, r28, 1
	ctx.r[10].s64 = ctx.r[28].s64 + 1;
	// 8304108C: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 83041090: 5545083C  slwi r5, r10, 1
	ctx.r[5].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 83041094: 386B0002  addi r3, r11, 2
	ctx.r[3].s64 = ctx.r[11].s64 + 2;
	// 83041098: 48167479  bl 0x831a8510
	ctx.lr = 0x8304109C;
	sub_831A8510(ctx, base);
	// 8304109C: 48000008  b 0x830410a4
	pc = 0x830410A4; continue 'dispatch;
	// 830410A0: B34B0002  sth r26, 2(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(2 as u32), ctx.r[26].u16 ) };
	// 830410A4: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 830410A8: 48167108  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830410B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830410B0 size=272
    let mut pc: u32 = 0x830410B0;
    'dispatch: loop {
        match pc {
            0x830410B0 => {
    //   block [0x830410B0..0x830411C0)
	// 830410B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830410B4: 481670B1  bl 0x831a8164
	ctx.lr = 0x830410B8;
	sub_831A8130(ctx, base);
	// 830410B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830410BC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830410C0: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 830410C4: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 830410C8: 809F002C  lwz r4, 0x2c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 830410CC: 28040000  cmplwi r4, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830410D0: 4182001C  beq 0x830410ec
	if ctx.cr[0].eq {
	pc = 0x830410EC; continue 'dispatch;
	}
	// 830410D4: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830410D8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830410DC: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 830410E0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830410E4: 4E800421  bctrl
	ctx.lr = 0x830410E8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830410E8: 937F002C  stw r27, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[27].u32 ) };
	// 830410EC: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 830410F0: 419A00B8  beq cr6, 0x830411a8
	if ctx.cr[6].eq {
	pc = 0x830411A8; continue 'dispatch;
	}
	// 830410F4: A17C0000  lhz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 830410F8: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830410FC: 41820028  beq 0x83041124
	if ctx.cr[0].eq {
	pc = 0x83041124; continue 'dispatch;
	}
	// 83041100: 397C0002  addi r11, r28, 2
	ctx.r[11].s64 = ctx.r[28].s64 + 2;
	// 83041104: 48000008  b 0x8304110c
	pc = 0x8304110C; continue 'dispatch;
	// 83041108: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 8304110C: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83041110: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83041114: 4082FFF4  bne 0x83041108
	if !ctx.cr[0].eq {
	pc = 0x83041108; continue 'dispatch;
	}
	// 83041118: 7D7C5850  subf r11, r28, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[28].s64;
	// 8304111C: 7D7E0E70  srawi r30, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[30].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 83041120: 48000008  b 0x83041128
	pc = 0x83041128; continue 'dispatch;
	// 83041124: 7F7EDB78  mr r30, r27
	ctx.r[30].u64 = ctx.r[27].u64;
	// 83041128: 3880002C  li r4, 0x2c
	ctx.r[4].s64 = 44;
	// 8304112C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83041130: 4BF90C81  bl 0x82fd1db0
	ctx.lr = 0x83041134;
	sub_82FD1DB0(ctx, base);
	// 83041134: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83041138: 395E0001  addi r10, r30, 1
	ctx.r[10].s64 = ctx.r[30].s64 + 1;
	// 8304113C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83041140: 555E083C  slwi r30, r10, 1
	ctx.r[30].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 83041144: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 83041148: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304114C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83041150: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 83041154: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83041158: 4E800421  bctrl
	ctx.lr = 0x8304115C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8304115C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 83041160: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 83041164: 907F002C  stw r3, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[3].u32 ) };
	// 83041168: 481673A9  bl 0x831a8510
	ctx.lr = 0x8304116C;
	sub_831A8510(ctx, base);
	// 8304116C: 2F1DFFFF  cmpwi cr6, r29, -1
	ctx.cr[6].compare_i32(ctx.r[29].s32, -1, &mut ctx.xer);
	// 83041170: 409A0018  bne cr6, 0x83041188
	if !ctx.cr[6].eq {
	pc = 0x83041188; continue 'dispatch;
	}
	// 83041174: 815F002C  lwz r10, 0x2c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 83041178: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 8304117C: 396BCC98  addi r11, r11, -0x3368
	ctx.r[11].s64 = ctx.r[11].s64 + -13160;
	// 83041180: 915F0030  stw r10, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[10].u32 ) };
	// 83041184: 48000030  b 0x830411b4
	pc = 0x830411B4; continue 'dispatch;
	// 83041188: 817F002C  lwz r11, 0x2c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 8304118C: 57AA083C  slwi r10, r29, 1
	ctx.r[10].u32 = ctx.r[29].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83041190: 7D4A5A14  add r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 83041194: 917F0034  stw r11, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 83041198: 396A0002  addi r11, r10, 2
	ctx.r[11].s64 = ctx.r[10].s64 + 2;
	// 8304119C: 917F0030  stw r11, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 830411A0: B36A0000  sth r27, 0(r10)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[27].u16 ) };
	// 830411A4: 48000014  b 0x830411b8
	pc = 0x830411B8; continue 'dispatch;
	// 830411A8: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 830411AC: 396B8158  addi r11, r11, -0x7ea8
	ctx.r[11].s64 = ctx.r[11].s64 + -32424;
	// 830411B0: 917F0030  stw r11, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 830411B4: 917F0034  stw r11, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 830411B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 830411BC: 48166FF8  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830411C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830411C0 size=12
    let mut pc: u32 = 0x830411C0;
    'dispatch: loop {
        match pc {
            0x830411C0 => {
    //   block [0x830411C0..0x830411CC)
	// 830411C0: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 830411C4: 386B32DC  addi r3, r11, 0x32dc
	ctx.r[3].s64 = ctx.r[11].s64 + 13020;
	// 830411C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830411D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830411D0 size=8
    let mut pc: u32 = 0x830411D0;
    'dispatch: loop {
        match pc {
            0x830411D0 => {
    //   block [0x830411D0..0x830411D8)
	// 830411D0: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830411D4: 8215E1EC  lwz r16, -0x1e14(r21)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-7700 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830411D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830411D8 size=112
    let mut pc: u32 = 0x830411D8;
    'dispatch: loop {
        match pc {
            0x830411D8 => {
    //   block [0x830411D8..0x83041248)
	// 830411D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830411DC: 38000000  li r0, 0
	ctx.r[0].s64 = 0;
	// 830411E0: 90010004  stw r0, 4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(4 as u32), ctx.r[0].u32 ) };
	// 830411E4: 48166F89  bl 0x831a816c
	ctx.lr = 0x830411E8;
	sub_831A8130(ctx, base);
	// 830411E8: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 830411EC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830411F0: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 830411F4: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 830411F8: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 830411FC: 409A0008  bne cr6, 0x83041204
	if !ctx.cr[6].eq {
	pc = 0x83041204; continue 'dispatch;
	}
	// 83041200: 83C30004  lwz r30, 4(r3)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 83041204: 54CB063F  clrlwi. r11, r6, 0x18
	ctx.r[11].u64 = ctx.r[6].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83041208: 41820024  beq 0x8304122c
	if ctx.cr[0].eq {
	pc = 0x8304122C; continue 'dispatch;
	}
	// 8304120C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83041210: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 83041214: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83041218: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8304121C: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 83041220: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83041224: 4E800421  bctrl
	ctx.lr = 0x83041228;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83041228: 7D084378  mr r8, r8
	ctx.r[8].u64 = ctx.r[8].u64;
	// 8304122C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83041230: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83041234: 4BF8F94D  bl 0x82fd0b80
	ctx.lr = 0x83041238;
	sub_82FD0B80(ctx, base);
	// 83041238: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 8304123C: 48166F80  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 83041240: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83041244: 4BFFFFF4  b 0x83041238
	pc = 0x83041238; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83041248(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83041248 size=8
    let mut pc: u32 = 0x83041248;
    'dispatch: loop {
        match pc {
            0x83041248 => {
    //   block [0x83041248..0x83041250)
	// 83041248: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8304124C: 8215E1EC  lwz r16, -0x1e14(r21)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-7700 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83041250(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83041250 size=20
    let mut pc: u32 = 0x83041250;
    'dispatch: loop {
        match pc {
            0x83041250 => {
    //   block [0x83041250..0x83041264)
	// 83041250: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83041254: 3C608304  lis r3, -0x7cfc
	ctx.r[3].s64 = -2096889856;
	// 83041258: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8304125C: 38631240  addi r3, r3, 0x1240
	ctx.r[3].s64 = ctx.r[3].s64 + 4672;
	// 83041260: 48166F5C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83041268(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83041268 size=188
    let mut pc: u32 = 0x83041268;
    'dispatch: loop {
        match pc {
            0x83041268 => {
    //   block [0x83041268..0x83041324)
	// 83041268: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304126C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83041270: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83041274: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83041278: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304127C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 83041280: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83041284: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 83041288: 419A0078  beq cr6, 0x83041300
	if ctx.cr[6].eq {
	pc = 0x83041300; continue 'dispatch;
	}
	// 8304128C: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 83041290: 809F0030  lwz r4, 0x30(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 83041294: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 83041298: 806BB944  lwz r3, -0x46bc(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18108 as u32) ) } as u64;
	// 8304129C: 4BFB91C5  bl 0x82ffa460
	ctx.lr = 0x830412A0;
	sub_82FFA460(ctx, base);
	// 830412A0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830412A4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830412A8: 41820008  beq 0x830412b0
	if ctx.cr[0].eq {
	pc = 0x830412B0; continue 'dispatch;
	}
	// 830412AC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830412B0: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 830412B4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830412B8: 409A0024  bne cr6, 0x830412dc
	if !ctx.cr[6].eq {
	pc = 0x830412DC; continue 'dispatch;
	}
	// 830412BC: 3880FFFF  li r4, -1
	ctx.r[4].s64 = -1;
	// 830412C0: 4BFB8039  bl 0x82ff92f8
	ctx.lr = 0x830412C4;
	sub_82FF92F8(ctx, base);
	// 830412C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 830412C8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 830412CC: 809F0030  lwz r4, 0x30(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 830412D0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830412D4: 4BFB862D  bl 0x82ff9900
	ctx.lr = 0x830412D8;
	sub_82FF9900(ctx, base);
	// 830412D8: 48000034  b 0x8304130c
	pc = 0x8304130C; continue 'dispatch;
	// 830412DC: 3880FFFE  li r4, -2
	ctx.r[4].s64 = -2;
	// 830412E0: 4BFB8019  bl 0x82ff92f8
	ctx.lr = 0x830412E4;
	sub_82FF92F8(ctx, base);
	// 830412E4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830412E8: 809F0018  lwz r4, 0x18(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 830412EC: 4BFB800D  bl 0x82ff92f8
	ctx.lr = 0x830412F0;
	sub_82FF92F8(ctx, base);
	// 830412F0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 830412F4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830412F8: 4BFB8909  bl 0x82ff9c00
	ctx.lr = 0x830412FC;
	sub_82FF9C00(ctx, base);
	// 830412FC: 48000010  b 0x8304130c
	pc = 0x8304130C; continue 'dispatch;
	// 83041300: 3880FFFD  li r4, -3
	ctx.r[4].s64 = -3;
	// 83041304: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83041308: 4BFB7FF1  bl 0x82ff92f8
	ctx.lr = 0x8304130C;
	sub_82FF92F8(ctx, base);
	// 8304130C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83041310: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83041314: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83041318: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8304131C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83041320: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83041328(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83041328 size=8
    let mut pc: u32 = 0x83041328;
    'dispatch: loop {
        match pc {
            0x83041328 => {
    //   block [0x83041328..0x83041330)
	// 83041328: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 8304132C: 8215E260  lwz r16, -0x1da0(r21)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-7584 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83041330(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83041330 size=576
    let mut pc: u32 = 0x83041330;
    'dispatch: loop {
        match pc {
            0x83041330 => {
    //   block [0x83041330..0x83041570)
	// 83041330: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83041334: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83041338: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8304133C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83041340: 3BE1FF70  addi r31, r1, -0x90
	ctx.r[31].s64 = ctx.r[1].s64 + -144;
	// 83041344: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83041348: 389F0050  addi r4, r31, 0x50
	ctx.r[4].s64 = ctx.r[31].s64 + 80;
	// 8304134C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83041350: 4BFB8229  bl 0x82ff9578
	ctx.lr = 0x83041354;
	sub_82FF9578(ctx, base);
	// 83041354: 817F0050  lwz r11, 0x50(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 83041358: 2F0BFFFF  cmpwi cr6, r11, -1
	ctx.cr[6].compare_i32(ctx.r[11].s32, -1, &mut ctx.xer);
	// 8304135C: 409A0064  bne cr6, 0x830413c0
	if !ctx.cr[6].eq {
	pc = 0x830413C0; continue 'dispatch;
	}
	// 83041360: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83041364: 38DF0060  addi r6, r31, 0x60
	ctx.r[6].s64 = ctx.r[31].s64 + 96;
	// 83041368: 38BF0068  addi r5, r31, 0x68
	ctx.r[5].s64 = ctx.r[31].s64 + 104;
	// 8304136C: 389F0054  addi r4, r31, 0x54
	ctx.r[4].s64 = ctx.r[31].s64 + 84;
	// 83041370: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83041374: 4BFB87B5  bl 0x82ff9b28
	ctx.lr = 0x83041378;
	sub_82FF9B28(ctx, base);
	// 83041378: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8304137C: 4BFB6D35  bl 0x82ff80b0
	ctx.lr = 0x83041380;
	sub_82FF80B0(ctx, base);
	// 83041380: 809F0054  lwz r4, 0x54(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 83041384: 907F0064  stw r3, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[3].u32 ) };
	// 83041388: 909F0060  stw r4, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[4].u32 ) };
	// 8304138C: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 83041390: 38BF0068  addi r5, r31, 0x68
	ctx.r[5].s64 = ctx.r[31].s64 + 104;
	// 83041394: 806BB944  lwz r3, -0x46bc(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18108 as u32) ) } as u64;
	// 83041398: 4BFB90C9  bl 0x82ffa460
	ctx.lr = 0x8304139C;
	sub_82FFA460(ctx, base);
	// 8304139C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830413A0: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 830413A4: 41820008  beq 0x830413ac
	if ctx.cr[0].eq {
	pc = 0x830413AC; continue 'dispatch;
	}
	// 830413A8: 83C30000  lwz r30, 0(r3)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830413AC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830413B0: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 830413B4: 4BF9170D  bl 0x82fd2ac0
	ctx.lr = 0x830413B8;
	sub_82FD2AC0(ctx, base);
	// 830413B8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830413BC: 4800019C  b 0x83041558
	pc = 0x83041558; continue 'dispatch;
	// 830413C0: 2F0BFFFD  cmpwi cr6, r11, -3
	ctx.cr[6].compare_i32(ctx.r[11].s32, -3, &mut ctx.xer);
	// 830413C4: 419A0190  beq cr6, 0x83041554
	if ctx.cr[6].eq {
	pc = 0x83041554; continue 'dispatch;
	}
	// 830413C8: 389F0058  addi r4, r31, 0x58
	ctx.r[4].s64 = ctx.r[31].s64 + 88;
	// 830413CC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830413D0: 4BFB81A9  bl 0x82ff9578
	ctx.lr = 0x830413D4;
	sub_82FF9578(ctx, base);
	// 830413D4: 817F0058  lwz r11, 0x58(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 830413D8: 2B0B001B  cmplwi cr6, r11, 0x1b
	ctx.cr[6].compare_u32(ctx.r[11].u32, 27 as u32, &mut ctx.xer);
	// 830413DC: 41990178  bgt cr6, 0x83041554
	if ctx.cr[6].gt {
	pc = 0x83041554; continue 'dispatch;
	}
	// 830413E0: 3D808216  lis r12, -0x7dea
	ctx.r[12].s64 = -2112487424;
	// 830413E4: 398CE238  addi r12, r12, -0x1dc8
	ctx.r[12].s64 = ctx.r[12].s64 + -7624;
	// 830413E8: 7C0C58AE  lbzx r0, r12, r11
	ctx.r[0].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[12].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 830413EC: 5400103A  slwi r0, r0, 2
	ctx.r[0].u32 = ctx.r[0].u32.wrapping_shl(2);
	ctx.r[0].u64 = ctx.r[0].u32 as u64;
	// 830413F0: 3D808304  lis r12, -0x7cfc
	ctx.r[12].s64 = -2096889856;
	// 830413F4: 398C1408  addi r12, r12, 0x1408
	ctx.r[12].s64 = ctx.r[12].s64 + 5128;
	// 830413F8: 7D8C0214  add r12, r12, r0
	ctx.r[12].u64 = ctx.r[12].u64 + ctx.r[0].u64;
	// 830413FC: 7D8903A6  mtctr r12
	ctx.ctr.u64 = ctx.r[12].u64;
	// 83041400: 60000000  nop
	// 83041404: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
	// 83041408: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 8304140C: 388B33F4  addi r4, r11, 0x33f4
	ctx.r[4].s64 = ctx.r[11].s64 + 13300;
	// 83041410: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83041414: 4BFB88AD  bl 0x82ff9cc0
	ctx.lr = 0x83041418;
	sub_82FF9CC0(ctx, base);
	// 83041418: 48000140  b 0x83041558
	pc = 0x83041558; continue 'dispatch;
	// 8304141C: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 83041420: 388B33B4  addi r4, r11, 0x33b4
	ctx.r[4].s64 = ctx.r[11].s64 + 13236;
	// 83041424: 4BFFFFEC  b 0x83041410
	pc = 0x83041410; continue 'dispatch;
	// 83041428: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 8304142C: 388B33AC  addi r4, r11, 0x33ac
	ctx.r[4].s64 = ctx.r[11].s64 + 13228;
	// 83041430: 4BFFFFE0  b 0x83041410
	pc = 0x83041410; continue 'dispatch;
	// 83041434: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 83041438: 388B3354  addi r4, r11, 0x3354
	ctx.r[4].s64 = ctx.r[11].s64 + 13140;
	// 8304143C: 4BFFFFD4  b 0x83041410
	pc = 0x83041410; continue 'dispatch;
	// 83041440: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 83041444: 388B334C  addi r4, r11, 0x334c
	ctx.r[4].s64 = ctx.r[11].s64 + 13132;
	// 83041448: 4BFFFFC8  b 0x83041410
	pc = 0x83041410; continue 'dispatch;
	// 8304144C: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 83041450: 388B32E4  addi r4, r11, 0x32e4
	ctx.r[4].s64 = ctx.r[11].s64 + 13028;
	// 83041454: 4BFFFFBC  b 0x83041410
	pc = 0x83041410; continue 'dispatch;
	// 83041458: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 8304145C: 388B33BC  addi r4, r11, 0x33bc
	ctx.r[4].s64 = ctx.r[11].s64 + 13244;
	// 83041460: 4BFFFFB0  b 0x83041410
	pc = 0x83041410; continue 'dispatch;
	// 83041464: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 83041468: 388B33C4  addi r4, r11, 0x33c4
	ctx.r[4].s64 = ctx.r[11].s64 + 13252;
	// 8304146C: 4BFFFFA4  b 0x83041410
	pc = 0x83041410; continue 'dispatch;
	// 83041470: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 83041474: 388B33DC  addi r4, r11, 0x33dc
	ctx.r[4].s64 = ctx.r[11].s64 + 13276;
	// 83041478: 4BFFFF98  b 0x83041410
	pc = 0x83041410; continue 'dispatch;
	// 8304147C: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 83041480: 388B33D4  addi r4, r11, 0x33d4
	ctx.r[4].s64 = ctx.r[11].s64 + 13268;
	// 83041484: 4BFFFF8C  b 0x83041410
	pc = 0x83041410; continue 'dispatch;
	// 83041488: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 8304148C: 388B33CC  addi r4, r11, 0x33cc
	ctx.r[4].s64 = ctx.r[11].s64 + 13260;
	// 83041490: 4BFFFF80  b 0x83041410
	pc = 0x83041410; continue 'dispatch;
	// 83041494: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 83041498: 388B3364  addi r4, r11, 0x3364
	ctx.r[4].s64 = ctx.r[11].s64 + 13156;
	// 8304149C: 4BFFFF74  b 0x83041410
	pc = 0x83041410; continue 'dispatch;
	// 830414A0: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 830414A4: 388B33A4  addi r4, r11, 0x33a4
	ctx.r[4].s64 = ctx.r[11].s64 + 13220;
	// 830414A8: 4BFFFF68  b 0x83041410
	pc = 0x83041410; continue 'dispatch;
	// 830414AC: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 830414B0: 388B339C  addi r4, r11, 0x339c
	ctx.r[4].s64 = ctx.r[11].s64 + 13212;
	// 830414B4: 4BFFFF5C  b 0x83041410
	pc = 0x83041410; continue 'dispatch;
	// 830414B8: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 830414BC: 388B3394  addi r4, r11, 0x3394
	ctx.r[4].s64 = ctx.r[11].s64 + 13204;
	// 830414C0: 4BFFFF50  b 0x83041410
	pc = 0x83041410; continue 'dispatch;
	// 830414C4: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 830414C8: 388B337C  addi r4, r11, 0x337c
	ctx.r[4].s64 = ctx.r[11].s64 + 13180;
	// 830414CC: 4BFFFF44  b 0x83041410
	pc = 0x83041410; continue 'dispatch;
	// 830414D0: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 830414D4: 388B336C  addi r4, r11, 0x336c
	ctx.r[4].s64 = ctx.r[11].s64 + 13164;
	// 830414D8: 4BFFFF38  b 0x83041410
	pc = 0x83041410; continue 'dispatch;
	// 830414DC: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 830414E0: 388B3374  addi r4, r11, 0x3374
	ctx.r[4].s64 = ctx.r[11].s64 + 13172;
	// 830414E4: 4BFFFF2C  b 0x83041410
	pc = 0x83041410; continue 'dispatch;
	// 830414E8: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 830414EC: 388B3384  addi r4, r11, 0x3384
	ctx.r[4].s64 = ctx.r[11].s64 + 13188;
	// 830414F0: 4BFFFF20  b 0x83041410
	pc = 0x83041410; continue 'dispatch;
	// 830414F4: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 830414F8: 388B338C  addi r4, r11, 0x338c
	ctx.r[4].s64 = ctx.r[11].s64 + 13196;
	// 830414FC: 4BFFFF14  b 0x83041410
	pc = 0x83041410; continue 'dispatch;
	// 83041500: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 83041504: 388B3344  addi r4, r11, 0x3344
	ctx.r[4].s64 = ctx.r[11].s64 + 13124;
	// 83041508: 4BFFFF08  b 0x83041410
	pc = 0x83041410; continue 'dispatch;
	// 8304150C: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 83041510: 388B333C  addi r4, r11, 0x333c
	ctx.r[4].s64 = ctx.r[11].s64 + 13116;
	// 83041514: 4BFFFEFC  b 0x83041410
	pc = 0x83041410; continue 'dispatch;
	// 83041518: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 8304151C: 388B3334  addi r4, r11, 0x3334
	ctx.r[4].s64 = ctx.r[11].s64 + 13108;
	// 83041520: 4BFFFEF0  b 0x83041410
	pc = 0x83041410; continue 'dispatch;
	// 83041524: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 83041528: 388B33EC  addi r4, r11, 0x33ec
	ctx.r[4].s64 = ctx.r[11].s64 + 13292;
	// 8304152C: 4BFFFEE4  b 0x83041410
	pc = 0x83041410; continue 'dispatch;
	// 83041530: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 83041534: 388B332C  addi r4, r11, 0x332c
	ctx.r[4].s64 = ctx.r[11].s64 + 13100;
	// 83041538: 4BFFFED8  b 0x83041410
	pc = 0x83041410; continue 'dispatch;
	// 8304153C: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 83041540: 388B3324  addi r4, r11, 0x3324
	ctx.r[4].s64 = ctx.r[11].s64 + 13092;
	// 83041544: 4BFFFECC  b 0x83041410
	pc = 0x83041410; continue 'dispatch;
	// 83041548: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 8304154C: 388B33E4  addi r4, r11, 0x33e4
	ctx.r[4].s64 = ctx.r[11].s64 + 13284;
	// 83041550: 4BFFFEC0  b 0x83041410
	pc = 0x83041410; continue 'dispatch;
	// 83041554: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83041558: 383F0090  addi r1, r31, 0x90
	ctx.r[1].s64 = ctx.r[31].s64 + 144;
	// 8304155C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83041560: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83041564: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83041568: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8304156C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83041570(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83041570 size=40
    let mut pc: u32 = 0x83041570;
    'dispatch: loop {
        match pc {
            0x83041570 => {
    //   block [0x83041570..0x83041598)
	// 83041570: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 83041574: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83041578: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8304157C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83041580: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 83041584: 4BF918D5  bl 0x82fd2e58
	ctx.lr = 0x83041588;
	sub_82FD2E58(ctx, base);
	// 83041588: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8304158C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83041590: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83041594: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83041598(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83041598 size=168
    let mut pc: u32 = 0x83041598;
    'dispatch: loop {
        match pc {
            0x83041598 => {
    //   block [0x83041598..0x83041640)
	// 83041598: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304159C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830415A0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830415A4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830415A8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830415AC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830415B0: 83DF0020  lwz r30, 0x20(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 830415B4: 281E0000  cmplwi r30, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830415B8: 41820014  beq 0x830415cc
	if ctx.cr[0].eq {
	pc = 0x830415CC; continue 'dispatch;
	}
	// 830415BC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830415C0: 4BFA56A9  bl 0x82fe6c68
	ctx.lr = 0x830415C4;
	sub_82FE6C68(ctx, base);
	// 830415C4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830415C8: 4BF96D19  bl 0x82fd82e0
	ctx.lr = 0x830415CC;
	sub_82FD82E0(ctx, base);
	// 830415CC: 83DF0028  lwz r30, 0x28(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 830415D0: 281E0000  cmplwi r30, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830415D4: 41820014  beq 0x830415e8
	if ctx.cr[0].eq {
	pc = 0x830415E8; continue 'dispatch;
	}
	// 830415D8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830415DC: 48047BA5  bl 0x83089180
	ctx.lr = 0x830415E0;
	sub_83089180(ctx, base);
	// 830415E0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830415E4: 4BF96CFD  bl 0x82fd82e0
	ctx.lr = 0x830415E8;
	sub_82FD82E0(ctx, base);
	// 830415E8: 809F0024  lwz r4, 0x24(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 830415EC: 28040000  cmplwi r4, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830415F0: 41820018  beq 0x83041608
	if ctx.cr[0].eq {
	pc = 0x83041608; continue 'dispatch;
	}
	// 830415F4: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830415F8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830415FC: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83041600: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83041604: 4E800421  bctrl
	ctx.lr = 0x83041608;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83041608: 809F002C  lwz r4, 0x2c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 8304160C: 28040000  cmplwi r4, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83041610: 41820018  beq 0x83041628
	if ctx.cr[0].eq {
	pc = 0x83041628; continue 'dispatch;
	}
	// 83041614: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83041618: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304161C: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83041620: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83041624: 4E800421  bctrl
	ctx.lr = 0x83041628;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83041628: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8304162C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83041630: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83041634: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83041638: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8304163C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83041640(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83041640 size=8
    let mut pc: u32 = 0x83041640;
    'dispatch: loop {
        match pc {
            0x83041640 => {
    //   block [0x83041640..0x83041648)
	// 83041640: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83041644: 8215E2C0  lwz r16, -0x1d40(r21)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-7488 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83041648(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83041648 size=780
    let mut pc: u32 = 0x83041648;
    'dispatch: loop {
        match pc {
            0x83041648 => {
    //   block [0x83041648..0x83041954)
	// 83041648: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304164C: 48166B1D  bl 0x831a8168
	ctx.lr = 0x83041650;
	sub_831A8130(ctx, base);
	// 83041650: 3BE1FF60  addi r31, r1, -0xa0
	ctx.r[31].s64 = ctx.r[1].s64 + -160;
	// 83041654: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83041658: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8304165C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83041660: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83041664: A97D0000  lha r11, 0(r29)
	ctx.r[11].s64 = (unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as i16) as i64;
	// 83041668: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 8304166C: 556BDFFF  rlwinm. r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83041670: 4182011C  beq 0x8304178c
	if ctx.cr[0].eq {
	pc = 0x8304178C; continue 'dispatch;
	}
	// 83041674: 889E0008  lbz r4, 8(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 83041678: 4BFB7B89  bl 0x82ff9200
	ctx.lr = 0x8304167C;
	sub_82FF9200(ctx, base);
	// 8304167C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83041680: A09E000A  lhz r4, 0xa(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(10 as u32) ) } as u64;
	// 83041684: 4BFB7BE5  bl 0x82ff9268
	ctx.lr = 0x83041688;
	sub_82FF9268(ctx, base);
	// 83041688: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8304168C: 809E000C  lwz r4, 0xc(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 83041690: 4BFB7C69  bl 0x82ff92f8
	ctx.lr = 0x83041694;
	sub_82FF92F8(ctx, base);
	// 83041694: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83041698: 809E0010  lwz r4, 0x10(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 8304169C: 4BFB7C5D  bl 0x82ff92f8
	ctx.lr = 0x830416A0;
	sub_82FF92F8(ctx, base);
	// 830416A0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830416A4: 809E0014  lwz r4, 0x14(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 830416A8: 4BFB7C51  bl 0x82ff92f8
	ctx.lr = 0x830416AC;
	sub_82FF92F8(ctx, base);
	// 830416AC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830416B0: 809E0018  lwz r4, 0x18(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 830416B4: 4BFB7C45  bl 0x82ff92f8
	ctx.lr = 0x830416B8;
	sub_82FF92F8(ctx, base);
	// 830416B8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830416BC: 809E0038  lwz r4, 0x38(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(56 as u32) ) } as u64;
	// 830416C0: 4BFB7C39  bl 0x82ff92f8
	ctx.lr = 0x830416C4;
	sub_82FF92F8(ctx, base);
	// 830416C4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830416C8: 889E003C  lbz r4, 0x3c(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(60 as u32) ) } as u64;
	// 830416CC: 4BFB7B35  bl 0x82ff9200
	ctx.lr = 0x830416D0;
	sub_82FF9200(ctx, base);
	// 830416D0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830416D4: 889E003D  lbz r4, 0x3d(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(61 as u32) ) } as u64;
	// 830416D8: 4BFB7B29  bl 0x82ff9200
	ctx.lr = 0x830416DC;
	sub_82FF9200(ctx, base);
	// 830416DC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830416E0: 889E003E  lbz r4, 0x3e(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(62 as u32) ) } as u64;
	// 830416E4: 4BFB7B1D  bl 0x82ff9200
	ctx.lr = 0x830416E8;
	sub_82FF9200(ctx, base);
	// 830416E8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830416EC: 809E001C  lwz r4, 0x1c(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 830416F0: 4BFFFB79  bl 0x83041268
	ctx.lr = 0x830416F4;
	sub_83041268(ctx, base);
	// 830416F4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 830416F8: 807E0020  lwz r3, 0x20(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 830416FC: 4800E4E5  bl 0x8304fbe0
	ctx.lr = 0x83041700;
	sub_8304FBE0(ctx, base);
	// 83041700: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 83041704: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83041708: 809E0024  lwz r4, 0x24(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 8304170C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83041710: 4BFB81F1  bl 0x82ff9900
	ctx.lr = 0x83041714;
	sub_82FF9900(ctx, base);
	// 83041714: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83041718: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8304171C: 394B8158  addi r10, r11, -0x7ea8
	ctx.r[10].s64 = ctx.r[11].s64 + -32424;
	// 83041720: 817E0034  lwz r11, 0x34(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(52 as u32) ) } as u64;
	// 83041724: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 83041728: 409A0010  bne cr6, 0x83041738
	if !ctx.cr[6].eq {
	pc = 0x83041738; continue 'dispatch;
	}
	// 8304172C: 3880FFFF  li r4, -1
	ctx.r[4].s64 = -1;
	// 83041730: 4BFB7BC9  bl 0x82ff92f8
	ctx.lr = 0x83041734;
	sub_82FF92F8(ctx, base);
	// 83041734: 48000218  b 0x8304194c
	pc = 0x8304194C; continue 'dispatch;
	// 83041738: 3D408214  lis r10, -0x7dec
	ctx.r[10].s64 = -2112618496;
	// 8304173C: 394ACC98  addi r10, r10, -0x3368
	ctx.r[10].s64 = ctx.r[10].s64 + -13160;
	// 83041740: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 83041744: 409A0014  bne cr6, 0x83041758
	if !ctx.cr[6].eq {
	pc = 0x83041758; continue 'dispatch;
	}
	// 83041748: 3880FFFE  li r4, -2
	ctx.r[4].s64 = -2;
	// 8304174C: 4BFB7BAD  bl 0x82ff92f8
	ctx.lr = 0x83041750;
	sub_82FF92F8(ctx, base);
	// 83041750: 809E0030  lwz r4, 0x30(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 83041754: 48000024  b 0x83041778
	pc = 0x83041778; continue 'dispatch;
	// 83041758: 3880FFFD  li r4, -3
	ctx.r[4].s64 = -3;
	// 8304175C: 4BFB7B9D  bl 0x82ff92f8
	ctx.lr = 0x83041760;
	sub_82FF92F8(ctx, base);
	// 83041760: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 83041764: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83041768: 809E0030  lwz r4, 0x30(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 8304176C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83041770: 4BFB8191  bl 0x82ff9900
	ctx.lr = 0x83041774;
	sub_82FF9900(ctx, base);
	// 83041774: 809E0034  lwz r4, 0x34(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(52 as u32) ) } as u64;
	// 83041778: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8304177C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 83041780: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83041784: 4BFB817D  bl 0x82ff9900
	ctx.lr = 0x83041788;
	sub_82FF9900(ctx, base);
	// 83041788: 480001C4  b 0x8304194c
	pc = 0x8304194C; continue 'dispatch;
	// 8304178C: 389E0008  addi r4, r30, 8
	ctx.r[4].s64 = ctx.r[30].s64 + 8;
	// 83041790: 4BFB7CF1  bl 0x82ff9480
	ctx.lr = 0x83041794;
	sub_82FF9480(ctx, base);
	// 83041794: 389E000A  addi r4, r30, 0xa
	ctx.r[4].s64 = ctx.r[30].s64 + 10;
	// 83041798: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8304179C: 4BFB7D4D  bl 0x82ff94e8
	ctx.lr = 0x830417A0;
	sub_82FF94E8(ctx, base);
	// 830417A0: 389E000C  addi r4, r30, 0xc
	ctx.r[4].s64 = ctx.r[30].s64 + 12;
	// 830417A4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830417A8: 4BFB7DD1  bl 0x82ff9578
	ctx.lr = 0x830417AC;
	sub_82FF9578(ctx, base);
	// 830417AC: 389E0010  addi r4, r30, 0x10
	ctx.r[4].s64 = ctx.r[30].s64 + 16;
	// 830417B0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830417B4: 4BFB7DC5  bl 0x82ff9578
	ctx.lr = 0x830417B8;
	sub_82FF9578(ctx, base);
	// 830417B8: 389E0014  addi r4, r30, 0x14
	ctx.r[4].s64 = ctx.r[30].s64 + 20;
	// 830417BC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830417C0: 4BFB7DB9  bl 0x82ff9578
	ctx.lr = 0x830417C4;
	sub_82FF9578(ctx, base);
	// 830417C4: 389F0050  addi r4, r31, 0x50
	ctx.r[4].s64 = ctx.r[31].s64 + 80;
	// 830417C8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830417CC: 4BFB7DAD  bl 0x82ff9578
	ctx.lr = 0x830417D0;
	sub_82FF9578(ctx, base);
	// 830417D0: 817F0050  lwz r11, 0x50(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 830417D4: 389F0050  addi r4, r31, 0x50
	ctx.r[4].s64 = ctx.r[31].s64 + 80;
	// 830417D8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830417DC: 917E0018  stw r11, 0x18(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 830417E0: 4BFB7D99  bl 0x82ff9578
	ctx.lr = 0x830417E4;
	sub_82FF9578(ctx, base);
	// 830417E4: 817F0050  lwz r11, 0x50(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 830417E8: 389E003C  addi r4, r30, 0x3c
	ctx.r[4].s64 = ctx.r[30].s64 + 60;
	// 830417EC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830417F0: 917E0038  stw r11, 0x38(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 830417F4: 4BFB7C8D  bl 0x82ff9480
	ctx.lr = 0x830417F8;
	sub_82FF9480(ctx, base);
	// 830417F8: 389E003D  addi r4, r30, 0x3d
	ctx.r[4].s64 = ctx.r[30].s64 + 61;
	// 830417FC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83041800: 4BFB7C81  bl 0x82ff9480
	ctx.lr = 0x83041804;
	sub_82FF9480(ctx, base);
	// 83041804: 389E003E  addi r4, r30, 0x3e
	ctx.r[4].s64 = ctx.r[30].s64 + 62;
	// 83041808: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8304180C: 4BFB7C75  bl 0x82ff9480
	ctx.lr = 0x83041810;
	sub_82FF9480(ctx, base);
	// 83041810: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83041814: 4BFFFB1D  bl 0x83041330
	ctx.lr = 0x83041818;
	sub_83041330(ctx, base);
	// 83041818: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8304181C: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 83041820: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 83041824: 3880001D  li r4, 0x1d
	ctx.r[4].s64 = 29;
	// 83041828: 387E0020  addi r3, r30, 0x20
	ctx.r[3].s64 = ctx.r[30].s64 + 32;
	// 8304182C: 917E001C  stw r11, 0x1c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 83041830: 4800C8A9  bl 0x8304e0d8
	ctx.lr = 0x83041834;
	sub_8304E0D8(ctx, base);
	// 83041834: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83041838: 38DF0064  addi r6, r31, 0x64
	ctx.r[6].s64 = ctx.r[31].s64 + 100;
	// 8304183C: 38BF0068  addi r5, r31, 0x68
	ctx.r[5].s64 = ctx.r[31].s64 + 104;
	// 83041840: 389E0024  addi r4, r30, 0x24
	ctx.r[4].s64 = ctx.r[30].s64 + 36;
	// 83041844: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83041848: 4BFB82E1  bl 0x82ff9b28
	ctx.lr = 0x8304184C;
	sub_82FF9B28(ctx, base);
	// 8304184C: 389F0058  addi r4, r31, 0x58
	ctx.r[4].s64 = ctx.r[31].s64 + 88;
	// 83041850: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83041854: 4BFB7D25  bl 0x82ff9578
	ctx.lr = 0x83041858;
	sub_82FF9578(ctx, base);
	// 83041858: 817F0058  lwz r11, 0x58(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 8304185C: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 83041860: 2F0BFFFF  cmpwi cr6, r11, -1
	ctx.cr[6].compare_i32(ctx.r[11].s32, -1, &mut ctx.xer);
	// 83041864: 409A003C  bne cr6, 0x830418a0
	if !ctx.cr[6].eq {
	pc = 0x830418A0; continue 'dispatch;
	}
	// 83041868: 809E002C  lwz r4, 0x2c(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(44 as u32) ) } as u64;
	// 8304186C: 28040000  cmplwi r4, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83041870: 4182001C  beq 0x8304188c
	if ctx.cr[0].eq {
	pc = 0x8304188C; continue 'dispatch;
	}
	// 83041874: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83041878: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8304187C: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83041880: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83041884: 4E800421  bctrl
	ctx.lr = 0x83041888;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83041888: 939E002C  stw r28, 0x2c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(44 as u32), ctx.r[28].u32 ) };
	// 8304188C: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83041890: 396B8158  addi r11, r11, -0x7ea8
	ctx.r[11].s64 = ctx.r[11].s64 + -32424;
	// 83041894: 917E0030  stw r11, 0x30(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 83041898: 917E0034  stw r11, 0x34(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 8304189C: 480000AC  b 0x83041948
	pc = 0x83041948; continue 'dispatch;
	// 830418A0: 2F0BFFFE  cmpwi cr6, r11, -2
	ctx.cr[6].compare_i32(ctx.r[11].s32, -2, &mut ctx.xer);
	// 830418A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 830418A8: 38DF0068  addi r6, r31, 0x68
	ctx.r[6].s64 = ctx.r[31].s64 + 104;
	// 830418AC: 38BF0064  addi r5, r31, 0x64
	ctx.r[5].s64 = ctx.r[31].s64 + 100;
	// 830418B0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830418B4: 409A0030  bne cr6, 0x830418e4
	if !ctx.cr[6].eq {
	pc = 0x830418E4; continue 'dispatch;
	}
	// 830418B8: 389F005C  addi r4, r31, 0x5c
	ctx.r[4].s64 = ctx.r[31].s64 + 92;
	// 830418BC: 4BFB826D  bl 0x82ff9b28
	ctx.lr = 0x830418C0;
	sub_82FF9B28(ctx, base);
	// 830418C0: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830418C4: 809F005C  lwz r4, 0x5c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 830418C8: 917F006C  stw r11, 0x6c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 830418CC: 909F0068  stw r4, 0x68(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(104 as u32), ctx.r[4].u32 ) };
	// 830418D0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830418D4: 4BFFF7DD  bl 0x830410b0
	ctx.lr = 0x830418D8;
	sub_830410B0(ctx, base);
	// 830418D8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830418DC: 387F0068  addi r3, r31, 0x68
	ctx.r[3].s64 = ctx.r[31].s64 + 104;
	// 830418E0: 48000064  b 0x83041944
	pc = 0x83041944; continue 'dispatch;
	// 830418E4: 389F0054  addi r4, r31, 0x54
	ctx.r[4].s64 = ctx.r[31].s64 + 84;
	// 830418E8: 4BFB8241  bl 0x82ff9b28
	ctx.lr = 0x830418EC;
	sub_82FF9B28(ctx, base);
	// 830418EC: 817F0054  lwz r11, 0x54(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 830418F0: 917F0070  stw r11, 0x70(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(112 as u32), ctx.r[11].u32 ) };
	// 830418F4: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830418F8: 917F0074  stw r11, 0x74(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 830418FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83041900: 38DF0068  addi r6, r31, 0x68
	ctx.r[6].s64 = ctx.r[31].s64 + 104;
	// 83041904: 38BF0064  addi r5, r31, 0x64
	ctx.r[5].s64 = ctx.r[31].s64 + 100;
	// 83041908: 389F0060  addi r4, r31, 0x60
	ctx.r[4].s64 = ctx.r[31].s64 + 96;
	// 8304190C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83041910: 4BFB8219  bl 0x82ff9b28
	ctx.lr = 0x83041914;
	sub_82FF9B28(ctx, base);
	// 83041914: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83041918: 80BF0060  lwz r5, 0x60(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 8304191C: 917F006C  stw r11, 0x6c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83041920: 90BF0068  stw r5, 0x68(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(104 as u32), ctx.r[5].u32 ) };
	// 83041924: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83041928: 809F0054  lwz r4, 0x54(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 8304192C: 4BFFF625  bl 0x83040f50
	ctx.lr = 0x83041930;
	sub_83040F50(ctx, base);
	// 83041930: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83041934: 387F0068  addi r3, r31, 0x68
	ctx.r[3].s64 = ctx.r[31].s64 + 104;
	// 83041938: 4BF91189  bl 0x82fd2ac0
	ctx.lr = 0x8304193C;
	sub_82FD2AC0(ctx, base);
	// 8304193C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83041940: 387F0070  addi r3, r31, 0x70
	ctx.r[3].s64 = ctx.r[31].s64 + 112;
	// 83041944: 4BF9117D  bl 0x82fd2ac0
	ctx.lr = 0x83041948;
	sub_82FD2AC0(ctx, base);
	// 83041948: 939E0028  stw r28, 0x28(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(40 as u32), ctx.r[28].u32 ) };
	// 8304194C: 383F00A0  addi r1, r31, 0xa0
	ctx.r[1].s64 = ctx.r[31].s64 + 160;
	// 83041950: 48166868  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83041954(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83041954 size=40
    let mut pc: u32 = 0x83041954;
    'dispatch: loop {
        match pc {
            0x83041954 => {
    //   block [0x83041954..0x8304197C)
	// 83041954: 3BECFF60  addi r31, r12, -0xa0
	ctx.r[31].s64 = ctx.r[12].s64 + -160;
	// 83041958: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8304195C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83041960: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83041964: 387F0068  addi r3, r31, 0x68
	ctx.r[3].s64 = ctx.r[31].s64 + 104;
	// 83041968: 4BF914F1  bl 0x82fd2e58
	ctx.lr = 0x8304196C;
	sub_82FD2E58(ctx, base);
	// 8304196C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83041970: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83041974: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83041978: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8304197C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8304197C size=40
    let mut pc: u32 = 0x8304197C;
    'dispatch: loop {
        match pc {
            0x8304197C => {
    //   block [0x8304197C..0x830419A4)
	// 8304197C: 3BECFF60  addi r31, r12, -0xa0
	ctx.r[31].s64 = ctx.r[12].s64 + -160;
	// 83041980: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83041984: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83041988: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8304198C: 387F0070  addi r3, r31, 0x70
	ctx.r[3].s64 = ctx.r[31].s64 + 112;
	// 83041990: 4BF914C9  bl 0x82fd2e58
	ctx.lr = 0x83041994;
	sub_82FD2E58(ctx, base);
	// 83041994: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83041998: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8304199C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830419A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830419A4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830419A4 size=40
    let mut pc: u32 = 0x830419A4;
    'dispatch: loop {
        match pc {
            0x830419A4 => {
    //   block [0x830419A4..0x830419CC)
	// 830419A4: 3BECFF60  addi r31, r12, -0xa0
	ctx.r[31].s64 = ctx.r[12].s64 + -160;
	// 830419A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830419AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830419B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830419B4: 387F0068  addi r3, r31, 0x68
	ctx.r[3].s64 = ctx.r[31].s64 + 104;
	// 830419B8: 4BF914A1  bl 0x82fd2e58
	ctx.lr = 0x830419BC;
	sub_82FD2E58(ctx, base);
	// 830419BC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830419C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830419C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830419C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830419D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830419D0 size=8
    let mut pc: u32 = 0x830419D0;
    'dispatch: loop {
        match pc {
            0x830419D0 => {
    //   block [0x830419D0..0x830419D8)
	// 830419D0: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 830419D4: 8215E328  lwz r16, -0x1cd8(r21)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-7384 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830419D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830419D8 size=84
    let mut pc: u32 = 0x830419D8;
    'dispatch: loop {
        match pc {
            0x830419D8 => {
    //   block [0x830419D8..0x83041A2C)
	// 830419D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830419DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830419E0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830419E4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830419E8: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 830419EC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830419F0: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 830419F4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830419F8: 396BE18C  addi r11, r11, -0x1e74
	ctx.r[11].s64 = ctx.r[11].s64 + -7796;
	// 830419FC: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 83041A00: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83041A04: 4BFFFB95  bl 0x83041598
	ctx.lr = 0x83041A08;
	sub_83041598(ctx, base);
	// 83041A08: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83041A0C: 396BA93C  addi r11, r11, -0x56c4
	ctx.r[11].s64 = ctx.r[11].s64 + -22212;
	// 83041A10: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83041A14: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 83041A18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83041A1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83041A20: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83041A24: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83041A28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83041A2C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83041A2C size=40
    let mut pc: u32 = 0x83041A2C;
    'dispatch: loop {
        match pc {
            0x83041A2C => {
    //   block [0x83041A2C..0x83041A54)
	// 83041A2C: 3BECFF90  addi r31, r12, -0x70
	ctx.r[31].s64 = ctx.r[12].s64 + -112;
	// 83041A30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83041A34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83041A38: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83041A3C: 807F0084  lwz r3, 0x84(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 83041A40: 4800AD21  bl 0x8304c760
	ctx.lr = 0x83041A44;
	sub_8304C760(ctx, base);
	// 83041A44: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83041A48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83041A4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83041A50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83041A58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83041A58 size=76
    let mut pc: u32 = 0x83041A58;
    'dispatch: loop {
        match pc {
            0x83041A58 => {
    //   block [0x83041A58..0x83041AA4)
	// 83041A58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83041A5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83041A60: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83041A64: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83041A68: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83041A6C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83041A70: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83041A74: 4BFFFF65  bl 0x830419d8
	ctx.lr = 0x83041A78;
	sub_830419D8(ctx, base);
	// 83041A78: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83041A7C: 4182000C  beq 0x83041a88
	if ctx.cr[0].eq {
	pc = 0x83041A88; continue 'dispatch;
	}
	// 83041A80: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83041A84: 4BF9685D  bl 0x82fd82e0
	ctx.lr = 0x83041A88;
	sub_82FD82E0(ctx, base);
	// 83041A88: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83041A8C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83041A90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83041A94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83041A98: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83041A9C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83041AA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83041AA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83041AA8 size=100
    let mut pc: u32 = 0x83041AA8;
    'dispatch: loop {
        match pc {
            0x83041AA8 => {
    //   block [0x83041AA8..0x83041B0C)
	// 83041AA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83041AAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83041AB0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83041AB4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83041AB8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83041ABC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83041AC0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83041AC4: 809F0024  lwz r4, 0x24(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 83041AC8: 28040000  cmplwi r4, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83041ACC: 41820018  beq 0x83041ae4
	if ctx.cr[0].eq {
	pc = 0x83041AE4; continue 'dispatch;
	}
	// 83041AD0: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83041AD4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83041AD8: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83041ADC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83041AE0: 4E800421  bctrl
	ctx.lr = 0x83041AE4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83041AE4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83041AE8: 809F0004  lwz r4, 4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83041AEC: 4BF8F095  bl 0x82fd0b80
	ctx.lr = 0x83041AF0;
	sub_82FD0B80(ctx, base);
	// 83041AF0: 907F0024  stw r3, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[3].u32 ) };
	// 83041AF4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83041AF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83041AFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83041B00: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83041B04: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83041B08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83041B10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83041B10 size=8
    let mut pc: u32 = 0x83041B10;
    'dispatch: loop {
        match pc {
            0x83041B10 => {
    //   block [0x83041B10..0x83041B18)
	// 83041B10: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83041B14: 8215E388  lwz r16, -0x1c78(r21)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-7288 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83041B18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83041B18 size=104
    let mut pc: u32 = 0x83041B18;
    'dispatch: loop {
        match pc {
            0x83041B18 => {
    //   block [0x83041B18..0x83041B80)
	// 83041B18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83041B1C: 48166641  bl 0x831a815c
	ctx.lr = 0x83041B20;
	sub_831A8130(ctx, base);
	// 83041B20: 3BE1FF70  addi r31, r1, -0x90
	ctx.r[31].s64 = ctx.r[1].s64 + -144;
	// 83041B24: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83041B28: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83041B2C: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 83041B30: 80DF00E4  lwz r6, 0xe4(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(228 as u32) ) } as u64;
	// 83041B34: 7CFC3B78  mr r28, r7
	ctx.r[28].u64 = ctx.r[7].u64;
	// 83041B38: 7D1B4378  mr r27, r8
	ctx.r[27].u64 = ctx.r[8].u64;
	// 83041B3C: 7D3A4B78  mr r26, r9
	ctx.r[26].u64 = ctx.r[9].u64;
	// 83041B40: 93DF00A4  stw r30, 0xa4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(164 as u32), ctx.r[30].u32 ) };
	// 83041B44: 7D595378  mr r25, r10
	ctx.r[25].u64 = ctx.r[10].u64;
	// 83041B48: 4BF973E9  bl 0x82fd8f30
	ctx.lr = 0x83041B4C;
	sub_82FD8F30(ctx, base);
	// 83041B4C: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 83041B50: 7F28CB78  mr r8, r25
	ctx.r[8].u64 = ctx.r[25].u64;
	// 83041B54: 396BE374  addi r11, r11, -0x1c8c
	ctx.r[11].s64 = ctx.r[11].s64 + -7308;
	// 83041B58: 7F47D378  mr r7, r26
	ctx.r[7].u64 = ctx.r[26].u64;
	// 83041B5C: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 83041B60: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 83041B64: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83041B68: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83041B6C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83041B70: 4BF977C9  bl 0x82fd9338
	ctx.lr = 0x83041B74;
	sub_82FD9338(ctx, base);
	// 83041B74: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83041B78: 383F0090  addi r1, r31, 0x90
	ctx.r[1].s64 = ctx.r[31].s64 + 144;
	// 83041B7C: 48166630  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83041B80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83041B80 size=40
    let mut pc: u32 = 0x83041B80;
    'dispatch: loop {
        match pc {
            0x83041B80 => {
    //   block [0x83041B80..0x83041BA8)
	// 83041B80: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 83041B84: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83041B88: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83041B8C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83041B90: 807F00A4  lwz r3, 0xa4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 83041B94: 4BF972E5  bl 0x82fd8e78
	ctx.lr = 0x83041B98;
	sub_82FD8E78(ctx, base);
	// 83041B98: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83041B9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83041BA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83041BA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83041BA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83041BA8 size=60
    let mut pc: u32 = 0x83041BA8;
    'dispatch: loop {
        match pc {
            0x83041BA8 => {
    //   block [0x83041BA8..0x83041BE4)
	// 83041BA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83041BAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83041BB0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83041BB4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83041BB8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83041BBC: 4BF973ED  bl 0x82fd8fa8
	ctx.lr = 0x83041BC0;
	sub_82FD8FA8(ctx, base);
	// 83041BC0: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 83041BC4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83041BC8: 396BE374  addi r11, r11, -0x1c8c
	ctx.r[11].s64 = ctx.r[11].s64 + -7308;
	// 83041BCC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83041BD0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83041BD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83041BD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83041BDC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83041BE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83041BE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83041BE8 size=8
    let mut pc: u32 = 0x83041BE8;
    'dispatch: loop {
        match pc {
            0x83041BE8 => {
    //   block [0x83041BE8..0x83041BF0)
	// 83041BE8: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83041BEC: 8215E3C0  lwz r16, -0x1c40(r21)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-7232 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83041BF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83041BF0 size=104
    let mut pc: u32 = 0x83041BF0;
    'dispatch: loop {
        match pc {
            0x83041BF0 => {
    //   block [0x83041BF0..0x83041C58)
	// 83041BF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83041BF4: 48166569  bl 0x831a815c
	ctx.lr = 0x83041BF8;
	sub_831A8130(ctx, base);
	// 83041BF8: 3BE1FF70  addi r31, r1, -0x90
	ctx.r[31].s64 = ctx.r[1].s64 + -144;
	// 83041BFC: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83041C00: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83041C04: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 83041C08: 80DF00E4  lwz r6, 0xe4(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(228 as u32) ) } as u64;
	// 83041C0C: 7CFC3B78  mr r28, r7
	ctx.r[28].u64 = ctx.r[7].u64;
	// 83041C10: 7D1B4378  mr r27, r8
	ctx.r[27].u64 = ctx.r[8].u64;
	// 83041C14: 7D3A4B78  mr r26, r9
	ctx.r[26].u64 = ctx.r[9].u64;
	// 83041C18: 93DF00A4  stw r30, 0xa4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(164 as u32), ctx.r[30].u32 ) };
	// 83041C1C: 7D595378  mr r25, r10
	ctx.r[25].u64 = ctx.r[10].u64;
	// 83041C20: 4BF97311  bl 0x82fd8f30
	ctx.lr = 0x83041C24;
	sub_82FD8F30(ctx, base);
	// 83041C24: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 83041C28: 7F28CB78  mr r8, r25
	ctx.r[8].u64 = ctx.r[25].u64;
	// 83041C2C: 396BE374  addi r11, r11, -0x1c8c
	ctx.r[11].s64 = ctx.r[11].s64 + -7308;
	// 83041C30: 7F47D378  mr r7, r26
	ctx.r[7].u64 = ctx.r[26].u64;
	// 83041C34: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 83041C38: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 83041C3C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83041C40: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83041C44: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83041C48: 4BF97789  bl 0x82fd93d0
	ctx.lr = 0x83041C4C;
	sub_82FD93D0(ctx, base);
	// 83041C4C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83041C50: 383F0090  addi r1, r31, 0x90
	ctx.r[1].s64 = ctx.r[31].s64 + 144;
	// 83041C54: 48166558  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83041C58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83041C58 size=40
    let mut pc: u32 = 0x83041C58;
    'dispatch: loop {
        match pc {
            0x83041C58 => {
    //   block [0x83041C58..0x83041C80)
	// 83041C58: 3BECFF70  addi r31, r12, -0x90
	ctx.r[31].s64 = ctx.r[12].s64 + -144;
	// 83041C5C: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83041C60: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83041C64: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83041C68: 807F00A4  lwz r3, 0xa4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 83041C6C: 4BF9720D  bl 0x82fd8e78
	ctx.lr = 0x83041C70;
	sub_82FD8E78(ctx, base);
	// 83041C70: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83041C74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83041C78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83041C7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83041C80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83041C80 size=16
    let mut pc: u32 = 0x83041C80;
    'dispatch: loop {
        match pc {
            0x83041C80 => {
    //   block [0x83041C80..0x83041C90)
	// 83041C80: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 83041C84: 396BE374  addi r11, r11, -0x1c8c
	ctx.r[11].s64 = ctx.r[11].s64 + -7308;
	// 83041C88: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83041C8C: 4BF971EC  b 0x82fd8e78
	sub_82FD8E78(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83041C90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83041C90 size=8
    let mut pc: u32 = 0x83041C90;
    'dispatch: loop {
        match pc {
            0x83041C90 => {
    //   block [0x83041C90..0x83041C98)
	// 83041C90: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83041C94: 8215E3F8  lwz r16, -0x1c08(r21)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-7176 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83041C98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83041C98 size=92
    let mut pc: u32 = 0x83041C98;
    'dispatch: loop {
        match pc {
            0x83041C98 => {
    //   block [0x83041C98..0x83041CF4)
	// 83041C98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83041C9C: 481664D1  bl 0x831a816c
	ctx.lr = 0x83041CA0;
	sub_831A8130(ctx, base);
	// 83041CA0: 3BE1FF80  addi r31, r1, -0x80
	ctx.r[31].s64 = ctx.r[1].s64 + -128;
	// 83041CA4: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83041CA8: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83041CAC: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 83041CB0: 809D0014  lwz r4, 0x14(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(20 as u32) ) } as u64;
	// 83041CB4: 93BF0094  stw r29, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[29].u32 ) };
	// 83041CB8: 4BF965E1  bl 0x82fd8298
	ctx.lr = 0x83041CBC;
	sub_82FD8298(ctx, base);
	// 83041CBC: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 83041CC0: 93DF0050  stw r30, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 83041CC4: 41820024  beq 0x83041ce8
	if ctx.cr[0].eq {
	pc = 0x83041CE8; continue 'dispatch;
	}
	// 83041CC8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83041CCC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83041CD0: 4BF972D9  bl 0x82fd8fa8
	ctx.lr = 0x83041CD4;
	sub_82FD8FA8(ctx, base);
	// 83041CD4: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 83041CD8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83041CDC: 396BE374  addi r11, r11, -0x1c8c
	ctx.r[11].s64 = ctx.r[11].s64 + -7308;
	// 83041CE0: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83041CE4: 48000008  b 0x83041cec
	pc = 0x83041CEC; continue 'dispatch;
	// 83041CE8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83041CEC: 383F0080  addi r1, r31, 0x80
	ctx.r[1].s64 = ctx.r[31].s64 + 128;
	// 83041CF0: 481664CC  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83041CF4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83041CF4 size=48
    let mut pc: u32 = 0x83041CF4;
    'dispatch: loop {
        match pc {
            0x83041CF4 => {
    //   block [0x83041CF4..0x83041D24)
	// 83041CF4: 3BECFF80  addi r31, r12, -0x80
	ctx.r[31].s64 = ctx.r[12].s64 + -128;
	// 83041CF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83041CFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83041D00: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83041D04: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 83041D08: 808B0014  lwz r4, 0x14(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 83041D0C: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 83041D10: 4BF965D1  bl 0x82fd82e0
	ctx.lr = 0x83041D14;
	sub_82FD82E0(ctx, base);
	// 83041D14: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83041D18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83041D1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83041D20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83041D28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83041D28 size=12
    let mut pc: u32 = 0x83041D28;
    'dispatch: loop {
        match pc {
            0x83041D28 => {
    //   block [0x83041D28..0x83041D34)
	// 83041D28: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83041D2C: 386B8538  addi r3, r11, -0x7ac8
	ctx.r[3].s64 = ctx.r[11].s64 + -31432;
	// 83041D30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83041D38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83041D38 size=88
    let mut pc: u32 = 0x83041D38;
    'dispatch: loop {
        match pc {
            0x83041D38 => {
    //   block [0x83041D38..0x83041D90)
	// 83041D38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83041D3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83041D40: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83041D44: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83041D48: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83041D4C: 3D608216  lis r11, -0x7dea
	ctx.r[11].s64 = -2112487424;
	// 83041D50: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83041D54: 396BE374  addi r11, r11, -0x1c8c
	ctx.r[11].s64 = ctx.r[11].s64 + -7308;
	// 83041D58: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83041D5C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83041D60: 4BF97119  bl 0x82fd8e78
	ctx.lr = 0x83041D64;
	sub_82FD8E78(ctx, base);
	// 83041D64: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83041D68: 4182000C  beq 0x83041d74
	if ctx.cr[0].eq {
	pc = 0x83041D74; continue 'dispatch;
	}
	// 83041D6C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83041D70: 4BF96571  bl 0x82fd82e0
	ctx.lr = 0x83041D74;
	sub_82FD82E0(ctx, base);
	// 83041D74: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83041D78: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83041D7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83041D80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83041D84: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83041D88: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83041D8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83041D90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83041D90 size=8
    let mut pc: u32 = 0x83041D90;
    'dispatch: loop {
        match pc {
            0x83041D90 => {
    //   block [0x83041D90..0x83041D98)
	// 83041D90: 831A93F0  lwz r24, -0x6c10(r26)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-27664 as u32) ) } as u64;
	// 83041D94: 8215E430  lwz r16, -0x1bd0(r21)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-7120 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83041D98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83041D98 size=72
    let mut pc: u32 = 0x83041D98;
    'dispatch: loop {
        match pc {
            0x83041D98 => {
    //   block [0x83041D98..0x83041DE0)
	// 83041D98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83041D9C: 481663D1  bl 0x831a816c
	ctx.lr = 0x83041DA0;
	sub_831A8130(ctx, base);
	// 83041DA0: 3BE1FF90  addi r31, r1, -0x70
	ctx.r[31].s64 = ctx.r[1].s64 + -112;
	// 83041DA4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83041DA8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83041DAC: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 83041DB0: 7CE63B78  mr r6, r7
	ctx.r[6].u64 = ctx.r[7].u64;
	// 83041DB4: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 83041DB8: 4BF97179  bl 0x82fd8f30
	ctx.lr = 0x83041DBC;
	sub_82FD8F30(ctx, base);
	// 83041DBC: 3D608214  lis r11, -0x7dec
	ctx.r[11].s64 = -2112618496;
	// 83041DC0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83041DC4: 396B3BC0  addi r11, r11, 0x3bc0
	ctx.r[11].s64 = ctx.r[11].s64 + 15296;
	// 83041DC8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83041DCC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83041DD0: 4BF974E9  bl 0x82fd92b8
	ctx.lr = 0x83041DD4;
	sub_82FD92B8(ctx, base);
	// 83041DD4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83041DD8: 383F0070  addi r1, r31, 0x70
	ctx.r[1].s64 = ctx.r[31].s64 + 112;
	// 83041DDC: 481663E0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


