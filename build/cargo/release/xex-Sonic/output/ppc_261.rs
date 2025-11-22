pub fn sub_830CBB98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830CBB98 size=104
    let mut pc: u32 = 0x830CBB98;
    'dispatch: loop {
        match pc {
            0x830CBB98 => {
    //   block [0x830CBB98..0x830CBC00)
	// 830CBB98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830CBB9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830CBBA0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830CBBA4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830CBBA8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830CBBAC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830CBBB0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 830CBBB4: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830CBBB8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 830CBBBC: 419A0008  beq cr6, 0x830cbbc4
	if ctx.cr[6].eq {
	pc = 0x830CBBC4; continue 'dispatch;
	}
	// 830CBBC0: 4B1F46A9  bl 0x822c0268
	ctx.lr = 0x830CBBC4;
	sub_822C0268(ctx, base);
	// 830CBBC4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830CBBC8: 57CA07FF  clrlwi. r10, r30, 0x1f
	ctx.r[10].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 830CBBCC: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 830CBBD0: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 830CBBD4: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 830CBBD8: 4182000C  beq 0x830cbbe4
	if ctx.cr[0].eq {
	pc = 0x830CBBE4; continue 'dispatch;
	}
	// 830CBBDC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830CBBE0: 4B1F4689  bl 0x822c0268
	ctx.lr = 0x830CBBE4;
	sub_822C0268(ctx, base);
	// 830CBBE4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830CBBE8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830CBBEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830CBBF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830CBBF4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830CBBF8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830CBBFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CBC00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830CBC00 size=144
    let mut pc: u32 = 0x830CBC00;
    'dispatch: loop {
        match pc {
            0x830CBC00 => {
    //   block [0x830CBC00..0x830CBC90)
	// 830CBC00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830CBC04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830CBC08: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830CBC0C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830CBC10: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830CBC14: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830CBC18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830CBC1C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 830CBC20: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 830CBC24: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 830CBC28: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 830CBC2C: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 830CBC30: 409A000C  bne cr6, 0x830cbc3c
	if !ctx.cr[6].eq {
	pc = 0x830CBC3C; continue 'dispatch;
	}
	// 830CBC34: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830CBC38: 48000040  b 0x830cbc78
	pc = 0x830CBC78; continue 'dispatch;
	// 830CBC3C: 3D600FFF  lis r11, 0xfff
	ctx.r[11].s64 = 268369920;
	// 830CBC40: 616BFFFF  ori r11, r11, 0xffff
	ctx.r[11].u64 = ctx.r[11].u64 | 65535;
	// 830CBC44: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 830CBC48: 4099000C  ble cr6, 0x830cbc54
	if !ctx.cr[6].gt {
	pc = 0x830CBC54; continue 'dispatch;
	}
	// 830CBC4C: 4BFF94ED  bl 0x830c5138
	ctx.lr = 0x830CBC50;
	sub_830C5138(ctx, base);
	// 830CBC50: 48000024  b 0x830cbc74
	pc = 0x830CBC74; continue 'dispatch;
	// 830CBC54: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830CBC58: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830CBC5C: 4BFF3CBD  bl 0x830bf918
	ctx.lr = 0x830CBC60;
	sub_830BF918(ctx, base);
	// 830CBC60: 57CB2036  slwi r11, r30, 4
	ctx.r[11].u32 = ctx.r[30].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 830CBC64: 907F0004  stw r3, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 830CBC68: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 830CBC6C: 907F0008  stw r3, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[3].u32 ) };
	// 830CBC70: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 830CBC74: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 830CBC78: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830CBC7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830CBC80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830CBC84: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830CBC88: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830CBC8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CBC90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830CBC90 size=580
    let mut pc: u32 = 0x830CBC90;
    'dispatch: loop {
        match pc {
            0x830CBC90 => {
    //   block [0x830CBC90..0x830CBED4)
	// 830CBC90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830CBC94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830CBC98: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830CBC9C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830CBCA0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830CBCA4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830CBCA8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 830CBCAC: 7F1EF840  cmplw cr6, r30, r31
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[31].u32, &mut ctx.xer);
	// 830CBCB0: 419A0208  beq cr6, 0x830cbeb8
	if ctx.cr[6].eq {
	pc = 0x830CBEB8; continue 'dispatch;
	}
	// 830CBCB4: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830CBCB8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830CBCBC: 419A0014  beq cr6, 0x830cbcd0
	if ctx.cr[6].eq {
	pc = 0x830CBCD0; continue 'dispatch;
	}
	// 830CBCC0: 811F0008  lwz r8, 8(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 830CBCC4: 7D4B4050  subf r10, r11, r8
	ctx.r[10].s64 = ctx.r[8].s64 - ctx.r[11].s64;
	// 830CBCC8: 7D472671  srawi. r7, r10, 4
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 4) - 1)) != 0);
	ctx.r[7].s64 = (ctx.r[10].s32 >> 4) as i64;
	ctx.cr[0].compare_i32(ctx.r[7].s32, 0, &mut ctx.xer);
	// 830CBCCC: 40820010  bne 0x830cbcdc
	if !ctx.cr[0].eq {
	pc = 0x830CBCDC; continue 'dispatch;
	}
	// 830CBCD0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830CBCD4: 4BFFFE6D  bl 0x830cbb40
	ctx.lr = 0x830CBCD8;
	sub_830CBB40(ctx, base);
	// 830CBCD8: 480001E0  b 0x830cbeb8
	pc = 0x830CBEB8; continue 'dispatch;
	// 830CBCDC: 815E0004  lwz r10, 4(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830CBCE0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 830CBCE4: 409A000C  bne cr6, 0x830cbcf0
	if !ctx.cr[6].eq {
	pc = 0x830CBCF0; continue 'dispatch;
	}
	// 830CBCE8: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 830CBCEC: 48000010  b 0x830cbcfc
	pc = 0x830CBCFC; continue 'dispatch;
	// 830CBCF0: 813E0008  lwz r9, 8(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 830CBCF4: 7D2A4850  subf r9, r10, r9
	ctx.r[9].s64 = ctx.r[9].s64 - ctx.r[10].s64;
	// 830CBCF8: 7D292670  srawi r9, r9, 4
	ctx.xer.ca = (ctx.r[9].s32 < 0) && ((ctx.r[9].u32 & ((1u32 << 4) - 1)) != 0);
	ctx.r[9].s64 = (ctx.r[9].s32 >> 4) as i64;
	// 830CBCFC: 7F074840  cmplw cr6, r7, r9
	ctx.cr[6].compare_u32(ctx.r[7].u32, ctx.r[9].u32, &mut ctx.xer);
	// 830CBD00: 4099003C  ble cr6, 0x830cbd3c
	if !ctx.cr[6].gt {
	pc = 0x830CBD3C; continue 'dispatch;
	}
	// 830CBD04: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 830CBD08: 409A0064  bne cr6, 0x830cbd6c
	if !ctx.cr[6].eq {
	pc = 0x830CBD6C; continue 'dispatch;
	}
	// 830CBD0C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830CBD10: 48000068  b 0x830cbd78
	pc = 0x830CBD78; continue 'dispatch;
	// 830CBD14: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830CBD18: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 830CBD1C: 812B0004  lwz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830CBD20: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 830CBD24: 812B0008  lwz r9, 8(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 830CBD28: 912A0008  stw r9, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 830CBD2C: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 830CBD30: 396B0010  addi r11, r11, 0x10
	ctx.r[11].s64 = ctx.r[11].s64 + 16;
	// 830CBD34: 912A000C  stw r9, 0xc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 830CBD38: 394A0010  addi r10, r10, 0x10
	ctx.r[10].s64 = ctx.r[10].s64 + 16;
	// 830CBD3C: 7F0B4040  cmplw cr6, r11, r8
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[8].u32, &mut ctx.xer);
	// 830CBD40: 409AFFD4  bne cr6, 0x830cbd14
	if !ctx.cr[6].eq {
	pc = 0x830CBD14; continue 'dispatch;
	}
	// 830CBD44: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830CBD48: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830CBD4C: 419A0010  beq cr6, 0x830cbd5c
	if ctx.cr[6].eq {
	pc = 0x830CBD5C; continue 'dispatch;
	}
	// 830CBD50: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 830CBD54: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 830CBD58: 7D6B2670  srawi r11, r11, 4
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 4) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 4) as i64;
	// 830CBD5C: 815E0004  lwz r10, 4(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830CBD60: 556B2036  slwi r11, r11, 4
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 830CBD64: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 830CBD68: 480000BC  b 0x830cbe24
	pc = 0x830CBE24; continue 'dispatch;
	// 830CBD6C: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 830CBD70: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 830CBD74: 7D6B2670  srawi r11, r11, 4
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 4) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 4) as i64;
	// 830CBD78: 7F075840  cmplw cr6, r7, r11
	ctx.cr[6].compare_u32(ctx.r[7].u32, ctx.r[11].u32, &mut ctx.xer);
	// 830CBD7C: 419900B0  bgt cr6, 0x830cbe2c
	if ctx.cr[6].gt {
	pc = 0x830CBE2C; continue 'dispatch;
	}
	// 830CBD80: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 830CBD84: 409A000C  bne cr6, 0x830cbd90
	if !ctx.cr[6].eq {
	pc = 0x830CBD90; continue 'dispatch;
	}
	// 830CBD88: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 830CBD8C: 48000010  b 0x830cbd9c
	pc = 0x830CBD9C; continue 'dispatch;
	// 830CBD90: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 830CBD94: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 830CBD98: 7D692670  srawi r9, r11, 4
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 4) - 1)) != 0);
	ctx.r[9].s64 = (ctx.r[11].s32 >> 4) as i64;
	// 830CBD9C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830CBDA0: 55282036  slwi r8, r9, 4
	ctx.r[8].u32 = ctx.r[9].u32.wrapping_shl(4);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 830CBDA4: 7D495378  mr r9, r10
	ctx.r[9].u64 = ctx.r[10].u64;
	// 830CBDA8: 7D485A14  add r10, r8, r11
	ctx.r[10].u64 = ctx.r[8].u64 + ctx.r[11].u64;
	// 830CBDAC: 4800002C  b 0x830cbdd8
	pc = 0x830CBDD8; continue 'dispatch;
	// 830CBDB0: 810B0000  lwz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830CBDB4: 91090000  stw r8, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 830CBDB8: 810B0004  lwz r8, 4(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830CBDBC: 91090004  stw r8, 4(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 830CBDC0: 810B0008  lwz r8, 8(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 830CBDC4: 91090008  stw r8, 8(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), ctx.r[8].u32 ) };
	// 830CBDC8: 810B000C  lwz r8, 0xc(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 830CBDCC: 396B0010  addi r11, r11, 0x10
	ctx.r[11].s64 = ctx.r[11].s64 + 16;
	// 830CBDD0: 9109000C  stw r8, 0xc(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 830CBDD4: 39290010  addi r9, r9, 0x10
	ctx.r[9].s64 = ctx.r[9].s64 + 16;
	// 830CBDD8: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 830CBDDC: 409AFFD4  bne cr6, 0x830cbdb0
	if !ctx.cr[6].eq {
	pc = 0x830CBDB0; continue 'dispatch;
	}
	// 830CBDE0: 813F0008  lwz r9, 8(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 830CBDE4: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 830CBDE8: 48000034  b 0x830cbe1c
	pc = 0x830CBE1C; continue 'dispatch;
	// 830CBDEC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830CBDF0: 419A0024  beq cr6, 0x830cbe14
	if ctx.cr[6].eq {
	pc = 0x830CBE14; continue 'dispatch;
	}
	// 830CBDF4: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 830CBDF8: 910B0000  stw r8, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 830CBDFC: 810A0004  lwz r8, 4(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 830CBE00: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 830CBE04: 810A0008  lwz r8, 8(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 830CBE08: 910B0008  stw r8, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[8].u32 ) };
	// 830CBE0C: 810A000C  lwz r8, 0xc(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 830CBE10: 910B000C  stw r8, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 830CBE14: 394A0010  addi r10, r10, 0x10
	ctx.r[10].s64 = ctx.r[10].s64 + 16;
	// 830CBE18: 396B0010  addi r11, r11, 0x10
	ctx.r[11].s64 = ctx.r[11].s64 + 16;
	// 830CBE1C: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 830CBE20: 409AFFCC  bne cr6, 0x830cbdec
	if !ctx.cr[6].eq {
	pc = 0x830CBDEC; continue 'dispatch;
	}
	// 830CBE24: 917E0008  stw r11, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 830CBE28: 48000090  b 0x830cbeb8
	pc = 0x830CBEB8; continue 'dispatch;
	// 830CBE2C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 830CBE30: 419A000C  beq cr6, 0x830cbe3c
	if ctx.cr[6].eq {
	pc = 0x830CBE3C; continue 'dispatch;
	}
	// 830CBE34: 7D435378  mr r3, r10
	ctx.r[3].u64 = ctx.r[10].u64;
	// 830CBE38: 4B1F4431  bl 0x822c0268
	ctx.lr = 0x830CBE3C;
	sub_822C0268(ctx, base);
	// 830CBE3C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830CBE40: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830CBE44: 409A000C  bne cr6, 0x830cbe50
	if !ctx.cr[6].eq {
	pc = 0x830CBE50; continue 'dispatch;
	}
	// 830CBE48: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830CBE4C: 48000010  b 0x830cbe5c
	pc = 0x830CBE5C; continue 'dispatch;
	// 830CBE50: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 830CBE54: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 830CBE58: 7D642670  srawi r4, r11, 4
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 4) - 1)) != 0);
	ctx.r[4].s64 = (ctx.r[11].s32 >> 4) as i64;
	// 830CBE5C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830CBE60: 4BFFFDA1  bl 0x830cbc00
	ctx.lr = 0x830CBE64;
	sub_830CBC00(ctx, base);
	// 830CBE64: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830CBE68: 41820050  beq 0x830cbeb8
	if ctx.cr[0].eq {
	pc = 0x830CBEB8; continue 'dispatch;
	}
	// 830CBE6C: 813F0008  lwz r9, 8(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 830CBE70: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830CBE74: 815E0004  lwz r10, 4(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830CBE78: 48000034  b 0x830cbeac
	pc = 0x830CBEAC; continue 'dispatch;
	// 830CBE7C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 830CBE80: 419A0024  beq cr6, 0x830cbea4
	if ctx.cr[6].eq {
	pc = 0x830CBEA4; continue 'dispatch;
	}
	// 830CBE84: 810B0000  lwz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830CBE88: 910A0000  stw r8, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 830CBE8C: 810B0004  lwz r8, 4(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830CBE90: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 830CBE94: 810B0008  lwz r8, 8(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 830CBE98: 910A0008  stw r8, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[8].u32 ) };
	// 830CBE9C: 810B000C  lwz r8, 0xc(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 830CBEA0: 910A000C  stw r8, 0xc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 830CBEA4: 396B0010  addi r11, r11, 0x10
	ctx.r[11].s64 = ctx.r[11].s64 + 16;
	// 830CBEA8: 394A0010  addi r10, r10, 0x10
	ctx.r[10].s64 = ctx.r[10].s64 + 16;
	// 830CBEAC: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 830CBEB0: 409AFFCC  bne cr6, 0x830cbe7c
	if !ctx.cr[6].eq {
	pc = 0x830CBE7C; continue 'dispatch;
	}
	// 830CBEB4: 915E0008  stw r10, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 830CBEB8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830CBEBC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830CBEC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830CBEC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830CBEC8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830CBECC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830CBED0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CBED8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830CBED8 size=180
    let mut pc: u32 = 0x830CBED8;
    'dispatch: loop {
        match pc {
            0x830CBED8 => {
    //   block [0x830CBED8..0x830CBF8C)
	// 830CBED8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830CBEDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830CBEE0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830CBEE4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830CBEE8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830CBEEC: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 830CBEF0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830CBEF4: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830CBEF8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830CBEFC: 409A000C  bne cr6, 0x830cbf08
	if !ctx.cr[6].eq {
	pc = 0x830CBF08; continue 'dispatch;
	}
	// 830CBF00: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830CBF04: 48000010  b 0x830cbf14
	pc = 0x830CBF14; continue 'dispatch;
	// 830CBF08: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 830CBF0C: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 830CBF10: 7D642670  srawi r4, r11, 4
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 4) - 1)) != 0);
	ctx.r[4].s64 = (ctx.r[11].s32 >> 4) as i64;
	// 830CBF14: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830CBF18: 4BFFFCE9  bl 0x830cbc00
	ctx.lr = 0x830CBF1C;
	sub_830CBC00(ctx, base);
	// 830CBF1C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830CBF20: 41820050  beq 0x830cbf70
	if ctx.cr[0].eq {
	pc = 0x830CBF70; continue 'dispatch;
	}
	// 830CBF24: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830CBF28: 813F0008  lwz r9, 8(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 830CBF2C: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830CBF30: 48000034  b 0x830cbf64
	pc = 0x830CBF64; continue 'dispatch;
	// 830CBF34: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830CBF38: 419A0024  beq cr6, 0x830cbf5c
	if ctx.cr[6].eq {
	pc = 0x830CBF5C; continue 'dispatch;
	}
	// 830CBF3C: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 830CBF40: 910B0000  stw r8, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 830CBF44: 810A0004  lwz r8, 4(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 830CBF48: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 830CBF4C: 810A0008  lwz r8, 8(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 830CBF50: 910B0008  stw r8, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[8].u32 ) };
	// 830CBF54: 810A000C  lwz r8, 0xc(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 830CBF58: 910B000C  stw r8, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 830CBF5C: 394A0010  addi r10, r10, 0x10
	ctx.r[10].s64 = ctx.r[10].s64 + 16;
	// 830CBF60: 396B0010  addi r11, r11, 0x10
	ctx.r[11].s64 = ctx.r[11].s64 + 16;
	// 830CBF64: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 830CBF68: 409AFFCC  bne cr6, 0x830cbf34
	if !ctx.cr[6].eq {
	pc = 0x830CBF34; continue 'dispatch;
	}
	// 830CBF6C: 917E0008  stw r11, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 830CBF70: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830CBF74: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830CBF78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830CBF7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830CBF80: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830CBF84: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830CBF88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CBF90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830CBF90 size=88
    let mut pc: u32 = 0x830CBF90;
    'dispatch: loop {
        match pc {
            0x830CBF90 => {
    //   block [0x830CBF90..0x830CBFE8)
	// 830CBF90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830CBF94: 480DC1D5  bl 0x831a8168
	ctx.lr = 0x830CBF98;
	sub_831A8130(ctx, base);
	// 830CBF98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830CBF9C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 830CBFA0: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 830CBFA4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830CBFA8: 7F03E840  cmplw cr6, r3, r29
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[29].u32, &mut ctx.xer);
	// 830CBFAC: 419A0030  beq cr6, 0x830cbfdc
	if ctx.cr[6].eq {
	pc = 0x830CBFDC; continue 'dispatch;
	}
	// 830CBFB0: 7F9F1850  subf r28, r31, r3
	ctx.r[28].s64 = ctx.r[3].s64 - ctx.r[31].s64;
	// 830CBFB4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830CBFB8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830CBFBC: 4BFFFCD5  bl 0x830cbc90
	ctx.lr = 0x830CBFC0;
	sub_830CBC90(ctx, base);
	// 830CBFC0: 7D7CFA14  add r11, r28, r31
	ctx.r[11].u64 = ctx.r[28].u64 + ctx.r[31].u64;
	// 830CBFC4: 3BDE0014  addi r30, r30, 0x14
	ctx.r[30].s64 = ctx.r[30].s64 + 20;
	// 830CBFC8: 7F1EE840  cmplw cr6, r30, r29
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[29].u32, &mut ctx.xer);
	// 830CBFCC: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 830CBFD0: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 830CBFD4: 3BFF0014  addi r31, r31, 0x14
	ctx.r[31].s64 = ctx.r[31].s64 + 20;
	// 830CBFD8: 409AFFDC  bne cr6, 0x830cbfb4
	if !ctx.cr[6].eq {
	pc = 0x830CBFB4; continue 'dispatch;
	}
	// 830CBFDC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830CBFE0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 830CBFE4: 480DC1D4  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CBFE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830CBFE8 size=84
    let mut pc: u32 = 0x830CBFE8;
    'dispatch: loop {
        match pc {
            0x830CBFE8 => {
    //   block [0x830CBFE8..0x830CC03C)
	// 830CBFE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830CBFEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830CBFF0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830CBFF4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830CBFF8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830CBFFC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 830CC000: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830CC004: 7F03F040  cmplw cr6, r3, r30
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[30].u32, &mut ctx.xer);
	// 830CC008: 419A001C  beq cr6, 0x830cc024
	if ctx.cr[6].eq {
	pc = 0x830CC024; continue 'dispatch;
	}
	// 830CC00C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830CC010: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830CC014: 4BFFFB85  bl 0x830cbb98
	ctx.lr = 0x830CC018;
	sub_830CBB98(ctx, base);
	// 830CC018: 3BFF0014  addi r31, r31, 0x14
	ctx.r[31].s64 = ctx.r[31].s64 + 20;
	// 830CC01C: 7F1FF040  cmplw cr6, r31, r30
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[30].u32, &mut ctx.xer);
	// 830CC020: 409AFFEC  bne cr6, 0x830cc00c
	if !ctx.cr[6].eq {
	pc = 0x830CC00C; continue 'dispatch;
	}
	// 830CC024: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830CC028: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830CC02C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830CC030: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830CC034: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830CC038: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CC040(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830CC040 size=88
    let mut pc: u32 = 0x830CC040;
    'dispatch: loop {
        match pc {
            0x830CC040 => {
    //   block [0x830CC040..0x830CC098)
	// 830CC040: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830CC044: 480DC125  bl 0x831a8168
	ctx.lr = 0x830CC048;
	sub_831A8130(ctx, base);
	// 830CC048: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830CC04C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 830CC050: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 830CC054: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 830CC058: 7F1D2040  cmplw cr6, r29, r4
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[4].u32, &mut ctx.xer);
	// 830CC05C: 419A0030  beq cr6, 0x830cc08c
	if ctx.cr[6].eq {
	pc = 0x830CC08C; continue 'dispatch;
	}
	// 830CC060: 7F9F2050  subf r28, r31, r4
	ctx.r[28].s64 = ctx.r[4].s64 - ctx.r[31].s64;
	// 830CC064: 3BDEFFEC  addi r30, r30, -0x14
	ctx.r[30].s64 = ctx.r[30].s64 + -20;
	// 830CC068: 3BFFFFEC  addi r31, r31, -0x14
	ctx.r[31].s64 = ctx.r[31].s64 + -20;
	// 830CC06C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830CC070: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830CC074: 4BFFFC1D  bl 0x830cbc90
	ctx.lr = 0x830CC078;
	sub_830CBC90(ctx, base);
	// 830CC078: 7D7CFA14  add r11, r28, r31
	ctx.r[11].u64 = ctx.r[28].u64 + ctx.r[31].u64;
	// 830CC07C: 7F1EE840  cmplw cr6, r30, r29
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[29].u32, &mut ctx.xer);
	// 830CC080: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 830CC084: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 830CC088: 409AFFDC  bne cr6, 0x830cc064
	if !ctx.cr[6].eq {
	pc = 0x830CC064; continue 'dispatch;
	}
	// 830CC08C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830CC090: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 830CC094: 480DC124  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CC098(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830CC098 size=92
    let mut pc: u32 = 0x830CC098;
    'dispatch: loop {
        match pc {
            0x830CC098 => {
    //   block [0x830CC098..0x830CC0F4)
	// 830CC098: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830CC09C: 480DC0D1  bl 0x831a816c
	ctx.lr = 0x830CC0A0;
	sub_831A8130(ctx, base);
	// 830CC0A0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830CC0A4: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 830CC0A8: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 830CC0AC: 7F03E840  cmplw cr6, r3, r29
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[29].u32, &mut ctx.xer);
	// 830CC0B0: 419A0038  beq cr6, 0x830cc0e8
	if ctx.cr[6].eq {
	pc = 0x830CC0E8; continue 'dispatch;
	}
	// 830CC0B4: 7FDF1850  subf r30, r31, r3
	ctx.r[30].s64 = ctx.r[3].s64 - ctx.r[31].s64;
	// 830CC0B8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 830CC0BC: 419A001C  beq cr6, 0x830cc0d8
	if ctx.cr[6].eq {
	pc = 0x830CC0D8; continue 'dispatch;
	}
	// 830CC0C0: 7C9EFA14  add r4, r30, r31
	ctx.r[4].u64 = ctx.r[30].u64 + ctx.r[31].u64;
	// 830CC0C4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830CC0C8: 4BFFFE11  bl 0x830cbed8
	ctx.lr = 0x830CC0CC;
	sub_830CBED8(ctx, base);
	// 830CC0CC: 397F0010  addi r11, r31, 0x10
	ctx.r[11].s64 = ctx.r[31].s64 + 16;
	// 830CC0D0: 7D7E582E  lwzx r11, r30, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 830CC0D4: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 830CC0D8: 3BFF0014  addi r31, r31, 0x14
	ctx.r[31].s64 = ctx.r[31].s64 + 20;
	// 830CC0DC: 7D7EFA14  add r11, r30, r31
	ctx.r[11].u64 = ctx.r[30].u64 + ctx.r[31].u64;
	// 830CC0E0: 7F0BE840  cmplw cr6, r11, r29
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[29].u32, &mut ctx.xer);
	// 830CC0E4: 409AFFD4  bne cr6, 0x830cc0b8
	if !ctx.cr[6].eq {
	pc = 0x830CC0B8; continue 'dispatch;
	}
	// 830CC0E8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830CC0EC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830CC0F0: 480DC0CC  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CC0F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830CC0F8 size=88
    let mut pc: u32 = 0x830CC0F8;
    'dispatch: loop {
        match pc {
            0x830CC0F8 => {
    //   block [0x830CC0F8..0x830CC150)
	// 830CC0F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830CC0FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830CC100: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830CC104: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830CC108: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830CC10C: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830CC110: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 830CC114: 419A0018  beq cr6, 0x830cc12c
	if ctx.cr[6].eq {
	pc = 0x830CC12C; continue 'dispatch;
	}
	// 830CC118: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 830CC11C: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 830CC120: 4BFFFEC9  bl 0x830cbfe8
	ctx.lr = 0x830CC124;
	sub_830CBFE8(ctx, base);
	// 830CC124: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830CC128: 4B1F4141  bl 0x822c0268
	ctx.lr = 0x830CC12C;
	sub_822C0268(ctx, base);
	// 830CC12C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830CC130: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 830CC134: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 830CC138: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 830CC13C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830CC140: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830CC144: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830CC148: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830CC14C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CC150(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830CC150 size=92
    let mut pc: u32 = 0x830CC150;
    'dispatch: loop {
        match pc {
            0x830CC150 => {
    //   block [0x830CC150..0x830CC1AC)
	// 830CC150: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830CC154: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830CC158: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830CC15C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830CC160: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830CC164: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830CC168: 80BF0004  lwz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830CC16C: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 830CC170: 7F052040  cmplw cr6, r5, r4
	ctx.cr[6].compare_u32(ctx.r[5].u32, ctx.r[4].u32, &mut ctx.xer);
	// 830CC174: 419A0020  beq cr6, 0x830cc194
	if ctx.cr[6].eq {
	pc = 0x830CC194; continue 'dispatch;
	}
	// 830CC178: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 830CC17C: 4BFFFE15  bl 0x830cbf90
	ctx.lr = 0x830CC180;
	sub_830CBF90(ctx, base);
	// 830CC180: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 830CC184: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 830CC188: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830CC18C: 4BFFFE5D  bl 0x830cbfe8
	ctx.lr = 0x830CC190;
	sub_830CBFE8(ctx, base);
	// 830CC190: 93DF0008  stw r30, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 830CC194: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830CC198: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830CC19C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830CC1A0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830CC1A4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830CC1A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CC1B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830CC1B0 size=96
    let mut pc: u32 = 0x830CC1B0;
    'dispatch: loop {
        match pc {
            0x830CC1B0 => {
    //   block [0x830CC1B0..0x830CC210)
	// 830CC1B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830CC1B4: 480DBFB1  bl 0x831a8164
	ctx.lr = 0x830CC1B8;
	sub_831A8130(ctx, base);
	// 830CC1B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830CC1BC: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 830CC1C0: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 830CC1C4: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 830CC1C8: 7FDDF378  mr r29, r30
	ctx.r[29].u64 = ctx.r[30].u64;
	// 830CC1CC: 7F9FE378  mr r31, r28
	ctx.r[31].u64 = ctx.r[28].u64;
	// 830CC1D0: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 830CC1D4: 419A002C  beq cr6, 0x830cc200
	if ctx.cr[6].eq {
	pc = 0x830CC200; continue 'dispatch;
	}
	// 830CC1D8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 830CC1DC: 419A0018  beq cr6, 0x830cc1f4
	if ctx.cr[6].eq {
	pc = 0x830CC1F4; continue 'dispatch;
	}
	// 830CC1E0: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 830CC1E4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830CC1E8: 4BFFFCF1  bl 0x830cbed8
	ctx.lr = 0x830CC1EC;
	sub_830CBED8(ctx, base);
	// 830CC1EC: 817B0010  lwz r11, 0x10(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(16 as u32) ) } as u64;
	// 830CC1F0: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 830CC1F4: 37BDFFFF  addic. r29, r29, -1
	ctx.xer.ca = (ctx.r[29].u32 > (!(-1 as u32)));
	ctx.r[29].s64 = ctx.r[29].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 830CC1F8: 3BFF0014  addi r31, r31, 0x14
	ctx.r[31].s64 = ctx.r[31].s64 + 20;
	// 830CC1FC: 4082FFDC  bne 0x830cc1d8
	if !ctx.cr[0].eq {
	pc = 0x830CC1D8; continue 'dispatch;
	}
	// 830CC200: 1D7E0014  mulli r11, r30, 0x14
	ctx.r[11].s64 = ctx.r[30].s64 * 20;
	// 830CC204: 7C6BE214  add r3, r11, r28
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[28].u64;
	// 830CC208: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 830CC20C: 480DBFA8  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CC210(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830CC210 size=64
    let mut pc: u32 = 0x830CC210;
    'dispatch: loop {
        match pc {
            0x830CC210 => {
    //   block [0x830CC210..0x830CC250)
	// 830CC210: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830CC214: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830CC218: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830CC21C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830CC220: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830CC224: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 830CC228: 3BEBC024  addi r31, r11, -0x3fdc
	ctx.r[31].s64 = ctx.r[11].s64 + -16348;
	// 830CC22C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830CC230: 4BFFF879  bl 0x830cbaa8
	ctx.lr = 0x830CC234;
	sub_830CBAA8(ctx, base);
	// 830CC234: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830CC238: 4BFFFF19  bl 0x830cc150
	ctx.lr = 0x830CC23C;
	sub_830CC150(ctx, base);
	// 830CC23C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830CC240: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830CC244: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830CC248: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830CC24C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CC250(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830CC250 size=688
    let mut pc: u32 = 0x830CC250;
    'dispatch: loop {
        match pc {
            0x830CC250 => {
    //   block [0x830CC250..0x830CC500)
	// 830CC250: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830CC254: 480DBF09  bl 0x831a815c
	ctx.lr = 0x830CC258;
	sub_831A8130(ctx, base);
	// 830CC258: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830CC25C: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 830CC260: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830CC264: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 830CC268: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830CC26C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830CC270: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 830CC274: 4BFFFC65  bl 0x830cbed8
	ctx.lr = 0x830CC278;
	sub_830CBED8(ctx, base);
	// 830CC278: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830CC27C: 3B400014  li r26, 0x14
	ctx.r[26].s64 = 20;
	// 830CC280: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830CC284: 833E0010  lwz r25, 0x10(r30)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 830CC288: 93210060  stw r25, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[25].u32 ) };
	// 830CC28C: 409A000C  bne cr6, 0x830cc298
	if !ctx.cr[6].eq {
	pc = 0x830CC298; continue 'dispatch;
	}
	// 830CC290: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 830CC294: 48000010  b 0x830cc2a4
	pc = 0x830CC2A4; continue 'dispatch;
	// 830CC298: 815F000C  lwz r10, 0xc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 830CC29C: 7D4B5050  subf r10, r11, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 830CC2A0: 7D0AD3D6  divw r8, r10, r26
	ctx.r[8].s32 = ctx.r[10].s32 / ctx.r[26].s32;
	// 830CC2A4: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 830CC2A8: 419A0240  beq cr6, 0x830cc4e8
	if ctx.cr[6].eq {
	pc = 0x830CC4E8; continue 'dispatch;
	}
	// 830CC2AC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830CC2B0: 409A000C  bne cr6, 0x830cc2bc
	if !ctx.cr[6].eq {
	pc = 0x830CC2BC; continue 'dispatch;
	}
	// 830CC2B4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 830CC2B8: 48000010  b 0x830cc2c8
	pc = 0x830CC2C8; continue 'dispatch;
	// 830CC2BC: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 830CC2C0: 7D4B5050  subf r10, r11, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 830CC2C4: 7D4AD3D6  divw r10, r10, r26
	ctx.r[10].s32 = ctx.r[10].s32 / ctx.r[26].s32;
	// 830CC2C8: 3D200CCC  lis r9, 0xccc
	ctx.r[9].s64 = 214695936;
	// 830CC2CC: 6129CCCC  ori r9, r9, 0xcccc
	ctx.r[9].u64 = ctx.r[9].u64 | 52428;
	// 830CC2D0: 7D4A4850  subf r10, r10, r9
	ctx.r[10].s64 = ctx.r[9].s64 - ctx.r[10].s64;
	// 830CC2D4: 7F0AD840  cmplw cr6, r10, r27
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[27].u32, &mut ctx.xer);
	// 830CC2D8: 4098000C  bge cr6, 0x830cc2e4
	if !ctx.cr[6].lt {
	pc = 0x830CC2E4; continue 'dispatch;
	}
	// 830CC2DC: 4BFF8E5D  bl 0x830c5138
	ctx.lr = 0x830CC2E0;
	sub_830C5138(ctx, base);
	// 830CC2E0: 48000208  b 0x830cc4e8
	pc = 0x830CC4E8; continue 'dispatch;
	// 830CC2E4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830CC2E8: 409A000C  bne cr6, 0x830cc2f4
	if !ctx.cr[6].eq {
	pc = 0x830CC2F4; continue 'dispatch;
	}
	// 830CC2EC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 830CC2F0: 48000010  b 0x830cc300
	pc = 0x830CC300; continue 'dispatch;
	// 830CC2F4: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 830CC2F8: 7D4B5050  subf r10, r11, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 830CC2FC: 7D4AD3D6  divw r10, r10, r26
	ctx.r[10].s32 = ctx.r[10].s32 / ctx.r[26].s32;
	// 830CC300: 7D4ADA14  add r10, r10, r27
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[27].u64;
	// 830CC304: 7F085040  cmplw cr6, r8, r10
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[10].u32, &mut ctx.xer);
	// 830CC308: 4098010C  bge cr6, 0x830cc414
	if !ctx.cr[6].lt {
	pc = 0x830CC414; continue 'dispatch;
	}
	// 830CC30C: 550AF87E  srwi r10, r8, 1
	ctx.r[10].u32 = ctx.r[8].u32.wrapping_shr(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 830CC310: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 830CC314: 7D2A4850  subf r9, r10, r9
	ctx.r[9].s64 = ctx.r[9].s64 - ctx.r[10].s64;
	// 830CC318: 7F094040  cmplw cr6, r9, r8
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[8].u32, &mut ctx.xer);
	// 830CC31C: 41980008  blt cr6, 0x830cc324
	if ctx.cr[6].lt {
	pc = 0x830CC324; continue 'dispatch;
	}
	// 830CC320: 7F8A4214  add r28, r10, r8
	ctx.r[28].u64 = ctx.r[10].u64 + ctx.r[8].u64;
	// 830CC324: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830CC328: 409A000C  bne cr6, 0x830cc334
	if !ctx.cr[6].eq {
	pc = 0x830CC334; continue 'dispatch;
	}
	// 830CC32C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 830CC330: 48000010  b 0x830cc340
	pc = 0x830CC340; continue 'dispatch;
	// 830CC334: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 830CC338: 7D4B5050  subf r10, r11, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 830CC33C: 7D4AD3D6  divw r10, r10, r26
	ctx.r[10].s32 = ctx.r[10].s32 / ctx.r[26].s32;
	// 830CC340: 7D4ADA14  add r10, r10, r27
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[27].u64;
	// 830CC344: 7F1C5040  cmplw cr6, r28, r10
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[10].u32, &mut ctx.xer);
	// 830CC348: 4098001C  bge cr6, 0x830cc364
	if !ctx.cr[6].lt {
	pc = 0x830CC364; continue 'dispatch;
	}
	// 830CC34C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830CC350: 419A0010  beq cr6, 0x830cc360
	if ctx.cr[6].eq {
	pc = 0x830CC360; continue 'dispatch;
	}
	// 830CC354: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 830CC358: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 830CC35C: 7D6BD3D6  divw r11, r11, r26
	ctx.r[11].s32 = ctx.r[11].s32 / ctx.r[26].s32;
	// 830CC360: 7F8BDA14  add r28, r11, r27
	ctx.r[28].u64 = ctx.r[11].u64 + ctx.r[27].u64;
	// 830CC364: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830CC368: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 830CC36C: 4BFFF6C5  bl 0x830cba30
	ctx.lr = 0x830CC370;
	sub_830CBA30(ctx, base);
	// 830CC370: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830CC374: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830CC378: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 830CC37C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 830CC380: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 830CC384: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 830CC388: 4BFFFD11  bl 0x830cc098
	ctx.lr = 0x830CC38C;
	sub_830CC098(ctx, base);
	// 830CC38C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 830CC390: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 830CC394: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 830CC398: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830CC39C: 4BFFFE15  bl 0x830cc1b0
	ctx.lr = 0x830CC3A0;
	sub_830CC1B0(ctx, base);
	// 830CC3A0: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 830CC3A4: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 830CC3A8: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 830CC3AC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830CC3B0: 4BFFFCE9  bl 0x830cc098
	ctx.lr = 0x830CC3B4;
	sub_830CC098(ctx, base);
	// 830CC3B4: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830CC3B8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 830CC3BC: 409A000C  bne cr6, 0x830cc3c8
	if !ctx.cr[6].eq {
	pc = 0x830CC3C8; continue 'dispatch;
	}
	// 830CC3C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830CC3C4: 48000010  b 0x830cc3d4
	pc = 0x830CC3D4; continue 'dispatch;
	// 830CC3C8: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 830CC3CC: 7D635850  subf r11, r3, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[3].s64;
	// 830CC3D0: 7D6BD3D6  divw r11, r11, r26
	ctx.r[11].s32 = ctx.r[11].s32 / ctx.r[26].s32;
	// 830CC3D4: 7FABDA14  add r29, r11, r27
	ctx.r[29].u64 = ctx.r[11].u64 + ctx.r[27].u64;
	// 830CC3D8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 830CC3DC: 419A0018  beq cr6, 0x830cc3f4
	if ctx.cr[6].eq {
	pc = 0x830CC3F4; continue 'dispatch;
	}
	// 830CC3E0: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 830CC3E4: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 830CC3E8: 4BFFFC01  bl 0x830cbfe8
	ctx.lr = 0x830CC3EC;
	sub_830CBFE8(ctx, base);
	// 830CC3EC: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830CC3F0: 4B1F3E79  bl 0x822c0268
	ctx.lr = 0x830CC3F4;
	sub_822C0268(ctx, base);
	// 830CC3F4: 1D7C0014  mulli r11, r28, 0x14
	ctx.r[11].s64 = ctx.r[28].s64 * 20;
	// 830CC3F8: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 830CC3FC: 1D5D0014  mulli r10, r29, 0x14
	ctx.r[10].s64 = ctx.r[29].s64 * 20;
	// 830CC400: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 830CC404: 7D4AF214  add r10, r10, r30
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[30].u64;
	// 830CC408: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 830CC40C: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 830CC410: 480000D8  b 0x830cc4e8
	pc = 0x830CC4E8; continue 'dispatch;
	// 830CC414: 83DF0008  lwz r30, 8(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 830CC418: 1F9B0014  mulli r28, r27, 0x14
	ctx.r[28].s64 = ctx.r[27].s64 * 20;
	// 830CC41C: 7D7DF050  subf r11, r29, r30
	ctx.r[11].s64 = ctx.r[30].s64 - ctx.r[29].s64;
	// 830CC420: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830CC424: 7D6BD3D6  divw r11, r11, r26
	ctx.r[11].s32 = ctx.r[11].s32 / ctx.r[26].s32;
	// 830CC428: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 830CC42C: 7F0BD840  cmplw cr6, r11, r27
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[27].u32, &mut ctx.xer);
	// 830CC430: 40980068  bge cr6, 0x830cc498
	if !ctx.cr[6].lt {
	pc = 0x830CC498; continue 'dispatch;
	}
	// 830CC434: 7CBCEA14  add r5, r28, r29
	ctx.r[5].u64 = ctx.r[28].u64 + ctx.r[29].u64;
	// 830CC438: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830CC43C: 4BFFFC5D  bl 0x830cc098
	ctx.lr = 0x830CC440;
	sub_830CC098(ctx, base);
	// 830CC440: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 830CC444: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 830CC448: 7D7D2050  subf r11, r29, r4
	ctx.r[11].s64 = ctx.r[4].s64 - ctx.r[29].s64;
	// 830CC44C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830CC450: 7D6BD3D6  divw r11, r11, r26
	ctx.r[11].s32 = ctx.r[11].s32 / ctx.r[26].s32;
	// 830CC454: 7CABD850  subf r5, r11, r27
	ctx.r[5].s64 = ctx.r[27].s64 - ctx.r[11].s64;
	// 830CC458: 4BFFFD59  bl 0x830cc1b0
	ctx.lr = 0x830CC45C;
	sub_830CC1B0(ctx, base);
	// 830CC45C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 830CC460: 7FBEEB78  mr r30, r29
	ctx.r[30].u64 = ctx.r[29].u64;
	// 830CC464: 7D7C5A14  add r11, r28, r11
	ctx.r[11].u64 = ctx.r[28].u64 + ctx.r[11].u64;
	// 830CC468: 7F9C5850  subf r28, r28, r11
	ctx.r[28].s64 = ctx.r[11].s64 - ctx.r[28].s64;
	// 830CC46C: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 830CC470: 7F1DE040  cmplw cr6, r29, r28
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[28].u32, &mut ctx.xer);
	// 830CC474: 419A0074  beq cr6, 0x830cc4e8
	if ctx.cr[6].eq {
	pc = 0x830CC4E8; continue 'dispatch;
	}
	// 830CC478: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 830CC47C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830CC480: 4BFFF811  bl 0x830cbc90
	ctx.lr = 0x830CC484;
	sub_830CBC90(ctx, base);
	// 830CC484: 933E0010  stw r25, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[25].u32 ) };
	// 830CC488: 3BDE0014  addi r30, r30, 0x14
	ctx.r[30].s64 = ctx.r[30].s64 + 20;
	// 830CC48C: 7F1EE040  cmplw cr6, r30, r28
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[28].u32, &mut ctx.xer);
	// 830CC490: 409AFFE8  bne cr6, 0x830cc478
	if !ctx.cr[6].eq {
	pc = 0x830CC478; continue 'dispatch;
	}
	// 830CC494: 48000054  b 0x830cc4e8
	pc = 0x830CC4E8; continue 'dispatch;
	// 830CC498: 7F7CF050  subf r27, r28, r30
	ctx.r[27].s64 = ctx.r[30].s64 - ctx.r[28].s64;
	// 830CC49C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 830CC4A0: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 830CC4A4: 4BFFFBF5  bl 0x830cc098
	ctx.lr = 0x830CC4A8;
	sub_830CC098(ctx, base);
	// 830CC4A8: 907F0008  stw r3, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[3].u32 ) };
	// 830CC4AC: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 830CC4B0: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 830CC4B4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830CC4B8: 4BFFFB89  bl 0x830cc040
	ctx.lr = 0x830CC4BC;
	sub_830CC040(ctx, base);
	// 830CC4BC: 7FDCEA14  add r30, r28, r29
	ctx.r[30].u64 = ctx.r[28].u64 + ctx.r[29].u64;
	// 830CC4C0: 7FBFEB78  mr r31, r29
	ctx.r[31].u64 = ctx.r[29].u64;
	// 830CC4C4: 7F1DF040  cmplw cr6, r29, r30
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[30].u32, &mut ctx.xer);
	// 830CC4C8: 419A0020  beq cr6, 0x830cc4e8
	if ctx.cr[6].eq {
	pc = 0x830CC4E8; continue 'dispatch;
	}
	// 830CC4CC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 830CC4D0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830CC4D4: 4BFFF7BD  bl 0x830cbc90
	ctx.lr = 0x830CC4D8;
	sub_830CBC90(ctx, base);
	// 830CC4D8: 933F0010  stw r25, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[25].u32 ) };
	// 830CC4DC: 3BFF0014  addi r31, r31, 0x14
	ctx.r[31].s64 = ctx.r[31].s64 + 20;
	// 830CC4E0: 7F1FF040  cmplw cr6, r31, r30
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[30].u32, &mut ctx.xer);
	// 830CC4E4: 409AFFE8  bne cr6, 0x830cc4cc
	if !ctx.cr[6].eq {
	pc = 0x830CC4CC; continue 'dispatch;
	}
	// 830CC4E8: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 830CC4EC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 830CC4F0: 419A0008  beq cr6, 0x830cc4f8
	if ctx.cr[6].eq {
	pc = 0x830CC4F8; continue 'dispatch;
	}
	// 830CC4F4: 4B1F3D75  bl 0x822c0268
	ctx.lr = 0x830CC4F8;
	sub_822C0268(ctx, base);
	// 830CC4F8: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 830CC4FC: 480DBCB0  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CC500(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830CC500 size=112
    let mut pc: u32 = 0x830CC500;
    'dispatch: loop {
        match pc {
            0x830CC500 => {
    //   block [0x830CC500..0x830CC570)
	// 830CC500: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830CC504: 480DBC69  bl 0x831a816c
	ctx.lr = 0x830CC508;
	sub_831A8130(ctx, base);
	// 830CC508: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830CC50C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 830CC510: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 830CC514: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 830CC518: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830CC51C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830CC520: 419A0018  beq cr6, 0x830cc538
	if ctx.cr[6].eq {
	pc = 0x830CC538; continue 'dispatch;
	}
	// 830CC524: 813F0008  lwz r9, 8(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 830CC528: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 830CC52C: 7D2B4850  subf r9, r11, r9
	ctx.r[9].s64 = ctx.r[9].s64 - ctx.r[11].s64;
	// 830CC530: 7D2953D7  divw. r9, r9, r10
	ctx.r[9].s32 = ctx.r[9].s32 / ctx.r[10].s32;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 830CC534: 4082000C  bne 0x830cc540
	if !ctx.cr[0].eq {
	pc = 0x830CC540; continue 'dispatch;
	}
	// 830CC538: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 830CC53C: 4800000C  b 0x830cc548
	pc = 0x830CC548; continue 'dispatch;
	// 830CC540: 7D6B2050  subf r11, r11, r4
	ctx.r[11].s64 = ctx.r[4].s64 - ctx.r[11].s64;
	// 830CC544: 7FCB53D6  divw r30, r11, r10
	ctx.r[30].s32 = ctx.r[11].s32 / ctx.r[10].s32;
	// 830CC548: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 830CC54C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830CC550: 4BFFFD01  bl 0x830cc250
	ctx.lr = 0x830CC554;
	sub_830CC250(ctx, base);
	// 830CC554: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830CC558: 1D7E0014  mulli r11, r30, 0x14
	ctx.r[11].s64 = ctx.r[30].s64 * 20;
	// 830CC55C: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 830CC560: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830CC564: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830CC568: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830CC56C: 480DBC50  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CC570(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830CC570 size=148
    let mut pc: u32 = 0x830CC570;
    'dispatch: loop {
        match pc {
            0x830CC570 => {
    //   block [0x830CC570..0x830CC604)
	// 830CC570: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830CC574: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830CC578: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830CC57C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830CC580: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830CC584: 7C862378  mr r6, r4
	ctx.r[6].u64 = ctx.r[4].u64;
	// 830CC588: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 830CC58C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830CC590: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830CC594: 409A000C  bne cr6, 0x830cc5a0
	if !ctx.cr[6].eq {
	pc = 0x830CC5A0; continue 'dispatch;
	}
	// 830CC598: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 830CC59C: 48000010  b 0x830cc5ac
	pc = 0x830CC5AC; continue 'dispatch;
	// 830CC5A0: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 830CC5A4: 7D4B5050  subf r10, r11, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 830CC5A8: 7D4A4BD6  divw r10, r10, r9
	ctx.r[10].s32 = ctx.r[10].s32 / ctx.r[9].s32;
	// 830CC5AC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830CC5B0: 419A0030  beq cr6, 0x830cc5e0
	if ctx.cr[6].eq {
	pc = 0x830CC5E0; continue 'dispatch;
	}
	// 830CC5B4: 811F000C  lwz r8, 0xc(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 830CC5B8: 7D6B4050  subf r11, r11, r8
	ctx.r[11].s64 = ctx.r[8].s64 - ctx.r[11].s64;
	// 830CC5BC: 7D6B4BD6  divw r11, r11, r9
	ctx.r[11].s32 = ctx.r[11].s32 / ctx.r[9].s32;
	// 830CC5C0: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 830CC5C4: 4098001C  bge cr6, 0x830cc5e0
	if !ctx.cr[6].lt {
	pc = 0x830CC5E0; continue 'dispatch;
	}
	// 830CC5C8: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 830CC5CC: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 830CC5D0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830CC5D4: 4BFFFBDD  bl 0x830cc1b0
	ctx.lr = 0x830CC5D8;
	sub_830CC1B0(ctx, base);
	// 830CC5D8: 907F0008  stw r3, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[3].u32 ) };
	// 830CC5DC: 48000014  b 0x830cc5f0
	pc = 0x830CC5F0; continue 'dispatch;
	// 830CC5E0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 830CC5E4: 80BF0008  lwz r5, 8(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 830CC5E8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830CC5EC: 4BFFFF15  bl 0x830cc500
	ctx.lr = 0x830CC5F0;
	sub_830CC500(ctx, base);
	// 830CC5F0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830CC5F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830CC5F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830CC5FC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830CC600: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CC608(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830CC608 size=16
    let mut pc: u32 = 0x830CC608;
    'dispatch: loop {
        match pc {
            0x830CC608 => {
    //   block [0x830CC608..0x830CC618)
	// 830CC608: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830CC60C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 830CC610: 386BC024  addi r3, r11, -0x3fdc
	ctx.r[3].s64 = ctx.r[11].s64 + -16348;
	// 830CC614: 4BFFFF5C  b 0x830cc570
	sub_830CC570(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CC618(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830CC618 size=20
    let mut pc: u32 = 0x830CC618;
    'dispatch: loop {
        match pc {
            0x830CC618 => {
    //   block [0x830CC618..0x830CC62C)
	// 830CC618: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 830CC61C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830CC620: 409A000C  bne cr6, 0x830cc62c
	if !ctx.cr[6].eq {
		sub_830CC62C(ctx, base);
		return;
	}
	// 830CC624: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 830CC628: 48000010  b 0x830cc638
	sub_830CC62C(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CC62C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x830CC62C size=144
    let mut pc: u32 = 0x830CC62C;
    'dispatch: loop {
        match pc {
            0x830CC62C => {
    //   block [0x830CC62C..0x830CC6BC)
	// 830CC62C: 81430008  lwz r10, 8(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 830CC630: 7D4B5050  subf r10, r11, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 830CC634: 7D492670  srawi r9, r10, 4
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 4) - 1)) != 0);
	ctx.r[9].s64 = (ctx.r[10].s32 >> 4) as i64;
	// 830CC638: 7F092800  cmpw cr6, r9, r5
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[5].s32, &mut ctx.xer);
	// 830CC63C: 40990008  ble cr6, 0x830cc644
	if !ctx.cr[6].gt {
	pc = 0x830CC644; continue 'dispatch;
	}
	// 830CC640: 7CA92B78  mr r9, r5
	ctx.r[9].u64 = ctx.r[5].u64;
	// 830CC644: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 830CC648: 40990054  ble cr6, 0x830cc69c
	if !ctx.cr[6].gt {
	pc = 0x830CC69C; continue 'dispatch;
	}
	// 830CC64C: 3D008200  lis r8, -0x7e00
	ctx.r[8].s64 = -2113929216;
	// 830CC650: 3CE08200  lis r7, -0x7e00
	ctx.r[7].s64 = -2113929216;
	// 830CC654: 7D2A4B78  mr r10, r9
	ctx.r[10].u64 = ctx.r[9].u64;
	// 830CC658: C00808A4  lfs f0, 0x8a4(r8)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 830CC65C: C1A708A8  lfs f13, 0x8a8(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(2216 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 830CC660: C18B0000  lfs f12, 0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 830CC664: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 830CC668: D1840000  stfs f12, 0(r4)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 830CC66C: C18B0004  lfs f12, 4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 830CC670: D1840004  stfs f12, 4(r4)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 830CC674: C18B0008  lfs f12, 8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 830CC678: D1840008  stfs f12, 8(r4)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 830CC67C: 810B000C  lwz r8, 0xc(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 830CC680: D1A4000C  stfs f13, 0xc(r4)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 830CC684: D0040014  stfs f0, 0x14(r4)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 830CC688: 91040010  stw r8, 0x10(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(16 as u32), ctx.r[8].u32 ) };
	// 830CC68C: D0040018  stfs f0, 0x18(r4)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 830CC690: 3884001C  addi r4, r4, 0x1c
	ctx.r[4].s64 = ctx.r[4].s64 + 28;
	// 830CC694: 396B0010  addi r11, r11, 0x10
	ctx.r[11].s64 = ctx.r[11].s64 + 16;
	// 830CC698: 4082FFC8  bne 0x830cc660
	if !ctx.cr[0].eq {
	pc = 0x830CC660; continue 'dispatch;
	}
	// 830CC69C: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 830CC6A0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830CC6A4: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 830CC6A8: 419A0028  beq cr6, 0x830cc6d0
	if ctx.cr[6].eq {
		sub_830CC6D0(ctx, base);
		return;
	}
	// 830CC6AC: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 830CC6B0: 419A0014  beq cr6, 0x830cc6c4
	if ctx.cr[6].eq {
		sub_830CC6C4(ctx, base);
		return;
	}
	// 830CC6B4: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 830CC6B8: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CC6BC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830CC6BC size=8
    let mut pc: u32 = 0x830CC6BC;
    'dispatch: loop {
        match pc {
            0x830CC6BC => {
    //   block [0x830CC6BC..0x830CC6C4)
	// 830CC6BC: 3869FFFF  addi r3, r9, -1
	ctx.r[3].s64 = ctx.r[9].s64 + -1;
	// 830CC6C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CC6C4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x830CC6C4 size=12
    let mut pc: u32 = 0x830CC6C4;
    'dispatch: loop {
        match pc {
            0x830CC6C4 => {
    //   block [0x830CC6C4..0x830CC6D0)
	// 830CC6C4: 7D2B0E70  srawi r11, r9, 1
	ctx.xer.ca = (ctx.r[9].s32 < 0) && ((ctx.r[9].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[9].s32 >> 1) as i64;
	// 830CC6C8: 7C6B0194  addze r3, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[3].s64 = tmp.s64;
	// 830CC6CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CC6D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830CC6D0 size=8
    let mut pc: u32 = 0x830CC6D0;
    'dispatch: loop {
        match pc {
            0x830CC6D0 => {
    //   block [0x830CC6D0..0x830CC6D8)
	// 830CC6D0: 7D234B78  mr r3, r9
	ctx.r[3].u64 = ctx.r[9].u64;
	// 830CC6D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CC6D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830CC6D8 size=1084
    let mut pc: u32 = 0x830CC6D8;
    'dispatch: loop {
        match pc {
            0x830CC6D8 => {
    //   block [0x830CC6D8..0x830CCB14)
	// 830CC6D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830CC6DC: 480DBA89  bl 0x831a8164
	ctx.lr = 0x830CC6E0;
	sub_831A8130(ctx, base);
	// 830CC6E0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830CC6E4: 81660000  lwz r11, 0(r6)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 830CC6E8: 39210050  addi r9, r1, 0x50
	ctx.r[9].s64 = ctx.r[1].s64 + 80;
	// 830CC6EC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830CC6F0: 81060004  lwz r8, 4(r6)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(4 as u32) ) } as u64;
	// 830CC6F4: 80E60008  lwz r7, 8(r6)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(8 as u32) ) } as u64;
	// 830CC6F8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 830CC6FC: 80C6000C  lwz r6, 0xc(r6)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(12 as u32) ) } as u64;
	// 830CC700: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 830CC704: 91690000  stw r11, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830CC708: 815E0004  lwz r10, 4(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830CC70C: 91090004  stw r8, 4(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 830CC710: 90E90008  stw r7, 8(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), ctx.r[7].u32 ) };
	// 830CC714: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 830CC718: 90C9000C  stw r6, 0xc(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(12 as u32), ctx.r[6].u32 ) };
	// 830CC71C: 409A000C  bne cr6, 0x830cc728
	if !ctx.cr[6].eq {
	pc = 0x830CC728; continue 'dispatch;
	}
	// 830CC720: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 830CC724: 48000010  b 0x830cc734
	pc = 0x830CC734; continue 'dispatch;
	// 830CC728: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 830CC72C: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 830CC730: 7D682670  srawi r8, r11, 4
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 4) - 1)) != 0);
	ctx.r[8].s64 = (ctx.r[11].s32 >> 4) as i64;
	// 830CC734: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 830CC738: 419A03D4  beq cr6, 0x830ccb0c
	if ctx.cr[6].eq {
	pc = 0x830CCB0C; continue 'dispatch;
	}
	// 830CC73C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 830CC740: 409A000C  bne cr6, 0x830cc74c
	if !ctx.cr[6].eq {
	pc = 0x830CC74C; continue 'dispatch;
	}
	// 830CC744: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830CC748: 48000010  b 0x830cc758
	pc = 0x830CC758; continue 'dispatch;
	// 830CC74C: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 830CC750: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 830CC754: 7D6B2670  srawi r11, r11, 4
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 4) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 4) as i64;
	// 830CC758: 3D200FFF  lis r9, 0xfff
	ctx.r[9].s64 = 268369920;
	// 830CC75C: 6129FFFF  ori r9, r9, 0xffff
	ctx.r[9].u64 = ctx.r[9].u64 | 65535;
	// 830CC760: 7D6B4850  subf r11, r11, r9
	ctx.r[11].s64 = ctx.r[9].s64 - ctx.r[11].s64;
	// 830CC764: 7F0BE040  cmplw cr6, r11, r28
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[28].u32, &mut ctx.xer);
	// 830CC768: 4098000C  bge cr6, 0x830cc774
	if !ctx.cr[6].lt {
	pc = 0x830CC774; continue 'dispatch;
	}
	// 830CC76C: 4BFF89CD  bl 0x830c5138
	ctx.lr = 0x830CC770;
	sub_830C5138(ctx, base);
	// 830CC770: 4800039C  b 0x830ccb0c
	pc = 0x830CCB0C; continue 'dispatch;
	// 830CC774: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 830CC778: 409A000C  bne cr6, 0x830cc784
	if !ctx.cr[6].eq {
	pc = 0x830CC784; continue 'dispatch;
	}
	// 830CC77C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830CC780: 48000010  b 0x830cc790
	pc = 0x830CC790; continue 'dispatch;
	// 830CC784: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 830CC788: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 830CC78C: 7D6B2670  srawi r11, r11, 4
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 4) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 4) as i64;
	// 830CC790: 7D6BE214  add r11, r11, r28
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[28].u64;
	// 830CC794: 7F085840  cmplw cr6, r8, r11
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[11].u32, &mut ctx.xer);
	// 830CC798: 409801A8  bge cr6, 0x830cc940
	if !ctx.cr[6].lt {
	pc = 0x830CC940; continue 'dispatch;
	}
	// 830CC79C: 550BF87E  srwi r11, r8, 1
	ctx.r[11].u32 = ctx.r[8].u32.wrapping_shr(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 830CC7A0: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 830CC7A4: 7D2B4850  subf r9, r11, r9
	ctx.r[9].s64 = ctx.r[9].s64 - ctx.r[11].s64;
	// 830CC7A8: 7F094040  cmplw cr6, r9, r8
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[8].u32, &mut ctx.xer);
	// 830CC7AC: 41980008  blt cr6, 0x830cc7b4
	if ctx.cr[6].lt {
	pc = 0x830CC7B4; continue 'dispatch;
	}
	// 830CC7B0: 7F6B4214  add r27, r11, r8
	ctx.r[27].u64 = ctx.r[11].u64 + ctx.r[8].u64;
	// 830CC7B4: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 830CC7B8: 409A000C  bne cr6, 0x830cc7c4
	if !ctx.cr[6].eq {
	pc = 0x830CC7C4; continue 'dispatch;
	}
	// 830CC7BC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830CC7C0: 48000010  b 0x830cc7d0
	pc = 0x830CC7D0; continue 'dispatch;
	// 830CC7C4: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 830CC7C8: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 830CC7CC: 7D6B2670  srawi r11, r11, 4
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 4) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 4) as i64;
	// 830CC7D0: 7D6BE214  add r11, r11, r28
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[28].u64;
	// 830CC7D4: 7F1B5840  cmplw cr6, r27, r11
	ctx.cr[6].compare_u32(ctx.r[27].u32, ctx.r[11].u32, &mut ctx.xer);
	// 830CC7D8: 40980024  bge cr6, 0x830cc7fc
	if !ctx.cr[6].lt {
	pc = 0x830CC7FC; continue 'dispatch;
	}
	// 830CC7DC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 830CC7E0: 409A000C  bne cr6, 0x830cc7ec
	if !ctx.cr[6].eq {
	pc = 0x830CC7EC; continue 'dispatch;
	}
	// 830CC7E4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830CC7E8: 48000010  b 0x830cc7f8
	pc = 0x830CC7F8; continue 'dispatch;
	// 830CC7EC: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 830CC7F0: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 830CC7F4: 7D6B2670  srawi r11, r11, 4
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 4) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 4) as i64;
	// 830CC7F8: 7F6BE214  add r27, r11, r28
	ctx.r[27].u64 = ctx.r[11].u64 + ctx.r[28].u64;
	// 830CC7FC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830CC800: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 830CC804: 4BFF3115  bl 0x830bf918
	ctx.lr = 0x830CC808;
	sub_830BF918(ctx, base);
	// 830CC808: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 830CC80C: 815E0004  lwz r10, 4(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830CC810: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 830CC814: 48000034  b 0x830cc848
	pc = 0x830CC848; continue 'dispatch;
	// 830CC818: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830CC81C: 419A0024  beq cr6, 0x830cc840
	if ctx.cr[6].eq {
	pc = 0x830CC840; continue 'dispatch;
	}
	// 830CC820: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 830CC824: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 830CC828: 812A0004  lwz r9, 4(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 830CC82C: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 830CC830: 812A0008  lwz r9, 8(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 830CC834: 912B0008  stw r9, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 830CC838: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 830CC83C: 912B000C  stw r9, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 830CC840: 394A0010  addi r10, r10, 0x10
	ctx.r[10].s64 = ctx.r[10].s64 + 16;
	// 830CC844: 396B0010  addi r11, r11, 0x10
	ctx.r[11].s64 = ctx.r[11].s64 + 16;
	// 830CC848: 7F0AF840  cmplw cr6, r10, r31
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[31].u32, &mut ctx.xer);
	// 830CC84C: 409AFFCC  bne cr6, 0x830cc818
	if !ctx.cr[6].eq {
	pc = 0x830CC818; continue 'dispatch;
	}
	// 830CC850: 7F89E378  mr r9, r28
	ctx.r[9].u64 = ctx.r[28].u64;
	// 830CC854: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 830CC858: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 830CC85C: 419A003C  beq cr6, 0x830cc898
	if ctx.cr[6].eq {
	pc = 0x830CC898; continue 'dispatch;
	}
	// 830CC860: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 830CC864: 419A0028  beq cr6, 0x830cc88c
	if ctx.cr[6].eq {
	pc = 0x830CC88C; continue 'dispatch;
	}
	// 830CC868: 39010050  addi r8, r1, 0x50
	ctx.r[8].s64 = ctx.r[1].s64 + 80;
	// 830CC86C: 80E80000  lwz r7, 0(r8)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 830CC870: 80C80004  lwz r6, 4(r8)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 830CC874: 80A80008  lwz r5, 8(r8)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(8 as u32) ) } as u64;
	// 830CC878: 8108000C  lwz r8, 0xc(r8)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(12 as u32) ) } as u64;
	// 830CC87C: 90EA0000  stw r7, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 830CC880: 90CA0004  stw r6, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[6].u32 ) };
	// 830CC884: 90AA0008  stw r5, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[5].u32 ) };
	// 830CC888: 910A000C  stw r8, 0xc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 830CC88C: 3529FFFF  addic. r9, r9, -1
	ctx.xer.ca = (ctx.r[9].u32 > (!(-1 as u32)));
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 830CC890: 394A0010  addi r10, r10, 0x10
	ctx.r[10].s64 = ctx.r[10].s64 + 16;
	// 830CC894: 4082FFCC  bne 0x830cc860
	if !ctx.cr[0].eq {
	pc = 0x830CC860; continue 'dispatch;
	}
	// 830CC898: 811E0008  lwz r8, 8(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 830CC89C: 57892036  slwi r9, r28, 4
	ctx.r[9].u32 = ctx.r[28].u32.wrapping_shl(4);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 830CC8A0: 7D495A14  add r10, r9, r11
	ctx.r[10].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 830CC8A4: 7F1F4040  cmplw cr6, r31, r8
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[8].u32, &mut ctx.xer);
	// 830CC8A8: 419A0048  beq cr6, 0x830cc8f0
	if ctx.cr[6].eq {
	pc = 0x830CC8F0; continue 'dispatch;
	}
	// 830CC8AC: 7D295050  subf r9, r9, r10
	ctx.r[9].s64 = ctx.r[10].s64 - ctx.r[9].s64;
	// 830CC8B0: 7D6B4850  subf r11, r11, r9
	ctx.r[11].s64 = ctx.r[9].s64 - ctx.r[11].s64;
	// 830CC8B4: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 830CC8B8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 830CC8BC: 419A0024  beq cr6, 0x830cc8e0
	if ctx.cr[6].eq {
	pc = 0x830CC8E0; continue 'dispatch;
	}
	// 830CC8C0: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830CC8C4: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 830CC8C8: 812B0004  lwz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830CC8CC: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 830CC8D0: 812B0008  lwz r9, 8(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 830CC8D4: 912A0008  stw r9, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 830CC8D8: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 830CC8DC: 912A000C  stw r9, 0xc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 830CC8E0: 396B0010  addi r11, r11, 0x10
	ctx.r[11].s64 = ctx.r[11].s64 + 16;
	// 830CC8E4: 394A0010  addi r10, r10, 0x10
	ctx.r[10].s64 = ctx.r[10].s64 + 16;
	// 830CC8E8: 7F0B4040  cmplw cr6, r11, r8
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[8].u32, &mut ctx.xer);
	// 830CC8EC: 409AFFCC  bne cr6, 0x830cc8b8
	if !ctx.cr[6].eq {
	pc = 0x830CC8B8; continue 'dispatch;
	}
	// 830CC8F0: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830CC8F4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 830CC8F8: 409A000C  bne cr6, 0x830cc904
	if !ctx.cr[6].eq {
	pc = 0x830CC904; continue 'dispatch;
	}
	// 830CC8FC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830CC900: 48000010  b 0x830cc910
	pc = 0x830CC910; continue 'dispatch;
	// 830CC904: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 830CC908: 7D635850  subf r11, r3, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[3].s64;
	// 830CC90C: 7D6B2670  srawi r11, r11, 4
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 4) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 4) as i64;
	// 830CC910: 7FEBE214  add r31, r11, r28
	ctx.r[31].u64 = ctx.r[11].u64 + ctx.r[28].u64;
	// 830CC914: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 830CC918: 419A0008  beq cr6, 0x830cc920
	if ctx.cr[6].eq {
	pc = 0x830CC920; continue 'dispatch;
	}
	// 830CC91C: 4B1F394D  bl 0x822c0268
	ctx.lr = 0x830CC920;
	sub_822C0268(ctx, base);
	// 830CC920: 576B2036  slwi r11, r27, 4
	ctx.r[11].u32 = ctx.r[27].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 830CC924: 93BE0004  stw r29, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 830CC928: 57EA2036  slwi r10, r31, 4
	ctx.r[10].u32 = ctx.r[31].u32.wrapping_shl(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 830CC92C: 7D6BEA14  add r11, r11, r29
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 830CC930: 7D4AEA14  add r10, r10, r29
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[29].u64;
	// 830CC934: 917E000C  stw r11, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 830CC938: 915E0008  stw r10, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 830CC93C: 480001D0  b 0x830ccb0c
	pc = 0x830CCB0C; continue 'dispatch;
	// 830CC940: 813E0008  lwz r9, 8(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 830CC944: 7D7F4850  subf r11, r31, r9
	ctx.r[11].s64 = ctx.r[9].s64 - ctx.r[31].s64;
	// 830CC948: 7D6B2670  srawi r11, r11, 4
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 4) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 4) as i64;
	// 830CC94C: 7F0BE040  cmplw cr6, r11, r28
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[28].u32, &mut ctx.xer);
	// 830CC950: 409800EC  bge cr6, 0x830cca3c
	if !ctx.cr[6].lt {
	pc = 0x830CCA3C; continue 'dispatch;
	}
	// 830CC954: 57882036  slwi r8, r28, 4
	ctx.r[8].u32 = ctx.r[28].u32.wrapping_shl(4);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 830CC958: 7F1F4840  cmplw cr6, r31, r9
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[9].u32, &mut ctx.xer);
	// 830CC95C: 7D68FA14  add r11, r8, r31
	ctx.r[11].u64 = ctx.r[8].u64 + ctx.r[31].u64;
	// 830CC960: 419A0040  beq cr6, 0x830cc9a0
	if ctx.cr[6].eq {
	pc = 0x830CC9A0; continue 'dispatch;
	}
	// 830CC964: 7D485850  subf r10, r8, r11
	ctx.r[10].s64 = ctx.r[11].s64 - ctx.r[8].s64;
	// 830CC968: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830CC96C: 419A0024  beq cr6, 0x830cc990
	if ctx.cr[6].eq {
	pc = 0x830CC990; continue 'dispatch;
	}
	// 830CC970: 80EA0000  lwz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 830CC974: 90EB0000  stw r7, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 830CC978: 80EA0004  lwz r7, 4(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 830CC97C: 90EB0004  stw r7, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 830CC980: 80EA0008  lwz r7, 8(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 830CC984: 90EB0008  stw r7, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[7].u32 ) };
	// 830CC988: 80EA000C  lwz r7, 0xc(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 830CC98C: 90EB000C  stw r7, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[7].u32 ) };
	// 830CC990: 394A0010  addi r10, r10, 0x10
	ctx.r[10].s64 = ctx.r[10].s64 + 16;
	// 830CC994: 396B0010  addi r11, r11, 0x10
	ctx.r[11].s64 = ctx.r[11].s64 + 16;
	// 830CC998: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 830CC99C: 409AFFCC  bne cr6, 0x830cc968
	if !ctx.cr[6].eq {
	pc = 0x830CC968; continue 'dispatch;
	}
	// 830CC9A0: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 830CC9A4: 7D5F5850  subf r10, r31, r11
	ctx.r[10].s64 = ctx.r[11].s64 - ctx.r[31].s64;
	// 830CC9A8: 7D4A2670  srawi r10, r10, 4
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 4) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[10].s32 >> 4) as i64;
	// 830CC9AC: 7D4AE051  subf. r10, r10, r28
	ctx.r[10].s64 = ctx.r[28].s64 - ctx.r[10].s64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 830CC9B0: 4182003C  beq 0x830cc9ec
	if ctx.cr[0].eq {
	pc = 0x830CC9EC; continue 'dispatch;
	}
	// 830CC9B4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830CC9B8: 419A0028  beq cr6, 0x830cc9e0
	if ctx.cr[6].eq {
	pc = 0x830CC9E0; continue 'dispatch;
	}
	// 830CC9BC: 39210050  addi r9, r1, 0x50
	ctx.r[9].s64 = ctx.r[1].s64 + 80;
	// 830CC9C0: 80E90000  lwz r7, 0(r9)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 830CC9C4: 80C90004  lwz r6, 4(r9)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 830CC9C8: 80A90008  lwz r5, 8(r9)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) } as u64;
	// 830CC9CC: 8129000C  lwz r9, 0xc(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(12 as u32) ) } as u64;
	// 830CC9D0: 90EB0000  stw r7, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 830CC9D4: 90CB0004  stw r6, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[6].u32 ) };
	// 830CC9D8: 90AB0008  stw r5, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[5].u32 ) };
	// 830CC9DC: 912B000C  stw r9, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 830CC9E0: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 830CC9E4: 396B0010  addi r11, r11, 0x10
	ctx.r[11].s64 = ctx.r[11].s64 + 16;
	// 830CC9E8: 4082FFCC  bne 0x830cc9b4
	if !ctx.cr[0].eq {
	pc = 0x830CC9B4; continue 'dispatch;
	}
	// 830CC9EC: 815E0008  lwz r10, 8(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 830CC9F0: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 830CC9F4: 7D485214  add r10, r8, r10
	ctx.r[10].u64 = ctx.r[8].u64 + ctx.r[10].u64;
	// 830CC9F8: 7D285050  subf r9, r8, r10
	ctx.r[9].s64 = ctx.r[10].s64 - ctx.r[8].s64;
	// 830CC9FC: 915E0008  stw r10, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 830CCA00: 7F1F4840  cmplw cr6, r31, r9
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[9].u32, &mut ctx.xer);
	// 830CCA04: 419A0108  beq cr6, 0x830ccb0c
	if ctx.cr[6].eq {
	pc = 0x830CCB0C; continue 'dispatch;
	}
	// 830CCA08: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 830CCA0C: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 830CCA10: 80EA0004  lwz r7, 4(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 830CCA14: 80CA0008  lwz r6, 8(r10)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 830CCA18: 814A000C  lwz r10, 0xc(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 830CCA1C: 910B0000  stw r8, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 830CCA20: 90EB0004  stw r7, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 830CCA24: 90CB0008  stw r6, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[6].u32 ) };
	// 830CCA28: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 830CCA2C: 396B0010  addi r11, r11, 0x10
	ctx.r[11].s64 = ctx.r[11].s64 + 16;
	// 830CCA30: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 830CCA34: 409AFFD4  bne cr6, 0x830cca08
	if !ctx.cr[6].eq {
	pc = 0x830CCA08; continue 'dispatch;
	}
	// 830CCA38: 480000D4  b 0x830ccb0c
	pc = 0x830CCB0C; continue 'dispatch;
	// 830CCA3C: 57872036  slwi r7, r28, 4
	ctx.r[7].u32 = ctx.r[28].u32.wrapping_shl(4);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 830CCA40: 7D2A4B78  mr r10, r9
	ctx.r[10].u64 = ctx.r[9].u64;
	// 830CCA44: 7D674850  subf r11, r7, r9
	ctx.r[11].s64 = ctx.r[9].s64 - ctx.r[7].s64;
	// 830CCA48: 7D685B78  mr r8, r11
	ctx.r[8].u64 = ctx.r[11].u64;
	// 830CCA4C: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 830CCA50: 419A003C  beq cr6, 0x830cca8c
	if ctx.cr[6].eq {
	pc = 0x830CCA8C; continue 'dispatch;
	}
	// 830CCA54: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 830CCA58: 419A0024  beq cr6, 0x830cca7c
	if ctx.cr[6].eq {
	pc = 0x830CCA7C; continue 'dispatch;
	}
	// 830CCA5C: 80C80000  lwz r6, 0(r8)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 830CCA60: 90CA0000  stw r6, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[6].u32 ) };
	// 830CCA64: 80C80004  lwz r6, 4(r8)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 830CCA68: 90CA0004  stw r6, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[6].u32 ) };
	// 830CCA6C: 80C80008  lwz r6, 8(r8)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(8 as u32) ) } as u64;
	// 830CCA70: 90CA0008  stw r6, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[6].u32 ) };
	// 830CCA74: 80C8000C  lwz r6, 0xc(r8)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(12 as u32) ) } as u64;
	// 830CCA78: 90CA000C  stw r6, 0xc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(12 as u32), ctx.r[6].u32 ) };
	// 830CCA7C: 39080010  addi r8, r8, 0x10
	ctx.r[8].s64 = ctx.r[8].s64 + 16;
	// 830CCA80: 394A0010  addi r10, r10, 0x10
	ctx.r[10].s64 = ctx.r[10].s64 + 16;
	// 830CCA84: 7F084840  cmplw cr6, r8, r9
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[9].u32, &mut ctx.xer);
	// 830CCA88: 409AFFCC  bne cr6, 0x830cca54
	if !ctx.cr[6].eq {
	pc = 0x830CCA54; continue 'dispatch;
	}
	// 830CCA8C: 915E0008  stw r10, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 830CCA90: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 830CCA94: 419A0038  beq cr6, 0x830ccacc
	if ctx.cr[6].eq {
	pc = 0x830CCACC; continue 'dispatch;
	}
	// 830CCA98: 7D475A14  add r10, r7, r11
	ctx.r[10].u64 = ctx.r[7].u64 + ctx.r[11].u64;
	// 830CCA9C: 396BFFF0  addi r11, r11, -0x10
	ctx.r[11].s64 = ctx.r[11].s64 + -16;
	// 830CCAA0: 394AFFF0  addi r10, r10, -0x10
	ctx.r[10].s64 = ctx.r[10].s64 + -16;
	// 830CCAA4: 7F0BF840  cmplw cr6, r11, r31
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[31].u32, &mut ctx.xer);
	// 830CCAA8: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830CCAAC: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 830CCAB0: 812B0004  lwz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830CCAB4: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 830CCAB8: 812B0008  lwz r9, 8(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 830CCABC: 912A0008  stw r9, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 830CCAC0: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 830CCAC4: 912A000C  stw r9, 0xc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 830CCAC8: 409AFFD4  bne cr6, 0x830cca9c
	if !ctx.cr[6].eq {
	pc = 0x830CCA9C; continue 'dispatch;
	}
	// 830CCACC: 7D47FA14  add r10, r7, r31
	ctx.r[10].u64 = ctx.r[7].u64 + ctx.r[31].u64;
	// 830CCAD0: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 830CCAD4: 7F1F5040  cmplw cr6, r31, r10
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[10].u32, &mut ctx.xer);
	// 830CCAD8: 419A0034  beq cr6, 0x830ccb0c
	if ctx.cr[6].eq {
	pc = 0x830CCB0C; continue 'dispatch;
	}
	// 830CCADC: 39210050  addi r9, r1, 0x50
	ctx.r[9].s64 = ctx.r[1].s64 + 80;
	// 830CCAE0: 81090000  lwz r8, 0(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 830CCAE4: 80E90004  lwz r7, 4(r9)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 830CCAE8: 80C90008  lwz r6, 8(r9)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) } as u64;
	// 830CCAEC: 8129000C  lwz r9, 0xc(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(12 as u32) ) } as u64;
	// 830CCAF0: 910B0000  stw r8, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 830CCAF4: 90EB0004  stw r7, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 830CCAF8: 90CB0008  stw r6, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[6].u32 ) };
	// 830CCAFC: 912B000C  stw r9, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 830CCB00: 396B0010  addi r11, r11, 0x10
	ctx.r[11].s64 = ctx.r[11].s64 + 16;
	// 830CCB04: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 830CCB08: 409AFFD4  bne cr6, 0x830ccadc
	if !ctx.cr[6].eq {
	pc = 0x830CCADC; continue 'dispatch;
	}
	// 830CCB0C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 830CCB10: 480DB6A4  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CCB18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830CCB18 size=108
    let mut pc: u32 = 0x830CCB18;
    'dispatch: loop {
        match pc {
            0x830CCB18 => {
    //   block [0x830CCB18..0x830CCB84)
	// 830CCB18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830CCB1C: 480DB651  bl 0x831a816c
	ctx.lr = 0x830CCB20;
	sub_831A8130(ctx, base);
	// 830CCB20: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830CCB24: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 830CCB28: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830CCB2C: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 830CCB30: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830CCB34: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830CCB38: 419A0014  beq cr6, 0x830ccb4c
	if ctx.cr[6].eq {
	pc = 0x830CCB4C; continue 'dispatch;
	}
	// 830CCB3C: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 830CCB40: 7D4B5050  subf r10, r11, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 830CCB44: 7D4A2671  srawi. r10, r10, 4
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 4) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[10].s32 >> 4) as i64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 830CCB48: 4082000C  bne 0x830ccb54
	if !ctx.cr[0].eq {
	pc = 0x830CCB54; continue 'dispatch;
	}
	// 830CCB4C: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 830CCB50: 4800000C  b 0x830ccb5c
	pc = 0x830CCB5C; continue 'dispatch;
	// 830CCB54: 7D6B2050  subf r11, r11, r4
	ctx.r[11].s64 = ctx.r[4].s64 - ctx.r[11].s64;
	// 830CCB58: 7D7D2670  srawi r29, r11, 4
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 4) - 1)) != 0);
	ctx.r[29].s64 = (ctx.r[11].s32 >> 4) as i64;
	// 830CCB5C: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 830CCB60: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830CCB64: 4BFFFB75  bl 0x830cc6d8
	ctx.lr = 0x830CCB68;
	sub_830CC6D8(ctx, base);
	// 830CCB68: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830CCB6C: 57AB2036  slwi r11, r29, 4
	ctx.r[11].u32 = ctx.r[29].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 830CCB70: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830CCB74: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 830CCB78: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830CCB7C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830CCB80: 480DB63C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CCB88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830CCB88 size=28
    let mut pc: u32 = 0x830CCB88;
    'dispatch: loop {
        match pc {
            0x830CCB88 => {
    //   block [0x830CCB88..0x830CCBA4)
	// 830CCB88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830CCB8C: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 830CCB90: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 830CCB94: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 830CCB98: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 830CCB9C: 91430010  stw r10, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 830CCBA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CCBA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830CCBA8 size=164
    let mut pc: u32 = 0x830CCBA8;
    'dispatch: loop {
        match pc {
            0x830CCBA8 => {
    //   block [0x830CCBA8..0x830CCC4C)
	// 830CCBA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830CCBAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830CCBB0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830CCBB4: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 830CCBB8: 7C862378  mr r6, r4
	ctx.r[6].u64 = ctx.r[4].u64;
	// 830CCBBC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830CCBC0: 409A000C  bne cr6, 0x830ccbcc
	if !ctx.cr[6].eq {
	pc = 0x830CCBCC; continue 'dispatch;
	}
	// 830CCBC4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 830CCBC8: 48000010  b 0x830ccbd8
	pc = 0x830CCBD8; continue 'dispatch;
	// 830CCBCC: 81430008  lwz r10, 8(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 830CCBD0: 7D4B5050  subf r10, r11, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 830CCBD4: 7D4A2670  srawi r10, r10, 4
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 4) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[10].s32 >> 4) as i64;
	// 830CCBD8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830CCBDC: 419A0050  beq cr6, 0x830ccc2c
	if ctx.cr[6].eq {
	pc = 0x830CCC2C; continue 'dispatch;
	}
	// 830CCBE0: 8123000C  lwz r9, 0xc(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 830CCBE4: 7D6B4850  subf r11, r11, r9
	ctx.r[11].s64 = ctx.r[9].s64 - ctx.r[11].s64;
	// 830CCBE8: 7D6B2670  srawi r11, r11, 4
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 4) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 4) as i64;
	// 830CCBEC: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 830CCBF0: 4098003C  bge cr6, 0x830ccc2c
	if !ctx.cr[6].lt {
	pc = 0x830CCC2C; continue 'dispatch;
	}
	// 830CCBF4: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 830CCBF8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830CCBFC: 419A0024  beq cr6, 0x830ccc20
	if ctx.cr[6].eq {
	pc = 0x830CCC20; continue 'dispatch;
	}
	// 830CCC00: 81460000  lwz r10, 0(r6)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 830CCC04: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 830CCC08: 81460004  lwz r10, 4(r6)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(4 as u32) ) } as u64;
	// 830CCC0C: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 830CCC10: 81460008  lwz r10, 8(r6)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(8 as u32) ) } as u64;
	// 830CCC14: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 830CCC18: 8146000C  lwz r10, 0xc(r6)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(12 as u32) ) } as u64;
	// 830CCC1C: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 830CCC20: 396B0010  addi r11, r11, 0x10
	ctx.r[11].s64 = ctx.r[11].s64 + 16;
	// 830CCC24: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 830CCC28: 48000014  b 0x830ccc3c
	pc = 0x830CCC3C; continue 'dispatch;
	// 830CCC2C: 80A30008  lwz r5, 8(r3)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 830CCC30: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 830CCC34: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830CCC38: 4BFFFEE1  bl 0x830ccb18
	ctx.lr = 0x830CCC3C;
	sub_830CCB18(ctx, base);
	// 830CCC3C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830CCC40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830CCC44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830CCC48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CCC50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830CCC50 size=4
    let mut pc: u32 = 0x830CCC50;
    'dispatch: loop {
        match pc {
            0x830CCC50 => {
    //   block [0x830CCC50..0x830CCC54)
	// 830CCC50: 4BFFFF58  b 0x830ccba8
	sub_830CCBA8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CCC58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830CCC58 size=88
    let mut pc: u32 = 0x830CCC58;
    'dispatch: loop {
        match pc {
            0x830CCC58 => {
    //   block [0x830CCC58..0x830CCCB0)
	// 830CCC58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830CCC5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830CCC60: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830CCC64: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830CCC68: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830CCC6C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 830CCC70: 419A002C  beq cr6, 0x830ccc9c
	if ctx.cr[6].eq {
	pc = 0x830CCC9C; continue 'dispatch;
	}
	// 830CCC74: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830CCC78: 48010861  bl 0x830dd4d8
	ctx.lr = 0x830CCC7C;
	sub_830DD4D8(ctx, base);
	// 830CCC7C: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830CCC80: 48010859  bl 0x830dd4d8
	ctx.lr = 0x830CCC84;
	sub_830DD4D8(ctx, base);
	// 830CCC84: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 830CCC88: 48010851  bl 0x830dd4d8
	ctx.lr = 0x830CCC8C;
	sub_830DD4D8(ctx, base);
	// 830CCC8C: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 830CCC90: 48010849  bl 0x830dd4d8
	ctx.lr = 0x830CCC94;
	sub_830DD4D8(ctx, base);
	// 830CCC94: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830CCC98: 48010841  bl 0x830dd4d8
	ctx.lr = 0x830CCC9C;
	sub_830DD4D8(ctx, base);
	// 830CCC9C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830CCCA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830CCCA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830CCCA8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830CCCAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CCCB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x830CCCB0 size=316
    let mut pc: u32 = 0x830CCCB0;
    'dispatch: loop {
        match pc {
            0x830CCCB0 => {
    //   block [0x830CCCB0..0x830CCDEC)
	// 830CCCB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830CCCB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830CCCB8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830CCCBC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830CCCC0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830CCCC4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830CCCC8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 830CCCCC: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 830CCCD0: 419A00FC  beq cr6, 0x830ccdcc
	if ctx.cr[6].eq {
	pc = 0x830CCDCC; continue 'dispatch;
	}
	// 830CCCD4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 830CCCD8: 419A00F4  beq cr6, 0x830ccdcc
	if ctx.cr[6].eq {
	pc = 0x830CCDCC; continue 'dispatch;
	}
	// 830CCCDC: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 830CCCE0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 830CCCE4: 419A00E8  beq cr6, 0x830ccdcc
	if ctx.cr[6].eq {
	pc = 0x830CCDCC; continue 'dispatch;
	}
	// 830CCCE8: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830CCCEC: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 830CCCF0: 419A00DC  beq cr6, 0x830ccdcc
	if ctx.cr[6].eq {
	pc = 0x830CCDCC; continue 'dispatch;
	}
	// 830CCCF4: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830CCCF8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830CCCFC: 419A00D0  beq cr6, 0x830ccdcc
	if ctx.cr[6].eq {
	pc = 0x830CCDCC; continue 'dispatch;
	}
	// 830CCD00: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830CCD04: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830CCD08: 419A00C4  beq cr6, 0x830ccdcc
	if ctx.cr[6].eq {
	pc = 0x830CCDCC; continue 'dispatch;
	}
	// 830CCD0C: 480E4DAD  bl 0x831b1ab8
	ctx.lr = 0x830CCD10;
	sub_831B1AB8(ctx, base);
	// 830CCD10: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 830CCD14: 4182000C  beq 0x830ccd20
	if ctx.cr[0].eq {
	pc = 0x830CCD20; continue 'dispatch;
	}
	// 830CCD18: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 830CCD1C: 480000B8  b 0x830ccdd4
	pc = 0x830CCDD4; continue 'dispatch;
	// 830CCD20: 809F0004  lwz r4, 4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830CCD24: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830CCD28: 480E4D91  bl 0x831b1ab8
	ctx.lr = 0x830CCD2C;
	sub_831B1AB8(ctx, base);
	// 830CCD2C: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 830CCD30: 4082FFE8  bne 0x830ccd18
	if !ctx.cr[0].eq {
	pc = 0x830CCD18; continue 'dispatch;
	}
	// 830CCD34: 807E0008  lwz r3, 8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 830CCD38: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 830CCD3C: 409A0020  bne cr6, 0x830ccd5c
	if !ctx.cr[6].eq {
	pc = 0x830CCD5C; continue 'dispatch;
	}
	// 830CCD40: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 830CCD44: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830CCD48: 409A000C  bne cr6, 0x830ccd54
	if !ctx.cr[6].eq {
	pc = 0x830CCD54; continue 'dispatch;
	}
	// 830CCD4C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830CCD50: 48000084  b 0x830ccdd4
	pc = 0x830CCDD4; continue 'dispatch;
	// 830CCD54: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 830CCD58: 419AFFC0  beq cr6, 0x830ccd18
	if ctx.cr[6].eq {
	pc = 0x830CCD18; continue 'dispatch;
	}
	// 830CCD5C: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 830CCD60: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 830CCD64: 419AFFB4  beq cr6, 0x830ccd18
	if ctx.cr[6].eq {
	pc = 0x830CCD18; continue 'dispatch;
	}
	// 830CCD68: 480E4D51  bl 0x831b1ab8
	ctx.lr = 0x830CCD6C;
	sub_831B1AB8(ctx, base);
	// 830CCD6C: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 830CCD70: 4082FFA8  bne 0x830ccd18
	if !ctx.cr[0].eq {
	pc = 0x830CCD18; continue 'dispatch;
	}
	// 830CCD74: 807E0010  lwz r3, 0x10(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 830CCD78: 809F0010  lwz r4, 0x10(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 830CCD7C: 7C6B0034  cntlzw r11, r3
	ctx.r[11].u64 = if ctx.r[3].u32 == 0 { 32 } else { ctx.r[3].u32.leading_zeros() as u64 };
	// 830CCD80: 7C8A0034  cntlzw r10, r4
	ctx.r[10].u64 = if ctx.r[4].u32 == 0 { 32 } else { ctx.r[4].u32.leading_zeros() as u64 };
	// 830CCD84: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 830CCD88: 554ADFFE  rlwinm r10, r10, 0x1b, 0x1f, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x0000001Fu64;
	// 830CCD8C: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 830CCD90: 694A0001  xori r10, r10, 1
	ctx.r[10].u64 = ctx.r[10].u64 ^ 1;
	// 830CCD94: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 830CCD98: 409AFF80  bne cr6, 0x830ccd18
	if !ctx.cr[6].eq {
	pc = 0x830CCD18; continue 'dispatch;
	}
	// 830CCD9C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 830CCDA0: 419A0018  beq cr6, 0x830ccdb8
	if ctx.cr[6].eq {
	pc = 0x830CCDB8; continue 'dispatch;
	}
	// 830CCDA4: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 830CCDA8: 419A0010  beq cr6, 0x830ccdb8
	if ctx.cr[6].eq {
	pc = 0x830CCDB8; continue 'dispatch;
	}
	// 830CCDAC: 480E4D0D  bl 0x831b1ab8
	ctx.lr = 0x830CCDB0;
	sub_831B1AB8(ctx, base);
	// 830CCDB0: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 830CCDB4: 4082FF64  bne 0x830ccd18
	if !ctx.cr[0].eq {
	pc = 0x830CCD18; continue 'dispatch;
	}
	// 830CCDB8: C01E000C  lfs f0, 0xc(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 830CCDBC: C1BF000C  lfs f13, 0xc(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 830CCDC0: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 830CCDC4: 419AFF88  beq cr6, 0x830ccd4c
	if ctx.cr[6].eq {
	pc = 0x830CCD4C; continue 'dispatch;
	}
	// 830CCDC8: 4BFFFF50  b 0x830ccd18
	pc = 0x830CCD18; continue 'dispatch;
	// 830CCDCC: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 830CCDD0: 60630057  ori r3, r3, 0x57
	ctx.r[3].u64 = ctx.r[3].u64 | 87;
	// 830CCDD4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830CCDD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830CCDDC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830CCDE0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830CCDE4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830CCDE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CCDF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830CCDF0 size=156
    let mut pc: u32 = 0x830CCDF0;
    'dispatch: loop {
        match pc {
            0x830CCDF0 => {
    //   block [0x830CCDF0..0x830CCE8C)
	// 830CCDF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830CCDF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830CCDF8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830CCDFC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830CCE00: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830CCE04: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830CCE08: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 830CCE0C: 419A0068  beq cr6, 0x830cce74
	if ctx.cr[6].eq {
	pc = 0x830CCE74; continue 'dispatch;
	}
	// 830CCE10: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830CCE14: 480106A5  bl 0x830dd4b8
	ctx.lr = 0x830CCE18;
	sub_830DD4B8(ctx, base);
	// 830CCE18: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 830CCE1C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830CCE20: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830CCE24: 419A0010  beq cr6, 0x830cce34
	if ctx.cr[6].eq {
	pc = 0x830CCE34; continue 'dispatch;
	}
	// 830CCE28: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 830CCE2C: 4801068D  bl 0x830dd4b8
	ctx.lr = 0x830CCE30;
	sub_830DD4B8(ctx, base);
	// 830CCE30: 7FE3FA14  add r31, r3, r31
	ctx.r[31].u64 = ctx.r[3].u64 + ctx.r[31].u64;
	// 830CCE34: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830CCE38: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 830CCE3C: 419A000C  beq cr6, 0x830cce48
	if ctx.cr[6].eq {
	pc = 0x830CCE48; continue 'dispatch;
	}
	// 830CCE40: 48010679  bl 0x830dd4b8
	ctx.lr = 0x830CCE44;
	sub_830DD4B8(ctx, base);
	// 830CCE44: 7FE3FA14  add r31, r3, r31
	ctx.r[31].u64 = ctx.r[3].u64 + ctx.r[31].u64;
	// 830CCE48: 807E0008  lwz r3, 8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 830CCE4C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 830CCE50: 419A000C  beq cr6, 0x830cce5c
	if ctx.cr[6].eq {
	pc = 0x830CCE5C; continue 'dispatch;
	}
	// 830CCE54: 48010665  bl 0x830dd4b8
	ctx.lr = 0x830CCE58;
	sub_830DD4B8(ctx, base);
	// 830CCE58: 7FE3FA14  add r31, r3, r31
	ctx.r[31].u64 = ctx.r[3].u64 + ctx.r[31].u64;
	// 830CCE5C: 807E0010  lwz r3, 0x10(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 830CCE60: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 830CCE64: 419A000C  beq cr6, 0x830cce70
	if ctx.cr[6].eq {
	pc = 0x830CCE70; continue 'dispatch;
	}
	// 830CCE68: 48010651  bl 0x830dd4b8
	ctx.lr = 0x830CCE6C;
	sub_830DD4B8(ctx, base);
	// 830CCE6C: 7FE3FA14  add r31, r3, r31
	ctx.r[31].u64 = ctx.r[3].u64 + ctx.r[31].u64;
	// 830CCE70: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830CCE74: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830CCE78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830CCE7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830CCE80: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830CCE84: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830CCE88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CCE90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x830CCE90 size=196
    let mut pc: u32 = 0x830CCE90;
    'dispatch: loop {
        match pc {
            0x830CCE90 => {
    //   block [0x830CCE90..0x830CCF54)
	// 830CCE90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830CCE94: 480DB2D5  bl 0x831a8168
	ctx.lr = 0x830CCE98;
	sub_831A8130(ctx, base);
	// 830CCE98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830CCE9C: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 830CCEA0: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 830CCEA4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 830CCEA8: 38600014  li r3, 0x14
	ctx.r[3].s64 = 20;
	// 830CCEAC: 93DC0000  stw r30, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 830CCEB0: 48010601  bl 0x830dd4b0
	ctx.lr = 0x830CCEB4;
	sub_830DD4B0(ctx, base);
	// 830CCEB4: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 830CCEB8: 40820010  bne 0x830ccec8
	if !ctx.cr[0].eq {
	pc = 0x830CCEC8; continue 'dispatch;
	}
	// 830CCEBC: 3FC08007  lis r30, -0x7ff9
	ctx.r[30].s64 = -2147024896;
	// 830CCEC0: 63DE000E  ori r30, r30, 0xe
	ctx.r[30].u64 = ctx.r[30].u64 | 14;
	// 830CCEC4: 48000068  b 0x830ccf2c
	pc = 0x830CCF2C; continue 'dispatch;
	// 830CCEC8: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 830CCECC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830CCED0: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 830CCED4: 93DF0008  stw r30, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 830CCED8: 93DF000C  stw r30, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[30].u32 ) };
	// 830CCEDC: 93DF0010  stw r30, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[30].u32 ) };
	// 830CCEE0: 809D0000  lwz r4, 0(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 830CCEE4: 48001DA5  bl 0x830cec88
	ctx.lr = 0x830CCEE8;
	sub_830CEC88(ctx, base);
	// 830CCEE8: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 830CCEEC: 41800040  blt 0x830ccf2c
	if ctx.cr[0].lt {
	pc = 0x830CCF2C; continue 'dispatch;
	}
	// 830CCEF0: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 830CCEF4: 809D0004  lwz r4, 4(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 830CCEF8: 48001D91  bl 0x830cec88
	ctx.lr = 0x830CCEFC;
	sub_830CEC88(ctx, base);
	// 830CCEFC: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 830CCF00: 4180002C  blt 0x830ccf2c
	if ctx.cr[0].lt {
	pc = 0x830CCF2C; continue 'dispatch;
	}
	// 830CCF04: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 830CCF08: 809D0008  lwz r4, 8(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 830CCF0C: 48001D7D  bl 0x830cec88
	ctx.lr = 0x830CCF10;
	sub_830CEC88(ctx, base);
	// 830CCF10: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 830CCF14: 41800018  blt 0x830ccf2c
	if ctx.cr[0].lt {
	pc = 0x830CCF2C; continue 'dispatch;
	}
	// 830CCF18: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 830CCF1C: 809D0010  lwz r4, 0x10(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 830CCF20: 48001D69  bl 0x830cec88
	ctx.lr = 0x830CCF24;
	sub_830CEC88(ctx, base);
	// 830CCF24: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 830CCF28: 40800014  bge 0x830ccf3c
	if !ctx.cr[0].lt {
	pc = 0x830CCF3C; continue 'dispatch;
	}
	// 830CCF2C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830CCF30: 4BFFFD29  bl 0x830ccc58
	ctx.lr = 0x830CCF34;
	sub_830CCC58(ctx, base);
	// 830CCF34: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830CCF38: 48000014  b 0x830ccf4c
	pc = 0x830CCF4C; continue 'dispatch;
	// 830CCF3C: C01D000C  lfs f0, 0xc(r29)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 830CCF40: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830CCF44: D01F000C  stfs f0, 0xc(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 830CCF48: 93FC0000  stw r31, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 830CCF4C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 830CCF50: 480DB268  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CCF58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830CCF58 size=152
    let mut pc: u32 = 0x830CCF58;
    'dispatch: loop {
        match pc {
            0x830CCF58 => {
    //   block [0x830CCF58..0x830CCFF0)
	// 830CCF58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830CCF5C: 480DB20D  bl 0x831a8168
	ctx.lr = 0x830CCF60;
	sub_831A8130(ctx, base);
	// 830CCF60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830CCF64: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830CCF68: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 830CCF6C: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830CCF70: 48010569  bl 0x830dd4d8
	ctx.lr = 0x830CCF74;
	sub_830DD4D8(ctx, base);
	// 830CCF74: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830CCF78: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 830CCF7C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830CCF80: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 830CCF84: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 830CCF88: 419A005C  beq cr6, 0x830ccfe4
	if ctx.cr[6].eq {
	pc = 0x830CCFE4; continue 'dispatch;
	}
	// 830CCF8C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 830CCF90: 480DC139  bl 0x831a90c8
	ctx.lr = 0x830CCF94;
	sub_831A90C8(ctx, base);
	// 830CCF94: 37C30001  addic. r30, r3, 1
	ctx.xer.ca = (ctx.r[3].u32 > (!(1 as u32)));
	ctx.r[30].s64 = ctx.r[3].s64 + 1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 830CCF98: 4182002C  beq 0x830ccfc4
	if ctx.cr[0].eq {
	pc = 0x830CCFC4; continue 'dispatch;
	}
	// 830CCF9C: 3D607FFF  lis r11, 0x7fff
	ctx.r[11].s64 = 2147418112;
	// 830CCFA0: 616BFFFF  ori r11, r11, 0xffff
	ctx.r[11].u64 = ctx.r[11].u64 | 65535;
	// 830CCFA4: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 830CCFA8: 4098001C  bge cr6, 0x830ccfc4
	if !ctx.cr[6].lt {
	pc = 0x830CCFC4; continue 'dispatch;
	}
	// 830CCFAC: 57DD083C  slwi r29, r30, 1
	ctx.r[29].u32 = ctx.r[30].u32.wrapping_shl(1);
	ctx.r[29].u64 = ctx.r[29].u32 as u64;
	// 830CCFB0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830CCFB4: 480104FD  bl 0x830dd4b0
	ctx.lr = 0x830CCFB8;
	sub_830DD4B0(ctx, base);
	// 830CCFB8: 907F0000  stw r3, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 830CCFBC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830CCFC0: 4082000C  bne 0x830ccfcc
	if !ctx.cr[0].eq {
	pc = 0x830CCFCC; continue 'dispatch;
	}
	// 830CCFC4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830CCFC8: 48000020  b 0x830ccfe8
	pc = 0x830CCFE8; continue 'dispatch;
	// 830CCFCC: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 830CCFD0: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 830CCFD4: 480DB53D  bl 0x831a8510
	ctx.lr = 0x830CCFD8;
	sub_831A8510(ctx, base);
	// 830CCFD8: 397EFFFF  addi r11, r30, -1
	ctx.r[11].s64 = ctx.r[30].s64 + -1;
	// 830CCFDC: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 830CCFE0: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 830CCFE4: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 830CCFE8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 830CCFEC: 480DB1CC  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CCFF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830CCFF0 size=96
    let mut pc: u32 = 0x830CCFF0;
    'dispatch: loop {
        match pc {
            0x830CCFF0 => {
    //   block [0x830CCFF0..0x830CD050)
	// 830CCFF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830CCFF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830CCFF8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830CCFFC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830CD000: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830CD004: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830CD008: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 830CD00C: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830CD010: 480104C9  bl 0x830dd4d8
	ctx.lr = 0x830CD014;
	sub_830DD4D8(ctx, base);
	// 830CD014: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830CD018: 57CA07FF  clrlwi. r10, r30, 0x1f
	ctx.r[10].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 830CD01C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830CD020: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 830CD024: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 830CD028: 4182000C  beq 0x830cd034
	if ctx.cr[0].eq {
	pc = 0x830CD034; continue 'dispatch;
	}
	// 830CD02C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830CD030: 480104A9  bl 0x830dd4d8
	ctx.lr = 0x830CD034;
	sub_830DD4D8(ctx, base);
	// 830CD034: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830CD038: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830CD03C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830CD040: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830CD044: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830CD048: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830CD04C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CD050(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830CD050 size=124
    let mut pc: u32 = 0x830CD050;
    'dispatch: loop {
        match pc {
            0x830CD050 => {
    //   block [0x830CD050..0x830CD0CC)
	// 830CD050: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830CD054: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830CD058: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830CD05C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830CD060: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830CD064: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830CD068: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830CD06C: 3BCBC034  addi r30, r11, -0x3fcc
	ctx.r[30].s64 = ctx.r[11].s64 + -16332;
	// 830CD070: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830CD074: 481758F9  bl 0x8324296c
	ctx.lr = 0x830CD078;
	// extern call 0x8324296C → crate::xboxkrnl::RtlEnterCriticalSection
	crate::xboxkrnl::RtlEnterCriticalSection(ctx, base);
	// 830CD078: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830CD07C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 830CD080: 3BEBC054  addi r31, r11, -0x3fac
	ctx.r[31].s64 = ctx.r[11].s64 + -16300;
	// 830CD084: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830CD088: 48010CD9  bl 0x830ddd60
	ctx.lr = 0x830CD08C;
	sub_830DDD60(ctx, base);
	// 830CD08C: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 830CD090: 409A000C  bne cr6, 0x830cd09c
	if !ctx.cr[6].eq {
	pc = 0x830CD09C; continue 'dispatch;
	}
	// 830CD094: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 830CD098: 48000010  b 0x830cd0a8
	pc = 0x830CD0A8; continue 'dispatch;
	// 830CD09C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830CD0A0: 546A103A  slwi r10, r3, 2
	ctx.r[10].u32 = ctx.r[3].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 830CD0A4: 7FEA582E  lwzx r31, r10, r11
	ctx.r[31].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 830CD0A8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830CD0AC: 481758B1  bl 0x8324295c
	ctx.lr = 0x830CD0B0;
	// extern call 0x8324295C → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 830CD0B0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830CD0B4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830CD0B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830CD0BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830CD0C0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830CD0C4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830CD0C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CD0D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830CD0D0 size=20
    let mut pc: u32 = 0x830CD0D0;
    'dispatch: loop {
        match pc {
            0x830CD0D0 => {
    //   block [0x830CD0D0..0x830CD0E4)
	// 830CD0D0: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830CD0D4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830CD0D8: 816BC050  lwz r11, -0x3fb0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16304 as u32) ) } as u64;
	// 830CD0DC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830CD0E0: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CD0E4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830CD0E4 size=8
    let mut pc: u32 = 0x830CD0E4;
    'dispatch: loop {
        match pc {
            0x830CD0E4 => {
    //   block [0x830CD0E4..0x830CD0EC)
	// 830CD0E4: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 830CD0E8: 4BFFFF68  b 0x830cd050
	sub_830CD050(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CD0EC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830CD0EC size=4
    let mut pc: u32 = 0x830CD0EC;
    'dispatch: loop {
        match pc {
            0x830CD0EC => {
    //   block [0x830CD0EC..0x830CD0F0)
	// 830CD0EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CD0F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830CD0F0 size=140
    let mut pc: u32 = 0x830CD0F0;
    'dispatch: loop {
        match pc {
            0x830CD0F0 => {
    //   block [0x830CD0F0..0x830CD17C)
	// 830CD0F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830CD0F4: 480DB075  bl 0x831a8168
	ctx.lr = 0x830CD0F8;
	sub_831A8130(ctx, base);
	// 830CD0F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830CD0FC: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830CD100: 3B8BC034  addi r28, r11, -0x3fcc
	ctx.r[28].s64 = ctx.r[11].s64 + -16332;
	// 830CD104: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 830CD108: 48175865  bl 0x8324296c
	ctx.lr = 0x830CD10C;
	// extern call 0x8324296C → crate::xboxkrnl::RtlEnterCriticalSection
	crate::xboxkrnl::RtlEnterCriticalSection(ctx, base);
	// 830CD10C: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830CD110: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 830CD114: 3BEBC054  addi r31, r11, -0x3fac
	ctx.r[31].s64 = ctx.r[11].s64 + -16300;
	// 830CD118: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 830CD11C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830CD120: 40990028  ble cr6, 0x830cd148
	if !ctx.cr[6].gt {
	pc = 0x830CD148; continue 'dispatch;
	}
	// 830CD124: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 830CD128: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830CD12C: 7C7D582E  lwzx r3, r29, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 830CD130: 4BFFFB29  bl 0x830ccc58
	ctx.lr = 0x830CD134;
	sub_830CCC58(ctx, base);
	// 830CD134: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 830CD138: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 830CD13C: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 830CD140: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 830CD144: 4198FFE4  blt cr6, 0x830cd128
	if ctx.cr[6].lt {
	pc = 0x830CD128; continue 'dispatch;
	}
	// 830CD148: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830CD14C: 48010E4D  bl 0x830ddf98
	ctx.lr = 0x830CD150;
	sub_830DDF98(ctx, base);
	// 830CD150: 3FE08339  lis r31, -0x7cc7
	ctx.r[31].s64 = -2093416448;
	// 830CD154: 807FC050  lwz r3, -0x3fb0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-16304 as u32) ) } as u64;
	// 830CD158: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 830CD15C: 419A0010  beq cr6, 0x830cd16c
	if ctx.cr[6].eq {
	pc = 0x830CD16C; continue 'dispatch;
	}
	// 830CD160: 48010379  bl 0x830dd4d8
	ctx.lr = 0x830CD164;
	sub_830DD4D8(ctx, base);
	// 830CD164: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830CD168: 917FC050  stw r11, -0x3fb0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-16304 as u32), ctx.r[11].u32 ) };
	// 830CD16C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 830CD170: 481757ED  bl 0x8324295c
	ctx.lr = 0x830CD174;
	// extern call 0x8324295C → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 830CD174: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 830CD178: 480DB040  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CD180(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830CD180 size=200
    let mut pc: u32 = 0x830CD180;
    'dispatch: loop {
        match pc {
            0x830CD180 => {
    //   block [0x830CD180..0x830CD248)
	// 830CD180: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830CD184: 480DAFE5  bl 0x831a8168
	ctx.lr = 0x830CD188;
	sub_831A8130(ctx, base);
	// 830CD188: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830CD18C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830CD190: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 830CD194: 409A0010  bne cr6, 0x830cd1a4
	if !ctx.cr[6].eq {
	pc = 0x830CD1A4; continue 'dispatch;
	}
	// 830CD198: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 830CD19C: 60630057  ori r3, r3, 0x57
	ctx.r[3].u64 = ctx.r[3].u64 | 87;
	// 830CD1A0: 480000A0  b 0x830cd240
	pc = 0x830CD240; continue 'dispatch;
	// 830CD1A4: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830CD1A8: 3BABC034  addi r29, r11, -0x3fcc
	ctx.r[29].s64 = ctx.r[11].s64 + -16332;
	// 830CD1AC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830CD1B0: 481757BD  bl 0x8324296c
	ctx.lr = 0x830CD1B4;
	// extern call 0x8324296C → crate::xboxkrnl::RtlEnterCriticalSection
	crate::xboxkrnl::RtlEnterCriticalSection(ctx, base);
	// 830CD1B4: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830CD1B8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 830CD1BC: 3BEBC054  addi r31, r11, -0x3fac
	ctx.r[31].s64 = ctx.r[11].s64 + -16300;
	// 830CD1C0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830CD1C4: 48010B9D  bl 0x830ddd60
	ctx.lr = 0x830CD1C8;
	sub_830DDD60(ctx, base);
	// 830CD1C8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830CD1CC: 2F1EFFFF  cmpwi cr6, r30, -1
	ctx.cr[6].compare_i32(ctx.r[30].s32, -1, &mut ctx.xer);
	// 830CD1D0: 40990064  ble cr6, 0x830cd234
	if !ctx.cr[6].gt {
	pc = 0x830CD234; continue 'dispatch;
	}
	// 830CD1D4: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 830CD1D8: 409A0040  bne cr6, 0x830cd218
	if !ctx.cr[6].eq {
	pc = 0x830CD218; continue 'dispatch;
	}
	// 830CD1DC: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 830CD1E0: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 830CD1E4: 40990020  ble cr6, 0x830cd204
	if !ctx.cr[6].gt {
	pc = 0x830CD204; continue 'dispatch;
	}
	// 830CD1E8: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830CD1EC: 3D408339  lis r10, -0x7cc7
	ctx.r[10].s64 = -2093416448;
	// 830CD1F0: 386AC050  addi r3, r10, -0x3fb0
	ctx.r[3].s64 = ctx.r[10].s64 + -16304;
	// 830CD1F4: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830CD1F8: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830CD1FC: 48001A8D  bl 0x830cec88
	ctx.lr = 0x830CD200;
	sub_830CEC88(ctx, base);
	// 830CD200: 48000018  b 0x830cd218
	pc = 0x830CD218; continue 'dispatch;
	// 830CD204: 3F808339  lis r28, -0x7cc7
	ctx.r[28].s64 = -2093416448;
	// 830CD208: 807CC050  lwz r3, -0x3fb0(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(-16304 as u32) ) } as u64;
	// 830CD20C: 480102CD  bl 0x830dd4d8
	ctx.lr = 0x830CD210;
	sub_830DD4D8(ctx, base);
	// 830CD210: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830CD214: 917CC050  stw r11, -0x3fb0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(-16304 as u32), ctx.r[11].u32 ) };
	// 830CD218: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830CD21C: 57CA103A  slwi r10, r30, 2
	ctx.r[10].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 830CD220: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 830CD224: 4BFFFA35  bl 0x830ccc58
	ctx.lr = 0x830CD228;
	sub_830CCC58(ctx, base);
	// 830CD228: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830CD22C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830CD230: 48017061  bl 0x830e4290
	ctx.lr = 0x830CD234;
	sub_830E4290(ctx, base);
	// 830CD234: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830CD238: 48175725  bl 0x8324295c
	ctx.lr = 0x830CD23C;
	// extern call 0x8324295C → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 830CD23C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830CD240: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 830CD244: 480DAF74  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CD248(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830CD248 size=252
    let mut pc: u32 = 0x830CD248;
    'dispatch: loop {
        match pc {
            0x830CD248 => {
    //   block [0x830CD248..0x830CD344)
	// 830CD248: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830CD24C: 480DAF21  bl 0x831a816c
	ctx.lr = 0x830CD250;
	sub_831A8130(ctx, base);
	// 830CD250: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830CD254: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830CD258: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830CD25C: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 830CD260: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 830CD264: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 830CD268: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 830CD26C: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 830CD270: 4BFFFCE9  bl 0x830ccf58
	ctx.lr = 0x830CD274;
	sub_830CCF58(ctx, base);
	// 830CD274: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 830CD278: 418200B8  beq 0x830cd330
	if ctx.cr[0].eq {
	pc = 0x830CD330; continue 'dispatch;
	}
	// 830CD27C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 830CD280: 37CB0001  addic. r30, r11, 1
	ctx.xer.ca = (ctx.r[11].u32 > (!(1 as u32)));
	ctx.r[30].s64 = ctx.r[11].s64 + 1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 830CD284: 408100AC  ble 0x830cd330
	if !ctx.cr[0].gt {
	pc = 0x830CD330; continue 'dispatch;
	}
	// 830CD288: 3D601555  lis r11, 0x1555
	ctx.r[11].s64 = 357892096;
	// 830CD28C: 616B5555  ori r11, r11, 0x5555
	ctx.r[11].u64 = ctx.r[11].u64 | 21845;
	// 830CD290: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 830CD294: 4098009C  bge cr6, 0x830cd330
	if !ctx.cr[6].lt {
	pc = 0x830CD330; continue 'dispatch;
	}
	// 830CD298: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 830CD29C: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 830CD2A0: 40990038  ble cr6, 0x830cd2d8
	if !ctx.cr[6].gt {
	pc = 0x830CD2D8; continue 'dispatch;
	}
	// 830CD2A4: 1C9E000C  mulli r4, r30, 0xc
	ctx.r[4].s64 = ctx.r[30].s64 * 12;
	// 830CD2A8: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830CD2AC: 48010245  bl 0x830dd4f0
	ctx.lr = 0x830CD2B0;
	sub_830DD4F0(ctx, base);
	// 830CD2B0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830CD2B4: 4182007C  beq 0x830cd330
	if ctx.cr[0].eq {
	pc = 0x830CD330; continue 'dispatch;
	}
	// 830CD2B8: 907F0000  stw r3, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 830CD2BC: 57C4103A  slwi r4, r30, 2
	ctx.r[4].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 830CD2C0: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830CD2C4: 4801022D  bl 0x830dd4f0
	ctx.lr = 0x830CD2C8;
	sub_830DD4F0(ctx, base);
	// 830CD2C8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830CD2CC: 41820064  beq 0x830cd330
	if ctx.cr[0].eq {
	pc = 0x830CD330; continue 'dispatch;
	}
	// 830CD2D0: 907F0004  stw r3, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 830CD2D4: 93DF000C  stw r30, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[30].u32 ) };
	// 830CD2D8: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 830CD2DC: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830CD2E0: 1D4B000C  mulli r10, r11, 0xc
	ctx.r[10].s64 = ctx.r[11].s64 * 12;
	// 830CD2E4: 7D4A4A15  add. r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 830CD2E8: 4182001C  beq 0x830cd304
	if ctx.cr[0].eq {
	pc = 0x830CD304; continue 'dispatch;
	}
	// 830CD2EC: 81210050  lwz r9, 0x50(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 830CD2F0: 81010054  lwz r8, 0x54(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 830CD2F4: 80E10058  lwz r7, 0x58(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 830CD2F8: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 830CD2FC: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 830CD300: 90EA0008  stw r7, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[7].u32 ) };
	// 830CD304: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830CD308: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 830CD30C: 7D6B5215  add. r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830CD310: 4182000C  beq 0x830cd31c
	if ctx.cr[0].eq {
	pc = 0x830CD31C; continue 'dispatch;
	}
	// 830CD314: 815D0000  lwz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 830CD318: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 830CD31C: 93DF0008  stw r30, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 830CD320: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830CD324: 480101B5  bl 0x830dd4d8
	ctx.lr = 0x830CD328;
	sub_830DD4D8(ctx, base);
	// 830CD328: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 830CD32C: 48000010  b 0x830cd33c
	pc = 0x830CD33C; continue 'dispatch;
	// 830CD330: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 830CD334: 480101A5  bl 0x830dd4d8
	ctx.lr = 0x830CD338;
	sub_830DD4D8(ctx, base);
	// 830CD338: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830CD33C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 830CD340: 480DAE7C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CD348(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830CD348 size=408
    let mut pc: u32 = 0x830CD348;
    'dispatch: loop {
        match pc {
            0x830CD348 => {
    //   block [0x830CD348..0x830CD4E0)
	// 830CD348: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830CD34C: 480DAE1D  bl 0x831a8168
	ctx.lr = 0x830CD350;
	sub_831A8130(ctx, base);
	// 830CD350: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830CD354: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830CD358: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 830CD35C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 830CD360: 409A0014  bne cr6, 0x830cd374
	if !ctx.cr[6].eq {
	pc = 0x830CD374; continue 'dispatch;
	}
	// 830CD364: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 830CD368: 60630057  ori r3, r3, 0x57
	ctx.r[3].u64 = ctx.r[3].u64 | 87;
	// 830CD36C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 830CD370: 480DAE48  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
	// 830CD374: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830CD378: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 830CD37C: 419A0014  beq cr6, 0x830cd390
	if ctx.cr[6].eq {
	pc = 0x830CD390; continue 'dispatch;
	}
	// 830CD380: 3880003B  li r4, 0x3b
	ctx.r[4].s64 = 59;
	// 830CD384: 480E4AF5  bl 0x831b1e78
	ctx.lr = 0x830CD388;
	sub_831B1E78(ctx, base);
	// 830CD388: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830CD38C: 4082FFD8  bne 0x830cd364
	if !ctx.cr[0].eq {
	pc = 0x830CD364; continue 'dispatch;
	}
	// 830CD390: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830CD394: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 830CD398: 419A0014  beq cr6, 0x830cd3ac
	if ctx.cr[6].eq {
	pc = 0x830CD3AC; continue 'dispatch;
	}
	// 830CD39C: 3880003B  li r4, 0x3b
	ctx.r[4].s64 = 59;
	// 830CD3A0: 480E4AD9  bl 0x831b1e78
	ctx.lr = 0x830CD3A4;
	sub_831B1E78(ctx, base);
	// 830CD3A4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830CD3A8: 4082FFBC  bne 0x830cd364
	if !ctx.cr[0].eq {
	pc = 0x830CD364; continue 'dispatch;
	}
	// 830CD3AC: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 830CD3B0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 830CD3B4: 419A0014  beq cr6, 0x830cd3c8
	if ctx.cr[6].eq {
	pc = 0x830CD3C8; continue 'dispatch;
	}
	// 830CD3B8: 3880003B  li r4, 0x3b
	ctx.r[4].s64 = 59;
	// 830CD3BC: 480E4ABD  bl 0x831b1e78
	ctx.lr = 0x830CD3C0;
	sub_831B1E78(ctx, base);
	// 830CD3C0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830CD3C4: 4082FFA0  bne 0x830cd364
	if !ctx.cr[0].eq {
	pc = 0x830CD364; continue 'dispatch;
	}
	// 830CD3C8: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830CD3CC: 480106A5  bl 0x830dda70
	ctx.lr = 0x830CD3D0;
	sub_830DDA70(ctx, base);
	// 830CD3D0: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 830CD3D4: 40820010  bne 0x830cd3e4
	if !ctx.cr[0].eq {
	pc = 0x830CD3E4; continue 'dispatch;
	}
	// 830CD3D8: 3C608030  lis r3, -0x7fd0
	ctx.r[3].s64 = -2144337920;
	// 830CD3DC: 6063001F  ori r3, r3, 0x1f
	ctx.r[3].u64 = ctx.r[3].u64 | 31;
	// 830CD3E0: 4BFFFF8C  b 0x830cd36c
	pc = 0x830CD36C; continue 'dispatch;
	// 830CD3E4: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 830CD3E8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 830CD3EC: 419A001C  beq cr6, 0x830cd408
	if ctx.cr[6].eq {
	pc = 0x830CD408; continue 'dispatch;
	}
	// 830CD3F0: A1630000  lhz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830CD3F4: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830CD3F8: 41820010  beq 0x830cd408
	if ctx.cr[0].eq {
	pc = 0x830CD408; continue 'dispatch;
	}
	// 830CD3FC: 48010675  bl 0x830dda70
	ctx.lr = 0x830CD400;
	sub_830DDA70(ctx, base);
	// 830CD400: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 830CD404: 4182FFD4  beq 0x830cd3d8
	if ctx.cr[0].eq {
	pc = 0x830CD3D8; continue 'dispatch;
	}
	// 830CD408: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830CD40C: 3BABC034  addi r29, r11, -0x3fcc
	ctx.r[29].s64 = ctx.r[11].s64 + -16332;
	// 830CD410: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830CD414: 48175559  bl 0x8324296c
	ctx.lr = 0x830CD418;
	// extern call 0x8324296C → crate::xboxkrnl::RtlEnterCriticalSection
	crate::xboxkrnl::RtlEnterCriticalSection(ctx, base);
	// 830CD418: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830CD41C: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830CD420: 3BCBC054  addi r30, r11, -0x3fac
	ctx.r[30].s64 = ctx.r[11].s64 + -16300;
	// 830CD424: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830CD428: 48010939  bl 0x830ddd60
	ctx.lr = 0x830CD42C;
	sub_830DDD60(ctx, base);
	// 830CD42C: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 830CD430: 419A0020  beq cr6, 0x830cd450
	if ctx.cr[6].eq {
	pc = 0x830CD450; continue 'dispatch;
	}
	// 830CD434: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830CD438: 546A103A  slwi r10, r3, 2
	ctx.r[10].u32 = ctx.r[3].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 830CD43C: 7D6A582E  lwzx r11, r10, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 830CD440: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830CD444: 419A000C  beq cr6, 0x830cd450
	if ctx.cr[6].eq {
	pc = 0x830CD450; continue 'dispatch;
	}
	// 830CD448: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830CD44C: 4BFFFD35  bl 0x830cd180
	ctx.lr = 0x830CD450;
	sub_830CD180(ctx, base);
	// 830CD450: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830CD454: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 830CD458: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830CD45C: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 830CD460: 4BFFFA31  bl 0x830cce90
	ctx.lr = 0x830CD464;
	sub_830CCE90(ctx, base);
	// 830CD464: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 830CD468: 40800018  bge 0x830cd480
	if !ctx.cr[0].lt {
	pc = 0x830CD480; continue 'dispatch;
	}
	// 830CD46C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830CD470: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830CD474: 481754E9  bl 0x8324295c
	ctx.lr = 0x830CD478;
	// extern call 0x8324295C → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 830CD478: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830CD47C: 4BFFFEF0  b 0x830cd36c
	pc = 0x830CD36C; continue 'dispatch;
	// 830CD480: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830CD484: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830CD488: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 830CD48C: 4BFFFDBD  bl 0x830cd248
	ctx.lr = 0x830CD490;
	sub_830CD248(ctx, base);
	// 830CD490: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 830CD494: 40820018  bne 0x830cd4ac
	if !ctx.cr[0].eq {
	pc = 0x830CD4AC; continue 'dispatch;
	}
	// 830CD498: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 830CD49C: 4BFFF7BD  bl 0x830ccc58
	ctx.lr = 0x830CD4A0;
	sub_830CCC58(ctx, base);
	// 830CD4A0: 3FE08007  lis r31, -0x7ff9
	ctx.r[31].s64 = -2147024896;
	// 830CD4A4: 63FF000E  ori r31, r31, 0xe
	ctx.r[31].u64 = ctx.r[31].u64 | 14;
	// 830CD4A8: 4BFFFFC8  b 0x830cd470
	pc = 0x830CD470; continue 'dispatch;
	// 830CD4AC: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 830CD4B0: 419A0028  beq cr6, 0x830cd4d8
	if ctx.cr[6].eq {
	pc = 0x830CD4D8; continue 'dispatch;
	}
	// 830CD4B4: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830CD4B8: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830CD4BC: 386BC050  addi r3, r11, -0x3fb0
	ctx.r[3].s64 = ctx.r[11].s64 + -16304;
	// 830CD4C0: 480017C9  bl 0x830cec88
	ctx.lr = 0x830CD4C4;
	sub_830CEC88(ctx, base);
	// 830CD4C4: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 830CD4C8: 40800010  bge 0x830cd4d8
	if !ctx.cr[0].lt {
	pc = 0x830CD4D8; continue 'dispatch;
	}
	// 830CD4CC: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830CD4D0: 4BFFFCB1  bl 0x830cd180
	ctx.lr = 0x830CD4D4;
	sub_830CD180(ctx, base);
	// 830CD4D4: 4BFFFFCC  b 0x830cd4a0
	pc = 0x830CD4A0; continue 'dispatch;
	// 830CD4D8: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 830CD4DC: 4BFFFF94  b 0x830cd470
	pc = 0x830CD470; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CD4E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830CD4E0 size=88
    let mut pc: u32 = 0x830CD4E0;
    'dispatch: loop {
        match pc {
            0x830CD4E0 => {
    //   block [0x830CD4E0..0x830CD538)
	// 830CD4E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830CD4E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830CD4E8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830CD4EC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830CD4F0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830CD4F4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830CD4F8: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 830CD4FC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 830CD500: 419A0018  beq cr6, 0x830cd518
	if ctx.cr[6].eq {
	pc = 0x830CD518; continue 'dispatch;
	}
	// 830CD504: 83E30004  lwz r31, 4(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 830CD508: 4800FFD1  bl 0x830dd4d8
	ctx.lr = 0x830CD50C;
	sub_830DD4D8(ctx, base);
	// 830CD50C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830CD510: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 830CD514: 409AFFF0  bne cr6, 0x830cd504
	if !ctx.cr[6].eq {
	pc = 0x830CD504; continue 'dispatch;
	}
	// 830CD518: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830CD51C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830CD520: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830CD524: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830CD528: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830CD52C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830CD530: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830CD534: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CD538(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830CD538 size=12
    let mut pc: u32 = 0x830CD538;
    'dispatch: loop {
        match pc {
            0x830CD538 => {
    //   block [0x830CD538..0x830CD544)
	// 830CD538: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 830CD53C: 38850004  addi r4, r5, 4
	ctx.r[4].s64 = ctx.r[5].s64 + 4;
	// 830CD540: 48012560  b 0x830dfaa0
	sub_830DFAA0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CD548(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830CD548 size=92
    let mut pc: u32 = 0x830CD548;
    'dispatch: loop {
        match pc {
            0x830CD548 => {
    //   block [0x830CD548..0x830CD5A4)
	// 830CD548: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830CD54C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830CD550: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830CD554: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830CD558: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 830CD55C: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 830CD560: 48017EC1  bl 0x830e5420
	ctx.lr = 0x830CD564;
	sub_830E5420(ctx, base);
	// 830CD564: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830CD568: 41820020  beq 0x830cd588
	if ctx.cr[0].eq {
	pc = 0x830CD588; continue 'dispatch;
	}
	// 830CD56C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830CD570: 2F0B0007  cmpwi cr6, r11, 7
	ctx.cr[6].compare_i32(ctx.r[11].s32, 7, &mut ctx.xer);
	// 830CD574: 409A0014  bne cr6, 0x830cd588
	if !ctx.cr[6].eq {
	pc = 0x830CD588; continue 'dispatch;
	}
	// 830CD578: 389F0004  addi r4, r31, 4
	ctx.r[4].s64 = ctx.r[31].s64 + 4;
	// 830CD57C: 48017D55  bl 0x830e52d0
	ctx.lr = 0x830CD580;
	sub_830E52D0(ctx, base);
	// 830CD580: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830CD584: 4800000C  b 0x830cd590
	pc = 0x830CD590; continue 'dispatch;
	// 830CD588: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 830CD58C: 60630057  ori r3, r3, 0x57
	ctx.r[3].u64 = ctx.r[3].u64 | 87;
	// 830CD590: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830CD594: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830CD598: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830CD59C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830CD5A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CD5A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830CD5A8 size=12
    let mut pc: u32 = 0x830CD5A8;
    'dispatch: loop {
        match pc {
            0x830CD5A8 => {
    //   block [0x830CD5A8..0x830CD5B4)
	// 830CD5A8: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 830CD5AC: 38850004  addi r4, r5, 4
	ctx.r[4].s64 = ctx.r[5].s64 + 4;
	// 830CD5B0: 480125E0  b 0x830dfb90
	sub_830DFB90(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CD5B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830CD5B8 size=12
    let mut pc: u32 = 0x830CD5B8;
    'dispatch: loop {
        match pc {
            0x830CD5B8 => {
    //   block [0x830CD5B8..0x830CD5C4)
	// 830CD5B8: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 830CD5BC: 38850004  addi r4, r5, 4
	ctx.r[4].s64 = ctx.r[5].s64 + 4;
	// 830CD5C0: 48012620  b 0x830dfbe0
	sub_830DFBE0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CD5C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830CD5C8 size=124
    let mut pc: u32 = 0x830CD5C8;
    'dispatch: loop {
        match pc {
            0x830CD5C8 => {
    //   block [0x830CD5C8..0x830CD644)
	// 830CD5C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830CD5CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830CD5D0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830CD5D4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830CD5D8: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 830CD5DC: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 830CD5E0: 48017E41  bl 0x830e5420
	ctx.lr = 0x830CD5E4;
	sub_830E5420(ctx, base);
	// 830CD5E4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830CD5E8: 41820040  beq 0x830cd628
	if ctx.cr[0].eq {
	pc = 0x830CD628; continue 'dispatch;
	}
	// 830CD5EC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830CD5F0: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 830CD5F4: 409A0034  bne cr6, 0x830cd628
	if !ctx.cr[6].eq {
	pc = 0x830CD628; continue 'dispatch;
	}
	// 830CD5F8: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830CD5FC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830CD600: 816300F0  lwz r11, 0xf0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(240 as u32) ) } as u64;
	// 830CD604: 419A0010  beq cr6, 0x830cd614
	if ctx.cr[6].eq {
	pc = 0x830CD614; continue 'dispatch;
	}
	// 830CD608: 616B4000  ori r11, r11, 0x4000
	ctx.r[11].u64 = ctx.r[11].u64 | 16384;
	// 830CD60C: 916300F0  stw r11, 0xf0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(240 as u32), ctx.r[11].u32 ) };
	// 830CD610: 48000010  b 0x830cd620
	pc = 0x830CD620; continue 'dispatch;
	// 830CD614: 556B04A0  rlwinm r11, r11, 0, 0x12, 0x10
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 830CD618: 916300F0  stw r11, 0xf0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(240 as u32), ctx.r[11].u32 ) };
	// 830CD61C: 48028585  bl 0x830f5ba0
	ctx.lr = 0x830CD620;
	sub_830F5BA0(ctx, base);
	// 830CD620: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830CD624: 4800000C  b 0x830cd630
	pc = 0x830CD630; continue 'dispatch;
	// 830CD628: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 830CD62C: 60630057  ori r3, r3, 0x57
	ctx.r[3].u64 = ctx.r[3].u64 | 87;
	// 830CD630: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830CD634: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830CD638: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830CD63C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830CD640: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CD648(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830CD648 size=96
    let mut pc: u32 = 0x830CD648;
    'dispatch: loop {
        match pc {
            0x830CD648 => {
    //   block [0x830CD648..0x830CD6A8)
	// 830CD648: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830CD64C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830CD650: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830CD654: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830CD658: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 830CD65C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830CD660: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 830CD664: 419A0010  beq cr6, 0x830cd674
	if ctx.cr[6].eq {
	pc = 0x830CD674; continue 'dispatch;
	}
	// 830CD668: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 830CD66C: 60630057  ori r3, r3, 0x57
	ctx.r[3].u64 = ctx.r[3].u64 | 87;
	// 830CD670: 48000024  b 0x830cd694
	pc = 0x830CD694; continue 'dispatch;
	// 830CD674: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 830CD678: 48017DA9  bl 0x830e5420
	ctx.lr = 0x830CD67C;
	sub_830E5420(ctx, base);
	// 830CD67C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830CD680: 41820010  beq 0x830cd690
	if ctx.cr[0].eq {
	pc = 0x830CD690; continue 'dispatch;
	}
	// 830CD684: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830CD688: 916300E8  stw r11, 0xe8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(232 as u32), ctx.r[11].u32 ) };
	// 830CD68C: 48023305  bl 0x830f0990
	ctx.lr = 0x830CD690;
	sub_830F0990(ctx, base);
	// 830CD690: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830CD694: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830CD698: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830CD69C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830CD6A0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830CD6A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CD6A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x830CD6A8 size=136
    let mut pc: u32 = 0x830CD6A8;
    'dispatch: loop {
        match pc {
            0x830CD6A8 => {
    //   block [0x830CD6A8..0x830CD730)
	// 830CD6A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830CD6AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830CD6B0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830CD6B4: DBE1FFE8  stfd f31, -0x18(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.f[31].u64 ) };
	// 830CD6B8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830CD6BC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830CD6C0: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 830CD6C4: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 830CD6C8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830CD6CC: 419A0044  beq cr6, 0x830cd710
	if ctx.cr[6].eq {
	pc = 0x830CD710; continue 'dispatch;
	}
	// 830CD6D0: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 830CD6D4: 80BF0014  lwz r5, 0x14(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 830CD6D8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 830CD6DC: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 830CD6E0: 4803D059  bl 0x8310a738
	ctx.lr = 0x830CD6E4;
	sub_8310A738(ctx, base);
	// 830CD6E4: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 830CD6E8: 41800020  blt 0x830cd708
	if ctx.cr[0].lt {
	pc = 0x830CD708; continue 'dispatch;
	}
	// 830CD6EC: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 830CD6F0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 830CD6F4: 419A0008  beq cr6, 0x830cd6fc
	if ctx.cr[6].eq {
	pc = 0x830CD6FC; continue 'dispatch;
	}
	// 830CD6F8: 4803C9D9  bl 0x8310a0d0
	ctx.lr = 0x830CD6FC;
	sub_8310A0D0(ctx, base);
	// 830CD6FC: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 830CD700: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830CD704: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 830CD708: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 830CD70C: 4198000C  blt cr6, 0x830cd718
	if ctx.cr[6].lt {
	pc = 0x830CD718; continue 'dispatch;
	}
	// 830CD710: D3FF0008  stfs f31, 8(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 830CD714: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830CD718: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830CD71C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830CD720: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830CD724: CBE1FFE8  lfd f31, -0x18(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830CD728: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830CD72C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CD730(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x830CD730 size=156
    let mut pc: u32 = 0x830CD730;
    'dispatch: loop {
        match pc {
            0x830CD730 => {
    //   block [0x830CD730..0x830CD7CC)
	// 830CD730: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830CD734: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830CD738: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830CD73C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830CD740: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830CD744: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830CD748: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 830CD74C: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 830CD750: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830CD754: 419A0058  beq cr6, 0x830cd7ac
	if ctx.cr[6].eq {
	pc = 0x830CD7AC; continue 'dispatch;
	}
	// 830CD758: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 830CD75C: 7D6BF278  xor r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 ^ ctx.r[30].u64;
	// 830CD760: 556B073D  rlwinm. r11, r11, 0, 0x1c, 0x1e
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830CD764: 41820048  beq 0x830cd7ac
	if ctx.cr[0].eq {
	pc = 0x830CD7AC; continue 'dispatch;
	}
	// 830CD768: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 830CD76C: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 830CD770: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 830CD774: C03F0008  lfs f1, 8(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 830CD778: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 830CD77C: 4803CFBD  bl 0x8310a738
	ctx.lr = 0x830CD780;
	sub_8310A738(ctx, base);
	// 830CD780: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 830CD784: 41800020  blt 0x830cd7a4
	if ctx.cr[0].lt {
	pc = 0x830CD7A4; continue 'dispatch;
	}
	// 830CD788: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 830CD78C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 830CD790: 419A0008  beq cr6, 0x830cd798
	if ctx.cr[6].eq {
	pc = 0x830CD798; continue 'dispatch;
	}
	// 830CD794: 4803C93D  bl 0x8310a0d0
	ctx.lr = 0x830CD798;
	sub_8310A0D0(ctx, base);
	// 830CD798: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 830CD79C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830CD7A0: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 830CD7A4: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 830CD7A8: 4198000C  blt cr6, 0x830cd7b4
	if ctx.cr[6].lt {
	pc = 0x830CD7B4; continue 'dispatch;
	}
	// 830CD7AC: 93DF0014  stw r30, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[30].u32 ) };
	// 830CD7B0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830CD7B4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830CD7B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830CD7BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830CD7C0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830CD7C4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830CD7C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CD7D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x830CD7D0 size=296
    let mut pc: u32 = 0x830CD7D0;
    'dispatch: loop {
        match pc {
            0x830CD7D0 => {
    //   block [0x830CD7D0..0x830CD8F8)
	// 830CD7D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830CD7D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830CD7D8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830CD7DC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830CD7E0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830CD7E4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 830CD7E8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830CD7EC: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830CD7F0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830CD7F4: 419A0084  beq cr6, 0x830cd878
	if ctx.cr[6].eq {
	pc = 0x830CD878; continue 'dispatch;
	}
	// 830CD7F8: 2B0B0028  cmplwi cr6, r11, 0x28
	ctx.cr[6].compare_u32(ctx.r[11].u32, 40 as u32, &mut ctx.xer);
	// 830CD7FC: 419A0048  beq cr6, 0x830cd844
	if ctx.cr[6].eq {
	pc = 0x830CD844; continue 'dispatch;
	}
	// 830CD800: 2B0B0029  cmplwi cr6, r11, 0x29
	ctx.cr[6].compare_u32(ctx.r[11].u32, 41 as u32, &mut ctx.xer);
	// 830CD804: 409A006C  bne cr6, 0x830cd870
	if !ctx.cr[6].eq {
	pc = 0x830CD870; continue 'dispatch;
	}
	// 830CD808: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 830CD80C: 38FF0014  addi r7, r31, 0x14
	ctx.r[7].s64 = ctx.r[31].s64 + 20;
	// 830CD810: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830CD814: 409A005C  bne cr6, 0x830cd870
	if !ctx.cr[6].eq {
	pc = 0x830CD870; continue 'dispatch;
	}
	// 830CD818: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 830CD81C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830CD820: 419A0050  beq cr6, 0x830cd870
	if ctx.cr[6].eq {
	pc = 0x830CD870; continue 'dispatch;
	}
	// 830CD824: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 830CD828: 80BF0018  lwz r5, 0x18(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 830CD82C: C03F000C  lfs f1, 0xc(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 830CD830: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 830CD834: 4803CF05  bl 0x8310a738
	ctx.lr = 0x830CD838;
	sub_8310A738(ctx, base);
	// 830CD838: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 830CD83C: 40800034  bge 0x830cd870
	if !ctx.cr[0].lt {
	pc = 0x830CD870; continue 'dispatch;
	}
	// 830CD840: 480000A0  b 0x830cd8e0
	pc = 0x830CD8E0; continue 'dispatch;
	// 830CD844: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 830CD848: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830CD84C: 556B077A  rlwinm r11, r11, 0, 0x1d, 0x1d
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 830CD850: 2B0B0004  cmplwi cr6, r11, 4
	ctx.cr[6].compare_u32(ctx.r[11].u32, 4 as u32, &mut ctx.xer);
	// 830CD854: 409A001C  bne cr6, 0x830cd870
	if !ctx.cr[6].eq {
	pc = 0x830CD870; continue 'dispatch;
	}
	// 830CD858: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 830CD85C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 830CD860: 419A0010  beq cr6, 0x830cd870
	if ctx.cr[6].eq {
	pc = 0x830CD870; continue 'dispatch;
	}
	// 830CD864: 4803C86D  bl 0x8310a0d0
	ctx.lr = 0x830CD868;
	sub_8310A0D0(ctx, base);
	// 830CD868: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830CD86C: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 830CD870: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830CD874: 4800006C  b 0x830cd8e0
	pc = 0x830CD8E0; continue 'dispatch;
	// 830CD878: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 830CD87C: 38FF0014  addi r7, r31, 0x14
	ctx.r[7].s64 = ctx.r[31].s64 + 20;
	// 830CD880: 917E0008  stw r11, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 830CD884: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 830CD888: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830CD88C: 409A002C  bne cr6, 0x830cd8b8
	if !ctx.cr[6].eq {
	pc = 0x830CD8B8; continue 'dispatch;
	}
	// 830CD890: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 830CD894: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830CD898: 419A0020  beq cr6, 0x830cd8b8
	if ctx.cr[6].eq {
	pc = 0x830CD8B8; continue 'dispatch;
	}
	// 830CD89C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 830CD8A0: 80BF0018  lwz r5, 0x18(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 830CD8A4: C03F000C  lfs f1, 0xc(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 830CD8A8: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 830CD8AC: 4803CE8D  bl 0x8310a738
	ctx.lr = 0x830CD8B0;
	sub_8310A738(ctx, base);
	// 830CD8B0: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 830CD8B4: 4180002C  blt 0x830cd8e0
	if ctx.cr[0].lt {
	pc = 0x830CD8E0; continue 'dispatch;
	}
	// 830CD8B8: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 830CD8BC: 815E0010  lwz r10, 0x10(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 830CD8C0: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 830CD8C4: 80DF0018  lwz r6, 0x18(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 830CD8C8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 830CD8CC: 80BF0020  lwz r5, 0x20(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 830CD8D0: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 830CD8D4: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830CD8D8: C02B08A4  lfs f1, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 830CD8DC: 4801DBB5  bl 0x830eb490
	ctx.lr = 0x830CD8E0;
	sub_830EB490(ctx, base);
	// 830CD8E0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830CD8E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830CD8E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830CD8EC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830CD8F0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830CD8F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CD8F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830CD8F8 size=76
    let mut pc: u32 = 0x830CD8F8;
    'dispatch: loop {
        match pc {
            0x830CD8F8 => {
    //   block [0x830CD8F8..0x830CD944)
	// 830CD8F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830CD8FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830CD900: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830CD904: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830CD908: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830CD90C: 3D608218  lis r11, -0x7de8
	ctx.r[11].s64 = -2112356352;
	// 830CD910: 396B7BB8  addi r11, r11, 0x7bb8
	ctx.r[11].s64 = ctx.r[11].s64 + 31672;
	// 830CD914: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 830CD918: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830CD91C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 830CD920: 419A0010  beq cr6, 0x830cd930
	if ctx.cr[6].eq {
	pc = 0x830CD930; continue 'dispatch;
	}
	// 830CD924: 48042B3D  bl 0x83110460
	ctx.lr = 0x830CD928;
	sub_83110460(ctx, base);
	// 830CD928: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830CD92C: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 830CD930: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830CD934: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830CD938: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830CD93C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830CD940: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CD948(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830CD948 size=160
    let mut pc: u32 = 0x830CD948;
    'dispatch: loop {
        match pc {
            0x830CD948 => {
    //   block [0x830CD948..0x830CD9E8)
	// 830CD948: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830CD94C: 480DA819  bl 0x831a8164
	ctx.lr = 0x830CD950;
	sub_831A8130(ctx, base);
	// 830CD950: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830CD954: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830CD958: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 830CD95C: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 830CD960: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 830CD964: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830CD968: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830CD96C: 4E800421  bctrl
	ctx.lr = 0x830CD970;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830CD970: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 830CD974: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 830CD978: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830CD97C: 419A0044  beq cr6, 0x830cd9c0
	if ctx.cr[6].eq {
	pc = 0x830CD9C0; continue 'dispatch;
	}
	// 830CD980: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 830CD984: 419A003C  beq cr6, 0x830cd9c0
	if ctx.cr[6].eq {
	pc = 0x830CD9C0; continue 'dispatch;
	}
	// 830CD988: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 830CD98C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830CD990: 419A0030  beq cr6, 0x830cd9c0
	if ctx.cr[6].eq {
	pc = 0x830CD9C0; continue 'dispatch;
	}
	// 830CD994: 3BDD0010  addi r30, r29, 0x10
	ctx.r[30].s64 = ctx.r[29].s64 + 16;
	// 830CD998: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 830CD99C: 809E0000  lwz r4, 0(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 830CD9A0: 480E4491  bl 0x831b1e30
	ctx.lr = 0x830CD9A4;
	sub_831B1E30(ctx, base);
	// 830CD9A4: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 830CD9A8: 41820028  beq 0x830cd9d0
	if ctx.cr[0].eq {
	pc = 0x830CD9D0; continue 'dispatch;
	}
	// 830CD9AC: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 830CD9B0: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 830CD9B4: 3BDE0030  addi r30, r30, 0x30
	ctx.r[30].s64 = ctx.r[30].s64 + 48;
	// 830CD9B8: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 830CD9BC: 4198FFDC  blt cr6, 0x830cd998
	if ctx.cr[6].lt {
	pc = 0x830CD998; continue 'dispatch;
	}
	// 830CD9C0: 3C608030  lis r3, -0x7fd0
	ctx.r[3].s64 = -2144337920;
	// 830CD9C4: 60630007  ori r3, r3, 7
	ctx.r[3].u64 = ctx.r[3].u64 | 7;
	// 830CD9C8: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 830CD9CC: 480DA7E8  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
	// 830CD9D0: 1D7F0030  mulli r11, r31, 0x30
	ctx.r[11].s64 = ctx.r[31].s64 * 48;
	// 830CD9D4: 7D6BEA14  add r11, r11, r29
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 830CD9D8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830CD9DC: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830CD9E0: 917B0000  stw r11, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830CD9E4: 4BFFFFE4  b 0x830cd9c8
	pc = 0x830CD9C8; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CD9E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830CD9E8 size=160
    let mut pc: u32 = 0x830CD9E8;
    'dispatch: loop {
        match pc {
            0x830CD9E8 => {
    //   block [0x830CD9E8..0x830CDA88)
	// 830CD9E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830CD9EC: 480DA779  bl 0x831a8164
	ctx.lr = 0x830CD9F0;
	sub_831A8130(ctx, base);
	// 830CD9F0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830CD9F4: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 830CD9F8: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 830CD9FC: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 830CDA00: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 830CDA04: 93FC0000  stw r31, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 830CDA08: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 830CDA0C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830CDA10: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830CDA14: 4E800421  bctrl
	ctx.lr = 0x830CDA18;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830CDA18: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 830CDA1C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 830CDA20: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830CDA24: 419A0040  beq cr6, 0x830cda64
	if ctx.cr[6].eq {
	pc = 0x830CDA64; continue 'dispatch;
	}
	// 830CDA28: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 830CDA2C: 419A0038  beq cr6, 0x830cda64
	if ctx.cr[6].eq {
	pc = 0x830CDA64; continue 'dispatch;
	}
	// 830CDA30: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830CDA34: 419A0030  beq cr6, 0x830cda64
	if ctx.cr[6].eq {
	pc = 0x830CDA64; continue 'dispatch;
	}
	// 830CDA38: 3BDD0010  addi r30, r29, 0x10
	ctx.r[30].s64 = ctx.r[29].s64 + 16;
	// 830CDA3C: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 830CDA40: 809E0000  lwz r4, 0(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 830CDA44: 480E43ED  bl 0x831b1e30
	ctx.lr = 0x830CDA48;
	sub_831B1E30(ctx, base);
	// 830CDA48: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 830CDA4C: 41820028  beq 0x830cda74
	if ctx.cr[0].eq {
	pc = 0x830CDA74; continue 'dispatch;
	}
	// 830CDA50: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 830CDA54: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 830CDA58: 3BDE0030  addi r30, r30, 0x30
	ctx.r[30].s64 = ctx.r[30].s64 + 48;
	// 830CDA5C: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 830CDA60: 4198FFDC  blt cr6, 0x830cda3c
	if ctx.cr[6].lt {
	pc = 0x830CDA3C; continue 'dispatch;
	}
	// 830CDA64: 3C608030  lis r3, -0x7fd0
	ctx.r[3].s64 = -2144337920;
	// 830CDA68: 60630007  ori r3, r3, 7
	ctx.r[3].u64 = ctx.r[3].u64 | 7;
	// 830CDA6C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 830CDA70: 480DA744  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
	// 830CDA74: 1D7F0030  mulli r11, r31, 0x30
	ctx.r[11].s64 = ctx.r[31].s64 * 48;
	// 830CDA78: 7D6BEA14  add r11, r11, r29
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 830CDA7C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830CDA80: 917C0000  stw r11, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830CDA84: 4BFFFFE8  b 0x830cda6c
	pc = 0x830CDA6C; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CDA88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830CDA88 size=132
    let mut pc: u32 = 0x830CDA88;
    'dispatch: loop {
        match pc {
            0x830CDA88 => {
    //   block [0x830CDA88..0x830CDB0C)
	// 830CDA88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830CDA8C: 480DA6E1  bl 0x831a816c
	ctx.lr = 0x830CDA90;
	sub_831A8130(ctx, base);
	// 830CDA90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830CDA94: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830CDA98: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 830CDA9C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 830CDAA0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 830CDAA4: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 830CDAA8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830CDAAC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830CDAB0: 4E800421  bctrl
	ctx.lr = 0x830CDAB4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830CDAB4: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 830CDAB8: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830CDABC: 7F1F5040  cmplw cr6, r31, r10
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[10].u32, &mut ctx.xer);
	// 830CDAC0: 4198003C  blt cr6, 0x830cdafc
	if ctx.cr[6].lt {
	pc = 0x830CDAFC; continue 'dispatch;
	}
	// 830CDAC4: 81210050  lwz r9, 0x50(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 830CDAC8: 1D290030  mulli r9, r9, 0x30
	ctx.r[9].s64 = ctx.r[9].s64 * 48;
	// 830CDACC: 7D295A14  add r9, r9, r11
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 830CDAD0: 8129FFD4  lwz r9, -0x2c(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-44 as u32) ) } as u64;
	// 830CDAD4: 7F1F4840  cmplw cr6, r31, r9
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[9].u32, &mut ctx.xer);
	// 830CDAD8: 41990024  bgt cr6, 0x830cdafc
	if ctx.cr[6].gt {
	pc = 0x830CDAFC; continue 'dispatch;
	}
	// 830CDADC: 7D4AF850  subf r10, r10, r31
	ctx.r[10].s64 = ctx.r[31].s64 - ctx.r[10].s64;
	// 830CDAE0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830CDAE4: 1D4A0030  mulli r10, r10, 0x30
	ctx.r[10].s64 = ctx.r[10].s64 * 48;
	// 830CDAE8: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 830CDAEC: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 830CDAF0: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 830CDAF4: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830CDAF8: 4800000C  b 0x830cdb04
	pc = 0x830CDB04; continue 'dispatch;
	// 830CDAFC: 3C608030  lis r3, -0x7fd0
	ctx.r[3].s64 = -2144337920;
	// 830CDB00: 60630009  ori r3, r3, 9
	ctx.r[3].u64 = ctx.r[3].u64 | 9;
	// 830CDB04: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 830CDB08: 480DA6B4  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CDB10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830CDB10 size=176
    let mut pc: u32 = 0x830CDB10;
    'dispatch: loop {
        match pc {
            0x830CDB10 => {
    //   block [0x830CDB10..0x830CDBC0)
	// 830CDB10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830CDB14: 480DA659  bl 0x831a816c
	ctx.lr = 0x830CDB18;
	sub_831A8130(ctx, base);
	// 830CDB18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830CDB1C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830CDB20: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 830CDB24: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 830CDB28: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 830CDB2C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 830CDB30: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830CDB34: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830CDB38: 4E800421  bctrl
	ctx.lr = 0x830CDB3C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830CDB3C: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 830CDB40: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 830CDB44: 4198006C  blt cr6, 0x830cdbb0
	if ctx.cr[6].lt {
	pc = 0x830CDBB0; continue 'dispatch;
	}
	// 830CDB48: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 830CDB4C: 1D4A0030  mulli r10, r10, 0x30
	ctx.r[10].s64 = ctx.r[10].s64 * 48;
	// 830CDB50: 7D4A1A14  add r10, r10, r3
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[3].u64;
	// 830CDB54: 814AFFD4  lwz r10, -0x2c(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-44 as u32) ) } as u64;
	// 830CDB58: 7F1F5040  cmplw cr6, r31, r10
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[10].u32, &mut ctx.xer);
	// 830CDB5C: 41990054  bgt cr6, 0x830cdbb0
	if ctx.cr[6].gt {
	pc = 0x830CDBB0; continue 'dispatch;
	}
	// 830CDB60: 7D6BF850  subf r11, r11, r31
	ctx.r[11].s64 = ctx.r[31].s64 - ctx.r[11].s64;
	// 830CDB64: 1D6B0030  mulli r11, r11, 0x30
	ctx.r[11].s64 = ctx.r[11].s64 * 48;
	// 830CDB68: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 830CDB6C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830CDB70: 554A07FF  clrlwi. r10, r10, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 830CDB74: 40820010  bne 0x830cdb84
	if !ctx.cr[0].eq {
	pc = 0x830CDB84; continue 'dispatch;
	}
	// 830CDB78: 3C608030  lis r3, -0x7fd0
	ctx.r[3].s64 = -2144337920;
	// 830CDB7C: 60630001  ori r3, r3, 1
	ctx.r[3].u64 = ctx.r[3].u64 | 1;
	// 830CDB80: 48000038  b 0x830cdbb8
	pc = 0x830CDBB8; continue 'dispatch;
	// 830CDB84: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 830CDB88: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830CDB8C: 409A0010  bne cr6, 0x830cdb9c
	if !ctx.cr[6].eq {
	pc = 0x830CDB9C; continue 'dispatch;
	}
	// 830CDB90: 3C608030  lis r3, -0x7fd0
	ctx.r[3].s64 = -2144337920;
	// 830CDB94: 60630002  ori r3, r3, 2
	ctx.r[3].u64 = ctx.r[3].u64 | 2;
	// 830CDB98: 48000020  b 0x830cdbb8
	pc = 0x830CDBB8; continue 'dispatch;
	// 830CDB9C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 830CDBA0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830CDBA4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 830CDBA8: 4E800421  bctrl
	ctx.lr = 0x830CDBAC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 830CDBAC: 4800000C  b 0x830cdbb8
	pc = 0x830CDBB8; continue 'dispatch;
	// 830CDBB0: 3C608030  lis r3, -0x7fd0
	ctx.r[3].s64 = -2144337920;
	// 830CDBB4: 60630009  ori r3, r3, 9
	ctx.r[3].u64 = ctx.r[3].u64 | 9;
	// 830CDBB8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 830CDBBC: 480DA600  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CDBC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x830CDBC0 size=48
    let mut pc: u32 = 0x830CDBC0;
    'dispatch: loop {
        match pc {
            0x830CDBC0 => {
    //   block [0x830CDBC0..0x830CDBF0)
	// 830CDBC0: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 830CDBC4: 419A002C  beq cr6, 0x830cdbf0
	if ctx.cr[6].eq {
		sub_830CDBF0(ctx, base);
		return;
	}
	// 830CDBC8: 81650000  lwz r11, 0(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 830CDBCC: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 830CDBD0: 409A0020  bne cr6, 0x830cdbf0
	if !ctx.cr[6].eq {
		sub_830CDBF0(ctx, base);
		return;
	}
	// 830CDBD4: 8164000C  lwz r11, 0xc(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(12 as u32) ) } as u64;
	// 830CDBD8: C0050004  lfs f0, 4(r5)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 830CDBDC: D0040004  stfs f0, 4(r4)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 830CDBE0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830CDBE4: 616B0001  ori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 | 1;
	// 830CDBE8: 9164000C  stw r11, 0xc(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 830CDBEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CDBF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830CDBF0 size=12
    let mut pc: u32 = 0x830CDBF0;
    'dispatch: loop {
        match pc {
            0x830CDBF0 => {
    //   block [0x830CDBF0..0x830CDBFC)
	// 830CDBF0: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 830CDBF4: 60630057  ori r3, r3, 0x57
	ctx.r[3].u64 = ctx.r[3].u64 | 87;
	// 830CDBF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CDC00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830CDC00 size=48
    let mut pc: u32 = 0x830CDC00;
    'dispatch: loop {
        match pc {
            0x830CDC00 => {
    //   block [0x830CDC00..0x830CDC30)
	// 830CDC00: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 830CDC04: 419A002C  beq cr6, 0x830cdc30
	if ctx.cr[6].eq {
		sub_830CDC30(ctx, base);
		return;
	}
	// 830CDC08: 81650000  lwz r11, 0(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 830CDC0C: 2F0B0006  cmpwi cr6, r11, 6
	ctx.cr[6].compare_i32(ctx.r[11].s32, 6, &mut ctx.xer);
	// 830CDC10: 409A0020  bne cr6, 0x830cdc30
	if !ctx.cr[6].eq {
		sub_830CDC30(ctx, base);
		return;
	}
	// 830CDC14: 81650004  lwz r11, 4(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) } as u64;
	// 830CDC18: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830CDC1C: 8144000C  lwz r10, 0xc(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(12 as u32) ) } as u64;
	// 830CDC20: 614A0001  ori r10, r10, 1
	ctx.r[10].u64 = ctx.r[10].u64 | 1;
	// 830CDC24: 91640008  stw r11, 8(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 830CDC28: 9144000C  stw r10, 0xc(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 830CDC2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CDC30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830CDC30 size=12
    let mut pc: u32 = 0x830CDC30;
    'dispatch: loop {
        match pc {
            0x830CDC30 => {
    //   block [0x830CDC30..0x830CDC3C)
	// 830CDC30: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 830CDC34: 60630057  ori r3, r3, 0x57
	ctx.r[3].u64 = ctx.r[3].u64 | 87;
	// 830CDC38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CDC40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830CDC40 size=20
    let mut pc: u32 = 0x830CDC40;
    'dispatch: loop {
        match pc {
            0x830CDC40 => {
    //   block [0x830CDC40..0x830CDC54)
	// 830CDC40: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 830CDC44: 409A0010  bne cr6, 0x830cdc54
	if !ctx.cr[6].eq {
		sub_830CDC54(ctx, base);
		return;
	}
	// 830CDC48: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 830CDC4C: 60630057  ori r3, r3, 0x57
	ctx.r[3].u64 = ctx.r[3].u64 | 87;
	// 830CDC50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CDC54(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830CDC54 size=16
    let mut pc: u32 = 0x830CDC54;
    'dispatch: loop {
        match pc {
            0x830CDC54 => {
    //   block [0x830CDC54..0x830CDC64)
	// 830CDC54: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 830CDC58: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830CDC5C: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830CDC60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CDC68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830CDC68 size=48
    let mut pc: u32 = 0x830CDC68;
    'dispatch: loop {
        match pc {
            0x830CDC68 => {
    //   block [0x830CDC68..0x830CDC98)
	// 830CDC68: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 830CDC6C: 419A002C  beq cr6, 0x830cdc98
	if ctx.cr[6].eq {
		sub_830CDC98(ctx, base);
		return;
	}
	// 830CDC70: 81650000  lwz r11, 0(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 830CDC74: 2F0B0006  cmpwi cr6, r11, 6
	ctx.cr[6].compare_i32(ctx.r[11].s32, 6, &mut ctx.xer);
	// 830CDC78: 409A0020  bne cr6, 0x830cdc98
	if !ctx.cr[6].eq {
		sub_830CDC98(ctx, base);
		return;
	}
	// 830CDC7C: 81650004  lwz r11, 4(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) } as u64;
	// 830CDC80: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830CDC84: 81440084  lwz r10, 0x84(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(132 as u32) ) } as u64;
	// 830CDC88: 614A0001  ori r10, r10, 1
	ctx.r[10].u64 = ctx.r[10].u64 | 1;
	// 830CDC8C: 91440084  stw r10, 0x84(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(132 as u32), ctx.r[10].u32 ) };
	// 830CDC90: 9164000C  stw r11, 0xc(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 830CDC94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CDC98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830CDC98 size=12
    let mut pc: u32 = 0x830CDC98;
    'dispatch: loop {
        match pc {
            0x830CDC98 => {
    //   block [0x830CDC98..0x830CDCA4)
	// 830CDC98: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 830CDC9C: 60630057  ori r3, r3, 0x57
	ctx.r[3].u64 = ctx.r[3].u64 | 87;
	// 830CDCA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CDCA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x830CDCA8 size=48
    let mut pc: u32 = 0x830CDCA8;
    'dispatch: loop {
        match pc {
            0x830CDCA8 => {
    //   block [0x830CDCA8..0x830CDCD8)
	// 830CDCA8: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 830CDCAC: 419A002C  beq cr6, 0x830cdcd8
	if ctx.cr[6].eq {
		sub_830CDCD8(ctx, base);
		return;
	}
	// 830CDCB0: 81650000  lwz r11, 0(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 830CDCB4: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 830CDCB8: 409A0020  bne cr6, 0x830cdcd8
	if !ctx.cr[6].eq {
		sub_830CDCD8(ctx, base);
		return;
	}
	// 830CDCBC: 81640084  lwz r11, 0x84(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(132 as u32) ) } as u64;
	// 830CDCC0: C0050004  lfs f0, 4(r5)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 830CDCC4: D0040040  stfs f0, 0x40(r4)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(64 as u32), tmp.u32 ) };
	// 830CDCC8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830CDCCC: 616B0002  ori r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u64 | 2;
	// 830CDCD0: 91640084  stw r11, 0x84(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(132 as u32), ctx.r[11].u32 ) };
	// 830CDCD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CDCD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830CDCD8 size=12
    let mut pc: u32 = 0x830CDCD8;
    'dispatch: loop {
        match pc {
            0x830CDCD8 => {
    //   block [0x830CDCD8..0x830CDCE4)
	// 830CDCD8: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 830CDCDC: 60630057  ori r3, r3, 0x57
	ctx.r[3].u64 = ctx.r[3].u64 | 87;
	// 830CDCE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CDCE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830CDCE8 size=60
    let mut pc: u32 = 0x830CDCE8;
    'dispatch: loop {
        match pc {
            0x830CDCE8 => {
    //   block [0x830CDCE8..0x830CDD24)
	// 830CDCE8: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 830CDCEC: 419A0038  beq cr6, 0x830cdd24
	if ctx.cr[6].eq {
		sub_830CDD24(ctx, base);
		return;
	}
	// 830CDCF0: 81650000  lwz r11, 0(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 830CDCF4: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 830CDCF8: 409A002C  bne cr6, 0x830cdd24
	if !ctx.cr[6].eq {
		sub_830CDD24(ctx, base);
		return;
	}
	// 830CDCFC: 81650004  lwz r11, 4(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) } as u64;
	// 830CDD00: 2B0B0003  cmplwi cr6, r11, 3
	ctx.cr[6].compare_u32(ctx.r[11].u32, 3 as u32, &mut ctx.xer);
	// 830CDD04: 41990020  bgt cr6, 0x830cdd24
	if ctx.cr[6].gt {
		sub_830CDD24(ctx, base);
		return;
	}
	// 830CDD08: 81440084  lwz r10, 0x84(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(132 as u32) ) } as u64;
	// 830CDD0C: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 830CDD10: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830CDD14: 554A07B6  rlwinm r10, r10, 0, 0x1e, 0x1b
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 830CDD18: 7D6B5378  or r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 830CDD1C: 91640084  stw r11, 0x84(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(132 as u32), ctx.r[11].u32 ) };
	// 830CDD20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CDD24(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830CDD24 size=12
    let mut pc: u32 = 0x830CDD24;
    'dispatch: loop {
        match pc {
            0x830CDD24 => {
    //   block [0x830CDD24..0x830CDD30)
	// 830CDD24: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 830CDD28: 60630057  ori r3, r3, 0x57
	ctx.r[3].u64 = ctx.r[3].u64 | 87;
	// 830CDD2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CDD30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830CDD30 size=60
    let mut pc: u32 = 0x830CDD30;
    'dispatch: loop {
        match pc {
            0x830CDD30 => {
    //   block [0x830CDD30..0x830CDD6C)
	// 830CDD30: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 830CDD34: 419A0038  beq cr6, 0x830cdd6c
	if ctx.cr[6].eq {
		sub_830CDD6C(ctx, base);
		return;
	}
	// 830CDD38: 81650000  lwz r11, 0(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 830CDD3C: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 830CDD40: 409A002C  bne cr6, 0x830cdd6c
	if !ctx.cr[6].eq {
		sub_830CDD6C(ctx, base);
		return;
	}
	// 830CDD44: 81650004  lwz r11, 4(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) } as u64;
	// 830CDD48: 2B0B0003  cmplwi cr6, r11, 3
	ctx.cr[6].compare_u32(ctx.r[11].u32, 3 as u32, &mut ctx.xer);
	// 830CDD4C: 41990020  bgt cr6, 0x830cdd6c
	if ctx.cr[6].gt {
		sub_830CDD6C(ctx, base);
		return;
	}
	// 830CDD50: 81440084  lwz r10, 0x84(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(132 as u32) ) } as u64;
	// 830CDD54: 556B2036  slwi r11, r11, 4
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 830CDD58: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830CDD5C: 554A0732  rlwinm r10, r10, 0, 0x1c, 0x19
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 830CDD60: 7D6B5378  or r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 830CDD64: 91640084  stw r11, 0x84(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(132 as u32), ctx.r[11].u32 ) };
	// 830CDD68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CDD6C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830CDD6C size=12
    let mut pc: u32 = 0x830CDD6C;
    'dispatch: loop {
        match pc {
            0x830CDD6C => {
    //   block [0x830CDD6C..0x830CDD78)
	// 830CDD6C: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 830CDD70: 60630057  ori r3, r3, 0x57
	ctx.r[3].u64 = ctx.r[3].u64 | 87;
	// 830CDD74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CDD78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830CDD78 size=52
    let mut pc: u32 = 0x830CDD78;
    'dispatch: loop {
        match pc {
            0x830CDD78 => {
    //   block [0x830CDD78..0x830CDDAC)
	// 830CDD78: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 830CDD7C: 419A0030  beq cr6, 0x830cddac
	if ctx.cr[6].eq {
		sub_830CDDAC(ctx, base);
		return;
	}
	// 830CDD80: 81650000  lwz r11, 0(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 830CDD84: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 830CDD88: 409A0024  bne cr6, 0x830cddac
	if !ctx.cr[6].eq {
		sub_830CDDAC(ctx, base);
		return;
	}
	// 830CDD8C: 81650004  lwz r11, 4(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) } as u64;
	// 830CDD90: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830CDD94: 81440084  lwz r10, 0x84(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(132 as u32) ) } as u64;
	// 830CDD98: 556B1032  rlwinm r11, r11, 2, 0, 0x19
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 830CDD9C: 554A06AE  rlwinm r10, r10, 0, 0x1a, 0x17
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 830CDDA0: 7D6B5378  or r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 830CDDA4: 91640084  stw r11, 0x84(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(132 as u32), ctx.r[11].u32 ) };
	// 830CDDA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CDDAC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830CDDAC size=12
    let mut pc: u32 = 0x830CDDAC;
    'dispatch: loop {
        match pc {
            0x830CDDAC => {
    //   block [0x830CDDAC..0x830CDDB8)
	// 830CDDAC: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 830CDDB0: 60630057  ori r3, r3, 0x57
	ctx.r[3].u64 = ctx.r[3].u64 | 87;
	// 830CDDB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CDDB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830CDDB8 size=56
    let mut pc: u32 = 0x830CDDB8;
    'dispatch: loop {
        match pc {
            0x830CDDB8 => {
    //   block [0x830CDDB8..0x830CDDF0)
	// 830CDDB8: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 830CDDBC: 419A0034  beq cr6, 0x830cddf0
	if ctx.cr[6].eq {
		sub_830CDDF0(ctx, base);
		return;
	}
	// 830CDDC0: 81650000  lwz r11, 0(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 830CDDC4: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 830CDDC8: 409A0028  bne cr6, 0x830cddf0
	if !ctx.cr[6].eq {
		sub_830CDDF0(ctx, base);
		return;
	}
	// 830CDDCC: 81650004  lwz r11, 4(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) } as u64;
	// 830CDDD0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830CDDD4: 81440084  lwz r10, 0x84(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(132 as u32) ) } as u64;
	// 830CDDD8: 556B402E  slwi r11, r11, 8
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(8);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 830CDDDC: 554A062A  rlwinm r10, r10, 0, 0x18, 0x15
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 830CDDE0: 7D6B5378  or r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[10].u64;
	// 830CDDE4: 616B0002  ori r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u64 | 2;
	// 830CDDE8: 91640084  stw r11, 0x84(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(132 as u32), ctx.r[11].u32 ) };
	// 830CDDEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CDDF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830CDDF0 size=12
    let mut pc: u32 = 0x830CDDF0;
    'dispatch: loop {
        match pc {
            0x830CDDF0 => {
    //   block [0x830CDDF0..0x830CDDFC)
	// 830CDDF0: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 830CDDF4: 60630057  ori r3, r3, 0x57
	ctx.r[3].u64 = ctx.r[3].u64 | 87;
	// 830CDDF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CDE00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830CDE00 size=120
    let mut pc: u32 = 0x830CDE00;
    'dispatch: loop {
        match pc {
            0x830CDE00 => {
    //   block [0x830CDE00..0x830CDE78)
	// 830CDE00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830CDE04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830CDE08: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830CDE0C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830CDE10: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830CDE14: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830CDE18: 4800F6C1  bl 0x830dd4d8
	ctx.lr = 0x830CDE1C;
	sub_830DD4D8(ctx, base);
	// 830CDE1C: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 830CDE20: 4800F6B9  bl 0x830dd4d8
	ctx.lr = 0x830CDE24;
	sub_830DD4D8(ctx, base);
	// 830CDE24: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 830CDE28: 4800F6B1  bl 0x830dd4d8
	ctx.lr = 0x830CDE2C;
	sub_830DD4D8(ctx, base);
	// 830CDE2C: 807F0018  lwz r3, 0x18(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 830CDE30: 4800F6A9  bl 0x830dd4d8
	ctx.lr = 0x830CDE34;
	sub_830DD4D8(ctx, base);
	// 830CDE34: 807F001C  lwz r3, 0x1c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 830CDE38: 4800F6A1  bl 0x830dd4d8
	ctx.lr = 0x830CDE3C;
	sub_830DD4D8(ctx, base);
	// 830CDE3C: 807F0020  lwz r3, 0x20(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 830CDE40: 4800F699  bl 0x830dd4d8
	ctx.lr = 0x830CDE44;
	sub_830DD4D8(ctx, base);
	// 830CDE44: 807F0024  lwz r3, 0x24(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 830CDE48: 4800F691  bl 0x830dd4d8
	ctx.lr = 0x830CDE4C;
	sub_830DD4D8(ctx, base);
	// 830CDE4C: 807F0028  lwz r3, 0x28(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 830CDE50: 4800F689  bl 0x830dd4d8
	ctx.lr = 0x830CDE54;
	sub_830DD4D8(ctx, base);
	// 830CDE54: 807F002C  lwz r3, 0x2c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 830CDE58: 4800F681  bl 0x830dd4d8
	ctx.lr = 0x830CDE5C;
	sub_830DD4D8(ctx, base);
	// 830CDE5C: 807F0034  lwz r3, 0x34(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 830CDE60: 4800F679  bl 0x830dd4d8
	ctx.lr = 0x830CDE64;
	sub_830DD4D8(ctx, base);
	// 830CDE64: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830CDE68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830CDE6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830CDE70: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830CDE74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CDE78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830CDE78 size=148
    let mut pc: u32 = 0x830CDE78;
    'dispatch: loop {
        match pc {
            0x830CDE78 => {
    //   block [0x830CDE78..0x830CDF0C)
	// 830CDE78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830CDE7C: 480DA2F1  bl 0x831a816c
	ctx.lr = 0x830CDE80;
	sub_831A8130(ctx, base);
	// 830CDE80: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830CDE84: 3D008339  lis r8, -0x7cc7
	ctx.r[8].s64 = -2093416448;
	// 830CDE88: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830CDE8C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 830CDE90: 3BEBC44C  addi r31, r11, -0x3bb4
	ctx.r[31].s64 = ctx.r[11].s64 + -15284;
	// 830CDE94: 8168C47C  lwz r11, -0x3b84(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(-15236 as u32) ) } as u64;
	// 830CDE98: 556A07FF  clrlwi. r10, r11, 0x1f
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 830CDE9C: 4082005C  bne 0x830cdef8
	if !ctx.cr[0].eq {
	pc = 0x830CDEF8; continue 'dispatch;
	}
	// 830CDEA0: 3D408218  lis r10, -0x7de8
	ctx.r[10].s64 = -2112356352;
	// 830CDEA4: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 830CDEA8: 392A7C24  addi r9, r10, 0x7c24
	ctx.r[9].s64 = ctx.r[10].s64 + 31780;
	// 830CDEAC: 616B0001  ori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 | 1;
	// 830CDEB0: 7FCAF378  mr r10, r30
	ctx.r[10].u64 = ctx.r[30].u64;
	// 830CDEB4: 913F0010  stw r9, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[9].u32 ) };
	// 830CDEB8: 9168C47C  stw r11, -0x3b84(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(-15236 as u32), ctx.r[11].u32 ) };
	// 830CDEBC: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 830CDEC0: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 830CDEC4: 7FC9F378  mr r9, r30
	ctx.r[9].u64 = ctx.r[30].u64;
	// 830CDEC8: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 830CDECC: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 830CDED0: 913F000C  stw r9, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 830CDED4: 915F0014  stw r10, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 830CDED8: 480230E9  bl 0x830f0fc0
	ctx.lr = 0x830CDEDC;
	sub_830F0FC0(ctx, base);
	// 830CDEDC: 907F0018  stw r3, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[3].u32 ) };
	// 830CDEE0: 93DF001C  stw r30, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[30].u32 ) };
	// 830CDEE4: 397F001C  addi r11, r31, 0x1c
	ctx.r[11].s64 = ctx.r[31].s64 + 28;
	// 830CDEE8: 93DF0020  stw r30, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[30].u32 ) };
	// 830CDEEC: 93DF0024  stw r30, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[30].u32 ) };
	// 830CDEF0: 93DF0028  stw r30, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[30].u32 ) };
	// 830CDEF4: 93DF002C  stw r30, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[30].u32 ) };
	// 830CDEF8: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 830CDEFC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830CDF00: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830CDF04: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830CDF08: 480DA2B4  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CDF10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830CDF10 size=148
    let mut pc: u32 = 0x830CDF10;
    'dispatch: loop {
        match pc {
            0x830CDF10 => {
    //   block [0x830CDF10..0x830CDFA4)
	// 830CDF10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830CDF14: 480DA259  bl 0x831a816c
	ctx.lr = 0x830CDF18;
	sub_831A8130(ctx, base);
	// 830CDF18: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830CDF1C: 3D008339  lis r8, -0x7cc7
	ctx.r[8].s64 = -2093416448;
	// 830CDF20: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830CDF24: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 830CDF28: 3BEBC480  addi r31, r11, -0x3b80
	ctx.r[31].s64 = ctx.r[11].s64 + -15232;
	// 830CDF2C: 8168C4B0  lwz r11, -0x3b50(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(-15184 as u32) ) } as u64;
	// 830CDF30: 556A07FF  clrlwi. r10, r11, 0x1f
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 830CDF34: 4082005C  bne 0x830cdf90
	if !ctx.cr[0].eq {
	pc = 0x830CDF90; continue 'dispatch;
	}
	// 830CDF38: 3D408218  lis r10, -0x7de8
	ctx.r[10].s64 = -2112356352;
	// 830CDF3C: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 830CDF40: 392A7C24  addi r9, r10, 0x7c24
	ctx.r[9].s64 = ctx.r[10].s64 + 31780;
	// 830CDF44: 616B0001  ori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 | 1;
	// 830CDF48: 7FCAF378  mr r10, r30
	ctx.r[10].u64 = ctx.r[30].u64;
	// 830CDF4C: 913F0010  stw r9, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[9].u32 ) };
	// 830CDF50: 9168C4B0  stw r11, -0x3b50(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(-15184 as u32), ctx.r[11].u32 ) };
	// 830CDF54: 39600008  li r11, 8
	ctx.r[11].s64 = 8;
	// 830CDF58: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 830CDF5C: 7FC9F378  mr r9, r30
	ctx.r[9].u64 = ctx.r[30].u64;
	// 830CDF60: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 830CDF64: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 830CDF68: 913F000C  stw r9, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 830CDF6C: 915F0014  stw r10, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 830CDF70: 48023051  bl 0x830f0fc0
	ctx.lr = 0x830CDF74;
	sub_830F0FC0(ctx, base);
	// 830CDF74: 907F0018  stw r3, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[3].u32 ) };
	// 830CDF78: 93DF001C  stw r30, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[30].u32 ) };
	// 830CDF7C: 397F001C  addi r11, r31, 0x1c
	ctx.r[11].s64 = ctx.r[31].s64 + 28;
	// 830CDF80: 93DF0020  stw r30, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[30].u32 ) };
	// 830CDF84: 93DF0024  stw r30, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[30].u32 ) };
	// 830CDF88: 93DF0028  stw r30, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[30].u32 ) };
	// 830CDF8C: 93DF002C  stw r30, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[30].u32 ) };
	// 830CDF90: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 830CDF94: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830CDF98: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830CDF9C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830CDFA0: 480DA21C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CDFA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830CDFA8 size=148
    let mut pc: u32 = 0x830CDFA8;
    'dispatch: loop {
        match pc {
            0x830CDFA8 => {
    //   block [0x830CDFA8..0x830CE03C)
	// 830CDFA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830CDFAC: 480DA1C1  bl 0x831a816c
	ctx.lr = 0x830CDFB0;
	sub_831A8130(ctx, base);
	// 830CDFB0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830CDFB4: 3D008339  lis r8, -0x7cc7
	ctx.r[8].s64 = -2093416448;
	// 830CDFB8: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830CDFBC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 830CDFC0: 3BEBC4B4  addi r31, r11, -0x3b4c
	ctx.r[31].s64 = ctx.r[11].s64 + -15180;
	// 830CDFC4: 8168C4E4  lwz r11, -0x3b1c(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(-15132 as u32) ) } as u64;
	// 830CDFC8: 556A07FF  clrlwi. r10, r11, 0x1f
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 830CDFCC: 4082005C  bne 0x830ce028
	if !ctx.cr[0].eq {
	pc = 0x830CE028; continue 'dispatch;
	}
	// 830CDFD0: 3D408218  lis r10, -0x7de8
	ctx.r[10].s64 = -2112356352;
	// 830CDFD4: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 830CDFD8: 392A7C24  addi r9, r10, 0x7c24
	ctx.r[9].s64 = ctx.r[10].s64 + 31780;
	// 830CDFDC: 616B0001  ori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 | 1;
	// 830CDFE0: 7FCAF378  mr r10, r30
	ctx.r[10].u64 = ctx.r[30].u64;
	// 830CDFE4: 913F0010  stw r9, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[9].u32 ) };
	// 830CDFE8: 9168C4E4  stw r11, -0x3b1c(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(-15132 as u32), ctx.r[11].u32 ) };
	// 830CDFEC: 39600008  li r11, 8
	ctx.r[11].s64 = 8;
	// 830CDFF0: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 830CDFF4: 7FC9F378  mr r9, r30
	ctx.r[9].u64 = ctx.r[30].u64;
	// 830CDFF8: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 830CDFFC: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 830CE000: 913F000C  stw r9, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 830CE004: 915F0014  stw r10, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 830CE008: 48022FB9  bl 0x830f0fc0
	ctx.lr = 0x830CE00C;
	sub_830F0FC0(ctx, base);
	// 830CE00C: 907F0018  stw r3, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[3].u32 ) };
	// 830CE010: 93DF001C  stw r30, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[30].u32 ) };
	// 830CE014: 397F001C  addi r11, r31, 0x1c
	ctx.r[11].s64 = ctx.r[31].s64 + 28;
	// 830CE018: 93DF0020  stw r30, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[30].u32 ) };
	// 830CE01C: 93DF0024  stw r30, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[30].u32 ) };
	// 830CE020: 93DF0028  stw r30, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[30].u32 ) };
	// 830CE024: 93DF002C  stw r30, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[30].u32 ) };
	// 830CE028: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 830CE02C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830CE030: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830CE034: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830CE038: 480DA184  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CE040(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830CE040 size=116
    let mut pc: u32 = 0x830CE040;
    'dispatch: loop {
        match pc {
            0x830CE040 => {
    //   block [0x830CE040..0x830CE0B4)
	// 830CE040: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830CE044: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830CE048: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830CE04C: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 830CE050: 419A0010  beq cr6, 0x830ce060
	if ctx.cr[6].eq {
	pc = 0x830CE060; continue 'dispatch;
	}
	// 830CE054: 81640008  lwz r11, 8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 830CE058: 616B0001  ori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 | 1;
	// 830CE05C: 91640008  stw r11, 8(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 830CE060: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 830CE064: 419A0040  beq cr6, 0x830ce0a4
	if ctx.cr[6].eq {
	pc = 0x830CE0A4; continue 'dispatch;
	}
	// 830CE068: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830CE06C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 830CE070: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 830CE074: 480116E5  bl 0x830df758
	ctx.lr = 0x830CE078;
	sub_830DF758(ctx, base);
	// 830CE078: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 830CE07C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 830CE080: 419A0024  beq cr6, 0x830ce0a4
	if ctx.cr[6].eq {
	pc = 0x830CE0A4; continue 'dispatch;
	}
	// 830CE084: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830CE088: 808BC06C  lwz r4, -0x3f94(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16276 as u32) ) } as u64;
	// 830CE08C: 48010A5D  bl 0x830deae8
	ctx.lr = 0x830CE090;
	sub_830DEAE8(ctx, base);
	// 830CE090: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830CE094: 41820010  beq 0x830ce0a4
	if ctx.cr[0].eq {
	pc = 0x830CE0A4; continue 'dispatch;
	}
	// 830CE098: 816300F0  lwz r11, 0xf0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(240 as u32) ) } as u64;
	// 830CE09C: 616B8000  ori r11, r11, 0x8000
	ctx.r[11].u64 = ctx.r[11].u64 | 32768;
	// 830CE0A0: 916300F0  stw r11, 0xf0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(240 as u32), ctx.r[11].u32 ) };
	// 830CE0A4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830CE0A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830CE0AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830CE0B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CE0B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830CE0B8 size=76
    let mut pc: u32 = 0x830CE0B8;
    'dispatch: loop {
        match pc {
            0x830CE0B8 => {
    //   block [0x830CE0B8..0x830CE104)
	// 830CE0B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830CE0BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830CE0C0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830CE0C4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830CE0C8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830CE0CC: 38601000  li r3, 0x1000
	ctx.r[3].s64 = 4096;
	// 830CE0D0: 4800F3E1  bl 0x830dd4b0
	ctx.lr = 0x830CE0D4;
	sub_830DD4B0(ctx, base);
	// 830CE0D4: 907F0000  stw r3, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 830CE0D8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830CE0DC: 40820010  bne 0x830ce0ec
	if !ctx.cr[0].eq {
	pc = 0x830CE0EC; continue 'dispatch;
	}
	// 830CE0E0: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 830CE0E4: 6063000E  ori r3, r3, 0xe
	ctx.r[3].u64 = ctx.r[3].u64 | 14;
	// 830CE0E8: 48000008  b 0x830ce0f0
	pc = 0x830CE0F0; continue 'dispatch;
	// 830CE0EC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830CE0F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830CE0F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830CE0F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830CE0FC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830CE100: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CE108(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830CE108 size=176
    let mut pc: u32 = 0x830CE108;
    'dispatch: loop {
        match pc {
            0x830CE108 => {
    //   block [0x830CE108..0x830CE1B8)
	// 830CE108: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830CE10C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830CE110: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830CE114: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830CE118: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830CE11C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830CE120: 386BC034  addi r3, r11, -0x3fcc
	ctx.r[3].s64 = ctx.r[11].s64 + -16332;
	// 830CE124: 481748B9  bl 0x832429dc
	ctx.lr = 0x830CE128;
	// extern call 0x832429DC → crate::xboxkrnl::RtlInitializeCriticalSection
	crate::xboxkrnl::RtlInitializeCriticalSection(ctx, base);
	// 830CE128: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830CE12C: 386BC110  addi r3, r11, -0x3ef0
	ctx.r[3].s64 = ctx.r[11].s64 + -16112;
	// 830CE130: 481748AD  bl 0x832429dc
	ctx.lr = 0x830CE134;
	// extern call 0x832429DC → crate::xboxkrnl::RtlInitializeCriticalSection
	crate::xboxkrnl::RtlInitializeCriticalSection(ctx, base);
	// 830CE134: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830CE138: 386BD7C4  addi r3, r11, -0x283c
	ctx.r[3].s64 = ctx.r[11].s64 + -10300;
	// 830CE13C: 481748A1  bl 0x832429dc
	ctx.lr = 0x830CE140;
	// extern call 0x832429DC → crate::xboxkrnl::RtlInitializeCriticalSection
	crate::xboxkrnl::RtlInitializeCriticalSection(ctx, base);
	// 830CE140: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830CE144: 386BD7A8  addi r3, r11, -0x2858
	ctx.r[3].s64 = ctx.r[11].s64 + -10328;
	// 830CE148: 48174895  bl 0x832429dc
	ctx.lr = 0x830CE14C;
	// extern call 0x832429DC → crate::xboxkrnl::RtlInitializeCriticalSection
	crate::xboxkrnl::RtlInitializeCriticalSection(ctx, base);
	// 830CE14C: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830CE150: 386BC164  addi r3, r11, -0x3e9c
	ctx.r[3].s64 = ctx.r[11].s64 + -16028;
	// 830CE154: 48174889  bl 0x832429dc
	ctx.lr = 0x830CE158;
	// extern call 0x832429DC → crate::xboxkrnl::RtlInitializeCriticalSection
	crate::xboxkrnl::RtlInitializeCriticalSection(ctx, base);
	// 830CE158: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830CE15C: 386BD970  addi r3, r11, -0x2690
	ctx.r[3].s64 = ctx.r[11].s64 + -9872;
	// 830CE160: 4817487D  bl 0x832429dc
	ctx.lr = 0x830CE164;
	// extern call 0x832429DC → crate::xboxkrnl::RtlInitializeCriticalSection
	crate::xboxkrnl::RtlInitializeCriticalSection(ctx, base);
	// 830CE164: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830CE168: 386BD948  addi r3, r11, -0x26b8
	ctx.r[3].s64 = ctx.r[11].s64 + -9912;
	// 830CE16C: 48174871  bl 0x832429dc
	ctx.lr = 0x830CE170;
	// extern call 0x832429DC → crate::xboxkrnl::RtlInitializeCriticalSection
	crate::xboxkrnl::RtlInitializeCriticalSection(ctx, base);
	// 830CE170: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830CE174: 386BD7E0  addi r3, r11, -0x2820
	ctx.r[3].s64 = ctx.r[11].s64 + -10272;
	// 830CE178: 48174865  bl 0x832429dc
	ctx.lr = 0x830CE17C;
	// extern call 0x832429DC → crate::xboxkrnl::RtlInitializeCriticalSection
	crate::xboxkrnl::RtlInitializeCriticalSection(ctx, base);
	// 830CE17C: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830CE180: 386BC12C  addi r3, r11, -0x3ed4
	ctx.r[3].s64 = ctx.r[11].s64 + -16084;
	// 830CE184: 48174859  bl 0x832429dc
	ctx.lr = 0x830CE188;
	// extern call 0x832429DC → crate::xboxkrnl::RtlInitializeCriticalSection
	crate::xboxkrnl::RtlInitializeCriticalSection(ctx, base);
	// 830CE188: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830CE18C: 386BD784  addi r3, r11, -0x287c
	ctx.r[3].s64 = ctx.r[11].s64 + -10364;
	// 830CE190: 4817484D  bl 0x832429dc
	ctx.lr = 0x830CE194;
	// extern call 0x832429DC → crate::xboxkrnl::RtlInitializeCriticalSection
	crate::xboxkrnl::RtlInitializeCriticalSection(ctx, base);
	// 830CE194: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830CE198: 386BC148  addi r3, r11, -0x3eb8
	ctx.r[3].s64 = ctx.r[11].s64 + -16056;
	// 830CE19C: 48174841  bl 0x832429dc
	ctx.lr = 0x830CE1A0;
	// extern call 0x832429DC → crate::xboxkrnl::RtlInitializeCriticalSection
	crate::xboxkrnl::RtlInitializeCriticalSection(ctx, base);
	// 830CE1A0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830CE1A4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830CE1A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830CE1AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830CE1B0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830CE1B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CE1B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830CE1B8 size=120
    let mut pc: u32 = 0x830CE1B8;
    'dispatch: loop {
        match pc {
            0x830CE1B8 => {
    //   block [0x830CE1B8..0x830CE230)
	// 830CE1B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 830CE1BC: 546B043E  clrlwi r11, r3, 0x10
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x0000FFFFu64;
	// 830CE1C0: 91440000  stw r10, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 830CE1C4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 830CE1C8: 419A005C  beq cr6, 0x830ce224
	if ctx.cr[6].eq {
	pc = 0x830CE224; continue 'dispatch;
	}
	// 830CE1CC: 3D408339  lis r10, -0x7cc7
	ctx.r[10].s64 = -2093416448;
	// 830CE1D0: 556B043E  clrlwi r11, r11, 0x10
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 830CE1D4: 394AC1B0  addi r10, r10, -0x3e50
	ctx.r[10].s64 = ctx.r[10].s64 + -15952;
	// 830CE1D8: 812A0220  lwz r9, 0x220(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(544 as u32) ) } as u64;
	// 830CE1DC: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 830CE1E0: 40980044  bge cr6, 0x830ce224
	if !ctx.cr[6].lt {
	pc = 0x830CE224; continue 'dispatch;
	}
	// 830CE1E4: 5568C9FA  rlwinm r8, r11, 0x19, 7, 0x1d
	ctx.r[8].u64 = ctx.r[11].u32 as u64 & 0x0000007Fu64;
	// 830CE1E8: 556B05FE  clrlwi r11, r11, 0x17
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000001FFu64;
	// 830CE1EC: 1D2B000C  mulli r9, r11, 0xc
	ctx.r[9].s64 = ctx.r[11].s64 * 12;
	// 830CE1F0: 7D68502E  lwzx r11, r8, r10
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 830CE1F4: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 830CE1F8: A14B0002  lhz r10, 2(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(2 as u32) ) } as u64;
	// 830CE1FC: 554A07FE  clrlwi r10, r10, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x00000001u64;
	// 830CE200: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 830CE204: 409A0020  bne cr6, 0x830ce224
	if !ctx.cr[6].eq {
	pc = 0x830CE224; continue 'dispatch;
	}
	// 830CE208: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830CE20C: 5469843E  srwi r9, r3, 0x10
	ctx.r[9].u32 = ctx.r[3].u32.wrapping_shr(16);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 830CE210: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 830CE214: 409A0010  bne cr6, 0x830ce224
	if !ctx.cr[6].eq {
	pc = 0x830CE224; continue 'dispatch;
	}
	// 830CE218: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830CE21C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830CE220: 409A0010  bne cr6, 0x830ce230
	if !ctx.cr[6].eq {
		sub_830CE230(ctx, base);
		return;
	}
	// 830CE224: 3C608030  lis r3, -0x7fd0
	ctx.r[3].s64 = -2144337920;
	// 830CE228: 6063000A  ori r3, r3, 0xa
	ctx.r[3].u64 = ctx.r[3].u64 | 10;
	// 830CE22C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CE230(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830CE230 size=16
    let mut pc: u32 = 0x830CE230;
    'dispatch: loop {
        match pc {
            0x830CE230 => {
    //   block [0x830CE230..0x830CE240)
	// 830CE230: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 830CE234: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830CE238: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830CE23C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CE240(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830CE240 size=148
    let mut pc: u32 = 0x830CE240;
    'dispatch: loop {
        match pc {
            0x830CE240 => {
    //   block [0x830CE240..0x830CE2D4)
	// 830CE240: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830CE244: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830CE248: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830CE24C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830CE250: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830CE254: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830CE258: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830CE25C: 3BCBC164  addi r30, r11, -0x3e9c
	ctx.r[30].s64 = ctx.r[11].s64 + -16028;
	// 830CE260: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830CE264: 48174709  bl 0x8324296c
	ctx.lr = 0x830CE268;
	// extern call 0x8324296C → crate::xboxkrnl::RtlEnterCriticalSection
	crate::xboxkrnl::RtlEnterCriticalSection(ctx, base);
	// 830CE268: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830CE26C: 396BC190  addi r11, r11, -0x3e70
	ctx.r[11].s64 = ctx.r[11].s64 + -15984;
	// 830CE270: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 830CE274: 7D2AFA14  add r9, r10, r31
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[31].u64;
	// 830CE278: 2B091000  cmplwi cr6, r9, 0x1000
	ctx.cr[6].compare_u32(ctx.r[9].u32, 4096 as u32, &mut ctx.xer);
	// 830CE27C: 41980014  blt cr6, 0x830ce290
	if ctx.cr[6].lt {
	pc = 0x830CE290; continue 'dispatch;
	}
	// 830CE280: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830CE284: 4800F22D  bl 0x830dd4b0
	ctx.lr = 0x830CE288;
	sub_830DD4B0(ctx, base);
	// 830CE288: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830CE28C: 48000024  b 0x830ce2b0
	pc = 0x830CE2B0; continue 'dispatch;
	// 830CE290: 810B0000  lwz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830CE294: 38E90007  addi r7, r9, 7
	ctx.r[7].s64 = ctx.r[9].s64 + 7;
	// 830CE298: 812B0004  lwz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830CE29C: 7FEA4214  add r31, r10, r8
	ctx.r[31].u64 = ctx.r[10].u64 + ctx.r[8].u64;
	// 830CE2A0: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 830CE2A4: 54E80038  rlwinm r8, r7, 0, 0, 0x1c
	ctx.r[8].u64 = ctx.r[7].u32 as u64 & 0xFFFFFFFFu64;
	// 830CE2A8: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 830CE2AC: 910B0008  stw r8, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[8].u32 ) };
	// 830CE2B0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830CE2B4: 481746A9  bl 0x8324295c
	ctx.lr = 0x830CE2B8;
	// extern call 0x8324295C → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 830CE2B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830CE2BC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830CE2C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830CE2C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830CE2C8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830CE2CC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830CE2D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CE2D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830CE2D8 size=140
    let mut pc: u32 = 0x830CE2D8;
    'dispatch: loop {
        match pc {
            0x830CE2D8 => {
    //   block [0x830CE2D8..0x830CE364)
	// 830CE2D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830CE2DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830CE2E0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830CE2E4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830CE2E8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830CE2EC: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830CE2F0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830CE2F4: 3BCBC164  addi r30, r11, -0x3e9c
	ctx.r[30].s64 = ctx.r[11].s64 + -16028;
	// 830CE2F8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830CE2FC: 48174671  bl 0x8324296c
	ctx.lr = 0x830CE300;
	// extern call 0x8324296C → crate::xboxkrnl::RtlEnterCriticalSection
	crate::xboxkrnl::RtlEnterCriticalSection(ctx, base);
	// 830CE300: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830CE304: 396BC190  addi r11, r11, -0x3e70
	ctx.r[11].s64 = ctx.r[11].s64 + -15984;
	// 830CE308: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830CE30C: 7F1F5040  cmplw cr6, r31, r10
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[10].u32, &mut ctx.xer);
	// 830CE310: 4198002C  blt cr6, 0x830ce33c
	if ctx.cr[6].lt {
	pc = 0x830CE33C; continue 'dispatch;
	}
	// 830CE314: 394A1000  addi r10, r10, 0x1000
	ctx.r[10].s64 = ctx.r[10].s64 + 4096;
	// 830CE318: 7F1F5040  cmplw cr6, r31, r10
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[10].u32, &mut ctx.xer);
	// 830CE31C: 40980020  bge cr6, 0x830ce33c
	if !ctx.cr[6].lt {
	pc = 0x830CE33C; continue 'dispatch;
	}
	// 830CE320: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830CE324: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 830CE328: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 830CE32C: 40820018  bne 0x830ce344
	if !ctx.cr[0].eq {
	pc = 0x830CE344; continue 'dispatch;
	}
	// 830CE330: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 830CE334: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 830CE338: 4800000C  b 0x830ce344
	pc = 0x830CE344; continue 'dispatch;
	// 830CE33C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830CE340: 4800F199  bl 0x830dd4d8
	ctx.lr = 0x830CE344;
	sub_830DD4D8(ctx, base);
	// 830CE344: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830CE348: 48174615  bl 0x8324295c
	ctx.lr = 0x830CE34C;
	// extern call 0x8324295C → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 830CE34C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830CE350: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830CE354: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830CE358: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830CE35C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830CE360: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CE368(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830CE368 size=116
    let mut pc: u32 = 0x830CE368;
    'dispatch: loop {
        match pc {
            0x830CE368 => {
    //   block [0x830CE368..0x830CE3DC)
	// 830CE368: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830CE36C: 480D9DFD  bl 0x831a8168
	ctx.lr = 0x830CE370;
	sub_831A8130(ctx, base);
	// 830CE370: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830CE374: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 830CE378: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 830CE37C: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 830CE380: 419A0048  beq cr6, 0x830ce3c8
	if ctx.cr[6].eq {
	pc = 0x830CE3C8; continue 'dispatch;
	}
	// 830CE384: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830CE388: 480DAD41  bl 0x831a90c8
	ctx.lr = 0x830CE38C;
	sub_831A90C8(ctx, base);
	// 830CE38C: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 830CE390: 41820038  beq 0x830ce3c8
	if ctx.cr[0].eq {
	pc = 0x830CE3C8; continue 'dispatch;
	}
	// 830CE394: 397E0001  addi r11, r30, 1
	ctx.r[11].s64 = ctx.r[30].s64 + 1;
	// 830CE398: 5563083C  slwi r3, r11, 1
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 830CE39C: 4BFFFEA5  bl 0x830ce240
	ctx.lr = 0x830CE3A0;
	sub_830CE240(ctx, base);
	// 830CE3A0: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 830CE3A4: 40820010  bne 0x830ce3b4
	if !ctx.cr[0].eq {
	pc = 0x830CE3B4; continue 'dispatch;
	}
	// 830CE3A8: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 830CE3AC: 6063000E  ori r3, r3, 0xe
	ctx.r[3].u64 = ctx.r[3].u64 | 14;
	// 830CE3B0: 48000024  b 0x830ce3d4
	pc = 0x830CE3D4; continue 'dispatch;
	// 830CE3B4: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 830CE3B8: 389E0001  addi r4, r30, 1
	ctx.r[4].s64 = ctx.r[30].s64 + 1;
	// 830CE3BC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830CE3C0: 480E3AF9  bl 0x831b1eb8
	ctx.lr = 0x830CE3C4;
	sub_831B1EB8(ctx, base);
	// 830CE3C4: 48000008  b 0x830ce3cc
	pc = 0x830CE3CC; continue 'dispatch;
	// 830CE3C8: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 830CE3CC: 93FC0000  stw r31, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 830CE3D0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830CE3D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 830CE3D8: 480D9DE0  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CE3E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830CE3E0 size=96
    let mut pc: u32 = 0x830CE3E0;
    'dispatch: loop {
        match pc {
            0x830CE3E0 => {
    //   block [0x830CE3E0..0x830CE440)
	// 830CE3E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830CE3E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830CE3E8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830CE3EC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830CE3F0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 830CE3F4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 830CE3F8: 419A002C  beq cr6, 0x830ce424
	if ctx.cr[6].eq {
	pc = 0x830CE424; continue 'dispatch;
	}
	// 830CE3FC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 830CE400: 419A0024  beq cr6, 0x830ce424
	if ctx.cr[6].eq {
	pc = 0x830CE424; continue 'dispatch;
	}
	// 830CE404: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 830CE408: 38A00028  li r5, 0x28
	ctx.r[5].s64 = 40;
	// 830CE40C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830CE410: 480DA101  bl 0x831a8510
	ctx.lr = 0x830CE414;
	sub_831A8510(ctx, base);
	// 830CE414: 39600028  li r11, 0x28
	ctx.r[11].s64 = 40;
	// 830CE418: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830CE41C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830CE420: 4800000C  b 0x830ce42c
	pc = 0x830CE42C; continue 'dispatch;
	// 830CE424: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 830CE428: 60630057  ori r3, r3, 0x57
	ctx.r[3].u64 = ctx.r[3].u64 | 87;
	// 830CE42C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830CE430: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830CE434: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830CE438: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830CE43C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CE440(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830CE440 size=32
    let mut pc: u32 = 0x830CE440;
    'dispatch: loop {
        match pc {
            0x830CE440 => {
    //   block [0x830CE440..0x830CE460)
	// 830CE440: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 830CE444: 419A001C  beq cr6, 0x830ce460
	if ctx.cr[6].eq {
		sub_830CE460(ctx, base);
		return;
	}
	// 830CE448: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 830CE44C: 419A0014  beq cr6, 0x830ce460
	if ctx.cr[6].eq {
		sub_830CE460(ctx, base);
		return;
	}
	// 830CE450: 81630028  lwz r11, 0x28(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 830CE454: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830CE458: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830CE45C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CE460(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830CE460 size=12
    let mut pc: u32 = 0x830CE460;
    'dispatch: loop {
        match pc {
            0x830CE460 => {
    //   block [0x830CE460..0x830CE46C)
	// 830CE460: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 830CE464: 60630057  ori r3, r3, 0x57
	ctx.r[3].u64 = ctx.r[3].u64 | 87;
	// 830CE468: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CE470(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830CE470 size=20
    let mut pc: u32 = 0x830CE470;
    'dispatch: loop {
        match pc {
            0x830CE470 => {
    //   block [0x830CE470..0x830CE484)
	// 830CE470: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 830CE474: 409A0010  bne cr6, 0x830ce484
	if !ctx.cr[6].eq {
		sub_830CE484(ctx, base);
		return;
	}
	// 830CE478: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 830CE47C: 60630057  ori r3, r3, 0x57
	ctx.r[3].u64 = ctx.r[3].u64 | 87;
	// 830CE480: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CE484(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830CE484 size=120
    let mut pc: u32 = 0x830CE484;
    'dispatch: loop {
        match pc {
            0x830CE484 => {
    //   block [0x830CE484..0x830CE4FC)
	// 830CE484: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 830CE488: 546B043E  clrlwi r11, r3, 0x10
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x0000FFFFu64;
	// 830CE48C: 91440000  stw r10, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 830CE490: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 830CE494: 419A005C  beq cr6, 0x830ce4f0
	if ctx.cr[6].eq {
	pc = 0x830CE4F0; continue 'dispatch;
	}
	// 830CE498: 3D408339  lis r10, -0x7cc7
	ctx.r[10].s64 = -2093416448;
	// 830CE49C: 556B043E  clrlwi r11, r11, 0x10
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 830CE4A0: 394AC1B0  addi r10, r10, -0x3e50
	ctx.r[10].s64 = ctx.r[10].s64 + -15952;
	// 830CE4A4: 812A0220  lwz r9, 0x220(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(544 as u32) ) } as u64;
	// 830CE4A8: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 830CE4AC: 40980044  bge cr6, 0x830ce4f0
	if !ctx.cr[6].lt {
	pc = 0x830CE4F0; continue 'dispatch;
	}
	// 830CE4B0: 5568C9FA  rlwinm r8, r11, 0x19, 7, 0x1d
	ctx.r[8].u64 = ctx.r[11].u32 as u64 & 0x0000007Fu64;
	// 830CE4B4: 556B05FE  clrlwi r11, r11, 0x17
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000001FFu64;
	// 830CE4B8: 1D2B000C  mulli r9, r11, 0xc
	ctx.r[9].s64 = ctx.r[11].s64 * 12;
	// 830CE4BC: 7D68502E  lwzx r11, r8, r10
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 830CE4C0: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 830CE4C4: A14B0002  lhz r10, 2(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(2 as u32) ) } as u64;
	// 830CE4C8: 554A07FE  clrlwi r10, r10, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x00000001u64;
	// 830CE4CC: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 830CE4D0: 409A0020  bne cr6, 0x830ce4f0
	if !ctx.cr[6].eq {
	pc = 0x830CE4F0; continue 'dispatch;
	}
	// 830CE4D4: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830CE4D8: 5469843E  srwi r9, r3, 0x10
	ctx.r[9].u32 = ctx.r[3].u32.wrapping_shr(16);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 830CE4DC: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 830CE4E0: 409A0010  bne cr6, 0x830ce4f0
	if !ctx.cr[6].eq {
	pc = 0x830CE4F0; continue 'dispatch;
	}
	// 830CE4E4: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830CE4E8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830CE4EC: 409A0010  bne cr6, 0x830ce4fc
	if !ctx.cr[6].eq {
		sub_830CE4FC(ctx, base);
		return;
	}
	// 830CE4F0: 3C608030  lis r3, -0x7fd0
	ctx.r[3].s64 = -2144337920;
	// 830CE4F4: 6063000A  ori r3, r3, 0xa
	ctx.r[3].u64 = ctx.r[3].u64 | 10;
	// 830CE4F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CE4FC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830CE4FC size=24
    let mut pc: u32 = 0x830CE4FC;
    'dispatch: loop {
        match pc {
            0x830CE4FC => {
    //   block [0x830CE4FC..0x830CE514)
	// 830CE4FC: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 830CE500: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830CE504: 419AFFEC  beq cr6, 0x830ce4f0
	if ctx.cr[6].eq {
		sub_830CE484(ctx, base);
		return;
	}
	// 830CE508: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830CE50C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830CE510: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CE518(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830CE518 size=20
    let mut pc: u32 = 0x830CE518;
    'dispatch: loop {
        match pc {
            0x830CE518 => {
    //   block [0x830CE518..0x830CE52C)
	// 830CE518: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 830CE51C: 409A0010  bne cr6, 0x830ce52c
	if !ctx.cr[6].eq {
		sub_830CE52C(ctx, base);
		return;
	}
	// 830CE520: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 830CE524: 60630057  ori r3, r3, 0x57
	ctx.r[3].u64 = ctx.r[3].u64 | 87;
	// 830CE528: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CE52C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830CE52C size=120
    let mut pc: u32 = 0x830CE52C;
    'dispatch: loop {
        match pc {
            0x830CE52C => {
    //   block [0x830CE52C..0x830CE5A4)
	// 830CE52C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 830CE530: 546B043E  clrlwi r11, r3, 0x10
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x0000FFFFu64;
	// 830CE534: 91440000  stw r10, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 830CE538: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 830CE53C: 419A005C  beq cr6, 0x830ce598
	if ctx.cr[6].eq {
	pc = 0x830CE598; continue 'dispatch;
	}
	// 830CE540: 3D408339  lis r10, -0x7cc7
	ctx.r[10].s64 = -2093416448;
	// 830CE544: 556B043E  clrlwi r11, r11, 0x10
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 830CE548: 394AC1B0  addi r10, r10, -0x3e50
	ctx.r[10].s64 = ctx.r[10].s64 + -15952;
	// 830CE54C: 812A0220  lwz r9, 0x220(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(544 as u32) ) } as u64;
	// 830CE550: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 830CE554: 40980044  bge cr6, 0x830ce598
	if !ctx.cr[6].lt {
	pc = 0x830CE598; continue 'dispatch;
	}
	// 830CE558: 5568C9FA  rlwinm r8, r11, 0x19, 7, 0x1d
	ctx.r[8].u64 = ctx.r[11].u32 as u64 & 0x0000007Fu64;
	// 830CE55C: 556B05FE  clrlwi r11, r11, 0x17
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000001FFu64;
	// 830CE560: 1D2B000C  mulli r9, r11, 0xc
	ctx.r[9].s64 = ctx.r[11].s64 * 12;
	// 830CE564: 7D68502E  lwzx r11, r8, r10
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 830CE568: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 830CE56C: A14B0002  lhz r10, 2(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(2 as u32) ) } as u64;
	// 830CE570: 554A07FE  clrlwi r10, r10, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x00000001u64;
	// 830CE574: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 830CE578: 409A0020  bne cr6, 0x830ce598
	if !ctx.cr[6].eq {
	pc = 0x830CE598; continue 'dispatch;
	}
	// 830CE57C: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830CE580: 5469843E  srwi r9, r3, 0x10
	ctx.r[9].u32 = ctx.r[3].u32.wrapping_shr(16);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 830CE584: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 830CE588: 409A0010  bne cr6, 0x830ce598
	if !ctx.cr[6].eq {
	pc = 0x830CE598; continue 'dispatch;
	}
	// 830CE58C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830CE590: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830CE594: 409A0010  bne cr6, 0x830ce5a4
	if !ctx.cr[6].eq {
		sub_830CE5A4(ctx, base);
		return;
	}
	// 830CE598: 3C608030  lis r3, -0x7fd0
	ctx.r[3].s64 = -2144337920;
	// 830CE59C: 6063000A  ori r3, r3, 0xa
	ctx.r[3].u64 = ctx.r[3].u64 | 10;
	// 830CE5A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CE5A4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830CE5A4 size=28
    let mut pc: u32 = 0x830CE5A4;
    'dispatch: loop {
        match pc {
            0x830CE5A4 => {
    //   block [0x830CE5A4..0x830CE5C0)
	// 830CE5A4: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830CE5A8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830CE5AC: 419A000C  beq cr6, 0x830ce5b8
	if ctx.cr[6].eq {
	pc = 0x830CE5B8; continue 'dispatch;
	}
	// 830CE5B0: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830CE5B4: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830CE5B8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830CE5BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CE5C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830CE5C0 size=52
    let mut pc: u32 = 0x830CE5C0;
    'dispatch: loop {
        match pc {
            0x830CE5C0 => {
    //   block [0x830CE5C0..0x830CE5F4)
	// 830CE5C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830CE5C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830CE5C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830CE5CC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 830CE5D0: 4BFFFBE9  bl 0x830ce1b8
	ctx.lr = 0x830CE5D4;
	sub_830CE1B8(ctx, base);
	// 830CE5D4: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 830CE5D8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830CE5DC: 41800008  blt 0x830ce5e4
	if ctx.cr[0].lt {
	pc = 0x830CE5E4; continue 'dispatch;
	}
	// 830CE5E0: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 830CE5E4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830CE5E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830CE5EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830CE5F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CE5F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830CE5F8 size=104
    let mut pc: u32 = 0x830CE5F8;
    'dispatch: loop {
        match pc {
            0x830CE5F8 => {
    //   block [0x830CE5F8..0x830CE660)
	// 830CE5F8: 546B043E  clrlwi r11, r3, 0x10
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x0000FFFFu64;
	// 830CE5FC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 830CE600: 419A0060  beq cr6, 0x830ce660
	if ctx.cr[6].eq {
		sub_830CE660(ctx, base);
		return;
	}
	// 830CE604: 3D408339  lis r10, -0x7cc7
	ctx.r[10].s64 = -2093416448;
	// 830CE608: 556B043E  clrlwi r11, r11, 0x10
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 830CE60C: 394AC1B0  addi r10, r10, -0x3e50
	ctx.r[10].s64 = ctx.r[10].s64 + -15952;
	// 830CE610: 812A0220  lwz r9, 0x220(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(544 as u32) ) } as u64;
	// 830CE614: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 830CE618: 40980048  bge cr6, 0x830ce660
	if !ctx.cr[6].lt {
		sub_830CE660(ctx, base);
		return;
	}
	// 830CE61C: 5568C9FA  rlwinm r8, r11, 0x19, 7, 0x1d
	ctx.r[8].u64 = ctx.r[11].u32 as u64 & 0x0000007Fu64;
	// 830CE620: 556B05FE  clrlwi r11, r11, 0x17
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000001FFu64;
	// 830CE624: 1D2B000C  mulli r9, r11, 0xc
	ctx.r[9].s64 = ctx.r[11].s64 * 12;
	// 830CE628: 7D68502E  lwzx r11, r8, r10
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 830CE62C: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 830CE630: A14B0002  lhz r10, 2(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(2 as u32) ) } as u64;
	// 830CE634: 554A07FE  clrlwi r10, r10, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x00000001u64;
	// 830CE638: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 830CE63C: 409A0024  bne cr6, 0x830ce660
	if !ctx.cr[6].eq {
		sub_830CE660(ctx, base);
		return;
	}
	// 830CE640: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830CE644: 5469843E  srwi r9, r3, 0x10
	ctx.r[9].u32 = ctx.r[3].u32.wrapping_shr(16);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 830CE648: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 830CE64C: 409A0014  bne cr6, 0x830ce660
	if !ctx.cr[6].eq {
		sub_830CE660(ctx, base);
		return;
	}
	// 830CE650: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830CE654: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 830CE658: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830CE65C: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CE660(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830CE660 size=8
    let mut pc: u32 = 0x830CE660;
    'dispatch: loop {
        match pc {
            0x830CE660 => {
    //   block [0x830CE660..0x830CE668)
	// 830CE660: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830CE664: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CE668(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830CE668 size=116
    let mut pc: u32 = 0x830CE668;
    'dispatch: loop {
        match pc {
            0x830CE668 => {
    //   block [0x830CE668..0x830CE6DC)
	// 830CE668: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 830CE66C: 546B043E  clrlwi r11, r3, 0x10
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x0000FFFFu64;
	// 830CE670: 91440000  stw r10, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 830CE674: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 830CE678: 419A005C  beq cr6, 0x830ce6d4
	if ctx.cr[6].eq {
	pc = 0x830CE6D4; continue 'dispatch;
	}
	// 830CE67C: 3D408339  lis r10, -0x7cc7
	ctx.r[10].s64 = -2093416448;
	// 830CE680: 556B043E  clrlwi r11, r11, 0x10
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 830CE684: 394AC1B0  addi r10, r10, -0x3e50
	ctx.r[10].s64 = ctx.r[10].s64 + -15952;
	// 830CE688: 812A0220  lwz r9, 0x220(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(544 as u32) ) } as u64;
	// 830CE68C: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 830CE690: 40980044  bge cr6, 0x830ce6d4
	if !ctx.cr[6].lt {
	pc = 0x830CE6D4; continue 'dispatch;
	}
	// 830CE694: 5568C9FA  rlwinm r8, r11, 0x19, 7, 0x1d
	ctx.r[8].u64 = ctx.r[11].u32 as u64 & 0x0000007Fu64;
	// 830CE698: 556B05FE  clrlwi r11, r11, 0x17
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000001FFu64;
	// 830CE69C: 1D2B000C  mulli r9, r11, 0xc
	ctx.r[9].s64 = ctx.r[11].s64 * 12;
	// 830CE6A0: 7D68502E  lwzx r11, r8, r10
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 830CE6A4: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 830CE6A8: A14B0002  lhz r10, 2(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(2 as u32) ) } as u64;
	// 830CE6AC: 554A07FE  clrlwi r10, r10, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x00000001u64;
	// 830CE6B0: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 830CE6B4: 409A0020  bne cr6, 0x830ce6d4
	if !ctx.cr[6].eq {
	pc = 0x830CE6D4; continue 'dispatch;
	}
	// 830CE6B8: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830CE6BC: 5469843E  srwi r9, r3, 0x10
	ctx.r[9].u32 = ctx.r[3].u32.wrapping_shr(16);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 830CE6C0: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 830CE6C4: 409A0010  bne cr6, 0x830ce6d4
	if !ctx.cr[6].eq {
	pc = 0x830CE6D4; continue 'dispatch;
	}
	// 830CE6C8: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830CE6CC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830CE6D0: 409A000C  bne cr6, 0x830ce6dc
	if !ctx.cr[6].eq {
		sub_830CE6DC(ctx, base);
		return;
	}
	// 830CE6D4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830CE6D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CE6DC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830CE6DC size=20
    let mut pc: u32 = 0x830CE6DC;
    'dispatch: loop {
        match pc {
            0x830CE6DC => {
    //   block [0x830CE6DC..0x830CE6F0)
	// 830CE6DC: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 830CE6E0: 814B0020  lwz r10, 0x20(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 830CE6E4: 91440000  stw r10, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 830CE6E8: 806B001C  lwz r3, 0x1c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 830CE6EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CE6F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830CE6F0 size=288
    let mut pc: u32 = 0x830CE6F0;
    'dispatch: loop {
        match pc {
            0x830CE6F0 => {
    //   block [0x830CE6F0..0x830CE810)
	// 830CE6F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830CE6F4: 480D9A69  bl 0x831a815c
	ctx.lr = 0x830CE6F8;
	sub_831A8130(ctx, base);
	// 830CE6F8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830CE6FC: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 830CE700: 7CB92B78  mr r25, r5
	ctx.r[25].u64 = ctx.r[5].u64;
	// 830CE704: 2B1A0000  cmplwi cr6, r26, 0
	ctx.cr[6].compare_u32(ctx.r[26].u32, 0 as u32, &mut ctx.xer);
	// 830CE708: 419A00F8  beq cr6, 0x830ce800
	if ctx.cr[6].eq {
	pc = 0x830CE800; continue 'dispatch;
	}
	// 830CE70C: 2B190000  cmplwi cr6, r25, 0
	ctx.cr[6].compare_u32(ctx.r[25].u32, 0 as u32, &mut ctx.xer);
	// 830CE710: 419A00F0  beq cr6, 0x830ce800
	if ctx.cr[6].eq {
	pc = 0x830CE800; continue 'dispatch;
	}
	// 830CE714: 546B043E  clrlwi r11, r3, 0x10
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x0000FFFFu64;
	// 830CE718: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 830CE71C: 419A005C  beq cr6, 0x830ce778
	if ctx.cr[6].eq {
	pc = 0x830CE778; continue 'dispatch;
	}
	// 830CE720: 3D408339  lis r10, -0x7cc7
	ctx.r[10].s64 = -2093416448;
	// 830CE724: 556B043E  clrlwi r11, r11, 0x10
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 830CE728: 394AC1B0  addi r10, r10, -0x3e50
	ctx.r[10].s64 = ctx.r[10].s64 + -15952;
	// 830CE72C: 812A0220  lwz r9, 0x220(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(544 as u32) ) } as u64;
	// 830CE730: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 830CE734: 40980044  bge cr6, 0x830ce778
	if !ctx.cr[6].lt {
	pc = 0x830CE778; continue 'dispatch;
	}
	// 830CE738: 5568C9FA  rlwinm r8, r11, 0x19, 7, 0x1d
	ctx.r[8].u64 = ctx.r[11].u32 as u64 & 0x0000007Fu64;
	// 830CE73C: 556B05FE  clrlwi r11, r11, 0x17
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000001FFu64;
	// 830CE740: 1D2B000C  mulli r9, r11, 0xc
	ctx.r[9].s64 = ctx.r[11].s64 * 12;
	// 830CE744: 7D68502E  lwzx r11, r8, r10
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 830CE748: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 830CE74C: A14B0002  lhz r10, 2(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(2 as u32) ) } as u64;
	// 830CE750: 554A07FE  clrlwi r10, r10, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x00000001u64;
	// 830CE754: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 830CE758: 409A0020  bne cr6, 0x830ce778
	if !ctx.cr[6].eq {
	pc = 0x830CE778; continue 'dispatch;
	}
	// 830CE75C: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830CE760: 5469843E  srwi r9, r3, 0x10
	ctx.r[9].u32 = ctx.r[3].u32.wrapping_shr(16);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 830CE764: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 830CE768: 409A0010  bne cr6, 0x830ce778
	if !ctx.cr[6].eq {
	pc = 0x830CE778; continue 'dispatch;
	}
	// 830CE76C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830CE770: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830CE774: 409A0010  bne cr6, 0x830ce784
	if !ctx.cr[6].eq {
	pc = 0x830CE784; continue 'dispatch;
	}
	// 830CE778: 3C608030  lis r3, -0x7fd0
	ctx.r[3].s64 = -2144337920;
	// 830CE77C: 6063000A  ori r3, r3, 0xa
	ctx.r[3].u64 = ctx.r[3].u64 | 10;
	// 830CE780: 48000088  b 0x830ce808
	pc = 0x830CE808; continue 'dispatch;
	// 830CE784: 83AB0018  lwz r29, 0x18(r11)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 830CE788: 4800004C  b 0x830ce7d4
	pc = 0x830CE7D4; continue 'dispatch;
	// 830CE78C: 837D001C  lwz r27, 0x1c(r29)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(28 as u32) ) } as u64;
	// 830CE790: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 830CE794: 419A003C  beq cr6, 0x830ce7d0
	if ctx.cr[6].eq {
	pc = 0x830CE7D0; continue 'dispatch;
	}
	// 830CE798: 839D0020  lwz r28, 0x20(r29)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(32 as u32) ) } as u64;
	// 830CE79C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 830CE7A0: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 830CE7A4: 419A002C  beq cr6, 0x830ce7d0
	if ctx.cr[6].eq {
	pc = 0x830CE7D0; continue 'dispatch;
	}
	// 830CE7A8: 3BDB0010  addi r30, r27, 0x10
	ctx.r[30].s64 = ctx.r[27].s64 + 16;
	// 830CE7AC: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 830CE7B0: 809E0000  lwz r4, 0(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 830CE7B4: 480E367D  bl 0x831b1e30
	ctx.lr = 0x830CE7B8;
	sub_831B1E30(ctx, base);
	// 830CE7B8: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 830CE7BC: 4182002C  beq 0x830ce7e8
	if ctx.cr[0].eq {
	pc = 0x830CE7E8; continue 'dispatch;
	}
	// 830CE7C0: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 830CE7C4: 3BDE0030  addi r30, r30, 0x30
	ctx.r[30].s64 = ctx.r[30].s64 + 48;
	// 830CE7C8: 7F1FE040  cmplw cr6, r31, r28
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[28].u32, &mut ctx.xer);
	// 830CE7CC: 4198FFE0  blt cr6, 0x830ce7ac
	if ctx.cr[6].lt {
	pc = 0x830CE7AC; continue 'dispatch;
	}
	// 830CE7D0: 83BD0028  lwz r29, 0x28(r29)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(40 as u32) ) } as u64;
	// 830CE7D4: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 830CE7D8: 409AFFB4  bne cr6, 0x830ce78c
	if !ctx.cr[6].eq {
	pc = 0x830CE78C; continue 'dispatch;
	}
	// 830CE7DC: 3C608030  lis r3, -0x7fd0
	ctx.r[3].s64 = -2144337920;
	// 830CE7E0: 60630007  ori r3, r3, 7
	ctx.r[3].u64 = ctx.r[3].u64 | 7;
	// 830CE7E4: 48000024  b 0x830ce808
	pc = 0x830CE808; continue 'dispatch;
	// 830CE7E8: 1D7F0030  mulli r11, r31, 0x30
	ctx.r[11].s64 = ctx.r[31].s64 * 48;
	// 830CE7EC: 7D6BDA14  add r11, r11, r27
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[27].u64;
	// 830CE7F0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830CE7F4: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830CE7F8: 91790000  stw r11, 0(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830CE7FC: 4800000C  b 0x830ce808
	pc = 0x830CE808; continue 'dispatch;
	// 830CE800: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 830CE804: 60630057  ori r3, r3, 0x57
	ctx.r[3].u64 = ctx.r[3].u64 | 87;
	// 830CE808: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 830CE80C: 480D99A0  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CE810(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830CE810 size=72
    let mut pc: u32 = 0x830CE810;
    'dispatch: loop {
        match pc {
            0x830CE810 => {
    //   block [0x830CE810..0x830CE858)
	// 830CE810: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 830CE814: 419A0044  beq cr6, 0x830ce858
	if ctx.cr[6].eq {
		sub_830CE858(ctx, base);
		return;
	}
	// 830CE818: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 830CE81C: 419A003C  beq cr6, 0x830ce858
	if ctx.cr[6].eq {
		sub_830CE858(ctx, base);
		return;
	}
	// 830CE820: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830CE824: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 830CE828: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830CE82C: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830CE830: 419A0028  beq cr6, 0x830ce858
	if ctx.cr[6].eq {
		sub_830CE858(ctx, base);
		return;
	}
	// 830CE834: 8163001C  lwz r11, 0x1c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 830CE838: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830CE83C: 419A0014  beq cr6, 0x830ce850
	if ctx.cr[6].eq {
	pc = 0x830CE850; continue 'dispatch;
	}
	// 830CE840: 81630020  lwz r11, 0x20(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 830CE844: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830CE848: 8163001C  lwz r11, 0x1c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 830CE84C: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830CE850: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830CE854: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CE858(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830CE858 size=12
    let mut pc: u32 = 0x830CE858;
    'dispatch: loop {
        match pc {
            0x830CE858 => {
    //   block [0x830CE858..0x830CE864)
	// 830CE858: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 830CE85C: 60630057  ori r3, r3, 0x57
	ctx.r[3].u64 = ctx.r[3].u64 | 87;
	// 830CE860: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CE868(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830CE868 size=184
    let mut pc: u32 = 0x830CE868;
    'dispatch: loop {
        match pc {
            0x830CE868 => {
    //   block [0x830CE868..0x830CE920)
	// 830CE868: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830CE86C: 480D98F1  bl 0x831a815c
	ctx.lr = 0x830CE870;
	sub_831A8130(ctx, base);
	// 830CE870: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830CE874: 7C992378  mr r25, r4
	ctx.r[25].u64 = ctx.r[4].u64;
	// 830CE878: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 830CE87C: 2B190000  cmplwi cr6, r25, 0
	ctx.cr[6].compare_u32(ctx.r[25].u32, 0 as u32, &mut ctx.xer);
	// 830CE880: 419A0090  beq cr6, 0x830ce910
	if ctx.cr[6].eq {
	pc = 0x830CE910; continue 'dispatch;
	}
	// 830CE884: 2B1A0000  cmplwi cr6, r26, 0
	ctx.cr[6].compare_u32(ctx.r[26].u32, 0 as u32, &mut ctx.xer);
	// 830CE888: 419A0088  beq cr6, 0x830ce910
	if ctx.cr[6].eq {
	pc = 0x830CE910; continue 'dispatch;
	}
	// 830CE88C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830CE890: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 830CE894: 917A0000  stw r11, 0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830CE898: 419A0078  beq cr6, 0x830ce910
	if ctx.cr[6].eq {
	pc = 0x830CE910; continue 'dispatch;
	}
	// 830CE89C: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 830CE8A0: 837C001C  lwz r27, 0x1c(r28)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(28 as u32) ) } as u64;
	// 830CE8A4: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 830CE8A8: 419A003C  beq cr6, 0x830ce8e4
	if ctx.cr[6].eq {
	pc = 0x830CE8E4; continue 'dispatch;
	}
	// 830CE8AC: 83BC0020  lwz r29, 0x20(r28)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(32 as u32) ) } as u64;
	// 830CE8B0: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 830CE8B4: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 830CE8B8: 419A002C  beq cr6, 0x830ce8e4
	if ctx.cr[6].eq {
	pc = 0x830CE8E4; continue 'dispatch;
	}
	// 830CE8BC: 3BDB0010  addi r30, r27, 0x10
	ctx.r[30].s64 = ctx.r[27].s64 + 16;
	// 830CE8C0: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 830CE8C4: 809E0000  lwz r4, 0(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 830CE8C8: 480E3569  bl 0x831b1e30
	ctx.lr = 0x830CE8CC;
	sub_831B1E30(ctx, base);
	// 830CE8CC: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 830CE8D0: 4182002C  beq 0x830ce8fc
	if ctx.cr[0].eq {
	pc = 0x830CE8FC; continue 'dispatch;
	}
	// 830CE8D4: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 830CE8D8: 3BDE0030  addi r30, r30, 0x30
	ctx.r[30].s64 = ctx.r[30].s64 + 48;
	// 830CE8DC: 7F1FE840  cmplw cr6, r31, r29
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[29].u32, &mut ctx.xer);
	// 830CE8E0: 4198FFE0  blt cr6, 0x830ce8c0
	if ctx.cr[6].lt {
	pc = 0x830CE8C0; continue 'dispatch;
	}
	// 830CE8E4: 839C0028  lwz r28, 0x28(r28)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(40 as u32) ) } as u64;
	// 830CE8E8: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 830CE8EC: 409AFFB4  bne cr6, 0x830ce8a0
	if !ctx.cr[6].eq {
	pc = 0x830CE8A0; continue 'dispatch;
	}
	// 830CE8F0: 3C608030  lis r3, -0x7fd0
	ctx.r[3].s64 = -2144337920;
	// 830CE8F4: 60630007  ori r3, r3, 7
	ctx.r[3].u64 = ctx.r[3].u64 | 7;
	// 830CE8F8: 48000020  b 0x830ce918
	pc = 0x830CE918; continue 'dispatch;
	// 830CE8FC: 1D7F0030  mulli r11, r31, 0x30
	ctx.r[11].s64 = ctx.r[31].s64 * 48;
	// 830CE900: 7D6BDA14  add r11, r11, r27
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[27].u64;
	// 830CE904: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830CE908: 917A0000  stw r11, 0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830CE90C: 4800000C  b 0x830ce918
	pc = 0x830CE918; continue 'dispatch;
	// 830CE910: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 830CE914: 60630057  ori r3, r3, 0x57
	ctx.r[3].u64 = ctx.r[3].u64 | 87;
	// 830CE918: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 830CE91C: 480D9890  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CE920(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830CE920 size=208
    let mut pc: u32 = 0x830CE920;
    'dispatch: loop {
        match pc {
            0x830CE920 => {
    //   block [0x830CE920..0x830CE9F0)
	// 830CE920: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830CE924: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830CE928: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830CE92C: 7C872378  mr r7, r4
	ctx.r[7].u64 = ctx.r[4].u64;
	// 830CE930: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 830CE934: 409A0010  bne cr6, 0x830ce944
	if !ctx.cr[6].eq {
	pc = 0x830CE944; continue 'dispatch;
	}
	// 830CE938: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 830CE93C: 60630057  ori r3, r3, 0x57
	ctx.r[3].u64 = ctx.r[3].u64 | 87;
	// 830CE940: 480000A0  b 0x830ce9e0
	pc = 0x830CE9E0; continue 'dispatch;
	// 830CE944: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 830CE948: 546B043E  clrlwi r11, r3, 0x10
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x0000FFFFu64;
	// 830CE94C: 91450000  stw r10, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 830CE950: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 830CE954: 419A005C  beq cr6, 0x830ce9b0
	if ctx.cr[6].eq {
	pc = 0x830CE9B0; continue 'dispatch;
	}
	// 830CE958: 3D408339  lis r10, -0x7cc7
	ctx.r[10].s64 = -2093416448;
	// 830CE95C: 556B043E  clrlwi r11, r11, 0x10
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 830CE960: 394AC1B0  addi r10, r10, -0x3e50
	ctx.r[10].s64 = ctx.r[10].s64 + -15952;
	// 830CE964: 812A0220  lwz r9, 0x220(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(544 as u32) ) } as u64;
	// 830CE968: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 830CE96C: 40980044  bge cr6, 0x830ce9b0
	if !ctx.cr[6].lt {
	pc = 0x830CE9B0; continue 'dispatch;
	}
	// 830CE970: 5568C9FA  rlwinm r8, r11, 0x19, 7, 0x1d
	ctx.r[8].u64 = ctx.r[11].u32 as u64 & 0x0000007Fu64;
	// 830CE974: 556B05FE  clrlwi r11, r11, 0x17
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000001FFu64;
	// 830CE978: 1D2B000C  mulli r9, r11, 0xc
	ctx.r[9].s64 = ctx.r[11].s64 * 12;
	// 830CE97C: 7D68502E  lwzx r11, r8, r10
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 830CE980: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 830CE984: A14B0002  lhz r10, 2(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(2 as u32) ) } as u64;
	// 830CE988: 554A07FE  clrlwi r10, r10, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x00000001u64;
	// 830CE98C: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 830CE990: 409A0020  bne cr6, 0x830ce9b0
	if !ctx.cr[6].eq {
	pc = 0x830CE9B0; continue 'dispatch;
	}
	// 830CE994: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830CE998: 5469843E  srwi r9, r3, 0x10
	ctx.r[9].u32 = ctx.r[3].u32.wrapping_shr(16);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 830CE99C: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 830CE9A0: 409A0010  bne cr6, 0x830ce9b0
	if !ctx.cr[6].eq {
	pc = 0x830CE9B0; continue 'dispatch;
	}
	// 830CE9A4: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830CE9A8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830CE9AC: 409A0010  bne cr6, 0x830ce9bc
	if !ctx.cr[6].eq {
	pc = 0x830CE9BC; continue 'dispatch;
	}
	// 830CE9B0: 3C608030  lis r3, -0x7fd0
	ctx.r[3].s64 = -2144337920;
	// 830CE9B4: 6063000A  ori r3, r3, 0xa
	ctx.r[3].u64 = ctx.r[3].u64 | 10;
	// 830CE9B8: 48000028  b 0x830ce9e0
	pc = 0x830CE9E0; continue 'dispatch;
	// 830CE9BC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 830CE9C0: 4BFFF7F9  bl 0x830ce1b8
	ctx.lr = 0x830CE9C4;
	sub_830CE1B8(ctx, base);
	// 830CE9C4: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 830CE9C8: 4180FF70  blt 0x830ce938
	if ctx.cr[0].lt {
	pc = 0x830CE938; continue 'dispatch;
	}
	// 830CE9CC: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 830CE9D0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 830CE9D4: 419AFF64  beq cr6, 0x830ce938
	if ctx.cr[6].eq {
	pc = 0x830CE938; continue 'dispatch;
	}
	// 830CE9D8: 7CE43B78  mr r4, r7
	ctx.r[4].u64 = ctx.r[7].u64;
	// 830CE9DC: 4BFFFE8D  bl 0x830ce868
	ctx.lr = 0x830CE9E0;
	sub_830CE868(ctx, base);
	// 830CE9E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830CE9E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830CE9E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830CE9EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CE9F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830CE9F0 size=20
    let mut pc: u32 = 0x830CE9F0;
    'dispatch: loop {
        match pc {
            0x830CE9F0 => {
    //   block [0x830CE9F0..0x830CEA04)
	// 830CE9F0: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 830CE9F4: 409A0010  bne cr6, 0x830cea04
	if !ctx.cr[6].eq {
		sub_830CEA04(ctx, base);
		return;
	}
	// 830CE9F8: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 830CE9FC: 60630057  ori r3, r3, 0x57
	ctx.r[3].u64 = ctx.r[3].u64 | 87;
	// 830CEA00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CEA04(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830CEA04 size=124
    let mut pc: u32 = 0x830CEA04;
    'dispatch: loop {
        match pc {
            0x830CEA04 => {
    //   block [0x830CEA04..0x830CEA80)
	// 830CEA04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 830CEA08: 546B043E  clrlwi r11, r3, 0x10
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x0000FFFFu64;
	// 830CEA0C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 830CEA10: 91050000  stw r8, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 830CEA14: 419A0054  beq cr6, 0x830cea68
	if ctx.cr[6].eq {
	pc = 0x830CEA68; continue 'dispatch;
	}
	// 830CEA18: 3D408339  lis r10, -0x7cc7
	ctx.r[10].s64 = -2093416448;
	// 830CEA1C: 556B043E  clrlwi r11, r11, 0x10
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 830CEA20: 394AC1B0  addi r10, r10, -0x3e50
	ctx.r[10].s64 = ctx.r[10].s64 + -15952;
	// 830CEA24: 812A0220  lwz r9, 0x220(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(544 as u32) ) } as u64;
	// 830CEA28: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 830CEA2C: 4098003C  bge cr6, 0x830cea68
	if !ctx.cr[6].lt {
	pc = 0x830CEA68; continue 'dispatch;
	}
	// 830CEA30: 5567C9FA  rlwinm r7, r11, 0x19, 7, 0x1d
	ctx.r[7].u64 = ctx.r[11].u32 as u64 & 0x0000007Fu64;
	// 830CEA34: 556B05FE  clrlwi r11, r11, 0x17
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000001FFu64;
	// 830CEA38: 1D2B000C  mulli r9, r11, 0xc
	ctx.r[9].s64 = ctx.r[11].s64 * 12;
	// 830CEA3C: 7D67502E  lwzx r11, r7, r10
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[7].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 830CEA40: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 830CEA44: A14B0002  lhz r10, 2(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(2 as u32) ) } as u64;
	// 830CEA48: 554A07FE  clrlwi r10, r10, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x00000001u64;
	// 830CEA4C: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 830CEA50: 409A0018  bne cr6, 0x830cea68
	if !ctx.cr[6].eq {
	pc = 0x830CEA68; continue 'dispatch;
	}
	// 830CEA54: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830CEA58: 5469843E  srwi r9, r3, 0x10
	ctx.r[9].u32 = ctx.r[3].u32.wrapping_shr(16);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 830CEA5C: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 830CEA60: 409A0008  bne cr6, 0x830cea68
	if !ctx.cr[6].eq {
	pc = 0x830CEA68; continue 'dispatch;
	}
	// 830CEA64: 810B0004  lwz r8, 4(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830CEA68: 7D074378  mr r7, r8
	ctx.r[7].u64 = ctx.r[8].u64;
	// 830CEA6C: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 830CEA70: 409A0010  bne cr6, 0x830cea80
	if !ctx.cr[6].eq {
		sub_830CEA80(ctx, base);
		return;
	}
	// 830CEA74: 3C608030  lis r3, -0x7fd0
	ctx.r[3].s64 = -2144337920;
	// 830CEA78: 6063000A  ori r3, r3, 0xa
	ctx.r[3].u64 = ctx.r[3].u64 | 10;
	// 830CEA7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CEA80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830CEA80 size=60
    let mut pc: u32 = 0x830CEA80;
    'dispatch: loop {
        match pc {
            0x830CEA80 => {
    //   block [0x830CEA80..0x830CEABC)
	// 830CEA80: 81680018  lwz r11, 0x18(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(24 as u32) ) } as u64;
	// 830CEA84: 81080020  lwz r8, 0x20(r8)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(32 as u32) ) } as u64;
	// 830CEA88: 814B002C  lwz r10, 0x2c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 830CEA8C: 7F045040  cmplw cr6, r4, r10
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[10].u32, &mut ctx.xer);
	// 830CEA90: 41980014  blt cr6, 0x830ceaa4
	if ctx.cr[6].lt {
	pc = 0x830CEAA4; continue 'dispatch;
	}
	// 830CEA94: 812B0030  lwz r9, 0x30(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 830CEA98: 7D495214  add r10, r9, r10
	ctx.r[10].u64 = ctx.r[9].u64 + ctx.r[10].u64;
	// 830CEA9C: 7F045040  cmplw cr6, r4, r10
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[10].u32, &mut ctx.xer);
	// 830CEAA0: 4198001C  blt cr6, 0x830ceabc
	if ctx.cr[6].lt {
		sub_830CEABC(ctx, base);
		return;
	}
	// 830CEAA4: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 830CEAA8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830CEAAC: 419A0038  beq cr6, 0x830ceae4
	if ctx.cr[6].eq {
		sub_830CEAE4(ctx, base);
		return;
	}
	// 830CEAB0: 80E70004  lwz r7, 4(r7)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(4 as u32) ) } as u64;
	// 830CEAB4: 81070020  lwz r8, 0x20(r7)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(32 as u32) ) } as u64;
	// 830CEAB8: 4BFFFFD0  b 0x830cea88
	pc = 0x830CEA88; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CEABC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830CEABC size=40
    let mut pc: u32 = 0x830CEABC;
    'dispatch: loop {
        match pc {
            0x830CEABC => {
    //   block [0x830CEABC..0x830CEAE4)
	// 830CEABC: 812B002C  lwz r9, 0x2c(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 830CEAC0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830CEAC4: 814B001C  lwz r10, 0x1c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 830CEAC8: 7D692050  subf r11, r9, r4
	ctx.r[11].s64 = ctx.r[4].s64 - ctx.r[9].s64;
	// 830CEACC: 1D6B0030  mulli r11, r11, 0x30
	ctx.r[11].s64 = ctx.r[11].s64 * 48;
	// 830CEAD0: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 830CEAD4: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 830CEAD8: 7D6B4214  add r11, r11, r8
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[8].u64;
	// 830CEADC: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830CEAE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CEAE4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830CEAE4 size=12
    let mut pc: u32 = 0x830CEAE4;
    'dispatch: loop {
        match pc {
            0x830CEAE4 => {
    //   block [0x830CEAE4..0x830CEAF0)
	// 830CEAE4: 3C608030  lis r3, -0x7fd0
	ctx.r[3].s64 = -2144337920;
	// 830CEAE8: 60630007  ori r3, r3, 7
	ctx.r[3].u64 = ctx.r[3].u64 | 7;
	// 830CEAEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CEAF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830CEAF0 size=108
    let mut pc: u32 = 0x830CEAF0;
    'dispatch: loop {
        match pc {
            0x830CEAF0 => {
    //   block [0x830CEAF0..0x830CEB5C)
	// 830CEAF0: 546B043E  clrlwi r11, r3, 0x10
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x0000FFFFu64;
	// 830CEAF4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 830CEAF8: 419A005C  beq cr6, 0x830ceb54
	if ctx.cr[6].eq {
	pc = 0x830CEB54; continue 'dispatch;
	}
	// 830CEAFC: 3D408339  lis r10, -0x7cc7
	ctx.r[10].s64 = -2093416448;
	// 830CEB00: 556B043E  clrlwi r11, r11, 0x10
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 830CEB04: 394AC1B0  addi r10, r10, -0x3e50
	ctx.r[10].s64 = ctx.r[10].s64 + -15952;
	// 830CEB08: 812A0220  lwz r9, 0x220(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(544 as u32) ) } as u64;
	// 830CEB0C: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 830CEB10: 40980044  bge cr6, 0x830ceb54
	if !ctx.cr[6].lt {
	pc = 0x830CEB54; continue 'dispatch;
	}
	// 830CEB14: 5568C9FA  rlwinm r8, r11, 0x19, 7, 0x1d
	ctx.r[8].u64 = ctx.r[11].u32 as u64 & 0x0000007Fu64;
	// 830CEB18: 556B05FE  clrlwi r11, r11, 0x17
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000001FFu64;
	// 830CEB1C: 1D2B000C  mulli r9, r11, 0xc
	ctx.r[9].s64 = ctx.r[11].s64 * 12;
	// 830CEB20: 7D68502E  lwzx r11, r8, r10
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 830CEB24: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 830CEB28: A14B0002  lhz r10, 2(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(2 as u32) ) } as u64;
	// 830CEB2C: 554A07FE  clrlwi r10, r10, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x00000001u64;
	// 830CEB30: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 830CEB34: 409A0020  bne cr6, 0x830ceb54
	if !ctx.cr[6].eq {
	pc = 0x830CEB54; continue 'dispatch;
	}
	// 830CEB38: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830CEB3C: 5469843E  srwi r9, r3, 0x10
	ctx.r[9].u32 = ctx.r[3].u32.wrapping_shr(16);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 830CEB40: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 830CEB44: 409A0010  bne cr6, 0x830ceb54
	if !ctx.cr[6].eq {
	pc = 0x830CEB54; continue 'dispatch;
	}
	// 830CEB48: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830CEB4C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830CEB50: 409A000C  bne cr6, 0x830ceb5c
	if !ctx.cr[6].eq {
		sub_830CEB5C(ctx, base);
		return;
	}
	// 830CEB54: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830CEB58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CEB5C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830CEB5C size=20
    let mut pc: u32 = 0x830CEB5C;
    'dispatch: loop {
        match pc {
            0x830CEB5C => {
    //   block [0x830CEB5C..0x830CEB70)
	// 830CEB5C: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 830CEB60: 7D6B2050  subf r11, r11, r4
	ctx.r[11].s64 = ctx.r[4].s64 - ctx.r[11].s64;
	// 830CEB64: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 830CEB68: 5563DFFE  rlwinm r3, r11, 0x1b, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 830CEB6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CEB70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830CEB70 size=16
    let mut pc: u32 = 0x830CEB70;
    'dispatch: loop {
        match pc {
            0x830CEB70 => {
    //   block [0x830CEB70..0x830CEB80)
	// 830CEB70: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 830CEB74: 409A000C  bne cr6, 0x830ceb80
	if !ctx.cr[6].eq {
		sub_830CEB80(ctx, base);
		return;
	}
	// 830CEB78: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830CEB7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CEB80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830CEB80 size=132
    let mut pc: u32 = 0x830CEB80;
    'dispatch: loop {
        match pc {
            0x830CEB80 => {
    //   block [0x830CEB80..0x830CEC04)
	// 830CEB80: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 830CEB84: 546B043E  clrlwi r11, r3, 0x10
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x0000FFFFu64;
	// 830CEB88: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 830CEB8C: 419A0054  beq cr6, 0x830cebe0
	if ctx.cr[6].eq {
	pc = 0x830CEBE0; continue 'dispatch;
	}
	// 830CEB90: 3D408339  lis r10, -0x7cc7
	ctx.r[10].s64 = -2093416448;
	// 830CEB94: 556B043E  clrlwi r11, r11, 0x10
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 830CEB98: 394AC1B0  addi r10, r10, -0x3e50
	ctx.r[10].s64 = ctx.r[10].s64 + -15952;
	// 830CEB9C: 812A0220  lwz r9, 0x220(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(544 as u32) ) } as u64;
	// 830CEBA0: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 830CEBA4: 4098003C  bge cr6, 0x830cebe0
	if !ctx.cr[6].lt {
	pc = 0x830CEBE0; continue 'dispatch;
	}
	// 830CEBA8: 5567C9FA  rlwinm r7, r11, 0x19, 7, 0x1d
	ctx.r[7].u64 = ctx.r[11].u32 as u64 & 0x0000007Fu64;
	// 830CEBAC: 556B05FE  clrlwi r11, r11, 0x17
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000001FFu64;
	// 830CEBB0: 1D2B000C  mulli r9, r11, 0xc
	ctx.r[9].s64 = ctx.r[11].s64 * 12;
	// 830CEBB4: 7D67502E  lwzx r11, r7, r10
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[7].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 830CEBB8: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 830CEBBC: A14B0002  lhz r10, 2(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(2 as u32) ) } as u64;
	// 830CEBC0: 554A07FE  clrlwi r10, r10, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x00000001u64;
	// 830CEBC4: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 830CEBC8: 409A0018  bne cr6, 0x830cebe0
	if !ctx.cr[6].eq {
	pc = 0x830CEBE0; continue 'dispatch;
	}
	// 830CEBCC: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830CEBD0: 5469843E  srwi r9, r3, 0x10
	ctx.r[9].u32 = ctx.r[3].u32.wrapping_shr(16);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 830CEBD4: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 830CEBD8: 409A0008  bne cr6, 0x830cebe0
	if !ctx.cr[6].eq {
	pc = 0x830CEBE0; continue 'dispatch;
	}
	// 830CEBDC: 810B0004  lwz r8, 4(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830CEBE0: 7D0B4378  mr r11, r8
	ctx.r[11].u64 = ctx.r[8].u64;
	// 830CEBE4: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 830CEBE8: 419AFF90  beq cr6, 0x830ceb78
	if ctx.cr[6].eq {
		sub_830CEB70(ctx, base);
		return;
	}
	// 830CEBEC: 814B0018  lwz r10, 0x18(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 830CEBF0: 7F0A2040  cmplw cr6, r10, r4
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[4].u32, &mut ctx.xer);
	// 830CEBF4: 419A0010  beq cr6, 0x830cec04
	if ctx.cr[6].eq {
		sub_830CEC04(ctx, base);
		return;
	}
	// 830CEBF8: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830CEBFC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830CEC00: 4BFFFFE8  b 0x830cebe8
	pc = 0x830CEBE8; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CEC04(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830CEC04 size=8
    let mut pc: u32 = 0x830CEC04;
    'dispatch: loop {
        match pc {
            0x830CEC04 => {
    //   block [0x830CEC04..0x830CEC0C)
	// 830CEC04: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830CEC08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CEC10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830CEC10 size=108
    let mut pc: u32 = 0x830CEC10;
    'dispatch: loop {
        match pc {
            0x830CEC10 => {
    //   block [0x830CEC10..0x830CEC7C)
	// 830CEC10: 546B043E  clrlwi r11, r3, 0x10
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x0000FFFFu64;
	// 830CEC14: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 830CEC18: 419A005C  beq cr6, 0x830cec74
	if ctx.cr[6].eq {
	pc = 0x830CEC74; continue 'dispatch;
	}
	// 830CEC1C: 3D408339  lis r10, -0x7cc7
	ctx.r[10].s64 = -2093416448;
	// 830CEC20: 556B043E  clrlwi r11, r11, 0x10
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 830CEC24: 394AC1B0  addi r10, r10, -0x3e50
	ctx.r[10].s64 = ctx.r[10].s64 + -15952;
	// 830CEC28: 812A0220  lwz r9, 0x220(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(544 as u32) ) } as u64;
	// 830CEC2C: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 830CEC30: 40980044  bge cr6, 0x830cec74
	if !ctx.cr[6].lt {
	pc = 0x830CEC74; continue 'dispatch;
	}
	// 830CEC34: 5568C9FA  rlwinm r8, r11, 0x19, 7, 0x1d
	ctx.r[8].u64 = ctx.r[11].u32 as u64 & 0x0000007Fu64;
	// 830CEC38: 556B05FE  clrlwi r11, r11, 0x17
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000001FFu64;
	// 830CEC3C: 1D2B000C  mulli r9, r11, 0xc
	ctx.r[9].s64 = ctx.r[11].s64 * 12;
	// 830CEC40: 7D68502E  lwzx r11, r8, r10
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 830CEC44: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 830CEC48: A14B0002  lhz r10, 2(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(2 as u32) ) } as u64;
	// 830CEC4C: 554A07FE  clrlwi r10, r10, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x00000001u64;
	// 830CEC50: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 830CEC54: 409A0020  bne cr6, 0x830cec74
	if !ctx.cr[6].eq {
	pc = 0x830CEC74; continue 'dispatch;
	}
	// 830CEC58: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830CEC5C: 5469843E  srwi r9, r3, 0x10
	ctx.r[9].u32 = ctx.r[3].u32.wrapping_shr(16);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 830CEC60: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 830CEC64: 409A0010  bne cr6, 0x830cec74
	if !ctx.cr[6].eq {
	pc = 0x830CEC74; continue 'dispatch;
	}
	// 830CEC68: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830CEC6C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830CEC70: 409A000C  bne cr6, 0x830cec7c
	if !ctx.cr[6].eq {
		sub_830CEC7C(ctx, base);
		return;
	}
	// 830CEC74: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830CEC78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CEC7C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830CEC7C size=12
    let mut pc: u32 = 0x830CEC7C;
    'dispatch: loop {
        match pc {
            0x830CEC7C => {
    //   block [0x830CEC7C..0x830CEC88)
	// 830CEC7C: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 830CEC80: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830CEC84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CEC88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830CEC88 size=124
    let mut pc: u32 = 0x830CEC88;
    'dispatch: loop {
        match pc {
            0x830CEC88 => {
    //   block [0x830CEC88..0x830CED04)
	// 830CEC88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830CEC8C: 480D94DD  bl 0x831a8168
	ctx.lr = 0x830CEC90;
	sub_831A8130(ctx, base);
	// 830CEC90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830CEC94: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 830CEC98: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 830CEC9C: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 830CECA0: 419A0048  beq cr6, 0x830cece8
	if ctx.cr[6].eq {
	pc = 0x830CECE8; continue 'dispatch;
	}
	// 830CECA4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830CECA8: 480DA421  bl 0x831a90c8
	ctx.lr = 0x830CECAC;
	sub_831A90C8(ctx, base);
	// 830CECAC: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 830CECB0: 41820038  beq 0x830cece8
	if ctx.cr[0].eq {
	pc = 0x830CECE8; continue 'dispatch;
	}
	// 830CECB4: 397E0001  addi r11, r30, 1
	ctx.r[11].s64 = ctx.r[30].s64 + 1;
	// 830CECB8: 5563083C  slwi r3, r11, 1
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 830CECBC: 4800E7F5  bl 0x830dd4b0
	ctx.lr = 0x830CECC0;
	sub_830DD4B0(ctx, base);
	// 830CECC0: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 830CECC4: 40820010  bne 0x830cecd4
	if !ctx.cr[0].eq {
	pc = 0x830CECD4; continue 'dispatch;
	}
	// 830CECC8: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 830CECCC: 6063000E  ori r3, r3, 0xe
	ctx.r[3].u64 = ctx.r[3].u64 | 14;
	// 830CECD0: 4800002C  b 0x830cecfc
	pc = 0x830CECFC; continue 'dispatch;
	// 830CECD4: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 830CECD8: 389E0001  addi r4, r30, 1
	ctx.r[4].s64 = ctx.r[30].s64 + 1;
	// 830CECDC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830CECE0: 480E31D9  bl 0x831b1eb8
	ctx.lr = 0x830CECE4;
	sub_831B1EB8(ctx, base);
	// 830CECE4: 48000008  b 0x830cecec
	pc = 0x830CECEC; continue 'dispatch;
	// 830CECE8: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 830CECEC: 807C0000  lwz r3, 0(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 830CECF0: 4800E7E9  bl 0x830dd4d8
	ctx.lr = 0x830CECF4;
	sub_830DD4D8(ctx, base);
	// 830CECF4: 93FC0000  stw r31, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 830CECF8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830CECFC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 830CED00: 480D94B8  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CED08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830CED08 size=12
    let mut pc: u32 = 0x830CED08;
    'dispatch: loop {
        match pc {
            0x830CED08 => {
    //   block [0x830CED08..0x830CED14)
	// 830CED08: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830CED0C: 806BC188  lwz r3, -0x3e78(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-15992 as u32) ) } as u64;
	// 830CED10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CED18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830CED18 size=12
    let mut pc: u32 = 0x830CED18;
    'dispatch: loop {
        match pc {
            0x830CED18 => {
    //   block [0x830CED18..0x830CED24)
	// 830CED18: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830CED1C: 806BC18C  lwz r3, -0x3e74(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-15988 as u32) ) } as u64;
	// 830CED20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CED28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830CED28 size=32
    let mut pc: u32 = 0x830CED28;
    'dispatch: loop {
        match pc {
            0x830CED28 => {
    //   block [0x830CED28..0x830CED48)
	// 830CED28: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 830CED2C: 7C691B78  mr r9, r3
	ctx.r[9].u64 = ctx.r[3].u64;
	// 830CED30: 386B0008  addi r3, r11, 8
	ctx.r[3].s64 = ctx.r[11].s64 + 8;
	// 830CED34: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 830CED38: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 830CED3C: 419A000C  beq cr6, 0x830ced48
	if ctx.cr[6].eq {
		sub_830CED48(ctx, base);
		return;
	}
	// 830CED40: 91440000  stw r10, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 830CED44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CED48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830CED48 size=80
    let mut pc: u32 = 0x830CED48;
    'dispatch: loop {
        match pc {
            0x830CED48 => {
    //   block [0x830CED48..0x830CED98)
	// 830CED48: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830CED4C: 3940001B  li r10, 0x1b
	ctx.r[10].s64 = 27;
	// 830CED50: 7D4B5396  divwu r10, r11, r10
	ctx.r[10].u32 = ctx.r[11].u32 / ctx.r[10].u32;
	// 830CED54: 1D4A001B  mulli r10, r10, 0x1b
	ctx.r[10].s64 = ctx.r[10].s64 * 27;
	// 830CED58: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 830CED5C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 830CED60: 2F0B001B  cmpwi cr6, r11, 0x1b
	ctx.cr[6].compare_i32(ctx.r[11].s32, 27, &mut ctx.xer);
	// 830CED64: 40980028  bge cr6, 0x830ced8c
	if !ctx.cr[6].lt {
	pc = 0x830CED8C; continue 'dispatch;
	}
	// 830CED68: 556A103A  slwi r10, r11, 2
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 830CED6C: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 830CED70: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 830CED74: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 830CED78: 409A0020  bne cr6, 0x830ced98
	if !ctx.cr[6].eq {
		sub_830CED98(ctx, base);
		return;
	}
	// 830CED7C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 830CED80: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 830CED84: 2F0B001B  cmpwi cr6, r11, 0x1b
	ctx.cr[6].compare_i32(ctx.r[11].s32, 27, &mut ctx.xer);
	// 830CED88: 4198FFE8  blt cr6, 0x830ced70
	if ctx.cr[6].lt {
	pc = 0x830CED70; continue 'dispatch;
	}
	// 830CED8C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830CED90: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830CED94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CED98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830CED98 size=12
    let mut pc: u32 = 0x830CED98;
    'dispatch: loop {
        match pc {
            0x830CED98 => {
    //   block [0x830CED98..0x830CEDA4)
	// 830CED98: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 830CED9C: 7D6B482E  lwzx r11, r11, r9
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 830CEDA0: 4BFFFFF0  b 0x830ced90
	sub_830CED48(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CEDA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830CEDA8 size=8
    let mut pc: u32 = 0x830CEDA8;
    'dispatch: loop {
        match pc {
            0x830CEDA8 => {
    //   block [0x830CEDA8..0x830CEDB0)
	// 830CEDA8: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 830CEDAC: 4802A2C4  b 0x830f9070
	sub_830F9070(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CEDB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830CEDB0 size=8
    let mut pc: u32 = 0x830CEDB0;
    'dispatch: loop {
        match pc {
            0x830CEDB0 => {
    //   block [0x830CEDB0..0x830CEDB8)
	// 830CEDB0: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 830CEDB4: 4801D53C  b 0x830ec2f0
	sub_830EC2F0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CEDB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830CEDB8 size=8
    let mut pc: u32 = 0x830CEDB8;
    'dispatch: loop {
        match pc {
            0x830CEDB8 => {
    //   block [0x830CEDB8..0x830CEDC0)
	// 830CEDB8: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 830CEDBC: 4BFFEA14  b 0x830cd7d0
	sub_830CD7D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CEDC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830CEDC0 size=40
    let mut pc: u32 = 0x830CEDC0;
    'dispatch: loop {
        match pc {
            0x830CEDC0 => {
    //   block [0x830CEDC0..0x830CEDE8)
	// 830CEDC0: 81640004  lwz r11, 4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 830CEDC4: 2B0B000B  cmplwi cr6, r11, 0xb
	ctx.cr[6].compare_u32(ctx.r[11].u32, 11 as u32, &mut ctx.xer);
	// 830CEDC8: 409A0018  bne cr6, 0x830cede0
	if !ctx.cr[6].eq {
	pc = 0x830CEDE0; continue 'dispatch;
	}
	// 830CEDCC: 81640010  lwz r11, 0x10(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(16 as u32) ) } as u64;
	// 830CEDD0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 830CEDD4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 830CEDD8: 91440008  stw r10, 8(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 830CEDDC: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 830CEDE0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830CEDE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CEDE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830CEDE8 size=8
    let mut pc: u32 = 0x830CEDE8;
    'dispatch: loop {
        match pc {
            0x830CEDE8 => {
    //   block [0x830CEDE8..0x830CEDF0)
	// 830CEDE8: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 830CEDEC: 480282EC  b 0x830f70d8
	sub_830F70D8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CEDF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830CEDF0 size=8
    let mut pc: u32 = 0x830CEDF0;
    'dispatch: loop {
        match pc {
            0x830CEDF0 => {
    //   block [0x830CEDF0..0x830CEDF8)
	// 830CEDF0: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 830CEDF4: 4801B024  b 0x830e9e18
	sub_830E9E18(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CEDF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830CEDF8 size=8
    let mut pc: u32 = 0x830CEDF8;
    'dispatch: loop {
        match pc {
            0x830CEDF8 => {
    //   block [0x830CEDF8..0x830CEE00)
	// 830CEDF8: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 830CEDFC: 4801C57C  b 0x830eb378
	sub_830EB378(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CEE00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830CEE00 size=8
    let mut pc: u32 = 0x830CEE00;
    'dispatch: loop {
        match pc {
            0x830CEE00 => {
    //   block [0x830CEE00..0x830CEE08)
	// 830CEE00: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 830CEE04: 4802AC6C  b 0x830f9a70
	sub_830F9A70(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CEE08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830CEE08 size=8
    let mut pc: u32 = 0x830CEE08;
    'dispatch: loop {
        match pc {
            0x830CEE08 => {
    //   block [0x830CEE08..0x830CEE10)
	// 830CEE08: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 830CEE0C: 4801BEB4  b 0x830eacc0
	sub_830EACC0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CEE10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830CEE10 size=8
    let mut pc: u32 = 0x830CEE10;
    'dispatch: loop {
        match pc {
            0x830CEE10 => {
    //   block [0x830CEE10..0x830CEE18)
	// 830CEE10: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 830CEE14: 4802876C  b 0x830f7580
	sub_830F7580(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CEE18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830CEE18 size=8
    let mut pc: u32 = 0x830CEE18;
    'dispatch: loop {
        match pc {
            0x830CEE18 => {
    //   block [0x830CEE18..0x830CEE20)
	// 830CEE18: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 830CEE1C: 4801C16C  b 0x830eaf88
	sub_830EAF88(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CEE20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830CEE20 size=8
    let mut pc: u32 = 0x830CEE20;
    'dispatch: loop {
        match pc {
            0x830CEE20 => {
    //   block [0x830CEE20..0x830CEE28)
	// 830CEE20: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 830CEE24: 4801CFEC  b 0x830ebe10
	sub_830EBE10(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CEE28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830CEE28 size=8
    let mut pc: u32 = 0x830CEE28;
    'dispatch: loop {
        match pc {
            0x830CEE28 => {
    //   block [0x830CEE28..0x830CEE30)
	// 830CEE28: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 830CEE2C: 4801D264  b 0x830ec090
	sub_830EC090(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CEE30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830CEE30 size=36
    let mut pc: u32 = 0x830CEE30;
    'dispatch: loop {
        match pc {
            0x830CEE30 => {
    //   block [0x830CEE30..0x830CEE54)
	// 830CEE30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830CEE34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830CEE38: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830CEE3C: 4800E69D  bl 0x830dd4d8
	ctx.lr = 0x830CEE40;
	sub_830DD4D8(ctx, base);
	// 830CEE40: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830CEE44: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830CEE48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830CEE4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830CEE50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CEE58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830CEE58 size=8
    let mut pc: u32 = 0x830CEE58;
    'dispatch: loop {
        match pc {
            0x830CEE58 => {
    //   block [0x830CEE58..0x830CEE60)
	// 830CEE58: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 830CEE5C: 48033434  b 0x83102290
	sub_83102290(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CEE60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830CEE60 size=8
    let mut pc: u32 = 0x830CEE60;
    'dispatch: loop {
        match pc {
            0x830CEE60 => {
    //   block [0x830CEE60..0x830CEE68)
	// 830CEE60: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 830CEE64: 48028734  b 0x830f7598
	sub_830F7598(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CEE68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830CEE68 size=8
    let mut pc: u32 = 0x830CEE68;
    'dispatch: loop {
        match pc {
            0x830CEE68 => {
    //   block [0x830CEE68..0x830CEE70)
	// 830CEE68: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 830CEE6C: 4801D76C  b 0x830ec5d8
	sub_830EC5D8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CEE70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830CEE70 size=8
    let mut pc: u32 = 0x830CEE70;
    'dispatch: loop {
        match pc {
            0x830CEE70 => {
    //   block [0x830CEE70..0x830CEE78)
	// 830CEE70: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 830CEE74: 4801DB54  b 0x830ec9c8
	sub_830EC9C8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CEE78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830CEE78 size=8
    let mut pc: u32 = 0x830CEE78;
    'dispatch: loop {
        match pc {
            0x830CEE78 => {
    //   block [0x830CEE78..0x830CEE80)
	// 830CEE78: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 830CEE7C: 4802274C  b 0x830f15c8
	sub_830F15C8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CEE80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830CEE80 size=8
    let mut pc: u32 = 0x830CEE80;
    'dispatch: loop {
        match pc {
            0x830CEE80 => {
    //   block [0x830CEE80..0x830CEE88)
	// 830CEE80: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 830CEE84: 4802D134  b 0x830fbfb8
	sub_830FBFB8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CEE88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830CEE88 size=8
    let mut pc: u32 = 0x830CEE88;
    'dispatch: loop {
        match pc {
            0x830CEE88 => {
    //   block [0x830CEE88..0x830CEE90)
	// 830CEE88: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 830CEE8C: 4802FBE4  b 0x830fea70
	sub_830FEA70(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CEE90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830CEE90 size=8
    let mut pc: u32 = 0x830CEE90;
    'dispatch: loop {
        match pc {
            0x830CEE90 => {
    //   block [0x830CEE90..0x830CEE98)
	// 830CEE90: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 830CEE94: 48029A34  b 0x830f88c8
	sub_830F88C8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CEE98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830CEE98 size=8
    let mut pc: u32 = 0x830CEE98;
    'dispatch: loop {
        match pc {
            0x830CEE98 => {
    //   block [0x830CEE98..0x830CEEA0)
	// 830CEE98: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 830CEE9C: 48029FCC  b 0x830f8e68
	sub_830F8E68(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CEEA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830CEEA0 size=8
    let mut pc: u32 = 0x830CEEA0;
    'dispatch: loop {
        match pc {
            0x830CEEA0 => {
    //   block [0x830CEEA0..0x830CEEA8)
	// 830CEEA0: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 830CEEA4: 48024F6C  b 0x830f3e10
	sub_830F3E10(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CEEA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830CEEA8 size=8
    let mut pc: u32 = 0x830CEEA8;
    'dispatch: loop {
        match pc {
            0x830CEEA8 => {
    //   block [0x830CEEA8..0x830CEEB0)
	// 830CEEA8: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 830CEEAC: 48025254  b 0x830f4100
	sub_830F4100(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CEEB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830CEEB0 size=8
    let mut pc: u32 = 0x830CEEB0;
    'dispatch: loop {
        match pc {
            0x830CEEB0 => {
    //   block [0x830CEEB0..0x830CEEB8)
	// 830CEEB0: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 830CEEB4: 4802EFAC  b 0x830fde60
	sub_830FDE60(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CEEB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830CEEB8 size=8
    let mut pc: u32 = 0x830CEEB8;
    'dispatch: loop {
        match pc {
            0x830CEEB8 => {
    //   block [0x830CEEB8..0x830CEEC0)
	// 830CEEB8: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 830CEEBC: 480175B4  b 0x830e6470
	sub_830E6470(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CEEC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830CEEC0 size=8
    let mut pc: u32 = 0x830CEEC0;
    'dispatch: loop {
        match pc {
            0x830CEEC0 => {
    //   block [0x830CEEC0..0x830CEEC8)
	// 830CEEC0: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 830CEEC4: 4801FB44  b 0x830eea08
	sub_830EEA08(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CEEC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830CEEC8 size=8
    let mut pc: u32 = 0x830CEEC8;
    'dispatch: loop {
        match pc {
            0x830CEEC8 => {
    //   block [0x830CEEC8..0x830CEED0)
	// 830CEEC8: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 830CEECC: 48025F44  b 0x830f4e10
	sub_830F4E10(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CEED0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830CEED0 size=8
    let mut pc: u32 = 0x830CEED0;
    'dispatch: loop {
        match pc {
            0x830CEED0 => {
    //   block [0x830CEED0..0x830CEED8)
	// 830CEED0: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 830CEED4: 48026074  b 0x830f4f48
	sub_830F4F48(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CEED8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830CEED8 size=8
    let mut pc: u32 = 0x830CEED8;
    'dispatch: loop {
        match pc {
            0x830CEED8 => {
    //   block [0x830CEED8..0x830CEEE0)
	// 830CEED8: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 830CEEDC: 48020194  b 0x830ef070
	sub_830EF070(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CEEE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830CEEE0 size=8
    let mut pc: u32 = 0x830CEEE0;
    'dispatch: loop {
        match pc {
            0x830CEEE0 => {
    //   block [0x830CEEE0..0x830CEEE8)
	// 830CEEE0: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 830CEEE4: 480264BC  b 0x830f53a0
	sub_830F53A0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CEEE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830CEEE8 size=8
    let mut pc: u32 = 0x830CEEE8;
    'dispatch: loop {
        match pc {
            0x830CEEE8 => {
    //   block [0x830CEEE8..0x830CEEF0)
	// 830CEEE8: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 830CEEEC: 4802080C  b 0x830ef6f8
	sub_830EF6F8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CEEF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830CEEF0 size=8
    let mut pc: u32 = 0x830CEEF0;
    'dispatch: loop {
        match pc {
            0x830CEEF0 => {
    //   block [0x830CEEF0..0x830CEEF8)
	// 830CEEF0: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 830CEEF4: 48020904  b 0x830ef7f8
	sub_830EF7F8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CEEF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830CEEF8 size=148
    let mut pc: u32 = 0x830CEEF8;
    'dispatch: loop {
        match pc {
            0x830CEEF8 => {
    //   block [0x830CEEF8..0x830CEF8C)
	// 830CEEF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830CEEFC: 480D9271  bl 0x831a816c
	ctx.lr = 0x830CEF00;
	sub_831A8130(ctx, base);
	// 830CEF00: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830CEF04: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 830CEF08: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 830CEF0C: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 830CEF10: A15D0000  lhz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 830CEF14: 48000014  b 0x830cef28
	pc = 0x830CEF28; continue 'dispatch;
	// 830CEF18: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 830CEF1C: 1D3F0021  mulli r9, r31, 0x21
	ctx.r[9].s64 = ctx.r[31].s64 * 33;
	// 830CEF20: 7FE95214  add r31, r9, r10
	ctx.r[31].u64 = ctx.r[9].u64 + ctx.r[10].u64;
	// 830CEF24: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 830CEF28: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830CEF2C: 4082FFEC  bne 0x830cef18
	if !ctx.cr[0].eq {
	pc = 0x830CEF18; continue 'dispatch;
	}
	// 830CEF30: 3960001B  li r11, 0x1b
	ctx.r[11].s64 = 27;
	// 830CEF34: 7D7F5B96  divwu r11, r31, r11
	ctx.r[11].u32 = ctx.r[31].u32 / ctx.r[11].u32;
	// 830CEF38: 1D6B001B  mulli r11, r11, 0x1b
	ctx.r[11].s64 = ctx.r[11].s64 * 27;
	// 830CEF3C: 7D6BF850  subf r11, r11, r31
	ctx.r[11].s64 = ctx.r[31].s64 - ctx.r[11].s64;
	// 830CEF40: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 830CEF44: 7FCB182E  lwzx r30, r11, r3
	ctx.r[30].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[3].u32)) } as u64;
	// 830CEF48: 48000028  b 0x830cef70
	pc = 0x830CEF70; continue 'dispatch;
	// 830CEF4C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 830CEF50: 7F0BF840  cmplw cr6, r11, r31
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[31].u32, &mut ctx.xer);
	// 830CEF54: 409A0018  bne cr6, 0x830cef6c
	if !ctx.cr[6].eq {
	pc = 0x830CEF6C; continue 'dispatch;
	}
	// 830CEF58: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 830CEF5C: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830CEF60: 480E2ED1  bl 0x831b1e30
	ctx.lr = 0x830CEF64;
	sub_831B1E30(ctx, base);
	// 830CEF64: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 830CEF68: 4182001C  beq 0x830cef84
	if ctx.cr[0].eq {
	pc = 0x830CEF84; continue 'dispatch;
	}
	// 830CEF6C: 83DE000C  lwz r30, 0xc(r30)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 830CEF70: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 830CEF74: 409AFFD8  bne cr6, 0x830cef4c
	if !ctx.cr[6].eq {
	pc = 0x830CEF4C; continue 'dispatch;
	}
	// 830CEF78: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830CEF7C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830CEF80: 480D923C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 830CEF84: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830CEF88: 4BFFFFF4  b 0x830cef7c
	pc = 0x830CEF7C; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CEF90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830CEF90 size=132
    let mut pc: u32 = 0x830CEF90;
    'dispatch: loop {
        match pc {
            0x830CEF90 => {
    //   block [0x830CEF90..0x830CF014)
	// 830CEF90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830CEF94: 480D91D9  bl 0x831a816c
	ctx.lr = 0x830CEF98;
	sub_831A8130(ctx, base);
	// 830CEF98: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830CEF9C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830CEFA0: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 830CEFA4: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830CEFA8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 830CEFAC: 419A0010  beq cr6, 0x830cefbc
	if ctx.cr[6].eq {
	pc = 0x830CEFBC; continue 'dispatch;
	}
	// 830CEFB0: 4800E529  bl 0x830dd4d8
	ctx.lr = 0x830CEFB4;
	sub_830DD4D8(ctx, base);
	// 830CEFB4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830CEFB8: 917E0004  stw r11, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 830CEFBC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830CEFC0: 480DA109  bl 0x831a90c8
	ctx.lr = 0x830CEFC4;
	sub_831A90C8(ctx, base);
	// 830CEFC4: 35630001  addic. r11, r3, 1
	ctx.xer.ca = (ctx.r[3].u32 > (!(1 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830CEFC8: 41820040  beq 0x830cf008
	if ctx.cr[0].eq {
	pc = 0x830CF008; continue 'dispatch;
	}
	// 830CEFCC: 3D407FFF  lis r10, 0x7fff
	ctx.r[10].s64 = 2147418112;
	// 830CEFD0: 614AFFFF  ori r10, r10, 0xffff
	ctx.r[10].u64 = ctx.r[10].u64 | 65535;
	// 830CEFD4: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 830CEFD8: 40980030  bge cr6, 0x830cf008
	if !ctx.cr[6].lt {
	pc = 0x830CF008; continue 'dispatch;
	}
	// 830CEFDC: 557F083C  slwi r31, r11, 1
	ctx.r[31].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[31].u64 = ctx.r[31].u32 as u64;
	// 830CEFE0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830CEFE4: 4800E4CD  bl 0x830dd4b0
	ctx.lr = 0x830CEFE8;
	sub_830DD4B0(ctx, base);
	// 830CEFE8: 907E0004  stw r3, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 830CEFEC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830CEFF0: 41820018  beq 0x830cf008
	if ctx.cr[0].eq {
	pc = 0x830CF008; continue 'dispatch;
	}
	// 830CEFF4: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 830CEFF8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 830CEFFC: 480D9515  bl 0x831a8510
	ctx.lr = 0x830CF000;
	sub_831A8510(ctx, base);
	// 830CF000: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 830CF004: 48000008  b 0x830cf00c
	pc = 0x830CF00C; continue 'dispatch;
	// 830CF008: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830CF00C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830CF010: 480D91AC  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CF018(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830CF018 size=104
    let mut pc: u32 = 0x830CF018;
    'dispatch: loop {
        match pc {
            0x830CF018 => {
    //   block [0x830CF018..0x830CF080)
	// 830CF018: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830CF01C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830CF020: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830CF024: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830CF028: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830CF02C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830CF030: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 830CF034: 807F0020  lwz r3, 0x20(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 830CF038: 4800E4A1  bl 0x830dd4d8
	ctx.lr = 0x830CF03C;
	sub_830DD4D8(ctx, base);
	// 830CF03C: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 830CF040: 4800E499  bl 0x830dd4d8
	ctx.lr = 0x830CF044;
	sub_830DD4D8(ctx, base);
	// 830CF044: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 830CF048: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 830CF04C: 419A0008  beq cr6, 0x830cf054
	if ctx.cr[6].eq {
	pc = 0x830CF054; continue 'dispatch;
	}
	// 830CF050: 4803B081  bl 0x8310a0d0
	ctx.lr = 0x830CF054;
	sub_8310A0D0(ctx, base);
	// 830CF054: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830CF058: 4182000C  beq 0x830cf064
	if ctx.cr[0].eq {
	pc = 0x830CF064; continue 'dispatch;
	}
	// 830CF05C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830CF060: 4800E479  bl 0x830dd4d8
	ctx.lr = 0x830CF064;
	sub_830DD4D8(ctx, base);
	// 830CF064: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830CF068: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830CF06C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830CF070: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830CF074: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830CF078: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830CF07C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CF080(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830CF080 size=96
    let mut pc: u32 = 0x830CF080;
    'dispatch: loop {
        match pc {
            0x830CF080 => {
    //   block [0x830CF080..0x830CF0E0)
	// 830CF080: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830CF084: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830CF088: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830CF08C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830CF090: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830CF094: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830CF098: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 830CF09C: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 830CF0A0: 4800E439  bl 0x830dd4d8
	ctx.lr = 0x830CF0A4;
	sub_830DD4D8(ctx, base);
	// 830CF0A4: 807F001C  lwz r3, 0x1c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 830CF0A8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 830CF0AC: 419A0008  beq cr6, 0x830cf0b4
	if ctx.cr[6].eq {
	pc = 0x830CF0B4; continue 'dispatch;
	}
	// 830CF0B0: 480413B1  bl 0x83110460
	ctx.lr = 0x830CF0B4;
	sub_83110460(ctx, base);
	// 830CF0B4: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830CF0B8: 4182000C  beq 0x830cf0c4
	if ctx.cr[0].eq {
	pc = 0x830CF0C4; continue 'dispatch;
	}
	// 830CF0BC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830CF0C0: 4800E419  bl 0x830dd4d8
	ctx.lr = 0x830CF0C4;
	sub_830DD4D8(ctx, base);
	// 830CF0C4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830CF0C8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830CF0CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830CF0D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830CF0D4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830CF0D8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830CF0DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CF0E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830CF0E0 size=96
    let mut pc: u32 = 0x830CF0E0;
    'dispatch: loop {
        match pc {
            0x830CF0E0 => {
    //   block [0x830CF0E0..0x830CF140)
	// 830CF0E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830CF0E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830CF0E8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830CF0EC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830CF0F0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830CF0F4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830CF0F8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 830CF0FC: 807F00F8  lwz r3, 0xf8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(248 as u32) ) } as u64;
	// 830CF100: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 830CF104: 419A0008  beq cr6, 0x830cf10c
	if ctx.cr[6].eq {
	pc = 0x830CF10C; continue 'dispatch;
	}
	// 830CF108: 4800E3D1  bl 0x830dd4d8
	ctx.lr = 0x830CF10C;
	sub_830DD4D8(ctx, base);
	// 830CF10C: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 830CF110: 48047CC1  bl 0x83116dd0
	ctx.lr = 0x830CF114;
	sub_83116DD0(ctx, base);
	// 830CF114: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830CF118: 4182000C  beq 0x830cf124
	if ctx.cr[0].eq {
	pc = 0x830CF124; continue 'dispatch;
	}
	// 830CF11C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830CF120: 4800E3B9  bl 0x830dd4d8
	ctx.lr = 0x830CF124;
	sub_830DD4D8(ctx, base);
	// 830CF124: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830CF128: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830CF12C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830CF130: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830CF134: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830CF138: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830CF13C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CF140(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830CF140 size=76
    let mut pc: u32 = 0x830CF140;
    'dispatch: loop {
        match pc {
            0x830CF140 => {
    //   block [0x830CF140..0x830CF18C)
	// 830CF140: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830CF144: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830CF148: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830CF14C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830CF150: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 830CF154: 3BE30004  addi r31, r3, 4
	ctx.r[31].s64 = ctx.r[3].s64 + 4;
	// 830CF158: 409A0008  bne cr6, 0x830cf160
	if !ctx.cr[6].eq {
	pc = 0x830CF160; continue 'dispatch;
	}
	// 830CF15C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 830CF160: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 830CF164: 4800E375  bl 0x830dd4d8
	ctx.lr = 0x830CF168;
	sub_830DD4D8(ctx, base);
	// 830CF168: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 830CF16C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 830CF170: 419A0008  beq cr6, 0x830cf178
	if ctx.cr[6].eq {
	pc = 0x830CF178; continue 'dispatch;
	}
	// 830CF174: 4803AF5D  bl 0x8310a0d0
	ctx.lr = 0x830CF178;
	sub_8310A0D0(ctx, base);
	// 830CF178: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830CF17C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830CF180: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830CF184: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830CF188: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CF190(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x830CF190 size=112
    let mut pc: u32 = 0x830CF190;
    'dispatch: loop {
        match pc {
            0x830CF190 => {
    //   block [0x830CF190..0x830CF200)
	// 830CF190: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830CF194: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830CF198: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830CF19C: 3981FFF0  addi r12, r1, -0x10
	ctx.r[12].s64 = ctx.r[1].s64 + -16;
	// 830CF1A0: 480D98D9  bl 0x831a8a78
	ctx.lr = 0x830CF1A4;
	sub_831A8A40(ctx, base);
	// 830CF1A4: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830CF1A8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 830CF1AC: FFE00890  fmr f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].f64 = ctx.f[1].f64;
	// 830CF1B0: 38800008  li r4, 8
	ctx.r[4].s64 = 8;
	// 830CF1B4: FFC01090  fmr f30, f2
	ctx.f[30].f64 = ctx.f[2].f64;
	// 830CF1B8: 7C691B78  mr r9, r3
	ctx.r[9].u64 = ctx.r[3].u64;
	// 830CF1BC: FFA01890  fmr f29, f3
	ctx.f[29].f64 = ctx.f[3].f64;
	// 830CF1C0: FF802090  fmr f28, f4
	ctx.f[28].f64 = ctx.f[4].f64;
	// 830CF1C4: 48032E2D  bl 0x83101ff0
	ctx.lr = 0x830CF1C8;
	sub_83101FF0(ctx, base);
	// 830CF1C8: 38A00010  li r5, 0x10
	ctx.r[5].s64 = 16;
	// 830CF1CC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 830CF1D0: 480348C9  bl 0x83103a98
	ctx.lr = 0x830CF1D4;
	sub_83103A98(ctx, base);
	// 830CF1D4: D3FF0000  stfs f31, 0(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 830CF1D8: D3DF0004  stfs f30, 4(r31)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 830CF1DC: D3BF0008  stfs f29, 8(r31)
	tmp.f32 = (ctx.f[29].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 830CF1E0: D39F000C  stfs f28, 0xc(r31)
	tmp.f32 = (ctx.f[28].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 830CF1E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 830CF1E8: 3981FFF0  addi r12, r1, -0x10
	ctx.r[12].s64 = ctx.r[1].s64 + -16;
	// 830CF1EC: 480D98D9  bl 0x831a8ac4
	ctx.lr = 0x830CF1F0;
	sub_831A8A8C(ctx, base);
	// 830CF1F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830CF1F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830CF1F8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830CF1FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CF200(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x830CF200 size=112
    let mut pc: u32 = 0x830CF200;
    'dispatch: loop {
        match pc {
            0x830CF200 => {
    //   block [0x830CF200..0x830CF270)
	// 830CF200: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830CF204: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830CF208: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830CF20C: 3981FFF0  addi r12, r1, -0x10
	ctx.r[12].s64 = ctx.r[1].s64 + -16;
	// 830CF210: 480D9869  bl 0x831a8a78
	ctx.lr = 0x830CF214;
	sub_831A8A40(ctx, base);
	// 830CF214: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830CF218: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 830CF21C: FFE00890  fmr f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].f64 = ctx.f[1].f64;
	// 830CF220: 38800010  li r4, 0x10
	ctx.r[4].s64 = 16;
	// 830CF224: FFC01090  fmr f30, f2
	ctx.f[30].f64 = ctx.f[2].f64;
	// 830CF228: 7C691B78  mr r9, r3
	ctx.r[9].u64 = ctx.r[3].u64;
	// 830CF22C: FFA01890  fmr f29, f3
	ctx.f[29].f64 = ctx.f[3].f64;
	// 830CF230: FF802090  fmr f28, f4
	ctx.f[28].f64 = ctx.f[4].f64;
	// 830CF234: 48032DBD  bl 0x83101ff0
	ctx.lr = 0x830CF238;
	sub_83101FF0(ctx, base);
	// 830CF238: 38A00010  li r5, 0x10
	ctx.r[5].s64 = 16;
	// 830CF23C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 830CF240: 48034859  bl 0x83103a98
	ctx.lr = 0x830CF244;
	sub_83103A98(ctx, base);
	// 830CF244: D3FF0000  stfs f31, 0(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 830CF248: D3DF0004  stfs f30, 4(r31)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 830CF24C: D3BF0008  stfs f29, 8(r31)
	tmp.f32 = (ctx.f[29].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 830CF250: D39F000C  stfs f28, 0xc(r31)
	tmp.f32 = (ctx.f[28].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 830CF254: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 830CF258: 3981FFF0  addi r12, r1, -0x10
	ctx.r[12].s64 = ctx.r[1].s64 + -16;
	// 830CF25C: 480D9869  bl 0x831a8ac4
	ctx.lr = 0x830CF260;
	sub_831A8A8C(ctx, base);
	// 830CF260: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830CF264: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830CF268: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830CF26C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CF270(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830CF270 size=80
    let mut pc: u32 = 0x830CF270;
    'dispatch: loop {
        match pc {
            0x830CF270 => {
    //   block [0x830CF270..0x830CF2C0)
	// 830CF270: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830CF274: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830CF278: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830CF27C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830CF280: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830CF284: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 830CF288: 38800012  li r4, 0x12
	ctx.r[4].s64 = 18;
	// 830CF28C: 7C691B78  mr r9, r3
	ctx.r[9].u64 = ctx.r[3].u64;
	// 830CF290: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 830CF294: 48032D5D  bl 0x83101ff0
	ctx.lr = 0x830CF298;
	sub_83101FF0(ctx, base);
	// 830CF298: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 830CF29C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 830CF2A0: 480347F9  bl 0x83103a98
	ctx.lr = 0x830CF2A4;
	sub_83103A98(ctx, base);
	// 830CF2A4: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 830CF2A8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830CF2AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830CF2B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830CF2B4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830CF2B8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830CF2BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CF2C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830CF2C0 size=72
    let mut pc: u32 = 0x830CF2C0;
    'dispatch: loop {
        match pc {
            0x830CF2C0 => {
    //   block [0x830CF2C0..0x830CF308)
	// 830CF2C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830CF2C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830CF2C8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830CF2CC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830CF2D0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 830CF2D4: 38800019  li r4, 0x19
	ctx.r[4].s64 = 25;
	// 830CF2D8: 7C691B78  mr r9, r3
	ctx.r[9].u64 = ctx.r[3].u64;
	// 830CF2DC: 48032D15  bl 0x83101ff0
	ctx.lr = 0x830CF2E0;
	sub_83101FF0(ctx, base);
	// 830CF2E0: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 830CF2E4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 830CF2E8: 480347B1  bl 0x83103a98
	ctx.lr = 0x830CF2EC;
	sub_83103A98(ctx, base);
	// 830CF2EC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830CF2F0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830CF2F4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830CF2F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830CF2FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830CF300: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830CF304: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CF308(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x830CF308 size=80
    let mut pc: u32 = 0x830CF308;
    'dispatch: loop {
        match pc {
            0x830CF308 => {
    //   block [0x830CF308..0x830CF358)
	// 830CF308: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830CF30C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830CF310: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830CF314: DBE1FFE8  stfd f31, -0x18(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.f[31].u64 ) };
	// 830CF318: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830CF31C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 830CF320: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 830CF324: 388007FA  li r4, 0x7fa
	ctx.r[4].s64 = 2042;
	// 830CF328: 7C691B78  mr r9, r3
	ctx.r[9].u64 = ctx.r[3].u64;
	// 830CF32C: 48032CC5  bl 0x83101ff0
	ctx.lr = 0x830CF330;
	sub_83101FF0(ctx, base);
	// 830CF330: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 830CF334: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 830CF338: 48034761  bl 0x83103a98
	ctx.lr = 0x830CF33C;
	sub_83103A98(ctx, base);
	// 830CF33C: D3FF0000  stfs f31, 0(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 830CF340: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830CF344: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830CF348: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830CF34C: CBE1FFE8  lfd f31, -0x18(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830CF350: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830CF354: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CF358(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830CF358 size=180
    let mut pc: u32 = 0x830CF358;
    'dispatch: loop {
        match pc {
            0x830CF358 => {
    //   block [0x830CF358..0x830CF40C)
	// 830CF358: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830CF35C: 480D8E11  bl 0x831a816c
	ctx.lr = 0x830CF360;
	sub_831A8130(ctx, base);
	// 830CF360: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830CF364: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 830CF368: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 830CF36C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830CF370: 7C7EE9D6  mullw r3, r30, r29
	ctx.r[3].s64 = (ctx.r[30].s32 as i64) * (ctx.r[29].s32 as i64);
	// 830CF374: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 830CF378: 41980084  blt cr6, 0x830cf3fc
	if ctx.cr[6].lt {
	pc = 0x830CF3FC; continue 'dispatch;
	}
	// 830CF37C: 2B1D0004  cmplwi cr6, r29, 4
	ctx.cr[6].compare_u32(ctx.r[29].u32, 4 as u32, &mut ctx.xer);
	// 830CF380: 4198007C  blt cr6, 0x830cf3fc
	if ctx.cr[6].lt {
	pc = 0x830CF3FC; continue 'dispatch;
	}
	// 830CF384: 3D607FFF  lis r11, 0x7fff
	ctx.r[11].s64 = 2147418112;
	// 830CF388: 0CDD0000  twi 6, r29, 0
	// 830CF38C: 616BFFFF  ori r11, r11, 0xffff
	ctx.r[11].u64 = ctx.r[11].u64 | 65535;
	// 830CF390: 7D6BEBD6  divw r11, r11, r29
	ctx.r[11].s32 = ctx.r[11].s32 / ctx.r[29].s32;
	// 830CF394: 7F035840  cmplw cr6, r3, r11
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[11].u32, &mut ctx.xer);
	// 830CF398: 40980064  bge cr6, 0x830cf3fc
	if !ctx.cr[6].lt {
	pc = 0x830CF3FC; continue 'dispatch;
	}
	// 830CF39C: 4800E115  bl 0x830dd4b0
	ctx.lr = 0x830CF3A0;
	sub_830DD4B0(ctx, base);
	// 830CF3A0: 907F0000  stw r3, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 830CF3A4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830CF3A8: 41820054  beq 0x830cf3fc
	if ctx.cr[0].eq {
	pc = 0x830CF3FC; continue 'dispatch;
	}
	// 830CF3AC: 357EFFFF  addic. r11, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830CF3B0: 907F000C  stw r3, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[3].u32 ) };
	// 830CF3B4: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 830CF3B8: 93BF0008  stw r29, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 830CF3BC: 4081001C  ble 0x830cf3d8
	if !ctx.cr[0].gt {
	pc = 0x830CF3D8; continue 'dispatch;
	}
	// 830CF3C0: 7D5D1A14  add r10, r29, r3
	ctx.r[10].u64 = ctx.r[29].u64 + ctx.r[3].u64;
	// 830CF3C4: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830CF3C8: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 830CF3CC: 83BF0008  lwz r29, 8(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 830CF3D0: 7C7D1A14  add r3, r29, r3
	ctx.r[3].u64 = ctx.r[29].u64 + ctx.r[3].u64;
	// 830CF3D4: 4082FFEC  bne 0x830cf3c0
	if !ctx.cr[0].eq {
	pc = 0x830CF3C0; continue 'dispatch;
	}
	// 830CF3D8: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830CF3DC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 830CF3E0: 813F0008  lwz r9, 8(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 830CF3E4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830CF3E8: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 830CF3EC: 811F0000  lwz r8, 0(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830CF3F0: 7D6B49D6  mullw r11, r11, r9
	ctx.r[11].s64 = (ctx.r[11].s32 as i64) * (ctx.r[9].s32 as i64);
	// 830CF3F4: 7D4B412E  stwx r10, r11, r8
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[8].u32), ctx.r[10].u32) };
	// 830CF3F8: 4800000C  b 0x830cf404
	pc = 0x830CF404; continue 'dispatch;
	// 830CF3FC: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 830CF400: 6063000E  ori r3, r3, 0xe
	ctx.r[3].u64 = ctx.r[3].u64 | 14;
	// 830CF404: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830CF408: 480D8DB4  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CF410(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830CF410 size=68
    let mut pc: u32 = 0x830CF410;
    'dispatch: loop {
        match pc {
            0x830CF410 => {
    //   block [0x830CF410..0x830CF454)
	// 830CF410: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830CF414: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830CF418: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830CF41C: 81650000  lwz r11, 0(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 830CF420: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 830CF424: 419A0010  beq cr6, 0x830cf434
	if ctx.cr[6].eq {
	pc = 0x830CF434; continue 'dispatch;
	}
	// 830CF428: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 830CF42C: 60630057  ori r3, r3, 0x57
	ctx.r[3].u64 = ctx.r[3].u64 | 87;
	// 830CF430: 48000014  b 0x830cf444
	pc = 0x830CF444; continue 'dispatch;
	// 830CF434: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 830CF438: 80850004  lwz r4, 4(r5)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) } as u64;
	// 830CF43C: 4BFFF7D5  bl 0x830cec10
	ctx.lr = 0x830CF440;
	sub_830CEC10(ctx, base);
	// 830CF440: 48010B11  bl 0x830dff50
	ctx.lr = 0x830CF444;
	sub_830DFF50(ctx, base);
	// 830CF444: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830CF448: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830CF44C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830CF450: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CF458(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830CF458 size=68
    let mut pc: u32 = 0x830CF458;
    'dispatch: loop {
        match pc {
            0x830CF458 => {
    //   block [0x830CF458..0x830CF49C)
	// 830CF458: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830CF45C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830CF460: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830CF464: 81650000  lwz r11, 0(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 830CF468: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 830CF46C: 419A0010  beq cr6, 0x830cf47c
	if ctx.cr[6].eq {
	pc = 0x830CF47C; continue 'dispatch;
	}
	// 830CF470: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 830CF474: 60630057  ori r3, r3, 0x57
	ctx.r[3].u64 = ctx.r[3].u64 | 87;
	// 830CF478: 48000014  b 0x830cf48c
	pc = 0x830CF48C; continue 'dispatch;
	// 830CF47C: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 830CF480: 80850004  lwz r4, 4(r5)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) } as u64;
	// 830CF484: 4BFFF78D  bl 0x830cec10
	ctx.lr = 0x830CF488;
	sub_830CEC10(ctx, base);
	// 830CF488: 48010B19  bl 0x830dffa0
	ctx.lr = 0x830CF48C;
	sub_830DFFA0(ctx, base);
	// 830CF48C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830CF490: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830CF494: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830CF498: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CF4A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830CF4A0 size=80
    let mut pc: u32 = 0x830CF4A0;
    'dispatch: loop {
        match pc {
            0x830CF4A0 => {
    //   block [0x830CF4A0..0x830CF4F0)
	// 830CF4A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830CF4A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830CF4A8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830CF4AC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830CF4B0: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 830CF4B4: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 830CF4B8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 830CF4BC: 4BFFF755  bl 0x830cec10
	ctx.lr = 0x830CF4C0;
	sub_830CEC10(ctx, base);
	// 830CF4C0: 48010B39  bl 0x830dfff8
	ctx.lr = 0x830CF4C4;
	sub_830DFFF8(ctx, base);
	// 830CF4C4: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 830CF4C8: 41800014  blt 0x830cf4dc
	if ctx.cr[0].lt {
	pc = 0x830CF4DC; continue 'dispatch;
	}
	// 830CF4CC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830CF4D0: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 830CF4D4: 4803312D  bl 0x83102600
	ctx.lr = 0x830CF4D8;
	sub_83102600(ctx, base);
	// 830CF4D8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830CF4DC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830CF4E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830CF4E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830CF4E8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830CF4EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CF4F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x830CF4F0 size=144
    let mut pc: u32 = 0x830CF4F0;
    'dispatch: loop {
        match pc {
            0x830CF4F0 => {
    //   block [0x830CF4F0..0x830CF580)
	// 830CF4F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830CF4F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830CF4F8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830CF4FC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830CF500: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830CF504: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830CF508: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 830CF50C: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 830CF510: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830CF514: 419A0048  beq cr6, 0x830cf55c
	if ctx.cr[6].eq {
	pc = 0x830CF55C; continue 'dispatch;
	}
	// 830CF518: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 830CF51C: 80BF0014  lwz r5, 0x14(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 830CF520: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 830CF524: C03F0008  lfs f1, 8(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 830CF528: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830CF52C: 4803B20D  bl 0x8310a738
	ctx.lr = 0x830CF530;
	sub_8310A738(ctx, base);
	// 830CF530: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 830CF534: 41800020  blt 0x830cf554
	if ctx.cr[0].lt {
	pc = 0x830CF554; continue 'dispatch;
	}
	// 830CF538: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 830CF53C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 830CF540: 419A0008  beq cr6, 0x830cf548
	if ctx.cr[6].eq {
	pc = 0x830CF548; continue 'dispatch;
	}
	// 830CF544: 4803AB8D  bl 0x8310a0d0
	ctx.lr = 0x830CF548;
	sub_8310A0D0(ctx, base);
	// 830CF548: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 830CF54C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830CF550: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 830CF554: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 830CF558: 41980010  blt cr6, 0x830cf568
	if ctx.cr[6].lt {
	pc = 0x830CF568; continue 'dispatch;
	}
	// 830CF55C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 830CF560: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 830CF564: 4BFFF725  bl 0x830cec88
	ctx.lr = 0x830CF568;
	sub_830CEC88(ctx, base);
	// 830CF568: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830CF56C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830CF570: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830CF574: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830CF578: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830CF57C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CF580(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830CF580 size=96
    let mut pc: u32 = 0x830CF580;
    'dispatch: loop {
        match pc {
            0x830CF580 => {
    //   block [0x830CF580..0x830CF5E0)
	// 830CF580: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830CF584: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830CF588: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830CF58C: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 830CF590: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 830CF594: 4BFFEEDD  bl 0x830ce470
	ctx.lr = 0x830CF598;
	sub_830CE470(ctx, base);
	// 830CF598: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 830CF59C: 41800034  blt 0x830cf5d0
	if ctx.cr[0].lt {
	pc = 0x830CF5D0; continue 'dispatch;
	}
	// 830CF5A0: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 830CF5A4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830CF5A8: 419A0020  beq cr6, 0x830cf5c8
	if ctx.cr[6].eq {
	pc = 0x830CF5C8; continue 'dispatch;
	}
	// 830CF5AC: 81450000  lwz r10, 0(r5)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 830CF5B0: 2F0A0003  cmpwi cr6, r10, 3
	ctx.cr[6].compare_i32(ctx.r[10].s32, 3, &mut ctx.xer);
	// 830CF5B4: 409A0014  bne cr6, 0x830cf5c8
	if !ctx.cr[6].eq {
	pc = 0x830CF5C8; continue 'dispatch;
	}
	// 830CF5B8: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 830CF5BC: 80850004  lwz r4, 4(r5)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) } as u64;
	// 830CF5C0: 4BFFE171  bl 0x830cd730
	ctx.lr = 0x830CF5C4;
	sub_830CD730(ctx, base);
	// 830CF5C4: 4800000C  b 0x830cf5d0
	pc = 0x830CF5D0; continue 'dispatch;
	// 830CF5C8: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 830CF5CC: 60630057  ori r3, r3, 0x57
	ctx.r[3].u64 = ctx.r[3].u64 | 87;
	// 830CF5D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830CF5D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830CF5D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830CF5DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CF5E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830CF5E0 size=44
    let mut pc: u32 = 0x830CF5E0;
    'dispatch: loop {
        match pc {
            0x830CF5E0 => {
    //   block [0x830CF5E0..0x830CF60C)
	// 830CF5E0: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 830CF5E4: 419A0050  beq cr6, 0x830cf634
	if ctx.cr[6].eq {
		sub_830CF634(ctx, base);
		return;
	}
	// 830CF5E8: 81650000  lwz r11, 0(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 830CF5EC: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 830CF5F0: 409A0044  bne cr6, 0x830cf634
	if !ctx.cr[6].eq {
		sub_830CF634(ctx, base);
		return;
	}
	// 830CF5F4: 81650004  lwz r11, 4(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) } as u64;
	// 830CF5F8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830CF5FC: 8164000C  lwz r11, 0xc(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(12 as u32) ) } as u64;
	// 830CF600: 419A000C  beq cr6, 0x830cf60c
	if ctx.cr[6].eq {
		sub_830CF60C(ctx, base);
		return;
	}
	// 830CF604: 616B0001  ori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 | 1;
	// 830CF608: 48000008  b 0x830cf610
	sub_830CF60C(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CF60C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830CF60C size=40
    let mut pc: u32 = 0x830CF60C;
    'dispatch: loop {
        match pc {
            0x830CF60C => {
    //   block [0x830CF60C..0x830CF634)
	// 830CF60C: 556B003C  rlwinm r11, r11, 0, 0, 0x1e
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 830CF610: 9164000C  stw r11, 0xc(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 830CF614: 81640010  lwz r11, 0x10(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(16 as u32) ) } as u64;
	// 830CF618: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830CF61C: 419A0010  beq cr6, 0x830cf62c
	if ctx.cr[6].eq {
	pc = 0x830CF62C; continue 'dispatch;
	}
	// 830CF620: 814B0084  lwz r10, 0x84(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(132 as u32) ) } as u64;
	// 830CF624: 614A0001  ori r10, r10, 1
	ctx.r[10].u64 = ctx.r[10].u64 | 1;
	// 830CF628: 914B0084  stw r10, 0x84(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(132 as u32), ctx.r[10].u32 ) };
	// 830CF62C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830CF630: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CF634(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830CF634 size=12
    let mut pc: u32 = 0x830CF634;
    'dispatch: loop {
        match pc {
            0x830CF634 => {
    //   block [0x830CF634..0x830CF640)
	// 830CF634: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 830CF638: 60630057  ori r3, r3, 0x57
	ctx.r[3].u64 = ctx.r[3].u64 | 87;
	// 830CF63C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CF640(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830CF640 size=20
    let mut pc: u32 = 0x830CF640;
    'dispatch: loop {
        match pc {
            0x830CF640 => {
    //   block [0x830CF640..0x830CF654)
	// 830CF640: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 830CF644: 409A0010  bne cr6, 0x830cf654
	if !ctx.cr[6].eq {
		sub_830CF654(ctx, base);
		return;
	}
	// 830CF648: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 830CF64C: 60630057  ori r3, r3, 0x57
	ctx.r[3].u64 = ctx.r[3].u64 | 87;
	// 830CF650: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CF654(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830CF654 size=24
    let mut pc: u32 = 0x830CF654;
    'dispatch: loop {
        match pc {
            0x830CF654 => {
    //   block [0x830CF654..0x830CF66C)
	// 830CF654: 81640004  lwz r11, 4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 830CF658: 7F035840  cmplw cr6, r3, r11
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[11].u32, &mut ctx.xer);
	// 830CF65C: 41980010  blt cr6, 0x830cf66c
	if ctx.cr[6].lt {
		sub_830CF66C(ctx, base);
		return;
	}
	// 830CF660: 3C608030  lis r3, -0x7fd0
	ctx.r[3].s64 = -2144337920;
	// 830CF664: 6063000C  ori r3, r3, 0xc
	ctx.r[3].u64 = ctx.r[3].u64 | 12;
	// 830CF668: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CF66C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830CF66C size=68
    let mut pc: u32 = 0x830CF66C;
    'dispatch: loop {
        match pc {
            0x830CF66C => {
    //   block [0x830CF66C..0x830CF6B0)
	// 830CF66C: 81440008  lwz r10, 8(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 830CF670: 546B1838  slwi r11, r3, 3
	ctx.r[11].u32 = ctx.r[3].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 830CF674: 81250004  lwz r9, 4(r5)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) } as u64;
	// 830CF678: 7D4B502E  lwzx r10, r11, r10
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 830CF67C: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 830CF680: 419A0028  beq cr6, 0x830cf6a8
	if ctx.cr[6].eq {
	pc = 0x830CF6A8; continue 'dispatch;
	}
	// 830CF684: 81440010  lwz r10, 0x10(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(16 as u32) ) } as u64;
	// 830CF688: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 830CF68C: 419A0010  beq cr6, 0x830cf69c
	if ctx.cr[6].eq {
	pc = 0x830CF69C; continue 'dispatch;
	}
	// 830CF690: 812A0084  lwz r9, 0x84(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(132 as u32) ) } as u64;
	// 830CF694: 61290001  ori r9, r9, 1
	ctx.r[9].u64 = ctx.r[9].u64 | 1;
	// 830CF698: 912A0084  stw r9, 0x84(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(132 as u32), ctx.r[9].u32 ) };
	// 830CF69C: 81440008  lwz r10, 8(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 830CF6A0: 81250004  lwz r9, 4(r5)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) } as u64;
	// 830CF6A4: 7D2B512E  stwx r9, r11, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[9].u32) };
	// 830CF6A8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830CF6AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CF6B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830CF6B0 size=20
    let mut pc: u32 = 0x830CF6B0;
    'dispatch: loop {
        match pc {
            0x830CF6B0 => {
    //   block [0x830CF6B0..0x830CF6C4)
	// 830CF6B0: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 830CF6B4: 409A0010  bne cr6, 0x830cf6c4
	if !ctx.cr[6].eq {
		sub_830CF6C4(ctx, base);
		return;
	}
	// 830CF6B8: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 830CF6BC: 60630057  ori r3, r3, 0x57
	ctx.r[3].u64 = ctx.r[3].u64 | 87;
	// 830CF6C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CF6C4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830CF6C4 size=24
    let mut pc: u32 = 0x830CF6C4;
    'dispatch: loop {
        match pc {
            0x830CF6C4 => {
    //   block [0x830CF6C4..0x830CF6DC)
	// 830CF6C4: 81640004  lwz r11, 4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 830CF6C8: 7F035840  cmplw cr6, r3, r11
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[11].u32, &mut ctx.xer);
	// 830CF6CC: 41980010  blt cr6, 0x830cf6dc
	if ctx.cr[6].lt {
		sub_830CF6DC(ctx, base);
		return;
	}
	// 830CF6D0: 3C608030  lis r3, -0x7fd0
	ctx.r[3].s64 = -2144337920;
	// 830CF6D4: 6063000C  ori r3, r3, 0xc
	ctx.r[3].u64 = ctx.r[3].u64 | 12;
	// 830CF6D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CF6DC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x830CF6DC size=76
    let mut pc: u32 = 0x830CF6DC;
    'dispatch: loop {
        match pc {
            0x830CF6DC => {
    //   block [0x830CF6DC..0x830CF728)
	// 830CF6DC: 81440008  lwz r10, 8(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 830CF6E0: 546B1838  slwi r11, r3, 3
	ctx.r[11].u32 = ctx.r[3].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 830CF6E4: C0050004  lfs f0, 4(r5)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 830CF6E8: 7D4A5A14  add r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 830CF6EC: C1AA0004  lfs f13, 4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 830CF6F0: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 830CF6F4: 419A002C  beq cr6, 0x830cf720
	if ctx.cr[6].eq {
	pc = 0x830CF720; continue 'dispatch;
	}
	// 830CF6F8: 81440010  lwz r10, 0x10(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(16 as u32) ) } as u64;
	// 830CF6FC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 830CF700: 419A0010  beq cr6, 0x830cf710
	if ctx.cr[6].eq {
	pc = 0x830CF710; continue 'dispatch;
	}
	// 830CF704: 812A0084  lwz r9, 0x84(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(132 as u32) ) } as u64;
	// 830CF708: 61290001  ori r9, r9, 1
	ctx.r[9].u64 = ctx.r[9].u64 | 1;
	// 830CF70C: 912A0084  stw r9, 0x84(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(132 as u32), ctx.r[9].u32 ) };
	// 830CF710: 81440008  lwz r10, 8(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 830CF714: C0050004  lfs f0, 4(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 830CF718: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 830CF71C: D00B0004  stfs f0, 4(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 830CF720: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830CF724: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CF728(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830CF728 size=100
    let mut pc: u32 = 0x830CF728;
    'dispatch: loop {
        match pc {
            0x830CF728 => {
    //   block [0x830CF728..0x830CF78C)
	// 830CF728: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830CF72C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830CF730: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830CF734: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 830CF738: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 830CF73C: 409A0010  bne cr6, 0x830cf74c
	if !ctx.cr[6].eq {
	pc = 0x830CF74C; continue 'dispatch;
	}
	// 830CF740: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 830CF744: 60630057  ori r3, r3, 0x57
	ctx.r[3].u64 = ctx.r[3].u64 | 87;
	// 830CF748: 48000034  b 0x830cf77c
	pc = 0x830CF77C; continue 'dispatch;
	// 830CF74C: 81440004  lwz r10, 4(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 830CF750: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 830CF754: 41980010  blt cr6, 0x830cf764
	if ctx.cr[6].lt {
	pc = 0x830CF764; continue 'dispatch;
	}
	// 830CF758: 3C608030  lis r3, -0x7fd0
	ctx.r[3].s64 = -2144337920;
	// 830CF75C: 6063000C  ori r3, r3, 0xc
	ctx.r[3].u64 = ctx.r[3].u64 | 12;
	// 830CF760: 4800001C  b 0x830cf77c
	pc = 0x830CF77C; continue 'dispatch;
	// 830CF764: 81440008  lwz r10, 8(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 830CF768: 556B1838  slwi r11, r11, 3
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 830CF76C: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 830CF770: 7C8B502E  lwzx r4, r11, r10
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 830CF774: 48032F1D  bl 0x83102690
	ctx.lr = 0x830CF778;
	sub_83102690(ctx, base);
	// 830CF778: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830CF77C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830CF780: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830CF784: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830CF788: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CF790(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x830CF790 size=104
    let mut pc: u32 = 0x830CF790;
    'dispatch: loop {
        match pc {
            0x830CF790 => {
    //   block [0x830CF790..0x830CF7F8)
	// 830CF790: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830CF794: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830CF798: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830CF79C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 830CF7A0: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 830CF7A4: 409A0010  bne cr6, 0x830cf7b4
	if !ctx.cr[6].eq {
	pc = 0x830CF7B4; continue 'dispatch;
	}
	// 830CF7A8: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 830CF7AC: 60630057  ori r3, r3, 0x57
	ctx.r[3].u64 = ctx.r[3].u64 | 87;
	// 830CF7B0: 48000038  b 0x830cf7e8
	pc = 0x830CF7E8; continue 'dispatch;
	// 830CF7B4: 81440004  lwz r10, 4(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 830CF7B8: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 830CF7BC: 41980010  blt cr6, 0x830cf7cc
	if ctx.cr[6].lt {
	pc = 0x830CF7CC; continue 'dispatch;
	}
	// 830CF7C0: 3C608030  lis r3, -0x7fd0
	ctx.r[3].s64 = -2144337920;
	// 830CF7C4: 6063000C  ori r3, r3, 0xc
	ctx.r[3].u64 = ctx.r[3].u64 | 12;
	// 830CF7C8: 48000020  b 0x830cf7e8
	pc = 0x830CF7E8; continue 'dispatch;
	// 830CF7CC: 81440008  lwz r10, 8(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 830CF7D0: 556B1838  slwi r11, r11, 3
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 830CF7D4: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 830CF7D8: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 830CF7DC: C02B0004  lfs f1, 4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 830CF7E0: 48032E69  bl 0x83102648
	ctx.lr = 0x830CF7E4;
	sub_83102648(ctx, base);
	// 830CF7E4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830CF7E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830CF7EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830CF7F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830CF7F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CF7F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x830CF7F8 size=260
    let mut pc: u32 = 0x830CF7F8;
    'dispatch: loop {
        match pc {
            0x830CF7F8 => {
    //   block [0x830CF7F8..0x830CF8FC)
	// 830CF7F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830CF7FC: 480D8971  bl 0x831a816c
	ctx.lr = 0x830CF800;
	sub_831A8130(ctx, base);
	// 830CF800: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830CF804: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830CF808: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 830CF80C: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 830CF810: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830CF814: 419A0010  beq cr6, 0x830cf824
	if ctx.cr[6].eq {
	pc = 0x830CF824; continue 'dispatch;
	}
	// 830CF818: 814B0084  lwz r10, 0x84(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(132 as u32) ) } as u64;
	// 830CF81C: 614A0001  ori r10, r10, 1
	ctx.r[10].u64 = ctx.r[10].u64 | 1;
	// 830CF820: 914B0084  stw r10, 0x84(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(132 as u32), ctx.r[10].u32 ) };
	// 830CF824: 3D601FFF  lis r11, 0x1fff
	ctx.r[11].s64 = 536805376;
	// 830CF828: 57A31838  slwi r3, r29, 3
	ctx.r[3].u32 = ctx.r[29].u32.wrapping_shl(3);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 830CF82C: 616BFFFF  ori r11, r11, 0xffff
	ctx.r[11].u64 = ctx.r[11].u64 | 65535;
	// 830CF830: 7F1D5840  cmplw cr6, r29, r11
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[11].u32, &mut ctx.xer);
	// 830CF834: 40990008  ble cr6, 0x830cf83c
	if !ctx.cr[6].gt {
	pc = 0x830CF83C; continue 'dispatch;
	}
	// 830CF838: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 830CF83C: 4800DC75  bl 0x830dd4b0
	ctx.lr = 0x830CF840;
	sub_830DD4B0(ctx, base);
	// 830CF840: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830CF844: 41820038  beq 0x830cf87c
	if ctx.cr[0].eq {
	pc = 0x830CF87C; continue 'dispatch;
	}
	// 830CF848: 357DFFFF  addic. r11, r29, -1
	ctx.xer.ca = (ctx.r[29].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[29].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830CF84C: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 830CF850: 41800024  blt 0x830cf874
	if ctx.cr[0].lt {
	pc = 0x830CF874; continue 'dispatch;
	}
	// 830CF854: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 830CF858: C00908A4  lfs f0, 0x8a4(r9)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 830CF85C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 830CF860: D00A0004  stfs f0, 4(r10)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 830CF864: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830CF868: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 830CF86C: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 830CF870: 4080FFEC  bge 0x830cf85c
	if !ctx.cr[0].lt {
	pc = 0x830CF85C; continue 'dispatch;
	}
	// 830CF874: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830CF878: 48000008  b 0x830cf880
	pc = 0x830CF880; continue 'dispatch;
	// 830CF87C: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 830CF880: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 830CF884: 409A0010  bne cr6, 0x830cf894
	if !ctx.cr[6].eq {
	pc = 0x830CF894; continue 'dispatch;
	}
	// 830CF888: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 830CF88C: 6063000E  ori r3, r3, 0xe
	ctx.r[3].u64 = ctx.r[3].u64 | 14;
	// 830CF890: 48000064  b 0x830cf8f4
	pc = 0x830CF8F4; continue 'dispatch;
	// 830CF894: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830CF898: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 830CF89C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830CF8A0: 40990040  ble cr6, 0x830cf8e0
	if !ctx.cr[6].gt {
	pc = 0x830CF8E0; continue 'dispatch;
	}
	// 830CF8A4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830CF8A8: 7F0AE800  cmpw cr6, r10, r29
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[29].s32, &mut ctx.xer);
	// 830CF8AC: 40980034  bge cr6, 0x830cf8e0
	if !ctx.cr[6].lt {
	pc = 0x830CF8E0; continue 'dispatch;
	}
	// 830CF8B0: 813F0008  lwz r9, 8(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 830CF8B4: 7D0BF214  add r8, r11, r30
	ctx.r[8].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 830CF8B8: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 830CF8BC: 7D295A14  add r9, r9, r11
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 830CF8C0: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 830CF8C4: 80E90000  lwz r7, 0(r9)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 830CF8C8: 90E80000  stw r7, 0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 830CF8CC: 81290004  lwz r9, 4(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 830CF8D0: 91280004  stw r9, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 830CF8D4: 813F0004  lwz r9, 4(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830CF8D8: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 830CF8DC: 4198FFCC  blt cr6, 0x830cf8a8
	if ctx.cr[6].lt {
	pc = 0x830CF8A8; continue 'dispatch;
	}
	// 830CF8E0: 93BF0004  stw r29, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 830CF8E4: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 830CF8E8: 4800DBF1  bl 0x830dd4d8
	ctx.lr = 0x830CF8EC;
	sub_830DD4D8(ctx, base);
	// 830CF8EC: 93DF0008  stw r30, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 830CF8F0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830CF8F4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830CF8F8: 480D88C4  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CF900(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830CF900 size=48
    let mut pc: u32 = 0x830CF900;
    'dispatch: loop {
        match pc {
            0x830CF900 => {
    //   block [0x830CF900..0x830CF930)
	// 830CF900: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 830CF904: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830CF908: 419A0028  beq cr6, 0x830cf930
	if ctx.cr[6].eq {
		sub_830CF930(ctx, base);
		return;
	}
	// 830CF90C: 81450000  lwz r10, 0(r5)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 830CF910: 2F0A0005  cmpwi cr6, r10, 5
	ctx.cr[6].compare_i32(ctx.r[10].s32, 5, &mut ctx.xer);
	// 830CF914: 409A001C  bne cr6, 0x830cf930
	if !ctx.cr[6].eq {
		sub_830CF930(ctx, base);
		return;
	}
	// 830CF918: 814B0084  lwz r10, 0x84(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(132 as u32) ) } as u64;
	// 830CF91C: 386B0010  addi r3, r11, 0x10
	ctx.r[3].s64 = ctx.r[11].s64 + 16;
	// 830CF920: 80850004  lwz r4, 4(r5)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) } as u64;
	// 830CF924: 614A0001  ori r10, r10, 1
	ctx.r[10].u64 = ctx.r[10].u64 | 1;
	// 830CF928: 914B0084  stw r10, 0x84(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(132 as u32), ctx.r[10].u32 ) };
	// 830CF92C: 4BFFF35C  b 0x830cec88
	sub_830CEC88(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CF930(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830CF930 size=12
    let mut pc: u32 = 0x830CF930;
    'dispatch: loop {
        match pc {
            0x830CF930 => {
    //   block [0x830CF930..0x830CF93C)
	// 830CF930: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 830CF934: 60630057  ori r3, r3, 0x57
	ctx.r[3].u64 = ctx.r[3].u64 | 87;
	// 830CF938: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CF940(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x830CF940 size=64
    let mut pc: u32 = 0x830CF940;
    'dispatch: loop {
        match pc {
            0x830CF940 => {
    //   block [0x830CF940..0x830CF980)
	// 830CF940: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 830CF944: 419A003C  beq cr6, 0x830cf980
	if ctx.cr[6].eq {
		sub_830CF980(ctx, base);
		return;
	}
	// 830CF948: 81650000  lwz r11, 0(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 830CF94C: 2F0B0007  cmpwi cr6, r11, 7
	ctx.cr[6].compare_i32(ctx.r[11].s32, 7, &mut ctx.xer);
	// 830CF950: 409A0030  bne cr6, 0x830cf980
	if !ctx.cr[6].eq {
		sub_830CF980(ctx, base);
		return;
	}
	// 830CF954: C0050004  lfs f0, 4(r5)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 830CF958: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830CF95C: C1A50008  lfs f13, 8(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(8 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 830CF960: C185000C  lfs f12, 0xc(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(12 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 830CF964: D0040028  stfs f0, 0x28(r4)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(40 as u32), tmp.u32 ) };
	// 830CF968: D1A4002C  stfs f13, 0x2c(r4)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(44 as u32), tmp.u32 ) };
	// 830CF96C: D1840030  stfs f12, 0x30(r4)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(48 as u32), tmp.u32 ) };
	// 830CF970: 81640084  lwz r11, 0x84(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(132 as u32) ) } as u64;
	// 830CF974: 616B0002  ori r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u64 | 2;
	// 830CF978: 91640084  stw r11, 0x84(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(132 as u32), ctx.r[11].u32 ) };
	// 830CF97C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CF980(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830CF980 size=12
    let mut pc: u32 = 0x830CF980;
    'dispatch: loop {
        match pc {
            0x830CF980 => {
    //   block [0x830CF980..0x830CF98C)
	// 830CF980: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 830CF984: 60630057  ori r3, r3, 0x57
	ctx.r[3].u64 = ctx.r[3].u64 | 87;
	// 830CF988: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CF990(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x830CF990 size=64
    let mut pc: u32 = 0x830CF990;
    'dispatch: loop {
        match pc {
            0x830CF990 => {
    //   block [0x830CF990..0x830CF9D0)
	// 830CF990: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 830CF994: 419A003C  beq cr6, 0x830cf9d0
	if ctx.cr[6].eq {
		sub_830CF9D0(ctx, base);
		return;
	}
	// 830CF998: 81650000  lwz r11, 0(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 830CF99C: 2F0B0007  cmpwi cr6, r11, 7
	ctx.cr[6].compare_i32(ctx.r[11].s32, 7, &mut ctx.xer);
	// 830CF9A0: 409A0030  bne cr6, 0x830cf9d0
	if !ctx.cr[6].eq {
		sub_830CF9D0(ctx, base);
		return;
	}
	// 830CF9A4: C0050004  lfs f0, 4(r5)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 830CF9A8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830CF9AC: C1A50008  lfs f13, 8(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(8 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 830CF9B0: C185000C  lfs f12, 0xc(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(12 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 830CF9B4: D0040034  stfs f0, 0x34(r4)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(52 as u32), tmp.u32 ) };
	// 830CF9B8: D1A40038  stfs f13, 0x38(r4)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(56 as u32), tmp.u32 ) };
	// 830CF9BC: D184003C  stfs f12, 0x3c(r4)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(60 as u32), tmp.u32 ) };
	// 830CF9C0: 81640084  lwz r11, 0x84(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(132 as u32) ) } as u64;
	// 830CF9C4: 616B0002  ori r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u64 | 2;
	// 830CF9C8: 91640084  stw r11, 0x84(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(132 as u32), ctx.r[11].u32 ) };
	// 830CF9CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CF9D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830CF9D0 size=12
    let mut pc: u32 = 0x830CF9D0;
    'dispatch: loop {
        match pc {
            0x830CF9D0 => {
    //   block [0x830CF9D0..0x830CF9DC)
	// 830CF9D0: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 830CF9D4: 60630057  ori r3, r3, 0x57
	ctx.r[3].u64 = ctx.r[3].u64 | 87;
	// 830CF9D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CF9E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830CF9E0 size=60
    let mut pc: u32 = 0x830CF9E0;
    'dispatch: loop {
        match pc {
            0x830CF9E0 => {
    //   block [0x830CF9E0..0x830CFA1C)
	// 830CF9E0: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 830CF9E4: 419A007C  beq cr6, 0x830cfa60
	if ctx.cr[6].eq {
		sub_830CFA60(ctx, base);
		return;
	}
	// 830CF9E8: 81650000  lwz r11, 0(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 830CF9EC: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 830CF9F0: 409A0070  bne cr6, 0x830cfa60
	if !ctx.cr[6].eq {
		sub_830CFA60(ctx, base);
		return;
	}
	// 830CF9F4: 81650004  lwz r11, 4(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) } as u64;
	// 830CF9F8: 81440088  lwz r10, 0x88(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(136 as u32) ) } as u64;
	// 830CF9FC: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 830CFA00: 419A0058  beq cr6, 0x830cfa58
	if ctx.cr[6].eq {
		sub_830CFA1C(ctx, base);
		return;
	}
	// 830CFA04: 91640088  stw r11, 0x88(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(136 as u32), ctx.r[11].u32 ) };
	// 830CFA08: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 830CFA0C: 409A0010  bne cr6, 0x830cfa1c
	if !ctx.cr[6].eq {
		sub_830CFA1C(ctx, base);
		return;
	}
	// 830CFA10: 81440020  lwz r10, 0x20(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(32 as u32) ) } as u64;
	// 830CFA14: 554A003C  rlwinm r10, r10, 0, 0, 0x1e
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 830CFA18: 48000014  b 0x830cfa2c
	sub_830CFA1C(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CFA1C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830CFA1C size=68
    let mut pc: u32 = 0x830CFA1C;
    'dispatch: loop {
        match pc {
            0x830CFA1C => {
    //   block [0x830CFA1C..0x830CFA60)
	// 830CFA1C: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 830CFA20: 409A002C  bne cr6, 0x830cfa4c
	if !ctx.cr[6].eq {
	pc = 0x830CFA4C; continue 'dispatch;
	}
	// 830CFA24: 81440020  lwz r10, 0x20(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(32 as u32) ) } as u64;
	// 830CFA28: 614A0001  ori r10, r10, 1
	ctx.r[10].u64 = ctx.r[10].u64 | 1;
	// 830CFA2C: 39640014  addi r11, r4, 0x14
	ctx.r[11].s64 = ctx.r[4].s64 + 20;
	// 830CFA30: 81640024  lwz r11, 0x24(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(36 as u32) ) } as u64;
	// 830CFA34: 91440020  stw r10, 0x20(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 830CFA38: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830CFA3C: 419A0010  beq cr6, 0x830cfa4c
	if ctx.cr[6].eq {
	pc = 0x830CFA4C; continue 'dispatch;
	}
	// 830CFA40: 814B0084  lwz r10, 0x84(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(132 as u32) ) } as u64;
	// 830CFA44: 614A0001  ori r10, r10, 1
	ctx.r[10].u64 = ctx.r[10].u64 | 1;
	// 830CFA48: 914B0084  stw r10, 0x84(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(132 as u32), ctx.r[10].u32 ) };
	// 830CFA4C: 81640084  lwz r11, 0x84(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(132 as u32) ) } as u64;
	// 830CFA50: 616B0003  ori r11, r11, 3
	ctx.r[11].u64 = ctx.r[11].u64 | 3;
	// 830CFA54: 91640084  stw r11, 0x84(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(132 as u32), ctx.r[11].u32 ) };
	// 830CFA58: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830CFA5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CFA60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830CFA60 size=12
    let mut pc: u32 = 0x830CFA60;
    'dispatch: loop {
        match pc {
            0x830CFA60 => {
    //   block [0x830CFA60..0x830CFA6C)
	// 830CFA60: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 830CFA64: 60630057  ori r3, r3, 0x57
	ctx.r[3].u64 = ctx.r[3].u64 | 87;
	// 830CFA68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CFA70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830CFA70 size=76
    let mut pc: u32 = 0x830CFA70;
    'dispatch: loop {
        match pc {
            0x830CFA70 => {
    //   block [0x830CFA70..0x830CFABC)
	// 830CFA70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830CFA74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830CFA78: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830CFA7C: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 830CFA80: 419A0024  beq cr6, 0x830cfaa4
	if ctx.cr[6].eq {
	pc = 0x830CFAA4; continue 'dispatch;
	}
	// 830CFA84: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 830CFA88: 419A001C  beq cr6, 0x830cfaa4
	if ctx.cr[6].eq {
	pc = 0x830CFAA4; continue 'dispatch;
	}
	// 830CFA8C: 81640084  lwz r11, 0x84(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(132 as u32) ) } as u64;
	// 830CFA90: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 830CFA94: 5564F7BE  rlwinm r4, r11, 0x1e, 0x1e, 0x1f
	ctx.r[4].u64 = ctx.r[11].u32 as u64 & 0x00000003u64;
	// 830CFA98: 48032B69  bl 0x83102600
	ctx.lr = 0x830CFA9C;
	sub_83102600(ctx, base);
	// 830CFA9C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830CFAA0: 4800000C  b 0x830cfaac
	pc = 0x830CFAAC; continue 'dispatch;
	// 830CFAA4: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 830CFAA8: 60630057  ori r3, r3, 0x57
	ctx.r[3].u64 = ctx.r[3].u64 | 87;
	// 830CFAAC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830CFAB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830CFAB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830CFAB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CFAC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830CFAC0 size=76
    let mut pc: u32 = 0x830CFAC0;
    'dispatch: loop {
        match pc {
            0x830CFAC0 => {
    //   block [0x830CFAC0..0x830CFB0C)
	// 830CFAC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830CFAC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830CFAC8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830CFACC: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 830CFAD0: 419A0024  beq cr6, 0x830cfaf4
	if ctx.cr[6].eq {
	pc = 0x830CFAF4; continue 'dispatch;
	}
	// 830CFAD4: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 830CFAD8: 419A001C  beq cr6, 0x830cfaf4
	if ctx.cr[6].eq {
	pc = 0x830CFAF4; continue 'dispatch;
	}
	// 830CFADC: 81640084  lwz r11, 0x84(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(132 as u32) ) } as u64;
	// 830CFAE0: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 830CFAE4: 5564E7BE  rlwinm r4, r11, 0x1c, 0x1e, 0x1f
	ctx.r[4].u64 = ctx.r[11].u32 as u64 & 0x0000000Fu64;
	// 830CFAE8: 48032B19  bl 0x83102600
	ctx.lr = 0x830CFAEC;
	sub_83102600(ctx, base);
	// 830CFAEC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830CFAF0: 4800000C  b 0x830cfafc
	pc = 0x830CFAFC; continue 'dispatch;
	// 830CFAF4: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 830CFAF8: 60630057  ori r3, r3, 0x57
	ctx.r[3].u64 = ctx.r[3].u64 | 87;
	// 830CFAFC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830CFB00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830CFB04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830CFB08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CFB10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830CFB10 size=76
    let mut pc: u32 = 0x830CFB10;
    'dispatch: loop {
        match pc {
            0x830CFB10 => {
    //   block [0x830CFB10..0x830CFB5C)
	// 830CFB10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830CFB14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830CFB18: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830CFB1C: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 830CFB20: 419A0024  beq cr6, 0x830cfb44
	if ctx.cr[6].eq {
	pc = 0x830CFB44; continue 'dispatch;
	}
	// 830CFB24: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 830CFB28: 419A001C  beq cr6, 0x830cfb44
	if ctx.cr[6].eq {
	pc = 0x830CFB44; continue 'dispatch;
	}
	// 830CFB2C: 81640084  lwz r11, 0x84(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(132 as u32) ) } as u64;
	// 830CFB30: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 830CFB34: 5564F6B6  rlwinm r4, r11, 0x1e, 0x1a, 0x1b
	ctx.r[4].u64 = ctx.r[11].u32 as u64 & 0x00000003u64;
	// 830CFB38: 48032AC9  bl 0x83102600
	ctx.lr = 0x830CFB3C;
	sub_83102600(ctx, base);
	// 830CFB3C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830CFB40: 4800000C  b 0x830cfb4c
	pc = 0x830CFB4C; continue 'dispatch;
	// 830CFB44: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 830CFB48: 60630057  ori r3, r3, 0x57
	ctx.r[3].u64 = ctx.r[3].u64 | 87;
	// 830CFB4C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830CFB50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830CFB54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830CFB58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CFB60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830CFB60 size=76
    let mut pc: u32 = 0x830CFB60;
    'dispatch: loop {
        match pc {
            0x830CFB60 => {
    //   block [0x830CFB60..0x830CFBAC)
	// 830CFB60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830CFB64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830CFB68: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830CFB6C: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 830CFB70: 419A0024  beq cr6, 0x830cfb94
	if ctx.cr[6].eq {
	pc = 0x830CFB94; continue 'dispatch;
	}
	// 830CFB74: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 830CFB78: 419A001C  beq cr6, 0x830cfb94
	if ctx.cr[6].eq {
	pc = 0x830CFB94; continue 'dispatch;
	}
	// 830CFB7C: 81640084  lwz r11, 0x84(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(132 as u32) ) } as u64;
	// 830CFB80: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 830CFB84: 5564C7BE  rlwinm r4, r11, 0x18, 0x1e, 0x1f
	ctx.r[4].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 830CFB88: 48032A79  bl 0x83102600
	ctx.lr = 0x830CFB8C;
	sub_83102600(ctx, base);
	// 830CFB8C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830CFB90: 4800000C  b 0x830cfb9c
	pc = 0x830CFB9C; continue 'dispatch;
	// 830CFB94: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 830CFB98: 60630057  ori r3, r3, 0x57
	ctx.r[3].u64 = ctx.r[3].u64 | 87;
	// 830CFB9C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830CFBA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830CFBA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830CFBA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CFBB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830CFBB0 size=168
    let mut pc: u32 = 0x830CFBB0;
    'dispatch: loop {
        match pc {
            0x830CFBB0 => {
    //   block [0x830CFBB0..0x830CFC58)
	// 830CFBB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830CFBB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830CFBB8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830CFBBC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830CFBC0: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 830CFBC4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 830CFBC8: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 830CFBCC: 4BFFE8A5  bl 0x830ce470
	ctx.lr = 0x830CFBD0;
	sub_830CE470(ctx, base);
	// 830CFBD0: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 830CFBD4: 41800070  blt 0x830cfc44
	if ctx.cr[0].lt {
	pc = 0x830CFC44; continue 'dispatch;
	}
	// 830CFBD8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 830CFBDC: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 830CFBE0: 409A005C  bne cr6, 0x830cfc3c
	if !ctx.cr[6].eq {
	pc = 0x830CFC3C; continue 'dispatch;
	}
	// 830CFBE4: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830CFBE8: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 830CFBEC: 419A0030  beq cr6, 0x830cfc1c
	if ctx.cr[6].eq {
	pc = 0x830CFC1C; continue 'dispatch;
	}
	// 830CFBF0: 2B0B0002  cmplwi cr6, r11, 2
	ctx.cr[6].compare_u32(ctx.r[11].u32, 2 as u32, &mut ctx.xer);
	// 830CFBF4: 409A0014  bne cr6, 0x830cfc08
	if !ctx.cr[6].eq {
	pc = 0x830CFC08; continue 'dispatch;
	}
	// 830CFBF8: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 830CFBFC: 814B0038  lwz r10, 0x38(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) } as u64;
	// 830CFC00: 614A0008  ori r10, r10, 8
	ctx.r[10].u64 = ctx.r[10].u64 | 8;
	// 830CFC04: 48000024  b 0x830cfc28
	pc = 0x830CFC28; continue 'dispatch;
	// 830CFC08: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830CFC0C: 409A0030  bne cr6, 0x830cfc3c
	if !ctx.cr[6].eq {
	pc = 0x830CFC3C; continue 'dispatch;
	}
	// 830CFC10: 4800F8A1  bl 0x830df4b0
	ctx.lr = 0x830CFC14;
	sub_830DF4B0(ctx, base);
	// 830CFC14: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 830CFC18: 4082FFE0  bne 0x830cfbf8
	if !ctx.cr[0].eq {
	pc = 0x830CFBF8; continue 'dispatch;
	}
	// 830CFC1C: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 830CFC20: 814B0038  lwz r10, 0x38(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) } as u64;
	// 830CFC24: 554A0776  rlwinm r10, r10, 0, 0x1d, 0x1b
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 830CFC28: 914B0038  stw r10, 0x38(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(56 as u32), ctx.r[10].u32 ) };
	// 830CFC2C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830CFC30: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830CFC34: 914B003C  stw r10, 0x3c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(60 as u32), ctx.r[10].u32 ) };
	// 830CFC38: 4800000C  b 0x830cfc44
	pc = 0x830CFC44; continue 'dispatch;
	// 830CFC3C: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 830CFC40: 60630057  ori r3, r3, 0x57
	ctx.r[3].u64 = ctx.r[3].u64 | 87;
	// 830CFC44: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830CFC48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830CFC4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830CFC50: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830CFC54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CFC58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830CFC58 size=204
    let mut pc: u32 = 0x830CFC58;
    'dispatch: loop {
        match pc {
            0x830CFC58 => {
    //   block [0x830CFC58..0x830CFD24)
	// 830CFC58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830CFC5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830CFC60: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830CFC64: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830CFC68: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830CFC6C: 81640004  lwz r11, 4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 830CFC70: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830CFC74: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830CFC78: 2B0B0803  cmplwi cr6, r11, 0x803
	ctx.cr[6].compare_u32(ctx.r[11].u32, 2051 as u32, &mut ctx.xer);
	// 830CFC7C: 419A0060  beq cr6, 0x830cfcdc
	if ctx.cr[6].eq {
	pc = 0x830CFCDC; continue 'dispatch;
	}
	// 830CFC80: 2B0B0804  cmplwi cr6, r11, 0x804
	ctx.cr[6].compare_u32(ctx.r[11].u32, 2052 as u32, &mut ctx.xer);
	// 830CFC84: 409A0088  bne cr6, 0x830cfd0c
	if !ctx.cr[6].eq {
	pc = 0x830CFD0C; continue 'dispatch;
	}
	// 830CFC88: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 830CFC8C: 83C40010  lwz r30, 0x10(r4)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(16 as u32) ) } as u64;
	// 830CFC90: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 830CFC94: 91640008  stw r11, 8(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 830CFC98: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830CFC9C: 4BFFEFED  bl 0x830cec88
	ctx.lr = 0x830CFCA0;
	sub_830CEC88(ctx, base);
	// 830CFCA0: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 830CFCA4: 41800068  blt 0x830cfd0c
	if ctx.cr[0].lt {
	pc = 0x830CFD0C; continue 'dispatch;
	}
	// 830CFCA8: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 830CFCAC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830CFCB0: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 830CFCB4: 419A000C  beq cr6, 0x830cfcc0
	if ctx.cr[6].eq {
	pc = 0x830CFCC0; continue 'dispatch;
	}
	// 830CFCB8: 616B0001  ori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 | 1;
	// 830CFCBC: 48000008  b 0x830cfcc4
	pc = 0x830CFCC4; continue 'dispatch;
	// 830CFCC0: 556B003C  rlwinm r11, r11, 0, 0, 0x1e
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 830CFCC4: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 830CFCC8: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 830CFCCC: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 830CFCD0: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 830CFCD4: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 830CFCD8: 48000034  b 0x830cfd0c
	pc = 0x830CFD0C; continue 'dispatch;
	// 830CFCDC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 830CFCE0: 81640010  lwz r11, 0x10(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(16 as u32) ) } as u64;
	// 830CFCE4: 91440008  stw r10, 8(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 830CFCE8: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 830CFCEC: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 830CFCF0: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 830CFCF4: 554A07FE  clrlwi r10, r10, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x00000001u64;
	// 830CFCF8: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 830CFCFC: 815F000C  lwz r10, 0xc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 830CFD00: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 830CFD04: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 830CFD08: 914B0010  stw r10, 0x10(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 830CFD0C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830CFD10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830CFD14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830CFD18: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830CFD1C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830CFD20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CFD28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830CFD28 size=96
    let mut pc: u32 = 0x830CFD28;
    'dispatch: loop {
        match pc {
            0x830CFD28 => {
    //   block [0x830CFD28..0x830CFD88)
	// 830CFD28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830CFD2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830CFD30: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830CFD34: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 830CFD38: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 830CFD3C: 4BFFE735  bl 0x830ce470
	ctx.lr = 0x830CFD40;
	sub_830CE470(ctx, base);
	// 830CFD40: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 830CFD44: 41800034  blt 0x830cfd78
	if ctx.cr[0].lt {
	pc = 0x830CFD78; continue 'dispatch;
	}
	// 830CFD48: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 830CFD4C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830CFD50: 419A0020  beq cr6, 0x830cfd70
	if ctx.cr[6].eq {
	pc = 0x830CFD70; continue 'dispatch;
	}
	// 830CFD54: 81450000  lwz r10, 0(r5)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 830CFD58: 2F0A0005  cmpwi cr6, r10, 5
	ctx.cr[6].compare_i32(ctx.r[10].s32, 5, &mut ctx.xer);
	// 830CFD5C: 409A0014  bne cr6, 0x830cfd70
	if !ctx.cr[6].eq {
	pc = 0x830CFD70; continue 'dispatch;
	}
	// 830CFD60: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 830CFD64: 80850004  lwz r4, 4(r5)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) } as u64;
	// 830CFD68: 4BFFF789  bl 0x830cf4f0
	ctx.lr = 0x830CFD6C;
	sub_830CF4F0(ctx, base);
	// 830CFD6C: 4800000C  b 0x830cfd78
	pc = 0x830CFD78; continue 'dispatch;
	// 830CFD70: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 830CFD74: 60630057  ori r3, r3, 0x57
	ctx.r[3].u64 = ctx.r[3].u64 | 87;
	// 830CFD78: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830CFD7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830CFD80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830CFD84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CFD88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x830CFD88 size=96
    let mut pc: u32 = 0x830CFD88;
    'dispatch: loop {
        match pc {
            0x830CFD88 => {
    //   block [0x830CFD88..0x830CFDE8)
	// 830CFD88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830CFD8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830CFD90: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830CFD94: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 830CFD98: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 830CFD9C: 4BFFE6D5  bl 0x830ce470
	ctx.lr = 0x830CFDA0;
	sub_830CE470(ctx, base);
	// 830CFDA0: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 830CFDA4: 41800034  blt 0x830cfdd8
	if ctx.cr[0].lt {
	pc = 0x830CFDD8; continue 'dispatch;
	}
	// 830CFDA8: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 830CFDAC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830CFDB0: 419A0020  beq cr6, 0x830cfdd0
	if ctx.cr[6].eq {
	pc = 0x830CFDD0; continue 'dispatch;
	}
	// 830CFDB4: 81450000  lwz r10, 0(r5)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 830CFDB8: 2F0A0004  cmpwi cr6, r10, 4
	ctx.cr[6].compare_i32(ctx.r[10].s32, 4, &mut ctx.xer);
	// 830CFDBC: 409A0014  bne cr6, 0x830cfdd0
	if !ctx.cr[6].eq {
	pc = 0x830CFDD0; continue 'dispatch;
	}
	// 830CFDC0: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 830CFDC4: C0250004  lfs f1, 4(r5)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 830CFDC8: 4BFFD8E1  bl 0x830cd6a8
	ctx.lr = 0x830CFDCC;
	sub_830CD6A8(ctx, base);
	// 830CFDCC: 4800000C  b 0x830cfdd8
	pc = 0x830CFDD8; continue 'dispatch;
	// 830CFDD0: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 830CFDD4: 60630057  ori r3, r3, 0x57
	ctx.r[3].u64 = ctx.r[3].u64 | 87;
	// 830CFDD8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830CFDDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830CFDE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830CFDE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CFDE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830CFDE8 size=132
    let mut pc: u32 = 0x830CFDE8;
    'dispatch: loop {
        match pc {
            0x830CFDE8 => {
    //   block [0x830CFDE8..0x830CFE6C)
	// 830CFDE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830CFDEC: 480D837D  bl 0x831a8168
	ctx.lr = 0x830CFDF0;
	sub_831A8130(ctx, base);
	// 830CFDF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830CFDF4: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 830CFDF8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830CFDFC: 578B07BD  rlwinm. r11, r28, 0, 0x1e, 0x1e
	ctx.r[11].u64 = ctx.r[28].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830CFE00: 41820048  beq 0x830cfe48
	if ctx.cr[0].eq {
	pc = 0x830CFE48; continue 'dispatch;
	}
	// 830CFE04: 815EFFFC  lwz r10, -4(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-4 as u32) ) } as u64;
	// 830CFE08: 3BBEFFFC  addi r29, r30, -4
	ctx.r[29].s64 = ctx.r[30].s64 + -4;
	// 830CFE0C: 1D6A000C  mulli r11, r10, 0xc
	ctx.r[11].s64 = ctx.r[10].s64 * 12;
	// 830CFE10: 37EAFFFF  addic. r31, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[31].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 830CFE14: 7FCBF214  add r30, r11, r30
	ctx.r[30].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 830CFE18: 41800018  blt 0x830cfe30
	if ctx.cr[0].lt {
	pc = 0x830CFE30; continue 'dispatch;
	}
	// 830CFE1C: 3BDEFFF4  addi r30, r30, -0xc
	ctx.r[30].s64 = ctx.r[30].s64 + -12;
	// 830CFE20: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830CFE24: 4803345D  bl 0x83103280
	ctx.lr = 0x830CFE28;
	sub_83103280(ctx, base);
	// 830CFE28: 37FFFFFF  addic. r31, r31, -1
	ctx.xer.ca = (ctx.r[31].u32 > (!(-1 as u32)));
	ctx.r[31].s64 = ctx.r[31].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 830CFE2C: 4080FFF0  bge 0x830cfe1c
	if !ctx.cr[0].lt {
	pc = 0x830CFE1C; continue 'dispatch;
	}
	// 830CFE30: 578B07FF  clrlwi. r11, r28, 0x1f
	ctx.r[11].u64 = ctx.r[28].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830CFE34: 4182000C  beq 0x830cfe40
	if ctx.cr[0].eq {
	pc = 0x830CFE40; continue 'dispatch;
	}
	// 830CFE38: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830CFE3C: 4800D69D  bl 0x830dd4d8
	ctx.lr = 0x830CFE40;
	sub_830DD4D8(ctx, base);
	// 830CFE40: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830CFE44: 48000020  b 0x830cfe64
	pc = 0x830CFE64; continue 'dispatch;
	// 830CFE48: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830CFE4C: 48033435  bl 0x83103280
	ctx.lr = 0x830CFE50;
	sub_83103280(ctx, base);
	// 830CFE50: 578B07FF  clrlwi. r11, r28, 0x1f
	ctx.r[11].u64 = ctx.r[28].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830CFE54: 4182000C  beq 0x830cfe60
	if ctx.cr[0].eq {
	pc = 0x830CFE60; continue 'dispatch;
	}
	// 830CFE58: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830CFE5C: 4800D67D  bl 0x830dd4d8
	ctx.lr = 0x830CFE60;
	sub_830DD4D8(ctx, base);
	// 830CFE60: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830CFE64: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 830CFE68: 480D8350  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CFE70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830CFE70 size=132
    let mut pc: u32 = 0x830CFE70;
    'dispatch: loop {
        match pc {
            0x830CFE70 => {
    //   block [0x830CFE70..0x830CFEF4)
	// 830CFE70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830CFE74: 480D82F5  bl 0x831a8168
	ctx.lr = 0x830CFE78;
	sub_831A8130(ctx, base);
	// 830CFE78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830CFE7C: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 830CFE80: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 830CFE84: 578B07BD  rlwinm. r11, r28, 0, 0x1e, 0x1e
	ctx.r[11].u64 = ctx.r[28].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830CFE88: 41820048  beq 0x830cfed0
	if ctx.cr[0].eq {
	pc = 0x830CFED0; continue 'dispatch;
	}
	// 830CFE8C: 815EFFFC  lwz r10, -4(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-4 as u32) ) } as u64;
	// 830CFE90: 3BBEFFFC  addi r29, r30, -4
	ctx.r[29].s64 = ctx.r[30].s64 + -4;
	// 830CFE94: 1D6A0014  mulli r11, r10, 0x14
	ctx.r[11].s64 = ctx.r[10].s64 * 20;
	// 830CFE98: 37EAFFFF  addic. r31, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[31].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 830CFE9C: 7FCBF214  add r30, r11, r30
	ctx.r[30].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 830CFEA0: 41800018  blt 0x830cfeb8
	if ctx.cr[0].lt {
	pc = 0x830CFEB8; continue 'dispatch;
	}
	// 830CFEA4: 3BDEFFEC  addi r30, r30, -0x14
	ctx.r[30].s64 = ctx.r[30].s64 + -20;
	// 830CFEA8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830CFEAC: 48032BDD  bl 0x83102a88
	ctx.lr = 0x830CFEB0;
	sub_83102A88(ctx, base);
	// 830CFEB0: 37FFFFFF  addic. r31, r31, -1
	ctx.xer.ca = (ctx.r[31].u32 > (!(-1 as u32)));
	ctx.r[31].s64 = ctx.r[31].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 830CFEB4: 4080FFF0  bge 0x830cfea4
	if !ctx.cr[0].lt {
	pc = 0x830CFEA4; continue 'dispatch;
	}
	// 830CFEB8: 578B07FF  clrlwi. r11, r28, 0x1f
	ctx.r[11].u64 = ctx.r[28].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830CFEBC: 4182000C  beq 0x830cfec8
	if ctx.cr[0].eq {
	pc = 0x830CFEC8; continue 'dispatch;
	}
	// 830CFEC0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830CFEC4: 4800D615  bl 0x830dd4d8
	ctx.lr = 0x830CFEC8;
	sub_830DD4D8(ctx, base);
	// 830CFEC8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830CFECC: 48000020  b 0x830cfeec
	pc = 0x830CFEEC; continue 'dispatch;
	// 830CFED0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830CFED4: 48032BB5  bl 0x83102a88
	ctx.lr = 0x830CFED8;
	sub_83102A88(ctx, base);
	// 830CFED8: 578B07FF  clrlwi. r11, r28, 0x1f
	ctx.r[11].u64 = ctx.r[28].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 830CFEDC: 4182000C  beq 0x830cfee8
	if ctx.cr[0].eq {
	pc = 0x830CFEE8; continue 'dispatch;
	}
	// 830CFEE0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830CFEE4: 4800D5F5  bl 0x830dd4d8
	ctx.lr = 0x830CFEE8;
	sub_830DD4D8(ctx, base);
	// 830CFEE8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830CFEEC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 830CFEF0: 480D82C8  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CFEF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830CFEF8 size=104
    let mut pc: u32 = 0x830CFEF8;
    'dispatch: loop {
        match pc {
            0x830CFEF8 => {
    //   block [0x830CFEF8..0x830CFF60)
	// 830CFEF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830CFEFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830CFF00: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830CFF04: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 830CFF08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830CFF0C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 830CFF10: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 830CFF14: 4BFFE55D  bl 0x830ce470
	ctx.lr = 0x830CFF18;
	sub_830CE470(ctx, base);
	// 830CFF18: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 830CFF1C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 830CFF20: 419A0028  beq cr6, 0x830cff48
	if ctx.cr[6].eq {
	pc = 0x830CFF48; continue 'dispatch;
	}
	// 830CFF24: 81650000  lwz r11, 0(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 830CFF28: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 830CFF2C: 409A001C  bne cr6, 0x830cff48
	if !ctx.cr[6].eq {
	pc = 0x830CFF48; continue 'dispatch;
	}
	// 830CFF30: 81650004  lwz r11, 4(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) } as u64;
	// 830CFF34: 2B0B0006  cmplwi cr6, r11, 6
	ctx.cr[6].compare_u32(ctx.r[11].u32, 6 as u32, &mut ctx.xer);
	// 830CFF38: 40980010  bge cr6, 0x830cff48
	if !ctx.cr[6].lt {
	pc = 0x830CFF48; continue 'dispatch;
	}
	// 830CFF3C: 916A0004  stw r11, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 830CFF40: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830CFF44: 4800000C  b 0x830cff50
	pc = 0x830CFF50; continue 'dispatch;
	// 830CFF48: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 830CFF4C: 60630057  ori r3, r3, 0x57
	ctx.r[3].u64 = ctx.r[3].u64 | 87;
	// 830CFF50: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830CFF54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830CFF58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830CFF5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CFF60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830CFF60 size=104
    let mut pc: u32 = 0x830CFF60;
    'dispatch: loop {
        match pc {
            0x830CFF60 => {
    //   block [0x830CFF60..0x830CFFC8)
	// 830CFF60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830CFF64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830CFF68: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830CFF6C: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 830CFF70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830CFF74: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 830CFF78: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 830CFF7C: 4BFFE4F5  bl 0x830ce470
	ctx.lr = 0x830CFF80;
	sub_830CE470(ctx, base);
	// 830CFF80: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 830CFF84: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 830CFF88: 419A0028  beq cr6, 0x830cffb0
	if ctx.cr[6].eq {
	pc = 0x830CFFB0; continue 'dispatch;
	}
	// 830CFF8C: 81650000  lwz r11, 0(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 830CFF90: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 830CFF94: 409A001C  bne cr6, 0x830cffb0
	if !ctx.cr[6].eq {
	pc = 0x830CFFB0; continue 'dispatch;
	}
	// 830CFF98: 81650004  lwz r11, 4(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) } as u64;
	// 830CFF9C: 2B0B0004  cmplwi cr6, r11, 4
	ctx.cr[6].compare_u32(ctx.r[11].u32, 4 as u32, &mut ctx.xer);
	// 830CFFA0: 40980010  bge cr6, 0x830cffb0
	if !ctx.cr[6].lt {
	pc = 0x830CFFB0; continue 'dispatch;
	}
	// 830CFFA4: 916A0004  stw r11, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 830CFFA8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830CFFAC: 4800000C  b 0x830cffb8
	pc = 0x830CFFB8; continue 'dispatch;
	// 830CFFB0: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 830CFFB4: 60630057  ori r3, r3, 0x57
	ctx.r[3].u64 = ctx.r[3].u64 | 87;
	// 830CFFB8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830CFFBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830CFFC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830CFFC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830CFFC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830CFFC8 size=104
    let mut pc: u32 = 0x830CFFC8;
    'dispatch: loop {
        match pc {
            0x830CFFC8 => {
    //   block [0x830CFFC8..0x830D0030)
	// 830CFFC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830CFFCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830CFFD0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830CFFD4: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 830CFFD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830CFFDC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 830CFFE0: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 830CFFE4: 4BFFE48D  bl 0x830ce470
	ctx.lr = 0x830CFFE8;
	sub_830CE470(ctx, base);
	// 830CFFE8: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 830CFFEC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 830CFFF0: 419A0028  beq cr6, 0x830d0018
	if ctx.cr[6].eq {
	pc = 0x830D0018; continue 'dispatch;
	}
	// 830CFFF4: 81650000  lwz r11, 0(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 830CFFF8: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 830CFFFC: 409A001C  bne cr6, 0x830d0018
	if !ctx.cr[6].eq {
	pc = 0x830D0018; continue 'dispatch;
	}
	// 830D0000: 81650004  lwz r11, 4(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) } as u64;
	// 830D0004: 2B0B0002  cmplwi cr6, r11, 2
	ctx.cr[6].compare_u32(ctx.r[11].u32, 2 as u32, &mut ctx.xer);
	// 830D0008: 40980010  bge cr6, 0x830d0018
	if !ctx.cr[6].lt {
	pc = 0x830D0018; continue 'dispatch;
	}
	// 830D000C: 916A0004  stw r11, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 830D0010: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830D0014: 4800000C  b 0x830d0020
	pc = 0x830D0020; continue 'dispatch;
	// 830D0018: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 830D001C: 60630057  ori r3, r3, 0x57
	ctx.r[3].u64 = ctx.r[3].u64 | 87;
	// 830D0020: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830D0024: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830D0028: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830D002C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830D0030(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x830D0030 size=288
    let mut pc: u32 = 0x830D0030;
    'dispatch: loop {
        match pc {
            0x830D0030 => {
    //   block [0x830D0030..0x830D0150)
	// 830D0030: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830D0034: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830D0038: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830D003C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830D0040: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830D0044: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 830D0048: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830D004C: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 830D0050: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 830D0054: 917F00BC  stw r11, 0xbc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(188 as u32), ctx.r[11].u32 ) };
	// 830D0058: 387F0064  addi r3, r31, 0x64
	ctx.r[3].s64 = ctx.r[31].s64 + 100;
	// 830D005C: 917F00C0  stw r11, 0xc0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(192 as u32), ctx.r[11].u32 ) };
	// 830D0060: 38A00024  li r5, 0x24
	ctx.r[5].s64 = 36;
	// 830D0064: 917F00C4  stw r11, 0xc4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(196 as u32), ctx.r[11].u32 ) };
	// 830D0068: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 830D006C: 917F00C8  stw r11, 0xc8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(200 as u32), ctx.r[11].u32 ) };
	// 830D0070: C00908A4  lfs f0, 0x8a4(r9)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 830D0074: 917F00CC  stw r11, 0xcc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(204 as u32), ctx.r[11].u32 ) };
	// 830D0078: 917F00D0  stw r11, 0xd0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(208 as u32), ctx.r[11].u32 ) };
	// 830D007C: 917F00D4  stw r11, 0xd4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(212 as u32), ctx.r[11].u32 ) };
	// 830D0080: 917F00D8  stw r11, 0xd8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(216 as u32), ctx.r[11].u32 ) };
	// 830D0084: D01F0034  stfs f0, 0x34(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), tmp.u32 ) };
	// 830D0088: D01F0038  stfs f0, 0x38(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), tmp.u32 ) };
	// 830D008C: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 830D0090: D01F003C  stfs f0, 0x3c(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), tmp.u32 ) };
	// 830D0094: 911F0008  stw r8, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[8].u32 ) };
	// 830D0098: D01F0040  stfs f0, 0x40(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), tmp.u32 ) };
	// 830D009C: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 830D00A0: D01F0090  stfs f0, 0x90(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(144 as u32), tmp.u32 ) };
	// 830D00A4: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 830D00A8: D01F0094  stfs f0, 0x94(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), tmp.u32 ) };
	// 830D00AC: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 830D00B0: D01F0098  stfs f0, 0x98(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(152 as u32), tmp.u32 ) };
	// 830D00B4: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 830D00B8: D01F0044  stfs f0, 0x44(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(68 as u32), tmp.u32 ) };
	// 830D00BC: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 830D00C0: D01F0048  stfs f0, 0x48(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), tmp.u32 ) };
	// 830D00C4: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 830D00C8: D01F004C  stfs f0, 0x4c(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(76 as u32), tmp.u32 ) };
	// 830D00CC: 917F0024  stw r11, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 830D00D0: D01F0050  stfs f0, 0x50(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 830D00D4: 917F0028  stw r11, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 830D00D8: D01F0054  stfs f0, 0x54(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 830D00DC: 917F002C  stw r11, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 830D00E0: D01F0058  stfs f0, 0x58(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), tmp.u32 ) };
	// 830D00E4: 917F0030  stw r11, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 830D00E8: D01F005C  stfs f0, 0x5c(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), tmp.u32 ) };
	// 830D00EC: 917F009C  stw r11, 0x9c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(156 as u32), ctx.r[11].u32 ) };
	// 830D00F0: D01F0060  stfs f0, 0x60(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), tmp.u32 ) };
	// 830D00F4: 917F00A0  stw r11, 0xa0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(160 as u32), ctx.r[11].u32 ) };
	// 830D00F8: D01F008C  stfs f0, 0x8c(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(140 as u32), tmp.u32 ) };
	// 830D00FC: 917F00A4  stw r11, 0xa4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(164 as u32), ctx.r[11].u32 ) };
	// 830D0100: D01F0088  stfs f0, 0x88(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(136 as u32), tmp.u32 ) };
	// 830D0104: 917F00A8  stw r11, 0xa8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(168 as u32), ctx.r[11].u32 ) };
	// 830D0108: 917F00AC  stw r11, 0xac(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(172 as u32), ctx.r[11].u32 ) };
	// 830D010C: 917F00B0  stw r11, 0xb0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(176 as u32), ctx.r[11].u32 ) };
	// 830D0110: 917F00B4  stw r11, 0xb4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(180 as u32), ctx.r[11].u32 ) };
	// 830D0114: 917F00B8  stw r11, 0xb8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(184 as u32), ctx.r[11].u32 ) };
	// 830D0118: 917F00E0  stw r11, 0xe0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(224 as u32), ctx.r[11].u32 ) };
	// 830D011C: 917F00E4  stw r11, 0xe4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(228 as u32), ctx.r[11].u32 ) };
	// 830D0120: 915F00E8  stw r10, 0xe8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(232 as u32), ctx.r[10].u32 ) };
	// 830D0124: 915F00DC  stw r10, 0xdc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(220 as u32), ctx.r[10].u32 ) };
	// 830D0128: 915F00EC  stw r10, 0xec(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(236 as u32), ctx.r[10].u32 ) };
	// 830D012C: 917F00F0  stw r11, 0xf0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(240 as u32), ctx.r[11].u32 ) };
	// 830D0130: 917F00F4  stw r11, 0xf4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(244 as u32), ctx.r[11].u32 ) };
	// 830D0134: 480D80AD  bl 0x831a81e0
	ctx.lr = 0x830D0138;
	sub_831A81E0(ctx, base);
	// 830D0138: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830D013C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830D0140: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830D0144: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830D0148: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830D014C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830D0150(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830D0150 size=116
    let mut pc: u32 = 0x830D0150;
    'dispatch: loop {
        match pc {
            0x830D0150 => {
    //   block [0x830D0150..0x830D01C4)
	// 830D0150: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830D0154: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830D0158: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830D015C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830D0160: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 830D0164: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 830D0168: 4BFFE309  bl 0x830ce470
	ctx.lr = 0x830D016C;
	sub_830CE470(ctx, base);
	// 830D016C: 83E10050  lwz r31, 0x50(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 830D0170: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 830D0174: 419A0034  beq cr6, 0x830d01a8
	if ctx.cr[6].eq {
	pc = 0x830D01A8; continue 'dispatch;
	}
	// 830D0178: 81650000  lwz r11, 0(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 830D017C: 2F0B0005  cmpwi cr6, r11, 5
	ctx.cr[6].compare_i32(ctx.r[11].s32, 5, &mut ctx.xer);
	// 830D0180: 409A0028  bne cr6, 0x830d01a8
	if !ctx.cr[6].eq {
	pc = 0x830D01A8; continue 'dispatch;
	}
	// 830D0184: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 830D0188: 80850004  lwz r4, 4(r5)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) } as u64;
	// 830D018C: 4BFFEAFD  bl 0x830cec88
	ctx.lr = 0x830D0190;
	sub_830CEC88(ctx, base);
	// 830D0190: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 830D0194: 4180001C  blt 0x830d01b0
	if ctx.cr[0].lt {
	pc = 0x830D01B0; continue 'dispatch;
	}
	// 830D0198: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830D019C: 4802E695  bl 0x830fe830
	ctx.lr = 0x830D01A0;
	sub_830FE830(ctx, base);
	// 830D01A0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830D01A4: 4800000C  b 0x830d01b0
	pc = 0x830D01B0; continue 'dispatch;
	// 830D01A8: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 830D01AC: 60630057  ori r3, r3, 0x57
	ctx.r[3].u64 = ctx.r[3].u64 | 87;
	// 830D01B0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830D01B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830D01B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830D01BC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830D01C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830D01C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830D01C8 size=116
    let mut pc: u32 = 0x830D01C8;
    'dispatch: loop {
        match pc {
            0x830D01C8 => {
    //   block [0x830D01C8..0x830D023C)
	// 830D01C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830D01CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830D01D0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830D01D4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830D01D8: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 830D01DC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 830D01E0: 4BFFE291  bl 0x830ce470
	ctx.lr = 0x830D01E4;
	sub_830CE470(ctx, base);
	// 830D01E4: 83E10050  lwz r31, 0x50(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 830D01E8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 830D01EC: 419A0034  beq cr6, 0x830d0220
	if ctx.cr[6].eq {
	pc = 0x830D0220; continue 'dispatch;
	}
	// 830D01F0: 81650000  lwz r11, 0(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 830D01F4: 2F0B0005  cmpwi cr6, r11, 5
	ctx.cr[6].compare_i32(ctx.r[11].s32, 5, &mut ctx.xer);
	// 830D01F8: 409A0028  bne cr6, 0x830d0220
	if !ctx.cr[6].eq {
	pc = 0x830D0220; continue 'dispatch;
	}
	// 830D01FC: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 830D0200: 80850004  lwz r4, 4(r5)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) } as u64;
	// 830D0204: 4BFFEA85  bl 0x830cec88
	ctx.lr = 0x830D0208;
	sub_830CEC88(ctx, base);
	// 830D0208: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 830D020C: 4180001C  blt 0x830d0228
	if ctx.cr[0].lt {
	pc = 0x830D0228; continue 'dispatch;
	}
	// 830D0210: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830D0214: 4802E61D  bl 0x830fe830
	ctx.lr = 0x830D0218;
	sub_830FE830(ctx, base);
	// 830D0218: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830D021C: 4800000C  b 0x830d0228
	pc = 0x830D0228; continue 'dispatch;
	// 830D0220: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 830D0224: 60630057  ori r3, r3, 0x57
	ctx.r[3].u64 = ctx.r[3].u64 | 87;
	// 830D0228: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830D022C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830D0230: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830D0234: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830D0238: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830D0240(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830D0240 size=116
    let mut pc: u32 = 0x830D0240;
    'dispatch: loop {
        match pc {
            0x830D0240 => {
    //   block [0x830D0240..0x830D02B4)
	// 830D0240: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830D0244: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830D0248: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830D024C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830D0250: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 830D0254: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 830D0258: 4BFFE219  bl 0x830ce470
	ctx.lr = 0x830D025C;
	sub_830CE470(ctx, base);
	// 830D025C: 83E10050  lwz r31, 0x50(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 830D0260: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 830D0264: 419A0034  beq cr6, 0x830d0298
	if ctx.cr[6].eq {
	pc = 0x830D0298; continue 'dispatch;
	}
	// 830D0268: 81650000  lwz r11, 0(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 830D026C: 2F0B0005  cmpwi cr6, r11, 5
	ctx.cr[6].compare_i32(ctx.r[11].s32, 5, &mut ctx.xer);
	// 830D0270: 409A0028  bne cr6, 0x830d0298
	if !ctx.cr[6].eq {
	pc = 0x830D0298; continue 'dispatch;
	}
	// 830D0274: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 830D0278: 80850004  lwz r4, 4(r5)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) } as u64;
	// 830D027C: 4BFFEA0D  bl 0x830cec88
	ctx.lr = 0x830D0280;
	sub_830CEC88(ctx, base);
	// 830D0280: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 830D0284: 4180001C  blt 0x830d02a0
	if ctx.cr[0].lt {
	pc = 0x830D02A0; continue 'dispatch;
	}
	// 830D0288: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830D028C: 4802E5A5  bl 0x830fe830
	ctx.lr = 0x830D0290;
	sub_830FE830(ctx, base);
	// 830D0290: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830D0294: 4800000C  b 0x830d02a0
	pc = 0x830D02A0; continue 'dispatch;
	// 830D0298: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 830D029C: 60630057  ori r3, r3, 0x57
	ctx.r[3].u64 = ctx.r[3].u64 | 87;
	// 830D02A0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830D02A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830D02A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830D02AC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830D02B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830D02B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830D02B8 size=80
    let mut pc: u32 = 0x830D02B8;
    'dispatch: loop {
        match pc {
            0x830D02B8 => {
    //   block [0x830D02B8..0x830D0308)
	// 830D02B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830D02BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830D02C0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830D02C4: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 830D02C8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 830D02CC: 4BFFE1A5  bl 0x830ce470
	ctx.lr = 0x830D02D0;
	sub_830CE470(ctx, base);
	// 830D02D0: 81650000  lwz r11, 0(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 830D02D4: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 830D02D8: 419A0010  beq cr6, 0x830d02e8
	if ctx.cr[6].eq {
	pc = 0x830D02E8; continue 'dispatch;
	}
	// 830D02DC: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 830D02E0: 60630057  ori r3, r3, 0x57
	ctx.r[3].u64 = ctx.r[3].u64 | 87;
	// 830D02E4: 48000014  b 0x830d02f8
	pc = 0x830D02F8; continue 'dispatch;
	// 830D02E8: 81650004  lwz r11, 4(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) } as u64;
	// 830D02EC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830D02F0: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 830D02F4: 916A0010  stw r11, 0x10(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 830D02F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830D02FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830D0300: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830D0304: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830D0308(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830D0308 size=248
    let mut pc: u32 = 0x830D0308;
    'dispatch: loop {
        match pc {
            0x830D0308 => {
    //   block [0x830D0308..0x830D0400)
	// 830D0308: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830D030C: 480D7E61  bl 0x831a816c
	ctx.lr = 0x830D0310;
	sub_831A8130(ctx, base);
	// 830D0310: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830D0314: 3D008339  lis r8, -0x7cc7
	ctx.r[8].s64 = -2093416448;
	// 830D0318: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 830D031C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 830D0320: 3BEB4880  addi r31, r11, 0x4880
	ctx.r[31].s64 = ctx.r[11].s64 + 18560;
	// 830D0324: 8168C4E8  lwz r11, -0x3b18(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(-15128 as u32) ) } as u64;
	// 830D0328: 556A07FF  clrlwi. r10, r11, 0x1f
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 830D032C: 408200C0  bne 0x830d03ec
	if !ctx.cr[0].eq {
	pc = 0x830D03EC; continue 'dispatch;
	}
	// 830D0330: 3D408219  lis r10, -0x7de7
	ctx.r[10].s64 = -2112290816;
	// 830D0334: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 830D0338: 392A8028  addi r9, r10, -0x7fd8
	ctx.r[9].s64 = ctx.r[10].s64 + -32728;
	// 830D033C: 616B0001  ori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 | 1;
	// 830D0340: 7FCAF378  mr r10, r30
	ctx.r[10].u64 = ctx.r[30].u64;
	// 830D0344: 913F0010  stw r9, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[9].u32 ) };
	// 830D0348: 9168C4E8  stw r11, -0x3b18(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(-15128 as u32), ctx.r[11].u32 ) };
	// 830D034C: 39600010  li r11, 0x10
	ctx.r[11].s64 = 16;
	// 830D0350: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 830D0354: 7FC9F378  mr r9, r30
	ctx.r[9].u64 = ctx.r[30].u64;
	// 830D0358: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 830D035C: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 830D0360: 913F000C  stw r9, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 830D0364: 915F0014  stw r10, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 830D0368: 48012B71  bl 0x830e2ed8
	ctx.lr = 0x830D036C;
	sub_830E2ED8(ctx, base);
	// 830D036C: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 830D0370: 3D20830D  lis r9, -0x7cf3
	ctx.r[9].s64 = -2096300032;
	// 830D0374: 907F0018  stw r3, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[3].u32 ) };
	// 830D0378: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 830D037C: 7FCAF378  mr r10, r30
	ctx.r[10].u64 = ctx.r[30].u64;
	// 830D0380: 392902B8  addi r9, r9, 0x2b8
	ctx.r[9].s64 = ctx.r[9].s64 + 696;
	// 830D0384: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 830D0388: 915F0020  stw r10, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 830D038C: 913F0024  stw r9, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[9].u32 ) };
	// 830D0390: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 830D0394: 392B8000  addi r9, r11, -0x8000
	ctx.r[9].s64 = ctx.r[11].s64 + -32768;
	// 830D0398: FBDF0028  std r30, 0x28(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[30].u64 ) };
	// 830D039C: 915F0034  stw r10, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[10].u32 ) };
	// 830D03A0: 913F0040  stw r9, 0x40(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), ctx.r[9].u32 ) };
	// 830D03A4: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 830D03A8: 3920001C  li r9, 0x1c
	ctx.r[9].s64 = 28;
	// 830D03AC: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 830D03B0: 917F0030  stw r11, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 830D03B4: 917F003C  stw r11, 0x3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[11].u32 ) };
	// 830D03B8: 397F0028  addi r11, r31, 0x28
	ctx.r[11].s64 = ctx.r[31].s64 + 40;
	// 830D03BC: 913F0038  stw r9, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[9].u32 ) };
	// 830D03C0: 915F0044  stw r10, 0x44(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(68 as u32), ctx.r[10].u32 ) };
	// 830D03C4: 48012B15  bl 0x830e2ed8
	ctx.lr = 0x830D03C8;
	sub_830E2ED8(ctx, base);
	// 830D03C8: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 830D03CC: 7FCAF378  mr r10, r30
	ctx.r[10].u64 = ctx.r[30].u64;
	// 830D03D0: 907F0048  stw r3, 0x48(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[3].u32 ) };
	// 830D03D4: 7FC9F378  mr r9, r30
	ctx.r[9].u64 = ctx.r[30].u64;
	// 830D03D8: 917F004C  stw r11, 0x4c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(76 as u32), ctx.r[11].u32 ) };
	// 830D03DC: 915F0050  stw r10, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 830D03E0: 397F0058  addi r11, r31, 0x58
	ctx.r[11].s64 = ctx.r[31].s64 + 88;
	// 830D03E4: 913F0054  stw r9, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[9].u32 ) };
	// 830D03E8: FBDF0058  std r30, 0x58(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[30].u64 ) };
	// 830D03EC: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 830D03F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830D03F4: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830D03F8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830D03FC: 480D7DC0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830D0400(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830D0400 size=88
    let mut pc: u32 = 0x830D0400;
    'dispatch: loop {
        match pc {
            0x830D0400 => {
    //   block [0x830D0400..0x830D0458)
	// 830D0400: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830D0404: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830D0408: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830D040C: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 830D0410: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 830D0414: 4BFFE05D  bl 0x830ce470
	ctx.lr = 0x830D0418;
	sub_830CE470(ctx, base);
	// 830D0418: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 830D041C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830D0420: 419A0020  beq cr6, 0x830d0440
	if ctx.cr[6].eq {
	pc = 0x830D0440; continue 'dispatch;
	}
	// 830D0424: 81450000  lwz r10, 0(r5)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 830D0428: 2F0A0002  cmpwi cr6, r10, 2
	ctx.cr[6].compare_i32(ctx.r[10].s32, 2, &mut ctx.xer);
	// 830D042C: 409A0014  bne cr6, 0x830d0440
	if !ctx.cr[6].eq {
	pc = 0x830D0440; continue 'dispatch;
	}
	// 830D0430: 81450004  lwz r10, 4(r5)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) } as u64;
	// 830D0434: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830D0438: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 830D043C: 4800000C  b 0x830d0448
	pc = 0x830D0448; continue 'dispatch;
	// 830D0440: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 830D0444: 60630057  ori r3, r3, 0x57
	ctx.r[3].u64 = ctx.r[3].u64 | 87;
	// 830D0448: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830D044C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830D0450: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830D0454: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830D0458(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830D0458 size=88
    let mut pc: u32 = 0x830D0458;
    'dispatch: loop {
        match pc {
            0x830D0458 => {
    //   block [0x830D0458..0x830D04B0)
	// 830D0458: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830D045C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830D0460: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830D0464: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 830D0468: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 830D046C: 4BFFE005  bl 0x830ce470
	ctx.lr = 0x830D0470;
	sub_830CE470(ctx, base);
	// 830D0470: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 830D0474: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830D0478: 419A0020  beq cr6, 0x830d0498
	if ctx.cr[6].eq {
	pc = 0x830D0498; continue 'dispatch;
	}
	// 830D047C: 81450000  lwz r10, 0(r5)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 830D0480: 2F0A0002  cmpwi cr6, r10, 2
	ctx.cr[6].compare_i32(ctx.r[10].s32, 2, &mut ctx.xer);
	// 830D0484: 409A0014  bne cr6, 0x830d0498
	if !ctx.cr[6].eq {
	pc = 0x830D0498; continue 'dispatch;
	}
	// 830D0488: 81450004  lwz r10, 4(r5)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) } as u64;
	// 830D048C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830D0490: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 830D0494: 4800000C  b 0x830d04a0
	pc = 0x830D04A0; continue 'dispatch;
	// 830D0498: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 830D049C: 60630057  ori r3, r3, 0x57
	ctx.r[3].u64 = ctx.r[3].u64 | 87;
	// 830D04A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830D04A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830D04A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830D04AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830D04B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830D04B0 size=88
    let mut pc: u32 = 0x830D04B0;
    'dispatch: loop {
        match pc {
            0x830D04B0 => {
    //   block [0x830D04B0..0x830D0508)
	// 830D04B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830D04B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830D04B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830D04BC: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 830D04C0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 830D04C4: 4BFFDFAD  bl 0x830ce470
	ctx.lr = 0x830D04C8;
	sub_830CE470(ctx, base);
	// 830D04C8: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 830D04CC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 830D04D0: 419A0020  beq cr6, 0x830d04f0
	if ctx.cr[6].eq {
	pc = 0x830D04F0; continue 'dispatch;
	}
	// 830D04D4: 81650000  lwz r11, 0(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 830D04D8: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 830D04DC: 409A0014  bne cr6, 0x830d04f0
	if !ctx.cr[6].eq {
	pc = 0x830D04F0; continue 'dispatch;
	}
	// 830D04E0: 80850004  lwz r4, 4(r5)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) } as u64;
	// 830D04E4: 480238C5  bl 0x830f3da8
	ctx.lr = 0x830D04E8;
	sub_830F3DA8(ctx, base);
	// 830D04E8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830D04EC: 4800000C  b 0x830d04f8
	pc = 0x830D04F8; continue 'dispatch;
	// 830D04F0: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 830D04F4: 60630057  ori r3, r3, 0x57
	ctx.r[3].u64 = ctx.r[3].u64 | 87;
	// 830D04F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830D04FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830D0500: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830D0504: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830D0508(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830D0508 size=88
    let mut pc: u32 = 0x830D0508;
    'dispatch: loop {
        match pc {
            0x830D0508 => {
    //   block [0x830D0508..0x830D0560)
	// 830D0508: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830D050C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830D0510: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830D0514: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 830D0518: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 830D051C: 4BFFDF55  bl 0x830ce470
	ctx.lr = 0x830D0520;
	sub_830CE470(ctx, base);
	// 830D0520: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 830D0524: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830D0528: 419A0020  beq cr6, 0x830d0548
	if ctx.cr[6].eq {
	pc = 0x830D0548; continue 'dispatch;
	}
	// 830D052C: 81450000  lwz r10, 0(r5)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 830D0530: 2F0A0002  cmpwi cr6, r10, 2
	ctx.cr[6].compare_i32(ctx.r[10].s32, 2, &mut ctx.xer);
	// 830D0534: 409A0014  bne cr6, 0x830d0548
	if !ctx.cr[6].eq {
	pc = 0x830D0548; continue 'dispatch;
	}
	// 830D0538: 81450004  lwz r10, 4(r5)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) } as u64;
	// 830D053C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830D0540: 914B0010  stw r10, 0x10(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 830D0544: 4800000C  b 0x830d0550
	pc = 0x830D0550; continue 'dispatch;
	// 830D0548: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 830D054C: 60630057  ori r3, r3, 0x57
	ctx.r[3].u64 = ctx.r[3].u64 | 87;
	// 830D0550: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830D0554: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830D0558: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830D055C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830D0560(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830D0560 size=88
    let mut pc: u32 = 0x830D0560;
    'dispatch: loop {
        match pc {
            0x830D0560 => {
    //   block [0x830D0560..0x830D05B8)
	// 830D0560: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830D0564: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830D0568: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830D056C: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 830D0570: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 830D0574: 4BFFDEFD  bl 0x830ce470
	ctx.lr = 0x830D0578;
	sub_830CE470(ctx, base);
	// 830D0578: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 830D057C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830D0580: 419A0020  beq cr6, 0x830d05a0
	if ctx.cr[6].eq {
	pc = 0x830D05A0; continue 'dispatch;
	}
	// 830D0584: 81450000  lwz r10, 0(r5)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 830D0588: 2F0A0002  cmpwi cr6, r10, 2
	ctx.cr[6].compare_i32(ctx.r[10].s32, 2, &mut ctx.xer);
	// 830D058C: 409A0014  bne cr6, 0x830d05a0
	if !ctx.cr[6].eq {
	pc = 0x830D05A0; continue 'dispatch;
	}
	// 830D0590: 81450004  lwz r10, 4(r5)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) } as u64;
	// 830D0594: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830D0598: 914B0014  stw r10, 0x14(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 830D059C: 4800000C  b 0x830d05a8
	pc = 0x830D05A8; continue 'dispatch;
	// 830D05A0: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 830D05A4: 60630057  ori r3, r3, 0x57
	ctx.r[3].u64 = ctx.r[3].u64 | 87;
	// 830D05A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830D05AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830D05B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830D05B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830D05B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830D05B8 size=88
    let mut pc: u32 = 0x830D05B8;
    'dispatch: loop {
        match pc {
            0x830D05B8 => {
    //   block [0x830D05B8..0x830D0610)
	// 830D05B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830D05BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830D05C0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830D05C4: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 830D05C8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 830D05CC: 4BFFDEA5  bl 0x830ce470
	ctx.lr = 0x830D05D0;
	sub_830CE470(ctx, base);
	// 830D05D0: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 830D05D4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 830D05D8: 419A0020  beq cr6, 0x830d05f8
	if ctx.cr[6].eq {
	pc = 0x830D05F8; continue 'dispatch;
	}
	// 830D05DC: 81650000  lwz r11, 0(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 830D05E0: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 830D05E4: 409A0014  bne cr6, 0x830d05f8
	if !ctx.cr[6].eq {
	pc = 0x830D05F8; continue 'dispatch;
	}
	// 830D05E8: 80850004  lwz r4, 4(r5)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) } as u64;
	// 830D05EC: 48023995  bl 0x830f3f80
	ctx.lr = 0x830D05F0;
	sub_830F3F80(ctx, base);
	// 830D05F0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830D05F4: 4800000C  b 0x830d0600
	pc = 0x830D0600; continue 'dispatch;
	// 830D05F8: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 830D05FC: 60630057  ori r3, r3, 0x57
	ctx.r[3].u64 = ctx.r[3].u64 | 87;
	// 830D0600: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830D0604: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830D0608: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830D060C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830D0610(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830D0610 size=88
    let mut pc: u32 = 0x830D0610;
    'dispatch: loop {
        match pc {
            0x830D0610 => {
    //   block [0x830D0610..0x830D0668)
	// 830D0610: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830D0614: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830D0618: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830D061C: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 830D0620: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 830D0624: 4BFFDE4D  bl 0x830ce470
	ctx.lr = 0x830D0628;
	sub_830CE470(ctx, base);
	// 830D0628: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 830D062C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830D0630: 419A0020  beq cr6, 0x830d0650
	if ctx.cr[6].eq {
	pc = 0x830D0650; continue 'dispatch;
	}
	// 830D0634: 81450000  lwz r10, 0(r5)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 830D0638: 2F0A0002  cmpwi cr6, r10, 2
	ctx.cr[6].compare_i32(ctx.r[10].s32, 2, &mut ctx.xer);
	// 830D063C: 409A0014  bne cr6, 0x830d0650
	if !ctx.cr[6].eq {
	pc = 0x830D0650; continue 'dispatch;
	}
	// 830D0640: 81450004  lwz r10, 4(r5)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) } as u64;
	// 830D0644: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830D0648: 914B001C  stw r10, 0x1c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(28 as u32), ctx.r[10].u32 ) };
	// 830D064C: 4800000C  b 0x830d0658
	pc = 0x830D0658; continue 'dispatch;
	// 830D0650: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 830D0654: 60630057  ori r3, r3, 0x57
	ctx.r[3].u64 = ctx.r[3].u64 | 87;
	// 830D0658: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830D065C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830D0660: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830D0664: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830D0668(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830D0668 size=112
    let mut pc: u32 = 0x830D0668;
    'dispatch: loop {
        match pc {
            0x830D0668 => {
    //   block [0x830D0668..0x830D06D8)
	// 830D0668: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830D066C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830D0670: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830D0674: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 830D0678: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 830D067C: 4BFFDDF5  bl 0x830ce470
	ctx.lr = 0x830D0680;
	sub_830CE470(ctx, base);
	// 830D0680: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 830D0684: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830D0688: 419A0038  beq cr6, 0x830d06c0
	if ctx.cr[6].eq {
	pc = 0x830D06C0; continue 'dispatch;
	}
	// 830D068C: 81450000  lwz r10, 0(r5)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 830D0690: 2F0A0001  cmpwi cr6, r10, 1
	ctx.cr[6].compare_i32(ctx.r[10].s32, 1, &mut ctx.xer);
	// 830D0694: 409A002C  bne cr6, 0x830d06c0
	if !ctx.cr[6].eq {
	pc = 0x830D06C0; continue 'dispatch;
	}
	// 830D0698: 81450004  lwz r10, 4(r5)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) } as u64;
	// 830D069C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830D06A0: 812B0048  lwz r9, 0x48(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(72 as u32) ) } as u64;
	// 830D06A4: 7D4A0034  cntlzw r10, r10
	ctx.r[10].u64 = if ctx.r[10].u32 == 0 { 32 } else { ctx.r[10].u32.leading_zeros() as u64 };
	// 830D06A8: 5529003C  rlwinm r9, r9, 0, 0, 0x1e
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0xFFFFFFFFu64;
	// 830D06AC: 554ADFFE  rlwinm r10, r10, 0x1b, 0x1f, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x0000001Fu64;
	// 830D06B0: 694A0001  xori r10, r10, 1
	ctx.r[10].u64 = ctx.r[10].u64 ^ 1;
	// 830D06B4: 7D4A4B78  or r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[9].u64;
	// 830D06B8: 914B0048  stw r10, 0x48(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(72 as u32), ctx.r[10].u32 ) };
	// 830D06BC: 4800000C  b 0x830d06c8
	pc = 0x830D06C8; continue 'dispatch;
	// 830D06C0: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 830D06C4: 60630057  ori r3, r3, 0x57
	ctx.r[3].u64 = ctx.r[3].u64 | 87;
	// 830D06C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830D06CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830D06D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830D06D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830D06D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830D06D8 size=64
    let mut pc: u32 = 0x830D06D8;
    'dispatch: loop {
        match pc {
            0x830D06D8 => {
    //   block [0x830D06D8..0x830D0718)
	// 830D06D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830D06DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830D06E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830D06E4: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 830D06E8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 830D06EC: 4BFFDD85  bl 0x830ce470
	ctx.lr = 0x830D06F0;
	sub_830CE470(ctx, base);
	// 830D06F0: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 830D06F4: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 830D06F8: 816B0048  lwz r11, 0x48(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(72 as u32) ) } as u64;
	// 830D06FC: 556407FE  clrlwi r4, r11, 0x1f
	ctx.r[4].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	// 830D0700: 48031E71  bl 0x83102570
	ctx.lr = 0x830D0704;
	sub_83102570(ctx, base);
	// 830D0704: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830D0708: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830D070C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830D0710: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830D0714: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830D0718(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x830D0718 size=252
    let mut pc: u32 = 0x830D0718;
    'dispatch: loop {
        match pc {
            0x830D0718 => {
    //   block [0x830D0718..0x830D0814)
	// 830D0718: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830D071C: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 830D0720: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 830D0724: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 830D0728: 3D008201  lis r8, -0x7dff
	ctx.r[8].s64 = -2113863680;
	// 830D072C: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 830D0730: 3CE08201  lis r7, -0x7dff
	ctx.r[7].s64 = -2113863680;
	// 830D0734: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 830D0738: 3CC08201  lis r6, -0x7dff
	ctx.r[6].s64 = -2113863680;
	// 830D073C: 99430010  stb r10, 0x10(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[10].u8 ) };
	// 830D0740: 91630014  stw r11, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 830D0744: 3CA00000  lis r5, 0
	ctx.r[5].s64 = 0;
	// 830D0748: 91630018  stw r11, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 830D074C: C00908A4  lfs f0, 0x8a4(r9)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 830D0750: 9163001C  stw r11, 0x1c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 830D0754: 60A9FFFE  ori r9, r5, 0xfffe
	ctx.r[9].u64 = ctx.r[5].u64 | 65534;
	// 830D0758: 99430020  stb r10, 0x20(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[10].u8 ) };
	// 830D075C: C1A8A1C4  lfs f13, -0x5e3c(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(-24124 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 830D0760: 91630024  stw r11, 0x24(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 830D0764: C187CFC8  lfs f12, -0x3038(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(-12344 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 830D0768: 91630028  stw r11, 0x28(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 830D076C: C166959C  lfs f11, -0x6a64(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-27236 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 830D0770: 9163002C  stw r11, 0x2c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 830D0774: 99430030  stb r10, 0x30(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(48 as u32), ctx.r[10].u8 ) };
	// 830D0778: 91630034  stw r11, 0x34(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 830D077C: 91630038  stw r11, 0x38(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 830D0780: 9163003C  stw r11, 0x3c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(60 as u32), ctx.r[11].u32 ) };
	// 830D0784: 99430040  stb r10, 0x40(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(64 as u32), ctx.r[10].u8 ) };
	// 830D0788: 91630098  stw r11, 0x98(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(152 as u32), ctx.r[11].u32 ) };
	// 830D078C: D003009C  stfs f0, 0x9c(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(156 as u32), tmp.u32 ) };
	// 830D0790: D00300A0  stfs f0, 0xa0(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(160 as u32), tmp.u32 ) };
	// 830D0794: 916300B8  stw r11, 0xb8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(184 as u32), ctx.r[11].u32 ) };
	// 830D0798: D00300A4  stfs f0, 0xa4(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(164 as u32), tmp.u32 ) };
	// 830D079C: D00300A8  stfs f0, 0xa8(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(168 as u32), tmp.u32 ) };
	// 830D07A0: D1A300AC  stfs f13, 0xac(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(172 as u32), tmp.u32 ) };
	// 830D07A4: D18300B0  stfs f12, 0xb0(r3)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(176 as u32), tmp.u32 ) };
	// 830D07A8: D16300B4  stfs f11, 0xb4(r3)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(180 as u32), tmp.u32 ) };
	// 830D07AC: 9163005C  stw r11, 0x5c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 830D07B0: D00300C0  stfs f0, 0xc0(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(192 as u32), tmp.u32 ) };
	// 830D07B4: 91630068  stw r11, 0x68(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 830D07B8: D00300C4  stfs f0, 0xc4(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(196 as u32), tmp.u32 ) };
	// 830D07BC: 91630064  stw r11, 0x64(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 830D07C0: D00300C8  stfs f0, 0xc8(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(200 as u32), tmp.u32 ) };
	// 830D07C4: 91630054  stw r11, 0x54(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 830D07C8: D00300CC  stfs f0, 0xcc(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(204 as u32), tmp.u32 ) };
	// 830D07CC: 91630058  stw r11, 0x58(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 830D07D0: 9163006C  stw r11, 0x6c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 830D07D4: 91630070  stw r11, 0x70(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(112 as u32), ctx.r[11].u32 ) };
	// 830D07D8: 91630080  stw r11, 0x80(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 830D07DC: 91630088  stw r11, 0x88(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(136 as u32), ctx.r[11].u32 ) };
	// 830D07E0: 91630074  stw r11, 0x74(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 830D07E4: 9163008C  stw r11, 0x8c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(140 as u32), ctx.r[11].u32 ) };
	// 830D07E8: 91230090  stw r9, 0x90(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(144 as u32), ctx.r[9].u32 ) };
	// 830D07EC: 91630094  stw r11, 0x94(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(148 as u32), ctx.r[11].u32 ) };
	// 830D07F0: 91630078  stw r11, 0x78(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(120 as u32), ctx.r[11].u32 ) };
	// 830D07F4: 9163007C  stw r11, 0x7c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(124 as u32), ctx.r[11].u32 ) };
	// 830D07F8: 996300BC  stb r11, 0xbc(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(188 as u32), ctx.r[11].u8 ) };
	// 830D07FC: 91630060  stw r11, 0x60(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 830D0800: 91630044  stw r11, 0x44(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(68 as u32), ctx.r[11].u32 ) };
	// 830D0804: 91630048  stw r11, 0x48(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(72 as u32), ctx.r[11].u32 ) };
	// 830D0808: 9163004C  stw r11, 0x4c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(76 as u32), ctx.r[11].u32 ) };
	// 830D080C: 91630050  stw r11, 0x50(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 830D0810: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830D0818(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830D0818 size=88
    let mut pc: u32 = 0x830D0818;
    'dispatch: loop {
        match pc {
            0x830D0818 => {
    //   block [0x830D0818..0x830D0870)
	// 830D0818: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830D081C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830D0820: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830D0824: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830D0828: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830D082C: 807F005C  lwz r3, 0x5c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 830D0830: 4800CCA9  bl 0x830dd4d8
	ctx.lr = 0x830D0834;
	sub_830DD4D8(ctx, base);
	// 830D0834: 807F0064  lwz r3, 0x64(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(100 as u32) ) } as u64;
	// 830D0838: 4800CCA1  bl 0x830dd4d8
	ctx.lr = 0x830D083C;
	sub_830DD4D8(ctx, base);
	// 830D083C: 807F0068  lwz r3, 0x68(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) } as u64;
	// 830D0840: 4800CC99  bl 0x830dd4d8
	ctx.lr = 0x830D0844;
	sub_830DD4D8(ctx, base);
	// 830D0844: 807F0078  lwz r3, 0x78(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(120 as u32) ) } as u64;
	// 830D0848: 4800CC91  bl 0x830dd4d8
	ctx.lr = 0x830D084C;
	sub_830DD4D8(ctx, base);
	// 830D084C: 807F004C  lwz r3, 0x4c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 830D0850: 4800CC89  bl 0x830dd4d8
	ctx.lr = 0x830D0854;
	sub_830DD4D8(ctx, base);
	// 830D0854: 387F0098  addi r3, r31, 0x98
	ctx.r[3].s64 = ctx.r[31].s64 + 152;
	// 830D0858: 4BFFCC89  bl 0x830cd4e0
	ctx.lr = 0x830D085C;
	sub_830CD4E0(ctx, base);
	// 830D085C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830D0860: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830D0864: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830D0868: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830D086C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830D0870(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830D0870 size=84
    let mut pc: u32 = 0x830D0870;
    'dispatch: loop {
        match pc {
            0x830D0870 => {
    //   block [0x830D0870..0x830D08C4)
	// 830D0870: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830D0874: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830D0878: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830D087C: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 830D0880: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 830D0884: 4BFFDBED  bl 0x830ce470
	ctx.lr = 0x830D0888;
	sub_830CE470(ctx, base);
	// 830D0888: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 830D088C: 41800028  blt 0x830d08b4
	if ctx.cr[0].lt {
	pc = 0x830D08B4; continue 'dispatch;
	}
	// 830D0890: 81650000  lwz r11, 0(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 830D0894: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 830D0898: 419A0010  beq cr6, 0x830d08a8
	if ctx.cr[6].eq {
	pc = 0x830D08A8; continue 'dispatch;
	}
	// 830D089C: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 830D08A0: 60630057  ori r3, r3, 0x57
	ctx.r[3].u64 = ctx.r[3].u64 | 87;
	// 830D08A4: 48000010  b 0x830d08b4
	pc = 0x830D08B4; continue 'dispatch;
	// 830D08A8: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 830D08AC: 80850004  lwz r4, 4(r5)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) } as u64;
	// 830D08B0: 48027381  bl 0x830f7c30
	ctx.lr = 0x830D08B4;
	sub_830F7C30(ctx, base);
	// 830D08B4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830D08B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830D08BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830D08C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830D08C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830D08C8 size=84
    let mut pc: u32 = 0x830D08C8;
    'dispatch: loop {
        match pc {
            0x830D08C8 => {
    //   block [0x830D08C8..0x830D091C)
	// 830D08C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830D08CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830D08D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830D08D4: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 830D08D8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 830D08DC: 4BFFDB95  bl 0x830ce470
	ctx.lr = 0x830D08E0;
	sub_830CE470(ctx, base);
	// 830D08E0: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 830D08E4: 41800028  blt 0x830d090c
	if ctx.cr[0].lt {
	pc = 0x830D090C; continue 'dispatch;
	}
	// 830D08E8: 81650000  lwz r11, 0(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 830D08EC: 2F0B0005  cmpwi cr6, r11, 5
	ctx.cr[6].compare_i32(ctx.r[11].s32, 5, &mut ctx.xer);
	// 830D08F0: 419A0010  beq cr6, 0x830d0900
	if ctx.cr[6].eq {
	pc = 0x830D0900; continue 'dispatch;
	}
	// 830D08F4: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 830D08F8: 60630057  ori r3, r3, 0x57
	ctx.r[3].u64 = ctx.r[3].u64 | 87;
	// 830D08FC: 48000010  b 0x830d090c
	pc = 0x830D090C; continue 'dispatch;
	// 830D0900: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 830D0904: 80850004  lwz r4, 4(r5)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) } as u64;
	// 830D0908: 48015899  bl 0x830e61a0
	ctx.lr = 0x830D090C;
	sub_830E61A0(ctx, base);
	// 830D090C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830D0910: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830D0914: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830D0918: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830D0920(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830D0920 size=84
    let mut pc: u32 = 0x830D0920;
    'dispatch: loop {
        match pc {
            0x830D0920 => {
    //   block [0x830D0920..0x830D0974)
	// 830D0920: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830D0924: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830D0928: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830D092C: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 830D0930: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 830D0934: 4BFFDB3D  bl 0x830ce470
	ctx.lr = 0x830D0938;
	sub_830CE470(ctx, base);
	// 830D0938: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 830D093C: 41800028  blt 0x830d0964
	if ctx.cr[0].lt {
	pc = 0x830D0964; continue 'dispatch;
	}
	// 830D0940: 81650000  lwz r11, 0(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 830D0944: 2F0B0005  cmpwi cr6, r11, 5
	ctx.cr[6].compare_i32(ctx.r[11].s32, 5, &mut ctx.xer);
	// 830D0948: 419A0010  beq cr6, 0x830d0958
	if ctx.cr[6].eq {
	pc = 0x830D0958; continue 'dispatch;
	}
	// 830D094C: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 830D0950: 60630057  ori r3, r3, 0x57
	ctx.r[3].u64 = ctx.r[3].u64 | 87;
	// 830D0954: 48000010  b 0x830d0964
	pc = 0x830D0964; continue 'dispatch;
	// 830D0958: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 830D095C: 80850004  lwz r4, 4(r5)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) } as u64;
	// 830D0960: 48027261  bl 0x830f7bc0
	ctx.lr = 0x830D0964;
	sub_830F7BC0(ctx, base);
	// 830D0964: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830D0968: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830D096C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830D0970: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830D0978(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830D0978 size=84
    let mut pc: u32 = 0x830D0978;
    'dispatch: loop {
        match pc {
            0x830D0978 => {
    //   block [0x830D0978..0x830D09CC)
	// 830D0978: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830D097C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830D0980: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830D0984: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 830D0988: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 830D098C: 4BFFDAE5  bl 0x830ce470
	ctx.lr = 0x830D0990;
	sub_830CE470(ctx, base);
	// 830D0990: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 830D0994: 41800028  blt 0x830d09bc
	if ctx.cr[0].lt {
	pc = 0x830D09BC; continue 'dispatch;
	}
	// 830D0998: 81650000  lwz r11, 0(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 830D099C: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 830D09A0: 419A0010  beq cr6, 0x830d09b0
	if ctx.cr[6].eq {
	pc = 0x830D09B0; continue 'dispatch;
	}
	// 830D09A4: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 830D09A8: 60630057  ori r3, r3, 0x57
	ctx.r[3].u64 = ctx.r[3].u64 | 87;
	// 830D09AC: 48000010  b 0x830d09bc
	pc = 0x830D09BC; continue 'dispatch;
	// 830D09B0: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 830D09B4: 80850004  lwz r4, 4(r5)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) } as u64;
	// 830D09B8: 4801D371  bl 0x830edd28
	ctx.lr = 0x830D09BC;
	sub_830EDD28(ctx, base);
	// 830D09BC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830D09C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830D09C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830D09C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830D09D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830D09D0 size=84
    let mut pc: u32 = 0x830D09D0;
    'dispatch: loop {
        match pc {
            0x830D09D0 => {
    //   block [0x830D09D0..0x830D0A24)
	// 830D09D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830D09D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830D09D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830D09DC: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 830D09E0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 830D09E4: 4BFFDA8D  bl 0x830ce470
	ctx.lr = 0x830D09E8;
	sub_830CE470(ctx, base);
	// 830D09E8: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 830D09EC: 41800028  blt 0x830d0a14
	if ctx.cr[0].lt {
	pc = 0x830D0A14; continue 'dispatch;
	}
	// 830D09F0: 81650000  lwz r11, 0(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 830D09F4: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 830D09F8: 419A0010  beq cr6, 0x830d0a08
	if ctx.cr[6].eq {
	pc = 0x830D0A08; continue 'dispatch;
	}
	// 830D09FC: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 830D0A00: 60630057  ori r3, r3, 0x57
	ctx.r[3].u64 = ctx.r[3].u64 | 87;
	// 830D0A04: 48000010  b 0x830d0a14
	pc = 0x830D0A14; continue 'dispatch;
	// 830D0A08: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 830D0A0C: 80850004  lwz r4, 4(r5)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) } as u64;
	// 830D0A10: 48027091  bl 0x830f7aa0
	ctx.lr = 0x830D0A14;
	sub_830F7AA0(ctx, base);
	// 830D0A14: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830D0A18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830D0A1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830D0A20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830D0A28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830D0A28 size=84
    let mut pc: u32 = 0x830D0A28;
    'dispatch: loop {
        match pc {
            0x830D0A28 => {
    //   block [0x830D0A28..0x830D0A7C)
	// 830D0A28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830D0A2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830D0A30: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830D0A34: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 830D0A38: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 830D0A3C: 4BFFDA35  bl 0x830ce470
	ctx.lr = 0x830D0A40;
	sub_830CE470(ctx, base);
	// 830D0A40: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 830D0A44: 41800028  blt 0x830d0a6c
	if ctx.cr[0].lt {
	pc = 0x830D0A6C; continue 'dispatch;
	}
	// 830D0A48: 81650000  lwz r11, 0(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 830D0A4C: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 830D0A50: 419A0010  beq cr6, 0x830d0a60
	if ctx.cr[6].eq {
	pc = 0x830D0A60; continue 'dispatch;
	}
	// 830D0A54: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 830D0A58: 60630057  ori r3, r3, 0x57
	ctx.r[3].u64 = ctx.r[3].u64 | 87;
	// 830D0A5C: 48000010  b 0x830d0a6c
	pc = 0x830D0A6C; continue 'dispatch;
	// 830D0A60: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 830D0A64: 80850004  lwz r4, 4(r5)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) } as u64;
	// 830D0A68: 480270F1  bl 0x830f7b58
	ctx.lr = 0x830D0A6C;
	sub_830F7B58(ctx, base);
	// 830D0A6C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830D0A70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830D0A74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830D0A78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830D0A80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830D0A80 size=332
    let mut pc: u32 = 0x830D0A80;
    'dispatch: loop {
        match pc {
            0x830D0A80 => {
    //   block [0x830D0A80..0x830D0BCC)
	// 830D0A80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830D0A84: 480D76E9  bl 0x831a816c
	ctx.lr = 0x830D0A88;
	sub_831A8130(ctx, base);
	// 830D0A88: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830D0A8C: 3D008339  lis r8, -0x7cc7
	ctx.r[8].s64 = -2093416448;
	// 830D0A90: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830D0A94: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 830D0A98: 3BEBC4F0  addi r31, r11, -0x3b10
	ctx.r[31].s64 = ctx.r[11].s64 + -15120;
	// 830D0A9C: 8168C580  lwz r11, -0x3a80(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(-14976 as u32) ) } as u64;
	// 830D0AA0: 556A07FF  clrlwi. r10, r11, 0x1f
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 830D0AA4: 40820114  bne 0x830d0bb8
	if !ctx.cr[0].eq {
	pc = 0x830D0BB8; continue 'dispatch;
	}
	// 830D0AA8: 3D408219  lis r10, -0x7de7
	ctx.r[10].s64 = -2112290816;
	// 830D0AAC: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 830D0AB0: 392A807C  addi r9, r10, -0x7f84
	ctx.r[9].s64 = ctx.r[10].s64 + -32644;
	// 830D0AB4: 616B0001  ori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 | 1;
	// 830D0AB8: 7FCAF378  mr r10, r30
	ctx.r[10].u64 = ctx.r[30].u64;
	// 830D0ABC: 913F0010  stw r9, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[9].u32 ) };
	// 830D0AC0: 9168C580  stw r11, -0x3a80(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(-14976 as u32), ctx.r[11].u32 ) };
	// 830D0AC4: 39600008  li r11, 8
	ctx.r[11].s64 = 8;
	// 830D0AC8: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 830D0ACC: 7FC9F378  mr r9, r30
	ctx.r[9].u64 = ctx.r[30].u64;
	// 830D0AD0: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 830D0AD4: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 830D0AD8: 913F000C  stw r9, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 830D0ADC: 915F0014  stw r10, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 830D0AE0: 480123F9  bl 0x830e2ed8
	ctx.lr = 0x830D0AE4;
	sub_830E2ED8(ctx, base);
	// 830D0AE4: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 830D0AE8: 3D20830D  lis r9, -0x7cf3
	ctx.r[9].s64 = -2096300032;
	// 830D0AEC: 907F0018  stw r3, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[3].u32 ) };
	// 830D0AF0: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 830D0AF4: 7FCAF378  mr r10, r30
	ctx.r[10].u64 = ctx.r[30].u64;
	// 830D0AF8: 39290BD0  addi r9, r9, 0xbd0
	ctx.r[9].s64 = ctx.r[9].s64 + 3024;
	// 830D0AFC: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 830D0B00: 915F0020  stw r10, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 830D0B04: 913F0024  stw r9, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[9].u32 ) };
	// 830D0B08: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 830D0B0C: 392B805C  addi r9, r11, -0x7fa4
	ctx.r[9].s64 = ctx.r[11].s64 + -32676;
	// 830D0B10: FBDF0028  std r30, 0x28(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[30].u64 ) };
	// 830D0B14: 915F0034  stw r10, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[10].u32 ) };
	// 830D0B18: 913F0040  stw r9, 0x40(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), ctx.r[9].u32 ) };
	// 830D0B1C: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 830D0B20: 39200090  li r9, 0x90
	ctx.r[9].s64 = 144;
	// 830D0B24: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 830D0B28: 917F0030  stw r11, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 830D0B2C: 917F003C  stw r11, 0x3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[11].u32 ) };
	// 830D0B30: 397F0028  addi r11, r31, 0x28
	ctx.r[11].s64 = ctx.r[31].s64 + 40;
	// 830D0B34: 913F0038  stw r9, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[9].u32 ) };
	// 830D0B38: 915F0044  stw r10, 0x44(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(68 as u32), ctx.r[10].u32 ) };
	// 830D0B3C: 4801239D  bl 0x830e2ed8
	ctx.lr = 0x830D0B40;
	sub_830E2ED8(ctx, base);
	// 830D0B40: 907F0048  stw r3, 0x48(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[3].u32 ) };
	// 830D0B44: 93DF004C  stw r30, 0x4c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(76 as u32), ctx.r[30].u32 ) };
	// 830D0B48: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 830D0B4C: 93DF0050  stw r30, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 830D0B50: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 830D0B54: 93DF0054  stw r30, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[30].u32 ) };
	// 830D0B58: 392B8044  addi r9, r11, -0x7fbc
	ctx.r[9].s64 = ctx.r[11].s64 + -32700;
	// 830D0B5C: 93DF0058  stw r30, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[30].u32 ) };
	// 830D0B60: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 830D0B64: 93DF005C  stw r30, 0x5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[30].u32 ) };
	// 830D0B68: 913F0070  stw r9, 0x70(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(112 as u32), ctx.r[9].u32 ) };
	// 830D0B6C: 39200084  li r9, 0x84
	ctx.r[9].s64 = 132;
	// 830D0B70: 915F0064  stw r10, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[10].u32 ) };
	// 830D0B74: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 830D0B78: 917F0060  stw r11, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 830D0B7C: 917F006C  stw r11, 0x6c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 830D0B80: 397F004C  addi r11, r31, 0x4c
	ctx.r[11].s64 = ctx.r[31].s64 + 76;
	// 830D0B84: 913F0068  stw r9, 0x68(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(104 as u32), ctx.r[9].u32 ) };
	// 830D0B88: 915F0074  stw r10, 0x74(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(116 as u32), ctx.r[10].u32 ) };
	// 830D0B8C: 4801234D  bl 0x830e2ed8
	ctx.lr = 0x830D0B90;
	sub_830E2ED8(ctx, base);
	// 830D0B90: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 830D0B94: 3D20830D  lis r9, -0x7cf3
	ctx.r[9].s64 = -2096300032;
	// 830D0B98: 907F0078  stw r3, 0x78(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(120 as u32), ctx.r[3].u32 ) };
	// 830D0B9C: 917F007C  stw r11, 0x7c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(124 as u32), ctx.r[11].u32 ) };
	// 830D0BA0: 397F0088  addi r11, r31, 0x88
	ctx.r[11].s64 = ctx.r[31].s64 + 136;
	// 830D0BA4: 7FCAF378  mr r10, r30
	ctx.r[10].u64 = ctx.r[30].u64;
	// 830D0BA8: 39690C38  addi r11, r9, 0xc38
	ctx.r[11].s64 = ctx.r[9].s64 + 3128;
	// 830D0BAC: 915F0080  stw r10, 0x80(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(128 as u32), ctx.r[10].u32 ) };
	// 830D0BB0: 917F0084  stw r11, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[11].u32 ) };
	// 830D0BB4: FBDF0088  std r30, 0x88(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(136 as u32), ctx.r[30].u64 ) };
	// 830D0BB8: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 830D0BBC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830D0BC0: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 830D0BC4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830D0BC8: 480D75F4  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830D0BD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830D0BD0 size=104
    let mut pc: u32 = 0x830D0BD0;
    'dispatch: loop {
        match pc {
            0x830D0BD0 => {
    //   block [0x830D0BD0..0x830D0C38)
	// 830D0BD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830D0BD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830D0BD8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830D0BDC: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 830D0BE0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 830D0BE4: 4BFFD88D  bl 0x830ce470
	ctx.lr = 0x830D0BE8;
	sub_830CE470(ctx, base);
	// 830D0BE8: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 830D0BEC: 4180003C  blt 0x830d0c28
	if ctx.cr[0].lt {
	pc = 0x830D0C28; continue 'dispatch;
	}
	// 830D0BF0: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 830D0BF4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830D0BF8: 419A0028  beq cr6, 0x830d0c20
	if ctx.cr[6].eq {
	pc = 0x830D0C20; continue 'dispatch;
	}
	// 830D0BFC: 81450000  lwz r10, 0(r5)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 830D0C00: 2F0A0003  cmpwi cr6, r10, 3
	ctx.cr[6].compare_i32(ctx.r[10].s32, 3, &mut ctx.xer);
	// 830D0C04: 409A001C  bne cr6, 0x830d0c20
	if !ctx.cr[6].eq {
	pc = 0x830D0C20; continue 'dispatch;
	}
	// 830D0C08: 81450004  lwz r10, 4(r5)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) } as u64;
	// 830D0C0C: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 830D0C10: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830D0C14: 992B000C  stb r9, 0xc(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[9].u8 ) };
	// 830D0C18: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 830D0C1C: 4800000C  b 0x830d0c28
	pc = 0x830D0C28; continue 'dispatch;
	// 830D0C20: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 830D0C24: 60630057  ori r3, r3, 0x57
	ctx.r[3].u64 = ctx.r[3].u64 | 87;
	// 830D0C28: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830D0C2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830D0C30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830D0C34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830D0C38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830D0C38 size=88
    let mut pc: u32 = 0x830D0C38;
    'dispatch: loop {
        match pc {
            0x830D0C38 => {
    //   block [0x830D0C38..0x830D0C90)
	// 830D0C38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830D0C3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830D0C40: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830D0C44: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 830D0C48: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 830D0C4C: 4BFFD825  bl 0x830ce470
	ctx.lr = 0x830D0C50;
	sub_830CE470(ctx, base);
	// 830D0C50: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 830D0C54: 4180002C  blt 0x830d0c80
	if ctx.cr[0].lt {
	pc = 0x830D0C80; continue 'dispatch;
	}
	// 830D0C58: 81650000  lwz r11, 0(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 830D0C5C: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 830D0C60: 419A0010  beq cr6, 0x830d0c70
	if ctx.cr[6].eq {
	pc = 0x830D0C70; continue 'dispatch;
	}
	// 830D0C64: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 830D0C68: 60630057  ori r3, r3, 0x57
	ctx.r[3].u64 = ctx.r[3].u64 | 87;
	// 830D0C6C: 48000014  b 0x830d0c80
	pc = 0x830D0C80; continue 'dispatch;
	// 830D0C70: 81650004  lwz r11, 4(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) } as u64;
	// 830D0C74: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830D0C78: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 830D0C7C: 916A0084  stw r11, 0x84(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(132 as u32), ctx.r[11].u32 ) };
	// 830D0C80: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830D0C84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830D0C88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830D0C8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830D0C90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830D0C90 size=120
    let mut pc: u32 = 0x830D0C90;
    'dispatch: loop {
        match pc {
            0x830D0C90 => {
    //   block [0x830D0C90..0x830D0D08)
	// 830D0C90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830D0C94: 480D74D9  bl 0x831a816c
	ctx.lr = 0x830D0C98;
	sub_831A8130(ctx, base);
	// 830D0C98: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830D0C9C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 830D0CA0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 830D0CA4: 3BDD0004  addi r30, r29, 4
	ctx.r[30].s64 = ctx.r[29].s64 + 4;
	// 830D0CA8: 807D0004  lwz r3, 4(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 830D0CAC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 830D0CB0: 409A001C  bne cr6, 0x830d0ccc
	if !ctx.cr[6].eq {
	pc = 0x830D0CCC; continue 'dispatch;
	}
	// 830D0CB4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 830D0CB8: 409A000C  bne cr6, 0x830d0cc4
	if !ctx.cr[6].eq {
	pc = 0x830D0CC4; continue 'dispatch;
	}
	// 830D0CBC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830D0CC0: 48000040  b 0x830d0d00
	pc = 0x830D0D00; continue 'dispatch;
	// 830D0CC4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 830D0CC8: 419A001C  beq cr6, 0x830d0ce4
	if ctx.cr[6].eq {
	pc = 0x830D0CE4; continue 'dispatch;
	}
	// 830D0CCC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 830D0CD0: 419A0014  beq cr6, 0x830d0ce4
	if ctx.cr[6].eq {
	pc = 0x830D0CE4; continue 'dispatch;
	}
	// 830D0CD4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 830D0CD8: 480E1159  bl 0x831b1e30
	ctx.lr = 0x830D0CDC;
	sub_831B1E30(ctx, base);
	// 830D0CDC: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 830D0CE0: 4182FFDC  beq 0x830d0cbc
	if ctx.cr[0].eq {
	pc = 0x830D0CBC; continue 'dispatch;
	}
	// 830D0CE4: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 830D0CE8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830D0CEC: 997D000C  stb r11, 0xc(r29)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[29].u32.wrapping_add(12 as u32), ctx.r[11].u8 ) };
	// 830D0CF0: 48015659  bl 0x830e6348
	ctx.lr = 0x830D0CF4;
	sub_830E6348(ctx, base);
	// 830D0CF4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 830D0CF8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830D0CFC: 4BFFDF8D  bl 0x830cec88
	ctx.lr = 0x830D0D00;
	sub_830CEC88(ctx, base);
	// 830D0D00: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830D0D04: 480D74B8  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830D0D08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830D0D08 size=116
    let mut pc: u32 = 0x830D0D08;
    'dispatch: loop {
        match pc {
            0x830D0D08 => {
    //   block [0x830D0D08..0x830D0D7C)
	// 830D0D08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830D0D0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830D0D10: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830D0D14: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 830D0D18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830D0D1C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 830D0D20: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 830D0D24: 4BFFD74D  bl 0x830ce470
	ctx.lr = 0x830D0D28;
	sub_830CE470(ctx, base);
	// 830D0D28: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 830D0D2C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 830D0D30: 419A0034  beq cr6, 0x830d0d64
	if ctx.cr[6].eq {
	pc = 0x830D0D64; continue 'dispatch;
	}
	// 830D0D34: 81650000  lwz r11, 0(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 830D0D38: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 830D0D3C: 409A0028  bne cr6, 0x830d0d64
	if !ctx.cr[6].eq {
	pc = 0x830D0D64; continue 'dispatch;
	}
	// 830D0D40: 81650004  lwz r11, 4(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) } as u64;
	// 830D0D44: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 830D0D48: 4198001C  blt cr6, 0x830d0d64
	if ctx.cr[6].lt {
	pc = 0x830D0D64; continue 'dispatch;
	}
	// 830D0D4C: 812A0008  lwz r9, 8(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 830D0D50: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 830D0D54: 41990010  bgt cr6, 0x830d0d64
	if ctx.cr[6].gt {
	pc = 0x830D0D64; continue 'dispatch;
	}
	// 830D0D58: 916A0014  stw r11, 0x14(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 830D0D5C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830D0D60: 4800000C  b 0x830d0d6c
	pc = 0x830D0D6C; continue 'dispatch;
	// 830D0D64: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 830D0D68: 60630057  ori r3, r3, 0x57
	ctx.r[3].u64 = ctx.r[3].u64 | 87;
	// 830D0D6C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830D0D70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830D0D74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830D0D78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830D0D80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830D0D80 size=108
    let mut pc: u32 = 0x830D0D80;
    'dispatch: loop {
        match pc {
            0x830D0D80 => {
    //   block [0x830D0D80..0x830D0DEC)
	// 830D0D80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830D0D84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830D0D88: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830D0D8C: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 830D0D90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830D0D94: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 830D0D98: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 830D0D9C: 4BFFD6D5  bl 0x830ce470
	ctx.lr = 0x830D0DA0;
	sub_830CE470(ctx, base);
	// 830D0DA0: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 830D0DA4: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 830D0DA8: 419A002C  beq cr6, 0x830d0dd4
	if ctx.cr[6].eq {
	pc = 0x830D0DD4; continue 'dispatch;
	}
	// 830D0DAC: 81650000  lwz r11, 0(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 830D0DB0: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 830D0DB4: 409A0020  bne cr6, 0x830d0dd4
	if !ctx.cr[6].eq {
	pc = 0x830D0DD4; continue 'dispatch;
	}
	// 830D0DB8: 81650004  lwz r11, 4(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) } as u64;
	// 830D0DBC: 812A0014  lwz r9, 0x14(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 830D0DC0: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 830D0DC4: 41980010  blt cr6, 0x830d0dd4
	if ctx.cr[6].lt {
	pc = 0x830D0DD4; continue 'dispatch;
	}
	// 830D0DC8: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 830D0DCC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830D0DD0: 4800000C  b 0x830d0ddc
	pc = 0x830D0DDC; continue 'dispatch;
	// 830D0DD4: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 830D0DD8: 60630057  ori r3, r3, 0x57
	ctx.r[3].u64 = ctx.r[3].u64 | 87;
	// 830D0DDC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830D0DE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830D0DE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830D0DE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830D0DF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830D0DF0 size=108
    let mut pc: u32 = 0x830D0DF0;
    'dispatch: loop {
        match pc {
            0x830D0DF0 => {
    //   block [0x830D0DF0..0x830D0E5C)
	// 830D0DF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830D0DF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830D0DF8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830D0DFC: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 830D0E00: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 830D0E04: 4BFFD66D  bl 0x830ce470
	ctx.lr = 0x830D0E08;
	sub_830CE470(ctx, base);
	// 830D0E08: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 830D0E0C: 41800040  blt 0x830d0e4c
	if ctx.cr[0].lt {
	pc = 0x830D0E4C; continue 'dispatch;
	}
	// 830D0E10: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 830D0E14: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830D0E18: 419A002C  beq cr6, 0x830d0e44
	if ctx.cr[6].eq {
	pc = 0x830D0E44; continue 'dispatch;
	}
	// 830D0E1C: 81450000  lwz r10, 0(r5)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 830D0E20: 2F0A0005  cmpwi cr6, r10, 5
	ctx.cr[6].compare_i32(ctx.r[10].s32, 5, &mut ctx.xer);
	// 830D0E24: 409A0020  bne cr6, 0x830d0e44
	if !ctx.cr[6].eq {
	pc = 0x830D0E44; continue 'dispatch;
	}
	// 830D0E28: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830D0E2C: 386B0008  addi r3, r11, 8
	ctx.r[3].s64 = ctx.r[11].s64 + 8;
	// 830D0E30: 80850004  lwz r4, 4(r5)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) } as u64;
	// 830D0E34: 614A0001  ori r10, r10, 1
	ctx.r[10].u64 = ctx.r[10].u64 | 1;
	// 830D0E38: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 830D0E3C: 4BFFDE4D  bl 0x830cec88
	ctx.lr = 0x830D0E40;
	sub_830CEC88(ctx, base);
	// 830D0E40: 4800000C  b 0x830d0e4c
	pc = 0x830D0E4C; continue 'dispatch;
	// 830D0E44: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 830D0E48: 60630057  ori r3, r3, 0x57
	ctx.r[3].u64 = ctx.r[3].u64 | 87;
	// 830D0E4C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830D0E50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830D0E54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830D0E58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830D0E60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830D0E60 size=112
    let mut pc: u32 = 0x830D0E60;
    'dispatch: loop {
        match pc {
            0x830D0E60 => {
    //   block [0x830D0E60..0x830D0ED0)
	// 830D0E60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830D0E64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830D0E68: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830D0E6C: 7C671B78  mr r7, r3
	ctx.r[7].u64 = ctx.r[3].u64;
	// 830D0E70: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 830D0E74: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 830D0E78: 4BFFD5F9  bl 0x830ce470
	ctx.lr = 0x830D0E7C;
	sub_830CE470(ctx, base);
	// 830D0E7C: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 830D0E80: 41800040  blt 0x830d0ec0
	if ctx.cr[0].lt {
	pc = 0x830D0EC0; continue 'dispatch;
	}
	// 830D0E84: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 830D0E88: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830D0E8C: 419A002C  beq cr6, 0x830d0eb8
	if ctx.cr[6].eq {
	pc = 0x830D0EB8; continue 'dispatch;
	}
	// 830D0E90: 81450000  lwz r10, 0(r5)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 830D0E94: 2F0A0003  cmpwi cr6, r10, 3
	ctx.cr[6].compare_i32(ctx.r[10].s32, 3, &mut ctx.xer);
	// 830D0E98: 409A0020  bne cr6, 0x830d0eb8
	if !ctx.cr[6].eq {
	pc = 0x830D0EB8; continue 'dispatch;
	}
	// 830D0E9C: 81450004  lwz r10, 4(r5)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) } as u64;
	// 830D0EA0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830D0EA4: 7D4B392E  stwx r10, r11, r7
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[7].u32), ctx.r[10].u32) };
	// 830D0EA8: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830D0EAC: 614A0002  ori r10, r10, 2
	ctx.r[10].u64 = ctx.r[10].u64 | 2;
	// 830D0EB0: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 830D0EB4: 4800000C  b 0x830d0ec0
	pc = 0x830D0EC0; continue 'dispatch;
	// 830D0EB8: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 830D0EBC: 60630057  ori r3, r3, 0x57
	ctx.r[3].u64 = ctx.r[3].u64 | 87;
	// 830D0EC0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830D0EC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830D0EC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830D0ECC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830D0ED0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830D0ED0 size=8
    let mut pc: u32 = 0x830D0ED0;
    'dispatch: loop {
        match pc {
            0x830D0ED0 => {
    //   block [0x830D0ED0..0x830D0ED8)
	// 830D0ED0: 3860000C  li r3, 0xc
	ctx.r[3].s64 = 12;
	// 830D0ED4: 4BFFFF8C  b 0x830d0e60
	sub_830D0E60(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830D0ED8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830D0ED8 size=8
    let mut pc: u32 = 0x830D0ED8;
    'dispatch: loop {
        match pc {
            0x830D0ED8 => {
    //   block [0x830D0ED8..0x830D0EE0)
	// 830D0ED8: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 830D0EDC: 4BFFFF84  b 0x830d0e60
	sub_830D0E60(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830D0EE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830D0EE0 size=8
    let mut pc: u32 = 0x830D0EE0;
    'dispatch: loop {
        match pc {
            0x830D0EE0 => {
    //   block [0x830D0EE0..0x830D0EE8)
	// 830D0EE0: 38600014  li r3, 0x14
	ctx.r[3].s64 = 20;
	// 830D0EE4: 4BFFFF7C  b 0x830d0e60
	sub_830D0E60(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830D0EE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x830D0EE8 size=8
    let mut pc: u32 = 0x830D0EE8;
    'dispatch: loop {
        match pc {
            0x830D0EE8 => {
    //   block [0x830D0EE8..0x830D0EF0)
	// 830D0EE8: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 830D0EEC: 4BFFFF74  b 0x830d0e60
	sub_830D0E60(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830D0EF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830D0EF0 size=108
    let mut pc: u32 = 0x830D0EF0;
    'dispatch: loop {
        match pc {
            0x830D0EF0 => {
    //   block [0x830D0EF0..0x830D0F5C)
	// 830D0EF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830D0EF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830D0EF8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830D0EFC: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 830D0F00: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 830D0F04: 4BFFD56D  bl 0x830ce470
	ctx.lr = 0x830D0F08;
	sub_830CE470(ctx, base);
	// 830D0F08: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 830D0F0C: 41800040  blt 0x830d0f4c
	if ctx.cr[0].lt {
	pc = 0x830D0F4C; continue 'dispatch;
	}
	// 830D0F10: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 830D0F14: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830D0F18: 419A002C  beq cr6, 0x830d0f44
	if ctx.cr[6].eq {
	pc = 0x830D0F44; continue 'dispatch;
	}
	// 830D0F1C: 81450000  lwz r10, 0(r5)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 830D0F20: 2F0A0001  cmpwi cr6, r10, 1
	ctx.cr[6].compare_i32(ctx.r[10].s32, 1, &mut ctx.xer);
	// 830D0F24: 409A0020  bne cr6, 0x830d0f44
	if !ctx.cr[6].eq {
	pc = 0x830D0F44; continue 'dispatch;
	}
	// 830D0F28: 81450004  lwz r10, 4(r5)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) } as u64;
	// 830D0F2C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830D0F30: 914B0020  stw r10, 0x20(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 830D0F34: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 830D0F38: 614A0002  ori r10, r10, 2
	ctx.r[10].u64 = ctx.r[10].u64 | 2;
	// 830D0F3C: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 830D0F40: 4800000C  b 0x830d0f4c
	pc = 0x830D0F4C; continue 'dispatch;
	// 830D0F44: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 830D0F48: 60630057  ori r3, r3, 0x57
	ctx.r[3].u64 = ctx.r[3].u64 | 87;
	// 830D0F4C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830D0F50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830D0F54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830D0F58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830D0F60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830D0F60 size=144
    let mut pc: u32 = 0x830D0F60;
    'dispatch: loop {
        match pc {
            0x830D0F60 => {
    //   block [0x830D0F60..0x830D0FF0)
	// 830D0F60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830D0F64: 480D7209  bl 0x831a816c
	ctx.lr = 0x830D0F68;
	sub_831A8130(ctx, base);
	// 830D0F68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830D0F6C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 830D0F70: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 830D0F74: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830D0F78: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 830D0F7C: 4BFFD4F5  bl 0x830ce470
	ctx.lr = 0x830D0F80;
	sub_830CE470(ctx, base);
	// 830D0F80: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 830D0F84: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830D0F88: 419A0058  beq cr6, 0x830d0fe0
	if ctx.cr[6].eq {
	pc = 0x830D0FE0; continue 'dispatch;
	}
	// 830D0F8C: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 830D0F90: 2F0A0005  cmpwi cr6, r10, 5
	ctx.cr[6].compare_i32(ctx.r[10].s32, 5, &mut ctx.xer);
	// 830D0F94: 409A004C  bne cr6, 0x830d0fe0
	if !ctx.cr[6].eq {
	pc = 0x830D0FE0; continue 'dispatch;
	}
	// 830D0F98: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 830D0F9C: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830D0FA0: 4BFFDCE9  bl 0x830cec88
	ctx.lr = 0x830D0FA4;
	sub_830CEC88(ctx, base);
	// 830D0FA4: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 830D0FA8: 4080000C  bge 0x830d0fb4
	if !ctx.cr[0].lt {
	pc = 0x830D0FB4; continue 'dispatch;
	}
	// 830D0FAC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830D0FB0: 48000038  b 0x830d0fe8
	pc = 0x830D0FE8; continue 'dispatch;
	// 830D0FB4: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 830D0FB8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 830D0FBC: 808BC06C  lwz r4, -0x3f94(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16276 as u32) ) } as u64;
	// 830D0FC0: 4800DB29  bl 0x830deae8
	ctx.lr = 0x830D0FC4;
	sub_830DEAE8(ctx, base);
	// 830D0FC4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830D0FC8: 4182FFE4  beq 0x830d0fac
	if ctx.cr[0].eq {
	pc = 0x830D0FAC; continue 'dispatch;
	}
	// 830D0FCC: 38630004  addi r3, r3, 4
	ctx.r[3].s64 = ctx.r[3].s64 + 4;
	// 830D0FD0: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830D0FD4: 4BFFDCB5  bl 0x830cec88
	ctx.lr = 0x830D0FD8;
	sub_830CEC88(ctx, base);
	// 830D0FD8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830D0FDC: 4BFFFFD0  b 0x830d0fac
	pc = 0x830D0FAC; continue 'dispatch;
	// 830D0FE0: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 830D0FE4: 60630057  ori r3, r3, 0x57
	ctx.r[3].u64 = ctx.r[3].u64 | 87;
	// 830D0FE8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 830D0FEC: 480D71D0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830D0FF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830D0FF0 size=96
    let mut pc: u32 = 0x830D0FF0;
    'dispatch: loop {
        match pc {
            0x830D0FF0 => {
    //   block [0x830D0FF0..0x830D1050)
	// 830D0FF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830D0FF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830D0FF8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830D0FFC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830D1000: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830D1004: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 830D1008: 3BDF0004  addi r30, r31, 4
	ctx.r[30].s64 = ctx.r[31].s64 + 4;
	// 830D100C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 830D1010: 48045D01  bl 0x83116d10
	ctx.lr = 0x830D1014;
	sub_83116D10(ctx, base);
	// 830D1014: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 830D1018: 281E0000  cmplwi r30, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 830D101C: 917F00F4  stw r11, 0xf4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(244 as u32), ctx.r[11].u32 ) };
	// 830D1020: 917F00F8  stw r11, 0xf8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(248 as u32), ctx.r[11].u32 ) };
	// 830D1024: 41820010  beq 0x830d1034
	if ctx.cr[0].eq {
	pc = 0x830D1034; continue 'dispatch;
	}
	// 830D1028: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 830D102C: 616B0001  ori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 | 1;
	// 830D1030: 917E0008  stw r11, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 830D1034: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 830D1038: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830D103C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830D1040: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830D1044: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830D1048: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830D104C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830D1050(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x830D1050 size=124
    let mut pc: u32 = 0x830D1050;
    'dispatch: loop {
        match pc {
            0x830D1050 => {
    //   block [0x830D1050..0x830D10CC)
	// 830D1050: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830D1054: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830D1058: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 830D105C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 830D1060: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830D1064: 7C872378  mr r7, r4
	ctx.r[7].u64 = ctx.r[4].u64;
	// 830D1068: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 830D106C: 7CE33B78  mr r3, r7
	ctx.r[3].u64 = ctx.r[7].u64;
	// 830D1070: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 830D1074: 4BFFD3FD  bl 0x830ce470
	ctx.lr = 0x830D1078;
	sub_830CE470(ctx, base);
	// 830D1078: 83E10050  lwz r31, 0x50(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 830D107C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 830D1080: 419A002C  beq cr6, 0x830d10ac
	if ctx.cr[6].eq {
	pc = 0x830D10AC; continue 'dispatch;
	}
	// 830D1084: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 830D1088: 2F0B0005  cmpwi cr6, r11, 5
	ctx.cr[6].compare_i32(ctx.r[11].s32, 5, &mut ctx.xer);
	// 830D108C: 409A0020  bne cr6, 0x830d10ac
	if !ctx.cr[6].eq {
	pc = 0x830D10AC; continue 'dispatch;
	}
	// 830D1090: 389F0004  addi r4, r31, 4
	ctx.r[4].s64 = ctx.r[31].s64 + 4;
	// 830D1094: 7CE33B78  mr r3, r7
	ctx.r[3].u64 = ctx.r[7].u64;
	// 830D1098: 4BFFCFA9  bl 0x830ce040
	ctx.lr = 0x830D109C;
	sub_830CE040(ctx, base);
	// 830D109C: 387F00F8  addi r3, r31, 0xf8
	ctx.r[3].s64 = ctx.r[31].s64 + 248;
	// 830D10A0: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 830D10A4: 4BFFDBE5  bl 0x830cec88
	ctx.lr = 0x830D10A8;
	sub_830CEC88(ctx, base);
	// 830D10A8: 4800000C  b 0x830d10b4
	pc = 0x830D10B4; continue 'dispatch;
	// 830D10AC: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 830D10B0: 60630057  ori r3, r3, 0x57
	ctx.r[3].u64 = ctx.r[3].u64 | 87;
	// 830D10B4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 830D10B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830D10BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830D10C0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 830D10C4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 830D10C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830D10D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x830D10D0 size=116
    let mut pc: u32 = 0x830D10D0;
    'dispatch: loop {
        match pc {
            0x830D10D0 => {
    //   block [0x830D10D0..0x830D1144)
	// 830D10D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830D10D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830D10D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830D10DC: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 830D10E0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 830D10E4: 4BFFD38D  bl 0x830ce470
	ctx.lr = 0x830D10E8;
	sub_830CE470(ctx, base);
	// 830D10E8: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 830D10EC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830D10F0: 419A003C  beq cr6, 0x830d112c
	if ctx.cr[6].eq {
	pc = 0x830D112C; continue 'dispatch;
	}
	// 830D10F4: 81450000  lwz r10, 0(r5)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 830D10F8: 2F0A0007  cmpwi cr6, r10, 7
	ctx.cr[6].compare_i32(ctx.r[10].s32, 7, &mut ctx.xer);
	// 830D10FC: 409A0030  bne cr6, 0x830d112c
	if !ctx.cr[6].eq {
	pc = 0x830D112C; continue 'dispatch;
	}
	// 830D1100: C0050004  lfs f0, 4(r5)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 830D1104: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830D1108: C1A50008  lfs f13, 8(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(8 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 830D110C: C185000C  lfs f12, 0xc(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(12 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 830D1110: D00B00B8  stfs f0, 0xb8(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(184 as u32), tmp.u32 ) };
	// 830D1114: D1AB00BC  stfs f13, 0xbc(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(188 as u32), tmp.u32 ) };
	// 830D1118: D18B00C0  stfs f12, 0xc0(r11)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(192 as u32), tmp.u32 ) };
	// 830D111C: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 830D1120: 614A0002  ori r10, r10, 2
	ctx.r[10].u64 = ctx.r[10].u64 | 2;
	// 830D1124: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 830D1128: 4800000C  b 0x830d1134
	pc = 0x830D1134; continue 'dispatch;
	// 830D112C: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 830D1130: 60630057  ori r3, r3, 0x57
	ctx.r[3].u64 = ctx.r[3].u64 | 87;
	// 830D1134: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830D1138: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830D113C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830D1140: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830D1148(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x830D1148 size=116
    let mut pc: u32 = 0x830D1148;
    'dispatch: loop {
        match pc {
            0x830D1148 => {
    //   block [0x830D1148..0x830D11BC)
	// 830D1148: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830D114C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830D1150: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830D1154: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 830D1158: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 830D115C: 4BFFD315  bl 0x830ce470
	ctx.lr = 0x830D1160;
	sub_830CE470(ctx, base);
	// 830D1160: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 830D1164: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830D1168: 419A003C  beq cr6, 0x830d11a4
	if ctx.cr[6].eq {
	pc = 0x830D11A4; continue 'dispatch;
	}
	// 830D116C: 81450000  lwz r10, 0(r5)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 830D1170: 2F0A0007  cmpwi cr6, r10, 7
	ctx.cr[6].compare_i32(ctx.r[10].s32, 7, &mut ctx.xer);
	// 830D1174: 409A0030  bne cr6, 0x830d11a4
	if !ctx.cr[6].eq {
	pc = 0x830D11A4; continue 'dispatch;
	}
	// 830D1178: C0050004  lfs f0, 4(r5)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 830D117C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830D1180: C1A50008  lfs f13, 8(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(8 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 830D1184: C185000C  lfs f12, 0xc(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(12 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 830D1188: D00B00C8  stfs f0, 0xc8(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(200 as u32), tmp.u32 ) };
	// 830D118C: D1AB00CC  stfs f13, 0xcc(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(204 as u32), tmp.u32 ) };
	// 830D1190: D18B00D0  stfs f12, 0xd0(r11)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(208 as u32), tmp.u32 ) };
	// 830D1194: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 830D1198: 614A0002  ori r10, r10, 2
	ctx.r[10].u64 = ctx.r[10].u64 | 2;
	// 830D119C: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 830D11A0: 4800000C  b 0x830d11ac
	pc = 0x830D11AC; continue 'dispatch;
	// 830D11A4: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 830D11A8: 60630057  ori r3, r3, 0x57
	ctx.r[3].u64 = ctx.r[3].u64 | 87;
	// 830D11AC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830D11B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830D11B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830D11B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830D11C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x830D11C0 size=100
    let mut pc: u32 = 0x830D11C0;
    'dispatch: loop {
        match pc {
            0x830D11C0 => {
    //   block [0x830D11C0..0x830D1224)
	// 830D11C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830D11C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830D11C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830D11CC: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 830D11D0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 830D11D4: 4BFFD29D  bl 0x830ce470
	ctx.lr = 0x830D11D8;
	sub_830CE470(ctx, base);
	// 830D11D8: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 830D11DC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830D11E0: 419A002C  beq cr6, 0x830d120c
	if ctx.cr[6].eq {
	pc = 0x830D120C; continue 'dispatch;
	}
	// 830D11E4: 81450000  lwz r10, 0(r5)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 830D11E8: 2F0A0004  cmpwi cr6, r10, 4
	ctx.cr[6].compare_i32(ctx.r[10].s32, 4, &mut ctx.xer);
	// 830D11EC: 409A0020  bne cr6, 0x830d120c
	if !ctx.cr[6].eq {
	pc = 0x830D120C; continue 'dispatch;
	}
	// 830D11F0: C0050004  lfs f0, 4(r5)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 830D11F4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830D11F8: D00B00C4  stfs f0, 0xc4(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(196 as u32), tmp.u32 ) };
	// 830D11FC: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 830D1200: 614A0002  ori r10, r10, 2
	ctx.r[10].u64 = ctx.r[10].u64 | 2;
	// 830D1204: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 830D1208: 4800000C  b 0x830d1214
	pc = 0x830D1214; continue 'dispatch;
	// 830D120C: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 830D1210: 60630057  ori r3, r3, 0x57
	ctx.r[3].u64 = ctx.r[3].u64 | 87;
	// 830D1214: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830D1218: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830D121C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830D1220: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_830D1228(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x830D1228 size=124
    let mut pc: u32 = 0x830D1228;
    'dispatch: loop {
        match pc {
            0x830D1228 => {
    //   block [0x830D1228..0x830D12A4)
	// 830D1228: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 830D122C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 830D1230: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 830D1234: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 830D1238: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 830D123C: 4BFFD235  bl 0x830ce470
	ctx.lr = 0x830D1240;
	sub_830CE470(ctx, base);
	// 830D1240: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 830D1244: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 830D1248: 419A0044  beq cr6, 0x830d128c
	if ctx.cr[6].eq {
	pc = 0x830D128C; continue 'dispatch;
	}
	// 830D124C: 81450000  lwz r10, 0(r5)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 830D1250: 2F0A0008  cmpwi cr6, r10, 8
	ctx.cr[6].compare_i32(ctx.r[10].s32, 8, &mut ctx.xer);
	// 830D1254: 409A0038  bne cr6, 0x830d128c
	if !ctx.cr[6].eq {
	pc = 0x830D128C; continue 'dispatch;
	}
	// 830D1258: C0050004  lfs f0, 4(r5)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 830D125C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 830D1260: D00B00D4  stfs f0, 0xd4(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(212 as u32), tmp.u32 ) };
	// 830D1264: C0050008  lfs f0, 8(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 830D1268: D00B00D8  stfs f0, 0xd8(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(216 as u32), tmp.u32 ) };
	// 830D126C: C005000C  lfs f0, 0xc(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 830D1270: D00B00DC  stfs f0, 0xdc(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(220 as u32), tmp.u32 ) };
	// 830D1274: C0050010  lfs f0, 0x10(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(16 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 830D1278: D00B00E0  stfs f0, 0xe0(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(224 as u32), tmp.u32 ) };
	// 830D127C: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 830D1280: 614A0002  ori r10, r10, 2
	ctx.r[10].u64 = ctx.r[10].u64 | 2;
	// 830D1284: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 830D1288: 4800000C  b 0x830d1294
	pc = 0x830D1294; continue 'dispatch;
	// 830D128C: 3C608007  lis r3, -0x7ff9
	ctx.r[3].s64 = -2147024896;
	// 830D1290: 60630057  ori r3, r3, 0x57
	ctx.r[3].u64 = ctx.r[3].u64 | 87;
	// 830D1294: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 830D1298: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 830D129C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 830D12A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


