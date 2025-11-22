pub fn sub_83203310(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83203310 size=4
    let mut pc: u32 = 0x83203310;
    'dispatch: loop {
        match pc {
            0x83203310 => {
    //   block [0x83203310..0x83203314)
	// 83203310: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83203318(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83203318 size=136
    let mut pc: u32 = 0x83203318;
    'dispatch: loop {
        match pc {
            0x83203318 => {
    //   block [0x83203318..0x832033A0)
	// 83203318: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8320331C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83203320: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83203324: 81430004  lwz r10, 4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 83203328: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8320332C: 81030008  lwz r8, 8(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 83203330: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 83203334: 419A0028  beq cr6, 0x8320335c
	if ctx.cr[6].eq {
	pc = 0x8320335C; continue 'dispatch;
	}
	// 83203338: 7D094378  mr r9, r8
	ctx.r[9].u64 = ctx.r[8].u64;
	// 8320333C: 80E90000  lwz r7, 0(r9)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 83203340: 7F043840  cmplw cr6, r4, r7
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[7].u32, &mut ctx.xer);
	// 83203344: 419A002C  beq cr6, 0x83203370
	if ctx.cr[6].eq {
	pc = 0x83203370; continue 'dispatch;
	}
	// 83203348: 80E30004  lwz r7, 4(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8320334C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83203350: 39290004  addi r9, r9, 4
	ctx.r[9].s64 = ctx.r[9].s64 + 4;
	// 83203354: 7F0B3840  cmplw cr6, r11, r7
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[7].u32, &mut ctx.xer);
	// 83203358: 4198FFE4  blt cr6, 0x8320333c
	if ctx.cr[6].lt {
	pc = 0x8320333C; continue 'dispatch;
	}
	// 8320335C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83203360: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83203364: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83203368: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8320336C: 4E800020  blr
	return;
	// 83203370: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 83203374: 40980024  bge cr6, 0x83203398
	if !ctx.cr[6].lt {
	pc = 0x83203398; continue 'dispatch;
	}
	// 83203378: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 8320337C: 5569103A  slwi r9, r11, 2
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 83203380: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 83203384: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 83203388: 7C694214  add r3, r9, r8
	ctx.r[3].u64 = ctx.r[9].u64 + ctx.r[8].u64;
	// 8320338C: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 83203390: 38830004  addi r4, r3, 4
	ctx.r[4].s64 = ctx.r[3].s64 + 4;
	// 83203394: 4BAA60ED  bl 0x82ca9480
	ctx.lr = 0x83203398;
	sub_82CA9480(ctx, base);
	// 83203398: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8320339C: 4BFFFFC4  b 0x83203360
	pc = 0x83203360; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832033A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832033A0 size=132
    let mut pc: u32 = 0x832033A0;
    'dispatch: loop {
        match pc {
            0x832033A0 => {
    //   block [0x832033A0..0x83203424)
	// 832033A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832033A4: 4BAA6069  bl 0x82ca940c
	ctx.lr = 0x832033A8;
	sub_82CA93D0(ctx, base);
	// 832033A8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832033AC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 832033B0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 832033B4: 397E0001  addi r11, r30, 1
	ctx.r[11].s64 = ctx.r[30].s64 + 1;
	// 832033B8: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832033BC: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 832033C0: 40990014  ble cr6, 0x832033d4
	if !ctx.cr[6].gt {
	pc = 0x832033D4; continue 'dispatch;
	}
	// 832033C4: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832033C8: 4800000C  b 0x832033d4
	pc = 0x832033D4; continue 'dispatch;
	// 832033CC: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 832033D0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 832033D4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 832033D8: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 832033DC: 4098FFF0  bge cr6, 0x832033cc
	if !ctx.cr[6].lt {
	pc = 0x832033CC; continue 'dispatch;
	}
	// 832033E0: 5564103A  slwi r4, r11, 2
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 832033E4: 83BF0008  lwz r29, 8(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 832033E8: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 832033EC: 4BFC8DAD  bl 0x831cc198
	ctx.lr = 0x832033F0;
	sub_831CC198(ctx, base);
	// 832033F0: 907F0008  stw r3, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[3].u32 ) };
	// 832033F4: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832033F8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 832033FC: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 83203400: 4BAA6081  bl 0x82ca9480
	ctx.lr = 0x83203404;
	sub_82CA9480(ctx, base);
	// 83203404: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83203408: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8320340C: 4BFC8EBD  bl 0x831cc2c8
	ctx.lr = 0x83203410;
	sub_831CC2C8(ctx, base);
	// 83203410: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83203414: 57CA103A  slwi r10, r30, 2
	ctx.r[10].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83203418: 7C6A5A14  add r3, r10, r11
	ctx.r[3].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 8320341C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83203420: 4BAA603C  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83203428(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83203428 size=172
    let mut pc: u32 = 0x83203428;
    'dispatch: loop {
        match pc {
            0x83203428 => {
    //   block [0x83203428..0x832034D4)
	// 83203428: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8320342C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83203430: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83203434: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83203438: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8320343C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83203440: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83203444: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83203448: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8320344C: 40990008  ble cr6, 0x83203454
	if !ctx.cr[6].gt {
	pc = 0x83203454; continue 'dispatch;
	}
	// 83203450: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 83203454: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83203458: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8320345C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83203460: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 83203464: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 83203468: 40990010  ble cr6, 0x83203478
	if !ctx.cr[6].gt {
	pc = 0x83203478; continue 'dispatch;
	}
	// 8320346C: 388BFFFF  addi r4, r11, -1
	ctx.r[4].s64 = ctx.r[11].s64 + -1;
	// 83203470: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83203474: 4BFFFF2D  bl 0x832033a0
	ctx.lr = 0x83203478;
	sub_832033A0(ctx, base);
	// 83203478: 813F0004  lwz r9, 4(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8320347C: 57CB103A  slwi r11, r30, 2
	ctx.r[11].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83203480: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83203484: 7D3E4850  subf r9, r30, r9
	ctx.r[9].s64 = ctx.r[9].s64 - ctx.r[30].s64;
	// 83203488: 7C6B5214  add r3, r11, r10
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8320348C: 3569FFFF  addic. r11, r9, -1
	ctx.xer.ca = (ctx.r[9].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[9].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83203490: 556A103A  slwi r10, r11, 2
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83203494: 7D4A1A14  add r10, r10, r3
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[3].u64;
	// 83203498: 4182001C  beq 0x832034b4
	if ctx.cr[0].eq {
	pc = 0x832034B4; continue 'dispatch;
	}
	// 8320349C: 810AFFFC  lwz r8, -4(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-4 as u32) ) } as u64;
	// 832034A0: 392AFFFC  addi r9, r10, -4
	ctx.r[9].s64 = ctx.r[10].s64 + -4;
	// 832034A4: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 832034A8: 910A0000  stw r8, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 832034AC: 7D2A4B78  mr r10, r9
	ctx.r[10].u64 = ctx.r[9].u64;
	// 832034B0: 4082FFEC  bne 0x8320349c
	if !ctx.cr[0].eq {
	pc = 0x8320349C; continue 'dispatch;
	}
	// 832034B4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832034B8: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 832034BC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 832034C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832034C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832034C8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 832034CC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832034D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832034D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832034D8 size=384
    let mut pc: u32 = 0x832034D8;
    'dispatch: loop {
        match pc {
            0x832034D8 => {
    //   block [0x832034D8..0x83203658)
	// 832034D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832034DC: 4BAA5F0D  bl 0x82ca93e8
	ctx.lr = 0x832034E0;
	sub_82CA93D0(ctx, base);
	// 832034E0: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832034E4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 832034E8: 82DE0000  lwz r22, 0(r30)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 832034EC: 807E0014  lwz r3, 0x14(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 832034F0: 56CB083C  slwi r11, r22, 1
	ctx.r[11].u32 = ctx.r[22].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 832034F4: 56C41838  slwi r4, r22, 3
	ctx.r[4].u32 = ctx.r[22].u32.wrapping_shl(3);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 832034F8: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 832034FC: 4BFC8C9D  bl 0x831cc198
	ctx.lr = 0x83203500;
	sub_831CC198(ctx, base);
	// 83203500: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83203504: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83203508: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 8320350C: 7C771B78  mr r23, r3
	ctx.r[23].u64 = ctx.r[3].u64;
	// 83203510: 4BAA64A1  bl 0x82ca99b0
	ctx.lr = 0x83203514;
	sub_82CA99B0(ctx, base);
	// 83203514: 3A800000  li r20, 0
	ctx.r[20].s64 = 0;
	// 83203518: 2F160000  cmpwi cr6, r22, 0
	ctx.cr[6].compare_i32(ctx.r[22].s32, 0, &mut ctx.xer);
	// 8320351C: 40990130  ble cr6, 0x8320364c
	if !ctx.cr[6].gt {
	pc = 0x8320364C; continue 'dispatch;
	}
	// 83203520: 56CB103A  slwi r11, r22, 2
	ctx.r[11].u32 = ctx.r[22].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83203524: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 83203528: 7EABBA14  add r21, r11, r23
	ctx.r[21].u64 = ctx.r[11].u64 + ctx.r[23].u64;
	// 8320352C: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 83203530: 7F3A582E  lwzx r25, r26, r11
	ctx.r[25].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 83203534: 2B190000  cmplwi cr6, r25, 0
	ctx.cr[6].compare_u32(ctx.r[25].u32, 0 as u32, &mut ctx.xer);
	// 83203538: 419A0100  beq cr6, 0x83203638
	if ctx.cr[6].eq {
	pc = 0x83203638; continue 'dispatch;
	}
	// 8320353C: 83BE0014  lwz r29, 0x14(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 83203540: 38800014  li r4, 0x14
	ctx.r[4].s64 = 20;
	// 83203544: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83203548: 4BFC8C51  bl 0x831cc198
	ctx.lr = 0x8320354C;
	sub_831CC198(ctx, base);
	// 8320354C: 37E30004  addic. r31, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[31].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 83203550: 93A30000  stw r29, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 83203554: 41820018  beq 0x8320356c
	if ctx.cr[0].eq {
	pc = 0x8320356C; continue 'dispatch;
	}
	// 83203558: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8320355C: 809E0014  lwz r4, 0x14(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 83203560: 4BFE2511  bl 0x831e5a70
	ctx.lr = 0x83203564;
	sub_831E5A70(ctx, base);
	// 83203564: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 83203568: 48000008  b 0x83203570
	pc = 0x83203570; continue 'dispatch;
	// 8320356C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83203570: 7D7AB92E  stwx r11, r26, r23
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[26].u32.wrapping_add(ctx.r[23].u32), ctx.r[11].u32) };
	// 83203574: 38800014  li r4, 0x14
	ctx.r[4].s64 = 20;
	// 83203578: 83BE0014  lwz r29, 0x14(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 8320357C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83203580: 4BFC8C19  bl 0x831cc198
	ctx.lr = 0x83203584;
	sub_831CC198(ctx, base);
	// 83203584: 37E30004  addic. r31, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[31].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 83203588: 93A30000  stw r29, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 8320358C: 41820018  beq 0x832035a4
	if ctx.cr[0].eq {
	pc = 0x832035A4; continue 'dispatch;
	}
	// 83203590: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83203594: 809E0014  lwz r4, 0x14(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 83203598: 4BFE24D9  bl 0x831e5a70
	ctx.lr = 0x8320359C;
	sub_831E5A70(ctx, base);
	// 8320359C: 7FFBFB78  mr r27, r31
	ctx.r[27].u64 = ctx.r[31].u64;
	// 832035A0: 48000008  b 0x832035a8
	pc = 0x832035A8; continue 'dispatch;
	// 832035A4: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 832035A8: 93750000  stw r27, 0(r21)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[21].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 832035AC: 83F90004  lwz r31, 4(r25)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(4 as u32) ) } as u64;
	// 832035B0: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 832035B4: 7F1AB82E  lwzx r24, r26, r23
	ctx.r[24].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[23].u32)) } as u64;
	// 832035B8: 419A0080  beq cr6, 0x83203638
	if ctx.cr[6].eq {
	pc = 0x83203638; continue 'dispatch;
	}
	// 832035BC: 57FC103A  slwi r28, r31, 2
	ctx.r[28].u32 = ctx.r[31].u32.wrapping_shl(2);
	ctx.r[28].u64 = ctx.r[28].u32 as u64;
	// 832035C0: 81790004  lwz r11, 4(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(4 as u32) ) } as u64;
	// 832035C4: 3BFFFFFF  addi r31, r31, -1
	ctx.r[31].s64 = ctx.r[31].s64 + -1;
	// 832035C8: 3B9CFFFC  addi r28, r28, -4
	ctx.r[28].s64 = ctx.r[28].s64 + -4;
	// 832035CC: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 832035D0: 40980010  bge cr6, 0x832035e0
	if !ctx.cr[6].lt {
	pc = 0x832035E0; continue 'dispatch;
	}
	// 832035D4: 81790008  lwz r11, 8(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(8 as u32) ) } as u64;
	// 832035D8: 7C6BE214  add r3, r11, r28
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[28].u64;
	// 832035DC: 48000010  b 0x832035ec
	pc = 0x832035EC; continue 'dispatch;
	// 832035E0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 832035E4: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 832035E8: 4BFFFDB9  bl 0x832033a0
	ctx.lr = 0x832035EC;
	sub_832033A0(ctx, base);
	// 832035EC: 83A30000  lwz r29, 0(r3)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 832035F0: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 832035F4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 832035F8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 832035FC: 4E800421  bctrl
	ctx.lr = 0x83203600;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83203600: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83203604: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 83203608: 7C6B5838  and r11, r3, r11
	ctx.r[11].u64 = ctx.r[3].u64 & ctx.r[11].u64;
	// 8320360C: 7F0BA040  cmplw cr6, r11, r20
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[20].u32, &mut ctx.xer);
	// 83203610: 409A0010  bne cr6, 0x83203620
	if !ctx.cr[6].eq {
	pc = 0x83203620; continue 'dispatch;
	}
	// 83203614: 80980004  lwz r4, 4(r24)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(4 as u32) ) } as u64;
	// 83203618: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 8320361C: 4800000C  b 0x83203628
	pc = 0x83203628; continue 'dispatch;
	// 83203620: 809B0004  lwz r4, 4(r27)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 83203624: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 83203628: 4BFFFD79  bl 0x832033a0
	ctx.lr = 0x8320362C;
	sub_832033A0(ctx, base);
	// 8320362C: 93A30000  stw r29, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 83203630: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 83203634: 409AFF8C  bne cr6, 0x832035c0
	if !ctx.cr[6].eq {
	pc = 0x832035C0; continue 'dispatch;
	}
	// 83203638: 3A940001  addi r20, r20, 1
	ctx.r[20].s64 = ctx.r[20].s64 + 1;
	// 8320363C: 3B5A0004  addi r26, r26, 4
	ctx.r[26].s64 = ctx.r[26].s64 + 4;
	// 83203640: 3AB50004  addi r21, r21, 4
	ctx.r[21].s64 = ctx.r[21].s64 + 4;
	// 83203644: 7F14B000  cmpw cr6, r20, r22
	ctx.cr[6].compare_i32(ctx.r[20].s32, ctx.r[22].s32, &mut ctx.xer);
	// 83203648: 4198FEE4  blt cr6, 0x8320352c
	if ctx.cr[6].lt {
	pc = 0x8320352C; continue 'dispatch;
	}
	// 8320364C: 92FE0008  stw r23, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[23].u32 ) };
	// 83203650: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 83203654: 4BAA5DE4  b 0x82ca9438
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83203658(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83203658 size=192
    let mut pc: u32 = 0x83203658;
    'dispatch: loop {
        match pc {
            0x83203658 => {
    //   block [0x83203658..0x83203718)
	// 83203658: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8320365C: 4BAA5DA5  bl 0x82ca9400
	ctx.lr = 0x83203660;
	sub_82CA93D0(ctx, base);
	// 83203660: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83203664: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 83203668: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 8320366C: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 83203670: 817B0010  lwz r11, 0x10(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(16 as u32) ) } as u64;
	// 83203674: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83203678: 4E800421  bctrl
	ctx.lr = 0x8320367C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8320367C: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 83203680: 815B0008  lwz r10, 8(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 83203684: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 83203688: 7D6B1838  and r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 & ctx.r[3].u64;
	// 8320368C: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83203690: 7FEB502E  lwzx r31, r11, r10
	ctx.r[31].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 83203694: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 83203698: 419A006C  beq cr6, 0x83203704
	if ctx.cr[6].eq {
	pc = 0x83203704; continue 'dispatch;
	}
	// 8320369C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832036A0: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 832036A4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 832036A8: 419A005C  beq cr6, 0x83203704
	if ctx.cr[6].eq {
	pc = 0x83203704; continue 'dispatch;
	}
	// 832036AC: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 832036B0: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 832036B4: 40980010  bge cr6, 0x832036c4
	if !ctx.cr[6].lt {
	pc = 0x832036C4; continue 'dispatch;
	}
	// 832036B8: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 832036BC: 7C6BE214  add r3, r11, r28
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[28].u64;
	// 832036C0: 48000010  b 0x832036d0
	pc = 0x832036D0; continue 'dispatch;
	// 832036C4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 832036C8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832036CC: 4BFFFCD5  bl 0x832033a0
	ctx.lr = 0x832036D0;
	sub_832033A0(ctx, base);
	// 832036D0: 83A30000  lwz r29, 0(r3)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 832036D4: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 832036D8: 817B000C  lwz r11, 0xc(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(12 as u32) ) } as u64;
	// 832036DC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 832036E0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 832036E4: 4E800421  bctrl
	ctx.lr = 0x832036E8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 832036E8: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 832036EC: 41820024  beq 0x83203710
	if ctx.cr[0].eq {
	pc = 0x83203710; continue 'dispatch;
	}
	// 832036F0: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832036F4: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 832036F8: 3B9C0004  addi r28, r28, 4
	ctx.r[28].s64 = ctx.r[28].s64 + 4;
	// 832036FC: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 83203700: 4198FFB8  blt cr6, 0x832036b8
	if ctx.cr[6].lt {
	pc = 0x832036B8; continue 'dispatch;
	}
	// 83203704: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83203708: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8320370C: 4BAA5D44  b 0x82ca9450
	sub_82CA9420(ctx, base);
	return;
	// 83203710: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83203714: 4BFFFFF4  b 0x83203708
	pc = 0x83203708; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83203718(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83203718 size=192
    let mut pc: u32 = 0x83203718;
    'dispatch: loop {
        match pc {
            0x83203718 => {
    //   block [0x83203718..0x832037D8)
	// 83203718: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8320371C: 4BAA5CE9  bl 0x82ca9404
	ctx.lr = 0x83203720;
	sub_82CA93D0(ctx, base);
	// 83203720: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83203724: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83203728: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 8320372C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83203730: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 83203734: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83203738: 4E800421  bctrl
	ctx.lr = 0x8320373C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8320373C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83203740: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83203744: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 83203748: 7D6B1838  and r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 & ctx.r[3].u64;
	// 8320374C: 557D103A  slwi r29, r11, 2
	ctx.r[29].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[29].u64 = ctx.r[29].u32 as u64;
	// 83203750: 7D6AE82E  lwzx r11, r10, r29
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 83203754: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83203758: 409A003C  bne cr6, 0x83203794
	if !ctx.cr[6].eq {
	pc = 0x83203794; continue 'dispatch;
	}
	// 8320375C: 837F0014  lwz r27, 0x14(r31)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 83203760: 38800014  li r4, 0x14
	ctx.r[4].s64 = 20;
	// 83203764: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 83203768: 4BFC8A31  bl 0x831cc198
	ctx.lr = 0x8320376C;
	sub_831CC198(ctx, base);
	// 8320376C: 37C30004  addic. r30, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[30].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 83203770: 93630000  stw r27, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 83203774: 41820014  beq 0x83203788
	if ctx.cr[0].eq {
	pc = 0x83203788; continue 'dispatch;
	}
	// 83203778: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8320377C: 809F0014  lwz r4, 0x14(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 83203780: 4BFE22F1  bl 0x831e5a70
	ctx.lr = 0x83203784;
	sub_831E5A70(ctx, base);
	// 83203784: 48000008  b 0x8320378c
	pc = 0x8320378C; continue 'dispatch;
	// 83203788: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8320378C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83203790: 7FCBE92E  stwx r30, r11, r29
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[29].u32), ctx.r[30].u32) };
	// 83203794: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83203798: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8320379C: 7FCBE82E  lwzx r30, r11, r29
	ctx.r[30].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 832037A0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 832037A4: 4BFFFC85  bl 0x83203428
	ctx.lr = 0x832037A8;
	sub_83203428(ctx, base);
	// 832037A8: 93830000  stw r28, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 832037AC: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 832037B0: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 832037B4: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 832037B8: 4099000C  ble cr6, 0x832037c4
	if !ctx.cr[6].gt {
	pc = 0x832037C4; continue 'dispatch;
	}
	// 832037BC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832037C0: 4BFFFD19  bl 0x832034d8
	ctx.lr = 0x832037C4;
	sub_832034D8(ctx, base);
	// 832037C4: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832037C8: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 832037CC: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832037D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832037D4: 4BAA5C80  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832037D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832037D8 size=228
    let mut pc: u32 = 0x832037D8;
    'dispatch: loop {
        match pc {
            0x832037D8 => {
    //   block [0x832037D8..0x832038BC)
	// 832037D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832037DC: 4BAA5C29  bl 0x82ca9404
	ctx.lr = 0x832037E0;
	sub_82CA93D0(ctx, base);
	// 832037E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832037E4: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 832037E8: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 832037EC: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 832037F0: 817C0010  lwz r11, 0x10(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(16 as u32) ) } as u64;
	// 832037F4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 832037F8: 4E800421  bctrl
	ctx.lr = 0x832037FC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 832037FC: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 83203800: 815C0008  lwz r10, 8(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 83203804: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 83203808: 7D6B1838  and r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 & ctx.r[3].u64;
	// 8320380C: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83203810: 7FEB502E  lwzx r31, r11, r10
	ctx.r[31].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 83203814: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 83203818: 419A009C  beq cr6, 0x832038b4
	if ctx.cr[6].eq {
	pc = 0x832038B4; continue 'dispatch;
	}
	// 8320381C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83203820: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 83203824: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83203828: 4099008C  ble cr6, 0x832038b4
	if !ctx.cr[6].gt {
	pc = 0x832038B4; continue 'dispatch;
	}
	// 8320382C: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 83203830: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83203834: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 83203838: 40980010  bge cr6, 0x83203848
	if !ctx.cr[6].lt {
	pc = 0x83203848; continue 'dispatch;
	}
	// 8320383C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83203840: 7C6BEA14  add r3, r11, r29
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 83203844: 48000010  b 0x83203854
	pc = 0x83203854; continue 'dispatch;
	// 83203848: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8320384C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83203850: 4BFFFB51  bl 0x832033a0
	ctx.lr = 0x83203854;
	sub_832033A0(ctx, base);
	// 83203854: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83203858: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8320385C: 817C000C  lwz r11, 0xc(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(12 as u32) ) } as u64;
	// 83203860: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83203864: 4E800421  bctrl
	ctx.lr = 0x83203868;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83203868: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8320386C: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 83203870: 41820018  beq 0x83203888
	if ctx.cr[0].eq {
	pc = 0x83203888; continue 'dispatch;
	}
	// 83203874: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 83203878: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 8320387C: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 83203880: 4198FFB0  blt cr6, 0x83203830
	if ctx.cr[6].lt {
	pc = 0x83203830; continue 'dispatch;
	}
	// 83203884: 48000030  b 0x832038b4
	pc = 0x832038B4; continue 'dispatch;
	// 83203888: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8320388C: 40980028  bge cr6, 0x832038b4
	if !ctx.cr[6].lt {
	pc = 0x832038B4; continue 'dispatch;
	}
	// 83203890: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 83203894: 813F0008  lwz r9, 8(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83203898: 57CA103A  slwi r10, r30, 2
	ctx.r[10].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8320389C: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832038A0: 7D7E5850  subf r11, r30, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[30].s64;
	// 832038A4: 7C6A4A14  add r3, r10, r9
	ctx.r[3].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 832038A8: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 832038AC: 38830004  addi r4, r3, 4
	ctx.r[4].s64 = ctx.r[3].s64 + 4;
	// 832038B0: 4BAA5BD1  bl 0x82ca9480
	ctx.lr = 0x832038B4;
	sub_82CA9480(ctx, base);
	// 832038B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832038B8: 4BAA5B9C  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832038C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832038C0 size=196
    let mut pc: u32 = 0x832038C0;
    'dispatch: loop {
        match pc {
            0x832038C0 => {
    //   block [0x832038C0..0x83203984)
	// 832038C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832038C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832038C8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832038CC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832038D0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 832038D4: 809F000C  lwz r4, 0xc(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 832038D8: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 832038DC: 41980038  blt cr6, 0x83203914
	if ctx.cr[6].lt {
	pc = 0x83203914; continue 'dispatch;
	}
	// 832038E0: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 832038E4: 3964FFFF  addi r11, r4, -1
	ctx.r[11].s64 = ctx.r[4].s64 + -1;
	// 832038E8: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 832038EC: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 832038F0: 7F045840  cmplw cr6, r4, r11
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[11].u32, &mut ctx.xer);
	// 832038F4: 40980014  bge cr6, 0x83203908
	if !ctx.cr[6].lt {
	pc = 0x83203908; continue 'dispatch;
	}
	// 832038F8: 81430008  lwz r10, 8(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 832038FC: 548B103A  slwi r11, r4, 2
	ctx.r[11].u32 = ctx.r[4].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83203900: 7C6B5214  add r3, r11, r10
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 83203904: 48000008  b 0x8320390c
	pc = 0x8320390C; continue 'dispatch;
	// 83203908: 4BFFFA99  bl 0x832033a0
	ctx.lr = 0x8320390C;
	sub_832033A0(ctx, base);
	// 8320390C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83203910: 4800005C  b 0x8320396c
	pc = 0x8320396C; continue 'dispatch;
	// 83203914: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83203918: 813F0004  lwz r9, 4(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8320391C: 39690001  addi r11, r9, 1
	ctx.r[11].s64 = ctx.r[9].s64 + 1;
	// 83203920: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 83203924: 7D695B78  mr r9, r11
	ctx.r[9].u64 = ctx.r[11].u64;
	// 83203928: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8320392C: 7F0B4040  cmplw cr6, r11, r8
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[8].u32, &mut ctx.xer);
	// 83203930: 40980038  bge cr6, 0x83203968
	if !ctx.cr[6].lt {
	pc = 0x83203968; continue 'dispatch;
	}
	// 83203934: 810A0008  lwz r8, 8(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 83203938: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8320393C: 7D68582E  lwzx r11, r8, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 83203940: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83203944: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 83203948: 419AFFD4  beq cr6, 0x8320391c
	if ctx.cr[6].eq {
	pc = 0x8320391C; continue 'dispatch;
	}
	// 8320394C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83203950: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83203954: 419AFFC8  beq cr6, 0x8320391c
	if ctx.cr[6].eq {
	pc = 0x8320391C; continue 'dispatch;
	}
	// 83203958: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8320395C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83203960: 388BFFFF  addi r4, r11, -1
	ctx.r[4].s64 = ctx.r[11].s64 + -1;
	// 83203964: 4BFFFF7C  b 0x832038e0
	pc = 0x832038E0; continue 'dispatch;
	// 83203968: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8320396C: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 83203970: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83203974: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83203978: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8320397C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83203980: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83203988(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83203988 size=24
    let mut pc: u32 = 0x83203988;
    'dispatch: loop {
        match pc {
            0x83203988 => {
    //   block [0x83203988..0x832039A0)
	// 83203988: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 8320398C: 90830000  stw r4, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 83203990: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 83203994: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 83203998: 9143000C  stw r10, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 8320399C: 4BFFFF24  b 0x832038c0
	sub_832038C0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832039A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832039A0 size=28
    let mut pc: u32 = 0x832039A0;
    'dispatch: loop {
        match pc {
            0x832039A0 => {
    //   block [0x832039A0..0x832039BC)
	// 832039A0: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 832039A4: 1D44000C  mulli r10, r4, 0xc
	ctx.r[10].s64 = ctx.r[4].s64 * 12;
	// 832039A8: 396BF3B8  addi r11, r11, -0xc48
	ctx.r[11].s64 = ctx.r[11].s64 + -3144;
	// 832039AC: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 832039B0: 7D6A582E  lwzx r11, r10, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 832039B4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 832039B8: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832039C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832039C0 size=120
    let mut pc: u32 = 0x832039C0;
    'dispatch: loop {
        match pc {
            0x832039C0 => {
    //   block [0x832039C0..0x83203A38)
	// 832039C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832039C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832039C8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 832039CC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832039D0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832039D4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 832039D8: 9081008C  stw r4, 0x8c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(140 as u32), ctx.r[4].u32 ) };
	// 832039DC: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 832039E0: 3961008C  addi r11, r1, 0x8c
	ctx.r[11].s64 = ctx.r[1].s64 + 140;
	// 832039E4: 7D7F58AE  lbzx r11, r31, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 832039E8: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 832039EC: 40820024  bne 0x83203a10
	if !ctx.cr[0].eq {
	pc = 0x83203A10; continue 'dispatch;
	}
	// 832039F0: 807E0030  lwz r3, 0x30(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 832039F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832039F8: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 832039FC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83203A00: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83203A04: 816B0054  lwz r11, 0x54(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(84 as u32) ) } as u64;
	// 83203A08: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83203A0C: 4E800421  bctrl
	ctx.lr = 0x83203A10;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83203A10: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 83203A14: 2F1F0004  cmpwi cr6, r31, 4
	ctx.cr[6].compare_i32(ctx.r[31].s32, 4, &mut ctx.xer);
	// 83203A18: 4198FFC8  blt cr6, 0x832039e0
	if ctx.cr[6].lt {
	pc = 0x832039E0; continue 'dispatch;
	}
	// 83203A1C: 807E0034  lwz r3, 0x34(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(52 as u32) ) } as u64;
	// 83203A20: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83203A24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83203A28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83203A2C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83203A30: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83203A34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83203A38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83203A38 size=144
    let mut pc: u32 = 0x83203A38;
    'dispatch: loop {
        match pc {
            0x83203A38 => {
    //   block [0x83203A38..0x83203AC8)
	// 83203A38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83203A3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83203A40: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83203A44: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83203A48: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83203A4C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83203A50: 9081008C  stw r4, 0x8c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(140 as u32), ctx.r[4].u32 ) };
	// 83203A54: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 83203A58: 4BFE1FD1  bl 0x831e5a28
	ctx.lr = 0x83203A5C;
	sub_831E5A28(ctx, base);
	// 83203A5C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83203A60: 4182004C  beq 0x83203aac
	if ctx.cr[0].eq {
	pc = 0x83203AAC; continue 'dispatch;
	}
	// 83203A64: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 83203A68: 3961008C  addi r11, r1, 0x8c
	ctx.r[11].s64 = ctx.r[1].s64 + 140;
	// 83203A6C: 7D7F58AE  lbzx r11, r31, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 83203A70: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83203A74: 40820024  bne 0x83203a98
	if !ctx.cr[0].eq {
	pc = 0x83203A98; continue 'dispatch;
	}
	// 83203A78: 807E0038  lwz r3, 0x38(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(56 as u32) ) } as u64;
	// 83203A7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 83203A80: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 83203A84: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83203A88: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83203A8C: 816B0054  lwz r11, 0x54(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(84 as u32) ) } as u64;
	// 83203A90: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83203A94: 4E800421  bctrl
	ctx.lr = 0x83203A98;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83203A98: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 83203A9C: 2F1F0004  cmpwi cr6, r31, 4
	ctx.cr[6].compare_i32(ctx.r[31].s32, 4, &mut ctx.xer);
	// 83203AA0: 4198FFC8  blt cr6, 0x83203a68
	if ctx.cr[6].lt {
	pc = 0x83203A68; continue 'dispatch;
	}
	// 83203AA4: 807E003C  lwz r3, 0x3c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(60 as u32) ) } as u64;
	// 83203AA8: 48000008  b 0x83203ab0
	pc = 0x83203AB0; continue 'dispatch;
	// 83203AAC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83203AB0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83203AB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83203AB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83203ABC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83203AC0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83203AC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83203AC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83203AC8 size=100
    let mut pc: u32 = 0x83203AC8;
    'dispatch: loop {
        match pc {
            0x83203AC8 => {
    //   block [0x83203AC8..0x83203B2C)
	// 83203AC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83203ACC: 4BAA593D  bl 0x82ca9408
	ctx.lr = 0x83203AD0;
	sub_82CA93D0(ctx, base);
	// 83203AD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83203AD4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83203AD8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 83203ADC: 38800010  li r4, 0x10
	ctx.r[4].s64 = 16;
	// 83203AE0: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 83203AE4: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 83203AE8: 838B05AC  lwz r28, 0x5ac(r11)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1452 as u32) ) } as u64;
	// 83203AEC: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83203AF0: 4BFC86A9  bl 0x831cc198
	ctx.lr = 0x83203AF4;
	sub_831CC198(ctx, base);
	// 83203AF4: 35630004  addic. r11, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83203AF8: 93830000  stw r28, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 83203AFC: 41820018  beq 0x83203b14
	if ctx.cr[0].eq {
	pc = 0x83203B14; continue 'dispatch;
	}
	// 83203B00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83203B04: 93CB0000  stw r30, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 83203B08: 93EB0004  stw r31, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 83203B0C: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 83203B10: 48000008  b 0x83203b18
	pc = 0x83203B18; continue 'dispatch;
	// 83203B14: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83203B18: 815D0004  lwz r10, 4(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 83203B1C: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 83203B20: 917D0004  stw r11, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 83203B24: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83203B28: 4BAA5930  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83203B30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83203B30 size=16
    let mut pc: u32 = 0x83203B30;
    'dispatch: loop {
        match pc {
            0x83203B30 => {
    //   block [0x83203B30..0x83203B40)
	// 83203B30: 8143002C  lwz r10, 0x2c(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) } as u64;
	// 83203B34: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 83203B38: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83203B3C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83203B40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83203B40 size=20
    let mut pc: u32 = 0x83203B40;
    'dispatch: loop {
        match pc {
            0x83203B40 => {
    //   block [0x83203B40..0x83203B54)
	// 83203B40: 812B0008  lwz r9, 8(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83203B44: 388BFFFC  addi r4, r11, -4
	ctx.r[4].s64 = ctx.r[11].s64 + -4;
	// 83203B48: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 83203B4C: 806BFFFC  lwz r3, -4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-4 as u32) ) } as u64;
	// 83203B50: 4BFC8778  b 0x831cc2c8
	sub_831CC2C8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83203B54(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83203B54 size=4
    let mut pc: u32 = 0x83203B54;
    'dispatch: loop {
        match pc {
            0x83203B54 => {
    //   block [0x83203B54..0x83203B58)
	// 83203B54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83203B58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83203B58 size=20
    let mut pc: u32 = 0x83203B58;
    'dispatch: loop {
        match pc {
            0x83203B58 => {
    //   block [0x83203B58..0x83203B6C)
	// 83203B58: 8063002C  lwz r3, 0x2c(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) } as u64;
	// 83203B5C: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 83203B60: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83203B64: 409A0008  bne cr6, 0x83203b6c
	if !ctx.cr[6].eq {
		sub_83203B6C(ctx, base);
		return;
	}
	// 83203B68: 4BFFFF60  b 0x83203ac8
	sub_83203AC8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83203B6C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83203B6C size=24
    let mut pc: u32 = 0x83203B6C;
    'dispatch: loop {
        match pc {
            0x83203B6C => {
    //   block [0x83203B6C..0x83203B84)
	// 83203B6C: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 83203B70: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83203B74: 7F045000  cmpw cr6, r4, r10
	ctx.cr[6].compare_i32(ctx.r[4].s32, ctx.r[10].s32, &mut ctx.xer);
	// 83203B78: 409A000C  bne cr6, 0x83203b84
	if !ctx.cr[6].eq {
		sub_83203B84(ctx, base);
		return;
	}
	// 83203B7C: 90AB0000  stw r5, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[5].u32 ) };
	// 83203B80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83203B84(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83203B84 size=4
    let mut pc: u32 = 0x83203B84;
    'dispatch: loop {
        match pc {
            0x83203B84 => {
    //   block [0x83203B84..0x83203B88)
	// 83203B84: 4BFFFF44  b 0x83203ac8
	sub_83203AC8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83203B88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83203B88 size=88
    let mut pc: u32 = 0x83203B88;
    'dispatch: loop {
        match pc {
            0x83203B88 => {
    //   block [0x83203B88..0x83203BE0)
	// 83203B88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83203B8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83203B90: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83203B94: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83203B98: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83203B9C: 48000024  b 0x83203bc0
	pc = 0x83203BC0; continue 'dispatch;
	// 83203BA0: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83203BA4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83203BA8: 419A0018  beq cr6, 0x83203bc0
	if ctx.cr[6].eq {
	pc = 0x83203BC0; continue 'dispatch;
	}
	// 83203BAC: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83203BB0: 388BFFFC  addi r4, r11, -4
	ctx.r[4].s64 = ctx.r[11].s64 + -4;
	// 83203BB4: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 83203BB8: 806BFFFC  lwz r3, -4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-4 as u32) ) } as u64;
	// 83203BBC: 4BFC870D  bl 0x831cc2c8
	ctx.lr = 0x83203BC0;
	sub_831CC2C8(ctx, base);
	// 83203BC0: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83203BC4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83203BC8: 409AFFD8  bne cr6, 0x83203ba0
	if !ctx.cr[6].eq {
	pc = 0x83203BA0; continue 'dispatch;
	}
	// 83203BCC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83203BD0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83203BD4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83203BD8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83203BDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83203BE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83203BE0 size=256
    let mut pc: u32 = 0x83203BE0;
    'dispatch: loop {
        match pc {
            0x83203BE0 => {
    //   block [0x83203BE0..0x83203CE0)
	// 83203BE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83203BE4: 4BAA5821  bl 0x82ca9404
	ctx.lr = 0x83203BE8;
	sub_82CA93D0(ctx, base);
	// 83203BE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83203BEC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83203BF0: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 83203BF4: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 83203BF8: 396BA998  addi r11, r11, -0x5668
	ctx.r[11].s64 = ctx.r[11].s64 + -22120;
	// 83203BFC: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 83203C00: 909F000C  stw r4, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[4].u32 ) };
	// 83203C04: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 83203C08: 90BF0020  stw r5, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[5].u32 ) };
	// 83203C0C: 3880000C  li r4, 0xc
	ctx.r[4].s64 = 12;
	// 83203C10: 9BBF0004  stb r29, 4(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[29].u8 ) };
	// 83203C14: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83203C18: 9BBF0005  stb r29, 5(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(5 as u32), ctx.r[29].u8 ) };
	// 83203C1C: 915F0010  stw r10, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 83203C20: 93BF0014  stw r29, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[29].u32 ) };
	// 83203C24: 93BF0018  stw r29, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[29].u32 ) };
	// 83203C28: 9BBF001C  stb r29, 0x1c(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[29].u8 ) };
	// 83203C2C: 9BBF001D  stb r29, 0x1d(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(29 as u32), ctx.r[29].u8 ) };
	// 83203C30: 839E05AC  lwz r28, 0x5ac(r30)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(1452 as u32) ) } as u64;
	// 83203C34: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83203C38: 4BFC8561  bl 0x831cc198
	ctx.lr = 0x83203C3C;
	sub_831CC198(ctx, base);
	// 83203C3C: 35630004  addic. r11, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83203C40: 93830000  stw r28, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 83203C44: 41820010  beq 0x83203c54
	if ctx.cr[0].eq {
	pc = 0x83203C54; continue 'dispatch;
	}
	// 83203C48: 93CB0000  stw r30, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 83203C4C: 93AB0004  stw r29, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 83203C50: 48000008  b 0x83203c58
	pc = 0x83203C58; continue 'dispatch;
	// 83203C54: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 83203C58: 917F002C  stw r11, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 83203C5C: 38800014  li r4, 0x14
	ctx.r[4].s64 = 20;
	// 83203C60: 837E05AC  lwz r27, 0x5ac(r30)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(1452 as u32) ) } as u64;
	// 83203C64: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 83203C68: 4BFC8531  bl 0x831cc198
	ctx.lr = 0x83203C6C;
	sub_831CC198(ctx, base);
	// 83203C6C: 37830004  addic. r28, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[28].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 83203C70: 93630000  stw r27, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 83203C74: 41820014  beq 0x83203c88
	if ctx.cr[0].eq {
	pc = 0x83203C88; continue 'dispatch;
	}
	// 83203C78: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83203C7C: 809E05AC  lwz r4, 0x5ac(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(1452 as u32) ) } as u64;
	// 83203C80: 4BFE1DF1  bl 0x831e5a70
	ctx.lr = 0x83203C84;
	sub_831E5A70(ctx, base);
	// 83203C84: 48000008  b 0x83203c8c
	pc = 0x83203C8C; continue 'dispatch;
	// 83203C88: 7FBCEB78  mr r28, r29
	ctx.r[28].u64 = ctx.r[29].u64;
	// 83203C8C: 939F0024  stw r28, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[28].u32 ) };
	// 83203C90: 38800014  li r4, 0x14
	ctx.r[4].s64 = 20;
	// 83203C94: 837E05AC  lwz r27, 0x5ac(r30)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(1452 as u32) ) } as u64;
	// 83203C98: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 83203C9C: 4BFC84FD  bl 0x831cc198
	ctx.lr = 0x83203CA0;
	sub_831CC198(ctx, base);
	// 83203CA0: 37830004  addic. r28, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[28].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 83203CA4: 93630000  stw r27, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 83203CA8: 41820014  beq 0x83203cbc
	if ctx.cr[0].eq {
	pc = 0x83203CBC; continue 'dispatch;
	}
	// 83203CAC: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83203CB0: 809E05AC  lwz r4, 0x5ac(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(1452 as u32) ) } as u64;
	// 83203CB4: 4BFE1DBD  bl 0x831e5a70
	ctx.lr = 0x83203CB8;
	sub_831E5A70(ctx, base);
	// 83203CB8: 7F9DE378  mr r29, r28
	ctx.r[29].u64 = ctx.r[28].u64;
	// 83203CBC: 93BF0028  stw r29, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[29].u32 ) };
	// 83203CC0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83203CC4: 817E0598  lwz r11, 0x598(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(1432 as u32) ) } as u64;
	// 83203CC8: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 83203CCC: 817E0598  lwz r11, 0x598(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(1432 as u32) ) } as u64;
	// 83203CD0: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83203CD4: 917E0598  stw r11, 0x598(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(1432 as u32), ctx.r[11].u32 ) };
	// 83203CD8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83203CDC: 4BAA5778  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83203CE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83203CE0 size=8
    let mut pc: u32 = 0x83203CE0;
    'dispatch: loop {
        match pc {
            0x83203CE0 => {
    //   block [0x83203CE0..0x83203CE8)
	// 83203CE0: 98830004  stb r4, 4(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[4].u8 ) };
	// 83203CE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83203CE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83203CE8 size=80
    let mut pc: u32 = 0x83203CE8;
    'dispatch: loop {
        match pc {
            0x83203CE8 => {
    //   block [0x83203CE8..0x83203D38)
	// 83203CE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83203CEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83203CF0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83203CF4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83203CF8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83203CFC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83203D00: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83203D04: 807F0028  lwz r3, 0x28(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 83203D08: 80830004  lwz r4, 4(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 83203D0C: 4BFFF695  bl 0x832033a0
	ctx.lr = 0x83203D10;
	sub_832033A0(ctx, base);
	// 83203D10: 93C30000  stw r30, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 83203D14: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 83203D18: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83203D1C: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 83203D20: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83203D24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83203D28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83203D2C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83203D30: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83203D34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83203D38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83203D38 size=124
    let mut pc: u32 = 0x83203D38;
    'dispatch: loop {
        match pc {
            0x83203D38 => {
    //   block [0x83203D38..0x83203DB4)
	// 83203D38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83203D3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83203D40: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83203D44: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83203D48: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83203D4C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83203D50: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 83203D54: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 83203D58: 2F040001  cmpwi cr6, r4, 1
	ctx.cr[6].compare_i32(ctx.r[4].s32, 1, &mut ctx.xer);
	// 83203D5C: 40990024  ble cr6, 0x83203d80
	if !ctx.cr[6].gt {
	pc = 0x83203D80; continue 'dispatch;
	}
	// 83203D60: 397E0020  addi r11, r30, 0x20
	ctx.r[11].s64 = ctx.r[30].s64 + 32;
	// 83203D64: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83203D68: 7F09F840  cmplw cr6, r9, r31
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[31].u32, &mut ctx.xer);
	// 83203D6C: 419A0030  beq cr6, 0x83203d9c
	if ctx.cr[6].eq {
	pc = 0x83203D9C; continue 'dispatch;
	}
	// 83203D70: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 83203D74: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 83203D78: 7F0A2000  cmpw cr6, r10, r4
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[4].s32, &mut ctx.xer);
	// 83203D7C: 4198FFE8  blt cr6, 0x83203d64
	if ctx.cr[6].lt {
	pc = 0x83203D64; continue 'dispatch;
	}
	// 83203D80: 807F0024  lwz r3, 0x24(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 83203D84: 80830004  lwz r4, 4(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 83203D88: 4BFFF619  bl 0x832033a0
	ctx.lr = 0x83203D8C;
	sub_832033A0(ctx, base);
	// 83203D8C: 93C30000  stw r30, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 83203D90: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 83203D94: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83203D98: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 83203D9C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83203DA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83203DA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83203DA8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83203DAC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83203DB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83203DB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x83203DB8 size=728
    let mut pc: u32 = 0x83203DB8;
    'dispatch: loop {
        match pc {
            0x83203DB8 => {
    //   block [0x83203DB8..0x83204090)
	// 83203DB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83203DBC: 4BAA5639  bl 0x82ca93f4
	ctx.lr = 0x83203DC0;
	sub_82CA93D0(ctx, base);
	// 83203DC0: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83203DC4: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 83203DC8: 7C982378  mr r24, r4
	ctx.r[24].u64 = ctx.r[4].u64;
	// 83203DCC: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 83203DD0: 817A002C  lwz r11, 0x2c(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(44 as u32) ) } as u64;
	// 83203DD4: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83203DD8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 83203DDC: 419A0010  beq cr6, 0x83203dec
	if ctx.cr[6].eq {
	pc = 0x83203DEC; continue 'dispatch;
	}
	// 83203DE0: 554B003E  slwi r11, r10, 0
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83203DE4: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83203DE8: 480002A0  b 0x83204088
	pc = 0x83204088; continue 'dispatch;
	// 83203DEC: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 83203DF0: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 83203DF4: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 83203DF8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83203DFC: 4E800421  bctrl
	ctx.lr = 0x83203E00;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83203E00: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83203E04: 3B200000  li r25, 0
	ctx.r[25].s64 = 0;
	// 83203E08: 3B600003  li r27, 3
	ctx.r[27].s64 = 3;
	// 83203E0C: 41820110  beq 0x83203f1c
	if ctx.cr[0].eq {
	pc = 0x83203F1C; continue 'dispatch;
	}
	// 83203E10: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 83203E14: 38600031  li r3, 0x31
	ctx.r[3].s64 = 49;
	// 83203E18: 48003DF9  bl 0x83207c10
	ctx.lr = 0x83203E1C;
	sub_83207C10(ctx, base);
	// 83203E1C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83203E20: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 83203E24: 809C0AB0  lwz r4, 0xab0(r28)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(2736 as u32) ) } as u64;
	// 83203E28: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 83203E2C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83203E30: C06B0C18  lfs f3, 0xc18(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3096 as u32) ) };
	ctx.f[3].f64 = (tmp.f32 as f64);
	// 83203E34: C08A0C14  lfs f4, 0xc14(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(3092 as u32) ) };
	ctx.f[4].f64 = (tmp.f32 as f64);
	// 83203E38: FC401890  fmr f2, f3
	ctx.f[2].f64 = ctx.f[3].f64;
	// 83203E3C: FC201890  fmr f1, f3
	ctx.f[1].f64 = ctx.f[3].f64;
	// 83203E40: 48002949  bl 0x83206788
	ctx.lr = 0x83203E44;
	sub_83206788(ctx, base);
	// 83203E44: 817F00E0  lwz r11, 0xe0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(224 as u32) ) } as u64;
	// 83203E48: 815F0020  lwz r10, 0x20(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 83203E4C: 917F0038  stw r11, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 83203E50: 806A0028  lwz r3, 0x28(r10)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(40 as u32) ) } as u64;
	// 83203E54: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 83203E58: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83203E5C: 4099000C  ble cr6, 0x83203e68
	if !ctx.cr[6].gt {
	pc = 0x83203E68; continue 'dispatch;
	}
	// 83203E60: 80630008  lwz r3, 8(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 83203E64: 4800000C  b 0x83203e70
	pc = 0x83203E70; continue 'dispatch;
	// 83203E68: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83203E6C: 4BFFF535  bl 0x832033a0
	ctx.lr = 0x83203E70;
	sub_832033A0(ctx, base);
	// 83203E70: 83C30000  lwz r30, 0(r3)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83203E74: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83203E78: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83203E7C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 83203E80: 4BFE1E81  bl 0x831e5d00
	ctx.lr = 0x83203E84;
	sub_831E5D00(ctx, base);
	// 83203E84: 83BE001C  lwz r29, 0x1c(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 83203E88: 817D002C  lwz r11, 0x2c(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(44 as u32) ) } as u64;
	// 83203E8C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83203E90: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83203E94: 409A0100  bne cr6, 0x83203f94
	if !ctx.cr[6].eq {
	pc = 0x83203F94; continue 'dispatch;
	}
	// 83203E98: 82FC05B0  lwz r23, 0x5b0(r28)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(1456 as u32) ) } as u64;
	// 83203E9C: 38800110  li r4, 0x110
	ctx.r[4].s64 = 272;
	// 83203EA0: 7EE3BB78  mr r3, r23
	ctx.r[3].u64 = ctx.r[23].u64;
	// 83203EA4: 4BFC82F5  bl 0x831cc198
	ctx.lr = 0x83203EA8;
	sub_831CC198(ctx, base);
	// 83203EA8: 35630004  addic. r11, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83203EAC: 92E30000  stw r23, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[23].u32 ) };
	// 83203EB0: 41820028  beq 0x83203ed8
	if ctx.cr[0].eq {
	pc = 0x83203ED8; continue 'dispatch;
	}
	// 83203EB4: 93CB0010  stw r30, 0x10(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[30].u32 ) };
	// 83203EB8: 7D7E5B78  mr r30, r11
	ctx.r[30].u64 = ctx.r[11].u64;
	// 83203EBC: 938B0104  stw r28, 0x104(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(260 as u32), ctx.r[28].u32 ) };
	// 83203EC0: 9B2B0108  stb r25, 0x108(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(264 as u32), ctx.r[25].u8 ) };
	// 83203EC4: 936B0000  stw r27, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 83203EC8: 936B0004  stw r27, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[27].u32 ) };
	// 83203ECC: 936B0008  stw r27, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[27].u32 ) };
	// 83203ED0: 936B000C  stw r27, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[27].u32 ) };
	// 83203ED4: 48000008  b 0x83203edc
	pc = 0x83203EDC; continue 'dispatch;
	// 83203ED8: 7F3ECB78  mr r30, r25
	ctx.r[30].u64 = ctx.r[25].u64;
	// 83203EDC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83203EE0: 4BFF4969  bl 0x831f8848
	ctx.lr = 0x83203EE4;
	sub_831F8848(ctx, base);
	// 83203EE4: 807D002C  lwz r3, 0x2c(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(44 as u32) ) } as u64;
	// 83203EE8: 80980030  lwz r4, 0x30(r24)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(48 as u32) ) } as u64;
	// 83203EEC: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 83203EF0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83203EF4: 419A001C  beq cr6, 0x83203f10
	if ctx.cr[6].eq {
	pc = 0x83203F10; continue 'dispatch;
	}
	// 83203EF8: 556B003E  slwi r11, r11, 0
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83203EFC: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83203F00: 7F045000  cmpw cr6, r4, r10
	ctx.cr[6].compare_i32(ctx.r[4].s32, ctx.r[10].s32, &mut ctx.xer);
	// 83203F04: 409A000C  bne cr6, 0x83203f10
	if !ctx.cr[6].eq {
	pc = 0x83203F10; continue 'dispatch;
	}
	// 83203F08: 93CB0000  stw r30, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 83203F0C: 48000088  b 0x83203f94
	pc = 0x83203F94; continue 'dispatch;
	// 83203F10: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 83203F14: 4BFFFBB5  bl 0x83203ac8
	ctx.lr = 0x83203F18;
	sub_83203AC8(ctx, base);
	// 83203F18: 4800007C  b 0x83203f94
	pc = 0x83203F94; continue 'dispatch;
	// 83203F1C: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 83203F20: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 83203F24: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 83203F28: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83203F2C: 4E800421  bctrl
	ctx.lr = 0x83203F30;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83203F30: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83203F34: 40820028  bne 0x83203f5c
	if !ctx.cr[0].eq {
	pc = 0x83203F5C; continue 'dispatch;
	}
	// 83203F38: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 83203F3C: 3D40820A  lis r10, -0x7df6
	ctx.r[10].s64 = -2113273856;
	// 83203F40: 3D208208  lis r9, -0x7df8
	ctx.r[9].s64 = -2113404928;
	// 83203F44: 38CBA810  addi r6, r11, -0x57f0
	ctx.r[6].s64 = ctx.r[11].s64 + -22512;
	// 83203F48: 38AA1700  addi r5, r10, 0x1700
	ctx.r[5].s64 = ctx.r[10].s64 + 5888;
	// 83203F4C: 3889B734  addi r4, r9, -0x48cc
	ctx.r[4].s64 = ctx.r[9].s64 + -18636;
	// 83203F50: 38E000EC  li r7, 0xec
	ctx.r[7].s64 = 236;
	// 83203F54: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83203F58: 4BF05E59  bl 0x83109db0
	ctx.lr = 0x83203F5C;
	sub_83109DB0(ctx, base);
	// 83203F5C: 83FC05AC  lwz r31, 0x5ac(r28)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(1452 as u32) ) } as u64;
	// 83203F60: 388003C4  li r4, 0x3c4
	ctx.r[4].s64 = 964;
	// 83203F64: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83203F68: 4BFC8231  bl 0x831cc198
	ctx.lr = 0x83203F6C;
	sub_831CC198(ctx, base);
	// 83203F6C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83203F70: 346B0004  addic. r3, r11, 4
	ctx.xer.ca = (ctx.r[11].u32 > (!(4 as u32)));
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83203F74: 93EB0000  stw r31, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 83203F78: 41820018  beq 0x83203f90
	if ctx.cr[0].eq {
	pc = 0x83203F90; continue 'dispatch;
	}
	// 83203F7C: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 83203F80: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 83203F84: 48002CBD  bl 0x83206c40
	ctx.lr = 0x83203F88;
	sub_83206C40(ctx, base);
	// 83203F88: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83203F8C: 48000008  b 0x83203f94
	pc = 0x83203F94; continue 'dispatch;
	// 83203F90: 7F3FCB78  mr r31, r25
	ctx.r[31].u64 = ctx.r[25].u64;
	// 83203F94: 935F001C  stw r26, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[26].u32 ) };
	// 83203F98: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83203F9C: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 83203FA0: 4BFFFD49  bl 0x83203ce8
	ctx.lr = 0x83203FA4;
	sub_83203CE8(ctx, base);
	// 83203FA4: 81780000  lwz r11, 0(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(0 as u32) ) } as u64;
	// 83203FA8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83203FAC: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 83203FB0: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 83203FB4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83203FB8: 4E800421  bctrl
	ctx.lr = 0x83203FBC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83203FBC: 897A0005  lbz r11, 5(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[26].u32.wrapping_add(5 as u32) ) } as u64;
	// 83203FC0: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83203FC4: 41820024  beq 0x83203fe8
	if ctx.cr[0].eq {
	pc = 0x83203FE8; continue 'dispatch;
	}
	// 83203FC8: 817A0020  lwz r11, 0x20(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(32 as u32) ) } as u64;
	// 83203FCC: 815A0010  lwz r10, 0x10(r26)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(16 as u32) ) } as u64;
	// 83203FD0: 813F00E4  lwz r9, 0xe4(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(228 as u32) ) } as u64;
	// 83203FD4: 61290040  ori r9, r9, 0x40
	ctx.r[9].u64 = ctx.r[9].u64 | 64;
	// 83203FD8: 917F0050  stw r11, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 83203FDC: 915F0038  stw r10, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[10].u32 ) };
	// 83203FE0: 913F00E4  stw r9, 0xe4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(228 as u32), ctx.r[9].u32 ) };
	// 83203FE4: 4800002C  b 0x83204010
	pc = 0x83204010; continue 'dispatch;
	// 83203FE8: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 83203FEC: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 83203FF0: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 83203FF4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83203FF8: 4E800421  bctrl
	ctx.lr = 0x83203FFC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83203FFC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83204000: 41820010  beq 0x83204010
	if ctx.cr[0].eq {
	pc = 0x83204010; continue 'dispatch;
	}
	// 83204004: 817F00E0  lwz r11, 0xe0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(224 as u32) ) } as u64;
	// 83204008: 933F0050  stw r25, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[25].u32 ) };
	// 8320400C: 917F0038  stw r11, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 83204010: 897A0004  lbz r11, 4(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 83204014: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83204018: 41820010  beq 0x83204028
	if ctx.cr[0].eq {
	pc = 0x83204028; continue 'dispatch;
	}
	// 8320401C: 817F00E4  lwz r11, 0xe4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(228 as u32) ) } as u64;
	// 83204020: 616B0020  ori r11, r11, 0x20
	ctx.r[11].u64 = ctx.r[11].u64 | 32;
	// 83204024: 917F00E4  stw r11, 0xe4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(228 as u32), ctx.r[11].u32 ) };
	// 83204028: 83DC05B0  lwz r30, 0x5b0(r28)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(1456 as u32) ) } as u64;
	// 8320402C: 38800110  li r4, 0x110
	ctx.r[4].s64 = 272;
	// 83204030: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83204034: 4BFC8165  bl 0x831cc198
	ctx.lr = 0x83204038;
	sub_831CC198(ctx, base);
	// 83204038: 35630004  addic. r11, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8320403C: 93C30000  stw r30, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 83204040: 41820028  beq 0x83204068
	if ctx.cr[0].eq {
	pc = 0x83204068; continue 'dispatch;
	}
	// 83204044: 93EB0010  stw r31, 0x10(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[31].u32 ) };
	// 83204048: 7D7F5B78  mr r31, r11
	ctx.r[31].u64 = ctx.r[11].u64;
	// 8320404C: 938B0104  stw r28, 0x104(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(260 as u32), ctx.r[28].u32 ) };
	// 83204050: 9B2B0108  stb r25, 0x108(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(264 as u32), ctx.r[25].u8 ) };
	// 83204054: 936B0000  stw r27, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 83204058: 936B0004  stw r27, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[27].u32 ) };
	// 8320405C: 936B0008  stw r27, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[27].u32 ) };
	// 83204060: 936B000C  stw r27, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[27].u32 ) };
	// 83204064: 48000008  b 0x8320406c
	pc = 0x8320406C; continue 'dispatch;
	// 83204068: 7F3FCB78  mr r31, r25
	ctx.r[31].u64 = ctx.r[25].u64;
	// 8320406C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83204070: 4BFF47D9  bl 0x831f8848
	ctx.lr = 0x83204074;
	sub_831F8848(ctx, base);
	// 83204074: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 83204078: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 8320407C: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 83204080: 48005301  bl 0x83209380
	ctx.lr = 0x83204084;
	sub_83209380(ctx, base);
	// 83204084: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83204088: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 8320408C: 4BAA53B8  b 0x82ca9444
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83204090(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83204090 size=164
    let mut pc: u32 = 0x83204090;
    'dispatch: loop {
        match pc {
            0x83204090 => {
    //   block [0x83204090..0x83204134)
	// 83204090: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83204094: 4BAA5371  bl 0x82ca9404
	ctx.lr = 0x83204098;
	sub_82CA93D0(ctx, base);
	// 83204098: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8320409C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 832040A0: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 832040A4: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 832040A8: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 832040AC: 807D0024  lwz r3, 0x24(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(36 as u32) ) } as u64;
	// 832040B0: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 832040B4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 832040B8: 40990074  ble cr6, 0x8320412c
	if !ctx.cr[6].gt {
	pc = 0x8320412C; continue 'dispatch;
	}
	// 832040BC: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 832040C0: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 832040C4: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 832040C8: 40980010  bge cr6, 0x832040d8
	if !ctx.cr[6].lt {
	pc = 0x832040D8; continue 'dispatch;
	}
	// 832040CC: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 832040D0: 7C6BF214  add r3, r11, r30
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 832040D4: 4800000C  b 0x832040e0
	pc = 0x832040E0; continue 'dispatch;
	// 832040D8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 832040DC: 4BFFF2C5  bl 0x832033a0
	ctx.lr = 0x832040E0;
	sub_832033A0(ctx, base);
	// 832040E0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 832040E4: 807D0024  lwz r3, 0x24(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(36 as u32) ) } as u64;
	// 832040E8: 7F0BE040  cmplw cr6, r11, r28
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[28].u32, &mut ctx.xer);
	// 832040EC: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 832040F0: 419A0018  beq cr6, 0x83204108
	if ctx.cr[6].eq {
	pc = 0x83204108; continue 'dispatch;
	}
	// 832040F4: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 832040F8: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 832040FC: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 83204100: 4198FFC0  blt cr6, 0x832040c0
	if ctx.cr[6].lt {
	pc = 0x832040C0; continue 'dispatch;
	}
	// 83204104: 48000028  b 0x8320412c
	pc = 0x8320412C; continue 'dispatch;
	// 83204108: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8320410C: 40980014  bge cr6, 0x83204120
	if !ctx.cr[6].lt {
	pc = 0x83204120; continue 'dispatch;
	}
	// 83204110: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 83204114: 57EA103A  slwi r10, r31, 2
	ctx.r[10].u32 = ctx.r[31].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83204118: 7C6B5214  add r3, r11, r10
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8320411C: 4800000C  b 0x83204128
	pc = 0x83204128; continue 'dispatch;
	// 83204120: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83204124: 4BFFF27D  bl 0x832033a0
	ctx.lr = 0x83204128;
	sub_832033A0(ctx, base);
	// 83204128: 93630000  stw r27, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 8320412C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83204130: 4BAA5324  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83204138(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83204138 size=132
    let mut pc: u32 = 0x83204138;
    'dispatch: loop {
        match pc {
            0x83204138 => {
    //   block [0x83204138..0x832041BC)
	// 83204138: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8320413C: 4BAA52CD  bl 0x82ca9408
	ctx.lr = 0x83204140;
	sub_82CA93D0(ctx, base);
	// 83204140: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83204144: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83204148: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 8320414C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 83204150: 807D0024  lwz r3, 0x24(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(36 as u32) ) } as u64;
	// 83204154: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 83204158: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8320415C: 40990058  ble cr6, 0x832041b4
	if !ctx.cr[6].gt {
	pc = 0x832041B4; continue 'dispatch;
	}
	// 83204160: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 83204164: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 83204168: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8320416C: 40980010  bge cr6, 0x8320417c
	if !ctx.cr[6].lt {
	pc = 0x8320417C; continue 'dispatch;
	}
	// 83204170: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 83204174: 7C6BF214  add r3, r11, r30
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 83204178: 4800000C  b 0x83204184
	pc = 0x83204184; continue 'dispatch;
	// 8320417C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83204180: 4BFFF221  bl 0x832033a0
	ctx.lr = 0x83204184;
	sub_832033A0(ctx, base);
	// 83204184: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83204188: 807D0024  lwz r3, 0x24(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(36 as u32) ) } as u64;
	// 8320418C: 7F0BE040  cmplw cr6, r11, r28
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[28].u32, &mut ctx.xer);
	// 83204190: 419A001C  beq cr6, 0x832041ac
	if ctx.cr[6].eq {
	pc = 0x832041AC; continue 'dispatch;
	}
	// 83204194: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 83204198: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 8320419C: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 832041A0: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 832041A4: 4198FFC0  blt cr6, 0x83204164
	if ctx.cr[6].lt {
	pc = 0x83204164; continue 'dispatch;
	}
	// 832041A8: 4800000C  b 0x832041b4
	pc = 0x832041B4; continue 'dispatch;
	// 832041AC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 832041B0: 4BFFF131  bl 0x832032e0
	ctx.lr = 0x832041B4;
	sub_832032E0(ctx, base);
	// 832041B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832041B8: 4BAA52A0  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832041C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832041C0 size=440
    let mut pc: u32 = 0x832041C0;
    'dispatch: loop {
        match pc {
            0x832041C0 => {
    //   block [0x832041C0..0x83204378)
	// 832041C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832041C4: 4BAA5235  bl 0x82ca93f8
	ctx.lr = 0x832041C8;
	sub_82CA93D0(ctx, base);
	// 832041C8: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832041CC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 832041D0: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 832041D4: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 832041D8: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 832041DC: 817D0034  lwz r11, 0x34(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(52 as u32) ) } as u64;
	// 832041E0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 832041E4: 409A0020  bne cr6, 0x83204204
	if !ctx.cr[6].eq {
	pc = 0x83204204; continue 'dispatch;
	}
	// 832041E8: 817F05E4  lwz r11, 0x5e4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1508 as u32) ) } as u64;
	// 832041EC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 832041F0: 38ABFFFF  addi r5, r11, -1
	ctx.r[5].s64 = ctx.r[11].s64 + -1;
	// 832041F4: 90BF05E4  stw r5, 0x5e4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1508 as u32), ctx.r[5].u32 ) };
	// 832041F8: 807E00AC  lwz r3, 0xac(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(172 as u32) ) } as u64;
	// 832041FC: 4BFF0A5D  bl 0x831f4c58
	ctx.lr = 0x83204200;
	sub_831F4C58(ctx, base);
	// 83204200: 907D0034  stw r3, 0x34(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(52 as u32), ctx.r[3].u32 ) };
	// 83204204: 83FF05AC  lwz r31, 0x5ac(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1452 as u32) ) } as u64;
	// 83204208: 388003C4  li r4, 0x3c4
	ctx.r[4].s64 = 964;
	// 8320420C: 831D0034  lwz r24, 0x34(r29)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(52 as u32) ) } as u64;
	// 83204210: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83204214: 4BFC7F85  bl 0x831cc198
	ctx.lr = 0x83204218;
	sub_831CC198(ctx, base);
	// 83204218: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8320421C: 346B0004  addic. r3, r11, 4
	ctx.xer.ca = (ctx.r[11].u32 > (!(4 as u32)));
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83204220: 93EB0000  stw r31, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 83204224: 41820014  beq 0x83204238
	if ctx.cr[0].eq {
	pc = 0x83204238; continue 'dispatch;
	}
	// 83204228: 809E000C  lwz r4, 0xc(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 8320422C: 48002725  bl 0x83206950
	ctx.lr = 0x83204230;
	sub_83206950(ctx, base);
	// 83204230: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83204234: 48000008  b 0x8320423c
	pc = 0x8320423C; continue 'dispatch;
	// 83204238: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8320423C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 83204240: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83204244: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83204248: 48001921  bl 0x83205b68
	ctx.lr = 0x8320424C;
	sub_83205B68(ctx, base);
	// 8320424C: 817D0020  lwz r11, 0x20(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(32 as u32) ) } as u64;
	// 83204250: 93FD0030  stw r31, 0x30(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(48 as u32), ctx.r[31].u32 ) };
	// 83204254: 2F0B0005  cmpwi cr6, r11, 5
	ctx.cr[6].compare_i32(ctx.r[11].s32, 5, &mut ctx.xer);
	// 83204258: 409A0008  bne cr6, 0x83204260
	if !ctx.cr[6].eq {
	pc = 0x83204260; continue 'dispatch;
	}
	// 8320425C: 93FE0074  stw r31, 0x74(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(116 as u32), ctx.r[31].u32 ) };
	// 83204260: 817D0020  lwz r11, 0x20(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(32 as u32) ) } as u64;
	// 83204264: 2F0B0006  cmpwi cr6, r11, 6
	ctx.cr[6].compare_i32(ctx.r[11].s32, 6, &mut ctx.xer);
	// 83204268: 409A0010  bne cr6, 0x83204278
	if !ctx.cr[6].eq {
	pc = 0x83204278; continue 'dispatch;
	}
	// 8320426C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83204270: 93FE0078  stw r31, 0x78(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(120 as u32), ctx.r[31].u32 ) };
	// 83204274: 997E007E  stb r11, 0x7e(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(126 as u32), ctx.r[11].u8 ) };
	// 83204278: 815D0020  lwz r10, 0x20(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(32 as u32) ) } as u64;
	// 8320427C: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 83204280: 2F0A0007  cmpwi cr6, r10, 7
	ctx.cr[6].compare_i32(ctx.r[10].s32, 7, &mut ctx.xer);
	// 83204284: 3B8BA808  addi r28, r11, -0x57f8
	ctx.r[28].s64 = ctx.r[11].s64 + -22520;
	// 83204288: 409A0020  bne cr6, 0x832042a8
	if !ctx.cr[6].eq {
	pc = 0x832042A8; continue 'dispatch;
	}
	// 8320428C: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 83204290: 917F0084  stw r11, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[11].u32 ) };
	// 83204294: 93FE0070  stw r31, 0x70(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(112 as u32), ctx.r[31].u32 ) };
	// 83204298: 807E0060  lwz r3, 0x60(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(96 as u32) ) } as u64;
	// 8320429C: 80830004  lwz r4, 4(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 832042A0: 4BFFF101  bl 0x832033a0
	ctx.lr = 0x832042A4;
	sub_832033A0(ctx, base);
	// 832042A4: 93E30000  stw r31, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 832042A8: 817D0020  lwz r11, 0x20(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(32 as u32) ) } as u64;
	// 832042AC: 2F0B0008  cmpwi cr6, r11, 8
	ctx.cr[6].compare_i32(ctx.r[11].s32, 8, &mut ctx.xer);
	// 832042B0: 409A0018  bne cr6, 0x832042c8
	if !ctx.cr[6].eq {
	pc = 0x832042C8; continue 'dispatch;
	}
	// 832042B4: 93FE006C  stw r31, 0x6c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(108 as u32), ctx.r[31].u32 ) };
	// 832042B8: 807E0060  lwz r3, 0x60(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(96 as u32) ) } as u64;
	// 832042BC: 80830004  lwz r4, 4(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 832042C0: 4BFFF0E1  bl 0x832033a0
	ctx.lr = 0x832042C4;
	sub_832033A0(ctx, base);
	// 832042C4: 93E30000  stw r31, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 832042C8: 7F05C378  mr r5, r24
	ctx.r[5].u64 = ctx.r[24].u64;
	// 832042CC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 832042D0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832042D4: 48001895  bl 0x83205b68
	ctx.lr = 0x832042D8;
	sub_83205B68(ctx, base);
	// 832042D8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 832042DC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 832042E0: 4BFE2079  bl 0x831e6358
	ctx.lr = 0x832042E4;
	sub_831E6358(ctx, base);
	// 832042E4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 832042E8: 807E00A8  lwz r3, 0xa8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(168 as u32) ) } as u64;
	// 832042EC: 480044B5  bl 0x832087a0
	ctx.lr = 0x832042F0;
	sub_832087A0(ctx, base);
	// 832042F0: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 832042F4: 3D408208  lis r10, -0x7df8
	ctx.r[10].s64 = -2113404928;
	// 832042F8: 93610054  stw r27, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[27].u32 ) };
	// 832042FC: 3D208209  lis r9, -0x7df7
	ctx.r[9].s64 = -2113339392;
	// 83204300: 3D008209  lis r8, -0x7df7
	ctx.r[8].s64 = -2113339392;
	// 83204304: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 83204308: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8320430C: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 83204310: 3B8AB734  addi r28, r10, -0x48cc
	ctx.r[28].s64 = ctx.r[10].s64 + -18636;
	// 83204314: 3BABA884  addi r29, r11, -0x577c
	ctx.r[29].s64 = ctx.r[11].s64 + -22396;
	// 83204318: 3B69A864  addi r27, r9, -0x579c
	ctx.r[27].s64 = ctx.r[9].s64 + -22428;
	// 8320431C: 3B48A810  addi r26, r8, -0x57f0
	ctx.r[26].s64 = ctx.r[8].s64 + -22512;
	// 83204320: 39610054  addi r11, r1, 0x54
	ctx.r[11].s64 = ctx.r[1].s64 + 84;
	// 83204324: 7F3E58AE  lbzx r25, r30, r11
	ctx.r[25].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 83204328: 2B190008  cmplwi cr6, r25, 8
	ctx.cr[6].compare_u32(ctx.r[25].u32, 8 as u32, &mut ctx.xer);
	// 8320432C: 4198001C  blt cr6, 0x83204348
	if ctx.cr[6].lt {
	pc = 0x83204348; continue 'dispatch;
	}
	// 83204330: 7F46D378  mr r6, r26
	ctx.r[6].u64 = ctx.r[26].u64;
	// 83204334: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 83204338: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8320433C: 38E00169  li r7, 0x169
	ctx.r[7].s64 = 361;
	// 83204340: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83204344: 4BF05A6D  bl 0x83109db0
	ctx.lr = 0x83204348;
	sub_83109DB0(ctx, base);
	// 83204348: 572B103A  slwi r11, r25, 2
	ctx.r[11].u32 = ctx.r[25].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8320434C: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 83204350: 7D6BE82E  lwzx r11, r11, r29
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 83204354: 7D7E51AE  stbx r11, r30, r10
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[30].u32.wrapping_add(ctx.r[10].u32), ctx.r[11].u8) };
	// 83204358: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 8320435C: 2F1E0004  cmpwi cr6, r30, 4
	ctx.cr[6].compare_i32(ctx.r[30].s32, 4, &mut ctx.xer);
	// 83204360: 4198FFC0  blt cr6, 0x83204320
	if ctx.cr[6].lt {
	pc = 0x83204320; continue 'dispatch;
	}
	// 83204364: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83204368: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 8320436C: 917F0080  stw r11, 0x80(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 83204370: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 83204374: 4BAA50D4  b 0x82ca9448
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83204378(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83204378 size=364
    let mut pc: u32 = 0x83204378;
    'dispatch: loop {
        match pc {
            0x83204378 => {
    //   block [0x83204378..0x832044E4)
	// 83204378: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8320437C: 4BAA507D  bl 0x82ca93f8
	ctx.lr = 0x83204380;
	sub_82CA93D0(ctx, base);
	// 83204380: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83204384: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83204388: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 8320438C: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 83204390: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 83204394: 817F003C  lwz r11, 0x3c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) } as u64;
	// 83204398: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8320439C: 409A0020  bne cr6, 0x832043bc
	if !ctx.cr[6].eq {
	pc = 0x832043BC; continue 'dispatch;
	}
	// 832043A0: 817D05E4  lwz r11, 0x5e4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(1508 as u32) ) } as u64;
	// 832043A4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 832043A8: 38ABFFFF  addi r5, r11, -1
	ctx.r[5].s64 = ctx.r[11].s64 + -1;
	// 832043AC: 90BD05E4  stw r5, 0x5e4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(1508 as u32), ctx.r[5].u32 ) };
	// 832043B0: 807E00AC  lwz r3, 0xac(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(172 as u32) ) } as u64;
	// 832043B4: 4BFF08A5  bl 0x831f4c58
	ctx.lr = 0x832043B8;
	sub_831F4C58(ctx, base);
	// 832043B8: 907F003C  stw r3, 0x3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[3].u32 ) };
	// 832043BC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 832043C0: 4BFE1669  bl 0x831e5a28
	ctx.lr = 0x832043C4;
	sub_831E5A28(ctx, base);
	// 832043C4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 832043C8: 41820110  beq 0x832044d8
	if ctx.cr[0].eq {
	pc = 0x832044D8; continue 'dispatch;
	}
	// 832043CC: 83BD05AC  lwz r29, 0x5ac(r29)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(1452 as u32) ) } as u64;
	// 832043D0: 388003C4  li r4, 0x3c4
	ctx.r[4].s64 = 964;
	// 832043D4: 831F003C  lwz r24, 0x3c(r31)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) } as u64;
	// 832043D8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 832043DC: 4BFC7DBD  bl 0x831cc198
	ctx.lr = 0x832043E0;
	sub_831CC198(ctx, base);
	// 832043E0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 832043E4: 346B0004  addic. r3, r11, 4
	ctx.xer.ca = (ctx.r[11].u32 > (!(4 as u32)));
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 832043E8: 93AB0000  stw r29, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 832043EC: 41820010  beq 0x832043fc
	if ctx.cr[0].eq {
	pc = 0x832043FC; continue 'dispatch;
	}
	// 832043F0: 809E000C  lwz r4, 0xc(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 832043F4: 4800255D  bl 0x83206950
	ctx.lr = 0x832043F8;
	sub_83206950(ctx, base);
	// 832043F8: 48000008  b 0x83204400
	pc = 0x83204400; continue 'dispatch;
	// 832043FC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83204400: 907F0038  stw r3, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[3].u32 ) };
	// 83204404: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 83204408: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8320440C: 4800175D  bl 0x83205b68
	ctx.lr = 0x83204410;
	sub_83205B68(ctx, base);
	// 83204410: 7F05C378  mr r5, r24
	ctx.r[5].u64 = ctx.r[24].u64;
	// 83204414: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83204418: 807F0038  lwz r3, 0x38(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 8320441C: 4800174D  bl 0x83205b68
	ctx.lr = 0x83204420;
	sub_83205B68(ctx, base);
	// 83204420: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83204424: 809F0038  lwz r4, 0x38(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 83204428: 4BFE1F31  bl 0x831e6358
	ctx.lr = 0x8320442C;
	sub_831E6358(ctx, base);
	// 8320442C: 809F0038  lwz r4, 0x38(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 83204430: 807E00A8  lwz r3, 0xa8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(168 as u32) ) } as u64;
	// 83204434: 48004325  bl 0x83208758
	ctx.lr = 0x83204438;
	sub_83208758(ctx, base);
	// 83204438: 807E0060  lwz r3, 0x60(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(96 as u32) ) } as u64;
	// 8320443C: 80830004  lwz r4, 4(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 83204440: 83DF0038  lwz r30, 0x38(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 83204444: 4BFFEF5D  bl 0x832033a0
	ctx.lr = 0x83204448;
	sub_832033A0(ctx, base);
	// 83204448: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 8320444C: 93C30000  stw r30, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 83204450: 3D408208  lis r10, -0x7df8
	ctx.r[10].s64 = -2113404928;
	// 83204454: 93810054  stw r28, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[28].u32 ) };
	// 83204458: 3D208209  lis r9, -0x7df7
	ctx.r[9].s64 = -2113339392;
	// 8320445C: 3D008209  lis r8, -0x7df7
	ctx.r[8].s64 = -2113339392;
	// 83204460: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 83204464: 816BA808  lwz r11, -0x57f8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-22520 as u32) ) } as u64;
	// 83204468: 3B8AB734  addi r28, r10, -0x48cc
	ctx.r[28].s64 = ctx.r[10].s64 + -18636;
	// 8320446C: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 83204470: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 83204474: 3BABA884  addi r29, r11, -0x577c
	ctx.r[29].s64 = ctx.r[11].s64 + -22396;
	// 83204478: 3B69A864  addi r27, r9, -0x579c
	ctx.r[27].s64 = ctx.r[9].s64 + -22428;
	// 8320447C: 3B48A810  addi r26, r8, -0x57f0
	ctx.r[26].s64 = ctx.r[8].s64 + -22512;
	// 83204480: 39610054  addi r11, r1, 0x54
	ctx.r[11].s64 = ctx.r[1].s64 + 84;
	// 83204484: 7F3E58AE  lbzx r25, r30, r11
	ctx.r[25].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 83204488: 2B190008  cmplwi cr6, r25, 8
	ctx.cr[6].compare_u32(ctx.r[25].u32, 8 as u32, &mut ctx.xer);
	// 8320448C: 4198001C  blt cr6, 0x832044a8
	if ctx.cr[6].lt {
	pc = 0x832044A8; continue 'dispatch;
	}
	// 83204490: 7F46D378  mr r6, r26
	ctx.r[6].u64 = ctx.r[26].u64;
	// 83204494: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 83204498: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8320449C: 38E00169  li r7, 0x169
	ctx.r[7].s64 = 361;
	// 832044A0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 832044A4: 4BF0590D  bl 0x83109db0
	ctx.lr = 0x832044A8;
	sub_83109DB0(ctx, base);
	// 832044A8: 572B103A  slwi r11, r25, 2
	ctx.r[11].u32 = ctx.r[25].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 832044AC: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 832044B0: 7D6BE82E  lwzx r11, r11, r29
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 832044B4: 7D7E51AE  stbx r11, r30, r10
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[30].u32.wrapping_add(ctx.r[10].u32), ctx.r[11].u8) };
	// 832044B8: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 832044BC: 2F1E0004  cmpwi cr6, r30, 4
	ctx.cr[6].compare_i32(ctx.r[30].s32, 4, &mut ctx.xer);
	// 832044C0: 4198FFC0  blt cr6, 0x83204480
	if ctx.cr[6].lt {
	pc = 0x83204480; continue 'dispatch;
	}
	// 832044C4: 817F0038  lwz r11, 0x38(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 832044C8: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 832044CC: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 832044D0: 914B0080  stw r10, 0x80(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(128 as u32), ctx.r[10].u32 ) };
	// 832044D4: 48000008  b 0x832044dc
	pc = 0x832044DC; continue 'dispatch;
	// 832044D8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832044DC: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 832044E0: 4BAA4F68  b 0x82ca9448
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832044E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832044E8 size=80
    let mut pc: u32 = 0x832044E8;
    'dispatch: loop {
        match pc {
            0x832044E8 => {
    //   block [0x832044E8..0x83204538)
	// 832044E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832044EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832044F0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832044F4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832044F8: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 832044FC: 83E3002C  lwz r31, 0x2c(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) } as u64;
	// 83204500: 396BA998  addi r11, r11, -0x5668
	ctx.r[11].s64 = ctx.r[11].s64 + -22120;
	// 83204504: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 83204508: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8320450C: 419A0018  beq cr6, 0x83204524
	if ctx.cr[6].eq {
	pc = 0x83204524; continue 'dispatch;
	}
	// 83204510: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83204514: 4BFFF675  bl 0x83203b88
	ctx.lr = 0x83204518;
	sub_83203B88(ctx, base);
	// 83204518: 389FFFFC  addi r4, r31, -4
	ctx.r[4].s64 = ctx.r[31].s64 + -4;
	// 8320451C: 807FFFFC  lwz r3, -4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-4 as u32) ) } as u64;
	// 83204520: 4BFC7DA9  bl 0x831cc2c8
	ctx.lr = 0x83204524;
	sub_831CC2C8(ctx, base);
	// 83204524: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83204528: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8320452C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83204530: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83204534: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83204538(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83204538 size=316
    let mut pc: u32 = 0x83204538;
    'dispatch: loop {
        match pc {
            0x83204538 => {
    //   block [0x83204538..0x83204674)
	// 83204538: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8320453C: 4BAA4ECD  bl 0x82ca9408
	ctx.lr = 0x83204540;
	sub_82CA93D0(ctx, base);
	// 83204540: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83204544: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83204548: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8320454C: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 83204550: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 83204554: 4BFFF68D  bl 0x83203be0
	ctx.lr = 0x83204558;
	sub_83203BE0(ctx, base);
	// 83204558: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 8320455C: 93BE0010  stw r29, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[29].u32 ) };
	// 83204560: 2F1C0001  cmpwi cr6, r28, 1
	ctx.cr[6].compare_i32(ctx.r[28].s32, 1, &mut ctx.xer);
	// 83204564: 396BA8A4  addi r11, r11, -0x575c
	ctx.r[11].s64 = ctx.r[11].s64 + -22364;
	// 83204568: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8320456C: 817F0588  lwz r11, 0x588(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1416 as u32) ) } as u64;
	// 83204570: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83204574: 917F0588  stw r11, 0x588(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1416 as u32), ctx.r[11].u32 ) };
	// 83204578: 419A005C  beq cr6, 0x832045d4
	if ctx.cr[6].eq {
	pc = 0x832045D4; continue 'dispatch;
	}
	// 8320457C: 2F1C0002  cmpwi cr6, r28, 2
	ctx.cr[6].compare_i32(ctx.r[28].s32, 2, &mut ctx.xer);
	// 83204580: 419A0030  beq cr6, 0x832045b0
	if ctx.cr[6].eq {
	pc = 0x832045B0; continue 'dispatch;
	}
	// 83204584: 2F1C0003  cmpwi cr6, r28, 3
	ctx.cr[6].compare_i32(ctx.r[28].s32, 3, &mut ctx.xer);
	// 83204588: 409A0070  bne cr6, 0x832045f8
	if !ctx.cr[6].eq {
	pc = 0x832045F8; continue 'dispatch;
	}
	// 8320458C: 817F0550  lwz r11, 0x550(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1360 as u32) ) } as u64;
	// 83204590: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 83204594: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 83204598: 41980060  blt cr6, 0x832045f8
	if ctx.cr[6].lt {
	pc = 0x832045F8; continue 'dispatch;
	}
	// 8320459C: 3960001E  li r11, 0x1e
	ctx.r[11].s64 = 30;
	// 832045A0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 832045A4: 917F0554  stw r11, 0x554(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1364 as u32), ctx.r[11].u32 ) };
	// 832045A8: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 832045AC: 4BAAB485  bl 0x82cafa30
	ctx.lr = 0x832045B0;
	sub_82CAFA30(ctx, base);
	// 832045B0: 817F0550  lwz r11, 0x550(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1360 as u32) ) } as u64;
	// 832045B4: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 832045B8: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 832045BC: 4198003C  blt cr6, 0x832045f8
	if ctx.cr[6].lt {
	pc = 0x832045F8; continue 'dispatch;
	}
	// 832045C0: 3960001C  li r11, 0x1c
	ctx.r[11].s64 = 28;
	// 832045C4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 832045C8: 917F0554  stw r11, 0x554(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1364 as u32), ctx.r[11].u32 ) };
	// 832045CC: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 832045D0: 4BAAB461  bl 0x82cafa30
	ctx.lr = 0x832045D4;
	sub_82CAFA30(ctx, base);
	// 832045D4: 817F0550  lwz r11, 0x550(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1360 as u32) ) } as u64;
	// 832045D8: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 832045DC: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 832045E0: 41980018  blt cr6, 0x832045f8
	if ctx.cr[6].lt {
	pc = 0x832045F8; continue 'dispatch;
	}
	// 832045E4: 3960001D  li r11, 0x1d
	ctx.r[11].s64 = 29;
	// 832045E8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 832045EC: 917F0554  stw r11, 0x554(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1364 as u32), ctx.r[11].u32 ) };
	// 832045F0: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 832045F4: 4BAAB43D  bl 0x82cafa30
	ctx.lr = 0x832045F8;
	sub_82CAFA30(ctx, base);
	// 832045F8: 83BF05AC  lwz r29, 0x5ac(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1452 as u32) ) } as u64;
	// 832045FC: 388003C4  li r4, 0x3c4
	ctx.r[4].s64 = 964;
	// 83204600: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83204604: 4BFC7B95  bl 0x831cc198
	ctx.lr = 0x83204608;
	sub_831CC198(ctx, base);
	// 83204608: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8320460C: 346B0004  addic. r3, r11, 4
	ctx.xer.ca = (ctx.r[11].u32 > (!(4 as u32)));
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83204610: 93AB0000  stw r29, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 83204614: 41820018  beq 0x8320462c
	if ctx.cr[0].eq {
	pc = 0x8320462C; continue 'dispatch;
	}
	// 83204618: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8320461C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83204620: 48002541  bl 0x83206b60
	ctx.lr = 0x83204624;
	sub_83206B60(ctx, base);
	// 83204624: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83204628: 48000008  b 0x83204630
	pc = 0x83204630; continue 'dispatch;
	// 8320462C: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 83204630: 817F0AB0  lwz r11, 0xab0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2736 as u32) ) } as u64;
	// 83204634: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83204638: 806B00A4  lwz r3, 0xa4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(164 as u32) ) } as u64;
	// 8320463C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83204640: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 83204644: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83204648: 4E800421  bctrl
	ctx.lr = 0x8320464C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8320464C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 83204650: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83204654: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83204658: 48001511  bl 0x83205b68
	ctx.lr = 0x8320465C;
	sub_83205B68(ctx, base);
	// 8320465C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83204660: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83204664: 4BFFF685  bl 0x83203ce8
	ctx.lr = 0x83204668;
	sub_83203CE8(ctx, base);
	// 83204668: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8320466C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83204670: 4BAA4DE8  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83204678(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83204678 size=96
    let mut pc: u32 = 0x83204678;
    'dispatch: loop {
        match pc {
            0x83204678 => {
    //   block [0x83204678..0x832046D8)
	// 83204678: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8320467C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83204680: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83204684: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83204688: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8320468C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83204690: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 83204694: 4BFFF54D  bl 0x83203be0
	ctx.lr = 0x83204698;
	sub_83203BE0(ctx, base);
	// 83204698: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 8320469C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832046A0: 396BA974  addi r11, r11, -0x568c
	ctx.r[11].s64 = ctx.r[11].s64 + -22156;
	// 832046A4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 832046A8: 817E0584  lwz r11, 0x584(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(1412 as u32) ) } as u64;
	// 832046AC: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 832046B0: 917F0030  stw r11, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 832046B4: 817E0584  lwz r11, 0x584(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(1412 as u32) ) } as u64;
	// 832046B8: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 832046BC: 917E0584  stw r11, 0x584(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(1412 as u32), ctx.r[11].u32 ) };
	// 832046C0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 832046C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832046C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832046CC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 832046D0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832046D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832046D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832046D8 size=68
    let mut pc: u32 = 0x832046D8;
    'dispatch: loop {
        match pc {
            0x832046D8 => {
    //   block [0x832046D8..0x8320471C)
	// 832046D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832046DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832046E0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832046E4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832046E8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 832046EC: 4BFFF4F5  bl 0x83203be0
	ctx.lr = 0x832046F0;
	sub_83203BE0(ctx, base);
	// 832046F0: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 832046F4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832046F8: 396BA8C8  addi r11, r11, -0x5738
	ctx.r[11].s64 = ctx.r[11].s64 + -22328;
	// 832046FC: 915F0034  stw r10, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[10].u32 ) };
	// 83204700: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83204704: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83204708: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8320470C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83204710: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83204714: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83204718: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83204720(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83204720 size=216
    let mut pc: u32 = 0x83204720;
    'dispatch: loop {
        match pc {
            0x83204720 => {
    //   block [0x83204720..0x832047F8)
	// 83204720: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83204724: 4BAA4CE5  bl 0x82ca9408
	ctx.lr = 0x83204728;
	sub_82CA93D0(ctx, base);
	// 83204728: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8320472C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83204730: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 83204734: 4BFFF4AD  bl 0x83203be0
	ctx.lr = 0x83204738;
	sub_83203BE0(ctx, base);
	// 83204738: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 8320473C: 80BF000C  lwz r5, 0xc(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 83204740: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 83204744: 809F0020  lwz r4, 0x20(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 83204748: 396BA8EC  addi r11, r11, -0x5714
	ctx.r[11].s64 = ctx.r[11].s64 + -22292;
	// 8320474C: 93BF0030  stw r29, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[29].u32 ) };
	// 83204750: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 83204754: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83204758: 93BF0034  stw r29, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[29].u32 ) };
	// 8320475C: 93BF0038  stw r29, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[29].u32 ) };
	// 83204760: 93BF003C  stw r29, 0x3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[29].u32 ) };
	// 83204764: 817E0AB0  lwz r11, 0xab0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(2736 as u32) ) } as u64;
	// 83204768: 386B00B8  addi r3, r11, 0xb8
	ctx.r[3].s64 = ctx.r[11].s64 + 184;
	// 8320476C: 4BFFCD65  bl 0x832014d0
	ctx.lr = 0x83204770;
	sub_832014D0(ctx, base);
	// 83204770: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83204774: 907F0010  stw r3, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[3].u32 ) };
	// 83204778: 997F0005  stb r11, 5(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(5 as u32), ctx.r[11].u8 ) };
	// 8320477C: 897E0564  lbz r11, 0x564(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(1380 as u32) ) } as u64;
	// 83204780: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83204784: 40820068  bne 0x832047ec
	if !ctx.cr[0].eq {
	pc = 0x832047EC; continue 'dispatch;
	}
	// 83204788: 839E05AC  lwz r28, 0x5ac(r30)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(1452 as u32) ) } as u64;
	// 8320478C: 388003C4  li r4, 0x3c4
	ctx.r[4].s64 = 964;
	// 83204790: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83204794: 4BFC7A05  bl 0x831cc198
	ctx.lr = 0x83204798;
	sub_831CC198(ctx, base);
	// 83204798: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8320479C: 346B0004  addic. r3, r11, 4
	ctx.xer.ca = (ctx.r[11].u32 > (!(4 as u32)));
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 832047A0: 938B0000  stw r28, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 832047A4: 41820010  beq 0x832047b4
	if ctx.cr[0].eq {
	pc = 0x832047B4; continue 'dispatch;
	}
	// 832047A8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 832047AC: 48002505  bl 0x83206cb0
	ctx.lr = 0x832047B0;
	sub_83206CB0(ctx, base);
	// 832047B0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 832047B4: 817E0AB0  lwz r11, 0xab0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(2736 as u32) ) } as u64;
	// 832047B8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 832047BC: 806B00A4  lwz r3, 0xa4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(164 as u32) ) } as u64;
	// 832047C0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 832047C4: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 832047C8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 832047CC: 4E800421  bctrl
	ctx.lr = 0x832047D0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 832047D0: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 832047D4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 832047D8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 832047DC: 4800138D  bl 0x83205b68
	ctx.lr = 0x832047E0;
	sub_83205B68(ctx, base);
	// 832047E0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 832047E4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832047E8: 4BFFF501  bl 0x83203ce8
	ctx.lr = 0x832047EC;
	sub_83203CE8(ctx, base);
	// 832047EC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832047F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832047F4: 4BAA4C64  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832047F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832047F8 size=268
    let mut pc: u32 = 0x832047F8;
    'dispatch: loop {
        match pc {
            0x832047F8 => {
    //   block [0x832047F8..0x83204904)
	// 832047F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832047FC: 4BAA4C09  bl 0x82ca9404
	ctx.lr = 0x83204800;
	sub_82CA93D0(ctx, base);
	// 83204800: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83204804: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 83204808: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 8320480C: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 83204810: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83204814: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 83204818: 4BFFF3C9  bl 0x83203be0
	ctx.lr = 0x8320481C;
	sub_83203BE0(ctx, base);
	// 8320481C: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 83204820: 80BF000C  lwz r5, 0xc(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 83204824: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 83204828: 396BA8EC  addi r11, r11, -0x5714
	ctx.r[11].s64 = ctx.r[11].s64 + -22292;
	// 8320482C: 93BF0030  stw r29, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[29].u32 ) };
	// 83204830: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 83204834: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83204838: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 8320483C: 93BF0034  stw r29, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[29].u32 ) };
	// 83204840: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 83204844: 93BF0038  stw r29, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[29].u32 ) };
	// 83204848: 93BF003C  stw r29, 0x3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[29].u32 ) };
	// 8320484C: 817E0AB0  lwz r11, 0xab0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(2736 as u32) ) } as u64;
	// 83204850: 386B00B8  addi r3, r11, 0xb8
	ctx.r[3].s64 = ctx.r[11].s64 + 184;
	// 83204854: 4BFFCD6D  bl 0x832015c0
	ctx.lr = 0x83204858;
	sub_832015C0(ctx, base);
	// 83204858: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8320485C: 907F0010  stw r3, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[3].u32 ) };
	// 83204860: 997F0005  stb r11, 5(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(5 as u32), ctx.r[11].u8 ) };
	// 83204864: 897E0564  lbz r11, 0x564(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(1380 as u32) ) } as u64;
	// 83204868: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8320486C: 41820028  beq 0x83204894
	if ctx.cr[0].eq {
	pc = 0x83204894; continue 'dispatch;
	}
	// 83204870: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 83204874: 3D408209  lis r10, -0x7df7
	ctx.r[10].s64 = -2113339392;
	// 83204878: 3D208208  lis r9, -0x7df8
	ctx.r[9].s64 = -2113404928;
	// 8320487C: 38CBA810  addi r6, r11, -0x57f0
	ctx.r[6].s64 = ctx.r[11].s64 + -22512;
	// 83204880: 38AAA910  addi r5, r10, -0x56f0
	ctx.r[5].s64 = ctx.r[10].s64 + -22256;
	// 83204884: 3889B734  addi r4, r9, -0x48cc
	ctx.r[4].s64 = ctx.r[9].s64 + -18636;
	// 83204888: 38E001F1  li r7, 0x1f1
	ctx.r[7].s64 = 497;
	// 8320488C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83204890: 4BF05521  bl 0x83109db0
	ctx.lr = 0x83204894;
	sub_83109DB0(ctx, base);
	// 83204894: 839E05AC  lwz r28, 0x5ac(r30)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(1452 as u32) ) } as u64;
	// 83204898: 388003C4  li r4, 0x3c4
	ctx.r[4].s64 = 964;
	// 8320489C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 832048A0: 4BFC78F9  bl 0x831cc198
	ctx.lr = 0x832048A4;
	sub_831CC198(ctx, base);
	// 832048A4: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 832048A8: 346B0004  addic. r3, r11, 4
	ctx.xer.ca = (ctx.r[11].u32 > (!(4 as u32)));
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 832048AC: 938B0000  stw r28, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 832048B0: 41820010  beq 0x832048c0
	if ctx.cr[0].eq {
	pc = 0x832048C0; continue 'dispatch;
	}
	// 832048B4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 832048B8: 480023F9  bl 0x83206cb0
	ctx.lr = 0x832048BC;
	sub_83206CB0(ctx, base);
	// 832048BC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 832048C0: 817E0AB0  lwz r11, 0xab0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(2736 as u32) ) } as u64;
	// 832048C4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 832048C8: 806B00A4  lwz r3, 0xa4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(164 as u32) ) } as u64;
	// 832048CC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 832048D0: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 832048D4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 832048D8: 4E800421  bctrl
	ctx.lr = 0x832048DC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 832048DC: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 832048E0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 832048E4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 832048E8: 48001281  bl 0x83205b68
	ctx.lr = 0x832048EC;
	sub_83205B68(ctx, base);
	// 832048EC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 832048F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832048F4: 4BFFF3F5  bl 0x83203ce8
	ctx.lr = 0x832048F8;
	sub_83203CE8(ctx, base);
	// 832048F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832048FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83204900: 4BAA4B54  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83204908(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83204908 size=84
    let mut pc: u32 = 0x83204908;
    'dispatch: loop {
        match pc {
            0x83204908 => {
    //   block [0x83204908..0x8320495C)
	// 83204908: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8320490C: 4BAA4B01  bl 0x82ca940c
	ctx.lr = 0x83204910;
	sub_82CA93D0(ctx, base);
	// 83204910: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83204914: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83204918: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8320491C: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 83204920: 4BFFF2C1  bl 0x83203be0
	ctx.lr = 0x83204924;
	sub_83203BE0(ctx, base);
	// 83204924: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 83204928: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8320492C: 396BA998  addi r11, r11, -0x5668
	ctx.r[11].s64 = ctx.r[11].s64 + -22120;
	// 83204930: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83204934: 807D0AB0  lwz r3, 0xab0(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(2736 as u32) ) } as u64;
	// 83204938: 4BFF2961  bl 0x831f7298
	ctx.lr = 0x8320493C;
	sub_831F7298(ctx, base);
	// 8320493C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83204940: 997F0005  stb r11, 5(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(5 as u32), ctx.r[11].u8 ) };
	// 83204944: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83204948: 93DF0010  stw r30, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[30].u32 ) };
	// 8320494C: 997F001D  stb r11, 0x1d(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(29 as u32), ctx.r[11].u8 ) };
	// 83204950: 997F001C  stb r11, 0x1c(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u8 ) };
	// 83204954: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83204958: 4BAA4B04  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83204960(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83204960 size=164
    let mut pc: u32 = 0x83204960;
    'dispatch: loop {
        match pc {
            0x83204960 => {
    //   block [0x83204960..0x83204A04)
	// 83204960: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83204964: 4BAA4AA1  bl 0x82ca9404
	ctx.lr = 0x83204968;
	sub_82CA93D0(ctx, base);
	// 83204968: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8320496C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83204970: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 83204974: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 83204978: 4BFFF269  bl 0x83203be0
	ctx.lr = 0x8320497C;
	sub_83203BE0(ctx, base);
	// 8320497C: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 83204980: 3D400000  lis r10, 0
	ctx.r[10].s64 = 0;
	// 83204984: 392BA950  addi r9, r11, -0x56b0
	ctx.r[9].s64 = ctx.r[11].s64 + -22192;
	// 83204988: 61488000  ori r8, r10, 0x8000
	ctx.r[8].u64 = ctx.r[10].u64 | 32768;
	// 8320498C: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 83204990: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 83204994: 3D408209  lis r10, -0x7df7
	ctx.r[10].s64 = -2113339392;
	// 83204998: 7F1E4000  cmpw cr6, r30, r8
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[8].s32, &mut ctx.xer);
	// 8320499C: 3B8BB734  addi r28, r11, -0x48cc
	ctx.r[28].s64 = ctx.r[11].s64 + -18636;
	// 832049A0: 3B6AA810  addi r27, r10, -0x57f0
	ctx.r[27].s64 = ctx.r[10].s64 + -22512;
	// 832049A4: 41980020  blt cr6, 0x832049c4
	if ctx.cr[6].lt {
	pc = 0x832049C4; continue 'dispatch;
	}
	// 832049A8: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 832049AC: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 832049B0: 38ABA940  addi r5, r11, -0x56c0
	ctx.r[5].s64 = ctx.r[11].s64 + -22208;
	// 832049B4: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 832049B8: 38E00241  li r7, 0x241
	ctx.r[7].s64 = 577;
	// 832049BC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 832049C0: 4BF053F1  bl 0x83109db0
	ctx.lr = 0x832049C4;
	sub_83109DB0(ctx, base);
	// 832049C4: 3D600001  lis r11, 1
	ctx.r[11].s64 = 65536;
	// 832049C8: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 832049CC: 41980020  blt cr6, 0x832049ec
	if ctx.cr[6].lt {
	pc = 0x832049EC; continue 'dispatch;
	}
	// 832049D0: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 832049D4: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 832049D8: 38ABA92C  addi r5, r11, -0x56d4
	ctx.r[5].s64 = ctx.r[11].s64 + -22228;
	// 832049DC: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 832049E0: 38E00242  li r7, 0x242
	ctx.r[7].s64 = 578;
	// 832049E4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 832049E8: 4BF053C9  bl 0x83109db0
	ctx.lr = 0x832049EC;
	sub_83109DB0(ctx, base);
	// 832049EC: 57CB801E  slwi r11, r30, 0x10
	ctx.r[11].u32 = ctx.r[30].u32.wrapping_shl(16);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 832049F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832049F4: 7D6BEA14  add r11, r11, r29
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 832049F8: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 832049FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83204A00: 4BAA4A54  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83204A08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83204A08 size=1104
    let mut pc: u32 = 0x83204A08;
    'dispatch: loop {
        match pc {
            0x83204A08 => {
    //   block [0x83204A08..0x83204E58)
	// 83204A08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83204A0C: 4BAA49E1  bl 0x82ca93ec
	ctx.lr = 0x83204A10;
	sub_82CA93D0(ctx, base);
	// 83204A10: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83204A14: 7C761B78  mr r22, r3
	ctx.r[22].u64 = ctx.r[3].u64;
	// 83204A18: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 83204A1C: 4BFFFC5D  bl 0x83204678
	ctx.lr = 0x83204A20;
	sub_83204678(ctx, base);
	// 83204A20: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 83204A24: 3D40831D  lis r10, -0x7ce3
	ctx.r[10].s64 = -2095251456;
	// 83204A28: 396BA974  addi r11, r11, -0x568c
	ctx.r[11].s64 = ctx.r[11].s64 + -22156;
	// 83204A2C: 3BAACC70  addi r29, r10, -0x3390
	ctx.r[29].s64 = ctx.r[10].s64 + -13200;
	// 83204A30: 91760000  stw r11, 0(r22)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[22].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83204A34: 807E05D0  lwz r3, 0x5d0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(1488 as u32) ) } as u64;
	// 83204A38: 83FE0AB0  lwz r31, 0xab0(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(2736 as u32) ) } as u64;
	// 83204A3C: 839E0600  lwz r28, 0x600(r30)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(1536 as u32) ) } as u64;
	// 83204A40: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83204A44: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 83204A48: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83204A4C: 4E800421  bctrl
	ctx.lr = 0x83204A50;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83204A50: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 83204A54: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83204A58: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83204A5C: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 83204A60: 7FC7F378  mr r7, r30
	ctx.r[7].u64 = ctx.r[30].u64;
	// 83204A64: 4BFDD38D  bl 0x831e1df0
	ctx.lr = 0x83204A68;
	sub_831E1DF0(ctx, base);
	// 83204A68: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 83204A6C: 408200BC  bne 0x83204b28
	if !ctx.cr[0].eq {
	pc = 0x83204B28; continue 'dispatch;
	}
	// 83204A70: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 83204A74: 80ABAA60  lwz r5, -0x55a0(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-21920 as u32) ) } as u64;
	// 83204A78: 39650006  addi r11, r5, 6
	ctx.r[11].s64 = ctx.r[5].s64 + 6;
	// 83204A7C: 557B103A  slwi r27, r11, 2
	ctx.r[27].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[27].u64 = ctx.r[27].u32 as u64;
	// 83204A80: 7D7BF82E  lwzx r11, r27, r31
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[27].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 83204A84: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83204A88: 409A007C  bne cr6, 0x83204b04
	if !ctx.cr[6].eq {
	pc = 0x83204B04; continue 'dispatch;
	}
	// 83204A8C: 3880000F  li r4, 0xf
	ctx.r[4].s64 = 15;
	// 83204A90: 807F00AC  lwz r3, 0xac(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(172 as u32) ) } as u64;
	// 83204A94: 4BFF0A15  bl 0x831f54a8
	ctx.lr = 0x83204A98;
	sub_831F54A8(ctx, base);
	// 83204A98: 83BE05AC  lwz r29, 0x5ac(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(1452 as u32) ) } as u64;
	// 83204A9C: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 83204AA0: 388003C4  li r4, 0x3c4
	ctx.r[4].s64 = 964;
	// 83204AA4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83204AA8: 4BFC76F1  bl 0x831cc198
	ctx.lr = 0x83204AAC;
	sub_831CC198(ctx, base);
	// 83204AAC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83204AB0: 346B0004  addic. r3, r11, 4
	ctx.xer.ca = (ctx.r[11].u32 > (!(4 as u32)));
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83204AB4: 93AB0000  stw r29, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 83204AB8: 41820018  beq 0x83204ad0
	if ctx.cr[0].eq {
	pc = 0x83204AD0; continue 'dispatch;
	}
	// 83204ABC: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 83204AC0: 7EC4B378  mr r4, r22
	ctx.r[4].u64 = ctx.r[22].u64;
	// 83204AC4: 4800217D  bl 0x83206c40
	ctx.lr = 0x83204AC8;
	sub_83206C40(ctx, base);
	// 83204AC8: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83204ACC: 48000008  b 0x83204ad4
	pc = 0x83204AD4; continue 'dispatch;
	// 83204AD0: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 83204AD4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83204AD8: 807F00A4  lwz r3, 0xa4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 83204ADC: 48003CC5  bl 0x832087a0
	ctx.lr = 0x83204AE0;
	sub_832087A0(ctx, base);
	// 83204AE0: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 83204AE4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83204AE8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83204AEC: 4800107D  bl 0x83205b68
	ctx.lr = 0x83204AF0;
	sub_83205B68(ctx, base);
	// 83204AF0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83204AF4: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83204AF8: 4BFFF1F1  bl 0x83203ce8
	ctx.lr = 0x83204AFC;
	sub_83203CE8(ctx, base);
	// 83204AFC: 7FBBF92E  stwx r29, r27, r31
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[27].u32.wrapping_add(ctx.r[31].u32), ctx.r[29].u32) };
	// 83204B00: 48000008  b 0x83204b08
	pc = 0x83204B08; continue 'dispatch;
	// 83204B04: 838B001C  lwz r28, 0x1c(r11)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 83204B08: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83204B0C: 38600031  li r3, 0x31
	ctx.r[3].s64 = 49;
	// 83204B10: 48003101  bl 0x83207c10
	ctx.lr = 0x83204B14;
	sub_83207C10(ctx, base);
	// 83204B14: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 83204B18: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83204B1C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83204B20: 48001049  bl 0x83205b68
	ctx.lr = 0x83204B24;
	sub_83205B68(ctx, base);
	// 83204B24: 48000300  b 0x83204e24
	pc = 0x83204E24; continue 'dispatch;
	// 83204B28: 3D60831D  lis r11, -0x7ce3
	ctx.r[11].s64 = -2095251456;
	// 83204B2C: 807E0600  lwz r3, 0x600(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(1536 as u32) ) } as u64;
	// 83204B30: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 83204B34: 38ABCC70  addi r5, r11, -0x3390
	ctx.r[5].s64 = ctx.r[11].s64 + -13200;
	// 83204B38: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 83204B3C: 4BFDD84D  bl 0x831e2388
	ctx.lr = 0x83204B40;
	sub_831E2388(ctx, base);
	// 83204B40: 81210050  lwz r9, 0x50(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83204B44: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 83204B48: 552A2036  slwi r10, r9, 4
	ctx.r[10].u32 = ctx.r[9].u32.wrapping_shl(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83204B4C: 3AABAB08  addi r21, r11, -0x54f8
	ctx.r[21].s64 = ctx.r[11].s64 + -21752;
	// 83204B50: 7D4AA82E  lwzx r10, r10, r21
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[21].u32)) } as u64;
	// 83204B54: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 83204B58: 419801EC  blt cr6, 0x83204d44
	if ctx.cr[6].lt {
	pc = 0x83204D44; continue 'dispatch;
	}
	// 83204B5C: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 83204B60: 554A2036  slwi r10, r10, 4
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83204B64: 3B6BAC38  addi r27, r11, -0x53c8
	ctx.r[27].s64 = ctx.r[11].s64 + -21448;
	// 83204B68: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 83204B6C: 393B0008  addi r9, r27, 8
	ctx.r[9].s64 = ctx.r[27].s64 + 8;
	// 83204B70: 396BAA60  addi r11, r11, -0x55a0
	ctx.r[11].s64 = ctx.r[11].s64 + -21920;
	// 83204B74: 7D0AD82E  lwzx r8, r10, r27
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[27].u32)) } as u64;
	// 83204B78: 7D4A482E  lwzx r10, r10, r9
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 83204B7C: 55091838  slwi r9, r8, 3
	ctx.r[9].u32 = ctx.r[8].u32.wrapping_shl(3);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 83204B80: 554A1838  slwi r10, r10, 3
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83204B84: 7CA9582E  lwzx r5, r9, r11
	ctx.r[5].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 83204B88: 7F4A582E  lwzx r26, r10, r11
	ctx.r[26].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 83204B8C: 39650006  addi r11, r5, 6
	ctx.r[11].s64 = ctx.r[5].s64 + 6;
	// 83204B90: 557C103A  slwi r28, r11, 2
	ctx.r[28].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[28].u64 = ctx.r[28].u32 as u64;
	// 83204B94: 397A0006  addi r11, r26, 6
	ctx.r[11].s64 = ctx.r[26].s64 + 6;
	// 83204B98: 5577103A  slwi r23, r11, 2
	ctx.r[23].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[23].u64 = ctx.r[23].u32 as u64;
	// 83204B9C: 7D7CF82E  lwzx r11, r28, r31
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[28].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 83204BA0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83204BA4: 7F17F82E  lwzx r24, r23, r31
	ctx.r[24].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[23].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 83204BA8: 409A007C  bne cr6, 0x83204c24
	if !ctx.cr[6].eq {
	pc = 0x83204C24; continue 'dispatch;
	}
	// 83204BAC: 3880000F  li r4, 0xf
	ctx.r[4].s64 = 15;
	// 83204BB0: 807F00AC  lwz r3, 0xac(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(172 as u32) ) } as u64;
	// 83204BB4: 4BFF08F5  bl 0x831f54a8
	ctx.lr = 0x83204BB8;
	sub_831F54A8(ctx, base);
	// 83204BB8: 83BE05AC  lwz r29, 0x5ac(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(1452 as u32) ) } as u64;
	// 83204BBC: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 83204BC0: 388003C4  li r4, 0x3c4
	ctx.r[4].s64 = 964;
	// 83204BC4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83204BC8: 4BFC75D1  bl 0x831cc198
	ctx.lr = 0x83204BCC;
	sub_831CC198(ctx, base);
	// 83204BCC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83204BD0: 346B0004  addic. r3, r11, 4
	ctx.xer.ca = (ctx.r[11].u32 > (!(4 as u32)));
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83204BD4: 93AB0000  stw r29, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 83204BD8: 41820018  beq 0x83204bf0
	if ctx.cr[0].eq {
	pc = 0x83204BF0; continue 'dispatch;
	}
	// 83204BDC: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 83204BE0: 7EC4B378  mr r4, r22
	ctx.r[4].u64 = ctx.r[22].u64;
	// 83204BE4: 4800205D  bl 0x83206c40
	ctx.lr = 0x83204BE8;
	sub_83206C40(ctx, base);
	// 83204BE8: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83204BEC: 48000008  b 0x83204bf4
	pc = 0x83204BF4; continue 'dispatch;
	// 83204BF0: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 83204BF4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83204BF8: 807F00A4  lwz r3, 0xa4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 83204BFC: 48003BA5  bl 0x832087a0
	ctx.lr = 0x83204C00;
	sub_832087A0(ctx, base);
	// 83204C00: 7F25CB78  mr r5, r25
	ctx.r[5].u64 = ctx.r[25].u64;
	// 83204C04: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83204C08: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83204C0C: 48000F5D  bl 0x83205b68
	ctx.lr = 0x83204C10;
	sub_83205B68(ctx, base);
	// 83204C10: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83204C14: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 83204C18: 4BFFF0D1  bl 0x83203ce8
	ctx.lr = 0x83204C1C;
	sub_83203CE8(ctx, base);
	// 83204C1C: 7FBCF92E  stwx r29, r28, r31
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[28].u32.wrapping_add(ctx.r[31].u32), ctx.r[29].u32) };
	// 83204C20: 48000008  b 0x83204c28
	pc = 0x83204C28; continue 'dispatch;
	// 83204C24: 832B001C  lwz r25, 0x1c(r11)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 83204C28: 2B180000  cmplwi cr6, r24, 0
	ctx.cr[6].compare_u32(ctx.r[24].u32, 0 as u32, &mut ctx.xer);
	// 83204C2C: 409A0080  bne cr6, 0x83204cac
	if !ctx.cr[6].eq {
	pc = 0x83204CAC; continue 'dispatch;
	}
	// 83204C30: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 83204C34: 807F00AC  lwz r3, 0xac(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(172 as u32) ) } as u64;
	// 83204C38: 3880000F  li r4, 0xf
	ctx.r[4].s64 = 15;
	// 83204C3C: 4BFF086D  bl 0x831f54a8
	ctx.lr = 0x83204C40;
	sub_831F54A8(ctx, base);
	// 83204C40: 83BE05AC  lwz r29, 0x5ac(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(1452 as u32) ) } as u64;
	// 83204C44: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 83204C48: 388003C4  li r4, 0x3c4
	ctx.r[4].s64 = 964;
	// 83204C4C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83204C50: 4BFC7549  bl 0x831cc198
	ctx.lr = 0x83204C54;
	sub_831CC198(ctx, base);
	// 83204C54: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83204C58: 346B0004  addic. r3, r11, 4
	ctx.xer.ca = (ctx.r[11].u32 > (!(4 as u32)));
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83204C5C: 93AB0000  stw r29, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 83204C60: 41820018  beq 0x83204c78
	if ctx.cr[0].eq {
	pc = 0x83204C78; continue 'dispatch;
	}
	// 83204C64: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 83204C68: 7EC4B378  mr r4, r22
	ctx.r[4].u64 = ctx.r[22].u64;
	// 83204C6C: 48001FD5  bl 0x83206c40
	ctx.lr = 0x83204C70;
	sub_83206C40(ctx, base);
	// 83204C70: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83204C74: 48000008  b 0x83204c7c
	pc = 0x83204C7C; continue 'dispatch;
	// 83204C78: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 83204C7C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83204C80: 807F00A4  lwz r3, 0xa4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 83204C84: 48003B1D  bl 0x832087a0
	ctx.lr = 0x83204C88;
	sub_832087A0(ctx, base);
	// 83204C88: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 83204C8C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83204C90: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83204C94: 48000ED5  bl 0x83205b68
	ctx.lr = 0x83204C98;
	sub_83205B68(ctx, base);
	// 83204C98: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83204C9C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83204CA0: 4BFFF049  bl 0x83203ce8
	ctx.lr = 0x83204CA4;
	sub_83203CE8(ctx, base);
	// 83204CA4: 7FB7F92E  stwx r29, r23, r31
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[23].u32.wrapping_add(ctx.r[31].u32), ctx.r[29].u32) };
	// 83204CA8: 48000008  b 0x83204cb0
	pc = 0x83204CB0; continue 'dispatch;
	// 83204CAC: 8398001C  lwz r28, 0x1c(r24)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(28 as u32) ) } as u64;
	// 83204CB0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83204CB4: 38600031  li r3, 0x31
	ctx.r[3].s64 = 49;
	// 83204CB8: 48002F59  bl 0x83207c10
	ctx.lr = 0x83204CBC;
	sub_83207C10(ctx, base);
	// 83204CBC: 7F25CB78  mr r5, r25
	ctx.r[5].u64 = ctx.r[25].u64;
	// 83204CC0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83204CC4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83204CC8: 48000EA1  bl 0x83205b68
	ctx.lr = 0x83204CCC;
	sub_83205B68(ctx, base);
	// 83204CCC: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83204CD0: 397B0004  addi r11, r27, 4
	ctx.r[11].s64 = ctx.r[27].s64 + 4;
	// 83204CD4: 554A2036  slwi r10, r10, 4
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83204CD8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83204CDC: 7EC5B378  mr r5, r22
	ctx.r[5].u64 = ctx.r[22].u64;
	// 83204CE0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83204CE4: 7D4AA82E  lwzx r10, r10, r21
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[21].u32)) } as u64;
	// 83204CE8: 554A2036  slwi r10, r10, 4
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83204CEC: 7D6A582E  lwzx r11, r10, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 83204CF0: 917D0084  stw r11, 0x84(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(132 as u32), ctx.r[11].u32 ) };
	// 83204CF4: 48000E75  bl 0x83205b68
	ctx.lr = 0x83204CF8;
	sub_83205B68(ctx, base);
	// 83204CF8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83204CFC: 7EC3B378  mr r3, r22
	ctx.r[3].u64 = ctx.r[22].u64;
	// 83204D00: 4BFFEFE9  bl 0x83203ce8
	ctx.lr = 0x83204D04;
	sub_83203CE8(ctx, base);
	// 83204D04: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83204D08: 807F00A4  lwz r3, 0xa4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 83204D0C: 48003A4D  bl 0x83208758
	ctx.lr = 0x83204D10;
	sub_83208758(ctx, base);
	// 83204D10: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83204D14: 38600031  li r3, 0x31
	ctx.r[3].s64 = 49;
	// 83204D18: 48002EF9  bl 0x83207c10
	ctx.lr = 0x83204D1C;
	sub_83207C10(ctx, base);
	// 83204D1C: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 83204D20: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83204D24: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83204D28: 48000E41  bl 0x83205b68
	ctx.lr = 0x83204D2C;
	sub_83205B68(ctx, base);
	// 83204D2C: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83204D30: 397B000C  addi r11, r27, 0xc
	ctx.r[11].s64 = ctx.r[27].s64 + 12;
	// 83204D34: 554A2036  slwi r10, r10, 4
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83204D38: 7D4AA82E  lwzx r10, r10, r21
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[21].u32)) } as u64;
	// 83204D3C: 554A2036  slwi r10, r10, 4
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83204D40: 480000DC  b 0x83204e1c
	pc = 0x83204E1C; continue 'dispatch;
	// 83204D44: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 83204D48: 552A2834  slwi r10, r9, 5
	ctx.r[10].u32 = ctx.r[9].u32.wrapping_shl(5);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83204D4C: 3B4BAC58  addi r26, r11, -0x53a8
	ctx.r[26].s64 = ctx.r[11].s64 + -21416;
	// 83204D50: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 83204D54: 396BAA60  addi r11, r11, -0x55a0
	ctx.r[11].s64 = ctx.r[11].s64 + -21920;
	// 83204D58: 7D4AD02E  lwzx r10, r10, r26
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[26].u32)) } as u64;
	// 83204D5C: 554A1838  slwi r10, r10, 3
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83204D60: 7CAA582E  lwzx r5, r10, r11
	ctx.r[5].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 83204D64: 39650006  addi r11, r5, 6
	ctx.r[11].s64 = ctx.r[5].s64 + 6;
	// 83204D68: 557B103A  slwi r27, r11, 2
	ctx.r[27].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[27].u64 = ctx.r[27].u32 as u64;
	// 83204D6C: 7D7BF82E  lwzx r11, r27, r31
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[27].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 83204D70: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83204D74: 409A007C  bne cr6, 0x83204df0
	if !ctx.cr[6].eq {
	pc = 0x83204DF0; continue 'dispatch;
	}
	// 83204D78: 3880000F  li r4, 0xf
	ctx.r[4].s64 = 15;
	// 83204D7C: 807F00AC  lwz r3, 0xac(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(172 as u32) ) } as u64;
	// 83204D80: 4BFF0729  bl 0x831f54a8
	ctx.lr = 0x83204D84;
	sub_831F54A8(ctx, base);
	// 83204D84: 83BE05AC  lwz r29, 0x5ac(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(1452 as u32) ) } as u64;
	// 83204D88: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 83204D8C: 388003C4  li r4, 0x3c4
	ctx.r[4].s64 = 964;
	// 83204D90: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83204D94: 4BFC7405  bl 0x831cc198
	ctx.lr = 0x83204D98;
	sub_831CC198(ctx, base);
	// 83204D98: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83204D9C: 346B0004  addic. r3, r11, 4
	ctx.xer.ca = (ctx.r[11].u32 > (!(4 as u32)));
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83204DA0: 93AB0000  stw r29, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 83204DA4: 41820018  beq 0x83204dbc
	if ctx.cr[0].eq {
	pc = 0x83204DBC; continue 'dispatch;
	}
	// 83204DA8: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 83204DAC: 7EC4B378  mr r4, r22
	ctx.r[4].u64 = ctx.r[22].u64;
	// 83204DB0: 48001E91  bl 0x83206c40
	ctx.lr = 0x83204DB4;
	sub_83206C40(ctx, base);
	// 83204DB4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83204DB8: 48000008  b 0x83204dc0
	pc = 0x83204DC0; continue 'dispatch;
	// 83204DBC: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 83204DC0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83204DC4: 807F00A4  lwz r3, 0xa4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 83204DC8: 480039D9  bl 0x832087a0
	ctx.lr = 0x83204DCC;
	sub_832087A0(ctx, base);
	// 83204DCC: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 83204DD0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83204DD4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83204DD8: 48000D91  bl 0x83205b68
	ctx.lr = 0x83204DDC;
	sub_83205B68(ctx, base);
	// 83204DDC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83204DE0: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83204DE4: 4BFFEF05  bl 0x83203ce8
	ctx.lr = 0x83204DE8;
	sub_83203CE8(ctx, base);
	// 83204DE8: 7FBBF92E  stwx r29, r27, r31
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[27].u32.wrapping_add(ctx.r[31].u32), ctx.r[29].u32) };
	// 83204DEC: 48000008  b 0x83204df4
	pc = 0x83204DF4; continue 'dispatch;
	// 83204DF0: 838B001C  lwz r28, 0x1c(r11)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 83204DF4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83204DF8: 38600031  li r3, 0x31
	ctx.r[3].s64 = 49;
	// 83204DFC: 48002E15  bl 0x83207c10
	ctx.lr = 0x83204E00;
	sub_83207C10(ctx, base);
	// 83204E00: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 83204E04: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83204E08: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83204E0C: 48000D5D  bl 0x83205b68
	ctx.lr = 0x83204E10;
	sub_83205B68(ctx, base);
	// 83204E10: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83204E14: 397A0004  addi r11, r26, 4
	ctx.r[11].s64 = ctx.r[26].s64 + 4;
	// 83204E18: 554A2834  slwi r10, r10, 5
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(5);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83204E1C: 7D6A582E  lwzx r11, r10, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 83204E20: 917E0084  stw r11, 0x84(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(132 as u32), ctx.r[11].u32 ) };
	// 83204E24: 7EC5B378  mr r5, r22
	ctx.r[5].u64 = ctx.r[22].u64;
	// 83204E28: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83204E2C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83204E30: 48000D39  bl 0x83205b68
	ctx.lr = 0x83204E34;
	sub_83205B68(ctx, base);
	// 83204E34: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83204E38: 7EC3B378  mr r3, r22
	ctx.r[3].u64 = ctx.r[22].u64;
	// 83204E3C: 4BFFEEAD  bl 0x83203ce8
	ctx.lr = 0x83204E40;
	sub_83203CE8(ctx, base);
	// 83204E40: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83204E44: 807F00A4  lwz r3, 0xa4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 83204E48: 48003911  bl 0x83208758
	ctx.lr = 0x83204E4C;
	sub_83208758(ctx, base);
	// 83204E4C: 7EC3B378  mr r3, r22
	ctx.r[3].u64 = ctx.r[22].u64;
	// 83204E50: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 83204E54: 4BAA45E8  b 0x82ca943c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83204E58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83204E58 size=276
    let mut pc: u32 = 0x83204E58;
    'dispatch: loop {
        match pc {
            0x83204E58 => {
    //   block [0x83204E58..0x83204F6C)
	// 83204E58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83204E5C: 4BAA45A5  bl 0x82ca9400
	ctx.lr = 0x83204E60;
	sub_82CA93D0(ctx, base);
	// 83204E60: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83204E64: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83204E68: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 83204E6C: 4BFFF80D  bl 0x83204678
	ctx.lr = 0x83204E70;
	sub_83204678(ctx, base);
	// 83204E70: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 83204E74: 3D408209  lis r10, -0x7df7
	ctx.r[10].s64 = -2113339392;
	// 83204E78: 396BA974  addi r11, r11, -0x568c
	ctx.r[11].s64 = ctx.r[11].s64 + -22156;
	// 83204E7C: 394AAA60  addi r10, r10, -0x55a0
	ctx.r[10].s64 = ctx.r[10].s64 + -21920;
	// 83204E80: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83204E84: 80AA00A0  lwz r5, 0xa0(r10)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(160 as u32) ) } as u64;
	// 83204E88: 39650006  addi r11, r5, 6
	ctx.r[11].s64 = ctx.r[5].s64 + 6;
	// 83204E8C: 83BF0AB0  lwz r29, 0xab0(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2736 as u32) ) } as u64;
	// 83204E90: 557A103A  slwi r26, r11, 2
	ctx.r[26].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[26].u64 = ctx.r[26].u32 as u64;
	// 83204E94: 7D7AE82E  lwzx r11, r26, r29
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 83204E98: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83204E9C: 409A007C  bne cr6, 0x83204f18
	if !ctx.cr[6].eq {
	pc = 0x83204F18; continue 'dispatch;
	}
	// 83204EA0: 3880000F  li r4, 0xf
	ctx.r[4].s64 = 15;
	// 83204EA4: 807D00AC  lwz r3, 0xac(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(172 as u32) ) } as u64;
	// 83204EA8: 4BFF0601  bl 0x831f54a8
	ctx.lr = 0x83204EAC;
	sub_831F54A8(ctx, base);
	// 83204EAC: 839F05AC  lwz r28, 0x5ac(r31)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1452 as u32) ) } as u64;
	// 83204EB0: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 83204EB4: 388003C4  li r4, 0x3c4
	ctx.r[4].s64 = 964;
	// 83204EB8: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83204EBC: 4BFC72DD  bl 0x831cc198
	ctx.lr = 0x83204EC0;
	sub_831CC198(ctx, base);
	// 83204EC0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83204EC4: 346B0004  addic. r3, r11, 4
	ctx.xer.ca = (ctx.r[11].u32 > (!(4 as u32)));
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83204EC8: 938B0000  stw r28, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 83204ECC: 41820018  beq 0x83204ee4
	if ctx.cr[0].eq {
	pc = 0x83204EE4; continue 'dispatch;
	}
	// 83204ED0: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 83204ED4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83204ED8: 48001D69  bl 0x83206c40
	ctx.lr = 0x83204EDC;
	sub_83206C40(ctx, base);
	// 83204EDC: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 83204EE0: 48000008  b 0x83204ee8
	pc = 0x83204EE8; continue 'dispatch;
	// 83204EE4: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 83204EE8: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 83204EEC: 807D00A4  lwz r3, 0xa4(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(164 as u32) ) } as u64;
	// 83204EF0: 480038B1  bl 0x832087a0
	ctx.lr = 0x83204EF4;
	sub_832087A0(ctx, base);
	// 83204EF4: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 83204EF8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83204EFC: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83204F00: 48000C69  bl 0x83205b68
	ctx.lr = 0x83204F04;
	sub_83205B68(ctx, base);
	// 83204F04: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 83204F08: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 83204F0C: 4BFFEDDD  bl 0x83203ce8
	ctx.lr = 0x83204F10;
	sub_83203CE8(ctx, base);
	// 83204F10: 7F9AE92E  stwx r28, r26, r29
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[26].u32.wrapping_add(ctx.r[29].u32), ctx.r[28].u32) };
	// 83204F14: 48000008  b 0x83204f1c
	pc = 0x83204F1C; continue 'dispatch;
	// 83204F18: 836B001C  lwz r27, 0x1c(r11)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 83204F1C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83204F20: 38600031  li r3, 0x31
	ctx.r[3].s64 = 49;
	// 83204F24: 48002CED  bl 0x83207c10
	ctx.lr = 0x83204F28;
	sub_83207C10(ctx, base);
	// 83204F28: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 83204F2C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83204F30: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83204F34: 48000C35  bl 0x83205b68
	ctx.lr = 0x83204F38;
	sub_83205B68(ctx, base);
	// 83204F38: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83204F3C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 83204F40: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83204F44: 48000C25  bl 0x83205b68
	ctx.lr = 0x83204F48;
	sub_83205B68(ctx, base);
	// 83204F48: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83204F4C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83204F50: 4BFFED99  bl 0x83203ce8
	ctx.lr = 0x83204F54;
	sub_83203CE8(ctx, base);
	// 83204F54: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83204F58: 807D00A4  lwz r3, 0xa4(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(164 as u32) ) } as u64;
	// 83204F5C: 480037FD  bl 0x83208758
	ctx.lr = 0x83204F60;
	sub_83208758(ctx, base);
	// 83204F60: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83204F64: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83204F68: 4BAA44E8  b 0x82ca9450
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83204F70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83204F70 size=516
    let mut pc: u32 = 0x83204F70;
    'dispatch: loop {
        match pc {
            0x83204F70 => {
    //   block [0x83204F70..0x83205174)
	// 83204F70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83204F74: 4BAA4479  bl 0x82ca93ec
	ctx.lr = 0x83204F78;
	sub_82CA93D0(ctx, base);
	// 83204F78: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83204F7C: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 83204F80: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 83204F84: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 83204F88: 4BFFF6F1  bl 0x83204678
	ctx.lr = 0x83204F8C;
	sub_83204678(ctx, base);
	// 83204F8C: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 83204F90: 3D40831D  lis r10, -0x7ce3
	ctx.r[10].s64 = -2095251456;
	// 83204F94: 396BA974  addi r11, r11, -0x568c
	ctx.r[11].s64 = ctx.r[11].s64 + -22156;
	// 83204F98: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 83204F9C: 917B0000  stw r11, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83204FA0: 38AACC70  addi r5, r10, -0x3390
	ctx.r[5].s64 = ctx.r[10].s64 + -13200;
	// 83204FA4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 83204FA8: 807F0600  lwz r3, 0x600(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1536 as u32) ) } as u64;
	// 83204FAC: 833F0AB0  lwz r25, 0xab0(r31)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2736 as u32) ) } as u64;
	// 83204FB0: 4BFDD3D9  bl 0x831e2388
	ctx.lr = 0x83204FB4;
	sub_831E2388(ctx, base);
	// 83204FB4: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 83204FB8: 3D408208  lis r10, -0x7df8
	ctx.r[10].s64 = -2113404928;
	// 83204FBC: 3D20820A  lis r9, -0x7df6
	ctx.r[9].s64 = -2113273856;
	// 83204FC0: 3D008209  lis r8, -0x7df7
	ctx.r[8].s64 = -2113339392;
	// 83204FC4: 2F1C000C  cmpwi cr6, r28, 0xc
	ctx.cr[6].compare_i32(ctx.r[28].s32, 12, &mut ctx.xer);
	// 83204FC8: 3B0BAC58  addi r24, r11, -0x53a8
	ctx.r[24].s64 = ctx.r[11].s64 + -21416;
	// 83204FCC: 3AEAB734  addi r23, r10, -0x48cc
	ctx.r[23].s64 = ctx.r[10].s64 + -18636;
	// 83204FD0: 3AC91700  addi r22, r9, 0x1700
	ctx.r[22].s64 = ctx.r[9].s64 + 5888;
	// 83204FD4: 3AA8A810  addi r21, r8, -0x57f0
	ctx.r[21].s64 = ctx.r[8].s64 + -22512;
	// 83204FD8: 419A0044  beq cr6, 0x8320501c
	if ctx.cr[6].eq {
	pc = 0x8320501C; continue 'dispatch;
	}
	// 83204FDC: 2F1C000D  cmpwi cr6, r28, 0xd
	ctx.cr[6].compare_i32(ctx.r[28].s32, 13, &mut ctx.xer);
	// 83204FE0: 419A0034  beq cr6, 0x83205014
	if ctx.cr[6].eq {
	pc = 0x83205014; continue 'dispatch;
	}
	// 83204FE4: 2F1C000E  cmpwi cr6, r28, 0xe
	ctx.cr[6].compare_i32(ctx.r[28].s32, 14, &mut ctx.xer);
	// 83204FE8: 419A0024  beq cr6, 0x8320500c
	if ctx.cr[6].eq {
	pc = 0x8320500C; continue 'dispatch;
	}
	// 83204FEC: 7EA6AB78  mr r6, r21
	ctx.r[6].u64 = ctx.r[21].u64;
	// 83204FF0: 7EC5B378  mr r5, r22
	ctx.r[5].u64 = ctx.r[22].u64;
	// 83204FF4: 7EE4BB78  mr r4, r23
	ctx.r[4].u64 = ctx.r[23].u64;
	// 83204FF8: 38E002F5  li r7, 0x2f5
	ctx.r[7].s64 = 757;
	// 83204FFC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83205000: 4BF04DB1  bl 0x83109db0
	ctx.lr = 0x83205004;
	sub_83109DB0(ctx, base);
	// 83205004: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83205008: 48000024  b 0x8320502c
	pc = 0x8320502C; continue 'dispatch;
	// 8320500C: 39580008  addi r10, r24, 8
	ctx.r[10].s64 = ctx.r[24].s64 + 8;
	// 83205010: 48000010  b 0x83205020
	pc = 0x83205020; continue 'dispatch;
	// 83205014: 39580018  addi r10, r24, 0x18
	ctx.r[10].s64 = ctx.r[24].s64 + 24;
	// 83205018: 48000008  b 0x83205020
	pc = 0x83205020; continue 'dispatch;
	// 8320501C: 39580010  addi r10, r24, 0x10
	ctx.r[10].s64 = ctx.r[24].s64 + 16;
	// 83205020: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83205024: 556B2834  slwi r11, r11, 5
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(5);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83205028: 7D6B502E  lwzx r11, r11, r10
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 8320502C: 3D408209  lis r10, -0x7df7
	ctx.r[10].s64 = -2113339392;
	// 83205030: 556B1838  slwi r11, r11, 3
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83205034: 394AAA60  addi r10, r10, -0x55a0
	ctx.r[10].s64 = ctx.r[10].s64 + -21920;
	// 83205038: 7CAB502E  lwzx r5, r11, r10
	ctx.r[5].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 8320503C: 39650006  addi r11, r5, 6
	ctx.r[11].s64 = ctx.r[5].s64 + 6;
	// 83205040: 557A103A  slwi r26, r11, 2
	ctx.r[26].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[26].u64 = ctx.r[26].u32 as u64;
	// 83205044: 7D7AC82E  lwzx r11, r26, r25
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[25].u32)) } as u64;
	// 83205048: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8320504C: 409A007C  bne cr6, 0x832050c8
	if !ctx.cr[6].eq {
	pc = 0x832050C8; continue 'dispatch;
	}
	// 83205050: 3880000F  li r4, 0xf
	ctx.r[4].s64 = 15;
	// 83205054: 807900AC  lwz r3, 0xac(r25)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(172 as u32) ) } as u64;
	// 83205058: 4BFF0451  bl 0x831f54a8
	ctx.lr = 0x8320505C;
	sub_831F54A8(ctx, base);
	// 8320505C: 83DF05AC  lwz r30, 0x5ac(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1452 as u32) ) } as u64;
	// 83205060: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83205064: 388003C4  li r4, 0x3c4
	ctx.r[4].s64 = 964;
	// 83205068: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8320506C: 4BFC712D  bl 0x831cc198
	ctx.lr = 0x83205070;
	sub_831CC198(ctx, base);
	// 83205070: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83205074: 346B0004  addic. r3, r11, 4
	ctx.xer.ca = (ctx.r[11].u32 > (!(4 as u32)));
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83205078: 93CB0000  stw r30, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 8320507C: 41820018  beq 0x83205094
	if ctx.cr[0].eq {
	pc = 0x83205094; continue 'dispatch;
	}
	// 83205080: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 83205084: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 83205088: 48001BB9  bl 0x83206c40
	ctx.lr = 0x8320508C;
	sub_83206C40(ctx, base);
	// 8320508C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83205090: 48000008  b 0x83205098
	pc = 0x83205098; continue 'dispatch;
	// 83205094: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 83205098: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8320509C: 807900A4  lwz r3, 0xa4(r25)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(164 as u32) ) } as u64;
	// 832050A0: 48003701  bl 0x832087a0
	ctx.lr = 0x832050A4;
	sub_832087A0(ctx, base);
	// 832050A4: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 832050A8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 832050AC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 832050B0: 48000AB9  bl 0x83205b68
	ctx.lr = 0x832050B4;
	sub_83205B68(ctx, base);
	// 832050B4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 832050B8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 832050BC: 4BFFEC2D  bl 0x83203ce8
	ctx.lr = 0x832050C0;
	sub_83203CE8(ctx, base);
	// 832050C0: 7FDAC92E  stwx r30, r26, r25
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[26].u32.wrapping_add(ctx.r[25].u32), ctx.r[30].u32) };
	// 832050C4: 48000008  b 0x832050cc
	pc = 0x832050CC; continue 'dispatch;
	// 832050C8: 83AB001C  lwz r29, 0x1c(r11)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 832050CC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 832050D0: 38600031  li r3, 0x31
	ctx.r[3].s64 = 49;
	// 832050D4: 48002B3D  bl 0x83207c10
	ctx.lr = 0x832050D8;
	sub_83207C10(ctx, base);
	// 832050D8: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 832050DC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 832050E0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 832050E4: 48000A85  bl 0x83205b68
	ctx.lr = 0x832050E8;
	sub_83205B68(ctx, base);
	// 832050E8: 2F1C000C  cmpwi cr6, r28, 0xc
	ctx.cr[6].compare_i32(ctx.r[28].s32, 12, &mut ctx.xer);
	// 832050EC: 419A0040  beq cr6, 0x8320512c
	if ctx.cr[6].eq {
	pc = 0x8320512C; continue 'dispatch;
	}
	// 832050F0: 2F1C000D  cmpwi cr6, r28, 0xd
	ctx.cr[6].compare_i32(ctx.r[28].s32, 13, &mut ctx.xer);
	// 832050F4: 419A0030  beq cr6, 0x83205124
	if ctx.cr[6].eq {
	pc = 0x83205124; continue 'dispatch;
	}
	// 832050F8: 2F1C000E  cmpwi cr6, r28, 0xe
	ctx.cr[6].compare_i32(ctx.r[28].s32, 14, &mut ctx.xer);
	// 832050FC: 419A0020  beq cr6, 0x8320511c
	if ctx.cr[6].eq {
	pc = 0x8320511C; continue 'dispatch;
	}
	// 83205100: 7EA6AB78  mr r6, r21
	ctx.r[6].u64 = ctx.r[21].u64;
	// 83205104: 7EC5B378  mr r5, r22
	ctx.r[5].u64 = ctx.r[22].u64;
	// 83205108: 7EE4BB78  mr r4, r23
	ctx.r[4].u64 = ctx.r[23].u64;
	// 8320510C: 38E00314  li r7, 0x314
	ctx.r[7].s64 = 788;
	// 83205110: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83205114: 4BF04C9D  bl 0x83109db0
	ctx.lr = 0x83205118;
	sub_83109DB0(ctx, base);
	// 83205118: 48000028  b 0x83205140
	pc = 0x83205140; continue 'dispatch;
	// 8320511C: 3958000C  addi r10, r24, 0xc
	ctx.r[10].s64 = ctx.r[24].s64 + 12;
	// 83205120: 48000010  b 0x83205130
	pc = 0x83205130; continue 'dispatch;
	// 83205124: 3958001C  addi r10, r24, 0x1c
	ctx.r[10].s64 = ctx.r[24].s64 + 28;
	// 83205128: 48000008  b 0x83205130
	pc = 0x83205130; continue 'dispatch;
	// 8320512C: 39580014  addi r10, r24, 0x14
	ctx.r[10].s64 = ctx.r[24].s64 + 20;
	// 83205130: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83205134: 556B2834  slwi r11, r11, 5
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(5);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83205138: 7D6B502E  lwzx r11, r11, r10
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 8320513C: 917F0084  stw r11, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[11].u32 ) };
	// 83205140: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 83205144: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83205148: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8320514C: 48000A1D  bl 0x83205b68
	ctx.lr = 0x83205150;
	sub_83205B68(ctx, base);
	// 83205150: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83205154: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 83205158: 4BFFEB91  bl 0x83203ce8
	ctx.lr = 0x8320515C;
	sub_83203CE8(ctx, base);
	// 8320515C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83205160: 807900A4  lwz r3, 0xa4(r25)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(164 as u32) ) } as u64;
	// 83205164: 480035F5  bl 0x83208758
	ctx.lr = 0x83205168;
	sub_83208758(ctx, base);
	// 83205168: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 8320516C: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 83205170: 4BAA42CC  b 0x82ca943c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83205178(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83205178 size=60
    let mut pc: u32 = 0x83205178;
    'dispatch: loop {
        match pc {
            0x83205178 => {
    //   block [0x83205178..0x832051B4)
	// 83205178: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8320517C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83205180: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83205184: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83205188: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8320518C: 4BFFF4ED  bl 0x83204678
	ctx.lr = 0x83205190;
	sub_83204678(ctx, base);
	// 83205190: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 83205194: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83205198: 396BA974  addi r11, r11, -0x568c
	ctx.r[11].s64 = ctx.r[11].s64 + -22156;
	// 8320519C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 832051A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832051A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832051A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832051AC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832051B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832051B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832051B8 size=76
    let mut pc: u32 = 0x832051B8;
    'dispatch: loop {
        match pc {
            0x832051B8 => {
    //   block [0x832051B8..0x83205204)
	// 832051B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832051BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832051C0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 832051C4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832051C8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832051CC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 832051D0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 832051D4: 4BFFEA0D  bl 0x83203be0
	ctx.lr = 0x832051D8;
	sub_83203BE0(ctx, base);
	// 832051D8: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 832051DC: 93DF0010  stw r30, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[30].u32 ) };
	// 832051E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832051E4: 396BA998  addi r11, r11, -0x5668
	ctx.r[11].s64 = ctx.r[11].s64 + -22120;
	// 832051E8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 832051EC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 832051F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832051F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832051F8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 832051FC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83205200: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83205208(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83205208 size=132
    let mut pc: u32 = 0x83205208;
    'dispatch: loop {
        match pc {
            0x83205208 => {
    //   block [0x83205208..0x8320528C)
	// 83205208: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8320520C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83205210: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83205214: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83205218: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8320521C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83205220: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83205224: 4BFFE9BD  bl 0x83203be0
	ctx.lr = 0x83205228;
	sub_83203BE0(ctx, base);
	// 83205228: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 8320522C: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 83205230: 396BA998  addi r11, r11, -0x5668
	ctx.r[11].s64 = ctx.r[11].s64 + -22120;
	// 83205234: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83205238: 419A0028  beq cr6, 0x83205260
	if ctx.cr[6].eq {
	pc = 0x83205260; continue 'dispatch;
	}
	// 8320523C: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 83205240: 3D408209  lis r10, -0x7df7
	ctx.r[10].s64 = -2113339392;
	// 83205244: 3D208208  lis r9, -0x7df8
	ctx.r[9].s64 = -2113404928;
	// 83205248: 38CBA810  addi r6, r11, -0x57f0
	ctx.r[6].s64 = ctx.r[11].s64 + -22512;
	// 8320524C: 38AAA9BC  addi r5, r10, -0x5644
	ctx.r[5].s64 = ctx.r[10].s64 + -22084;
	// 83205250: 3889B734  addi r4, r9, -0x48cc
	ctx.r[4].s64 = ctx.r[9].s64 + -18636;
	// 83205254: 38E00353  li r7, 0x353
	ctx.r[7].s64 = 851;
	// 83205258: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8320525C: 4BF04B55  bl 0x83109db0
	ctx.lr = 0x83205260;
	sub_83109DB0(ctx, base);
	// 83205260: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83205264: 39400020  li r10, 0x20
	ctx.r[10].s64 = 32;
	// 83205268: 997F0005  stb r11, 5(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(5 as u32), ctx.r[11].u8 ) };
	// 8320526C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83205270: 915F0010  stw r10, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 83205274: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83205278: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8320527C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83205280: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83205284: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83205288: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83205290(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83205290 size=140
    let mut pc: u32 = 0x83205290;
    'dispatch: loop {
        match pc {
            0x83205290 => {
    //   block [0x83205290..0x8320531C)
	// 83205290: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83205294: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83205298: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8320529C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832052A0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832052A4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 832052A8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 832052AC: 4BFFE935  bl 0x83203be0
	ctx.lr = 0x832052B0;
	sub_83203BE0(ctx, base);
	// 832052B0: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 832052B4: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 832052B8: 396BA998  addi r11, r11, -0x5668
	ctx.r[11].s64 = ctx.r[11].s64 + -22120;
	// 832052BC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 832052C0: 4198000C  blt cr6, 0x832052cc
	if ctx.cr[6].lt {
	pc = 0x832052CC; continue 'dispatch;
	}
	// 832052C4: 2F1E0004  cmpwi cr6, r30, 4
	ctx.cr[6].compare_i32(ctx.r[30].s32, 4, &mut ctx.xer);
	// 832052C8: 40990028  ble cr6, 0x832052f0
	if !ctx.cr[6].gt {
	pc = 0x832052F0; continue 'dispatch;
	}
	// 832052CC: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 832052D0: 3D408209  lis r10, -0x7df7
	ctx.r[10].s64 = -2113339392;
	// 832052D4: 3D208208  lis r9, -0x7df8
	ctx.r[9].s64 = -2113404928;
	// 832052D8: 38CBA810  addi r6, r11, -0x57f0
	ctx.r[6].s64 = ctx.r[11].s64 + -22512;
	// 832052DC: 38AAA9C8  addi r5, r10, -0x5638
	ctx.r[5].s64 = ctx.r[10].s64 + -22072;
	// 832052E0: 3889B734  addi r4, r9, -0x48cc
	ctx.r[4].s64 = ctx.r[9].s64 + -18636;
	// 832052E4: 38E0035E  li r7, 0x35e
	ctx.r[7].s64 = 862;
	// 832052E8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 832052EC: 4BF04AC5  bl 0x83109db0
	ctx.lr = 0x832052F0;
	sub_83109DB0(ctx, base);
	// 832052F0: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 832052F4: 395E0021  addi r10, r30, 0x21
	ctx.r[10].s64 = ctx.r[30].s64 + 33;
	// 832052F8: 997F0005  stb r11, 5(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(5 as u32), ctx.r[11].u8 ) };
	// 832052FC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83205300: 915F0010  stw r10, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 83205304: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83205308: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8320530C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83205310: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83205314: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83205318: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83205320(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83205320 size=88
    let mut pc: u32 = 0x83205320;
    'dispatch: loop {
        match pc {
            0x83205320 => {
    //   block [0x83205320..0x83205378)
	// 83205320: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83205324: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83205328: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8320532C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83205330: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83205334: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83205338: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8320533C: 4BFFF1AD  bl 0x832044e8
	ctx.lr = 0x83205340;
	sub_832044E8(ctx, base);
	// 83205340: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83205344: 41820018  beq 0x8320535c
	if ctx.cr[0].eq {
	pc = 0x8320535C; continue 'dispatch;
	}
	// 83205348: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8320534C: 419A0010  beq cr6, 0x8320535c
	if ctx.cr[6].eq {
	pc = 0x8320535C; continue 'dispatch;
	}
	// 83205350: 389FFFFC  addi r4, r31, -4
	ctx.r[4].s64 = ctx.r[31].s64 + -4;
	// 83205354: 807FFFFC  lwz r3, -4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-4 as u32) ) } as u64;
	// 83205358: 4BFC6F71  bl 0x831cc2c8
	ctx.lr = 0x8320535C;
	sub_831CC2C8(ctx, base);
	// 8320535C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83205360: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83205364: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83205368: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8320536C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83205370: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83205374: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83205378(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83205378 size=100
    let mut pc: u32 = 0x83205378;
    'dispatch: loop {
        match pc {
            0x83205378 => {
    //   block [0x83205378..0x832053DC)
	// 83205378: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8320537C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83205380: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83205384: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83205388: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8320538C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83205390: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 83205394: 4BFFE84D  bl 0x83203be0
	ctx.lr = 0x83205398;
	sub_83203BE0(ctx, base);
	// 83205398: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 8320539C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832053A0: 396BA9E8  addi r11, r11, -0x5618
	ctx.r[11].s64 = ctx.r[11].s64 + -22040;
	// 832053A4: 915F0030  stw r10, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[10].u32 ) };
	// 832053A8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832053AC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 832053B0: 817E058C  lwz r11, 0x58c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(1420 as u32) ) } as u64;
	// 832053B4: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 832053B8: 817E058C  lwz r11, 0x58c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(1420 as u32) ) } as u64;
	// 832053BC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 832053C0: 917E058C  stw r11, 0x58c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(1420 as u32), ctx.r[11].u32 ) };
	// 832053C4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 832053C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832053CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832053D0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 832053D4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832053D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832053E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832053E0 size=100
    let mut pc: u32 = 0x832053E0;
    'dispatch: loop {
        match pc {
            0x832053E0 => {
    //   block [0x832053E0..0x83205444)
	// 832053E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832053E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832053E8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 832053EC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832053F0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832053F4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 832053F8: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 832053FC: 4BFFE7E5  bl 0x83203be0
	ctx.lr = 0x83205400;
	sub_83203BE0(ctx, base);
	// 83205400: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 83205404: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83205408: 396BA9E8  addi r11, r11, -0x5618
	ctx.r[11].s64 = ctx.r[11].s64 + -22040;
	// 8320540C: 915F0030  stw r10, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[10].u32 ) };
	// 83205410: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83205414: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83205418: 817E0590  lwz r11, 0x590(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(1424 as u32) ) } as u64;
	// 8320541C: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 83205420: 817E0590  lwz r11, 0x590(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(1424 as u32) ) } as u64;
	// 83205424: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83205428: 917E0590  stw r11, 0x590(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(1424 as u32), ctx.r[11].u32 ) };
	// 8320542C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83205430: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83205434: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83205438: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8320543C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83205440: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83205448(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83205448 size=100
    let mut pc: u32 = 0x83205448;
    'dispatch: loop {
        match pc {
            0x83205448 => {
    //   block [0x83205448..0x832054AC)
	// 83205448: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8320544C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83205450: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83205454: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83205458: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8320545C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83205460: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 83205464: 4BFFE77D  bl 0x83203be0
	ctx.lr = 0x83205468;
	sub_83203BE0(ctx, base);
	// 83205468: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 8320546C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83205470: 396BA9E8  addi r11, r11, -0x5618
	ctx.r[11].s64 = ctx.r[11].s64 + -22040;
	// 83205474: 915F0030  stw r10, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[10].u32 ) };
	// 83205478: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8320547C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83205480: 817E0594  lwz r11, 0x594(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(1428 as u32) ) } as u64;
	// 83205484: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 83205488: 817E0594  lwz r11, 0x594(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(1428 as u32) ) } as u64;
	// 8320548C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83205490: 917E0594  stw r11, 0x594(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(1428 as u32), ctx.r[11].u32 ) };
	// 83205494: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83205498: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8320549C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832054A0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 832054A4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832054A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832054B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x832054B0 size=12
    let mut pc: u32 = 0x832054B0;
    'dispatch: loop {
        match pc {
            0x832054B0 => {
    //   block [0x832054B0..0x832054BC)
	// 832054B0: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 832054B4: C02B0C18  lfs f1, 0xc18(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3096 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 832054B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832054C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832054C0 size=172
    let mut pc: u32 = 0x832054C0;
    'dispatch: loop {
        match pc {
            0x832054C0 => {
    //   block [0x832054C0..0x8320556C)
	// 832054C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832054C4: 4BAA3F41  bl 0x82ca9404
	ctx.lr = 0x832054C8;
	sub_82CA93D0(ctx, base);
	// 832054C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832054CC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 832054D0: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 832054D4: 895F00A4  lbz r10, 0xa4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 832054D8: 897C00A4  lbz r11, 0xa4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(164 as u32) ) } as u64;
	// 832054DC: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 832054E0: 419A000C  beq cr6, 0x832054ec
	if ctx.cr[6].eq {
	pc = 0x832054EC; continue 'dispatch;
	}
	// 832054E4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 832054E8: 4800007C  b 0x83205564
	pc = 0x83205564; continue 'dispatch;
	// 832054EC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 832054F0: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 832054F4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832054F8: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 832054FC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83205500: 4E800421  bctrl
	ctx.lr = 0x83205504;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83205504: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 83205508: 41980058  blt cr6, 0x83205560
	if ctx.cr[6].lt {
	pc = 0x83205560; continue 'dispatch;
	}
	// 8320550C: 3BBF0099  addi r29, r31, 0x99
	ctx.r[29].s64 = ctx.r[31].s64 + 153;
	// 83205510: 7F7FE050  subf r27, r31, r28
	ctx.r[27].s64 = ctx.r[28].s64 - ctx.r[31].s64;
	// 83205514: 7D7BE8AE  lbzx r11, r27, r29
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[27].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 83205518: 895D0000  lbz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8320551C: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 83205520: 409AFFC4  bne cr6, 0x832054e4
	if !ctx.cr[6].eq {
	pc = 0x832054E4; continue 'dispatch;
	}
	// 83205524: 7D7EE214  add r11, r30, r28
	ctx.r[11].u64 = ctx.r[30].u64 + ctx.r[28].u64;
	// 83205528: 7D5EFA14  add r10, r30, r31
	ctx.r[10].u64 = ctx.r[30].u64 + ctx.r[31].u64;
	// 8320552C: 896B009E  lbz r11, 0x9e(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(158 as u32) ) } as u64;
	// 83205530: 894A009E  lbz r10, 0x9e(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(158 as u32) ) } as u64;
	// 83205534: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 83205538: 409AFFAC  bne cr6, 0x832054e4
	if !ctx.cr[6].eq {
	pc = 0x832054E4; continue 'dispatch;
	}
	// 8320553C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83205540: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 83205544: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 83205548: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8320554C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83205550: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83205554: 4E800421  bctrl
	ctx.lr = 0x83205558;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83205558: 7F1E1800  cmpw cr6, r30, r3
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[3].s32, &mut ctx.xer);
	// 8320555C: 4099FFB8  ble cr6, 0x83205514
	if !ctx.cr[6].gt {
	pc = 0x83205514; continue 'dispatch;
	}
	// 83205560: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83205564: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83205568: 4BAA3EEC  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83205570(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83205570 size=128
    let mut pc: u32 = 0x83205570;
    'dispatch: loop {
        match pc {
            0x83205570 => {
    //   block [0x83205570..0x832055F0)
	// 83205570: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83205574: 4BAA3E99  bl 0x82ca940c
	ctx.lr = 0x83205578;
	sub_82CA93D0(ctx, base);
	// 83205578: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8320557C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83205580: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 83205584: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 83205588: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8320558C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 83205590: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83205594: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83205598: 816B0050  lwz r11, 0x50(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(80 as u32) ) } as u64;
	// 8320559C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 832055A0: 4E800421  bctrl
	ctx.lr = 0x832055A4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 832055A4: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 832055A8: 419A0028  beq cr6, 0x832055d0
	if ctx.cr[6].eq {
	pc = 0x832055D0; continue 'dispatch;
	}
	// 832055AC: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 832055B0: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 832055B4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 832055B8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 832055BC: 816B0050  lwz r11, 0x50(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(80 as u32) ) } as u64;
	// 832055C0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 832055C4: 4E800421  bctrl
	ctx.lr = 0x832055C8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 832055C8: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 832055CC: 409A001C  bne cr6, 0x832055e8
	if !ctx.cr[6].eq {
	pc = 0x832055E8; continue 'dispatch;
	}
	// 832055D0: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 832055D4: 2F1F0004  cmpwi cr6, r31, 4
	ctx.cr[6].compare_i32(ctx.r[31].s32, 4, &mut ctx.xer);
	// 832055D8: 4198FFB0  blt cr6, 0x83205588
	if ctx.cr[6].lt {
	pc = 0x83205588; continue 'dispatch;
	}
	// 832055DC: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 832055E0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 832055E4: 4BAA3E78  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
	// 832055E8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 832055EC: 4BFFFFF4  b 0x832055e0
	pc = 0x832055E0; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832055F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832055F0 size=88
    let mut pc: u32 = 0x832055F0;
    'dispatch: loop {
        match pc {
            0x832055F0 => {
    //   block [0x832055F0..0x83205648)
	// 832055F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832055F4: 4BAA3E19  bl 0x82ca940c
	ctx.lr = 0x832055F8;
	sub_82CA93D0(ctx, base);
	// 832055F8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832055FC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83205600: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 83205604: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 83205608: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8320560C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 83205610: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83205614: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83205618: 816B0050  lwz r11, 0x50(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(80 as u32) ) } as u64;
	// 8320561C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83205620: 4E800421  bctrl
	ctx.lr = 0x83205624;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83205624: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 83205628: 419A0008  beq cr6, 0x83205630
	if ctx.cr[6].eq {
	pc = 0x83205630; continue 'dispatch;
	}
	// 8320562C: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 83205630: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 83205634: 2F1F0004  cmpwi cr6, r31, 4
	ctx.cr[6].compare_i32(ctx.r[31].s32, 4, &mut ctx.xer);
	// 83205638: 4198FFD0  blt cr6, 0x83205608
	if ctx.cr[6].lt {
	pc = 0x83205608; continue 'dispatch;
	}
	// 8320563C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83205640: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83205644: 4BAA3E18  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83205648(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83205648 size=116
    let mut pc: u32 = 0x83205648;
    'dispatch: loop {
        match pc {
            0x83205648 => {
    //   block [0x83205648..0x832056BC)
	// 83205648: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8320564C: 4BAA3DC1  bl 0x82ca940c
	ctx.lr = 0x83205650;
	sub_82CA93D0(ctx, base);
	// 83205650: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83205654: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83205658: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8320565C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 83205660: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 83205664: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 83205668: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8320566C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83205670: 816B0050  lwz r11, 0x50(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(80 as u32) ) } as u64;
	// 83205674: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83205678: 4E800421  bctrl
	ctx.lr = 0x8320567C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8320567C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83205680: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 83205684: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 83205688: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8320568C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83205690: 816B0054  lwz r11, 0x54(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(84 as u32) ) } as u64;
	// 83205694: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 83205698: 409A0008  bne cr6, 0x832056a0
	if !ctx.cr[6].eq {
	pc = 0x832056A0; continue 'dispatch;
	}
	// 8320569C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 832056A0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 832056A4: 4E800421  bctrl
	ctx.lr = 0x832056A8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 832056A8: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 832056AC: 2F1F0004  cmpwi cr6, r31, 4
	ctx.cr[6].compare_i32(ctx.r[31].s32, 4, &mut ctx.xer);
	// 832056B0: 4198FFB0  blt cr6, 0x83205660
	if ctx.cr[6].lt {
	pc = 0x83205660; continue 'dispatch;
	}
	// 832056B4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 832056B8: 4BAA3DA4  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832056C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832056C0 size=52
    let mut pc: u32 = 0x832056C0;
    'dispatch: loop {
        match pc {
            0x832056C0 => {
    //   block [0x832056C0..0x832056F4)
	// 832056C0: 89630080  lbz r11, 0x80(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(128 as u32) ) } as u64;
	// 832056C4: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 832056C8: 419A002C  beq cr6, 0x832056f4
	if ctx.cr[6].eq {
		sub_832056F4(ctx, base);
		return;
	}
	// 832056CC: 89630081  lbz r11, 0x81(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(129 as u32) ) } as u64;
	// 832056D0: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 832056D4: 419A0020  beq cr6, 0x832056f4
	if ctx.cr[6].eq {
		sub_832056F4(ctx, base);
		return;
	}
	// 832056D8: 89630082  lbz r11, 0x82(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(130 as u32) ) } as u64;
	// 832056DC: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 832056E0: 419A0014  beq cr6, 0x832056f4
	if ctx.cr[6].eq {
		sub_832056F4(ctx, base);
		return;
	}
	// 832056E4: 89630083  lbz r11, 0x83(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(131 as u32) ) } as u64;
	// 832056E8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 832056EC: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 832056F0: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832056F4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832056F4 size=8
    let mut pc: u32 = 0x832056F4;
    'dispatch: loop {
        match pc {
            0x832056F4 => {
    //   block [0x832056F4..0x832056FC)
	// 832056F4: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 832056F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83205700(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83205700 size=44
    let mut pc: u32 = 0x83205700;
    'dispatch: loop {
        match pc {
            0x83205700 => {
    //   block [0x83205700..0x8320572C)
	// 83205700: 3D608331  lis r11, -0x7ccf
	ctx.r[11].s64 = -2093940736;
	// 83205704: 1D430034  mulli r10, r3, 0x34
	ctx.r[10].s64 = ctx.r[3].s64 * 52;
	// 83205708: 396B2850  addi r11, r11, 0x2850
	ctx.r[11].s64 = ctx.r[11].s64 + 10320;
	// 8320570C: 3D208209  lis r9, -0x7df7
	ctx.r[9].s64 = -2113339392;
	// 83205710: 396B0030  addi r11, r11, 0x30
	ctx.r[11].s64 = ctx.r[11].s64 + 48;
	// 83205714: 3929AEB8  addi r9, r9, -0x5148
	ctx.r[9].s64 = ctx.r[9].s64 + -20808;
	// 83205718: 7D6A582E  lwzx r11, r10, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8320571C: 556B1838  slwi r11, r11, 3
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83205720: 7D6B482E  lwzx r11, r11, r9
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 83205724: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83205728: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83205730(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83205730 size=96
    let mut pc: u32 = 0x83205730;
    'dispatch: loop {
        match pc {
            0x83205730 => {
    //   block [0x83205730..0x83205790)
	// 83205730: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83205734: 4BAA3CD9  bl 0x82ca940c
	ctx.lr = 0x83205738;
	sub_82CA93D0(ctx, base);
	// 83205738: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8320573C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 83205740: 388003C4  li r4, 0x3c4
	ctx.r[4].s64 = 964;
	// 83205744: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83205748: 807D05AC  lwz r3, 0x5ac(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(1452 as u32) ) } as u64;
	// 8320574C: 4BFC6A4D  bl 0x831cc198
	ctx.lr = 0x83205750;
	sub_831CC198(ctx, base);
	// 83205750: 817D05AC  lwz r11, 0x5ac(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(1452 as u32) ) } as u64;
	// 83205754: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83205758: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8320575C: 3BDF0004  addi r30, r31, 4
	ctx.r[30].s64 = ctx.r[31].s64 + 4;
	// 83205760: 38A003C0  li r5, 0x3c0
	ctx.r[5].s64 = 960;
	// 83205764: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83205768: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8320576C: 4BAA3D15  bl 0x82ca9480
	ctx.lr = 0x83205770;
	sub_82CA9480(ctx, base);
	// 83205770: 817D0560  lwz r11, 0x560(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(1376 as u32) ) } as u64;
	// 83205774: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83205778: 917F00E4  stw r11, 0xe4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(228 as u32), ctx.r[11].u32 ) };
	// 8320577C: 817D0560  lwz r11, 0x560(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(1376 as u32) ) } as u64;
	// 83205780: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83205784: 917D0560  stw r11, 0x560(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(1376 as u32), ctx.r[11].u32 ) };
	// 83205788: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8320578C: 4BAA3CD0  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83205790(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83205790 size=544
    let mut pc: u32 = 0x83205790;
    'dispatch: loop {
        match pc {
            0x83205790 => {
    //   block [0x83205790..0x832059B0)
	// 83205790: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83205794: 4BAA3C65  bl 0x82ca93f8
	ctx.lr = 0x83205798;
	sub_82CA93D0(ctx, base);
	// 83205798: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8320579C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 832057A0: 3CE08209  lis r7, -0x7df7
	ctx.r[7].s64 = -2113339392;
	// 832057A4: 3D008209  lis r8, -0x7df7
	ctx.r[8].s64 = -2113339392;
	// 832057A8: 3D208208  lis r9, -0x7df8
	ctx.r[9].s64 = -2113404928;
	// 832057AC: 3D408209  lis r10, -0x7df7
	ctx.r[10].s64 = -2113339392;
	// 832057B0: 80DF0010  lwz r6, 0x10(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 832057B4: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 832057B8: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 832057BC: 2F060000  cmpwi cr6, r6, 0
	ctx.cr[6].compare_i32(ctx.r[6].s32, 0, &mut ctx.xer);
	// 832057C0: 3B47B010  addi r26, r7, -0x4ff0
	ctx.r[26].s64 = ctx.r[7].s64 + -20464;
	// 832057C4: 3B28AFE8  addi r25, r8, -0x5018
	ctx.r[25].s64 = ctx.r[8].s64 + -20504;
	// 832057C8: 3B89B734  addi r28, r9, -0x48cc
	ctx.r[28].s64 = ctx.r[9].s64 + -18636;
	// 832057CC: 3B0AAFDC  addi r24, r10, -0x5024
	ctx.r[24].s64 = ctx.r[10].s64 + -20516;
	// 832057D0: 3B6BAF78  addi r27, r11, -0x5088
	ctx.r[27].s64 = ctx.r[11].s64 + -20616;
	// 832057D4: 409900B8  ble cr6, 0x8320588c
	if !ctx.cr[6].gt {
	pc = 0x8320588C; continue 'dispatch;
	}
	// 832057D8: 3BDF001C  addi r30, r31, 0x1c
	ctx.r[30].s64 = ctx.r[31].s64 + 28;
	// 832057DC: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 832057E0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 832057E4: 409A001C  bne cr6, 0x83205800
	if !ctx.cr[6].eq {
	pc = 0x83205800; continue 'dispatch;
	}
	// 832057E8: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 832057EC: 7F05C378  mr r5, r24
	ctx.r[5].u64 = ctx.r[24].u64;
	// 832057F0: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 832057F4: 38E00088  li r7, 0x88
	ctx.r[7].s64 = 136;
	// 832057F8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 832057FC: 4BF045B5  bl 0x83109db0
	ctx.lr = 0x83205800;
	sub_83109DB0(ctx, base);
	// 83205800: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83205804: 815E0034  lwz r10, 0x34(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(52 as u32) ) } as u64;
	// 83205808: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 8320580C: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 83205810: 419A001C  beq cr6, 0x8320582c
	if ctx.cr[6].eq {
	pc = 0x8320582C; continue 'dispatch;
	}
	// 83205814: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 83205818: 7F25CB78  mr r5, r25
	ctx.r[5].u64 = ctx.r[25].u64;
	// 8320581C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 83205820: 38E00089  li r7, 0x89
	ctx.r[7].s64 = 137;
	// 83205824: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83205828: 4BF04589  bl 0x83109db0
	ctx.lr = 0x8320582C;
	sub_83109DB0(ctx, base);
	// 8320582C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83205830: 815E001C  lwz r10, 0x1c(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 83205834: 812B0010  lwz r9, 0x10(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 83205838: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 8320583C: 419A0028  beq cr6, 0x83205864
	if ctx.cr[6].eq {
	pc = 0x83205864; continue 'dispatch;
	}
	// 83205840: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 83205844: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 83205848: 419A001C  beq cr6, 0x83205864
	if ctx.cr[6].eq {
	pc = 0x83205864; continue 'dispatch;
	}
	// 8320584C: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 83205850: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 83205854: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 83205858: 38E0008B  li r7, 0x8b
	ctx.r[7].s64 = 139;
	// 8320585C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83205860: 4BF04551  bl 0x83109db0
	ctx.lr = 0x83205864;
	sub_83109DB0(ctx, base);
	// 83205864: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83205868: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8320586C: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 83205870: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83205874: 4E800421  bctrl
	ctx.lr = 0x83205878;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83205878: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8320587C: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 83205880: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 83205884: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 83205888: 4198FF54  blt cr6, 0x832057dc
	if ctx.cr[6].lt {
	pc = 0x832057DC; continue 'dispatch;
	}
	// 8320588C: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 83205890: 3BA00001  li r29, 1
	ctx.r[29].s64 = 1;
	// 83205894: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 83205898: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8320589C: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 832058A0: 409900DC  ble cr6, 0x8320597c
	if !ctx.cr[6].gt {
	pc = 0x8320597C; continue 'dispatch;
	}
	// 832058A4: 3BDF0020  addi r30, r31, 0x20
	ctx.r[30].s64 = ctx.r[31].s64 + 32;
	// 832058A8: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 832058AC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 832058B0: 409A001C  bne cr6, 0x832058cc
	if !ctx.cr[6].eq {
	pc = 0x832058CC; continue 'dispatch;
	}
	// 832058B4: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 832058B8: 7F05C378  mr r5, r24
	ctx.r[5].u64 = ctx.r[24].u64;
	// 832058BC: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 832058C0: 38E00091  li r7, 0x91
	ctx.r[7].s64 = 145;
	// 832058C4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 832058C8: 4BF044E9  bl 0x83109db0
	ctx.lr = 0x832058CC;
	sub_83109DB0(ctx, base);
	// 832058CC: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 832058D0: 815E0034  lwz r10, 0x34(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(52 as u32) ) } as u64;
	// 832058D4: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 832058D8: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 832058DC: 419A001C  beq cr6, 0x832058f8
	if ctx.cr[6].eq {
	pc = 0x832058F8; continue 'dispatch;
	}
	// 832058E0: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 832058E4: 7F25CB78  mr r5, r25
	ctx.r[5].u64 = ctx.r[25].u64;
	// 832058E8: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 832058EC: 38E00092  li r7, 0x92
	ctx.r[7].s64 = 146;
	// 832058F0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 832058F4: 4BF044BD  bl 0x83109db0
	ctx.lr = 0x832058F8;
	sub_83109DB0(ctx, base);
	// 832058F8: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 832058FC: 815E001C  lwz r10, 0x1c(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 83205900: 812B0010  lwz r9, 0x10(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 83205904: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 83205908: 419A0028  beq cr6, 0x83205930
	if ctx.cr[6].eq {
	pc = 0x83205930; continue 'dispatch;
	}
	// 8320590C: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 83205910: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 83205914: 419A001C  beq cr6, 0x83205930
	if ctx.cr[6].eq {
	pc = 0x83205930; continue 'dispatch;
	}
	// 83205918: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 8320591C: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 83205920: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 83205924: 38E00094  li r7, 0x94
	ctx.r[7].s64 = 148;
	// 83205928: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8320592C: 4BF04485  bl 0x83109db0
	ctx.lr = 0x83205930;
	sub_83109DB0(ctx, base);
	// 83205930: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83205934: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83205938: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 8320593C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83205940: 4E800421  bctrl
	ctx.lr = 0x83205944;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83205944: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83205948: 40820018  bne 0x83205960
	if !ctx.cr[0].eq {
	pc = 0x83205960; continue 'dispatch;
	}
	// 8320594C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83205950: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83205954: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 83205958: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8320595C: 4E800421  bctrl
	ctx.lr = 0x83205960;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83205960: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 83205964: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 83205968: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8320596C: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 83205970: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 83205974: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 83205978: 4198FF30  blt cr6, 0x832058a8
	if ctx.cr[6].lt {
	pc = 0x832058A8; continue 'dispatch;
	}
	// 8320597C: 817F03B4  lwz r11, 0x3b4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(948 as u32) ) } as u64;
	// 83205980: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83205984: 409A0020  bne cr6, 0x832059a4
	if !ctx.cr[6].eq {
	pc = 0x832059A4; continue 'dispatch;
	}
	// 83205988: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 8320598C: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 83205990: 38ABAFC8  addi r5, r11, -0x5038
	ctx.r[5].s64 = ctx.r[11].s64 + -20536;
	// 83205994: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 83205998: 38E00099  li r7, 0x99
	ctx.r[7].s64 = 153;
	// 8320599C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 832059A0: 4BF04411  bl 0x83109db0
	ctx.lr = 0x832059A4;
	sub_83109DB0(ctx, base);
	// 832059A4: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 832059A8: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 832059AC: 4BAA3A9C  b 0x82ca9448
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832059B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832059B0 size=28
    let mut pc: u32 = 0x832059B0;
    'dispatch: loop {
        match pc {
            0x832059B0 => {
    //   block [0x832059B0..0x832059CC)
	// 832059B0: 3964003A  addi r11, r4, 0x3a
	ctx.r[11].s64 = ctx.r[4].s64 + 58;
	// 832059B4: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 832059B8: 7D6B182E  lwzx r11, r11, r3
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[3].u32)) } as u64;
	// 832059BC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 832059C0: 419A000C  beq cr6, 0x832059cc
	if ctx.cr[6].eq {
		sub_832059CC(ctx, base);
		return;
	}
	// 832059C4: 816B0050  lwz r11, 0x50(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(80 as u32) ) } as u64;
	// 832059C8: 48000010  b 0x832059d8
	sub_832059CC(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832059CC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832059CC size=56
    let mut pc: u32 = 0x832059CC;
    'dispatch: loop {
        match pc {
            0x832059CC => {
    //   block [0x832059CC..0x83205A04)
	// 832059CC: 39640014  addi r11, r4, 0x14
	ctx.r[11].s64 = ctx.r[4].s64 + 20;
	// 832059D0: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 832059D4: 7D6B182E  lwzx r11, r11, r3
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[3].u32)) } as u64;
	// 832059D8: 3D408209  lis r10, -0x7df7
	ctx.r[10].s64 = -2113339392;
	// 832059DC: 1D2B000C  mulli r9, r11, 0xc
	ctx.r[9].s64 = ctx.r[11].s64 * 12;
	// 832059E0: 396AF3B8  addi r11, r10, -0xc48
	ctx.r[11].s64 = ctx.r[10].s64 + -3144;
	// 832059E4: 396B0007  addi r11, r11, 7
	ctx.r[11].s64 = ctx.r[11].s64 + 7;
	// 832059E8: 7D6958AE  lbzx r11, r9, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 832059EC: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 832059F0: 419A0014  beq cr6, 0x83205a04
	if ctx.cr[6].eq {
		sub_83205A04(ctx, base);
		return;
	}
	// 832059F4: 396BFFFE  addi r11, r11, -2
	ctx.r[11].s64 = ctx.r[11].s64 + -2;
	// 832059F8: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 832059FC: 5563DFFE  rlwinm r3, r11, 0x1b, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 83205A00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83205A04(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83205A04 size=8
    let mut pc: u32 = 0x83205A04;
    'dispatch: loop {
        match pc {
            0x83205A04 => {
    //   block [0x83205A04..0x83205A0C)
	// 83205A04: 38600002  li r3, 2
	ctx.r[3].s64 = 2;
	// 83205A08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83205A10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83205A10 size=344
    let mut pc: u32 = 0x83205A10;
    'dispatch: loop {
        match pc {
            0x83205A10 => {
    //   block [0x83205A10..0x83205B68)
	// 83205A10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83205A14: 4BAA39F1  bl 0x82ca9404
	ctx.lr = 0x83205A18;
	sub_82CA93D0(ctx, base);
	// 83205A18: 3D408209  lis r10, -0x7df7
	ctx.r[10].s64 = -2113339392;
	// 83205A1C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83205A20: 392AAF70  addi r9, r10, -0x5090
	ctx.r[9].s64 = ctx.r[10].s64 + -20624;
	// 83205A24: 3BE0FFFF  li r31, -1
	ctx.r[31].s64 = -1;
	// 83205A28: 91630034  stw r11, 0x34(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 83205A2C: 38800030  li r4, 0x30
	ctx.r[4].s64 = 48;
	// 83205A30: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 83205A34: 91630014  stw r11, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 83205A38: 38E0000C  li r7, 0xc
	ctx.r[7].s64 = 12;
	// 83205A3C: 93E30038  stw r31, 0x38(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(56 as u32), ctx.r[31].u32 ) };
	// 83205A40: 7D7E5B78  mr r30, r11
	ctx.r[30].u64 = ctx.r[11].u64;
	// 83205A44: 90830050  stw r4, 0x50(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(80 as u32), ctx.r[4].u32 ) };
	// 83205A48: 38C30098  addi r6, r3, 0x98
	ctx.r[6].s64 = ctx.r[3].s64 + 152;
	// 83205A4C: 8149FFF8  lwz r10, -8(r9)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83205A50: 38A3009E  addi r5, r3, 0x9e
	ctx.r[5].s64 = ctx.r[3].s64 + 158;
	// 83205A54: 91430080  stw r10, 0x80(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(128 as u32), ctx.r[10].u32 ) };
	// 83205A58: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 83205A5C: 8109FFF0  lwz r8, -0x10(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-16 as u32) ) } as u64;
	// 83205A60: 910303B0  stw r8, 0x3b0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(944 as u32), ctx.r[8].u32 ) };
	// 83205A64: 3D008331  lis r8, -0x7ccf
	ctx.r[8].s64 = -2093940736;
	// 83205A68: 7FC33D2C  stwbrx r30, r3, r7
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[3].u32.wrapping_add(ctx.r[7].u32), (ctx.r[30].u32.swap_bytes())) };
	// 83205A6C: 3BC82850  addi r30, r8, 0x2850
	ctx.r[30].s64 = ctx.r[8].s64 + 10320;
	// 83205A70: 83890004  lwz r28, 4(r9)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 83205A74: 83A90000  lwz r29, 0(r9)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 83205A78: 99630098  stb r11, 0x98(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(152 as u32), ctx.r[11].u8 ) };
	// 83205A7C: 9963009E  stb r11, 0x9e(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(158 as u32), ctx.r[11].u8 ) };
	// 83205A80: 9163001C  stw r11, 0x1c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 83205A84: 916300C8  stw r11, 0xc8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(200 as u32), ctx.r[11].u32 ) };
	// 83205A88: 390A000E  addi r8, r10, 0xe
	ctx.r[8].s64 = ctx.r[10].s64 + 14;
	// 83205A8C: 38EA0014  addi r7, r10, 0x14
	ctx.r[7].s64 = ctx.r[10].s64 + 20;
	// 83205A90: 551B103A  slwi r27, r8, 2
	ctx.r[27].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[27].u64 = ctx.r[27].u32 as u64;
	// 83205A94: 390A0020  addi r8, r10, 0x20
	ctx.r[8].s64 = ctx.r[10].s64 + 32;
	// 83205A98: 54E7103A  slwi r7, r7, 2
	ctx.r[7].u32 = ctx.r[7].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 83205A9C: 5508103A  slwi r8, r8, 2
	ctx.r[8].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 83205AA0: 7FFB192E  stwx r31, r27, r3
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[27].u32.wrapping_add(ctx.r[3].u32), ctx.r[31].u32) };
	// 83205AA4: 7C87192E  stwx r4, r7, r3
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[7].u32.wrapping_add(ctx.r[3].u32), ctx.r[4].u32) };
	// 83205AA8: 80E9FFF0  lwz r7, -0x10(r9)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-16 as u32) ) } as u64;
	// 83205AAC: 7CE8192E  stwx r7, r8, r3
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[3].u32), ctx.r[7].u32) };
	// 83205AB0: 80E30018  lwz r7, 0x18(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 83205AB4: 1CE70034  mulli r7, r7, 0x34
	ctx.r[7].s64 = ctx.r[7].s64 * 52;
	// 83205AB8: 7CE7F02E  lwzx r7, r7, r30
	ctx.r[7].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[7].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 83205ABC: 54E7F7FF  rlwinm. r7, r7, 0x1e, 0x1f, 0x1f
	ctx.r[7].u64 = ctx.r[7].u32 as u64 & 0x00000003u64;
	ctx.cr[0].compare_i32(ctx.r[7].s32, 0, &mut ctx.xer);
	// 83205AC0: 41820018  beq 0x83205ad8
	if ctx.cr[0].eq {
	pc = 0x83205AD8; continue 'dispatch;
	}
	// 83205AC4: 2F0A0001  cmpwi cr6, r10, 1
	ctx.cr[6].compare_i32(ctx.r[10].s32, 1, &mut ctx.xer);
	// 83205AC8: 409A000C  bne cr6, 0x83205ad4
	if !ctx.cr[6].eq {
	pc = 0x83205AD4; continue 'dispatch;
	}
	// 83205ACC: 93830084  stw r28, 0x84(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(132 as u32), ctx.r[28].u32 ) };
	// 83205AD0: 48000008  b 0x83205ad8
	pc = 0x83205AD8; continue 'dispatch;
	// 83205AD4: 7FA8192E  stwx r29, r8, r3
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[3].u32), ctx.r[29].u32) };
	// 83205AD8: 390A0007  addi r8, r10, 7
	ctx.r[8].s64 = ctx.r[10].s64 + 7;
	// 83205ADC: 7D6651AE  stbx r11, r6, r10
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[6].u32.wrapping_add(ctx.r[10].u32), ctx.r[11].u8) };
	// 83205AE0: 38EA0032  addi r7, r10, 0x32
	ctx.r[7].s64 = ctx.r[10].s64 + 50;
	// 83205AE4: 7D6551AE  stbx r11, r5, r10
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[5].u32.wrapping_add(ctx.r[10].u32), ctx.r[11].u8) };
	// 83205AE8: 5508103A  slwi r8, r8, 2
	ctx.r[8].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 83205AEC: 54E7103A  slwi r7, r7, 2
	ctx.r[7].u32 = ctx.r[7].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 83205AF0: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 83205AF4: 2F0A0006  cmpwi cr6, r10, 6
	ctx.cr[6].compare_i32(ctx.r[10].s32, 6, &mut ctx.xer);
	// 83205AF8: 7D68192E  stwx r11, r8, r3
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[3].u32), ctx.r[11].u32) };
	// 83205AFC: 7D67192E  stwx r11, r7, r3
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[7].u32.wrapping_add(ctx.r[3].u32), ctx.r[11].u32) };
	// 83205B00: 4198FF88  blt cr6, 0x83205a88
	if ctx.cr[6].lt {
	pc = 0x83205A88; continue 'dispatch;
	}
	// 83205B04: 3943016C  addi r10, r3, 0x16c
	ctx.r[10].s64 = ctx.r[3].s64 + 364;
	// 83205B08: 996300A4  stb r11, 0xa4(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(164 as u32), ctx.r[11].u8 ) };
	// 83205B0C: 38E300E8  addi r7, r3, 0xe8
	ctx.r[7].s64 = ctx.r[3].s64 + 232;
	// 83205B10: 39000020  li r8, 0x20
	ctx.r[8].s64 = 32;
	// 83205B14: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 83205B18: 91670000  stw r11, 0(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83205B1C: 3508FFFF  addic. r8, r8, -1
	ctx.xer.ca = (ctx.r[8].u32 > (!(-1 as u32)));
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 83205B20: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 83205B24: 38E70004  addi r7, r7, 4
	ctx.r[7].s64 = ctx.r[7].s64 + 4;
	// 83205B28: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 83205B2C: 912A0008  stw r9, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 83205B30: 912A000C  stw r9, 0xc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 83205B34: 394A0010  addi r10, r10, 0x10
	ctx.r[10].s64 = ctx.r[10].s64 + 16;
	// 83205B38: 4082FFE0  bne 0x83205b18
	if !ctx.cr[0].eq {
	pc = 0x83205B18; continue 'dispatch;
	}
	// 83205B3C: 390303A0  addi r8, r3, 0x3a0
	ctx.r[8].s64 = ctx.r[3].s64 + 928;
	// 83205B40: 39430384  addi r10, r3, 0x384
	ctx.r[10].s64 = ctx.r[3].s64 + 900;
	// 83205B44: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 83205B48: 916AFFFC  stw r11, -4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-4 as u32), ctx.r[11].u32 ) };
	// 83205B4C: 3529FFFF  addic. r9, r9, -1
	ctx.xer.ca = (ctx.r[9].u32 > (!(-1 as u32)));
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 83205B50: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83205B54: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 83205B58: 91680000  stw r11, 0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83205B5C: 39080004  addi r8, r8, 4
	ctx.r[8].s64 = ctx.r[8].s64 + 4;
	// 83205B60: 4082FFE8  bne 0x83205b48
	if !ctx.cr[0].eq {
	pc = 0x83205B48; continue 'dispatch;
	}
	// 83205B64: 4BAA38F0  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83205B68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83205B68 size=120
    let mut pc: u32 = 0x83205B68;
    'dispatch: loop {
        match pc {
            0x83205B68 => {
    //   block [0x83205B68..0x83205BE0)
	// 83205B68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83205B6C: 4BAA38A1  bl 0x82ca940c
	ctx.lr = 0x83205B70;
	sub_82CA93D0(ctx, base);
	// 83205B70: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83205B74: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 83205B78: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83205B7C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 83205B80: 2F1F0006  cmpwi cr6, r31, 6
	ctx.cr[6].compare_i32(ctx.r[31].s32, 6, &mut ctx.xer);
	// 83205B84: 41980028  blt cr6, 0x83205bac
	if ctx.cr[6].lt {
	pc = 0x83205BAC; continue 'dispatch;
	}
	// 83205B88: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 83205B8C: 3D408209  lis r10, -0x7df7
	ctx.r[10].s64 = -2113339392;
	// 83205B90: 3D208208  lis r9, -0x7df8
	ctx.r[9].s64 = -2113404928;
	// 83205B94: 38CBAF78  addi r6, r11, -0x5088
	ctx.r[6].s64 = ctx.r[11].s64 + -20616;
	// 83205B98: 38AAB064  addi r5, r10, -0x4f9c
	ctx.r[5].s64 = ctx.r[10].s64 + -20380;
	// 83205B9C: 3889B734  addi r4, r9, -0x48cc
	ctx.r[4].s64 = ctx.r[9].s64 + -18636;
	// 83205BA0: 38E000FB  li r7, 0xfb
	ctx.r[7].s64 = 251;
	// 83205BA4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83205BA8: 4BF04209  bl 0x83109db0
	ctx.lr = 0x83205BAC;
	sub_83109DB0(ctx, base);
	// 83205BAC: 397F0014  addi r11, r31, 0x14
	ctx.r[11].s64 = ctx.r[31].s64 + 20;
	// 83205BB0: 815D0020  lwz r10, 0x20(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(32 as u32) ) } as u64;
	// 83205BB4: 393F000E  addi r9, r31, 0xe
	ctx.r[9].s64 = ctx.r[31].s64 + 14;
	// 83205BB8: 391F0007  addi r8, r31, 7
	ctx.r[8].s64 = ctx.r[31].s64 + 7;
	// 83205BBC: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83205BC0: 5529103A  slwi r9, r9, 2
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 83205BC4: 5508103A  slwi r8, r8, 2
	ctx.r[8].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 83205BC8: 7D4BF12E  stwx r10, r11, r30
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32), ctx.r[10].u32) };
	// 83205BCC: 817D000C  lwz r11, 0xc(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 83205BD0: 7D69F12E  stwx r11, r9, r30
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[30].u32), ctx.r[11].u32) };
	// 83205BD4: 7FA8F12E  stwx r29, r8, r30
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[30].u32), ctx.r[29].u32) };
	// 83205BD8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83205BDC: 4BAA3880  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83205BE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x83205BE0 size=980
    let mut pc: u32 = 0x83205BE0;
    'dispatch: loop {
        match pc {
            0x83205BE0 => {
    //   block [0x83205BE0..0x83205FB4)
	// 83205BE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83205BE4: 4BAA3821  bl 0x82ca9404
	ctx.lr = 0x83205BE8;
	sub_82CA93D0(ctx, base);
	// 83205BE8: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83205BEC: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 83205BF0: F8C100D8  std r6, 0xd8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(216 as u32), ctx.r[6].u64 ) };
	// 83205BF4: F8E100E0  std r7, 0xe0(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(224 as u32), ctx.r[7].u64 ) };
	// 83205BF8: 3D408209  lis r10, -0x7df7
	ctx.r[10].s64 = -2113339392;
	// 83205BFC: F90100E8  std r8, 0xe8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(232 as u32), ctx.r[8].u64 ) };
	// 83205C00: 390100E0  addi r8, r1, 0xe0
	ctx.r[8].s64 = ctx.r[1].s64 + 224;
	// 83205C04: F92100F0  std r9, 0xf0(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(240 as u32), ctx.r[9].u64 ) };
	// 83205C08: 392100D8  addi r9, r1, 0xd8
	ctx.r[9].s64 = ctx.r[1].s64 + 216;
	// 83205C0C: 394AAA58  addi r10, r10, -0x55a8
	ctx.r[10].s64 = ctx.r[10].s64 + -21928;
	// 83205C10: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 83205C14: 83EBAF60  lwz r31, -0x50a0(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-20640 as u32) ) } as u64;
	// 83205C18: 396100E8  addi r11, r1, 0xe8
	ctx.r[11].s64 = ctx.r[1].s64 + 232;
	// 83205C1C: 91210070  stw r9, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[9].u32 ) };
	// 83205C20: 392100F0  addi r9, r1, 0xf0
	ctx.r[9].s64 = ctx.r[1].s64 + 240;
	// 83205C24: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83205C28: 91610078  stw r11, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[11].u32 ) };
	// 83205C2C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 83205C30: 9121007C  stw r9, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[9].u32 ) };
	// 83205C34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 83205C38: 91410060  stw r10, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[10].u32 ) };
	// 83205C3C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83205C40: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 83205C44: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 83205C48: 91410064  stw r10, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[10].u32 ) };
	// 83205C4C: 39010070  addi r8, r1, 0x70
	ctx.r[8].s64 = ctx.r[1].s64 + 112;
	// 83205C50: 91410068  stw r10, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[10].u32 ) };
	// 83205C54: 9141006C  stw r10, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[10].u32 ) };
	// 83205C58: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83205C5C: 2F060000  cmpwi cr6, r6, 0
	ctx.cr[6].compare_i32(ctx.r[6].s32, 0, &mut ctx.xer);
	// 83205C60: 41980080  blt cr6, 0x83205ce0
	if ctx.cr[6].lt {
	pc = 0x83205CE0; continue 'dispatch;
	}
	// 83205C64: 38E10060  addi r7, r1, 0x60
	ctx.r[7].s64 = ctx.r[1].s64 + 96;
	// 83205C68: 81680000  lwz r11, 0(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 83205C6C: 7F093000  cmpw cr6, r9, r6
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[6].s32, &mut ctx.xer);
	// 83205C70: 419A004C  beq cr6, 0x83205cbc
	if ctx.cr[6].eq {
	pc = 0x83205CBC; continue 'dispatch;
	}
	// 83205C74: 81470000  lwz r10, 0(r7)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 83205C78: 838B0000  lwz r28, 0(r11)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83205C7C: 836A0000  lwz r27, 0(r10)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 83205C80: 7F1BE040  cmplw cr6, r27, r28
	ctx.cr[6].compare_u32(ctx.r[27].u32, ctx.r[28].u32, &mut ctx.xer);
	// 83205C84: 409A0018  bne cr6, 0x83205c9c
	if !ctx.cr[6].eq {
	pc = 0x83205C9C; continue 'dispatch;
	}
	// 83205C88: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83205C8C: 814A0004  lwz r10, 4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 83205C90: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 83205C94: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83205C98: 419A0008  beq cr6, 0x83205ca0
	if ctx.cr[6].eq {
	pc = 0x83205CA0; continue 'dispatch;
	}
	// 83205C9C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83205CA0: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83205CA4: 40820034  bne 0x83205cd8
	if !ctx.cr[0].eq {
	pc = 0x83205CD8; continue 'dispatch;
	}
	// 83205CA8: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 83205CAC: 38E70004  addi r7, r7, 4
	ctx.r[7].s64 = ctx.r[7].s64 + 4;
	// 83205CB0: 7F093000  cmpw cr6, r9, r6
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[6].s32, &mut ctx.xer);
	// 83205CB4: 4099FFB4  ble cr6, 0x83205c68
	if !ctx.cr[6].gt {
	pc = 0x83205C68; continue 'dispatch;
	}
	// 83205CB8: 48000028  b 0x83205ce0
	pc = 0x83205CE0; continue 'dispatch;
	// 83205CBC: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 83205CC0: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83205CC4: 7CC93378  mr r9, r6
	ctx.r[9].u64 = ctx.r[6].u64;
	// 83205CC8: 38C60001  addi r6, r6, 1
	ctx.r[6].s64 = ctx.r[6].s64 + 1;
	// 83205CCC: 38630004  addi r3, r3, 4
	ctx.r[3].s64 = ctx.r[3].s64 + 4;
	// 83205CD0: 7D2551AE  stbx r9, r5, r10
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[5].u32.wrapping_add(ctx.r[10].u32), ctx.r[9].u8) };
	// 83205CD4: 4800000C  b 0x83205ce0
	pc = 0x83205CE0; continue 'dispatch;
	// 83205CD8: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 83205CDC: 7D2559AE  stbx r9, r5, r11
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[5].u32.wrapping_add(ctx.r[11].u32), ctx.r[9].u8) };
	// 83205CE0: 38A50001  addi r5, r5, 1
	ctx.r[5].s64 = ctx.r[5].s64 + 1;
	// 83205CE4: 39080004  addi r8, r8, 4
	ctx.r[8].s64 = ctx.r[8].s64 + 4;
	// 83205CE8: 2F050004  cmpwi cr6, r5, 4
	ctx.cr[6].compare_i32(ctx.r[5].s32, 4, &mut ctx.xer);
	// 83205CEC: 4198FF6C  blt cr6, 0x83205c58
	if ctx.cr[6].lt {
	pc = 0x83205C58; continue 'dispatch;
	}
	// 83205CF0: 93E10054  stw r31, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[31].u32 ) };
	// 83205CF4: 2F060000  cmpwi cr6, r6, 0
	ctx.cr[6].compare_i32(ctx.r[6].s32, 0, &mut ctx.xer);
	// 83205CF8: 4099008C  ble cr6, 0x83205d84
	if !ctx.cr[6].gt {
	pc = 0x83205D84; continue 'dispatch;
	}
	// 83205CFC: 3866FFFF  addi r3, r6, -1
	ctx.r[3].s64 = ctx.r[6].s64 + -1;
	// 83205D00: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 83205D04: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83205D08: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83205D0C: 4099006C  ble cr6, 0x83205d78
	if !ctx.cr[6].gt {
	pc = 0x83205D78; continue 'dispatch;
	}
	// 83205D10: 39410060  addi r10, r1, 0x60
	ctx.r[10].s64 = ctx.r[1].s64 + 96;
	// 83205D14: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 83205D18: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 83205D1C: 81090000  lwz r8, 0(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 83205D20: 80EB0000  lwz r7, 0(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83205D24: 7F083840  cmplw cr6, r8, r7
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[7].u32, &mut ctx.xer);
	// 83205D28: 409A000C  bne cr6, 0x83205d34
	if !ctx.cr[6].eq {
	pc = 0x83205D34; continue 'dispatch;
	}
	// 83205D2C: 81090004  lwz r8, 4(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 83205D30: 80EB0004  lwz r7, 4(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83205D34: 7D074010  subfc r8, r7, r8
	ctx.xer.ca = ctx.r[8].u32 >= ctx.r[7].u32;
	ctx.r[8].s64 = ctx.r[8].s64 - ctx.r[7].s64;
	// 83205D38: 7D084110  subfe r8, r8, r8
	let x = (!ctx.r[8].u32);
	let y = ctx.r[8].u32;
	let s = x.wrapping_add(y);
	let res = s.wrapping_add(ctx.xer.ca as u32);
	tmp.u8 = (s < x) as u8 | (res < s) as u8;
	ctx.r[8].u32 = res;
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	ctx.xer.ca = (tmp.u8 != 0);
	// 83205D3C: 550807FE  clrlwi r8, r8, 0x1f
	ctx.r[8].u64 = ctx.r[8].u32 as u64 & 0x00000001u64;
	// 83205D40: 5508063F  clrlwi. r8, r8, 0x18
	ctx.r[8].u64 = ctx.r[8].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 83205D44: 41820024  beq 0x83205d68
	if ctx.cr[0].eq {
	pc = 0x83205D68; continue 'dispatch;
	}
	// 83205D48: 39010054  addi r8, r1, 0x54
	ctx.r[8].s64 = ctx.r[1].s64 + 84;
	// 83205D4C: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83205D50: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 83205D54: 7D654214  add r11, r5, r8
	ctx.r[11].u64 = ctx.r[5].u64 + ctx.r[8].u64;
	// 83205D58: 7D2540AE  lbzx r9, r5, r8
	ctx.r[9].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[5].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 83205D5C: 88EB0001  lbz r7, 1(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(1 as u32) ) } as u64;
	// 83205D60: 992B0001  stb r9, 1(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(1 as u32), ctx.r[9].u8 ) };
	// 83205D64: 7CE541AE  stbx r7, r5, r8
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[5].u32.wrapping_add(ctx.r[8].u32), ctx.r[7].u8) };
	// 83205D68: 38A50001  addi r5, r5, 1
	ctx.r[5].s64 = ctx.r[5].s64 + 1;
	// 83205D6C: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 83205D70: 7F051800  cmpw cr6, r5, r3
	ctx.cr[6].compare_i32(ctx.r[5].s32, ctx.r[3].s32, &mut ctx.xer);
	// 83205D74: 4198FFA0  blt cr6, 0x83205d14
	if ctx.cr[6].lt {
	pc = 0x83205D14; continue 'dispatch;
	}
	// 83205D78: 37FFFFFF  addic. r31, r31, -1
	ctx.xer.ca = (ctx.r[31].u32 > (!(-1 as u32)));
	ctx.r[31].s64 = ctx.r[31].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 83205D7C: 3863FFFF  addi r3, r3, -1
	ctx.r[3].s64 = ctx.r[3].s64 + -1;
	// 83205D80: 4082FF84  bne 0x83205d04
	if !ctx.cr[0].eq {
	pc = 0x83205D04; continue 'dispatch;
	}
	// 83205D84: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83205D88: 2F060000  cmpwi cr6, r6, 0
	ctx.cr[6].compare_i32(ctx.r[6].s32, 0, &mut ctx.xer);
	// 83205D8C: 40990024  ble cr6, 0x83205db0
	if !ctx.cr[6].gt {
	pc = 0x83205DB0; continue 'dispatch;
	}
	// 83205D90: 39410054  addi r10, r1, 0x54
	ctx.r[10].s64 = ctx.r[1].s64 + 84;
	// 83205D94: 39210058  addi r9, r1, 0x58
	ctx.r[9].s64 = ctx.r[1].s64 + 88;
	// 83205D98: 7D685B78  mr r8, r11
	ctx.r[8].u64 = ctx.r[11].u64;
	// 83205D9C: 7D4B50AE  lbzx r10, r11, r10
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 83205DA0: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83205DA4: 7F0B3000  cmpw cr6, r11, r6
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[6].s32, &mut ctx.xer);
	// 83205DA8: 7D0A49AE  stbx r8, r10, r9
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32), ctx.r[8].u8) };
	// 83205DAC: 4198FFE4  blt cr6, 0x83205d90
	if ctx.cr[6].lt {
	pc = 0x83205D90; continue 'dispatch;
	}
	// 83205DB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83205DB4: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 83205DB8: 39210058  addi r9, r1, 0x58
	ctx.r[9].s64 = ctx.r[1].s64 + 88;
	// 83205DBC: 39010054  addi r8, r1, 0x54
	ctx.r[8].s64 = ctx.r[1].s64 + 84;
	// 83205DC0: 7D4B50AE  lbzx r10, r11, r10
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 83205DC4: 7D4A48AE  lbzx r10, r10, r9
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 83205DC8: 7D4B41AE  stbx r10, r11, r8
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[8].u32), ctx.r[10].u8) };
	// 83205DCC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83205DD0: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 83205DD4: 4198FFE0  blt cr6, 0x83205db4
	if ctx.cr[6].lt {
	pc = 0x83205DB4; continue 'dispatch;
	}
	// 83205DD8: 817E03B8  lwz r11, 0x3b8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(952 as u32) ) } as u64;
	// 83205DDC: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 83205DE0: 2F060001  cmpwi cr6, r6, 1
	ctx.cr[6].compare_i32(ctx.r[6].s32, 1, &mut ctx.xer);
	// 83205DE4: 896B0571  lbz r11, 0x571(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(1393 as u32) ) } as u64;
	// 83205DE8: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83205DEC: 418200DC  beq 0x83205ec8
	if ctx.cr[0].eq {
	pc = 0x83205EC8; continue 'dispatch;
	}
	// 83205DF0: 419A00B0  beq cr6, 0x83205ea0
	if ctx.cr[6].eq {
	pc = 0x83205EA0; continue 'dispatch;
	}
	// 83205DF4: 2F060002  cmpwi cr6, r6, 2
	ctx.cr[6].compare_i32(ctx.r[6].s32, 2, &mut ctx.xer);
	// 83205DF8: 419A008C  beq cr6, 0x83205e84
	if ctx.cr[6].eq {
	pc = 0x83205E84; continue 'dispatch;
	}
	// 83205DFC: 2F060003  cmpwi cr6, r6, 3
	ctx.cr[6].compare_i32(ctx.r[6].s32, 3, &mut ctx.xer);
	// 83205E00: 419A0060  beq cr6, 0x83205e60
	if ctx.cr[6].eq {
	pc = 0x83205E60; continue 'dispatch;
	}
	// 83205E04: 2F060004  cmpwi cr6, r6, 4
	ctx.cr[6].compare_i32(ctx.r[6].s32, 4, &mut ctx.xer);
	// 83205E08: 419A002C  beq cr6, 0x83205e34
	if ctx.cr[6].eq {
	pc = 0x83205E34; continue 'dispatch;
	}
	// 83205E0C: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 83205E10: 3D40820A  lis r10, -0x7df6
	ctx.r[10].s64 = -2113273856;
	// 83205E14: 3D208208  lis r9, -0x7df8
	ctx.r[9].s64 = -2113404928;
	// 83205E18: 38CBAF78  addi r6, r11, -0x5088
	ctx.r[6].s64 = ctx.r[11].s64 + -20616;
	// 83205E1C: 38AA1700  addi r5, r10, 0x1700
	ctx.r[5].s64 = ctx.r[10].s64 + 5888;
	// 83205E20: 3889B734  addi r4, r9, -0x48cc
	ctx.r[4].s64 = ctx.r[9].s64 + -18636;
	// 83205E24: 38E00181  li r7, 0x181
	ctx.r[7].s64 = 385;
	// 83205E28: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83205E2C: 4BF03F85  bl 0x83109db0
	ctx.lr = 0x83205E30;
	sub_83109DB0(ctx, base);
	// 83205E30: 48000084  b 0x83205eb4
	pc = 0x83205EB4; continue 'dispatch;
	// 83205E34: 8161006C  lwz r11, 0x6c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(108 as u32) ) } as u64;
	// 83205E38: 81410068  lwz r10, 0x68(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 83205E3C: 81210064  lwz r9, 0x64(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 83205E40: 81010060  lwz r8, 0x60(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 83205E44: 806400AC  lwz r3, 0xac(r4)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(172 as u32) ) } as u64;
	// 83205E48: E8EB0000  ld r7, 0(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 83205E4C: E8CA0000  ld r6, 0(r10)
	ctx.r[6].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	// 83205E50: E8A90000  ld r5, 0(r9)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) };
	// 83205E54: E8880000  ld r4, 0(r8)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) };
	// 83205E58: 4BFEF9B1  bl 0x831f5808
	ctx.lr = 0x83205E5C;
	sub_831F5808(ctx, base);
	// 83205E5C: 48000054  b 0x83205eb0
	pc = 0x83205EB0; continue 'dispatch;
	// 83205E60: 81610068  lwz r11, 0x68(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 83205E64: 81410064  lwz r10, 0x64(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 83205E68: 81210060  lwz r9, 0x60(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 83205E6C: 806400AC  lwz r3, 0xac(r4)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(172 as u32) ) } as u64;
	// 83205E70: E8CB0000  ld r6, 0(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 83205E74: E8AA0000  ld r5, 0(r10)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	// 83205E78: E8890000  ld r4, 0(r9)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) };
	// 83205E7C: 4BFEF96D  bl 0x831f57e8
	ctx.lr = 0x83205E80;
	sub_831F57E8(ctx, base);
	// 83205E80: 48000030  b 0x83205eb0
	pc = 0x83205EB0; continue 'dispatch;
	// 83205E84: 81610064  lwz r11, 0x64(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 83205E88: 81410060  lwz r10, 0x60(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 83205E8C: 806400AC  lwz r3, 0xac(r4)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(172 as u32) ) } as u64;
	// 83205E90: E8AB0000  ld r5, 0(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 83205E94: E88A0000  ld r4, 0(r10)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	// 83205E98: 4BFEF931  bl 0x831f57c8
	ctx.lr = 0x83205E9C;
	sub_831F57C8(ctx, base);
	// 83205E9C: 48000014  b 0x83205eb0
	pc = 0x83205EB0; continue 'dispatch;
	// 83205EA0: 81610060  lwz r11, 0x60(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 83205EA4: 806400AC  lwz r3, 0xac(r4)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(172 as u32) ) } as u64;
	// 83205EA8: E88B0000  ld r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 83205EAC: 4BFEF8FD  bl 0x831f57a8
	ctx.lr = 0x83205EB0;
	sub_831F57A8(ctx, base);
	// 83205EB0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83205EB4: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 83205EB8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83205EBC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83205EC0: 4BFDFE41  bl 0x831e5d00
	ctx.lr = 0x83205EC4;
	sub_831E5D00(ctx, base);
	// 83205EC4: 480000D8  b 0x83205f9c
	pc = 0x83205F9C; continue 'dispatch;
	// 83205EC8: 419A00B0  beq cr6, 0x83205f78
	if ctx.cr[6].eq {
	pc = 0x83205F78; continue 'dispatch;
	}
	// 83205ECC: 2F060002  cmpwi cr6, r6, 2
	ctx.cr[6].compare_i32(ctx.r[6].s32, 2, &mut ctx.xer);
	// 83205ED0: 419A008C  beq cr6, 0x83205f5c
	if ctx.cr[6].eq {
	pc = 0x83205F5C; continue 'dispatch;
	}
	// 83205ED4: 2F060003  cmpwi cr6, r6, 3
	ctx.cr[6].compare_i32(ctx.r[6].s32, 3, &mut ctx.xer);
	// 83205ED8: 419A0060  beq cr6, 0x83205f38
	if ctx.cr[6].eq {
	pc = 0x83205F38; continue 'dispatch;
	}
	// 83205EDC: 2F060004  cmpwi cr6, r6, 4
	ctx.cr[6].compare_i32(ctx.r[6].s32, 4, &mut ctx.xer);
	// 83205EE0: 419A002C  beq cr6, 0x83205f0c
	if ctx.cr[6].eq {
	pc = 0x83205F0C; continue 'dispatch;
	}
	// 83205EE4: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 83205EE8: 3D40820A  lis r10, -0x7df6
	ctx.r[10].s64 = -2113273856;
	// 83205EEC: 3D208208  lis r9, -0x7df8
	ctx.r[9].s64 = -2113404928;
	// 83205EF0: 38CBAF78  addi r6, r11, -0x5088
	ctx.r[6].s64 = ctx.r[11].s64 + -20616;
	// 83205EF4: 38AA1700  addi r5, r10, 0x1700
	ctx.r[5].s64 = ctx.r[10].s64 + 5888;
	// 83205EF8: 3889B734  addi r4, r9, -0x48cc
	ctx.r[4].s64 = ctx.r[9].s64 + -18636;
	// 83205EFC: 38E0019A  li r7, 0x19a
	ctx.r[7].s64 = 410;
	// 83205F00: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83205F04: 4BF03EAD  bl 0x83109db0
	ctx.lr = 0x83205F08;
	sub_83109DB0(ctx, base);
	// 83205F08: 48000084  b 0x83205f8c
	pc = 0x83205F8C; continue 'dispatch;
	// 83205F0C: 8161006C  lwz r11, 0x6c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(108 as u32) ) } as u64;
	// 83205F10: 81410068  lwz r10, 0x68(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 83205F14: 81210064  lwz r9, 0x64(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 83205F18: 81010060  lwz r8, 0x60(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 83205F1C: 806400AC  lwz r3, 0xac(r4)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(172 as u32) ) } as u64;
	// 83205F20: E8EB0000  ld r7, 0(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 83205F24: E8CA0000  ld r6, 0(r10)
	ctx.r[6].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	// 83205F28: E8A90000  ld r5, 0(r9)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) };
	// 83205F2C: E8880000  ld r4, 0(r8)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) };
	// 83205F30: 4BFF1259  bl 0x831f7188
	ctx.lr = 0x83205F34;
	sub_831F7188(ctx, base);
	// 83205F34: 48000054  b 0x83205f88
	pc = 0x83205F88; continue 'dispatch;
	// 83205F38: 81610068  lwz r11, 0x68(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 83205F3C: 81410064  lwz r10, 0x64(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 83205F40: 81210060  lwz r9, 0x60(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 83205F44: 806400AC  lwz r3, 0xac(r4)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(172 as u32) ) } as u64;
	// 83205F48: E8CB0000  ld r6, 0(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 83205F4C: E8AA0000  ld r5, 0(r10)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	// 83205F50: E8890000  ld r4, 0(r9)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) };
	// 83205F54: 4BFF1215  bl 0x831f7168
	ctx.lr = 0x83205F58;
	sub_831F7168(ctx, base);
	// 83205F58: 48000030  b 0x83205f88
	pc = 0x83205F88; continue 'dispatch;
	// 83205F5C: 81610064  lwz r11, 0x64(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 83205F60: 81410060  lwz r10, 0x60(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 83205F64: 806400AC  lwz r3, 0xac(r4)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(172 as u32) ) } as u64;
	// 83205F68: E8AB0000  ld r5, 0(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 83205F6C: E88A0000  ld r4, 0(r10)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	// 83205F70: 4BFF11D9  bl 0x831f7148
	ctx.lr = 0x83205F74;
	sub_831F7148(ctx, base);
	// 83205F74: 48000014  b 0x83205f88
	pc = 0x83205F88; continue 'dispatch;
	// 83205F78: 81610060  lwz r11, 0x60(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 83205F7C: 806400AC  lwz r3, 0xac(r4)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(172 as u32) ) } as u64;
	// 83205F80: E88B0000  ld r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 83205F84: 4BFF11A5  bl 0x831f7128
	ctx.lr = 0x83205F88;
	sub_831F7128(ctx, base);
	// 83205F88: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83205F8C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 83205F90: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83205F94: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83205F98: 4BFFFBD1  bl 0x83205b68
	ctx.lr = 0x83205F9C;
	sub_83205B68(ctx, base);
	// 83205F9C: 397D0020  addi r11, r29, 0x20
	ctx.r[11].s64 = ctx.r[29].s64 + 32;
	// 83205FA0: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 83205FA4: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83205FA8: 7D4BF12E  stwx r10, r11, r30
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32), ctx.r[10].u32) };
	// 83205FAC: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 83205FB0: 4BAA34A4  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83205FB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83205FB8 size=160
    let mut pc: u32 = 0x83205FB8;
    'dispatch: loop {
        match pc {
            0x83205FB8 => {
    //   block [0x83205FB8..0x83206058)
	// 83205FB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83205FBC: 4BAA344D  bl 0x82ca9408
	ctx.lr = 0x83205FC0;
	sub_82CA93D0(ctx, base);
	// 83205FC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83205FC4: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 83205FC8: 817C00E4  lwz r11, 0xe4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(228 as u32) ) } as u64;
	// 83205FCC: 556B003C  rlwinm r11, r11, 0, 0, 0x1e
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 83205FD0: 917C00E4  stw r11, 0xe4(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(228 as u32), ctx.r[11].u32 ) };
	// 83205FD4: 4BFFD115  bl 0x832030e8
	ctx.lr = 0x83205FD8;
	sub_832030E8(ctx, base);
	// 83205FD8: 817C00E4  lwz r11, 0xe4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(228 as u32) ) } as u64;
	// 83205FDC: 556BFFFF  rlwinm. r11, r11, 0x1f, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83205FE0: 41820070  beq 0x83206050
	if ctx.cr[0].eq {
	pc = 0x83206050; continue 'dispatch;
	}
	// 83205FE4: 817C03B8  lwz r11, 0x3b8(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(952 as u32) ) } as u64;
	// 83205FE8: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 83205FEC: 816B0AB0  lwz r11, 0xab0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2736 as u32) ) } as u64;
	// 83205FF0: 83EB0060  lwz r31, 0x60(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(96 as u32) ) } as u64;
	// 83205FF4: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83205FF8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83205FFC: 419A0054  beq cr6, 0x83206050
	if ctx.cr[6].eq {
	pc = 0x83206050; continue 'dispatch;
	}
	// 83206000: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 83206004: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 83206008: 40980010  bge cr6, 0x83206018
	if !ctx.cr[6].lt {
	pc = 0x83206018; continue 'dispatch;
	}
	// 8320600C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83206010: 7C6BEA14  add r3, r11, r29
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 83206014: 48000010  b 0x83206024
	pc = 0x83206024; continue 'dispatch;
	// 83206018: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8320601C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83206020: 4BFFD381  bl 0x832033a0
	ctx.lr = 0x83206024;
	sub_832033A0(ctx, base);
	// 83206024: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83206028: 7F0BE040  cmplw cr6, r11, r28
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[28].u32, &mut ctx.xer);
	// 8320602C: 409A0010  bne cr6, 0x8320603c
	if !ctx.cr[6].eq {
	pc = 0x8320603C; continue 'dispatch;
	}
	// 83206030: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83206034: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83206038: 4BFFD2A9  bl 0x832032e0
	ctx.lr = 0x8320603C;
	sub_832032E0(ctx, base);
	// 8320603C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83206040: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 83206044: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 83206048: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8320604C: 4198FFC0  blt cr6, 0x8320600c
	if ctx.cr[6].lt {
	pc = 0x8320600C; continue 'dispatch;
	}
	// 83206050: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83206054: 4BAA3404  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83206058(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83206058 size=92
    let mut pc: u32 = 0x83206058;
    'dispatch: loop {
        match pc {
            0x83206058 => {
    //   block [0x83206058..0x832060B4)
	// 83206058: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8320605C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83206060: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83206064: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83206068: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8320606C: 817F0050  lwz r11, 0x50(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 83206070: 2F0B000B  cmpwi cr6, r11, 0xb
	ctx.cr[6].compare_i32(ctx.r[11].s32, 11, &mut ctx.xer);
	// 83206074: 409A0018  bne cr6, 0x8320608c
	if !ctx.cr[6].eq {
	pc = 0x8320608C; continue 'dispatch;
	}
	// 83206078: 817F03B8  lwz r11, 0x3b8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(952 as u32) ) } as u64;
	// 8320607C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83206080: 816B0AB0  lwz r11, 0xab0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2736 as u32) ) } as u64;
	// 83206084: 806B00AC  lwz r3, 0xac(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(172 as u32) ) } as u64;
	// 83206088: 4BFEF079  bl 0x831f5100
	ctx.lr = 0x8320608C;
	sub_831F5100(ctx, base);
	// 8320608C: 817F00E4  lwz r11, 0xe4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(228 as u32) ) } as u64;
	// 83206090: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83206094: 556B003C  rlwinm r11, r11, 0, 0, 0x1e
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 83206098: 917F00E4  stw r11, 0xe4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(228 as u32), ctx.r[11].u32 ) };
	// 8320609C: 4BFFD04D  bl 0x832030e8
	ctx.lr = 0x832060A0;
	sub_832030E8(ctx, base);
	// 832060A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832060A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832060A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832060AC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832060B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832060B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832060B8 size=56
    let mut pc: u32 = 0x832060B8;
    'dispatch: loop {
        match pc {
            0x832060B8 => {
    //   block [0x832060B8..0x832060F0)
	// 832060B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832060BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832060C0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832060C4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832060C8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 832060CC: 4BFFD01D  bl 0x832030e8
	ctx.lr = 0x832060D0;
	sub_832030E8(ctx, base);
	// 832060D0: 817F00E4  lwz r11, 0xe4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(228 as u32) ) } as u64;
	// 832060D4: 556B003C  rlwinm r11, r11, 0, 0, 0x1e
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 832060D8: 917F00E4  stw r11, 0xe4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(228 as u32), ctx.r[11].u32 ) };
	// 832060DC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832060E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832060E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832060E8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832060EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832060F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832060F0 size=64
    let mut pc: u32 = 0x832060F0;
    'dispatch: loop {
        match pc {
            0x832060F0 => {
    //   block [0x832060F0..0x83206130)
	// 832060F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832060F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832060F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832060FC: 81630018  lwz r11, 0x18(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 83206100: 2F0B0089  cmpwi cr6, r11, 0x89
	ctx.cr[6].compare_i32(ctx.r[11].s32, 137, &mut ctx.xer);
	// 83206104: 409A0014  bne cr6, 0x83206118
	if !ctx.cr[6].eq {
	pc = 0x83206118; continue 'dispatch;
	}
	// 83206108: 4BFF2261  bl 0x831f8368
	ctx.lr = 0x8320610C;
	sub_831F8368(ctx, base);
	// 8320610C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83206110: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83206114: 40820008  bne 0x8320611c
	if !ctx.cr[0].eq {
	pc = 0x8320611C; continue 'dispatch;
	}
	// 83206118: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8320611C: 5563063E  clrlwi r3, r11, 0x18
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 83206120: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83206124: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83206128: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8320612C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83206130(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83206130 size=36
    let mut pc: u32 = 0x83206130;
    'dispatch: loop {
        match pc {
            0x83206130 => {
    //   block [0x83206130..0x83206154)
	// 83206130: 816303BC  lwz r11, 0x3bc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(956 as u32) ) } as u64;
	// 83206134: 81440868  lwz r10, 0x868(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(2152 as u32) ) } as u64;
	// 83206138: 7D6A5851  subf. r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8320613C: 40800008  bge 0x83206144
	if !ctx.cr[0].lt {
	pc = 0x83206144; continue 'dispatch;
	}
	// 83206140: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83206144: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 83206148: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 8320614C: 5563DFFE  rlwinm r3, r11, 0x1b, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 83206150: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83206158(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83206158 size=28
    let mut pc: u32 = 0x83206158;
    'dispatch: loop {
        match pc {
            0x83206158 => {
    //   block [0x83206158..0x83206174)
	// 83206158: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 8320615C: 409A0018  bne cr6, 0x83206174
	if !ctx.cr[6].eq {
		sub_83206174(ctx, base);
		return;
	}
	// 83206160: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 83206164: 81430080  lwz r10, 0x80(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(128 as u32) ) } as u64;
	// 83206168: 396BAF60  addi r11, r11, -0x50a0
	ctx.r[11].s64 = ctx.r[11].s64 + -20640;
	// 8320616C: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83206170: 48000018  b 0x83206188
	sub_83206174(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83206174(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83206174 size=40
    let mut pc: u32 = 0x83206174;
    'dispatch: loop {
        match pc {
            0x83206174 => {
    //   block [0x83206174..0x8320619C)
	// 83206174: 39640020  addi r11, r4, 0x20
	ctx.r[11].s64 = ctx.r[4].s64 + 32;
	// 83206178: 3D408209  lis r10, -0x7df7
	ctx.r[10].s64 = -2113339392;
	// 8320617C: 5569103A  slwi r9, r11, 2
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 83206180: 816AAF60  lwz r11, -0x50a0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-20640 as u32) ) } as u64;
	// 83206184: 7D49182E  lwzx r10, r9, r3
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[3].u32)) } as u64;
	// 83206188: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 8320618C: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 83206190: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 83206194: 69630001  xori r3, r11, 1
	ctx.r[3].u64 = ctx.r[11].u64 ^ 1;
	// 83206198: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832061A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832061A0 size=52
    let mut pc: u32 = 0x832061A0;
    'dispatch: loop {
        match pc {
            0x832061A0 => {
    //   block [0x832061A0..0x832061D4)
	// 832061A0: 81640AB0  lwz r11, 0xab0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(2736 as u32) ) } as u64;
	// 832061A4: 896B0860  lbz r11, 0x860(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(2144 as u32) ) } as u64;
	// 832061A8: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 832061AC: 4182008C  beq 0x83206238
	if ctx.cr[0].eq {
		sub_83206238(ctx, base);
		return;
	}
	// 832061B0: 2F030014  cmpwi cr6, r3, 0x14
	ctx.cr[6].compare_i32(ctx.r[3].s32, 20, &mut ctx.xer);
	// 832061B4: 41990034  bgt cr6, 0x832061e8
	if ctx.cr[6].gt {
		sub_832061E8(ctx, base);
		return;
	}
	// 832061B8: 2F030013  cmpwi cr6, r3, 0x13
	ctx.cr[6].compare_i32(ctx.r[3].s32, 19, &mut ctx.xer);
	// 832061BC: 40980054  bge cr6, 0x83206210
	if !ctx.cr[6].lt {
		sub_832061E8(ctx, base);
		return;
	}
	// 832061C0: 2F030004  cmpwi cr6, r3, 4
	ctx.cr[6].compare_i32(ctx.r[3].s32, 4, &mut ctx.xer);
	// 832061C4: 419A004C  beq cr6, 0x83206210
	if ctx.cr[6].eq {
		sub_832061E8(ctx, base);
		return;
	}
	// 832061C8: 40990068  ble cr6, 0x83206230
	if !ctx.cr[6].gt {
		sub_83206230(ctx, base);
		return;
	}
	// 832061CC: 2F030007  cmpwi cr6, r3, 7
	ctx.cr[6].compare_i32(ctx.r[3].s32, 7, &mut ctx.xer);
	// 832061D0: 4C990020  blelr cr6
	if !ctx.cr[6].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832061D4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832061D4 size=16
    let mut pc: u32 = 0x832061D4;
    'dispatch: loop {
        match pc {
            0x832061D4 => {
    //   block [0x832061D4..0x832061E4)
	// 832061D4: 2F030008  cmpwi cr6, r3, 8
	ctx.cr[6].compare_i32(ctx.r[3].s32, 8, &mut ctx.xer);
	// 832061D8: 419A0040  beq cr6, 0x83206218
	if ctx.cr[6].eq {
		sub_83206218(ctx, base);
		return;
	}
	// 832061DC: 2F030009  cmpwi cr6, r3, 9
	ctx.cr[6].compare_i32(ctx.r[3].s32, 9, &mut ctx.xer);
	// 832061E0: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832061E4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832061E4 size=4
    let mut pc: u32 = 0x832061E4;
    'dispatch: loop {
        match pc {
            0x832061E4 => {
    //   block [0x832061E4..0x832061E8)
	// 832061E4: 4800004C  b 0x83206230
	sub_83206230(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832061E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832061E8 size=48
    let mut pc: u32 = 0x832061E8;
    'dispatch: loop {
        match pc {
            0x832061E8 => {
    //   block [0x832061E8..0x83206218)
	// 832061E8: 2F030021  cmpwi cr6, r3, 0x21
	ctx.cr[6].compare_i32(ctx.r[3].s32, 33, &mut ctx.xer);
	// 832061EC: 41990034  bgt cr6, 0x83206220
	if ctx.cr[6].gt {
		sub_83206220(ctx, base);
		return;
	}
	// 832061F0: 2F030020  cmpwi cr6, r3, 0x20
	ctx.cr[6].compare_i32(ctx.r[3].s32, 32, &mut ctx.xer);
	// 832061F4: 4098001C  bge cr6, 0x83206210
	if !ctx.cr[6].lt {
	pc = 0x83206210; continue 'dispatch;
	}
	// 832061F8: 2F030015  cmpwi cr6, r3, 0x15
	ctx.cr[6].compare_i32(ctx.r[3].s32, 21, &mut ctx.xer);
	// 832061FC: 41980034  blt cr6, 0x83206230
	if ctx.cr[6].lt {
		sub_83206230(ctx, base);
		return;
	}
	// 83206200: 2F030016  cmpwi cr6, r3, 0x16
	ctx.cr[6].compare_i32(ctx.r[3].s32, 22, &mut ctx.xer);
	// 83206204: 40990014  ble cr6, 0x83206218
	if !ctx.cr[6].gt {
		sub_83206218(ctx, base);
		return;
	}
	// 83206208: 2F030018  cmpwi cr6, r3, 0x18
	ctx.cr[6].compare_i32(ctx.r[3].s32, 24, &mut ctx.xer);
	// 8320620C: 41990024  bgt cr6, 0x83206230
	if ctx.cr[6].gt {
		sub_83206230(ctx, base);
		return;
	}
	// 83206210: 38600004  li r3, 4
	ctx.r[3].s64 = 4;
	// 83206214: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83206218(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83206218 size=8
    let mut pc: u32 = 0x83206218;
    'dispatch: loop {
        match pc {
            0x83206218 => {
    //   block [0x83206218..0x83206220)
	// 83206218: 38600008  li r3, 8
	ctx.r[3].s64 = 8;
	// 8320621C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83206220(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83206220 size=16
    let mut pc: u32 = 0x83206220;
    'dispatch: loop {
        match pc {
            0x83206220 => {
    //   block [0x83206220..0x83206230)
	// 83206220: 2F03002B  cmpwi cr6, r3, 0x2b
	ctx.cr[6].compare_i32(ctx.r[3].s32, 43, &mut ctx.xer);
	// 83206224: 4198000C  blt cr6, 0x83206230
	if ctx.cr[6].lt {
		sub_83206230(ctx, base);
		return;
	}
	// 83206228: 2F03002C  cmpwi cr6, r3, 0x2c
	ctx.cr[6].compare_i32(ctx.r[3].s32, 44, &mut ctx.xer);
	// 8320622C: 4C990020  blelr cr6
	if !ctx.cr[6].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83206230(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83206230 size=8
    let mut pc: u32 = 0x83206230;
    'dispatch: loop {
        match pc {
            0x83206230 => {
    //   block [0x83206230..0x83206238)
	// 83206230: 38600030  li r3, 0x30
	ctx.r[3].s64 = 48;
	// 83206234: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83206238(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83206238 size=20
    let mut pc: u32 = 0x83206238;
    'dispatch: loop {
        match pc {
            0x83206238 => {
    //   block [0x83206238..0x8320624C)
	// 83206238: 2F030007  cmpwi cr6, r3, 7
	ctx.cr[6].compare_i32(ctx.r[3].s32, 7, &mut ctx.xer);
	// 8320623C: 419A0010  beq cr6, 0x8320624c
	if ctx.cr[6].eq {
		sub_8320624C(ctx, base);
		return;
	}
	// 83206240: 2F030008  cmpwi cr6, r3, 8
	ctx.cr[6].compare_i32(ctx.r[3].s32, 8, &mut ctx.xer);
	// 83206244: 419AFFD4  beq cr6, 0x83206218
	if ctx.cr[6].eq {
		sub_83206218(ctx, base);
		return;
	}
	// 83206248: 4BFFFFE8  b 0x83206230
	sub_83206230(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8320624C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8320624C size=8
    let mut pc: u32 = 0x8320624C;
    'dispatch: loop {
        match pc {
            0x8320624C => {
    //   block [0x8320624C..0x83206254)
	// 8320624C: 38600007  li r3, 7
	ctx.r[3].s64 = 7;
	// 83206250: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83206258(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83206258 size=200
    let mut pc: u32 = 0x83206258;
    'dispatch: loop {
        match pc {
            0x83206258 => {
    //   block [0x83206258..0x83206320)
	// 83206258: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8320625C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83206260: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83206264: 3944003A  addi r10, r4, 0x3a
	ctx.r[10].s64 = ctx.r[4].s64 + 58;
	// 83206268: 7C691B78  mr r9, r3
	ctx.r[9].u64 = ctx.r[3].u64;
	// 8320626C: 5548103A  slwi r8, r10, 2
	ctx.r[8].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 83206270: 7CAB2B78  mr r11, r5
	ctx.r[11].u64 = ctx.r[5].u64;
	// 83206274: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83206278: 7C68482E  lwzx r3, r8, r9
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 8320627C: 4800002C  b 0x832062a8
	pc = 0x832062A8; continue 'dispatch;
	// 83206280: 554A063F  clrlwi. r10, r10, 0x18
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 83206284: 4082002C  bne 0x832062b0
	if !ctx.cr[0].eq {
	pc = 0x832062B0; continue 'dispatch;
	}
	// 83206288: 814B00E4  lwz r10, 0xe4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(228 as u32) ) } as u64;
	// 8320628C: 554807FF  clrlwi. r8, r10, 0x1f
	ctx.r[8].u64 = ctx.r[10].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 83206290: 4182000C  beq 0x8320629c
	if ctx.cr[0].eq {
	pc = 0x8320629C; continue 'dispatch;
	}
	// 83206294: 7F035840  cmplw cr6, r3, r11
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[11].u32, &mut ctx.xer);
	// 83206298: 419A002C  beq cr6, 0x832062c4
	if ctx.cr[6].eq {
	pc = 0x832062C4; continue 'dispatch;
	}
	// 8320629C: 7D4A50F8  nor r10, r10, r10
	ctx.r[10].u64 = !(ctx.r[10].u64 | ctx.r[10].u64);
	// 832062A0: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 832062A4: 554AF7FE  rlwinm r10, r10, 0x1e, 0x1f, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x00000003u64;
	// 832062A8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 832062AC: 409AFFD4  bne cr6, 0x83206280
	if !ctx.cr[6].eq {
	pc = 0x83206280; continue 'dispatch;
	}
	// 832062B0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 832062B4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832062B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832062BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832062C0: 4E800020  blr
	return;
	// 832062C4: 81690014  lwz r11, 0x14(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(20 as u32) ) } as u64;
	// 832062C8: 7F045800  cmpw cr6, r4, r11
	ctx.cr[6].compare_i32(ctx.r[4].s32, ctx.r[11].s32, &mut ctx.xer);
	// 832062CC: 409A0010  bne cr6, 0x832062dc
	if !ctx.cr[6].eq {
	pc = 0x832062DC; continue 'dispatch;
	}
	// 832062D0: 816900E4  lwz r11, 0xe4(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(228 as u32) ) } as u64;
	// 832062D4: 556BBFFF  rlwinm. r11, r11, 0x17, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000001FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 832062D8: 40820040  bne 0x83206318
	if !ctx.cr[0].eq {
	pc = 0x83206318; continue 'dispatch;
	}
	// 832062DC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 832062E0: 816B0034  lwz r11, 0x34(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 832062E4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 832062E8: 4E800421  bctrl
	ctx.lr = 0x832062EC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 832062EC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 832062F0: 40820028  bne 0x83206318
	if !ctx.cr[0].eq {
	pc = 0x83206318; continue 'dispatch;
	}
	// 832062F4: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 832062F8: 3D408209  lis r10, -0x7df7
	ctx.r[10].s64 = -2113339392;
	// 832062FC: 3D208208  lis r9, -0x7df8
	ctx.r[9].s64 = -2113404928;
	// 83206300: 38CBAF78  addi r6, r11, -0x5088
	ctx.r[6].s64 = ctx.r[11].s64 + -20616;
	// 83206304: 38AAB078  addi r5, r10, -0x4f88
	ctx.r[5].s64 = ctx.r[10].s64 + -20360;
	// 83206308: 3889B734  addi r4, r9, -0x48cc
	ctx.r[4].s64 = ctx.r[9].s64 + -18636;
	// 8320630C: 38E00287  li r7, 0x287
	ctx.r[7].s64 = 647;
	// 83206310: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83206314: 4BF03A9D  bl 0x83109db0
	ctx.lr = 0x83206318;
	sub_83109DB0(ctx, base);
	// 83206318: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8320631C: 4BFFFF98  b 0x832062b4
	pc = 0x832062B4; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83206320(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83206320 size=44
    let mut pc: u32 = 0x83206320;
    'dispatch: loop {
        match pc {
            0x83206320 => {
    //   block [0x83206320..0x8320634C)
	// 83206320: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83206324: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83206328: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 8320632C: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 83206330: 814B00E4  lwz r10, 0xe4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(228 as u32) ) } as u64;
	// 83206334: 90CB0038  stw r6, 0x38(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(56 as u32), ctx.r[6].u32 ) };
	// 83206338: 614A0042  ori r10, r10, 0x42
	ctx.r[10].u64 = ctx.r[10].u64 | 66;
	// 8320633C: 90AB0050  stw r5, 0x50(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(80 as u32), ctx.r[5].u32 ) };
	// 83206340: 912B0010  stw r9, 0x10(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[9].u32 ) };
	// 83206344: 914B00E4  stw r10, 0xe4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(228 as u32), ctx.r[10].u32 ) };
	// 83206348: 4BFDFA60  b 0x831e5da8
	sub_831E5DA8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83206350(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83206350 size=28
    let mut pc: u32 = 0x83206350;
    'dispatch: loop {
        match pc {
            0x83206350 => {
    //   block [0x83206350..0x8320636C)
	// 83206350: 548B063E  clrlwi r11, r4, 0x18
	ctx.r[11].u64 = ctx.r[4].u32 as u64 & 0x000000FFu64;
	// 83206354: 908300A8  stw r4, 0xa8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(168 as u32), ctx.r[4].u32 ) };
	// 83206358: 99630084  stb r11, 0x84(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(132 as u32), ctx.r[11].u8 ) };
	// 8320635C: 99630085  stb r11, 0x85(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(133 as u32), ctx.r[11].u8 ) };
	// 83206360: 99630086  stb r11, 0x86(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(134 as u32), ctx.r[11].u8 ) };
	// 83206364: 99630087  stb r11, 0x87(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(135 as u32), ctx.r[11].u8 ) };
	// 83206368: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83206370(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83206370 size=120
    let mut pc: u32 = 0x83206370;
    'dispatch: loop {
        match pc {
            0x83206370 => {
    //   block [0x83206370..0x832063E8)
	// 83206370: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83206374: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83206378: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8320637C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83206380: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83206384: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83206388: 807E00B4  lwz r3, 0xb4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(180 as u32) ) } as u64;
	// 8320638C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83206390: 409A0040  bne cr6, 0x832063d0
	if !ctx.cr[6].eq {
	pc = 0x832063D0; continue 'dispatch;
	}
	// 83206394: 83FE00EC  lwz r31, 0xec(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(236 as u32) ) } as u64;
	// 83206398: 48000024  b 0x832063bc
	pc = 0x832063BC; continue 'dispatch;
	// 8320639C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 832063A0: 816B0044  lwz r11, 0x44(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(68 as u32) ) } as u64;
	// 832063A4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 832063A8: 4E800421  bctrl
	ctx.lr = 0x832063AC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 832063AC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 832063B0: 41820018  beq 0x832063c8
	if ctx.cr[0].eq {
	pc = 0x832063C8; continue 'dispatch;
	}
	// 832063B4: 817F00EC  lwz r11, 0xec(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(236 as u32) ) } as u64;
	// 832063B8: 83EB00EC  lwz r31, 0xec(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(236 as u32) ) } as u64;
	// 832063BC: 807F00EC  lwz r3, 0xec(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(236 as u32) ) } as u64;
	// 832063C0: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 832063C4: 409AFFD8  bne cr6, 0x8320639c
	if !ctx.cr[6].eq {
	pc = 0x8320639C; continue 'dispatch;
	}
	// 832063C8: 93FE00B4  stw r31, 0xb4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(180 as u32), ctx.r[31].u32 ) };
	// 832063CC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832063D0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 832063D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832063D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832063DC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 832063E0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832063E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832063E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x832063E8 size=140
    let mut pc: u32 = 0x832063E8;
    'dispatch: loop {
        match pc {
            0x832063E8 => {
    //   block [0x832063E8..0x83206474)
	// 832063E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832063EC: 4BAA301D  bl 0x82ca9408
	ctx.lr = 0x832063F0;
	sub_82CA93D0(ctx, base);
	// 832063F0: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832063F4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 832063F8: 3D40831D  lis r10, -0x7ce3
	ctx.r[10].s64 = -2095251456;
	// 832063FC: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 83206400: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 83206404: 38EACC70  addi r7, r10, -0x3390
	ctx.r[7].s64 = ctx.r[10].s64 + -13200;
	// 83206408: 817F03B8  lwz r11, 0x3b8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(952 as u32) ) } as u64;
	// 8320640C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83206410: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 83206414: 809F00AC  lwz r4, 0xac(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(172 as u32) ) } as u64;
	// 83206418: 7D685B78  mr r8, r11
	ctx.r[8].u64 = ctx.r[11].u64;
	// 8320641C: 80CB0568  lwz r6, 0x568(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1384 as u32) ) } as u64;
	// 83206420: 806B0600  lwz r3, 0x600(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1536 as u32) ) } as u64;
	// 83206424: 4BFDC17D  bl 0x831e25a0
	ctx.lr = 0x83206428;
	sub_831E25A0(ctx, base);
	// 83206428: 3D60831D  lis r11, -0x7ce3
	ctx.r[11].s64 = -2095251456;
	// 8320642C: 809F00AC  lwz r4, 0xac(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(172 as u32) ) } as u64;
	// 83206430: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 83206434: 38EBCC70  addi r7, r11, -0x3390
	ctx.r[7].s64 = ctx.r[11].s64 + -13200;
	// 83206438: 817F03B8  lwz r11, 0x3b8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(952 as u32) ) } as u64;
	// 8320643C: 7D685B78  mr r8, r11
	ctx.r[8].u64 = ctx.r[11].u64;
	// 83206440: 80CB0568  lwz r6, 0x568(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1384 as u32) ) } as u64;
	// 83206444: 806B0600  lwz r3, 0x600(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1536 as u32) ) } as u64;
	// 83206448: 4BFDC5F9  bl 0x831e2a40
	ctx.lr = 0x8320644C;
	sub_831E2A40(ctx, base);
	// 8320644C: 81610070  lwz r11, 0x70(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	// 83206450: 81410074  lwz r10, 0x74(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 83206454: 7D6B1670  srawi r11, r11, 2
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 2) as i64;
	// 83206458: 7D6B0194  addze r11, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[11].s64 = tmp.s64;
	// 8320645C: 915E0000  stw r10, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83206460: 815F00A8  lwz r10, 0xa8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(168 as u32) ) } as u64;
	// 83206464: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 83206468: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8320646C: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 83206470: 4BAA2FE8  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83206478(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83206478 size=296
    let mut pc: u32 = 0x83206478;
    'dispatch: loop {
        match pc {
            0x83206478 => {
    //   block [0x83206478..0x832065A0)
	// 83206478: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8320647C: 4BAA2F81  bl 0x82ca93fc
	ctx.lr = 0x83206480;
	sub_82CA93D0(ctx, base);
	// 83206480: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83206484: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83206488: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 8320648C: 7CB92B78  mr r25, r5
	ctx.r[25].u64 = ctx.r[5].u64;
	// 83206490: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 83206494: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83206498: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8320649C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 832064A0: 4E800421  bctrl
	ctx.lr = 0x832064A4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 832064A4: 3D608331  lis r11, -0x7ccf
	ctx.r[11].s64 = -2093940736;
	// 832064A8: 813F0018  lwz r9, 0x18(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 832064AC: 3D408208  lis r10, -0x7df8
	ctx.r[10].s64 = -2113404928;
	// 832064B0: 396B2850  addi r11, r11, 0x2850
	ctx.r[11].s64 = ctx.r[11].s64 + 10320;
	// 832064B4: 1D090034  mulli r8, r9, 0x34
	ctx.r[8].s64 = ctx.r[9].s64 * 52;
	// 832064B8: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 832064BC: 3D208209  lis r9, -0x7df7
	ctx.r[9].s64 = -2113339392;
	// 832064C0: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 832064C4: 3BCAB734  addi r30, r10, -0x48cc
	ctx.r[30].s64 = ctx.r[10].s64 + -18636;
	// 832064C8: 3BA9AF78  addi r29, r9, -0x5088
	ctx.r[29].s64 = ctx.r[9].s64 + -20616;
	// 832064CC: 7D68582E  lwzx r11, r8, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 832064D0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 832064D4: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 832064D8: 409A0020  bne cr6, 0x832064f8
	if !ctx.cr[6].eq {
	pc = 0x832064F8; continue 'dispatch;
	}
	// 832064DC: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 832064E0: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 832064E4: 38ABB0F8  addi r5, r11, -0x4f08
	ctx.r[5].s64 = ctx.r[11].s64 + -20232;
	// 832064E8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 832064EC: 38E00598  li r7, 0x598
	ctx.r[7].s64 = 1432;
	// 832064F0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 832064F4: 4BF038BD  bl 0x83109db0
	ctx.lr = 0x832064F8;
	sub_83109DB0(ctx, base);
	// 832064F8: 578B063F  clrlwi. r11, r28, 0x18
	ctx.r[11].u64 = ctx.r[28].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 832064FC: 4182006C  beq 0x83206568
	if ctx.cr[0].eq {
	pc = 0x83206568; continue 'dispatch;
	}
	// 83206500: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 83206504: 2F0B0034  cmpwi cr6, r11, 0x34
	ctx.cr[6].compare_i32(ctx.r[11].s32, 52, &mut ctx.xer);
	// 83206508: 419A0020  beq cr6, 0x83206528
	if ctx.cr[6].eq {
	pc = 0x83206528; continue 'dispatch;
	}
	// 8320650C: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 83206510: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 83206514: 38ABB0E0  addi r5, r11, -0x4f20
	ctx.r[5].s64 = ctx.r[11].s64 + -20256;
	// 83206518: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8320651C: 38E0059A  li r7, 0x59a
	ctx.r[7].s64 = 1434;
	// 83206520: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83206524: 4BF0388D  bl 0x83109db0
	ctx.lr = 0x83206528;
	sub_83109DB0(ctx, base);
	// 83206528: 897F009A  lbz r11, 0x9a(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(154 as u32) ) } as u64;
	// 8320652C: 39400035  li r10, 0x35
	ctx.r[10].s64 = 53;
	// 83206530: 893F0099  lbz r9, 0x99(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(153 as u32) ) } as u64;
	// 83206534: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 83206538: 915F0018  stw r10, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[10].u32 ) };
	// 8320653C: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 83206540: 997F009A  stb r11, 0x9a(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(154 as u32), ctx.r[11].u8 ) };
	// 83206544: 7F095840  cmplw cr6, r9, r11
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[11].u32, &mut ctx.xer);
	// 83206548: 419A0020  beq cr6, 0x83206568
	if ctx.cr[6].eq {
	pc = 0x83206568; continue 'dispatch;
	}
	// 8320654C: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 83206550: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 83206554: 38ABB0BC  addi r5, r11, -0x4f44
	ctx.r[5].s64 = ctx.r[11].s64 + -20292;
	// 83206558: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8320655C: 38E0059E  li r7, 0x59e
	ctx.r[7].s64 = 1438;
	// 83206560: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83206564: 4BF0384D  bl 0x83109db0
	ctx.lr = 0x83206568;
	sub_83109DB0(ctx, base);
	// 83206568: 576B063E  clrlwi r11, r27, 0x18
	ctx.r[11].u64 = ctx.r[27].u32 as u64 & 0x000000FFu64;
	// 8320656C: 2F1A0002  cmpwi cr6, r26, 2
	ctx.cr[6].compare_i32(ctx.r[26].s32, 2, &mut ctx.xer);
	// 83206570: 997F0084  stb r11, 0x84(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[11].u8 ) };
	// 83206574: 997F0085  stb r11, 0x85(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(133 as u32), ctx.r[11].u8 ) };
	// 83206578: 997F0086  stb r11, 0x86(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(134 as u32), ctx.r[11].u8 ) };
	// 8320657C: 997F0087  stb r11, 0x87(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(135 as u32), ctx.r[11].u8 ) };
	// 83206580: 409A0018  bne cr6, 0x83206598
	if !ctx.cr[6].eq {
	pc = 0x83206598; continue 'dispatch;
	}
	// 83206584: 572B063E  clrlwi r11, r25, 0x18
	ctx.r[11].u64 = ctx.r[25].u32 as u64 & 0x000000FFu64;
	// 83206588: 997F0088  stb r11, 0x88(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(136 as u32), ctx.r[11].u8 ) };
	// 8320658C: 997F0089  stb r11, 0x89(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(137 as u32), ctx.r[11].u8 ) };
	// 83206590: 997F008A  stb r11, 0x8a(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(138 as u32), ctx.r[11].u8 ) };
	// 83206594: 997F008B  stb r11, 0x8b(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(139 as u32), ctx.r[11].u8 ) };
	// 83206598: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8320659C: 4BAA2EB0  b 0x82ca944c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832065A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832065A0 size=188
    let mut pc: u32 = 0x832065A0;
    'dispatch: loop {
        match pc {
            0x832065A0 => {
    //   block [0x832065A0..0x8320665C)
	// 832065A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832065A4: 4BAA2E65  bl 0x82ca9408
	ctx.lr = 0x832065A8;
	sub_82CA93D0(ctx, base);
	// 832065A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832065AC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 832065B0: 3D408331  lis r10, -0x7ccf
	ctx.r[10].s64 = -2093940736;
	// 832065B4: 3D208208  lis r9, -0x7df8
	ctx.r[9].s64 = -2113404928;
	// 832065B8: 394A2850  addi r10, r10, 0x2850
	ctx.r[10].s64 = ctx.r[10].s64 + 10320;
	// 832065BC: 3BC9B734  addi r30, r9, -0x48cc
	ctx.r[30].s64 = ctx.r[9].s64 + -18636;
	// 832065C0: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 832065C4: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 832065C8: 1D0B0034  mulli r8, r11, 0x34
	ctx.r[8].s64 = ctx.r[11].s64 * 52;
	// 832065CC: 7D48502E  lwzx r10, r8, r10
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 832065D0: 38EBFFCB  addi r7, r11, -0x35
	ctx.r[7].s64 = ctx.r[11].s64 + -53;
	// 832065D4: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 832065D8: 7CE70034  cntlzw r7, r7
	ctx.r[7].u64 = if ctx.r[7].u32 == 0 { 32 } else { ctx.r[7].u32.leading_zeros() as u64 };
	// 832065DC: 915F0018  stw r10, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[10].u32 ) };
	// 832065E0: 3BABAF78  addi r29, r11, -0x5088
	ctx.r[29].s64 = ctx.r[11].s64 + -20616;
	// 832065E4: 54FCDFFE  rlwinm r28, r7, 0x1b, 0x1f, 0x1f
	ctx.r[28].u64 = ctx.r[7].u32 as u64 & 0x0000001Fu64;
	// 832065E8: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 832065EC: 409A0020  bne cr6, 0x8320660c
	if !ctx.cr[6].eq {
	pc = 0x8320660C; continue 'dispatch;
	}
	// 832065F0: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 832065F4: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 832065F8: 38ABB0F8  addi r5, r11, -0x4f08
	ctx.r[5].s64 = ctx.r[11].s64 + -20232;
	// 832065FC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83206600: 38E005B9  li r7, 0x5b9
	ctx.r[7].s64 = 1465;
	// 83206604: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83206608: 4BF037A9  bl 0x83109db0
	ctx.lr = 0x8320660C;
	sub_83109DB0(ctx, base);
	// 8320660C: 578B063F  clrlwi. r11, r28, 0x18
	ctx.r[11].u64 = ctx.r[28].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83206610: 41820040  beq 0x83206650
	if ctx.cr[0].eq {
	pc = 0x83206650; continue 'dispatch;
	}
	// 83206614: 897F0099  lbz r11, 0x99(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(153 as u32) ) } as u64;
	// 83206618: 895F009A  lbz r10, 0x9a(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(154 as u32) ) } as u64;
	// 8320661C: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 83206620: 419A0020  beq cr6, 0x83206640
	if ctx.cr[6].eq {
	pc = 0x83206640; continue 'dispatch;
	}
	// 83206624: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 83206628: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 8320662C: 38ABB0BC  addi r5, r11, -0x4f44
	ctx.r[5].s64 = ctx.r[11].s64 + -20292;
	// 83206630: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83206634: 38E005BC  li r7, 0x5bc
	ctx.r[7].s64 = 1468;
	// 83206638: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8320663C: 4BF03775  bl 0x83109db0
	ctx.lr = 0x83206640;
	sub_83109DB0(ctx, base);
	// 83206640: 897F009A  lbz r11, 0x9a(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(154 as u32) ) } as u64;
	// 83206644: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 83206648: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8320664C: 997F009A  stb r11, 0x9a(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(154 as u32), ctx.r[11].u8 ) };
	// 83206650: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83206654: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83206658: 4BAA2E00  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83206660(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83206660 size=116
    let mut pc: u32 = 0x83206660;
    'dispatch: loop {
        match pc {
            0x83206660 => {
    //   block [0x83206660..0x832066D4)
	// 83206660: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83206664: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83206668: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8320666C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83206670: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83206674: 3D408208  lis r10, -0x7df8
	ctx.r[10].s64 = -2113404928;
	// 83206678: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8320667C: 394A7D38  addi r10, r10, 0x7d38
	ctx.r[10].s64 = ctx.r[10].s64 + 32056;
	// 83206680: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 83206684: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83206688: 909F0018  stw r4, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[4].u32 ) };
	// 8320668C: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 83206690: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 83206694: 81450560  lwz r10, 0x560(r5)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(1376 as u32) ) } as u64;
	// 83206698: 915F00E0  stw r10, 0xe0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(224 as u32), ctx.r[10].u32 ) };
	// 8320669C: 81450560  lwz r10, 0x560(r5)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(1376 as u32) ) } as u64;
	// 832066A0: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 832066A4: 91450560  stw r10, 0x560(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(1376 as u32), ctx.r[10].u32 ) };
	// 832066A8: 917F037C  stw r11, 0x37c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(892 as u32), ctx.r[11].u32 ) };
	// 832066AC: 917F03B4  stw r11, 0x3b4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(948 as u32), ctx.r[11].u32 ) };
	// 832066B0: 90BF03B8  stw r5, 0x3b8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(952 as u32), ctx.r[5].u32 ) };
	// 832066B4: 917F03BC  stw r11, 0x3bc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(956 as u32), ctx.r[11].u32 ) };
	// 832066B8: 913F00E4  stw r9, 0xe4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(228 as u32), ctx.r[9].u32 ) };
	// 832066BC: 4BFFF355  bl 0x83205a10
	ctx.lr = 0x832066C0;
	sub_83205A10(ctx, base);
	// 832066C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832066C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832066C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832066CC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832066D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832066D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832066D8 size=176
    let mut pc: u32 = 0x832066D8;
    'dispatch: loop {
        match pc {
            0x832066D8 => {
    //   block [0x832066D8..0x83206788)
	// 832066D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832066DC: 4BAA2D31  bl 0x82ca940c
	ctx.lr = 0x832066E0;
	sub_82CA93D0(ctx, base);
	// 832066E0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832066E4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 832066E8: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 832066EC: 817F03B8  lwz r11, 0x3b8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(952 as u32) ) } as u64;
	// 832066F0: 896B0571  lbz r11, 0x571(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(1393 as u32) ) } as u64;
	// 832066F4: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 832066F8: 41820028  beq 0x83206720
	if ctx.cr[0].eq {
	pc = 0x83206720; continue 'dispatch;
	}
	// 832066FC: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 83206700: 3D408208  lis r10, -0x7df8
	ctx.r[10].s64 = -2113404928;
	// 83206704: 3D208208  lis r9, -0x7df8
	ctx.r[9].s64 = -2113404928;
	// 83206708: 38CBAF78  addi r6, r11, -0x5088
	ctx.r[6].s64 = ctx.r[11].s64 + -20616;
	// 8320670C: 38AA70B8  addi r5, r10, 0x70b8
	ctx.r[5].s64 = ctx.r[10].s64 + 28856;
	// 83206710: 3889B734  addi r4, r9, -0x48cc
	ctx.r[4].s64 = ctx.r[9].s64 + -18636;
	// 83206714: 38E000B8  li r7, 0xb8
	ctx.r[7].s64 = 184;
	// 83206718: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8320671C: 4BF03695  bl 0x83109db0
	ctx.lr = 0x83206720;
	sub_83109DB0(ctx, base);
	// 83206720: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 83206724: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 83206728: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8320672C: 388B0001  addi r4, r11, 1
	ctx.r[4].s64 = ctx.r[11].s64 + 1;
	// 83206730: 4BFFF439  bl 0x83205b68
	ctx.lr = 0x83206734;
	sub_83205B68(ctx, base);
	// 83206734: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 83206738: 3BCB0001  addi r30, r11, 1
	ctx.r[30].s64 = ctx.r[11].s64 + 1;
	// 8320673C: 93DF0014  stw r30, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[30].u32 ) };
	// 83206740: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 83206744: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83206748: 40990038  ble cr6, 0x83206780
	if !ctx.cr[6].gt {
	pc = 0x83206780; continue 'dispatch;
	}
	// 8320674C: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 83206750: 7F1D5840  cmplw cr6, r29, r11
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[11].u32, &mut ctx.xer);
	// 83206754: 409A002C  bne cr6, 0x83206780
	if !ctx.cr[6].eq {
	pc = 0x83206780; continue 'dispatch;
	}
	// 83206758: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8320675C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83206760: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83206764: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83206768: 4E800421  bctrl
	ctx.lr = 0x8320676C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8320676C: 7F1E1800  cmpw cr6, r30, r3
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[3].s32, &mut ctx.xer);
	// 83206770: 40990010  ble cr6, 0x83206780
	if !ctx.cr[6].gt {
	pc = 0x83206780; continue 'dispatch;
	}
	// 83206774: 817F00E4  lwz r11, 0xe4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(228 as u32) ) } as u64;
	// 83206778: 616B0200  ori r11, r11, 0x200
	ctx.r[11].u64 = ctx.r[11].u64 | 512;
	// 8320677C: 917F00E4  stw r11, 0xe4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(228 as u32), ctx.r[11].u32 ) };
	// 83206780: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83206784: 4BAA2CD8  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83206788(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x83206788 size=84
    let mut pc: u32 = 0x83206788;
    'dispatch: loop {
        match pc {
            0x83206788 => {
    //   block [0x83206788..0x832067DC)
	// 83206788: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8320678C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83206790: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83206794: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83206798: D0210054  stfs f1, 0x54(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 8320679C: D041005C  stfs f2, 0x5c(r1)
	tmp.f32 = (ctx.f[2].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), tmp.u32 ) };
	// 832067A0: D0610064  stfs f3, 0x64(r1)
	tmp.f32 = (ctx.f[3].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), tmp.u32 ) };
	// 832067A4: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 832067A8: D081006C  stfs f4, 0x6c(r1)
	tmp.f32 = (ctx.f[4].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), tmp.u32 ) };
	// 832067AC: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 832067B0: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 832067B4: 91610068  stw r11, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 832067B8: E8C10050  ld r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 832067BC: E8E10058  ld r7, 0x58(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 832067C0: E9010060  ld r8, 0x60(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) };
	// 832067C4: E9210068  ld r9, 0x68(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) };
	// 832067C8: 4BFFF419  bl 0x83205be0
	ctx.lr = 0x832067CC;
	sub_83205BE0(ctx, base);
	// 832067CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832067D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832067D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832067D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832067E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832067E0 size=84
    let mut pc: u32 = 0x832067E0;
    'dispatch: loop {
        match pc {
            0x832067E0 => {
    //   block [0x832067E0..0x83206834)
	// 832067E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832067E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832067E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832067EC: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 832067F0: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 832067F4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 832067F8: 91010064  stw r8, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[8].u32 ) };
	// 832067FC: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 83206800: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 83206804: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 83206808: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 8320680C: 91610068  stw r11, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 83206810: E8C10050  ld r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 83206814: E8E10058  ld r7, 0x58(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 83206818: E9010060  ld r8, 0x60(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) };
	// 8320681C: E9210068  ld r9, 0x68(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) };
	// 83206820: 4BFFF3C1  bl 0x83205be0
	ctx.lr = 0x83206824;
	sub_83205BE0(ctx, base);
	// 83206824: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83206828: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8320682C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83206830: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83206838(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83206838 size=92
    let mut pc: u32 = 0x83206838;
    'dispatch: loop {
        match pc {
            0x83206838 => {
    //   block [0x83206838..0x83206894)
	// 83206838: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8320683C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83206840: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83206844: 4BFFFE1D  bl 0x83206660
	ctx.lr = 0x83206848;
	sub_83206660(ctx, base);
	// 83206848: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 8320684C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 83206850: 392BB1E0  addi r9, r11, -0x4e20
	ctx.r[9].s64 = ctx.r[11].s64 + -20000;
	// 83206854: 91430014  stw r10, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 83206858: 39630050  addi r11, r3, 0x50
	ctx.r[11].s64 = ctx.r[3].s64 + 80;
	// 8320685C: 91430010  stw r10, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 83206860: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 83206864: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 83206868: 3920FFFF  li r9, -1
	ctx.r[9].s64 = -1;
	// 8320686C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83206870: 912BFFE8  stw r9, -0x18(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-24 as u32), ctx.r[9].u32 ) };
	// 83206874: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 83206878: 910B0000  stw r8, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 8320687C: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 83206880: 4082FFE8  bne 0x83206868
	if !ctx.cr[0].eq {
	pc = 0x83206868; continue 'dispatch;
	}
	// 83206884: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83206888: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8320688C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83206890: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83206898(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83206898 size=76
    let mut pc: u32 = 0x83206898;
    'dispatch: loop {
        match pc {
            0x83206898 => {
    //   block [0x83206898..0x832068E4)
	// 83206898: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8320689C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832068A0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832068A4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832068A8: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 832068AC: 38800031  li r4, 0x31
	ctx.r[4].s64 = 49;
	// 832068B0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 832068B4: 4BFFFF85  bl 0x83206838
	ctx.lr = 0x832068B8;
	sub_83206838(ctx, base);
	// 832068B8: 3D408209  lis r10, -0x7df7
	ctx.r[10].s64 = -2113339392;
	// 832068BC: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 832068C0: 394AB1E0  addi r10, r10, -0x4e20
	ctx.r[10].s64 = ctx.r[10].s64 + -20000;
	// 832068C4: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 832068C8: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 832068CC: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 832068D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832068D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832068D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832068DC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832068E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832068E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832068E8 size=84
    let mut pc: u32 = 0x832068E8;
    'dispatch: loop {
        match pc {
            0x832068E8 => {
    //   block [0x832068E8..0x8320693C)
	// 832068E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832068EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832068F0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832068F4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832068F8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 832068FC: 4BFFFD65  bl 0x83206660
	ctx.lr = 0x83206900;
	sub_83206660(ctx, base);
	// 83206900: 3D208209  lis r9, -0x7df7
	ctx.r[9].s64 = -2113339392;
	// 83206904: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83206908: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8320690C: 3929B248  addi r9, r9, -0x4db8
	ctx.r[9].s64 = ctx.r[9].s64 + -19896;
	// 83206910: 917F00AC  stw r11, 0xac(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(172 as u32), ctx.r[11].u32 ) };
	// 83206914: 915F0014  stw r10, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 83206918: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8320691C: 917F00B0  stw r11, 0xb0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(176 as u32), ctx.r[11].u32 ) };
	// 83206920: 917F00B4  stw r11, 0xb4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(180 as u32), ctx.r[11].u32 ) };
	// 83206924: 915F0010  stw r10, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 83206928: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8320692C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83206930: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83206934: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83206938: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83206940(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83206940 size=12
    let mut pc: u32 = 0x83206940;
    'dispatch: loop {
        match pc {
            0x83206940 => {
    //   block [0x83206940..0x8320694C)
	// 83206940: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 83206944: 386BB2B0  addi r3, r11, -0x4d50
	ctx.r[3].s64 = ctx.r[11].s64 + -19792;
	// 83206948: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83206950(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83206950 size=108
    let mut pc: u32 = 0x83206950;
    'dispatch: loop {
        match pc {
            0x83206950 => {
    //   block [0x83206950..0x832069BC)
	// 83206950: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83206954: 4BAA2AB9  bl 0x82ca940c
	ctx.lr = 0x83206958;
	sub_82CA93D0(ctx, base);
	// 83206958: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8320695C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 83206960: 38800030  li r4, 0x30
	ctx.r[4].s64 = 48;
	// 83206964: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 83206968: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8320696C: 4BFFFCF5  bl 0x83206660
	ctx.lr = 0x83206970;
	sub_83206660(ctx, base);
	// 83206970: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 83206974: 3BE00001  li r31, 1
	ctx.r[31].s64 = 1;
	// 83206978: 396BB2B8  addi r11, r11, -0x4d48
	ctx.r[11].s64 = ctx.r[11].s64 + -19784;
	// 8320697C: 93FE0010  stw r31, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[31].u32 ) };
	// 83206980: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83206984: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83206988: 817E00E4  lwz r11, 0xe4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(228 as u32) ) } as u64;
	// 8320698C: 616B0012  ori r11, r11, 0x12
	ctx.r[11].u64 = ctx.r[11].u64 | 18;
	// 83206990: 917E00E4  stw r11, 0xe4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(228 as u32), ctx.r[11].u32 ) };
	// 83206994: 807D0AB0  lwz r3, 0xab0(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(2736 as u32) ) } as u64;
	// 83206998: 4BFDF411  bl 0x831e5da8
	ctx.lr = 0x8320699C;
	sub_831E5DA8(ctx, base);
	// 8320699C: 39600030  li r11, 0x30
	ctx.r[11].s64 = 48;
	// 832069A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832069A4: 917E0018  stw r11, 0x18(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 832069A8: 93FE0014  stw r31, 0x14(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), ctx.r[31].u32 ) };
	// 832069AC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 832069B0: 915E0010  stw r10, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 832069B4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 832069B8: 4BAA2AA4  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832069C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832069C0 size=12
    let mut pc: u32 = 0x832069C0;
    'dispatch: loop {
        match pc {
            0x832069C0 => {
    //   block [0x832069C0..0x832069CC)
	// 832069C0: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 832069C4: 386BFFAC  addi r3, r11, -0x54
	ctx.r[3].s64 = ctx.r[11].s64 + -84;
	// 832069C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832069D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832069D0 size=84
    let mut pc: u32 = 0x832069D0;
    'dispatch: loop {
        match pc {
            0x832069D0 => {
    //   block [0x832069D0..0x83206A24)
	// 832069D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832069D4: 4BAA2A39  bl 0x82ca940c
	ctx.lr = 0x832069D8;
	sub_82CA93D0(ctx, base);
	// 832069D8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832069DC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 832069E0: 7CC43378  mr r4, r6
	ctx.r[4].u64 = ctx.r[6].u64;
	// 832069E4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 832069E8: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 832069EC: 4BFFFF65  bl 0x83206950
	ctx.lr = 0x832069F0;
	sub_83206950(ctx, base);
	// 832069F0: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 832069F4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832069F8: 396BB320  addi r11, r11, -0x4ce0
	ctx.r[11].s64 = ctx.r[11].s64 + -19680;
	// 832069FC: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 83206A00: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83206A04: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83206A08: 4BFFF161  bl 0x83205b68
	ctx.lr = 0x83206A0C;
	sub_83205B68(ctx, base);
	// 83206A0C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83206A10: 93BF0080  stw r29, 0x80(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(128 as u32), ctx.r[29].u32 ) };
	// 83206A14: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83206A18: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 83206A1C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83206A20: 4BAA2A3C  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83206A28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83206A28 size=12
    let mut pc: u32 = 0x83206A28;
    'dispatch: loop {
        match pc {
            0x83206A28 => {
    //   block [0x83206A28..0x83206A34)
	// 83206A28: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 83206A2C: 386BB388  addi r3, r11, -0x4c78
	ctx.r[3].s64 = ctx.r[11].s64 + -19576;
	// 83206A30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83206A38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83206A38 size=12
    let mut pc: u32 = 0x83206A38;
    'dispatch: loop {
        match pc {
            0x83206A38 => {
    //   block [0x83206A38..0x83206A44)
	// 83206A38: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 83206A3C: 386BB394  addi r3, r11, -0x4c6c
	ctx.r[3].s64 = ctx.r[11].s64 + -19564;
	// 83206A40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83206A48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83206A48 size=72
    let mut pc: u32 = 0x83206A48;
    'dispatch: loop {
        match pc {
            0x83206A48 => {
    //   block [0x83206A48..0x83206A90)
	// 83206A48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83206A4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83206A50: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83206A54: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83206A58: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 83206A5C: 38800082  li r4, 0x82
	ctx.r[4].s64 = 130;
	// 83206A60: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83206A64: 4BFFFBFD  bl 0x83206660
	ctx.lr = 0x83206A68;
	sub_83206660(ctx, base);
	// 83206A68: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 83206A6C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83206A70: 396BB3A0  addi r11, r11, -0x4c60
	ctx.r[11].s64 = ctx.r[11].s64 + -19552;
	// 83206A74: 915F0010  stw r10, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 83206A78: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83206A7C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83206A80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83206A84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83206A88: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83206A8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83206A90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83206A90 size=12
    let mut pc: u32 = 0x83206A90;
    'dispatch: loop {
        match pc {
            0x83206A90 => {
    //   block [0x83206A90..0x83206A9C)
	// 83206A90: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 83206A94: 386BB21C  addi r3, r11, -0x4de4
	ctx.r[3].s64 = ctx.r[11].s64 + -19940;
	// 83206A98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83206AA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83206AA0 size=72
    let mut pc: u32 = 0x83206AA0;
    'dispatch: loop {
        match pc {
            0x83206AA0 => {
    //   block [0x83206AA0..0x83206AE8)
	// 83206AA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83206AA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83206AA8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83206AAC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83206AB0: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 83206AB4: 38800083  li r4, 0x83
	ctx.r[4].s64 = 131;
	// 83206AB8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83206ABC: 4BFFFBA5  bl 0x83206660
	ctx.lr = 0x83206AC0;
	sub_83206660(ctx, base);
	// 83206AC0: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 83206AC4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83206AC8: 396BB408  addi r11, r11, -0x4bf8
	ctx.r[11].s64 = ctx.r[11].s64 + -19448;
	// 83206ACC: 915F0010  stw r10, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 83206AD0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83206AD4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83206AD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83206ADC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83206AE0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83206AE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83206AE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83206AE8 size=12
    let mut pc: u32 = 0x83206AE8;
    'dispatch: loop {
        match pc {
            0x83206AE8 => {
    //   block [0x83206AE8..0x83206AF4)
	// 83206AE8: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83206AEC: 386BFC00  addi r3, r11, -0x400
	ctx.r[3].s64 = ctx.r[11].s64 + -1024;
	// 83206AF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83206AF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83206AF8 size=84
    let mut pc: u32 = 0x83206AF8;
    'dispatch: loop {
        match pc {
            0x83206AF8 => {
    //   block [0x83206AF8..0x83206B4C)
	// 83206AF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83206AFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83206B00: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83206B04: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83206B08: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 83206B0C: 38800075  li r4, 0x75
	ctx.r[4].s64 = 117;
	// 83206B10: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83206B14: 4BFFFB4D  bl 0x83206660
	ctx.lr = 0x83206B18;
	sub_83206660(ctx, base);
	// 83206B18: 3D408209  lis r10, -0x7df7
	ctx.r[10].s64 = -2113339392;
	// 83206B1C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83206B20: 394AB470  addi r10, r10, -0x4b90
	ctx.r[10].s64 = ctx.r[10].s64 + -19344;
	// 83206B24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83206B28: 917F0050  stw r11, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 83206B2C: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83206B30: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 83206B34: 913F0014  stw r9, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[9].u32 ) };
	// 83206B38: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83206B3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83206B40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83206B44: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83206B48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83206B50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83206B50 size=12
    let mut pc: u32 = 0x83206B50;
    'dispatch: loop {
        match pc {
            0x83206B50 => {
    //   block [0x83206B50..0x83206B5C)
	// 83206B50: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 83206B54: 386BB4D8  addi r3, r11, -0x4b28
	ctx.r[3].s64 = ctx.r[11].s64 + -19240;
	// 83206B58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83206B60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83206B60 size=116
    let mut pc: u32 = 0x83206B60;
    'dispatch: loop {
        match pc {
            0x83206B60 => {
    //   block [0x83206B60..0x83206BD4)
	// 83206B60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83206B64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83206B68: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83206B6C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83206B70: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83206B74: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83206B78: 38800075  li r4, 0x75
	ctx.r[4].s64 = 117;
	// 83206B7C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83206B80: 4BFFFAE1  bl 0x83206660
	ctx.lr = 0x83206B84;
	sub_83206660(ctx, base);
	// 83206B84: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 83206B88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83206B8C: 392BB470  addi r9, r11, -0x4b90
	ctx.r[9].s64 = ctx.r[11].s64 + -19344;
	// 83206B90: 915F0010  stw r10, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 83206B94: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83206B98: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 83206B9C: 813E0020  lwz r9, 0x20(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 83206BA0: 913F0050  stw r9, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 83206BA4: 813E000C  lwz r9, 0xc(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 83206BA8: 913F0038  stw r9, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[9].u32 ) };
	// 83206BAC: 93DF001C  stw r30, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[30].u32 ) };
	// 83206BB0: 997E001C  stb r11, 0x1c(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(28 as u32), ctx.r[11].u8 ) };
	// 83206BB4: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 83206BB8: 915F0014  stw r10, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 83206BBC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83206BC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83206BC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83206BC8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83206BCC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83206BD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83206BD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83206BD8 size=84
    let mut pc: u32 = 0x83206BD8;
    'dispatch: loop {
        match pc {
            0x83206BD8 => {
    //   block [0x83206BD8..0x83206C2C)
	// 83206BD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83206BDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83206BE0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83206BE4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83206BE8: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 83206BEC: 38800076  li r4, 0x76
	ctx.r[4].s64 = 118;
	// 83206BF0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83206BF4: 4BFFFA6D  bl 0x83206660
	ctx.lr = 0x83206BF8;
	sub_83206660(ctx, base);
	// 83206BF8: 3D408209  lis r10, -0x7df7
	ctx.r[10].s64 = -2113339392;
	// 83206BFC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83206C00: 394AB4E8  addi r10, r10, -0x4b18
	ctx.r[10].s64 = ctx.r[10].s64 + -19224;
	// 83206C04: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 83206C08: 917F0050  stw r11, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 83206C0C: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83206C10: 913F0010  stw r9, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[9].u32 ) };
	// 83206C14: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 83206C18: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83206C1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83206C20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83206C24: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83206C28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83206C30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83206C30 size=12
    let mut pc: u32 = 0x83206C30;
    'dispatch: loop {
        match pc {
            0x83206C30 => {
    //   block [0x83206C30..0x83206C3C)
	// 83206C30: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 83206C34: 386BB550  addi r3, r11, -0x4ab0
	ctx.r[3].s64 = ctx.r[11].s64 + -19120;
	// 83206C38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83206C40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83206C40 size=112
    let mut pc: u32 = 0x83206C40;
    'dispatch: loop {
        match pc {
            0x83206C40 => {
    //   block [0x83206C40..0x83206CB0)
	// 83206C40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83206C44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83206C48: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83206C4C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83206C50: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83206C54: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83206C58: 38800076  li r4, 0x76
	ctx.r[4].s64 = 118;
	// 83206C5C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83206C60: 4BFFFA01  bl 0x83206660
	ctx.lr = 0x83206C64;
	sub_83206660(ctx, base);
	// 83206C64: 3D408209  lis r10, -0x7df7
	ctx.r[10].s64 = -2113339392;
	// 83206C68: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83206C6C: 394AB4E8  addi r10, r10, -0x4b18
	ctx.r[10].s64 = ctx.r[10].s64 + -19224;
	// 83206C70: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 83206C74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83206C78: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83206C7C: 815E0020  lwz r10, 0x20(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 83206C80: 915F0050  stw r10, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 83206C84: 815E000C  lwz r10, 0xc(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 83206C88: 915F0038  stw r10, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[10].u32 ) };
	// 83206C8C: 93DF001C  stw r30, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[30].u32 ) };
	// 83206C90: 997E001C  stb r11, 0x1c(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(28 as u32), ctx.r[11].u8 ) };
	// 83206C94: 913F0014  stw r9, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[9].u32 ) };
	// 83206C98: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83206C9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83206CA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83206CA4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83206CA8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83206CAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83206CB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83206CB0 size=92
    let mut pc: u32 = 0x83206CB0;
    'dispatch: loop {
        match pc {
            0x83206CB0 => {
    //   block [0x83206CB0..0x83206D0C)
	// 83206CB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83206CB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83206CB8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83206CBC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83206CC0: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 83206CC4: 3880007A  li r4, 0x7a
	ctx.r[4].s64 = 122;
	// 83206CC8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83206CCC: 4BFFF995  bl 0x83206660
	ctx.lr = 0x83206CD0;
	sub_83206660(ctx, base);
	// 83206CD0: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 83206CD4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 83206CD8: 396BB560  addi r11, r11, -0x4aa0
	ctx.r[11].s64 = ctx.r[11].s64 + -19104;
	// 83206CDC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83206CE0: 915F0010  stw r10, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 83206CE4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83206CE8: 913F0014  stw r9, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[9].u32 ) };
	// 83206CEC: 817F00E4  lwz r11, 0xe4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(228 as u32) ) } as u64;
	// 83206CF0: 616B0040  ori r11, r11, 0x40
	ctx.r[11].u64 = ctx.r[11].u64 | 64;
	// 83206CF4: 917F00E4  stw r11, 0xe4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(228 as u32), ctx.r[11].u32 ) };
	// 83206CF8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83206CFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83206D00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83206D04: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83206D08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83206D10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83206D10 size=12
    let mut pc: u32 = 0x83206D10;
    'dispatch: loop {
        match pc {
            0x83206D10 => {
    //   block [0x83206D10..0x83206D1C)
	// 83206D10: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 83206D14: 386BB5C8  addi r3, r11, -0x4a38
	ctx.r[3].s64 = ctx.r[11].s64 + -19000;
	// 83206D18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83206D20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83206D20 size=72
    let mut pc: u32 = 0x83206D20;
    'dispatch: loop {
        match pc {
            0x83206D20 => {
    //   block [0x83206D20..0x83206D68)
	// 83206D20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83206D24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83206D28: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83206D2C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83206D30: 38800080  li r4, 0x80
	ctx.r[4].s64 = 128;
	// 83206D34: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83206D38: 4BFFF929  bl 0x83206660
	ctx.lr = 0x83206D3C;
	sub_83206660(ctx, base);
	// 83206D3C: 3D408209  lis r10, -0x7df7
	ctx.r[10].s64 = -2113339392;
	// 83206D40: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83206D44: 394AB5D8  addi r10, r10, -0x4a28
	ctx.r[10].s64 = ctx.r[10].s64 + -18984;
	// 83206D48: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 83206D4C: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83206D50: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 83206D54: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83206D58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83206D5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83206D60: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83206D64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83206D68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83206D68 size=12
    let mut pc: u32 = 0x83206D68;
    'dispatch: loop {
        match pc {
            0x83206D68 => {
    //   block [0x83206D68..0x83206D74)
	// 83206D68: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 83206D6C: 386BB640  addi r3, r11, -0x49c0
	ctx.r[3].s64 = ctx.r[11].s64 + -18880;
	// 83206D70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83206D78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83206D78 size=72
    let mut pc: u32 = 0x83206D78;
    'dispatch: loop {
        match pc {
            0x83206D78 => {
    //   block [0x83206D78..0x83206DC0)
	// 83206D78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83206D7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83206D80: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83206D84: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83206D88: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 83206D8C: 38800089  li r4, 0x89
	ctx.r[4].s64 = 137;
	// 83206D90: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83206D94: 4BFFF8CD  bl 0x83206660
	ctx.lr = 0x83206D98;
	sub_83206660(ctx, base);
	// 83206D98: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 83206D9C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 83206DA0: 396BB650  addi r11, r11, -0x49b0
	ctx.r[11].s64 = ctx.r[11].s64 + -18864;
	// 83206DA4: 915F0010  stw r10, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 83206DA8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83206DAC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83206DB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83206DB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83206DB8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83206DBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83206DC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83206DC0 size=12
    let mut pc: u32 = 0x83206DC0;
    'dispatch: loop {
        match pc {
            0x83206DC0 => {
    //   block [0x83206DC0..0x83206DCC)
	// 83206DC0: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 83206DC4: 386BB6B8  addi r3, r11, -0x4948
	ctx.r[3].s64 = ctx.r[11].s64 + -18760;
	// 83206DC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83206DD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83206DD0 size=108
    let mut pc: u32 = 0x83206DD0;
    'dispatch: loop {
        match pc {
            0x83206DD0 => {
    //   block [0x83206DD0..0x83206E3C)
	// 83206DD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83206DD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83206DD8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83206DDC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83206DE0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83206DE4: 3880007D  li r4, 0x7d
	ctx.r[4].s64 = 125;
	// 83206DE8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83206DEC: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 83206DF0: 4BFFF871  bl 0x83206660
	ctx.lr = 0x83206DF4;
	sub_83206660(ctx, base);
	// 83206DF4: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 83206DF8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 83206DFC: 396BB6C0  addi r11, r11, -0x4940
	ctx.r[11].s64 = ctx.r[11].s64 + -18752;
	// 83206E00: 915F0010  stw r10, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 83206E04: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83206E08: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83206E0C: 817F00E4  lwz r11, 0xe4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(228 as u32) ) } as u64;
	// 83206E10: 616B0018  ori r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u64 | 24;
	// 83206E14: 917F00E4  stw r11, 0xe4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(228 as u32), ctx.r[11].u32 ) };
	// 83206E18: 807E0AB0  lwz r3, 0xab0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(2736 as u32) ) } as u64;
	// 83206E1C: 4BFDEF8D  bl 0x831e5da8
	ctx.lr = 0x83206E20;
	sub_831E5DA8(ctx, base);
	// 83206E20: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83206E24: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83206E28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83206E2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83206E30: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83206E34: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83206E38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83206E40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83206E40 size=12
    let mut pc: u32 = 0x83206E40;
    'dispatch: loop {
        match pc {
            0x83206E40 => {
    //   block [0x83206E40..0x83206E4C)
	// 83206E40: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 83206E44: 386BB728  addi r3, r11, -0x48d8
	ctx.r[3].s64 = ctx.r[11].s64 + -18648;
	// 83206E48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83206E50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83206E50 size=72
    let mut pc: u32 = 0x83206E50;
    'dispatch: loop {
        match pc {
            0x83206E50 => {
    //   block [0x83206E50..0x83206E98)
	// 83206E50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83206E54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83206E58: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83206E5C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83206E60: 3880007E  li r4, 0x7e
	ctx.r[4].s64 = 126;
	// 83206E64: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83206E68: 4BFFF7F9  bl 0x83206660
	ctx.lr = 0x83206E6C;
	sub_83206660(ctx, base);
	// 83206E6C: 3D408209  lis r10, -0x7df7
	ctx.r[10].s64 = -2113339392;
	// 83206E70: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83206E74: 394AB730  addi r10, r10, -0x48d0
	ctx.r[10].s64 = ctx.r[10].s64 + -18640;
	// 83206E78: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 83206E7C: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83206E80: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 83206E84: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83206E88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83206E8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83206E90: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83206E94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83206E98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83206E98 size=12
    let mut pc: u32 = 0x83206E98;
    'dispatch: loop {
        match pc {
            0x83206E98 => {
    //   block [0x83206E98..0x83206EA4)
	// 83206E98: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 83206E9C: 386BB798  addi r3, r11, -0x4868
	ctx.r[3].s64 = ctx.r[11].s64 + -18536;
	// 83206EA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83206EA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83206EA8 size=76
    let mut pc: u32 = 0x83206EA8;
    'dispatch: loop {
        match pc {
            0x83206EA8 => {
    //   block [0x83206EA8..0x83206EF4)
	// 83206EA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83206EAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83206EB0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83206EB4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83206EB8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83206EBC: 4BFFF7A5  bl 0x83206660
	ctx.lr = 0x83206EC0;
	sub_83206660(ctx, base);
	// 83206EC0: 3D408209  lis r10, -0x7df7
	ctx.r[10].s64 = -2113339392;
	// 83206EC4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83206EC8: 394AB7A8  addi r10, r10, -0x4858
	ctx.r[10].s64 = ctx.r[10].s64 + -18520;
	// 83206ECC: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 83206ED0: 917F00A8  stw r11, 0xa8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(168 as u32), ctx.r[11].u32 ) };
	// 83206ED4: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83206ED8: 913F0010  stw r9, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[9].u32 ) };
	// 83206EDC: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 83206EE0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83206EE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83206EE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83206EEC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83206EF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83206EF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83206EF8 size=12
    let mut pc: u32 = 0x83206EF8;
    'dispatch: loop {
        match pc {
            0x83206EF8 => {
    //   block [0x83206EF8..0x83206F04)
	// 83206EF8: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 83206EFC: 386BB878  addi r3, r11, -0x4788
	ctx.r[3].s64 = ctx.r[11].s64 + -18312;
	// 83206F00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83206F08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83206F08 size=12
    let mut pc: u32 = 0x83206F08;
    'dispatch: loop {
        match pc {
            0x83206F08 => {
    //   block [0x83206F08..0x83206F14)
	// 83206F08: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 83206F0C: 386BB8F0  addi r3, r11, -0x4710
	ctx.r[3].s64 = ctx.r[11].s64 + -18192;
	// 83206F10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83206F18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83206F18 size=12
    let mut pc: u32 = 0x83206F18;
    'dispatch: loop {
        match pc {
            0x83206F18 => {
    //   block [0x83206F18..0x83206F24)
	// 83206F18: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 83206F1C: 386BB968  addi r3, r11, -0x4698
	ctx.r[3].s64 = ctx.r[11].s64 + -18072;
	// 83206F20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83206F28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83206F28 size=12
    let mut pc: u32 = 0x83206F28;
    'dispatch: loop {
        match pc {
            0x83206F28 => {
    //   block [0x83206F28..0x83206F34)
	// 83206F28: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 83206F2C: 386BB9E0  addi r3, r11, -0x4620
	ctx.r[3].s64 = ctx.r[11].s64 + -17952;
	// 83206F30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83206F38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83206F38 size=76
    let mut pc: u32 = 0x83206F38;
    'dispatch: loop {
        match pc {
            0x83206F38 => {
    //   block [0x83206F38..0x83206F84)
	// 83206F38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83206F3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83206F40: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83206F44: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83206F48: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83206F4C: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 83206F50: 548A07FF  clrlwi. r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 83206F54: 396B7718  addi r11, r11, 0x7718
	ctx.r[11].s64 = ctx.r[11].s64 + 30488;
	// 83206F58: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83206F5C: 41820010  beq 0x83206f6c
	if ctx.cr[0].eq {
	pc = 0x83206F6C; continue 'dispatch;
	}
	// 83206F60: 389FFFFC  addi r4, r31, -4
	ctx.r[4].s64 = ctx.r[31].s64 + -4;
	// 83206F64: 807FFFFC  lwz r3, -4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-4 as u32) ) } as u64;
	// 83206F68: 4BFC5361  bl 0x831cc2c8
	ctx.lr = 0x83206F6C;
	sub_831CC2C8(ctx, base);
	// 83206F6C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83206F70: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83206F74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83206F78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83206F7C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83206F80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83206F88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83206F88 size=12
    let mut pc: u32 = 0x83206F88;
    'dispatch: loop {
        match pc {
            0x83206F88 => {
    //   block [0x83206F88..0x83206F94)
	// 83206F88: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 83206F8C: 386B07F4  addi r3, r11, 0x7f4
	ctx.r[3].s64 = ctx.r[11].s64 + 2036;
	// 83206F90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83206F98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83206F98 size=52
    let mut pc: u32 = 0x83206F98;
    'dispatch: loop {
        match pc {
            0x83206F98 => {
    //   block [0x83206F98..0x83206FCC)
	// 83206F98: 2F060004  cmpwi cr6, r6, 4
	ctx.cr[6].compare_i32(ctx.r[6].s32, 4, &mut ctx.xer);
	// 83206F9C: 41980030  blt cr6, 0x83206fcc
	if ctx.cr[6].lt {
		sub_83206FCC(ctx, base);
		return;
	}
	// 83206FA0: 2F060005  cmpwi cr6, r6, 5
	ctx.cr[6].compare_i32(ctx.r[6].s32, 5, &mut ctx.xer);
	// 83206FA4: 41990028  bgt cr6, 0x83206fcc
	if ctx.cr[6].gt {
		sub_83206FCC(ctx, base);
		return;
	}
	// 83206FA8: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 83206FAC: 3D40820A  lis r10, -0x7df6
	ctx.r[10].s64 = -2113273856;
	// 83206FB0: 3D208208  lis r9, -0x7df8
	ctx.r[9].s64 = -2113404928;
	// 83206FB4: 38CB6188  addi r6, r11, 0x6188
	ctx.r[6].s64 = ctx.r[11].s64 + 24968;
	// 83206FB8: 38AA1700  addi r5, r10, 0x1700
	ctx.r[5].s64 = ctx.r[10].s64 + 5888;
	// 83206FBC: 3889B734  addi r4, r9, -0x48cc
	ctx.r[4].s64 = ctx.r[9].s64 + -18636;
	// 83206FC0: 38E006B0  li r7, 0x6b0
	ctx.r[7].s64 = 1712;
	// 83206FC4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83206FC8: 4BF02DE8  b 0x83109db0
	sub_83109DB0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83206FCC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83206FCC size=20
    let mut pc: u32 = 0x83206FCC;
    'dispatch: loop {
        match pc {
            0x83206FCC => {
    //   block [0x83206FCC..0x83206FE0)
	// 83206FCC: 39640020  addi r11, r4, 0x20
	ctx.r[11].s64 = ctx.r[4].s64 + 32;
	// 83206FD0: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83206FD4: 7D6B2A14  add r11, r11, r5
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[5].u64;
	// 83206FD8: 7CCB19AE  stbx r6, r11, r3
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[3].u32), ctx.r[6].u8) };
	// 83206FDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83206FE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83206FE0 size=112
    let mut pc: u32 = 0x83206FE0;
    'dispatch: loop {
        match pc {
            0x83206FE0 => {
    //   block [0x83206FE0..0x83207050)
	// 83206FE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83206FE4: 4BAA2429  bl 0x82ca940c
	ctx.lr = 0x83206FE8;
	sub_82CA93D0(ctx, base);
	// 83206FE8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83206FEC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83206FF0: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 83206FF4: 7CFD3B78  mr r29, r7
	ctx.r[29].u64 = ctx.r[7].u64;
	// 83206FF8: 4BFFF669  bl 0x83206660
	ctx.lr = 0x83206FFC;
	sub_83206660(ctx, base);
	// 83206FFC: 3D408209  lis r10, -0x7df7
	ctx.r[10].s64 = -2113339392;
	// 83207000: 93BF00A8  stw r29, 0xa8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(168 as u32), ctx.r[29].u32 ) };
	// 83207004: 3D008209  lis r8, -0x7df7
	ctx.r[8].s64 = -2113339392;
	// 83207008: 93DF00AC  stw r30, 0xac(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(172 as u32), ctx.r[30].u32 ) };
	// 8320700C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83207010: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 83207014: 3908B9F0  addi r8, r8, -0x4610
	ctx.r[8].s64 = ctx.r[8].s64 + -17936;
	// 83207018: 997F00B0  stb r11, 0xb0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(176 as u32), ctx.r[11].u8 ) };
	// 8320701C: 814AAF64  lwz r10, -0x509c(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-20636 as u32) ) } as u64;
	// 83207020: 38E0001E  li r7, 0x1e
	ctx.r[7].s64 = 30;
	// 83207024: 913F0014  stw r9, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[9].u32 ) };
	// 83207028: 913F0010  stw r9, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[9].u32 ) };
	// 8320702C: 911F0000  stw r8, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 83207030: 917F00B4  stw r11, 0xb4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(180 as u32), ctx.r[11].u32 ) };
	// 83207034: 917F00B8  stw r11, 0xb8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(184 as u32), ctx.r[11].u32 ) };
	// 83207038: 917F0050  stw r11, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8320703C: 90FF0054  stw r7, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 83207040: 917F003C  stw r11, 0x3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[11].u32 ) };
	// 83207044: 915F0080  stw r10, 0x80(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(128 as u32), ctx.r[10].u32 ) };
	// 83207048: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8320704C: 4BAA2410  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83207050(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83207050 size=192
    let mut pc: u32 = 0x83207050;
    'dispatch: loop {
        match pc {
            0x83207050 => {
    //   block [0x83207050..0x83207110)
	// 83207050: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83207054: 4BAA23B5  bl 0x82ca9408
	ctx.lr = 0x83207058;
	sub_82CA93D0(ctx, base);
	// 83207058: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8320705C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83207060: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 83207064: 4BFFF5FD  bl 0x83206660
	ctx.lr = 0x83207068;
	sub_83206660(ctx, base);
	// 83207068: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 8320706C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 83207070: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 83207074: 396BBA58  addi r11, r11, -0x45a8
	ctx.r[11].s64 = ctx.r[11].s64 + -17832;
	// 83207078: 93FD00AC  stw r31, 0xac(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(172 as u32), ctx.r[31].u32 ) };
	// 8320707C: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 83207080: 93DD0014  stw r30, 0x14(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(20 as u32), ctx.r[30].u32 ) };
	// 83207084: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83207088: 3880001E  li r4, 0x1e
	ctx.r[4].s64 = 30;
	// 8320708C: 93DD0010  stw r30, 0x10(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(16 as u32), ctx.r[30].u32 ) };
	// 83207090: 915D00B0  stw r10, 0xb0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(176 as u32), ctx.r[10].u32 ) };
	// 83207094: 93FD0050  stw r31, 0x50(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 83207098: 93FD0054  stw r31, 0x54(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(84 as u32), ctx.r[31].u32 ) };
	// 8320709C: 93FD006C  stw r31, 0x6c(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(108 as u32), ctx.r[31].u32 ) };
	// 832070A0: 93FD0068  stw r31, 0x68(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(104 as u32), ctx.r[31].u32 ) };
	// 832070A4: 807C05AC  lwz r3, 0x5ac(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(1452 as u32) ) } as u64;
	// 832070A8: 4BFC50F1  bl 0x831cc198
	ctx.lr = 0x832070AC;
	sub_831CC198(ctx, base);
	// 832070AC: 38A0001E  li r5, 0x1e
	ctx.r[5].s64 = 30;
	// 832070B0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 832070B4: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 832070B8: 4BAA28F9  bl 0x82ca99b0
	ctx.lr = 0x832070BC;
	sub_82CA99B0(ctx, base);
	// 832070BC: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 832070C0: 39400007  li r10, 7
	ctx.r[10].s64 = 7;
	// 832070C4: B3DC000E  sth r30, 0xe(r28)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[28].u32.wrapping_add(14 as u32), ctx.r[30].u16 ) };
	// 832070C8: B17C0000  sth r11, 0(r28)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[11].u16 ) };
	// 832070CC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 832070D0: B17C0002  sth r11, 2(r28)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[28].u32.wrapping_add(2 as u32), ctx.r[11].u16 ) };
	// 832070D4: B17C0004  sth r11, 4(r28)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[28].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 832070D8: B15C0006  sth r10, 6(r28)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[28].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 832070DC: B15C0008  sth r10, 8(r28)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[28].u32.wrapping_add(8 as u32), ctx.r[10].u16 ) };
	// 832070E0: B17C000A  sth r11, 0xa(r28)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[28].u32.wrapping_add(10 as u32), ctx.r[11].u16 ) };
	// 832070E4: B17C000C  sth r11, 0xc(r28)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[28].u32.wrapping_add(12 as u32), ctx.r[11].u16 ) };
	// 832070E8: B3FC0010  sth r31, 0x10(r28)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[28].u32.wrapping_add(16 as u32), ctx.r[31].u16 ) };
	// 832070EC: B3DC0012  sth r30, 0x12(r28)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[28].u32.wrapping_add(18 as u32), ctx.r[30].u16 ) };
	// 832070F0: B3FC0016  sth r31, 0x16(r28)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[28].u32.wrapping_add(22 as u32), ctx.r[31].u16 ) };
	// 832070F4: B3FC0018  sth r31, 0x18(r28)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[28].u32.wrapping_add(24 as u32), ctx.r[31].u16 ) };
	// 832070F8: B3FC001A  sth r31, 0x1a(r28)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[28].u32.wrapping_add(26 as u32), ctx.r[31].u16 ) };
	// 832070FC: B3FC001C  sth r31, 0x1c(r28)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[28].u32.wrapping_add(28 as u32), ctx.r[31].u16 ) };
	// 83207100: B3FC0014  sth r31, 0x14(r28)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[28].u32.wrapping_add(20 as u32), ctx.r[31].u16 ) };
	// 83207104: 939D00B8  stw r28, 0xb8(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(184 as u32), ctx.r[28].u32 ) };
	// 83207108: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8320710C: 4BAA234C  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83207110(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83207110 size=76
    let mut pc: u32 = 0x83207110;
    'dispatch: loop {
        match pc {
            0x83207110 => {
    //   block [0x83207110..0x8320715C)
	// 83207110: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83207114: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83207118: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8320711C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83207120: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83207124: 4BFFF82D  bl 0x83206950
	ctx.lr = 0x83207128;
	sub_83206950(ctx, base);
	// 83207128: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 8320712C: 39400015  li r10, 0x15
	ctx.r[10].s64 = 21;
	// 83207130: 396BBAC0  addi r11, r11, -0x4540
	ctx.r[11].s64 = ctx.r[11].s64 + -17728;
	// 83207134: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 83207138: 915F0050  stw r10, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 8320713C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83207140: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83207144: 913F0014  stw r9, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[9].u32 ) };
	// 83207148: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8320714C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83207150: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83207154: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83207158: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83207160(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83207160 size=12
    let mut pc: u32 = 0x83207160;
    'dispatch: loop {
        match pc {
            0x83207160 => {
    //   block [0x83207160..0x8320716C)
	// 83207160: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 83207164: 386BBB28  addi r3, r11, -0x44d8
	ctx.r[3].s64 = ctx.r[11].s64 + -17624;
	// 83207168: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83207170(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83207170 size=136
    let mut pc: u32 = 0x83207170;
    'dispatch: loop {
        match pc {
            0x83207170 => {
    //   block [0x83207170..0x832071F8)
	// 83207170: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83207174: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83207178: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8320717C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83207180: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83207184: 38800086  li r4, 0x86
	ctx.r[4].s64 = 134;
	// 83207188: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8320718C: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 83207190: 4BFFF4D1  bl 0x83206660
	ctx.lr = 0x83207194;
	sub_83206660(ctx, base);
	// 83207194: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 83207198: 3D408209  lis r10, -0x7df7
	ctx.r[10].s64 = -2113339392;
	// 8320719C: 396BBB40  addi r11, r11, -0x44c0
	ctx.r[11].s64 = ctx.r[11].s64 + -17600;
	// 832071A0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 832071A4: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 832071A8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 832071AC: 913F0010  stw r9, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[9].u32 ) };
	// 832071B0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832071B4: 911F0014  stw r8, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[8].u32 ) };
	// 832071B8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 832071BC: 816AAF6C  lwz r11, -0x5094(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-20628 as u32) ) } as u64;
	// 832071C0: 917F0080  stw r11, 0x80(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 832071C4: 817F00E4  lwz r11, 0xe4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(228 as u32) ) } as u64;
	// 832071C8: 90FF0010  stw r7, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[7].u32 ) };
	// 832071CC: 616B0018  ori r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u64 | 24;
	// 832071D0: 917F00E4  stw r11, 0xe4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(228 as u32), ctx.r[11].u32 ) };
	// 832071D4: 807E0AB0  lwz r3, 0xab0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(2736 as u32) ) } as u64;
	// 832071D8: 4BFDEBD1  bl 0x831e5da8
	ctx.lr = 0x832071DC;
	sub_831E5DA8(ctx, base);
	// 832071DC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832071E0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 832071E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832071E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832071EC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 832071F0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832071F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832071F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832071F8 size=12
    let mut pc: u32 = 0x832071F8;
    'dispatch: loop {
        match pc {
            0x832071F8 => {
    //   block [0x832071F8..0x83207204)
	// 832071F8: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 832071FC: 386BBBA8  addi r3, r11, -0x4458
	ctx.r[3].s64 = ctx.r[11].s64 + -17496;
	// 83207200: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83207208(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83207208 size=116
    let mut pc: u32 = 0x83207208;
    'dispatch: loop {
        match pc {
            0x83207208 => {
    //   block [0x83207208..0x8320727C)
	// 83207208: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8320720C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83207210: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83207214: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83207218: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8320721C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83207220: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 83207224: 4BFFF43D  bl 0x83206660
	ctx.lr = 0x83207228;
	sub_83206660(ctx, base);
	// 83207228: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8320722C: 3D408209  lis r10, -0x7df7
	ctx.r[10].s64 = -2113339392;
	// 83207230: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 83207234: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83207238: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 8320723C: 394ABBB0  addi r10, r10, -0x4450
	ctx.r[10].s64 = ctx.r[10].s64 + -17488;
	// 83207240: 913F00A8  stw r9, 0xa8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(168 as u32), ctx.r[9].u32 ) };
	// 83207244: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83207248: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8320724C: 817F00E4  lwz r11, 0xe4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(228 as u32) ) } as u64;
	// 83207250: 616B0018  ori r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u64 | 24;
	// 83207254: 917F00E4  stw r11, 0xe4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(228 as u32), ctx.r[11].u32 ) };
	// 83207258: 807E0AB0  lwz r3, 0xab0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(2736 as u32) ) } as u64;
	// 8320725C: 4BFDEB4D  bl 0x831e5da8
	ctx.lr = 0x83207260;
	sub_831E5DA8(ctx, base);
	// 83207260: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83207264: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83207268: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8320726C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83207270: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83207274: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83207278: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83207280(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83207280 size=12
    let mut pc: u32 = 0x83207280;
    'dispatch: loop {
        match pc {
            0x83207280 => {
    //   block [0x83207280..0x8320728C)
	// 83207280: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 83207284: 386BBC18  addi r3, r11, -0x43e8
	ctx.r[3].s64 = ctx.r[11].s64 + -17384;
	// 83207288: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83207290(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83207290 size=120
    let mut pc: u32 = 0x83207290;
    'dispatch: loop {
        match pc {
            0x83207290 => {
    //   block [0x83207290..0x83207308)
	// 83207290: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83207294: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83207298: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8320729C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832072A0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832072A4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 832072A8: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 832072AC: 4BFFF3B5  bl 0x83206660
	ctx.lr = 0x832072B0;
	sub_83206660(ctx, base);
	// 832072B0: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 832072B4: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 832072B8: 396BBC20  addi r11, r11, -0x43e0
	ctx.r[11].s64 = ctx.r[11].s64 + -17376;
	// 832072BC: 915F0014  stw r10, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 832072C0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 832072C4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 832072C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832072CC: 913F0010  stw r9, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[9].u32 ) };
	// 832072D0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 832072D4: 915F00A8  stw r10, 0xa8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(168 as u32), ctx.r[10].u32 ) };
	// 832072D8: 817F00E4  lwz r11, 0xe4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(228 as u32) ) } as u64;
	// 832072DC: 616B0018  ori r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u64 | 24;
	// 832072E0: 917F00E4  stw r11, 0xe4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(228 as u32), ctx.r[11].u32 ) };
	// 832072E4: 807E0AB0  lwz r3, 0xab0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(2736 as u32) ) } as u64;
	// 832072E8: 4BFDEAC1  bl 0x831e5da8
	ctx.lr = 0x832072EC;
	sub_831E5DA8(ctx, base);
	// 832072EC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832072F0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 832072F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832072F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832072FC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83207300: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83207304: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83207308(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83207308 size=12
    let mut pc: u32 = 0x83207308;
    'dispatch: loop {
        match pc {
            0x83207308 => {
    //   block [0x83207308..0x83207314)
	// 83207308: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 8320730C: 386BBC88  addi r3, r11, -0x4378
	ctx.r[3].s64 = ctx.r[11].s64 + -17272;
	// 83207310: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83207318(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83207318 size=116
    let mut pc: u32 = 0x83207318;
    'dispatch: loop {
        match pc {
            0x83207318 => {
    //   block [0x83207318..0x8320738C)
	// 83207318: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8320731C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83207320: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83207324: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83207328: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8320732C: 38800088  li r4, 0x88
	ctx.r[4].s64 = 136;
	// 83207330: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83207334: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 83207338: 4BFFF329  bl 0x83206660
	ctx.lr = 0x8320733C;
	sub_83206660(ctx, base);
	// 8320733C: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 83207340: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83207344: 396BBC98  addi r11, r11, -0x4368
	ctx.r[11].s64 = ctx.r[11].s64 + -17256;
	// 83207348: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8320734C: 915F0010  stw r10, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 83207350: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83207354: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83207358: 913F0014  stw r9, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[9].u32 ) };
	// 8320735C: 817F00E4  lwz r11, 0xe4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(228 as u32) ) } as u64;
	// 83207360: 616B0018  ori r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u64 | 24;
	// 83207364: 917F00E4  stw r11, 0xe4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(228 as u32), ctx.r[11].u32 ) };
	// 83207368: 807E0AB0  lwz r3, 0xab0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(2736 as u32) ) } as u64;
	// 8320736C: 4BFDEA3D  bl 0x831e5da8
	ctx.lr = 0x83207370;
	sub_831E5DA8(ctx, base);
	// 83207370: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83207374: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83207378: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8320737C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83207380: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83207384: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83207388: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83207390(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83207390 size=12
    let mut pc: u32 = 0x83207390;
    'dispatch: loop {
        match pc {
            0x83207390 => {
    //   block [0x83207390..0x8320739C)
	// 83207390: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 83207394: 386BBD00  addi r3, r11, -0x4300
	ctx.r[3].s64 = ctx.r[11].s64 + -17152;
	// 83207398: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832073A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832073A0 size=116
    let mut pc: u32 = 0x832073A0;
    'dispatch: loop {
        match pc {
            0x832073A0 => {
    //   block [0x832073A0..0x83207414)
	// 832073A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832073A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832073A8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 832073AC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832073B0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832073B4: 38800087  li r4, 0x87
	ctx.r[4].s64 = 135;
	// 832073B8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 832073BC: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 832073C0: 4BFFF2A1  bl 0x83206660
	ctx.lr = 0x832073C4;
	sub_83206660(ctx, base);
	// 832073C4: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 832073C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832073CC: 396BBD10  addi r11, r11, -0x42f0
	ctx.r[11].s64 = ctx.r[11].s64 + -17136;
	// 832073D0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 832073D4: 915F0010  stw r10, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 832073D8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 832073DC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 832073E0: 913F0014  stw r9, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[9].u32 ) };
	// 832073E4: 817F00E4  lwz r11, 0xe4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(228 as u32) ) } as u64;
	// 832073E8: 616B0018  ori r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u64 | 24;
	// 832073EC: 917F00E4  stw r11, 0xe4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(228 as u32), ctx.r[11].u32 ) };
	// 832073F0: 807E0AB0  lwz r3, 0xab0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(2736 as u32) ) } as u64;
	// 832073F4: 4BFDE9B5  bl 0x831e5da8
	ctx.lr = 0x832073F8;
	sub_831E5DA8(ctx, base);
	// 832073F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832073FC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83207400: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83207404: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83207408: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8320740C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83207410: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83207418(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83207418 size=12
    let mut pc: u32 = 0x83207418;
    'dispatch: loop {
        match pc {
            0x83207418 => {
    //   block [0x83207418..0x83207424)
	// 83207418: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 8320741C: 386BBD78  addi r3, r11, -0x4288
	ctx.r[3].s64 = ctx.r[11].s64 + -17032;
	// 83207420: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83207428(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83207428 size=84
    let mut pc: u32 = 0x83207428;
    'dispatch: loop {
        match pc {
            0x83207428 => {
    //   block [0x83207428..0x8320747C)
	// 83207428: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8320742C: 4BAA1FE1  bl 0x82ca940c
	ctx.lr = 0x83207430;
	sub_82CA93D0(ctx, base);
	// 83207430: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83207434: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83207438: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8320743C: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 83207440: 3880008B  li r4, 0x8b
	ctx.r[4].s64 = 139;
	// 83207444: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83207448: 4BFFF219  bl 0x83206660
	ctx.lr = 0x8320744C;
	sub_83206660(ctx, base);
	// 8320744C: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 83207450: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83207454: 93DF00A8  stw r30, 0xa8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(168 as u32), ctx.r[30].u32 ) };
	// 83207458: 396BBD88  addi r11, r11, -0x4278
	ctx.r[11].s64 = ctx.r[11].s64 + -17016;
	// 8320745C: 93BF00AC  stw r29, 0xac(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(172 as u32), ctx.r[29].u32 ) };
	// 83207460: 915F0010  stw r10, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 83207464: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83207468: 817F00E4  lwz r11, 0xe4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(228 as u32) ) } as u64;
	// 8320746C: 616B0100  ori r11, r11, 0x100
	ctx.r[11].u64 = ctx.r[11].u64 | 256;
	// 83207470: 917F00E4  stw r11, 0xe4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(228 as u32), ctx.r[11].u32 ) };
	// 83207474: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83207478: 4BAA1FE4  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83207480(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83207480 size=84
    let mut pc: u32 = 0x83207480;
    'dispatch: loop {
        match pc {
            0x83207480 => {
    //   block [0x83207480..0x832074D4)
	// 83207480: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83207484: 4BAA1F89  bl 0x82ca940c
	ctx.lr = 0x83207488;
	sub_82CA93D0(ctx, base);
	// 83207488: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8320748C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83207490: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 83207494: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 83207498: 3880008A  li r4, 0x8a
	ctx.r[4].s64 = 138;
	// 8320749C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 832074A0: 4BFFF1C1  bl 0x83206660
	ctx.lr = 0x832074A4;
	sub_83206660(ctx, base);
	// 832074A4: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 832074A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832074AC: 93DF00A8  stw r30, 0xa8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(168 as u32), ctx.r[30].u32 ) };
	// 832074B0: 396BBDF0  addi r11, r11, -0x4210
	ctx.r[11].s64 = ctx.r[11].s64 + -16912;
	// 832074B4: 93BF00AC  stw r29, 0xac(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(172 as u32), ctx.r[29].u32 ) };
	// 832074B8: 915F0010  stw r10, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 832074BC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 832074C0: 817F00E4  lwz r11, 0xe4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(228 as u32) ) } as u64;
	// 832074C4: 616B0100  ori r11, r11, 0x100
	ctx.r[11].u64 = ctx.r[11].u64 | 256;
	// 832074C8: 917F00E4  stw r11, 0xe4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(228 as u32), ctx.r[11].u32 ) };
	// 832074CC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 832074D0: 4BAA1F8C  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832074D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832074D8 size=84
    let mut pc: u32 = 0x832074D8;
    'dispatch: loop {
        match pc {
            0x832074D8 => {
    //   block [0x832074D8..0x8320752C)
	// 832074D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832074DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832074E0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832074E4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832074E8: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 832074EC: 3880008D  li r4, 0x8d
	ctx.r[4].s64 = 141;
	// 832074F0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 832074F4: 4BFFF16D  bl 0x83206660
	ctx.lr = 0x832074F8;
	sub_83206660(ctx, base);
	// 832074F8: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 832074FC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83207500: 396BBE58  addi r11, r11, -0x41a8
	ctx.r[11].s64 = ctx.r[11].s64 + -16808;
	// 83207504: 915F0010  stw r10, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 83207508: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8320750C: 817F00E4  lwz r11, 0xe4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(228 as u32) ) } as u64;
	// 83207510: 616B0100  ori r11, r11, 0x100
	ctx.r[11].u64 = ctx.r[11].u64 | 256;
	// 83207514: 917F00E4  stw r11, 0xe4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(228 as u32), ctx.r[11].u32 ) };
	// 83207518: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8320751C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83207520: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83207524: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83207528: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83207530(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83207530 size=84
    let mut pc: u32 = 0x83207530;
    'dispatch: loop {
        match pc {
            0x83207530 => {
    //   block [0x83207530..0x83207584)
	// 83207530: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83207534: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83207538: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8320753C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83207540: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 83207544: 3880008C  li r4, 0x8c
	ctx.r[4].s64 = 140;
	// 83207548: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8320754C: 4BFFF115  bl 0x83206660
	ctx.lr = 0x83207550;
	sub_83206660(ctx, base);
	// 83207550: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 83207554: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83207558: 396BBEC0  addi r11, r11, -0x4140
	ctx.r[11].s64 = ctx.r[11].s64 + -16704;
	// 8320755C: 915F0010  stw r10, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 83207560: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83207564: 817F00E4  lwz r11, 0xe4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(228 as u32) ) } as u64;
	// 83207568: 616B0100  ori r11, r11, 0x100
	ctx.r[11].u64 = ctx.r[11].u64 | 256;
	// 8320756C: 917F00E4  stw r11, 0xe4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(228 as u32), ctx.r[11].u32 ) };
	// 83207570: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83207574: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83207578: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8320757C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83207580: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83207588(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83207588 size=80
    let mut pc: u32 = 0x83207588;
    'dispatch: loop {
        match pc {
            0x83207588 => {
    //   block [0x83207588..0x832075D8)
	// 83207588: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8320758C: 4BAA1E81  bl 0x82ca940c
	ctx.lr = 0x83207590;
	sub_82CA93D0(ctx, base);
	// 83207590: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83207594: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 83207598: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8320759C: 388003C4  li r4, 0x3c4
	ctx.r[4].s64 = 964;
	// 832075A0: 83BF05AC  lwz r29, 0x5ac(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1452 as u32) ) } as u64;
	// 832075A4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 832075A8: 4BFC4BF1  bl 0x831cc198
	ctx.lr = 0x832075AC;
	sub_831CC198(ctx, base);
	// 832075AC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 832075B0: 346B0004  addic. r3, r11, 4
	ctx.xer.ca = (ctx.r[11].u32 > (!(4 as u32)));
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 832075B4: 93AB0000  stw r29, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 832075B8: 41820014  beq 0x832075cc
	if ctx.cr[0].eq {
	pc = 0x832075CC; continue 'dispatch;
	}
	// 832075BC: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 832075C0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 832075C4: 4BFFF325  bl 0x832068e8
	ctx.lr = 0x832075C8;
	sub_832068E8(ctx, base);
	// 832075C8: 48000008  b 0x832075d0
	pc = 0x832075D0; continue 'dispatch;
	// 832075CC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 832075D0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 832075D4: 4BAA1E88  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832075D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832075D8 size=108
    let mut pc: u32 = 0x832075D8;
    'dispatch: loop {
        match pc {
            0x832075D8 => {
    //   block [0x832075D8..0x83207644)
	// 832075D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832075DC: 4BAA1E31  bl 0x82ca940c
	ctx.lr = 0x832075E0;
	sub_82CA93D0(ctx, base);
	// 832075E0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832075E4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 832075E8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 832075EC: 388003C4  li r4, 0x3c4
	ctx.r[4].s64 = 964;
	// 832075F0: 83BF05AC  lwz r29, 0x5ac(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1452 as u32) ) } as u64;
	// 832075F4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 832075F8: 4BFC4BA1  bl 0x831cc198
	ctx.lr = 0x832075FC;
	sub_831CC198(ctx, base);
	// 832075FC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83207600: 346B0004  addic. r3, r11, 4
	ctx.xer.ca = (ctx.r[11].u32 > (!(4 as u32)));
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83207604: 93AB0000  stw r29, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 83207608: 41820030  beq 0x83207638
	if ctx.cr[0].eq {
	pc = 0x83207638; continue 'dispatch;
	}
	// 8320760C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 83207610: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83207614: 4BFFF04D  bl 0x83206660
	ctx.lr = 0x83207618;
	sub_83206660(ctx, base);
	// 83207618: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8320761C: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 83207620: 91430010  stw r10, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 83207624: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 83207628: 396BB178  addi r11, r11, -0x4e88
	ctx.r[11].s64 = ctx.r[11].s64 + -20104;
	// 8320762C: 91430014  stw r10, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 83207630: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83207634: 48000008  b 0x8320763c
	pc = 0x8320763C; continue 'dispatch;
	// 83207638: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8320763C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83207640: 4BAA1E1C  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83207648(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83207648 size=80
    let mut pc: u32 = 0x83207648;
    'dispatch: loop {
        match pc {
            0x83207648 => {
    //   block [0x83207648..0x83207698)
	// 83207648: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8320764C: 4BAA1DC1  bl 0x82ca940c
	ctx.lr = 0x83207650;
	sub_82CA93D0(ctx, base);
	// 83207650: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83207654: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 83207658: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8320765C: 388003C4  li r4, 0x3c4
	ctx.r[4].s64 = 964;
	// 83207660: 83BF05AC  lwz r29, 0x5ac(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1452 as u32) ) } as u64;
	// 83207664: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83207668: 4BFC4B31  bl 0x831cc198
	ctx.lr = 0x8320766C;
	sub_831CC198(ctx, base);
	// 8320766C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83207670: 346B0004  addic. r3, r11, 4
	ctx.xer.ca = (ctx.r[11].u32 > (!(4 as u32)));
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83207674: 93AB0000  stw r29, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 83207678: 41820014  beq 0x8320768c
	if ctx.cr[0].eq {
	pc = 0x8320768C; continue 'dispatch;
	}
	// 8320767C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 83207680: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83207684: 4BFFFB85  bl 0x83207208
	ctx.lr = 0x83207688;
	sub_83207208(ctx, base);
	// 83207688: 48000008  b 0x83207690
	pc = 0x83207690; continue 'dispatch;
	// 8320768C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83207690: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83207694: 4BAA1DC8  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83207698(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83207698 size=80
    let mut pc: u32 = 0x83207698;
    'dispatch: loop {
        match pc {
            0x83207698 => {
    //   block [0x83207698..0x832076E8)
	// 83207698: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8320769C: 4BAA1D71  bl 0x82ca940c
	ctx.lr = 0x832076A0;
	sub_82CA93D0(ctx, base);
	// 832076A0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832076A4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 832076A8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 832076AC: 388003C4  li r4, 0x3c4
	ctx.r[4].s64 = 964;
	// 832076B0: 83BF05AC  lwz r29, 0x5ac(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1452 as u32) ) } as u64;
	// 832076B4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 832076B8: 4BFC4AE1  bl 0x831cc198
	ctx.lr = 0x832076BC;
	sub_831CC198(ctx, base);
	// 832076BC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 832076C0: 346B0004  addic. r3, r11, 4
	ctx.xer.ca = (ctx.r[11].u32 > (!(4 as u32)));
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 832076C4: 93AB0000  stw r29, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 832076C8: 41820014  beq 0x832076dc
	if ctx.cr[0].eq {
	pc = 0x832076DC; continue 'dispatch;
	}
	// 832076CC: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 832076D0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 832076D4: 4BFFFBBD  bl 0x83207290
	ctx.lr = 0x832076D8;
	sub_83207290(ctx, base);
	// 832076D8: 48000008  b 0x832076e0
	pc = 0x832076E0; continue 'dispatch;
	// 832076DC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 832076E0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 832076E4: 4BAA1D78  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832076E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832076E8 size=80
    let mut pc: u32 = 0x832076E8;
    'dispatch: loop {
        match pc {
            0x832076E8 => {
    //   block [0x832076E8..0x83207738)
	// 832076E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832076EC: 4BAA1D21  bl 0x82ca940c
	ctx.lr = 0x832076F0;
	sub_82CA93D0(ctx, base);
	// 832076F0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832076F4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 832076F8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 832076FC: 388003C4  li r4, 0x3c4
	ctx.r[4].s64 = 964;
	// 83207700: 83BF05AC  lwz r29, 0x5ac(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1452 as u32) ) } as u64;
	// 83207704: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83207708: 4BFC4A91  bl 0x831cc198
	ctx.lr = 0x8320770C;
	sub_831CC198(ctx, base);
	// 8320770C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83207710: 346B0004  addic. r3, r11, 4
	ctx.xer.ca = (ctx.r[11].u32 > (!(4 as u32)));
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83207714: 93AB0000  stw r29, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 83207718: 41820014  beq 0x8320772c
	if ctx.cr[0].eq {
	pc = 0x8320772C; continue 'dispatch;
	}
	// 8320771C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 83207720: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83207724: 4BFFFA4D  bl 0x83207170
	ctx.lr = 0x83207728;
	sub_83207170(ctx, base);
	// 83207728: 48000008  b 0x83207730
	pc = 0x83207730; continue 'dispatch;
	// 8320772C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83207730: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83207734: 4BAA1D28  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83207738(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83207738 size=80
    let mut pc: u32 = 0x83207738;
    'dispatch: loop {
        match pc {
            0x83207738 => {
    //   block [0x83207738..0x83207788)
	// 83207738: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8320773C: 4BAA1CD1  bl 0x82ca940c
	ctx.lr = 0x83207740;
	sub_82CA93D0(ctx, base);
	// 83207740: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83207744: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 83207748: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8320774C: 388003C4  li r4, 0x3c4
	ctx.r[4].s64 = 964;
	// 83207750: 83BF05AC  lwz r29, 0x5ac(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1452 as u32) ) } as u64;
	// 83207754: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83207758: 4BFC4A41  bl 0x831cc198
	ctx.lr = 0x8320775C;
	sub_831CC198(ctx, base);
	// 8320775C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83207760: 346B0004  addic. r3, r11, 4
	ctx.xer.ca = (ctx.r[11].u32 > (!(4 as u32)));
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83207764: 93AB0000  stw r29, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 83207768: 41820014  beq 0x8320777c
	if ctx.cr[0].eq {
	pc = 0x8320777C; continue 'dispatch;
	}
	// 8320776C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 83207770: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83207774: 4BFFFBA5  bl 0x83207318
	ctx.lr = 0x83207778;
	sub_83207318(ctx, base);
	// 83207778: 48000008  b 0x83207780
	pc = 0x83207780; continue 'dispatch;
	// 8320777C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83207780: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83207784: 4BAA1CD8  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83207788(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83207788 size=80
    let mut pc: u32 = 0x83207788;
    'dispatch: loop {
        match pc {
            0x83207788 => {
    //   block [0x83207788..0x832077D8)
	// 83207788: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8320778C: 4BAA1C81  bl 0x82ca940c
	ctx.lr = 0x83207790;
	sub_82CA93D0(ctx, base);
	// 83207790: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83207794: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 83207798: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8320779C: 388003C4  li r4, 0x3c4
	ctx.r[4].s64 = 964;
	// 832077A0: 83BF05AC  lwz r29, 0x5ac(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1452 as u32) ) } as u64;
	// 832077A4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 832077A8: 4BFC49F1  bl 0x831cc198
	ctx.lr = 0x832077AC;
	sub_831CC198(ctx, base);
	// 832077AC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 832077B0: 346B0004  addic. r3, r11, 4
	ctx.xer.ca = (ctx.r[11].u32 > (!(4 as u32)));
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 832077B4: 93AB0000  stw r29, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 832077B8: 41820014  beq 0x832077cc
	if ctx.cr[0].eq {
	pc = 0x832077CC; continue 'dispatch;
	}
	// 832077BC: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 832077C0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 832077C4: 4BFFFBDD  bl 0x832073a0
	ctx.lr = 0x832077C8;
	sub_832073A0(ctx, base);
	// 832077C8: 48000008  b 0x832077d0
	pc = 0x832077D0; continue 'dispatch;
	// 832077CC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 832077D0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 832077D4: 4BAA1C88  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832077D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832077D8 size=80
    let mut pc: u32 = 0x832077D8;
    'dispatch: loop {
        match pc {
            0x832077D8 => {
    //   block [0x832077D8..0x83207828)
	// 832077D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832077DC: 4BAA1C31  bl 0x82ca940c
	ctx.lr = 0x832077E0;
	sub_82CA93D0(ctx, base);
	// 832077E0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832077E4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 832077E8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 832077EC: 388003C4  li r4, 0x3c4
	ctx.r[4].s64 = 964;
	// 832077F0: 83BF05AC  lwz r29, 0x5ac(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1452 as u32) ) } as u64;
	// 832077F4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 832077F8: 4BFC49A1  bl 0x831cc198
	ctx.lr = 0x832077FC;
	sub_831CC198(ctx, base);
	// 832077FC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83207800: 346B0004  addic. r3, r11, 4
	ctx.xer.ca = (ctx.r[11].u32 > (!(4 as u32)));
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83207804: 93AB0000  stw r29, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 83207808: 41820014  beq 0x8320781c
	if ctx.cr[0].eq {
	pc = 0x8320781C; continue 'dispatch;
	}
	// 8320780C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 83207810: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83207814: 4BFFF83D  bl 0x83207050
	ctx.lr = 0x83207818;
	sub_83207050(ctx, base);
	// 83207818: 48000008  b 0x83207820
	pc = 0x83207820; continue 'dispatch;
	// 8320781C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83207820: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83207824: 4BAA1C38  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83207828(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83207828 size=124
    let mut pc: u32 = 0x83207828;
    'dispatch: loop {
        match pc {
            0x83207828 => {
    //   block [0x83207828..0x832078A4)
	// 83207828: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8320782C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83207830: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83207834: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83207838: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8320783C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 83207840: 388003C4  li r4, 0x3c4
	ctx.r[4].s64 = 964;
	// 83207844: 83DF05AC  lwz r30, 0x5ac(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1452 as u32) ) } as u64;
	// 83207848: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8320784C: 4BFC494D  bl 0x831cc198
	ctx.lr = 0x83207850;
	sub_831CC198(ctx, base);
	// 83207850: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83207854: 346B0004  addic. r3, r11, 4
	ctx.xer.ca = (ctx.r[11].u32 > (!(4 as u32)));
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83207858: 93CB0000  stw r30, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 8320785C: 4182002C  beq 0x83207888
	if ctx.cr[0].eq {
	pc = 0x83207888; continue 'dispatch;
	}
	// 83207860: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 83207864: 38800080  li r4, 0x80
	ctx.r[4].s64 = 128;
	// 83207868: 4BFFEDF9  bl 0x83206660
	ctx.lr = 0x8320786C;
	sub_83206660(ctx, base);
	// 8320786C: 3D408209  lis r10, -0x7df7
	ctx.r[10].s64 = -2113339392;
	// 83207870: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83207874: 394AB5D8  addi r10, r10, -0x4a28
	ctx.r[10].s64 = ctx.r[10].s64 + -18984;
	// 83207878: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 8320787C: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83207880: 91630014  stw r11, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 83207884: 48000008  b 0x8320788c
	pc = 0x8320788C; continue 'dispatch;
	// 83207888: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8320788C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83207890: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83207894: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83207898: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8320789C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832078A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832078A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832078A8 size=80
    let mut pc: u32 = 0x832078A8;
    'dispatch: loop {
        match pc {
            0x832078A8 => {
    //   block [0x832078A8..0x832078F8)
	// 832078A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832078AC: 4BAA1B61  bl 0x82ca940c
	ctx.lr = 0x832078B0;
	sub_82CA93D0(ctx, base);
	// 832078B0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832078B4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 832078B8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 832078BC: 388003C4  li r4, 0x3c4
	ctx.r[4].s64 = 964;
	// 832078C0: 83BF05AC  lwz r29, 0x5ac(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1452 as u32) ) } as u64;
	// 832078C4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 832078C8: 4BFC48D1  bl 0x831cc198
	ctx.lr = 0x832078CC;
	sub_831CC198(ctx, base);
	// 832078CC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 832078D0: 346B0004  addic. r3, r11, 4
	ctx.xer.ca = (ctx.r[11].u32 > (!(4 as u32)));
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 832078D4: 93AB0000  stw r29, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 832078D8: 41820014  beq 0x832078ec
	if ctx.cr[0].eq {
	pc = 0x832078EC; continue 'dispatch;
	}
	// 832078DC: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 832078E0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 832078E4: 4BFFF4ED  bl 0x83206dd0
	ctx.lr = 0x832078E8;
	sub_83206DD0(ctx, base);
	// 832078E8: 48000008  b 0x832078f0
	pc = 0x832078F0; continue 'dispatch;
	// 832078EC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 832078F0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 832078F4: 4BAA1B68  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832078F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832078F8 size=124
    let mut pc: u32 = 0x832078F8;
    'dispatch: loop {
        match pc {
            0x832078F8 => {
    //   block [0x832078F8..0x83207974)
	// 832078F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832078FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83207900: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83207904: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83207908: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8320790C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 83207910: 388003C4  li r4, 0x3c4
	ctx.r[4].s64 = 964;
	// 83207914: 83DF05AC  lwz r30, 0x5ac(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1452 as u32) ) } as u64;
	// 83207918: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8320791C: 4BFC487D  bl 0x831cc198
	ctx.lr = 0x83207920;
	sub_831CC198(ctx, base);
	// 83207920: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83207924: 346B0004  addic. r3, r11, 4
	ctx.xer.ca = (ctx.r[11].u32 > (!(4 as u32)));
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83207928: 93CB0000  stw r30, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 8320792C: 4182002C  beq 0x83207958
	if ctx.cr[0].eq {
	pc = 0x83207958; continue 'dispatch;
	}
	// 83207930: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 83207934: 3880007E  li r4, 0x7e
	ctx.r[4].s64 = 126;
	// 83207938: 4BFFED29  bl 0x83206660
	ctx.lr = 0x8320793C;
	sub_83206660(ctx, base);
	// 8320793C: 3D408209  lis r10, -0x7df7
	ctx.r[10].s64 = -2113339392;
	// 83207940: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83207944: 394AB730  addi r10, r10, -0x48d0
	ctx.r[10].s64 = ctx.r[10].s64 + -18640;
	// 83207948: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 8320794C: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83207950: 91630014  stw r11, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 83207954: 48000008  b 0x8320795c
	pc = 0x8320795C; continue 'dispatch;
	// 83207958: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8320795C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83207960: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83207964: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83207968: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8320796C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83207970: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83207978(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83207978 size=112
    let mut pc: u32 = 0x83207978;
    'dispatch: loop {
        match pc {
            0x83207978 => {
    //   block [0x83207978..0x832079E8)
	// 83207978: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8320797C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83207980: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83207984: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83207988: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8320798C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 83207990: 388003C4  li r4, 0x3c4
	ctx.r[4].s64 = 964;
	// 83207994: 83DF05AC  lwz r30, 0x5ac(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1452 as u32) ) } as u64;
	// 83207998: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8320799C: 4BFC47FD  bl 0x831cc198
	ctx.lr = 0x832079A0;
	sub_831CC198(ctx, base);
	// 832079A0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 832079A4: 346B0004  addic. r3, r11, 4
	ctx.xer.ca = (ctx.r[11].u32 > (!(4 as u32)));
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 832079A8: 93CB0000  stw r30, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 832079AC: 41820020  beq 0x832079cc
	if ctx.cr[0].eq {
	pc = 0x832079CC; continue 'dispatch;
	}
	// 832079B0: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 832079B4: 3880008E  li r4, 0x8e
	ctx.r[4].s64 = 142;
	// 832079B8: 4BFFF4F1  bl 0x83206ea8
	ctx.lr = 0x832079BC;
	sub_83206EA8(ctx, base);
	// 832079BC: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 832079C0: 396BB888  addi r11, r11, -0x4778
	ctx.r[11].s64 = ctx.r[11].s64 + -18296;
	// 832079C4: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 832079C8: 48000008  b 0x832079d0
	pc = 0x832079D0; continue 'dispatch;
	// 832079CC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 832079D0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 832079D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832079D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832079DC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 832079E0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832079E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832079E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832079E8 size=112
    let mut pc: u32 = 0x832079E8;
    'dispatch: loop {
        match pc {
            0x832079E8 => {
    //   block [0x832079E8..0x83207A58)
	// 832079E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832079EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832079F0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 832079F4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832079F8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832079FC: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 83207A00: 388003C4  li r4, 0x3c4
	ctx.r[4].s64 = 964;
	// 83207A04: 83DF05AC  lwz r30, 0x5ac(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1452 as u32) ) } as u64;
	// 83207A08: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83207A0C: 4BFC478D  bl 0x831cc198
	ctx.lr = 0x83207A10;
	sub_831CC198(ctx, base);
	// 83207A10: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83207A14: 346B0004  addic. r3, r11, 4
	ctx.xer.ca = (ctx.r[11].u32 > (!(4 as u32)));
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83207A18: 93CB0000  stw r30, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 83207A1C: 41820020  beq 0x83207a3c
	if ctx.cr[0].eq {
	pc = 0x83207A3C; continue 'dispatch;
	}
	// 83207A20: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 83207A24: 3880008F  li r4, 0x8f
	ctx.r[4].s64 = 143;
	// 83207A28: 4BFFF481  bl 0x83206ea8
	ctx.lr = 0x83207A2C;
	sub_83206EA8(ctx, base);
	// 83207A2C: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 83207A30: 396BB900  addi r11, r11, -0x4700
	ctx.r[11].s64 = ctx.r[11].s64 + -18176;
	// 83207A34: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83207A38: 48000008  b 0x83207a40
	pc = 0x83207A40; continue 'dispatch;
	// 83207A3C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83207A40: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83207A44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83207A48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83207A4C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83207A50: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83207A54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83207A58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83207A58 size=112
    let mut pc: u32 = 0x83207A58;
    'dispatch: loop {
        match pc {
            0x83207A58 => {
    //   block [0x83207A58..0x83207AC8)
	// 83207A58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83207A5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83207A60: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83207A64: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83207A68: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83207A6C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 83207A70: 388003C4  li r4, 0x3c4
	ctx.r[4].s64 = 964;
	// 83207A74: 83DF05AC  lwz r30, 0x5ac(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1452 as u32) ) } as u64;
	// 83207A78: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83207A7C: 4BFC471D  bl 0x831cc198
	ctx.lr = 0x83207A80;
	sub_831CC198(ctx, base);
	// 83207A80: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83207A84: 346B0004  addic. r3, r11, 4
	ctx.xer.ca = (ctx.r[11].u32 > (!(4 as u32)));
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83207A88: 93CB0000  stw r30, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 83207A8C: 41820020  beq 0x83207aac
	if ctx.cr[0].eq {
	pc = 0x83207AAC; continue 'dispatch;
	}
	// 83207A90: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 83207A94: 38800090  li r4, 0x90
	ctx.r[4].s64 = 144;
	// 83207A98: 4BFFF411  bl 0x83206ea8
	ctx.lr = 0x83207A9C;
	sub_83206EA8(ctx, base);
	// 83207A9C: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 83207AA0: 396BB978  addi r11, r11, -0x4688
	ctx.r[11].s64 = ctx.r[11].s64 + -18056;
	// 83207AA4: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83207AA8: 48000008  b 0x83207ab0
	pc = 0x83207AB0; continue 'dispatch;
	// 83207AAC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83207AB0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83207AB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83207AB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83207ABC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83207AC0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83207AC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83207AC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83207AC8 size=112
    let mut pc: u32 = 0x83207AC8;
    'dispatch: loop {
        match pc {
            0x83207AC8 => {
    //   block [0x83207AC8..0x83207B38)
	// 83207AC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83207ACC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83207AD0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83207AD4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83207AD8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83207ADC: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 83207AE0: 388003C4  li r4, 0x3c4
	ctx.r[4].s64 = 964;
	// 83207AE4: 83DF05AC  lwz r30, 0x5ac(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1452 as u32) ) } as u64;
	// 83207AE8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83207AEC: 4BFC46AD  bl 0x831cc198
	ctx.lr = 0x83207AF0;
	sub_831CC198(ctx, base);
	// 83207AF0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83207AF4: 346B0004  addic. r3, r11, 4
	ctx.xer.ca = (ctx.r[11].u32 > (!(4 as u32)));
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83207AF8: 93CB0000  stw r30, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 83207AFC: 41820020  beq 0x83207b1c
	if ctx.cr[0].eq {
	pc = 0x83207B1C; continue 'dispatch;
	}
	// 83207B00: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 83207B04: 38800091  li r4, 0x91
	ctx.r[4].s64 = 145;
	// 83207B08: 4BFFF3A1  bl 0x83206ea8
	ctx.lr = 0x83207B0C;
	sub_83206EA8(ctx, base);
	// 83207B0C: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 83207B10: 396BB810  addi r11, r11, -0x47f0
	ctx.r[11].s64 = ctx.r[11].s64 + -18416;
	// 83207B14: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83207B18: 48000008  b 0x83207b20
	pc = 0x83207B20; continue 'dispatch;
	// 83207B1C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83207B20: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83207B24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83207B28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83207B2C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83207B30: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83207B34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83207B38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83207B38 size=108
    let mut pc: u32 = 0x83207B38;
    'dispatch: loop {
        match pc {
            0x83207B38 => {
    //   block [0x83207B38..0x83207BA4)
	// 83207B38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83207B3C: 4BAA18D1  bl 0x82ca940c
	ctx.lr = 0x83207B40;
	sub_82CA93D0(ctx, base);
	// 83207B40: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83207B44: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 83207B48: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83207B4C: 388003C4  li r4, 0x3c4
	ctx.r[4].s64 = 964;
	// 83207B50: 83BF05AC  lwz r29, 0x5ac(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1452 as u32) ) } as u64;
	// 83207B54: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83207B58: 4BFC4641  bl 0x831cc198
	ctx.lr = 0x83207B5C;
	sub_831CC198(ctx, base);
	// 83207B5C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83207B60: 346B0004  addic. r3, r11, 4
	ctx.xer.ca = (ctx.r[11].u32 > (!(4 as u32)));
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83207B64: 93AB0000  stw r29, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 83207B68: 41820030  beq 0x83207b98
	if ctx.cr[0].eq {
	pc = 0x83207B98; continue 'dispatch;
	}
	// 83207B6C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 83207B70: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83207B74: 4BFFEAED  bl 0x83206660
	ctx.lr = 0x83207B78;
	sub_83206660(ctx, base);
	// 83207B78: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 83207B7C: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 83207B80: 91430010  stw r10, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 83207B84: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83207B88: 396BBF28  addi r11, r11, -0x40d8
	ctx.r[11].s64 = ctx.r[11].s64 + -16600;
	// 83207B8C: 91430014  stw r10, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 83207B90: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83207B94: 48000008  b 0x83207b9c
	pc = 0x83207B9C; continue 'dispatch;
	// 83207B98: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83207B9C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83207BA0: 4BAA18BC  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83207BA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83207BA8 size=104
    let mut pc: u32 = 0x83207BA8;
    'dispatch: loop {
        match pc {
            0x83207BA8 => {
    //   block [0x83207BA8..0x83207C10)
	// 83207BA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83207BAC: 4BAA1861  bl 0x82ca940c
	ctx.lr = 0x83207BB0;
	sub_82CA93D0(ctx, base);
	// 83207BB0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83207BB4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 83207BB8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83207BBC: 388003C4  li r4, 0x3c4
	ctx.r[4].s64 = 964;
	// 83207BC0: 83BF05AC  lwz r29, 0x5ac(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1452 as u32) ) } as u64;
	// 83207BC4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83207BC8: 4BFC45D1  bl 0x831cc198
	ctx.lr = 0x83207BCC;
	sub_831CC198(ctx, base);
	// 83207BCC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83207BD0: 346B0004  addic. r3, r11, 4
	ctx.xer.ca = (ctx.r[11].u32 > (!(4 as u32)));
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83207BD4: 93AB0000  stw r29, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 83207BD8: 4182002C  beq 0x83207c04
	if ctx.cr[0].eq {
	pc = 0x83207C04; continue 'dispatch;
	}
	// 83207BDC: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 83207BE0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83207BE4: 4BFFEA7D  bl 0x83206660
	ctx.lr = 0x83207BE8;
	sub_83206660(ctx, base);
	// 83207BE8: 3D408209  lis r10, -0x7df7
	ctx.r[10].s64 = -2113339392;
	// 83207BEC: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83207BF0: 394AB110  addi r10, r10, -0x4ef0
	ctx.r[10].s64 = ctx.r[10].s64 + -20208;
	// 83207BF4: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 83207BF8: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83207BFC: 91630014  stw r11, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 83207C00: 48000008  b 0x83207c08
	pc = 0x83207C08; continue 'dispatch;
	// 83207C04: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83207C08: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83207C0C: 4BAA1850  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83207C10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83207C10 size=80
    let mut pc: u32 = 0x83207C10;
    'dispatch: loop {
        match pc {
            0x83207C10 => {
    //   block [0x83207C10..0x83207C60)
	// 83207C10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83207C14: 4BAA17F9  bl 0x82ca940c
	ctx.lr = 0x83207C18;
	sub_82CA93D0(ctx, base);
	// 83207C18: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83207C1C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 83207C20: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83207C24: 388003C4  li r4, 0x3c4
	ctx.r[4].s64 = 964;
	// 83207C28: 83BF05AC  lwz r29, 0x5ac(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1452 as u32) ) } as u64;
	// 83207C2C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83207C30: 4BFC4569  bl 0x831cc198
	ctx.lr = 0x83207C34;
	sub_831CC198(ctx, base);
	// 83207C34: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83207C38: 346B0004  addic. r3, r11, 4
	ctx.xer.ca = (ctx.r[11].u32 > (!(4 as u32)));
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83207C3C: 93AB0000  stw r29, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 83207C40: 41820014  beq 0x83207c54
	if ctx.cr[0].eq {
	pc = 0x83207C54; continue 'dispatch;
	}
	// 83207C44: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 83207C48: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83207C4C: 4BFFEBED  bl 0x83206838
	ctx.lr = 0x83207C50;
	sub_83206838(ctx, base);
	// 83207C50: 48000008  b 0x83207c58
	pc = 0x83207C58; continue 'dispatch;
	// 83207C54: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83207C58: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83207C5C: 4BAA1800  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83207C60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83207C60 size=108
    let mut pc: u32 = 0x83207C60;
    'dispatch: loop {
        match pc {
            0x83207C60 => {
    //   block [0x83207C60..0x83207CCC)
	// 83207C60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83207C64: 4BAA17A9  bl 0x82ca940c
	ctx.lr = 0x83207C68;
	sub_82CA93D0(ctx, base);
	// 83207C68: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83207C6C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 83207C70: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83207C74: 388003C4  li r4, 0x3c4
	ctx.r[4].s64 = 964;
	// 83207C78: 83BF05AC  lwz r29, 0x5ac(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1452 as u32) ) } as u64;
	// 83207C7C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83207C80: 4BFC4519  bl 0x831cc198
	ctx.lr = 0x83207C84;
	sub_831CC198(ctx, base);
	// 83207C84: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83207C88: 346B0004  addic. r3, r11, 4
	ctx.xer.ca = (ctx.r[11].u32 > (!(4 as u32)));
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83207C8C: 93AB0000  stw r29, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 83207C90: 41820030  beq 0x83207cc0
	if ctx.cr[0].eq {
	pc = 0x83207CC0; continue 'dispatch;
	}
	// 83207C94: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 83207C98: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83207C9C: 4BFFE9C5  bl 0x83206660
	ctx.lr = 0x83207CA0;
	sub_83206660(ctx, base);
	// 83207CA0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 83207CA4: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 83207CA8: 91430010  stw r10, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 83207CAC: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 83207CB0: 396B7DA8  addi r11, r11, 0x7da8
	ctx.r[11].s64 = ctx.r[11].s64 + 32168;
	// 83207CB4: 91430014  stw r10, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 83207CB8: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83207CBC: 48000008  b 0x83207cc4
	pc = 0x83207CC4; continue 'dispatch;
	// 83207CC0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83207CC4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83207CC8: 4BAA1794  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83207CD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83207CD0 size=96
    let mut pc: u32 = 0x83207CD0;
    'dispatch: loop {
        match pc {
            0x83207CD0 => {
    //   block [0x83207CD0..0x83207D30)
	// 83207CD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83207CD4: 4BAA1731  bl 0x82ca9404
	ctx.lr = 0x83207CD8;
	sub_82CA93D0(ctx, base);
	// 83207CD8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83207CDC: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 83207CE0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83207CE4: 388003C4  li r4, 0x3c4
	ctx.r[4].s64 = 964;
	// 83207CE8: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 83207CEC: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 83207CF0: 837F05AC  lwz r27, 0x5ac(r31)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1452 as u32) ) } as u64;
	// 83207CF4: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 83207CF8: 4BFC44A1  bl 0x831cc198
	ctx.lr = 0x83207CFC;
	sub_831CC198(ctx, base);
	// 83207CFC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83207D00: 346B0004  addic. r3, r11, 4
	ctx.xer.ca = (ctx.r[11].u32 > (!(4 as u32)));
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83207D04: 936B0000  stw r27, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 83207D08: 4182001C  beq 0x83207d24
	if ctx.cr[0].eq {
	pc = 0x83207D24; continue 'dispatch;
	}
	// 83207D0C: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 83207D10: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 83207D14: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 83207D18: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83207D1C: 4BFFF2C5  bl 0x83206fe0
	ctx.lr = 0x83207D20;
	sub_83206FE0(ctx, base);
	// 83207D20: 48000008  b 0x83207d28
	pc = 0x83207D28; continue 'dispatch;
	// 83207D24: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83207D28: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83207D2C: 4BAA1728  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83207D30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83207D30 size=88
    let mut pc: u32 = 0x83207D30;
    'dispatch: loop {
        match pc {
            0x83207D30 => {
    //   block [0x83207D30..0x83207D88)
	// 83207D30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83207D34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83207D38: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83207D3C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83207D40: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83207D44: 4BFFE91D  bl 0x83206660
	ctx.lr = 0x83207D48;
	sub_83206660(ctx, base);
	// 83207D48: 3D408209  lis r10, -0x7df7
	ctx.r[10].s64 = -2113339392;
	// 83207D4C: 3D208209  lis r9, -0x7df7
	ctx.r[9].s64 = -2113339392;
	// 83207D50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83207D54: 394ABF28  addi r10, r10, -0x40d8
	ctx.r[10].s64 = ctx.r[10].s64 + -16600;
	// 83207D58: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 83207D5C: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 83207D60: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83207D64: 8149AF6C  lwz r10, -0x5094(r9)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-20628 as u32) ) } as u64;
	// 83207D68: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 83207D6C: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 83207D70: 915F0080  stw r10, 0x80(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(128 as u32), ctx.r[10].u32 ) };
	// 83207D74: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83207D78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83207D7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83207D80: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83207D84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83207D88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83207D88 size=80
    let mut pc: u32 = 0x83207D88;
    'dispatch: loop {
        match pc {
            0x83207D88 => {
    //   block [0x83207D88..0x83207DD8)
	// 83207D88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83207D8C: 4BAA1681  bl 0x82ca940c
	ctx.lr = 0x83207D90;
	sub_82CA93D0(ctx, base);
	// 83207D90: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83207D94: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 83207D98: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83207D9C: 388003C4  li r4, 0x3c4
	ctx.r[4].s64 = 964;
	// 83207DA0: 83BF05AC  lwz r29, 0x5ac(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1452 as u32) ) } as u64;
	// 83207DA4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83207DA8: 4BFC43F1  bl 0x831cc198
	ctx.lr = 0x83207DAC;
	sub_831CC198(ctx, base);
	// 83207DAC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83207DB0: 346B0004  addic. r3, r11, 4
	ctx.xer.ca = (ctx.r[11].u32 > (!(4 as u32)));
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83207DB4: 93AB0000  stw r29, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 83207DB8: 41820014  beq 0x83207dcc
	if ctx.cr[0].eq {
	pc = 0x83207DCC; continue 'dispatch;
	}
	// 83207DBC: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 83207DC0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83207DC4: 4BFFFF6D  bl 0x83207d30
	ctx.lr = 0x83207DC8;
	sub_83207D30(ctx, base);
	// 83207DC8: 48000008  b 0x83207dd0
	pc = 0x83207DD0; continue 'dispatch;
	// 83207DCC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83207DD0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83207DD4: 4BAA1688  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83207DD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83207DD8 size=240
    let mut pc: u32 = 0x83207DD8;
    'dispatch: loop {
        match pc {
            0x83207DD8 => {
    //   block [0x83207DD8..0x83207EC8)
	// 83207DD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83207DDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83207DE0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83207DE4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83207DE8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83207DEC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83207DF0: 38640080  addi r3, r4, 0x80
	ctx.r[3].s64 = ctx.r[4].s64 + 128;
	// 83207DF4: 909F0020  stw r4, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[4].u32 ) };
	// 83207DF8: 4BFFB349  bl 0x83203140
	ctx.lr = 0x83207DFC;
	sub_83203140(ctx, base);
	// 83207DFC: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 83207E00: 907F001C  stw r3, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[3].u32 ) };
	// 83207E04: 39430001  addi r10, r3, 1
	ctx.r[10].s64 = ctx.r[3].s64 + 1;
	// 83207E08: 555E103A  slwi r30, r10, 2
	ctx.r[30].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 83207E0C: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 83207E10: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83207E14: 917F0024  stw r11, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 83207E18: 806B05B0  lwz r3, 0x5b0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1456 as u32) ) } as u64;
	// 83207E1C: 4BFC437D  bl 0x831cc198
	ctx.lr = 0x83207E20;
	sub_831CC198(ctx, base);
	// 83207E20: 907F0008  stw r3, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[3].u32 ) };
	// 83207E24: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83207E28: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 83207E2C: 806B05B0  lwz r3, 0x5b0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1456 as u32) ) } as u64;
	// 83207E30: 4BFC4369  bl 0x831cc198
	ctx.lr = 0x83207E34;
	sub_831CC198(ctx, base);
	// 83207E34: 907F0004  stw r3, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 83207E38: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 83207E3C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83207E40: 806B05B0  lwz r3, 0x5b0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1456 as u32) ) } as u64;
	// 83207E44: 4BFC4355  bl 0x831cc198
	ctx.lr = 0x83207E48;
	sub_831CC198(ctx, base);
	// 83207E48: 907F0010  stw r3, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[3].u32 ) };
	// 83207E4C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83207E50: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 83207E54: 806B05B0  lwz r3, 0x5b0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1456 as u32) ) } as u64;
	// 83207E58: 4BFC4341  bl 0x831cc198
	ctx.lr = 0x83207E5C;
	sub_831CC198(ctx, base);
	// 83207E5C: 907F000C  stw r3, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[3].u32 ) };
	// 83207E60: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 83207E64: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83207E68: 806B05B0  lwz r3, 0x5b0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1456 as u32) ) } as u64;
	// 83207E6C: 4BFC432D  bl 0x831cc198
	ctx.lr = 0x83207E70;
	sub_831CC198(ctx, base);
	// 83207E70: 907F0000  stw r3, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 83207E74: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 83207E78: 815F0024  lwz r10, 0x24(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 83207E7C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83207E80: 5564103A  slwi r4, r11, 2
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 83207E84: 806A05B0  lwz r3, 0x5b0(r10)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(1456 as u32) ) } as u64;
	// 83207E88: 4BFC4311  bl 0x831cc198
	ctx.lr = 0x83207E8C;
	sub_831CC198(ctx, base);
	// 83207E8C: 907F0018  stw r3, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[3].u32 ) };
	// 83207E90: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 83207E94: 815F0024  lwz r10, 0x24(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 83207E98: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83207E9C: 5564103A  slwi r4, r11, 2
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 83207EA0: 806A05B0  lwz r3, 0x5b0(r10)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(1456 as u32) ) } as u64;
	// 83207EA4: 4BFC42F5  bl 0x831cc198
	ctx.lr = 0x83207EA8;
	sub_831CC198(ctx, base);
	// 83207EA8: 907F0014  stw r3, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[3].u32 ) };
	// 83207EAC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83207EB0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83207EB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83207EB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83207EBC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83207EC0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83207EC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83207EC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83207EC8 size=276
    let mut pc: u32 = 0x83207EC8;
    'dispatch: loop {
        match pc {
            0x83207EC8 => {
    //   block [0x83207EC8..0x83207FDC)
	// 83207EC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83207ECC: 4BAA153D  bl 0x82ca9408
	ctx.lr = 0x83207ED0;
	sub_82CA93D0(ctx, base);
	// 83207ED0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83207ED4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83207ED8: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 83207EDC: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 83207EE0: 4800003C  b 0x83207f1c
	pc = 0x83207F1C; continue 'dispatch;
	// 83207EE4: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 83207EE8: 7D5E582E  lwzx r10, r30, r11
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 83207EEC: 814A0004  lwz r10, 4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 83207EF0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 83207EF4: 419A0038  beq cr6, 0x83207f2c
	if ctx.cr[6].eq {
	pc = 0x83207F2C; continue 'dispatch;
	}
	// 83207EF8: 7D7E582E  lwzx r11, r30, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 83207EFC: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 83207F00: 83AB0004  lwz r29, 4(r11)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83207F04: 41820010  beq 0x83207f14
	if ctx.cr[0].eq {
	pc = 0x83207F14; continue 'dispatch;
	}
	// 83207F08: 388BFFFC  addi r4, r11, -4
	ctx.r[4].s64 = ctx.r[11].s64 + -4;
	// 83207F0C: 806BFFFC  lwz r3, -4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-4 as u32) ) } as u64;
	// 83207F10: 4BFC43B9  bl 0x831cc2c8
	ctx.lr = 0x83207F14;
	sub_831CC2C8(ctx, base);
	// 83207F14: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 83207F18: 7FBE592E  stwx r29, r30, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32), ctx.r[29].u32) };
	// 83207F1C: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 83207F20: 7D7E582E  lwzx r11, r30, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 83207F24: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83207F28: 409AFFBC  bne cr6, 0x83207ee4
	if !ctx.cr[6].eq {
	pc = 0x83207EE4; continue 'dispatch;
	}
	// 83207F2C: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 83207F30: 7D7E582E  lwzx r11, r30, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 83207F34: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83207F38: 419A0010  beq cr6, 0x83207f48
	if ctx.cr[6].eq {
	pc = 0x83207F48; continue 'dispatch;
	}
	// 83207F3C: 388BFFFC  addi r4, r11, -4
	ctx.r[4].s64 = ctx.r[11].s64 + -4;
	// 83207F40: 806BFFFC  lwz r3, -4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-4 as u32) ) } as u64;
	// 83207F44: 4BFC4385  bl 0x831cc2c8
	ctx.lr = 0x83207F48;
	sub_831CC2C8(ctx, base);
	// 83207F48: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 83207F4C: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 83207F50: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 83207F54: 7F1C5840  cmplw cr6, r28, r11
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[11].u32, &mut ctx.xer);
	// 83207F58: 4099FFC4  ble cr6, 0x83207f1c
	if !ctx.cr[6].gt {
	pc = 0x83207F1C; continue 'dispatch;
	}
	// 83207F5C: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 83207F60: 809F0014  lwz r4, 0x14(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 83207F64: 806B05B0  lwz r3, 0x5b0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1456 as u32) ) } as u64;
	// 83207F68: 4BFC4361  bl 0x831cc2c8
	ctx.lr = 0x83207F6C;
	sub_831CC2C8(ctx, base);
	// 83207F6C: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 83207F70: 809F0018  lwz r4, 0x18(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 83207F74: 806B05B0  lwz r3, 0x5b0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1456 as u32) ) } as u64;
	// 83207F78: 4BFC4351  bl 0x831cc2c8
	ctx.lr = 0x83207F7C;
	sub_831CC2C8(ctx, base);
	// 83207F7C: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 83207F80: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83207F84: 806B05B0  lwz r3, 0x5b0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1456 as u32) ) } as u64;
	// 83207F88: 4BFC4341  bl 0x831cc2c8
	ctx.lr = 0x83207F8C;
	sub_831CC2C8(ctx, base);
	// 83207F8C: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 83207F90: 809F000C  lwz r4, 0xc(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 83207F94: 806B05B0  lwz r3, 0x5b0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1456 as u32) ) } as u64;
	// 83207F98: 4BFC4331  bl 0x831cc2c8
	ctx.lr = 0x83207F9C;
	sub_831CC2C8(ctx, base);
	// 83207F9C: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 83207FA0: 809F0010  lwz r4, 0x10(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 83207FA4: 806B05B0  lwz r3, 0x5b0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1456 as u32) ) } as u64;
	// 83207FA8: 4BFC4321  bl 0x831cc2c8
	ctx.lr = 0x83207FAC;
	sub_831CC2C8(ctx, base);
	// 83207FAC: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 83207FB0: 809F0004  lwz r4, 4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83207FB4: 806B05B0  lwz r3, 0x5b0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1456 as u32) ) } as u64;
	// 83207FB8: 4BFC4311  bl 0x831cc2c8
	ctx.lr = 0x83207FBC;
	sub_831CC2C8(ctx, base);
	// 83207FBC: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 83207FC0: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83207FC4: 806B05B0  lwz r3, 0x5b0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1456 as u32) ) } as u64;
	// 83207FC8: 4BFC4301  bl 0x831cc2c8
	ctx.lr = 0x83207FCC;
	sub_831CC2C8(ctx, base);
	// 83207FCC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83207FD0: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 83207FD4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83207FD8: 4BAA1480  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83207FE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83207FE0 size=356
    let mut pc: u32 = 0x83207FE0;
    'dispatch: loop {
        match pc {
            0x83207FE0 => {
    //   block [0x83207FE0..0x83208144)
	// 83207FE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83207FE4: 4BAA1409  bl 0x82ca93ec
	ctx.lr = 0x83207FE8;
	sub_82CA93D0(ctx, base);
	// 83207FE8: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83207FEC: 7C771B78  mr r23, r3
	ctx.r[23].u64 = ctx.r[3].u64;
	// 83207FF0: 80770020  lwz r3, 0x20(r23)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(32 as u32) ) } as u64;
	// 83207FF4: 4BFE17AD  bl 0x831e97a0
	ctx.lr = 0x83207FF8;
	sub_831E97A0(ctx, base);
	// 83207FF8: 8177001C  lwz r11, 0x1c(r23)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(28 as u32) ) } as u64;
	// 83207FFC: 3AA00001  li r21, 1
	ctx.r[21].s64 = 1;
	// 83208000: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 83208004: 41980138  blt cr6, 0x8320813c
	if ctx.cr[6].lt {
	pc = 0x8320813C; continue 'dispatch;
	}
	// 83208008: 3AC30004  addi r22, r3, 4
	ctx.r[22].s64 = ctx.r[3].s64 + 4;
	// 8320800C: 83360000  lwz r25, 0(r22)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(0 as u32) ) } as u64;
	// 83208010: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 83208014: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 83208018: 3BA00001  li r29, 1
	ctx.r[29].s64 = 1;
	// 8320801C: 83190048  lwz r24, 0x48(r25)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(72 as u32) ) } as u64;
	// 83208020: 81790038  lwz r11, 0x38(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(56 as u32) ) } as u64;
	// 83208024: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83208028: 7F1D5040  cmplw cr6, r29, r10
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[10].u32, &mut ctx.xer);
	// 8320802C: 4099000C  ble cr6, 0x83208038
	if !ctx.cr[6].gt {
	pc = 0x83208038; continue 'dispatch;
	}
	// 83208030: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83208034: 48000010  b 0x83208044
	pc = 0x83208044; continue 'dispatch;
	// 83208038: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8320803C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83208040: 7FEAF02E  lwzx r31, r10, r30
	ctx.r[31].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 83208044: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83208048: 41820034  beq 0x8320807c
	if ctx.cr[0].eq {
	pc = 0x8320807C; continue 'dispatch;
	}
	// 8320804C: 817F0048  lwz r11, 0x48(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 83208050: 81570008  lwz r10, 8(r23)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(8 as u32) ) } as u64;
	// 83208054: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83208058: 7D6B502E  lwzx r11, r11, r10
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 8320805C: 7F0BC040  cmplw cr6, r11, r24
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[24].u32, &mut ctx.xer);
	// 83208060: 419A0010  beq cr6, 0x83208070
	if ctx.cr[6].eq {
	pc = 0x83208070; continue 'dispatch;
	}
	// 83208064: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83208068: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 8320806C: 48000CED  bl 0x83208d58
	ctx.lr = 0x83208070;
	sub_83208D58(ctx, base);
	// 83208070: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 83208074: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 83208078: 4BFFFFA8  b 0x83208020
	pc = 0x83208020; continue 'dispatch;
	// 8320807C: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 83208080: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 83208084: 3B400001  li r26, 1
	ctx.r[26].s64 = 1;
	// 83208088: 81790058  lwz r11, 0x58(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(88 as u32) ) } as u64;
	// 8320808C: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83208090: 7F1A5040  cmplw cr6, r26, r10
	ctx.cr[6].compare_u32(ctx.r[26].u32, ctx.r[10].u32, &mut ctx.xer);
	// 83208094: 4099000C  ble cr6, 0x832080a0
	if !ctx.cr[6].gt {
	pc = 0x832080A0; continue 'dispatch;
	}
	// 83208098: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8320809C: 48000010  b 0x832080ac
	pc = 0x832080AC; continue 'dispatch;
	// 832080A0: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 832080A4: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 832080A8: 7F6AE02E  lwzx r27, r10, r28
	ctx.r[27].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 832080AC: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 832080B0: 41820078  beq 0x83208128
	if ctx.cr[0].eq {
	pc = 0x83208128; continue 'dispatch;
	}
	// 832080B4: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 832080B8: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 832080BC: 3BA00001  li r29, 1
	ctx.r[29].s64 = 1;
	// 832080C0: 817B005C  lwz r11, 0x5c(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(92 as u32) ) } as u64;
	// 832080C4: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 832080C8: 7F1D5040  cmplw cr6, r29, r10
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[10].u32, &mut ctx.xer);
	// 832080CC: 4099000C  ble cr6, 0x832080d8
	if !ctx.cr[6].gt {
	pc = 0x832080D8; continue 'dispatch;
	}
	// 832080D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832080D4: 48000010  b 0x832080e4
	pc = 0x832080E4; continue 'dispatch;
	// 832080D8: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 832080DC: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 832080E0: 7FEAF02E  lwzx r31, r10, r30
	ctx.r[31].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 832080E4: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 832080E8: 41820034  beq 0x8320811c
	if ctx.cr[0].eq {
	pc = 0x8320811C; continue 'dispatch;
	}
	// 832080EC: 817F0048  lwz r11, 0x48(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 832080F0: 81570008  lwz r10, 8(r23)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(8 as u32) ) } as u64;
	// 832080F4: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 832080F8: 7D6B502E  lwzx r11, r11, r10
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 832080FC: 7F0BC040  cmplw cr6, r11, r24
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[24].u32, &mut ctx.xer);
	// 83208100: 419A0010  beq cr6, 0x83208110
	if ctx.cr[6].eq {
	pc = 0x83208110; continue 'dispatch;
	}
	// 83208104: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83208108: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 8320810C: 48000C4D  bl 0x83208d58
	ctx.lr = 0x83208110;
	sub_83208D58(ctx, base);
	// 83208110: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 83208114: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 83208118: 4BFFFFA8  b 0x832080c0
	pc = 0x832080C0; continue 'dispatch;
	// 8320811C: 3B5A0001  addi r26, r26, 1
	ctx.r[26].s64 = ctx.r[26].s64 + 1;
	// 83208120: 3B9C0004  addi r28, r28, 4
	ctx.r[28].s64 = ctx.r[28].s64 + 4;
	// 83208124: 4BFFFF64  b 0x83208088
	pc = 0x83208088; continue 'dispatch;
	// 83208128: 8177001C  lwz r11, 0x1c(r23)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(28 as u32) ) } as u64;
	// 8320812C: 3AB50001  addi r21, r21, 1
	ctx.r[21].s64 = ctx.r[21].s64 + 1;
	// 83208130: 3AD60004  addi r22, r22, 4
	ctx.r[22].s64 = ctx.r[22].s64 + 4;
	// 83208134: 7F155840  cmplw cr6, r21, r11
	ctx.cr[6].compare_u32(ctx.r[21].u32, ctx.r[11].u32, &mut ctx.xer);
	// 83208138: 4099FED4  ble cr6, 0x8320800c
	if !ctx.cr[6].gt {
	pc = 0x8320800C; continue 'dispatch;
	}
	// 8320813C: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 83208140: 4BAA12FC  b 0x82ca943c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83208148(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83208148 size=252
    let mut pc: u32 = 0x83208148;
    'dispatch: loop {
        match pc {
            0x83208148 => {
    //   block [0x83208148..0x83208244)
	// 83208148: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8320814C: 4BAA12BD  bl 0x82ca9408
	ctx.lr = 0x83208150;
	sub_82CA93D0(ctx, base);
	// 83208150: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83208154: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83208158: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 8320815C: 38800014  li r4, 0x14
	ctx.r[4].s64 = 20;
	// 83208160: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 83208164: 83CB05B0  lwz r30, 0x5b0(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1456 as u32) ) } as u64;
	// 83208168: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8320816C: 4BFC402D  bl 0x831cc198
	ctx.lr = 0x83208170;
	sub_831CC198(ctx, base);
	// 83208170: 37A30004  addic. r29, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[29].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 83208174: 93C30000  stw r30, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 83208178: 41820018  beq 0x83208190
	if ctx.cr[0].eq {
	pc = 0x83208190; continue 'dispatch;
	}
	// 8320817C: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 83208180: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83208184: 808B05B0  lwz r4, 0x5b0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1456 as u32) ) } as u64;
	// 83208188: 4BFDD8E9  bl 0x831e5a70
	ctx.lr = 0x8320818C;
	sub_831E5A70(ctx, base);
	// 8320818C: 48000008  b 0x83208194
	pc = 0x83208194; continue 'dispatch;
	// 83208190: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 83208194: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 83208198: 4800001C  b 0x832081b4
	pc = 0x832081B4; continue 'dispatch;
	// 8320819C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 832081A0: 809D0004  lwz r4, 4(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 832081A4: 4BFFB1FD  bl 0x832033a0
	ctx.lr = 0x832081A8;
	sub_832033A0(ctx, base);
	// 832081A8: 93830000  stw r28, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 832081AC: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 832081B0: 7F8BF02E  lwzx r28, r11, r30
	ctx.r[28].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 832081B4: 579E103A  slwi r30, r28, 2
	ctx.r[30].u32 = ctx.r[28].u32.wrapping_shl(2);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 832081B8: 7D4BF02E  lwzx r10, r11, r30
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 832081BC: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 832081C0: 7D6A582E  lwzx r11, r10, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 832081C4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 832081C8: 409AFFD4  bne cr6, 0x8320819c
	if !ctx.cr[6].eq {
	pc = 0x8320819C; continue 'dispatch;
	}
	// 832081CC: 48000058  b 0x83208224
	pc = 0x83208224; continue 'dispatch;
	// 832081D0: 4800D281  bl 0x83215450
	ctx.lr = 0x832081D4;
	sub_83215450(ctx, base);
	// 832081D4: 815F000C  lwz r10, 0xc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 832081D8: 546B103A  slwi r11, r3, 2
	ctx.r[11].u32 = ctx.r[3].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 832081DC: 813F0010  lwz r9, 0x10(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 832081E0: 811F0004  lwz r8, 4(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832081E4: 7D4A582E  lwzx r10, r10, r11
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 832081E8: 7CE9582E  lwzx r7, r9, r11
	ctx.r[7].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 832081EC: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 832081F0: 54E7103A  slwi r7, r7, 2
	ctx.r[7].u32 = ctx.r[7].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 832081F4: 7D29502E  lwzx r9, r9, r10
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 832081F8: 7CE7402E  lwzx r7, r7, r8
	ctx.r[7].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[7].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 832081FC: 5529103A  slwi r9, r9, 2
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 83208200: 7D29402E  lwzx r9, r9, r8
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 83208204: 7F093840  cmplw cr6, r9, r7
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[7].u32, &mut ctx.xer);
	// 83208208: 40980010  bge cr6, 0x83208218
	if !ctx.cr[6].lt {
	pc = 0x83208218; continue 'dispatch;
	}
	// 8320820C: 813F0010  lwz r9, 0x10(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 83208210: 7D09502E  lwzx r8, r9, r10
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 83208214: 7D09592E  stwx r8, r9, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32), ctx.r[8].u32) };
	// 83208218: 813F000C  lwz r9, 0xc(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8320821C: 7D49502E  lwzx r10, r9, r10
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 83208220: 7D49592E  stwx r10, r9, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32), ctx.r[10].u32) };
	// 83208224: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 83208228: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8320822C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83208230: 409AFFA0  bne cr6, 0x832081d0
	if !ctx.cr[6].eq {
	pc = 0x832081D0; continue 'dispatch;
	}
	// 83208234: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83208238: 4800BA11  bl 0x83213c48
	ctx.lr = 0x8320823C;
	sub_83213C48(ctx, base);
	// 8320823C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83208240: 4BAA1218  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83208248(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83208248 size=80
    let mut pc: u32 = 0x83208248;
    'dispatch: loop {
        match pc {
            0x83208248 => {
    //   block [0x83208248..0x83208298)
	// 83208248: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8320824C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83208250: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83208254: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83208258: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8320825C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83208260: 549E103A  slwi r30, r4, 2
	ctx.r[30].u32 = ctx.r[4].u32.wrapping_shl(2);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 83208264: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 83208268: 7D6BF02E  lwzx r11, r11, r30
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 8320826C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83208270: 419A0008  beq cr6, 0x83208278
	if ctx.cr[6].eq {
	pc = 0x83208278; continue 'dispatch;
	}
	// 83208274: 4BFFFED5  bl 0x83208148
	ctx.lr = 0x83208278;
	sub_83208148(ctx, base);
	// 83208278: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8320827C: 7C6BF02E  lwzx r3, r11, r30
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 83208280: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83208284: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83208288: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8320828C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83208290: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83208294: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83208298(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83208298 size=344
    let mut pc: u32 = 0x83208298;
    'dispatch: loop {
        match pc {
            0x83208298 => {
    //   block [0x83208298..0x832083F0)
	// 83208298: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8320829C: 4BAA1169  bl 0x82ca9404
	ctx.lr = 0x832082A0;
	sub_82CA93D0(ctx, base);
	// 832082A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832082A4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 832082A8: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 832082AC: 38800014  li r4, 0x14
	ctx.r[4].s64 = 20;
	// 832082B0: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 832082B4: 814B0864  lwz r10, 0x864(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2148 as u32) ) } as u64;
	// 832082B8: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 832082BC: 914B0864  stw r10, 0x864(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(2148 as u32), ctx.r[10].u32 ) };
	// 832082C0: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 832082C4: 83CB05B0  lwz r30, 0x5b0(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1456 as u32) ) } as u64;
	// 832082C8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 832082CC: 4BFC3ECD  bl 0x831cc198
	ctx.lr = 0x832082D0;
	sub_831CC198(ctx, base);
	// 832082D0: 37830004  addic. r28, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[28].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 832082D4: 93C30000  stw r30, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 832082D8: 41820018  beq 0x832082f0
	if ctx.cr[0].eq {
	pc = 0x832082F0; continue 'dispatch;
	}
	// 832082DC: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 832082E0: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 832082E4: 808B05B0  lwz r4, 0x5b0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1456 as u32) ) } as u64;
	// 832082E8: 4BFDD789  bl 0x831e5a70
	ctx.lr = 0x832082EC;
	sub_831E5A70(ctx, base);
	// 832082EC: 48000008  b 0x832082f4
	pc = 0x832082F4; continue 'dispatch;
	// 832082F0: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 832082F4: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 832082F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832082FC: 917D0048  stw r11, 0x48(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(72 as u32), ctx.r[11].u32 ) };
	// 83208300: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83208304: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83208308: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8320830C: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 83208310: 815D0048  lwz r10, 0x48(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(72 as u32) ) } as u64;
	// 83208314: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83208318: 7FAA592E  stwx r29, r10, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32), ctx.r[29].u32) };
	// 8320831C: 809C0004  lwz r4, 4(r28)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 83208320: 4BFFB081  bl 0x832033a0
	ctx.lr = 0x83208324;
	sub_832033A0(ctx, base);
	// 83208324: 93A30000  stw r29, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 83208328: 817D0048  lwz r11, 0x48(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(72 as u32) ) } as u64;
	// 8320832C: 556A103A  slwi r10, r11, 2
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83208330: 813F0004  lwz r9, 4(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83208334: 3B600002  li r27, 2
	ctx.r[27].s64 = 2;
	// 83208338: 7D6A492E  stwx r11, r10, r9
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32), ctx.r[11].u32) };
	// 8320833C: 815D0048  lwz r10, 0x48(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(72 as u32) ) } as u64;
	// 83208340: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 83208344: 5549103A  slwi r9, r10, 2
	ctx.r[9].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 83208348: 7D49592E  stwx r10, r9, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32), ctx.r[10].u32) };
	// 8320834C: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 83208350: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83208354: 808B0864  lwz r4, 0x864(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2148 as u32) ) } as u64;
	// 83208358: 480008F1  bl 0x83208c48
	ctx.lr = 0x8320835C;
	sub_83208C48(ctx, base);
	// 8320835C: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 83208360: 41820064  beq 0x832083c4
	if ctx.cr[0].eq {
	pc = 0x832083C4; continue 'dispatch;
	}
	// 83208364: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83208368: 809C0004  lwz r4, 4(r28)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 8320836C: 4BFFB035  bl 0x832033a0
	ctx.lr = 0x83208370;
	sub_832033A0(ctx, base);
	// 83208370: 93C30000  stw r30, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 83208374: 937E0048  stw r27, 0x48(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(72 as u32), ctx.r[27].u32 ) };
	// 83208378: 576B103A  slwi r11, r27, 2
	ctx.r[11].u32 = ctx.r[27].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8320837C: 815D0048  lwz r10, 0x48(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(72 as u32) ) } as u64;
	// 83208380: 3B7B0001  addi r27, r27, 1
	ctx.r[27].s64 = ctx.r[27].s64 + 1;
	// 83208384: 7FDDF378  mr r29, r30
	ctx.r[29].u64 = ctx.r[30].u64;
	// 83208388: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8320838C: 7D4B492E  stwx r10, r11, r9
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32), ctx.r[10].u32) };
	// 83208390: 817E0048  lwz r11, 0x48(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(72 as u32) ) } as u64;
	// 83208394: 815F0018  lwz r10, 0x18(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 83208398: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8320839C: 7FCB512E  stwx r30, r11, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[30].u32) };
	// 832083A0: 817E0048  lwz r11, 0x48(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(72 as u32) ) } as u64;
	// 832083A4: 815F0020  lwz r10, 0x20(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 832083A8: 814A0864  lwz r10, 0x864(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2148 as u32) ) } as u64;
	// 832083AC: 915E0080  stw r10, 0x80(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(128 as u32), ctx.r[10].u32 ) };
	// 832083B0: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832083B4: 5569103A  slwi r9, r11, 2
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 832083B8: 7D69512E  stwx r11, r9, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32), ctx.r[11].u32) };
	// 832083BC: 815E0048  lwz r10, 0x48(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(72 as u32) ) } as u64;
	// 832083C0: 4BFFFF80  b 0x83208340
	pc = 0x83208340; continue 'dispatch;
	// 832083C4: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 832083C8: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 832083CC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 832083D0: 419A0010  beq cr6, 0x832083e0
	if ctx.cr[6].eq {
	pc = 0x832083E0; continue 'dispatch;
	}
	// 832083D4: 4800D07D  bl 0x83215450
	ctx.lr = 0x832083D8;
	sub_83215450(ctx, base);
	// 832083D8: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 832083DC: 4BFFFF70  b 0x8320834c
	pc = 0x8320834C; continue 'dispatch;
	// 832083E0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 832083E4: 4800B865  bl 0x83213c48
	ctx.lr = 0x832083E8;
	sub_83213C48(ctx, base);
	// 832083E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832083EC: 4BAA1068  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832083F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832083F0 size=724
    let mut pc: u32 = 0x832083F0;
    'dispatch: loop {
        match pc {
            0x832083F0 => {
    //   block [0x832083F0..0x832086C4)
	// 832083F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832083F4: 4BAA0FFD  bl 0x82ca93f0
	ctx.lr = 0x832083F8;
	sub_82CA93D0(ctx, base);
	// 832083F8: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832083FC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83208400: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 83208404: 82CB00A4  lwz r22, 0xa4(r11)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(164 as u32) ) } as u64;
	// 83208408: 81560048  lwz r10, 0x48(r22)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(72 as u32) ) } as u64;
	// 8320840C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 83208410: 409A0024  bne cr6, 0x83208434
	if !ctx.cr[6].eq {
	pc = 0x83208434; continue 'dispatch;
	}
	// 83208414: 816B0088  lwz r11, 0x88(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(136 as u32) ) } as u64;
	// 83208418: 48000010  b 0x83208428
	pc = 0x83208428; continue 'dispatch;
	// 8320841C: 814B0048  lwz r10, 0x48(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(72 as u32) ) } as u64;
	// 83208420: 914B004C  stw r10, 0x4c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(76 as u32), ctx.r[10].u32 ) };
	// 83208424: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83208428: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8320842C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 83208430: 409AFFEC  bne cr6, 0x8320841c
	if !ctx.cr[6].eq {
	pc = 0x8320841C; continue 'dispatch;
	}
	// 83208434: 7EC4B378  mr r4, r22
	ctx.r[4].u64 = ctx.r[22].u64;
	// 83208438: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8320843C: 4BFFFE5D  bl 0x83208298
	ctx.lr = 0x83208440;
	sub_83208298(ctx, base);
	// 83208440: 807F0020  lwz r3, 0x20(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 83208444: 80BF001C  lwz r5, 0x1c(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 83208448: 809F0018  lwz r4, 0x18(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 8320844C: 4BFDDBC5  bl 0x831e6010
	ctx.lr = 0x83208450;
	sub_831E6010(ctx, base);
	// 83208450: 3AE00000  li r23, 0
	ctx.r[23].s64 = 0;
	// 83208454: 831F001C  lwz r24, 0x1c(r31)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 83208458: 2B180001  cmplwi cr6, r24, 1
	ctx.cr[6].compare_u32(ctx.r[24].u32, 1 as u32, &mut ctx.xer);
	// 8320845C: 40990168  ble cr6, 0x832085c4
	if !ctx.cr[6].gt {
	pc = 0x832085C4; continue 'dispatch;
	}
	// 83208460: 571D103A  slwi r29, r24, 2
	ctx.r[29].u32 = ctx.r[24].u32.wrapping_shl(2);
	ctx.r[29].u64 = ctx.r[29].u32 as u64;
	// 83208464: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 83208468: 7EFCBB78  mr r28, r23
	ctx.r[28].u64 = ctx.r[23].u64;
	// 8320846C: 7EFBBB78  mr r27, r23
	ctx.r[27].u64 = ctx.r[23].u64;
	// 83208470: 3B400001  li r26, 1
	ctx.r[26].s64 = 1;
	// 83208474: 7F3D582E  lwzx r25, r29, r11
	ctx.r[25].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 83208478: 8179003C  lwz r11, 0x3c(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(60 as u32) ) } as u64;
	// 8320847C: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83208480: 7F1A5040  cmplw cr6, r26, r10
	ctx.cr[6].compare_u32(ctx.r[26].u32, ctx.r[10].u32, &mut ctx.xer);
	// 83208484: 4099000C  ble cr6, 0x83208490
	if !ctx.cr[6].gt {
	pc = 0x83208490; continue 'dispatch;
	}
	// 83208488: 7EEBBB78  mr r11, r23
	ctx.r[11].u64 = ctx.r[23].u64;
	// 8320848C: 48000010  b 0x8320849c
	pc = 0x8320849C; continue 'dispatch;
	// 83208490: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83208494: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83208498: 7F8AD82E  lwzx r28, r10, r27
	ctx.r[28].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[27].u32)) } as u64;
	// 8320849C: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 832084A0: 41820038  beq 0x832084d8
	if ctx.cr[0].eq {
	pc = 0x832084D8; continue 'dispatch;
	}
	// 832084A4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832084A8: 83DF0004  lwz r30, 4(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832084AC: 809C0048  lwz r4, 0x48(r28)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(72 as u32) ) } as u64;
	// 832084B0: 4BFFFD99  bl 0x83208248
	ctx.lr = 0x832084B4;
	sub_83208248(ctx, base);
	// 832084B4: 546B103A  slwi r11, r3, 2
	ctx.r[11].u32 = ctx.r[3].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 832084B8: 7D5DF02E  lwzx r10, r29, r30
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 832084BC: 7D6BF02E  lwzx r11, r11, r30
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 832084C0: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 832084C4: 40980008  bge cr6, 0x832084cc
	if !ctx.cr[6].lt {
	pc = 0x832084CC; continue 'dispatch;
	}
	// 832084C8: 7D7DF12E  stwx r11, r29, r30
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[29].u32.wrapping_add(ctx.r[30].u32), ctx.r[11].u32) };
	// 832084CC: 3B5A0001  addi r26, r26, 1
	ctx.r[26].s64 = ctx.r[26].s64 + 1;
	// 832084D0: 3B7B0004  addi r27, r27, 4
	ctx.r[27].s64 = ctx.r[27].s64 + 4;
	// 832084D4: 4BFFFFA4  b 0x83208478
	pc = 0x83208478; continue 'dispatch;
	// 832084D8: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 832084DC: 3880000C  li r4, 0xc
	ctx.r[4].s64 = 12;
	// 832084E0: 83CB05B0  lwz r30, 0x5b0(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1456 as u32) ) } as u64;
	// 832084E4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 832084E8: 4BFC3CB1  bl 0x831cc198
	ctx.lr = 0x832084EC;
	sub_831CC198(ctx, base);
	// 832084EC: 35630004  addic. r11, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 832084F0: 93C30000  stw r30, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 832084F4: 41820010  beq 0x83208504
	if ctx.cr[0].eq {
	pc = 0x83208504; continue 'dispatch;
	}
	// 832084F8: 92EB0000  stw r23, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[23].u32 ) };
	// 832084FC: 92EB0004  stw r23, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[23].u32 ) };
	// 83208500: 48000008  b 0x83208508
	pc = 0x83208508; continue 'dispatch;
	// 83208504: 7EEBBB78  mr r11, r23
	ctx.r[11].u64 = ctx.r[23].u64;
	// 83208508: 930B0000  stw r24, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[24].u32 ) };
	// 8320850C: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83208510: 813F0014  lwz r9, 0x14(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 83208514: 7D5D502E  lwzx r10, r29, r10
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 83208518: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8320851C: 7D4A482E  lwzx r10, r10, r9
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 83208520: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 83208524: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83208528: 813F0014  lwz r9, 0x14(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 8320852C: 7D5D502E  lwzx r10, r29, r10
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 83208530: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83208534: 7D6A492E  stwx r11, r10, r9
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32), ctx.r[11].u32) };
	// 83208538: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8320853C: 7D7D582E  lwzx r11, r29, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 83208540: 815F000C  lwz r10, 0xc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 83208544: 7D7D512E  stwx r11, r29, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[29].u32.wrapping_add(ctx.r[10].u32), ctx.r[11].u32) };
	// 83208548: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8320854C: 815F0014  lwz r10, 0x14(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 83208550: 7D7D582E  lwzx r11, r29, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 83208554: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83208558: 7FCB502E  lwzx r30, r11, r10
	ctx.r[30].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 8320855C: 48000050  b 0x832085ac
	pc = 0x832085AC; continue 'dispatch;
	// 83208560: 839E0000  lwz r28, 0(r30)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83208564: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83208568: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8320856C: 4BFFFCDD  bl 0x83208248
	ctx.lr = 0x83208570;
	sub_83208248(ctx, base);
	// 83208570: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83208574: 578B103A  slwi r11, r28, 2
	ctx.r[11].u32 = ctx.r[28].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83208578: 5469103A  slwi r9, r3, 2
	ctx.r[9].u32 = ctx.r[3].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 8320857C: 7D0B502E  lwzx r8, r11, r10
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 83208580: 7D49502E  lwzx r10, r9, r10
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 83208584: 7F085040  cmplw cr6, r8, r10
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[10].u32, &mut ctx.xer);
	// 83208588: 409A0018  bne cr6, 0x832085a0
	if !ctx.cr[6].eq {
	pc = 0x832085A0; continue 'dispatch;
	}
	// 8320858C: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83208590: 813F0008  lwz r9, 8(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83208594: 7D4B502E  lwzx r10, r11, r10
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 83208598: 7D49592E  stwx r10, r9, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32), ctx.r[10].u32) };
	// 8320859C: 4800000C  b 0x832085a8
	pc = 0x832085A8; continue 'dispatch;
	// 832085A0: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 832085A4: 7C6A592E  stwx r3, r10, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32), ctx.r[3].u32) };
	// 832085A8: 83DE0004  lwz r30, 4(r30)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 832085AC: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 832085B0: 409AFFB0  bne cr6, 0x83208560
	if !ctx.cr[6].eq {
	pc = 0x83208560; continue 'dispatch;
	}
	// 832085B4: 3B18FFFF  addi r24, r24, -1
	ctx.r[24].s64 = ctx.r[24].s64 + -1;
	// 832085B8: 3BBDFFFC  addi r29, r29, -4
	ctx.r[29].s64 = ctx.r[29].s64 + -4;
	// 832085BC: 2B180001  cmplwi cr6, r24, 1
	ctx.cr[6].compare_u32(ctx.r[24].u32, 1 as u32, &mut ctx.xer);
	// 832085C0: 4199FEA4  bgt cr6, 0x83208464
	if ctx.cr[6].gt {
	pc = 0x83208464; continue 'dispatch;
	}
	// 832085C4: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 832085C8: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 832085CC: 92EB0004  stw r23, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[23].u32 ) };
	// 832085D0: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 832085D4: 2B0B0002  cmplwi cr6, r11, 2
	ctx.cr[6].compare_u32(ctx.r[11].u32, 2 as u32, &mut ctx.xer);
	// 832085D8: 41980048  blt cr6, 0x83208620
	if ctx.cr[6].lt {
	pc = 0x83208620; continue 'dispatch;
	}
	// 832085DC: 39600008  li r11, 8
	ctx.r[11].s64 = 8;
	// 832085E0: 813F0004  lwz r9, 4(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832085E4: 811F0008  lwz r8, 8(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 832085E8: 7D2B482E  lwzx r9, r11, r9
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 832085EC: 7D08582E  lwzx r8, r8, r11
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 832085F0: 7F084840  cmplw cr6, r8, r9
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[9].u32, &mut ctx.xer);
	// 832085F4: 419A0018  beq cr6, 0x8320860c
	if ctx.cr[6].eq {
	pc = 0x8320860C; continue 'dispatch;
	}
	// 832085F8: 813F0008  lwz r9, 8(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 832085FC: 7D0B482E  lwzx r8, r11, r9
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 83208600: 5508103A  slwi r8, r8, 2
	ctx.r[8].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 83208604: 7D08482E  lwzx r8, r8, r9
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 83208608: 7D0B492E  stwx r8, r11, r9
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32), ctx.r[8].u32) };
	// 8320860C: 813F001C  lwz r9, 0x1c(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 83208610: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 83208614: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 83208618: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 8320861C: 4099FFC4  ble cr6, 0x832085e0
	if !ctx.cr[6].gt {
	pc = 0x832085E0; continue 'dispatch;
	}
	// 83208620: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 83208624: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 83208628: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 8320862C: 41980058  blt cr6, 0x83208684
	if ctx.cr[6].lt {
	pc = 0x83208684; continue 'dispatch;
	}
	// 83208630: 2B1E0001  cmplwi cr6, r30, 1
	ctx.cr[6].compare_u32(ctx.r[30].u32, 1 as u32, &mut ctx.xer);
	// 83208634: 409A000C  bne cr6, 0x83208640
	if !ctx.cr[6].eq {
	pc = 0x83208640; continue 'dispatch;
	}
	// 83208638: 7EE3BB78  mr r3, r23
	ctx.r[3].u64 = ctx.r[23].u64;
	// 8320863C: 4800001C  b 0x83208658
	pc = 0x83208658; continue 'dispatch;
	// 83208640: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83208644: 57CA103A  slwi r10, r30, 2
	ctx.r[10].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83208648: 813F0018  lwz r9, 0x18(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 8320864C: 7D6A582E  lwzx r11, r10, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 83208650: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83208654: 7C6B482E  lwzx r3, r11, r9
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 83208658: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 8320865C: 57CA103A  slwi r10, r30, 2
	ctx.r[10].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83208660: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83208664: 7C8A582E  lwzx r4, r10, r11
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 83208668: 90640054  stw r3, 0x54(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 8320866C: 419A0008  beq cr6, 0x83208674
	if ctx.cr[6].eq {
	pc = 0x83208674; continue 'dispatch;
	}
	// 83208670: 480006B1  bl 0x83208d20
	ctx.lr = 0x83208674;
	sub_83208D20(ctx, base);
	// 83208674: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 83208678: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 8320867C: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 83208680: 4099FFB0  ble cr6, 0x83208630
	if !ctx.cr[6].gt {
	pc = 0x83208630; continue 'dispatch;
	}
	// 83208684: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83208688: 4BFFF959  bl 0x83207fe0
	ctx.lr = 0x8320868C;
	sub_83207FE0(ctx, base);
	// 8320868C: 8176004C  lwz r11, 0x4c(r22)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(76 as u32) ) } as u64;
	// 83208690: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83208694: 409A0028  bne cr6, 0x832086bc
	if !ctx.cr[6].eq {
	pc = 0x832086BC; continue 'dispatch;
	}
	// 83208698: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 8320869C: 816B0088  lwz r11, 0x88(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(136 as u32) ) } as u64;
	// 832086A0: 48000010  b 0x832086b0
	pc = 0x832086B0; continue 'dispatch;
	// 832086A4: 814B004C  lwz r10, 0x4c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(76 as u32) ) } as u64;
	// 832086A8: 914B0048  stw r10, 0x48(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(72 as u32), ctx.r[10].u32 ) };
	// 832086AC: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 832086B0: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 832086B4: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 832086B8: 409AFFEC  bne cr6, 0x832086a4
	if !ctx.cr[6].eq {
	pc = 0x832086A4; continue 'dispatch;
	}
	// 832086BC: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 832086C0: 4BAA0D80  b 0x82ca9440
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832086C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832086C8 size=140
    let mut pc: u32 = 0x832086C8;
    'dispatch: loop {
        match pc {
            0x832086C8 => {
    //   block [0x832086C8..0x83208754)
	// 832086C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832086CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832086D0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 832086D4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832086D8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832086DC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 832086E0: 3880002C  li r4, 0x2c
	ctx.r[4].s64 = 44;
	// 832086E4: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 832086E8: 83CB05B0  lwz r30, 0x5b0(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1456 as u32) ) } as u64;
	// 832086EC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 832086F0: 4BFC3AA9  bl 0x831cc198
	ctx.lr = 0x832086F4;
	sub_831CC198(ctx, base);
	// 832086F4: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 832086F8: 346B0004  addic. r3, r11, 4
	ctx.xer.ca = (ctx.r[11].u32 > (!(4 as u32)));
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 832086FC: 93CB0000  stw r30, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 83208700: 41820014  beq 0x83208714
	if ctx.cr[0].eq {
	pc = 0x83208714; continue 'dispatch;
	}
	// 83208704: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83208708: 4BFFF6D1  bl 0x83207dd8
	ctx.lr = 0x8320870C;
	sub_83207DD8(ctx, base);
	// 8320870C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83208710: 48000008  b 0x83208718
	pc = 0x83208718; continue 'dispatch;
	// 83208714: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 83208718: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8320871C: 4BFFFCD5  bl 0x832083f0
	ctx.lr = 0x83208720;
	sub_832083F0(ctx, base);
	// 83208720: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 83208724: 419A0018  beq cr6, 0x8320873c
	if ctx.cr[6].eq {
	pc = 0x8320873C; continue 'dispatch;
	}
	// 83208728: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8320872C: 4BFFF79D  bl 0x83207ec8
	ctx.lr = 0x83208730;
	sub_83207EC8(ctx, base);
	// 83208730: 389FFFFC  addi r4, r31, -4
	ctx.r[4].s64 = ctx.r[31].s64 + -4;
	// 83208734: 807FFFFC  lwz r3, -4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-4 as u32) ) } as u64;
	// 83208738: 4BFC3B91  bl 0x831cc2c8
	ctx.lr = 0x8320873C;
	sub_831CC2C8(ctx, base);
	// 8320873C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83208740: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83208744: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83208748: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8320874C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83208750: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83208758(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83208758 size=68
    let mut pc: u32 = 0x83208758;
    'dispatch: loop {
        match pc {
            0x83208758 => {
    //   block [0x83208758..0x8320879C)
	// 83208758: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8320875C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83208760: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83208764: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83208768: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8320876C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83208770: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83208774: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83208778: 809F0074  lwz r4, 0x74(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(116 as u32) ) } as u64;
	// 8320877C: 4BFFA92D  bl 0x832030a8
	ctx.lr = 0x83208780;
	sub_832030A8(ctx, base);
	// 83208780: 93FE03B4  stw r31, 0x3b4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(948 as u32), ctx.r[31].u32 ) };
	// 83208784: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83208788: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8320878C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83208790: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83208794: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83208798: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832087A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832087A0 size=68
    let mut pc: u32 = 0x832087A0;
    'dispatch: loop {
        match pc {
            0x832087A0 => {
    //   block [0x832087A0..0x832087E4)
	// 832087A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832087A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832087A8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 832087AC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832087B0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832087B4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 832087B8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 832087BC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 832087C0: 809F0070  lwz r4, 0x70(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(112 as u32) ) } as u64;
	// 832087C4: 4BFFA905  bl 0x832030c8
	ctx.lr = 0x832087C8;
	sub_832030C8(ctx, base);
	// 832087C8: 93FE03B4  stw r31, 0x3b4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(948 as u32), ctx.r[31].u32 ) };
	// 832087CC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 832087D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832087D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832087D8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 832087DC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832087E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832087E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832087E8 size=96
    let mut pc: u32 = 0x832087E8;
    'dispatch: loop {
        match pc {
            0x832087E8 => {
    //   block [0x832087E8..0x83208848)
	// 832087E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832087EC: 4BAA0C21  bl 0x82ca940c
	ctx.lr = 0x832087F0;
	sub_82CA93D0(ctx, base);
	// 832087F0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832087F4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 832087F8: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 832087FC: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 83208800: 817F03B4  lwz r11, 0x3b4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(948 as u32) ) } as u64;
	// 83208804: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83208808: 409A0028  bne cr6, 0x83208830
	if !ctx.cr[6].eq {
	pc = 0x83208830; continue 'dispatch;
	}
	// 8320880C: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 83208810: 3D408209  lis r10, -0x7df7
	ctx.r[10].s64 = -2113339392;
	// 83208814: 3D208208  lis r9, -0x7df8
	ctx.r[9].s64 = -2113404928;
	// 83208818: 38CBBFB0  addi r6, r11, -0x4050
	ctx.r[6].s64 = ctx.r[11].s64 + -16464;
	// 8320881C: 38AABF90  addi r5, r10, -0x4070
	ctx.r[5].s64 = ctx.r[10].s64 + -16496;
	// 83208820: 3889B734  addi r4, r9, -0x48cc
	ctx.r[4].s64 = ctx.r[9].s64 + -18636;
	// 83208824: 38E000B5  li r7, 0xb5
	ctx.r[7].s64 = 181;
	// 83208828: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8320882C: 4BF01585  bl 0x83109db0
	ctx.lr = 0x83208830;
	sub_83109DB0(ctx, base);
	// 83208830: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83208834: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83208838: 4BFFA891  bl 0x832030c8
	ctx.lr = 0x8320883C;
	sub_832030C8(ctx, base);
	// 8320883C: 93BE03B4  stw r29, 0x3b4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(948 as u32), ctx.r[29].u32 ) };
	// 83208840: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83208844: 4BAA0C18  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83208848(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83208848 size=96
    let mut pc: u32 = 0x83208848;
    'dispatch: loop {
        match pc {
            0x83208848 => {
    //   block [0x83208848..0x832088A8)
	// 83208848: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8320884C: 4BAA0BC1  bl 0x82ca940c
	ctx.lr = 0x83208850;
	sub_82CA93D0(ctx, base);
	// 83208850: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83208854: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 83208858: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8320885C: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 83208860: 817F03B4  lwz r11, 0x3b4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(948 as u32) ) } as u64;
	// 83208864: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83208868: 409A0028  bne cr6, 0x83208890
	if !ctx.cr[6].eq {
	pc = 0x83208890; continue 'dispatch;
	}
	// 8320886C: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 83208870: 3D408209  lis r10, -0x7df7
	ctx.r[10].s64 = -2113339392;
	// 83208874: 3D208208  lis r9, -0x7df8
	ctx.r[9].s64 = -2113404928;
	// 83208878: 38CBBFB0  addi r6, r11, -0x4050
	ctx.r[6].s64 = ctx.r[11].s64 + -16464;
	// 8320887C: 38AAC000  addi r5, r10, -0x4000
	ctx.r[5].s64 = ctx.r[10].s64 + -16384;
	// 83208880: 3889B734  addi r4, r9, -0x48cc
	ctx.r[4].s64 = ctx.r[9].s64 + -18636;
	// 83208884: 38E000C1  li r7, 0xc1
	ctx.r[7].s64 = 193;
	// 83208888: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8320888C: 4BF01525  bl 0x83109db0
	ctx.lr = 0x83208890;
	sub_83109DB0(ctx, base);
	// 83208890: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83208894: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83208898: 4BFFA811  bl 0x832030a8
	ctx.lr = 0x8320889C;
	sub_832030A8(ctx, base);
	// 8320889C: 93BE03B4  stw r29, 0x3b4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(948 as u32), ctx.r[29].u32 ) };
	// 832088A0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 832088A4: 4BAA0BB8  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832088A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832088A8 size=72
    let mut pc: u32 = 0x832088A8;
    'dispatch: loop {
        match pc {
            0x832088A8 => {
    //   block [0x832088A8..0x832088F0)
	// 832088A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832088AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832088B0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 832088B4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832088B8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832088BC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 832088C0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 832088C4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 832088C8: 809F0088  lwz r4, 0x88(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(136 as u32) ) } as u64;
	// 832088CC: 4BFFA7FD  bl 0x832030c8
	ctx.lr = 0x832088D0;
	sub_832030C8(ctx, base);
	// 832088D0: 93DF0088  stw r30, 0x88(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(136 as u32), ctx.r[30].u32 ) };
	// 832088D4: 93FE03B4  stw r31, 0x3b4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(948 as u32), ctx.r[31].u32 ) };
	// 832088D8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 832088DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832088E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832088E4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 832088E8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832088EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832088F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832088F0 size=136
    let mut pc: u32 = 0x832088F0;
    'dispatch: loop {
        match pc {
            0x832088F0 => {
    //   block [0x832088F0..0x83208978)
	// 832088F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832088F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832088F8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 832088FC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83208900: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83208904: 83E3001C  lwz r31, 0x1c(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 83208908: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8320890C: 4800003C  b 0x83208948
	pc = 0x83208948; continue 'dispatch;
	// 83208910: 817F00E4  lwz r11, 0xe4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(228 as u32) ) } as u64;
	// 83208914: 556B07FF  clrlwi. r11, r11, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83208918: 4182002C  beq 0x83208944
	if ctx.cr[0].eq {
	pc = 0x83208944; continue 'dispatch;
	}
	// 8320891C: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 83208920: 2F1E0001  cmpwi cr6, r30, 1
	ctx.cr[6].compare_i32(ctx.r[30].s32, 1, &mut ctx.xer);
	// 83208924: 409A0020  bne cr6, 0x83208944
	if !ctx.cr[6].eq {
	pc = 0x83208944; continue 'dispatch;
	}
	// 83208928: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8320892C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83208930: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 83208934: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83208938: 4E800421  bctrl
	ctx.lr = 0x8320893C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8320893C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83208940: 41820030  beq 0x83208970
	if ctx.cr[0].eq {
	pc = 0x83208970; continue 'dispatch;
	}
	// 83208944: 83FF0008  lwz r31, 8(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83208948: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8320894C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83208950: 409AFFC0  bne cr6, 0x83208910
	if !ctx.cr[6].eq {
	pc = 0x83208910; continue 'dispatch;
	}
	// 83208954: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83208958: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8320895C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83208960: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83208964: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83208968: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8320896C: 4E800020  blr
	return;
	// 83208970: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83208974: 4BFFFFE4  b 0x83208958
	pc = 0x83208958; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83208978(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83208978 size=608
    let mut pc: u32 = 0x83208978;
    'dispatch: loop {
        match pc {
            0x83208978 => {
    //   block [0x83208978..0x83208BD8)
	// 83208978: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8320897C: 4BAA0A89  bl 0x82ca9404
	ctx.lr = 0x83208980;
	sub_82CA93D0(ctx, base);
	// 83208980: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83208984: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83208988: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 8320898C: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 83208990: 396BC01C  addi r11, r11, -0x3fe4
	ctx.r[11].s64 = ctx.r[11].s64 + -16356;
	// 83208994: 3B9F0014  addi r28, r31, 0x14
	ctx.r[28].s64 = ctx.r[31].s64 + 20;
	// 83208998: 909F000C  stw r4, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[4].u32 ) };
	// 8320899C: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 832089A0: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 832089A4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 832089A8: 93DF0008  stw r30, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 832089AC: 93DF0010  stw r30, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[30].u32 ) };
	// 832089B0: 4BFFA759  bl 0x83203108
	ctx.lr = 0x832089B4;
	sub_83203108(ctx, base);
	// 832089B4: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 832089B8: 39200400  li r9, 0x400
	ctx.r[9].s64 = 1024;
	// 832089BC: 3900FFFF  li r8, -1
	ctx.r[8].s64 = -1;
	// 832089C0: 38800014  li r4, 0x14
	ctx.r[4].s64 = 20;
	// 832089C4: 814B056C  lwz r10, 0x56c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1388 as u32) ) } as u64;
	// 832089C8: 915F0030  stw r10, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[10].u32 ) };
	// 832089CC: 814B056C  lwz r10, 0x56c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1388 as u32) ) } as u64;
	// 832089D0: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 832089D4: 914B056C  stw r10, 0x56c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(1388 as u32), ctx.r[10].u32 ) };
	// 832089D8: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 832089DC: 93DF0034  stw r30, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[30].u32 ) };
	// 832089E0: 913F0048  stw r9, 0x48(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[9].u32 ) };
	// 832089E4: 913F004C  stw r9, 0x4c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(76 as u32), ctx.r[9].u32 ) };
	// 832089E8: 93DF0050  stw r30, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 832089EC: 93DF0054  stw r30, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[30].u32 ) };
	// 832089F0: 93DF0060  stw r30, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[30].u32 ) };
	// 832089F4: 93DF0064  stw r30, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[30].u32 ) };
	// 832089F8: 93DF0068  stw r30, 0x68(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(104 as u32), ctx.r[30].u32 ) };
	// 832089FC: 93DF006C  stw r30, 0x6c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(108 as u32), ctx.r[30].u32 ) };
	// 83208A00: 93DF0078  stw r30, 0x78(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(120 as u32), ctx.r[30].u32 ) };
	// 83208A04: 9BDF007C  stb r30, 0x7c(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(124 as u32), ctx.r[30].u8 ) };
	// 83208A08: 911F0080  stw r8, 0x80(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(128 as u32), ctx.r[8].u32 ) };
	// 83208A0C: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 83208A10: 836B05AC  lwz r27, 0x5ac(r11)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1452 as u32) ) } as u64;
	// 83208A14: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 83208A18: 4BFC3781  bl 0x831cc198
	ctx.lr = 0x83208A1C;
	sub_831CC198(ctx, base);
	// 83208A1C: 37A30004  addic. r29, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[29].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 83208A20: 93630000  stw r27, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 83208A24: 41820018  beq 0x83208a3c
	if ctx.cr[0].eq {
	pc = 0x83208A3C; continue 'dispatch;
	}
	// 83208A28: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 83208A2C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83208A30: 808B05AC  lwz r4, 0x5ac(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1452 as u32) ) } as u64;
	// 83208A34: 4BFDD03D  bl 0x831e5a70
	ctx.lr = 0x83208A38;
	sub_831E5A70(ctx, base);
	// 83208A38: 48000008  b 0x83208a40
	pc = 0x83208A40; continue 'dispatch;
	// 83208A3C: 7FDDF378  mr r29, r30
	ctx.r[29].u64 = ctx.r[30].u64;
	// 83208A40: 93BF002C  stw r29, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[29].u32 ) };
	// 83208A44: 38800014  li r4, 0x14
	ctx.r[4].s64 = 20;
	// 83208A48: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 83208A4C: 836B05AC  lwz r27, 0x5ac(r11)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1452 as u32) ) } as u64;
	// 83208A50: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 83208A54: 4BFC3745  bl 0x831cc198
	ctx.lr = 0x83208A58;
	sub_831CC198(ctx, base);
	// 83208A58: 37A30004  addic. r29, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[29].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 83208A5C: 93630000  stw r27, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 83208A60: 41820018  beq 0x83208a78
	if ctx.cr[0].eq {
	pc = 0x83208A78; continue 'dispatch;
	}
	// 83208A64: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 83208A68: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83208A6C: 808B05AC  lwz r4, 0x5ac(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1452 as u32) ) } as u64;
	// 83208A70: 4BFDD001  bl 0x831e5a70
	ctx.lr = 0x83208A74;
	sub_831E5A70(ctx, base);
	// 83208A74: 48000008  b 0x83208a7c
	pc = 0x83208A7C; continue 'dispatch;
	// 83208A78: 7FDDF378  mr r29, r30
	ctx.r[29].u64 = ctx.r[30].u64;
	// 83208A7C: 93BF005C  stw r29, 0x5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[29].u32 ) };
	// 83208A80: 38800014  li r4, 0x14
	ctx.r[4].s64 = 20;
	// 83208A84: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 83208A88: 836B05AC  lwz r27, 0x5ac(r11)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1452 as u32) ) } as u64;
	// 83208A8C: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 83208A90: 4BFC3709  bl 0x831cc198
	ctx.lr = 0x83208A94;
	sub_831CC198(ctx, base);
	// 83208A94: 37A30004  addic. r29, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[29].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 83208A98: 93630000  stw r27, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 83208A9C: 41820018  beq 0x83208ab4
	if ctx.cr[0].eq {
	pc = 0x83208AB4; continue 'dispatch;
	}
	// 83208AA0: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 83208AA4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83208AA8: 808B05AC  lwz r4, 0x5ac(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1452 as u32) ) } as u64;
	// 83208AAC: 4BFDCFC5  bl 0x831e5a70
	ctx.lr = 0x83208AB0;
	sub_831E5A70(ctx, base);
	// 83208AB0: 48000008  b 0x83208ab8
	pc = 0x83208AB8; continue 'dispatch;
	// 83208AB4: 7FDDF378  mr r29, r30
	ctx.r[29].u64 = ctx.r[30].u64;
	// 83208AB8: 93BF0058  stw r29, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[29].u32 ) };
	// 83208ABC: 38800014  li r4, 0x14
	ctx.r[4].s64 = 20;
	// 83208AC0: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 83208AC4: 836B05AC  lwz r27, 0x5ac(r11)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1452 as u32) ) } as u64;
	// 83208AC8: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 83208ACC: 4BFC36CD  bl 0x831cc198
	ctx.lr = 0x83208AD0;
	sub_831CC198(ctx, base);
	// 83208AD0: 37A30004  addic. r29, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[29].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 83208AD4: 93630000  stw r27, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 83208AD8: 41820018  beq 0x83208af0
	if ctx.cr[0].eq {
	pc = 0x83208AF0; continue 'dispatch;
	}
	// 83208ADC: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 83208AE0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83208AE4: 808B05AC  lwz r4, 0x5ac(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1452 as u32) ) } as u64;
	// 83208AE8: 4BFDCF89  bl 0x831e5a70
	ctx.lr = 0x83208AEC;
	sub_831E5A70(ctx, base);
	// 83208AEC: 48000008  b 0x83208af4
	pc = 0x83208AF4; continue 'dispatch;
	// 83208AF0: 7FDDF378  mr r29, r30
	ctx.r[29].u64 = ctx.r[30].u64;
	// 83208AF4: 93BF0038  stw r29, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[29].u32 ) };
	// 83208AF8: 38800014  li r4, 0x14
	ctx.r[4].s64 = 20;
	// 83208AFC: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 83208B00: 836B05AC  lwz r27, 0x5ac(r11)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1452 as u32) ) } as u64;
	// 83208B04: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 83208B08: 4BFC3691  bl 0x831cc198
	ctx.lr = 0x83208B0C;
	sub_831CC198(ctx, base);
	// 83208B0C: 37A30004  addic. r29, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[29].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 83208B10: 93630000  stw r27, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 83208B14: 41820018  beq 0x83208b2c
	if ctx.cr[0].eq {
	pc = 0x83208B2C; continue 'dispatch;
	}
	// 83208B18: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 83208B1C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83208B20: 808B05AC  lwz r4, 0x5ac(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1452 as u32) ) } as u64;
	// 83208B24: 4BFDCF4D  bl 0x831e5a70
	ctx.lr = 0x83208B28;
	sub_831E5A70(ctx, base);
	// 83208B28: 48000008  b 0x83208b30
	pc = 0x83208B30; continue 'dispatch;
	// 83208B2C: 7FDDF378  mr r29, r30
	ctx.r[29].u64 = ctx.r[30].u64;
	// 83208B30: 93BF003C  stw r29, 0x3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[29].u32 ) };
	// 83208B34: 388003C4  li r4, 0x3c4
	ctx.r[4].s64 = 964;
	// 83208B38: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 83208B3C: 83AB05AC  lwz r29, 0x5ac(r11)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1452 as u32) ) } as u64;
	// 83208B40: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83208B44: 4BFC3655  bl 0x831cc198
	ctx.lr = 0x83208B48;
	sub_831CC198(ctx, base);
	// 83208B48: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83208B4C: 346B0004  addic. r3, r11, 4
	ctx.xer.ca = (ctx.r[11].u32 > (!(4 as u32)));
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83208B50: 93AB0000  stw r29, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 83208B54: 41820010  beq 0x83208b64
	if ctx.cr[0].eq {
	pc = 0x83208B64; continue 'dispatch;
	}
	// 83208B58: 809F000C  lwz r4, 0xc(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 83208B5C: 4BFFDEED  bl 0x83206a48
	ctx.lr = 0x83208B60;
	sub_83206A48(ctx, base);
	// 83208B60: 48000008  b 0x83208b68
	pc = 0x83208B68; continue 'dispatch;
	// 83208B64: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83208B68: 907F0070  stw r3, 0x70(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(112 as u32), ctx.r[3].u32 ) };
	// 83208B6C: 388003C4  li r4, 0x3c4
	ctx.r[4].s64 = 964;
	// 83208B70: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 83208B74: 83AB05AC  lwz r29, 0x5ac(r11)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1452 as u32) ) } as u64;
	// 83208B78: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83208B7C: 4BFC361D  bl 0x831cc198
	ctx.lr = 0x83208B80;
	sub_831CC198(ctx, base);
	// 83208B80: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83208B84: 346B0004  addic. r3, r11, 4
	ctx.xer.ca = (ctx.r[11].u32 > (!(4 as u32)));
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83208B88: 93AB0000  stw r29, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 83208B8C: 41820010  beq 0x83208b9c
	if ctx.cr[0].eq {
	pc = 0x83208B9C; continue 'dispatch;
	}
	// 83208B90: 809F000C  lwz r4, 0xc(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 83208B94: 4BFFDF0D  bl 0x83206aa0
	ctx.lr = 0x83208B98;
	sub_83206AA0(ctx, base);
	// 83208B98: 48000008  b 0x83208ba0
	pc = 0x83208BA0; continue 'dispatch;
	// 83208B9C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83208BA0: 907F0074  stw r3, 0x74(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(116 as u32), ctx.r[3].u32 ) };
	// 83208BA4: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83208BA8: 809F0070  lwz r4, 0x70(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(112 as u32) ) } as u64;
	// 83208BAC: 4BFFA5E5  bl 0x83203190
	ctx.lr = 0x83208BB0;
	sub_83203190(ctx, base);
	// 83208BB0: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83208BB4: 809F0074  lwz r4, 0x74(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(116 as u32) ) } as u64;
	// 83208BB8: 4BFFA5B1  bl 0x83203168
	ctx.lr = 0x83208BBC;
	sub_83203168(ctx, base);
	// 83208BBC: 817F0070  lwz r11, 0x70(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(112 as u32) ) } as u64;
	// 83208BC0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83208BC4: 93EB03B4  stw r31, 0x3b4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(948 as u32), ctx.r[31].u32 ) };
	// 83208BC8: 817F0074  lwz r11, 0x74(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(116 as u32) ) } as u64;
	// 83208BCC: 93EB03B4  stw r31, 0x3b4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(948 as u32), ctx.r[31].u32 ) };
	// 83208BD0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83208BD4: 4BAA0880  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83208BD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83208BD8 size=12
    let mut pc: u32 = 0x83208BD8;
    'dispatch: loop {
        match pc {
            0x83208BD8 => {
    //   block [0x83208BD8..0x83208BE4)
	// 83208BD8: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 83208BDC: 386BC04C  addi r3, r11, -0x3fb4
	ctx.r[3].s64 = ctx.r[11].s64 + -16308;
	// 83208BE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83208BE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83208BE8 size=40
    let mut pc: u32 = 0x83208BE8;
    'dispatch: loop {
        match pc {
            0x83208BE8 => {
    //   block [0x83208BE8..0x83208C10)
	// 83208BE8: 8143003C  lwz r10, 0x3c(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(60 as u32) ) } as u64;
	// 83208BEC: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83208BF0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83208BF4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83208BF8: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 83208BFC: 80CA0004  lwz r6, 4(r10)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 83208C00: 7F073040  cmplw cr6, r7, r6
	ctx.cr[6].compare_u32(ctx.r[7].u32, ctx.r[6].u32, &mut ctx.xer);
	// 83208C04: 4099000C  ble cr6, 0x83208c10
	if !ctx.cr[6].gt {
		sub_83208C10(ctx, base);
		return;
	}
	// 83208C08: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83208C0C: 48000010  b 0x83208c1c
	sub_83208C10(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83208C10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83208C10 size=28
    let mut pc: u32 = 0x83208C10;
    'dispatch: loop {
        match pc {
            0x83208C10 => {
    //   block [0x83208C10..0x83208C2C)
	// 83208C10: 810A0008  lwz r8, 8(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 83208C14: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 83208C18: 7D08582E  lwzx r8, r8, r11
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 83208C1C: 5529063F  clrlwi. r9, r9, 0x18
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 83208C20: 4182001C  beq 0x83208c3c
	if ctx.cr[0].eq {
		sub_83208C3C(ctx, base);
		return;
	}
	// 83208C24: 7F044040  cmplw cr6, r4, r8
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[8].u32, &mut ctx.xer);
	// 83208C28: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83208C2C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83208C2C size=16
    let mut pc: u32 = 0x83208C2C;
    'dispatch: loop {
        match pc {
            0x83208C2C => {
    //   block [0x83208C2C..0x83208C3C)
	// 83208C2C: 38630001  addi r3, r3, 1
	ctx.r[3].s64 = ctx.r[3].s64 + 1;
	// 83208C30: 38E70001  addi r7, r7, 1
	ctx.r[7].s64 = ctx.r[7].s64 + 1;
	// 83208C34: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 83208C38: 4BFFFFC8  b 0x83208c00
	sub_83208BE8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83208C3C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83208C3C size=8
    let mut pc: u32 = 0x83208C3C;
    'dispatch: loop {
        match pc {
            0x83208C3C => {
    //   block [0x83208C3C..0x83208C44)
	// 83208C3C: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 83208C40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83208C48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83208C48 size=36
    let mut pc: u32 = 0x83208C48;
    'dispatch: loop {
        match pc {
            0x83208C48 => {
    //   block [0x83208C48..0x83208C6C)
	// 83208C48: 81630038  lwz r11, 0x38(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(56 as u32) ) } as u64;
	// 83208C4C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83208C50: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83208C54: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 83208C58: 80EB0004  lwz r7, 4(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83208C5C: 7F083840  cmplw cr6, r8, r7
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[7].u32, &mut ctx.xer);
	// 83208C60: 4099000C  ble cr6, 0x83208c6c
	if !ctx.cr[6].gt {
		sub_83208C6C(ctx, base);
		return;
	}
	// 83208C64: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83208C68: 48000010  b 0x83208c78
	sub_83208C6C(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83208C6C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83208C6C size=32
    let mut pc: u32 = 0x83208C6C;
    'dispatch: loop {
        match pc {
            0x83208C6C => {
    //   block [0x83208C6C..0x83208C8C)
	// 83208C6C: 80CB0008  lwz r6, 8(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83208C70: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 83208C74: 7C66502E  lwzx r3, r6, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[6].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 83208C78: 5529063F  clrlwi. r9, r9, 0x18
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 83208C7C: 4182001C  beq 0x83208c98
	if ctx.cr[0].eq {
		sub_83208C98(ctx, base);
		return;
	}
	// 83208C80: 81230080  lwz r9, 0x80(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(128 as u32) ) } as u64;
	// 83208C84: 7F092000  cmpw cr6, r9, r4
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[4].s32, &mut ctx.xer);
	// 83208C88: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83208C8C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83208C8C size=12
    let mut pc: u32 = 0x83208C8C;
    'dispatch: loop {
        match pc {
            0x83208C8C => {
    //   block [0x83208C8C..0x83208C98)
	// 83208C8C: 39080001  addi r8, r8, 1
	ctx.r[8].s64 = ctx.r[8].s64 + 1;
	// 83208C90: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 83208C94: 4BFFFFC8  b 0x83208c5c
	sub_83208C48(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83208C98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83208C98 size=8
    let mut pc: u32 = 0x83208C98;
    'dispatch: loop {
        match pc {
            0x83208C98 => {
    //   block [0x83208C98..0x83208CA0)
	// 83208C98: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83208C9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83208CA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x83208CA0 size=24
    let mut pc: u32 = 0x83208CA0;
    'dispatch: loop {
        match pc {
            0x83208CA0 => {
    //   block [0x83208CA0..0x83208CB8)
	// 83208CA0: 81630038  lwz r11, 0x38(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(56 as u32) ) } as u64;
	// 83208CA4: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83208CA8: 216B0001  subfic r11, r11, 1
	ctx.xer.ca = ctx.r[11].u32 <= 1 as u32;
	ctx.r[11].s64 = (1 as i64) - ctx.r[11].s64;
	// 83208CAC: 7D6B5910  subfe r11, r11, r11
	let x = (!ctx.r[11].u32);
	let y = ctx.r[11].u32;
	let s = x.wrapping_add(y);
	let res = s.wrapping_add(ctx.xer.ca as u32);
	tmp.u8 = (s < x) as u8 | (res < s) as u8;
	ctx.r[11].u32 = res;
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	ctx.xer.ca = (tmp.u8 != 0);
	// 83208CB0: 556307FE  clrlwi r3, r11, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	// 83208CB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83208CB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x83208CB8 size=24
    let mut pc: u32 = 0x83208CB8;
    'dispatch: loop {
        match pc {
            0x83208CB8 => {
    //   block [0x83208CB8..0x83208CD0)
	// 83208CB8: 8163003C  lwz r11, 0x3c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(60 as u32) ) } as u64;
	// 83208CBC: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83208CC0: 216B0001  subfic r11, r11, 1
	ctx.xer.ca = ctx.r[11].u32 <= 1 as u32;
	ctx.r[11].s64 = (1 as i64) - ctx.r[11].s64;
	// 83208CC4: 7D6B5910  subfe r11, r11, r11
	let x = (!ctx.r[11].u32);
	let y = ctx.r[11].u32;
	let s = x.wrapping_add(y);
	let res = s.wrapping_add(ctx.xer.ca as u32);
	tmp.u8 = (s < x) as u8 | (res < s) as u8;
	ctx.r[11].u32 = res;
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	ctx.xer.ca = (tmp.u8 != 0);
	// 83208CC8: 556307FE  clrlwi r3, r11, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	// 83208CCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83208CD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83208CD0 size=12
    let mut pc: u32 = 0x83208CD0;
    'dispatch: loop {
        match pc {
            0x83208CD0 => {
    //   block [0x83208CD0..0x83208CDC)
	// 83208CD0: 81630038  lwz r11, 0x38(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(56 as u32) ) } as u64;
	// 83208CD4: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83208CD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83208CE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83208CE0 size=12
    let mut pc: u32 = 0x83208CE0;
    'dispatch: loop {
        match pc {
            0x83208CE0 => {
    //   block [0x83208CE0..0x83208CEC)
	// 83208CE0: 8163003C  lwz r11, 0x3c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(60 as u32) ) } as u64;
	// 83208CE4: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83208CE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83208CF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x83208CF0 size=24
    let mut pc: u32 = 0x83208CF0;
    'dispatch: loop {
        match pc {
            0x83208CF0 => {
    //   block [0x83208CF0..0x83208D08)
	// 83208CF0: 81630038  lwz r11, 0x38(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(56 as u32) ) } as u64;
	// 83208CF4: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83208CF8: 216B0000  subfic r11, r11, 0
	ctx.xer.ca = ctx.r[11].u32 <= 0 as u32;
	ctx.r[11].s64 = (0 as i64) - ctx.r[11].s64;
	// 83208CFC: 7D6B5910  subfe r11, r11, r11
	let x = (!ctx.r[11].u32);
	let y = ctx.r[11].u32;
	let s = x.wrapping_add(y);
	let res = s.wrapping_add(ctx.xer.ca as u32);
	tmp.u8 = (s < x) as u8 | (res < s) as u8;
	ctx.r[11].u32 = res;
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	ctx.xer.ca = (tmp.u8 != 0);
	// 83208D00: 556307FE  clrlwi r3, r11, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	// 83208D04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83208D08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x83208D08 size=24
    let mut pc: u32 = 0x83208D08;
    'dispatch: loop {
        match pc {
            0x83208D08 => {
    //   block [0x83208D08..0x83208D20)
	// 83208D08: 81630058  lwz r11, 0x58(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(88 as u32) ) } as u64;
	// 83208D0C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83208D10: 216B0000  subfic r11, r11, 0
	ctx.xer.ca = ctx.r[11].u32 <= 0 as u32;
	ctx.r[11].s64 = (0 as i64) - ctx.r[11].s64;
	// 83208D14: 7D6B5910  subfe r11, r11, r11
	let x = (!ctx.r[11].u32);
	let y = ctx.r[11].u32;
	let s = x.wrapping_add(y);
	let res = s.wrapping_add(ctx.xer.ca as u32);
	tmp.u8 = (s < x) as u8 | (res < s) as u8;
	ctx.r[11].u32 = res;
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	ctx.xer.ca = (tmp.u8 != 0);
	// 83208D18: 556307FE  clrlwi r3, r11, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	// 83208D1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83208D20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83208D20 size=56
    let mut pc: u32 = 0x83208D20;
    'dispatch: loop {
        match pc {
            0x83208D20 => {
    //   block [0x83208D20..0x83208D58)
	// 83208D20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83208D24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83208D28: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83208D2C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83208D30: 80630058  lwz r3, 0x58(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(88 as u32) ) } as u64;
	// 83208D34: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 83208D38: 80830004  lwz r4, 4(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 83208D3C: 4BFFA665  bl 0x832033a0
	ctx.lr = 0x83208D40;
	sub_832033A0(ctx, base);
	// 83208D40: 93E30000  stw r31, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 83208D44: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83208D48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83208D4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83208D50: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83208D54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83208D58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83208D58 size=56
    let mut pc: u32 = 0x83208D58;
    'dispatch: loop {
        match pc {
            0x83208D58 => {
    //   block [0x83208D58..0x83208D90)
	// 83208D58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83208D5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83208D60: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83208D64: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83208D68: 8063005C  lwz r3, 0x5c(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(92 as u32) ) } as u64;
	// 83208D6C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 83208D70: 80830004  lwz r4, 4(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 83208D74: 4BFFA62D  bl 0x832033a0
	ctx.lr = 0x83208D78;
	sub_832033A0(ctx, base);
	// 83208D78: 93E30000  stw r31, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 83208D7C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83208D80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83208D84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83208D88: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83208D8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83208D90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83208D90 size=232
    let mut pc: u32 = 0x83208D90;
    'dispatch: loop {
        match pc {
            0x83208D90 => {
    //   block [0x83208D90..0x83208E78)
	// 83208D90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83208D94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83208D98: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83208D9C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83208DA0: 83E30090  lwz r31, 0x90(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(144 as u32) ) } as u64;
	// 83208DA4: 480000B4  b 0x83208e58
	pc = 0x83208E58; continue 'dispatch;
	// 83208DA8: 817F0038  lwz r11, 0x38(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 83208DAC: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83208DB0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83208DB4: 409900AC  ble cr6, 0x83208e60
	if !ctx.cr[6].gt {
	pc = 0x83208E60; continue 'dispatch;
	}
	// 83208DB8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83208DBC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83208DC0: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 83208DC4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83208DC8: 4E800421  bctrl
	ctx.lr = 0x83208DCC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83208DCC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83208DD0: 40820090  bne 0x83208e60
	if !ctx.cr[0].eq {
	pc = 0x83208E60; continue 'dispatch;
	}
	// 83208DD4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83208DD8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83208DDC: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 83208DE0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83208DE4: 4E800421  bctrl
	ctx.lr = 0x83208DE8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83208DE8: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83208DEC: 40820074  bne 0x83208e60
	if !ctx.cr[0].eq {
	pc = 0x83208E60; continue 'dispatch;
	}
	// 83208DF0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83208DF4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83208DF8: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 83208DFC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83208E00: 4E800421  bctrl
	ctx.lr = 0x83208E04;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83208E04: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83208E08: 4182000C  beq 0x83208e14
	if ctx.cr[0].eq {
	pc = 0x83208E14; continue 'dispatch;
	}
	// 83208E0C: 83FF009C  lwz r31, 0x9c(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(156 as u32) ) } as u64;
	// 83208E10: 48000048  b 0x83208e58
	pc = 0x83208E58; continue 'dispatch;
	// 83208E14: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83208E18: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83208E1C: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 83208E20: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83208E24: 4E800421  bctrl
	ctx.lr = 0x83208E28;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83208E28: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83208E2C: 4182000C  beq 0x83208e38
	if ctx.cr[0].eq {
	pc = 0x83208E38; continue 'dispatch;
	}
	// 83208E30: 83FF0098  lwz r31, 0x98(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(152 as u32) ) } as u64;
	// 83208E34: 48000024  b 0x83208e58
	pc = 0x83208E58; continue 'dispatch;
	// 83208E38: 817F0038  lwz r11, 0x38(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 83208E3C: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83208E40: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 83208E44: 4099000C  ble cr6, 0x83208e50
	if !ctx.cr[6].gt {
	pc = 0x83208E50; continue 'dispatch;
	}
	// 83208E48: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83208E4C: 48000008  b 0x83208e54
	pc = 0x83208E54; continue 'dispatch;
	// 83208E50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83208E54: 83EB0000  lwz r31, 0(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83208E58: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 83208E5C: 409AFF4C  bne cr6, 0x83208da8
	if !ctx.cr[6].eq {
	pc = 0x83208DA8; continue 'dispatch;
	}
	// 83208E60: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83208E64: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83208E68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83208E6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83208E70: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83208E74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83208E78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83208E78 size=236
    let mut pc: u32 = 0x83208E78;
    'dispatch: loop {
        match pc {
            0x83208E78 => {
    //   block [0x83208E78..0x83208F64)
	// 83208E78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83208E7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83208E80: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83208E84: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83208E88: 83E30094  lwz r31, 0x94(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(148 as u32) ) } as u64;
	// 83208E8C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 83208E90: 419A00C0  beq cr6, 0x83208f50
	if ctx.cr[6].eq {
	pc = 0x83208F50; continue 'dispatch;
	}
	// 83208E94: 817F0038  lwz r11, 0x38(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 83208E98: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83208E9C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83208EA0: 409900AC  ble cr6, 0x83208f4c
	if !ctx.cr[6].gt {
	pc = 0x83208F4C; continue 'dispatch;
	}
	// 83208EA4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83208EA8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83208EAC: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 83208EB0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83208EB4: 4E800421  bctrl
	ctx.lr = 0x83208EB8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83208EB8: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83208EBC: 40820090  bne 0x83208f4c
	if !ctx.cr[0].eq {
	pc = 0x83208F4C; continue 'dispatch;
	}
	// 83208EC0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83208EC4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83208EC8: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 83208ECC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83208ED0: 4E800421  bctrl
	ctx.lr = 0x83208ED4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83208ED4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83208ED8: 40820074  bne 0x83208f4c
	if !ctx.cr[0].eq {
	pc = 0x83208F4C; continue 'dispatch;
	}
	// 83208EDC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83208EE0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83208EE4: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 83208EE8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83208EEC: 4E800421  bctrl
	ctx.lr = 0x83208EF0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83208EF0: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83208EF4: 4182000C  beq 0x83208f00
	if ctx.cr[0].eq {
	pc = 0x83208F00; continue 'dispatch;
	}
	// 83208EF8: 83FF009C  lwz r31, 0x9c(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(156 as u32) ) } as u64;
	// 83208EFC: 48000048  b 0x83208f44
	pc = 0x83208F44; continue 'dispatch;
	// 83208F00: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83208F04: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83208F08: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 83208F0C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83208F10: 4E800421  bctrl
	ctx.lr = 0x83208F14;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83208F14: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83208F18: 4182000C  beq 0x83208f24
	if ctx.cr[0].eq {
	pc = 0x83208F24; continue 'dispatch;
	}
	// 83208F1C: 83FF0098  lwz r31, 0x98(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(152 as u32) ) } as u64;
	// 83208F20: 48000024  b 0x83208f44
	pc = 0x83208F44; continue 'dispatch;
	// 83208F24: 817F0038  lwz r11, 0x38(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 83208F28: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83208F2C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 83208F30: 4099000C  ble cr6, 0x83208f3c
	if !ctx.cr[6].gt {
	pc = 0x83208F3C; continue 'dispatch;
	}
	// 83208F34: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83208F38: 48000008  b 0x83208f40
	pc = 0x83208F40; continue 'dispatch;
	// 83208F3C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83208F40: 83EB0000  lwz r31, 0(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83208F44: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 83208F48: 409AFF4C  bne cr6, 0x83208e94
	if !ctx.cr[6].eq {
	pc = 0x83208E94; continue 'dispatch;
	}
	// 83208F4C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83208F50: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83208F54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83208F58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83208F5C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83208F60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83208F68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83208F68 size=300
    let mut pc: u32 = 0x83208F68;
    'dispatch: loop {
        match pc {
            0x83208F68 => {
    //   block [0x83208F68..0x83209094)
	// 83208F68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83208F6C: 4BAA0481  bl 0x82ca93ec
	ctx.lr = 0x83208F70;
	sub_82CA93D0(ctx, base);
	// 83208F70: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83208F74: 7C761B78  mr r22, r3
	ctx.r[22].u64 = ctx.r[3].u64;
	// 83208F78: 3D408208  lis r10, -0x7df8
	ctx.r[10].s64 = -2113404928;
	// 83208F7C: 3D20820A  lis r9, -0x7df6
	ctx.r[9].s64 = -2113273856;
	// 83208F80: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 83208F84: 3B4AB734  addi r26, r10, -0x48cc
	ctx.r[26].s64 = ctx.r[10].s64 + -18636;
	// 83208F88: 82B600A4  lwz r21, 0xa4(r22)
	ctx.r[21].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(164 as u32) ) } as u64;
	// 83208F8C: 3B291700  addi r25, r9, 0x1700
	ctx.r[25].s64 = ctx.r[9].s64 + 5888;
	// 83208F90: 3B0BBFB0  addi r24, r11, -0x4050
	ctx.r[24].s64 = ctx.r[11].s64 + -16464;
	// 83208F94: 83960088  lwz r28, 0x88(r22)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(136 as u32) ) } as u64;
	// 83208F98: 3AE00000  li r23, 0
	ctx.r[23].s64 = 0;
	// 83208F9C: 837C0008  lwz r27, 8(r28)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 83208FA0: 480000DC  b 0x8320907c
	pc = 0x8320907C; continue 'dispatch;
	// 83208FA4: 7F1CA840  cmplw cr6, r28, r21
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[21].u32, &mut ctx.xer);
	// 83208FA8: 419A00CC  beq cr6, 0x83209074
	if ctx.cr[6].eq {
	pc = 0x83209074; continue 'dispatch;
	}
	// 83208FAC: 817C003C  lwz r11, 0x3c(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(60 as u32) ) } as u64;
	// 83208FB0: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83208FB4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83208FB8: 409A00BC  bne cr6, 0x83209074
	if !ctx.cr[6].eq {
	pc = 0x83209074; continue 'dispatch;
	}
	// 83208FBC: 7F06C378  mr r6, r24
	ctx.r[6].u64 = ctx.r[24].u64;
	// 83208FC0: 7F25CB78  mr r5, r25
	ctx.r[5].u64 = ctx.r[25].u64;
	// 83208FC4: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 83208FC8: 38E0022F  li r7, 0x22f
	ctx.r[7].s64 = 559;
	// 83208FCC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83208FD0: 4BF00DE1  bl 0x83109db0
	ctx.lr = 0x83208FD4;
	sub_83109DB0(ctx, base);
	// 83208FD4: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 83208FD8: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 83208FDC: 3BA00001  li r29, 1
	ctx.r[29].s64 = 1;
	// 83208FE0: 817C0038  lwz r11, 0x38(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(56 as u32) ) } as u64;
	// 83208FE4: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83208FE8: 7F1D5040  cmplw cr6, r29, r10
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[10].u32, &mut ctx.xer);
	// 83208FEC: 4099000C  ble cr6, 0x83208ff8
	if !ctx.cr[6].gt {
	pc = 0x83208FF8; continue 'dispatch;
	}
	// 83208FF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83208FF4: 48000010  b 0x83209004
	pc = 0x83209004; continue 'dispatch;
	// 83208FF8: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83208FFC: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83209000: 7FCAF82E  lwzx r30, r10, r31
	ctx.r[30].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 83209004: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83209008: 41820064  beq 0x8320906c
	if ctx.cr[0].eq {
	pc = 0x8320906C; continue 'dispatch;
	}
	// 8320900C: 817E003C  lwz r11, 0x3c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(60 as u32) ) } as u64;
	// 83209010: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 83209014: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 83209018: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8320901C: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83209020: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 83209024: 812B0004  lwz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83209028: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 8320902C: 812B0008  lwz r9, 8(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83209030: 912A0008  stw r9, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 83209034: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 83209038: 916A000C  stw r11, 0xc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 8320903C: 4BFFA2DD  bl 0x83203318
	ctx.lr = 0x83209040;
	sub_83203318(ctx, base);
	// 83209040: 817E003C  lwz r11, 0x3c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(60 as u32) ) } as u64;
	// 83209044: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83209048: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8320904C: 409A0008  bne cr6, 0x83209054
	if !ctx.cr[6].eq {
	pc = 0x83209054; continue 'dispatch;
	}
	// 83209050: 3AE00001  li r23, 1
	ctx.r[23].s64 = 1;
	// 83209054: 80810058  lwz r4, 0x58(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 83209058: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 8320905C: 4BFC326D  bl 0x831cc2c8
	ctx.lr = 0x83209060;
	sub_831CC2C8(ctx, base);
	// 83209060: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 83209064: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 83209068: 4BFFFF78  b 0x83208fe0
	pc = 0x83208FE0; continue 'dispatch;
	// 8320906C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83209070: 4BFFA009  bl 0x83203078
	ctx.lr = 0x83209074;
	sub_83203078(ctx, base);
	// 83209074: 7F7CDB78  mr r28, r27
	ctx.r[28].u64 = ctx.r[27].u64;
	// 83209078: 837B0008  lwz r27, 8(r27)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 8320907C: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 83209080: 409AFF24  bne cr6, 0x83208fa4
	if !ctx.cr[6].eq {
	pc = 0x83208FA4; continue 'dispatch;
	}
	// 83209084: 56EB063F  clrlwi. r11, r23, 0x18
	ctx.r[11].u64 = ctx.r[23].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83209088: 4082FF0C  bne 0x83208f94
	if !ctx.cr[0].eq {
	pc = 0x83208F94; continue 'dispatch;
	}
	// 8320908C: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 83209090: 4BAA03AC  b 0x82ca943c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83209098(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83209098 size=44
    let mut pc: u32 = 0x83209098;
    'dispatch: loop {
        match pc {
            0x83209098 => {
    //   block [0x83209098..0x832090C4)
	// 83209098: 81030004  lwz r8, 4(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8320909C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832090A0: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 832090A4: 40990040  ble cr6, 0x832090e4
	if !ctx.cr[6].gt {
		sub_832090C4(ctx, base);
		return;
	}
	// 832090A8: 80E40000  lwz r7, 0(r4)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 832090AC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832090B0: 7F094040  cmplw cr6, r9, r8
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[8].u32, &mut ctx.xer);
	// 832090B4: 40980010  bge cr6, 0x832090c4
	if !ctx.cr[6].lt {
		sub_832090C4(ctx, base);
		return;
	}
	// 832090B8: 81430008  lwz r10, 8(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 832090BC: 7D4A5A14  add r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 832090C0: 48000008  b 0x832090c8
	sub_832090C4(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832090C4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832090C4 size=40
    let mut pc: u32 = 0x832090C4;
    'dispatch: loop {
        match pc {
            0x832090C4 => {
    //   block [0x832090C4..0x832090EC)
	// 832090C4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832090C8: 814A0000  lwz r10, 0(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 832090CC: 7F0A3840  cmplw cr6, r10, r7
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[7].u32, &mut ctx.xer);
	// 832090D0: 419A001C  beq cr6, 0x832090ec
	if ctx.cr[6].eq {
		sub_832090EC(ctx, base);
		return;
	}
	// 832090D4: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 832090D8: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 832090DC: 7F094000  cmpw cr6, r9, r8
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[8].s32, &mut ctx.xer);
	// 832090E0: 4198FFD0  blt cr6, 0x832090b0
	if ctx.cr[6].lt {
		sub_83209098(ctx, base);
		return;
	}
	// 832090E4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 832090E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832090EC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832090EC size=8
    let mut pc: u32 = 0x832090EC;
    'dispatch: loop {
        match pc {
            0x832090EC => {
    //   block [0x832090EC..0x832090F4)
	// 832090EC: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 832090F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832090F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832090F8 size=136
    let mut pc: u32 = 0x832090F8;
    'dispatch: loop {
        match pc {
            0x832090F8 => {
    //   block [0x832090F8..0x83209180)
	// 832090F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832090FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83209100: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83209104: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83209108: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8320910C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83209110: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83209114: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 83209118: 419A000C  beq cr6, 0x83209124
	if ctx.cr[6].eq {
	pc = 0x83209124; continue 'dispatch;
	}
	// 8320911C: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 83209120: 409A0028  bne cr6, 0x83209148
	if !ctx.cr[6].eq {
	pc = 0x83209148; continue 'dispatch;
	}
	// 83209124: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 83209128: 3D408209  lis r10, -0x7df7
	ctx.r[10].s64 = -2113339392;
	// 8320912C: 3D208208  lis r9, -0x7df8
	ctx.r[9].s64 = -2113404928;
	// 83209130: 38CBBFB0  addi r6, r11, -0x4050
	ctx.r[6].s64 = ctx.r[11].s64 + -16464;
	// 83209134: 38AAC054  addi r5, r10, -0x3fac
	ctx.r[5].s64 = ctx.r[10].s64 + -16300;
	// 83209138: 3889B734  addi r4, r9, -0x48cc
	ctx.r[4].s64 = ctx.r[9].s64 + -18636;
	// 8320913C: 38E00028  li r7, 0x28
	ctx.r[7].s64 = 40;
	// 83209140: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83209144: 4BF00C6D  bl 0x83109db0
	ctx.lr = 0x83209148;
	sub_83109DB0(ctx, base);
	// 83209148: 807F0038  lwz r3, 0x38(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 8320914C: 80830004  lwz r4, 4(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 83209150: 4BFFA251  bl 0x832033a0
	ctx.lr = 0x83209154;
	sub_832033A0(ctx, base);
	// 83209154: 93C30000  stw r30, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 83209158: 807E003C  lwz r3, 0x3c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(60 as u32) ) } as u64;
	// 8320915C: 80830004  lwz r4, 4(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 83209160: 4BFFA241  bl 0x832033a0
	ctx.lr = 0x83209164;
	sub_832033A0(ctx, base);
	// 83209164: 93E30000  stw r31, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 83209168: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8320916C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83209170: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83209174: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83209178: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8320917C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83209180(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83209180 size=180
    let mut pc: u32 = 0x83209180;
    'dispatch: loop {
        match pc {
            0x83209180 => {
    //   block [0x83209180..0x83209234)
	// 83209180: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83209184: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83209188: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8320918C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83209190: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83209194: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 83209198: 396BC01C  addi r11, r11, -0x3fe4
	ctx.r[11].s64 = ctx.r[11].s64 + -16356;
	// 8320919C: 807F003C  lwz r3, 0x3c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) } as u64;
	// 832091A0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 832091A4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832091A8: 419A000C  beq cr6, 0x832091b4
	if ctx.cr[6].eq {
	pc = 0x832091B4; continue 'dispatch;
	}
	// 832091AC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 832091B0: 4800AA99  bl 0x83213c48
	ctx.lr = 0x832091B4;
	sub_83213C48(ctx, base);
	// 832091B4: 807F0038  lwz r3, 0x38(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 832091B8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832091BC: 419A000C  beq cr6, 0x832091c8
	if ctx.cr[6].eq {
	pc = 0x832091C8; continue 'dispatch;
	}
	// 832091C0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 832091C4: 4800AA85  bl 0x83213c48
	ctx.lr = 0x832091C8;
	sub_83213C48(ctx, base);
	// 832091C8: 807F0058  lwz r3, 0x58(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 832091CC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832091D0: 419A000C  beq cr6, 0x832091dc
	if ctx.cr[6].eq {
	pc = 0x832091DC; continue 'dispatch;
	}
	// 832091D4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 832091D8: 4800AA71  bl 0x83213c48
	ctx.lr = 0x832091DC;
	sub_83213C48(ctx, base);
	// 832091DC: 807F005C  lwz r3, 0x5c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 832091E0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832091E4: 419A000C  beq cr6, 0x832091f0
	if ctx.cr[6].eq {
	pc = 0x832091F0; continue 'dispatch;
	}
	// 832091E8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 832091EC: 4800AA5D  bl 0x83213c48
	ctx.lr = 0x832091F0;
	sub_83213C48(ctx, base);
	// 832091F0: 807F002C  lwz r3, 0x2c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 832091F4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 832091F8: 419A000C  beq cr6, 0x83209204
	if ctx.cr[6].eq {
	pc = 0x83209204; continue 'dispatch;
	}
	// 832091FC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83209200: 4800AA49  bl 0x83213c48
	ctx.lr = 0x83209204;
	sub_83213C48(ctx, base);
	// 83209204: 387F0014  addi r3, r31, 0x14
	ctx.r[3].s64 = ctx.r[31].s64 + 20;
	// 83209208: 4BFF9FA9  bl 0x832031b0
	ctx.lr = 0x8320920C;
	sub_832031B0(ctx, base);
	// 8320920C: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 83209210: 396B7718  addi r11, r11, 0x7718
	ctx.r[11].s64 = ctx.r[11].s64 + 30488;
	// 83209214: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 83209218: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 8320921C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83209220: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83209224: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83209228: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8320922C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83209230: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83209238(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83209238 size=256
    let mut pc: u32 = 0x83209238;
    'dispatch: loop {
        match pc {
            0x83209238 => {
    //   block [0x83209238..0x83209338)
	// 83209238: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8320923C: 4BAA01D1  bl 0x82ca940c
	ctx.lr = 0x83209240;
	sub_82CA93D0(ctx, base);
	// 83209240: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83209244: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83209248: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8320924C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 83209250: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83209254: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83209258: 807E0038  lwz r3, 0x38(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(56 as u32) ) } as u64;
	// 8320925C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83209260: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 83209264: 80E30004  lwz r7, 4(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 83209268: 7F093840  cmplw cr6, r9, r7
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[7].u32, &mut ctx.xer);
	// 8320926C: 4099000C  ble cr6, 0x83209278
	if !ctx.cr[6].gt {
	pc = 0x83209278; continue 'dispatch;
	}
	// 83209270: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83209274: 48000010  b 0x83209284
	pc = 0x83209284; continue 'dispatch;
	// 83209278: 81430008  lwz r10, 8(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 8320927C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 83209280: 7D4A582E  lwzx r10, r10, r11
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 83209284: 5508063F  clrlwi. r8, r8, 0x18
	ctx.r[8].u64 = ctx.r[8].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 83209288: 41820040  beq 0x832092c8
	if ctx.cr[0].eq {
	pc = 0x832092C8; continue 'dispatch;
	}
	// 8320928C: 7F0AF840  cmplw cr6, r10, r31
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[31].u32, &mut ctx.xer);
	// 83209290: 419A0014  beq cr6, 0x832092a4
	if ctx.cr[6].eq {
	pc = 0x832092A4; continue 'dispatch;
	}
	// 83209294: 38840001  addi r4, r4, 1
	ctx.r[4].s64 = ctx.r[4].s64 + 1;
	// 83209298: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 8320929C: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 832092A0: 4BFFFFC8  b 0x83209268
	pc = 0x83209268; continue 'dispatch;
	// 832092A4: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 832092A8: 7F045840  cmplw cr6, r4, r11
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[11].u32, &mut ctx.xer);
	// 832092AC: 40980014  bge cr6, 0x832092c0
	if !ctx.cr[6].lt {
	pc = 0x832092C0; continue 'dispatch;
	}
	// 832092B0: 81430008  lwz r10, 8(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 832092B4: 548B103A  slwi r11, r4, 2
	ctx.r[11].u32 = ctx.r[4].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 832092B8: 7C6A5A14  add r3, r10, r11
	ctx.r[3].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 832092BC: 48000008  b 0x832092c4
	pc = 0x832092C4; continue 'dispatch;
	// 832092C0: 4BFFA0E1  bl 0x832033a0
	ctx.lr = 0x832092C4;
	sub_832033A0(ctx, base);
	// 832092C4: 93A30000  stw r29, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 832092C8: 807F003C  lwz r3, 0x3c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) } as u64;
	// 832092CC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 832092D0: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832092D4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832092D8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 832092DC: 80E30004  lwz r7, 4(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 832092E0: 7F083840  cmplw cr6, r8, r7
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[7].u32, &mut ctx.xer);
	// 832092E4: 4099000C  ble cr6, 0x832092f0
	if !ctx.cr[6].gt {
	pc = 0x832092F0; continue 'dispatch;
	}
	// 832092E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832092EC: 48000010  b 0x832092fc
	pc = 0x832092FC; continue 'dispatch;
	// 832092F0: 81230008  lwz r9, 8(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 832092F4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 832092F8: 7D29582E  lwzx r9, r9, r11
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 832092FC: 554A063F  clrlwi. r10, r10, 0x18
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 83209300: 41820020  beq 0x83209320
	if ctx.cr[0].eq {
	pc = 0x83209320; continue 'dispatch;
	}
	// 83209304: 7F09F040  cmplw cr6, r9, r30
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[30].u32, &mut ctx.xer);
	// 83209308: 419A0014  beq cr6, 0x8320931c
	if ctx.cr[6].eq {
	pc = 0x8320931C; continue 'dispatch;
	}
	// 8320930C: 38840001  addi r4, r4, 1
	ctx.r[4].s64 = ctx.r[4].s64 + 1;
	// 83209310: 39080001  addi r8, r8, 1
	ctx.r[8].s64 = ctx.r[8].s64 + 1;
	// 83209314: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 83209318: 4BFFFFC8  b 0x832092e0
	pc = 0x832092E0; continue 'dispatch;
	// 8320931C: 4BFF9FC5  bl 0x832032e0
	ctx.lr = 0x83209320;
	sub_832032E0(ctx, base);
	// 83209320: 807D003C  lwz r3, 0x3c(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(60 as u32) ) } as u64;
	// 83209324: 80830004  lwz r4, 4(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 83209328: 4BFFA079  bl 0x832033a0
	ctx.lr = 0x8320932C;
	sub_832033A0(ctx, base);
	// 8320932C: 93C30000  stw r30, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 83209330: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83209334: 4BAA0128  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83209338(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83209338 size=68
    let mut pc: u32 = 0x83209338;
    'dispatch: loop {
        match pc {
            0x83209338 => {
    //   block [0x83209338..0x8320937C)
	// 83209338: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8320933C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83209340: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83209344: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83209348: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8320934C: 4800000C  b 0x83209358
	pc = 0x83209358; continue 'dispatch;
	// 83209350: 4800C101  bl 0x83215450
	ctx.lr = 0x83209354;
	sub_83215450(ctx, base);
	// 83209354: 4BFFA7DD  bl 0x83203b30
	ctx.lr = 0x83209358;
	sub_83203B30(ctx, base);
	// 83209358: 807F002C  lwz r3, 0x2c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 8320935C: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 83209360: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83209364: 409AFFEC  bne cr6, 0x83209350
	if !ctx.cr[6].eq {
	pc = 0x83209350; continue 'dispatch;
	}
	// 83209368: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8320936C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83209370: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83209374: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83209378: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83209380(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83209380 size=92
    let mut pc: u32 = 0x83209380;
    'dispatch: loop {
        match pc {
            0x83209380 => {
    //   block [0x83209380..0x832093DC)
	// 83209380: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83209384: 4BAA0085  bl 0x82ca9408
	ctx.lr = 0x83209388;
	sub_82CA93D0(ctx, base);
	// 83209388: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8320938C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 83209390: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83209394: 93A1009C  stw r29, 0x9c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(156 as u32), ctx.r[29].u32 ) };
	// 83209398: 3881009C  addi r4, r1, 0x9c
	ctx.r[4].s64 = ctx.r[1].s64 + 156;
	// 8320939C: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 832093A0: 83FE002C  lwz r31, 0x2c(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(44 as u32) ) } as u64;
	// 832093A4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832093A8: 4BFFFCF1  bl 0x83209098
	ctx.lr = 0x832093AC;
	sub_83209098(ctx, base);
	// 832093AC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 832093B0: 40820014  bne 0x832093c4
	if !ctx.cr[0].eq {
	pc = 0x832093C4; continue 'dispatch;
	}
	// 832093B4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832093B8: 809F0004  lwz r4, 4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 832093BC: 4BFF9FE5  bl 0x832033a0
	ctx.lr = 0x832093C0;
	sub_832033A0(ctx, base);
	// 832093C0: 93A30000  stw r29, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 832093C4: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 832093C8: 809E0030  lwz r4, 0x30(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 832093CC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 832093D0: 4BFFA789  bl 0x83203b58
	ctx.lr = 0x832093D4;
	sub_83203B58(ctx, base);
	// 832093D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832093D8: 4BAA0080  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832093E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832093E0 size=68
    let mut pc: u32 = 0x832093E0;
    'dispatch: loop {
        match pc {
            0x832093E0 => {
    //   block [0x832093E0..0x83209424)
	// 832093E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832093E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832093E8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832093EC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832093F0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 832093F4: 4BFFF585  bl 0x83208978
	ctx.lr = 0x832093F8;
	sub_83208978(ctx, base);
	// 832093F8: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 832093FC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83209400: 396BC070  addi r11, r11, -0x3f90
	ctx.r[11].s64 = ctx.r[11].s64 + -16272;
	// 83209404: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83209408: 817F0070  lwz r11, 0x70(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(112 as u32) ) } as u64;
	// 8320940C: 917F0088  stw r11, 0x88(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(136 as u32), ctx.r[11].u32 ) };
	// 83209410: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83209414: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83209418: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8320941C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83209420: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83209428(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83209428 size=12
    let mut pc: u32 = 0x83209428;
    'dispatch: loop {
        match pc {
            0x83209428 => {
    //   block [0x83209428..0x83209434)
	// 83209428: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 8320942C: 386BC0A0  addi r3, r11, -0x3f60
	ctx.r[3].s64 = ctx.r[11].s64 + -16224;
	// 83209430: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83209438(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83209438 size=60
    let mut pc: u32 = 0x83209438;
    'dispatch: loop {
        match pc {
            0x83209438 => {
    //   block [0x83209438..0x83209474)
	// 83209438: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8320943C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83209440: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83209444: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83209448: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8320944C: 4BFFF52D  bl 0x83208978
	ctx.lr = 0x83209450;
	sub_83208978(ctx, base);
	// 83209450: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 83209454: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83209458: 396BC0A8  addi r11, r11, -0x3f58
	ctx.r[11].s64 = ctx.r[11].s64 + -16216;
	// 8320945C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83209460: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83209464: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83209468: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8320946C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83209470: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83209478(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83209478 size=12
    let mut pc: u32 = 0x83209478;
    'dispatch: loop {
        match pc {
            0x83209478 => {
    //   block [0x83209478..0x83209484)
	// 83209478: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 8320947C: 386BC0D8  addi r3, r11, -0x3f28
	ctx.r[3].s64 = ctx.r[11].s64 + -16168;
	// 83209480: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83209488(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83209488 size=140
    let mut pc: u32 = 0x83209488;
    'dispatch: loop {
        match pc {
            0x83209488 => {
    //   block [0x83209488..0x83209514)
	// 83209488: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8320948C: 4BA9FF7D  bl 0x82ca9408
	ctx.lr = 0x83209490;
	sub_82CA93D0(ctx, base);
	// 83209490: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83209494: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83209498: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8320949C: 4BFFF4DD  bl 0x83208978
	ctx.lr = 0x832094A0;
	sub_83208978(ctx, base);
	// 832094A0: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 832094A4: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 832094A8: 396BC0E0  addi r11, r11, -0x3f20
	ctx.r[11].s64 = ctx.r[11].s64 + -16160;
	// 832094AC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 832094B0: 93DF0088  stw r30, 0x88(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(136 as u32), ctx.r[30].u32 ) };
	// 832094B4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 832094B8: 3880008C  li r4, 0x8c
	ctx.r[4].s64 = 140;
	// 832094BC: 995F008C  stb r10, 0x8c(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(140 as u32), ctx.r[10].u8 ) };
	// 832094C0: 93DF0090  stw r30, 0x90(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(144 as u32), ctx.r[30].u32 ) };
	// 832094C4: 93DF0094  stw r30, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[30].u32 ) };
	// 832094C8: 93DF009C  stw r30, 0x9c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(156 as u32), ctx.r[30].u32 ) };
	// 832094CC: 839D05AC  lwz r28, 0x5ac(r29)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(1452 as u32) ) } as u64;
	// 832094D0: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 832094D4: 4BFC2CC5  bl 0x831cc198
	ctx.lr = 0x832094D8;
	sub_831CC198(ctx, base);
	// 832094D8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 832094DC: 346B0004  addic. r3, r11, 4
	ctx.xer.ca = (ctx.r[11].u32 > (!(4 as u32)));
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 832094E0: 938B0000  stw r28, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 832094E4: 41820014  beq 0x832094f8
	if ctx.cr[0].eq {
	pc = 0x832094F8; continue 'dispatch;
	}
	// 832094E8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 832094EC: 4BFFF48D  bl 0x83208978
	ctx.lr = 0x832094F0;
	sub_83208978(ctx, base);
	// 832094F0: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 832094F4: 48000008  b 0x832094fc
	pc = 0x832094FC; continue 'dispatch;
	// 832094F8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 832094FC: 909F0090  stw r4, 0x90(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(144 as u32), ctx.r[4].u32 ) };
	// 83209500: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83209504: 4BFFFBF5  bl 0x832090f8
	ctx.lr = 0x83209508;
	sub_832090F8(ctx, base);
	// 83209508: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8320950C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83209510: 4BA9FF48  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83209518(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83209518 size=12
    let mut pc: u32 = 0x83209518;
    'dispatch: loop {
        match pc {
            0x83209518 => {
    //   block [0x83209518..0x83209524)
	// 83209518: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 8320951C: 386BC114  addi r3, r11, -0x3eec
	ctx.r[3].s64 = ctx.r[11].s64 + -16108;
	// 83209520: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83209528(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83209528 size=80
    let mut pc: u32 = 0x83209528;
    'dispatch: loop {
        match pc {
            0x83209528 => {
    //   block [0x83209528..0x83209578)
	// 83209528: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8320952C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83209530: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83209534: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83209538: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8320953C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83209540: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 83209544: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83209548: 4BFFF431  bl 0x83208978
	ctx.lr = 0x8320954C;
	sub_83208978(ctx, base);
	// 8320954C: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 83209550: 93DF0088  stw r30, 0x88(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(136 as u32), ctx.r[30].u32 ) };
	// 83209554: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83209558: 396BC120  addi r11, r11, -0x3ee0
	ctx.r[11].s64 = ctx.r[11].s64 + -16096;
	// 8320955C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83209560: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83209564: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83209568: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8320956C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83209570: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83209574: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83209578(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83209578 size=12
    let mut pc: u32 = 0x83209578;
    'dispatch: loop {
        match pc {
            0x83209578 => {
    //   block [0x83209578..0x83209584)
	// 83209578: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 8320957C: 386BC150  addi r3, r11, -0x3eb0
	ctx.r[3].s64 = ctx.r[11].s64 + -16048;
	// 83209580: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83209588(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83209588 size=80
    let mut pc: u32 = 0x83209588;
    'dispatch: loop {
        match pc {
            0x83209588 => {
    //   block [0x83209588..0x832095D8)
	// 83209588: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8320958C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83209590: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83209594: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83209598: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8320959C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 832095A0: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 832095A4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 832095A8: 4BFFF3D1  bl 0x83208978
	ctx.lr = 0x832095AC;
	sub_83208978(ctx, base);
	// 832095AC: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 832095B0: 93DF0088  stw r30, 0x88(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(136 as u32), ctx.r[30].u32 ) };
	// 832095B4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832095B8: 396BC158  addi r11, r11, -0x3ea8
	ctx.r[11].s64 = ctx.r[11].s64 + -16040;
	// 832095BC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 832095C0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 832095C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832095C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832095CC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 832095D0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832095D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832095D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832095D8 size=12
    let mut pc: u32 = 0x832095D8;
    'dispatch: loop {
        match pc {
            0x832095D8 => {
    //   block [0x832095D8..0x832095E4)
	// 832095D8: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 832095DC: 386BC188  addi r3, r11, -0x3e78
	ctx.r[3].s64 = ctx.r[11].s64 + -15992;
	// 832095E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832095E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832095E8 size=12
    let mut pc: u32 = 0x832095E8;
    'dispatch: loop {
        match pc {
            0x832095E8 => {
    //   block [0x832095E8..0x832095F4)
	// 832095E8: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 832095EC: 386BC1C4  addi r3, r11, -0x3e3c
	ctx.r[3].s64 = ctx.r[11].s64 + -15932;
	// 832095F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832095F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832095F8 size=12
    let mut pc: u32 = 0x832095F8;
    'dispatch: loop {
        match pc {
            0x832095F8 => {
    //   block [0x832095F8..0x83209604)
	// 832095F8: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 832095FC: 386BC200  addi r3, r11, -0x3e00
	ctx.r[3].s64 = ctx.r[11].s64 + -15872;
	// 83209600: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83209608(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83209608 size=12
    let mut pc: u32 = 0x83209608;
    'dispatch: loop {
        match pc {
            0x83209608 => {
    //   block [0x83209608..0x83209614)
	// 83209608: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 8320960C: 386BC244  addi r3, r11, -0x3dbc
	ctx.r[3].s64 = ctx.r[11].s64 + -15804;
	// 83209610: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83209618(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83209618 size=12
    let mut pc: u32 = 0x83209618;
    'dispatch: loop {
        match pc {
            0x83209618 => {
    //   block [0x83209618..0x83209624)
	// 83209618: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 8320961C: 386BC280  addi r3, r11, -0x3d80
	ctx.r[3].s64 = ctx.r[11].s64 + -15744;
	// 83209620: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83209628(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83209628 size=88
    let mut pc: u32 = 0x83209628;
    'dispatch: loop {
        match pc {
            0x83209628 => {
    //   block [0x83209628..0x83209680)
	// 83209628: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8320962C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83209630: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83209634: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83209638: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8320963C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83209640: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83209644: 4BFFFB3D  bl 0x83209180
	ctx.lr = 0x83209648;
	sub_83209180(ctx, base);
	// 83209648: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8320964C: 41820018  beq 0x83209664
	if ctx.cr[0].eq {
	pc = 0x83209664; continue 'dispatch;
	}
	// 83209650: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 83209654: 419A0010  beq cr6, 0x83209664
	if ctx.cr[6].eq {
	pc = 0x83209664; continue 'dispatch;
	}
	// 83209658: 389FFFFC  addi r4, r31, -4
	ctx.r[4].s64 = ctx.r[31].s64 + -4;
	// 8320965C: 807FFFFC  lwz r3, -4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-4 as u32) ) } as u64;
	// 83209660: 4BFC2C69  bl 0x831cc2c8
	ctx.lr = 0x83209664;
	sub_831CC2C8(ctx, base);
	// 83209664: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83209668: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8320966C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83209670: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83209674: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83209678: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8320967C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83209680(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83209680 size=68
    let mut pc: u32 = 0x83209680;
    'dispatch: loop {
        match pc {
            0x83209680 => {
    //   block [0x83209680..0x832096C4)
	// 83209680: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83209684: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83209688: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8320968C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83209690: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83209694: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 83209698: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 8320969C: 4BFFFB9D  bl 0x83209238
	ctx.lr = 0x832096A0;
	sub_83209238(ctx, base);
	// 832096A0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 832096A4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 832096A8: 4BFFFA51  bl 0x832090f8
	ctx.lr = 0x832096AC;
	sub_832090F8(ctx, base);
	// 832096AC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 832096B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832096B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832096B8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 832096BC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832096C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832096C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832096C8 size=288
    let mut pc: u32 = 0x832096C8;
    'dispatch: loop {
        match pc {
            0x832096C8 => {
    //   block [0x832096C8..0x832097E8)
	// 832096C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832096CC: 4BA9FD39  bl 0x82ca9404
	ctx.lr = 0x832096D0;
	sub_82CA93D0(ctx, base);
	// 832096D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832096D4: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 832096D8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 832096DC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 832096E0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 832096E4: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 832096E8: 4BFFF291  bl 0x83208978
	ctx.lr = 0x832096EC;
	sub_83208978(ctx, base);
	// 832096EC: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 832096F0: 937F0094  stw r27, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[27].u32 ) };
	// 832096F4: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 832096F8: 93DF0088  stw r30, 0x88(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(136 as u32), ctx.r[30].u32 ) };
	// 832096FC: 396BC28C  addi r11, r11, -0x3d74
	ctx.r[11].s64 = ctx.r[11].s64 + -15732;
	// 83209700: 9B9F008C  stb r28, 0x8c(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(140 as u32), ctx.r[28].u8 ) };
	// 83209704: 38800090  li r4, 0x90
	ctx.r[4].s64 = 144;
	// 83209708: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8320970C: 9B9F008D  stb r28, 0x8d(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(141 as u32), ctx.r[28].u8 ) };
	// 83209710: 837D05AC  lwz r27, 0x5ac(r29)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(1452 as u32) ) } as u64;
	// 83209714: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 83209718: 4BFC2A81  bl 0x831cc198
	ctx.lr = 0x8320971C;
	sub_831CC198(ctx, base);
	// 8320971C: 37C30004  addic. r30, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[30].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 83209720: 93630000  stw r27, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 83209724: 41820028  beq 0x8320974c
	if ctx.cr[0].eq {
	pc = 0x8320974C; continue 'dispatch;
	}
	// 83209728: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8320972C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83209730: 4BFFF249  bl 0x83208978
	ctx.lr = 0x83209734;
	sub_83208978(ctx, base);
	// 83209734: 3D408209  lis r10, -0x7df7
	ctx.r[10].s64 = -2113339392;
	// 83209738: 93FE0088  stw r31, 0x88(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(136 as u32), ctx.r[31].u32 ) };
	// 8320973C: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 83209740: 394AC194  addi r10, r10, -0x3e6c
	ctx.r[10].s64 = ctx.r[10].s64 + -15980;
	// 83209744: 915E0000  stw r10, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83209748: 48000008  b 0x83209750
	pc = 0x83209750; continue 'dispatch;
	// 8320974C: 7F8BE378  mr r11, r28
	ctx.r[11].u64 = ctx.r[28].u64;
	// 83209750: 917F0098  stw r11, 0x98(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(152 as u32), ctx.r[11].u32 ) };
	// 83209754: 38800090  li r4, 0x90
	ctx.r[4].s64 = 144;
	// 83209758: 837D05AC  lwz r27, 0x5ac(r29)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(1452 as u32) ) } as u64;
	// 8320975C: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 83209760: 4BFC2A39  bl 0x831cc198
	ctx.lr = 0x83209764;
	sub_831CC198(ctx, base);
	// 83209764: 37C30004  addic. r30, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[30].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 83209768: 93630000  stw r27, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 8320976C: 41820028  beq 0x83209794
	if ctx.cr[0].eq {
	pc = 0x83209794; continue 'dispatch;
	}
	// 83209770: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83209774: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83209778: 4BFFF201  bl 0x83208978
	ctx.lr = 0x8320977C;
	sub_83208978(ctx, base);
	// 8320977C: 3D408209  lis r10, -0x7df7
	ctx.r[10].s64 = -2113339392;
	// 83209780: 93FE0088  stw r31, 0x88(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(136 as u32), ctx.r[31].u32 ) };
	// 83209784: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 83209788: 394AC1D0  addi r10, r10, -0x3e30
	ctx.r[10].s64 = ctx.r[10].s64 + -15920;
	// 8320978C: 915E0000  stw r10, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83209790: 48000008  b 0x83209798
	pc = 0x83209798; continue 'dispatch;
	// 83209794: 7F8BE378  mr r11, r28
	ctx.r[11].u64 = ctx.r[28].u64;
	// 83209798: 917F009C  stw r11, 0x9c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(156 as u32), ctx.r[11].u32 ) };
	// 8320979C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 832097A0: 807F0098  lwz r3, 0x98(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(152 as u32) ) } as u64;
	// 832097A4: 4BFFF955  bl 0x832090f8
	ctx.lr = 0x832097A8;
	sub_832090F8(ctx, base);
	// 832097A8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 832097AC: 3860007D  li r3, 0x7d
	ctx.r[3].s64 = 125;
	// 832097B0: 4BFFBF51  bl 0x83205700
	ctx.lr = 0x832097B4;
	sub_83205700(ctx, base);
	// 832097B4: 817F0088  lwz r11, 0x88(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(136 as u32) ) } as u64;
	// 832097B8: 39400027  li r10, 0x27
	ctx.r[10].s64 = 39;
	// 832097BC: 907F0090  stw r3, 0x90(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(144 as u32), ctx.r[3].u32 ) };
	// 832097C0: 91430050  stw r10, 0x50(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 832097C4: 91630038  stw r11, 0x38(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 832097C8: 809F0074  lwz r4, 0x74(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(116 as u32) ) } as u64;
	// 832097CC: 83DF0090  lwz r30, 0x90(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(144 as u32) ) } as u64;
	// 832097D0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 832097D4: 4BFF98D5  bl 0x832030a8
	ctx.lr = 0x832097D8;
	sub_832030A8(ctx, base);
	// 832097D8: 93FE03B4  stw r31, 0x3b4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(948 as u32), ctx.r[31].u32 ) };
	// 832097DC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832097E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832097E4: 4BA9FC70  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832097E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832097E8 size=12
    let mut pc: u32 = 0x832097E8;
    'dispatch: loop {
        match pc {
            0x832097E8 => {
    //   block [0x832097E8..0x832097F4)
	// 832097E8: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 832097EC: 386BC2BC  addi r3, r11, -0x3d44
	ctx.r[3].s64 = ctx.r[11].s64 + -15684;
	// 832097F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832097F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832097F8 size=244
    let mut pc: u32 = 0x832097F8;
    'dispatch: loop {
        match pc {
            0x832097F8 => {
    //   block [0x832097F8..0x832098EC)
	// 832097F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832097FC: 4BA9FC09  bl 0x82ca9404
	ctx.lr = 0x83209800;
	sub_82CA93D0(ctx, base);
	// 83209800: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83209804: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 83209808: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8320980C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83209810: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83209814: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 83209818: 4BFFF161  bl 0x83208978
	ctx.lr = 0x8320981C;
	sub_83208978(ctx, base);
	// 8320981C: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 83209820: 937F009C  stw r27, 0x9c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(156 as u32), ctx.r[27].u32 ) };
	// 83209824: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 83209828: 93DF0088  stw r30, 0x88(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(136 as u32), ctx.r[30].u32 ) };
	// 8320982C: 396BC0E0  addi r11, r11, -0x3f20
	ctx.r[11].s64 = ctx.r[11].s64 + -16160;
	// 83209830: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 83209834: 939F0090  stw r28, 0x90(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(144 as u32), ctx.r[28].u32 ) };
	// 83209838: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8320983C: 3880008C  li r4, 0x8c
	ctx.r[4].s64 = 140;
	// 83209840: 995F008C  stb r10, 0x8c(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(140 as u32), ctx.r[10].u8 ) };
	// 83209844: 939F0094  stw r28, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[28].u32 ) };
	// 83209848: 83DD05AC  lwz r30, 0x5ac(r29)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(1452 as u32) ) } as u64;
	// 8320984C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83209850: 4BFC2949  bl 0x831cc198
	ctx.lr = 0x83209854;
	sub_831CC198(ctx, base);
	// 83209854: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83209858: 346B0004  addic. r3, r11, 4
	ctx.xer.ca = (ctx.r[11].u32 > (!(4 as u32)));
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8320985C: 93CB0000  stw r30, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 83209860: 41820014  beq 0x83209874
	if ctx.cr[0].eq {
	pc = 0x83209874; continue 'dispatch;
	}
	// 83209864: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83209868: 4BFFF111  bl 0x83208978
	ctx.lr = 0x8320986C;
	sub_83208978(ctx, base);
	// 8320986C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 83209870: 48000008  b 0x83209878
	pc = 0x83209878; continue 'dispatch;
	// 83209874: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 83209878: 909F0090  stw r4, 0x90(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(144 as u32), ctx.r[4].u32 ) };
	// 8320987C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83209880: 4BFFF879  bl 0x832090f8
	ctx.lr = 0x83209884;
	sub_832090F8(ctx, base);
	// 83209884: 809F0074  lwz r4, 0x74(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(116 as u32) ) } as u64;
	// 83209888: 83DF0088  lwz r30, 0x88(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(136 as u32) ) } as u64;
	// 8320988C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83209890: 4BFF9819  bl 0x832030a8
	ctx.lr = 0x83209894;
	sub_832030A8(ctx, base);
	// 83209894: 93FE03B4  stw r31, 0x3b4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(948 as u32), ctx.r[31].u32 ) };
	// 83209898: 38800090  li r4, 0x90
	ctx.r[4].s64 = 144;
	// 8320989C: 837D05AC  lwz r27, 0x5ac(r29)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(1452 as u32) ) } as u64;
	// 832098A0: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 832098A4: 4BFC28F5  bl 0x831cc198
	ctx.lr = 0x832098A8;
	sub_831CC198(ctx, base);
	// 832098A8: 37C30004  addic. r30, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[30].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 832098AC: 93630000  stw r27, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 832098B0: 41820028  beq 0x832098d8
	if ctx.cr[0].eq {
	pc = 0x832098D8; continue 'dispatch;
	}
	// 832098B4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 832098B8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 832098BC: 4BFFF0BD  bl 0x83208978
	ctx.lr = 0x832098C0;
	sub_83208978(ctx, base);
	// 832098C0: 3D408209  lis r10, -0x7df7
	ctx.r[10].s64 = -2113339392;
	// 832098C4: 93FE0088  stw r31, 0x88(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(136 as u32), ctx.r[31].u32 ) };
	// 832098C8: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 832098CC: 394AC214  addi r10, r10, -0x3dec
	ctx.r[10].s64 = ctx.r[10].s64 + -15852;
	// 832098D0: 915E0000  stw r10, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 832098D4: 48000008  b 0x832098dc
	pc = 0x832098DC; continue 'dispatch;
	// 832098D8: 7F8BE378  mr r11, r28
	ctx.r[11].u64 = ctx.r[28].u64;
	// 832098DC: 917F0098  stw r11, 0x98(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(152 as u32), ctx.r[11].u32 ) };
	// 832098E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832098E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832098E8: 4BA9FB6C  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832098F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832098F0 size=156
    let mut pc: u32 = 0x832098F0;
    'dispatch: loop {
        match pc {
            0x832098F0 => {
    //   block [0x832098F0..0x8320998C)
	// 832098F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832098F4: 4BA9FB15  bl 0x82ca9408
	ctx.lr = 0x832098F8;
	sub_82CA93D0(ctx, base);
	// 832098F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832098FC: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 83209900: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83209904: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83209908: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8320990C: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 83209910: 4BFFFB79  bl 0x83209488
	ctx.lr = 0x83209914;
	sub_83209488(ctx, base);
	// 83209914: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 83209918: 93DF0088  stw r30, 0x88(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(136 as u32), ctx.r[30].u32 ) };
	// 8320991C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83209920: 809F0074  lwz r4, 0x74(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(116 as u32) ) } as u64;
	// 83209924: 396BC2C8  addi r11, r11, -0x3d38
	ctx.r[11].s64 = ctx.r[11].s64 + -15672;
	// 83209928: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8320992C: 4BFF977D  bl 0x832030a8
	ctx.lr = 0x83209930;
	sub_832030A8(ctx, base);
	// 83209930: 93FE03B4  stw r31, 0x3b4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(948 as u32), ctx.r[31].u32 ) };
	// 83209934: 939F009C  stw r28, 0x9c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(156 as u32), ctx.r[28].u32 ) };
	// 83209938: 38800090  li r4, 0x90
	ctx.r[4].s64 = 144;
	// 8320993C: 839D05AC  lwz r28, 0x5ac(r29)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(1452 as u32) ) } as u64;
	// 83209940: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83209944: 4BFC2855  bl 0x831cc198
	ctx.lr = 0x83209948;
	sub_831CC198(ctx, base);
	// 83209948: 37C30004  addic. r30, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[30].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8320994C: 93830000  stw r28, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 83209950: 41820028  beq 0x83209978
	if ctx.cr[0].eq {
	pc = 0x83209978; continue 'dispatch;
	}
	// 83209954: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83209958: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8320995C: 4BFFF01D  bl 0x83208978
	ctx.lr = 0x83209960;
	sub_83208978(ctx, base);
	// 83209960: 3D408209  lis r10, -0x7df7
	ctx.r[10].s64 = -2113339392;
	// 83209964: 93FE0088  stw r31, 0x88(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(136 as u32), ctx.r[31].u32 ) };
	// 83209968: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 8320996C: 394AC250  addi r10, r10, -0x3db0
	ctx.r[10].s64 = ctx.r[10].s64 + -15792;
	// 83209970: 915E0000  stw r10, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83209974: 48000008  b 0x8320997c
	pc = 0x8320997C; continue 'dispatch;
	// 83209978: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8320997C: 917F0098  stw r11, 0x98(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(152 as u32), ctx.r[11].u32 ) };
	// 83209980: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83209984: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83209988: 4BA9FAD0  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83209990(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83209990 size=12
    let mut pc: u32 = 0x83209990;
    'dispatch: loop {
        match pc {
            0x83209990 => {
    //   block [0x83209990..0x8320999C)
	// 83209990: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 83209994: 386BC2FC  addi r3, r11, -0x3d04
	ctx.r[3].s64 = ctx.r[11].s64 + -15620;
	// 83209998: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832099A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832099A0 size=160
    let mut pc: u32 = 0x832099A0;
    'dispatch: loop {
        match pc {
            0x832099A0 => {
    //   block [0x832099A0..0x83209A40)
	// 832099A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832099A4: 4BA9FA5D  bl 0x82ca9400
	ctx.lr = 0x832099A8;
	sub_82CA93D0(ctx, base);
	// 832099A8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832099AC: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 832099B0: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 832099B4: 3D408209  lis r10, -0x7df7
	ctx.r[10].s64 = -2113339392;
	// 832099B8: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 832099BC: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 832099C0: 3B8BB734  addi r28, r11, -0x48cc
	ctx.r[28].s64 = ctx.r[11].s64 + -18636;
	// 832099C4: 3B6AE8E8  addi r27, r10, -0x1718
	ctx.r[27].s64 = ctx.r[10].s64 + -5912;
	// 832099C8: 4198000C  blt cr6, 0x832099d4
	if ctx.cr[6].lt {
	pc = 0x832099D4; continue 'dispatch;
	}
	// 832099CC: 2F1F0088  cmpwi cr6, r31, 0x88
	ctx.cr[6].compare_i32(ctx.r[31].s32, 136, &mut ctx.xer);
	// 832099D0: 41980020  blt cr6, 0x832099f0
	if ctx.cr[6].lt {
	pc = 0x832099F0; continue 'dispatch;
	}
	// 832099D4: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 832099D8: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 832099DC: 38ABE8B4  addi r5, r11, -0x174c
	ctx.r[5].s64 = ctx.r[11].s64 + -5964;
	// 832099E0: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 832099E4: 38E00044  li r7, 0x44
	ctx.r[7].s64 = 68;
	// 832099E8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 832099EC: 4BF003C5  bl 0x83109db0
	ctx.lr = 0x832099F0;
	sub_83109DB0(ctx, base);
	// 832099F0: 3D608331  lis r11, -0x7ccf
	ctx.r[11].s64 = -2093940736;
	// 832099F4: 1FDF0024  mulli r30, r31, 0x24
	ctx.r[30].s64 = ctx.r[31].s64 * 36;
	// 832099F8: 3BEB14C0  addi r31, r11, 0x14c0
	ctx.r[31].s64 = ctx.r[11].s64 + 5312;
	// 832099FC: 397F0014  addi r11, r31, 0x14
	ctx.r[11].s64 = ctx.r[31].s64 + 20;
	// 83209A00: 7FBE582E  lwzx r29, r30, r11
	ctx.r[29].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 83209A04: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 83209A08: 409A0020  bne cr6, 0x83209a28
	if !ctx.cr[6].eq {
	pc = 0x83209A28; continue 'dispatch;
	}
	// 83209A0C: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 83209A10: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 83209A14: 38ABE8A8  addi r5, r11, -0x1758
	ctx.r[5].s64 = ctx.r[11].s64 + -5976;
	// 83209A18: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 83209A1C: 38E00047  li r7, 0x47
	ctx.r[7].s64 = 71;
	// 83209A20: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83209A24: 4BF0038D  bl 0x83109db0
	ctx.lr = 0x83209A28;
	sub_83109DB0(ctx, base);
	// 83209A28: 809A000C  lwz r4, 0xc(r26)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(12 as u32) ) } as u64;
	// 83209A2C: 7C7EF82E  lwzx r3, r30, r31
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 83209A30: 7FA903A6  mtctr r29
	ctx.ctr.u64 = ctx.r[29].u64;
	// 83209A34: 4E800421  bctrl
	ctx.lr = 0x83209A38;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83209A38: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83209A3C: 4BA9FA14  b 0x82ca9450
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83209A40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83209A40 size=140
    let mut pc: u32 = 0x83209A40;
    'dispatch: loop {
        match pc {
            0x83209A40 => {
    //   block [0x83209A40..0x83209ACC)
	// 83209A40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83209A44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83209A48: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83209A4C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83209A50: 7C6B0774  extsb r11, r3
	ctx.r[11].s64 = ctx.r[3].s8 as i64;
	// 83209A54: 3BEBFFD0  addi r31, r11, -0x30
	ctx.r[31].s64 = ctx.r[11].s64 + -48;
	// 83209A58: 2F1F0001  cmpwi cr6, r31, 1
	ctx.cr[6].compare_i32(ctx.r[31].s32, 1, &mut ctx.xer);
	// 83209A5C: 40990018  ble cr6, 0x83209a74
	if !ctx.cr[6].gt {
	pc = 0x83209A74; continue 'dispatch;
	}
	// 83209A60: 2F0B005F  cmpwi cr6, r11, 0x5f
	ctx.cr[6].compare_i32(ctx.r[11].s32, 95, &mut ctx.xer);
	// 83209A64: 409A000C  bne cr6, 0x83209a70
	if !ctx.cr[6].eq {
	pc = 0x83209A70; continue 'dispatch;
	}
	// 83209A68: 3BE00002  li r31, 2
	ctx.r[31].s64 = 2;
	// 83209A6C: 4800003C  b 0x83209aa8
	pc = 0x83209AA8; continue 'dispatch;
	// 83209A70: 3BEBFF8C  addi r31, r11, -0x74
	ctx.r[31].s64 = ctx.r[11].s64 + -116;
	// 83209A74: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 83209A78: 4198000C  blt cr6, 0x83209a84
	if ctx.cr[6].lt {
	pc = 0x83209A84; continue 'dispatch;
	}
	// 83209A7C: 2F1F0006  cmpwi cr6, r31, 6
	ctx.cr[6].compare_i32(ctx.r[31].s32, 6, &mut ctx.xer);
	// 83209A80: 40990028  ble cr6, 0x83209aa8
	if !ctx.cr[6].gt {
	pc = 0x83209AA8; continue 'dispatch;
	}
	// 83209A84: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 83209A88: 3D408209  lis r10, -0x7df7
	ctx.r[10].s64 = -2113339392;
	// 83209A8C: 3D208208  lis r9, -0x7df8
	ctx.r[9].s64 = -2113404928;
	// 83209A90: 38CBE8E8  addi r6, r11, -0x1718
	ctx.r[6].s64 = ctx.r[11].s64 + -5912;
	// 83209A94: 38AAE93C  addi r5, r10, -0x16c4
	ctx.r[5].s64 = ctx.r[10].s64 + -5828;
	// 83209A98: 3889B734  addi r4, r9, -0x48cc
	ctx.r[4].s64 = ctx.r[9].s64 + -18636;
	// 83209A9C: 38E003F7  li r7, 0x3f7
	ctx.r[7].s64 = 1015;
	// 83209AA0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83209AA4: 4BF0030D  bl 0x83109db0
	ctx.lr = 0x83209AA8;
	sub_83109DB0(ctx, base);
	// 83209AA8: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 83209AAC: 57EA103A  slwi r10, r31, 2
	ctx.r[10].u32 = ctx.r[31].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83209AB0: 396BC348  addi r11, r11, -0x3cb8
	ctx.r[11].s64 = ctx.r[11].s64 + -15544;
	// 83209AB4: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 83209AB8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83209ABC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83209AC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83209AC4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83209AC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83209AD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83209AD0 size=120
    let mut pc: u32 = 0x83209AD0;
    'dispatch: loop {
        match pc {
            0x83209AD0 => {
    //   block [0x83209AD0..0x83209B48)
	// 83209AD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83209AD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83209AD8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83209ADC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83209AE0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83209AE4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83209AE8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83209AEC: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 83209AF0: 409A0028  bne cr6, 0x83209b18
	if !ctx.cr[6].eq {
	pc = 0x83209B18; continue 'dispatch;
	}
	// 83209AF4: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 83209AF8: 3D408208  lis r10, -0x7df8
	ctx.r[10].s64 = -2113404928;
	// 83209AFC: 3D208208  lis r9, -0x7df8
	ctx.r[9].s64 = -2113404928;
	// 83209B00: 38CB6188  addi r6, r11, 0x6188
	ctx.r[6].s64 = ctx.r[11].s64 + 24968;
	// 83209B04: 38AA61D8  addi r5, r10, 0x61d8
	ctx.r[5].s64 = ctx.r[10].s64 + 25048;
	// 83209B08: 3889B734  addi r4, r9, -0x48cc
	ctx.r[4].s64 = ctx.r[9].s64 + -18636;
	// 83209B0C: 38E0078C  li r7, 0x78c
	ctx.r[7].s64 = 1932;
	// 83209B10: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83209B14: 4BF0029D  bl 0x83109db0
	ctx.lr = 0x83209B18;
	sub_83109DB0(ctx, base);
	// 83209B18: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 83209B1C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 83209B20: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83209B24: 388B0001  addi r4, r11, 1
	ctx.r[4].s64 = ctx.r[11].s64 + 1;
	// 83209B28: 909F0014  stw r4, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[4].u32 ) };
	// 83209B2C: 4BFFC03D  bl 0x83205b68
	ctx.lr = 0x83209B30;
	sub_83205B68(ctx, base);
	// 83209B30: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83209B34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83209B38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83209B3C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83209B40: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83209B44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83209B48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83209B48 size=124
    let mut pc: u32 = 0x83209B48;
    'dispatch: loop {
        match pc {
            0x83209B48 => {
    //   block [0x83209B48..0x83209BC4)
	// 83209B48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83209B4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83209B50: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83209B54: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83209B58: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83209B5C: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 83209B60: 4198000C  blt cr6, 0x83209b6c
	if ctx.cr[6].lt {
	pc = 0x83209B6C; continue 'dispatch;
	}
	// 83209B64: 2F1F0088  cmpwi cr6, r31, 0x88
	ctx.cr[6].compare_i32(ctx.r[31].s32, 136, &mut ctx.xer);
	// 83209B68: 41980028  blt cr6, 0x83209b90
	if ctx.cr[6].lt {
	pc = 0x83209B90; continue 'dispatch;
	}
	// 83209B6C: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 83209B70: 3D408209  lis r10, -0x7df7
	ctx.r[10].s64 = -2113339392;
	// 83209B74: 3D208208  lis r9, -0x7df8
	ctx.r[9].s64 = -2113404928;
	// 83209B78: 38CBE988  addi r6, r11, -0x1678
	ctx.r[6].s64 = ctx.r[11].s64 + -5752;
	// 83209B7C: 38AAE950  addi r5, r10, -0x16b0
	ctx.r[5].s64 = ctx.r[10].s64 + -5808;
	// 83209B80: 3889B734  addi r4, r9, -0x48cc
	ctx.r[4].s64 = ctx.r[9].s64 + -18636;
	// 83209B84: 38E0146A  li r7, 0x146a
	ctx.r[7].s64 = 5226;
	// 83209B88: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83209B8C: 4BF00225  bl 0x83109db0
	ctx.lr = 0x83209B90;
	sub_83109DB0(ctx, base);
	// 83209B90: 3D608331  lis r11, -0x7ccf
	ctx.r[11].s64 = -2093940736;
	// 83209B94: 1D5F0024  mulli r10, r31, 0x24
	ctx.r[10].s64 = ctx.r[31].s64 * 36;
	// 83209B98: 396B14C0  addi r11, r11, 0x14c0
	ctx.r[11].s64 = ctx.r[11].s64 + 5312;
	// 83209B9C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83209BA0: 7C8A5A14  add r4, r10, r11
	ctx.r[4].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 83209BA4: 38A00024  li r5, 0x24
	ctx.r[5].s64 = 36;
	// 83209BA8: 4BA9F8D9  bl 0x82ca9480
	ctx.lr = 0x83209BAC;
	sub_82CA9480(ctx, base);
	// 83209BAC: 80610058  lwz r3, 0x58(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 83209BB0: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83209BB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83209BB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83209BBC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83209BC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83209BC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83209BC8 size=112
    let mut pc: u32 = 0x83209BC8;
    'dispatch: loop {
        match pc {
            0x83209BC8 => {
    //   block [0x83209BC8..0x83209C38)
	// 83209BC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83209BCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83209BD0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83209BD4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83209BD8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83209BDC: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 83209BE0: 4198000C  blt cr6, 0x83209bec
	if ctx.cr[6].lt {
	pc = 0x83209BEC; continue 'dispatch;
	}
	// 83209BE4: 2F1F0088  cmpwi cr6, r31, 0x88
	ctx.cr[6].compare_i32(ctx.r[31].s32, 136, &mut ctx.xer);
	// 83209BE8: 41980028  blt cr6, 0x83209c10
	if ctx.cr[6].lt {
	pc = 0x83209C10; continue 'dispatch;
	}
	// 83209BEC: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 83209BF0: 3D408209  lis r10, -0x7df7
	ctx.r[10].s64 = -2113339392;
	// 83209BF4: 3D208208  lis r9, -0x7df8
	ctx.r[9].s64 = -2113404928;
	// 83209BF8: 38CBE988  addi r6, r11, -0x1678
	ctx.r[6].s64 = ctx.r[11].s64 + -5752;
	// 83209BFC: 38AAE950  addi r5, r10, -0x16b0
	ctx.r[5].s64 = ctx.r[10].s64 + -5808;
	// 83209C00: 3889B734  addi r4, r9, -0x48cc
	ctx.r[4].s64 = ctx.r[9].s64 + -18636;
	// 83209C04: 38E01477  li r7, 0x1477
	ctx.r[7].s64 = 5239;
	// 83209C08: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83209C0C: 4BF001A5  bl 0x83109db0
	ctx.lr = 0x83209C10;
	sub_83109DB0(ctx, base);
	// 83209C10: 3D608331  lis r11, -0x7ccf
	ctx.r[11].s64 = -2093940736;
	// 83209C14: 1D5F0024  mulli r10, r31, 0x24
	ctx.r[10].s64 = ctx.r[31].s64 * 36;
	// 83209C18: 396B14C0  addi r11, r11, 0x14c0
	ctx.r[11].s64 = ctx.r[11].s64 + 5312;
	// 83209C1C: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 83209C20: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 83209C24: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83209C28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83209C2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83209C30: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83209C34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83209C38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83209C38 size=112
    let mut pc: u32 = 0x83209C38;
    'dispatch: loop {
        match pc {
            0x83209C38 => {
    //   block [0x83209C38..0x83209CA8)
	// 83209C38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83209C3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83209C40: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83209C44: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83209C48: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83209C4C: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 83209C50: 4198000C  blt cr6, 0x83209c5c
	if ctx.cr[6].lt {
	pc = 0x83209C5C; continue 'dispatch;
	}
	// 83209C54: 2F1F0088  cmpwi cr6, r31, 0x88
	ctx.cr[6].compare_i32(ctx.r[31].s32, 136, &mut ctx.xer);
	// 83209C58: 41980028  blt cr6, 0x83209c80
	if ctx.cr[6].lt {
	pc = 0x83209C80; continue 'dispatch;
	}
	// 83209C5C: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 83209C60: 3D408209  lis r10, -0x7df7
	ctx.r[10].s64 = -2113339392;
	// 83209C64: 3D208208  lis r9, -0x7df8
	ctx.r[9].s64 = -2113404928;
	// 83209C68: 38CBE988  addi r6, r11, -0x1678
	ctx.r[6].s64 = ctx.r[11].s64 + -5752;
	// 83209C6C: 38AAE950  addi r5, r10, -0x16b0
	ctx.r[5].s64 = ctx.r[10].s64 + -5808;
	// 83209C70: 3889B734  addi r4, r9, -0x48cc
	ctx.r[4].s64 = ctx.r[9].s64 + -18636;
	// 83209C74: 38E014C7  li r7, 0x14c7
	ctx.r[7].s64 = 5319;
	// 83209C78: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83209C7C: 4BF00135  bl 0x83109db0
	ctx.lr = 0x83209C80;
	sub_83109DB0(ctx, base);
	// 83209C80: 3D608331  lis r11, -0x7ccf
	ctx.r[11].s64 = -2093940736;
	// 83209C84: 1D5F0024  mulli r10, r31, 0x24
	ctx.r[10].s64 = ctx.r[31].s64 * 36;
	// 83209C88: 396B14C0  addi r11, r11, 0x14c0
	ctx.r[11].s64 = ctx.r[11].s64 + 5312;
	// 83209C8C: 396B000C  addi r11, r11, 0xc
	ctx.r[11].s64 = ctx.r[11].s64 + 12;
	// 83209C90: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 83209C94: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83209C98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83209C9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83209CA0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83209CA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83209CA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83209CA8 size=112
    let mut pc: u32 = 0x83209CA8;
    'dispatch: loop {
        match pc {
            0x83209CA8 => {
    //   block [0x83209CA8..0x83209D18)
	// 83209CA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83209CAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83209CB0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83209CB4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83209CB8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83209CBC: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 83209CC0: 4198000C  blt cr6, 0x83209ccc
	if ctx.cr[6].lt {
	pc = 0x83209CCC; continue 'dispatch;
	}
	// 83209CC4: 2F1F0088  cmpwi cr6, r31, 0x88
	ctx.cr[6].compare_i32(ctx.r[31].s32, 136, &mut ctx.xer);
	// 83209CC8: 41980028  blt cr6, 0x83209cf0
	if ctx.cr[6].lt {
	pc = 0x83209CF0; continue 'dispatch;
	}
	// 83209CCC: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 83209CD0: 3D408209  lis r10, -0x7df7
	ctx.r[10].s64 = -2113339392;
	// 83209CD4: 3D208208  lis r9, -0x7df8
	ctx.r[9].s64 = -2113404928;
	// 83209CD8: 38CBE988  addi r6, r11, -0x1678
	ctx.r[6].s64 = ctx.r[11].s64 + -5752;
	// 83209CDC: 38AAE950  addi r5, r10, -0x16b0
	ctx.r[5].s64 = ctx.r[10].s64 + -5808;
	// 83209CE0: 3889B734  addi r4, r9, -0x48cc
	ctx.r[4].s64 = ctx.r[9].s64 + -18636;
	// 83209CE4: 38E014E7  li r7, 0x14e7
	ctx.r[7].s64 = 5351;
	// 83209CE8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83209CEC: 4BF000C5  bl 0x83109db0
	ctx.lr = 0x83209CF0;
	sub_83109DB0(ctx, base);
	// 83209CF0: 3D608331  lis r11, -0x7ccf
	ctx.r[11].s64 = -2093940736;
	// 83209CF4: 1D5F0024  mulli r10, r31, 0x24
	ctx.r[10].s64 = ctx.r[31].s64 * 36;
	// 83209CF8: 396B14C0  addi r11, r11, 0x14c0
	ctx.r[11].s64 = ctx.r[11].s64 + 5312;
	// 83209CFC: 396B0018  addi r11, r11, 0x18
	ctx.r[11].s64 = ctx.r[11].s64 + 24;
	// 83209D00: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 83209D04: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83209D08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83209D0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83209D10: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83209D14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83209D18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83209D18 size=112
    let mut pc: u32 = 0x83209D18;
    'dispatch: loop {
        match pc {
            0x83209D18 => {
    //   block [0x83209D18..0x83209D88)
	// 83209D18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83209D1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83209D20: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83209D24: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83209D28: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83209D2C: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 83209D30: 4198000C  blt cr6, 0x83209d3c
	if ctx.cr[6].lt {
	pc = 0x83209D3C; continue 'dispatch;
	}
	// 83209D34: 2F1F0088  cmpwi cr6, r31, 0x88
	ctx.cr[6].compare_i32(ctx.r[31].s32, 136, &mut ctx.xer);
	// 83209D38: 41980028  blt cr6, 0x83209d60
	if ctx.cr[6].lt {
	pc = 0x83209D60; continue 'dispatch;
	}
	// 83209D3C: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 83209D40: 3D408209  lis r10, -0x7df7
	ctx.r[10].s64 = -2113339392;
	// 83209D44: 3D208208  lis r9, -0x7df8
	ctx.r[9].s64 = -2113404928;
	// 83209D48: 38CBE988  addi r6, r11, -0x1678
	ctx.r[6].s64 = ctx.r[11].s64 + -5752;
	// 83209D4C: 38AAE950  addi r5, r10, -0x16b0
	ctx.r[5].s64 = ctx.r[10].s64 + -5808;
	// 83209D50: 3889B734  addi r4, r9, -0x48cc
	ctx.r[4].s64 = ctx.r[9].s64 + -18636;
	// 83209D54: 38E014F3  li r7, 0x14f3
	ctx.r[7].s64 = 5363;
	// 83209D58: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83209D5C: 4BF00055  bl 0x83109db0
	ctx.lr = 0x83209D60;
	sub_83109DB0(ctx, base);
	// 83209D60: 3D608331  lis r11, -0x7ccf
	ctx.r[11].s64 = -2093940736;
	// 83209D64: 1D5F0024  mulli r10, r31, 0x24
	ctx.r[10].s64 = ctx.r[31].s64 * 36;
	// 83209D68: 396B14C0  addi r11, r11, 0x14c0
	ctx.r[11].s64 = ctx.r[11].s64 + 5312;
	// 83209D6C: 396B0010  addi r11, r11, 0x10
	ctx.r[11].s64 = ctx.r[11].s64 + 16;
	// 83209D70: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 83209D74: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83209D78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83209D7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83209D80: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83209D84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83209D88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83209D88 size=124
    let mut pc: u32 = 0x83209D88;
    'dispatch: loop {
        match pc {
            0x83209D88 => {
    //   block [0x83209D88..0x83209E04)
	// 83209D88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83209D8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83209D90: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83209D94: 2B030001  cmplwi cr6, r3, 1
	ctx.cr[6].compare_u32(ctx.r[3].u32, 1 as u32, &mut ctx.xer);
	// 83209D98: 41980058  blt cr6, 0x83209df0
	if ctx.cr[6].lt {
	pc = 0x83209DF0; continue 'dispatch;
	}
	// 83209D9C: 419A004C  beq cr6, 0x83209de8
	if ctx.cr[6].eq {
	pc = 0x83209DE8; continue 'dispatch;
	}
	// 83209DA0: 2B030003  cmplwi cr6, r3, 3
	ctx.cr[6].compare_u32(ctx.r[3].u32, 3 as u32, &mut ctx.xer);
	// 83209DA4: 4198003C  blt cr6, 0x83209de0
	if ctx.cr[6].lt {
	pc = 0x83209DE0; continue 'dispatch;
	}
	// 83209DA8: 419A0030  beq cr6, 0x83209dd8
	if ctx.cr[6].eq {
	pc = 0x83209DD8; continue 'dispatch;
	}
	// 83209DAC: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 83209DB0: 3D40820A  lis r10, -0x7df6
	ctx.r[10].s64 = -2113273856;
	// 83209DB4: 3D208208  lis r9, -0x7df8
	ctx.r[9].s64 = -2113404928;
	// 83209DB8: 38CBE988  addi r6, r11, -0x1678
	ctx.r[6].s64 = ctx.r[11].s64 + -5752;
	// 83209DBC: 38AA1700  addi r5, r10, 0x1700
	ctx.r[5].s64 = ctx.r[10].s64 + 5888;
	// 83209DC0: 3889B734  addi r4, r9, -0x48cc
	ctx.r[4].s64 = ctx.r[9].s64 + -18636;
	// 83209DC4: 38E01517  li r7, 0x1517
	ctx.r[7].s64 = 5399;
	// 83209DC8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83209DCC: 4BEFFFE5  bl 0x83109db0
	ctx.lr = 0x83209DD0;
	sub_83109DB0(ctx, base);
	// 83209DD0: 38600006  li r3, 6
	ctx.r[3].s64 = 6;
	// 83209DD4: 48000020  b 0x83209df4
	pc = 0x83209DF4; continue 'dispatch;
	// 83209DD8: 38600005  li r3, 5
	ctx.r[3].s64 = 5;
	// 83209DDC: 48000018  b 0x83209df4
	pc = 0x83209DF4; continue 'dispatch;
	// 83209DE0: 38600004  li r3, 4
	ctx.r[3].s64 = 4;
	// 83209DE4: 48000010  b 0x83209df4
	pc = 0x83209DF4; continue 'dispatch;
	// 83209DE8: 38600002  li r3, 2
	ctx.r[3].s64 = 2;
	// 83209DEC: 48000008  b 0x83209df4
	pc = 0x83209DF4; continue 'dispatch;
	// 83209DF0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83209DF4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83209DF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83209DFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83209E00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83209E08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83209E08 size=152
    let mut pc: u32 = 0x83209E08;
    'dispatch: loop {
        match pc {
            0x83209E08 => {
    //   block [0x83209E08..0x83209EA0)
	// 83209E08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83209E0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83209E10: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83209E14: 2B030001  cmplwi cr6, r3, 1
	ctx.cr[6].compare_u32(ctx.r[3].u32, 1 as u32, &mut ctx.xer);
	// 83209E18: 41980074  blt cr6, 0x83209e8c
	if ctx.cr[6].lt {
	pc = 0x83209E8C; continue 'dispatch;
	}
	// 83209E1C: 419A0068  beq cr6, 0x83209e84
	if ctx.cr[6].eq {
	pc = 0x83209E84; continue 'dispatch;
	}
	// 83209E20: 2B030003  cmplwi cr6, r3, 3
	ctx.cr[6].compare_u32(ctx.r[3].u32, 3 as u32, &mut ctx.xer);
	// 83209E24: 41980058  blt cr6, 0x83209e7c
	if ctx.cr[6].lt {
	pc = 0x83209E7C; continue 'dispatch;
	}
	// 83209E28: 419A004C  beq cr6, 0x83209e74
	if ctx.cr[6].eq {
	pc = 0x83209E74; continue 'dispatch;
	}
	// 83209E2C: 2B030005  cmplwi cr6, r3, 5
	ctx.cr[6].compare_u32(ctx.r[3].u32, 5 as u32, &mut ctx.xer);
	// 83209E30: 4198003C  blt cr6, 0x83209e6c
	if ctx.cr[6].lt {
	pc = 0x83209E6C; continue 'dispatch;
	}
	// 83209E34: 419A0030  beq cr6, 0x83209e64
	if ctx.cr[6].eq {
	pc = 0x83209E64; continue 'dispatch;
	}
	// 83209E38: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 83209E3C: 3D40820A  lis r10, -0x7df6
	ctx.r[10].s64 = -2113273856;
	// 83209E40: 3D208208  lis r9, -0x7df8
	ctx.r[9].s64 = -2113404928;
	// 83209E44: 38CBE988  addi r6, r11, -0x1678
	ctx.r[6].s64 = ctx.r[11].s64 + -5752;
	// 83209E48: 38AA1700  addi r5, r10, 0x1700
	ctx.r[5].s64 = ctx.r[10].s64 + 5888;
	// 83209E4C: 3889B734  addi r4, r9, -0x48cc
	ctx.r[4].s64 = ctx.r[9].s64 + -18636;
	// 83209E50: 38E0152E  li r7, 0x152e
	ctx.r[7].s64 = 5422;
	// 83209E54: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83209E58: 4BEFFF59  bl 0x83109db0
	ctx.lr = 0x83209E5C;
	sub_83109DB0(ctx, base);
	// 83209E5C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83209E60: 48000030  b 0x83209e90
	pc = 0x83209E90; continue 'dispatch;
	// 83209E64: 38600013  li r3, 0x13
	ctx.r[3].s64 = 19;
	// 83209E68: 48000028  b 0x83209e90
	pc = 0x83209E90; continue 'dispatch;
	// 83209E6C: 38600017  li r3, 0x17
	ctx.r[3].s64 = 23;
	// 83209E70: 48000020  b 0x83209e90
	pc = 0x83209E90; continue 'dispatch;
	// 83209E74: 38600016  li r3, 0x16
	ctx.r[3].s64 = 22;
	// 83209E78: 48000018  b 0x83209e90
	pc = 0x83209E90; continue 'dispatch;
	// 83209E7C: 38600015  li r3, 0x15
	ctx.r[3].s64 = 21;
	// 83209E80: 48000010  b 0x83209e90
	pc = 0x83209E90; continue 'dispatch;
	// 83209E84: 38600006  li r3, 6
	ctx.r[3].s64 = 6;
	// 83209E88: 48000008  b 0x83209e90
	pc = 0x83209E90; continue 'dispatch;
	// 83209E8C: 38600005  li r3, 5
	ctx.r[3].s64 = 5;
	// 83209E90: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83209E94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83209E98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83209E9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83209EA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83209EA0 size=108
    let mut pc: u32 = 0x83209EA0;
    'dispatch: loop {
        match pc {
            0x83209EA0 => {
    //   block [0x83209EA0..0x83209F0C)
	// 83209EA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83209EA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83209EA8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83209EAC: 2B030001  cmplwi cr6, r3, 1
	ctx.cr[6].compare_u32(ctx.r[3].u32, 1 as u32, &mut ctx.xer);
	// 83209EB0: 41980038  blt cr6, 0x83209ee8
	if ctx.cr[6].lt {
	pc = 0x83209EE8; continue 'dispatch;
	}
	// 83209EB4: 419A0050  beq cr6, 0x83209f04
	if ctx.cr[6].eq {
	pc = 0x83209F04; continue 'dispatch;
	}
	// 83209EB8: 2B030003  cmplwi cr6, r3, 3
	ctx.cr[6].compare_u32(ctx.r[3].u32, 3 as u32, &mut ctx.xer);
	// 83209EBC: 41980040  blt cr6, 0x83209efc
	if ctx.cr[6].lt {
	pc = 0x83209EFC; continue 'dispatch;
	}
	// 83209EC0: 419A0028  beq cr6, 0x83209ee8
	if ctx.cr[6].eq {
	pc = 0x83209EE8; continue 'dispatch;
	}
	// 83209EC4: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 83209EC8: 3D40820A  lis r10, -0x7df6
	ctx.r[10].s64 = -2113273856;
	// 83209ECC: 3D208208  lis r9, -0x7df8
	ctx.r[9].s64 = -2113404928;
	// 83209ED0: 38CBE988  addi r6, r11, -0x1678
	ctx.r[6].s64 = ctx.r[11].s64 + -5752;
	// 83209ED4: 38AA1700  addi r5, r10, 0x1700
	ctx.r[5].s64 = ctx.r[10].s64 + 5888;
	// 83209ED8: 3889B734  addi r4, r9, -0x48cc
	ctx.r[4].s64 = ctx.r[9].s64 + -18636;
	// 83209EDC: 38E01548  li r7, 0x1548
	ctx.r[7].s64 = 5448;
	// 83209EE0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83209EE4: 4BEFFECD  bl 0x83109db0
	ctx.lr = 0x83209EE8;
	sub_83109DB0(ctx, base);
	// 83209EE8: 38600003  li r3, 3
	ctx.r[3].s64 = 3;
	// 83209EEC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83209EF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83209EF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83209EF8: 4E800020  blr
	return;
	// 83209EFC: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83209F00: 4BFFFFEC  b 0x83209eec
	pc = 0x83209EEC; continue 'dispatch;
	// 83209F04: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83209F08: 4BFFFFE4  b 0x83209eec
	pc = 0x83209EEC; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83209F10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83209F10 size=84
    let mut pc: u32 = 0x83209F10;
    'dispatch: loop {
        match pc {
            0x83209F10 => {
    //   block [0x83209F10..0x83209F64)
	// 83209F10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83209F14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83209F18: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83209F1C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83209F20: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83209F24: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83209F28: 83FE0014  lwz r31, 0x14(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 83209F2C: 48000014  b 0x83209f40
	pc = 0x83209F40; continue 'dispatch;
	// 83209F30: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83209F34: 809E0000  lwz r4, 0(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83209F38: 4BFFF1C1  bl 0x832090f8
	ctx.lr = 0x83209F3C;
	sub_832090F8(ctx, base);
	// 83209F3C: 83FF0008  lwz r31, 8(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83209F40: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83209F44: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83209F48: 409AFFE8  bne cr6, 0x83209f30
	if !ctx.cr[6].eq {
	pc = 0x83209F30; continue 'dispatch;
	}
	// 83209F4C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83209F50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83209F54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83209F58: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83209F5C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83209F60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83209F68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83209F68 size=120
    let mut pc: u32 = 0x83209F68;
    'dispatch: loop {
        match pc {
            0x83209F68 => {
    //   block [0x83209F68..0x83209FE0)
	// 83209F68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83209F6C: 4BA9F4A1  bl 0x82ca940c
	ctx.lr = 0x83209F70;
	sub_82CA93D0(ctx, base);
	// 83209F70: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83209F74: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83209F78: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 83209F7C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 83209F80: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83209F84: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83209F88: 4800C831  bl 0x832167b8
	ctx.lr = 0x83209F8C;
	sub_832167B8(ctx, base);
	// 83209F8C: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 83209F90: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 83209F94: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 83209F98: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83209F9C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83209FA0: 816B0058  lwz r11, 0x58(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(88 as u32) ) } as u64;
	// 83209FA4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83209FA8: 4E800421  bctrl
	ctx.lr = 0x83209FAC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83209FAC: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 83209FB0: 2F1F0004  cmpwi cr6, r31, 4
	ctx.cr[6].compare_i32(ctx.r[31].s32, 4, &mut ctx.xer);
	// 83209FB4: 4198FFCC  blt cr6, 0x83209f80
	if ctx.cr[6].lt {
	pc = 0x83209F80; continue 'dispatch;
	}
	// 83209FB8: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83209FBC: 556B0253  rlwinm. r11, r11, 0, 9, 9
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83209FC0: 41820018  beq 0x83209fd8
	if ctx.cr[0].eq {
	pc = 0x83209FD8; continue 'dispatch;
	}
	// 83209FC4: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83209FC8: 3980BA98  li r12, -0x4568
	ctx.r[12].s64 = -17768;
	// 83209FCC: 7D6B6038  and r11, r11, r12
	ctx.r[11].u64 = ctx.r[11].u64 & ctx.r[12].u64;
	// 83209FD0: 616B3210  ori r11, r11, 0x3210
	ctx.r[11].u64 = ctx.r[11].u64 | 12816;
	// 83209FD4: 917E0004  stw r11, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 83209FD8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83209FDC: 4BA9F480  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83209FE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83209FE0 size=532
    let mut pc: u32 = 0x83209FE0;
    'dispatch: loop {
        match pc {
            0x83209FE0 => {
    //   block [0x83209FE0..0x8320A1F4)
	// 83209FE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83209FE4: 4BA9F421  bl 0x82ca9404
	ctx.lr = 0x83209FE8;
	sub_82CA93D0(ctx, base);
	// 83209FE8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83209FEC: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 83209FF0: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 83209FF4: 3B600001  li r27, 1
	ctx.r[27].s64 = 1;
	// 83209FF8: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 83209FFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8320A000: 9B610051  stb r27, 0x51(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(81 as u32), ctx.r[27].u8 ) };
	// 8320A004: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 8320A008: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8320A00C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8320A010: 99410052  stb r10, 0x52(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(82 as u32), ctx.r[10].u8 ) };
	// 8320A014: 7CFF3B78  mr r31, r7
	ctx.r[31].u64 = ctx.r[7].u64;
	// 8320A018: 99210050  stb r9, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u8 ) };
	// 8320A01C: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8320A020: 99010053  stb r8, 0x53(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(83 as u32), ctx.r[8].u8 ) };
	// 8320A024: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8320A028: 419A000C  beq cr6, 0x8320a034
	if ctx.cr[6].eq {
	pc = 0x8320A034; continue 'dispatch;
	}
	// 8320A02C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8320A030: 409A0028  bne cr6, 0x8320a058
	if !ctx.cr[6].eq {
	pc = 0x8320A058; continue 'dispatch;
	}
	// 8320A034: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 8320A038: 3D408209  lis r10, -0x7df7
	ctx.r[10].s64 = -2113339392;
	// 8320A03C: 3D208208  lis r9, -0x7df8
	ctx.r[9].s64 = -2113404928;
	// 8320A040: 38CBE8E8  addi r6, r11, -0x1718
	ctx.r[6].s64 = ctx.r[11].s64 + -5912;
	// 8320A044: 38AAE9DC  addi r5, r10, -0x1624
	ctx.r[5].s64 = ctx.r[10].s64 + -5668;
	// 8320A048: 3889B734  addi r4, r9, -0x48cc
	ctx.r[4].s64 = ctx.r[9].s64 + -18636;
	// 8320A04C: 38E010DF  li r7, 0x10df
	ctx.r[7].s64 = 4319;
	// 8320A050: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8320A054: 4BEFFD5D  bl 0x83109db0
	ctx.lr = 0x8320A058;
	sub_83109DB0(ctx, base);
	// 8320A058: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 8320A05C: 419A00D4  beq cr6, 0x8320a130
	if ctx.cr[6].eq {
	pc = 0x8320A130; continue 'dispatch;
	}
	// 8320A060: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8320A064: 556B0253  rlwinm. r11, r11, 0, 9, 9
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8320A068: 4182002C  beq 0x8320a094
	if ctx.cr[0].eq {
	pc = 0x8320A094; continue 'dispatch;
	}
	// 8320A06C: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 8320A070: 556A8BFE  srwi r10, r11, 0xf
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shr(15);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8320A074: 5569AAFE  srwi r9, r11, 0xb
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shr(11);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 8320A078: 5568C9FE  srwi r8, r11, 7
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shr(7);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 8320A07C: 7D4A4838  and r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 & ctx.r[9].u64;
	// 8320A080: 556BE8FE  srwi r11, r11, 3
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shr(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8320A084: 7D4A4038  and r10, r10, r8
	ctx.r[10].u64 = ctx.r[10].u64 & ctx.r[8].u64;
	// 8320A088: 7D4B5838  and r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 & ctx.r[11].u64;
	// 8320A08C: 556B07FE  clrlwi r11, r11, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	// 8320A090: 48000008  b 0x8320a098
	pc = 0x8320A098; continue 'dispatch;
	// 8320A094: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8320A098: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8320A09C: 419A001C  beq cr6, 0x8320a0b8
	if ctx.cr[6].eq {
	pc = 0x8320A0B8; continue 'dispatch;
	}
	// 8320A0A0: 7D7EFA14  add r11, r30, r31
	ctx.r[11].u64 = ctx.r[30].u64 + ctx.r[31].u64;
	// 8320A0A4: 394B0098  addi r10, r11, 0x98
	ctx.r[10].s64 = ctx.r[11].s64 + 152;
	// 8320A0A8: 894B0098  lbz r10, 0x98(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(152 as u32) ) } as u64;
	// 8320A0AC: 7D4A0034  cntlzw r10, r10
	ctx.r[10].u64 = if ctx.r[10].u32 == 0 { 32 } else { ctx.r[10].u32.leading_zeros() as u64 };
	// 8320A0B0: 554ADFFE  rlwinm r10, r10, 0x1b, 0x1f, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x0000001Fu64;
	// 8320A0B4: 994B0098  stb r10, 0x98(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(152 as u32), ctx.r[10].u8 ) };
	// 8320A0B8: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8320A0BC: 556B0253  rlwinm. r11, r11, 0, 9, 9
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8320A0C0: 41820010  beq 0x8320a0d0
	if ctx.cr[0].eq {
	pc = 0x8320A0D0; continue 'dispatch;
	}
	// 8320A0C4: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 8320A0C8: 556B67FE  rlwinm r11, r11, 0xc, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000FFFFFu64;
	// 8320A0CC: 48000008  b 0x8320a0d4
	pc = 0x8320A0D4; continue 'dispatch;
	// 8320A0D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8320A0D4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8320A0D8: 419A000C  beq cr6, 0x8320a0e4
	if ctx.cr[6].eq {
	pc = 0x8320A0E4; continue 'dispatch;
	}
	// 8320A0DC: 7D7EFA14  add r11, r30, r31
	ctx.r[11].u64 = ctx.r[30].u64 + ctx.r[31].u64;
	// 8320A0E0: 9B6B009E  stb r27, 0x9e(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(158 as u32), ctx.r[27].u8 ) };
	// 8320A0E4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8320A0E8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8320A0EC: 4800C6CD  bl 0x832167b8
	ctx.lr = 0x8320A0F0;
	sub_832167B8(ctx, base);
	// 8320A0F0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8320A0F4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8320A0F8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8320A0FC: 99610050  stb r11, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u8 ) };
	// 8320A100: 4800C6B9  bl 0x832167b8
	ctx.lr = 0x8320A104;
	sub_832167B8(ctx, base);
	// 8320A104: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8320A108: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 8320A10C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8320A110: 99610051  stb r11, 0x51(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(81 as u32), ctx.r[11].u8 ) };
	// 8320A114: 4800C6A5  bl 0x832167b8
	ctx.lr = 0x8320A118;
	sub_832167B8(ctx, base);
	// 8320A118: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8320A11C: 38800003  li r4, 3
	ctx.r[4].s64 = 3;
	// 8320A120: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8320A124: 99610052  stb r11, 0x52(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(82 as u32), ctx.r[11].u8 ) };
	// 8320A128: 4800C691  bl 0x832167b8
	ctx.lr = 0x8320A12C;
	sub_832167B8(ctx, base);
	// 8320A12C: 98610053  stb r3, 0x53(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(83 as u32), ctx.r[3].u8 ) };
	// 8320A130: 89610054  lbz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8320A134: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 8320A138: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8320A13C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8320A140: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8320A144: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8320A148: 7D6B50AE  lbzx r11, r11, r10
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 8320A14C: 81290058  lwz r9, 0x58(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(88 as u32) ) } as u64;
	// 8320A150: 7D660774  extsb r6, r11
	ctx.r[6].s64 = ctx.r[11].s8 as i64;
	// 8320A154: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 8320A158: 4E800421  bctrl
	ctx.lr = 0x8320A15C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8320A15C: 89610055  lbz r11, 0x55(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(85 as u32) ) } as u64;
	// 8320A160: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 8320A164: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8320A168: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8320A16C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8320A170: 7D6B50AE  lbzx r11, r11, r10
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 8320A174: 7D660774  extsb r6, r11
	ctx.r[6].s64 = ctx.r[11].s8 as i64;
	// 8320A178: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8320A17C: 816B0058  lwz r11, 0x58(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(88 as u32) ) } as u64;
	// 8320A180: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8320A184: 4E800421  bctrl
	ctx.lr = 0x8320A188;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8320A188: 89610056  lbz r11, 0x56(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(86 as u32) ) } as u64;
	// 8320A18C: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 8320A190: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 8320A194: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8320A198: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8320A19C: 7D6B50AE  lbzx r11, r11, r10
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 8320A1A0: 7D660774  extsb r6, r11
	ctx.r[6].s64 = ctx.r[11].s8 as i64;
	// 8320A1A4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8320A1A8: 816B0058  lwz r11, 0x58(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(88 as u32) ) } as u64;
	// 8320A1AC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8320A1B0: 4E800421  bctrl
	ctx.lr = 0x8320A1B4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8320A1B4: 89610057  lbz r11, 0x57(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(87 as u32) ) } as u64;
	// 8320A1B8: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 8320A1BC: 38A00003  li r5, 3
	ctx.r[5].s64 = 3;
	// 8320A1C0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8320A1C4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8320A1C8: 7D6B50AE  lbzx r11, r11, r10
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 8320A1CC: 7D660774  extsb r6, r11
	ctx.r[6].s64 = ctx.r[11].s8 as i64;
	// 8320A1D0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8320A1D4: 816B0058  lwz r11, 0x58(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(88 as u32) ) } as u64;
	// 8320A1D8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8320A1DC: 4E800421  bctrl
	ctx.lr = 0x8320A1E0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8320A1E0: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 8320A1E4: 816BC364  lwz r11, -0x3c9c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-15516 as u32) ) } as u64;
	// 8320A1E8: 917C0000  stw r11, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8320A1EC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8320A1F0: 4BA9F264  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8320A1F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8320A1F8 size=820
    let mut pc: u32 = 0x8320A1F8;
    'dispatch: loop {
        match pc {
            0x8320A1F8 => {
    //   block [0x8320A1F8..0x8320A52C)
	// 8320A1F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8320A1FC: 4BA9F1F9  bl 0x82ca93f4
	ctx.lr = 0x8320A200;
	sub_82CA93D0(ctx, base);
	// 8320A200: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8320A204: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8320A208: 4BFDB821  bl 0x831e5a28
	ctx.lr = 0x8320A20C;
	sub_831E5A28(ctx, base);
	// 8320A20C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8320A210: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8320A214: 408200C4  bne 0x8320a2d8
	if !ctx.cr[0].eq {
	pc = 0x8320A2D8; continue 'dispatch;
	}
	// 8320A218: 897F0844  lbz r11, 0x844(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(2116 as u32) ) } as u64;
	// 8320A21C: 3B9F00B8  addi r28, r31, 0xb8
	ctx.r[28].s64 = ctx.r[31].s64 + 184;
	// 8320A220: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8320A224: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8320A228: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8320A22C: 41820020  beq 0x8320a24c
	if ctx.cr[0].eq {
	pc = 0x8320A24C; continue 'dispatch;
	}
	// 8320A230: 80DF084C  lwz r6, 0x84c(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2124 as u32) ) } as u64;
	// 8320A234: 80FF0850  lwz r7, 0x850(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2128 as u32) ) } as u64;
	// 8320A238: 7D6600D0  neg r11, r6
	ctx.r[11].s64 = -ctx.r[6].s64;
	// 8320A23C: 80BF0848  lwz r5, 0x848(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2120 as u32) ) } as u64;
	// 8320A240: 917F0854  stw r11, 0x854(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(2132 as u32), ctx.r[11].u32 ) };
	// 8320A244: 4BFF7C3D  bl 0x83201e80
	ctx.lr = 0x8320A248;
	sub_83201E80(ctx, base);
	// 8320A248: 48000018  b 0x8320a260
	pc = 0x8320A260; continue 'dispatch;
	// 8320A24C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8320A250: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8320A254: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8320A258: 4BFF7C29  bl 0x83201e80
	ctx.lr = 0x8320A25C;
	sub_83201E80(ctx, base);
	// 8320A25C: 93DF0854  stw r30, 0x854(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(2132 as u32), ctx.r[30].u32 ) };
	// 8320A260: 39630001  addi r11, r3, 1
	ctx.r[11].s64 = ctx.r[3].s64 + 1;
	// 8320A264: 3BBF0448  addi r29, r31, 0x448
	ctx.r[29].s64 = ctx.r[31].s64 + 1096;
	// 8320A268: 917F0824  stw r11, 0x824(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(2084 as u32), ctx.r[11].u32 ) };
	// 8320A26C: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8320A270: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8320A274: 419A0050  beq cr6, 0x8320a2c4
	if ctx.cr[6].eq {
	pc = 0x8320A2C4; continue 'dispatch;
	}
	// 8320A278: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8320A27C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8320A280: 4BFED019  bl 0x831f7298
	ctx.lr = 0x8320A284;
	sub_831F7298(ctx, base);
	// 8320A284: 897F0844  lbz r11, 0x844(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(2116 as u32) ) } as u64;
	// 8320A288: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8320A28C: 41820038  beq 0x8320a2c4
	if ctx.cr[0].eq {
	pc = 0x8320A2C4; continue 'dispatch;
	}
	// 8320A290: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8320A294: 815F0848  lwz r10, 0x848(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2120 as u32) ) } as u64;
	// 8320A298: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 8320A29C: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 8320A2A0: 409A0024  bne cr6, 0x8320a2c4
	if !ctx.cr[6].eq {
	pc = 0x8320A2C4; continue 'dispatch;
	}
	// 8320A2A4: 817F0850  lwz r11, 0x850(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2128 as u32) ) } as u64;
	// 8320A2A8: 815F084C  lwz r10, 0x84c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2124 as u32) ) } as u64;
	// 8320A2AC: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 8320A2B0: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8320A2B4: 40980010  bge cr6, 0x8320a2c4
	if !ctx.cr[6].lt {
	pc = 0x8320A2C4; continue 'dispatch;
	}
	// 8320A2B8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8320A2BC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8320A2C0: 4BFECF71  bl 0x831f7230
	ctx.lr = 0x8320A2C4;
	sub_831F7230(ctx, base);
	// 8320A2C4: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 8320A2C8: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 8320A2CC: 2F1E0010  cmpwi cr6, r30, 0x10
	ctx.cr[6].compare_i32(ctx.r[30].s32, 16, &mut ctx.xer);
	// 8320A2D0: 4198FF9C  blt cr6, 0x8320a26c
	if ctx.cr[6].lt {
	pc = 0x8320A26C; continue 'dispatch;
	}
	// 8320A2D4: 48000248  b 0x8320a51c
	pc = 0x8320A51C; continue 'dispatch;
	// 8320A2D8: 897F0834  lbz r11, 0x834(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(2100 as u32) ) } as u64;
	// 8320A2DC: 3AE00001  li r23, 1
	ctx.r[23].s64 = 1;
	// 8320A2E0: 93DF0840  stw r30, 0x840(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(2112 as u32), ctx.r[30].u32 ) };
	// 8320A2E4: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8320A2E8: 93DF0854  stw r30, 0x854(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(2132 as u32), ctx.r[30].u32 ) };
	// 8320A2EC: 7EF8BB78  mr r24, r23
	ctx.r[24].u64 = ctx.r[23].u64;
	// 8320A2F0: 418200E8  beq 0x8320a3d8
	if ctx.cr[0].eq {
	pc = 0x8320A3D8; continue 'dispatch;
	}
	// 8320A2F4: 817F0838  lwz r11, 0x838(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2104 as u32) ) } as u64;
	// 8320A2F8: 813F083C  lwz r9, 0x83c(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2108 as u32) ) } as u64;
	// 8320A2FC: 210B0001  subfic r8, r11, 1
	ctx.xer.ca = ctx.r[11].u32 <= 1 as u32;
	ctx.r[8].s64 = (1 as i64) - ctx.r[11].s64;
	// 8320A300: 7D4B4850  subf r10, r11, r9
	ctx.r[10].s64 = ctx.r[9].s64 - ctx.r[11].s64;
	// 8320A304: 7D7D5B78  mr r29, r11
	ctx.r[29].u64 = ctx.r[11].u64;
	// 8320A308: 911F0840  stw r8, 0x840(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(2112 as u32), ctx.r[8].u32 ) };
	// 8320A30C: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 8320A310: 3B0A0001  addi r24, r10, 1
	ctx.r[24].s64 = ctx.r[10].s64 + 1;
	// 8320A314: 409800C4  bge cr6, 0x8320a3d8
	if !ctx.cr[6].lt {
	pc = 0x8320A3D8; continue 'dispatch;
	}
	// 8320A318: 817F005C  lwz r11, 0x5c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 8320A31C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8320A320: 807F00AC  lwz r3, 0xac(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(172 as u32) ) } as u64;
	// 8320A324: 808B002C  lwz r4, 0x2c(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 8320A328: 4BFEA911  bl 0x831f4c38
	ctx.lr = 0x8320A32C;
	sub_831F4C38(ctx, base);
	// 8320A32C: 7C7A1B79  or. r26, r3, r3
	ctx.r[26].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[26].s32, 0, &mut ctx.xer);
	// 8320A330: 41820098  beq 0x8320a3c8
	if ctx.cr[0].eq {
	pc = 0x8320A3C8; continue 'dispatch;
	}
	// 8320A334: 7FDCF378  mr r28, r30
	ctx.r[28].u64 = ctx.r[30].u64;
	// 8320A338: 7FDBF378  mr r27, r30
	ctx.r[27].u64 = ctx.r[30].u64;
	// 8320A33C: 7EF9BB78  mr r25, r23
	ctx.r[25].u64 = ctx.r[23].u64;
	// 8320A340: 817A0028  lwz r11, 0x28(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(40 as u32) ) } as u64;
	// 8320A344: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8320A348: 7F195040  cmplw cr6, r25, r10
	ctx.cr[6].compare_u32(ctx.r[25].u32, ctx.r[10].u32, &mut ctx.xer);
	// 8320A34C: 4099000C  ble cr6, 0x8320a358
	if !ctx.cr[6].gt {
	pc = 0x8320A358; continue 'dispatch;
	}
	// 8320A350: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 8320A354: 48000010  b 0x8320a364
	pc = 0x8320A364; continue 'dispatch;
	// 8320A358: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8320A35C: 7EEBBB78  mr r11, r23
	ctx.r[11].u64 = ctx.r[23].u64;
	// 8320A360: 7F8AD82E  lwzx r28, r10, r27
	ctx.r[28].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[27].u32)) } as u64;
	// 8320A364: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8320A368: 41820030  beq 0x8320a398
	if ctx.cr[0].eq {
	pc = 0x8320A398; continue 'dispatch;
	}
	// 8320A36C: 817C00E4  lwz r11, 0xe4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(228 as u32) ) } as u64;
	// 8320A370: 556A07FF  clrlwi. r10, r11, 0x1f
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8320A374: 41820018  beq 0x8320a38c
	if ctx.cr[0].eq {
	pc = 0x8320A38C; continue 'dispatch;
	}
	// 8320A378: 616B0010  ori r11, r11, 0x10
	ctx.r[11].u64 = ctx.r[11].u64 | 16;
	// 8320A37C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8320A380: 917C00E4  stw r11, 0xe4(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(228 as u32), ctx.r[11].u32 ) };
	// 8320A384: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8320A388: 4BFDBA21  bl 0x831e5da8
	ctx.lr = 0x8320A38C;
	sub_831E5DA8(ctx, base);
	// 8320A38C: 3B390001  addi r25, r25, 1
	ctx.r[25].s64 = ctx.r[25].s64 + 1;
	// 8320A390: 3B7B0004  addi r27, r27, 4
	ctx.r[27].s64 = ctx.r[27].s64 + 4;
	// 8320A394: 4BFFFFAC  b 0x8320a340
	pc = 0x8320A340; continue 'dispatch;
	// 8320A398: 817F0840  lwz r11, 0x840(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2112 as u32) ) } as u64;
	// 8320A39C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8320A3A0: 9AFA0005  stb r23, 5(r26)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[26].u32.wrapping_add(5 as u32), ctx.r[23].u8 ) };
	// 8320A3A4: 7D6BEA14  add r11, r11, r29
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 8320A3A8: 917A0010  stw r11, 0x10(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 8320A3AC: 817F0840  lwz r11, 0x840(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2112 as u32) ) } as u64;
	// 8320A3B0: 7C8BEA14  add r4, r11, r29
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 8320A3B4: 4BFECEE5  bl 0x831f7298
	ctx.lr = 0x8320A3B8;
	sub_831F7298(ctx, base);
	// 8320A3B8: 817F0840  lwz r11, 0x840(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2112 as u32) ) } as u64;
	// 8320A3BC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8320A3C0: 7C8BEA14  add r4, r11, r29
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 8320A3C4: 4BFECE6D  bl 0x831f7230
	ctx.lr = 0x8320A3C8;
	sub_831F7230(ctx, base);
	// 8320A3C8: 817F083C  lwz r11, 0x83c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2108 as u32) ) } as u64;
	// 8320A3CC: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 8320A3D0: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8320A3D4: 4198FF44  blt cr6, 0x8320a318
	if ctx.cr[6].lt {
	pc = 0x8320A318; continue 'dispatch;
	}
	// 8320A3D8: 897F0844  lbz r11, 0x844(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(2116 as u32) ) } as u64;
	// 8320A3DC: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8320A3E0: 41820118  beq 0x8320a4f8
	if ctx.cr[0].eq {
	pc = 0x8320A4F8; continue 'dispatch;
	}
	// 8320A3E4: 817F084C  lwz r11, 0x84c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2124 as u32) ) } as u64;
	// 8320A3E8: 815F0850  lwz r10, 0x850(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2128 as u32) ) } as u64;
	// 8320A3EC: 7D2BC050  subf r9, r11, r24
	ctx.r[9].s64 = ctx.r[24].s64 - ctx.r[11].s64;
	// 8320A3F0: 7D7D5B78  mr r29, r11
	ctx.r[29].u64 = ctx.r[11].u64;
	// 8320A3F4: 913F0854  stw r9, 0x854(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(2132 as u32), ctx.r[9].u32 ) };
	// 8320A3F8: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 8320A3FC: 409800FC  bge cr6, 0x8320a4f8
	if !ctx.cr[6].lt {
	pc = 0x8320A4F8; continue 'dispatch;
	}
	// 8320A400: 3D008209  lis r8, -0x7df7
	ctx.r[8].s64 = -2113339392;
	// 8320A404: 3D208208  lis r9, -0x7df8
	ctx.r[9].s64 = -2113404928;
	// 8320A408: 3D408209  lis r10, -0x7df7
	ctx.r[10].s64 = -2113339392;
	// 8320A40C: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 8320A410: 3F408209  lis r26, -0x7df7
	ctx.r[26].s64 = -2113339392;
	// 8320A414: 3B28EA08  addi r25, r8, -0x15f8
	ctx.r[25].s64 = ctx.r[8].s64 + -5624;
	// 8320A418: 3B89B734  addi r28, r9, -0x48cc
	ctx.r[28].s64 = ctx.r[9].s64 + -18636;
	// 8320A41C: 3B0AEA00  addi r24, r10, -0x1600
	ctx.r[24].s64 = ctx.r[10].s64 + -5632;
	// 8320A420: 3B6BE8E8  addi r27, r11, -0x1718
	ctx.r[27].s64 = ctx.r[11].s64 + -5912;
	// 8320A424: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8320A428: 809F0848  lwz r4, 0x848(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2120 as u32) ) } as u64;
	// 8320A42C: 807F00AC  lwz r3, 0xac(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(172 as u32) ) } as u64;
	// 8320A430: 4BFEA809  bl 0x831f4c38
	ctx.lr = 0x8320A434;
	sub_831F4C38(ctx, base);
	// 8320A434: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8320A438: 418200B0  beq 0x8320a4e8
	if ctx.cr[0].eq {
	pc = 0x8320A4E8; continue 'dispatch;
	}
	// 8320A43C: 81630038  lwz r11, 0x38(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(56 as u32) ) } as u64;
	// 8320A440: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8320A444: 814B00E4  lwz r10, 0xe4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(228 as u32) ) } as u64;
	// 8320A448: 614A0008  ori r10, r10, 8
	ctx.r[10].u64 = ctx.r[10].u64 | 8;
	// 8320A44C: 914B00E4  stw r10, 0xe4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(228 as u32), ctx.r[10].u32 ) };
	// 8320A450: 809AC370  lwz r4, -0x3c90(r26)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(-15504 as u32) ) } as u64;
	// 8320A454: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8320A458: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 8320A45C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8320A460: 4E800421  bctrl
	ctx.lr = 0x8320A464;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8320A464: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8320A468: 4082001C  bne 0x8320a484
	if !ctx.cr[0].eq {
	pc = 0x8320A484; continue 'dispatch;
	}
	// 8320A46C: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 8320A470: 7F05C378  mr r5, r24
	ctx.r[5].u64 = ctx.r[24].u64;
	// 8320A474: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8320A478: 38E01199  li r7, 0x1199
	ctx.r[7].s64 = 4505;
	// 8320A47C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8320A480: 4BEFF931  bl 0x83109db0
	ctx.lr = 0x8320A484;
	sub_83109DB0(ctx, base);
	// 8320A484: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8320A488: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8320A48C: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 8320A490: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8320A494: 4E800421  bctrl
	ctx.lr = 0x8320A498;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8320A498: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8320A49C: 4082001C  bne 0x8320a4b8
	if !ctx.cr[0].eq {
	pc = 0x8320A4B8; continue 'dispatch;
	}
	// 8320A4A0: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 8320A4A4: 7F25CB78  mr r5, r25
	ctx.r[5].u64 = ctx.r[25].u64;
	// 8320A4A8: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8320A4AC: 38E0119A  li r7, 0x119a
	ctx.r[7].s64 = 4506;
	// 8320A4B0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8320A4B4: 4BEFF8FD  bl 0x83109db0
	ctx.lr = 0x8320A4B8;
	sub_83109DB0(ctx, base);
	// 8320A4B8: 817F0854  lwz r11, 0x854(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2132 as u32) ) } as u64;
	// 8320A4BC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8320A4C0: 9AFE0005  stb r23, 5(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(5 as u32), ctx.r[23].u8 ) };
	// 8320A4C4: 7D6BEA14  add r11, r11, r29
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 8320A4C8: 917E0010  stw r11, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 8320A4CC: 817F0854  lwz r11, 0x854(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2132 as u32) ) } as u64;
	// 8320A4D0: 7C8BEA14  add r4, r11, r29
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 8320A4D4: 4BFECDC5  bl 0x831f7298
	ctx.lr = 0x8320A4D8;
	sub_831F7298(ctx, base);
	// 8320A4D8: 817F0854  lwz r11, 0x854(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2132 as u32) ) } as u64;
	// 8320A4DC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8320A4E0: 7C8BEA14  add r4, r11, r29
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 8320A4E4: 4BFECD4D  bl 0x831f7230
	ctx.lr = 0x8320A4E8;
	sub_831F7230(ctx, base);
	// 8320A4E8: 817F0850  lwz r11, 0x850(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2128 as u32) ) } as u64;
	// 8320A4EC: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 8320A4F0: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8320A4F4: 4198FF30  blt cr6, 0x8320a424
	if ctx.cr[6].lt {
	pc = 0x8320A424; continue 'dispatch;
	}
	// 8320A4F8: 3B9F00B8  addi r28, r31, 0xb8
	ctx.r[28].s64 = ctx.r[31].s64 + 184;
	// 8320A4FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8320A500: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8320A504: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8320A508: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8320A50C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8320A510: 4BFF7971  bl 0x83201e80
	ctx.lr = 0x8320A514;
	sub_83201E80(ctx, base);
	// 8320A514: 39630001  addi r11, r3, 1
	ctx.r[11].s64 = ctx.r[3].s64 + 1;
	// 8320A518: 917F0824  stw r11, 0x824(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(2084 as u32), ctx.r[11].u32 ) };
	// 8320A51C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8320A520: 4BFF6EC9  bl 0x832013e8
	ctx.lr = 0x8320A524;
	sub_832013E8(ctx, base);
	// 8320A524: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 8320A528: 4BA9EF1C  b 0x82ca9444
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8320A530(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8320A530 size=276
    let mut pc: u32 = 0x8320A530;
    'dispatch: loop {
        match pc {
            0x8320A530 => {
    //   block [0x8320A530..0x8320A644)
	// 8320A530: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8320A534: 4BA9EED1  bl 0x82ca9404
	ctx.lr = 0x8320A538;
	sub_82CA93D0(ctx, base);
	// 8320A538: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8320A53C: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 8320A540: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8320A544: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 8320A548: 817C0578  lwz r11, 0x578(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(1400 as u32) ) } as u64;
	// 8320A54C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8320A550: 409A0040  bne cr6, 0x8320a590
	if !ctx.cr[6].eq {
	pc = 0x8320A590; continue 'dispatch;
	}
	// 8320A554: 837C05B0  lwz r27, 0x5b0(r28)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(1456 as u32) ) } as u64;
	// 8320A558: 38800028  li r4, 0x28
	ctx.r[4].s64 = 40;
	// 8320A55C: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 8320A560: 4BFC1C39  bl 0x831cc198
	ctx.lr = 0x8320A564;
	sub_831CC198(ctx, base);
	// 8320A564: 37E30004  addic. r31, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[31].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 8320A568: 93630000  stw r27, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 8320A56C: 4182001C  beq 0x8320a588
	if ctx.cr[0].eq {
	pc = 0x8320A588; continue 'dispatch;
	}
	// 8320A570: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 8320A574: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 8320A578: 93BF0000  stw r29, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 8320A57C: 93BF0008  stw r29, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 8320A580: 4BFF8B89  bl 0x83203108
	ctx.lr = 0x8320A584;
	sub_83203108(ctx, base);
	// 8320A584: 48000008  b 0x8320a58c
	pc = 0x8320A58C; continue 'dispatch;
	// 8320A588: 7FBFEB78  mr r31, r29
	ctx.r[31].u64 = ctx.r[29].u64;
	// 8320A58C: 93FC0578  stw r31, 0x578(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(1400 as u32), ctx.r[31].u32 ) };
	// 8320A590: 807C0578  lwz r3, 0x578(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(1400 as u32) ) } as u64;
	// 8320A594: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8320A598: 419A0054  beq cr6, 0x8320a5ec
	if ctx.cr[6].eq {
	pc = 0x8320A5EC; continue 'dispatch;
	}
	// 8320A59C: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8320A5A0: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8320A5A4: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8320A5A8: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 8320A5AC: 409A0028  bne cr6, 0x8320a5d4
	if !ctx.cr[6].eq {
	pc = 0x8320A5D4; continue 'dispatch;
	}
	// 8320A5B0: 813E0004  lwz r9, 4(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8320A5B4: 810B0004  lwz r8, 4(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8320A5B8: 7F094000  cmpw cr6, r9, r8
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[8].s32, &mut ctx.xer);
	// 8320A5BC: 409A0018  bne cr6, 0x8320a5d4
	if !ctx.cr[6].eq {
	pc = 0x8320A5D4; continue 'dispatch;
	}
	// 8320A5C0: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8320A5C4: 813E0008  lwz r9, 8(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8320A5C8: 7F095800  cmpw cr6, r9, r11
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8320A5CC: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8320A5D0: 419A0008  beq cr6, 0x8320a5d8
	if ctx.cr[6].eq {
	pc = 0x8320A5D8; continue 'dispatch;
	}
	// 8320A5D4: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 8320A5D8: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8320A5DC: 40820060  bne 0x8320a63c
	if !ctx.cr[0].eq {
	pc = 0x8320A63C; continue 'dispatch;
	}
	// 8320A5E0: 80630008  lwz r3, 8(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 8320A5E4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8320A5E8: 409AFFB8  bne cr6, 0x8320a5a0
	if !ctx.cr[6].eq {
	pc = 0x8320A5A0; continue 'dispatch;
	}
	// 8320A5EC: 837C05B0  lwz r27, 0x5b0(r28)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(1456 as u32) ) } as u64;
	// 8320A5F0: 38800028  li r4, 0x28
	ctx.r[4].s64 = 40;
	// 8320A5F4: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 8320A5F8: 4BFC1BA1  bl 0x831cc198
	ctx.lr = 0x8320A5FC;
	sub_831CC198(ctx, base);
	// 8320A5FC: 37E30004  addic. r31, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[31].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 8320A600: 93630000  stw r27, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 8320A604: 41820020  beq 0x8320a624
	if ctx.cr[0].eq {
	pc = 0x8320A624; continue 'dispatch;
	}
	// 8320A608: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 8320A60C: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 8320A610: 93BF0000  stw r29, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 8320A614: 93BF0008  stw r29, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 8320A618: 4BFF8AF1  bl 0x83203108
	ctx.lr = 0x8320A61C;
	sub_83203108(ctx, base);
	// 8320A61C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8320A620: 48000008  b 0x8320a628
	pc = 0x8320A628; continue 'dispatch;
	// 8320A624: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8320A628: 817C0578  lwz r11, 0x578(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(1400 as u32) ) } as u64;
	// 8320A62C: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8320A630: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8320A634: 817C0578  lwz r11, 0x578(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(1400 as u32) ) } as u64;
	// 8320A638: 906B0008  stw r3, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[3].u32 ) };
	// 8320A63C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8320A640: 4BA9EE14  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


